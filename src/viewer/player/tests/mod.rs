use super::*;
use bevy::ecs::system::RunSystemOnce;
use bevy::mesh::MeshPlugin;
use bevy::time::TimeUpdateStrategy;
use std::time::Duration;

fn camera_transition_world(physics_disabled: bool, collisions_ready: bool) -> World {
    let mut world = World::new();
    world.insert_resource(CameraModeState {
        collision_build_complete: true,
        collisions_ready,
        ..default()
    });
    world.insert_resource(PhysicsDisabled(physics_disabled));
    world.insert_resource(PlayerNoClip::default());
    world.insert_resource(RefRegistry::default());
    world.insert_resource(crate::console::ConsoleSessionStore::default());
    world.spawn((
        Camera3d::default(),
        Transform::from_xyz(2.0, 3.0, 4.0),
        FlyCamera {
            yaw: 0.0,
            pitch: 0.0,
            speed: 8.0,
        },
    ));
    world
}

fn synthetic_collision_placement(
    classification: PreparedPhysicsClassification,
    translation: [f32; 3],
) -> crate::vsa::PreparedPlacement {
    crate::vsa::PreparedPlacement {
        reference_form_id: 1,
        base_form_id: 1,
        asset_path: None,
        translation,
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        error: None,
        physics_asset_path: None,
        physics_source: None,
        physics_classification: classification,
        step_support: false,
        mutability: Default::default(),
        mutability_root_form_id: None,
        reference_kind: "REFR".into(),
        base_kind: "STAT".into(),
        editor_id: None,
        display_name: None,
        count: 1,
        semantic: crate::vsa::PreparedSemantic::Static,
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        linked_reference_form_id: None,
        inventory: Vec::new(),
        audio: Default::default(),
        ao_mode: "ao-none".into(),
    }
}

#[test]
fn fps_transition_round_trips_hierarchy_and_player_reference() {
    let mut world = camera_transition_world(false, true);
    assert_eq!(
        set_camera_mode(&mut world, CameraMode::Fps),
        Ok(CameraMode::Fps)
    );
    let player = crate::console::resolve_reference(&world, "player").unwrap();
    assert_eq!(world.resource::<CameraModeState>().player, Some(player));
    assert!(!world.resource::<PlayerNoClip>().0);
    let session = crate::console::ConsoleSessionId::new("ui");
    world
        .resource_mut::<crate::console::ConsoleSessionStore>()
        .select(session.clone(), player);
    let mut cameras = world.query_filtered::<&ChildOf, With<Camera3d>>();
    assert_eq!(cameras.single(&world).unwrap().parent(), player);

    assert_eq!(
        set_camera_mode(&mut world, CameraMode::Free),
        Ok(CameraMode::Free)
    );
    assert!(!world.entities().contains(player));
    assert!(crate::console::resolve_reference(&world, "player").is_err());
    assert_eq!(
        world
            .resource::<crate::console::ConsoleSessionStore>()
            .selected(&session),
        None
    );
    let mut cameras = world.query_filtered::<Entity, (With<Camera3d>, Without<ChildOf>)>();
    assert!(cameras.single(&world).is_ok());
}

