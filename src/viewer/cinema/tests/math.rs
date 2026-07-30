use super::*;

fn close(a: Vec3, b: Vec3) -> bool {
    (a - b).length() < 1e-4
}

#[test]
fn horizontal_forward_flattens_and_normalizes() {
    assert!(close(
        horizontal_forward(Vec3::new(0.0, -5.0, -2.0)),
        Vec3::NEG_Z
    ));
    assert!(close(horizontal_forward(Vec3::new(3.0, 9.0, 0.0)), Vec3::X));
}

#[test]
fn horizontal_forward_falls_back_when_looking_straight_down() {
    assert!(close(
        horizontal_forward(Vec3::new(0.0, -1.0, 0.0)),
        Vec3::NEG_Z
    ));
}

#[test]
fn follow_sits_behind_and_above_the_target() {
    // Target at origin facing -Z; camera should be +Z behind, +Y above.
    let pose = follow_pose(Vec3::ZERO, Vec3::NEG_Z, 4.0, 2.0);
    assert!(close(pose.eye, Vec3::new(0.0, 2.0, 4.0)));
    assert!(close(pose.focus, Vec3::ZERO));
}

#[test]
fn follow_tracks_a_moved_target() {
    let pose = follow_pose(Vec3::new(10.0, 0.0, 5.0), Vec3::X, 3.0, 1.0);
    // Facing +X, so "behind" is -X.
    assert!(close(pose.eye, Vec3::new(7.0, 1.0, 5.0)));
    assert!(close(pose.focus, Vec3::new(10.0, 0.0, 5.0)));
}

#[test]
fn orbit_places_camera_on_the_circle() {
    let center = Vec3::new(1.0, 0.0, 2.0);
    let zero = orbit_pose(center, 5.0, 0.0, 1.0);
    assert!(close(zero.eye, Vec3::new(6.0, 1.0, 2.0)));
    assert!(close(zero.focus, center));

    let quarter = orbit_pose(center, 5.0, std::f32::consts::FRAC_PI_2, 1.0);
    assert!(close(quarter.eye, Vec3::new(1.0, 1.0, 7.0)));
}

#[test]
fn smoothstep_is_clamped_and_symmetric() {
    assert_eq!(smoothstep(-1.0), 0.0);
    assert_eq!(smoothstep(0.0), 0.0);
    assert_eq!(smoothstep(1.0), 1.0);
    assert_eq!(smoothstep(2.0), 1.0);
    assert!((smoothstep(0.5) - 0.5).abs() < 1e-6);
    // Ease: slower than linear near the start.
    assert!(smoothstep(0.25) < 0.25);
}

#[test]
fn polyline_interpolates_uniformly_by_segment() {
    let points = [
        Vec3::ZERO,
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 10.0),
    ];
    assert!(close(sample_polyline(&points, 0.0), points[0]));
    assert!(close(sample_polyline(&points, 1.0), points[2]));
    // Halfway is the end of the first of two equal segments.
    assert!(close(
        sample_polyline(&points, 0.5),
        Vec3::new(10.0, 0.0, 0.0)
    ));
    // Quarter through: midpoint of the first segment.
    assert!(close(
        sample_polyline(&points, 0.25),
        Vec3::new(5.0, 0.0, 0.0)
    ));
}

#[test]
fn polyline_handles_empty_and_single_point() {
    assert!(close(sample_polyline(&[], 0.5), Vec3::ZERO));
    let one = [Vec3::new(2.0, 3.0, 4.0)];
    assert!(close(sample_polyline(&one, 0.7), one[0]));
}

#[test]
fn path_eases_and_clamps_over_time() {
    let points = [Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0)];
    assert!(close(sample_path(&points, 0.0, 4.0), Vec3::ZERO));
    assert!(close(
        sample_path(&points, 4.0, 4.0),
        Vec3::new(10.0, 0.0, 0.0)
    ));
    // Past the end clamps to the final waypoint.
    assert!(close(
        sample_path(&points, 99.0, 4.0),
        Vec3::new(10.0, 0.0, 0.0)
    ));
    // Non-positive duration snaps to the end.
    assert!(close(
        sample_path(&points, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 0.0)
    ));
    // Ease in: at half the duration the camera is exactly halfway
    // (smoothstep(0.5) == 0.5), but a quarter in it lags a linear dolly.
    assert!(close(
        sample_path(&points, 2.0, 4.0),
        Vec3::new(5.0, 0.0, 0.0)
    ));
    assert!(sample_path(&points, 1.0, 4.0).x < 2.5);
}

#[test]
fn tangent_points_along_direction_of_travel() {
    let points = [Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0)];
    let tangent = path_tangent(&points, 2.0, 4.0).unwrap();
    assert!(close(tangent, Vec3::X));
    assert!(path_tangent(&[Vec3::ZERO], 0.0, 1.0).is_none());
}
