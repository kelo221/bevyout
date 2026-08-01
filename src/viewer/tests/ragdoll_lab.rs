use super::*;
use std::time::Duration;

use crate::vsa::PreparedPhysicsSource;
use bevy::time::TimeUpdateStrategy;

fn body(group_id: u32) -> PreparedPhysicsBody {
    PreparedPhysicsBody {
        group_id,
        node: Some(format!("Bip01 Test {group_id}")),
        shapes: vec![PreparedPhysicsShape::Sphere {
            center: [0.0; 3],
            radius: 0.25,
        }],
        ..default()
    }
}

#[test]
fn form_ids_accept_plain_and_prefixed_hex() {
    assert_eq!(parse_form_id("00041606").unwrap(), 0x0004_1606);
    assert_eq!(parse_form_id("0x00041606").unwrap(), 0x0004_1606);
    assert!(parse_form_id("raider").is_err());
}

#[test]
fn actor_sidecar_rejects_incomplete_joint_endpoints() {
    let mut asset = PreparedPhysicsAsset {
        schema_version: 3,
        source: PreparedPhysicsSource::AuthoredHavok,
        bodies: vec![body(1)],
        joints: vec![Default::default()],
    };
    asset.joints[0].body_a = 1;
    asset.joints[0].body_b = 2;
    assert!(
        validate_actor_physics(&asset)
            .unwrap_err()
            .to_string()
            .contains("1-2")
    );
}

#[test]
fn actor_sidecar_rejects_unconstrained_or_disconnected_bodies() {
    let asset = PreparedPhysicsAsset {
        schema_version: 3,
        source: PreparedPhysicsSource::AuthoredHavok,
        bodies: vec![body(1), body(2), body(3)],
        joints: Vec::new(),
    };
    let error = validate_actor_physics(&asset).unwrap_err().to_string();
    assert!(error.contains("disconnected joint graph"));
    assert!(error.contains("2, 3"));
}

#[test]
fn avian_uses_converter_authored_local_z_twist_axis() {
    assert_eq!(avian_twist_axis(), Vec3::Z);
}

#[test]
fn complete_local_joint_frame_keeps_xyzw_orientation() {
    let expected = Quat::from_euler(EulerRot::XYZ, 0.3, -0.4, 0.7).normalize();
    let actual = local_joint_frame(expected.to_array());
    assert!(actual.angle_between(expected) < 1.0e-6);
}

#[test]
fn prepared_lab_body_uses_limp_zero_bounce_tuning() {
    let mut source = body(1);
    source.linear_damping = 0.1;
    source.angular_damping = 0.05;
    source.friction = 0.3;
    source.restitution = 0.9;
    source.linear_velocity = [4.0, 5.0, 6.0];
    source.angular_velocity = [1.0, 2.0, 3.0];
    source.sleep_enabled = false;

    let (prepared, _) = prepared_dynamic_body(&source).unwrap();

    assert_eq!(prepared.linear_velocity, [0.0; 3]);
    assert_eq!(prepared.angular_velocity, [0.0; 3]);
    assert!(prepared.linear_damping >= 0.6);
    assert!(prepared.angular_damping >= 1.0);
    assert!(prepared.friction >= 0.6);
    assert_eq!(prepared.restitution, 0.0);
    assert!(prepared.sleep_enabled);
}

#[test]
fn lab_ragdoll_collides_with_floor_but_not_itself() {
    let floor = lab_floor_collision_layers();
    let ragdoll = lab_ragdoll_collision_layers();

    assert!(floor.interacts_with(ragdoll));
    assert!(ragdoll.interacts_with(floor));
    assert!(!ragdoll.interacts_with(ragdoll));
}

#[test]
fn lab_sleep_threshold_allows_only_settled_contact_drift() {
    let threshold = lab_sleep_threshold();

    assert_eq!(threshold.linear, 0.2);
    assert_eq!(threshold.angular, 0.8);
}

#[test]
fn prismatic_limits_preserve_order_and_scale() {
    assert_eq!(
        ordered_limits(Some(-0.2), Some(0.4), 2.0),
        (true, -0.4, 0.8)
    );
    assert_eq!(ordered_limits(Some(0.5), Some(-0.5), 1.0), (true, 0.5, 0.5));
    assert_eq!(ordered_limits(None, None, 1.0), (false, 0.0, 0.0));
}

fn avian_test_app(gravity: Vec3) -> App {
    let step = Duration::from_secs_f32(1.0 / 60.0);
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        avian::PhysicsPlugins::default(),
        TransformPlugin,
    ));
    app.insert_resource(avian::SubstepCount(8));
    app.insert_resource(avian::Gravity(gravity));
    app.insert_resource(Time::<Fixed>::from_duration(step));
    app.insert_resource(TimeUpdateStrategy::ManualDuration(step));
    app.finish();
    app
}

fn run_steps(app: &mut App, count: usize) {
    for _ in 0..count {
        app.update();
    }
}

