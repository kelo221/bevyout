//! `bevy_landmass` runtime plugin + `tna` (test nav agent) console command
//! family (issue #112, M4 wave 3; reworked #114, M4 wave 5). Owns one
//! `Archipelago3d` per active cell (one `Island3dBundle` per prepared nav
//! mesh within it, one `AnimationLink3dBundle` per intra-cell door-link
//! descriptor), built lazily from `PreparedSceneManifest::nav_graph` on the
//! first `tna spawn`, torn down on cell swap.
//!
//! Movement is physics-authoritative (#114): each nav agent carries its own
//! `bevy_boxddd` capsule KCC, mirroring `player/movement.rs`'s controller
//! (same free `move_mover`/`try_step_up`/`try_step_down`/
//! `try_forward_step_support` sweep functions, same collision filters). The
//! landmass-computed desired velocity is the KCC's *input*; the KCC resolves
//! collision/steps/slopes/gravity and moves the `Transform`, and the actual
//! post-collision velocity is what gets fed back to landmass's
//! `Velocity3d` -- navigation proposes, physics disposes. The navmesh
//! `sample_point` Y-snap from wave 3's kinematic spike is gone; landmass's
//! own `AgentState::AgentNotOnNavMesh` is the off-navmesh diagnostic now.
//! Deterministic grounded/collision/stuck decisions live in the pure
//! `movement_policy` module; this file only feeds it observations.
//!
//! Up to [`MAX_TEST_AGENTS`] agents can be spawned at once (bounded local
//! avoidance is otherwise unobservable with a single test agent): `tna`
//! subcommands take an optional leading agent index, with every
//! previously-single-agent command form left unchanged and defaulting to
//! agent 0.
//!
//! Mid-route door gating (issue #137): a closed door does not always sit at
//! a #113 link boundary. Real FO3 doors are single-sided NAVM triangles
//! (`landmass_graph::single_sided_doors`) -- **every real door triangle
//! sampled across the prepared catalog resolves to a travel destination**
//! (an earlier revision of this module gated only the non-travel subset,
//! which is empty on real data and so never fired at all; see the git
//! history for that dead end). Several travel-door triangles also have 2-3
//! walkable neighbors, so the mesh continues right through them: an
//! ordinary `tna goto` past a closed travel door crosses its triangle as
//! plain ground, with nothing pausing the agent for the door to open.
//! Two mechanisms were weighed for gating that crossing:
//!
//! - **Off-mesh links across the door triangle** (excluding the polygon and
//!   bridging it with an `AnimationLink3d`, the same shape the two-sided
//!   #113 door links use): rejected. That shape needs two *distinct*
//!   geometric points, one per side of the doorway, for landmass to resolve
//!   the link as a bridge between the two now-disconnected regions.
//!   `DoorLinkSide` only carries the door triangle's centroid (no per-edge
//!   door association exists, per its own doc comment) -- there is no
//!   "other side" point to link to without new edge-adjacency geometry, and
//!   excluding the polygon without a working bridge would just replace
//!   "clips through" with "always unreachable", locked or not.
//! - **Route-crossing proximity check** (the one implemented): every
//!   single-sided door's triangle -- travel-door candidate or not -- stays
//!   walkable at all times and is a crossing-gate candidate
//!   (`NavArchipelagoState::mid_route_doors`), exactly mirroring how
//!   `TRAVEL_ARRIVAL_DISTANCE` already gates travel-door *arrival*.
//!   Proximity to a candidate's triangle midpoint, checked inside
//!   `drive_door_link_for_agent`'s existing `Idle`/`Failed`/`TravelReached`
//!   arm right after the travel-arrival check, fires the *same*
//!   `DoorLinkEvent::LinkReached` the off-mesh link case fires -- but
//!   always with an `IntraCell` destination, never `Travel`: crossing a
//!   travel door's triangle mid-route (a `goto`, or a `tna travel` to a
//!   *different* door) is not the agent's own travel terminal, so it must
//!   not hand off to another cell. The one door a candidate is deliberately
//!   *not* gated for is the agent's own `travel_intent` target -- the
//!   travel-arrival check just above already owns that door's full
//!   pause -> open -> traverse -> `TravelReached` -> handoff lifecycle, and
//!   double-gating the same door for the same agent would fight it.
//!   A closed-unlocked crossing resolves through the ordinary
//!   `Paused` -> `Traversing` -> `Idle` path; a locked one never opens
//!   through the scripted boundary and reaches the *existing* `Failed`
//!   terminal via `MAX_WAIT_TICKS`, deterministically stopping the agent
//!   instead of letting it clip through. No off-mesh gap exists to cross,
//!   so resuming completes the crossing in the same tick instead of
//!   spawning a `DoorTraversal` (see the `Paused` arm's `None` case).
//!   Reuses `door_link.rs`'s FSM and `door_availability_system`'s existing
//!   generic per-door-form-ID tracking unchanged -- crossing-gate
//!   candidates are just more entries in the same `door_usable`/
//!   `door_lock_info` maps two-sided and travel doors already populate, so
//!   an unlock flips usability and triggers the same one repath (target
//!   re-insertion, and a `request_door_open` retry for any agent paused on
//!   that exact door) with zero new structural-update code.
//!
//! Wave 5 added scope (movement fidelity, user-directed): three changes to
//! the same seam, landed together.
//!
//! 1. **Fixed-timestep movement.** `apply_agent_physics_movement` and every
//!    per-agent system that must stay in lockstep with it (door-link
//!    lifecycle, traversal lerp, availability polling, diagnostic logging)
//!    moved from the variable-rate `Update` schedule to `FixedUpdate`,
//!    driven by the same `Time<Fixed>` clock the player KCC and
//!    `bevy_landmass` itself already use -- the whole chain keeps its
//!    original relative ordering (`despawn_stale_navmesh_archipelago` ->
//!    `restore_ledgered_agents_system` -> `door_availability_system` ->
//!    `door_link_system` -> `apply_agent_physics_movement` ->
//!    `door_traversal_system` -> `log_agent_state_changes` ->
//!    `log_path_latency`), just under a deterministic fixed cadence instead
//!    of one that varies with render frame time. Landmass's own systems stay
//!    in `FixedPreUpdate` (`Landmass3dPlugin`'s default schedule), which
//!    always runs before `FixedUpdate` within the same fixed tick, so
//!    `AgentDesiredVelocity3d` is fresh by the time movement reads it.
//! 2. **Player as a landmass `Character3d`.** A non-agent RVO obstacle
//!    (`spawn_player_nav_character`) mirrors the FPS player's position and
//!    *actual* post-collision KCC velocity every fixed tick
//!    (`sync_player_nav_character`, running before `LandmassSystems::
//!    SyncValues` reads it), so nav agents predict and avoid the moving
//!    player the same way they already avoid each other. It lives exactly
//!    as long as its archipelago -- spawned alongside the islands in
//!    `ensure_archipelago`, torn down with everything else in
//!    `teardown_archipelago` -- so a cell swap re-associates it with the
//!    freshly rebuilt archipelago the same way agents themselves do, rather
//!    than needing a separate lifecycle. Physics colliders remain the hard
//!    collision backstop; this is soft steering only.
//! 3. **Configurable nav-solve interval.** `NavSolveRate` (console-settable
//!    via `tna solverate [<n>]`) gates `LandmassSystems::Update` (the
//!    pathfinding+avoidance solve, the expensive part) behind
//!    `movement_policy::should_solve` and a per-tick step counter
//!    (`NavSolveStepCounter`, `advance_nav_solve_step_counter`), so the
//!    solve can run less often than every fixed tick while movement itself
//!    still runs every tick, integrating whichever desired velocity the
//!    last solve produced.
//!
//! Cross-mesh portal traversal (issue #154, M4 wave 8): a merge-seam
//! crossing used to reuse the exact same `DoorTraversal` component/lerp a
//! door-link crossing does (0.6 s fixed transform interpolation regardless
//! of what, if anything, sits between the two portal points -- a real
//! collision-blocked review finding). It is now its own `MergeTraversal`
//! component/`merge_traversal_system`, swept with the same physics KCC
//! (`step_agent_kcc`) ordinary movement uses: a portal that turns out to be
//! blocked by collision reports the same stable `nav agent
//! collision-blocked <id>`/`nav agent stuck <id>` lines ordinary blocked/
//! stuck movement already does, instead of always completing. Door-link
//! traversal (`DoorTraversal`/`door_traversal_system`) is unchanged --
//! this issue owns merge-link traversal only, not the door pause -> open ->
//! traverse lifecycle. The prepared merge data itself also changed shape
//! this issue: `vsa::prepare::nav_graph::compute_mesh_merges` now validates
//! each cross-mesh boundary-edge candidate (mutual-nearest correspondence,
//! opposing directions, an overlapping interval, step-height clearance)
//! before it becomes a `PreparedNavMeshMerge`, which carries the matched
//! edges' vertex-index identity and a clamped world-space portal interval
//! on both sides -- `landmass_graph::merge_link_descriptors` links the two
//! sides' interval midpoints (not triangle centroids) with a real
//! traversal-distance `AnimationLink3d` cost (`spawn_link_pair`'s new
//! `cost` parameter), in place of the previous flat `1.0` every link used.
use std::collections::HashMap;
use std::sync::Arc;

use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_boxddd::boxddd;
use bevy_boxddd::prelude::BoxdddPhysicsContext;
use bevy_landmass::coords::ThreeD;
use bevy_landmass::prelude::*;
use bevy_landmass::{
    NavMeshHandle, PauseAgent, PointSampleDistance3d, TargetReachedCondition, UsingAnimationLink,
};
use serde_json::json;

use crate::console::{ConsoleCommandResult, ConsoleError, ConsoleInvocation};
#[cfg(test)]
use crate::vsa::PreparedSceneManifest;

use super::super::openmw_player::GRAVITY;
use super::super::player::{CellPhysicsReadiness, PhysicsDisabled};
use super::super::{interaction, player};
use super::{door_link, landmass_graph, ledger_policy, movement_policy, repath};

const AGENT_RADIUS: f32 = 0.35;
const AGENT_HEIGHT: f32 = 1.8;
const AGENT_DESIRED_SPEED: f32 = 2.5;
const AGENT_MAX_SPEED: f32 = 3.5;
const AGENT_TARGET_REACHED_DISTANCE: f32 = 0.5;
/// Fixed kinematic crossing duration for a door-link traversal (spike
/// simplification -- #113 can derive this from the link's real length and
/// the agent's desired speed instead).
const DOOR_TRAVERSAL_SECONDS: f32 = 0.6;
/// How close (metres, horizontal -- see `movement_policy::nav_point_reached`)
/// the agent must get to a travel door's triangle midpoint before the door
/// lifecycle starts (issue #113 feature 3). Slightly wider than
/// `AGENT_TARGET_REACHED_DISTANCE` so landmass's own target-reached stop
/// always lands inside it.
const TRAVEL_ARRIVAL_DISTANCE: f32 = 0.75;
/// How close (metres, horizontal) the agent must get to a mid-route door's
/// triangle midpoint before the crossing gate (issue #137) evaluates it.
/// Same value and rationale as `TRAVEL_ARRIVAL_DISTANCE`, kept as its own
/// named constant since the two triggers are conceptually distinct (an
/// arrival at a routing target vs. a crossing check along an otherwise-
/// uninterrupted route).
const MID_ROUTE_DOOR_GATE_DISTANCE: f32 = 0.75;
/// How close (metres, horizontal) a swept merge-portal crossing (issue
/// #154 feature 4) must get to its far portal point before it counts as
/// complete. Same value/rationale as `AGENT_TARGET_REACHED_DISTANCE`.
const MERGE_TRAVERSAL_REACHED_DISTANCE: f32 = 0.5;
/// Multiplier applied to a swept merge-portal crossing's straight-line
/// time-at-desired-speed to get its timeout budget (issue #154 feature 4).
/// An *absolute wall-clock deadline* rather than ordinary movement's
/// resettable "ticks since last measurable progress" window
/// (`movement_policy::decide_stuck`): a capsule wedged against a wall can
/// keep creeping forward by an amount just under
/// `movement_policy::STUCK_PROGRESS_EPSILON` every tick indefinitely
/// without any no-progress counter ever latching (observed while writing
/// this traversal's own tests), so "genuinely blocked" here means "took
/// far longer than a clear crossing plausibly would", not "stopped making
/// any measurable progress at all". Generous slack for landmass/avoidance
/// steering to not follow a perfectly straight line.
const MERGE_TRAVERSAL_TIMEOUT_FACTOR: f32 = 4.0;
/// Fixed floor (seconds) added to the computed timeout (issue #154 feature
/// 4) so a very short crossing still gets a sane minimum window instead of
/// a near-zero deadline.
const MERGE_TRAVERSAL_TIMEOUT_FLOOR_SECONDS: f32 = 1.0;

/// The point-sampling envelope for the archipelago options
/// (`ensure_archipelago`): how far landmass itself may look for the navmesh
/// around an agent/target point when deciding on/off-mesh state
/// (`AgentState::AgentNotOnNavMesh`) -- the off-navmesh diagnostic input
/// now that physics (#114), not this sampling, is ground authority.
///
/// `from_agent_radius(0.35)` alone gives a 0.07 m horizontal / 0.35 m
/// below sampling envelope -- far too tight for FO3 data, where the NAVM
/// surface sits below the placed feet position and stairs/slopes put the
/// agent well above the polygon plane (the landmass FAQ's
/// vertical-sampling guidance; confirmed empirically: the default envelope
/// reports `AgentNotOnNavMesh` for an agent standing on the
/// MegatonPlayerHouse mesh). These are humanoid-scale distances instead.
const AGENT_POINT_SAMPLE_DISTANCE: PointSampleDistance3d = PointSampleDistance3d {
    horizontal_distance: 1.0,
    distance_above: 1.0,
    distance_below: 2.0,
    vertical_preference_ratio: 2.0,
    animation_link_max_vertical_distance: 1.0,
};

/// Bounded multi-agent cap (issue #114 feature 4): small and fixed so local
/// avoidance among same-cell test agents is observable without an
/// unbounded actor budget. Every previously single-agent `tna` command form
/// still works unchanged, addressing agent index 0.
pub(crate) const MAX_TEST_AGENTS: usize = 4;

/// The ledger/tracing identity for agent `index` (`0..MAX_TEST_AGENTS`):
/// stable, 1-based so it never collides with the "no id" sentinel `0`,
/// consistent with wave 3/4's single `TEST_AGENT_ID = 1`. Formatted as a
/// small decimal in tracing lines (it identifies a spawn slot, not a
/// FormID), but still handed to `ledger_policy` as a plain `u32`.
fn agent_ledger_id(index: usize) -> u32 {
    index as u32 + 1
}

/// Marks a test nav agent this console command family drives. `Entity`
/// identity plus `TestNavAgentState::index_of` recovers which of the
/// bounded `MAX_TEST_AGENTS` slots an entity belongs to.
#[derive(Component)]
struct TestNavAgentMarker;

/// Present on the agent entity while it is kinematically crossing a
/// door-link edge (`start` -> `end`), holding `apply_agent_physics_movement`
/// off the transform until the crossing completes.
#[derive(Component)]
struct DoorTraversal {
    start: Vec3,
    end: Vec3,
    elapsed: f32,
}

/// Present on the agent entity while it is physically sweeping across a
/// same-cell cross-mesh merge portal (issue #154 feature 4): unlike
/// [`DoorTraversal`] (a scripted lerp across a door's off-mesh gap, driven
/// by the door lifecycle FSM waiting on the door to open), a merge crossing
/// has nothing to wait on -- there is no door -- so it is instead swept
/// toward the far portal point with the same physics KCC ordinary movement
/// uses (`step_agent_kcc`, driven by `merge_traversal_system`), so a portal
/// whose far side is actually blocked by collision stops the agent for
/// real instead of always completing over a fixed window regardless of what
/// is in the way.
#[derive(Component)]
struct MergeTraversal {
    target: Vec3,
    /// Seconds elapsed since this crossing started.
    elapsed: f32,
    /// Absolute wall-clock deadline (seconds): computed once at traversal
    /// start from the *initial* straight-line distance to `target`, not
    /// recomputed per tick. See [`MERGE_TRAVERSAL_TIMEOUT_FACTOR`]'s doc
    /// comment for why this is a fixed deadline rather than a resettable
    /// no-progress counter (`AgentKcc::best_distance`/
    /// `ticks_without_progress`, owned by the #157 stuck-progress issue and
    /// unsuitable here regardless -- a portal crossing is a distinct,
    /// much shorter-lived motion regime from ordinary route following).
    timeout: f32,
}

/// Per-agent physics-authoritative KCC state (issue #114): the capsule
/// mover's own velocity (landmass's desired velocity is only ever this
/// tick's *input*), grounded state, and the deterministic stuck-tracking
/// counters `movement_policy::decide_stuck` consumes. One per agent entity,
/// inserted at spawn (`spawn_test_agent`) alongside `TestNavAgentMarker`.
#[derive(Component, Default, Clone, Copy)]
struct AgentKcc {
    velocity: Vec3,
    grounded: bool,
    /// Smallest `movement_policy::StuckObservation::distance_to_target`
    /// observed so far along the current route (reset to `f32::MAX`
    /// whenever a new `tna goto`/`tna travel` target is set, or to the
    /// current value on arrival -- see `apply_agent_physics_movement`).
    /// Since issue #157 this is the negated `route_progress` below, not a
    /// literal distance to the final target; see `movement_policy`'s module
    /// doc comment for why.
    best_distance: f32,
    /// Running integral of `movement_policy::route_progress_delta` over the
    /// whole route (issue #157): metres of real, KCC-resolved motion
    /// achieved along whatever direction landmass was steering toward at
    /// the time, accumulated tick over tick. Never reset on its own --
    /// `best_distance`'s own reset-to-`f32::MAX` handling re-baselines every
    /// comparison to "progress since then" regardless of this field's
    /// absolute running total, so a fresh target does not need a fresh
    /// zero here.
    route_progress: f32,
    ticks_without_progress: u32,
    recovery_active: bool,
    /// This tick's `movement_policy::decide_collision_outcome` classification
    /// (`tna status`'s `blocked=` field; the stable `nav agent
    /// collision-blocked <id>` line fires on the rising edge only).
    collision_blocked: bool,
    /// Latched by `movement_policy::decide_stuck`'s `Stuck` outcome; cleared
    /// by the next `tna goto`/`tna travel` (`tna status`'s `stuck=` field;
    /// the stable `nav agent stuck <id>` line fires on the rising edge).
    stuck: bool,
}

/// The previous and latest solved desired velocities landmass has produced
/// for this agent (issue #114 added scope, wave 5: configurable nav-solve
/// interval). `update_agent_desired_velocity_blend` shifts this pair
/// (`previous <- old latest`, `latest <- this tick's `AgentDesiredVelocity3d`)
/// only on a fixed tick that actually solved (`movement_policy::
/// should_solve`); `apply_agent_physics_movement` blends between them every
/// tick via `movement_policy::solve_blend_fraction` rather than reading
/// `AgentDesiredVelocity3d` directly, so a throttled solve rate (`tna
/// solverate`) produces a continuously sliding steering direction instead of
/// a value that snaps every `interval` ticks. Both default to zero, the same
/// value a freshly spawned `AgentDesiredVelocity3d` starts at, so an agent
/// with no solve yet simply has no desired motion either way.
#[derive(Component, Default, Clone, Copy)]
struct AgentDesiredVelocityBlend {
    previous: Vec3,
    latest: Vec3,
}

/// What an off-mesh animation-link entity represents (issue #113): a
/// same-cell cross-mesh merge seam (always open, crossed without any door
/// interaction) or an intra-cell two-sided door link (wave 3's pause ->
/// open -> traverse lifecycle).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LinkKind {
    Merge,
    Door { form_id: u32 },
}

/// A two-sided intra-cell door link currently excluded from route planning
/// because its door is locked (issue #113 feature 3: "blocked until
/// usable"). The geometry is retained so `door_availability_system` can
/// spawn the real animation link the moment the door becomes usable.
#[derive(Debug, Clone, Copy)]
struct BlockedDoorLink {
    door_form_id: u32,
    start: Vec3,
    end: Vec3,
}

/// A travel door reachable from this cell's nav mesh (issue #113 feature
/// 3): its single-sided triangle midpoint (the routing target), the door
/// placement's own position (the traversal end point -- the agent walks
/// *to* the door, never through into the unloaded destination cell), and
/// the destination cell the existing world-transition metadata (#51/#52)
/// resolves it to.
#[derive(Debug, Clone, Copy)]
struct TravelDoorLink {
    triangle_midpoint: Vec3,
    door_position: Vec3,
    destination_cell_form_id: u32,
    /// The door reference FormID in the destination cell this travel door
    /// pairs with (issue #134): the ledger's `DoorMarker` spawn kind
    /// resolves the agent's restore position from this door's own placed
    /// position once the destination cell is active.
    destination_door_form_id: u32,
}

/// One archipelago + its islands/links for the currently loaded cell,
/// built lazily by `ensure_archipelago` and torn down by
/// `despawn_stale_navmesh_archipelago` on cell swap (mirrors
/// `nav_overlay::despawn_stale_nav_overlay`'s pattern).
#[derive(Resource, Default)]
struct NavArchipelagoState {
    cell_form_id: Option<u32>,
    archipelago: Option<Entity>,
    /// The landmass `Character3d` mirroring the FPS player (issue #114
    /// added scope, wave 5): a non-agent RVO obstacle agents steer around
    /// but that landmass never moves. Lives exactly as long as
    /// `archipelago` -- spawned alongside it in `ensure_archipelago`,
    /// despawned with everything else in `teardown_archipelago` -- so a
    /// cell swap re-associates it with the freshly rebuilt archipelago the
    /// same way agents themselves do.
    player_character: Option<Entity>,
    islands: Vec<Entity>,
    links: Vec<Entity>,
    /// Animation-link entity -> what it represents, so `door_link_system`
    /// can map a `ReachedAnimationLink3d.link_entity` back to either a door
    /// reference to activate or a merge seam to cross directly.
    link_kinds: HashMap<Entity, LinkKind>,
    /// Two-sided door links currently excluded as blocked (locked door).
    blocked_door_links: Vec<BlockedDoorLink>,
    /// Door reference FormID -> terminal travel-link data.
    travel_doors: HashMap<u32, TravelDoorLink>,
    /// Every single-sided door's triangle (issue #137) -- a crossing-gate
    /// candidate regardless of whether it also resolves to a travel
    /// destination (real data: nearly all of them do; see the module doc),
    /// ordered the same way `landmass_graph::single_sided_doors` returns
    /// them (deterministic). `drive_door_link_for_agent` excludes an
    /// agent's own active `travel_intent` door from this set at check time
    /// -- that one door stays owned by the travel-arrival lifecycle.
    mid_route_doors: Vec<MidRouteDoor>,
    /// Last observed per-door usability (open, or not locked), for
    /// `door_availability_system`'s change detection -- exactly one repath
    /// per actual flip.
    door_usable: HashMap<u32, bool>,
    /// Doors' prepared lock/key data + placement entity resolution inputs,
    /// captured from the manifest at build time so the availability poll
    /// does not re-borrow the manifest every frame.
    door_lock_info: HashMap<u32, DoorLockInfo>,
}

#[derive(Debug, Clone, Copy)]
struct DoorLockInfo {
    lock_level: Option<i8>,
    key_form_id: Option<u32>,
}

/// A door crossable mid-route (issue #137): any single-sided door
/// triangle, travel-door candidate or not -- real FO3 data shows nearly
/// every door resolves to a travel destination, so restricting this set to
/// non-travel doors left it empty and never gated anything. Left part of
/// the walkable island (see `nav/agent.rs`'s module doc for why); gated at
/// runtime by proximity to `midpoint`, the same way `TravelDoorLink`'s own
/// arrival check works, *except* for the one door a given agent's own
/// `travel_intent` currently targets.
#[derive(Debug, Clone, Copy)]
struct MidRouteDoor {
    door_form_id: u32,
    midpoint: Vec3,
}

/// Per-agent bookkeeping that used to live in the single-agent
/// `TestNavAgentState` (waves 3/4), now a `Component` on each agent entity
/// so `MAX_TEST_AGENTS` agents can each carry their own door-link/travel/
/// diagnostics state without a parallel resource-side index.
#[derive(Component, Default)]
struct AgentRuntime {
    door_link: door_link::DoorLinkState,
    /// Set by `door_link_system` when a link is first reached, consumed by
    /// the same system once the door opens to start the `DoorTraversal`.
    pending_traversal: Option<(Vec3, Vec3)>,
    /// The link the agent is currently interacting with (for `tna status`'s
    /// `link=` report and for `door_traversal_system` to know whether a
    /// finished crossing should drive the door state machine).
    active_link: Option<LinkKind>,
    /// A pending travel-door route (issue #113 feature 3): the agent is
    /// heading to this door's triangle; arrival starts the door lifecycle
    /// with a `Travel` destination. Consumed by #134's `tna travel`.
    travel_intent: Option<u32>,
    /// `Time::elapsed_secs()` when the last `tna goto` ran, for the
    /// path-latency log line.
    goto_started_at: Option<f32>,
    latency_logged: bool,
    /// Last `AgentState` `log_agent_state_changes` reported, so the stable
    /// evidence lines fire once per actual change instead of every frame.
    last_logged_state: Option<AgentState>,
}

