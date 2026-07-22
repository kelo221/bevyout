//! Pure AI package-family dispatch (issues #196 Travel+Patrol, #197
//! Idle+Eat+Sleep).
//!
//! Selection (#193) decides *which* package is active, the lifecycle (#194)
//! owns *what happens to it over time*, and resolution (#195) answers *where*
//! its location/target is. This module is the missing driver those three were
//! waiting for: a single [`FamilyDriver`] whose [`FamilyDriver::tick`] matches
//! the active package's [`PackageFamily`] onto the concrete per-family
//! behaviour, and reports back both a [`LifecycleSignal`] (advance/complete/
//! fail -- the transitions `lifecycle.rs` had no non-test caller for) and, at
//! most, one [`FamilyRequest`].
//!
//! # One movement authority (verdict §2.3)
//!
//! A family may only ever *request* two things: navigation ([`FamilyRequest::
//! Route`]/[`FamilyRequest::Stop`]) and an animation state
//! ([`FamilyRequest::Play`]). It never produces a position -- occupying an
//! interaction point sets the actor's pose once *through the nav route*, not by
//! writing a transform and not via clip root motion. The Bevy adapter
//! (`ai::family_runtime`) is the only code that turns these requests into an
//! `AgentTarget3d` insertion and a `request_actor_animation` call; its
//! minimal-`App` test asserts a driven family writes no `Transform.translation`
//! of its own.
//!
//! std-only (no Bevy, no serde) so it compiles verbatim into
//! `tests/features.rs` via `#[path]`, exactly like `selection.rs`/
//! `lifecycle.rs`/`resolution.rs`.

use std::collections::HashSet;

/// After this many consecutive route failures a family gives up on the current
/// attempt and returns [`LifecycleSignal::Fail`], handing the retry/backoff
/// decision to the lifecycle (#194). Below the ceiling the family simply
/// re-issues the route -- the "re-issue on route failure" the Travel/Patrol
/// brief calls for -- so a transient off-mesh blip does not abandon the route.
const MAX_ROUTE_REISSUES: u32 = 3;

/// The five package families this wave drives. Follow (type 1) and Sandbox
/// (type 12) are explicitly out of scope (#198, needs #185 doors).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageFamily {
    /// FO3 PACK type 6: route to a single resolved location, then complete.
    Travel,
    /// FO3 PACK type 13: traverse authored patrol markers in order, waiting at
    /// each; cyclic, so it never completes on its own.
    Patrol,
    /// FO3 PACK type 5 (Wander, driven in place this wave): idle at the
    /// current/resolved location with the authored idle-marker orientation.
    Idle,
    /// FO3 PACK type 3: occupy an eat interaction point and play the eat state.
    Eat,
    /// FO3 PACK type 4: occupy a sleep interaction point and play the sleep
    /// state.
    Sleep,
}

impl PackageFamily {
    /// Maps a prepared `PKDT.type` onto its family, or `None` for a type this
    /// wave does not drive. The one dispatch entry point -- the same `u8`
    /// values `ai_package_commands::package_type_label` names.
    #[must_use]
    pub fn from_package_type(package_type: u8) -> Option<Self> {
        match package_type {
            3 => Some(Self::Eat),
            4 => Some(Self::Sleep),
            5 => Some(Self::Idle),
            6 => Some(Self::Travel),
            13 => Some(Self::Patrol),
            _ => None,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Travel => "travel",
            Self::Patrol => "patrol",
            Self::Idle => "idle",
            Self::Eat => "eat",
            Self::Sleep => "sleep",
        }
    }
}

/// An animation state a family asks the actor to hold in place. The Bevy
/// adapter maps this onto the runtime `ActorAnimationState`; `Eat`/`Sleep` map
/// to the idle clip until dedicated furniture clips exist, but stay distinct
/// here so the intent is preserved for `showpackages` and future clip work.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamilyAnimation {
    Idle,
    Eat,
    Sleep,
}

/// The *only* things a family may request. Navigation and animation -- never a
/// transform write (module doc comment, verdict §2.3).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FamilyRequest {
    /// Route the actor to this resolved world point through the nav seam.
    Route([f32; 3]),
    /// Clear the current nav route (the actor has arrived / is done moving).
    Stop,
    /// Play this animation state in place.
    Play(FamilyAnimation),
}

