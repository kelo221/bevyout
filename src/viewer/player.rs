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
struct FootstepState {
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
struct KccState {
    velocity: Vec3,
    grounded: bool,
}

/// Previous fixed-step position used only to render an interpolated FPS camera.
///
/// The player's [`Transform`] remains authoritative for physics and gameplay;
/// this component is deliberately just a small, allocation-free history sample.
#[derive(Component, Clone, Copy, Debug)]
struct PlayerRenderHistory {
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
struct CollisionSurface {
    material: Option<u32>,
}

#[derive(Resource, Default)]
struct PreparedCollisionWorld {
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

fn capture_player_render_history(
    mut players: Query<(&Transform, &mut PlayerRenderHistory), With<FpsPlayer>>,
) {
    let Ok((transform, mut history)) = players.single_mut() else {
        return;
    };
    history.previous_position = transform.translation;
}

fn interpolate_fps_camera(
    fixed_time: Res<Time<Fixed>>,
    state: Res<CameraModeState>,
    players: Query<(&Transform, &PlayerRenderHistory), With<FpsPlayer>>,
    mut cameras: Query<(&Transform, &mut GlobalTransform), With<Camera3d>>,
) {
    if state.mode != CameraMode::Fps {
        return;
    }
    let Some(player_entity) = state.player else {
        return;
    };
    let Ok((player_transform, history)) = players.get(player_entity) else {
        return;
    };
    let Ok((camera_transform, mut camera_global)) = cameras.single_mut() else {
        return;
    };

    let interpolated_player_position = interpolate_render_position(
        history.previous_position,
        player_transform.translation,
        fixed_time.overstep_fraction(),
    );

    // Rebuild the authoritative camera pose from local transforms instead of
    // reading the previous render override. This prevents offsets accumulating
    // when Bevy skips propagation for an unchanged hierarchy.
    let authoritative_camera =
        GlobalTransform::from(*player_transform).mul_transform(*camera_transform);
    let (scale, rotation, translation) = authoritative_camera.to_scale_rotation_translation();
    let render_translation =
        translation + (interpolated_player_position - player_transform.translation);
    *camera_global = GlobalTransform::from(Transform {
        translation: render_translation,
        rotation,
        scale,
    });
}

fn interpolate_render_position(previous: Vec3, current: Vec3, alpha: f32) -> Vec3 {
    previous.lerp(current, alpha.clamp(0.0, 1.0))
}

pub(crate) fn fps_mouse_look(
    mut mouse: MessageReader<MouseMotion>,
    cursor_options: Single<&CursorOptions>,
    state: Res<CameraModeState>,
    mut players: Query<(&mut FpsPlayer, &mut Transform), Without<ChildOf>>,
    mut cameras: Query<FpsCameraQuery<'_>, (With<Camera3d>, With<ChildOf>)>,
) {
    let delta = mouse
        .read()
        .fold(Vec2::ZERO, |sum, event| sum + event.delta);
    if state.mode != CameraMode::Fps
        || !matches!(cursor_options.grab_mode, CursorGrabMode::Locked)
        || delta == Vec2::ZERO
    {
        return;
    }

    let Ok((mut player, mut player_transform)) = players.single_mut() else {
        return;
    };
    let Ok((mut camera_transform, mut fly_camera)) = cameras.single_mut() else {
        return;
    };
    player.yaw -= delta.x * MOUSE_SENSITIVITY;
    player.pitch = (player.pitch - delta.y * MOUSE_SENSITIVITY).clamp(-1.5, 1.5);
    player_transform.rotation = Quat::from_rotation_y(player.yaw);
    camera_transform.rotation = Quat::from_rotation_x(player.pitch);
    fly_camera.yaw = player.yaw;
    fly_camera.pitch = player.pitch;
}

fn apply_player_controls(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<CameraModeState>,
    physics_disabled: Res<PhysicsDisabled>,
    time: Res<Time<Fixed>>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    mut players: Query<(
        &FpsPlayer,
        &mut Transform,
        &mut KccState,
        &mut LocomotionState,
    )>,
) {
    let Ok((player, mut transform, mut kcc, mut locomotion)) = players.single_mut() else {
        return;
    };
    let jump_pressed = keys.pressed(KeyCode::Space);
    let jump_started = jump_pressed && !locomotion.jump_was_pressed();
    locomotion.set_jump_pressed(jump_pressed);
    if physics_disabled.0 || state.mode != CameraMode::Fps {
        kcc.velocity = Vec3::ZERO;
        kcc.grounded = false;
        return;
    }
    let Some(world) = context.world_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let mover = boxddd::Capsule::new(
        [0.0, -(CAPSULE_HEIGHT * 0.5 - CAPSULE_RADIUS), 0.0],
        [0.0, CAPSULE_HEIGHT * 0.5 - CAPSULE_RADIUS, 0.0],
        CAPSULE_RADIUS,
    );
    let filter = boxddd::QueryFilter::new()
        .category_bits(PLAYER_QUERY)
        .mask_bits(WORLD_STATIC | WORLD_DYNAMIC);
    let origin = to_box_vec3(transform.translation);
    let initial_planes = world
        .collide_mover(origin, &mover, filter)
        .unwrap_or_default();
    let mut grounded = has_walkable_plane(&initial_planes);
    kcc.grounded = grounded;

    let yaw = Quat::from_rotation_y(player.yaw);
    let mut input = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        input -= Vec3::Z;
    }
    if keys.pressed(KeyCode::KeyS) {
        input += Vec3::Z;
    }
    if keys.pressed(KeyCode::KeyD) {
        input += Vec3::X;
    }
    if keys.pressed(KeyCode::KeyA) {
        input -= Vec3::X;
    }
    let world_input = yaw * input;
    let ground_target = air_control_motion(world_input, false) * PLAYER_SPEED;
    if jump_started && grounded {
        let (height, direction) = jump_profile(world_input);
        kcc.velocity.y = (2.0 * GRAVITY * height).sqrt();
        if let Some(direction) = direction {
            let airtime = 2.0 * kcc.velocity.y / GRAVITY;
            let inherited_speed = Vec3::new(kcc.velocity.x, 0.0, kcc.velocity.z)
                .dot(direction)
                .max(0.0);
            let distance_speed = DIRECTIONAL_JUMP_HORIZONTAL_DISTANCE / airtime;
            let launch_speed = inherited_speed
                .max(ground_target.length())
                .max(distance_speed);
            kcc.velocity.x = direction.x * launch_speed;
            kcc.velocity.z = direction.z * launch_speed;
        } else {
            kcc.velocity.x = ground_target.x;
            kcc.velocity.z = ground_target.z;
        }
        locomotion.mark_jump_started();
        grounded = false;
    }