/// The bounded roster of spawned test-agent entities, indexed by agent
/// index (`0..MAX_TEST_AGENTS`). All other per-agent state
/// (`AgentRuntime`, `AgentKcc`, door-link/traversal components) lives on
/// the entity itself; this resource only answers "which entity is agent
/// N" and its inverse.
#[derive(Resource)]
struct TestNavAgentState {
    entities: [Option<Entity>; MAX_TEST_AGENTS],
}

impl Default for TestNavAgentState {
    fn default() -> Self {
        Self {
            entities: [None; MAX_TEST_AGENTS],
        }
    }
}

impl TestNavAgentState {
    fn index_of(&self, entity: Entity) -> Option<usize> {
        self.entities.iter().position(|slot| *slot == Some(entity))
    }

    /// Every currently-spawned `(index, entity)` pair, in index order.
    fn active(&self) -> impl Iterator<Item = (usize, Entity)> + '_ {
        self.entities
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| slot.map(|entity| (index, entity)))
    }
}

/// Intercell nav-agent ledger (issue #134): survives cell-swap teardown
/// (an ordinary Bevy `Resource`, untouched by `teardown_archipelago`) so an
/// agent handed off through a travel door, or frozen in place by a
/// player-initiated swap, can be restored once its cell is active again.
#[derive(Resource, Default)]
struct NavAgentLedger(ledger_policy::Ledger);

/// The origin door reference the player just used to trigger a cell swap
/// (issue #134), noted by `note_player_swap_door` and consumed exactly
/// once by `despawn_stale_navmesh_archipelago` the next time it detects
/// the resulting stale archipelago. `None` when the swap-triggering cause
/// carried no door (there is currently no such path at runtime, but the
/// consumer treats an absent note as "no eligibility information" rather
/// than assuming a door, so any future non-door cell change still freezes
/// a live agent instead of losing it).
#[derive(Resource, Default)]
struct PendingPlayerSwapDoor(Option<u32>);

/// Console-configurable nav-solve interval (issue #114 added scope, wave 5):
/// `LandmassSystems::Update` (the pathfinding+avoidance solve) only runs
/// every `NavSolveRate`-th fixed tick, gated by `nav_solve_gate` against
/// `NavSolveStepCounter`; `apply_agent_physics_movement` still runs -- and
/// moves the agent -- every fixed tick regardless, blending toward whichever
/// desired velocity the last solve produced (`AgentDesiredVelocityBlend`).
/// `tna solverate [<n>]` is the console knob.
#[derive(Resource, Clone, Copy, Debug)]
struct NavSolveRate(u32);

impl Default for NavSolveRate {
    fn default() -> Self {
        Self(movement_policy::DEFAULT_NAV_SOLVE_INTERVAL)
    }
}

/// Fixed-tick counter driving `NavSolveRate`'s gate: incremented once per
/// `FixedPreUpdate` pass by `advance_nav_solve_step_counter`, before
/// `LandmassSystems::SyncExistence` runs, so both `nav_solve_gate` (deciding
/// whether `LandmassSystems::Update` runs this tick) and
/// `apply_agent_physics_movement`/`update_agent_desired_velocity_blend`
/// (reading it later in the same tick, in `FixedUpdate`/after
/// `LandmassSystems::Output`) see the same stable value.
#[derive(Resource, Default)]
struct NavSolveStepCounter(u64);

fn advance_nav_solve_step_counter(mut counter: ResMut<NavSolveStepCounter>) {
    counter.0 = counter.0.wrapping_add(1);
}

/// Run condition gating `LandmassSystems::Update` -- the actual
/// pathfinding+avoidance solve -- to `NavSolveRate`'s configured interval.
fn nav_solve_gate(counter: Res<NavSolveStepCounter>, rate: Res<NavSolveRate>) -> bool {
    movement_policy::should_solve(counter.0, rate.0)
}

/// Shifts each agent's `AgentDesiredVelocityBlend` on a fixed tick that
/// actually solved (issue #114 added scope, wave 5): `previous <- old
/// latest`, `latest <- this tick's freshly-synced `AgentDesiredVelocity3d``.
/// Runs in `FixedPreUpdate` after `LandmassSystems::Output`, so
/// `AgentDesiredVelocity3d` already reflects this tick's solve (or, on a
/// gated-off tick, whatever value the last solve left it holding). A no-op
/// on a gated-off tick -- the blend pair is left exactly as the last real
/// solve set it, for `apply_agent_physics_movement` to keep interpolating
/// toward.
fn update_agent_desired_velocity_blend(
    counter: Res<NavSolveStepCounter>,
    rate: Res<NavSolveRate>,
    mut agents: Query<
        (&AgentDesiredVelocity3d, &mut AgentDesiredVelocityBlend),
        With<TestNavAgentMarker>,
    >,
) {
    if !movement_policy::should_solve(counter.0, rate.0) {
        return;
    }
    for (desired, mut blend) in &mut agents {
        blend.previous = blend.latest;
        blend.latest = desired.velocity();
    }
}

pub(crate) struct NavBackendPlugin;

impl Plugin for NavBackendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Landmass3dPlugin::default())
            .init_resource::<NavArchipelagoState>()
            .init_resource::<TestNavAgentState>()
            .init_resource::<NavAgentLedger>()
            .init_resource::<PendingPlayerSwapDoor>()
            .init_resource::<NavSolveRate>()
            .init_resource::<NavSolveStepCounter>()
            .configure_sets(
                FixedPreUpdate,
                LandmassSystems::Update.run_if(nav_solve_gate),
            )
            .add_systems(
                FixedPreUpdate,
                advance_nav_solve_step_counter.before(LandmassSystems::SyncExistence),
            )
            .add_systems(
                FixedPreUpdate,
                sync_player_nav_character.before(LandmassSystems::SyncValues),
            )
            .add_systems(
                FixedPreUpdate,
                update_agent_desired_velocity_blend.after(LandmassSystems::Output),
            )
            .add_systems(
                FixedUpdate,
                (
                    despawn_stale_navmesh_archipelago,
                    restore_ledgered_agents_system,
                    door_availability_system,
                    door_link_system,
                    apply_agent_physics_movement,
                    merge_traversal_system,
                    door_traversal_system,
                    log_agent_state_changes,
                    log_path_latency,
                )
                    .chain(),
            );
    }
}

pub(crate) fn install(app: &mut App) {
    app.add_plugins(NavBackendPlugin);
}

/// Called from `world::swap::activate_resident_cell` (issue #134) once a
/// player-initiated cell swap is definitely proceeding: records which
/// origin door reference the player used so the next
/// `despawn_stale_navmesh_archipelago` pass (whichever frame it detects
/// the resulting stale archipelago) can decide follow-through vs. freeze
/// for any live nav agent still in the departing cell.
pub(crate) fn note_player_swap_door(world: &mut World, door_form_id: u32) {
    world.resource_mut::<PendingPlayerSwapDoor>().0 = Some(door_form_id);
}

fn no_nav_graph_error() -> ConsoleError {
    ConsoleError::new("no_nav_graph", "no nav graph prepared for this cell")
}

fn teardown_archipelago(world: &mut World) {
    let state = std::mem::take(&mut *world.resource_mut::<NavArchipelagoState>());
    for entity in state
        .links
        .into_iter()
        .chain(state.islands)
        .chain(state.player_character)
        .chain(state.archipelago)
    {
        if let Ok(entity) = world.get_entity_mut(entity) {
            entity.despawn();
        }
    }
}

/// Builds (or reuses, if already current for this cell) the archipelago,
/// islands, and door-link entities for the active cell's prepared nav
/// graph. Lazy: only called from `tna spawn`, never eagerly per cell swap.
fn ensure_archipelago(world: &mut World) -> Result<(), ConsoleError> {
    let (current_cell, path, travel_destinations, door_lock_info, door_positions) = {
        let manifest = world
            .get_resource::<crate::viewer::LoadedSceneManifest>()
            .ok_or_else(no_nav_graph_error)?;
        let path = super::nav_graph_path(manifest).ok_or_else(no_nav_graph_error)?;
        let travel_destinations = super::travel_door_destinations(manifest);
        let mut door_lock_info = HashMap::new();
        let mut door_positions = HashMap::new();
        for placement in &manifest.placements {
            if let crate::vsa::PreparedSemantic::Door(door) = &placement.semantic {
                door_lock_info.insert(
                    placement.reference_form_id,
                    DoorLockInfo {
                        lock_level: door.lock_level,
                        key_form_id: door.key_form_id,
                    },
                );
                door_positions.insert(
                    placement.reference_form_id,
                    Vec3::from_array(placement.translation),
                );
            }
        }
        (
            manifest.cell.form_id,
            path,
            travel_destinations,
            door_lock_info,
            door_positions,
        )
    };

    let already_current = {
        let state = world.resource::<NavArchipelagoState>();
        state.cell_form_id == Some(current_cell) && state.archipelago.is_some()
    };
    if already_current {
        return Ok(());
    }
    teardown_archipelago(world);

    let graph = super::read_nav_graph(&path).map_err(|error| {
        warn!("nav graph read failed at {}: {error:#}", path.display());
        no_nav_graph_error()
    })?;
    let mesh_inputs = super::mesh_inputs(&graph);
    let merge_inputs = super::merge_inputs(&graph);

    // Widen the sample distances to humanoid scale (see
    // `AGENT_POINT_SAMPLE_DISTANCE`'s doc comment for the real-data
    // evidence); keep the rest of `from_agent_radius`'s avoidance defaults.
    let mut options = ArchipelagoOptions::from_agent_radius(AGENT_RADIUS);
    options.point_sample_distance = AGENT_POINT_SAMPLE_DISTANCE;
    let archipelago_entity = world.spawn(Archipelago3d::new(options)).id();

    let mut islands = Vec::new();
    for mesh in &mesh_inputs {
        let result = landmass_graph::build_navigation_mesh(mesh, &merge_inputs);
        for diagnostic in &result.diagnostics {
            warn!(
                "nav landmass conversion mesh {:08x}: {}",
                mesh.form_id, diagnostic.message
            );
        }
        let Some(valid) = result.nav_mesh else {
            continue;
        };
        let handle = world.resource_mut::<Assets<NavMesh3d>>().add(NavMesh3d {
            nav_mesh: Arc::new(valid),
        });
        let island_entity = world
            .spawn(Island3dBundle {
                island: Island,
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
                nav_mesh: NavMeshHandle::<ThreeD>(handle),
            })
            .id();
        islands.push(island_entity);
    }

    if islands.is_empty() {
        if let Ok(entity) = world.get_entity_mut(archipelago_entity) {
            entity.despawn();
        }
        return Err(ConsoleError::new(
            "nav_mesh_invalid",
            "prepared nav graph produced no valid landmass islands",
        ));
    }

    // Issue #114 added scope: the player's mirrored landmass character
    // lives exactly as long as this archipelago (see `NavArchipelagoState::
    // player_character`'s doc comment).
    let player_character = spawn_player_nav_character(world, archipelago_entity);

    let mut links = Vec::new();
    let mut link_kinds = HashMap::new();
    let mut blocked_door_links = Vec::new();
    let mut door_usable = HashMap::new();

    // Same-cell cross-mesh merge links (issue #113 feature 2). Real FO3
    // meshes never share seam vertex positions, so landmass's native island
    // boundary linking cannot connect them (it needs coincident boundary
    // vertices); generated walk-through animation links across the matched
    // boundary edges are the path real data takes.
    for descriptor in landmass_graph::merge_link_descriptors(&mesh_inputs, &merge_inputs) {
        let start = Vec3::from_array(descriptor.side_a.midpoint);
        let end = Vec3::from_array(descriptor.side_b.midpoint);
        // Issue #154 feature 3: real traversal-distance cost, floored well
        // above zero -- `AnimationLink3d::cost` must stay strictly positive
        // regardless of how tight a validated portal's overlap ended up.
        let cost = descriptor.distance.max(0.01);
        for link_entity in spawn_link_pair(world, archipelago_entity, start, end, cost) {
            link_kinds.insert(link_entity, LinkKind::Merge);
            links.push(link_entity);
        }
    }

    // Two-sided intra-cell door links (wave 3). A locked door is excluded
    // from route planning as blocked until usable (issue #113 feature 3);
    // `door_availability_system` spawns the link when the door becomes
    // usable and triggers a repath.
    for descriptor in landmass_graph::door_link_descriptors(&mesh_inputs) {
        let start = Vec3::from_array(descriptor.side_a.midpoint);
        let end = Vec3::from_array(descriptor.side_b.midpoint);
        let usable = door_usable_now(world, descriptor.door_form_id, &door_lock_info);
        door_usable.insert(descriptor.door_form_id, usable);
        if usable {
            for link_entity in spawn_link_pair(world, archipelago_entity, start, end, 1.0) {
                link_kinds.insert(
                    link_entity,
                    LinkKind::Door {
                        form_id: descriptor.door_form_id,
                    },
                );
                links.push(link_entity);
            }
        } else {
            info!(
                "nav agent door link {:08x} blocked: locked",
                descriptor.door_form_id
            );
            blocked_door_links.push(BlockedDoorLink {
                door_form_id: descriptor.door_form_id,
                start,
                end,
            });
        }
    }

    // Single-sided door triangles (issue #113 feature 3 / #137). Every one
    // is a crossing-gate candidate (issue #137: real data shows nearly all
    // of them also resolve to a travel destination, so the candidate set
    // cannot be limited to the non-travel subset -- see this file's module
    // doc). One whose reference *does* resolve to a travel door additionally
    // gets a terminal travel link: no landmass animation link is spawned
    // for it -- the far side lives in another cell's NAVM, so there is
    // nothing on-mesh to link to; the agent routes *to* the triangle and
    // the travel-arrival lifecycle runs there when it is the agent's own
    // `travel_intent` target, or the crossing gate runs when it is merely
    // on the way to somewhere else.
    let mut travel_doors = HashMap::new();
    let mut mid_route_doors = Vec::new();
    for door in landmass_graph::single_sided_doors(&mesh_inputs) {
        let triangle_midpoint = Vec3::from_array(door.side.midpoint);
        door_usable.insert(
            door.door_form_id,
            door_usable_now(world, door.door_form_id, &door_lock_info),
        );
        mid_route_doors.push(MidRouteDoor {
            door_form_id: door.door_form_id,
            midpoint: triangle_midpoint,
        });
        if let Some(&destination) = travel_destinations.get(&door.door_form_id) {
            let door_position = door_positions
                .get(&door.door_form_id)
                .copied()
                .unwrap_or(triangle_midpoint);
            info!(
                "nav agent travel door {:08x} -> cell {:08x}",
                door.door_form_id, destination.cell_form_id
            );
            travel_doors.insert(
                door.door_form_id,
                TravelDoorLink {
                    triangle_midpoint,
                    door_position,
                    destination_cell_form_id: destination.cell_form_id,
                    destination_door_form_id: destination.door_reference_form_id,
                },
            );
        }
    }

    *world.resource_mut::<NavArchipelagoState>() = NavArchipelagoState {
        cell_form_id: Some(current_cell),
        archipelago: Some(archipelago_entity),
        player_character: Some(player_character),
        islands,
        links,
        link_kinds,
        blocked_door_links,
        travel_doors,
        mid_route_doors,
        door_usable,
        door_lock_info,
    };
    Ok(())
}

/// Spawns one logical off-mesh link as *two* unidirectional
/// `AnimationLink3d`s (start -> end and end -> start) rather than one
/// `bidirectional: true` link: landmass 0.9.1's bidirectional path
/// (`nav_data.rs`'s reverse `OffMeshLink` insert) indexes the *start*
/// island's polygon array with the *end* portal's polygon index when
/// computing `destination_type_index`, which panics ("index out of bounds")
/// the moment the two ends sit on different islands -- exactly the
/// cross-mesh case every link this module spawns is for. Confirmed on real
/// FranklinMetro02 data (end polygon 260 vs start island's 72 polygons);
/// two unidirectional links take the correctly-indexed non-bidirectional
/// path and are semantically identical. Reported upstream as
/// <https://github.com/andriyDev/landmass/issues/192>; collapse back to one
/// `bidirectional: true` link once a fixed release is adopted.
///
/// `cost` (issue #154 feature 3): door links keep passing the previous flat
/// `1.0`; merge links pass their own `MergeLinkDescriptor::distance` (real
/// traversal distance between the two portal-interval midpoints) so
/// landmass's route cost reflects how far a crossing actually moves the
/// agent instead of treating every merge seam as equally cheap.
fn spawn_link_pair(
    world: &mut World,
    archipelago_entity: Entity,
    start: Vec3,
    end: Vec3,
    cost: f32,
) -> [Entity; 2] {
    let mut spawn_one = |from: Vec3, to: Vec3| {
        world
            .spawn(AnimationLink3dBundle {
                link: AnimationLink3d {
                    start_edge: (from, from),
                    end_edge: (to, to),
                    kind: 0,
                    cost,
                    bidirectional: false,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            })
            .id()
    };
    [spawn_one(start, end), spawn_one(end, start)]
}

/// Spawns the landmass `Character3d` mirroring the FPS player (issue #114
/// added scope, wave 5): a non-agent RVO obstacle nav agents steer around
/// but that landmass itself never moves. `Character<CS>` requires
/// `Transform`/`Velocity3d` (`bevy_landmass`'s own `#[require(...)]`), so
/// this only needs to seed the bundle plus a starting `Transform` -- an
/// initial placement at the player's current position so the entity is
/// never left at the origin for even one tick; `sync_player_nav_character`
/// takes over every fixed tick after that, before `LandmassSystems::
/// SyncValues` reads it. `player_transform_query` returning `None` (no FPS
/// player yet -- e.g. before `initialize_default_fps` has run) is not an
/// error here: the character still needs to exist so agents already routed
/// have something to sync onto once the player does appear.
fn spawn_player_nav_character(world: &mut World, archipelago_entity: Entity) -> Entity {
    let position = player_transform_query(world).unwrap_or(Vec3::ZERO);
    world
        .spawn((
            Character3dBundle {
                character: default(),
                settings: CharacterSettings {
                    radius: player::CAPSULE_RADIUS,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            },
            Transform::from_translation(position),
        ))
        .id()
}

/// Mirrors the FPS player onto its landmass character every fixed tick
/// (issue #114 added scope, wave 5), before `LandmassSystems::SyncValues`
/// reads `Transform`/`Velocity3d`: agents predict and avoid the player using
/// its *actual* post-collision KCC velocity, matching
/// `apply_agent_physics_movement`'s own physics-authoritative feedback
/// convention rather than desired input. A no-op whenever no archipelago has
/// ever been built (`tna spawn` never ran -- the common case) or the FPS
/// player does not currently exist (startup, or a console-harness test
/// world) -- never panics either way.
fn sync_player_nav_character(
    archipelago_state: Res<NavArchipelagoState>,
    mut characters: Query<(&mut Transform, &mut Velocity3d)>,
    players: Query<(&GlobalTransform, &player::KccState), With<player::FpsPlayer>>,
) {
    let Some(character_entity) = archipelago_state.player_character else {
        return;
    };
    let Ok((player_transform, kcc)) = players.single() else {
        return;
    };
    let Ok((mut transform, mut velocity)) = characters.get_mut(character_entity) else {
        return;
    };
    transform.translation = player_transform.translation();
    velocity.velocity = kcc.velocity;
}

/// The live `(open, locked)` observation for `door_form_id`: `open` reads
/// the runtime `InteractionState.open` set (guarded on `RefRegistry` being
/// present -- `resolve_reference` panics without one, which minimal test
/// worlds may not have), `locked` runs the same `interaction::door_is_locked`
/// check the activation prompt uses (never a second lock model) against its
/// prepared lock/key data and the player's inventory. A door with no
/// prepared lock info is never locked.
fn door_open_and_locked(
    world: &World,
    door_form_id: u32,
    door_lock_info: &HashMap<u32, DoorLockInfo>,
) -> (bool, bool) {
    let open = world
        .get_resource::<crate::console::RefRegistry>()
        .is_some()
        && crate::console::resolve_reference(world, &format!("{door_form_id:08x}"))
            .ok()
            .is_some_and(|entity| {
                world
                    .get_resource::<interaction::InteractionState>()
                    .is_some_and(|state| state.open.contains(&entity))
            });
    let locked = door_lock_info.get(&door_form_id).is_some_and(|info| {
        let door = crate::vsa::PreparedDoor {
            lock_level: info.lock_level,
            key_form_id: info.key_form_id,
            destination: None,
        };
        match world.get_resource::<interaction::PlayerInventory>() {
            Some(inventory) => interaction::door_is_locked(&door, inventory),
            // No inventory resource (minimal test worlds): key
            // possession can't help, so locked is decided by the lock
            // level alone.
            None => door.lock_level.is_some_and(|level| level > 0),
        }
    });
    (open, locked)
}

/// Whether `door_form_id` is currently usable for route planning: already
/// open, or not locked (`repath::door_usable`'s rule). A door with no
/// prepared lock info is usable.
fn door_usable_now(
    world: &World,
    door_form_id: u32,
    door_lock_info: &HashMap<u32, DoorLockInfo>,
) -> bool {
    let (open, locked) = door_open_and_locked(world, door_form_id, door_lock_info);
    repath::door_usable(repath::DoorObservation { locked, open })
}

fn player_transform_query(world: &mut World) -> Option<Vec3> {
    let mut query = world.query_filtered::<&GlobalTransform, With<player::FpsPlayer>>();
    query.single(world).ok().map(|t| t.translation())
}

fn player_entity_query(world: &mut World) -> Option<Entity> {
    let mut query = world.query_filtered::<Entity, With<player::FpsPlayer>>();
    query.single(world).ok()
}

/// `tna` command dispatcher: `invocation.args[0]` is the subcommand;
/// `tna` with no arguments prints usage rather than erroring.
pub(crate) fn tna_command(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let Some(subcommand) = invocation.args.first() else {
        return Ok(usage_reply());
    };
    let rest = &invocation.args[1..];
    match subcommand.as_str() {
        "spawn" => spawn_agent(world, rest),
        "goto" => goto_agent(world, rest),
        "travel" => travel_agent(world, rest),
        "status" => agent_status(world, rest),
        "despawn" => despawn_agent(world, rest),
        "solverate" => solve_rate_command(world, rest),
        other => Err(ConsoleError::new(
            "unknown_subcommand",
            format!(
                "unknown tna subcommand '{other}'; expected spawn, goto, travel, status, despawn, or solverate"
            ),
        )),
    }
}

fn usage_reply() -> ConsoleCommandResult {
    let usage = "usage: tna spawn [<index>]|goto [<index>] <x> <y> <z>|goto [<index>] player|travel [<index>] <door-formid>|status [<index>]|despawn [<index>]|solverate [<n>]";
    ConsoleCommandResult::new(json!({ "usage": usage }), vec![usage.to_string()])
}

/// `tna solverate [<n>]` (issue #114 added scope, wave 5): reports the
/// current `NavSolveRate` divisor with no argument, following the
/// `getrender`/`setrender` get-or-set convention; sets it with one. `n` must
/// be a positive integer (`0` would mean "never solve", not "always solve";
/// `movement_policy::should_solve`/`solve_blend_fraction` both clamp
/// defensively too, but the console rejects it outright rather than
/// silently reinterpreting it).
fn solve_rate_command(
    world: &mut World,
    rest: &[String],
) -> Result<ConsoleCommandResult, ConsoleError> {
    match rest {
        [] => {
            let interval = world.resource::<NavSolveRate>().0;
            Ok(ConsoleCommandResult::new(
                json!({ "interval": interval }),
                vec![format!("nav solve rate interval={interval}")],
            ))
        }
        [value] => {
            let interval = value
                .parse::<u32>()
                .ok()
                .filter(|&n| n >= 1)
                .ok_or_else(|| {
                    ConsoleError::new(
                        "bad_type",
                        "tna solverate interval must be a positive integer",
                    )
                })?;
            world.resource_mut::<NavSolveRate>().0 = interval;
            info!("nav solve rate interval={interval}");
            Ok(ConsoleCommandResult::new(
                json!({ "interval": interval }),
                vec![format!("nav solve rate interval set to {interval}")],
            ))
        }
        _ => Err(ConsoleError::new(
            "bad_arity",
            "tna solverate accepts at most one interval",
        )),
    }
}

/// Parses an agent index argument, bounded to `0..MAX_TEST_AGENTS`. Every
/// `tna` subcommand that used to address the single spike agent now takes
/// this as an optional leading token; omitting it defaults to agent 0
/// (issue #114 feature 4's back-compat requirement).
fn parse_agent_index(value: &str) -> Result<usize, ConsoleError> {
    value
        .parse::<usize>()
        .ok()
        .filter(|&index| index < MAX_TEST_AGENTS)
        .ok_or_else(|| {
            ConsoleError::new(
                "bad_agent_index",
                format!("agent index must be an integer 0..{}", MAX_TEST_AGENTS - 1),
            )
        })
}

/// Spawns the capsule mesh + `bevy_landmass` agent entity at `position` in
/// the already-current archipelago (`ensure_archipelago` must have run),
/// with its `AgentRuntime`/`AgentKcc` components at their defaults. Shared
/// by `spawn_agent` (the `tna spawn` console command, positioned at the
/// player) and `restore_ledgered_agent` (issue #134, positioned at a
/// resolved ledger spawn point) -- neither sets `TestNavAgentState` itself,
/// so callers own that and its accompanying log line.
fn spawn_test_agent(world: &mut World, position: Vec3) -> Entity {
    let archipelago_entity = world
        .resource::<NavArchipelagoState>()
        .archipelago
        .expect("ensure_archipelago populated the archipelago");

    let cylinder_height = (AGENT_HEIGHT - 2.0 * AGENT_RADIUS).max(0.0);
    let mesh = world
        .resource_mut::<Assets<Mesh>>()
        .add(Capsule3d::new(AGENT_RADIUS, cylinder_height));
    let material = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.85, 0.9),
            ..default()
        });

    let agent_entity = world
        .spawn((
            TestNavAgentMarker,
            AgentRuntime::default(),
            AgentKcc::default(),
            AgentDesiredVelocityBlend::default(),
            Transform::from_translation(position),
            Visibility::Inherited,
            Agent3dBundle {
                agent: default(),
                settings: AgentSettings {
                    radius: AGENT_RADIUS,
                    desired_speed: AGENT_DESIRED_SPEED,
                    max_speed: AGENT_MAX_SPEED,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            },
            TargetReachedCondition::Distance(Some(AGENT_TARGET_REACHED_DISTANCE)),
        ))
        .id();
    // Zero offset (issue #114 real-data regression fix, M4 wave 5): the
    // parent `agent_entity`'s `Transform` is already the capsule *centre*
    // (physics-authoritative movement positions it there, mirroring the
    // player's own capsule-centre convention -- see `spawn_bare_agent`'s
    // doc comment and the horizontal-distance regression fix a few commits
    // back), not feet level like the wave-3/4 navmesh-Y-snapped kinematic
    // agent this `AGENT_HEIGHT / 2.0` offset used to compensate for. Lifting
    // the visual child by another half-height on top of an already-centred
    // parent double-counts that offset, floating the rendered capsule a
    // full half-height above the floor even though the physics capsule
    // (steps/slopes) sits correctly. `Capsule3d`'s mesh is centred at its
    // own local origin, so a zero-offset child renders centred exactly on
    // the parent -- the capsule bottom lands on the feet/floor.
    let visual = world
        .spawn((Mesh3d(mesh), MeshMaterial3d(material), Transform::IDENTITY))
        .id();
    world.entity_mut(agent_entity).add_child(visual);
    agent_entity
}

