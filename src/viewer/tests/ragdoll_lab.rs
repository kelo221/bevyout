use super::*;

use crate::vsa::PreparedPhysicsSource;

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
fn prismatic_limits_preserve_order_and_scale() {
    assert_eq!(
        ordered_limits(Some(-0.2), Some(0.4), 2.0),
        (true, -0.4, 0.8)
    );
    assert_eq!(ordered_limits(Some(0.5), Some(-0.5), 1.0), (true, 0.5, 0.5));
    assert_eq!(ordered_limits(None, None, 1.0), (false, 0.0, 0.0));
}