#[test]
fn fps_transition_forces_no_clip_without_usable_physics() {
    for (physics_disabled, collisions_ready) in [(true, false), (false, false)] {
        let mut world = camera_transition_world(physics_disabled, collisions_ready);
        assert_eq!(
            set_camera_mode(&mut world, CameraMode::Fps),
            Ok(CameraMode::Fps)
        );
        assert!(world.resource::<PlayerNoClip>().0);
    }
}

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
fn console_transform_adapter_synchronizes_fps_angles_and_clears_velocity() {
    let mut world = World::new();
    let position = Vec3::new(1.0, 2.0, 3.0);
    let player_entity = world
        .spawn((
            FpsPlayer {
                yaw: 0.0,
                pitch: 0.0,
            },
            KccState {
                velocity: Vec3::splat(5.0),
                grounded: true,
            },
            FootstepState::default(),
            PlayerRenderHistory::new(position),
            Transform::from_translation(position),
        ))
        .id();
    let camera_entity = world
        .spawn((
            Camera3d::default(),
            Transform::default(),
            FlyCamera {
                yaw: 0.0,
                pitch: 0.0,
                speed: 8.0,
            },
            ChildOf(player_entity),
        ))
        .id();

    assert!(console_set_angles(
        &mut world,
        player_entity,
        Vec3::new(30.0, 90.0, 0.0),
    ));
    let angles = console_get_angles(&world, player_entity).unwrap();
    assert!((angles.x - 30.0).abs() < 0.001);
    assert!((angles.y - 90.0).abs() < 0.001);
    let player = world.get::<FpsPlayer>(player_entity).unwrap();
    assert!((player.yaw.to_degrees() - 90.0).abs() < 0.001);
    let camera = world.get::<FlyCamera>(camera_entity).unwrap();
    assert!((camera.pitch.to_degrees() - 30.0).abs() < 0.001);
    let kcc = world.get::<KccState>(player_entity).unwrap();
    assert_eq!(kcc.velocity, Vec3::ZERO);
    assert!(!kcc.grounded);
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
fn grounded_camera_vertical_smoothing_is_monotonic_and_frame_rate_independent() {
    fn simulate(delta_seconds: f32, frames: usize) -> f32 {
        let mut history = PlayerRenderHistory::new(Vec3::ZERO);
        assert_eq!(
            smooth_grounded_camera_y(&mut history, 0.0, true, delta_seconds),
            0.0
        );
        let mut previous = 0.0;
        for _ in 0..frames {
            let smoothed = smooth_grounded_camera_y(&mut history, 0.3, true, delta_seconds);
            assert!(smoothed >= previous);
            assert!(smoothed <= 0.3);
            previous = smoothed;
        }
        previous
    }

    let at_100_hz = simulate(0.01, 12);
    let at_200_hz = simulate(0.005, 24);
    assert!((at_100_hz - at_200_hz).abs() < 0.000_01);
    assert!((at_100_hz - 0.285).abs() < 0.000_1);
}

#[test]
fn camera_vertical_smoothing_resets_for_air_landings_and_discontinuities() {
    let mut history = PlayerRenderHistory::new(Vec3::ZERO);
    smooth_grounded_camera_y(&mut history, 0.0, true, 1.0 / 60.0);

    let stair = smooth_grounded_camera_y(&mut history, 0.3, true, 1.0 / 60.0);
    assert!((0.0..0.3).contains(&stair));

    let airborne = smooth_grounded_camera_y(&mut history, 0.6, false, 1.0 / 60.0);
    assert_eq!(airborne, 0.6);
    let landed = smooth_grounded_camera_y(&mut history, 0.2, true, 1.0 / 60.0);
    assert_eq!(landed, 0.2);

    let continuous = smooth_grounded_camera_y(&mut history, 0.3, true, 1.0 / 60.0);
    assert!((0.2..0.3).contains(&continuous));
    let teleported = smooth_grounded_camera_y(
        &mut history,
        0.3 + STEP_SWEEP_DISTANCE + 0.01,
        true,
        1.0 / 60.0,
    );
    assert_eq!(teleported, 0.3 + STEP_SWEEP_DISTANCE + 0.01);
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
fn collision_geometry_toggle_only_changes_native_debug_collection() {
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
fn prepared_shape_categories_expose_only_static_structural_step_support() {
    assert_eq!(
        prepared_shape_category(false, true, PreparedPhysicsClassification::Static),
        WORLD_STATIC | STEP_SUPPORT
    );
    assert_eq!(
        prepared_shape_category(false, false, PreparedPhysicsClassification::Static),
        WORLD_STATIC
    );
    assert_eq!(
        prepared_shape_category(false, true, PreparedPhysicsClassification::Kinematic),
        WORLD_STATIC
    );
    assert_eq!(
        prepared_shape_category(true, true, PreparedPhysicsClassification::Dynamic),
        WORLD_DYNAMIC
    );
}

#[test]
fn stair_debug_toggle_changes_rejection_logging() {
    let mut settings = StepDebugSettings::default();
    assert!(!settings.enabled);
    flip_step_debug(&mut settings);
    assert!(settings.enabled);
    flip_step_debug(&mut settings);
    assert!(!settings.enabled);
}

#[test]
fn boxddd_capsule_cast_stops_on_static_floor() {
    use bevy_boxddd::boxddd::{BoxHull, Capsule, QueryFilter, Vec3 as BoxVec3, World, WorldDef};

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
fn dynamic_body_created_after_static_phase_stays_supported() {
    use bevy_boxddd::boxddd::{Sphere, Vec3 as BoxVec3, World, WorldDef};

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
            boxddd::Transform::new(
                BoxVec3::new(0.0, -0.25, 0.0),
                bevy_boxddd::boxddd::Quat::IDENTITY,
            ),
        ),
    );

    // This models the direct startup path: publish the completed static phase
    // before the dynamic placement receives gravity, without requiring a
    // simulation step to have occurred first.
    world
        .try_rebuild_static_tree()
        .expect("BoxDDD static tree rebuild");
    let prop = world.create_body(
        BodyDef::builder()
            .body_type(BodyType::Dynamic)
            .position([0.0, 1.5, 0.0])
            .build(),
    );
    world.create_sphere_shape(
        prop,
        &ShapeDef::builder().density(1.0).friction(0.8).build(),
        &Sphere::new(BoxVec3::ZERO, 0.3),
    );
    for _ in 0..180 {
        world.step(1.0 / 60.0, 4);
    }

    let y = world.body_position(prop).y;
    assert!(
        (0.20..0.45).contains(&y),
        "dynamic prop should settle on the static floor, got y={y}"
    );
}

#[test]
fn dynamic_body_created_after_static_mesh_phase_stays_supported() {
    use bevy_boxddd::boxddd::{Filter, Sphere, Vec3 as BoxVec3, World, WorldDef};

    let mut world = World::new(
        WorldDef::builder()
            .gravity(BoxVec3::new(0.0, -GRAVITY, 0.0))
            .build(),
    )
    .expect("BoxDDD world");
    let floor = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    world
        .try_create_mesh_shape(
            floor,
            &ShapeDef::builder()
                .filter(Filter {
                    category_bits: WORLD_STATIC,
                    mask_bits: WORLD_DYNAMIC,
                    group_index: 0,
                })
                .build(),
            bevy_boxddd::boxddd::MeshData::box_mesh(
                BoxVec3::new(0.0, -0.25, 0.0),
                [5.0, 0.25, 5.0],
                true,
            )
            .expect("BoxDDD floor mesh"),
            BoxVec3::new(1.0, 1.0, 1.0),
        )
        .expect("BoxDDD static mesh");
    world
        .try_rebuild_static_tree()
        .expect("BoxDDD static mesh tree rebuild");
    let prop = world.create_body(
        BodyDef::builder()
            .body_type(BodyType::Dynamic)
            .position([0.0, 1.5, 0.0])
            .build(),
    );
    world.create_sphere_shape(
        prop,
        &ShapeDef::builder()
            .density(1.0)
            .friction(0.8)
            .filter(Filter {
                category_bits: WORLD_DYNAMIC,
                mask_bits: WORLD_STATIC,
                group_index: 0,
            })
            .build(),
        &Sphere::new(BoxVec3::ZERO, 0.3),
    );
    for _ in 0..180 {
        world.step(1.0 / 60.0, 4);
    }

    let y = world.body_position(prop).y;
    assert!(
        (0.20..0.45).contains(&y),
        "dynamic prop should settle on the static mesh floor, got y={y}"
    );
}

#[test]
fn prepared_downward_wound_mesh_supports_dynamic_prop() {
    use bevy_boxddd::boxddd::{Vec3 as BoxVec3, World, WorldDef};

    let mut world = World::new(
        WorldDef::builder()
            .gravity(BoxVec3::new(0.0, -GRAVITY, 0.0))
            .build(),
    )
    .expect("BoxDDD world");
    let static_body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    let static_placement =
        synthetic_collision_placement(PreparedPhysicsClassification::Static, [0.0, 0.0, 0.0]);
    let static_body_data = PreparedPhysicsBody::default();
    let downward_floor = PreparedPhysicsShape::TriangleMesh {
        vertices: vec![
            [-5.0, 0.0, -5.0],
            [5.0, 0.0, -5.0],
            [5.0, 0.0, 5.0],
            [-5.0, 0.0, 5.0],
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
    };
    create_prepared_shape(
        &mut world,
        static_body,
        &static_body_data,
        &downward_floor,
        &static_placement,
        PreparedShapeOptions {
            dynamic: false,
            local_space: false,
            collision_group: 0,
        },
    )
    .expect("prepared downward-wound floor");
    world
        .try_rebuild_static_tree()
        .expect("BoxDDD static tree rebuild");

    let dynamic_placement =
        synthetic_collision_placement(PreparedPhysicsClassification::Dynamic, [0.0, 1.5, 0.0]);
    let dynamic_body_data = PreparedPhysicsBody::default();
    let dynamic_body = create_dynamic_body(&mut world, &dynamic_placement, &dynamic_body_data);
    create_prepared_shape(
        &mut world,
        dynamic_body,
        &dynamic_body_data,
        &PreparedPhysicsShape::Sphere {
            center: [0.0, 0.0, 0.0],
            radius: 0.3,
        },
        &dynamic_placement,
        PreparedShapeOptions {
            dynamic: true,
            local_space: true,
            collision_group: 0,
        },
    )
    .expect("prepared dynamic prop");
    for _ in 0..180 {
        world.step(1.0 / 60.0, 4);
    }

    let y = world.body_position(dynamic_body).y;
    assert!(
        (0.20..0.45).contains(&y),
        "dynamic prop should settle on a downward-wound prepared floor, got y={y}"
    );
}

#[test]
fn player_physics_readiness_releases_after_static_phase() {
    assert!(!CellPhysicsReadiness::BuildingStatic.static_collision_ready());
    assert!(CellPhysicsReadiness::BuildingDynamic.static_collision_ready());
    assert!(CellPhysicsReadiness::Ready.static_collision_ready());
}

#[test]
fn dynamic_transform_sync_uses_move_events_and_tracks_sleeping_bodies() {
    use bevy_boxddd::boxddd::{Sphere, Vec3 as BoxVec3, World, WorldDef};

    let mut native_world = World::new(
        WorldDef::builder()
            .gravity(BoxVec3::new(0.0, -GRAVITY, 0.0))
            .build(),
    )
    .expect("BoxDDD world");
    let floor = native_world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    native_world.create_hull_shape(
        floor,
        &ShapeDef::default(),
        &BoxHull::transformed(
            5.0,
            0.25,
            5.0,
            boxddd::Transform::new(BoxVec3::new(0.0, -0.25, 0.0), boxddd::Quat::IDENTITY),
        ),
    );
    let body = native_world.create_body(
        BodyDef::builder()
            .body_type(BodyType::Dynamic)
            .position([0.0, 1.5, 0.0])
            .build(),
    );
    native_world.create_sphere_shape(
        body,
        &ShapeDef::builder().density(1.0).friction(0.8).build(),
        &Sphere::new(BoxVec3::ZERO, 0.3),
    );

    let mut app = App::new();
    let entity = app
        .world_mut()
        .spawn((PhysicsCollider, Transform::from_xyz(0.0, 1.5, 0.0)))
        .id();
    let untouched = app
        .world_mut()
        .spawn((PhysicsCollider, Transform::from_xyz(9.0, 9.0, 9.0)))
        .id();
    let mut collision_world = PreparedCollisionWorld::default();
    collision_world.dynamic_bodies.insert(entity, body);
    collision_world.dynamic_entities.insert(body, entity);
    app.insert_resource(collision_world)
        .insert_resource(CollisionRuntimeStats {
            dynamic_bodies: 1,
            awake_dynamic_bodies: 1,
            ..default()
        })
        .insert_non_send(BoxdddPhysicsContext::from_world(native_world))
        .add_systems(Update, sync_dynamic_transforms);

    for _ in 0..240 {
        app.world_mut()
            .non_send_mut::<BoxdddPhysicsContext>()
            .world_mut()
            .expect("native world")
            .step(1.0 / 60.0, 4);
        app.world_mut().run_schedule(Update);
    }

    let settled = app.world().get::<Transform>(entity).unwrap().translation;
    assert!((0.29..0.31).contains(&settled.y));
    assert_eq!(
        app.world().get::<Transform>(untouched).unwrap().translation,
        Vec3::splat(9.0)
    );
    let stats = app.world().resource::<CollisionRuntimeStats>();
    assert_eq!(stats.awake_dynamic_bodies, 0);
    assert_eq!(stats.sleeping_dynamic_bodies, 1);
    assert_eq!(stats.dynamic_transform_updates, 0);

    app.world_mut()
        .non_send_mut::<BoxdddPhysicsContext>()
        .world_mut()
        .expect("native world")
        .try_apply_linear_impulse_to_center(body, [1.0, 0.0, 0.0], true)
        .unwrap();
    app.world_mut()
        .non_send_mut::<BoxdddPhysicsContext>()
        .world_mut()
        .expect("native world")
        .step(1.0 / 60.0, 4);
    app.world_mut().run_schedule(Update);

    let stats = app.world().resource::<CollisionRuntimeStats>();
    assert_eq!(stats.awake_dynamic_bodies, 1);
    assert_eq!(stats.sleeping_dynamic_bodies, 0);
    assert_eq!(stats.dynamic_transform_updates, 1);
    assert!(
        app.world().get::<Transform>(entity).unwrap().translation.x > settled.x,
        "a waking impulse must resume event-driven transform synchronization"
    );
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

fn fixture_capsule() -> boxddd::Capsule {
    boxddd::Capsule::new(
        [0.0, -(CAPSULE_HEIGHT * 0.5 - CAPSULE_RADIUS), 0.0],
        [0.0, CAPSULE_HEIGHT * 0.5 - CAPSULE_RADIUS, 0.0],
        CAPSULE_RADIUS,
    )
}

fn fixture_shape_def(step_support: bool) -> ShapeDef {
    ShapeDef::builder()
        .filter(Filter {
            category_bits: WORLD_STATIC | if step_support { STEP_SUPPORT } else { 0 },
            mask_bits: PLAYER_QUERY,
            group_index: 0,
        })
        .build()
}

fn add_fixture_box_with_step_support(
    world: &mut boxddd::World,
    center: boxddd::Vec3,
    half_extents: boxddd::Vec3,
    step_support: bool,
) {
    let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    world.create_hull_shape(
        body,
        &fixture_shape_def(step_support),
        &BoxHull::transformed(
            half_extents.x,
            half_extents.y,
            half_extents.z,
            boxddd::Transform::new(center, boxddd::Quat::IDENTITY),
        ),
    );
}

fn add_fixture_box(world: &mut boxddd::World, center: boxddd::Vec3, half_extents: boxddd::Vec3) {
    add_fixture_box_with_step_support(world, center, half_extents, true);
}

fn add_fixture_prop(world: &mut boxddd::World, center: boxddd::Vec3, half_extents: boxddd::Vec3) {
    add_fixture_box_with_step_support(world, center, half_extents, false);
}

fn stair_fixture() -> boxddd::World {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    let half_step = STEP_HEIGHT * 0.5;
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(-1.0, -0.05, 0.0),
        boxddd::Vec3::new(1.0, 0.05, 1.0),
    );
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(0.20, half_step * 0.5, 0.0),
        boxddd::Vec3::new(0.20, half_step * 0.5, 1.0),
    );
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(1.20, STEP_HEIGHT * 0.5, 0.0),
        boxddd::Vec3::new(0.80, STEP_HEIGHT * 0.5, 1.0),
    );
    world
}

fn megaton_shack_stair_fixture() -> boxddd::World {
    const RISER_HEIGHT: f32 = 24.0 / 70.0;
    const TREAD_DEPTH: f32 = 32.0 / 70.0;
    const STEP_COUNT: usize = 7;

    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(-1.0, -0.05, 0.0),
        boxddd::Vec3::new(1.0, 0.05, 1.0),
    );
    for step in 1..=STEP_COUNT {
        let height = step as f32 * RISER_HEIGHT;
        add_fixture_box(
            &mut world,
            boxddd::Vec3::new((step as f32 - 0.5) * TREAD_DEPTH, height * 0.5, 0.0),
            boxddd::Vec3::new(TREAD_DEPTH * 0.5, height * 0.5, 1.0),
        );
    }
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(
            STEP_COUNT as f32 * TREAD_DEPTH + 0.5,
            STEP_COUNT as f32 * RISER_HEIGHT * 0.5,
            0.0,
        ),
        boxddd::Vec3::new(0.5, STEP_COUNT as f32 * RISER_HEIGHT * 0.5, 1.0),
    );
    world
}

