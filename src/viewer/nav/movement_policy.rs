//! Pure std-only movement-policy decisions for the physics-authoritative
//! nav-agent capsule (issue #114, M4 wave 5): grounded/airborne
//! transitions, collision-rejection clamping (desired vs achieved
//! velocity), and stuck detection/recovery. `nav/agent.rs`'s Bevy systems
//! only feed observations sampled from the real `bevy_boxddd` KCC sweep
//! into these functions -- no decision-making lives in the Bevy system
//! itself. Mirrors `src/viewer/world/policy.rs`'s established pure
//! decision-table pattern: `std`-only (no `bevy`/`bevy_landmass` import) so
//! `tests/features.rs` can include it verbatim via `#[path]`.
//!
//! Progress signal choice (issue #157): `decide_stuck` (below) is a generic
//! "is this monotone metric still improving" decision table -- it was fed
//! monotone *distance-to-the-final-`AgentTarget3d`* through M4 wave 5/6, but
//! that measure regresses for whole detour legs on any valid route that must
//! initially move away from the final target (around a long wall, doubling
//! back through a doorway), false-triggering recovery/failure on a perfectly
//! healthy route. Of the candidate corridor-progress signals (distance to
//! the current landmass steering waypoint, decreasing remaining path
//! length, windowed net displacement), this module picks the simplest one
//! actually available without reaching into `bevy_landmass`'s archipelago
//! internals: `route_progress_delta` projects each tick's real, KCC-
//! resolved achieved horizontal velocity onto whatever direction landmass
//! is *currently* steering toward (its desired horizontal velocity this
//! tick) -- reusing exactly the `desired`/`achieved` pair
//! `decide_collision_outcome` already samples. `nav/agent.rs` integrates
//! this over time into a running total and negates it into the same
//! monotone-minimum-tracked "distance" shape `decide_stuck` already
//! expects, so the decision table itself needs no changes: a detour leg
//! keeps registering fresh progress every tick it is actually walking,
//! because the projection follows the corridor's current steering
//! direction, not the straight-line final target. A genuinely wedged agent
//! (KCC sweep achieves near-zero motion regardless of direction) still
//! flatlines this signal exactly as it flatlined the old one, so the
//! existing wedge/blocked recovery behaviour is unchanged.
//!
//! Known limitation (external architecture review, issue #157 follow-up):
//! because `route_progress_delta` only ever compares this tick's achieved
//! motion against *this same tick's* desired direction, an agent whose
//! desired direction keeps flipping under oscillating avoidance steering --
//! and which fully achieves each flip -- reads as perpetual corridor
//! progress here, even though its net position barely moves (it is
//! effectively orbiting in place). The old distance-to-final-target signal
//! would eventually have caught that case; this one does not. This is an
//! accepted trade-off, not an oversight: the field failure mode this signal
//! exists to catch is a genuine collision wedge (the KCC sweep achieves
//! near-zero motion regardless of the desired direction), and that case
//! still flatlines the signal exactly as before. Pinned by
//! `tests/features.rs`'s `nav_stuck_progress.feature` step section (the
//! oscillating-route scenario) rather than left implicit.

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

/// Horizontal (XZ-plane) distance between two points, ignoring Y entirely.
///
/// Regression fix (issue #114 added scope, M4 wave 5): every agent-vs-nav-
/// point comparison in `nav/agent.rs` (travel-door arrival, the #137
/// mid-route crossing gate, and stuck-vs-target progress) used to be a
/// plain 3D distance back when the wave-3/4 kinematic agent Y-snapped its
/// `Transform` onto the navmesh surface every tick. Physics-authoritative
/// movement removed that snap -- `Transform.translation` is now the capsule
/// *centre*, roughly half `AGENT_HEIGHT` above the feet-level points the
/// nav graph's door midpoints and route targets sit at -- so a 3D distance
/// against those points always overshoots by about that constant offset
/// and every proximity gate silently stopped firing. Nav is fundamentally
/// an on-surface concept (this also matches landmass's own
/// `TargetReachedCondition::Distance`, which is horizontal too, and is why
/// `AgentState::ReachedTarget` kept firing correctly through the same
/// regression window while these checks did not): every site below compares
/// on this plane instead of in 3D.
pub(crate) fn horizontal_distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dx = a[0] - b[0];
    let dz = a[2] - b[2];
    (dx * dx + dz * dz).sqrt()
}

