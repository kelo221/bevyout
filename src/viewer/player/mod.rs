use anyhow::{Context, Result};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::transform::TransformSystems;
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy_boxddd::boxddd::{
    self, BodyDef, BodyId, BodyType, BoxHull, CollisionPlane, Filter, Hull, ShapeDef, ShapeId,
};
use bevy_boxddd::prelude::{
    BoxdddDebugDrawSettings, BoxdddPhysicsContext, BoxdddPhysicsPlugin, BoxdddPhysicsSettings,
    draw_debug_gizmos,
};
use bevy_boxddd::resources::BoxdddErrorPolicy;
use bevy_boxddd::systems::{step_world, sync_boxddd_transforms_to_bevy};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::app_state::{AppState, GameplayModal};
use crate::console::RefRegistry;
use crate::vsa::{
    PreparedPhysicsAsset, PreparedPhysicsBody, PreparedPhysicsClassification, PreparedPhysicsShape,
    PreparedPhysicsSource, PreparedSceneManifest, body_blocks_player, read_physics_asset,
};

use super::FlyCamera;
use super::audio::{PlayFootstep, PlayLanding};
use super::openmw_player::{
    AIR_CONTROL_FACTOR, DEFAULT_STEP_HEIGHT, DIRECTIONAL_JUMP_HORIZONTAL_DISTANCE, GRAVITY,
    LocomotionState, air_control_motion, jump_profile,
};

mod camera;
mod collision;
mod movement;
mod surface;

pub(crate) use camera::*;
pub(crate) use collision::*;
pub(crate) use movement::*;
pub(crate) use surface::*;

pub(crate) const CAPSULE_RADIUS: f32 = 0.35;
pub(crate) const CAPSULE_HEIGHT: f32 = 1.8;
pub(crate) const EYE_HEIGHT: f32 = 1.6;
const CAMERA_LOCAL_HEIGHT: f32 = EYE_HEIGHT - CAPSULE_HEIGHT * 0.5;
const PLAYER_SPEED: f32 = 4.5;
const MOUSE_SENSITIVITY: f32 = 0.002;
const FOOTSTEP_DISTANCE: f32 = 1.45;
const DEFAULT_FOOTSTEP_SURFACE: &str = "concrete";
const CAMERA_VERTICAL_SETTLE_SECONDS: f32 = 0.12;
const CAMERA_VERTICAL_SETTLE_LOG_FACTOR: f32 = 2.995_732_3;
const MAX_SLIDE_PASSES: usize = 4;
const STEP_HEIGHT: f32 = DEFAULT_STEP_HEIGHT;
const STEP_CLEARANCE: f32 = 0.02;
const STEP_SWEEP_DISTANCE: f32 = STEP_HEIGHT + STEP_CLEARANCE;
const STEP_VALIDATION_EPSILON: f32 = 0.001;
const WALKABLE_SLOPE_COS: f32 = 0.70710677;
const WORLD_STATIC: u64 = 1;
const WORLD_DYNAMIC: u64 = 2;
const PLAYER_QUERY: u64 = 4;
const PLAYER_PROXY: u64 = 8;
const STEP_SUPPORT: u64 = 16;

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CameraMode {
    Free,
    Fps,
}

#[derive(Resource, Debug)]
pub(crate) struct CameraModeState {
    pub(crate) mode: CameraMode,
    pub(crate) player: Option<Entity>,
    pub(crate) collisions_ready: bool,
}

impl Default for CameraModeState {
    fn default() -> Self {
        Self {
            mode: CameraMode::Free,
            player: None,
            collisions_ready: false,
        }
    }
}

#[derive(Component, Debug)]
pub(crate) struct FpsPlayer {
    yaw: f32,
    pitch: f32,
}

/// Marker used by render diagnostics for player and prepared physics entities.
#[derive(Component)]
pub(crate) struct PhysicsCollider;

#[derive(Component, Debug)]
pub(crate) struct FootstepState {
    last_position: Vec3,
    distance: f32,
    step_index: usize,
    initialized: bool,
}

impl Default for FootstepState {
    fn default() -> Self {
        Self {
            last_position: Vec3::ZERO,
            distance: 0.0,
            step_index: 0,
            initialized: false,
        }
    }
}

impl FootstepState {
    fn reset_tracking(&mut self) {
        self.initialized = false;
        self.distance = 0.0;
    }

    fn reset_at(&mut self, position: Vec3) {
        self.last_position = position;
        self.distance = 0.0;
        self.initialized = true;
    }