fn authored_half_triangle_stair_fixture() -> boxddd::World {
    const RISER_HEIGHT: f32 = 24.0 / 70.0;
    const TREAD_DEPTH: f32 = 32.0 / 70.0;
    const STAIR_WIDTH: f32 = 152.0 / 70.0;
    const STEP_COUNT: usize = 7;

    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    let near_z = -STAIR_WIDTH * 0.5;
    let far_z = STAIR_WIDTH * 0.5;
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(-1.0, -0.05, 0.0),
        boxddd::Vec3::new(1.0, 0.05, STAIR_WIDTH * 0.5),
    );

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut add_triangle = |a: boxddd::Vec3, b: boxddd::Vec3, c: boxddd::Vec3| {
        let first = i32::try_from(vertices.len()).expect("fixture vertex count");
        vertices.extend([a, b, c]);
        indices.extend([first, first + 1, first + 2]);
    };

    for step in 1..=STEP_COUNT {
        let x0 = (step as f32 - 1.0) * TREAD_DEPTH;
        let x1 = step as f32 * TREAD_DEPTH;
        let lower_y = (step as f32 - 1.0) * RISER_HEIGHT;
        let upper_y = step as f32 * RISER_HEIGHT;

        if step % 2 == 1 {
            add_triangle(
                boxddd::Vec3::new(x0, upper_y, far_z),
                boxddd::Vec3::new(x1, upper_y, near_z),
                boxddd::Vec3::new(x0, upper_y, near_z),
            );
        } else {
            add_triangle(
                boxddd::Vec3::new(x1, upper_y, near_z),
                boxddd::Vec3::new(x0, upper_y, near_z),
                boxddd::Vec3::new(x1, upper_y, far_z),
            );
        }

        let lower_near = boxddd::Vec3::new(x0, lower_y, near_z);
        let upper_near = boxddd::Vec3::new(x0, upper_y, near_z);
        let upper_far = boxddd::Vec3::new(x0, upper_y, far_z);
        let lower_far = boxddd::Vec3::new(x0, lower_y, far_z);
        add_triangle(lower_near, upper_far, upper_near);
        add_triangle(lower_near, lower_far, upper_far);
    }

    let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    let mesh = boxddd::MeshData::builder(vertices, indices)
        .build()
        .expect("alternating authored stair mesh");
    world
        .try_create_mesh_shape(
            body,
            &fixture_shape_def(true),
            mesh,
            boxddd::Vec3::new(1.0, 1.0, 1.0),
        )
        .expect("alternating authored stair shape");

    let landing_height = STEP_COUNT as f32 * RISER_HEIGHT;
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(
            STEP_COUNT as f32 * TREAD_DEPTH + 0.5,
            landing_height * 0.5,
            0.0,
        ),
        boxddd::Vec3::new(0.5, landing_height * 0.5, STAIR_WIDTH * 0.5),
    );
    world
}

