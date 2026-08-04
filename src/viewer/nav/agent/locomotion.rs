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

/// Number of fixed ticks in the signed motion window. The nav/physics schedule
/// runs at 64 Hz, so this is a 125 ms net-motion sample. That is long enough
/// to cancel the equal-and-opposite KCC corrections seen when an agent is
/// holding against a corner, while short enough that a real stop or start is
/// reflected promptly.
pub(crate) const MOTION_WINDOW_TICKS: usize = 8;

/// Minimum directional agreement for a window to count as locomotion. A
/// non-zero average alone is not sufficient: a collision jitter sequence can
/// have a small residual when one side contributes an extra sample. Requiring
/// three quarters of the window's signed motion to agree keeps genuine travel responsive
/// while treating that residual as a stop.
pub(crate) const MOTION_COHERENCE_THRESHOLD: f32 = 0.75;

/// Minimum time a different locomotion verdict must remain stable before it
/// can replace the current clip. This is a transition debounce, not a
/// movement timer: it only protects the animation boundary from a candidate
/// that flips faster than a visible clip transition can begin.
pub(crate) const LOCOMOTION_CHANGE_DWELL_SECONDS: f32 = 0.1;

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
    assert!(MOTION_WINDOW_TICKS > 0);
};

/// A fixed-tick rolling average of signed achieved horizontal velocity.
///
/// Averaging the vector, rather than its scalar length, is the important bit:
/// `+v, -v` collision jitter has a large instantaneous speed but zero net
/// travel. Keeping the samples in a bounded ring makes the filter state
/// `Copy`, so it can live directly on [`NavBoundActor`] without a heap
/// allocation or a second ECS resource.
#[derive(Debug, Clone, Copy)]
pub(crate) struct VelocityWindow {
    samples: [[f32; 2]; MOTION_WINDOW_TICKS],
    next: usize,
    len: usize,
    sum: [f32; 2],
    magnitude_sum: f32,
}

impl Default for VelocityWindow {
    fn default() -> Self {
        Self {
            samples: [[0.0; 2]; MOTION_WINDOW_TICKS],
            next: 0,
            len: 0,
            sum: [0.0; 2],
            magnitude_sum: 0.0,
        }
    }
}

impl VelocityWindow {
    pub(crate) fn push(&mut self, raw: [f32; 2]) -> [f32; 2] {
        if self.len == MOTION_WINDOW_TICKS {
            let old = self.samples[self.next];
            self.sum[0] -= old[0];
            self.sum[1] -= old[1];
            self.magnitude_sum -= (old[0] * old[0] + old[1] * old[1]).sqrt();
        } else {
            self.len += 1;
        }
        self.samples[self.next] = raw;
        self.sum[0] += raw[0];
        self.sum[1] += raw[1];
        self.magnitude_sum += (raw[0] * raw[0] + raw[1] * raw[1]).sqrt();
        self.next = (self.next + 1) % MOTION_WINDOW_TICKS;
        let divisor = self.len as f32;
        [self.sum[0] / divisor, self.sum[1] / divisor]
    }

    /// Ratio of net signed motion to all achieved motion in the window. One
    /// means every sample agrees; zero means equal-and-opposite corrections.
    pub(crate) fn coherence(&self) -> f32 {
        if self.magnitude_sum <= f32::EPSILON {
            0.0
        } else {
            (self.sum[0] * self.sum[0] + self.sum[1] * self.sum[1]).sqrt() / self.magnitude_sum
        }
    }
}

/// A fixed-tick rolling average of the yaw rate actually applied by the
/// facing system. It uses the same short net-motion window as velocity so a
/// holding actor's equal-and-opposite heading corrections do not restart
/// opposite one-shot turn clips every tick.
#[derive(Debug, Clone, Copy)]
pub(crate) struct YawRateWindow {
    samples: [f32; MOTION_WINDOW_TICKS],
    next: usize,
    len: usize,
    sum: f32,
    magnitude_sum: f32,
}

impl Default for YawRateWindow {
    fn default() -> Self {
        Self {
            samples: [0.0; MOTION_WINDOW_TICKS],
            next: 0,
            len: 0,
            sum: 0.0,
            magnitude_sum: 0.0,
        }
    }
}

impl YawRateWindow {
    pub(crate) fn push(&mut self, raw: f32) -> f32 {
        if self.len == MOTION_WINDOW_TICKS {
            let old = self.samples[self.next];
            self.sum -= old;
            self.magnitude_sum -= old.abs();
        } else {
            self.len += 1;
        }
        self.samples[self.next] = raw;
        self.sum += raw;
        self.magnitude_sum += raw.abs();
        self.next = (self.next + 1) % MOTION_WINDOW_TICKS;
        self.sum / self.len as f32
    }

    /// Ratio of net signed yaw to all yaw in the window. One means a
    /// sustained turn; zero means equal-and-opposite heading corrections.
    pub(crate) fn coherence(&self) -> f32 {
        if self.magnitude_sum <= f32::EPSILON {
            0.0
        } else {
            self.sum.abs() / self.magnitude_sum
        }
    }
}

/// Debounces the state selected by [`next_locomotion_state`]. Navigation can
/// legitimately report a few opposite corrections while a KCC is resolving a
/// corner; requiring a short stable candidate keeps those corrections from
/// repeatedly restarting gameplay clips.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct LocomotionDebounce {
    pending: Option<LocomotionState>,
    elapsed: f32,
}

impl LocomotionDebounce {
    pub(crate) fn update(
        &mut self,
        current: LocomotionState,
        candidate: LocomotionState,
        dt: f32,
    ) -> LocomotionState {
        if candidate == current {
            self.pending = None;
            self.elapsed = 0.0;
            return current;
        }
        if self.pending != Some(candidate) {
            self.pending = Some(candidate);
            self.elapsed = 0.0;
        }
        self.elapsed += dt.max(0.0);
        if self.elapsed >= LOCOMOTION_CHANGE_DWELL_SECONDS {
            self.pending = None;
            self.elapsed = 0.0;
            candidate
        } else {
            current
        }
    }
}

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
#[path = "../tests/locomotion.rs"]
mod tests;