    if grounded {
        kcc.velocity.x = ground_target.x;
        kcc.velocity.z = ground_target.z;
        if kcc.velocity.y < 0.0 {
            kcc.velocity.y = 0.0;
        }
    } else {
        // Air control changes the horizontal velocity gradually. It no longer
        // treats the reduced OpenMW air-control factor as a lower terminal
        // speed, which used to make a forward jump visibly lose momentum.
        let air_velocity = apply_air_control(
            Vec3::new(kcc.velocity.x, 0.0, kcc.velocity.z),
            world_input,
            dt,
        );
        kcc.velocity.x = air_velocity.x;
        kcc.velocity.z = air_velocity.z;
        kcc.velocity.y -= GRAVITY * dt;
    }

    let desired_delta = to_box_vec3(kcc.velocity * dt);
    let (mut position, planes) = move_mover(world, origin, &mover, desired_delta, filter);
    grounded = has_walkable_plane(&planes);
    if !grounded && kcc.velocity.y <= 0.0 {
        let snap = to_box_vec3(Vec3::new(0.0, -GROUND_SNAP_DISTANCE, 0.0));
        let fraction = world
            .cast_mover(position, &mover, snap, filter)
            .unwrap_or(1.0)
            .clamp(0.0, 1.0);
        if fraction < 1.0 {
            position = add_box_vec3(position, scale_box_vec3(snap, fraction));
            grounded = true;
        }
    }
    if grounded && kcc.velocity.y < 0.0 {
        kcc.velocity.y = 0.0;
    }
    kcc.grounded = grounded;
    transform.translation = from_box_vec3(position);
}

fn move_mover(
    world: &mut boxddd::World,
    mut position: boxddd::Vec3,
    mover: &boxddd::Capsule,
    mut remaining: boxddd::Vec3,
    filter: boxddd::QueryFilter,
) -> (boxddd::Vec3, Vec<boxddd::MoverPlane>) {
    for _ in 0..MAX_SLIDE_PASSES {
        if box_vec_length_squared(remaining) <= f32::EPSILON {
            break;
        }
        let fraction = world
            .cast_mover(position, mover, remaining, filter)
            .unwrap_or(1.0)
            .clamp(0.0, 1.0);
        position = add_box_vec3(position, scale_box_vec3(remaining, fraction));
        remaining = scale_box_vec3(remaining, 1.0 - fraction);
        let planes = world
            .collide_mover(position, mover, filter)
            .unwrap_or_default();
        if planes.is_empty() {
            break;
        }
        if planes
            .iter()
            .any(|plane| plane.plane.normal.y < WALKABLE_SLOPE_COS)
            && (remaining.x * remaining.x + remaining.z * remaining.z) > f32::EPSILON
            && let Some(stepped) = try_step_up(world, position, mover, remaining, filter)
        {
            position = stepped;
            break;
        }
        let mut solver_planes = planes
            .iter()
            .filter_map(|plane| CollisionPlane::new(plane.plane, STEP_HEIGHT, true).ok())
            .collect::<Vec<_>>();
        if let Ok(correction) = boxddd::solve_planes(boxddd::Vec3::ZERO, &mut solver_planes) {
            position = add_box_vec3(position, correction.delta);
        }
        remaining = boxddd::clip_vector(remaining, &solver_planes).unwrap_or(boxddd::Vec3::ZERO);
        if fraction >= 1.0 && box_vec_length_squared(remaining) <= f32::EPSILON {
            break;
        }
    }
    let planes = world
        .collide_mover(position, mover, filter)
        .unwrap_or_default();
    (position, planes)
}

fn try_step_up(
    world: &mut boxddd::World,
    position: boxddd::Vec3,
    mover: &boxddd::Capsule,
    remaining: boxddd::Vec3,
    filter: boxddd::QueryFilter,
) -> Option<boxddd::Vec3> {
    let up = boxddd::Vec3::new(0.0, STEP_HEIGHT, 0.0);
    if world.cast_mover(position, mover, up, filter).ok()? < 1.0 {
        return None;
    }
    let elevated = add_box_vec3(position, up);
    let horizontal = boxddd::Vec3::new(remaining.x, 0.0, remaining.z);
    let horizontal_fraction = world.cast_mover(elevated, mover, horizontal, filter).ok()?;
    let elevated = add_box_vec3(elevated, scale_box_vec3(horizontal, horizontal_fraction));
    let down = boxddd::Vec3::new(0.0, -STEP_HEIGHT, 0.0);
    let down_fraction = world.cast_mover(elevated, mover, down, filter).ok()?;
    if down_fraction >= 1.0 {
        return None;
    }
    let stepped = add_box_vec3(elevated, scale_box_vec3(down, down_fraction));
    let planes = world.collide_mover(stepped, mover, filter).ok()?;
    has_walkable_plane(&planes).then_some(stepped)
}

#[allow(clippy::too_many_arguments)]
fn build_prepared_colliders(
    mut commands: Commands,
    physics_disabled: Res<PhysicsDisabled>,
    manifest: Res<PreparedSceneManifest>,
    physics_assets: Res<PreparedPhysicsAssets>,
    mut state: ResMut<CameraModeState>,
    mut stats: ResMut<CollisionRuntimeStats>,
    mut collision_world: ResMut<PreparedCollisionWorld>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    roots: Query<(Entity, &super::interaction::PlacementRoot)>,
) {
    if physics_disabled.0 {
        return;
    }
    let Some(world) = context.world_mut() else {
        return;
    };
    let started = Instant::now();
    let static_body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    collision_world.static_body = Some(static_body);
    stats.sidecar_bytes = physics_assets.payload_bytes;
    for asset in physics_assets.assets.values() {
        match asset.source {
            PreparedPhysicsSource::AuthoredHavok => stats.authored_assets += 1,
            PreparedPhysicsSource::GeneratedRender => stats.fallback_assets += 1,
        }
    }
    let root_by_reference = roots
        .iter()
        .map(|(entity, root)| (root.placement().reference_form_id, entity))
        .collect::<HashMap<_, _>>();
    let mut unknown_layers = HashSet::new();
    let mut static_marker_spawned = false;
    for placement in manifest
        .placements
        .iter()
        .filter(|placement| placement.initially_enabled)
    {
        let Some(path) = placement.physics_asset_path.as_ref() else {
            continue;
        };
        let Some(asset) = physics_assets.assets.get(path) else {
            warn!("missing preloaded physics sidecar {path}");
            continue;
        };
        let dynamic_entity = (placement.physics_classification
            == PreparedPhysicsClassification::Dynamic)
            .then(|| root_by_reference.get(&placement.reference_form_id).copied())
            .flatten();
        for body in &asset.bodies {
            if !body_blocks_player(body) {
                stats.filtered_shapes += body.shapes.len();
                continue;
            }
            if body.layer > 43 && unknown_layers.insert(body.layer) {
                warn!(
                    "unknown Fallout Havok layer {} remains solid (reference {:08x})",
                    body.layer, placement.reference_form_id
                );
            }
            let (body_id, dynamic) = if let Some(entity) = dynamic_entity {
                let body_id = create_dynamic_body(world, placement, body);
                collision_world.dynamic_bodies.insert(entity, body_id);
                commands.entity(entity).insert(PhysicsCollider);
                stats.dynamic_bodies += 1;
                (body_id, true)
            } else {
                if !static_marker_spawned {
                    commands.spawn(PhysicsCollider);
                    static_marker_spawned = true;
                }
                (static_body, false)
            };
            stats.bodies += 1;
            for shape in &body.shapes {
                let result = create_prepared_shape(world, body_id, body, shape, placement, dynamic);
                let Some((shape_id, triangles)) = result else {
                    stats.filtered_shapes += 1;
                    warn!(
                        "BoxDDD rejected {} Havok shape for reference {:08x} body {}",
                        shape.kind(),
                        placement.reference_form_id,
                        body.group_id
                    );
                    continue;
                };
                collision_world.surfaces.insert(
                    shape_id,
                    CollisionSurface {
                        material: body.material,
                    },
                );
                stats.shapes += 1;
                stats.packed_triangles += triangles;
                *stats.shape_kinds.entry(shape.kind()).or_default() += 1;
            }
            if dynamic {
                normalize_dynamic_mass(world, body_id, body, placement.scale.abs());
            }
        }
    }
    stats.cooking_millis = started.elapsed().as_secs_f64() * 1000.0;
    state.collisions_ready = stats.shapes > 0;
    info!(
        "BoxDDD prepared collision: {} authored / {} fallback assets, {} bodies ({} dynamic), {} shapes, {} packed triangles, {} filtered, {:.1} ms cook, {} sidecar bytes",
        stats.authored_assets,
        stats.fallback_assets,
        stats.bodies,
        stats.dynamic_bodies,
        stats.shapes,
        stats.packed_triangles,
        stats.filtered_shapes,
        stats.cooking_millis,
        stats.sidecar_bytes,
    );
    if !state.collisions_ready {
        warn!(
            "prepared physics produced no active player-blocking shapes; FPS mode is unavailable"
        );
    }
}

fn create_dynamic_body(
    world: &mut boxddd::World,
    placement: &crate::vsa::PreparedPlacement,
    body: &PreparedPhysicsBody,
) -> BodyId {
    let rotation = Quat::from_array(placement.rotation_xyzw).normalize();
    let scale = placement.scale.abs();
    let mut linear_velocity = rotation * (Vec3::from_array(body.linear_velocity) * scale);
    let mut angular_velocity = rotation * Vec3::from_array(body.angular_velocity);
    if body.max_linear_velocity > 0.0 {
        linear_velocity = linear_velocity.clamp_length_max(body.max_linear_velocity * scale);
    }
    if body.max_angular_velocity > 0.0 {
        angular_velocity = angular_velocity.clamp_length_max(body.max_angular_velocity);
    }
    let body_id = world.create_body(
        BodyDef::builder()
            .body_type(BodyType::Dynamic)
            .position(to_box_vec3(Vec3::from_array(placement.translation)))
            .rotation(to_box_quat(rotation))
            .linear_velocity(to_box_vec3(linear_velocity))
            .angular_velocity(to_box_vec3(angular_velocity))
            .gravity_scale(body.gravity_factor)
            .bullet(body.ccd_enabled)
            .build(),
    );
    let _ = world.try_set_body_linear_damping(body_id, body.linear_damping.max(0.0));
    let _ = world.try_set_body_angular_damping(body_id, body.angular_damping.max(0.0));
    let _ = world.try_enable_body_sleep(body_id, body.sleep_enabled);
    body_id
}

fn create_prepared_shape(
    world: &mut boxddd::World,
    body_id: BodyId,
    body: &PreparedPhysicsBody,
    shape: &PreparedPhysicsShape,
    placement: &crate::vsa::PreparedPlacement,
    dynamic: bool,
) -> Option<(ShapeId, usize)> {
    if dynamic && !shape.supports_dynamic() {
        return None;
    }
    let scale = placement.scale.abs().max(0.0001);
    let placement_rotation = Quat::from_array(placement.rotation_xyzw).normalize();
    let placement_translation = Vec3::from_array(placement.translation);
    let point = |value: [f32; 3]| {
        let local = Vec3::from_array(value) * scale;
        if dynamic {
            local
        } else {
            placement_rotation * local + placement_translation
        }
    };
    let rotation = |value: [f32; 4]| {
        let local = Quat::from_array(value).normalize();
        if dynamic {
            local
        } else {
            placement_rotation * local
        }
    };
    let category = if dynamic { WORLD_DYNAMIC } else { WORLD_STATIC };
    let mask = if dynamic {
        WORLD_STATIC | WORLD_DYNAMIC | PLAYER_PROXY | PLAYER_QUERY
    } else {
        WORLD_DYNAMIC | PLAYER_QUERY
    };
    let shape_def = ShapeDef::builder()
        .density(if dynamic { 1.0 } else { 0.0 })
        .friction(body.friction.max(0.0))
        .restitution(body.restitution.max(0.0))
        .filter(Filter {
            category_bits: category,
            mask_bits: mask,
            group_index: 0,
        })
        .user_material_id(u64::from(body.material.unwrap_or(0)))
        .build();
    let shape_id = match shape {
        PreparedPhysicsShape::Box {
            center,
            half_extents,
            rotation_xyzw,
        } => {
            let center = point(*center);
            let rotation = rotation(*rotation_xyzw);
            let half = Vec3::from_array(*half_extents) * scale;
            let hull = BoxHull::transformed(
                half.x,
                half.y,
                half.z,
                boxddd::Transform::new(to_box_vec3(center), to_box_quat(rotation)),
            );
            world
                .try_create_hull_shape(body_id, &shape_def, &hull)
                .ok()?
        }
        PreparedPhysicsShape::Sphere { center, radius } => world
            .try_create_sphere_shape(
                body_id,
                &shape_def,
                &boxddd::Sphere::new(to_box_vec3(point(*center)), radius * scale),
            )
            .ok()?,
        PreparedPhysicsShape::Capsule {
            point1,
            point2,
            radius,
        } => world
            .try_create_capsule_shape(
                body_id,
                &shape_def,
                &boxddd::Capsule::new(
                    to_box_vec3(point(*point1)),
                    to_box_vec3(point(*point2)),
                    radius * scale,
                ),
            )
            .ok()?,
        PreparedPhysicsShape::ConvexHull { points } => {
            let points = points
                .iter()
                .map(|value| to_box_vec3(point(*value)))
                .collect::<Vec<_>>();
            let hull = Hull::from_points(&points, 64).ok()?;
            world
                .try_create_created_hull_shape(body_id, &shape_def, &hull)
                .ok()?
        }
        PreparedPhysicsShape::TriangleMesh { vertices, indices } => {
            let vertices = vertices
                .iter()
                .map(|value| to_box_vec3(point(*value)))
                .collect::<Vec<_>>();
            let indices = indices
                .iter()
                .map(|index| i32::try_from(*index).ok())
                .collect::<Option<Vec<_>>>()?;
            let mesh = boxddd::MeshData::builder(vertices, indices).build().ok()?;
            world
                .try_create_mesh_shape(body_id, &shape_def, mesh, boxddd::Vec3::new(1.0, 1.0, 1.0))
                .ok()?
        }
    };
    Some((shape_id, shape.triangle_count()))
}

fn normalize_dynamic_mass(
    world: &mut boxddd::World,
    body_id: BodyId,
    body: &PreparedPhysicsBody,
    scale: f32,
) {
    if !body.mass.is_finite() || body.mass <= 0.0 {
        return;
    }
    if world.try_apply_mass_from_shapes(body_id).is_err() {
        return;
    }
    let Ok(mut mass_data) = world.try_body_mass_data(body_id) else {
        return;
    };
    let ratio = if mass_data.mass > f32::EPSILON {
        body.mass / mass_data.mass
    } else {
        1.0
    };
    mass_data.mass = body.mass;
    mass_data.center = to_box_vec3(Vec3::from_array(body.center_of_mass) * scale);
    let authored_inertia_valid = body.inertia.iter().flatten().all(|value| value.is_finite())
        && body.inertia[0][0] > 0.0
        && body.inertia[1][1] > 0.0
        && body.inertia[2][2] > 0.0;
    if authored_inertia_valid {
        let inertia_scale = scale * scale;
        mass_data.inertia = boxddd::Matrix3 {
            cx: boxddd::Vec3::new(
                body.inertia[0][0] * inertia_scale,
                body.inertia[1][0] * inertia_scale,
                body.inertia[2][0] * inertia_scale,
            ),
            cy: boxddd::Vec3::new(
                body.inertia[0][1] * inertia_scale,
                body.inertia[1][1] * inertia_scale,
                body.inertia[2][1] * inertia_scale,
            ),
            cz: boxddd::Vec3::new(
                body.inertia[0][2] * inertia_scale,
                body.inertia[1][2] * inertia_scale,
                body.inertia[2][2] * inertia_scale,
            ),
        };
    } else {
        mass_data.inertia.cx = scale_box_vec3(mass_data.inertia.cx, ratio);
        mass_data.inertia.cy = scale_box_vec3(mass_data.inertia.cy, ratio);
        mass_data.inertia.cz = scale_box_vec3(mass_data.inertia.cz, ratio);
    }
    let _ = world.try_set_body_mass_data(body_id, mass_data);
}

fn cleanup_removed_dynamic_bodies(
    mut collision_world: ResMut<PreparedCollisionWorld>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    roots: Query<Entity, With<super::interaction::PlacementRoot>>,
) {
    let live = roots.iter().collect::<HashSet<_>>();
    let removed = collision_world
        .dynamic_bodies
        .keys()
        .filter(|entity| !live.contains(entity))
        .copied()
        .collect::<Vec<_>>();
    let Some(world) = context.world_mut() else {
        return;
    };
    for entity in removed {
        if let Some(body) = collision_world.dynamic_bodies.remove(&entity) {
            let _ = world.try_destroy_body(body);
        }
    }
}

fn sync_dynamic_transforms(
    collision_world: Res<PreparedCollisionWorld>,
    context: NonSend<BoxdddPhysicsContext>,
    mut roots: Query<&mut Transform, With<super::interaction::PlacementRoot>>,
) {
    let Some(world) = context.world() else {
        return;
    };
    for (entity, body) in &collision_world.dynamic_bodies {
        let (Ok(mut transform), Ok(physics_transform)) =
            (roots.get_mut(*entity), world.try_body_transform(*body))
        else {
            continue;
        };
        transform.translation = Vec3::new(
            physics_transform.p.x,
            physics_transform.p.y,
            physics_transform.p.z,
        );
        transform.rotation = from_box_quat(physics_transform.q);
    }
}

fn sync_player_proxy(
    physics_disabled: Res<PhysicsDisabled>,
    state: Res<CameraModeState>,
    time: Res<Time<Fixed>>,
    mut collision_world: ResMut<PreparedCollisionWorld>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    players: Query<&Transform, With<FpsPlayer>>,
) {
    let Some(world) = context.world_mut() else {
        return;
    };
    let player_transform = (!physics_disabled.0 && state.mode == CameraMode::Fps)
        .then(|| players.single().ok())
        .flatten();
    let Some(transform) = player_transform else {
        if let Some(body) = collision_world.player_proxy.take() {
            let _ = world.try_destroy_body(body);
        }
        return;
    };
    let target = boxddd::WorldTransform::new(
        to_box_vec3(transform.translation).into(),
        to_box_quat(transform.rotation),
    );
    if let Some(body) = collision_world.player_proxy {
        let _ = world.try_set_body_target_transform(body, target, time.delta_secs(), true);
        return;
    }
    let body = world.create_body(
        BodyDef::builder()
            .body_type(BodyType::Kinematic)
            .position(to_box_vec3(transform.translation))
            .rotation(to_box_quat(transform.rotation))
            .build(),
    );
    let shape_def = ShapeDef::builder()
        .density(0.0)
        .friction(0.8)
        .filter(Filter {
            category_bits: PLAYER_PROXY,
            mask_bits: WORLD_DYNAMIC,
            group_index: 0,
        })
        .build();
    let capsule = boxddd::Capsule::new(
        [0.0, -(CAPSULE_HEIGHT * 0.5 - CAPSULE_RADIUS), 0.0],
        [0.0, CAPSULE_HEIGHT * 0.5 - CAPSULE_RADIUS, 0.0],
        CAPSULE_RADIUS,
    );
    if world
        .try_create_capsule_shape(body, &shape_def, &capsule)
        .is_ok()
    {
        collision_world.player_proxy = Some(body);
    } else {
        let _ = world.try_destroy_body(body);
    }
}

fn emit_landing_events(
    state: Res<CameraModeState>,
    context: NonSend<BoxdddPhysicsContext>,
    collision_world: Res<PreparedCollisionWorld>,
    mut landings: MessageWriter<PlayLanding>,
    mut players: Query<
        (
            Entity,
            &Transform,
            &KccState,
            &mut LocomotionState,
            &mut FootstepState,
        ),
        With<FpsPlayer>,
    >,
) {
    let Ok((entity, transform, kcc, mut locomotion, mut footstep)) = players.single_mut() else {
        return;
    };
    if state.mode != CameraMode::Fps {
        locomotion.reset(transform.translation);
        footstep.initialized = false;
        footstep.distance = 0.0;
        return;
    }

    let surface = probe_surface(entity, transform.translation, &context, &collision_world);
    let airborne = !kcc.grounded || surface.is_none();
    let Some(impact) = locomotion.update(transform.translation, kcc.velocity.y, airborne) else {
        return;
    };
    let Some(surface) = surface else {
        return;
    };
    landings.write(PlayLanding {
        surface: surface.into(),
        variant: impact.variant,
        hard: impact.hard,
    });
    footstep.last_position = transform.translation;
    footstep.distance = 0.0;
}

fn emit_footsteps(
    state: Res<CameraModeState>,
    context: NonSend<BoxdddPhysicsContext>,
    collision_world: Res<PreparedCollisionWorld>,
    mut footsteps: MessageWriter<PlayFootstep>,
    mut players: Query<(Entity, &Transform, &KccState, &mut FootstepState), With<FpsPlayer>>,
) {
    let Ok((entity, transform, kcc, mut footstep)) = players.single_mut() else {
        return;
    };
    if state.mode != CameraMode::Fps {
        footstep.initialized = false;
        footstep.distance = 0.0;
        return;
    }
    let position = transform.translation;
    if !footstep.initialized {
        footstep.last_position = position;
        footstep.initialized = true;
        return;
    }
    let delta = position - footstep.last_position;
    footstep.last_position = position;
    let Some(surface) = probe_surface(entity, position, &context, &collision_world) else {
        footstep.distance = 0.0;
        return;
    };
    if kcc.velocity.y.abs() > 2.5 || !kcc.grounded {
        footstep.distance = 0.0;
        return;
    }

    let horizontal_delta = Vec3::new(delta.x, 0.0, delta.z).length();
    if horizontal_delta <= f32::EPSILON {
        return;
    }
    footstep.distance += horizontal_delta;
    while footstep.distance >= FOOTSTEP_DISTANCE {
        footstep.distance -= FOOTSTEP_DISTANCE;
        footsteps.write(PlayFootstep {
            surface: surface.into(),
            right: footstep.step_index % 2 == 1,
            variant: footstep.step_index / 2,
        });
        footstep.step_index = footstep.step_index.wrapping_add(1);
    }
}

fn probe_surface(
    _entity: Entity,
    position: Vec3,
    context: &BoxdddPhysicsContext,
    collision_world: &PreparedCollisionWorld,
) -> Option<&'static str> {
    let world = context.world()?;
    let origin = to_box_vec3(position - Vec3::Y * (CAPSULE_HEIGHT * 0.5 - 0.06));
    let translation = to_box_vec3(Vec3::new(0.0, -0.24, 0.0));
    let filter = boxddd::QueryFilter::new()
        .category_bits(PLAYER_QUERY)
        .mask_bits(WORLD_STATIC | WORLD_DYNAMIC);
    let hits = world.cast_ray(origin, translation, filter).ok()?;
    let hit = hits
        .iter()
        .min_by(|left, right| left.fraction.total_cmp(&right.fraction))?;
    let material = collision_world
        .surfaces
        .get(&hit.shape_id)
        .and_then(|surface| surface.material)
        .or_else(|| (hit.user_material_id != 0).then_some(hit.user_material_id as u32));
    Some(surface_family(material))
}

fn has_walkable_plane(planes: &[boxddd::MoverPlane]) -> bool {
    planes
        .iter()
        .any(|plane| plane.plane.normal.y >= WALKABLE_SLOPE_COS)
}

fn to_box_vec3(value: Vec3) -> boxddd::Vec3 {
    boxddd::Vec3::new(value.x, value.y, value.z)
}

fn from_box_vec3(value: boxddd::Vec3) -> Vec3 {
    Vec3::new(value.x, value.y, value.z)
}

fn to_box_quat(value: Quat) -> boxddd::Quat {
    boxddd::Quat::new(boxddd::Vec3::new(value.x, value.y, value.z), value.w)
}

fn from_box_quat(value: boxddd::Quat) -> Quat {
    Quat::from_xyzw(value.v.x, value.v.y, value.v.z, value.s).normalize()
}

fn add_box_vec3(left: boxddd::Vec3, right: boxddd::Vec3) -> boxddd::Vec3 {
    boxddd::Vec3::new(left.x + right.x, left.y + right.y, left.z + right.z)
}

fn scale_box_vec3(value: boxddd::Vec3, scalar: f32) -> boxddd::Vec3 {
    boxddd::Vec3::new(value.x * scalar, value.y * scalar, value.z * scalar)
}

fn box_vec_length_squared(value: boxddd::Vec3) -> f32 {
    value.x * value.x + value.y * value.y + value.z * value.z
}

fn approach(current: f32, target: f32, max_change: f32) -> f32 {
    let delta = target - current;
    if delta.abs() <= max_change {
        target
    } else {
        current + delta.signum() * max_change
    }
}

fn apply_air_control(current: Vec3, input: Vec3, dt: f32) -> Vec3 {
    let air_direction = air_control_motion(input, true).normalize_or_zero();
    let air_target = air_direction * PLAYER_SPEED;
    let max_change = PLAYER_SPEED * AIR_CONTROL_FACTOR * 8.0 * dt;
    Vec3::new(
        approach(current.x, air_target.x, max_change),
        0.0,
        approach(current.z, air_target.z, max_change),
    )
}

fn surface_family(material: Option<u32>) -> &'static str {
    let Some(material) = material else {
        return "concrete";
    };
    match material % 32 {
        2 => "dirt",
        4 => "grass",
        5 | 11 | 13 | 14 | 15 | 20 | 21 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 => {
            "metal_solid"
        }
        8 => "water",
        9 | 12 => "wood",
        16 => "metal_hollow",
        17 => "metal_sheet",
        18 => "gravel",
        19 => "concrete_broken",
        _ => "concrete",
    }
}