fn single_step_fixture(height: f32) -> boxddd::World {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(-1.0, -0.05, 0.0),
        boxddd::Vec3::new(1.0, 0.05, 1.0),
    );
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(1.0, height * 0.5, 0.0),
        boxddd::Vec3::new(1.0, height * 0.5, 1.0),
    );
    world
}

fn move_grounded_fixture(
    world: &mut boxddd::World,
    position: boxddd::Vec3,
    delta: boxddd::Vec3,
    allow_step_up: bool,
) -> (boxddd::Vec3, bool) {
    let mover = fixture_capsule();
    let collision_filter = player_collision_filter();
    let support_filter = stair_support_filter();
    let (mut position, planes, stepped_up, _) = move_mover(
        world,
        position,
        &mover,
        delta,
        collision_filter,
        support_filter,
        allow_step_up,
        false,
    );
    let mut grounded = stepped_up || has_walkable_plane(&planes);
    if !grounded {
        if allow_step_up
            && let Some(supported) =
                try_forward_step_support(world, position, delta, support_filter)
        {
            position = supported;
            grounded = true;
        } else if let Some(snapped) = try_step_down(
            world,
            position,
            &mover,
            delta,
            collision_filter,
            support_filter,
        ) {
            position = snapped;
            grounded = true;
        }
    }
    (position, grounded)
}

#[test]
fn grounded_capsule_climbs_and_descends_openmw_height_stairs() {
    let mut world = stair_fixture();
    let mut position = boxddd::Vec3::new(-0.70, CAPSULE_HEIGHT * 0.5, 0.0);
    for tick in 0..28 {
        let (next, grounded) = move_grounded_fixture(
            &mut world,
            position,
            boxddd::Vec3::new(PLAYER_SPEED / 60.0, 0.0, 0.0),
            true,
        );
        assert!(
            grounded,
            "ascending stairs must retain ground support at tick {tick}: ({:.3}, {:.3}, {:.3})",
            next.x, next.y, next.z
        );
        position = next;
    }
    assert!(position.x > 0.8, "the capsule should clear both risers");
    assert!((position.y - (CAPSULE_HEIGHT * 0.5 + STEP_HEIGHT)).abs() < 0.01);

    for tick in 0..28 {
        let (next, grounded) = move_grounded_fixture(
            &mut world,
            position,
            boxddd::Vec3::new(-PLAYER_SPEED / 60.0, 0.0, 0.0),
            true,
        );
        assert!(
            grounded,
            "descending stairs must retain ground support at tick {tick}: ({:.3}, {:.3}, {:.3})",
            next.x, next.y, next.z
        );
        position = next;
    }
    assert!(position.x < -0.4);
    assert!((position.y - CAPSULE_HEIGHT * 0.5).abs() < 0.01);
}

#[test]
fn grounded_capsule_climbs_consecutive_megaton_shack_treads() {
    const RISER_HEIGHT: f32 = 24.0 / 70.0;
    const TREAD_DEPTH: f32 = 32.0 / 70.0;
    const STEP_COUNT: usize = 7;

    let mut world = megaton_shack_stair_fixture();
    let mut position = boxddd::Vec3::new(-0.70, CAPSULE_HEIGHT * 0.5, 0.0);
    for tick in 0..72 {
        let (next, grounded) = move_grounded_fixture(
            &mut world,
            position,
            boxddd::Vec3::new(PLAYER_SPEED / 60.0, 0.0, 0.0),
            true,
        );
        assert!(
            grounded,
            "Megaton stair ascent lost support at tick {tick}: ({:.3}, {:.3}, {:.3})",
            next.x, next.y, next.z
        );
        position = next;
        if position.x > STEP_COUNT as f32 * TREAD_DEPTH + 0.1 {
            break;
        }
    }
    assert!(
        position.x > STEP_COUNT as f32 * TREAD_DEPTH,
        "capsule stalled on a consecutive tread at ({:.3}, {:.3}, {:.3})",
        position.x,
        position.y,
        position.z
    );
    assert!((position.y - (CAPSULE_HEIGHT * 0.5 + STEP_COUNT as f32 * RISER_HEIGHT)).abs() < 0.01);
}

#[test]
fn grounded_capsule_crosses_all_authored_half_triangle_treads() {
    const RISER_HEIGHT: f32 = 24.0 / 70.0;
    const TREAD_DEPTH: f32 = 32.0 / 70.0;
    const STEP_COUNT: usize = 7;

    let mut world = authored_half_triangle_stair_fixture();
    let mut position = boxddd::Vec3::new(-0.70, CAPSULE_HEIGHT * 0.5, -0.02);
    for tick in 0..90 {
        let (next, grounded) = move_grounded_fixture(
            &mut world,
            position,
            boxddd::Vec3::new(PLAYER_SPEED / 60.0, 0.0, 0.0),
            true,
        );
        assert!(
            grounded,
            "authored half-triangle stair lost support at tick {tick}: ({:.3}, {:.3}, {:.3})",
            next.x, next.y, next.z
        );
        position = next;
        if position.x > STEP_COUNT as f32 * TREAD_DEPTH + 0.1 {
            break;
        }
    }
    assert!(
        position.x > STEP_COUNT as f32 * TREAD_DEPTH,
        "capsule stalled on an authored half-triangle tread at ({:.3}, {:.3}, {:.3})",
        position.x,
        position.y,
        position.z
    );
    assert!((position.y - (CAPSULE_HEIGHT * 0.5 + STEP_COUNT as f32 * RISER_HEIGHT)).abs() < 0.01);
}

