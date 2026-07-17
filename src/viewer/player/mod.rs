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
use crate::console::{ConsoleSessionStore, RefRegistry};
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
// Pure std-only equip/unequip rules (issue #98); see its module doc comment
// for why `tests/features.rs` can include it verbatim via `#[path]`.
pub(crate) mod equipment;
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
    pub(crate) collision_build_complete: bool,
    pub(crate) collisions_ready: bool,
    pub(crate) startup_initialized: bool,
}

impl Default for CameraModeState {
    fn default() -> Self {
        Self {
            mode: CameraMode::Free,
            player: None,
            collision_build_complete: false,
            collisions_ready: false,
            startup_initialized: false,
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
    /// `pub(crate)` (issue #114 added scope, M4 wave 5) so
    /// `nav/agent.rs`'s player-mirroring landmass `Character3d` can read the
    /// player's *actual* post-collision KCC velocity every fixed tick --
    /// the only cross-module edit this wave makes to the player controller
    /// itself.
    pub(crate) velocity: Vec3,
    grounded: bool,
}

#[derive(Resource, Clone, Copy, Debug, Default)]
pub(crate) struct PlayerNoClip(pub(crate) bool);

/// Runtime readiness of the active cell's staggered physics build. A swap
/// keeps the active-cell transition instant, but the player waits until the
/// destination's static collision exists and dynamic props wait until their
/// own queue phase.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum CellPhysicsReadiness {
    #[default]
    Ready,
    BuildingStatic,
    BuildingDynamic,
}

impl CellPhysicsReadiness {
    pub(crate) fn static_collision_ready(self) -> bool {
        self != Self::BuildingStatic
    }
}

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
    pub(crate) awake_dynamic_bodies: usize,
    pub(crate) sleeping_dynamic_bodies: usize,
    pub(crate) dynamic_transform_updates: usize,
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
    dynamic_entities: HashMap<BodyId, Entity>,
    sleeping_dynamic_bodies: HashSet<Entity>,
    /// Issue #64: keyframed (door/activator) bodies as their own kinematic
    /// boxddd bodies, keyed by placement root and driven every fixed step
    /// from their animated scene node's pose.
    keyframed_bodies: HashMap<Entity, Vec<collision::KeyframedColliderBinding>>,
    player_proxy: Option<BodyId>,
    surfaces: HashMap<ShapeId, CollisionSurface>,
    /// Issue #63: which cell's build created which shapes/bodies, so
    /// swap-away and eviction can tear down exactly that set (see
    /// `collision::teardown_cell_colliders`).
    ledger: super::world::CellColliderLedger<ShapeId, BodyId>,
}

impl PreparedCollisionWorld {
    /// Issues #60/#61: read-only lookup for `world::persist`'s capture
    /// (velocity snapshot) and apply (live-body restore) paths.
    pub(crate) fn dynamic_body_of(&self, entity: Entity) -> Option<BodyId> {
        self.dynamic_bodies.get(&entity).copied()
    }
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
    loaded.ensure_loaded_for(manifest, asset_root)?;
    Ok(loaded)
}

impl PreparedPhysicsAssets {
    /// Issue #52: loads and merges in whichever of `manifest`'s physics
    /// sidecars are not already present, for use when a door swap activates
    /// a cell whose physics was never loaded at startup (unlike the startup
    /// cell, which `load_prepared_physics_assets` covers up front).
    /// Sidecars already present (by relative path) are left untouched, so
    /// this is safe to call again for a cell that was already loaded.
    pub(crate) fn ensure_loaded_for(
        &mut self,
        manifest: &PreparedSceneManifest,
        asset_root: &Path,
    ) -> Result<()> {
        for placement in &manifest.placements {
            let Some(relative_path) = placement.physics_asset_path.as_ref() else {
                continue;
            };
            if self.assets.contains_key(relative_path) {
                continue;
            }
            let path = asset_root.join(relative_path.replace('/', std::path::MAIN_SEPARATOR_STR));
            self.payload_bytes = self.payload_bytes.saturating_add(
                fs::metadata(&path)
                    .map(|metadata| metadata.len())
                    .unwrap_or(0),
            );
            let asset = read_physics_asset(&path)
                .with_context(|| format!("loading prepared physics for {relative_path}"))?;
            self.assets.insert(relative_path.clone(), asset);
        }
        Ok(())
    }

