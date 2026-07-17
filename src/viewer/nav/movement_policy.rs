//! Pure std-only movement-policy decisions for the physics-authoritative
//! nav-agent capsule (issue #114, M4 wave 5): grounded/airborne
//! transitions, collision-rejection clamping (desired vs achieved
//! velocity), and stuck detection/recovery. `nav/agent.rs`'s Bevy systems
//! only feed observations sampled from the real `bevy_boxddd` KCC sweep
//! into these functions -- no decision-making lives in the Bevy system
//! itself. Mirrors `src/viewer/world/policy.rs`'s established pure
//! decision-table pattern: `std`-only (no `bevy`/`bevy_landmass` import) so
//! `tests/features.rs` can include it verbatim via `#[path]`.

/// Ticks (fixed-cadence movement observations) without net progress toward
/// the current waypoint before a stuck agent first attempts recovery.
pub(crate) const STUCK_RECOVERY_TICKS: u32 = 60;
/// Further ticks without progress, after recovery starts, before the agent
/// gives up deterministically.
pub(crate) const STUCK_FAILURE_TICKS: u32 = 60;
/// Minimum net progress (metres closer to the waypoint) that counts as
/// "still making progress" and resets the stuck window.
pub(crate) const STUCK_PROGRESS_EPSILON: f32 = 0.05;
/// Achieved/desired horizontal-speed ratio below which a tick counts as
/// collision-rejected rather than merely slowed (e.g. sliding along a
/// wall at reduced but non-trivial speed stays `Clear`).
pub(crate) const COLLISION_BLOCK_RATIO: f32 = 0.15;

/// Observation feeding `decide_grounded`: whether this tick's KCC sweep
/// found a walkable support plane directly, or reached one via a step-up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct GroundedObservation {
    pub(crate) has_walkable_plane: bool,
    pub(crate) stepped_up: bool,
}

/// Whether the agent counts as grounded after this tick's sweep. Mirrors
/// the player KCC's own `stepped_up || has_walkable_plane(..)` rule
/// (`player/movement.rs::apply_player_controls`), factored out here so the
/// rule is independently testable without a `boxddd::World`.
pub(crate) fn decide_grounded(observation: GroundedObservation) -> bool {
    observation.stepped_up || observation.has_walkable_plane
}

/// Observation feeding `decide_collision_outcome`: the horizontal speed
/// landmass's desired velocity asked for this tick, versus the horizontal
/// speed the KCC sweep actually achieved (real post-collision displacement
/// over dt).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct VelocityObservation {
    pub(crate) desired_horizontal_speed: f32,
    pub(crate) achieved_horizontal_speed: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CollisionOutcome {
    Clear,
    Blocked,
}

/// Collision-rejection clamping decision: compares desired against
/// achieved horizontal speed. A desired move that resolves to (near-)zero
/// achieved speed -- a wall square in front of the agent -- is `Blocked`;
/// sliding along a wall at reduced but non-trivial speed, or standing
/// still with no desired motion, is `Clear`. This table only classifies
/// the tick for diagnostics (`nav agent collision-blocked <id>`); the
/// KCC's own achieved delta is always what gets fed back to landmass's
/// `Velocity3d`, regardless of this outcome.
pub(crate) fn decide_collision_outcome(observation: VelocityObservation) -> CollisionOutcome {
    if observation.desired_horizontal_speed <= f32::EPSILON {
        return CollisionOutcome::Clear;
    }
    let ratio = observation.achieved_horizontal_speed / observation.desired_horizontal_speed;
    if ratio < COLLISION_BLOCK_RATIO {
        CollisionOutcome::Blocked
    } else {
        CollisionOutcome::Clear
    }
}

/// Observation feeding `decide_stuck`: current distance to the active
/// waypoint, the best (smallest) distance recorded so far along this
/// route, how many consecutive ticks have passed with no net progress, and
/// whether a recovery attempt (e.g. a forced repath) is already active.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct StuckObservation {
    pub(crate) distance_to_target: f32,
    pub(crate) best_distance_so_far: f32,
    pub(crate) ticks_without_progress: u32,
    pub(crate) recovery_active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StuckDecision {
    /// Net progress observed this tick (or still inside the initial
    /// window): reset tracking, no recovery in flight.
    Progressing,
    /// No progress for `STUCK_RECOVERY_TICKS`: start recovery once (the
    /// caller re-issues the current target so landmass replans).
    StartRecovery,
    /// Recovery already started; still within its own window.
    RecoveryPending,
    /// No progress for `STUCK_RECOVERY_TICKS + STUCK_FAILURE_TICKS` even
    /// after recovery started: deterministic stuck failure.
    Stuck,
}