#[test]
fn nosing_probe_includes_completed_horizontal_step_progress() {
    let position = boxddd::Vec3::new(41.9458, 107.2987, -24.5269);
    let horizontal_progress = boxddd::Vec3::new(0.0699, 0.0, 0.0026);
    let elevated = add_box_vec3(
        position,
        add_box_vec3(
            horizontal_progress,
            boxddd::Vec3::new(0.0, STEP_SWEEP_DISTANCE, 0.0),
        ),
    );
    let direction = scale_box_vec3(
        horizontal_progress,
        1.0 / box_vec_length_squared(horizontal_progress).sqrt(),
    );
    let origin = forward_step_probe_origins(elevated, direction, 106.4)[0].0;

    assert!(origin.x > 42.38);
    assert!(origin.x - position.x > CAPSULE_RADIUS + STEP_CLEARANCE + 0.06);
    assert!(origin.z > position.z);
}

#[test]
fn stair_support_samples_across_missing_half_of_authored_tread() {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    let mesh = boxddd::MeshData::builder(
        vec![
            boxddd::Vec3::new(42.74287, 106.74286, -25.82881),
            boxddd::Vec3::new(42.28572, 106.74286, -25.82881),
            boxddd::Vec3::new(42.74286, 106.74286, -23.65714),
        ],
        vec![0, 1, 2],
    )
    .build()
    .expect("single authored tread triangle");
    world
        .try_create_mesh_shape(
            body,
            &fixture_shape_def(true),
            mesh,
            boxddd::Vec3::new(1.0, 1.0, 1.0),
        )
        .expect("triangle mesh shape");

    let horizontal = boxddd::Vec3::new(0.0700, 0.0, 0.0011);
    let direction = scale_box_vec3(horizontal, 1.0 / box_vec_length_squared(horizontal).sqrt());
    let elevated = boxddd::Vec3::new(42.0158, 107.8045, -24.7846);
    let origins = forward_step_probe_origins(elevated, direction, 106.4);
    let down = boxddd::Vec3::new(0.0, -(STEP_SWEEP_DISTANCE + STEP_CLEARANCE), 0.0);
    let filter = stair_support_filter();

    for (origin, _, _) in &origins[..3] {
        let hits = world
            .cast_ray(*origin, down, filter)
            .expect("near support ray");
        assert!(
            hits.iter().all(|hit| hit.normal.y < WALKABLE_SLOPE_COS),
            "all three near rays must reproduce the logged diagonal hole"
        );
    }

    let (support, total_hits, ray_errors) = probe_walkable_step_support(
        &mut world,
        &origins,
        down,
        106.4 + f32::EPSILON,
        106.4 + STEP_HEIGHT + STEP_VALIDATION_EPSILON,
        filter,
    );
    let (point, forward_offset, _) = support.expect("forward footprint row should find the tread");
    assert_eq!(ray_errors, 0);
    assert!(total_hits > 0);
    assert_eq!(forward_offset, CAPSULE_RADIUS * 0.5);
    assert!((point.y - 106.74286).abs() < 0.0001);

    let current_horizontal = boxddd::Vec3::new(0.0731, 0.0, -0.0005);
    let current_direction = scale_box_vec3(
        current_horizontal,
        1.0 / box_vec_length_squared(current_horizontal).sqrt(),
    );
    let current_center = boxddd::Vec3::new(42.4119, 106.7579, -24.7540);
    let current_origins = step_support_probe_origins(current_center, current_direction);
    let current_probe_down = boxddd::Vec3::new(0.0, -STEP_CLEARANCE * 2.0, 0.0);
    for (origin, _, _) in &current_origins[..3] {
        let hits = world
            .cast_ray(*origin, current_probe_down, filter)
            .expect("current support ray");
        assert!(
            hits.iter().all(|hit| hit.normal.y < WALKABLE_SLOPE_COS),
            "the three near current-support rays must reproduce the latest logged hole"
        );
    }
    let (current_support, _, _) = probe_walkable_step_support(
        &mut world,
        &current_origins,
        current_probe_down,
        106.7379 - STEP_CLEARANCE - STEP_VALIDATION_EPSILON,
        106.7379 + STEP_CLEARANCE + STEP_VALIDATION_EPSILON,
        filter,
    );
    let (current_point, current_forward, _) =
        current_support.expect("footprint must recover current tread support");
    assert_eq!(current_forward, CAPSULE_RADIUS * 0.5);
    assert!((current_point.y - 106.74286).abs() < 0.0001);

    let raised_position = boxddd::Vec3::new(42.0158, 106.74286 + CAPSULE_HEIGHT * 0.5, -24.7846);
    let supported = try_forward_step_support(&mut world, raised_position, horizontal, filter)
        .expect("lateral footprint support should persist after the step correction");
    assert!((supported.y - raised_position.y).abs() < 0.0001);
}

#[test]
fn arlington_last_landing_probe_reaches_past_authored_triangle_gap() {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    let mesh = boxddd::MeshData::builder(
        vec![
            // Current tread beneath the logged player position.
            boxddd::Vec3::new(-12.114_286, 3.6572, 30.857_143),
            boxddd::Vec3::new(-12.571_429, 3.6572, 30.857_143),
            boxddd::Vec3::new(-12.114_286, 3.6572, 32.0),
            // The final riser that blocks the capsule.
            boxddd::Vec3::new(-12.571_429, 4.0, 32.0),
            boxddd::Vec3::new(-12.571_429, 3.6572, 32.0),
            boxddd::Vec3::new(-12.571_429, 4.0, 30.857_143),
            boxddd::Vec3::new(-12.571_429, 3.6572, 32.0),
            boxddd::Vec3::new(-12.571_429, 4.0, 30.857_143),
            boxddd::Vec3::new(-12.571_429, 3.6572, 30.857_143),
            // Only the upward-wound half of the authored top landing.
            boxddd::Vec3::new(-13.714_286, 4.0, 32.0),
            boxddd::Vec3::new(-12.571_429, 4.0, 32.0),
            boxddd::Vec3::new(-13.714_286, 4.0, 30.857_143),
        ],
        (0..12).collect::<Vec<i32>>(),
    )
    .build()
    .expect("Arlington final-landing triangle");
    world
        .try_create_mesh_shape(
            body,
            &fixture_shape_def(true),
            mesh,
            boxddd::Vec3::new(1.0, 1.0, 1.0),
        )
        .expect("Arlington final-landing shape");

    let position = boxddd::Vec3::new(-12.2265, 4.5572, 31.0733);
    let horizontal = boxddd::Vec3::new(-0.0653, 0.0, -0.0073);
    let direction = scale_box_vec3(horizontal, 1.0 / box_vec_length_squared(horizontal).sqrt());
    let elevated = add_box_vec3(
        position,
        add_box_vec3(horizontal, boxddd::Vec3::new(0.0, STEP_SWEEP_DISTANCE, 0.0)),
    );
    let current_ground_y = position.y - CAPSULE_HEIGHT * 0.5;
    let origins = forward_step_probe_origins(elevated, direction, current_ground_y);
    let down = boxddd::Vec3::new(0.0, -(STEP_SWEEP_DISTANCE + STEP_CLEARANCE), 0.0);
    let min_support_y = current_ground_y + f32::EPSILON;
    let max_support_y = current_ground_y + STEP_HEIGHT + STEP_VALIDATION_EPSILON;
    let filter = stair_support_filter();

    let (old_footprint_support, _, _) = probe_walkable_step_support(
        &mut world,
        &origins[..9],
        down,
        min_support_y,
        max_support_y,
        filter,
    );
    assert!(
        old_footprint_support.is_none(),
        "the original three probe rows must reproduce the logged final-step gap"
    );

    let (support, total_hits, ray_errors) = probe_walkable_step_support(
        &mut world,
        &origins,
        down,
        min_support_y,
        max_support_y,
        filter,
    );
    let (point, forward_offset, _) =
        support.expect("the extended landing probe should reach the walkable triangle half");
    assert_eq!(ray_errors, 0);
    assert!(total_hits > 0);
    assert_eq!(forward_offset, CAPSULE_RADIUS * 3.0);
    assert!((point.y - 4.0).abs() < 0.0001);

    let stepped = try_step_up(
        &mut world,
        position,
        &fixture_capsule(),
        horizontal,
        player_collision_filter(),
        filter,
        false,
    )
    .expect("the Arlington-shaped controller fixture should accept the final step");
    assert!((stepped.y - (position.y + 0.3428)).abs() < 0.0001);
}

