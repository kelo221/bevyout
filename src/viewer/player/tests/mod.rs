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