fn spawn_agent(world: &mut World, rest: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    let index = match rest {
        [] => 0,
        [index] => parse_agent_index(index)?,
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna spawn accepts at most one agent index",
            ));
        }
    };
    ensure_archipelago(world)?;
    if world.resource::<TestNavAgentState>().entities[index].is_some() {
        return Err(ConsoleError::new(
            "already_spawned",
            "a test nav agent is already spawned at this index; use tna despawn first",
        ));
    }
    let position = player_transform_query(world)
        .ok_or_else(|| ConsoleError::new("player_unavailable", "the FPS player does not exist"))?;
    let agent_entity = spawn_test_agent(world, position);
    world.resource_mut::<TestNavAgentState>().entities[index] = Some(agent_entity);
    info!(
        "nav agent {index} spawn position=({:.2},{:.2},{:.2})",
        position.x, position.y, position.z
    );
    Ok(ConsoleCommandResult::new(
        json!({ "index": index, "position": [position.x, position.y, position.z] }),
        vec![format!(
            "nav agent {index} spawned at ({:.2}, {:.2}, {:.2})",
            position.x, position.y, position.z
        )],
    ))
}

/// Parses a bare or `0x`-prefixed hex FormID argument (`tna travel`'s door
/// selector), mirroring `console::parse_item_form_id`'s grammar -- that
/// helper is private to `console.rs`, outside this wave's file-ownership
/// boundary, so this is a small intentional duplicate rather than a new
/// cross-module dependency.
fn parse_form_id(value: &str) -> Option<u32> {
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    ((1..=8).contains(&digits.len()) && digits.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| u32::from_str_radix(digits, 16).ok())
        .flatten()
}

/// `tna travel [<index>] <door-formid>` (issue #134; indexed #114): routes
/// the given agent through the given travel door end-to-end, wiring up
/// `request_travel`.
fn travel_agent(world: &mut World, rest: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    let (index, door) = match rest {
        [door] => (0, door),
        [index, door] => (parse_agent_index(index)?, door),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna travel requires [<index>] <door-formid>",
            ));
        }
    };
    let door_form_id = parse_form_id(door)
        .ok_or_else(|| ConsoleError::new("bad_type", "tna travel door FormID must be hex"))?;
    request_travel(world, index, door_form_id)?;
    Ok(ConsoleCommandResult::new(
        json!({ "index": index, "door_form_id": door_form_id }),
        vec![format!(
            "nav agent {index} travel requested to door {door_form_id:08x}"
        )],
    ))
}

fn parse_goto_point(x: &str, y: &str, z: &str) -> Result<Vec3, ConsoleError> {
    let parse = |value: &str| {
        value.parse::<f32>().map_err(|_| {
            ConsoleError::new("bad_type", "tna goto coordinates must be finite numbers")
        })
    };
    Ok(Vec3::new(parse(x)?, parse(y)?, parse(z)?))
}

fn goto_player_target(world: &mut World) -> Result<AgentTarget3d, ConsoleError> {
    let player_entity = player_entity_query(world)
        .ok_or_else(|| ConsoleError::new("player_unavailable", "the FPS player does not exist"))?;
    Ok(AgentTarget3d::Entity(player_entity))
}

/// `tna goto [<index>] <x> <y> <z>|player` (indexed #114): the leading
/// index token is optional and distinguished purely by argument count, so
/// every previously single-agent form (`goto <x> <y> <z>`, `goto player`)
/// is unchanged and still addresses agent 0.
fn goto_agent(world: &mut World, rest: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    let (index, target, description) = match rest {
        [value] if value == "player" => (0, goto_player_target(world)?, "player".to_string()),
        [index, value] if value == "player" => (
            parse_agent_index(index)?,
            goto_player_target(world)?,
            "player".to_string(),
        ),
        [x, y, z] => {
            let point = parse_goto_point(x, y, z)?;
            (
                0,
                AgentTarget3d::Point(point),
                format!("({:.2}, {:.2}, {:.2})", point.x, point.y, point.z),
            )
        }
        [index, x, y, z] => {
            let point = parse_goto_point(x, y, z)?;
            (
                parse_agent_index(index)?,
                AgentTarget3d::Point(point),
                format!("({:.2}, {:.2}, {:.2})", point.x, point.y, point.z),
            )
        }
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna goto requires [<index>] <x> <y> <z> or [<index>] player",
            ));
        }
    };
    let Some(agent_entity) = world.resource::<TestNavAgentState>().entities[index] else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned at this index; use tna spawn first",
        ));
    };
    world.entity_mut(agent_entity).insert(target);
    let elapsed = world.resource::<Time>().elapsed_secs();
    if let Some(mut runtime) = world.get_mut::<AgentRuntime>(agent_entity) {
        runtime.goto_started_at = Some(elapsed);
        runtime.latency_logged = false;
    }
    // A fresh target resets the pure stuck-tracking window (movement_policy)
    // -- the agent gets a clean run at the new waypoint.
    if let Some(mut kcc) = world.get_mut::<AgentKcc>(agent_entity) {
        kcc.best_distance = f32::MAX;
        kcc.ticks_without_progress = 0;
        kcc.recovery_active = false;
        kcc.stuck = false;
    }
    Ok(ConsoleCommandResult::new(
        json!({ "index": index, "target": description }),
        vec![format!("nav agent {index} target set to {description}")],
    ))
}

fn agent_status(world: &mut World, rest: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    let index = match rest {
        [] => 0,
        [index] => parse_agent_index(index)?,
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna status accepts at most one agent index",
            ));
        }
    };
    let Some(agent_entity) = world.resource::<TestNavAgentState>().entities[index] else {
        // Issue #134: a handed-off or frozen agent has no live entity but
        // still exists in the ledger -- report that instead of the "no
        // agent" error `tna spawn` would otherwise imply is needed.
        if let Some(entry) = world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(agent_ledger_id(index))
        {
            let line = format!(
                "nav agent {index} handed off to cell {:08x}",
                entry.cell_form_id
            );
            return Ok(ConsoleCommandResult::new(
                json!({ "index": index, "status": "handed-off", "cell": entry.cell_form_id }),
                vec![line],
            ));
        }
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned at this index; use tna spawn first",
        ));
    };
    let position = world
        .get::<GlobalTransform>(agent_entity)
        .map(|t| t.translation())
        .unwrap_or_default();
    let landmass_state = world
        .get::<AgentState>(agent_entity)
        .copied()
        .unwrap_or_default();
    let (door_link_state, link_desc) = match world.get::<AgentRuntime>(agent_entity) {
        Some(runtime) => (runtime.door_link, active_link_description(runtime)),
        None => (door_link::DoorLinkState::default(), None),
    };
    let (grounded, stuck, collision_blocked) = world
        .get::<AgentKcc>(agent_entity)
        .map(|kcc| (kcc.grounded, kcc.stuck, kcc.collision_blocked))
        .unwrap_or_default();
    let status = resolve_status(landmass_state, door_link_state);
    let target_desc = world
        .get::<AgentTarget3d>(agent_entity)
        .map(describe_target)
        .unwrap_or_else(|| "none".to_string());
    let mut line = format!(
        "nav agent {index} status={} position=({:.2},{:.2},{:.2}) target={} grounded={grounded} stuck={stuck} blocked={collision_blocked}",
        status.as_str(),
        position.x,
        position.y,
        position.z,
        target_desc
    );
    if let Some(link) = &link_desc {
        line.push_str(&format!(" link={link}"));
    }
    Ok(ConsoleCommandResult::new(
        json!({
            "index": index,
            "status": status.as_str(),
            "position": [position.x, position.y, position.z],
            "target": target_desc,
            "link": link_desc,
            "grounded": grounded,
            "stuck": stuck,
            "blocked": collision_blocked,
        }),
        vec![line],
    ))
}

fn resolve_status(
    landmass_state: AgentState,
    door_link_state: door_link::DoorLinkState,
) -> landmass_graph::NavAgentStatus {
    if door_link::is_travel_reached(door_link_state) {
        return landmass_graph::NavAgentStatus::TravelReached;
    }
    if door_link::is_paused(door_link_state) || door_link::is_failed(door_link_state) {
        return landmass_graph::NavAgentStatus::Paused;
    }
    landmass_graph::map_agent_state(landmass_state)
}

/// Issue #151: one deterministic line per currently-spawned test nav agent
/// for the console debug-info HUD, reusing the exact same
/// status/grounded/stuck/blocked fields `tna status` (`agent_status` above)
/// reports -- read-only, so this can run from a plain `Update` HUD system
/// instead of needing the console command's `&mut World`/`ConsoleInvocation`
/// plumbing.
pub(crate) fn hud_agent_status_lines(world: &World) -> Vec<String> {
    let Some(state) = world.get_resource::<TestNavAgentState>() else {
        return Vec::new();
    };
    state
        .entities
        .iter()
        .enumerate()
        .filter_map(|(index, entity)| {
            let entity = (*entity)?;
            let position = world
                .get::<GlobalTransform>(entity)
                .map(|transform| transform.translation())
                .unwrap_or_default();
            let landmass_state = world.get::<AgentState>(entity).copied().unwrap_or_default();
            let door_link_state = world
                .get::<AgentRuntime>(entity)
                .map(|runtime| runtime.door_link)
                .unwrap_or_default();
            let (grounded, stuck, collision_blocked) = world
                .get::<AgentKcc>(entity)
                .map(|kcc| (kcc.grounded, kcc.stuck, kcc.collision_blocked))
                .unwrap_or_default();
            let status = resolve_status(landmass_state, door_link_state);
            Some(format!(
                "nav agent {index} status={} position=({:.2},{:.2},{:.2}) grounded={grounded} stuck={stuck} blocked={collision_blocked}",
                status.as_str(),
                position.x,
                position.y,
                position.z,
            ))
        })
        .collect()
}

/// The `link=` suffix for `tna status` (issue #113 feature 5): the active
/// link kind while interacting with one (`merge` while crossing a merge
/// seam, `door <formid>` through a door lifecycle), else `None`.
fn active_link_description(runtime: &AgentRuntime) -> Option<String> {
    match runtime.active_link {
        Some(LinkKind::Merge) => Some("merge".to_string()),
        Some(LinkKind::Door { form_id }) => Some(format!("door {form_id:08x}")),
        None => match runtime.door_link {
            door_link::DoorLinkState::TravelReached {
                door_form_id,
                destination_cell_form_id,
            } => Some(format!(
                "door {door_form_id:08x} cell {destination_cell_form_id:08x}"
            )),
            _ => None,
        },
    }
}

fn describe_target(target: &AgentTarget3d) -> String {
    match target {
        AgentTarget3d::None => "none".to_string(),
        AgentTarget3d::Point(point) => format!("({:.2}, {:.2}, {:.2})", point.x, point.y, point.z),
        AgentTarget3d::Entity(entity) => format!("entity:{entity:?}"),
    }
}

fn despawn_agent(world: &mut World, rest: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    let index = match rest {
        [] => 0,
        [index] => parse_agent_index(index)?,
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna despawn accepts at most one agent index",
            ));
        }
    };
    let Some(agent_entity) = world.resource::<TestNavAgentState>().entities[index] else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned at this index; use tna spawn first",
        ));
    };
    if let Ok(entity) = world.get_entity_mut(agent_entity) {
        entity.despawn();
    }
    world.resource_mut::<TestNavAgentState>().entities[index] = None;
    Ok(ConsoleCommandResult::new(
        json!({ "index": index, "despawned": true }),
        vec![format!("nav agent {index} despawned")],
    ))
}

// ---------------------------------------------------------------------
// Runtime systems
// ---------------------------------------------------------------------

/// One tick of the physics-authoritative agent KCC (issue #114): sweeps
/// `mover` through the real `boxddd::World` using the same free
/// `move_mover`/step-support helpers the player capsule controller uses
/// (`player/movement.rs`), taking landmass's desired horizontal velocity as
/// this tick's input and integrating gravity exactly like the player when
/// airborne. Grounded/step decisions are `movement_policy::decide_grounded`
/// calls, not inline logic. Pulled out of the Bevy system so it is directly
/// testable against a real `boxddd::World` fixture (`#[cfg(test)]`, mirrors
/// `player/tests/mod.rs`'s own `move_mover` fixtures) without a Bevy `App`.
/// Returns the new world position, the new KCC velocity to remember for
/// next tick, and the new grounded state.
#[allow(clippy::too_many_arguments)]
fn step_agent_kcc(
    world: &mut boxddd::World,
    mover: &boxddd::Capsule,
    collision_filter: boxddd::QueryFilter,
    support_filter: boxddd::QueryFilter,
    origin: Vec3,
    kcc_velocity_in: Vec3,
    grounded_in: bool,
    desired_horizontal: Vec2,
    dt: f32,
) -> (Vec3, Vec3, bool) {
    let box_origin = player::to_box_vec3(origin);
    let initial_planes = world
        .collide_mover(box_origin, mover, collision_filter)
        .unwrap_or_default();
    // Preserve support for one solve while crossing a tread edge, exactly
    // like the player controller -- the final sweep below decides whether
    // that support still exists.
    let mut grounded = grounded_in
        || movement_policy::decide_grounded(movement_policy::GroundedObservation {
            has_walkable_plane: player::has_walkable_plane(&initial_planes),
            stepped_up: false,
        });

    let mut velocity = kcc_velocity_in;
    velocity.x = desired_horizontal.x;
    velocity.z = desired_horizontal.y;
    if grounded {
        if velocity.y < 0.0 {
            velocity.y = 0.0;
        }
    } else {
        velocity.y -= GRAVITY * dt;
    }

    let desired_delta = player::to_box_vec3(velocity * dt);
    let intentional_horizontal_motion = desired_horizontal.length_squared() > f32::EPSILON;
    let (mut position, planes, stepped_up, _attempted) = player::move_mover(
        world,
        box_origin,
        mover,
        desired_delta,
        collision_filter,
        support_filter,
        grounded && intentional_horizontal_motion,
        false,
    );
    grounded = movement_policy::decide_grounded(movement_policy::GroundedObservation {
        has_walkable_plane: player::has_walkable_plane(&planes),
        stepped_up,
    });
    if !grounded && velocity.y <= 0.0 {
        if intentional_horizontal_motion
            && let Some(supported) =
                player::try_forward_step_support(world, position, desired_delta, support_filter)
        {
            position = supported;
            grounded = true;
        } else if let Some(snapped) = player::try_step_down(
            world,
            position,
            mover,
            desired_delta,
            collision_filter,
            support_filter,
        ) {
            position = snapped;
            grounded = true;
        }
    }
    if grounded && velocity.y < 0.0 {
        velocity.y = 0.0;
    }
    (player::from_box_vec3(position), velocity, grounded)
}

type AgentPhysicsQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static mut Transform,
        &'static mut Velocity3d,
        &'static AgentDesiredVelocityBlend,
        &'static mut AgentKcc,
        Option<&'static AgentTarget3d>,
        Option<&'static AgentState>,
    ),
    (
        With<TestNavAgentMarker>,
        Without<DoorTraversal>,
        // Issue #154 feature 4: a merge-portal crossing is swept by its own
        // `merge_traversal_system`, not this system.
        Without<MergeTraversal>,
    ),
>;

/// Physics-authoritative agent movement (issue #114): landmass's desired
/// velocity is this tick's *input*, not the final motion -- specifically
/// the blend `movement_policy::solve_blend_fraction` produces between
/// `AgentDesiredVelocityBlend`'s previous and latest solved values (issue
/// #114 added scope, wave 5's configurable solve interval), not
/// `AgentDesiredVelocity3d` directly; at the default interval of `1` this
/// blend is always exactly the latest value, so behaviour is unchanged from
/// a `1`-tick solve cadence. Each agent's own capsule KCC (`step_agent_kcc`,
/// mirroring `player/movement.rs`'s controller against the same shared
/// `bevy_boxddd` world) resolves collision/steps/slopes/gravity and moves
/// the `Transform`; the actual achieved velocity is what gets written back
/// to `Velocity3d` for landmass to plan against next solve -- navigation
/// proposes, physics disposes. Gated on `CellPhysicsReadiness` exactly like
/// the player (`player/movement.rs::apply_player_controls`): an agent must
/// not move through geometry that has not finished building. Door-link
/// crossings (`DoorTraversal`) are excluded; the lerp owns the transform
/// there. Feeds `movement_policy::decide_collision_outcome`/`decide_stuck`
/// per agent and emits the stable `nav agent collision-blocked <id>`/`nav
/// agent stuck <id>` lines on the rising edge, plus one forced repath
/// (re-inserting the current target) the first time an agent's route stops
/// making progress.
#[allow(clippy::too_many_arguments)]
fn apply_agent_physics_movement(
    time: Res<Time>,
    physics_disabled: Res<PhysicsDisabled>,
    cell_physics: Res<CellPhysicsReadiness>,
    roster: Res<TestNavAgentState>,
    solve_counter: Res<NavSolveStepCounter>,
    solve_rate: Res<NavSolveRate>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    mut agents: AgentPhysicsQuery<'_, '_>,
    targets: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    if physics_disabled.0 || !cell_physics.static_collision_ready() {
        for (_, _, mut velocity, _, mut kcc, _, _) in &mut agents {
            velocity.velocity = Vec3::ZERO;
            kcc.velocity = Vec3::ZERO;
            kcc.grounded = false;
        }
        return;
    }
    let Some(world) = context.world_mut() else {
        return;
    };
    let mover = boxddd::Capsule::new(
        [0.0, -(AGENT_HEIGHT * 0.5 - AGENT_RADIUS), 0.0],
        [0.0, AGENT_HEIGHT * 0.5 - AGENT_RADIUS, 0.0],
        AGENT_RADIUS,
    );
    // Same collision categories the player capsule queries against -- one
    // shared static/dynamic world, no separate agent-only layer.
    let collision_filter = player::player_collision_filter();
    let support_filter = player::stair_support_filter();
    // Same blend fraction for every agent this tick -- the solve gate is
    // global, not per-agent.
    let solve_blend_fraction = movement_policy::solve_blend_fraction(
        movement_policy::steps_since_solve(solve_counter.0, solve_rate.0),
        solve_rate.0,
    );

    for (entity, mut transform, mut velocity, blend, mut kcc, target, agent_state) in &mut agents {
        let desired_velocity = blend.previous.lerp(blend.latest, solve_blend_fraction);
        let desired_horizontal = Vec2::new(desired_velocity.x, desired_velocity.z);
        let (new_position, new_kcc_velocity, grounded) = step_agent_kcc(
            world,
            &mover,
            collision_filter,
            support_filter,
            transform.translation,
            kcc.velocity,
            kcc.grounded,
            desired_horizontal,
            dt,
        );
        let achieved = (new_position - transform.translation) / dt;
        transform.translation = new_position;
        kcc.velocity = new_kcc_velocity;
        kcc.grounded = grounded;
        velocity.velocity = achieved;

        let outcome =
            movement_policy::decide_collision_outcome(movement_policy::VelocityObservation {
                desired_horizontal_speed: desired_horizontal.length(),
                achieved_horizontal_speed: Vec2::new(achieved.x, achieved.z).length(),
            });
        let was_blocked = kcc.collision_blocked;
        kcc.collision_blocked = matches!(outcome, movement_policy::CollisionOutcome::Blocked);
        if kcc.collision_blocked
            && !was_blocked
            && let Some(index) = roster.index_of(entity)
        {
            info!("nav agent collision-blocked {index}");
        }

        // Stuck detection needs a live target distance -- an agent with no
        // active point/entity target is never stuck (nothing to make
        // progress toward).
        let target_point = match target {
            Some(AgentTarget3d::Point(point)) => Some(*point),
            Some(AgentTarget3d::Entity(target_entity)) => targets
                .get(*target_entity)
                .ok()
                .map(GlobalTransform::translation),
            _ => None,
        };
        let Some(target_point) = target_point else {
            continue;
        };
        // Horizontal, not 3D: `new_position` is the capsule *centre*,
        // `target_point` a feet-level nav-graph point (see
        // `movement_policy::horizontal_distance`'s doc comment) -- a 3D
        // distance never closes the constant ~half-`AGENT_HEIGHT` gap
        // between them, so the agent would falsely latch `stuck` at every
        // target it has actually reached. No vertical guard here (unlike
        // the door-proximity gates): there is exactly one committed target,
        // not an array of candidates a stray vertical match could
        // misidentify, and a route with real elevation change still wants
        // horizontal progress to dominate the distance-to-target measure.
        //
        // This `distance` now feeds only the arrival short-circuit just
        // below (`arrival_resets_stuck`, issue #136 follow-up) -- literal
        // proximity to the final target is exactly what "have I arrived"
        // means. Issue #157: it no longer feeds `decide_stuck` itself, see
        // `progress_distance` below.
        let distance =
            movement_policy::horizontal_distance(new_position.to_array(), target_point.to_array());

        // Corridor-progress signal (issue #157, see `movement_policy`'s
        // module doc comment): integrate this tick's real, KCC-resolved
        // horizontal motion projected onto whatever direction landmass is
        // *currently* steering toward, then negate the running total into
        // the same monotone "smaller is better" shape `decide_stuck`
        // already expects. A route leg that must move away from the final
        // target keeps registering fresh progress every tick it is
        // actually walking the corridor, unlike literal
        // distance-to-final-target.
        kcc.route_progress += movement_policy::route_progress_delta(
            [desired_horizontal.x, desired_horizontal.y],
            [achieved.x, achieved.z],
        ) * dt;
        let progress_distance = -kcc.route_progress;
        if kcc.best_distance == f32::MAX {
            kcc.best_distance = progress_distance;
        }
        // Genuinely at the target: "reached" and "stuck" are mutually
        // exclusive outcomes, not independent facts. Without this
        // short-circuit, `decide_stuck`'s "no further improvement possible"
        // window (`STUCK_RECOVERY_TICKS + STUCK_FAILURE_TICKS`, ~2 s at
        // 64 Hz) would eventually latch `stuck` for *any* agent that simply
        // arrives and stops -- once at the closest point it can get,
        // distance stops strictly decreasing regardless of *why* (blocked,
        // or done). A `Stuck` latch from a previous route also clears here:
        // reaching a (possibly re-issued) target is unambiguous progress.
        //
        // Two independent signals feed this, not just the horizontal
        // distance threshold (issue #136 follow-up): `bevy_landmass`'s own
        // `AgentState::ReachedTarget` is authoritative ground truth for
        // "landmass itself has stopped issuing meaningful movement toward
        // this target" and must reset stuck detection even when this
        // system's own recomputed distance disagrees. Post-erosion, a raw
        // (un-sampled) `AgentTarget3d::Point` from `tna goto` can have its
        // nearest reachable point end up farther than
        // `AGENT_TARGET_REACHED_DISTANCE` from the literal requested
        // coordinate (the walkable boundary shrank inward by the agent
        // radius) even though the agent is genuinely as close as it can
        // physically get -- without also trusting landmass's own state
        // here, the no-progress window would eventually (and incorrectly)
        // latch `stuck` on a route that had already finished. See
        // `movement_policy::arrival_resets_stuck`'s doc comment.
        let landmass_reached = matches!(agent_state, Some(AgentState::ReachedTarget));
        if movement_policy::arrival_resets_stuck(
            distance,
            AGENT_TARGET_REACHED_DISTANCE,
            landmass_reached,
        ) {
            kcc.best_distance = progress_distance;
            kcc.ticks_without_progress = 0;
            kcc.recovery_active = false;
            kcc.stuck = false;
            continue;
        }
        let decision = movement_policy::decide_stuck(movement_policy::StuckObservation {
            distance_to_target: progress_distance,
            best_distance_so_far: kcc.best_distance,
            ticks_without_progress: kcc.ticks_without_progress,
            recovery_active: kcc.recovery_active,
        });
        let progressed =
            progress_distance + movement_policy::STUCK_PROGRESS_EPSILON < kcc.best_distance;
        if progressed {
            kcc.best_distance = progress_distance;
            kcc.ticks_without_progress = 0;
            kcc.recovery_active = false;
        } else {
            kcc.ticks_without_progress = kcc.ticks_without_progress.saturating_add(1);
        }
        match decision {
            movement_policy::StuckDecision::Progressing => {}
            movement_policy::StuckDecision::StartRecovery => {
                kcc.recovery_active = true;
                // Force a landmass repath by re-inserting the current
                // target -- the same technique `door_availability_system`
                // uses on a door-usability flip. `AgentTarget3d` is not
                // `Clone`; rebuild the equivalent value by matching its
                // variants.
                let rebuilt = match target {
                    Some(AgentTarget3d::Point(point)) => Some(AgentTarget3d::Point(*point)),
                    Some(AgentTarget3d::Entity(target_entity)) => {
                        Some(AgentTarget3d::Entity(*target_entity))
                    }
                    _ => None,
                };
                if let Some(rebuilt) = rebuilt {
                    commands.entity(entity).insert(rebuilt);
                }
                if let Some(index) = roster.index_of(entity) {
                    info!("nav agent stuck-recovery {index}");
                }
            }
            movement_policy::StuckDecision::RecoveryPending => {}
            movement_policy::StuckDecision::Stuck => {
                let was_stuck = kcc.stuck;
                kcc.stuck = true;
                if !was_stuck && let Some(index) = roster.index_of(entity) {
                    info!("nav agent stuck {index}");
                }
            }
        }
    }
}

