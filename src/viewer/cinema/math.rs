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
#[path = "tests/math.rs"]
mod tests;