#[test]
fn step_solver_accepts_exact_limit_and_rejects_higher_risers() {
    let start = boxddd::Vec3::new(-0.70, CAPSULE_HEIGHT * 0.5, 0.0);
    let delta = boxddd::Vec3::new(PLAYER_SPEED / 60.0, 0.0, 0.0);

    let mut exact = single_step_fixture(STEP_HEIGHT);
    let mut position = start;
    let mut reached_tread = false;
    for _ in 0..20 {
        let (next, grounded) = move_grounded_fixture(&mut exact, position, delta, true);
        assert!(grounded);
        reached_tread |= next.y > start.y + STEP_HEIGHT * 0.5;
        if reached_tread {
            assert!(
                (next.y - (CAPSULE_HEIGHT * 0.5 + STEP_HEIGHT)).abs() < 0.01,
                "a successful step must not launch above or dip below the tread"
            );
        }
        position = next;
    }
    assert!(
        position.x > 0.0,
        "exact-height step stalled at ({:.3}, {:.3}, {:.3})",
        position.x,
        position.y,
        position.z
    );
    assert!((position.y - (CAPSULE_HEIGHT * 0.5 + STEP_HEIGHT)).abs() < 0.01);

    let mut too_high = single_step_fixture(STEP_HEIGHT + 0.01);
    position = start;
    for _ in 0..20 {
        let (next, grounded) = move_grounded_fixture(&mut too_high, position, delta, true);
        assert!(grounded);
        position = next;
    }
    assert!(position.x < 0.0);
    assert!((position.y - CAPSULE_HEIGHT * 0.5).abs() < 0.01);
}

#[test]
fn excluded_props_block_movement_without_triggering_step_up() {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(0.0, -0.05, 0.0),
        boxddd::Vec3::new(2.0, 0.05, 1.0),
    );
    add_fixture_prop(
        &mut world,
        boxddd::Vec3::new(0.30, 0.15, 0.0),
        boxddd::Vec3::new(0.30, 0.15, 0.50),
    );

    let mut position = boxddd::Vec3::new(-0.70, CAPSULE_HEIGHT * 0.5, 0.0);
    let delta = boxddd::Vec3::new(PLAYER_SPEED / 60.0, 0.0, 0.0);
    for tick in 0..24 {
        let (next, grounded) = move_grounded_fixture(&mut world, position, delta, true);
        assert!(grounded, "floor support was lost at tick {tick}");
        position = next;
    }

    assert!(
        position.x < -0.25,
        "the excluded prop must remain a solid blocker: x={:.3}",
        position.x
    );
    assert!((position.y - CAPSULE_HEIGHT * 0.5).abs() < 0.01);
}

#[test]
fn excluded_props_remain_valid_normal_fall_landings() {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(0.0, -0.05, 0.0),
        boxddd::Vec3::new(2.0, 0.05, 1.0),
    );
    add_fixture_prop(
        &mut world,
        boxddd::Vec3::new(0.0, 0.15, 0.0),
        boxddd::Vec3::new(0.40, 0.15, 0.40),
    );

    let mover = fixture_capsule();
    let start = boxddd::Vec3::new(0.0, 2.0, 0.0);
    let (landed, planes, _, _) = move_mover(
        &mut world,
        start,
        &mover,
        boxddd::Vec3::new(0.0, -2.0, 0.0),
        player_collision_filter(),
        stair_support_filter(),
        false,
        false,
    );

    assert!(has_walkable_plane(&planes));
    assert!((landed.y - (CAPSULE_HEIGHT * 0.5 + 0.30)).abs() < 0.01);
}

#[test]
fn ground_adhesion_does_not_snap_to_excluded_props() {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_fixture_prop(
        &mut world,
        boxddd::Vec3::new(0.0, 0.0, 0.0),
        boxddd::Vec3::new(0.50, 0.10, 0.50),
    );
    let mover = fixture_capsule();
    let position = boxddd::Vec3::new(0.0, CAPSULE_HEIGHT * 0.5 + 0.30, 0.0);

    assert!(
        try_step_down(
            &mut world,
            position,
            &mover,
            boxddd::Vec3::ZERO,
            player_collision_filter(),
            stair_support_filter(),
        )
        .is_none()
    );
}

#[test]
fn step_solver_uses_only_the_overhead_needed_by_the_riser() {
    const RISER_HEIGHT: f32 = 24.0 / 70.0;

    let mut world = single_step_fixture(RISER_HEIGHT);
    let ceiling_bottom = CAPSULE_HEIGHT + RISER_HEIGHT + STEP_CLEARANCE + 0.01;
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(0.5, ceiling_bottom + 0.05, 0.0),
        boxddd::Vec3::new(2.0, 0.05, 1.0),
    );
    let mut position = boxddd::Vec3::new(-0.70, CAPSULE_HEIGHT * 0.5, 0.0);
    let delta = boxddd::Vec3::new(PLAYER_SPEED / 60.0, 0.0, 0.0);
    for _ in 0..20 {
        let (next, grounded) = move_grounded_fixture(&mut world, position, delta, true);
        assert!(grounded);
        position = next;
    }
    assert!(
        position.x > 0.0,
        "the actual riser plus clearance fits below the ceiling"
    );
    assert!((position.y - (CAPSULE_HEIGHT * 0.5 + RISER_HEIGHT)).abs() < 0.01);
}

#[test]
fn step_solver_rejects_jumps_tall_obstacles_and_blocked_headroom() {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    let tall_obstacle_height = STEP_HEIGHT + 0.01;
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(0.0, -0.05, 0.0),
        boxddd::Vec3::new(2.0, 0.05, 1.0),
    );
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(0.5, tall_obstacle_height * 0.5, 0.0),
        boxddd::Vec3::new(0.5, tall_obstacle_height * 0.5, 1.0),
    );
    let start = boxddd::Vec3::new(-0.60, CAPSULE_HEIGHT * 0.5, 0.0);
    let (blocked, _) =
        move_grounded_fixture(&mut world, start, boxddd::Vec3::new(0.8, 0.0, 0.0), true);
    assert!(blocked.x < 0.0);
    assert!((blocked.y - start.y).abs() < 0.01);

    let mut stairs = stair_fixture();
    let (jumping, _) =
        move_grounded_fixture(&mut stairs, start, boxddd::Vec3::new(0.8, 0.0, 0.0), false);
    assert!(
        jumping.x < 0.0,
        "airborne movement must not invoke stair stepping"
    );

    add_fixture_box(
        &mut stairs,
        boxddd::Vec3::new(0.0, 1.95, 0.0),
        boxddd::Vec3::new(1.0, 0.05, 1.0),
    );
    let (head_blocked, _) =
        move_grounded_fixture(&mut stairs, start, boxddd::Vec3::new(0.8, 0.0, 0.0), true);
    assert!(head_blocked.x < 0.0);
}

