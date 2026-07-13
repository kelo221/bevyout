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

use crate::vsa::{
    PreparedPhysicsAsset, PreparedPhysicsBody, PreparedPhysicsClassification, PreparedPhysicsShape,
    PreparedPhysicsSource, PreparedSceneManifest, body_blocks_player, read_physics_asset,
};

use super::FlyCamera;
use super::audio::{PlayFootstep, PlayLanding};
use super::openmw_player::{
    AIR_CONTROL_FACTOR, DIRECTIONAL_JUMP_HORIZONTAL_DISTANCE, GRAVITY, LocomotionState,
    air_control_motion, jump_profile,
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
const MAX_SLIDE_PASSES: usize = 4;
const STEP_HEIGHT: f32 = 0.30;
const GROUND_SNAP_DISTANCE: f32 = 0.20;
const WALKABLE_SLOPE_COS: f32 = 0.70710677;
const WORLD_STATIC: u64 = 1;
const WORLD_DYNAMIC: u64 = 2;
const PLAYER_QUERY: u64 = 4;
const PLAYER_PROXY: u64 = 8;

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

#[derive(Component, Debug, Default)]
pub(crate) struct KccState {
    velocity: Vec3,
    grounded: bool,
}

/// Previous fixed-step position used only to render an interpolated FPS camera.
///
/// The player's [`Transform`] remains authoritative for physics and gameplay;
/// this component is deliberately just a small, allocation-free history sample.
#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct PlayerRenderHistory {
    previous_position: Vec3,
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
    .add_systems(Startup, spawn_collider_debug_hud)
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
        ),
    )
    .add_systems(
        PostUpdate,
        interpolate_fps_camera.after(TransformSystems::Propagate),
    )
    .add_systems(Update, (toggle_collider_debug, update_collider_debug_hud))
    .add_systems(Update, draw_debug_gizmos);
}

#[derive(Component)]
struct ColliderDebugHud;

fn spawn_collider_debug_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("Colliders: Off — F4"),
        ColliderDebugHud,
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

pub(crate) fn toggle_camera_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut state: ResMut<CameraModeState>,
    mut cameras: Query<ToggleCameraQuery<'_>, (With<Camera3d>, Without<FpsPlayer>)>,
    players: Query<&Transform, With<FpsPlayer>>,
) {
    if !tab_pressed(&keys) {
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
                    PlayerRenderHistory {
                        previous_position: player_center,
                    },
                    Transform::from_translation(player_center)
                        .with_rotation(Quat::from_rotation_y(yaw)),
                ))
                .id();

            camera_transform.translation = Vec3::new(0.0, CAMERA_LOCAL_HEIGHT, 0.0);
            camera_transform.rotation = Quat::from_rotation_x(pitch);
            commands.entity(camera_entity).insert(ChildOf(player));
            state.mode = CameraMode::Fps;
            state.player = Some(player);
            info!("camera mode: FPS player (Tab to return to free camera)");
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
            state.mode = CameraMode::Free;
            state.player = None;
            info!("camera mode: free camera (Tab to enter FPS player)");
        }
    }
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
