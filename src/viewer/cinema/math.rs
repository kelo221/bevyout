//! Pure cinema-camera geometry (issue #209).
//!
//! Every function here is `std`/`glam`-only: no Bevy, no ECS, no world
//! access. The Bevy systems in `super` are thin consumers that feed in a
//! target's world position/facing and turn the returned [`CameraPose`] into a
//! `Transform`. Keeping the geometry pure is what lets these be unit-tested
//! directly (the pattern AGENTS.md describes for `world::policy`).

use glam::Vec3;

/// A camera placement: where the camera sits (`eye`) and the point it frames
/// (`focus`). The consumer builds `Transform::from_translation(eye)
/// .looking_at(focus, Vec3::Y)` from this.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CameraPose {
    pub(crate) eye: Vec3,
    pub(crate) focus: Vec3,
}

/// Horizontal (XZ) component of a facing direction, normalized. Falls back to
/// world `-Z` when the input has no horizontal extent (looking straight
/// up/down), so a follow offset is always well-defined.
pub(crate) fn horizontal_forward(forward: Vec3) -> Vec3 {
    let flat = Vec3::new(forward.x, 0.0, forward.z);
    flat.try_normalize().unwrap_or(Vec3::NEG_Z)
}

/// Follow pose: sit `dist` metres behind the target along its horizontal
/// facing and `height` metres above it, framing the target itself. "Behind"
/// is the opposite of the target's forward direction, so a walking actor
/// stays framed from over-the-shoulder.
pub(crate) fn follow_pose(target: Vec3, forward: Vec3, dist: f32, height: f32) -> CameraPose {
    let behind = horizontal_forward(forward);
    CameraPose {
        eye: target - behind * dist + Vec3::Y * height,
        focus: target,
    }
}

/// Orbit pose: place the camera on a circle of `radius` around `center` at
/// angle `angle_rad` (measured in the XZ plane from +X toward +Z) and
/// `height` metres above the centre, always looking at the centre.
pub(crate) fn orbit_pose(center: Vec3, radius: f32, angle_rad: f32, height: f32) -> CameraPose {
    let offset = Vec3::new(angle_rad.cos() * radius, height, angle_rad.sin() * radius);
    CameraPose {
        eye: center + offset,
        focus: center,
    }
}

/// Cubic ease in/out (smoothstep) of a `[0, 1]` fraction. Values outside the
/// range are clamped, so callers can pass raw `elapsed / seconds`.
pub(crate) fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Position along a polyline at fraction `t01` in `[0, 1]`, with each segment
/// given an equal share of the parameter (uniform-by-segment). An empty slice
/// yields the origin; a single point yields that point.
pub(crate) fn sample_polyline(points: &[Vec3], t01: f32) -> Vec3 {
    match points {
        [] => Vec3::ZERO,
        [single] => *single,
        _ => {
            let t = t01.clamp(0.0, 1.0);
            let segments = points.len() - 1;
            let scaled = t * segments as f32;
            let index = (scaled.floor() as usize).min(segments - 1);
            let local = scaled - index as f32;
            points[index].lerp(points[index + 1], local)
        }
    }
}

/// Eased dolly position: how far along the `waypoints` polyline the camera has
/// travelled after `elapsed` of a `seconds`-long move, with ease in/out. A
/// non-positive `seconds` snaps straight to the final waypoint.
pub(crate) fn sample_path(waypoints: &[Vec3], elapsed: f32, seconds: f32) -> Vec3 {
    let raw = if seconds > 0.0 {
        (elapsed / seconds).clamp(0.0, 1.0)
    } else {
        1.0
    };
    sample_polyline(waypoints, smoothstep(raw))
}

/// Direction of travel along the path at the current time, for aiming the
/// camera when it is not following a target. Sampled as the difference between
/// the current position and a point a small step earlier, so it points the way
/// the dolly is moving. Returns `None` when the move has no meaningful
/// tangent (empty/degenerate path or a stationary end).
pub(crate) fn path_tangent(waypoints: &[Vec3], elapsed: f32, seconds: f32) -> Option<Vec3> {
    if waypoints.len() < 2 {
        return None;
    }
    let ahead = sample_path(waypoints, elapsed + seconds.max(0.001) * 0.02, seconds);
    let behind = sample_path(waypoints, elapsed - seconds.max(0.001) * 0.02, seconds);
    (ahead - behind).try_normalize()
}

#[cfg(test)]
mod tests {
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
}