    fn record_motion(&mut self, position: Vec3, grounded: bool) {
        if !self.initialized {
            self.last_position = position;
            self.initialized = true;
            return;
        }

        let delta = position - self.last_position;
        self.last_position = position;
        if !grounded {
            self.distance = 0.0;
            return;
        }
        self.distance += Vec3::new(delta.x, 0.0, delta.z).length();
    }

    fn take_step(&mut self) -> Option<(bool, usize)> {
        if self.distance < FOOTSTEP_DISTANCE {
            return None;
        }
        self.distance -= FOOTSTEP_DISTANCE;
        let right = self.step_index % 2 == 1;
        let variant = self.step_index / 2;
        self.step_index = self.step_index.wrapping_add(1);
        Some((right, variant))
    }
}

#[derive(Component, Debug, Default)]
pub(crate) struct KccState {
    velocity: Vec3,
    grounded: bool,
}

#[derive(Resource, Clone, Copy, Debug, Default)]
pub(crate) struct PlayerNoClip(pub(crate) bool);

/// Previous fixed-step position used only to render an interpolated FPS camera.
///
/// The player's [`Transform`] remains authoritative for physics and gameplay;
/// this component is deliberately just a small, allocation-free history sample.
#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct PlayerRenderHistory {
    previous_position: Vec3,
    smoothed_y: f32,
    last_target_y: f32,
    was_grounded: bool,
    vertical_initialized: bool,
}

impl PlayerRenderHistory {
    fn new(position: Vec3) -> Self {
        Self {
            previous_position: position,
            smoothed_y: position.y,
            last_target_y: position.y,
            was_grounded: false,
            vertical_initialized: false,
        }
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub(crate) struct CollisionRuntimeStats {
    pub(crate) authored_assets: usize,
    pub(crate) fallback_assets: usize,
    pub(crate) bodies: usize,
    pub(crate) shapes: usize,
    pub(crate) filtered_shapes: usize,
    pub(crate) packed_triangles: usize,
    pub(crate) dynamic_bodies: usize,
    pub(crate) sidecar_bytes: u64,
    pub(crate) cooking_millis: f64,
    pub(crate) shape_kinds: HashMap<&'static str, usize>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CollisionSurface {
    material: Option<u32>,
}

#[derive(Resource, Default)]
pub(crate) struct PreparedCollisionWorld {
    static_body: Option<BodyId>,
    dynamic_bodies: HashMap<Entity, BodyId>,
    player_proxy: Option<BodyId>,
    surfaces: HashMap<ShapeId, CollisionSurface>,
}

#[derive(Resource, Default)]
pub(crate) struct PreparedPhysicsAssets {
    assets: HashMap<String, PreparedPhysicsAsset>,
    payload_bytes: u64,
}

pub(crate) fn load_prepared_physics_assets(
    manifest: &PreparedSceneManifest,
    asset_root: &Path,
) -> Result<PreparedPhysicsAssets> {
    let mut loaded = PreparedPhysicsAssets::default();
    for placement in &manifest.placements {
        let Some(relative_path) = placement.physics_asset_path.as_ref() else {
            continue;
        };
        if loaded.assets.contains_key(relative_path) {
            continue;
        }
        let path = asset_root.join(relative_path.replace('/', std::path::MAIN_SEPARATOR_STR));
        loaded.payload_bytes = loaded.payload_bytes.saturating_add(
            fs::metadata(&path)
                .map(|metadata| metadata.len())
                .unwrap_or(0),
        );
        let asset = read_physics_asset(&path)
            .with_context(|| format!("loading prepared physics for {relative_path}"))?;
        loaded.assets.insert(relative_path.clone(), asset);
    }
    Ok(loaded)
}

#[derive(Resource, Clone, Copy, Debug, Default)]
pub(crate) struct PhysicsDisabled(pub(crate) bool);

#[derive(Resource, Clone, Copy, Debug, Default)]
pub(crate) struct StepDebugSettings {
    enabled: bool,
    next_log_at: f64,
}

type FpsCameraQuery<'w> = (&'w mut Transform, &'w mut FlyCamera);
type ToggleCameraQuery<'w> = (
    Entity,
    &'w mut Transform,
    &'w mut FlyCamera,
    &'w GlobalTransform,
    Has<ChildOf>,
);
pub(crate) fn install(app: &mut App, disable_physics: bool) {
    app.add_plugins(BoxdddPhysicsPlugin::new(BoxdddPhysicsSettings {
        gravity: Vec3::new(0.0, -GRAVITY, 0.0),
        error_policy: BoxdddErrorPolicy::MessageAndLog,
        ..default()
    }))
    .insert_resource(CameraModeState::default())
    .insert_resource(CollisionRuntimeStats::default())
    .insert_resource(PreparedCollisionWorld::default())
    .insert_resource(PhysicsDisabled(disable_physics))
    .insert_resource(PlayerNoClip::default())
    .insert_resource(StepDebugSettings::default())
    .add_systems(Startup, (spawn_collider_debug_hud, spawn_step_debug_hud))
    .add_systems(PostStartup, build_prepared_colliders)
    .add_systems(
        FixedUpdate,
        (
            cleanup_removed_dynamic_bodies.before(step_world),
            capture_player_render_history.before(apply_player_controls),
            apply_player_controls.before(sync_player_proxy),
            sync_player_proxy.before(step_world),
            emit_landing_events.after(step_world),
            emit_footsteps.after(emit_landing_events),
            sync_dynamic_transforms.after(sync_boxddd_transforms_to_bevy),
        )
            .run_if(in_state(AppState::InGame))
            .run_if(in_state(GameplayModal::None)),
    )
    .add_systems(
        PostUpdate,
        interpolate_fps_camera
            .after(TransformSystems::Propagate)
            .run_if(in_state(AppState::InGame))
            .run_if(in_state(GameplayModal::None)),
    )
    .add_systems(
        Update,
        (
            toggle_collider_debug,
            update_collider_debug_hud,
            toggle_step_debug,
            update_step_debug_hud,
        ),
    )
    .add_systems(Update, draw_debug_gizmos);
}

#[derive(Component)]
struct ColliderDebugHud;

fn spawn_collider_debug_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("Colliders: Off — F4"),
        ColliderDebugHud,
        super::console::DiagnosticUi,
        TextColor(Color::srgb(0.7, 0.9, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            right: px(10),
            bottom: px(10),
            ..default()
        },
        ZIndex(120),
    ));
}

