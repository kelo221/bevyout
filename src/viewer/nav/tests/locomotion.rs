use super::*;

fn moving(speed: f32) -> LocomotionObservation {
    LocomotionObservation {
        achieved_horizontal_speed: speed,
        yaw_rate: 0.0,
    }
}

fn turning(yaw_rate: f32) -> LocomotionObservation {
    LocomotionObservation {
        achieved_horizontal_speed: 0.0,
        yaw_rate,
    }
}

#[test]
fn a_stationary_agent_is_idle() {
    assert_eq!(
        next_locomotion_state(LocomotionState::Idle, moving(0.0)),
        LocomotionState::Idle
    );
}

#[test]
fn the_walk_band_is_asymmetric_on_both_edges() {
    // Inside the band (between exit and enter): an idle agent stays
    // idle, a walking one keeps walking. That asymmetry *is* the
    // hysteresis -- the same input maps to two different outputs.
    let inside = (WALK_EXIT_SPEED + WALK_ENTER_SPEED) * 0.5;
    assert_eq!(
        next_locomotion_state(LocomotionState::Idle, moving(inside)),
        LocomotionState::Idle
    );
    assert_eq!(
        next_locomotion_state(LocomotionState::Walk, moving(inside)),
        LocomotionState::Walk
    );
    // Outside both edges the verdict is unambiguous.
    assert_eq!(
        next_locomotion_state(LocomotionState::Idle, moving(WALK_ENTER_SPEED)),
        LocomotionState::Walk
    );
    assert_eq!(
        next_locomotion_state(LocomotionState::Walk, moving(WALK_EXIT_SPEED - 0.01)),
        LocomotionState::Idle
    );
}

#[test]
fn the_run_band_is_asymmetric_on_both_edges() {
    let inside = (RUN_EXIT_SPEED + RUN_ENTER_SPEED) * 0.5;
    assert_eq!(
        next_locomotion_state(LocomotionState::Walk, moving(inside)),
        LocomotionState::Walk
    );
    assert_eq!(
        next_locomotion_state(LocomotionState::Run, moving(inside)),
        LocomotionState::Run
    );
    assert_eq!(
        next_locomotion_state(LocomotionState::Walk, moving(RUN_ENTER_SPEED)),
        LocomotionState::Run
    );
    assert_eq!(
        next_locomotion_state(LocomotionState::Run, moving(RUN_EXIT_SPEED - 0.01)),
        LocomotionState::Walk
    );
}

#[test]
fn the_turn_band_is_asymmetric_on_both_edges_and_signed() {
    let inside = (TURN_EXIT_RATE + TURN_ENTER_RATE) * 0.5;
    assert_eq!(
        next_locomotion_state(LocomotionState::Idle, turning(inside)),
        LocomotionState::Idle
    );
    assert_eq!(
        next_locomotion_state(LocomotionState::TurnLeft, turning(inside)),
        LocomotionState::TurnLeft
    );
    assert_eq!(
        next_locomotion_state(LocomotionState::Idle, turning(-TURN_ENTER_RATE)),
        LocomotionState::TurnRight
    );
    assert_eq!(
        next_locomotion_state(LocomotionState::TurnRight, turning(-inside)),
        LocomotionState::TurnRight
    );
    assert_eq!(
        next_locomotion_state(LocomotionState::TurnRight, turning(-TURN_EXIT_RATE + 0.01)),
        LocomotionState::Idle
    );
}

/// The actual requirement the bands exist for, stated directly: a speed
/// oscillating across the raw enter threshold must not produce a
/// changing state. A single-threshold implementation fails this.
#[test]
fn a_speed_oscillating_across_the_raw_run_threshold_does_not_flap() {
    let mut state = LocomotionState::Walk;
    for tick in 0..64 {
        let speed = if tick % 2 == 0 {
            RUN_ENTER_SPEED - 0.2
        } else {
            RUN_ENTER_SPEED - 0.05
        };
        state = next_locomotion_state(state, moving(speed));
        assert_eq!(state, LocomotionState::Walk, "flapped at tick {tick}");
    }
}

#[test]
fn a_speed_oscillating_across_the_raw_walk_threshold_does_not_flap() {
    let mut state = LocomotionState::Idle;
    for tick in 0..64 {
        let speed = if tick % 2 == 0 {
            WALK_ENTER_SPEED - 0.12
        } else {
            WALK_ENTER_SPEED - 0.01
        };
        state = next_locomotion_state(state, moving(speed));
        assert_eq!(state, LocomotionState::Idle, "flapped at tick {tick}");
    }
}

