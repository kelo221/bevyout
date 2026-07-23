//! Pure std-only locomotion-animation policy for nav-bound actors (issue
//! #188): maps the horizontal speed a nav agent's KCC **actually achieved**
//! this tick, plus the yaw rate it actually turned through, onto the
//! locomotion clip an actor should be playing.
//!
//! Std-only (no `bevy`/`bevy_landmass`/`bevyout_core` import) so
//! `tests/features.rs` can include it verbatim via `#[path]`, mirroring
//! `movement_policy.rs` and `fall_guard.rs`. The Bevy consumer lives in
//! `nav/actor_binding.rs`; it does nothing but sample the `desired`/
//! `achieved` pair `apply_agent_physics_movement` already computed for
//! `movement_policy::decide_collision_outcome`, feed it here, and hand the
//! verdict to `actor_animation::request_actor_animation`.
//!
//! # Achieved, never desired
//!
//! The input is achieved motion on purpose. An agent wedged against a wall
//! has a large *desired* velocity and near-zero *achieved* velocity: playing
//! its walk clip would show feet striding across a floor the actor is not
//! crossing. That is the display-side twin of the movement-authority
//! confusion the navmesh post-mortem's §2.3 documents, so this module reads
//! only the side physics actually delivered.
//!
//! # Why hysteresis, and why bands rather than timers
//!
//! Every threshold below is a *pair*: a higher rate to enter a state and a
//! lower one to leave it. A single threshold makes an agent whose speed sits
//! on it -- which is the common case, because landmass steers toward a
//! constant desired speed and local avoidance perturbs it -- alternate clips
//! every tick. Bands are preferred over a dwell timer because they are
//! frame-rate independent and need no state beyond the previous verdict,
//! which the caller already holds; a timer would have to be threaded through
//! the fixed-tick cadence and would still flap for any oscillation slower
//! than the dwell.
//!
//! # Constant derivations
//!
//! All speeds derive from [`ROUTE_SPEED_METRES_PER_SECOND`], the agent
//! desired speed `nav/agent.rs` configures on every `AgentSettings`. All
//! rates derive from the 64 Hz fixed tick the viewer runs its nav/physics
//! schedule at, so the "leave" edges line up with the per-frame epsilons
//! `actor_animation` has used for unbound actors since #106 -- a bound and
//! an unbound actor at the same speed should not disagree about whether they
//! are moving.

/// The desired route speed every nav agent is configured with, mirroring
/// `nav/agent.rs`'s `AGENT_DESIRED_SPEED`. Duplicated as a plain literal
/// because that constant lives in a Bevy module this std-only one cannot
/// import; `actor_binding.rs` carries a compile-time assertion that the two
/// still agree, so the duplicate cannot silently drift.
pub(crate) const ROUTE_SPEED_METRES_PER_SECOND: f32 = 2.5;

/// The viewer's fixed-tick rate (Bevy's `Time<Fixed>` default, 64 Hz), used
/// only to convert `actor_animation`'s existing per-frame epsilons into the
/// per-second rates this module compares against.
const FIXED_TICK_HZ: f32 = 64.0;

/// Achieved horizontal speed (m/s) at or above which a stationary agent
/// starts walking: 10% of the route speed. Deliberately well above the
/// leave edge below so a KCC settling onto a floor, or micro-sliding along
/// a wall while pressed against it, does not read as locomotion.
pub(crate) const WALK_ENTER_SPEED: f32 = 0.1 * ROUTE_SPEED_METRES_PER_SECOND;

/// Achieved horizontal speed (m/s) below which a moving agent returns to
/// idle: half the enter edge, which is also `actor_animation`'s existing
/// `MOVEMENT_EPSILON_METRES` (0.002 m of travel per frame) expressed as a
/// speed at the 64 Hz fixed tick -- 0.002 * 64 = 0.128 m/s, within 3% of
/// 0.125. A bound actor and an unbound one therefore agree on "moving".
pub(crate) const WALK_EXIT_SPEED: f32 = 0.5 * WALK_ENTER_SPEED;

/// Achieved horizontal speed (m/s) at or above which a walking agent breaks
/// into a run: 70% of the route speed. It has to sit below the route speed
/// or an agent on a clear straight corridor -- the case the run clip exists
/// for -- would never reach it, and it has to sit above the leave edge by a
/// margin wider than the speed ripple local avoidance produces.
pub(crate) const RUN_ENTER_SPEED: f32 = 0.7 * ROUTE_SPEED_METRES_PER_SECOND;

/// Achieved horizontal speed (m/s) below which a running agent drops back to
/// a walk: 56% of the route speed, i.e. 1.4 m/s, the classical human
/// comfortable-walking speed and the anchor the walk clip is authored
/// around. The 0.35 m/s band between this and [`RUN_ENTER_SPEED`] is 10% of
/// `AGENT_MAX_SPEED`, comfortably wider than the per-tick speed ripple an
/// agent rounding a corner shows.
pub(crate) const RUN_EXIT_SPEED: f32 = 0.56 * ROUTE_SPEED_METRES_PER_SECOND;