    /// Merges a sidecar the preloader already read and parsed off the main
    /// thread (issue #51), so a later door swap's staggered collider build
    /// finds it cached instead of doing the file I/O inside the transition
    /// window. Paths already present are left untouched.
    pub(crate) fn insert_preloaded(
        &mut self,
        relative_path: String,
        byte_len: u64,
        asset: PreparedPhysicsAsset,
    ) {
        if self.assets.contains_key(&relative_path) {
            return;
        }
        self.payload_bytes = self.payload_bytes.saturating_add(byte_len);
        self.assets.insert(relative_path, asset);
    }

    /// Issue #52's runtime variant of the loop above: loads one sidecar by
    /// its manifest-relative path if it is not already present, warning
    /// (never erroring -- a swap must not crash the viewer) on read
    /// failure. Returns whether the sidecar is now available. Called from
    /// the staggered collider build (`advance_pending_collider_builds`),
    /// so a destination cell's sidecar file I/O is spread across frames on
    /// the same per-frame budget as the collider construction itself.
    pub(crate) fn ensure_sidecar_loaded(&mut self, relative_path: &str, asset_root: &Path) -> bool {
        if self.assets.contains_key(relative_path) {
            return true;
        }
        let path = asset_root.join(relative_path.replace('/', std::path::MAIN_SEPARATOR_STR));
        self.payload_bytes = self.payload_bytes.saturating_add(
            fs::metadata(&path)
                .map(|metadata| metadata.len())
                .unwrap_or(0),
        );
        match read_physics_asset(&path) {
            Ok(asset) => {
                self.assets.insert(relative_path.to_owned(), asset);
                true
            }
            Err(error) => {
                warn!("could not load physics sidecar {relative_path}: {error}");
                false
            }
        }
    }
}

#[derive(Resource, Clone, Copy, Debug, Default)]
pub(crate) struct PhysicsDisabled(pub(crate) bool);

#[derive(Resource, Clone, Copy, Debug, Default)]
pub(crate) struct StepDebugSettings {
    enabled: bool,
    next_log_at: f64,
}

