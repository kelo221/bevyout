//! Deterministic camera-relative placement policy for player-dropped items.

pub(crate) const DROP_DISTANCE_METERS: f32 = 1.0;
pub(crate) const DROP_RETREAT_STEP_METERS: f32 = 0.1;
pub(crate) const DROP_MIN_DISTANCE_METERS: f32 = 0.1;
pub(crate) const DROP_COLLISION_CLEARANCE_METERS: f32 = 0.05;
pub(crate) const DROP_FALLBACK_CLEARANCE_METERS: f32 = 0.15;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DropPlacementMode {
    Camera,
    Retreat,
    PlayerFallback,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DropPlacementDecision {
    pub(crate) mode: DropPlacementMode,
    pub(crate) distance: Option<f32>,
}

impl DropPlacementDecision {
    fn camera(distance: f32) -> Self {
        Self {
            mode: DropPlacementMode::Camera,
            distance: Some(distance),
        }
    }

    fn retreat(distance: f32) -> Self {
        Self {
            mode: DropPlacementMode::Retreat,
            distance: Some(distance),
        }
    }

    pub(crate) fn fallback() -> Self {
        Self {
            mode: DropPlacementMode::PlayerFallback,
            distance: None,
        }
    }
}

/// Selects the furthest clear camera-relative candidate.
///
/// The callback returns `true` when the candidate is blocked. Treating query
/// failures as blocked makes the runtime path converge on the unconditional
/// player-top fallback instead of rejecting the drop.
pub(crate) fn choose_candidate(mut is_blocked: impl FnMut(f32) -> bool) -> DropPlacementDecision {
    let mut distance = DROP_DISTANCE_METERS;
    let mut first = true;
    loop {
        if !is_blocked(distance) {
            return if first {
                DropPlacementDecision::camera(distance)
            } else {
                DropPlacementDecision::retreat(distance)
            };
        }
        if distance <= DROP_MIN_DISTANCE_METERS {
            return DropPlacementDecision::fallback();
        }
        distance = (distance - DROP_RETREAT_STEP_METERS).max(DROP_MIN_DISTANCE_METERS);
        first = false;
    }
}

#[cfg(test)]
#[path = "tests/drop_policy.rs"]
mod tests;