/// Yaw rate (rad/s) at or above which a stationary agent plays a turn-in-
/// place clip: 45 deg/s, a quarter of the 180 deg/s about-face
/// `actor_binding`'s facing rate performs, so a genuine route-start pivot
/// trips it immediately while an agent merely settling its facing does not.
pub(crate) const TURN_ENTER_RATE: f32 = std::f32::consts::FRAC_PI_4;

/// Yaw rate (rad/s) below which a turning agent returns to idle: 15 deg/s,
/// which is `actor_animation`'s existing `TURN_EPSILON_RADIANS` (0.004 rad
/// per frame) at the 64 Hz fixed tick -- 0.004 * 64 = 0.256 rad/s, within 2%
/// of PI/12. Same "bound and unbound actors agree" rationale as
/// [`WALK_EXIT_SPEED`].
pub(crate) const TURN_EXIT_RATE: f32 = std::f32::consts::FRAC_PI_2 / 6.0;

/// Time constant (seconds) for [`smooth_achieved_speed`]'s exponential moving
/// average. About 7-8 fixed ticks at [`FIXED_TICK_HZ`] -- long enough to
/// absorb the achieved-speed ripple `apply_agent_physics_movement` produces
/// tick to tick (issue #224: observed live swinging the raw sample between
/// near-zero and full route speed, which crosses every hysteresis band in
/// this module every tick, not just the raw threshold the bands already
/// guard), short enough that a genuine stop still reads as idle within a
/// few frames rather than gliding to a stop over a visible fraction of a
/// second.
const SPEED_SMOOTHING_TIME_CONSTANT_SECONDS: f32 = 0.12;

/// Exponentially smooths `raw` (this tick's achieved horizontal speed sample)
/// toward `previous` (the smoothed value the caller carried over from last
/// tick), frame-rate independent via `dt`.
///
/// This runs *before* [`next_locomotion_state`], not instead of it: the
/// hysteresis bands there guard against a speed sitting near one raw
/// threshold; this guards against the caller's raw sample itself swinging
/// across the *entire* band pair tick to tick, which no threshold placement
/// can absorb. `dt <= 0.0` (a paused or zero-length tick) returns `previous`
/// unchanged rather than dividing by zero.
pub(crate) fn smooth_achieved_speed(previous: f32, raw: f32, dt: f32) -> f32 {
    if dt <= 0.0 {
        return previous;
    }
    let alpha = (dt / (SPEED_SMOOTHING_TIME_CONSTANT_SECONDS + dt)).clamp(0.0, 1.0);
    previous + (raw - previous) * alpha
}

const _: () = {
    // The exit edges are documented as restatements of `actor_animation`'s
    // per-frame epsilons at the fixed tick. Pin that claim rather than
    // leaving it in prose: 0.002 m/frame and 0.004 rad/frame, each matched
    // to within 0.01 of its per-second equivalent (`f32::abs` is not const,
    // hence the two-sided form).
    assert!(WALK_EXIT_SPEED - 0.002 * FIXED_TICK_HZ < 0.01);
    assert!(0.002 * FIXED_TICK_HZ - WALK_EXIT_SPEED < 0.01);
    assert!(TURN_EXIT_RATE - 0.004 * FIXED_TICK_HZ < 0.01);
    assert!(0.004 * FIXED_TICK_HZ - TURN_EXIT_RATE < 0.01);
    // Every band must be a real band, not a degenerate single threshold.
    assert!(WALK_EXIT_SPEED < WALK_ENTER_SPEED);
    assert!(RUN_EXIT_SPEED < RUN_ENTER_SPEED);
    assert!(TURN_EXIT_RATE < TURN_ENTER_RATE);
    // A running agent must pass through walking on the way to idle: the run
    // leave edge sits above the walk enter edge.
    assert!(WALK_ENTER_SPEED < RUN_EXIT_SPEED);
};

/// The locomotion clip family an actor should be playing. Deliberately its
/// own enum rather than `actor_animation::policy::ActorAnimationState`:
/// that type also carries `Equip`/`Unequip`, which locomotion has no opinion
/// about, and it lives behind a `bevyout_core` import this std-only module
/// must not take. `actor_binding.rs` owns the one-line total mapping.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum LocomotionState {
    #[default]
    Idle,
    Walk,
    Run,
    TurnLeft,
    TurnRight,
}

impl LocomotionState {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Walk => "walk",
            Self::Run => "run",
            Self::TurnLeft => "turn_left",
            Self::TurnRight => "turn_right",
        }
    }

    /// Whether this state means the agent is translating across the floor
    /// (as opposed to standing still or pivoting on the spot).
    const fn is_translating(self) -> bool {
        matches!(self, Self::Walk | Self::Run)
    }

    const fn is_turning(self) -> bool {
        matches!(self, Self::TurnLeft | Self::TurnRight)
    }
}