fn toggle_collider_debug(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<BoxdddDebugDrawSettings>,
) {
    if keys.just_pressed(KeyCode::F4) {
        flip_collider_debug(&mut settings);
        info!(
            "BoxDDD native collider overlay: {}",
            if settings.enabled { "on" } else { "off" }
        );
    }
}

fn flip_collider_debug(settings: &mut BoxdddDebugDrawSettings) {
    settings.enabled = !settings.enabled;
}

fn update_collider_debug_hud(
    settings: Res<BoxdddDebugDrawSettings>,
    mut text: Single<&mut Text, With<ColliderDebugHud>>,
) {
    text.0 = format!(
        "Colliders: {} — F4",
        if settings.enabled { "On" } else { "Off" }
    );
}

#[derive(Component)]
struct StepDebugHud;

fn spawn_step_debug_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("Stair logs: Off — F5"),
        StepDebugHud,
        super::console::DiagnosticUi,
        TextColor(Color::srgb(0.7, 0.9, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            right: px(10),
            bottom: px(34),
            ..default()
        },
        ZIndex(120),
    ));
}

fn toggle_step_debug(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<StepDebugSettings>) {
    if keys.just_pressed(KeyCode::F5) {
        flip_step_debug(&mut settings);
        info!(
            target: "bevyout::stair_debug",
            "[stair-debug] rejection logging {} (F5)",
            if settings.enabled { "enabled" } else { "disabled" }
        );
    }
}

fn flip_step_debug(settings: &mut StepDebugSettings) {
    settings.enabled = !settings.enabled;
    settings.next_log_at = 0.0;
}

fn update_step_debug_hud(
    settings: Res<StepDebugSettings>,
    mut text: Single<&mut Text, With<StepDebugHud>>,
) {
    text.0 = format!(
        "Stair logs: {} — F5",
        if settings.enabled { "On" } else { "Off" }
    );
}