fn speed_of(velocity: [f32; 2]) -> f32 {
    (velocity[0] * velocity[0] + velocity[1] * velocity[1]).sqrt()
}

/// Regression for #224: smoothing scalar speed cannot distinguish genuine
/// travel from equal-and-opposite collision jitter. The signed velocity must
/// cancel over the window so a stationary actor does not run in place.
#[test]
fn net_zero_velocity_jitter_settles_to_idle() {
    let mut window = VelocityWindow::default();
    let mut state = LocomotionState::Idle;

    for tick in 0..128 {
        let direction = if tick % 2 == 0 { 1.0 } else { -1.0 };
        let net_velocity = window.push([direction * ROUTE_SPEED_METRES_PER_SECOND, 0.0]);
        let speed = if window.coherence() >= MOTION_COHERENCE_THRESHOLD {
            speed_of(net_velocity)
        } else {
            0.0
        };
        state = next_locomotion_state(state, moving(speed));
        if tick >= 40 {
            assert_eq!(state, LocomotionState::Idle, "jitter moved at tick {tick}");
        }
    }
}

#[test]
fn a_small_signed_velocity_residual_without_directional_agreement_is_idle() {
    let mut window = VelocityWindow::default();
    for tick in 0..MOTION_WINDOW_TICKS {
        let direction = if tick < MOTION_WINDOW_TICKS - 3 {
            1.0
        } else {
            -1.0
        };
        window.push([direction * ROUTE_SPEED_METRES_PER_SECOND, 0.0]);
    }
    assert!(window.coherence() < MOTION_COHERENCE_THRESHOLD);
    let speed = if window.coherence() >= MOTION_COHERENCE_THRESHOLD {
        speed_of(window.push([0.0, 0.0]))
    } else {
        0.0
    };
    assert_eq!(
        next_locomotion_state(LocomotionState::Idle, moving(speed)),
        LocomotionState::Idle
    );
}

#[test]
fn steady_signed_velocity_still_settles_to_run() {
    let mut window = VelocityWindow::default();
    let mut state = LocomotionState::Idle;

    for _ in 0..128 {
        let net_velocity = window.push([ROUTE_SPEED_METRES_PER_SECOND, 0.0]);
        state = next_locomotion_state(state, moving(speed_of(net_velocity)));
    }

    assert_eq!(state, LocomotionState::Run);
}

#[test]
fn a_yaw_rate_oscillating_across_the_raw_turn_threshold_does_not_flap() {
    let mut state = LocomotionState::Idle;
    for tick in 0..64 {
        let rate = if tick % 2 == 0 {
            TURN_ENTER_RATE - 0.3
        } else {
            TURN_ENTER_RATE - 0.01
        };
        state = next_locomotion_state(state, turning(rate));
        assert_eq!(state, LocomotionState::Idle, "flapped at tick {tick}");
    }
}

/// Regression for the live agent freeze: a holding actor can apply a
/// full-strength yaw correction whose sign alternates every fixed tick while
/// its desired direction settles. Feeding that raw signal to the classifier
/// restarts the opposite one-shot turn clip every frame; the smoothed signal
/// must settle to idle instead.
#[test]
fn a_sign_alternating_yaw_rate_settles_to_idle() {
    let mut window = YawRateWindow::default();
    let mut state = LocomotionState::Idle;
    for tick in 0..128 {
        let raw = if tick % 2 == 0 {
            std::f32::consts::PI
        } else {
            -std::f32::consts::PI
        };
        let net_yaw_rate = window.push(raw);
        let yaw_rate = if window.coherence() >= MOTION_COHERENCE_THRESHOLD {
            net_yaw_rate
        } else {
            0.0
        };
        state = next_locomotion_state(state, turning(yaw_rate));
        if tick >= MOTION_WINDOW_TICKS {
            assert_eq!(state, LocomotionState::Idle, "turn flap at tick {tick}");
        }
    }
}

