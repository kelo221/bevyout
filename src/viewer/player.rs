use avian3d::prelude::*;
use bevy::color::LinearRgba;
use bevy::gltf::{GltfExtras, GltfMeshName};
use bevy::input::mouse::MouseMotion;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy_tnua::builtins::{
    TnuaBuiltinJump, TnuaBuiltinJumpConfig, TnuaBuiltinWalk, TnuaBuiltinWalkConfig,
};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::prelude::*;
use serde::Deserialize;

use super::FlyCamera;
use super::audio::{PlayFootstep, PlayLanding};
use super::openmw_player::{
    DIRECTIONAL_JUMP_HEIGHT, DIRECTIONAL_JUMP_HORIZONTAL_DISTANCE, GRAVITY, LocomotionState,
    STATIONARY_JUMP_HEIGHT, air_control_motion, jump_profile,
};

pub(crate) const CAPSULE_RADIUS: f32 = 0.35;
pub(crate) const CAPSULE_HEIGHT: f32 = 1.8;
pub(crate) const EYE_HEIGHT: f32 = 1.6;
const CAMERA_LOCAL_HEIGHT: f32 = EYE_HEIGHT - CAPSULE_HEIGHT * 0.5;
const PLAYER_SPEED: f32 = 4.5;
const MOUSE_SENSITIVITY: f32 = 0.002;
const FOOTSTEP_DISTANCE: f32 = 1.45;

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
pub(crate) enum ControlScheme {
    Jump(TnuaBuiltinJump),
    DirectionalJump(TnuaBuiltinJump),
}

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

type FpsCameraQuery<'w> = (&'w mut Transform, &'w mut FlyCamera);
type ToggleCameraQuery<'w> = (Entity, &'w mut Transform, &'w mut FlyCamera, Has<ChildOf>);
type LocomotionQuery<'w> = (
    Entity,
    &'w Transform,
    &'w LinearVelocity,
    &'w TnuaController<ControlScheme>,
    &'w mut LocomotionState,
    &'w mut FootstepState,
);
type StaticCollisionQuery<'w> = (
    Entity,
    &'w Mesh3d,
    Option<&'w GltfMeshName>,
    Option<&'w MeshMaterial3d<StandardMaterial>>,
    Option<&'w GltfExtras>,
    Option<&'w ChildOf>,
    Option<&'w SceneColliderProcessed>,
);

pub(crate) fn install(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        TnuaControllerPlugin::<ControlScheme>::new(FixedUpdate),
        TnuaAvian3dPlugin::new(FixedUpdate),
    ))
    .insert_resource(Gravity(Vec3::new(0.0, -GRAVITY, 0.0)))
    .insert_resource(CameraModeState::default())
    .insert_resource(StaticCollisionStats::default())
    .add_systems(Update, build_static_colliders.before(toggle_camera_mode))
    .add_systems(
        FixedUpdate,
        apply_player_controls.in_set(TnuaUserControlsSystems),
    )
    .add_systems(Update, emit_landing_events.after(build_static_colliders))
    .add_systems(Update, emit_footsteps.after(emit_landing_events));
}

pub(crate) fn toggle_camera_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut state: ResMut<CameraModeState>,
    mut control_scheme_configs: ResMut<Assets<ControlSchemeConfig>>,
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
                    Transform::from_translation(player_center)
                        .with_rotation(Quat::from_rotation_y(yaw)),
                    RigidBody::Dynamic,
                    Collider::capsule(CAPSULE_RADIUS, CAPSULE_HEIGHT - CAPSULE_RADIUS * 2.0),
                    TnuaController::<ControlScheme>::default(),
                    TnuaConfig::<ControlScheme>(control_scheme_configs.add(ControlSchemeConfig {
                        basis: TnuaBuiltinWalkConfig {
                            speed: PLAYER_SPEED,
                            float_height: CAPSULE_HEIGHT * 0.5 + 0.02,
                            max_slope: 45.0_f32.to_radians(),
                            ..default()
                        },
                        jump: openmw_jump_config(STATIONARY_JUMP_HEIGHT, 0.0),
                        directional_jump: openmw_jump_config(
                            DIRECTIONAL_JUMP_HEIGHT,
                            DIRECTIONAL_JUMP_HORIZONTAL_DISTANCE,
                        ),
                    })),
                    TnuaAvian3dSensorShape(Collider::cylinder(CAPSULE_RADIUS * 0.96, 0.0)),
                    LockedAxes::ROTATION_LOCKED,
                ))
                .id();

            // EYE_HEIGHT is measured from the ground; the camera is parented
            // to the capsule center, so use the eye-to-center offset locally.
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
    mut players: Query<(
        &FpsPlayer,
        &mut TnuaController<ControlScheme>,
        &mut LocomotionState,
    )>,
) {
    let Ok((player, mut controller, mut locomotion)) = players.single_mut() else {
        return;
    };
    controller.initiate_action_feeding();
    let jump_pressed = keys.pressed(KeyCode::Space);
    let jump_started = jump_pressed && !locomotion.jump_was_pressed();
    locomotion.set_jump_pressed(jump_pressed);
    if state.mode != CameraMode::Fps {
        controller.basis = TnuaBuiltinWalk {
            desired_motion: Vec3::ZERO,
            ..default()
        };
        return;
    }

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
    let airborne = controller.is_airborne().unwrap_or(false);
    controller.basis = TnuaBuiltinWalk {
        desired_motion: air_control_motion(world_input, airborne),
        ..default()
    };
    if jump_started && !airborne {
        let (_height, direction) = jump_profile(world_input);
        locomotion.mark_jump_started();
        if let Some(direction) = direction {
            controller.action(ControlScheme::DirectionalJump(TnuaBuiltinJump {
                horizontal_displacement: Some(direction),
                ..default()
            }));
        } else {
            controller.action(ControlScheme::Jump(TnuaBuiltinJump::default()));
        }
    }
}