pub(crate) fn toggle_camera_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut state: ResMut<CameraModeState>,
    mut cameras: Query<ToggleCameraQuery<'_>, (With<Camera3d>, Without<FpsPlayer>)>,
    players: Query<&Transform, With<FpsPlayer>>,
    mut references: ResMut<RefRegistry>,
) {
    if !camera_toggle_pressed(&keys) {
        return;
    }

    let Ok((camera_entity, mut camera_transform, mut fly_camera, camera_global, has_parent)) =
        cameras.single_mut()
    else {
        warn!("cannot toggle camera mode: expected one camera");
        return;
    };

    match state.mode {
        CameraMode::Free => {
            if !state.collisions_ready {
                warn!(
                    "FPS mode unavailable: no static scene collision geometry has finished building"
                );
                return;
            }
            if has_parent || state.player.is_some() {
                warn!("cannot enter FPS mode: camera/player hierarchy is already active");
                return;
            }

            let (yaw, pitch) = camera_angles(camera_transform.rotation);
            fly_camera.yaw = yaw;
            fly_camera.pitch = pitch;
            let player_center = camera_transform.translation - Vec3::Y * CAMERA_LOCAL_HEIGHT;
            let player = commands
                .spawn((
                    FpsPlayer { yaw, pitch },
                    FootstepState::default(),
                    LocomotionState::default(),
                    KccState::default(),
                    PhysicsCollider,
                    PlayerRenderHistory::new(player_center),
                    Transform::from_translation(player_center)
                        .with_rotation(Quat::from_rotation_y(yaw)),
                ))
                .id();
            references.set_player(player);

            camera_transform.translation = Vec3::new(0.0, CAMERA_LOCAL_HEIGHT, 0.0);
            camera_transform.rotation = Quat::from_rotation_x(pitch);
            commands.entity(camera_entity).insert(ChildOf(player));
            state.mode = CameraMode::Fps;
            state.player = Some(player);
            info!("camera mode: FPS player (V to return to free camera)");
        }
        CameraMode::Fps => {
            let Some(player_entity) = state.player else {
                warn!("cannot leave FPS mode: player entity is missing");
                state.mode = CameraMode::Free;
                return;
            };
            let Ok(_player_transform) = players.get(player_entity) else {
                warn!("cannot leave FPS mode: player entity is not available yet");
                return;
            };
            if !has_parent {
                warn!("cannot leave FPS mode: camera is not parented to the player");
                return;
            }

            // Use the last rendered camera pose so leaving FPS mode does not
            // snap back to the latest unsmoothed fixed-step player position.
            let world_camera = *camera_global;
            let (scale, rotation, translation) = world_camera.to_scale_rotation_translation();
            let (yaw, pitch) = camera_angles(rotation);
            fly_camera.yaw = yaw;
            fly_camera.pitch = pitch;
            camera_transform.translation = translation;
            camera_transform.rotation = rotation;
            camera_transform.scale = scale;
            commands.entity(camera_entity).remove::<ChildOf>();
            commands.entity(player_entity).despawn();
            references.clear_player(player_entity);
            state.mode = CameraMode::Free;
            state.player = None;
            info!("camera mode: free camera (V to enter FPS player)");
        }
    }
}

pub(crate) fn console_transform_mutated(world: &mut World, entity: Entity) {
    if world.get::<FpsPlayer>(entity).is_none() {
        return;
    }
    let position = world
        .get::<Transform>(entity)
        .map(|transform| transform.translation)
        .unwrap_or(Vec3::ZERO);
    if let Some(mut kcc) = world.get_mut::<KccState>(entity) {
        kcc.velocity = Vec3::ZERO;
        kcc.grounded = false;
    }
    if let Some(mut footsteps) = world.get_mut::<FootstepState>(entity) {
        footsteps.reset_at(position);
    }
    if let Some(mut history) = world.get_mut::<PlayerRenderHistory>(entity) {
        *history = PlayerRenderHistory::new(position);
    }
}

pub(crate) fn console_get_angles(world: &World, entity: Entity) -> Option<Vec3> {
    world
        .get::<FpsPlayer>(entity)
        .map(|player| Vec3::new(player.pitch.to_degrees(), player.yaw.to_degrees(), 0.0))
}

pub(crate) fn console_set_angles(world: &mut World, entity: Entity, angles: Vec3) -> bool {
    if world.get::<FpsPlayer>(entity).is_none() {
        return false;
    }
    let yaw = angles.y.to_radians();
    let pitch = angles.x.to_radians().clamp(-1.5, 1.5);
    if let Some(mut player) = world.get_mut::<FpsPlayer>(entity) {
        player.yaw = yaw;
        player.pitch = pitch;
    }
    if let Some(mut transform) = world.get_mut::<Transform>(entity) {
        transform.rotation = Quat::from_rotation_y(yaw);
    }
    let camera_entity = {
        let mut cameras = world.query_filtered::<(Entity, &ChildOf), With<Camera3d>>();
        cameras
            .iter(world)
            .find_map(|(camera, parent)| (parent.parent() == entity).then_some(camera))
    };
    if let Some(camera_entity) = camera_entity {
        if let Some(mut transform) = world.get_mut::<Transform>(camera_entity) {
            transform.rotation = Quat::from_rotation_x(pitch);
        }
        if let Some(mut camera) = world.get_mut::<FlyCamera>(camera_entity) {
            camera.yaw = yaw;
            camera.pitch = pitch;
        }
    }
    console_transform_mutated(world, entity);
    true
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