/// What the family tells the lifecycle to do this tick. Exactly the four
/// transitions `PackageLifecycle` exposes for a running package.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleSignal {
    /// Keep running; nothing to transition.
    Continue,
    /// A procedure step boundary was crossed (`lifecycle.advance_step()`).
    AdvanceStep,
    /// The package finished successfully (`lifecycle.complete()`).
    Complete,
    /// The current attempt failed (`lifecycle.fail()` -> backoff + retry).
    Fail,
}

/// The per-tick observation the Bevy adapter samples from the bound nav agent.
/// All plain data -- no engine handles leak into this module.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FamilyObservation {
    pub actor_position: [f32; 3],
    /// The nav agent reports it reached its current route target
    /// (`AgentState::ReachedTarget`).
    pub nav_reached: bool,
    /// The last route request could not be pathed (`AgentState::NoPath` /
    /// `AgentNotOnNavMesh`).
    pub route_failed: bool,
}

/// One authored waypoint a family drives to: a patrol marker, an idle marker,
/// or an eat/sleep interaction point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Waypoint {
    pub position: [f32; 3],
    /// Seconds to wait/idle on arrival (patrol markers); `0` passes through.
    pub wait_seconds: f32,
    /// Authored facing yaw radians (idle/patrol-marker orientation), if any.
    pub orientation_yaw: Option<f32>,
    /// The interaction-point id to claim/release while occupied (eat/sleep
    /// furniture); `None` for plain travel/patrol markers.
    pub interaction_point: Option<u32>,
}

impl Waypoint {
    /// A plain positional waypoint (travel destination / idle location).
    #[must_use]
    pub fn at(position: [f32; 3]) -> Self {
        Self {
            position,
            wait_seconds: 0.0,
            orientation_yaw: None,
            interaction_point: None,
        }
    }
}

/// The outcome of one [`FamilyDriver::tick`]: at most one request to apply, and
/// the lifecycle transition to run.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FamilyStep {
    pub request: Option<FamilyRequest>,
    pub signal: LifecycleSignal,
}

impl FamilyStep {
    fn new(request: Option<FamilyRequest>, signal: LifecycleSignal) -> Self {
        Self { request, signal }
    }
}

fn distance_squared(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx * dx + dy * dy + dz * dz
}

/// Picks the nearest waypoint to `from` whose interaction point is not already
/// claimed in `occupied`. Waypoints without an interaction point are eligible
/// (they cannot collide). Returns the chosen index, or `None` when every
/// candidate is occupied or the list is empty -- a deterministic reason for the
/// eat/sleep family to find no free furniture.
#[must_use]
pub fn select_interaction_point(
    waypoints: &[Waypoint],
    from: [f32; 3],
    occupied: &HashSet<u32>,
) -> Option<usize> {
    waypoints
        .iter()
        .enumerate()
        .filter(|(_, waypoint)| {
            waypoint
                .interaction_point
                .is_none_or(|point| !occupied.contains(&point))
        })
        .min_by(|(_, left), (_, right)| {
            distance_squared(left.position, from)
                .partial_cmp(&distance_squared(right.position, from))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(index, _)| index)
}

/// The internal progress label a driver is in, surfaced by
/// [`FamilyDriver::step_label`] for the `runpackage` console view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Progress {
    Routing,
    Waiting,
    Idling,
    Occupying,
    Done,
}

impl Progress {
    const fn label(self) -> &'static str {
        match self {
            Self::Routing => "routing",
            Self::Waiting => "waiting",
            Self::Idling => "idling",
            Self::Occupying => "occupying",
            Self::Done => "done",
        }
    }
}

