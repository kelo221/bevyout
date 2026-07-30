use super::*;

#[test]
fn matching_viewmodel_asset_is_retained() {
    assert!(should_retain_viewmodel(
        Some("assets/pistol.glb"),
        Some("assets/pistol.glb"),
        true,
        true,
    ));
}

#[test]
fn changed_viewmodel_asset_requests_respawn() {
    assert!(!should_retain_viewmodel(
        Some("assets/laser.glb"),
        Some("assets/pistol.glb"),
        true,
        true,
    ));
}

#[test]
fn missing_desired_asset_does_not_retain_presentation() {
    assert!(!should_retain_viewmodel(
        None,
        Some("assets/pistol.glb"),
        true,
        true,
    ));
}

#[test]
fn recoil_and_reload_are_distinct_camera_local_poses() {
    let idle = action_transform(WeaponAction::Idle, 0.5);
    let recoil = action_transform(WeaponAction::Firing, 0.5);
    let reload = action_transform(WeaponAction::Reloading, 0.5);
    assert_ne!(recoil.translation, idle.translation);
    assert_ne!(reload.translation, recoil.translation);
    assert!(reload.translation.y < idle.translation.y);
}

#[test]
fn idle_transform_uses_left_handed_weapon_orientation() {
    let expected = Quat::from_rotation_z(-0.5 * PI) * Quat::from_rotation_y(0.5 * PI);
    assert!(idle_transform().rotation.abs_diff_eq(expected, 1e-6));
}

#[test]
fn viewmodel_global_transform_follows_interpolated_camera() {
    let camera = GlobalTransform::from(Transform::from_translation(Vec3::new(2.0, 3.0, 4.0)));
    let local = Transform::from_translation(Vec3::new(0.25, -0.2, -0.6));
    let global = compose_global_transform(camera, local);
    assert!(
        global
            .translation()
            .abs_diff_eq(Vec3::new(2.25, 2.8, 3.4), 1e-6)
    );
}