/// Whether `agent` counts as having reached the horizontal-proximity nav
/// point `point` (a travel-door or mid-route-crossing midpoint): within
/// `radius` on the XZ plane, the same offset-tolerant frame
/// `horizontal_distance` documents, *and* within `max_vertical_gap` of it
/// vertically. The vertical guard is not about tolerating the agent's own
/// capsule-centre-vs-feet offset (the horizontal check already ignores Y
/// entirely) -- it rejects a different, unrelated point that merely shares
/// X/Z, such as a door on another floor directly above or below the one the
/// agent is actually walking on.
pub(crate) fn nav_point_reached(
    agent: [f32; 3],
    point: [f32; 3],
    radius: f32,
    max_vertical_gap: f32,
) -> bool {
    (agent[1] - point[1]).abs() <= max_vertical_gap && horizontal_distance(agent, point) <= radius
}

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

/// This tick's real (KCC-resolved) horizontal displacement, projected onto
/// whatever direction landmass is *currently* asking the agent to move in
/// (issue #157): positive when the achieved motion carries the agent
/// forward along its current steering direction, negative when collision or
/// avoidance pushes it backward along that same axis, and exactly `0.0`
/// when `desired` carries no horizontal motion at all -- there is nothing
/// to project onto, so this tick contributes neither progress nor regress
/// (mirrors `decide_collision_outcome`'s own "no desired motion is never
/// blocked" rule).
///
/// Both vectors are `[x, z]` horizontal-plane pairs, matching every other
/// site in this module (see `horizontal_distance`'s doc comment for why Y is
/// excluded). This is a rate (metres/second, the same units `desired` and
/// `achieved` already are as velocities) -- the caller integrates it by
/// `dt` into a running corridor-progress total, exactly like `achieved`
/// itself is already `(new_position - old_position) / dt` before being fed
/// back to landmass. Unlike distance-to-the-final-target, this keeps
/// registering progress on a route leg that must move *away* from the final
/// target (around a wall, through a doorway) as long as the agent is
/// actually walking the corridor landmass just steered it onto.
pub(crate) fn route_progress_delta(desired: [f32; 2], achieved: [f32; 2]) -> f32 {
    let desired_len = (desired[0] * desired[0] + desired[1] * desired[1]).sqrt();
    if desired_len <= f32::EPSILON {
        return 0.0;
    }
    let direction = [desired[0] / desired_len, desired[1] / desired_len];
    achieved[0] * direction[0] + achieved[1] * direction[1]
}

/// Observation feeding `decide_stuck`: a monotone progress metric this
/// module treats generically -- smaller is better, and the decision table
/// below only cares whether it keeps beating its own running minimum, never
/// what it physically measures. Through M4 wave 5/6 `nav/agent.rs` fed
/// literal horizontal distance to the final `AgentTarget3d`; since issue
/// #157 it feeds the negated, integrated `route_progress_delta` instead (see
/// this module's doc comment) so a detour leg that must move away from the
/// final target does not read as "no progress." `ticks_without_progress`
/// counts consecutive ticks with no new minimum, and `recovery_active`
/// reports whether a recovery attempt (e.g. a forced repath) is already in
/// flight.
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

/// Whether the agent counts as arrived and should have its stuck-detection
/// state fully reset this tick -- either its own recomputed horizontal
/// distance is within `reached_distance` of the target (the original
/// check), OR `bevy_landmass` itself already reports
/// `AgentState::ReachedTarget` (`landmass_reached`).
///
/// Regression fix (issue #136 follow-up): before navmesh erosion existed,
/// these two signals always agreed closely enough that only the distance
/// check was needed. Eroding the walkable boundary inward by the agent
/// radius moves the navmesh's constrained/sampled point for a raw
/// (un-sampled) `tna goto` target, so the literal requested coordinate's
/// nearest *reachable* point can end up farther than `reached_distance`
/// away even though the agent is genuinely as close as it can physically
/// get and landmass has already stopped issuing meaningful movement.
/// Without also trusting `landmass_reached`, `decide_stuck`'s no-progress
/// window would eventually (and incorrectly) latch `Stuck` on a route that
/// had already finished -- "reached" and "stuck" must stay mutually
/// exclusive outcomes.
pub(crate) fn arrival_resets_stuck(
    distance_to_target: f32,
    reached_distance: f32,
    landmass_reached: bool,
) -> bool {
    distance_to_target <= reached_distance || landmass_reached
}

#[cfg(test)]
#[path = "tests/movement_policy.rs"]
mod tests;