/// Drives one active package's family behaviour over time. Holds only the
/// bookkeeping a family needs; the lifecycle owns phase/step/elapsed and
/// persistence.
#[derive(Debug, Clone)]
pub struct FamilyDriver {
    family: PackageFamily,
    waypoints: Vec<Waypoint>,
    index: usize,
    arrival_tolerance: f32,
    waiting: bool,
    wait_remaining: f32,
    /// The point the family has currently asked nav to route to; re-issued only
    /// when it changes (or is cleared by a route failure), so a driver does not
    /// spam `goto` every tick.
    routed_target: Option<[f32; 3]>,
    /// The animation last asked for, so `Play` is emitted only on change.
    last_play: Option<FamilyAnimation>,
    route_failures: u32,
    occupied_point: Option<u32>,
    progress: Progress,
}

impl FamilyDriver {
    /// Builds a driver for `family` over `waypoints` (already in traversal
    /// order). `arrival_tolerance` is the horizontal-ish distance within which a
    /// waypoint counts as reached even before the nav agent's own
    /// reached-target latches.
    #[must_use]
    pub fn new(family: PackageFamily, waypoints: Vec<Waypoint>, arrival_tolerance: f32) -> Self {
        Self {
            family,
            waypoints,
            index: 0,
            arrival_tolerance: arrival_tolerance.max(0.0),
            waiting: false,
            wait_remaining: 0.0,
            routed_target: None,
            last_play: None,
            route_failures: 0,
            occupied_point: None,
            progress: Progress::Routing,
        }
    }

    #[must_use]
    pub fn family(&self) -> PackageFamily {
        self.family
    }

    /// The waypoint the driver is currently working toward.
    #[must_use]
    pub fn current_target(&self) -> Option<[f32; 3]> {
        self.waypoints.get(self.index).map(|w| w.position)
    }

    /// The authored facing at the current waypoint, if any.
    #[must_use]
    pub fn current_orientation_yaw(&self) -> Option<f32> {
        self.waypoints
            .get(self.index)
            .and_then(|w| w.orientation_yaw)
    }

    #[must_use]
    pub fn marker_index(&self) -> usize {
        self.index
    }

    #[must_use]
    pub fn marker_count(&self) -> usize {
        self.waypoints.len()
    }

    /// The interaction point this driver currently occupies, if any.
    #[must_use]
    pub fn occupied_point(&self) -> Option<u32> {
        self.occupied_point
    }