fn openmw_jump_config(height: f32, horizontal_distance: f32) -> TnuaBuiltinJumpConfig {
    TnuaBuiltinJumpConfig {
        height,
        upslope_extra_gravity: 0.0,
        takeoff_extra_gravity: 0.0,
        takeoff_above_velocity: f32::INFINITY,
        fall_extra_gravity: 0.0,
        shorten_extra_gravity: 0.0,
        peak_prevention_at_upward_velocity: 0.0,
        peak_prevention_extra_gravity: 0.0,
        reschedule_cooldown: None,
        input_buffer_time: 0.0,
        horizontal_distance,
        disable_force_forward_after_peak: true,
    }
}

fn build_static_colliders(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    mut state: ResMut<CameraModeState>,
    mut stats: ResMut<StaticCollisionStats>,
    query: Query<StaticCollisionQuery<'_>>,
    parents: Query<&GltfExtras>,
) {
    for (entity, mesh_handle, name, material_handle, extras, child_of, processed) in &query {
        if processed.is_some() {
            continue;
        }

        let name = name.map(|name| name.0.as_str()).unwrap_or("<unnamed>");
        let extras = extras
            .or_else(|| child_of.and_then(|child| parents.get(child.0).ok()))
            .and_then(parse_collision_extras);
        if let Some(extras) = extras.as_ref().filter(|extras| extras.bevyout_collision) {
            let Some(mesh) = meshes.get(&mesh_handle.0) else {
                continue;
            };
            let Some(collider) = Collider::trimesh_from_mesh(mesh) else {
                commands.entity(entity).insert(SceneColliderProcessed);
                stats.processed += 1;
                stats.skipped += 1;
                continue;
            };
            let triangles = mesh.indices().map(|indices| indices.len() / 3).unwrap_or(0);
            commands.entity(entity).insert((
                RigidBody::Static,
                Sensor,
                collider,
                FootstepSurface(extras.bevyout_havok_material),
                Visibility::Hidden,
                SceneColliderProcessed,
            ));
            stats.processed += 1;
            stats.built += 1;
            stats.triangles += triangles;
            continue;
        }
        if is_non_collidable_name(name) {
            commands.entity(entity).insert(SceneColliderProcessed);
            stats.processed += 1;
            stats.skipped += 1;
            continue;
        }

        let Some(material_handle) = material_handle else {
            commands.entity(entity).insert(SceneColliderProcessed);
            stats.processed += 1;
            stats.skipped += 1;
            continue;
        };
        let Some(material) = materials.get(&material_handle.0) else {
            continue;
        };
        if !is_collidable_material(material) {
            commands.entity(entity).insert(SceneColliderProcessed);
            stats.processed += 1;
            stats.skipped += 1;
            continue;
        }

        let Some(mesh) = meshes.get(&mesh_handle.0) else {
            continue;
        };
        let Some(collider) = Collider::trimesh_from_mesh(mesh) else {
            commands.entity(entity).insert(SceneColliderProcessed);
            stats.processed += 1;
            stats.skipped += 1;
            continue;
        };
        let triangles = mesh.indices().map(|indices| indices.len() / 3).unwrap_or(0);
        commands.entity(entity).insert((
            RigidBody::Static,
            collider,
            FootstepSurface(None),
            SceneColliderProcessed,
        ));
        stats.processed += 1;
        stats.built += 1;
        stats.triangles += triangles;
    }

    state.collisions_ready = stats.built > 0;
    if stats.processed > stats.last_reported_processed {
        info!(
            "scene collision build: {} colliders, {} triangles, {} skipped",
            stats.built, stats.triangles, stats.skipped
        );
        stats.last_reported_processed = stats.processed;
    }
    if stats.processed > 0 && stats.built == 0 && !stats.no_geometry_reported {
        warn!(
            "scene collision build produced no usable static geometry; FPS mode remains unavailable (processed {}, skipped {})",
            stats.processed, stats.skipped
        );
        stats.no_geometry_reported = true;
    }
}