type FpsCameraQuery<'w> = (&'w mut Transform, &'w mut FlyCamera);
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
    .insert_resource(CellPhysicsReadiness::default())
    .insert_resource(StepDebugSettings::default())
    .insert_resource(PendingColliderBuild::default())
    .add_systems(Startup, (spawn_collider_debug_hud, spawn_step_debug_hud))
    .add_systems(
        Update,
        advance_pending_collider_builds.run_if(in_state(AppState::InGame)),
    )
    .add_systems(
        FixedUpdate,
        (
            cleanup_removed_dynamic_bodies.before(step_world),
            drive_keyframed_colliders.before(step_world),
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
    .add_systems(Update, (update_collider_debug_hud, update_step_debug_hud))
    .add_systems(
        Update,
        initialize_default_fps.run_if(in_state(AppState::InGame)),
    )
    .add_systems(Update, draw_debug_gizmos);
}

#[derive(Component)]
struct ColliderDebugHud;

fn spawn_collider_debug_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("Colliders: Off"),
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

pub(crate) fn flip_collider_debug(settings: &mut BoxdddDebugDrawSettings) {
    settings.enabled = !settings.enabled;
}

fn update_collider_debug_hud(
    settings: Res<BoxdddDebugDrawSettings>,
    stats: Res<CollisionRuntimeStats>,
    mut text: Single<&mut Text, With<ColliderDebugHud>>,
) {
    text.0 = format!(
        "Colliders: {} | Dynamic: {} ({} awake / {} sleeping) | Sync: {}",
        if settings.enabled { "On" } else { "Off" },
        stats.dynamic_bodies,
        stats.awake_dynamic_bodies,
        stats.sleeping_dynamic_bodies,
        stats.dynamic_transform_updates,
    );
}

#[derive(Component)]
struct StepDebugHud;

fn spawn_step_debug_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("Stair logs: Off"),
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

pub(crate) fn flip_step_debug(settings: &mut StepDebugSettings) {
    settings.enabled = !settings.enabled;
    settings.next_log_at = 0.0;
}

pub(crate) fn step_debug_enabled(settings: &StepDebugSettings) -> bool {
    settings.enabled
}

fn update_step_debug_hud(
    settings: Res<StepDebugSettings>,
    mut text: Single<&mut Text, With<StepDebugHud>>,
) {
    text.0 = format!(
        "Stair logs: {}",
        if settings.enabled { "On" } else { "Off" }
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CameraModeError {
    CameraUnavailable,
    HierarchyInvalid,
    PlayerUnavailable,
}

pub(crate) fn initialize_default_fps(world: &mut World) {
    let state = world.resource::<CameraModeState>();
    if state.startup_initialized || !state.collision_build_complete {
        return;
    }
    match set_camera_mode(world, CameraMode::Fps) {
        Ok(_) => {
            world.resource_mut::<CameraModeState>().startup_initialized = true;
            let no_clip = world.resource::<PlayerNoClip>().0;
            info!(
                "camera mode: FPS player{}",
                if no_clip { " (forced no-clip)" } else { "" }
            );
        }
        Err(CameraModeError::CameraUnavailable) => {}
        Err(error) => warn!("could not initialize FPS camera: {error:?}"),
    }
}

pub(crate) fn toggle_camera_mode_now(world: &mut World) -> Result<CameraMode, CameraModeError> {
    let target = match world.resource::<CameraModeState>().mode {
        CameraMode::Free => CameraMode::Fps,
        CameraMode::Fps => CameraMode::Free,
    };
    set_camera_mode(world, target)
}

pub(crate) fn set_camera_mode(
    world: &mut World,
    target: CameraMode,
) -> Result<CameraMode, CameraModeError> {
    if world.resource::<CameraModeState>().mode == target {
        return Ok(target);
    }
    let camera = {
        let mut cameras = world.query_filtered::<(
            Entity,
            &Transform,
            &FlyCamera,
            &GlobalTransform,
            Option<&ChildOf>,
        ), With<Camera3d>>();
        let mut cameras = cameras.iter(world);
        let Some((entity, transform, fly, global, parent)) = cameras.next() else {
            return Err(CameraModeError::CameraUnavailable);
        };
        if cameras.next().is_some() {
            return Err(CameraModeError::CameraUnavailable);
        }
        (
            entity,
            *transform,
            (fly.yaw, fly.pitch),
            *global,
            parent.map(ChildOf::parent),
        )
    };

    match target {
        CameraMode::Fps => {
            if camera.4.is_some() || world.resource::<CameraModeState>().player.is_some() {
                return Err(CameraModeError::HierarchyInvalid);
            }
            let (yaw, pitch) = camera_angles(camera.1.rotation);
            let player_center = camera.1.translation - Vec3::Y * CAMERA_LOCAL_HEIGHT;
            let player = world
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
            if let Some(mut transform) = world.get_mut::<Transform>(camera.0) {
                transform.translation = Vec3::new(0.0, CAMERA_LOCAL_HEIGHT, 0.0);
                transform.rotation = Quat::from_rotation_x(pitch);
            }
            if let Some(mut fly) = world.get_mut::<FlyCamera>(camera.0) {
                fly.yaw = yaw;
                fly.pitch = pitch;
            }
            world.entity_mut(camera.0).insert(ChildOf(player));
            world.resource_mut::<RefRegistry>().set_player(player);
            let forced_no_clip = {
                let state = world.resource::<CameraModeState>();
                world.resource::<PhysicsDisabled>().0 || !state.collisions_ready
            };
            if forced_no_clip {
                world.resource_mut::<PlayerNoClip>().0 = true;
            }
            let mut state = world.resource_mut::<CameraModeState>();
            state.mode = CameraMode::Fps;
            state.player = Some(player);
        }
        CameraMode::Free => {
            let Some(player) = world.resource::<CameraModeState>().player else {
                return Err(CameraModeError::PlayerUnavailable);
            };
            if !world.entities().contains(player) || camera.4 != Some(player) {
                return Err(CameraModeError::HierarchyInvalid);
            }
            let (scale, rotation, translation) = camera.3.to_scale_rotation_translation();
            let (yaw, pitch) = camera_angles(rotation);
            world.entity_mut(camera.0).remove::<ChildOf>();
            if let Some(mut transform) = world.get_mut::<Transform>(camera.0) {
                transform.translation = translation;
                transform.rotation = rotation;
                transform.scale = scale;
            }
            if let Some(mut fly) = world.get_mut::<FlyCamera>(camera.0) {
                fly.yaw = yaw;
                fly.pitch = pitch;
            }
            world.resource_mut::<RefRegistry>().clear_player(player);
            world
                .resource_mut::<ConsoleSessionStore>()
                .clear_entity(player);
            world.despawn(player);
            let mut state = world.resource_mut::<CameraModeState>();
            state.mode = CameraMode::Free;
            state.player = None;
        }
    }
    Ok(target)
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

/// Issue #52: teleports whichever camera representation is currently
/// active (the FPS player body, or the free camera) to a door
/// destination's ground-level `translation`/`rotation_xyzw` (already in
/// Bevy coordinates, matching `PreparedDoorDestination`). Mirrors how
/// `scene::transition_camera_position` places the free camera at startup
/// (`translation + EYE_HEIGHT`), and resets the FPS body's physics,
/// footstep, and render-interpolation history exactly like a console
/// `setpos`/`setangle` would, via `console_transform_mutated`.
pub(crate) fn teleport_active_player(
    world: &mut World,
    translation: Vec3,
    rotation_xyzw: [f32; 4],
) {
    let rotation = Quat::from_xyzw(
        rotation_xyzw[0],
        rotation_xyzw[1],
        rotation_xyzw[2],
        rotation_xyzw[3],
    )
    .normalize();
    let (yaw, pitch) = camera_angles(rotation);
    let mode = world.resource::<CameraModeState>().mode;
    match mode {
        CameraMode::Fps => {
            let Some(player) = world.resource::<CameraModeState>().player else {
                return;
            };
            // The FPS body's `Transform` is the capsule *center*; the door
            // destination is ground-level. Eye = ground + EYE_HEIGHT, and
            // center = eye - CAMERA_LOCAL_HEIGHT (see `set_camera_mode`),
            // so center = ground + CAPSULE_HEIGHT * 0.5.
            let player_center = translation + Vec3::Y * (CAPSULE_HEIGHT * 0.5);
            if let Some(mut transform) = world.get_mut::<Transform>(player) {
                transform.translation = player_center;
                transform.rotation = Quat::from_rotation_y(yaw);
            }
            if let Some(mut fps_player) = world.get_mut::<FpsPlayer>(player) {
                fps_player.yaw = yaw;
                fps_player.pitch = pitch;
            }
            let camera_entity = {
                let mut cameras = world.query_filtered::<(Entity, &ChildOf), With<Camera3d>>();
                cameras
                    .iter(world)
                    .find_map(|(camera, parent)| (parent.parent() == player).then_some(camera))
            };
            if let Some(camera_entity) = camera_entity {
                if let Some(mut transform) = world.get_mut::<Transform>(camera_entity) {
                    transform.rotation = Quat::from_rotation_x(pitch);
                }
                if let Some(mut fly) = world.get_mut::<FlyCamera>(camera_entity) {
                    fly.yaw = yaw;
                    fly.pitch = pitch;
                }
            }
            console_transform_mutated(world, player);
        }
        CameraMode::Free => {
            let camera_entity = {
                let mut cameras =
                    world.query_filtered::<Entity, (With<Camera3d>, Without<ChildOf>)>();
                cameras.iter(world).next()
            };
            let Some(camera_entity) = camera_entity else {
                return;
            };
            let eye_position = translation + Vec3::Y * EYE_HEIGHT;
            if let Some(mut transform) = world.get_mut::<Transform>(camera_entity) {
                transform.translation = eye_position;
                transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
            }
            if let Some(mut fly) = world.get_mut::<FlyCamera>(camera_entity) {
                fly.yaw = yaw;
                fly.pitch = pitch;
            }
        }
    }
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