#[test]
fn a_small_signed_yaw_residual_without_directional_agreement_is_idle() {
    let mut window = YawRateWindow::default();
    for tick in 0..MOTION_WINDOW_TICKS {
        let rate = if tick < MOTION_WINDOW_TICKS - 3 {
            std::f32::consts::PI
        } else {
            -std::f32::consts::PI
        };
        window.push(rate);
    }
    assert!(window.coherence() < MOTION_COHERENCE_THRESHOLD);
    let yaw_rate = if window.coherence() >= MOTION_COHERENCE_THRESHOLD {
        window.push(0.0)
    } else {
        0.0
    };
    assert_eq!(
        next_locomotion_state(LocomotionState::Idle, turning(yaw_rate)),
        LocomotionState::Idle
    );
}

#[test]
fn a_candidate_that_flips_before_the_dwell_does_not_restart_the_clip() {
    let mut debounce = LocomotionDebounce::default();
    let mut state = LocomotionState::Idle;
    for tick in 0..64 {
        let candidate = if tick % 2 == 0 {
            LocomotionState::Walk
        } else {
            LocomotionState::Idle
        };
        state = debounce.update(state, candidate, 1.0 / FIXED_TICK_HZ);
        assert_eq!(
            state,
            LocomotionState::Idle,
            "candidate flap at tick {tick}"
        );
    }
}

#[test]
fn a_sustained_candidate_commits_after_the_dwell() {
    let mut debounce = LocomotionDebounce::default();
    let mut state = LocomotionState::Idle;
    for _ in 0..8 {
        state = debounce.update(state, LocomotionState::Walk, 1.0 / FIXED_TICK_HZ);
    }
    assert_eq!(state, LocomotionState::Walk);
}

#[test]
fn a_sustained_turn_holds_through_one_opposite_rate_sample() {
    let mut window = YawRateWindow::default();
    let mut state = LocomotionState::Idle;
    for tick in 0..128 {
        let raw = if tick == 64 {
            -std::f32::consts::PI
        } else {
            std::f32::consts::PI
        };
        let net_yaw_rate = window.push(raw);
        state = next_locomotion_state(state, turning(net_yaw_rate));
        if tick >= 16 {
            assert_eq!(state, LocomotionState::TurnLeft, "lost turn at tick {tick}");
        }
    }
}

#[test]
fn a_decaying_yaw_rate_returns_to_idle() {
    let mut window = YawRateWindow::default();
    let mut state = LocomotionState::TurnLeft;
    window.push(TURN_ENTER_RATE * 1.5);
    for _ in 0..128 {
        let net_yaw_rate = window.push(0.0);
        state = next_locomotion_state(state, turning(net_yaw_rate));
    }
    assert_eq!(state, LocomotionState::Idle);
}

#[test]
fn translation_beats_turning_so_a_corner_keeps_the_walk_clip() {
    assert_eq!(
        next_locomotion_state(
            LocomotionState::Walk,
            LocomotionObservation {
                achieved_horizontal_speed: 1.0,
                yaw_rate: 2.0,
            },
        ),
        LocomotionState::Walk
    );
}

/// A wedged agent -- navigation desires full route speed, the KCC
/// achieves nothing -- must not play a walk clip. Only the achieved side
/// is an input, so this falls out of the signature, and this test pins
/// it against anyone reintroducing the desired side as a fallback.
#[test]
fn a_wedged_agent_that_achieves_no_motion_is_idle() {
    assert_eq!(
        next_locomotion_state(LocomotionState::Run, moving(0.0)),
        LocomotionState::Idle
    );
    assert_eq!(
        next_locomotion_state(LocomotionState::Walk, moving(0.0)),
        LocomotionState::Idle
    );
}

#[test]
fn a_run_decelerating_to_a_halt_passes_through_walk() {
    // The `WALK_ENTER_SPEED < RUN_EXIT_SPEED` const assertion in shape:
    // there is no speed at which a running agent jumps straight to idle
    // while still translating.
    let mut state = LocomotionState::Run;
    let mut seen = Vec::new();
    for step in 0..=25u16 {
        let speed = ROUTE_SPEED_METRES_PER_SECOND * (1.0 - f32::from(step) / 25.0);
        state = next_locomotion_state(state, moving(speed));
        if seen.last() != Some(&state) {
            seen.push(state);
        }
    }
    assert_eq!(
        seen,
        vec![
            LocomotionState::Run,
            LocomotionState::Walk,
            LocomotionState::Idle
        ]
    );
}