#[test]
fn ground_snap_rejects_drops_over_step_height() {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(0.5, 0.20, 0.0),
        boxddd::Vec3::new(0.5, 0.20, 1.0),
    );
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(-1.0, -0.05, 0.0),
        boxddd::Vec3::new(1.0, 0.05, 1.0),
    );
    let mover = fixture_capsule();
    let collision_filter = player_collision_filter();
    let support_filter = stair_support_filter();
    let position = boxddd::Vec3::new(-0.40, CAPSULE_HEIGHT * 0.5 + STEP_HEIGHT + 0.01, 0.0);
    assert!(
        try_step_down(
            &mut world,
            position,
            &mover,
            boxddd::Vec3::ZERO,
            collision_filter,
            support_filter,
        )
        .is_none()
    );
}

#[test]
fn footstep_state_counts_planar_ground_motion_and_alternates_variants() {
    let mut state = FootstepState::default();
    state.record_motion(Vec3::ZERO, true);
    state.record_motion(Vec3::new(FOOTSTEP_DISTANCE, 0.15, 0.0), true);
    assert_eq!(state.take_step(), Some((false, 0)));
    assert_eq!(state.take_step(), None);

    state.record_motion(Vec3::new(FOOTSTEP_DISTANCE * 2.0, 0.30, 0.0), true);
    assert_eq!(state.take_step(), Some((true, 0)));
    state.record_motion(Vec3::new(FOOTSTEP_DISTANCE * 4.0, 0.30, 0.0), true);
    assert_eq!(state.take_step(), Some((false, 1)));
    assert_eq!(state.take_step(), Some((true, 1)));
}

#[test]
fn airborne_motion_clears_partial_stride_and_missing_surface_defaults_to_concrete() {
    let mut state = FootstepState::default();
    state.record_motion(Vec3::ZERO, true);
    state.record_motion(Vec3::new(FOOTSTEP_DISTANCE * 0.75, 0.0, 0.0), true);
    state.record_motion(Vec3::new(FOOTSTEP_DISTANCE, 0.5, 0.0), false);
    state.record_motion(Vec3::new(FOOTSTEP_DISTANCE * 1.5, 0.0, 0.0), true);
    assert_eq!(state.take_step(), None);
    assert_eq!(footstep_surface_or_default(None), "concrete");
    assert_eq!(footstep_surface_or_default(Some("wood")), "wood");
}

// Issue #64: keyframed door colliders follow their animated node — pin the
// rigid-delta math that re-applies node motion on top of the placement root.
#[test]
fn keyframed_body_target_reapplies_the_node_delta_on_the_root() {
    use bevy::math::Affine3A;
    let root = GlobalTransform::from(Transform::from_xyz(10.0, 0.0, 0.0));
    let rest = Affine3A::from_translation(Vec3::new(0.0, 1.0, 0.0));
    // the node slid up by 2 in root space (a vault door half opening)
    let node = GlobalTransform::from(Transform::from_xyz(10.0, 3.0, 0.0));
    let (translation, rotation) = keyframed_body_target(&root, rest, &node).unwrap();
    assert!((translation - Vec3::new(10.0, 2.0, 0.0)).length() < 1e-5);
    assert!(rotation.angle_between(Quat::IDENTITY) < 1e-5);
}

#[test]
fn keyframed_body_target_with_identity_rest_matches_the_node_pose() {
    use bevy::math::Affine3A;
    let root = GlobalTransform::from(
        Transform::from_xyz(5.0, 0.0, 2.0).with_rotation(Quat::from_rotation_y(0.7)),
    );
    let node = GlobalTransform::from(
        Transform::from_xyz(6.0, 1.0, 2.0).with_rotation(Quat::from_rotation_y(1.3)),
    );
    let (translation, rotation) = keyframed_body_target(&root, Affine3A::IDENTITY, &node).unwrap();
    assert!((translation - Vec3::new(6.0, 1.0, 2.0)).length() < 1e-5);
    assert!(rotation.angle_between(Quat::from_rotation_y(1.3)) < 1e-4);
}

#[test]
fn ragdoll_reset_plans_every_captured_node_local_transform() {
    let mut world = World::new();
    let root = world.spawn_empty().id();
    let upper_body = world.spawn_empty().id();
    let forearm = world.spawn_empty().id();
    let upper_body_rest =
        Transform::from_xyz(0.0, 0.9, 0.0).with_rotation(Quat::from_rotation_z(0.2));
    let forearm_rest =
        Transform::from_xyz(0.35, 0.0, 0.0).with_rotation(Quat::from_rotation_y(-0.4));
    let binding = RagdollNodeBinding {
        root,
        node: "UpperBody".into(),
        node_entities: vec![upper_body, forearm],
        rest_body: None,
        rest_node_globals: Vec::new(),
        rest_node_locals: vec![upper_body_rest, forearm_rest],
    };

    let restores = collect_ragdoll_node_restores([&binding]);

    assert_eq!(restores.len(), 2);
    assert_eq!(restores[0], (upper_body, upper_body_rest));
    assert_eq!(restores[1], (forearm, forearm_rest));
}

#[test]
fn ragdoll_node_matching_bridges_niftools_and_pynifly_bone_spelling() {
    assert!(actor_node_names_match("Bip01 Forearm.L", "Bip01 L Forearm"));
    assert!(actor_node_names_match("Bip01 Foot.R", "Bip01 R Foot"));
    assert!(actor_node_names_match("Bip01 Spine2", "Bip01 Spine2"));
    assert!(!actor_node_names_match(
        "Bip01 Forearm.L",
        "Bip01 R Forearm"
    ));
}

#[test]
fn articulated_ragdoll_bodies_never_drive_the_placement_root() {
    assert!(!dynamic_body_drives_placement_root(true, true));
    assert!(!dynamic_body_drives_placement_root(true, false));
    assert!(dynamic_body_drives_placement_root(false, true));
    assert!(!dynamic_body_drives_placement_root(false, false));
}

#[test]
fn ragdoll_body_recentering_moves_shapes_and_joint_frames_to_limb_space() {
    let mut body = PreparedPhysicsBody {
        center_of_mass: [0.0, 1.0, 0.0],
        shapes: vec![PreparedPhysicsShape::Capsule {
            point1: [0.0, 0.8, 0.0],
            point2: [0.0, 1.2, 0.0],
            radius: 0.1,
        }],
        ..default()
    };

    let anchor = recenter_ragdoll_body(&mut body);

    assert!((anchor - Vec3::Y).length() < 1e-6);
    assert_eq!(body.center_of_mass, [0.0, 0.0, 0.0]);
    let PreparedPhysicsShape::Capsule { point1, point2, .. } = &body.shapes[0] else {
        panic!("expected capsule");
    };
    assert!((Vec3::from_array(*point1) - Vec3::new(0.0, -0.2, 0.0)).length() < 1e-6);
    assert!((Vec3::from_array(*point2) - Vec3::new(0.0, 0.2, 0.0)).length() < 1e-6);
    assert!(
        (ragdoll_joint_local_anchor([0.0, 1.2, 0.0], anchor, 1.0) - Vec3::new(0.0, 0.2, 0.0))
            .length()
            < 1e-6
    );
}

