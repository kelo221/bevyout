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
mod tests {
    use super::*;

    #[test]
    fn clear_path_keeps_the_one_metre_camera_distance() {
        let decision = choose_candidate(|_| false);
        assert_eq!(decision.mode, DropPlacementMode::Camera);
        assert_eq!(decision.distance, Some(1.0));
    }

    #[test]
    fn blocked_candidates_retreat_by_ten_centimetres() {
        let decision = choose_candidate(|distance| distance > 0.75);
        assert_eq!(decision.mode, DropPlacementMode::Retreat);
        assert!((decision.distance.expect("retreat should choose a distance") - 0.7).abs() < 0.001);
    }

    #[test]
    fn an_unresolvable_path_uses_the_player_fallback() {
        let decision = choose_candidate(|_| true);
        assert_eq!(decision.mode, DropPlacementMode::PlayerFallback);
        assert_eq!(decision.distance, None);
    }

    #[test]
    fn query_failure_can_be_represented_as_a_blocked_candidate() {
        let query_failed = true;
        let decision = choose_candidate(|_| query_failed);
        assert_eq!(decision.mode, DropPlacementMode::PlayerFallback);
    }
}
