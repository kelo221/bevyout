//! Pure AI package-family dispatch (issues #196 Travel+Patrol, #197
//! Idle+Eat+Sleep, #198 Follow+Sandbox).
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

/// The seven package families this driver drives.
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
    /// FO3 PACK type 1: maintain a follow-distance band from a *moving* leader,
    /// re-pathing as it moves. Hysteresis (an inner/outer band, not one
    /// threshold) keeps the follower from stuttering start/stop at the edge.
    /// A follow blocked by a door it cannot open names that door and abandons
    /// (verdict §4.5, #185) rather than re-pathing forever.
    Follow,
    /// FO3 PACK type 12 (Sandbox): bounded, deterministic random movement
    /// within a radius of the resolved package location, idling between legs.
    /// Unlike every other moving family, a sandboxing actor does **not** open
    /// doors (OpenMW `AiWander` parity -- gated in `nav::agent`).
    Sandbox,
}

impl PackageFamily {
    /// Maps a prepared `PKDT.type` onto its family, or `None` for a type this
    /// driver does not handle. The one dispatch entry point -- the same `u8`
    /// values `ai_package_commands::package_type_label` names.
    #[must_use]
    pub fn from_package_type(package_type: u8) -> Option<Self> {
        match package_type {
            1 => Some(Self::Follow),
            3 => Some(Self::Eat),
            4 => Some(Self::Sleep),
            5 => Some(Self::Idle),
            6 => Some(Self::Travel),
            12 => Some(Self::Sandbox),
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
            Self::Follow => "follow",
            Self::Sandbox => "sandbox",
        }
    }

    /// Whether an actor running this family may open doors in its path. Only
    /// [`Self::Sandbox`] (Wander) is refused -- the OpenMW `openDoors()`
    /// early-return for wander packages, wired at the nav door-open seam.
    #[must_use]
    pub const fn opens_doors(self) -> bool {
        !matches!(self, Self::Sandbox)
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
    /// The current world position of a follow family's leader, or `None` when
    /// the leader is gone (target loss). Ignored by the non-follow families.
    pub target_position: Option<[f32; 3]>,
    /// The FormID of a door the agent's route gave up on (#185's `Failed`
    /// door-link terminal naming the blocking door). A follow family treats
    /// this as "locked, cannot open" and abandons the attempt; the other
    /// families ignore it.
    pub blocking_door: Option<u32>,
}

impl FamilyObservation {
    /// A plain observation with no follow leader and no blocking door -- the
    /// shape the five scheduled families (Travel/Patrol/Idle/Eat/Sleep) read.
    #[must_use]
    pub fn new(actor_position: [f32; 3], nav_reached: bool, route_failed: bool) -> Self {
        Self {
            actor_position,
            nav_reached,
            route_failed,
            target_position: None,
            blocking_door: None,
        }
    }
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
    /// A follow family gave up on a door it cannot open (its FormID is on
    /// [`FamilyDriver::blocked_door`]).
    Blocked,
    Done,
}

impl Progress {
    const fn label(self) -> &'static str {
        match self {
            Self::Routing => "routing",
            Self::Waiting => "waiting",
            Self::Idling => "idling",
            Self::Occupying => "occupying",
            Self::Blocked => "blocked",
            Self::Done => "done",
        }
    }
}

/// A follow family's hysteresis band. The follower *closes* toward the leader
/// once it drifts past `outer`, and *holds* (idle) once it is back within
/// `inner`. Two distinct thresholds (`inner < outer`) are the hysteresis: a
/// single threshold would stutter start/stop as the leader jitters right at the
/// edge, so a follower that has started closing keeps closing until it is well
/// inside the band, and one that is holding stays put until the leader is well
/// outside it.
#[derive(Debug, Clone, Copy, PartialEq)]
struct FollowState {
    /// Hold radius (band minimum): stop closing once this near the leader.
    inner: f32,
    /// Chase radius (band maximum): start closing once this far from the leader.
    outer: f32,
    /// Re-issue the nav route toward the moving leader only after it has moved
    /// at least this far from the last routed point, so a follower does not
    /// spam `goto` every frame while the leader takes a single step.
    repath_epsilon: f32,
    /// The hysteresis latch: currently closing toward the leader (vs holding).
    closing: bool,
}