fn door_traversal_system(
    time: Res<Time>,
    mut agents: Query<
        (
            Entity,
            &mut Transform,
            &mut DoorTraversal,
            &mut AgentRuntime,
        ),
        With<TestNavAgentMarker>,
    >,
    mut roster: ResMut<TestNavAgentState>,
    archipelago_state: Res<NavArchipelagoState>,
    mut ledger: ResMut<NavAgentLedger>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut traversal, mut runtime) in &mut agents {
        traversal.elapsed += time.delta_secs();
        let t = (traversal.elapsed / DOOR_TRAVERSAL_SECONDS).clamp(0.0, 1.0);
        transform.translation = traversal.start.lerp(traversal.end, t);
        if t >= 1.0 {
            commands
                .entity(entity)
                .remove::<DoorTraversal>()
                .remove::<UsingAnimationLink>();
            match runtime.active_link {
                // Issue #154: a merge-seam crossing no longer drives
                // `DoorTraversal` at all (see `merge_traversal_system`), so
                // `Some(LinkKind::Merge)` should never actually reach here
                // in practice -- kept alongside `None` defensively (just
                // clear the link) rather than treated as an invariant
                // violation, the same conservative stance this match
                // already took for the "no active link" case.
                Some(LinkKind::Merge) | None => {
                    runtime.active_link = None;
                }
                Some(LinkKind::Door { .. }) => {
                    let new_state = door_link::transition(
                        runtime.door_link,
                        door_link::DoorLinkEvent::TraversalComplete,
                    );
                    let Some((door_form_id, destination_cell_form_id)) = (match new_state {
                        door_link::DoorLinkState::TravelReached {
                            door_form_id,
                            destination_cell_form_id,
                        } => Some((door_form_id, destination_cell_form_id)),
                        _ => None,
                    }) else {
                        runtime.door_link = new_state;
                        runtime.active_link = None;
                        continue;
                    };
                    let Some(index) = roster.index_of(entity) else {
                        continue;
                    };
                    let agent_id = agent_ledger_id(index);
                    // Issue #113's terminal travel seam: the agent stopped at
                    // the traversed door. Issue #134 owns what happens next:
                    // the agent leaves the active cell entirely, ledgered for
                    // the destination cell at that door's own paired marker.
                    info!(
                        "nav agent travel reached {door_form_id:08x} -> cell {destination_cell_form_id:08x}"
                    );
                    let destination_door_form_id = archipelago_state
                        .travel_doors
                        .get(&door_form_id)
                        .map(|link| link.destination_door_form_id);
                    match destination_door_form_id {
                        Some(destination_door_form_id) => {
                            ledger.0.record(ledger_policy::LedgerEntry {
                                agent_id,
                                cell_form_id: destination_cell_form_id,
                                spawn_kind: ledger_policy::SpawnKind::DoorMarker {
                                    destination_door_form_id,
                                },
                                remaining_target: None,
                            });
                            info!(
                                "nav agent handoff {agent_id:08x} -> cell {destination_cell_form_id:08x}"
                            );
                            commands.entity(entity).despawn();
                            roster.entities[index] = None;
                        }
                        None => {
                            // Defensive fallback (should not happen:
                            // `travel_doors` always carries this once
                            // `TravelReached` fired through it) -- keep
                            // #113's original behaviour rather than losing
                            // the agent silently.
                            warn!(
                                "nav agent handoff {door_form_id:08x}: no destination door metadata; agent left at the travel door"
                            );
                            commands.entity(entity).remove::<AgentTarget3d>();
                            runtime.travel_intent = None;
                            runtime.door_link = new_state;
                            runtime.active_link = None;
                        }
                    }
                }
            }
        }
    }
}

type MergeTraversalQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static mut Transform,
        &'static mut AgentKcc,
        &'static mut MergeTraversal,
        &'static mut AgentRuntime,
    ),
    With<TestNavAgentMarker>,
>;

/// Sweeps every agent currently crossing a same-cell merge portal (issue
/// #154 feature 4) toward the far portal point with the same
/// `step_agent_kcc` physics `apply_agent_physics_movement` uses for
/// ordinary movement, instead of `door_traversal_system`'s scripted lerp --
/// a portal candidate that turns out to be collision-blocked must fail
/// visibly (the same stable `nav agent collision-blocked <id>`/`nav agent
/// stuck <id>` lines ordinary movement already reports, feeding `tna
/// status`'s `blocked=`/`stuck=` fields) rather than teleport the agent
/// through geometry. Runs in the same `FixedUpdate` chain slot
/// `door_traversal_system` occupies for the door lifecycle
/// (`NavBackendPlugin::build`): the two systems drive entirely disjoint
/// entity sets (`MergeTraversal` vs. `DoorTraversal`), so ordering between
/// them does not matter, and `apply_agent_physics_movement` itself excludes
/// both marker components (`AgentPhysicsQuery`) so nothing ever
/// double-drives the same entity's `Transform` in one tick.
#[allow(clippy::too_many_arguments)]
fn merge_traversal_system(
    time: Res<Time>,
    physics_disabled: Res<PhysicsDisabled>,
    cell_physics: Res<CellPhysicsReadiness>,
    roster: Res<TestNavAgentState>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    mut agents: MergeTraversalQuery<'_, '_>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 || physics_disabled.0 || !cell_physics.static_collision_ready() {
        // Physics is not ready to collide against yet (or is globally
        // disabled): hold every in-progress crossing in place rather than
        // advance it against a world that cannot resolve collision this
        // tick, mirroring `apply_agent_physics_movement`'s own guard.
        return;
    }
    let Some(world) = context.world_mut() else {
        return;
    };
    let mover = boxddd::Capsule::new(
        [0.0, -(AGENT_HEIGHT * 0.5 - AGENT_RADIUS), 0.0],
        [0.0, AGENT_HEIGHT * 0.5 - AGENT_RADIUS, 0.0],
        AGENT_RADIUS,
    );
    let collision_filter = player::player_collision_filter();
    let support_filter = player::stair_support_filter();

    for (entity, mut transform, mut kcc, mut traversal, mut runtime) in &mut agents {
        if movement_policy::nav_point_reached(
            transform.translation.to_array(),
            traversal.target.to_array(),
            MERGE_TRAVERSAL_REACHED_DISTANCE,
            AGENT_HEIGHT,
        ) {
            commands
                .entity(entity)
                .remove::<MergeTraversal>()
                .remove::<UsingAnimationLink>();
            runtime.active_link = None;
            continue;
        }

        traversal.elapsed += dt;
        if traversal.elapsed > traversal.timeout {
            let was_reported = kcc.stuck || kcc.collision_blocked;
            kcc.stuck = true;
            kcc.collision_blocked = true;
            commands
                .entity(entity)
                .remove::<MergeTraversal>()
                .remove::<UsingAnimationLink>();
            runtime.active_link = None;
            // Minimum viable mitigation for a blocked-portal repath loop
            // (issue #154 review correction, feature 4): the agent's
            // current route/travel-door target is cleared outright rather
            // than left in place, so `LandmassSystems::Update`'s next solve
            // has nothing left to path across the same blocked link with --
            // an idle agent that goes nowhere, instead of one that
            // silently re-selects the identical invalid link and repeats
            // this exact failure forever.
            //
            // ponytail: this is not per-portal-link quarantine (excluding
            // just the blocked seam while keeping the agent's real
            // destination, so a repath can route *around* it) -- that
            // needs a landmass-side mechanism to exclude one specific
            // off-mesh link from a route, which does not exist yet for
            // merge links (F155's door work adds a per-polygon *type-index*
            // cost override, but merge-link polygons have no type tagging).
            // Left for a follow-up once that infra exists; flagged to the
            // orchestrator for its own issue.
            commands.entity(entity).remove::<AgentTarget3d>();
            runtime.travel_intent = None;
            if !was_reported && let Some(index) = roster.index_of(entity) {
                warn!(
                    "nav agent portal blocked: swept crossing did not reach the far side within {:.1}s; agent stopped mid-portal and its route was cleared",
                    traversal.timeout
                );
                info!("nav agent collision-blocked {index}");
                info!("nav agent stuck {index}");
            }
            continue;
        }

        let to_target = traversal.target - transform.translation;
        let horizontal = Vec2::new(to_target.x, to_target.z);
        let desired_horizontal = if horizontal.length() > f32::EPSILON {
            horizontal.normalize() * AGENT_DESIRED_SPEED
        } else {
            Vec2::ZERO
        };
        let (new_position, new_velocity, grounded) = step_agent_kcc(
            world,
            &mover,
            collision_filter,
            support_filter,
            transform.translation,
            kcc.velocity,
            kcc.grounded,
            desired_horizontal,
            dt,
        );
        transform.translation = new_position;
        kcc.velocity = new_velocity;
        kcc.grounded = grounded;
    }
}

/// The wall-clock deadline (seconds) for a swept merge-portal crossing of
/// `initial_distance` metres (issue #154 feature 4) -- see
/// [`MERGE_TRAVERSAL_TIMEOUT_FACTOR`]'s doc comment for why this is an
/// absolute budget rather than a resettable no-progress counter.
fn merge_traversal_timeout(initial_distance: f32) -> f32 {
    (initial_distance.max(0.0) / AGENT_DESIRED_SPEED) * MERGE_TRAVERSAL_TIMEOUT_FACTOR
        + MERGE_TRAVERSAL_TIMEOUT_FLOOR_SECONDS
}

/// Requests the door `door_form_id` open through the same boundary the
/// `activate` console command uses. A door that is currently locked is
/// deliberately *not* scripted open (issue #113 feature 3: no teleporting
/// through closed doors, and a locked door resolves to the deterministic
/// `Failed` outcome via the wait bound) -- `scripted_door_open` bypasses
/// locks by design (dev tooling), so the lock gate lives here.
fn request_door_open(world: &mut World, door_form_id: u32) {
    let lock_info = world
        .resource::<NavArchipelagoState>()
        .door_lock_info
        .clone();
    if !door_usable_now(world, door_form_id, &lock_info) {
        info!("nav agent door {door_form_id:08x} locked; waiting");
        return;
    }
    if world
        .get_resource::<crate::console::RefRegistry>()
        .is_none()
    {
        warn!("nav agent door {door_form_id:08x}: reference not resolvable");
        return;
    }
    match crate::console::resolve_reference(world, &format!("{door_form_id:08x}")) {
        Ok(door_entity) => {
            interaction::scripted_door_open(world, door_entity);
        }
        Err(_) => {
            warn!("nav agent door {door_form_id:08x}: reference not resolvable");
        }
    }
}

/// Drives the door-link lifecycle for every currently-spawned agent:
/// detects each reaching an off-mesh link (a merge seam is crossed
/// directly; a door link runs the pause -> scripted-open -> wait ->
/// traverse lifecycle) or arriving at a travel door's triangle (issue #113
/// feature 3), requests the door open through the same boundary the
/// `activate` console command uses (`interaction::scripted_door_open`),
/// polls `InteractionState.open`, and starts the kinematic crossing once
/// the door is open. An exclusive (`&mut World`) system since it needs to
/// both query components and call into `interaction`'s `&mut World`-based
/// scripted door boundary in the same step.
fn door_link_system(world: &mut World) {
    let active: Vec<Entity> = world
        .resource::<TestNavAgentState>()
        .active()
        .map(|(_, entity)| entity)
        .collect();
    for agent_entity in active {
        if world.get_entity(agent_entity).is_err() {
            continue;
        }
        drive_door_link_for_agent(world, agent_entity);
    }
}

fn drive_door_link_for_agent(world: &mut World, agent_entity: Entity) {
    let Some(current_state) = world.get::<AgentRuntime>(agent_entity).map(|r| r.door_link) else {
        return;
    };

    match current_state {
        door_link::DoorLinkState::Idle
        | door_link::DoorLinkState::Failed { .. }
        | door_link::DoorLinkState::TravelReached { .. } => {
            // Travel arrival (issue #113 feature 3): a pending travel
            // intent whose door triangle the agent has reached starts the
            // door lifecycle with a Travel destination.
            let travel_arrival = world
                .get::<AgentRuntime>(agent_entity)
                .and_then(|runtime| runtime.travel_intent)
                .and_then(|door_form_id| {
                    let link = world
                        .resource::<NavArchipelagoState>()
                        .travel_doors
                        .get(&door_form_id)
                        .copied()?;
                    let position = world.get::<Transform>(agent_entity)?.translation;
                    movement_policy::nav_point_reached(
                        position.to_array(),
                        link.triangle_midpoint.to_array(),
                        TRAVEL_ARRIVAL_DISTANCE,
                        AGENT_HEIGHT,
                    )
                    .then_some((door_form_id, link))
                });
            if let Some((door_form_id, link)) = travel_arrival {
                let new_state = door_link::transition(
                    current_state,
                    door_link::DoorLinkEvent::LinkReached {
                        door_form_id,
                        destination: door_link::LinkDestination::Travel {
                            destination_cell_form_id: link.destination_cell_form_id,
                        },
                    },
                );
                world.entity_mut(agent_entity).insert(PauseAgent);
                request_door_open(world, door_form_id);
                info!("nav agent door wait {door_form_id:08x}");
                let mut runtime = world.get_mut::<AgentRuntime>(agent_entity).unwrap();
                runtime.door_link = new_state;
                runtime.active_link = Some(LinkKind::Door {
                    form_id: door_form_id,
                });
                runtime.pending_traversal = Some((link.triangle_midpoint, link.door_position));
                return;
            }

            // Mid-route door crossing (issue #137): a door's triangle
            // reached by ordinary walking -- not a #113 link endpoint, and
            // not (see the exclusion below) the agent's own active travel
            // terminal -- has no `ReachedAnimationLink3d` event to trigger
            // on. Proximity to its midpoint is the trigger instead,
            // mirroring the travel-door arrival check just above. Only
            // considered while the agent is actually routed somewhere: an
            // idle agent standing near a closed door should not be paused.
            // Skipped in the same tick a travel arrival already fired (that
            // already `return`ed above).
            let has_target = !matches!(
                world.get::<AgentTarget3d>(agent_entity),
                None | Some(AgentTarget3d::None)
            );
            // The agent's own travel terminal (if any) is excluded from the
            // candidate set: that door's full pause -> open -> traverse ->
            // handoff lifecycle is the travel-arrival check's job, not this
            // one's -- double-gating the same door for the same agent would
            // fight it. A *different* agent's travel intent, or this
            // agent's `goto` merely crossing someone else's travel door, is
            // not excluded.
            let travel_target_door = world
                .get::<AgentRuntime>(agent_entity)
                .and_then(|runtime| runtime.travel_intent);
            let mid_route_crossing = has_target
                .then(|| world.get::<Transform>(agent_entity).map(|t| t.translation))
                .flatten()
                .and_then(|position| {
                    world
                        .resource::<NavArchipelagoState>()
                        .mid_route_doors
                        .iter()
                        .find(|door| {
                            Some(door.door_form_id) != travel_target_door
                                && movement_policy::nav_point_reached(
                                    position.to_array(),
                                    door.midpoint.to_array(),
                                    MID_ROUTE_DOOR_GATE_DISTANCE,
                                    AGENT_HEIGHT,
                                )
                        })
                        .copied()
                });
            if let Some(door) = mid_route_crossing {
                let lock_info = world
                    .resource::<NavArchipelagoState>()
                    .door_lock_info
                    .clone();
                let (door_open, door_locked) =
                    door_open_and_locked(world, door.door_form_id, &lock_info);
                let gate = door_link::crossing_gate(door_link::CrossingObservation {
                    door_open,
                    door_locked,
                });
                if gate != door_link::CrossingGate::Pass {
                    let new_state = door_link::transition(
                        current_state,
                        door_link::DoorLinkEvent::LinkReached {
                            door_form_id: door.door_form_id,
                            destination: door_link::LinkDestination::IntraCell,
                        },
                    );
                    world.entity_mut(agent_entity).insert(PauseAgent);
                    request_door_open(world, door.door_form_id);
                    info!("nav agent door wait {:08x}", door.door_form_id);
                    let mut runtime = world.get_mut::<AgentRuntime>(agent_entity).unwrap();
                    runtime.door_link = new_state;
                    runtime.active_link = Some(LinkKind::Door {
                        form_id: door.door_form_id,
                    });
                    // No `pending_traversal`: the agent is already standing
                    // on continuous walkable ground (there is no off-mesh
                    // gap to lerp across). The `Paused` arm below treats an
                    // absent `pending_traversal` as an instant
                    // `TraversalComplete` on resume.
                    return;
                }
                // Already open: plain walkable ground, nothing to gate.
                // Fall through to the (extremely unlikely) chance the same
                // tick also reached an unrelated animation link.
            }

            let Some(reached) = world.get::<ReachedAnimationLink3d>(agent_entity) else {
                return;
            };
            let link_entity = reached.link_entity;
            let start_point = reached.start_point;
            let end_point = reached.end_point;
            let Some(&link_kind) = world
                .resource::<NavArchipelagoState>()
                .link_kinds
                .get(&link_entity)
            else {
                return;
            };
            match link_kind {
                LinkKind::Merge => {
                    // A merge seam has no door to wait on (issue #154
                    // feature 4): sweep the agent to the far portal point
                    // with the physics KCC (`merge_traversal_system`)
                    // instead of the door lifecycle's scripted lerp -- a
                    // portal whose far side is actually blocked must stop
                    // the agent for real, not clip it through. `start_point`
                    // is unused here (unlike a door traversal's fixed lerp
                    // start): the sweep simply starts from wherever the
                    // agent's `Transform` actually is this tick. The
                    // timeout budget is derived from the *actual* current
                    // position, not `start_point`, for the same reason.
                    let initial_distance = world
                        .get::<Transform>(agent_entity)
                        .map(|transform| {
                            movement_policy::horizontal_distance(
                                transform.translation.to_array(),
                                end_point.to_array(),
                            )
                        })
                        .unwrap_or(0.0);
                    world.entity_mut(agent_entity).insert((
                        UsingAnimationLink,
                        MergeTraversal {
                            target: end_point,
                            elapsed: 0.0,
                            timeout: merge_traversal_timeout(initial_distance),
                        },
                    ));
                    world
                        .get_mut::<AgentRuntime>(agent_entity)
                        .unwrap()
                        .active_link = Some(LinkKind::Merge);
                }
                LinkKind::Door {
                    form_id: door_form_id,
                } => {
                    let new_state = door_link::transition(
                        current_state,
                        door_link::DoorLinkEvent::LinkReached {
                            door_form_id,
                            destination: door_link::LinkDestination::IntraCell,
                        },
                    );
                    world.entity_mut(agent_entity).insert(PauseAgent);
                    request_door_open(world, door_form_id);
                    info!("nav agent door wait {door_form_id:08x}");
                    let mut runtime = world.get_mut::<AgentRuntime>(agent_entity).unwrap();
                    runtime.door_link = new_state;
                    runtime.active_link = Some(link_kind);
                    runtime.pending_traversal = Some((start_point, end_point));
                }
            }
        }
        door_link::DoorLinkState::Paused { door_form_id, .. } => {
            let door_open = world
                .get_resource::<crate::console::RefRegistry>()
                .is_some()
                && crate::console::resolve_reference(world, &format!("{door_form_id:08x}"))
                    .ok()
                    .is_some_and(|door_entity| {
                        world
                            .resource::<interaction::InteractionState>()
                            .open
                            .contains(&door_entity)
                    });
            let new_state =
                door_link::transition(current_state, door_link::DoorLinkEvent::Tick { door_open });
            if door_link::is_traversing(new_state) {
                world.entity_mut(agent_entity).remove::<PauseAgent>();
                let pending = world
                    .get_mut::<AgentRuntime>(agent_entity)
                    .unwrap()
                    .pending_traversal
                    .take();
                match pending {
                    Some((start, end)) => {
                        world.entity_mut(agent_entity).insert((
                            UsingAnimationLink,
                            DoorTraversal {
                                start,
                                end,
                                elapsed: 0.0,
                            },
                        ));
                        world
                            .get_mut::<AgentRuntime>(agent_entity)
                            .unwrap()
                            .door_link = new_state;
                    }
                    None => {
                        // Mid-route door (issue #137): no off-mesh gap to
                        // cross -- the agent is already standing on
                        // continuous walkable ground, so resuming
                        // completes the crossing in the same tick instead
                        // of waiting on `door_traversal_system` (which only
                        // drives entities carrying `DoorTraversal`).
                        let mut runtime = world.get_mut::<AgentRuntime>(agent_entity).unwrap();
                        runtime.door_link = door_link::transition(
                            new_state,
                            door_link::DoorLinkEvent::TraversalComplete,
                        );
                        runtime.active_link = None;
                    }
                }
                info!("nav agent door resume {door_form_id:08x}");
            } else if door_link::is_failed(new_state) {
                warn!(
                    "nav agent door {door_form_id:08x}: gave up waiting for it to open; agent stopped at the link"
                );
                info!("nav agent unreachable");
                let mut runtime = world.get_mut::<AgentRuntime>(agent_entity).unwrap();
                runtime.active_link = None;
                runtime.travel_intent = None;
                runtime.door_link = new_state;
            } else {
                world
                    .get_mut::<AgentRuntime>(agent_entity)
                    .unwrap()
                    .door_link = new_state;
            }
        }
        door_link::DoorLinkState::Traversing { .. } => {
            // `door_traversal_system` owns the crossing and emits
            // `TraversalComplete` once it finishes.
        }
    }
}