    /// A short human label for the current step (`runpackage`/`showpackages`).
    #[must_use]
    pub fn step_label(&self) -> &'static str {
        self.progress.label()
    }

    /// Releases any occupancy claim (called on preempt or completion). Returns
    /// the freed interaction point so the adapter can drop it from the shared
    /// occupancy registry, and resets the animation intent.
    pub fn release(&mut self) -> Option<u32> {
        self.last_play = None;
        self.occupied_point.take()
    }

    /// Advances the family one tick against `obs`. Returns the request to apply
    /// (if any) and the lifecycle transition to run.
    pub fn tick(&mut self, obs: &FamilyObservation, dt: f32) -> FamilyStep {
        // Route failure is shared across every family: re-issue up to the
        // ceiling, then fail the attempt (lifecycle owns backoff + retry).
        if obs.route_failed {
            self.route_failures += 1;
            if self.route_failures > MAX_ROUTE_REISSUES {
                self.route_failures = 0;
                self.routed_target = None;
                return FamilyStep::new(None, LifecycleSignal::Fail);
            }
            // Force the next `ensure_route` to re-emit.
            self.routed_target = None;
        } else {
            self.route_failures = 0;
        }

        match self.family {
            PackageFamily::Travel => self.tick_travel(obs),
            PackageFamily::Patrol => self.tick_patrol(obs, dt),
            PackageFamily::Idle => self.tick_idle(obs),
            PackageFamily::Eat => self.tick_occupy(obs, FamilyAnimation::Eat),
            PackageFamily::Sleep => self.tick_occupy(obs, FamilyAnimation::Sleep),
        }
    }

    fn arrived(&self, obs: &FamilyObservation, target: [f32; 3]) -> bool {
        obs.nav_reached
            || distance_squared(obs.actor_position, target)
                <= self.arrival_tolerance * self.arrival_tolerance
    }

    /// Emits a `Route` only when the destination changed (idempotent otherwise).
    fn ensure_route(&mut self, target: [f32; 3]) -> Option<FamilyRequest> {
        if self.routed_target == Some(target) {
            None
        } else {
            self.routed_target = Some(target);
            self.last_play = None;
            Some(FamilyRequest::Route(target))
        }
    }

    /// Emits a `Play` only when the requested animation changed.
    fn ensure_play(&mut self, animation: FamilyAnimation) -> Option<FamilyRequest> {
        if self.last_play == Some(animation) {
            None
        } else {
            self.last_play = Some(animation);
            Some(FamilyRequest::Play(animation))
        }
    }

    fn tick_travel(&mut self, obs: &FamilyObservation) -> FamilyStep {
        let Some(target) = self.current_target() else {
            self.progress = Progress::Done;
            return FamilyStep::new(None, LifecycleSignal::Complete);
        };
        if self.arrived(obs, target) {
            self.progress = Progress::Done;
            // Stop steering; locomotion idles the actor once it is stationary.
            return FamilyStep::new(Some(FamilyRequest::Stop), LifecycleSignal::Complete);
        }
        self.progress = Progress::Routing;
        FamilyStep::new(self.ensure_route(target), LifecycleSignal::Continue)
    }

    fn tick_patrol(&mut self, obs: &FamilyObservation, dt: f32) -> FamilyStep {
        let Some(target) = self.current_target() else {
            self.progress = Progress::Done;
            return FamilyStep::new(None, LifecycleSignal::Complete);
        };
        if self.waiting {
            self.wait_remaining -= dt.max(0.0);
            if self.wait_remaining <= 0.0 {
                self.waiting = false;
                return self.advance_marker();
            }
            self.progress = Progress::Waiting;
            return FamilyStep::new(
                self.ensure_play(FamilyAnimation::Idle),
                LifecycleSignal::Continue,
            );
        }
        if self.arrived(obs, target) {
            let wait = self.waypoints[self.index].wait_seconds;
            if wait > 0.0 {
                self.waiting = true;
                self.wait_remaining = wait;
                self.progress = Progress::Waiting;
                return FamilyStep::new(
                    self.ensure_play(FamilyAnimation::Idle),
                    LifecycleSignal::Continue,
                );
            }
            return self.advance_marker();
        }
        self.progress = Progress::Routing;
        FamilyStep::new(self.ensure_route(target), LifecycleSignal::Continue)
    }

    /// Moves to the next patrol marker (cyclic) and routes toward it, reporting
    /// the step boundary to the lifecycle.
    fn advance_marker(&mut self) -> FamilyStep {
        if self.waypoints.is_empty() {
            self.progress = Progress::Done;
            return FamilyStep::new(None, LifecycleSignal::Complete);
        }
        self.index = (self.index + 1) % self.waypoints.len();
        self.routed_target = None;
        self.last_play = None;
        self.progress = Progress::Routing;
        let next = self.waypoints[self.index].position;
        FamilyStep::new(self.ensure_route(next), LifecycleSignal::AdvanceStep)
    }

    fn tick_idle(&mut self, obs: &FamilyObservation) -> FamilyStep {
        let Some(target) = self.current_target() else {
            self.progress = Progress::Idling;
            return FamilyStep::new(
                self.ensure_play(FamilyAnimation::Idle),
                LifecycleSignal::Continue,
            );
        };
        if self.arrived(obs, target) {
            self.progress = Progress::Idling;
            return FamilyStep::new(
                self.ensure_play(FamilyAnimation::Idle),
                LifecycleSignal::Continue,
            );
        }
        self.progress = Progress::Routing;
        FamilyStep::new(self.ensure_route(target), LifecycleSignal::Continue)
    }

    fn tick_occupy(&mut self, obs: &FamilyObservation, animation: FamilyAnimation) -> FamilyStep {
        let Some(target) = self.current_target() else {
            self.progress = Progress::Done;
            return FamilyStep::new(None, LifecycleSignal::Fail);
        };
        if self.arrived(obs, target) {
            // The final approach *is* the position set: nav put the actor on the
            // interaction point; occupancy is a claim, not a teleport.
            if self.occupied_point.is_none() {
                self.occupied_point = self.waypoints[self.index].interaction_point;
            }
            self.progress = Progress::Occupying;
            return FamilyStep::new(self.ensure_play(animation), LifecycleSignal::Continue);
        }
        self.progress = Progress::Routing;
        FamilyStep::new(self.ensure_route(target), LifecycleSignal::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obs(position: [f32; 3], nav_reached: bool) -> FamilyObservation {
        FamilyObservation {
            actor_position: position,
            nav_reached,
            route_failed: false,
        }
    }

    fn failed_obs(position: [f32; 3]) -> FamilyObservation {
        FamilyObservation {
            actor_position: position,
            nav_reached: false,
            route_failed: true,
        }
    }

    #[test]
    fn dispatch_maps_the_five_supported_types() {
        assert_eq!(
            PackageFamily::from_package_type(3),
            Some(PackageFamily::Eat)
        );
        assert_eq!(
            PackageFamily::from_package_type(4),
            Some(PackageFamily::Sleep)
        );
        assert_eq!(
            PackageFamily::from_package_type(5),
            Some(PackageFamily::Idle)
        );
        assert_eq!(
            PackageFamily::from_package_type(6),
            Some(PackageFamily::Travel)
        );
        assert_eq!(
            PackageFamily::from_package_type(13),
            Some(PackageFamily::Patrol)
        );
        // Follow (1) and Sandbox (12) are #198.
        assert_eq!(PackageFamily::from_package_type(1), None);
        assert_eq!(PackageFamily::from_package_type(12), None);
    }

    #[test]
    fn travel_routes_then_completes_on_arrival() {
        let mut driver = FamilyDriver::new(
            PackageFamily::Travel,
            vec![Waypoint::at([10.0, 0.0, 0.0])],
            0.5,
        );
        // First tick: routes to the destination.
        let step = driver.tick(&obs([0.0, 0.0, 0.0], false), 0.1);
        assert_eq!(step.request, Some(FamilyRequest::Route([10.0, 0.0, 0.0])));
        assert_eq!(step.signal, LifecycleSignal::Continue);
        // Not re-issued while unchanged and still en route.
        let step = driver.tick(&obs([5.0, 0.0, 0.0], false), 0.1);
        assert_eq!(step.request, None);
        assert_eq!(step.signal, LifecycleSignal::Continue);
        // Arrival: stop steering and complete.
        let step = driver.tick(&obs([10.0, 0.0, 0.0], true), 0.1);
        assert_eq!(step.request, Some(FamilyRequest::Stop));
        assert_eq!(step.signal, LifecycleSignal::Complete);
    }

    #[test]
    fn arrival_tolerance_is_a_radius_not_the_nav_flag() {
        let mut inside = FamilyDriver::new(
            PackageFamily::Travel,
            vec![Waypoint::at([0.0, 0.0, 0.0])],
            1.0,
        );
        // Within the 1.0 tolerance but nav has not latched reached: still counts.
        let step = inside.tick(&obs([0.6, 0.0, 0.6], false), 0.1);
        assert_eq!(step.signal, LifecycleSignal::Complete);

        let mut outside = FamilyDriver::new(
            PackageFamily::Travel,
            vec![Waypoint::at([0.0, 0.0, 0.0])],
            1.0,
        );
        // Just outside the tolerance: keeps routing.
        let step = outside.tick(&obs([1.5, 0.0, 0.0], false), 0.1);
        assert_eq!(step.signal, LifecycleSignal::Continue);
        assert!(matches!(step.request, Some(FamilyRequest::Route(_))));
    }

    #[test]
    fn travel_re_issues_route_on_failure_then_fails_the_attempt() {
        let mut driver = FamilyDriver::new(
            PackageFamily::Travel,
            vec![Waypoint::at([10.0, 0.0, 0.0])],
            0.5,
        );
        driver.tick(&obs([0.0, 0.0, 0.0], false), 0.1); // initial route
        // Each failure below the ceiling re-issues the route rather than failing.
        for _ in 0..MAX_ROUTE_REISSUES {
            let step = driver.tick(&failed_obs([0.0, 0.0, 0.0]), 0.1);
            assert_eq!(
                step.request,
                Some(FamilyRequest::Route([10.0, 0.0, 0.0])),
                "route re-issued on failure"
            );
            assert_eq!(step.signal, LifecycleSignal::Continue);
        }
        // One failure past the ceiling gives up this attempt.
        let step = driver.tick(&failed_obs([0.0, 0.0, 0.0]), 0.1);
        assert_eq!(step.signal, LifecycleSignal::Fail);
    }

    #[test]
    fn patrol_visits_markers_in_order_and_cycles() {
        let markers = vec![
            Waypoint {
                position: [0.0, 0.0, 0.0],
                wait_seconds: 0.0,
                orientation_yaw: None,
                interaction_point: None,
            },
            Waypoint {
                position: [10.0, 0.0, 0.0],
                wait_seconds: 0.0,
                orientation_yaw: None,
                interaction_point: None,
            },
            Waypoint {
                position: [10.0, 0.0, 10.0],
                wait_seconds: 0.0,
                orientation_yaw: None,
                interaction_point: None,
            },
        ];
        let mut driver = FamilyDriver::new(PackageFamily::Patrol, markers, 0.5);
        // Route to marker 0.
        let step = driver.tick(&obs([-5.0, 0.0, 0.0], false), 0.1);
        assert_eq!(step.request, Some(FamilyRequest::Route([0.0, 0.0, 0.0])));
        assert_eq!(driver.marker_index(), 0);
        // Reach 0 (no wait) -> advance to 1.
        let step = driver.tick(&obs([0.0, 0.0, 0.0], true), 0.1);
        assert_eq!(step.signal, LifecycleSignal::AdvanceStep);
        assert_eq!(step.request, Some(FamilyRequest::Route([10.0, 0.0, 0.0])));
        assert_eq!(driver.marker_index(), 1);
        // Reach 1 -> advance to 2.
        let step = driver.tick(&obs([10.0, 0.0, 0.0], true), 0.1);
        assert_eq!(step.signal, LifecycleSignal::AdvanceStep);
        assert_eq!(driver.marker_index(), 2);
        // Reach 2 -> cycles back to 0.
        let step = driver.tick(&obs([10.0, 0.0, 10.0], true), 0.1);
        assert_eq!(step.signal, LifecycleSignal::AdvanceStep);
        assert_eq!(driver.marker_index(), 0);
        assert_eq!(step.request, Some(FamilyRequest::Route([0.0, 0.0, 0.0])));
    }

    #[test]
    fn patrol_waits_and_idles_at_a_marker_before_advancing() {
        let markers = vec![
            Waypoint {
                position: [0.0, 0.0, 0.0],
                wait_seconds: 2.0,
                orientation_yaw: None,
                interaction_point: None,
            },
            Waypoint::at([10.0, 0.0, 0.0]),
        ];
        let mut driver = FamilyDriver::new(PackageFamily::Patrol, markers, 0.5);
        driver.tick(&obs([-1.0, 0.0, 0.0], false), 0.1); // route to 0
        // Arrive: begins the wait, idling in place, NOT advancing yet.
        let step = driver.tick(&obs([0.0, 0.0, 0.0], true), 0.1);
        assert_eq!(
            step.request,
            Some(FamilyRequest::Play(FamilyAnimation::Idle))
        );
        assert_eq!(step.signal, LifecycleSignal::Continue);
        assert_eq!(driver.step_label(), "waiting");
        assert_eq!(driver.marker_index(), 0);
        // Still waiting mid-window.
        let step = driver.tick(&obs([0.0, 0.0, 0.0], true), 1.0);
        assert_eq!(step.signal, LifecycleSignal::Continue);
        assert_eq!(driver.marker_index(), 0);
        // Wait elapses -> advance to marker 1.
        let step = driver.tick(&obs([0.0, 0.0, 0.0], true), 1.5);
        assert_eq!(step.signal, LifecycleSignal::AdvanceStep);
        assert_eq!(driver.marker_index(), 1);
    }

    #[test]
    fn idle_routes_to_the_location_then_plays_idle_forever() {
        let mut driver = FamilyDriver::new(
            PackageFamily::Idle,
            vec![Waypoint::at([4.0, 0.0, 0.0])],
            0.5,
        );
        let step = driver.tick(&obs([0.0, 0.0, 0.0], false), 0.1);
        assert_eq!(step.request, Some(FamilyRequest::Route([4.0, 0.0, 0.0])));
        // Arrive -> idle, never completes.
        let step = driver.tick(&obs([4.0, 0.0, 0.0], true), 0.1);
        assert_eq!(
            step.request,
            Some(FamilyRequest::Play(FamilyAnimation::Idle))
        );
        assert_eq!(step.signal, LifecycleSignal::Continue);
        assert_eq!(driver.step_label(), "idling");
        // Idempotent play: no repeat request while already idling.
        let step = driver.tick(&obs([4.0, 0.0, 0.0], true), 0.1);
        assert_eq!(step.request, None);
        assert_eq!(step.signal, LifecycleSignal::Continue);
    }

    #[test]
    fn idle_carries_the_authored_orientation() {
        let driver = FamilyDriver::new(
            PackageFamily::Idle,
            vec![Waypoint {
                position: [0.0, 0.0, 0.0],
                wait_seconds: 0.0,
                orientation_yaw: Some(1.5),
                interaction_point: None,
            }],
            0.5,
        );
        assert_eq!(driver.current_orientation_yaw(), Some(1.5));
    }

    #[test]
    fn eat_routes_then_occupies_and_plays_eat() {
        let mut driver = FamilyDriver::new(
            PackageFamily::Eat,
            vec![Waypoint {
                position: [3.0, 0.0, 0.0],
                wait_seconds: 0.0,
                orientation_yaw: None,
                interaction_point: Some(0xF00D),
            }],
            0.5,
        );
        let step = driver.tick(&obs([0.0, 0.0, 0.0], false), 0.1);
        assert_eq!(step.request, Some(FamilyRequest::Route([3.0, 0.0, 0.0])));
        assert_eq!(driver.occupied_point(), None);
        // Arrive: claims the furniture and plays the eat state.
        let step = driver.tick(&obs([3.0, 0.0, 0.0], true), 0.1);
        assert_eq!(
            step.request,
            Some(FamilyRequest::Play(FamilyAnimation::Eat))
        );
        assert_eq!(step.signal, LifecycleSignal::Continue);
        assert_eq!(driver.occupied_point(), Some(0xF00D));
        assert_eq!(driver.step_label(), "occupying");
    }

    #[test]
    fn sleep_releases_its_occupancy_on_preempt() {
        let mut driver = FamilyDriver::new(
            PackageFamily::Sleep,
            vec![Waypoint {
                position: [0.0, 0.0, 0.0],
                wait_seconds: 0.0,
                orientation_yaw: None,
                interaction_point: Some(0xBED),
            }],
            0.5,
        );
        driver.tick(&obs([0.0, 0.0, 0.0], true), 0.1); // occupy
        assert_eq!(driver.occupied_point(), Some(0xBED));
        // Preempt/complete releases the claim and returns it for the registry.
        assert_eq!(driver.release(), Some(0xBED));
        assert_eq!(driver.occupied_point(), None);
        // Idempotent release.
        assert_eq!(driver.release(), None);
    }

    #[test]
    fn interaction_point_selection_picks_nearest_free() {
        let waypoints = vec![
            Waypoint {
                position: [10.0, 0.0, 0.0],
                wait_seconds: 0.0,
                orientation_yaw: None,
                interaction_point: Some(1),
            },
            Waypoint {
                position: [2.0, 0.0, 0.0],
                wait_seconds: 0.0,
                orientation_yaw: None,
                interaction_point: Some(2),
            },
        ];
        let free = HashSet::new();
        assert_eq!(
            select_interaction_point(&waypoints, [0.0, 0.0, 0.0], &free),
            Some(1),
            "nearest is index 1 (point 2)"
        );
        // With point 2 occupied, the farther free point 1 is chosen instead.
        let mut occupied = HashSet::new();
        occupied.insert(2u32);
        assert_eq!(
            select_interaction_point(&waypoints, [0.0, 0.0, 0.0], &occupied),
            Some(0)
        );
        // Everything occupied -> no free furniture.
        occupied.insert(1u32);
        assert_eq!(
            select_interaction_point(&waypoints, [0.0, 0.0, 0.0], &occupied),
            None
        );
    }
}
