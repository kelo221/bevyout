use bevy::color::LinearRgba;
use bevy::gltf::{GltfExtras, GltfMeshName};
use bevy::input::mouse::MouseMotion;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, VertexAttributeValues};
use bevy::transform::TransformSystems;
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy_boxddd::boxddd::{
    self, BodyDef, BodyId, BodyType, CollisionPlane, Filter, ShapeDef, ShapeId,
};
use bevy_boxddd::prelude::{BoxdddPhysicsContext, BoxdddPhysicsPlugin, BoxdddPhysicsSettings};
use bevy_boxddd::resources::BoxdddErrorPolicy;
use serde::Deserialize;
use std::collections::HashMap;

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
const WORLD_SOLID: u64 = 1;
const SURFACE_HINT: u64 = 2;
const PLAYER_QUERY: u64 = 4;

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

#[derive(Component)]
struct SceneColliderProcessed;

/// Marker used by render diagnostics for both the player mover and static mesh bridge.
#[derive(Component)]
pub(crate) struct PhysicsCollider;

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct FootstepSurface(pub(crate) Option<u32>);

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

#[derive(Debug, Default, Deserialize)]
struct CollisionExtras {
    #[serde(default)]
    bevyout_collision: bool,
    #[serde(default)]
    bevyout_havok_material: Option<u32>,
}

#[derive(Resource, Default)]
struct StaticCollisionStats {
    processed: usize,
    built: usize,
    skipped: usize,
    triangles: usize,
    last_reported_processed: usize,
    no_geometry_reported: bool,
}

#[derive(Clone, Copy, Debug)]
struct CollisionSurface {
    authored: bool,
    material: Option<u32>,
}

#[derive(Resource, Default)]
struct StaticCollisionWorld {
    body: Option<BodyId>,
    surfaces: HashMap<ShapeId, CollisionSurface>,
}

#[derive(Resource, Clone, Copy, Debug, Default)]
pub(crate) struct PhysicsDisabled(pub(crate) bool);

type FpsCameraQuery<'w> = (&'w mut Transform, &'w mut FlyCamera);
type ToggleCameraQuery<'w> = (Entity, &'w mut Transform, &'w mut FlyCamera, Has<ChildOf>);
type StaticCollisionQuery<'w> = (
    Entity,
    &'w Mesh3d,
    &'w GlobalTransform,
    Option<&'w GltfMeshName>,
    Option<&'w MeshMaterial3d<StandardMaterial>>,
    Option<&'w GltfExtras>,
    Option<&'w ChildOf>,
);