#[test]
fn avian_revolute_limb_retains_its_hinge_anchor() {
    let mut app = avian_test_app(Vec3::ZERO);
    let anchor = app
        .world_mut()
        .spawn((
            avian::RigidBody::Static,
            avian::Position(Vec3::new(0.0, 2.0, 0.0)),
        ))
        .id();
    let limb = app
        .world_mut()
        .spawn((
            avian::RigidBody::Dynamic,
            avian::Position(Vec3::new(0.0, 1.0, 0.0)),
            avian::Collider::capsule(0.15, 0.7),
            avian::AngularVelocity(Vec3::new(0.0, 0.0, 2.0)),
        ))
        .id();
    app.world_mut().spawn((
        avian::RevoluteJoint::new(anchor, limb)
            .with_local_anchor1(Vec3::new(0.0, -1.0, 0.0))
            .with_local_anchor2(Vec3::ZERO)
            .with_angle_limits(-0.75, 0.75),
        avian::JointCollisionDisabled,
    ));
    run_steps(&mut app, 120);
    let anchor_position = app.world().get::<avian::Position>(anchor).unwrap().0
        + app.world().get::<avian::Rotation>(anchor).unwrap().0 * Vec3::new(0.0, -1.0, 0.0);
    let limb_position = app.world().get::<avian::Position>(limb).unwrap().0;
    assert!(anchor_position.distance(limb_position) < 0.05);
}

#[test]
fn avian_spherical_shoulder_keeps_anchor_and_local_z_twist() {
    let mut app = avian_test_app(Vec3::ZERO);
    let torso = app
        .world_mut()
        .spawn((
            avian::RigidBody::Static,
            avian::Position(Vec3::new(0.0, 2.0, 0.0)),
        ))
        .id();
    let arm = app
        .world_mut()
        .spawn((
            avian::RigidBody::Dynamic,
            avian::Position(Vec3::new(1.0, 2.0, 0.0)),
            avian::Collider::capsule(0.12, 0.7),
            avian::AngularVelocity(Vec3::new(0.5, 0.0, 1.0)),
        ))
        .id();
    app.world_mut().spawn((
        avian::SphericalJoint::new(torso, arm)
            .with_local_anchor1(Vec3::X)
            .with_local_anchor2(Vec3::ZERO)
            .with_twist_axis(avian_twist_axis())
            .with_swing_limits(-0.8, 0.8)
            .with_twist_limits(-0.4, 0.4),
        avian::JointCollisionDisabled,
    ));
    run_steps(&mut app, 120);
    let torso_anchor = app.world().get::<avian::Position>(torso).unwrap().0 + Vec3::X;
    let arm_anchor = app.world().get::<avian::Position>(arm).unwrap().0;
    assert!(torso_anchor.distance(arm_anchor) < 0.05);
}

#[test]
fn avian_chain_lands_without_separating_or_tunnelling() {
    let mut app = avian_test_app(Vec3::new(0.0, -9.81, 0.0));
    app.world_mut().spawn((
        avian::RigidBody::Static,
        avian::Collider::cuboid(8.0, 0.5, 8.0),
        avian::Position(Vec3::new(0.0, -0.25, 0.0)),
    ));
    let mut bodies = Vec::new();
    for index in 0..3 {
        bodies.push(
            app.world_mut()
                .spawn((
                    avian::RigidBody::Dynamic,
                    avian::Collider::sphere(0.2),
                    avian::Position(Vec3::new(0.0, 2.5 + index as f32 * 0.4, 0.0)),
                    avian::LinearDamping(0.4),
                    avian::AngularDamping(0.4),
                ))
                .id(),
        );
    }
    for pair in bodies.windows(2) {
        app.world_mut().spawn((
            avian::SphericalJoint::new(pair[0], pair[1])
                .with_local_anchor1(Vec3::new(0.0, 0.2, 0.0))
                .with_local_anchor2(Vec3::new(0.0, -0.2, 0.0)),
            avian::JointCollisionDisabled,
        ));
    }
    run_steps(&mut app, 360);
    let positions = bodies
        .iter()
        .map(|body| app.world().get::<avian::Position>(*body).unwrap().0)
        .collect::<Vec<_>>();
    assert!(positions.iter().all(|position| position.y >= 0.15));
    assert!(
        positions
            .windows(2)
            .all(|pair| pair[0].distance(pair[1]) < 0.5)
    );
}

#[test]
fn avian_damped_body_eventually_sleeps() {
    let mut app = avian_test_app(Vec3::new(0.0, -9.81, 0.0));
    app.world_mut().spawn((
        avian::RigidBody::Static,
        avian::Collider::cuboid(8.0, 0.5, 8.0),
        avian::Position(Vec3::new(0.0, -0.25, 0.0)),
    ));
    let body = app
        .world_mut()
        .spawn((
            avian::RigidBody::Dynamic,
            avian::Collider::sphere(0.2),
            avian::Position(Vec3::new(0.0, 1.0, 0.0)),
            avian::LinearDamping(1.0),
            avian::AngularDamping(1.0),
        ))
        .id();
    run_steps(&mut app, 600);
    assert!(app.world().entity(body).contains::<avian::Sleeping>());
}