/// Polls every tracked door's usability once per frame and reacts to
/// *changes* only (issue #113 feature 4): the pure `repath::decide` table
/// turns a flip into a repath, applied here as (a) spawning/despawning the
/// affected two-sided door link so route planning includes/excludes it, (b)
/// re-inserting the agent's current target so landmass replans, and (c)
/// while paused at that very door, requesting the (now unlocked) door open.
/// Exactly one repath per actual state change -- the cached `door_usable`
/// map is the change detector.
fn door_availability_system(world: &mut World) {
    let tracked: Vec<(u32, bool)> = world
        .resource::<NavArchipelagoState>()
        .door_usable
        .iter()
        .map(|(&form_id, &usable)| (form_id, usable))
        .collect();
    if tracked.is_empty() {
        return;
    }
    let lock_info = world
        .resource::<NavArchipelagoState>()
        .door_lock_info
        .clone();
    for (door_form_id, was_usable) in tracked {
        let now_usable = door_usable_now(world, door_form_id, &lock_info);
        if now_usable == was_usable {
            continue;
        }
        world
            .resource_mut::<NavArchipelagoState>()
            .door_usable
            .insert(door_form_id, now_usable);

        let observation = repath::RepathObservation {
            door_became_blocked: !now_usable,
            door_became_unblocked: now_usable,
            ..Default::default()
        };
        if repath::decide(observation) != repath::RepathDecision::Repath {
            continue;
        }

        // Structural link update for two-sided door links.
        if now_usable {
            let blocked = {
                let mut state = world.resource_mut::<NavArchipelagoState>();
                let index = state
                    .blocked_door_links
                    .iter()
                    .position(|link| link.door_form_id == door_form_id);
                index.map(|index| state.blocked_door_links.remove(index))
            };
            if let Some(link) = blocked {
                let archipelago_entity = world
                    .resource::<NavArchipelagoState>()
                    .archipelago
                    .expect("availability tracking implies a built archipelago");
                for link_entity in
                    spawn_link_pair(world, archipelago_entity, link.start, link.end, 1.0)
                {
                    let mut state = world.resource_mut::<NavArchipelagoState>();
                    state.link_kinds.insert(
                        link_entity,
                        LinkKind::Door {
                            form_id: door_form_id,
                        },
                    );
                    state.links.push(link_entity);
                }
            }
        } else {
            let removed: Vec<Entity> = {
                let state = world.resource::<NavArchipelagoState>();
                state
                    .link_kinds
                    .iter()
                    .filter(|(_, kind)| {
                        matches!(kind, LinkKind::Door { form_id } if *form_id == door_form_id)
                    })
                    .map(|(&entity, _)| entity)
                    .collect()
            };
            // The door's link is spawned as a unidirectional pair (see
            // `spawn_link_pair`); despawn every entity but record only one
            // blocked entry, from the first entity's own orientation, so a
            // later unblock respawns exactly one pair.
            let mut recorded = false;
            for link_entity in removed {
                let (start, end) = world
                    .get::<AnimationLink3d>(link_entity)
                    .map(|link| (link.start_edge.0, link.end_edge.0))
                    .unwrap_or_default();
                if let Ok(entity) = world.get_entity_mut(link_entity) {
                    entity.despawn();
                }
                let mut state = world.resource_mut::<NavArchipelagoState>();
                state.link_kinds.remove(&link_entity);
                state.links.retain(|entity| *entity != link_entity);
                if !recorded {
                    state.blocked_door_links.push(BlockedDoorLink {
                        door_form_id,
                        start,
                        end,
                    });
                    recorded = true;
                }
            }
        }

        // Route refresh: re-insert every active agent's current target so
        // landmass replans with the updated link set. `AgentTarget3d` is not
        // `Clone`; rebuild the equivalent value by matching its variants.
        let active_agents: Vec<Entity> = world
            .resource::<TestNavAgentState>()
            .active()
            .map(|(_, entity)| entity)
            .collect();
        for agent_entity in &active_agents {
            let target =
                world
                    .get::<AgentTarget3d>(*agent_entity)
                    .and_then(|target| match target {
                        AgentTarget3d::None => None,
                        AgentTarget3d::Point(point) => Some(AgentTarget3d::Point(*point)),
                        AgentTarget3d::Entity(entity) => Some(AgentTarget3d::Entity(*entity)),
                    });
            if let Some(target) = target {
                world.entity_mut(*agent_entity).insert(target);
            }
        }

        // Any agent paused waiting on this exact door can now proceed.
        if now_usable {
            let paused_on_this_door = active_agents.iter().copied().any(|agent_entity| {
                matches!(
                    world.get::<AgentRuntime>(agent_entity).map(|r| r.door_link),
                    Some(door_link::DoorLinkState::Paused { door_form_id: paused, .. }) if paused == door_form_id
                )
            });
            if paused_on_this_door {
                request_door_open(world, door_form_id);
            }
        }

        info!(
            "nav agent repath door {door_form_id:08x} {}",
            if now_usable { "unblocked" } else { "blocked" }
        );
    }
}

/// Routes agent `index` to `door_form_id`'s travel-door triangle and arms
/// the travel lifecycle (issue #113 feature 3; indexed #114). The traversal
/// terminates at the door with the `TravelReached` status;
/// `door_traversal_system` (issue #134) consumes that seam and hands the
/// agent off to the destination cell. Wired to the console as `tna travel
/// [<index>] <door-formid>`.
pub(crate) fn request_travel(
    world: &mut World,
    index: usize,
    door_form_id: u32,
) -> Result<(), ConsoleError> {
    let Some(agent_entity) = world.resource::<TestNavAgentState>().entities[index] else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned at this index; use tna spawn first",
        ));
    };
    if world
        .get::<AgentRuntime>(agent_entity)
        .is_some_and(|runtime| runtime.travel_intent.is_some())
    {
        return Err(ConsoleError::new(
            "travel_in_progress",
            "a travel request is already in progress",
        ));
    }
    let Some(link) = world
        .resource::<NavArchipelagoState>()
        .travel_doors
        .get(&door_form_id)
        .copied()
    else {
        return Err(ConsoleError::new(
            "unknown_travel_door",
            format!("no travel door {door_form_id:08x} is reachable from this cell's nav mesh"),
        ));
    };
    world
        .entity_mut(agent_entity)
        .insert(AgentTarget3d::Point(link.triangle_midpoint));
    if let Some(mut runtime) = world.get_mut::<AgentRuntime>(agent_entity) {
        runtime.travel_intent = Some(door_form_id);
    }
    info!("nav agent {index} travel start {door_form_id:08x}");
    Ok(())
}

/// Logs the stable evidence lines exactly once per actual state change, for
/// every currently-spawned agent. `bevy_landmass`'s `sync_agent_state`
/// rewrites `AgentState` every frame (Bevy change detection triggers on the
/// write, not the value), so a `Changed<AgentState>` filter would re-log
/// every frame; the previous value is tracked per agent in `AgentRuntime`
/// instead. `AgentState::AgentNotOnNavMesh` is the off-navmesh diagnostic
/// now that physics (#114), not navmesh sampling, is ground authority --
/// its own stable `nav agent off-navmesh <id>` line.
fn log_agent_state_changes(
    mut agents: Query<(Entity, &AgentState, &mut AgentRuntime), With<TestNavAgentMarker>>,
    roster: Res<TestNavAgentState>,
) {
    for (entity, agent_state, mut runtime) in &mut agents {
        if runtime.last_logged_state == Some(*agent_state) {
            continue;
        }
        runtime.last_logged_state = Some(*agent_state);
        let Some(index) = roster.index_of(entity) else {
            continue;
        };
        match agent_state {
            AgentState::ReachedTarget => info!("nav agent {index} reached"),
            AgentState::AgentNotOnNavMesh => info!("nav agent off-navmesh {index}"),
            AgentState::TargetNotOnNavMesh | AgentState::NoPath => {
                info!("nav agent {index} unreachable state={agent_state:?}");
            }
            _ => {}
        }
    }
}

fn log_path_latency(
    time: Res<Time>,
    mut agents: Query<(Entity, &AgentState, &mut AgentRuntime), With<TestNavAgentMarker>>,
    roster: Res<TestNavAgentState>,
) {
    for (entity, agent_state, mut runtime) in &mut agents {
        if runtime.latency_logged {
            continue;
        }
        let Some(started_at) = runtime.goto_started_at else {
            continue;
        };
        if matches!(agent_state, AgentState::Moving | AgentState::ReachedTarget) {
            let Some(index) = roster.index_of(entity) else {
                continue;
            };
            let latency_ms = (time.elapsed_secs() - started_at) * 1000.0;
            info!("nav agent {index} path latency_ms={latency_ms:.1}");
            runtime.latency_logged = true;
        }
    }
}

/// Mirrors `nav_overlay::despawn_stale_nav_overlay`'s pattern: the moment
/// the active cell no longer matches the archipelago's cell, tear it down.
/// `PreparedSceneManifest` is optional so this system never panics in a
/// console-harness test world that never inserted one.
///
/// Issue #134 shipped amendment: wave 3's original behaviour despawned any
/// live test agent along with the stale archipelago, silently losing it.
/// That is replaced with `ledger_departing_agent`, which runs *before*
/// `teardown_archipelago` (it needs `NavArchipelagoState.travel_doors`,
/// still populated for the departing cell) and ledgers the agent instead --
/// follow-through to the destination cell if `note_player_swap_door` noted
/// the exact door the agent's active route was targeting, otherwise frozen
/// in place in the departing cell.
fn despawn_stale_navmesh_archipelago(world: &mut World) {
    let Some(current_cell) = world
        .get_resource::<crate::viewer::LoadedSceneManifest>()
        .map(|manifest| manifest.cell.form_id)
    else {
        return;
    };
    let Some(source_cell) = world
        .resource::<NavArchipelagoState>()
        .cell_form_id
        .filter(|&cell| cell != current_cell)
    else {
        return;
    };
    let used_door = world.resource_mut::<PendingPlayerSwapDoor>().0.take();
    ledger_departing_agent(world, source_cell, used_door);
    teardown_archipelago(world);
}

/// The point-target component of `AgentTarget3d`, if any -- an `Entity`
/// target (e.g. `tna goto player`) cannot be meaningfully frozen, since the
/// target entity will not exist once the agent is restored, so it is
/// dropped rather than ledgered.
fn point_target(target: &AgentTarget3d) -> Option<[f32; 3]> {
    match target {
        AgentTarget3d::Point(point) => Some(point.to_array()),
        _ => None,
    }
}

/// Issue #134's player-initiated-swap ledgering (multi-agent since #114):
/// despawns every live test agent into `NavAgentLedger`, deciding
/// follow-through vs. freeze per agent via
/// `ledger_policy::decide_swap_eligibility`. Must run before
/// `teardown_archipelago` clears `NavArchipelagoState.travel_doors`, the
/// source of a follow-through's destination-door metadata.
fn ledger_departing_agent(world: &mut World, source_cell: u32, used_door: Option<u32>) {
    let active: Vec<(usize, Entity)> = world.resource::<TestNavAgentState>().active().collect();
    for (index, agent_entity) in active {
        ledger_departing_one_agent(world, index, agent_entity, source_cell, used_door);
    }
    *world.resource_mut::<TestNavAgentState>() = TestNavAgentState::default();
}

fn ledger_departing_one_agent(
    world: &mut World,
    index: usize,
    agent_entity: Entity,
    source_cell: u32,
    used_door: Option<u32>,
) {
    let agent_id = agent_ledger_id(index);
    let route_door = world
        .get::<AgentRuntime>(agent_entity)
        .and_then(|runtime| runtime.travel_intent);
    let position = world
        .get::<Transform>(agent_entity)
        .map(|transform| transform.translation.to_array())
        .unwrap_or_default();
    let remaining_target = world
        .get::<AgentTarget3d>(agent_entity)
        .and_then(point_target);

    let follow_through_link = used_door.and_then(|door| {
        let eligible = ledger_policy::decide_swap_eligibility(route_door, door)
            == ledger_policy::SwapEligibility::FollowThrough;
        eligible
            .then(|| {
                world
                    .resource::<NavArchipelagoState>()
                    .travel_doors
                    .get(&door)
                    .copied()
            })
            .flatten()
    });

    if let Some(link) = follow_through_link {
        world
            .resource_mut::<NavAgentLedger>()
            .0
            .record(ledger_policy::LedgerEntry {
                agent_id,
                cell_form_id: link.destination_cell_form_id,
                spawn_kind: ledger_policy::SpawnKind::DoorMarker {
                    destination_door_form_id: link.destination_door_form_id,
                },
                remaining_target: None,
            });
        info!(
            "nav agent handoff {agent_id:08x} -> cell {:08x}",
            link.destination_cell_form_id
        );
    } else {
        world
            .resource_mut::<NavAgentLedger>()
            .0
            .record(ledger_policy::LedgerEntry {
                agent_id,
                cell_form_id: source_cell,
                spawn_kind: ledger_policy::SpawnKind::FrozenPosition { position },
                remaining_target,
            });
        info!("nav agent freeze {agent_id:08x} cell {source_cell:08x}");
    }

    if let Ok(entity) = world.get_entity_mut(agent_entity) {
        entity.despawn();
    }
}

/// The placed position of the door reference `door_form_id` in the
/// *active* cell's manifest, if it exists there (issue #134's `DoorMarker`
/// spawn resolution).
fn door_position_in_active_cell(world: &World, door_form_id: u32) -> Option<Vec3> {
    world
        .get_resource::<crate::viewer::LoadedSceneManifest>()?
        .placements
        .iter()
        .find(|placement| placement.reference_form_id == door_form_id)
        .map(|placement| Vec3::from_array(placement.translation))
}

/// Issue #134's restore side: once a cell containing ledgered entries
/// becomes active, claims them (`ledger_policy::Ledger::claim_for_
/// activation`) and spawns each restored entry, or diagnoses a stale
/// `DoorMarker` entry whose destination door is missing from this cell's
/// manifest. Cheap no-op the overwhelming majority of frames: bails before
/// touching the archipelago unless the ledger actually holds an entry for
/// the active cell.
fn restore_ledgered_agents_system(world: &mut World) {
    let Some(current_cell) = world
        .get_resource::<crate::viewer::LoadedSceneManifest>()
        .map(|manifest| manifest.cell.form_id)
    else {
        return;
    };
    let has_pending = (0..MAX_TEST_AGENTS).any(|index| {
        world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(agent_ledger_id(index))
            .is_some_and(|entry| entry.cell_form_id == current_cell)
    });
    if !has_pending {
        return;
    }

    let known_door_form_ids: std::collections::HashSet<u32> = world
        .resource::<crate::viewer::LoadedSceneManifest>()
        .placements
        .iter()
        .filter(|placement| matches!(placement.semantic, crate::vsa::PreparedSemantic::Door(_)))
        .map(|placement| placement.reference_form_id)
        .collect();

    let claim = world
        .resource_mut::<NavAgentLedger>()
        .0
        .claim_for_activation(current_cell, &known_door_form_ids);

    for stale in claim.stale {
        warn!(
            "nav agent ledger stale {:08x} cell {:08x}: destination door {:08x} absent from the active cell",
            stale.agent_id, stale.cell_form_id, stale.missing_door_form_id
        );
    }

    for entry in claim.restored {
        let Some(index) = entry
            .agent_id
            .checked_sub(1)
            .map(|zero_based| zero_based as usize)
            .filter(|&index| index < MAX_TEST_AGENTS)
        else {
            warn!(
                "nav agent restore {:08x} cell {:08x}: agent id outside the bounded roster; entry dropped",
                entry.agent_id, entry.cell_form_id
            );
            continue;
        };
        // An entry is only ever ledgered while no entity exists at that
        // index, so a live entity here means restoration already happened
        // this activation (or `tna spawn` ran first) -- do not double-spawn.
        if world.resource::<TestNavAgentState>().entities[index].is_some() {
            continue;
        }
        restore_ledgered_agent(world, index, entry);
    }
}

