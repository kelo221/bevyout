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
fn route_progress_delta_rewards_achieved_motion_along_the_desired_direction() {
    assert_eq!(route_progress_delta([2.0, 0.0], [1.5, 0.0]), 1.5);
}

#[test]
fn route_progress_delta_is_zero_when_nothing_is_desired() {
    // No steering direction to project onto -- real achieved motion
    // (e.g. residual momentum) counts as neither progress nor regress.
    assert_eq!(route_progress_delta([0.0, 0.0], [3.0, 3.0]), 0.0);
}

#[test]
fn route_progress_delta_penalizes_achieved_motion_opposite_the_desired_direction() {
    assert_eq!(route_progress_delta([1.0, 0.0], [-0.5, 0.0]), -0.5);
}

#[test]
fn route_progress_delta_only_counts_the_component_along_the_desired_direction() {
    // Desired straight along +x; achieved motion is purely lateral
    // (+z) -- none of it is progress toward where landmass is steering.
    assert_eq!(route_progress_delta([1.0, 0.0], [0.0, 5.0]), 0.0);
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
fn close_distance_alone_resets_stuck_without_landmass_agreeing() {
    // Pre-existing behaviour: the recomputed horizontal distance being
    // within range is sufficient on its own.
    assert!(arrival_resets_stuck(0.3, 0.5, false));
}

#[test]
fn landmass_reached_alone_resets_stuck_even_far_from_target() {
    // Regression (issue #136 follow-up): after erosion, a raw
    // (un-sampled) goto target's nearest reachable point can sit
    // farther than `reached_distance` from the literal requested
    // coordinate even though landmass has already stopped the agent.
    // `landmass_reached` must reset stuck detection on its own,
    // independent of how far the recomputed distance says it is.
    assert!(arrival_resets_stuck(0.64, 0.5, true));
}

#[test]
fn neither_signal_does_not_reset_stuck() {
    assert!(!arrival_resets_stuck(0.64, 0.5, false));
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

#[test]
fn horizontal_distance_ignores_a_large_vertical_gap() {
    // The exact regression: a capsule-centre agent ~0.9 m above a
    // feet-level point with the same X/Z must read as zero apart.
    assert_eq!(
        horizontal_distance([154.66, 41.10, -108.22], [154.66, 40.20, -108.22]),
        0.0
    );
}

#[test]
fn horizontal_distance_is_plain_pythagoras_on_x_and_z() {
    assert_eq!(horizontal_distance([0.0, 0.0, 0.0], [3.0, 99.0, 4.0]), 5.0);
}

#[test]
fn nav_point_reached_tolerates_the_capsule_centre_vs_feet_offset() {
    // The exact Vault101a (00028579) travel-door-arrival numbers from
    // the regression report: ~0.48 m horizontally away, ~0.9 m
    // vertically (agent capsule centre above the feet-level door
    // midpoint) -- ~1.0 m in 3D, which is why the old 3D `<= 0.75`
    // check missed it.
    let agent = [154.02, 37.47, -36.81];
    let point = [154.13, 36.57, -36.34];
    assert!(nav_point_reached(agent, point, 0.75, 1.8));
}

#[test]
fn nav_point_reached_still_rejects_a_point_too_far_horizontally() {
    let agent = [0.0, 0.0, 0.0];
    let point = [5.0, 0.0, 0.0];
    assert!(!nav_point_reached(agent, point, 0.75, 1.8));
}

#[test]
fn nav_point_reached_rejects_a_point_on_a_different_floor() {
    let agent = [0.0, 0.0, 0.0];
    // Same X/Z, but 3 storeys away vertically -- an unrelated door, not
    // the capsule-centre-vs-feet offset this whole fix tolerates.
    let point = [0.0, 9.0, 0.0];
    assert!(!nav_point_reached(agent, point, 0.75, 1.8));
}