pub(crate) fn install(app: &mut App, disable_physics: bool) {
    app.add_plugins(BoxdddPhysicsPlugin::new(BoxdddPhysicsSettings {
        gravity: Vec3::new(0.0, -GRAVITY, 0.0),
        error_policy: BoxdddErrorPolicy::MessageAndLog,
        ..default()
    }))
    .insert_resource(CameraModeState::default())
    .insert_resource(StaticCollisionStats::default())
    .insert_resource(StaticCollisionWorld::default())
    .insert_resource(PhysicsDisabled(disable_physics))
    .add_systems(
        PostUpdate,
        build_static_colliders.after(TransformSystems::Propagate),
    )
    .add_systems(FixedUpdate, apply_player_controls)
    .add_systems(Update, emit_landing_events.after(build_static_colliders))
    .add_systems(Update, emit_footsteps.after(emit_landing_events));
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

    let Ok((camera_entity, mut camera_transform, mut fly_camera, has_parent)) =
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
            let Ok(player_transform) = players.get(player_entity) else {
                warn!("cannot leave FPS mode: player entity is not available yet");
                return;
            };
            if !has_parent {
                warn!("cannot leave FPS mode: camera is not parented to the player");
                return;
            }

            let world_camera =
                GlobalTransform::from(*player_transform).mul_transform(*camera_transform);
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
        .mask_bits(WORLD_SOLID);
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
fn build_static_colliders(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    physics_disabled: Res<PhysicsDisabled>,
    mut state: ResMut<CameraModeState>,
    mut stats: ResMut<StaticCollisionStats>,
    mut collision_world: ResMut<StaticCollisionWorld>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    query: Query<StaticCollisionQuery<'_>, Without<SceneColliderProcessed>>,
    parents: Query<&GltfExtras>,
) {
    if physics_disabled.0 {
        return;
    }
    let Some(world) = context.world_mut() else {
        return;
    };
    let body = match collision_world.body {
        Some(body) => body,
        None => {
            let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
            collision_world.body = Some(body);
            body
        }
    };

    for (entity, mesh_handle, global_transform, name, material_handle, extras, child_of) in &query {
        let name = name.map(|name| name.0.as_str()).unwrap_or("<unnamed>");
        let extras = extras
            .or_else(|| child_of.and_then(|child| parents.get(child.0).ok()))
            .and_then(parse_collision_extras);
        let authored = extras.as_ref().filter(|extras| extras.bevyout_collision);

        let should_build = if authored.is_some() {
            true
        } else if is_non_collidable_name(name) {
            false
        } else {
            material_handle
                .and_then(|handle| materials.get(&handle.0))
                .is_some_and(is_collidable_material)
        };
        if !should_build {
            commands.entity(entity).insert(SceneColliderProcessed);
            stats.processed += 1;
            stats.skipped += 1;
            continue;
        }

        let Some(mesh) = meshes.get(&mesh_handle.0) else {
            continue;
        };
        let authored = authored.is_some();
        let material = extras
            .as_ref()
            .and_then(|extras| extras.bevyout_havok_material);
        let category = if authored { SURFACE_HINT } else { WORLD_SOLID };
        let sensor = authored;
        let Some((shape_id, triangles)) = create_mesh_shape(
            world,
            body,
            mesh,
            *global_transform,
            category,
            sensor,
            material,
        ) else {
            commands.entity(entity).insert(SceneColliderProcessed);
            stats.processed += 1;
            stats.skipped += 1;
            continue;
        };
        collision_world
            .surfaces
            .insert(shape_id, CollisionSurface { authored, material });
        if authored {
            commands.entity(entity).insert((
                FootstepSurface(material),
                Visibility::Hidden,
                SceneColliderProcessed,
                PhysicsCollider,
            ));
        } else {
            commands.entity(entity).insert((
                FootstepSurface(None),
                SceneColliderProcessed,
                PhysicsCollider,
            ));
        }
        stats.processed += 1;
        stats.built += 1;
        stats.triangles += triangles;
    }

    state.collisions_ready = stats.built > 0;
    if stats.processed > stats.last_reported_processed {
        info!(
            "BoxDDD scene collision build: {} colliders, {} triangles, {} skipped",
            stats.built, stats.triangles, stats.skipped
        );
        stats.last_reported_processed = stats.processed;
    }
    if stats.processed > 0 && stats.built == 0 && !stats.no_geometry_reported {
        warn!(
            "BoxDDD scene collision build produced no usable static geometry; FPS mode remains unavailable (processed {}, skipped {})",
            stats.processed, stats.skipped
        );
        stats.no_geometry_reported = true;
    }
}

fn create_mesh_shape(
    world: &mut boxddd::World,
    body: BodyId,
    mesh: &Mesh,
    transform: GlobalTransform,
    category: u64,
    sensor: bool,
    material: Option<u32>,
) -> Option<(ShapeId, usize)> {
    let positions = match mesh.attribute(Mesh::ATTRIBUTE_POSITION)? {
        VertexAttributeValues::Float32x3(values) => values,
        _ => return None,
    };
    if positions.len() < 3 {
        return None;
    }
    let indices = match mesh.indices() {
        Some(Indices::U16(values)) => values.iter().map(|index| u32::from(*index)).collect(),
        Some(Indices::U32(values)) => values.clone(),
        None => (0..positions.len() as u32).collect(),
    };
    let triangles = indices.len() / 3;
    if triangles == 0 {
        return None;
    }
    let vertices = positions
        .iter()
        .map(|position| {
            let point = transform.transform_point(Vec3::from_array(*position));
            boxddd::Vec3::new(point.x, point.y, point.z)
        })
        .collect::<Vec<_>>();
    let mut triangle_indices = Vec::with_capacity(triangles * 3);
    for triangle in indices.chunks_exact(3) {
        if triangle
            .iter()
            .all(|index| (*index as usize) < vertices.len())
        {
            triangle_indices.extend(triangle.iter().map(|index| *index as i32));
        }
    }
    if triangle_indices.is_empty() {
        return None;
    }
    let triangle_count = triangle_indices.len() / 3;
    let mesh_data = boxddd::MeshData::builder(vertices, triangle_indices)
        .build()
        .ok()?;
    let filter = Filter {
        category_bits: category,
        mask_bits: if sensor {
            PLAYER_QUERY | SURFACE_HINT
        } else {
            PLAYER_QUERY
        },
        group_index: 0,
    };
    let shape_def = ShapeDef::builder()
        .density(0.0)
        .friction(0.8)
        .filter(filter)
        .sensor(sensor)
        .user_material_id(u64::from(material.unwrap_or(0)))
        .build();
    let shape_id = world
        .try_create_mesh_shape(
            body,
            &shape_def,
            mesh_data,
            boxddd::Vec3::new(1.0, 1.0, 1.0),
        )
        .ok()?;
    Some((shape_id, triangle_count))
}

fn parse_collision_extras(extras: &GltfExtras) -> Option<CollisionExtras> {
    serde_json::from_str::<CollisionExtras>(&extras.value).ok()
}

fn emit_landing_events(
    state: Res<CameraModeState>,
    context: NonSend<BoxdddPhysicsContext>,
    collision_world: Res<StaticCollisionWorld>,
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
    collision_world: Res<StaticCollisionWorld>,
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
    collision_world: &StaticCollisionWorld,
) -> Option<&'static str> {
    let world = context.world()?;
    let origin = to_box_vec3(position - Vec3::Y * (CAPSULE_HEIGHT * 0.5 - 0.06));
    let translation = to_box_vec3(Vec3::new(0.0, -0.24, 0.0));
    let filter = boxddd::QueryFilter::new()
        .category_bits(PLAYER_QUERY)
        .mask_bits(WORLD_SOLID | SURFACE_HINT);
    let hits = world.cast_ray(origin, translation, filter).ok()?;
    let authored = hits.iter().filter(|hit| {
        collision_world
            .surfaces
            .get(&hit.shape_id)
            .is_some_and(|surface| surface.authored)
    });
    let hit = authored
        .min_by(|left, right| left.fraction.total_cmp(&right.fraction))
        .or_else(|| {
            hits.iter()
                .min_by(|left, right| left.fraction.total_cmp(&right.fraction))
        })?;
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

fn is_non_collidable_name(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    [
        "shadefade",
        "fxglowsimplefill",
        "editormarker",
        "editor_marker",
    ]
    .iter()
    .any(|excluded| name.contains(excluded))
}

fn is_collidable_material(material: &StandardMaterial) -> bool {
    matches!(material.alpha_mode, AlphaMode::Opaque | AlphaMode::Mask(_))
        && material.emissive == LinearRgba::BLACK
        && material.emissive_texture.is_none()
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
    fn effect_mesh_names_are_not_collidable() {
        assert!(is_non_collidable_name("FXGlowSimpleFill:mesh"));
        assert!(is_non_collidable_name("EditorMarker"));
        assert!(!is_non_collidable_name("WasteRmTallCorner01"));
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
    fn emissive_and_translucent_materials_are_not_collidable() {
        let emissive = StandardMaterial {
            emissive: LinearRgba::WHITE,
            ..default()
        };
        assert!(!is_collidable_material(&emissive));

        let translucent = StandardMaterial {
            alpha_mode: AlphaMode::Blend,
            ..default()
        };
        assert!(!is_collidable_material(&translucent));
        assert!(is_collidable_material(&StandardMaterial::default()));
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
    fn collision_extras_parse_material() {
        let extras = GltfExtras {
            value: r#"{
                "bevyout_collision": true,
                "bevyout_havok_material": 73
            }"#
            .into(),
        };
        let parsed = parse_collision_extras(&extras).unwrap();
        assert!(parsed.bevyout_collision);
        assert_eq!(parsed.bevyout_havok_material, Some(73));
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