/// One tick's motion, sampled from the KCC after it has resolved collision.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct LocomotionObservation {
    /// Length of the achieved horizontal velocity, m/s. This is
    /// `(new_position - old_position).xz() / dt` as
    /// `apply_agent_physics_movement` already computes it -- the real,
    /// collision-resolved motion, not landmass's desired velocity.
    pub(crate) achieved_horizontal_speed: f32,
    /// Signed yaw rate, rad/s. Positive is a left turn, matching Bevy's
    /// `EulerRot::YXZ` yaw sign (counter-clockwise about +Y seen from
    /// above) and `actor_animation`'s own existing `yaw_delta > 0 =>
    /// TurnLeft` convention for unbound actors.
    pub(crate) yaw_rate: f32,
}

/// The locomotion state an actor previously in `previous` should move to,
/// given this tick's `observation`.
///
/// Translation dominates turning: an agent walking around a corner is
/// walking, not turning in place, so the turn clips are only ever selected
/// for an agent whose achieved translation is below the walk band. That
/// ordering is also what keeps the turn band from stealing frames from a
/// route: landmass steers hardest exactly where the corridor bends.
pub(crate) fn next_locomotion_state(
    previous: LocomotionState,
    observation: LocomotionObservation,
) -> LocomotionState {
    let speed = observation.achieved_horizontal_speed;
    let walk_threshold = if previous.is_translating() {
        WALK_EXIT_SPEED
    } else {
        WALK_ENTER_SPEED
    };
    if speed >= walk_threshold {
        let run_threshold = if previous == LocomotionState::Run {
            RUN_EXIT_SPEED
        } else {
            RUN_ENTER_SPEED
        };
        return if speed >= run_threshold {
            LocomotionState::Run
        } else {
            LocomotionState::Walk
        };
    }
    let turn_threshold = if previous.is_turning() {
        TURN_EXIT_RATE
    } else {
        TURN_ENTER_RATE
    };
    if observation.yaw_rate >= turn_threshold {
        LocomotionState::TurnLeft
    } else if observation.yaw_rate <= -turn_threshold {
        LocomotionState::TurnRight
    } else {
        LocomotionState::Idle
    }
}

#[cfg(test)]
mod tests {
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

    /// The regression test for #224's live bug: an achieved speed sample
    /// alternating every tick between full route speed and near-zero (what
    /// `apply_agent_physics_movement` was observed producing) swings clean
    /// across every hysteresis band above, not just the raw threshold they
    /// guard -- so feeding it straight to [`next_locomotion_state`] flaps
    /// idle/run every tick (the control assertion below). Routing it through
    /// [`smooth_achieved_speed`] first settles the classifier once the EMA
    /// has warmed up.
    #[test]
    fn smoothing_stops_an_alternating_achieved_speed_from_flapping_the_classifier() {
        const TICK_SECONDS: f32 = 1.0 / FIXED_TICK_HZ;
        const WARMUP_TICKS: usize = 32;
        const TOTAL_TICKS: usize = 128;

        // Control: the same raw alternation fed directly into the classifier,
        // with no smoothing, does flap almost every tick -- this is what
        // distinguishes this test from `a_speed_oscillating_across_the_raw_
        // run_threshold_does_not_flap` above, whose raw input stays within
        // one band pair. This one swings the full route speed, clean across
        // every band.
        let mut raw_state = LocomotionState::Idle;
        let mut raw_changes = 0;
        for tick in 0..TOTAL_TICKS {
            let raw = if tick % 2 == 0 {
                ROUTE_SPEED_METRES_PER_SECOND
            } else {
                0.0
            };
            let next = next_locomotion_state(raw_state, moving(raw));
            if next != raw_state {
                raw_changes += 1;
            }
            raw_state = next;
        }
        assert!(
            raw_changes > TOTAL_TICKS / 2,
            "expected the unsmoothed classifier to flap nearly every tick, got {raw_changes} changes"
        );

        // The fix: the same alternation, smoothed first.
        let mut smoothed = 0.0f32;
        let mut state = LocomotionState::Idle;
        let mut changes_after_warmup = 0;
        for tick in 0..TOTAL_TICKS {
            let raw = if tick % 2 == 0 {
                ROUTE_SPEED_METRES_PER_SECOND
            } else {
                0.0
            };
            smoothed = smooth_achieved_speed(smoothed, raw, TICK_SECONDS);
            let next = next_locomotion_state(state, moving(smoothed));
            if tick >= WARMUP_TICKS && next != state {
                changes_after_warmup += 1;
            }
            state = next;
        }
        assert!(
            changes_after_warmup <= 1,
            "classifier still flapped {changes_after_warmup} times per tick after the EMA warmed up"
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
}