/// A sandbox/wander family's bounded, deterministic roaming state. Points are
/// drawn from a seeded PRNG so the same actor wanders the same sequence every
/// run (and across a save/load resume); every point lies within `radius` of
/// `center` on the ground plane.
#[derive(Debug, Clone, Copy, PartialEq)]
struct WanderState {
    center: [f32; 3],
    radius: f32,
    /// Seconds to idle at each reached point before roaming to the next.
    idle_seconds: f32,
    /// Deterministic PRNG state (xorshift64*); never zero.
    rng: u64,
    /// The point currently being roamed to, if one has been drawn yet.
    current_point: Option<[f32; 3]>,
    /// Currently idling at a reached point (vs en route to the next).
    idling: bool,
    idle_remaining: f32,
}

impl WanderState {
    fn new(center: [f32; 3], radius: f32, idle_seconds: f32, seed: u64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            idle_seconds: idle_seconds.max(0.0),
            // xorshift64* is stuck at zero; substitute a fixed nonzero seed.
            rng: if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            },
            current_point: None,
            idling: false,
            idle_remaining: 0.0,
        }
    }

    /// Advances the PRNG and returns the next draw in `[0, 1)`.
    fn next_unit(&mut self) -> f32 {
        // xorshift64* -- deterministic, no dependencies, good enough for a
        // spatial jitter that only needs to be reproducible and bounded.
        let mut x = self.rng;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.rng = x;
        let scrambled = x.wrapping_mul(0x2545_F491_4F6C_DD1D);
        // Top 24 bits -> a float in [0, 1); plenty of resolution for jitter.
        ((scrambled >> 40) as f32) / ((1u64 << 24) as f32)
    }

    /// Draws the next roam point: uniform within the disc of `radius` around
    /// `center` on the XZ ground plane (`center`'s Y is kept).
    fn next_point(&mut self) -> [f32; 3] {
        let angle = self.next_unit() * std::f32::consts::TAU;
        // sqrt of the radial draw makes the distribution uniform over area
        // rather than clustered at the centre.
        let distance = self.radius * self.next_unit().sqrt();
        [
            self.center[0] + distance * angle.cos(),
            self.center[1],
            self.center[2] + distance * angle.sin(),
        ]
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
    /// Follow-family band state; `Some` only for [`PackageFamily::Follow`].
    follow: Option<FollowState>,
    /// Sandbox-family roam state; `Some` only for [`PackageFamily::Sandbox`].
    wander: Option<WanderState>,
    /// The door a follow gave up on (its `Unreachable`-naming terminal), so the
    /// Bevy adapter can log the specific blocking door and the console can
    /// report it.
    blocked_door: Option<u32>,
}

impl FamilyDriver {
    /// Builds a driver for one of the five scheduled families (Travel, Patrol,
    /// Idle, Eat, Sleep) over `waypoints` (already in traversal order).
    /// `arrival_tolerance` is the horizontal-ish distance within which a
    /// waypoint counts as reached even before the nav agent's own
    /// reached-target latches. Use [`FamilyDriver::follow`] /
    /// [`FamilyDriver::wander`] for the dynamic families.
    #[must_use]
    pub fn new(family: PackageFamily, waypoints: Vec<Waypoint>, arrival_tolerance: f32) -> Self {
        Self::base(family, waypoints, arrival_tolerance)
    }