fn restore_ledgered_agent(world: &mut World, index: usize, entry: ledger_policy::LedgerEntry) {
    if ensure_archipelago(world).is_err() {
        warn!(
            "nav agent restore {:08x} cell {:08x}: no nav graph for this cell; ledger entry dropped",
            entry.agent_id, entry.cell_form_id
        );
        return;
    }
    let position = match entry.spawn_kind {
        ledger_policy::SpawnKind::FrozenPosition { position } => Vec3::from_array(position),
        ledger_policy::SpawnKind::DoorMarker {
            destination_door_form_id,
        } => match door_position_in_active_cell(world, destination_door_form_id) {
            Some(position) => position,
            None => {
                // `restore_ledgered_agents_system` already filtered stale
                // entries against `known_door_form_ids` from the same
                // manifest this reads, so this should not happen; guarded
                // defensively rather than trusted.
                warn!(
                    "nav agent restore {:08x} cell {:08x}: destination door {destination_door_form_id:08x} placement missing; ledger entry dropped",
                    entry.agent_id, entry.cell_form_id
                );
                return;
            }
        },
    };
    let agent_entity = spawn_test_agent(world, position);
    if let Some(target) = entry.remaining_target {
        world
            .entity_mut(agent_entity)
            .insert(AgentTarget3d::Point(Vec3::from_array(target)));
    }
    world.resource_mut::<TestNavAgentState>().entities[index] = Some(agent_entity);
    info!(
        "nav agent restore {:08x} cell {:08x}",
        entry.agent_id, entry.cell_form_id
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::console::ConsoleSessionId;
    use bevy_boxddd::boxddd::{BodyDef, BodyType, BoxHull, Filter, ShapeDef};

    fn invocation(args: &[&str]) -> ConsoleInvocation {
        ConsoleInvocation {
            request_id: 1,
            frame: 1,
            session: ConsoleSessionId::new("test"),
            command: "tna".into(),
            args: args.iter().map(|arg| arg.to_string()).collect(),
            target: None,
        }
    }

    fn harness_world() -> World {
        let mut world = World::new();
        world.init_resource::<NavArchipelagoState>();
        world.init_resource::<TestNavAgentState>();
        world.init_resource::<NavAgentLedger>();
        world.init_resource::<PendingPlayerSwapDoor>();
        world
    }

    /// A permissive `boxddd` collision filter/shape pairing scoped to these
    /// tests: `step_agent_kcc` takes its filters as parameters, so a fixture
    /// world only needs *a* consistent category/mask pair, not the real
    /// player categories (those are private to `player/mod.rs`).
    fn fixture_filter() -> boxddd::QueryFilter {
        boxddd::QueryFilter::new().category_bits(1).mask_bits(1)
    }

    fn fixture_shape_def() -> ShapeDef {
        ShapeDef::builder()
            .filter(Filter {
                category_bits: 1,
                mask_bits: 1,
                group_index: 0,
            })
            .build()
    }

    fn fixture_capsule() -> boxddd::Capsule {
        boxddd::Capsule::new(
            [0.0, -(AGENT_HEIGHT * 0.5 - AGENT_RADIUS), 0.0],
            [0.0, AGENT_HEIGHT * 0.5 - AGENT_RADIUS, 0.0],
            AGENT_RADIUS,
        )
    }

    fn add_fixture_box(
        world: &mut boxddd::World,
        center: boxddd::Vec3,
        half_extents: boxddd::Vec3,
    ) {
        let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
        world.create_hull_shape(
            body,
            &fixture_shape_def(),
            &BoxHull::transformed(
                half_extents.x,
                half_extents.y,
                half_extents.z,
                boxddd::Transform::new(center, boxddd::Quat::IDENTITY),
            ),
        );
    }

    /// Issue #114: the navmesh `sample_point` Y-snap from wave 3's kinematic
    /// spike is gone -- physics is ground authority now. Drops the agent
    /// capsule from above a flat floor collider through `step_agent_kcc`
    /// (the same free helper `apply_agent_physics_movement` calls) with no
    /// landmass/App involved at all, and asserts it settles to rest on the
    /// floor via real `boxddd` collision.
    #[test]
    fn agent_kcc_settles_onto_a_flat_floor_via_physics_collision() {
        let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
        // Floor top face at y = 0.0.
        add_fixture_box(
            &mut world,
            boxddd::Vec3::new(0.0, -0.1, 0.0),
            boxddd::Vec3::new(5.0, 0.1, 5.0),
        );
        let mover = fixture_capsule();
        let filter = fixture_filter();

        let mut position = Vec3::new(0.0, 3.0, 0.0);
        let mut velocity = Vec3::ZERO;
        let mut grounded = false;
        for _ in 0..300 {
            let (new_position, new_velocity, new_grounded) = step_agent_kcc(
                &mut world,
                &mover,
                filter,
                filter,
                position,
                velocity,
                grounded,
                Vec2::ZERO,
                1.0 / 60.0,
            );
            position = new_position;
            velocity = new_velocity;
            grounded = new_grounded;
            if grounded {
                break;
            }
        }
        assert!(grounded, "agent must come to rest on the floor via physics");
        let expected_y = AGENT_HEIGHT / 2.0;
        assert!(
            (position.y - expected_y).abs() < 0.05,
            "agent y should settle near the floor (expected {expected_y}), got {}",
            position.y
        );
        assert_eq!(
            velocity.y, 0.0,
            "vertical velocity is cleared once grounded"
        );
    }

    /// Issue #114 minimal-World test: desired vs. actual velocity feedback.
    /// A wall square in front of a grounded agent means the KCC sweep
    /// achieves (near-)zero horizontal displacement no matter what landmass
    /// desired -- `movement_policy::decide_collision_outcome` must classify
    /// that as `Blocked`, and the achieved velocity handed back to landmass
    /// is the real, near-zero one, not the desired one.
    #[test]
    fn a_blocked_agent_reports_its_real_near_zero_velocity() {
        let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
        add_fixture_box(
            &mut world,
            boxddd::Vec3::new(0.0, -0.1, 0.0),
            boxddd::Vec3::new(5.0, 0.1, 5.0),
        );
        // A wall immediately in front (+X) of the agent's start position.
        add_fixture_box(
            &mut world,
            boxddd::Vec3::new(1.0, 1.0, 0.0),
            boxddd::Vec3::new(0.1, 2.0, 5.0),
        );
        let mover = fixture_capsule();
        let filter = fixture_filter();
        let desired_horizontal = Vec2::new(AGENT_DESIRED_SPEED, 0.0);

        // Walk toward the wall for two seconds -- plenty of time to close
        // the ~0.55 m gap and press into it -- then look at the final
        // tick's achieved displacement, once the agent is pinned.
        let mut position = Vec3::new(0.0, AGENT_HEIGHT / 2.0, 0.0);
        let mut velocity = Vec3::ZERO;
        let mut grounded = true;
        let mut achieved = Vec3::ZERO;
        for _ in 0..120 {
            let (new_position, new_velocity, new_grounded) = step_agent_kcc(
                &mut world,
                &mover,
                filter,
                filter,
                position,
                velocity,
                grounded,
                desired_horizontal,
                1.0 / 60.0,
            );
            achieved = (new_position - position) / (1.0 / 60.0);
            position = new_position;
            velocity = new_velocity;
            grounded = new_grounded;
        }
        assert!(grounded, "agent stays grounded while blocked by a wall");
        assert!(
            Vec2::new(achieved.x, achieved.z).length() < 0.2,
            "achieved horizontal speed should be near zero pinned against a wall, got {achieved:?}"
        );
        let outcome =
            movement_policy::decide_collision_outcome(movement_policy::VelocityObservation {
                desired_horizontal_speed: desired_horizontal.length(),
                achieved_horizontal_speed: Vec2::new(achieved.x, achieved.z).length(),
            });
        assert_eq!(outcome, movement_policy::CollisionOutcome::Blocked);
        assert_eq!(
            velocity.x, desired_horizontal.x,
            "the KCC's own remembered velocity still reflects the input -- it is the *achieved transform delta*, not this, that gets fed back to landmass"
        );
    }

    /// Issue #114 minimal-World test: grounded gating on
    /// `CellPhysicsReadiness`, mirroring the player controller's own guard
    /// (`player/movement.rs::apply_player_controls`). While the destination
    /// cell's static collision has not finished building, the agent must
    /// not move through geometry that is not there yet -- velocity and
    /// grounded state are forced to zero/false every tick.
    #[test]
    fn physics_movement_zeroes_velocity_while_cell_physics_is_building() {
        use bevy::ecs::system::RunSystemOnce;

        let mut world = World::new();
        world.init_resource::<TestNavAgentState>();
        world.insert_resource(PhysicsDisabled(false));
        world.insert_resource(CellPhysicsReadiness::BuildingStatic);
        world.init_resource::<Time>();
        world
            .resource_mut::<Time>()
            .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
        world.insert_non_send(BoxdddPhysicsContext::disabled());
        world.init_resource::<NavSolveStepCounter>();
        world.init_resource::<NavSolveRate>();

        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentKcc {
                    velocity: Vec3::splat(3.0),
                    grounded: true,
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
                Velocity3d::default(),
                AgentDesiredVelocityBlend::default(),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);

        world
            .run_system_once(apply_agent_physics_movement)
            .expect("system runs");

        let kcc = world.get::<AgentKcc>(agent).unwrap();
        assert_eq!(kcc.velocity, Vec3::ZERO);
        assert!(!kcc.grounded);
        assert_eq!(world.get::<Velocity3d>(agent).unwrap().velocity, Vec3::ZERO);
    }

    /// Regression test (issue #114 added scope, M4 wave 5 real-data
    /// acceptance finding): the stuck-vs-target distance must also compare
    /// on the horizontal plane, exactly like the two door-proximity gates
    /// above. A target sitting directly below/above the agent (same X/Z,
    /// wildly different Y -- capsule-centre vs. feet-level, or simply a
    /// route target at a different storey) must never latch `stuck` purely
    /// from that vertical gap as long as the agent is not moving away from
    /// it horizontally: a 3D distance check would never close that gap and
    /// would falsely report `stuck` at a target the agent has, on the
    /// ground plane that actually matters for navigation, already reached.
    #[test]
    fn stuck_detection_does_not_false_trigger_against_a_vertically_offset_target() {
        use bevy::ecs::system::RunSystemOnce;

        let mut world = World::new();
        world.init_resource::<TestNavAgentState>();
        world.insert_resource(PhysicsDisabled(false));
        world.insert_resource(CellPhysicsReadiness::Ready);
        world.init_resource::<Time>();
        world
            .resource_mut::<Time>()
            .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
        world.insert_non_send(BoxdddPhysicsContext::from_world(
            boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world"),
        ));
        world.init_resource::<NavSolveStepCounter>();
        world.init_resource::<NavSolveRate>();

        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentKcc::default(),
                Transform::from_xyz(0.0, 5.0, 0.0),
                Velocity3d::default(),
                // No desired motion at all: the agent stays put on X/Z
                // (only falling under gravity in an empty physics world),
                // exactly on top of its target's X/Z but ~5 m above its Y.
                AgentDesiredVelocityBlend::default(),
                AgentTarget3d::Point(Vec3::new(0.0, 0.0, 0.0)),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);

        let ticks =
            movement_policy::STUCK_RECOVERY_TICKS + movement_policy::STUCK_FAILURE_TICKS + 10;
        for _ in 0..ticks {
            world
                .run_system_once(apply_agent_physics_movement)
                .expect("system runs");
        }

        let kcc = world.get::<AgentKcc>(agent).unwrap();
        assert!(
            !kcc.stuck,
            "a target directly above/below the agent must never latch `stuck` via the horizontal-only distance check"
        );
    }

    /// Issue #154 feature 4: a clear merge-portal crossing sweeps the
    /// agent to the far portal point (not an instant teleport/lerp -- the
    /// KCC needs several ticks to physically cover the distance) and clears
    /// `MergeTraversal`/`UsingAnimationLink`/`active_link` once it arrives.
    #[test]
    fn merge_traversal_system_sweeps_the_agent_to_the_far_portal_point() {
        use bevy::ecs::system::RunSystemOnce;

        let mut world = World::new();
        world.init_resource::<TestNavAgentState>();
        world.insert_resource(PhysicsDisabled(false));
        world.insert_resource(CellPhysicsReadiness::Ready);
        world.init_resource::<Time>();
        world
            .resource_mut::<Time>()
            .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
        // `merge_traversal_system` collides against the real
        // `player::player_collision_filter()`/`stair_support_filter()`
        // queries (same as `apply_agent_physics_movement`), so the fixture
        // geometry must use `add_player_compatible_floor`'s filter, not
        // `add_fixture_box`'s self-consistent-but-unrelated one (that
        // mismatch was the root cause of an earlier version of this test
        // free-falling straight through the floor).
        let mut physics_world =
            boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
        add_player_compatible_floor(
            &mut physics_world,
            boxddd::Vec3::new(0.0, -0.1, 0.0),
            boxddd::Vec3::new(5.0, 0.1, 5.0),
        );
        world.insert_non_send(BoxdddPhysicsContext::from_world(physics_world));

        let target = Vec3::new(2.0, AGENT_HEIGHT / 2.0, 0.0);
        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentKcc {
                    grounded: true,
                    ..default()
                },
                Transform::from_xyz(0.0, AGENT_HEIGHT / 2.0, 0.0),
                AgentRuntime {
                    active_link: Some(LinkKind::Merge),
                    ..default()
                },
                MergeTraversal {
                    target,
                    elapsed: 0.0,
                    timeout: merge_traversal_timeout(2.0),
                },
                UsingAnimationLink,
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);

        // Plenty of ticks for the KCC (desired speed 2.5 m/s) to cover the
        // 2 m crossing on flat open ground.
        for _ in 0..180 {
            world
                .run_system_once(merge_traversal_system)
                .expect("system runs");
            if world.get::<MergeTraversal>(agent).is_none() {
                break;
            }
        }

        assert!(
            world.get::<MergeTraversal>(agent).is_none(),
            "a clear crossing must complete and remove MergeTraversal"
        );
        assert!(world.get::<UsingAnimationLink>(agent).is_none());
        assert_eq!(world.get::<AgentRuntime>(agent).unwrap().active_link, None);
        let position = world.get::<Transform>(agent).unwrap().translation;
        assert!(
            (position.x - target.x).abs() < MERGE_TRAVERSAL_REACHED_DISTANCE + 0.1,
            "agent should have swept to the far portal point, got {position:?}"
        );
        let kcc = world.get::<AgentKcc>(agent).unwrap();
        assert!(!kcc.stuck, "a clear crossing must never latch stuck");
        assert!(!kcc.collision_blocked);
    }

    /// Issue #154 feature 4: a merge-portal crossing whose far side is
    /// walled off must fail visibly through the existing stuck/blocked
    /// reporting (`kcc.stuck`/`kcc.collision_blocked`, the same fields
    /// `tna status` and the stable `nav agent stuck <id>`/`nav agent
    /// collision-blocked <id>` log lines already use) rather than
    /// teleporting the agent through the wall via a scripted lerp.
    #[test]
    fn merge_traversal_system_reports_stuck_when_the_portal_is_walled_off() {
        use bevy::ecs::system::RunSystemOnce;

        let mut world = World::new();
        world.init_resource::<TestNavAgentState>();
        world.insert_resource(PhysicsDisabled(false));
        world.insert_resource(CellPhysicsReadiness::Ready);
        world.init_resource::<Time>();
        world
            .resource_mut::<Time>()
            .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
        // Same real-player-filter requirement as the sweep test above: both
        // the floor and the wall must use `add_player_compatible_floor`'s
        // filter, not `add_fixture_box`'s, or `merge_traversal_system`'s
        // real collision query never sees either shape and the agent free-
        // falls through both -- which happened to still end in `stuck` (via
        // the vertical-gap guard on `nav_point_reached` never being
        // satisfied while falling) for the wrong reason entirely, not
        // because the wall actually blocked it.
        let mut physics_world =
            boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
        add_player_compatible_floor(
            &mut physics_world,
            boxddd::Vec3::new(0.0, -0.1, 0.0),
            boxddd::Vec3::new(5.0, 0.1, 5.0),
        );
        // A wall immediately in front (+X) of the agent's start position,
        // between it and the portal's far point.
        add_player_compatible_floor(
            &mut physics_world,
            boxddd::Vec3::new(1.0, 1.0, 0.0),
            boxddd::Vec3::new(0.1, 2.0, 5.0),
        );
        world.insert_non_send(BoxdddPhysicsContext::from_world(physics_world));

        let target = Vec3::new(5.0, AGENT_HEIGHT / 2.0, 0.0);
        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentKcc {
                    grounded: true,
                    ..default()
                },
                Transform::from_xyz(0.0, AGENT_HEIGHT / 2.0, 0.0),
                AgentRuntime {
                    active_link: Some(LinkKind::Merge),
                    ..default()
                },
                MergeTraversal {
                    target,
                    elapsed: 0.0,
                    timeout: merge_traversal_timeout(5.0),
                },
                UsingAnimationLink,
                AgentTarget3d::Point(target),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);

        // The agent makes genuine initial progress closing the ~0.55 m gap
        // to the wall before it wedges (see
        // `a_blocked_agent_reports_its_real_near_zero_velocity`'s own
        // 120-tick budget for the same fixture shape) and then keeps
        // creeping forward by less than a measurable step forever --
        // that's exactly why this traversal uses an absolute deadline
        // rather than a resettable no-progress counter (see
        // `MERGE_TRAVERSAL_TIMEOUT_FACTOR`'s doc comment). Run comfortably
        // past the computed timeout in fixed-tick terms.
        let dt = 1.0 / 60.0;
        let ticks_to_timeout = (merge_traversal_timeout(5.0) / dt).ceil() as usize;
        for _ in 0..(ticks_to_timeout + 60) {
            world
                .run_system_once(merge_traversal_system)
                .expect("system runs");
            if world.get::<MergeTraversal>(agent).is_none() {
                break;
            }
        }

        let kcc = world.get::<AgentKcc>(agent).unwrap();
        assert!(
            kcc.stuck,
            "a wall-blocked crossing must report stuck, not silently keep pushing forever"
        );
        assert!(kcc.collision_blocked);
        assert!(
            world.get::<MergeTraversal>(agent).is_none(),
            "the traversal must stop, not keep the agent pinned mid-portal indefinitely"
        );
        assert!(world.get::<UsingAnimationLink>(agent).is_none());
        assert_eq!(world.get::<AgentRuntime>(agent).unwrap().active_link, None);
        let position = world.get::<Transform>(agent).unwrap().translation;
        assert!(
            position.x < target.x - 1.0,
            "the agent must never teleport through the wall to the far portal point, got {position:?}"
        );
        // Review correction (issue #154 feature 4): the blocked crossing's
        // route must be cleared, not left in place -- otherwise the next
        // landmass solve re-selects the exact same invalid link and the
        // agent repeats this failure forever.
        assert!(
            world.get::<AgentTarget3d>(agent).is_none(),
            "a blocked portal crossing must clear the agent's target so it does not immediately re-select the same blocked link"
        );
        assert!(
            world
                .get::<AgentRuntime>(agent)
                .unwrap()
                .travel_intent
                .is_none()
        );
    }

    /// Regression test (issue #114 added scope, M4 wave 5 real-data
    /// acceptance finding): `spawn_test_agent`'s visual child must sit
    /// exactly centred on its parent (zero local offset), never raised.
    /// Physics-authoritative movement's parent `Transform` is already the
    /// capsule *centre* -- the wave-3/4 kinematic agent's `AGENT_HEIGHT /
    /// 2.0` visual-lift compensated for that agent's `Transform` instead
    /// sitting at feet level (navmesh-Y-snapped every tick); reintroducing
    /// that lift on a now-already-centred parent double-counts it and
    /// floats the rendered capsule a full half-height above the floor even
    /// though the physics capsule (steps/slopes) sits correctly. Tied
    /// explicitly to the centre-based parent so this can't silently
    /// regress if someone reintroduces a feet-level assumption for either
    /// side of the parent/child pair.
    #[test]
    fn the_visual_capsule_is_centred_on_the_agent_parent_not_raised_above_it() {
        let mut world = World::new();
        world.init_resource::<Assets<Mesh>>();
        world.init_resource::<Assets<StandardMaterial>>();
        world.init_resource::<NavArchipelagoState>();
        let archipelago_entity = world.spawn_empty().id();
        world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago_entity);

        // A parent position with a nonzero, non-round Y so an accidental
        // absolute (rather than relative-to-parent) offset would also be
        // caught.
        let parent_position = Vec3::new(1.0, 2.0, 3.0);
        let agent = spawn_test_agent(&mut world, parent_position);

        let children = world
            .get::<Children>(agent)
            .expect("spawn_test_agent adds exactly one visual child");
        assert_eq!(children.len(), 1, "exactly one visual child");
        let visual = children[0];

        // A zero local offset is exactly the "world Y equals the parent's
        // world Y" statement for a child with no rotation/scale on the
        // parent (`spawn_test_agent`'s agent entity carries neither) --
        // asserted directly on the local `Transform` rather than via
        // `GlobalTransform`, which this bare `World` never propagates.
        let visual_local = world.get::<Transform>(visual).unwrap();
        assert_eq!(
            visual_local.translation,
            Vec3::ZERO,
            "the visual child must be centred on the agent parent (zero local offset) -- \
             the parent transform is already the capsule centre post-#114, not feet level"
        );
    }

    #[test]
    fn no_args_prints_usage_without_erroring() {
        let mut world = harness_world();
        let result = tna_command(&mut world, &invocation(&[])).expect("usage is not an error");
        assert_eq!(result.log.len(), 1);
        assert!(result.log[0].starts_with("usage:"));
    }

    #[test]
    fn unknown_subcommand_is_an_error() {
        let mut world = harness_world();
        let error = tna_command(&mut world, &invocation(&["dance"])).unwrap_err();
        assert_eq!(error.code, "unknown_subcommand");
    }

    #[test]
    fn spawn_without_a_nav_graph_reuses_the_no_nav_graph_wording() {
        let mut world = harness_world();
        let error = tna_command(&mut world, &invocation(&["spawn"])).unwrap_err();
        assert_eq!(error.code, "no_nav_graph");
        assert_eq!(error.message, "no nav graph prepared for this cell");
    }

    #[test]
    fn goto_without_a_spawned_agent_is_an_error() {
        let mut world = harness_world();
        let error = tna_command(&mut world, &invocation(&["goto", "1", "2", "3"])).unwrap_err();
        assert_eq!(error.code, "no_agent");
    }

    #[test]
    fn goto_bad_arity_is_rejected() {
        let mut world = harness_world();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(Entity::PLACEHOLDER);
        let error = tna_command(&mut world, &invocation(&["goto", "1", "2"])).unwrap_err();
        assert_eq!(error.code, "bad_arity");
    }

    #[test]
    fn status_without_a_spawned_agent_is_an_error() {
        let mut world = harness_world();
        let error = tna_command(&mut world, &invocation(&["status"])).unwrap_err();
        assert_eq!(error.code, "no_agent");
    }

    #[test]
    fn despawn_without_a_spawned_agent_is_an_error() {
        let mut world = harness_world();
        let error = tna_command(&mut world, &invocation(&["despawn"])).unwrap_err();
        assert_eq!(error.code, "no_agent");
    }

    #[test]
    fn despawn_round_trip_clears_state() {
        let mut world = harness_world();
        let entity = world.spawn(TestNavAgentMarker).id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(entity);
        let result = tna_command(&mut world, &invocation(&["despawn"])).expect("despawn succeeds");
        assert_eq!(result.log, ["nav agent 0 despawned"]);
        assert!(world.resource::<TestNavAgentState>().entities[0].is_none());
        assert!(world.get_entity(entity).is_err());
    }

    /// Issue #134 shipped amendment: wave 3's teardown used to despawn a
    /// live test agent along with the stale archipelago, losing it. It is
    /// now ledgered instead -- here with no door noted
    /// (`PendingPlayerSwapDoor` defaults to `None`), so the agent freezes
    /// in the *departing* cell at its current position rather than being
    /// silently dropped.
    #[test]
    fn archipelago_teardown_on_cell_swap_ledgers_the_agent_instead_of_losing_it() {
        let mut world = harness_world();
        let archipelago = world.spawn_empty().id();
        let island = world.spawn_empty().id();
        world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xC0DE);
        world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago);
        world.resource_mut::<NavArchipelagoState>().islands = vec![island];
        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentRuntime::default(),
                Transform::from_xyz(1.0, 2.0, 3.0),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);

        world.insert_resource(crate::viewer::LoadedSceneManifest(minimal_manifest(0xBEEF)));
        despawn_stale_navmesh_archipelago(&mut world);

        assert!(world.get_entity(archipelago).is_err());
        assert!(world.get_entity(island).is_err());
        assert!(world.get_entity(agent).is_err(), "the live entity is gone");
        assert!(
            world
                .resource::<NavArchipelagoState>()
                .cell_form_id
                .is_none()
        );
        assert!(world.resource::<TestNavAgentState>().entities[0].is_none());

        let entry = world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(agent_ledger_id(0))
            .expect("the agent must be ledgered, not lost");
        assert_eq!(entry.cell_form_id, 0xC0DE, "frozen in the departing cell");
        assert_eq!(
            entry.spawn_kind,
            ledger_policy::SpawnKind::FrozenPosition {
                position: [1.0, 2.0, 3.0]
            }
        );
    }

    /// Issue #134: a player-initiated swap through the exact door a live
    /// agent's active route was targeting hands it off to the destination
    /// cell (follow-through) instead of freezing it in the departing cell.
    #[test]
    fn a_player_swap_through_the_agents_own_route_door_follows_through() {
        let mut world = harness_world();
        world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xC0DE);
        world.resource_mut::<NavArchipelagoState>().archipelago = Some(world.spawn_empty().id());
        world
            .resource_mut::<NavArchipelagoState>()
            .travel_doors
            .insert(
                0x99,
                TravelDoorLink {
                    triangle_midpoint: Vec3::ZERO,
                    door_position: Vec3::ZERO,
                    destination_cell_form_id: 0xBEEF,
                    destination_door_form_id: 0x1234,
                },
            );
        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentRuntime {
                    travel_intent: Some(0x99),
                    ..default()
                },
                Transform::from_xyz(5.0, 0.0, 0.0),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
        world.resource_mut::<PendingPlayerSwapDoor>().0 = Some(0x99);

        world.insert_resource(crate::viewer::LoadedSceneManifest(minimal_manifest(0xBEEF)));
        despawn_stale_navmesh_archipelago(&mut world);

        assert!(world.get_entity(agent).is_err());
        let entry = world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(agent_ledger_id(0))
            .expect("the agent must be ledgered");
        assert_eq!(
            entry.cell_form_id, 0xBEEF,
            "ledgered to the destination cell"
        );
        assert_eq!(
            entry.spawn_kind,
            ledger_policy::SpawnKind::DoorMarker {
                destination_door_form_id: 0x1234
            }
        );
    }

    /// Issue #134: a player swap through a door the agent's route was *not*
    /// targeting freezes it in the departing cell, same as an untargeted
    /// idle agent -- strict eligibility, no offscreen pathfinding.
    #[test]
    fn a_player_swap_through_a_different_door_still_freezes_the_agent() {
        let mut world = harness_world();
        world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xC0DE);
        world.resource_mut::<NavArchipelagoState>().archipelago = Some(world.spawn_empty().id());
        let agent = world
            .spawn((
                TestNavAgentMarker,
                // The agent is routed to a different travel door than the
                // one the player used.
                AgentRuntime {
                    travel_intent: Some(0x50),
                    ..default()
                },
                Transform::from_xyz(7.0, 0.0, 0.0),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
        world.resource_mut::<PendingPlayerSwapDoor>().0 = Some(0x99);

        world.insert_resource(crate::viewer::LoadedSceneManifest(minimal_manifest(0xBEEF)));
        despawn_stale_navmesh_archipelago(&mut world);

        assert!(world.get_entity(agent).is_err());
        let entry = world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(agent_ledger_id(0))
            .expect("the agent must be ledgered, not lost");
        assert_eq!(entry.cell_form_id, 0xC0DE, "frozen in the departing cell");
        assert_eq!(
            entry.spawn_kind,
            ledger_policy::SpawnKind::FrozenPosition {
                position: [7.0, 0.0, 0.0]
            }
        );
    }

    /// Issue #134: a cell claimed by a ledgered entry spawns exactly one
    /// agent on activation, at the destination door's own placed position.
    #[test]
    fn matching_cell_activation_restores_exactly_one_ledgered_agent() {
        let mut world = harness_world();
        world.init_resource::<Assets<Mesh>>();
        world.init_resource::<Assets<StandardMaterial>>();
        world
            .resource_mut::<NavAgentLedger>()
            .0
            .record(ledger_policy::LedgerEntry {
                agent_id: agent_ledger_id(0),
                cell_form_id: 0xBEEF,
                spawn_kind: ledger_policy::SpawnKind::DoorMarker {
                    destination_door_form_id: 0x1234,
                },
                remaining_target: None,
            });

        let mut manifest = minimal_manifest(0xBEEF);
        manifest
            .placements
            .push(door_placement_at(0x1234, [9.0, 1.0, 2.0]));
        // `PreparedSceneManifest.nav_graph` only needs to be `Some` here --
        // `ensure_archipelago` short-circuits on its `already_current`
        // check (below) before it would ever read this path from disk.
        manifest.nav_graph = Some(crate::vsa::PreparedNavGraphSource::default());
        world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));
        // Pre-seed the archipelago as already current for 0xBEEF so
        // `ensure_archipelago` returns immediately without any real
        // `bevy_landmass`/file-I/O plumbing -- this test is about the
        // ledger claim + spawn-count contract, not archipelago building
        // (already covered by other tests/real-data acceptance).
        let archipelago_entity = world.spawn_empty().id();
        world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xBEEF);
        world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago_entity);

        restore_ledgered_agents_system(&mut world);

        let mut query = world.query_filtered::<Entity, With<TestNavAgentMarker>>();
        let agents: Vec<Entity> = query.iter(&world).collect();
        assert_eq!(agents.len(), 1, "exactly one agent must be spawned");
        assert!(
            world
                .resource::<NavAgentLedger>()
                .0
                .entry_for(agent_ledger_id(0))
                .is_none(),
            "the claimed entry must be consumed"
        );
        let agent_entity = agents[0];
        let position = world.get::<Transform>(agent_entity).unwrap().translation;
        assert_eq!(
            position,
            Vec3::new(9.0, 1.0, 2.0),
            "spawned at the door marker"
        );
        assert_eq!(
            world.resource::<TestNavAgentState>().entities[0],
            Some(agent_entity)
        );
    }

    #[test]
    fn resolve_status_prefers_door_link_pause_over_landmass_state() {
        let paused = door_link::DoorLinkState::Paused {
            door_form_id: 0x99,
            waited_ticks: 1,
            destination: door_link::LinkDestination::IntraCell,
        };
        assert_eq!(
            resolve_status(AgentState::Moving, paused),
            landmass_graph::NavAgentStatus::Paused
        );
        assert_eq!(
            resolve_status(AgentState::Idle, door_link::DoorLinkState::Idle),
            landmass_graph::NavAgentStatus::Idle
        );
    }

    #[test]
    fn resolve_status_reports_travel_reached_as_its_own_status() {
        let reached = door_link::DoorLinkState::TravelReached {
            door_form_id: 0x99,
            destination_cell_form_id: 0xC0DE,
        };
        assert_eq!(
            resolve_status(AgentState::Idle, reached),
            landmass_graph::NavAgentStatus::TravelReached
        );
        assert_eq!(
            landmass_graph::NavAgentStatus::TravelReached.as_str(),
            "travel-reached"
        );
    }

    /// Plan #113 minimal-App test: a travel-door request routes the agent
    /// to the door triangle and, on arrival, drives the existing
    /// `DoorLinkState` lifecycle (pause -> scripted-open boundary -> wait
    /// -> traverse) to the `TravelReached` terminal seam.
    #[test]
    fn travel_request_routes_to_the_door_and_completes_the_lifecycle() {
        let mut world = harness_world();
        world.init_resource::<Time>();
        world.init_resource::<interaction::InteractionState>();
        let mut registry = crate::console::RefRegistry::default();
        // `scripted_door_open` requires the resolved entity to carry a
        // `PlacementRoot` (the same invariant the `activate` command has).
        let door_entity = world
            .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
            .id();
        registry.register(door_entity, 0x99, None);
        world.insert_resource(registry);

        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentRuntime::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
        world
            .resource_mut::<NavArchipelagoState>()
            .travel_doors
            .insert(
                0x99,
                TravelDoorLink {
                    triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                    door_position: Vec3::new(6.0, 0.0, 0.0),
                    destination_cell_form_id: 0xC0DE,
                    destination_door_form_id: 0x1234,
                },
            );

        request_travel(&mut world, 0, 0x99).expect("travel request succeeds");
        assert!(matches!(
            world.get::<AgentTarget3d>(agent),
            Some(AgentTarget3d::Point(point)) if *point == Vec3::new(5.0, 0.0, 0.0)
        ));

        // Not yet at the door: the lifecycle must not start.
        door_link_system(&mut world);
        assert_eq!(
            world.get::<AgentRuntime>(agent).unwrap().door_link,
            door_link::DoorLinkState::Idle
        );

        // Arrive at the triangle midpoint: pause + door-open request.
        world.get_mut::<Transform>(agent).unwrap().translation = Vec3::new(5.0, 0.0, 0.0);
        door_link_system(&mut world);
        assert!(is_paused(&world, agent));
        assert!(world.get::<PauseAgent>(agent).is_some());

        // The unlocked door was scripted open through the interaction
        // boundary by the arrival itself (same code path as `activate`).
        assert!(
            world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&door_entity),
            "arrival must scripted-open the unlocked door"
        );

        // The open door resumes into the kinematic crossing.
        door_link_system(&mut world);
        assert!(door_link::is_traversing(
            world.get::<AgentRuntime>(agent).unwrap().door_link
        ));
        assert!(world.get::<DoorTraversal>(agent).is_some());

        // Complete the crossing (elapsed already past the fixed duration).
        use bevy::ecs::system::RunSystemOnce;
        world.get_mut::<DoorTraversal>(agent).unwrap().elapsed = 10.0;
        world
            .run_system_once(door_traversal_system)
            .expect("traversal system runs");

        // Issue #134: the agent is handed off, not left standing at the
        // door -- despawned from the active cell and ledgered for the
        // destination cell at the paired door's marker.
        assert!(
            world.get_entity(agent).is_err(),
            "the agent must leave the active cell entirely on handoff"
        );
        assert!(world.resource::<TestNavAgentState>().entities[0].is_none());
        let entry = world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(agent_ledger_id(0))
            .expect("the agent must be ledgered on handoff");
        assert_eq!(entry.cell_form_id, 0xC0DE);
        assert_eq!(
            entry.spawn_kind,
            ledger_policy::SpawnKind::DoorMarker {
                destination_door_form_id: 0x1234
            }
        );
    }

    /// Regression test (issue #114 added scope, M4 wave 5 real-data
    /// acceptance finding): physics-authoritative movement's `Transform` is
    /// the capsule *centre*, not feet-level like `triangle_midpoint` (a
    /// nav-graph point). The wave-3/4 kinematic agent Y-snapped its
    /// `Transform` onto the navmesh every tick, incidentally erasing this
    /// gap; every other travel-arrival test in this file sets the agent's Y
    /// to match the door's exactly, which is why the regression this test
    /// targets shipped unnoticed. A ~0.9 m vertical offset (roughly
    /// `AGENT_HEIGHT / 2`, matching the real Vault101a 00028579 numbers from
    /// acceptance) must not stop the arrival gate from firing.
    #[test]
    fn travel_arrival_tolerates_the_agent_capsule_centre_sitting_above_the_feet_level_door_midpoint()
     {
        let mut world = harness_world();
        world.init_resource::<Time>();
        world.init_resource::<interaction::InteractionState>();
        let mut registry = crate::console::RefRegistry::default();
        let door_entity = world
            .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
            .id();
        registry.register(door_entity, 0x99, None);
        world.insert_resource(registry);

        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentRuntime::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
        world
            .resource_mut::<NavArchipelagoState>()
            .travel_doors
            .insert(
                0x99,
                TravelDoorLink {
                    // Feet-level midpoint, exactly like real prepared nav
                    // graph data.
                    triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                    door_position: Vec3::new(6.0, 0.0, 0.0),
                    destination_cell_form_id: 0xC0DE,
                    destination_door_form_id: 0x1234,
                },
            );

        request_travel(&mut world, 0, 0x99).expect("travel request succeeds");

        // Arrive horizontally at the triangle midpoint, but at capsule-
        // centre height (0.9 m above the feet-level midpoint) -- the exact
        // shape of the regression: a 3D distance check would read ~0.9 m,
        // just outside `TRAVEL_ARRIVAL_DISTANCE` (0.75 m), and never pause.
        world.get_mut::<Transform>(agent).unwrap().translation = Vec3::new(5.0, 0.9, 0.0);
        door_link_system(&mut world);
        assert!(
            is_paused(&world, agent),
            "the horizontal-plane arrival check must still fire despite the capsule-centre-vs-feet vertical offset"
        );
        assert!(world.get::<PauseAgent>(agent).is_some());
        assert!(
            world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&door_entity),
            "arrival must scripted-open the unlocked door"
        );
    }

    fn is_paused(world: &World, agent: Entity) -> bool {
        door_link::is_paused(
            world
                .get::<AgentRuntime>(agent)
                .map(|runtime| runtime.door_link)
                .unwrap_or_default(),
        )
    }

    /// Minimal travel-door placement for the lifecycle tests.
    fn door_placement(reference_form_id: u32) -> crate::vsa::PreparedPlacement {
        crate::vsa::PreparedPlacement {
            reference_form_id,
            base_form_id: 1,
            asset_path: None,
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            error: None,
            physics_asset_path: None,
            physics_source: None,
            physics_classification: Default::default(),
            step_support: false,
            mutability: Default::default(),
            mutability_root_form_id: None,
            reference_kind: "REFR".into(),
            base_kind: "DOOR".into(),
            editor_id: None,
            display_name: None,
            count: 1,
            semantic: crate::vsa::PreparedSemantic::Door(crate::vsa::PreparedDoor {
                lock_level: None,
                key_form_id: None,
                destination: None,
            }),
            initially_enabled: true,
            enable_parent: None,
            owner_form_id: None,
            owner_faction_rank: None,
            inventory: Vec::new(),
            audio: Default::default(),
            ao_mode: "ao-none".into(),
        }
    }

    /// A door placement at a specific position (issue #134's restore
    /// tests, which spawn at a resolved door-marker position).
    fn door_placement_at(
        reference_form_id: u32,
        translation: [f32; 3],
    ) -> crate::vsa::PreparedPlacement {
        crate::vsa::PreparedPlacement {
            translation,
            ..door_placement(reference_form_id)
        }
    }

    /// Plan #113 minimal-App test: a locked travel door never scripted-opens
    /// (no teleporting through closed doors) and resolves to the existing
    /// deterministic `Failed` status via the wait bound.
    #[test]
    fn locked_travel_door_fails_deterministically_without_opening() {
        let mut world = harness_world();
        world.init_resource::<interaction::InteractionState>();
        let mut registry = crate::console::RefRegistry::default();
        let door_entity = world.spawn_empty().id();
        registry.register(door_entity, 0x99, None);
        world.insert_resource(registry);

        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentRuntime::default(),
                Transform::from_xyz(5.0, 0.0, 0.0),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
        {
            let mut state = world.resource_mut::<NavArchipelagoState>();
            state.travel_doors.insert(
                0x99,
                TravelDoorLink {
                    triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                    door_position: Vec3::new(6.0, 0.0, 0.0),
                    destination_cell_form_id: 0xC0DE,
                    destination_door_form_id: 0x1234,
                },
            );
            state.door_lock_info.insert(
                0x99,
                DoorLockInfo {
                    lock_level: Some(50),
                    key_form_id: None,
                },
            );
            state.door_usable.insert(0x99, false);
        }

        request_travel(&mut world, 0, 0x99).expect("routing to a locked door is allowed");
        door_link_system(&mut world);
        assert!(is_paused(&world, agent));
        assert!(
            !world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&door_entity),
            "a locked door must never be scripted open by the nav agent"
        );

        for _ in 0..door_link::MAX_WAIT_TICKS {
            door_link_system(&mut world);
        }
        assert_eq!(
            world.get::<AgentRuntime>(agent).unwrap().door_link,
            door_link::DoorLinkState::Failed { door_form_id: 0x99 }
        );
    }

    /// Plan #113 minimal-App test: a door state change triggers exactly one
    /// repath -- the blocked two-sided link spawns once when the door
    /// becomes usable, and repeated polls with no further change do
    /// nothing.
    #[test]
    fn a_door_state_change_triggers_exactly_one_repath() {
        let mut world = harness_world();
        world.init_resource::<interaction::InteractionState>();
        let mut registry = crate::console::RefRegistry::default();
        let door_entity = world.spawn_empty().id();
        registry.register(door_entity, 0x99, None);
        world.insert_resource(registry);

        let archipelago = world.spawn_empty().id();
        {
            let mut state = world.resource_mut::<NavArchipelagoState>();
            state.archipelago = Some(archipelago);
            state.door_lock_info.insert(
                0x99,
                DoorLockInfo {
                    lock_level: Some(50),
                    key_form_id: None,
                },
            );
            state.door_usable.insert(0x99, false);
            state.blocked_door_links.push(BlockedDoorLink {
                door_form_id: 0x99,
                start: Vec3::ZERO,
                end: Vec3::new(1.0, 0.0, 0.0),
            });
        }

        // No change: locked stays locked, nothing spawns.
        door_availability_system(&mut world);
        assert!(world.resource::<NavArchipelagoState>().links.is_empty());
        assert_eq!(
            world
                .resource::<NavArchipelagoState>()
                .blocked_door_links
                .len(),
            1
        );

        // The door opens (e.g. the player activates it): one flip, one
        // repath -- the link spawns (one unidirectional pair, see
        // `spawn_link_pair`) and the blocked entry is consumed.
        world
            .resource_mut::<interaction::InteractionState>()
            .open
            .insert(door_entity);
        door_availability_system(&mut world);
        assert_eq!(world.resource::<NavArchipelagoState>().links.len(), 2);
        assert!(
            world
                .resource::<NavArchipelagoState>()
                .blocked_door_links
                .is_empty()
        );

        // Steady state: repeated polls never spawn another link pair.
        door_availability_system(&mut world);
        door_availability_system(&mut world);
        assert_eq!(world.resource::<NavArchipelagoState>().links.len(), 2);
    }

    /// Plan #137 minimal-App test (real-data-corrected): a `goto` past a
    /// closed unlocked door mid-route drives the existing `DoorLinkState`
    /// lifecycle exactly once via the crossing-check trigger, then returns
    /// to `Idle` in the same cell. The door is *also* registered as a
    /// travel door (`travel_doors`) -- real FO3 data shows nearly every
    /// single-sided door resolves to a travel destination, and this is the
    /// exact case the orchestrator's real-data review found ungated: an
    /// agent with no `travel_intent` for this door must not be handed off
    /// (no ledger entry, no despawn, no `DoorTraversal` -- there is no
    /// off-mesh gap to lerp across since it merely crosses the triangle on
    /// the way to a farther point).
    #[test]
    fn a_goto_crossing_a_closed_unlocked_travel_door_mid_route_drives_the_lifecycle_once_with_no_handoff()
     {
        let mut world = harness_world();
        world.init_resource::<Time>();
        world.init_resource::<interaction::InteractionState>();
        let mut registry = crate::console::RefRegistry::default();
        let door_entity = world
            .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
            .id();
        registry.register(door_entity, 0x99, None);
        world.insert_resource(registry);

        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentRuntime::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
                // A plain `goto` well beyond the door -- no `travel_intent`
                // for this (or any) door.
                AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
        {
            let mut state = world.resource_mut::<NavArchipelagoState>();
            state.mid_route_doors.push(MidRouteDoor {
                door_form_id: 0x99,
                midpoint: Vec3::new(5.0, 0.0, 0.0),
            });
            // The same door is also a travel-door candidate -- this is the
            // real-data shape (see this file's module doc): the crossing
            // gate must still apply, and must still not hand off.
            state.travel_doors.insert(
                0x99,
                TravelDoorLink {
                    triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                    door_position: Vec3::new(5.0, 0.0, 0.0),
                    destination_cell_form_id: 0xC0DE,
                    destination_door_form_id: 0x1234,
                },
            );
        }

        // Not yet at the door: the lifecycle must not start.
        door_link_system(&mut world);
        assert_eq!(
            world.get::<AgentRuntime>(agent).unwrap().door_link,
            door_link::DoorLinkState::Idle
        );

        // Arrive at the triangle midpoint: pause + scripted-open request.
        world.get_mut::<Transform>(agent).unwrap().translation = Vec3::new(5.0, 0.0, 0.0);
        door_link_system(&mut world);
        assert!(is_paused(&world, agent));
        assert!(world.get::<PauseAgent>(agent).is_some());
        assert!(
            world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&door_entity),
            "arrival must scripted-open the unlocked door"
        );

        // The open door resumes -- and, unlike the off-mesh link cases,
        // the crossing completes in the same tick.
        door_link_system(&mut world);
        assert_eq!(
            world.get::<AgentRuntime>(agent).unwrap().door_link,
            door_link::DoorLinkState::Idle,
            "an intra-cell mid-route crossing returns to Idle, not a handoff"
        );
        assert!(world.get::<DoorTraversal>(agent).is_none());
        assert!(world.get::<PauseAgent>(agent).is_none());
        assert!(
            world
                .get::<AgentRuntime>(agent)
                .unwrap()
                .active_link
                .is_none()
        );
        assert!(
            world.get_entity(agent).is_ok(),
            "the agent stays in the active cell"
        );
        assert!(
            world
                .resource::<NavAgentLedger>()
                .0
                .entry_for(agent_ledger_id(0))
                .is_none(),
            "crossing a travel door's triangle mid-route (not the agent's own travel_intent) must not ledger a handoff"
        );
    }

    /// Regression test (issue #114 added scope, M4 wave 5 real-data
    /// acceptance finding): same shape as
    /// `travel_arrival_tolerates_the_agent_capsule_centre_sitting_above_the_feet_level_door_midpoint`,
    /// for the #137 mid-route crossing gate -- a capsule-centre agent above
    /// a feet-level `MidRouteDoor::midpoint` must still trigger the
    /// crossing gate instead of silently clipping through the closed door.
    #[test]
    fn mid_route_crossing_gate_tolerates_the_agent_capsule_centre_vertical_offset() {
        let mut world = harness_world();
        world.init_resource::<Time>();
        world.init_resource::<interaction::InteractionState>();
        let mut registry = crate::console::RefRegistry::default();
        let door_entity = world
            .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
            .id();
        registry.register(door_entity, 0x99, None);
        world.insert_resource(registry);

        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentRuntime::default(),
                Transform::from_xyz(0.0, 0.9, 0.0),
                AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
        world
            .resource_mut::<NavArchipelagoState>()
            .mid_route_doors
            .push(MidRouteDoor {
                door_form_id: 0x99,
                // Feet-level midpoint, exactly like real prepared nav graph
                // data -- the agent's own Y stays at capsule-centre height
                // (0.9 m) the whole time, never snapped down to match it.
                midpoint: Vec3::new(5.0, 0.0, 0.0),
            });

        world.get_mut::<Transform>(agent).unwrap().translation = Vec3::new(5.0, 0.9, 0.0);
        door_link_system(&mut world);
        assert!(
            is_paused(&world, agent),
            "the horizontal-plane crossing gate must still fire despite the capsule-centre-vs-feet vertical offset"
        );
        assert!(
            world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&door_entity),
            "arrival must scripted-open the unlocked door"
        );
    }

    /// Plan #137 minimal-App test (real-data-corrected): a `tna travel`
    /// request to a door still produces the full travel lifecycle and
    /// handoff, even though the very same door is also a crossing-gate
    /// candidate (`mid_route_doors`) -- the agent's own `travel_intent`
    /// must exclude that door from the crossing check, or the two paths
    /// would fight over the same arrival.
    #[test]
    fn a_travel_request_to_a_door_still_hands_off_even_though_it_is_also_a_crossing_gate_candidate()
    {
        let mut world = harness_world();
        world.init_resource::<Time>();
        world.init_resource::<interaction::InteractionState>();
        let mut registry = crate::console::RefRegistry::default();
        let door_entity = world
            .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
            .id();
        registry.register(door_entity, 0x99, None);
        world.insert_resource(registry);

        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentRuntime::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
        {
            let mut state = world.resource_mut::<NavArchipelagoState>();
            state.travel_doors.insert(
                0x99,
                TravelDoorLink {
                    triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                    door_position: Vec3::new(6.0, 0.0, 0.0),
                    destination_cell_form_id: 0xC0DE,
                    destination_door_form_id: 0x1234,
                },
            );
            // The crossing-gate candidate set also carries this door (the
            // real-data shape) -- it must not hijack the travel-arrival
            // check once `travel_intent` targets it.
            state.mid_route_doors.push(MidRouteDoor {
                door_form_id: 0x99,
                midpoint: Vec3::new(5.0, 0.0, 0.0),
            });
        }

        request_travel(&mut world, 0, 0x99).expect("travel request succeeds");
        assert!(matches!(
            world.get::<AgentTarget3d>(agent),
            Some(AgentTarget3d::Point(point)) if *point == Vec3::new(5.0, 0.0, 0.0)
        ));

        world.get_mut::<Transform>(agent).unwrap().translation = Vec3::new(5.0, 0.0, 0.0);
        door_link_system(&mut world);
        assert!(is_paused(&world, agent));
        assert!(
            world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&door_entity),
            "arrival must scripted-open the unlocked door"
        );

        door_link_system(&mut world);
        assert!(
            door_link::is_traversing(world.get::<AgentRuntime>(agent).unwrap().door_link),
            "the travel-arrival check, not the crossing gate, must own this door once travel_intent targets it"
        );
        assert!(
            world.get::<DoorTraversal>(agent).is_some(),
            "a real travel handoff crosses through a DoorTraversal lerp, unlike the gap-less crossing-gate case"
        );

        use bevy::ecs::system::RunSystemOnce;
        world.get_mut::<DoorTraversal>(agent).unwrap().elapsed = 10.0;
        world
            .run_system_once(door_traversal_system)
            .expect("traversal system runs");

        assert!(
            world.get_entity(agent).is_err(),
            "a real travel_intent arrival must still hand the agent off to the destination cell"
        );
        let entry = world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(agent_ledger_id(0))
            .expect("the agent must be ledgered on handoff");
        assert_eq!(entry.cell_form_id, 0xC0DE);
    }

    /// Plan #137 minimal-App test (real-data-corrected): a locked door
    /// crossed mid-route -- again also registered as a travel door, the
    /// real-data shape -- by an agent with no `travel_intent` for it never
    /// scripted-opens and resolves to the existing deterministic `Failed`
    /// outcome via the wait bound, instead of letting the agent clip
    /// through.
    #[test]
    fn a_goto_crossing_a_locked_travel_door_mid_route_fails_deterministically_without_opening() {
        let mut world = harness_world();
        world.init_resource::<interaction::InteractionState>();
        let mut registry = crate::console::RefRegistry::default();
        let door_entity = world.spawn_empty().id();
        registry.register(door_entity, 0x99, None);
        world.insert_resource(registry);

        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentRuntime::default(),
                Transform::from_xyz(5.0, 0.0, 0.0),
                AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
        {
            let mut state = world.resource_mut::<NavArchipelagoState>();
            state.mid_route_doors.push(MidRouteDoor {
                door_form_id: 0x99,
                midpoint: Vec3::new(5.0, 0.0, 0.0),
            });
            state.travel_doors.insert(
                0x99,
                TravelDoorLink {
                    triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                    door_position: Vec3::new(5.0, 0.0, 0.0),
                    destination_cell_form_id: 0xC0DE,
                    destination_door_form_id: 0x1234,
                },
            );
            state.door_lock_info.insert(
                0x99,
                DoorLockInfo {
                    lock_level: Some(50),
                    key_form_id: None,
                },
            );
            state.door_usable.insert(0x99, false);
        }

        door_link_system(&mut world);
        assert!(is_paused(&world, agent));
        assert!(
            !world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&door_entity),
            "a locked door must never be scripted open by the nav agent"
        );

        for _ in 0..door_link::MAX_WAIT_TICKS {
            door_link_system(&mut world);
        }
        assert_eq!(
            world.get::<AgentRuntime>(agent).unwrap().door_link,
            door_link::DoorLinkState::Failed { door_form_id: 0x99 }
        );
        assert!(
            world.get::<PauseAgent>(agent).is_some(),
            "the agent stays stopped at the link instead of clipping through"
        );
    }

    /// Plan #137 minimal-App test: a mid-route door's usability flip reuses
    /// `door_availability_system` unchanged -- the same generic per-door
    /// tracking two-sided/travel doors already populate -- so clearing a
    /// lock while an agent waits on it triggers exactly one repath (a
    /// `request_door_open` retry) that frees the paused agent.
    #[test]
    fn unlocking_a_mid_route_door_triggers_one_repath_that_frees_a_paused_agent() {
        let mut world = harness_world();
        world.init_resource::<interaction::InteractionState>();
        let mut registry = crate::console::RefRegistry::default();
        let door_entity = world
            .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
            .id();
        registry.register(door_entity, 0x99, None);
        world.insert_resource(registry);

        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentRuntime::default(),
                Transform::from_xyz(5.0, 0.0, 0.0),
                AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
        {
            let mut state = world.resource_mut::<NavArchipelagoState>();
            state.mid_route_doors.push(MidRouteDoor {
                door_form_id: 0x99,
                midpoint: Vec3::new(5.0, 0.0, 0.0),
            });
            state.door_lock_info.insert(
                0x99,
                DoorLockInfo {
                    lock_level: Some(50),
                    key_form_id: None,
                },
            );
            state.door_usable.insert(0x99, false);
        }

        // The agent walks up to the locked door and waits.
        door_link_system(&mut world);
        assert!(is_paused(&world, agent));
        assert!(
            !world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&door_entity)
        );

        // No change: nothing happens.
        door_availability_system(&mut world);
        assert!(is_paused(&world, agent));
        assert!(
            !world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&door_entity)
        );

        // The lock is cleared (e.g. the player picks/keys it elsewhere):
        // one usability flip, one repath -- `door_availability_system`
        // requests the door open again for the agent already paused on it.
        world
            .resource_mut::<NavArchipelagoState>()
            .door_lock_info
            .insert(
                0x99,
                DoorLockInfo {
                    lock_level: None,
                    key_form_id: None,
                },
            );
        door_availability_system(&mut world);
        assert!(
            world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&door_entity),
            "the repath must retry the scripted-open request for the door the agent is paused on"
        );

        // Steady state: repeated polls do not re-trigger the repath.
        door_availability_system(&mut world);
        door_availability_system(&mut world);

        // The next tick resumes and completes the (gap-less) crossing.
        door_link_system(&mut world);
        assert_eq!(
            world.get::<AgentRuntime>(agent).unwrap().door_link,
            door_link::DoorLinkState::Idle
        );
    }

    /// Plan #113 minimal-App test: never two concurrent travel requests.
    #[test]
    fn concurrent_travel_requests_are_rejected() {
        let mut world = harness_world();
        let agent = world
            .spawn((TestNavAgentMarker, AgentRuntime::default()))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
        world
            .resource_mut::<NavArchipelagoState>()
            .travel_doors
            .insert(
                0x99,
                TravelDoorLink {
                    triangle_midpoint: Vec3::ZERO,
                    door_position: Vec3::ZERO,
                    destination_cell_form_id: 0xC0DE,
                    destination_door_form_id: 0x1234,
                },
            );
        request_travel(&mut world, 0, 0x99).expect("first request succeeds");
        let error = request_travel(&mut world, 0, 0x99).unwrap_err();
        assert_eq!(error.code, "travel_in_progress");
    }

    #[test]
    fn travel_request_errors_without_an_agent_or_a_known_door() {
        let mut world = harness_world();
        assert_eq!(
            request_travel(&mut world, 0, 0x99).unwrap_err().code,
            "no_agent"
        );
        let agent = world
            .spawn((TestNavAgentMarker, AgentRuntime::default()))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
        assert_eq!(
            request_travel(&mut world, 0, 0x99).unwrap_err().code,
            "unknown_travel_door"
        );
    }

    #[test]
    fn active_link_description_reports_merge_door_and_travel_reached() {
        let mut runtime = AgentRuntime::default();
        assert_eq!(active_link_description(&runtime), None);

        runtime.active_link = Some(LinkKind::Merge);
        assert_eq!(active_link_description(&runtime), Some("merge".to_string()));

        runtime.active_link = Some(LinkKind::Door { form_id: 0x99 });
        assert_eq!(
            active_link_description(&runtime),
            Some("door 00000099".to_string())
        );

        runtime.active_link = None;
        runtime.door_link = door_link::DoorLinkState::TravelReached {
            door_form_id: 0x99,
            destination_cell_form_id: 0xC0DE,
        };
        assert_eq!(
            active_link_description(&runtime),
            Some("door 00000099 cell 0000c0de".to_string())
        );
    }

    /// Issue #114 feature 4: `tna spawn`'s index is a bounded, independent
    /// slot -- occupying index 0 does not block index 1, and an index at or
    /// past `MAX_TEST_AGENTS` is rejected before anything else runs.
    #[test]
    fn spawn_indices_are_independent_slots_bounded_by_the_cap() {
        let mut world = harness_world();
        // Pre-seed the archipelago as already current so `ensure_archipelago`
        // (which `spawn_agent` always calls first, same as wave 3/4) returns
        // immediately without needing a real manifest/nav-graph file --
        // this test is about the index/occupancy contract, not archipelago
        // building.
        let mut manifest = minimal_manifest(0xBEEF);
        manifest.nav_graph = Some(crate::vsa::PreparedNavGraphSource::default());
        world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));
        world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xBEEF);
        world.resource_mut::<NavArchipelagoState>().archipelago = Some(world.spawn_empty().id());
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(Entity::PLACEHOLDER);

        let error = tna_command(&mut world, &invocation(&["spawn", "0"])).unwrap_err();
        assert_eq!(error.code, "already_spawned");

        // A different index is an independent slot -- it gets past the
        // occupancy check to the next requirement (a live FPS player),
        // proving index 0's occupancy did not block it.
        let error = tna_command(&mut world, &invocation(&["spawn", "1"])).unwrap_err();
        assert_eq!(error.code, "player_unavailable");

        let out_of_range = MAX_TEST_AGENTS.to_string();
        let error = tna_command(&mut world, &invocation(&["spawn", &out_of_range])).unwrap_err();
        assert_eq!(error.code, "bad_agent_index");
    }

    /// Issue #114 feature 4: an indexed `tna goto` addresses exactly the
    /// named agent slot, leaving every other slot's target untouched --
    /// the back-compat bare form (no index) still defaults to agent 0.
    #[test]
    fn indexed_goto_addresses_only_the_named_agent_slot() {
        let mut world = harness_world();
        world.init_resource::<Time>();
        let agent0 = world
            .spawn((
                TestNavAgentMarker,
                AgentRuntime::default(),
                AgentKcc::default(),
            ))
            .id();
        let agent1 = world
            .spawn((
                TestNavAgentMarker,
                AgentRuntime::default(),
                AgentKcc::default(),
            ))
            .id();
        world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent0);
        world.resource_mut::<TestNavAgentState>().entities[1] = Some(agent1);

        tna_command(&mut world, &invocation(&["goto", "1", "5", "6", "7"]))
            .expect("indexed goto succeeds");

        assert!(
            matches!(
                world.get::<AgentTarget3d>(agent1),
                Some(AgentTarget3d::Point(point)) if *point == Vec3::new(5.0, 6.0, 7.0)
            ),
            "agent 1 got the target"
        );
        assert!(
            world.get::<AgentTarget3d>(agent0).is_none(),
            "agent 0 is untouched by an indexed goto for a different agent"
        );
    }

    fn minimal_manifest(cell_form_id: u32) -> PreparedSceneManifest {
        PreparedSceneManifest {
            schema_version: 17,
            prepare_revision: None,
            converter_revision: None,
            physics_schema_version: None,
            asset_root: ".".into(),
            source_plugin: "Fallout3.esm".into(),
            source_fingerprint: "content-hash".into(),
            item_catalog_path: None,
            item_catalog_revision: None,
            item_catalog_hash: None,
            recipe_catalog_path: None,
            recipe_catalog_revision: None,
            recipe_catalog_hash: None,
            actor_catalog_path: None,
            actor_catalog_revision: None,
            actor_catalog_hash: None,
            source_plugins: Vec::new(),
            visual_issues: Vec::new(),
            cell: crate::vsa::CellInfo {
                form_id: cell_form_id,
                editor_id: None,
                name: None,
                interior: true,
                ambient_rgba: [0.0; 4],
                directional_rgba: [0.0; 4],
                image_space_form_id: None,
                image_space: None,
                lighting_template_form_id: None,
                lighting_template_flags: 0,
                lighting_template: None,
                raw_lighting: None,
                effective_lighting: None,
                water_form_id: None,
                water_height: None,
                grid: None,
                worldspace_form_id: None,
            },
            placements: Vec::new(),
            lights: Vec::new(),
            diagnostics: Vec::new(),
            navmeshes: Vec::new(),
            nav_graph: None,
            cell_audio: Default::default(),
            audio_clips: Vec::new(),
            footstep_sets: Vec::new(),
            hard_landing_clips: Vec::new(),
            bake: None,
            static_point_shadows: None,
            mutability_summary: Default::default(),
            leveled_lists: Default::default(),
        }
    }

    // -----------------------------------------------------------------
    // Wave 5 added scope (#114 movement fidelity): fixed-timestep movement,
    // player-as-landmass-character avoidance, configurable solve interval.
    // -----------------------------------------------------------------

    /// A `boxddd` collision filter compatible with the *real* hardcoded
    /// `player::player_collision_filter()`/`stair_support_filter()` queries
    /// `apply_agent_physics_movement` uses (those category constants are
    /// private to `player/mod.rs`, so this mirrors their known bit values --
    /// `WORLD_STATIC = 1`, `STEP_SUPPORT = 16` -- directly): a floor shape
    /// built with it is both an ordinary collision surface and a
    /// step-support surface. `mask_bits` is maximally permissive since a
    /// static, passive shape like a floor is only ever the *target* of a
    /// query, never the querying side.
    fn fixture_floor_filter() -> Filter {
        Filter {
            category_bits: 1 | 16,
            mask_bits: u64::MAX,
            group_index: 0,
        }
    }

    /// A flat floor box (top face at `center.y + half_extents.y`) using
    /// [`fixture_floor_filter`] rather than [`fixture_shape_def`]'s
    /// self-consistent-but-arbitrary filter, so the real
    /// `apply_agent_physics_movement` system (not just the pure
    /// `step_agent_kcc`/`move_mover` helpers, which take their filter as a
    /// parameter) actually collides with and stands on it.
    fn add_player_compatible_floor(
        world: &mut boxddd::World,
        center: boxddd::Vec3,
        half_extents: boxddd::Vec3,
    ) {
        let shape_def = ShapeDef::builder().filter(fixture_floor_filter()).build();
        let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
        world.create_hull_shape(
            body,
            &shape_def,
            &BoxHull::transformed(
                half_extents.x,
                half_extents.y,
                half_extents.z,
                boxddd::Transform::new(center, boxddd::Quat::IDENTITY),
            ),
        );
    }

    /// Builds a minimal `App` with the full `NavBackendPlugin` wiring:
    /// `Landmass3dPlugin` (in `FixedPreUpdate`) plus this file's own
    /// `FixedUpdate` agent chain and the solve-rate gate on
    /// `LandmassSystems::Update`, exactly as `install` wires it in the real
    /// viewer -- plus `TransformPlugin` so `GlobalTransform` reflects
    /// `Transform` without needing a full render/window stack. Physics
    /// readiness resources (`PhysicsDisabled`, `CellPhysicsReadiness`) and a
    /// `BoxdddPhysicsContext` holding a flat floor spanning
    /// [`spawn_fixture_island`]'s 4x4 footprint (top face at `y = 0.0`,
    /// matching the island mesh plane exactly) are inserted directly rather
    /// than through `player::install`, which pulls in the full window/input/
    /// asset surface these tests do not need. A real floor -- not just an
    /// empty physics world -- matters here: without one the capsule free-
    /// falls under gravity every tick and drifts outside the navmesh's
    /// vertical sampling envelope within a couple dozen ticks, flipping the
    /// agent to `AgentState::AgentNotOnNavMesh` and losing its desired
    /// velocity entirely (confirmed the hard way while writing the
    /// avoidance-deflection test below).
    fn fixed_tick_test_app() -> App {
        let mut app = App::new();
        app.add_plugins((
            bevy::MinimalPlugins,
            bevy::asset::AssetPlugin::default(),
            bevy::transform::TransformPlugin,
            NavBackendPlugin,
        ));
        app.insert_resource(PhysicsDisabled(false));
        app.insert_resource(CellPhysicsReadiness::Ready);
        let mut physics_world =
            boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
        add_player_compatible_floor(
            &mut physics_world,
            boxddd::Vec3::new(2.0, -0.1, 2.0),
            boxddd::Vec3::new(4.0, 0.1, 4.0),
        );
        app.world_mut()
            .insert_non_send(BoxdddPhysicsContext::from_world(physics_world));
        app
    }

    /// Advances exactly one fixed tick by hand: advances `Time<Fixed>` by
    /// its configured timestep, publishes that as the generic `Res<Time>`
    /// clock the way the real fixed-main loop does
    /// (`bevy_time::fixed::run_fixed_main_schedule`'s own per-expend body),
    /// then runs `FixedPreUpdate` (landmass, the player-character sync, and
    /// the solve-rate bookkeeping) followed by `FixedUpdate` (this file's
    /// agent chain) directly by schedule label -- the same technique
    /// `nav_overlay.rs`'s own landmass harness test uses for
    /// `FixedPreUpdate` alone, extended across both schedules so a whole
    /// tick is deterministic with no dependency on real wall-clock elapsed
    /// time.
    fn run_one_fixed_tick(world: &mut World) {
        let timestep = world.resource::<Time<Fixed>>().timestep();
        world.resource_mut::<Time<Fixed>>().advance_by(timestep);
        let generic = world.resource::<Time<Fixed>>().as_generic();
        *world.resource_mut::<Time>() = generic;
        world.run_schedule(FixedPreUpdate);
        world.run_schedule(FixedUpdate);
    }

    /// Spawns the same synthetic two-triangle 4x4 island fixture
    /// `nav_overlay.rs`'s own landmass harness test uses, wired directly
    /// into `NavArchipelagoState` (bypassing the manifest/
    /// `ensure_archipelago` plumbing these unit tests do not need). Returns
    /// the archipelago entity.
    fn spawn_fixture_island(world: &mut World) -> Entity {
        let mesh_input = landmass_graph::MeshInput {
            form_id: 0x10,
            vertices: vec![
                [0.0, 0.0, 0.0],
                [4.0, 0.0, 0.0],
                [0.0, 0.0, 4.0],
                [4.0, 0.0, 4.0],
            ],
            polygons: vec![
                landmass_graph::PolygonInput {
                    index: 0,
                    vertex_indices: [0, 1, 2],
                    is_water: false,
                },
                landmass_graph::PolygonInput {
                    index: 1,
                    vertex_indices: [1, 3, 2],
                    is_water: false,
                },
            ],
            doors: Vec::new(),
        };
        let valid = landmass_graph::build_navigation_mesh(&mesh_input, &[])
            .nav_mesh
            .expect("synthetic square validates");
        let nav_mesh_handle = world.resource_mut::<Assets<NavMesh3d>>().add(NavMesh3d {
            nav_mesh: Arc::new(valid),
        });
        // Same widened envelope `ensure_archipelago` applies for real cells
        // (see `AGENT_POINT_SAMPLE_DISTANCE`'s doc comment): the physics
        // capsule's `Transform` is centre-height above the mesh plane, well
        // outside `from_agent_radius`'s own tight default.
        let mut options = ArchipelagoOptions::from_agent_radius(AGENT_RADIUS);
        options.point_sample_distance = AGENT_POINT_SAMPLE_DISTANCE;
        let archipelago_entity = world.spawn(Archipelago3d::new(options)).id();
        world.spawn(Island3dBundle {
            island: Island,
            archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            nav_mesh: NavMeshHandle::<ThreeD>(nav_mesh_handle),
        });
        world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago_entity);
        archipelago_entity
    }

    /// Spawns a bare nav agent (no console-tracked `TestNavAgentState` slot,
    /// no visual mesh) directly into `archipelago_entity`, targeting `target`
    /// from `start`. Mirrors the component set `spawn_test_agent` builds,
    /// minus the roster bookkeeping and visuals these App-level movement
    /// tests do not need.
    fn spawn_bare_agent(
        world: &mut World,
        archipelago_entity: Entity,
        start: Vec3,
        target: Vec3,
    ) -> Entity {
        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentKcc::default(),
                AgentDesiredVelocityBlend::default(),
                Transform::from_translation(start),
                Agent3dBundle {
                    agent: default(),
                    settings: AgentSettings {
                        radius: AGENT_RADIUS,
                        desired_speed: AGENT_DESIRED_SPEED,
                        max_speed: AGENT_MAX_SPEED,
                    },
                    archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
                },
                TargetReachedCondition::Distance(Some(AGENT_TARGET_REACHED_DISTANCE)),
            ))
            .id();
        world.entity_mut(agent).insert(AgentTarget3d::Point(target));
        agent
    }

    /// Task 1 (fixed-timestep movement) + the solve-rate gate: the agent
    /// keeps advancing horizontally toward its target on every fixed tick,
    /// including a tick the solve is gated off on (`NavSolveRate(2)`).
    /// Warms up over a few ticks first so both halves of the blend
    /// (`AgentDesiredVelocityBlend`) hold real, nonzero solved values rather
    /// than the zero-initialized default.
    #[test]
    fn movement_runs_every_fixed_tick_including_when_the_solve_is_gated_off() {
        let mut app = fixed_tick_test_app();
        let archipelago_entity = spawn_fixture_island(app.world_mut());
        // `Transform.translation` is the capsule *centre* (mirrors
        // production: `spawn_test_agent` places new agents at the player's
        // own capsule-centre position), so standing on a floor whose top
        // face is at `y = 0.0` means starting at `y = AGENT_HEIGHT / 2`, not
        // `y = 0.0` -- the target's own Y does not matter to physics, only
        // to which navmesh point it samples onto.
        let agent = spawn_bare_agent(
            app.world_mut(),
            archipelago_entity,
            Vec3::new(0.5, AGENT_HEIGHT / 2.0, 0.5),
            Vec3::new(3.5, 0.0, 3.5),
        );
        app.world_mut().insert_resource(NavSolveRate(2));

        for _ in 0..4 {
            run_one_fixed_tick(app.world_mut());
        }
        let step = app.world().resource::<NavSolveStepCounter>().0;
        assert_eq!(step, 4, "four ticks were driven by hand");
        assert!(
            movement_policy::should_solve(step, 2),
            "tick 4 is a solve tick at interval 2 -- the warm-up assumption this test relies on"
        );
        let position_after_solve_tick = app.world().get::<Transform>(agent).unwrap().translation;

        // Tick 5: a skip tick (5 % 2 = 1). Movement must still run.
        run_one_fixed_tick(app.world_mut());
        assert!(
            !movement_policy::should_solve(5, 2),
            "tick 5 is a skip tick at interval 2 -- the assertion below relies on this"
        );
        let position_after_skip_tick = app.world().get::<Transform>(agent).unwrap().translation;

        assert_ne!(
            Vec2::new(position_after_solve_tick.x, position_after_solve_tick.z),
            Vec2::new(position_after_skip_tick.x, position_after_skip_tick.z),
            "the agent must keep moving horizontally on a fixed tick the solve is gated off"
        );
    }

    /// Task 2: a landmass character mirrors the FPS player's position and
    /// actual KCC velocity every fixed tick, and is present in the same
    /// archipelago the agent/island use (`ArchipelagoRef3d` points at it).
    /// The player entity is spawned through the real production path
    /// (`player::set_camera_mode`) rather than constructed by hand: both
    /// `FpsPlayer` and the rest of `KccState`'s fields are private outside
    /// `player`, and this wave's file-ownership boundary allows exactly one
    /// accessor edit to `player/mod.rs` (`KccState::velocity`, made
    /// `pub(crate)`), not a test-only constructor.
    #[test]
    fn a_landmass_character_mirrors_the_player_and_exists_in_the_archipelago() {
        let mut app = fixed_tick_test_app();
        let archipelago_entity = spawn_fixture_island(app.world_mut());
        let character_entity = spawn_player_nav_character(app.world_mut(), archipelago_entity);
        app.world_mut()
            .resource_mut::<NavArchipelagoState>()
            .player_character = Some(character_entity);

        app.world_mut().init_resource::<player::CameraModeState>();
        app.world_mut().init_resource::<player::PlayerNoClip>();
        app.world_mut()
            .insert_resource(player::PhysicsDisabled(false));
        app.world_mut()
            .init_resource::<crate::console::RefRegistry>();
        let camera_local_height = player::EYE_HEIGHT - player::CAPSULE_HEIGHT * 0.5;
        let player_center = Vec3::new(1.0, 0.0, 1.0);
        let camera_transform =
            Transform::from_translation(player_center + Vec3::Y * camera_local_height);
        app.world_mut().spawn((
            Camera3d::default(),
            camera_transform,
            GlobalTransform::from(camera_transform),
            crate::viewer::FlyCamera {
                yaw: 0.0,
                pitch: 0.0,
                speed: 0.0,
            },
        ));
        player::set_camera_mode(app.world_mut(), player::CameraMode::Fps)
            .expect("an FPS player spawns from a fresh Free-mode camera");
        let player_entity = app
            .world()
            .resource::<player::CameraModeState>()
            .player
            .expect("set_camera_mode recorded the new player entity");

        let player_velocity = Vec3::new(1.5, 0.0, -0.5);
        app.world_mut()
            .get_mut::<player::KccState>(player_entity)
            .expect("set_camera_mode spawned a KccState")
            .velocity = player_velocity;

        // Force transform propagation once so the player's `GlobalTransform`
        // reflects the `Transform` `set_camera_mode` just set -- this
        // minimal App has no render/window stack driving `app.update()`, so
        // propagation is run directly by schedule label.
        app.world_mut().run_schedule(PostUpdate);

        run_one_fixed_tick(app.world_mut());

        let character_transform = app.world().get::<Transform>(character_entity).unwrap();
        assert!(
            character_transform.translation.distance(player_center) < 1e-3,
            "the character must mirror the player's position, got {:?}",
            character_transform.translation
        );
        let character_velocity = app
            .world()
            .get::<Velocity3d>(character_entity)
            .unwrap()
            .velocity;
        assert_eq!(
            character_velocity, player_velocity,
            "the character must mirror the player's actual KCC velocity"
        );

        let archipelago_ref = app
            .world()
            .get::<ArchipelagoRef3d>(character_entity)
            .expect("the character carries an ArchipelagoRef3d");
        assert_eq!(
            archipelago_ref.entity, archipelago_entity,
            "the character is present in the same archipelago the agent/island use"
        );
    }

    /// Task 2 (continued): a landmass character standing directly on an
    /// agent's straight-line path deflects the agent's desired velocity away
    /// from that straight line -- RVO avoidance treating the character as a
    /// non-agent obstacle, driven against a real archipelago (the same
    /// pattern `nav_overlay.rs`'s own landmass harness test uses).
    #[test]
    fn a_landmass_character_in_the_agents_path_deflects_its_desired_velocity() {
        let mut app = fixed_tick_test_app();
        let archipelago_entity = spawn_fixture_island(app.world_mut());

        // `start`/`target` are the logical navmesh-plane (`y = 0.0`) points
        // the straight-line/character-placement math below works in;
        // `spawn_bare_agent` gets a *capsule-centre* start position instead
        // (`Transform.translation` is the capsule centre, mirroring
        // production's `spawn_test_agent`) so it actually stands on
        // `fixed_tick_test_app`'s floor rather than starting embedded in it.
        let start = Vec3::new(0.5, 0.0, 0.5);
        let target = Vec3::new(3.5, 0.0, 3.5);
        let agent = spawn_bare_agent(
            app.world_mut(),
            archipelago_entity,
            Vec3::new(start.x, AGENT_HEIGHT / 2.0, start.z),
            target,
        );

        // A character close enough to the agent's straight-line path that a
        // collision is predicted from the very first tick (RVO's avoidance
        // only predicts a collision within its 0.5s time horizon --
        // `ArchipelagoOptions::from_agent_radius`'s default -- so a
        // character sitting far down the path would not yet register as a
        // threat at the agent's initial, still-ramping-up speed), nudged a
        // hair off the path's exact centreline. A perfectly centred,
        // perfectly head-on approach is a degenerate case for RVO/ORCA --
        // slowing straight down is exactly as valid a non-colliding
        // solution as swerving either way when the geometry is perfectly
        // symmetric, and dodgy_2d picks that (confirmed empirically: dead-
        // centre placement here converges on a shrinking, undeflected
        // desired velocity, not a sideways one). A small perpendicular
        // offset breaks the symmetry the same way a real player almost
        // never walks exactly down an agent's route centreline.
        let direction = (target - start).normalize();
        let perpendicular = Vec3::new(-direction.z, 0.0, direction.x);
        let close_point = start + direction * 1.0 + perpendicular * 0.15;
        app.world_mut().spawn((
            Character3dBundle {
                character: default(),
                settings: CharacterSettings {
                    radius: player::CAPSULE_RADIUS,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            },
            Transform::from_translation(close_point),
            Velocity3d::default(),
        ));

        // Default solve rate (every tick): let the solve settle over enough
        // ticks for the agent to close in on the character and for RVO's
        // avoidance response to actually deflect it.
        for _ in 0..60 {
            run_one_fixed_tick(app.world_mut());
        }

        let blend = app.world().get::<AgentDesiredVelocityBlend>(agent).unwrap();
        let desired = blend.latest;
        assert!(
            desired.length() > 0.01,
            "the agent must still have a nonzero desired velocity with the character present, got {desired:?}"
        );

        let straight_line = (target - start).normalize();
        let desired_direction = desired.normalize();
        let cos_angle = straight_line.dot(desired_direction);
        assert!(
            cos_angle < 0.99,
            "a character blocking the straight-line path must deflect the agent's desired velocity away from it (cos={cos_angle}, desired={desired:?})"
        );
    }

    /// Task 3 (solve-output interpolation, user-directed addendum): at
    /// interval 2, on the in-between (skip) tick, the desired velocity
    /// `apply_agent_physics_movement` actually applies is strictly between
    /// the two most recently completed solve outputs -- not equal to
    /// either. At interval 1, it is always exactly the latest solved value,
    /// regardless of whatever `previous` holds -- confirming the
    /// interpolation is an exact no-op at the default rate. Uses an empty
    /// `boxddd::World` (no static geometry) so the achieved horizontal
    /// velocity written back to `Velocity3d` is the *unobstructed* applied
    /// input exactly -- a direct, physics-real assertion on the actual
    /// consuming system, not just the pure `solve_blend_fraction` table.
    #[test]
    fn desired_velocity_blends_between_solves_and_is_exact_at_interval_one() {
        use bevy::ecs::system::RunSystemOnce;

        fn blend_test_world(blend: AgentDesiredVelocityBlend) -> (World, Entity) {
            let mut world = World::new();
            world.init_resource::<TestNavAgentState>();
            world.insert_resource(PhysicsDisabled(false));
            world.insert_resource(CellPhysicsReadiness::Ready);
            world.init_resource::<Time>();
            world
                .resource_mut::<Time>()
                .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
            world.insert_non_send(BoxdddPhysicsContext::from_world(
                boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world"),
            ));
            let agent = world
                .spawn((
                    TestNavAgentMarker,
                    AgentKcc::default(),
                    blend,
                    Transform::from_xyz(0.0, 5.0, 0.0),
                    Velocity3d::default(),
                ))
                .id();
            (world, agent)
        }

        let previous = Vec3::new(2.5, 0.0, 0.0);
        let latest = Vec3::new(0.0, 0.0, 2.5);
        let blend = AgentDesiredVelocityBlend { previous, latest };

        // Interval 2, on a skip tick (3 % 2 = 1, fraction 0.5): strictly
        // between the two, not equal to either.
        let (mut world, agent) = blend_test_world(blend);
        world.insert_resource(NavSolveRate(2));
        world.insert_resource(NavSolveStepCounter(3));
        world
            .run_system_once(apply_agent_physics_movement)
            .expect("system runs");
        let achieved = world.get::<Velocity3d>(agent).unwrap().velocity;
        assert!(
            achieved.x > 0.0 && achieved.x < previous.x,
            "achieved.x={} must be strictly between 0.0 (latest.x) and {} (previous.x)",
            achieved.x,
            previous.x
        );
        assert!(
            achieved.z > 0.0 && achieved.z < latest.z,
            "achieved.z={} must be strictly between 0.0 (previous.z) and {} (latest.z)",
            achieved.z,
            latest.z
        );

        // Interval 1: always exactly the latest value, regardless of the
        // step counter or of `previous`.
        let (mut world, agent) = blend_test_world(blend);
        world.insert_resource(NavSolveRate(1));
        world.insert_resource(NavSolveStepCounter(7));
        world
            .run_system_once(apply_agent_physics_movement)
            .expect("system runs");
        let achieved = world.get::<Velocity3d>(agent).unwrap().velocity;
        assert!(
            (achieved.x - latest.x).abs() < 1e-3,
            "at interval 1 the applied value must equal `latest` exactly, got achieved.x={}",
            achieved.x
        );
        assert!(
            (achieved.z - latest.z).abs() < 1e-3,
            "at interval 1 the applied value must equal `latest` exactly, got achieved.z={}",
            achieved.z
        );
    }
}
