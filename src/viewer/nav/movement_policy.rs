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

/// Default nav-solve interval (issue #114 added scope, M4 wave 5): solve
/// every fixed tick. `tna solverate [<n>]` is the console knob that changes
/// it at runtime.
pub(crate) const DEFAULT_NAV_SOLVE_INTERVAL: u32 = 1;

/// How many fixed ticks have elapsed since the last tick that actually ran
/// the solve, given the current step counter and the configured `interval`:
/// `0` on a solve tick itself, climbing toward `interval - 1` on the ticks
/// in between before resetting to `0` at the next solve. `interval` is
/// clamped to at least `1` so a misconfigured `0` can never divide by zero
/// or, worse, be read as "never solve at all". The same modulo both
/// `should_solve` and `solve_blend_fraction` need, exposed once so callers
/// needing both cannot compute it two different (and possibly
/// inconsistent) ways.
pub(crate) fn steps_since_solve(step: u64, interval: u32) -> u32 {
    let interval = u64::from(interval.max(1));
    (step % interval) as u32
}

/// Whether `LandmassSystems::Update` (the pathfinding+avoidance solve)
/// should run on fixed-tick `step`, given the configured `interval` (solve
/// on every `interval`-th tick; `1` solves every tick).
///
/// Movement (`apply_agent_physics_movement`) is *not* gated by this
/// decision -- it always runs every fixed tick and blends between the
/// previous and latest solved desired velocities via
/// `solve_blend_fraction` rather than snapping straight to a stale-until-
/// the-next-solve constant; only the (comparatively expensive) path/
/// avoidance solve itself is throttled.
pub(crate) fn should_solve(step: u64, interval: u32) -> bool {
    steps_since_solve(step, interval) == 0
}

/// The lerp fraction `apply_agent_physics_movement` blends an agent's
/// previous and latest solved desired velocities by, given
/// `steps_since_solve` (see above) and the configured solve `interval`.
///
/// At `interval <= 1` every tick solves, so there is nothing to
/// interpolate -- this always returns `1.0` (fully the latest, freshly
/// solved value) regardless of `steps_since_solve`, an exact no-op even
/// though whatever "previous" happens to hold is never consulted.
///
/// At `interval > 1` the fraction is `steps_since_solve / interval`,
/// climbing `0 -> (interval - 1) / interval` across the ticks between two
/// solves. Paired with the caller's own shift-on-solve bookkeeping
/// (`previous <- old latest`, `latest <- new solve`, applied only on a tick
/// that actually solved), this makes the blend *one interval behind*: on
/// the solve tick itself the fraction is `0`, so the newly landed value is
/// not applied immediately -- movement keeps sliding from the previous pair
/// toward it, only fully catching up by the time the *next* solve lands
/// (whose own fraction-`0` tick then applies exactly that value). This
/// trades one interval of extra latency for a genuinely continuous signal
/// with no snap at the solve boundary; extrapolating toward a solve that
/// has not happened yet is not causally possible, so this -- interpolating
/// within the last *completed* interval -- is the option available.
pub(crate) fn solve_blend_fraction(steps_since_solve: u32, interval: u32) -> f32 {
    if interval <= 1 {
        return 1.0;
    }
    (steps_since_solve.min(interval) as f32) / interval as f32
}

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

    #[test]
    fn an_interval_of_one_solves_every_step() {
        assert!(should_solve(1, 1));
        assert!(should_solve(2, 1));
        assert!(should_solve(3, 1));
    }

    #[test]
    fn an_interval_of_two_solves_on_even_steps_only() {
        assert!(!should_solve(1, 2));
        assert!(should_solve(2, 2));
        assert!(!should_solve(3, 2));
        assert!(should_solve(4, 2));
    }

    #[test]
    fn an_interval_of_zero_is_clamped_to_one_and_never_skips() {
        assert!(should_solve(1, 0));
        assert!(should_solve(2, 0));
        assert!(should_solve(3, 0));
    }

    #[test]
    fn steps_since_solve_resets_to_zero_on_a_solve_tick_and_climbs_between() {
        assert_eq!(steps_since_solve(1, 2), 1);
        assert_eq!(steps_since_solve(2, 2), 0);
        assert_eq!(steps_since_solve(3, 2), 1);
        assert_eq!(steps_since_solve(4, 2), 0);
    }

    #[test]
    fn an_interval_of_one_never_interpolates() {
        assert_eq!(solve_blend_fraction(0, 1), 1.0);
        // Even a nonsensical non-zero `steps_since_solve` at interval 1 (it
        // should never occur in practice -- `steps_since_solve(_, 1)` is
        // always `0`) still fully resolves to the latest value.
        assert_eq!(solve_blend_fraction(5, 1), 1.0);
    }

    #[test]
    fn an_interval_of_two_blends_halfway_on_the_in_between_step() {
        assert_eq!(solve_blend_fraction(0, 2), 0.0);
        assert_eq!(solve_blend_fraction(1, 2), 0.5);
    }

    #[test]
    fn a_wider_interval_blends_in_even_fractions() {
        assert_eq!(solve_blend_fraction(0, 4), 0.0);
        assert_eq!(solve_blend_fraction(1, 4), 0.25);
        assert_eq!(solve_blend_fraction(2, 4), 0.5);
        assert_eq!(solve_blend_fraction(3, 4), 0.75);
    }
}