    fn base(family: PackageFamily, waypoints: Vec<Waypoint>, arrival_tolerance: f32) -> Self {
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
            follow: None,
            wander: None,
            blocked_door: None,
        }
    }

    /// Builds a [`PackageFamily::Follow`] driver maintaining the distance band
    /// `[band_min, band_max]` (metres) from the leader observed each tick via
    /// [`FamilyObservation::target_position`]. `band_min < band_max` is the
    /// hysteresis; the constructor sorts them so the caller cannot invert it.
    #[must_use]
    pub fn follow(band_min: f32, band_max: f32, arrival_tolerance: f32) -> Self {
        let lo = band_min.max(0.0).min(band_max.max(0.0));
        let hi = band_min.max(band_max).max(0.0);
        let mut driver = Self::base(PackageFamily::Follow, Vec::new(), arrival_tolerance);
        driver.progress = Progress::Idling;
        driver.follow = Some(FollowState {
            inner: lo,
            // Guarantee a nonzero dead band even if the caller passed equal
            // thresholds, so there is always hysteresis.
            outer: hi.max(lo + f32::EPSILON),
            repath_epsilon: arrival_tolerance.max(0.0).max(0.25),
            closing: false,
        });
        driver
    }

    /// Builds a [`PackageFamily::Sandbox`] driver roaming within `radius`
    /// metres of `center`, idling `idle_seconds` between legs. `seed` makes the
    /// roam sequence deterministic and reproducible.
    #[must_use]
    pub fn wander(
        center: [f32; 3],
        radius: f32,
        idle_seconds: f32,
        seed: u64,
        arrival_tolerance: f32,
    ) -> Self {
        let mut driver = Self::base(PackageFamily::Sandbox, Vec::new(), arrival_tolerance);
        driver.wander = Some(WanderState::new(center, radius, idle_seconds, seed));
        driver
    }

    #[must_use]
    pub fn family(&self) -> PackageFamily {
        self.family
    }

    /// The point the driver is currently working toward: a waypoint for the
    /// scheduled families, the last-routed leader point for follow, or the
    /// current roam point for sandbox.
    #[must_use]
    pub fn current_target(&self) -> Option<[f32; 3]> {
        match self.family {
            PackageFamily::Follow => self.routed_target,
            PackageFamily::Sandbox => self.wander.and_then(|w| w.current_point),
            _ => self.waypoints.get(self.index).map(|w| w.position),
        }
    }

    /// The door a follow family gave up on (its `Unreachable`-naming terminal),
    /// or `None`.
    #[must_use]
    pub fn blocked_door(&self) -> Option<u32> {
        self.blocked_door
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
        match self.family {
            // The dynamic families own their own route-failure handling (a
            // follow re-paths a moving leader; a sandbox re-rolls its point),
            // so they bypass the scheduled preamble below.
            PackageFamily::Follow => self.tick_follow(obs),
            PackageFamily::Sandbox => self.tick_wander(obs, dt),
            _ => self.tick_scheduled(obs, dt),
        }
    }

    /// The five schedule-driven families (Travel/Patrol/Idle/Eat/Sleep).
    fn tick_scheduled(&mut self, obs: &FamilyObservation, dt: f32) -> FamilyStep {
        // Route failure is shared across the scheduled families: re-issue up to
        // the ceiling, then fail the attempt (lifecycle owns backoff + retry).
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
            PackageFamily::Follow | PackageFamily::Sandbox => {
                unreachable!("dynamic families are handled in tick()")
            }
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

    /// Follow (#198): maintain the distance band from a moving leader.
    fn tick_follow(&mut self, obs: &FamilyObservation) -> FamilyStep {
        let mut follow = self.follow.expect("follow driver has follow config");

        // Named locked-door terminal (verdict §4.5, #185). A door the agent
        // could not open surfaces as `blocking_door`; a follow names it and
        // abandons the attempt (OpenMW would try a key -- key retrieval is a
        // #198 non-goal) instead of re-pathing at it forever. The lifecycle's
        // backoff/retry then decides whether to try again or give up.
        if let Some(door) = obs.blocking_door {
            self.blocked_door = Some(door);
            self.progress = Progress::Blocked;
            self.routed_target = None;
            follow.closing = false;
            self.follow = Some(follow);
            return FamilyStep::new(Some(FamilyRequest::Stop), LifecycleSignal::Fail);
        }
        self.blocked_door = None;

        // Target loss: the leader is gone. Stop steering toward its stale last
        // position and idle in place; the follow persists (Continue) and
        // resumes closing if the leader reappears.
        let Some(target) = obs.target_position else {
            follow.closing = false;
            self.progress = Progress::Idling;
            let request = if self.routed_target.take().is_some() {
                Some(FamilyRequest::Stop)
            } else {
                self.ensure_play(FamilyAnimation::Idle)
            };
            self.follow = Some(follow);
            return FamilyStep::new(request, LifecycleSignal::Continue);
        };

        let distance_sq = distance_squared(obs.actor_position, target);
        let request = if follow.closing {
            // Closing: keep chasing until well inside the inner band.
            if distance_sq <= follow.inner * follow.inner {
                follow.closing = false;
                self.progress = Progress::Idling;
                self.routed_target = None;
                Some(FamilyRequest::Stop)
            } else {
                self.progress = Progress::Routing;
                self.ensure_route_follow(target, follow.repath_epsilon)
            }
        } else {
            // Holding: only start closing once the leader drifts past `outer`.
            if distance_sq >= follow.outer * follow.outer {
                follow.closing = true;
                self.progress = Progress::Routing;
                self.ensure_route_follow(target, follow.repath_epsilon)
            } else {
                self.progress = Progress::Idling;
                self.ensure_play(FamilyAnimation::Idle)
            }
        };
        self.follow = Some(follow);
        // Follow never completes on its own -- the leader may always move again.
        FamilyStep::new(request, LifecycleSignal::Continue)
    }

    /// Re-routes toward a moving leader only after it has moved at least
    /// `epsilon` from the last routed point (idempotent otherwise), so a
    /// follower does not spam a fresh `goto` on every frame of leader motion.
    fn ensure_route_follow(&mut self, target: [f32; 3], epsilon: f32) -> Option<FamilyRequest> {
        let moved_enough = match self.routed_target {
            Some(previous) => distance_squared(previous, target) >= epsilon * epsilon,
            None => true,
        };
        if moved_enough {
            self.routed_target = Some(target);
            self.last_play = None;
            Some(FamilyRequest::Route(target))
        } else {
            None
        }
    }

    /// Sandbox/Wander (#198): bounded deterministic roaming with idle legs.
    fn tick_wander(&mut self, obs: &FamilyObservation, dt: f32) -> FamilyStep {
        let mut wander = self.wander.expect("sandbox driver has wander config");

        // A roam point that cannot be pathed is re-rolled rather than failing
        // the whole package; the shared ceiling still bounds a wholly off-mesh
        // area so the family gives up instead of spinning forever.
        if obs.route_failed {
            self.route_failures += 1;
            if self.route_failures > MAX_ROUTE_REISSUES {
                self.route_failures = 0;
                self.wander = Some(wander);
                return FamilyStep::new(None, LifecycleSignal::Fail);
            }
            let point = wander.next_point();
            wander.current_point = Some(point);
            wander.idling = false;
            self.routed_target = None;
            self.progress = Progress::Routing;
            let request = self.ensure_route(point);
            self.wander = Some(wander);
            return FamilyStep::new(request, LifecycleSignal::Continue);
        }
        self.route_failures = 0;

        // First tick: draw and route to the opening roam point.
        let Some(point) = wander.current_point else {
            let point = wander.next_point();
            wander.current_point = Some(point);
            self.progress = Progress::Routing;
            let request = self.ensure_route(point);
            self.wander = Some(wander);
            return FamilyStep::new(request, LifecycleSignal::Continue);
        };

        if wander.idling {
            wander.idle_remaining -= dt.max(0.0);
            if wander.idle_remaining <= 0.0 {
                wander.idling = false;
                let next = wander.next_point();
                wander.current_point = Some(next);
                self.routed_target = None;
                self.progress = Progress::Routing;
                let request = self.ensure_route(next);
                self.wander = Some(wander);
                // Each completed roam leg is a step boundary for the lifecycle.
                return FamilyStep::new(request, LifecycleSignal::AdvanceStep);
            }
            self.progress = Progress::Idling;
            let request = self.ensure_play(FamilyAnimation::Idle);
            self.wander = Some(wander);
            return FamilyStep::new(request, LifecycleSignal::Continue);
        }

        if self.arrived(obs, point) {
            wander.idling = true;
            wander.idle_remaining = wander.idle_seconds;
            self.progress = Progress::Idling;
            let request = self.ensure_play(FamilyAnimation::Idle);
            self.wander = Some(wander);
            return FamilyStep::new(request, LifecycleSignal::Continue);
        }
        self.progress = Progress::Routing;
        let request = self.ensure_route(point);
        self.wander = Some(wander);
        FamilyStep::new(request, LifecycleSignal::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obs(position: [f32; 3], nav_reached: bool) -> FamilyObservation {
        FamilyObservation::new(position, nav_reached, false)
    }

    fn failed_obs(position: [f32; 3]) -> FamilyObservation {
        FamilyObservation::new(position, false, true)
    }

    fn follow_obs(actor: [f32; 3], leader: [f32; 3]) -> FamilyObservation {
        FamilyObservation {
            target_position: Some(leader),
            ..FamilyObservation::new(actor, false, false)
        }
    }

    #[test]
    fn dispatch_maps_the_seven_supported_types() {
        assert_eq!(
            PackageFamily::from_package_type(1),
            Some(PackageFamily::Follow)
        );
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
            PackageFamily::from_package_type(12),
            Some(PackageFamily::Sandbox)
        );
        assert_eq!(
            PackageFamily::from_package_type(13),
            Some(PackageFamily::Patrol)
        );
        // An unmapped type is still not driven.
        assert_eq!(PackageFamily::from_package_type(9), None);
    }

    #[test]
    fn only_sandbox_refuses_to_open_doors() {
        assert!(!PackageFamily::Sandbox.opens_doors());
        for family in [
            PackageFamily::Travel,
            PackageFamily::Patrol,
            PackageFamily::Idle,
            PackageFamily::Eat,
            PackageFamily::Sleep,
            PackageFamily::Follow,
        ] {
            assert!(family.opens_doors(), "{} should open doors", family.label());
        }
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

    // -- Follow (#198) -------------------------------------------------------

    #[test]
    fn follow_closes_when_past_the_outer_band_and_re_paths_the_moving_leader() {
        // Band [2, 5]: hold within 2m, chase past 5m.
        let mut driver = FamilyDriver::follow(2.0, 5.0, 0.5);
        // Leader 8m away (> outer): start closing, route to it.
        let step = driver.tick(&follow_obs([0.0, 0.0, 0.0], [8.0, 0.0, 0.0]), 0.1);
        assert_eq!(step.request, Some(FamilyRequest::Route([8.0, 0.0, 0.0])));
        assert_eq!(step.signal, LifecycleSignal::Continue);
        assert_eq!(driver.step_label(), "routing");
        // Leader took a small step (< repath epsilon-ish): no fresh route spam.
        let step = driver.tick(&follow_obs([1.0, 0.0, 0.0], [8.1, 0.0, 0.0]), 0.1);
        assert_eq!(step.request, None);
        // Leader moved well away: re-path to the new leader point.
        let step = driver.tick(&follow_obs([2.0, 0.0, 0.0], [12.0, 0.0, 0.0]), 0.1);
        assert_eq!(step.request, Some(FamilyRequest::Route([12.0, 0.0, 0.0])));
    }

    #[test]
    fn follow_hysteresis_does_not_stutter_at_the_band_edge() {
        // Band [2, 5]. The follower must not flip start/stop while the leader
        // hovers *between* the thresholds (the dead band).
        let mut driver = FamilyDriver::follow(2.0, 5.0, 0.5);
        // Start closing (leader at 6m > outer).
        let start = driver.tick(&follow_obs([0.0, 0.0, 0.0], [6.0, 0.0, 0.0]), 0.1);
        assert!(matches!(start.request, Some(FamilyRequest::Route(_))));
        // Now actor is 3.5m from the leader: inside the dead band (2..5). A
        // single-threshold follower would already stop; hysteresis keeps it
        // closing (no Stop), so no stutter.
        let mid = driver.tick(&follow_obs([2.5, 0.0, 0.0], [6.0, 0.0, 0.0]), 0.1);
        assert_ne!(mid.request, Some(FamilyRequest::Stop));
        assert_eq!(driver.step_label(), "routing");
        // Reaches the inner band (1.5m <= 2): now it holds.
        let close = driver.tick(&follow_obs([4.5, 0.0, 0.0], [6.0, 0.0, 0.0]), 0.1);
        assert_eq!(close.request, Some(FamilyRequest::Stop));
        assert_eq!(driver.step_label(), "idling");
        // Leader drifts to 3.5m again (still inside the dead band): the holder
        // stays put -- does NOT resume closing. No stutter on the way back in.
        let hold = driver.tick(&follow_obs([4.5, 0.0, 0.0], [8.0, 0.0, 0.0]), 0.1);
        // 3.5m < outer(5): still holding, only an (idempotent) idle at most.
        assert_ne!(hold.request, Some(FamilyRequest::Route([8.0, 0.0, 0.0])));
    }

    #[test]
    fn follow_holds_and_idles_within_the_band() {
        let mut driver = FamilyDriver::follow(2.0, 5.0, 0.5);
        // Leader 3m away (inside band): hold, play idle, never route.
        let step = driver.tick(&follow_obs([0.0, 0.0, 0.0], [3.0, 0.0, 0.0]), 0.1);
        assert_eq!(
            step.request,
            Some(FamilyRequest::Play(FamilyAnimation::Idle))
        );
        assert_eq!(step.signal, LifecycleSignal::Continue);
        // Idempotent: no repeat idle while still holding.
        let step = driver.tick(&follow_obs([0.0, 0.0, 0.0], [3.0, 0.0, 0.0]), 0.1);
        assert_eq!(step.request, None);
    }

    #[test]
    fn follow_target_loss_stops_and_idles_without_completing() {
        let mut driver = FamilyDriver::follow(2.0, 5.0, 0.5);
        // Close toward a leader first so there is an active route to clear.
        driver.tick(&follow_obs([0.0, 0.0, 0.0], [9.0, 0.0, 0.0]), 0.1);
        // Leader vanishes: clear the route, then idle -- and never complete.
        let lost = FamilyObservation::new([1.0, 0.0, 0.0], false, false);
        let step = driver.tick(&lost, 0.1);
        assert_eq!(step.request, Some(FamilyRequest::Stop));
        assert_eq!(step.signal, LifecycleSignal::Continue);
        assert_eq!(driver.step_label(), "idling");
        let step = driver.tick(&lost, 0.1);
        assert_eq!(
            step.request,
            Some(FamilyRequest::Play(FamilyAnimation::Idle))
        );
        assert_eq!(step.signal, LifecycleSignal::Continue);
    }

    #[test]
    fn follow_blocked_by_a_locked_door_names_it_and_abandons() {
        let mut driver = FamilyDriver::follow(2.0, 5.0, 0.5);
        driver.tick(&follow_obs([0.0, 0.0, 0.0], [20.0, 0.0, 0.0]), 0.1);
        // The route gave up on a locked door: name it, stop, fail the attempt.
        let blocked = FamilyObservation {
            route_failed: true,
            blocking_door: Some(0x0001_ABCD),
            ..follow_obs([5.0, 0.0, 0.0], [20.0, 0.0, 0.0])
        };
        let step = driver.tick(&blocked, 0.1);
        assert_eq!(step.request, Some(FamilyRequest::Stop));
        assert_eq!(step.signal, LifecycleSignal::Fail);
        assert_eq!(driver.blocked_door(), Some(0x0001_ABCD));
        assert_eq!(driver.step_label(), "blocked");
    }

    #[test]
    fn follow_plain_no_path_keeps_trying_the_leader_not_the_door_terminal() {
        // A no-path *without* a blocking door is not the named-door terminal:
        // the follow keeps re-pathing the leader (it may just have stepped
        // off-mesh for a frame), so it does NOT report a blocked door.
        let mut driver = FamilyDriver::follow(2.0, 5.0, 0.5);
        let failing = FamilyObservation {
            route_failed: true,
            blocking_door: None,
            ..follow_obs([0.0, 0.0, 0.0], [9.0, 0.0, 0.0])
        };
        let step = driver.tick(&failing, 0.1);
        assert!(matches!(step.request, Some(FamilyRequest::Route(_))));
        assert_eq!(step.signal, LifecycleSignal::Continue);
        assert_eq!(driver.blocked_door(), None);
    }

    // -- Sandbox / Wander (#198) --------------------------------------------

    #[test]
    fn wander_routes_within_the_radius_and_idles_between_legs() {
        let center = [10.0, 0.0, 10.0];
        let radius = 6.0;
        let mut driver = FamilyDriver::wander(center, radius, 2.0, 0x00C0_FFEE, 0.5);
        // First tick draws a roam point and routes to it -- within the radius.
        let step = driver.tick(&FamilyObservation::new(center, false, false), 0.1);
        let Some(FamilyRequest::Route(point)) = step.request else {
            panic!("expected a roam route, got {:?}", step.request);
        };
        let dx = point[0] - center[0];
        let dz = point[2] - center[2];
        assert!(
            (dx * dx + dz * dz).sqrt() <= radius + 1e-3,
            "roam point {point:?} escaped the radius"
        );
        assert_eq!(point[1], center[1], "roam stays on the ground plane");
        // Arrive: idle in place for the dwell, no immediate re-route.
        let step = driver.tick(&FamilyObservation::new(point, true, false), 0.1);
        assert_eq!(
            step.request,
            Some(FamilyRequest::Play(FamilyAnimation::Idle))
        );
        assert_eq!(driver.step_label(), "idling");
        // Dwell elapses: roam to the next point (a step boundary).
        let step = driver.tick(&FamilyObservation::new(point, true, false), 5.0);
        assert!(matches!(step.request, Some(FamilyRequest::Route(_))));
        assert_eq!(step.signal, LifecycleSignal::AdvanceStep);
    }

    #[test]
    fn wander_point_selection_is_deterministic_for_a_seed() {
        let sequence = |seed: u64| -> Vec<[f32; 3]> {
            let mut state = WanderState::new([0.0, 0.0, 0.0], 5.0, 0.0, seed);
            (0..8).map(|_| state.next_point()).collect()
        };
        // Same seed -> identical sequence (reproducible across runs/resumes).
        assert_eq!(sequence(0xABCD), sequence(0xABCD));
        // Different seeds diverge (not a constant generator).
        assert_ne!(sequence(0xABCD), sequence(0x1234));
    }

    #[test]
    fn wander_points_are_always_bounded_by_the_radius() {
        let radius = 4.0;
        let mut state = WanderState::new([1.0, 2.0, 3.0], radius, 0.0, 42);
        for _ in 0..2000 {
            let point = state.next_point();
            let dx = point[0] - 1.0;
            let dz = point[2] - 3.0;
            assert!((dx * dx + dz * dz).sqrt() <= radius + 1e-3);
            assert_eq!(point[1], 2.0);
        }
    }

    #[test]
    fn wander_re_rolls_an_unreachable_point_then_fails_after_the_ceiling() {
        let mut driver = FamilyDriver::wander([0.0, 0.0, 0.0], 5.0, 1.0, 7, 0.5);
        driver.tick(&FamilyObservation::new([0.0, 0.0, 0.0], false, false), 0.1);
        // Each failure below the ceiling re-rolls to a fresh routed point.
        for _ in 0..MAX_ROUTE_REISSUES {
            let step = driver.tick(&failed_obs([0.0, 0.0, 0.0]), 0.1);
            assert!(matches!(step.request, Some(FamilyRequest::Route(_))));
            assert_eq!(step.signal, LifecycleSignal::Continue);
        }
        // One past the ceiling gives up the attempt.
        let step = driver.tick(&failed_obs([0.0, 0.0, 0.0]), 0.1);
        assert_eq!(step.signal, LifecycleSignal::Fail);
    }
}