fn parse_collision_extras(extras: &GltfExtras) -> Option<CollisionExtras> {
    serde_json::from_str::<CollisionExtras>(&extras.value).ok()
}

fn emit_landing_events(
    state: Res<CameraModeState>,
    spatial_query: SpatialQuery,
    surfaces: Query<&FootstepSurface>,
    mut landings: MessageWriter<PlayLanding>,
    mut players: Query<LocomotionQuery<'_>, With<FpsPlayer>>,
) {
    let Ok((entity, transform, velocity, controller, mut locomotion, mut footstep)) =
        players.single_mut()
    else {
        return;
    };
    if state.mode != CameraMode::Fps {
        locomotion.reset(transform.translation);
        footstep.initialized = false;
        footstep.distance = 0.0;
        return;
    }

    let controller_airborne = controller.is_airborne().unwrap_or(false);
    let surface = probe_surface(entity, transform.translation, &spatial_query, &surfaces);
    // Tnua can briefly report grounded when a jump is stopped by a ceiling. Do
    // not consume the jump until the ground probe also confirms a real landing.
    let airborne = controller_airborne || surface.is_none();
    let Some(impact) = locomotion.update(transform.translation, velocity.0.y, airborne) else {
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
    spatial_query: SpatialQuery,
    surfaces: Query<&FootstepSurface>,
    mut footsteps: MessageWriter<PlayFootstep>,
    mut players: Query<(Entity, &Transform, &LinearVelocity, &mut FootstepState), With<FpsPlayer>>,
) {
    let Ok((entity, transform, velocity, mut footstep)) = players.single_mut() else {
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

    let Some(surface) = probe_surface(entity, position, &spatial_query, &surfaces) else {
        footstep.distance = 0.0;
        return;
    };
    if velocity.0.y.abs() > 2.5 {
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
    entity: Entity,
    position: Vec3,
    spatial_query: &SpatialQuery,
    surfaces: &Query<'_, '_, &FootstepSurface>,
) -> Option<&'static str> {
    let origin = position - Vec3::Y * (CAPSULE_HEIGHT * 0.5 - 0.06);
    let filter = SpatialQueryFilter::default().with_excluded_entities([entity]);
    let hits = spatial_query.ray_hits(origin, Dir3::NEG_Y, 0.24, 16, true, &filter);
    let authored_hit = hits.iter().filter(|hit| {
        surfaces
            .get(hit.entity)
            .is_ok_and(|surface| surface.0.is_some())
    });
    let hit = authored_hit
        .min_by(|left, right| left.distance.total_cmp(&right.distance))
        .or_else(|| {
            hits.iter()
                .min_by(|left, right| left.distance.total_cmp(&right.distance))
        })?;
    Some(
        surfaces
            .get(hit.entity)
            .map_or("concrete", |surface| surface_family(surface.0)),
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
    use avian3d::math::Vector;
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
    fn avian_capsule_falls_and_lands_on_static_floor() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            AssetPlugin::default(),
            MeshPlugin,
            PhysicsPlugins::default(),
        ))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
            1.0 / 60.0,
        )));
        app.finish();
        assert!(
            app.world().contains_resource::<Messages<CollisionStart>>(),
            "Avian collision messages should be initialized"
        );
        let player = app
            .world_mut()
            .spawn((
                RigidBody::Dynamic,
                Collider::capsule(CAPSULE_RADIUS, CAPSULE_HEIGHT - CAPSULE_RADIUS * 2.0),
                Position(Vector::new(0.0, 5.0, 0.0)),
                LockedAxes::ROTATION_LOCKED,
            ))
            .id();
        app.world_mut().spawn((
            RigidBody::Static,
            Collider::cuboid(20.0, 1.0, 20.0),
            Position(Vector::ZERO),
        ));

        for _ in 0..180 {
            app.update();
        }

        let position = app
            .world()
            .get::<Position>(player)
            .expect("the dynamic body has a physics position")
            .0;
        assert!(position.y < 5.0);
        assert!(position.y > 0.5);
    }
}