#[test]
fn ragdoll_parts_preserve_non_adjacent_self_collision() {
    let first = ragdoll_collision_group(0x0004_1606);
    let second = ragdoll_collision_group(0x0004_161a);
    assert_eq!(first, 0);
    assert_eq!(second, 0);
}

#[test]
fn ragdoll_body_tuning_is_limp_damped_and_starts_without_artificial_motion() {
    let mut root = PreparedPhysicsBody {
        node: Some("Bip01 NonAccum".into()),
        linear_damping: 0.1,
        angular_damping: 0.05,
        friction: 0.3,
        restitution: 0.8,
        ..default()
    };
    let mut limb = PreparedPhysicsBody {
        node: Some("Bip01 Forearm.L".into()),
        ..root.clone()
    };

    tune_ragdoll_body(&mut root);
    tune_ragdoll_body(&mut limb);

    assert_eq!(root.linear_damping, 0.2);
    assert_eq!(root.angular_damping, 0.25);
    assert_eq!(root.friction, 0.6);
    assert_eq!(root.restitution, 0.0);
    assert_eq!(root.linear_velocity, [0.0; 3]);
    assert_eq!(root.angular_velocity, [0.0; 3]);
    assert!(root.sleep_enabled);
    assert_eq!(limb.linear_velocity, [0.0; 3]);
    assert_eq!(limb.angular_velocity, [0.0; 3]);
}

#[test]
fn ragdoll_child_local_pose_is_resolved_against_the_driven_parent_pose() {
    use bevy::math::Affine3A;
    let parent_world = Affine3A::from_translation(Vec3::new(4.0, 1.0, -2.0));
    let child_world = Affine3A::from_translation(Vec3::new(4.4, 1.2, -2.0));

    let local = ragdoll_local_transform(parent_world, child_world).unwrap();

    assert!((local.translation - Vec3::new(0.4, 0.2, 0.0)).length() < 1e-5);
    assert!(local.rotation.angle_between(Quat::IDENTITY) < 1e-5);
}

#[test]
fn ragdoll_parent_world_resolves_through_undriven_intermediate_bones() {
    use bevy::math::Affine3A;
    use std::collections::HashMap;

    let mut world = World::new();
    let spine = world.spawn_empty().id();
    let clavicle = world.spawn_empty().id();
    let twist = world.spawn_empty().id();
    let upper_arm = world.spawn_empty().id();
    let desired_worlds = HashMap::from([(
        spine,
        Affine3A::from_rotation_translation(Quat::from_rotation_z(0.5), Vec3::new(4.0, 1.0, -2.0)),
    )]);
    let parents = HashMap::from([(clavicle, spine), (twist, clavicle), (upper_arm, twist)]);
    let locals = HashMap::from([
        (clavicle, Transform::from_xyz(0.2, 0.1, 0.0)),
        (twist, Transform::from_xyz(0.3, 0.0, 0.0)),
    ]);

    let resolved =
        ragdoll_resolved_world(twist, &desired_worlds, &parents, &HashMap::new(), &locals).unwrap();
    let expected = desired_worlds[&spine]
        * locals[&clavicle].compute_affine()
        * locals[&twist].compute_affine();

    assert!((resolved.translation - expected.translation).length() < 1e-5);
    let probe = Vec3::new(0.3, 0.4, 0.5);
    assert!((resolved.transform_point3(probe) - expected.transform_point3(probe)).length() < 1e-5);
}

#[test]
fn ragdoll_spawns_from_the_actors_current_runtime_transform() {
    let prepared =
        synthetic_collision_placement(PreparedPhysicsClassification::Static, [1.0, 2.0, 3.0]);
    let runtime = Transform::from_xyz(8.0, 9.0, 10.0)
        .with_rotation(Quat::from_rotation_y(0.75))
        .with_scale(Vec3::splat(1.2));

    let placement = ragdoll_runtime_placement(&prepared, &runtime);

    assert_eq!(placement.translation, [8.0, 9.0, 10.0]);
    assert!(Quat::from_array(placement.rotation_xyzw).angle_between(runtime.rotation) < 1e-5);
    assert!((placement.scale - 1.2).abs() < 1e-6);
}

#[test]
fn diagnostic_lines_stack_below_fps_in_the_top_right() {
    let mut world = World::new();
    world.run_system_once(spawn_step_debug_hud).unwrap();
    world.run_system_once(spawn_collider_debug_hud).unwrap();

    let step = world
        .query_filtered::<&Node, With<StepDebugHud>>()
        .single(&world)
        .unwrap();
    assert_eq!(step.position_type, PositionType::Absolute);
    assert_eq!(step.top, Val::Px(STEP_DEBUG_HUD_TOP_PX));
    assert_eq!(step.right, Val::Px(10.0));
    assert_eq!(step.bottom, Val::Auto);

    let collider = world
        .query_filtered::<&Node, With<ColliderDebugHud>>()
        .single(&world)
        .unwrap();
    assert_eq!(collider.position_type, PositionType::Absolute);
    assert_eq!(collider.top, Val::Px(COLLIDER_DEBUG_HUD_TOP_PX));
    assert_eq!(collider.right, Val::Px(10.0));
    assert_eq!(collider.bottom, Val::Auto);
}

#[test]
fn player_debug_hud_uses_the_top_left_diagnostic_layout() {
    let mut world = World::new();
    world.run_system_once(spawn_player_debug_hud).unwrap();

    let node = world
        .query_filtered::<&Node, With<PlayerDebugHud>>()
        .single(&world)
        .unwrap();
    assert_eq!(node.position_type, PositionType::Absolute);
    assert_eq!(node.left, Val::Px(PLAYER_DEBUG_HUD_LEFT_PX));
    assert_eq!(node.top, Val::Px(PLAYER_DEBUG_HUD_TOP_PX));
    assert_eq!(node.right, Val::Auto);
    assert_eq!(node.bottom, Val::Auto);
}

#[test]
fn player_debug_text_formats_position_and_look_angles() {
    let transform = Transform::from_xyz(12.3456, 6.7894, -4.3215);
    let player = FpsPlayer {
        yaw: 90.0_f32.to_radians(),
        pitch: -12.0_f32.to_radians(),
    };

    assert_eq!(
        player_debug_text(Some((&transform, &player))),
        "PLAYER POS 12.346, 6.789, -4.321\nLOOK YAW 90.0 deg | PITCH -12.0 deg"
    );
}

#[test]
fn player_debug_text_shows_fallback_without_an_fps_player() {
    assert_eq!(
        player_debug_text(None),
        "PLAYER POS --\nLOOK YAW -- | PITCH --"
    );

    let mut world = World::new();
    world.spawn((Text::new("stale"), PlayerDebugHud));
    world.run_system_once(update_player_debug_hud).unwrap();
    let text = world
        .query_filtered::<&Text, With<PlayerDebugHud>>()
        .single(&world)
        .unwrap();
    assert_eq!(text.0, "PLAYER POS --\nLOOK YAW -- | PITCH --");
}