/// Deterministic stuck detection/recovery decision table: no RNG, no
/// wall-clock time, purely a function of the observed distances and tick
/// counters the caller tracks per agent.
pub(crate) fn decide_stuck(observation: StuckObservation) -> StuckDecision {
    let progressed =
        observation.distance_to_target + STUCK_PROGRESS_EPSILON < observation.best_distance_so_far;
    if progressed {
        return StuckDecision::Progressing;
    }
    if observation.ticks_without_progress < STUCK_RECOVERY_TICKS {
        return StuckDecision::Progressing;
    }
    if !observation.recovery_active {
        return StuckDecision::StartRecovery;
    }
    if observation.ticks_without_progress < STUCK_RECOVERY_TICKS + STUCK_FAILURE_TICKS {
        StuckDecision::RecoveryPending
    } else {
        StuckDecision::Stuck
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grounded_requires_either_a_walkable_plane_or_a_successful_step_up() {
        assert!(!decide_grounded(GroundedObservation::default()));
        assert!(decide_grounded(GroundedObservation {
            has_walkable_plane: true,
            stepped_up: false,
        }));
        assert!(decide_grounded(GroundedObservation {
            has_walkable_plane: false,
            stepped_up: true,
        }));
    }

    #[test]
    fn standing_still_with_no_desired_motion_is_never_blocked() {
        assert_eq!(
            decide_collision_outcome(VelocityObservation {
                desired_horizontal_speed: 0.0,
                achieved_horizontal_speed: 0.0,
            }),
            CollisionOutcome::Clear
        );
    }

    #[test]
    fn a_wall_square_in_front_of_the_agent_is_blocked() {
        assert_eq!(
            decide_collision_outcome(VelocityObservation {
                desired_horizontal_speed: 2.5,
                achieved_horizontal_speed: 0.05,
            }),
            CollisionOutcome::Blocked
        );
    }

    #[test]
    fn sliding_along_a_wall_at_reduced_speed_stays_clear() {
        assert_eq!(
            decide_collision_outcome(VelocityObservation {
                desired_horizontal_speed: 2.5,
                achieved_horizontal_speed: 1.2,
            }),
            CollisionOutcome::Clear
        );
    }

    #[test]
    fn progress_toward_the_waypoint_resets_the_stuck_window() {
        assert_eq!(
            decide_stuck(StuckObservation {
                distance_to_target: 4.0,
                best_distance_so_far: 5.0,
                ticks_without_progress: 200,
                recovery_active: true,
            }),
            StuckDecision::Progressing
        );
    }

    #[test]
    fn no_progress_within_the_recovery_window_keeps_progressing() {
        assert_eq!(
            decide_stuck(StuckObservation {
                distance_to_target: 5.0,
                best_distance_so_far: 5.0,
                ticks_without_progress: STUCK_RECOVERY_TICKS - 1,
                recovery_active: false,
            }),
            StuckDecision::Progressing
        );
    }

    #[test]
    fn exhausting_the_recovery_window_starts_recovery_exactly_once() {
        assert_eq!(
            decide_stuck(StuckObservation {
                distance_to_target: 5.0,
                best_distance_so_far: 5.0,
                ticks_without_progress: STUCK_RECOVERY_TICKS,
                recovery_active: false,
            }),
            StuckDecision::StartRecovery
        );
        // Recovery already active: no repeated StartRecovery.
        assert_eq!(
            decide_stuck(StuckObservation {
                distance_to_target: 5.0,
                best_distance_so_far: 5.0,
                ticks_without_progress: STUCK_RECOVERY_TICKS,
                recovery_active: true,
            }),
            StuckDecision::RecoveryPending
        );
    }

    #[test]
    fn exhausting_the_failure_window_after_recovery_fails_deterministically() {
        assert_eq!(
            decide_stuck(StuckObservation {
                distance_to_target: 5.0,
                best_distance_so_far: 5.0,
                ticks_without_progress: STUCK_RECOVERY_TICKS + STUCK_FAILURE_TICKS - 1,
                recovery_active: true,
            }),
            StuckDecision::RecoveryPending
        );
        assert_eq!(
            decide_stuck(StuckObservation {
                distance_to_target: 5.0,
                best_distance_so_far: 5.0,
                ticks_without_progress: STUCK_RECOVERY_TICKS + STUCK_FAILURE_TICKS,
                recovery_active: true,
            }),
            StuckDecision::Stuck
        );
    }
}