fn camera_angles(rotation: Quat) -> (f32, f32) {
    let (yaw, pitch, _) = rotation.to_euler(EulerRot::YXZ);
    (yaw, pitch.clamp(-1.5, 1.5))
}

fn tab_pressed(keys: &ButtonInput<KeyCode>) -> bool {
    keys.just_pressed(KeyCode::Tab)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::mesh::MeshPlugin;
    use bevy::time::TimeUpdateStrategy;
    use std::time::Duration;

    #[test]
    fn camera_angle_round_trip_preserves_yaw_and_pitch() {
        let yaw = 0.73;
        let pitch = -0.41;
        let rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
        let (actual_yaw, actual_pitch) = camera_angles(rotation);
        assert!((actual_yaw - yaw).abs() < 0.0001);
        assert!((actual_pitch - pitch).abs() < 0.0001);
    }

    #[test]
    fn capsule_center_offset_places_eye_at_requested_height() {
        assert!((CAMERA_LOCAL_HEIGHT - 0.7).abs() < f32::EPSILON);
        assert!((CAPSULE_HEIGHT * 0.5 + CAMERA_LOCAL_HEIGHT - EYE_HEIGHT).abs() < f32::EPSILON);
    }

    #[test]
    fn render_position_interpolates_without_extrapolation() {
        let previous = Vec3::new(1.0, 2.0, 3.0);
        let current = Vec3::new(5.0, 6.0, 7.0);
        assert_eq!(
            interpolate_render_position(previous, current, 0.0),
            previous
        );
        assert_eq!(
            interpolate_render_position(previous, current, 0.5),
            Vec3::new(3.0, 4.0, 5.0)
        );
        assert_eq!(interpolate_render_position(previous, current, 1.0), current);
        assert_eq!(interpolate_render_position(previous, current, 2.0), current);
    }

    #[test]
    fn tab_toggle_is_edge_triggered() {
        let mut keys = ButtonInput::<KeyCode>::default();
        assert!(!tab_pressed(&keys));
        keys.press(KeyCode::Tab);
        assert!(tab_pressed(&keys));
        keys.clear();
        assert!(!tab_pressed(&keys));
    }

    #[test]
    fn havok_material_ids_map_to_footstep_families_across_variants() {
        assert_eq!(surface_family(Some(0)), "concrete");
        assert_eq!(surface_family(Some(34)), "dirt");
        assert_eq!(surface_family(Some(69)), "metal_solid");
        assert_eq!(surface_family(Some(81)), "metal_sheet");
        assert_eq!(surface_family(Some(115)), "concrete_broken");
        assert_eq!(surface_family(None), "concrete");
    }

    #[test]
    fn f4_state_only_changes_native_debug_collection() {
        let mut settings = BoxdddDebugDrawSettings::default();
        let collision_filter = WORLD_STATIC | WORLD_DYNAMIC;
        assert!(!settings.enabled);
        flip_collider_debug(&mut settings);
        assert!(settings.enabled);
        assert_eq!(collision_filter, WORLD_STATIC | WORLD_DYNAMIC);
        flip_collider_debug(&mut settings);
        assert!(!settings.enabled);
    }

    #[test]
    fn boxddd_capsule_cast_stops_on_static_floor() {
        use bevy_boxddd::boxddd::{
            BoxHull, Capsule, QueryFilter, Vec3 as BoxVec3, World, WorldDef,
        };

        let mut world = World::new(WorldDef::default()).expect("BoxDDD world");
        let floor = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
        world.create_hull_shape(floor, &ShapeDef::default(), &BoxHull::new(10.0, 0.5, 10.0));
        let mover = Capsule::new([0.0, -0.55, 0.0], [0.0, 0.55, 0.0], CAPSULE_RADIUS);
        let start = BoxVec3::new(0.0, 5.0, 0.0);
        let fraction = world
            .cast_mover(
                start,
                &mover,
                BoxVec3::new(0.0, -10.0, 0.0),
                QueryFilter::default(),
            )
            .expect("capsule cast");
        assert!(fraction < 1.0);
    }

    #[test]
    fn dynamic_props_settle_collide_push_sync_and_cleanup() {
        use bevy_boxddd::boxddd::{
            Capsule, Quat as BoxQuat, Sphere, Vec3 as BoxVec3, World, WorldDef, WorldTransform,
        };

        let mut world = World::new(
            WorldDef::builder()
                .gravity(BoxVec3::new(0.0, -GRAVITY, 0.0))
                .build(),
        )
        .expect("BoxDDD world");
        let floor = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
        world.create_hull_shape(
            floor,
            &ShapeDef::default(),
            &BoxHull::transformed(
                5.0,
                0.25,
                5.0,
                boxddd::Transform::new(BoxVec3::new(0.0, -0.25, 0.0), BoxQuat::IDENTITY),
            ),
        );
        let make_prop = |world: &mut World, position: [f32; 3]| {
            let body = world.create_body(
                BodyDef::builder()
                    .body_type(BodyType::Dynamic)
                    .position(position)
                    .build(),
            );
            world.create_sphere_shape(
                body,
                &ShapeDef::builder().density(1.0).friction(0.8).build(),
                &Sphere::new(BoxVec3::ZERO, 0.3),
            );
            body
        };
        let first = make_prop(&mut world, [0.0, 1.5, 0.0]);
        let second = make_prop(&mut world, [0.7, 0.3, 0.0]);
        for _ in 0..180 {
            world.step(1.0 / 60.0, 4);
        }
        assert!(world.body_position(first).y < 0.45);
        assert!(world.body_position(first).y > 0.20);

        let proxy = world.create_body(
            BodyDef::builder()
                .body_type(BodyType::Kinematic)
                .position([-1.5, 0.9, 0.0])
                .build(),
        );
        world.create_capsule_shape(
            proxy,
            &ShapeDef::builder().friction(0.8).build(),
            &Capsule::new([0.0, -0.55, 0.0], [0.0, 0.55, 0.0], 0.35),
        );
        let second_before = world.body_position(second).x;
        for step in 1..=120 {
            let x = -1.5 + step as f32 * 0.025;
            world
                .try_set_body_target_transform(
                    proxy,
                    WorldTransform::new(BoxVec3::new(x, 0.9, 0.0).into(), BoxQuat::IDENTITY),
                    1.0 / 60.0,
                    true,
                )
                .unwrap();
            world.step(1.0 / 60.0, 4);
        }
        assert!(world.body_position(first).x > 0.1);
        assert!(world.body_position(second).x > second_before);

        let synced = world.body_transform(first);
        let mut bevy_transform = Transform::from_scale(Vec3::splat(1.25));
        bevy_transform.translation = Vec3::new(synced.p.x, synced.p.y, synced.p.z);
        bevy_transform.rotation = from_box_quat(synced.q);
        assert_eq!(bevy_transform.scale, Vec3::splat(1.25));
        assert!(bevy_transform.translation.x > 0.1);

        world.try_destroy_body(first).unwrap();
        assert!(world.try_body_transform(first).is_err());
    }

    #[test]
    fn boxddd_plugin_initializes_native_context() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            AssetPlugin::default(),
            MeshPlugin,
            BoxdddPhysicsPlugin::default(),
        ))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
            1.0 / 60.0,
        )));
        app.update();
        assert!(app.world().get_non_send::<BoxdddPhysicsContext>().is_some());
    }

    #[test]
    fn forward_air_control_preserves_jump_momentum() {
        let velocity = apply_air_control(Vec3::new(PLAYER_SPEED, 0.0, 0.0), Vec3::X, 1.0 / 60.0);
        assert!((velocity.x - PLAYER_SPEED).abs() < f32::EPSILON);
        assert!(velocity.z.abs() < f32::EPSILON);
    }
}
