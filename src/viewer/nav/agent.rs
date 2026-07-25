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
//! Any number of agents can be spawned at once (issue #215 removed the
//! original 4-slot cap -- local avoidance among same-cell test agents was
//! otherwise unobservable with a single test agent, but the roster itself
//! is a growable `Vec`, not a fixed budget): `tna` subcommands take an
//! optional leading agent index, with every previously-single-agent command
//! form left unchanged and defaulting to agent 0.
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
//! - **Route-crossing containment check** (the one implemented; corridor-
//!   based since issue #155, see below): every single-sided door's
//!   triangle -- travel-door candidate or not -- stays walkable at all
//!   times and is a crossing-gate candidate
//!   (`NavArchipelagoState::mid_route_doors`), exactly mirroring how
//!   `TRAVEL_ARRIVAL_DISTANCE` already gates travel-door *arrival*.
//!   Containment of the agent's own position within a candidate's actual
//!   triangle footprint (`landmass_graph::point_in_door_triangle`), checked
//!   inside `drive_door_link_for_agent`'s existing `Idle`/`Failed`/
//!   `TravelReached` arm right after the travel-arrival check, fires the
//!   *same* `DoorLinkEvent::LinkReached` the off-mesh link case fires --
//!   but always with an `IntraCell` destination, never `Travel`: crossing a
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
//! Doors as conditional route topology (issue #155, M4 wave 8): the
//! mid-route gate above still only fires once the agent is already
//! standing on the door triangle -- it says nothing to `landmass`'s own
//! solver about a locked door being avoidable, so a route whose *only*
//! path crossed a locked door used to walk all the way there before
//! discovering the failure. Three changes close that gap:
//!
//! 1. **Typed door polygons** (`landmass_graph::door_type_indices`/
//!    `build_navigation_mesh`'s new parameter): every door-associated
//!    triangle, across every mesh in the archipelago, gets a `landmass`
//!    polygon type index unique to its door FormID (`0` stays "ordinary
//!    walkable ground", untouched). Typing only changes which
//!    `type_index_to_cost` a polygon looks up during a solve -- it does not
//!    remove the polygon or its adjacency, so an unlocked typed door still
//!    connects its neighbours exactly as before.
//! 2. **Query-time lock exclusion** (`LOCKED_DOOR_TYPE_INDEX_COST`,
//!    `apply_door_lock_overrides`): every spawned agent carries a
//!    `bevy_landmass::AgentTypeIndexCostOverrides` component rebuilt from
//!    `NavArchipelagoState::door_usable` at spawn and on every
//!    `door_availability_system` flip -- a locked door's type index gets
//!    the sentinel cost, an unlocked/open one gets none (the component is
//!    replaced wholesale each rebuild, not patched, since `bevy_landmass`
//!    exposes no public "remove one override" call). `landmass` retries
//!    pathfinding on its own every tick an agent has no current path
//!    (`does_agent_need_repath`'s `current_path: None` case), so an agent
//!    that is idle or already failed picks up a lock change on its very
//!    next solve with no explicit retarget needed; an agent already
//!    mid-transit through a door at the instant it locks keeps following
//!    its already-computed (structurally still valid) path until its next
//!    genuine repath -- a known, narrow scope cut (see `agent.rs`'s test
//!    module for the invariant coverage this does provide).
//! 3. **Distinct failure status** (`resolve_status`): a door lifecycle that
//!    gave up waiting (`door_link::DoorLinkState::Failed`, the same
//!    `MAX_WAIT_TICKS` terminal a locked mid-route crossing above also
//!    reaches) now resolves to `NavAgentStatus::Unreachable` instead of
//!    `Paused` -- the same word the stable `nav agent unreachable` log line
//!    at that call site already used, so `tna status`/the HUD finally agree
//!    with the log instead of contradicting it.
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
//!
//! M4 wave 10 (#162, #168, #169), landing together against this same seam:
//!
//! - **Preferred-path base cost** (#168): issue #156 typed every authored
//!   preferred-pathing polygon (`landmass_graph::preferred_pathing_type_
//!   index`) but never priced the type, so routing behaviour was
//!   unchanged. `ensure_archipelago` now calls `Archipelago::
//!   set_type_index_cost` with [`PREFERRED_PATHING_TYPE_INDEX_COST`] (a
//!   fixed `< 1.0` multiplier, see that constant's own doc comment) once
//!   per build -- a shared terrain preference every agent in the
//!   archipelago gets, unlike the two per-agent mechanisms below.
//! - **Lock overrides derived at build** (#169): `ensure_archipelago`
//!   used to rebuild `door_lock_info` purely from the manifest's authored
//!   data on every (re)build, silently discarding any `setlock` recorded
//!   in `NavArchipelagoState.door_lock_info` before that build ran -- an
//!   early `setlock` (issued before the very first `tna spawn`, since
//!   `NavArchipelagoState` is `init_resource`d and writable well before
//!   that) was lost, leaving the door's query-time cost override at its
//!   stale authored value. The freshly-read authored map is now overlaid
//!   with whatever the *current* resource already holds, per door FormID,
//!   before the resource is reset -- runtime always wins over authored for
//!   any door a `setlock` has actually touched.
//! - **Per-link portal quarantine** (#162): `merge_traversal_system`'s
//!   timeout branch used to clear the agent's `AgentTarget3d`/
//!   `travel_intent` outright on a blocked merge crossing (the wave-8
//!   minimum-viable mitigation) so the solver could not immediately
//!   re-select the same bad link -- effective, but it threw the real
//!   destination away instead of routing around the one bad seam. Every
//!   validated merge candidate now gets its own `landmass` animation-link
//!   `kind` (`landmass_graph::merge_link_kind`, distinct from door
//!   locking's polygon *type-index* scheme -- a merge portal has no
//!   polygon to type, but `AnimationLink3d::kind` is a property of the
//!   off-mesh link itself, so this achieves exact single-link granularity
//!   for free). A timed-out crossing adds that one kind to the agent's own
//!   `AgentRuntime::quarantined_merge_link_kinds` and rebuilds its
//!   `PermittedAnimationLinks` (`permitted_animation_links_for`) to
//!   exclude just it -- kind `0` (every door link) is never touched, so a
//!   blocked merge seam can never lock an unrelated door. The real target
//!   is kept, not cleared: `AgentTarget3d` is blanked to `None` for
//!   exactly one tick (`PendingMergeRepath`, `resume_pending_merge_repath_
//!   system`) to force landmass's own solver to discard the now-stale
//!   corridor and search again with the updated exclusion (see that
//!   system's doc comment for why a per-agent field change alone is not
//!   enough), instead of resuming the identical path through the same
//!   blocked link. The quarantine is per-agent and lives only as long as
//!   the routing intent it excludes something from: cleared to empty on
//!   every new `tna goto`/`tna travel` target (`clear_merge_link_
//!   quarantine`) and implicitly on despawn/hand-off (it is ordinary
//!   `AgentRuntime`/`PermittedAnimationLinks` component state, gone with
//!   the entity) -- never a global or persistent portal blacklist. When no
//!   alternate route exists, landmass's own solve reports `NoPath`
//!   through the existing `AgentState::NoPath -> NavAgentStatus::
//!   Unreachable` mapping, the same fail-fast surface every other
//!   unreachable-route case already uses.
//!
//! Key-aware locked doors (issue #185, following up on #177): every
//! `door_open_and_locked`/`door_usable_now` call above used to decide
//! "locked" by checking the *player's* `PlayerInventory` for the door's key
//! -- a stand-in that predates #188's bound actors, and simply wrong for an
//! NPC nav agent, which has no relationship to the player's inventory.
//! OpenMW's `AiPackage::openDoors()` searches the *routing actor's own*
//! inventory instead, so both functions now take an `Option<Entity>` naming
//! whose key to check: `Some(agent_entity)` at every genuinely per-agent
//! call site (`apply_door_lock_overrides`, `request_door_open`, and every
//! `drive_door_link_for_agent` lock/open check), `None` for the two
//! agent-independent bookkeeping paths (`ensure_archipelago`'s initial
//! build, `door_availability_system`'s change-detection poll) that have no
//! particular actor to ask. The actual key/lock/trap decision table itself
//! is `openmw_doors::door_openable` (see that module's own doc comment and
//! provenance files) -- a trapped door is an unconditional non-openable
//! veto, a deliberate simplification of OpenMW's literal fall-through since
//! this project has no trap-spring mechanic to make opening one safe.
//! `apply_door_lock_overrides` re-checks a door the shared, actor-
//! independent cache calls unusable against this agent's own key
//! specifically (`agent_may_open_with_key`) before pricing it as an
//! impassable barrier -- narrowly scoped to doors that actually have
//! `door_lock_info` at all, since the same override cache is shared with
//! #177's lock-less activator-blocker class. `goto_agent`/`request_travel`
//! both re-run this override rebuild on every fresh target, so a key
//! granted mid-session (`console::giveitem`) or a lock/key change
//! (`setlock`'s new optional key argument) is picked up by the very next
//! routing command without needing a respawn.
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;

use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_boxddd::boxddd;
use bevy_boxddd::prelude::BoxdddPhysicsContext;
use bevy_landmass::coords::ThreeD;
use bevy_landmass::prelude::*;
use bevy_landmass::{
    AgentTypeIndexCostOverrides, NavMeshHandle, PauseAgent, PermittedAnimationLinks,
    PointSampleDistance3d, TargetReachedCondition, UsingAnimationLink,
};
use serde_json::json;

use crate::console::{ConsoleCommandResult, ConsoleError, ConsoleInvocation};
use crate::viewer::actor::ActorRuntime;
#[cfg(test)]
use crate::vsa::PreparedSceneManifest;

use super::super::openmw_player::GRAVITY;
use super::super::player::{CellPhysicsReadiness, PhysicsDisabled};
use super::super::{interaction, player};
use super::{door_link, landmass_graph, ledger_policy, movement_policy, repath};

// Issue #164 fall-out-of-world guard. The pure policy is a std-only sibling
// module (same `#[path]` include tests/features.rs uses); declared here as a
// private submodule of `agent` -- via `#[path]` so it resolves to the shared
// `nav/fall_guard.rs` file -- rather than in `nav/mod.rs`, whose ownership
// this wave's guard work does not include.
#[path = "fall_guard.rs"]
mod fall_guard;

// Issue #188. `locomotion` is the pure std-only speed/turn -> clip policy
// (same `#[path]` submodule rationale as `fall_guard` above: it is included
// verbatim by `tests/features.rs` too). `actor_binding` is its Bevy
// consumer plus the projected-actor <-> nav-agent binding itself; it lives
// out here rather than in this module root because `agent.rs` is already
// 9k+ lines doing six jobs (post-mortem verdict §2.6) and must not grow a
// seventh inline.
#[path = "locomotion.rs"]
mod locomotion;

#[path = "actor_binding.rs"]
mod actor_binding;

// Issue #185. Pure OpenMW-derived key/lock/trap decision rule (see
// `openmw_doors/README.md`/`NOTICE.md` for provenance); same `#[path]`
// submodule rationale as `fall_guard`/`locomotion` above -- included
// verbatim by `tests/features.rs`.
#[path = "openmw_doors/mod.rs"]
mod openmw_doors;

const AGENT_RADIUS: f32 = 0.35;
const AGENT_HEIGHT: f32 = 1.8;
const AGENT_DESIRED_SPEED: f32 = 2.5;
/// Contact normals at or above this Y are floor-like; below it they are
/// walls/steep faces, i.e. the things that actually block an agent.
const WALKABLE_CONTACT_NORMAL_Y: f32 = std::f32::consts::FRAC_1_SQRT_2;
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
/// Per-agent `landmass` type-index cost applied to a locked door's polygon
/// type (issue #155 feature 2): high enough that any plausible detour
/// within a single loaded cell is always cheaper, without being literal
/// `f32::INFINITY` itself, after verification against `landmass` 0.9.2's
/// source -- see below for why a large finite sentinel, the wave plan's
/// other option, would silently fail the "exact exclusion" requirement.
///
/// **Why exclusion, not just a strong penalty:** the issue's acceptance
/// test is "locking a door on the only route makes the route fail at query
/// time" -- `AgentState::NoPath`, not merely a longer/costlier path. `A*`
/// never refuses to return a path solely because it is expensive: any
/// *finite* cost, however large, still yields a "successful" (if ugly)
/// route whenever one topologically exists, so a large-but-finite sentinel
/// would make a locked door on the *only* route still pathable -- the
/// exact bug this feature exists to fix. Only an actually-unbounded
/// (`is_finite() == false`) cost makes `landmass` refuse the edge outright.
///
/// **Verification (per the wave plan's "verified-safe exclusion semantics"
/// requirement, recorded on issue #155):** `landmass` 0.9.2's own
/// `pathfinding.rs::ArchipelagoPathProblem::successors` filters every
/// *destination*-node candidate on `target_node_cost.is_finite()` *before*
/// ever computing `distance * current_node_cost` for it (the multiplication
/// that a literal `INFINITY` cost could otherwise turn into `0.0 * inf =
/// NaN` if `distance` happened to be exactly `0.0`) -- so transitioning
/// *into* a locked-cost polygon is filtered out categorically, never
/// multiplied at all. `landmass`'s own test (`pathfinding_test.rs::
/// infinite_or_nan_cost_cannot_find_path_between_nodes`) pins exactly this
/// with a literal `f32::INFINITY` type-index cost on the only route between
/// two nodes and confirms it produces `path: None`, `explored_nodes: 1`,
/// with no panic -- the identical shape this project's own invariant tests
/// (`agent.rs`'s `door_topology_mesh`-based tests) exercise against a live
/// `Archipelago3d`.
///
/// **The one gap literal infinity leaves open:** `current_node_cost` (the
/// *source* polygon's own cost, used for the edges *leaving* it) is read
/// unconditionally in `successors`, with no `is_finite()` guard -- so an
/// agent whose search *starts* already standing inside a just-locked
/// polygon, at a position exactly `0.0` from one of its edge midpoints,
/// could in principle still hit `0.0 * inf = NaN` for that one polygon's
/// own outgoing edges. This project's structural exclusion for two-sided
/// doors (the off-mesh link is despawned outright, not cost-penalised) and
/// the corridor-based mid-route gate (`point_in_door_triangle`, issue
/// #137/#155 -- an agent is paused for a closed/locked door the moment its
/// position enters the door triangle, rather than being left free to
/// settle at an arbitrary point within it) both keep an agent from
/// starting a fresh path query with its position resolved to a locked
/// door's own polygon in the first place; combined with "distance to an
/// edge midpoint is exactly `0.0`" being a measure-zero floating-point
/// coincidence even then, this stays a theoretical edge case rather than
/// an observed one -- recorded here rather than worked around, since doing
/// so would mean giving up on `NoPath` for the one-route-only case, this
/// feature's actual requirement.
const LOCKED_DOOR_TYPE_INDEX_COST: f32 = f32::INFINITY;
/// Per-agent `landmass` type-index cost applied to a *closed but openable*
/// blocker's interior polygons (issue #177 acceptance correction): the ground
/// inside a shut door slab, for a door the agent can actually open.
///
/// Deliberately large-but-finite, the exact opposite of
/// [`LOCKED_DOOR_TYPE_INDEX_COST`]'s reasoning above, because the requirement
/// is the opposite. A locked door must make its route *fail* (`NoPath`), so
/// only an unbounded cost will do. A closed-but-unlocked door must remain
/// **passable**: the crossing gate that opens it only fires once the agent is
/// standing on the doorway, so a route the solver refuses to plan is a route
/// the agent never walks, a door it never approaches, and a door it therefore
/// never opens -- the chicken-and-egg the first cut of this issue shipped,
/// where every in-cell door became a wall that reported `unreachable` from
/// the spawn point. This matches FO3/GECK semantics: NPCs path through closed
/// doors and open them; only locked doors are barriers.
///
/// The magnitude makes any genuinely available detour cheaper, so an agent
/// prefers an open route and only commits to opening a door when that is
/// really the way through, while staying far below `f32::MAX` so accumulated
/// path costs cannot overflow to infinity and re-introduce the exclusion this
/// constant exists to avoid.
const CLOSED_DOOR_TYPE_INDEX_COST: f32 = 1000.0;
/// Archipelago-wide base cost for every authored preferred-pathing polygon
/// (issue #168): `landmass_graph::preferred_pathing_type_index`'s type index
/// gets this via `Archipelago::set_type_index_cost` in `ensure_archipelago`,
/// the base-cost wiring issue #156 (feature 1, typing only) left for a
/// future wave. `1.0` is `landmass`'s implicit cost for any type index
/// nothing has overridden (`landmass::pathfinding::type_index_to_cost`,
/// confirmed by `nav_data.rs`'s own `HashMap::new()` default lookup) -- a
/// value strictly less than that makes distance-weighted route selection
/// favour a preferred corridor over a same-length ordinary one, matching
/// GECK `PREFERRED_PATHING` semantics (NPCs are drawn to these paths, not
/// forced onto them the way a locked door is forced off one). `0.5`:
/// meaningfully cheaper (half the per-metre cost) without approaching the
/// near-zero costs that would pathologically attract routing regardless of
/// how long a preferred-path detour actually is. Verified against
/// `landmass` 0.9.2's only `set_type_index_cost` validation
/// (`nav_data.rs::set_type_index_cost`: `cost <= 0.0` is rejected via
/// `SetTypeIndexCostError::NonPositiveCost`; `NaN`/`f32::INFINITY` both
/// otherwise pass unchecked, unlike that guard) -- `0.5` is comfortably a
/// positive, finite, ordinary multiplier, nothing like
/// [`LOCKED_DOOR_TYPE_INDEX_COST`]'s deliberate sentinel-infinity shape.
/// Applied once per archipelago build via `Archipelago::set_type_index_cost`
/// (a base cost every agent shares), not per-agent
/// `AgentTypeIndexCostOverrides` -- that mechanism stays reserved for door
/// lock exclusion (#155) and merge-portal quarantine (#162), both
/// per-agent exceptions to the shared baseline this constant sets.
const PREFERRED_PATHING_TYPE_INDEX_COST: f32 = 0.5;
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
/// How close (metres, full 3D) `validate_merge_link_collision`'s one-shot
/// `player::move_mover` slide must land to a merge candidate's far portal
/// point to count as "arrived" (issue #154 real-data acceptance
/// correction). Deliberately looser than `MERGE_TRAVERSAL_REACHED_DISTANCE`
/// (which is horizontal-only and compares against a live, already-moving
/// agent): this is a single static slide budgeted at
/// `player::mod::MAX_SLIDE_PASSES` correction passes, not a full per-tick
/// crossing, so a small full-3D residual after sliding off one nearby
/// surface is expected on an otherwise-clear seam.
const MERGE_LINK_SWEEP_TOLERANCE: f32 = 0.6;

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

/// How far ahead (seconds) landmass's local avoidance may look when treating
/// *navmesh border edges* as ORCA obstacles (issue #184). This **disables**
/// border avoidance: it is expressed as a negligible positive horizon rather
/// than `0.0` only because `dodgy_2d` divides by it (`left_cutoff =
/// (vertex - position) / time_horizon`), so zero would put `inf`/`NaN` into the
/// linear program. At `AGENT_MAX_SPEED` this is a lookahead well under a
/// millimetre -- far below any geometric tolerance in the pipeline -- while
/// keeping every squared quantity comfortably inside `f32` range.
/// `avoidance_time_horizon` (agent/character avoidance, issue #114 feature 4)
/// is a separate option and keeps landmass's own `0.5` default; only borders
/// are switched off here.
///
/// Two independent facts make navmesh borders the wrong avoidance authority
/// for this project:
///
/// 1. **Physics owns wall clearance, not the navmesh.** Since issue #114
///    movement is physics-authoritative -- the agent capsule KCC resolves the
///    real cooked colliders every tick (`step_agent_kcc`) -- and since issues
///    #153/#171 the *prepared* mesh boundary is already the agent-radius
///    clearance boundary (`vsa::prepare::nav_clearance`). Border avoidance is
///    therefore a second, redundant wall-avoidance layer, and unlike the first
///    it acts in *velocity space with no contact*, so its failures are
///    invisible to every collision diagnostic.
/// 2. **`dodgy_2d` is strictly 2D, and landmass feeds it vertically-separated
///    geometry.** `landmass::avoidance::nav_mesh_borders_to_dodgy_obstacles`
///    walks outward through *connected* polygons for `neighbourhood` metres and
///    projects every border it finds onto the XZ plane. A staircase is walkable
///    ground connected to the landing above it, so its rail gets flattened onto
///    the landing's own footprint -- on 00024512, 125 border edges spanning
///    y 39.17..40.47, up to a metre below the agent's surface, with #171's
///    sub-triangle re-triangulation contributing many near-collinear slivers.
///    In any multi-level cell that set simply is not a valid description of
///    what the agent must avoid, and `dodgy_2d` treats obstacle lines as *hard*
///    constraints: a degenerate set makes `solve_linear_program` infeasible and
///    it falls back to "whatever solution we get even if it's infeasible".
///
/// The observable failure is a velocity that decays by exactly
/// `1 - dt / horizon` per tick toward zero. On 00024512 an agent with 1.35 m of
/// clearance in every direction and a completely free capsule sweep crept to a
/// permanent halt against borders 2-3.5 m away, reporting
/// `reason=no_contact_no_progress` -- four waves' worth of collider hunting for
/// an obstacle that was never physical. Merely shortening the horizon is not
/// enough: at one fixed tick the decay stops, but a border projected onto the
/// agent's own position still hard-blocks it (measured on this file's own
/// `stall_fixture_mesh`, which halts at the projected stair cap until the
/// horizon goes below ~1e-3).
const NAV_BORDER_AVOIDANCE_TIME_HORIZON: f32 = 1e-4;

/// The archipelago options every build shares (issue #184): landmass's
/// `from_agent_radius` avoidance defaults, with the point-sampling envelope
/// widened to humanoid scale (`AGENT_POINT_SAMPLE_DISTANCE`) and navmesh-border
/// ORCA avoidance clamped to one tick (`NAV_BORDER_AVOIDANCE_TIME_HORIZON`).
/// One helper rather than per-call-site literals so `ensure_archipelago` and
/// every test harness cannot drift apart on exactly the options a stall
/// regression depends on.
fn archipelago_options() -> ArchipelagoOptions<ThreeD> {
    let mut options = ArchipelagoOptions::from_agent_radius(AGENT_RADIUS);
    options.point_sample_distance = AGENT_POINT_SAMPLE_DISTANCE;
    options.obstacle_avoidance_time_horizon = NAV_BORDER_AVOIDANCE_TIME_HORIZON;
    options
}

/// `TestNavAgentState`'s initial pre-allocated slot count (issue #215):
/// purely a small default capacity carried over from the original fixed
/// roster size, not a cap -- `TestNavAgentState::set` grows the vector on
/// demand past this for any higher index.
const INITIAL_AGENT_SLOTS: usize = 4;

/// Defensive ceiling for the console-addressed debug roster. The roster is
/// growable well past the old four-agent limit, but it is still a dense `Vec`:
/// accepting an arbitrary `usize` would let input such as `usize::MAX` request
/// an impossible allocation. 65,536 simultaneous debug agents is far beyond a
/// viable scene while keeping worst-case growth small and ledger IDs unique.
const MAX_AGENT_INDEX: usize = u16::MAX as usize;

/// The ledger/tracing identity for agent `index`: stable, 1-based so it
/// never collides with the "no id" sentinel `0`, consistent with wave 3/4's
/// single `TEST_AGENT_ID = 1`. Formatted as a small decimal in tracing lines
/// (it identifies a spawn slot, not a FormID), but still handed to
/// `ledger_policy` as a plain `u32`.
fn agent_ledger_id(index: usize) -> u32 {
    debug_assert!(index <= MAX_AGENT_INDEX);
    index as u32 + 1
}

/// Marks a test nav agent this console command family drives. `Entity`
/// identity plus `TestNavAgentState::index_of` recovers which of the
/// (unbounded, issue #215) roster slots an entity belongs to.
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
    /// This specific link's `landmass` animation-link kind (issue #162,
    /// `landmass_graph::merge_link_kind`), captured from the matched
    /// `LinkKind::Merge` at crossing start -- the identity
    /// `merge_traversal_system`'s timeout branch quarantines for this
    /// agent alone if the crossing fails, instead of clearing the whole
    /// route.
    link_kind: usize,
}

/// A capture of `AgentTarget3d`'s two meaningful variants (issue #162):
/// `AgentTarget3d` itself is not `Clone` (`bevy_landmass::AgentTarget`'s
/// derive is `Component, Default` only), so this is the plain-data stand-in
/// [`PendingMergeRepath`] holds across the one-tick target-blank window
/// `resume_pending_merge_repath_system` closes. `None` is deliberately not
/// representable here: `merge_traversal_system`'s timeout branch only ever
/// captures a target worth restoring, never a blank one (see its call
/// site).
#[derive(Debug, Clone, Copy)]
enum AgentTargetSnapshot {
    Point(Vec3),
    Entity(Entity),
}

impl AgentTargetSnapshot {
    fn capture(target: &AgentTarget3d) -> Option<Self> {
        match target {
            AgentTarget3d::Point(point) => Some(Self::Point(*point)),
            AgentTarget3d::Entity(entity) => Some(Self::Entity(*entity)),
            AgentTarget3d::None => None,
        }
    }

    fn to_agent_target(self) -> AgentTarget3d {
        match self {
            Self::Point(point) => AgentTarget3d::Point(point),
            Self::Entity(entity) => AgentTarget3d::Entity(entity),
        }
    }
}

/// Present on an agent entity for exactly one fixed tick after
/// `merge_traversal_system`'s timeout branch (issue #162) deliberately
/// blanks `AgentTarget3d` to `AgentTarget3d::None` -- see
/// `resume_pending_merge_repath_system`'s doc comment for why that blank
/// tick is necessary to force a genuine repath rather than a per-agent
/// field change alone.
#[derive(Component)]
struct PendingMergeRepath {
    target: AgentTargetSnapshot,
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
    /// Y offset from this agent entity's `Transform.translation` to the KCC
    /// capsule *centre* (issue #188). Zero for `tna` capsules, whose entity
    /// transform already is the capsule centre; `AGENT_HEIGHT / 2.0` for a
    /// bound actor, whose placement root sits at feet level. Applied by
    /// `apply_agent_physics_movement` on the way into and back out of the
    /// sweep, so exactly one convention (capsule centre) reaches the KCC
    /// regardless of what the entity's own transform means.
    capsule_centre_offset_y: f32,
    /// This tick's desired horizontal velocity, exactly as
    /// `apply_agent_physics_movement` blended it before handing it to
    /// `step_agent_kcc` (issue #188). Stashed here purely so the locomotion
    /// consumer in `actor_binding` can *reuse* the `desired`/`achieved` pair
    /// this system already computes instead of recomputing a second,
    /// possibly disagreeing one.
    last_desired_horizontal: Vec2,
    /// The achieved half of that same pair: signed horizontal velocity in
    /// `[x, z]`. The sign lets the locomotion window cancel back-and-forth
    /// collision jitter instead of mistaking its magnitude for travel.
    last_achieved_horizontal: Vec2,
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
/// open -> traverse lifecycle). `Merge`'s `kind` (issue #162) is this
/// specific portal's `landmass` animation-link kind
/// (`landmass_graph::merge_link_kind`), the identity a per-agent quarantine
/// excludes -- carried alongside the variant (not looked up separately)
/// so `drive_door_link_for_agent` can stash it straight onto the
/// `MergeTraversal` it starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LinkKind {
    Merge { kind: usize },
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
    /// Door FormID -> `landmass` polygon type index (issue #155 feature 1),
    /// the same archipelago-wide mapping `landmass_graph::door_type_indices`
    /// computed for this build's `build_navigation_mesh` calls -- kept here
    /// so `door_availability_system`/`spawn_test_agent` can translate a
    /// door's lock state into the matching `AgentTypeIndexCostOverrides`
    /// entry without recomputing it from the raw mesh inputs every time.
    door_type_indices: BTreeMap<u32, usize>,
    /// Blocker FormID -> `landmass` polygon type index for the *blocking*
    /// derived association class (issue #177, `landmass_graph::
    /// closed_door_type_indices`): the polygons that lie wholly inside the
    /// blocker's collision volume. Priced [`LOCKED_DOOR_TYPE_INDEX_COST`]
    /// whenever the blocker is *closed* -- lock or no lock -- so no route
    /// can ever be planned through the inside of a closed door slab, which
    /// is what let an agent walk in and wedge against it in physics.
    closed_door_type_indices: BTreeMap<u32, usize>,
    /// Blockers that own a runtime open/close FSM (`landmass_graph::
    /// openable_blockers`). Decides whether a closed blocker's interior is
    /// merely expensive ([`CLOSED_DOOR_TYPE_INDEX_COST`]) or impassable.
    openable_blockers: BTreeSet<u32>,
    /// Last observed per-door *open* state, the change detector for the
    /// closed-blocker override above (`door_usable` cannot serve: an
    /// unlocked door is usable whether it is open or shut).
    door_open: HashMap<u32, bool>,
    /// How many distinct merge-portal `landmass` animation-link kinds this
    /// build assigned (issue #162 feature 1, `landmass_graph::
    /// merge_link_kind`): every validated merge candidate this build
    /// spawned a link pair for got kind `1..=merge_link_kind_count`, in
    /// spawn order. `permitted_animation_links_for` needs this to build
    /// the "everything except the quarantined kinds" allow-list
    /// `landmass::PermittedAnimationLinks::Kinds` requires. `0` when this
    /// cell has no merge portals at all.
    merge_link_kind_count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct DoorLockInfo {
    lock_level: Option<i8>,
    key_form_id: Option<u32>,
    /// Issue #185: `PreparedDoor::trapped`, captured the same way
    /// `lock_level`/`key_form_id` are.
    trapped: bool,
}

/// A door crossable mid-route (issue #137): any single-sided door
/// triangle, travel-door candidate or not -- real FO3 data shows nearly
/// every door resolves to a travel destination, so restricting this set to
/// non-travel doors left it empty and never gated anything. Left part of
/// the walkable island (see `nav/agent.rs`'s module doc for why); gated at
/// runtime by whether the agent's own position is inside `vertices`'
/// footprint (issue #155 feature 3, `landmass_graph::point_in_door_triangle`
/// -- replacing this file's earlier `MID_ROUTE_DOOR_GATE_DISTANCE`
/// centroid-proximity scan, which could gate a route that merely passed
/// *near* a doorway without ever crossing it), *except* for the one door a
/// given agent's own `travel_intent` currently targets.
#[derive(Debug, Clone, Copy)]
struct MidRouteDoor {
    door_form_id: u32,
    vertices: [Vec3; 3],
}

/// Per-agent bookkeeping that used to live in the single-agent
/// `TestNavAgentState` (waves 3/4), now a `Component` on each agent entity
/// so any number of agents (issue #215) can each carry their own
/// door-link/travel/diagnostics state without a parallel resource-side
/// index.
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
    /// Per-agent merge-portal quarantine (issue #162 feature 2): every
    /// `landmass_graph::merge_link_kind` this agent has timed out crossing,
    /// excluded from its own subsequent repaths via `PermittedAnimationLinks`
    /// (`merge_traversal_system`'s timeout branch adds to this;
    /// `permitted_animation_links_for` derives the component from it).
    /// Deliberately per-agent, never global/persistent: cleared to empty on
    /// every new `tna goto`/`tna travel` target (`clear_merge_link_
    /// quarantine`, called from `goto_agent`/`request_travel`) and
    /// implicitly on despawn/hand-off, since this field -- like the rest of
    /// `AgentRuntime` -- lives only as long as the entity itself.
    quarantined_merge_link_kinds: BTreeSet<usize>,
}

/// The growable roster of spawned test-agent entities, indexed by agent
/// index. All other per-agent state (`AgentRuntime`, `AgentKcc`,
/// door-link/traversal components) lives on the entity itself; this
/// resource only answers "which entity is agent N" and its inverse.
///
/// Issue #215: this used to be a fixed `[Option<Entity>; 4]` -- `bind_agent`
/// and `spawn_agent` rejected any index at or past that size, capping the
/// whole cell at 4 concurrent agents even though every other per-agent
/// state already lives on the entity and has no such limit. `entities` is
/// now a plain `Vec` that grows through [`Self::set`] on demand. A generous
/// defensive ceiling prevents hostile indices from turning that growth into
/// an out-of-memory allocation.
#[derive(Resource)]
struct TestNavAgentState {
    entities: Vec<Option<Entity>>,
}

impl Default for TestNavAgentState {
    fn default() -> Self {
        Self {
            entities: vec![None; INITIAL_AGENT_SLOTS],
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

    /// The entity occupying `index`, if any -- `None` both for an empty
    /// slot and for an index past the current length (an unallocated slot
    /// reads exactly like an empty one, not an error).
    fn get(&self, index: usize) -> Option<Entity> {
        self.entities.get(index).copied().flatten()
    }

    /// Whether `index` is currently occupied.
    fn is_occupied(&self, index: usize) -> bool {
        self.get(index).is_some()
    }

    /// Sets slot `index` to `value`, growing the vector with `None`s first
    /// if `index` is past the current length -- the growable replacement
    /// for the old fixed-size array's direct indexing (issue #215).
    fn set(&mut self, index: usize, value: Option<Entity>) {
        debug_assert!(index <= MAX_AGENT_INDEX);
        if index >= self.entities.len() {
            self.entities.resize(index + 1, None);
        }
        self.entities[index] = value;
    }
}

/// Renders one nav agent's identity for the stable `nav agent ...` evidence
/// lines and the debug HUD (issue #241).
///
/// Those lines used to interpolate the [`TestNavAgentState`] roster index and
/// were skipped *entirely* for an agent that had none -- which is every actor
/// the autonomous package driver binds through [`bind_agent_entity`], since
/// that path deliberately takes no console index. Five behaviours rode on the
/// same gate (door links, the fall guard, collision/stuck telemetry, state-
/// change logging, the HUD), so an autonomously-routed NPC silently lost all
/// of them and reported nothing at all -- which is why earlier investigations
/// into "actors get stuck" (#148) saw zero telemetry for exactly the actors
/// that were stuck. The roster is now purely console *addressing* (`tna goto
/// 2`), and the index is only the preferred *rendering* of an identity:
///
/// - a console-addressed agent keeps its bare decimal index, so every existing
///   line, doc and manual script reads exactly as before;
/// - any other bound actor renders its reference FormID in the `{formid:08x}`
///   form used everywhere else here -- always 8 hex digits, so it can never be
///   misread as a roster index;
/// - a bare `tna` capsule with neither falls back to its `e<entity>` id.
fn format_agent_id(
    roster_index: Option<usize>,
    reference_form_id: Option<u32>,
    entity: Entity,
) -> String {
    match (roster_index, reference_form_id) {
        (Some(index), _) => index.to_string(),
        (None, Some(form_id)) => format!("{form_id:08x}"),
        (None, None) => format!("e{entity}"),
    }
}

/// [`format_agent_id`] resolved against a whole world, for the exclusive
/// (`&mut World`) systems that have no query params to read the two sources
/// from.
fn agent_log_id(world: &World, entity: Entity) -> String {
    format_agent_id(
        world
            .get_resource::<TestNavAgentState>()
            .and_then(|roster| roster.index_of(entity)),
        world
            .get::<ActorRuntime>(entity)
            .map(|actor| actor.reference_form_id),
        entity,
    )
}

/// Every live nav agent, in a deterministic spawn-ish order, for the systems
/// that used to walk `TestNavAgentState::active()` instead. Sorted rather
/// than left in archetype order so a tick that drives several agents through
/// shared state (the door lifecycle) stays reproducible. Keyed on
/// `Entity::index_u32`, not `Entity`'s own `Ord`: the latter compares
/// `to_bits`, which bevy documents as opaque and which in practice orders
/// indices *backwards*.
fn all_agent_entities(world: &mut World) -> Vec<Entity> {
    let mut agents: Vec<Entity> = world
        .query_filtered::<Entity, With<TestNavAgentMarker>>()
        .iter(world)
        .collect();
    agents.sort_unstable_by_key(|entity| entity.index_u32());
    agents
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

/// Issue #164: despawns any nav agent that has fallen clear out of the world
/// -- its capsule-centre Y dropped a whole [`fall_guard::FALL_GUARD_MARGIN_METRES`]
/// below the active cell's lowest prepared geometry Y -- instead of letting
/// it descend forever under gravity (FranklinMetro02's walkable-navmesh-over-
/// missing-collision regions, cell `0001a273`). The verdict is the pure
/// `fall_guard::evaluate_fall` policy; this system only samples each agent's
/// real transform Y and the cell bounds, applies the verdict, and -- on a
/// fall -- emits the stable `nav agent fell out of world <id> ...` line,
/// despawns the entity, and clears its roster slot (the same teardown `tna
/// despawn` performs), so a subsequent `tna spawn` at that index works
/// normally. Runs after `apply_agent_physics_movement` in the `FixedUpdate`
/// chain, i.e. after this tick's gravity integration has updated the
/// capsule's Y. Player handling is out of scope (issue #164).
fn nav_fall_guard_system(world: &mut World) {
    let Some(min_y) = world.resource::<NavCellFallBounds>().min_y else {
        return;
    };
    // Issue #241: every nav agent, not just the roster-indexed ones -- an
    // autonomously-bound actor falling through missing collision must be
    // caught by the same guard.
    let fallen: Vec<(Entity, f32)> = all_agent_entities(world)
        .into_iter()
        .filter_map(|entity| {
            let agent_y = world.get::<Transform>(entity)?.translation.y;
            (fall_guard::evaluate_fall(min_y, agent_y) == fall_guard::FallVerdict::FellOutOfWorld)
                .then_some((entity, agent_y))
        })
        .collect();
    if fallen.is_empty() {
        return;
    }
    let kill_z = fall_guard::fall_kill_z(min_y);
    for (entity, agent_y) in fallen {
        let id = agent_log_id(world, entity);
        warn!("nav agent fell out of world {id} y={agent_y} kill_z={kill_z}");
        // Issue #188 feature 5: a fallen *bound actor* releases its agent
        // and its locomotion animation state instead of being deleted --
        // this guard exists to end a runaway descent, not to destroy an NPC
        // the world/actor slice owns. A `tna` capsule still despawns whole.
        if world.get::<actor_binding::NavBoundActor>(entity).is_some() {
            release_bound_actor(world, entity);
        } else if let Ok(entity_mut) = world.get_entity_mut(entity) {
            entity_mut.despawn();
        }
        // Only a *console-addressed* agent owns a roster slot to free; an
        // autonomously-bound one never had one (issue #241).
        if let Some(index) = world.resource::<TestNavAgentState>().index_of(entity) {
            world.resource_mut::<TestNavAgentState>().set(index, None);
        }
    }
}

/// The active cell's minimum prepared geometry Y (issue #164), captured from
/// the whole-graph nav bounds in `ensure_archipelago` when the archipelago
/// is (re)built. `None` until a cell with a prepared nav graph is loaded;
/// `nav_fall_guard_system` derives the kill plane from it via
/// `fall_guard::fall_kill_z`. Kept as an ordinary resource (not part of
/// `NavArchipelagoState`, which `teardown_archipelago` blanks) so its value
/// simply reflects whatever cell was last built.
#[derive(Resource, Default)]
struct NavCellFallBounds {
    min_y: Option<f32>,
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
            .init_resource::<NavCellFallBounds>()
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
                FixedPreUpdate,
                // Issue #162: gated on the same `nav_solve_gate` condition
                // as `LandmassSystems::Update` itself (not every fixed
                // tick) -- see `resume_pending_merge_repath_system`'s doc
                // comment for why its one-tick-later restore must line up
                // with an actual solve tick, not an arbitrary movement
                // tick, when `NavSolveRate` throttles the solve below the
                // fixed-tick cadence.
                resume_pending_merge_repath_system
                    .after(LandmassSystems::Output)
                    .run_if(nav_solve_gate),
            )
            .add_systems(
                FixedUpdate,
                (
                    despawn_stale_navmesh_archipelago,
                    restore_ledgered_agents_system,
                    door_availability_system,
                    door_link_system,
                    apply_agent_physics_movement,
                    nav_fall_guard_system,
                    // Issue #188, strictly after the KCC has moved and the
                    // fall guard has had its say: facing reads the desired/
                    // achieved pair this tick's movement just stashed, and
                    // the locomotion request must not be issued for an agent
                    // the guard is about to release.
                    actor_binding::face_bound_actors,
                    actor_binding::drive_bound_actor_locomotion,
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
    let (current_cell, path, travel_destinations, mut door_lock_info, door_positions) = {
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
                        trapped: door.trapped,
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

    // Issue #169: a runtime lock change already recorded before this build
    // -- an early `setlock` issued before any archipelago exists yet (this
    // resource is `init_resource`d at plugin install, so it exists and is
    // writable long before the first `tna spawn`), or one that landed
    // earlier in the same session -- must survive the rebuild below. The
    // authored baseline just read from the manifest is the fallback, not
    // the winner: overlay whatever `NavArchipelagoState.door_lock_info`
    // (the exact map `set_door_lock_level`/`setlock` writes into) already
    // holds for each door FormID before `teardown_archipelago` resets the
    // resource to its default a few lines down. A door this session never
    // touched keeps its authored value untouched.
    for (&door_form_id, &info) in &world.resource::<NavArchipelagoState>().door_lock_info {
        door_lock_info.insert(door_form_id, info);
    }

    teardown_archipelago(world);

    let graph = super::read_nav_graph(&path).map_err(|error| {
        warn!("nav graph read failed at {}: {error:#}", path.display());
        no_nav_graph_error()
    })?;
    // Issue #164: capture this cell's lowest prepared geometry Y so the fall
    // guard can derive its kill plane from real per-cell bounds rather than a
    // hard-coded world Y (see `fall_guard`'s module doc).
    world.resource_mut::<NavCellFallBounds>().min_y = Some(graph.bounds.min[1]);
    let mesh_inputs = super::mesh_inputs(&graph);
    let merge_inputs = super::merge_inputs(&graph);
    // Issue #155 feature 1: one archipelago-wide door FormID -> type index
    // mapping, computed once before any mesh's conversion (every mesh must
    // agree on the same door's type index -- see `door_type_indices`'s doc
    // comment).
    let door_type_indices = landmass_graph::door_type_indices(&mesh_inputs);
    // Issue #177: the blocking derived-association class takes its own
    // indices, allocated above every `door_type_indices` one.
    let closed_door_type_indices =
        landmass_graph::closed_door_type_indices(&mesh_inputs, &door_type_indices);
    let openable_blockers = landmass_graph::openable_blockers(&mesh_inputs);

    // Widened sample distances plus the clamped border-avoidance horizon --
    // see `archipelago_options`, which every build (runtime and test) shares.
    let archipelago_entity = world.spawn(Archipelago3d::new(archipelago_options())).id();
    apply_preferred_pathing_base_cost(
        world,
        archipelago_entity,
        &door_type_indices,
        &closed_door_type_indices,
    );

    let mut islands = Vec::new();
    for mesh in &mesh_inputs {
        let result = landmass_graph::build_navigation_mesh(
            mesh,
            &merge_inputs,
            &door_type_indices,
            &closed_door_type_indices,
        );
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
    let mut door_open = HashMap::new();

    // Same-cell cross-mesh merge links (issue #113 feature 2). Real FO3
    // meshes never share seam vertex positions, so landmass's native island
    // boundary linking cannot connect them (it needs coincident boundary
    // vertices); generated walk-through animation links across the matched
    // boundary edges are the path real data takes.
    //
    // Runtime collision-visibility validation (issue #154 real-data
    // acceptance correction) runs as its own pass first, in a scoped
    // borrow of the physics world that cannot overlap the `world` borrow
    // `spawn_link_pair` needs below -- see `validate_merge_link_collision`'s
    // doc comment for why prepare-side geometry alone is not enough. A
    // cell whose static collision has not finished building yet (rare in
    // practice: `tna spawn` is a manual console action issued after the
    // cell is already visibly loaded) skips validation rather than
    // dropping every merge link's connectivity for the session.
    let candidate_merge_descriptors =
        landmass_graph::merge_link_descriptors(&mesh_inputs, &merge_inputs);
    let physics_disabled = world.resource::<PhysicsDisabled>().0;
    let physics_ready = !physics_disabled
        && world
            .get_resource::<CellPhysicsReadiness>()
            .is_some_and(|readiness| readiness.static_collision_ready());
    let validated_merge_descriptors = if !physics_ready {
        candidate_merge_descriptors
    } else if let Some(physics_world) = world
        .get_non_send_mut::<BoxdddPhysicsContext>()
        .as_deref_mut()
        .and_then(BoxdddPhysicsContext::world_mut)
    {
        let mover = boxddd::Capsule::new(
            [0.0, -(AGENT_HEIGHT * 0.5 - AGENT_RADIUS), 0.0],
            [0.0, AGENT_HEIGHT * 0.5 - AGENT_RADIUS, 0.0],
            AGENT_RADIUS,
        );
        let collision_filter = player::player_collision_filter();
        let support_filter = player::stair_support_filter();
        let mut validated = Vec::with_capacity(candidate_merge_descriptors.len());
        for descriptor in candidate_merge_descriptors {
            let start = Vec3::from_array(descriptor.side_a.midpoint);
            let end = Vec3::from_array(descriptor.side_b.midpoint);
            match validate_merge_link_collision(
                physics_world,
                &mover,
                collision_filter,
                support_filter,
                start,
                end,
            ) {
                Ok(()) => validated.push(descriptor),
                Err(reason) => {
                    warn!(
                        "nav merge link mesh {:08x} triangle {} <-> mesh {:08x} triangle {}: dropped ({})",
                        descriptor.side_a.mesh_form_id,
                        descriptor.side_a.polygon_index,
                        descriptor.side_b.mesh_form_id,
                        descriptor.side_b.polygon_index,
                        reason.as_str(),
                    );
                }
            }
        }
        validated
    } else {
        candidate_merge_descriptors
    };

    // Issue #162 feature 1: each validated merge candidate gets its own
    // `landmass` animation-link kind (`landmass_graph::merge_link_kind`,
    // deterministic by position in this already-deterministic order), the
    // identity a per-agent quarantine (`PermittedAnimationLinks`) can
    // exclude without touching any other link -- see that function's doc
    // comment for why this achieves exact single-link granularity, unlike
    // door locking's polygon type-index scheme.
    let merge_link_kind_count = validated_merge_descriptors.len();
    for (index, descriptor) in validated_merge_descriptors.into_iter().enumerate() {
        let start = Vec3::from_array(descriptor.side_a.midpoint);
        let end = Vec3::from_array(descriptor.side_b.midpoint);
        // Issue #154 feature 3: real traversal-distance cost, floored well
        // above zero -- `AnimationLink3d::cost` must stay strictly positive
        // regardless of how tight a validated portal's overlap ended up.
        let cost = descriptor.distance.max(0.01);
        let kind = landmass_graph::merge_link_kind(index);
        for link_entity in spawn_link_pair(world, archipelago_entity, start, end, cost, kind) {
            link_kinds.insert(link_entity, LinkKind::Merge { kind });
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
        let usable = door_usable_now(world, None, descriptor.door_form_id, &door_lock_info);
        door_usable.insert(descriptor.door_form_id, usable);
        if usable {
            // Kind 0 (issue #162): every door link shares the reserved
            // "never quarantined" kind, so a blocked merge portal can never
            // make a door impassable for an agent that has nothing to do
            // with it.
            for link_entity in spawn_link_pair(world, archipelago_entity, start, end, 1.0, 0) {
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
            door_usable_now(world, None, door.door_form_id, &door_lock_info),
        );
        mid_route_doors.push(MidRouteDoor {
            door_form_id: door.door_form_id,
            vertices: door.vertices.map(Vec3::from_array),
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

    // Derived blocker crossing gates (issue #177). Ordinary in-cell doors
    // carry no authored `NVDP` triangle association at all, so none of the
    // loops above ever saw them; `prepare` now derives the association from
    // the door's own collision footprint and this feeds it into the exact
    // same `mid_route_doors` crossing-gate set an authored travel-door
    // triangle uses -- pause -> request open -> wait -> traverse -> resume.
    // A blocker's *interior* polygons are deliberately absent here: they are
    // priced impassable while it is closed instead (`closed_door_type_
    // indices`), so an agent is never asked to stop inside a solid slab.
    for gate in landmass_graph::derived_door_gates(&mesh_inputs) {
        door_usable
            .entry(gate.door_form_id)
            .or_insert_with(|| door_usable_now(world, None, gate.door_form_id, &door_lock_info));
        mid_route_doors.push(MidRouteDoor {
            door_form_id: gate.door_form_id,
            vertices: gate.vertices.map(Vec3::from_array),
        });
    }
    // Every blocker with a closed-state override needs an open-state entry,
    // whether or not it also has a crossing gate: `door_availability_system`
    // polls exactly the tracked set.
    for &blocker_form_id in closed_door_type_indices.keys() {
        door_usable
            .entry(blocker_form_id)
            .or_insert_with(|| door_usable_now(world, None, blocker_form_id, &door_lock_info));
    }
    for &door_form_id in door_usable.keys() {
        let (open, _) = door_open_and_locked(world, None, door_form_id, &door_lock_info);
        door_open.insert(door_form_id, open);
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
        door_type_indices,
        closed_door_type_indices,
        openable_blockers,
        door_open,
        merge_link_kind_count,
    };
    Ok(())
}

/// Issue #168: applies [`PREFERRED_PATHING_TYPE_INDEX_COST`] as the
/// archipelago-wide base cost for `landmass_graph::preferred_pathing_type_
/// index`'s type index, so authored preferred-pathing polygons (#156
/// feature 1) actually route cheaper instead of the type existing but never
/// being priced. Called once per archipelago build (`ensure_archipelago`,
/// right after the entity is spawned), not per-agent: this is a shared
/// terrain preference every agent in this archipelago gets, unlike door
/// locking (#155) or merge-portal quarantine (#162), which are per-agent
/// `AgentTypeIndexCostOverrides`/`PermittedAnimationLinks` exceptions to
/// this shared baseline. `set_type_index_cost` only errors on a
/// non-positive cost (`landmass` 0.9.2's `SetTypeIndexCostError::
/// NonPositiveCost`, see the constant's own doc comment for the
/// verification) -- unreachable for this fixed, positive, compile-time
/// constant, so the `Result` is discarded rather than propagated.
fn apply_preferred_pathing_base_cost(
    world: &mut World,
    archipelago_entity: Entity,
    door_type_indices: &BTreeMap<u32, usize>,
    closed_door_type_indices: &BTreeMap<u32, usize>,
) {
    let preferred_pathing_index =
        landmass_graph::preferred_pathing_type_index(door_type_indices, closed_door_type_indices);
    if let Ok(mut entity) = world.get_entity_mut(archipelago_entity)
        && let Some(mut archipelago) = entity.get_mut::<Archipelago3d>()
    {
        let _ = archipelago
            .set_type_index_cost(preferred_pathing_index, PREFERRED_PATHING_TYPE_INDEX_COST);
    }
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
///
/// `kind` (issue #162 feature 1): every door link passes the reserved `0`
/// (never quarantined); a merge link passes its own deterministic
/// `landmass_graph::merge_link_kind`, giving `PermittedAnimationLinks` a
/// per-link identity to exclude for one agent without touching any other
/// link -- both unidirectional links of one logical portal get the *same*
/// `kind`, so a quarantine excludes the whole crossing in either direction.
fn spawn_link_pair(
    world: &mut World,
    archipelago_entity: Entity,
    start: Vec3,
    end: Vec3,
    cost: f32,
    kind: usize,
) -> [Entity; 2] {
    let mut spawn_one = |from: Vec3, to: Vec3| {
        world
            .spawn(AnimationLink3dBundle {
                link: AnimationLink3d {
                    start_edge: (from, from),
                    end_edge: (to, to),
                    kind,
                    cost,
                    bidirectional: false,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            })
            .id()
    };
    [spawn_one(start, end), spawn_one(end, start)]
}

/// Why a merge portal candidate failed runtime collision-visibility
/// validation (issue #154 real-data acceptance correction). Reported once
/// per dropped link via a stable `warn!` line naming both sides' mesh/
/// triangle ids (`ensure_archipelago`).
#[derive(Debug, Clone, Copy)]
enum MergeLinkRejection {
    /// The capsule sweep from the near portal point to the far one did not
    /// reach the far point without first contacting something.
    SweptBlocked,
    /// No walkable ground support was found within step height below the
    /// crossing's midpoint or its far point.
    NoGroundSupport,
}

impl MergeLinkRejection {
    fn as_str(self) -> &'static str {
        match self {
            MergeLinkRejection::SweptBlocked => "swept blocked",
            MergeLinkRejection::NoGroundSupport => "no ground support",
        }
    }
}

/// Runtime collision-visibility validation for one merge portal candidate
/// (issue #154 real-data acceptance correction): prepare-side geometric
/// validation (opposing directions, an overlapping interval --
/// `vsa::prepare::nav_graph::validate_portal_candidate`) has no cooked
/// physics to check against, and real FranklinMetro02 data showed it can
/// accept a candidate that is a genuine seam in the abstract navmesh
/// topology but empty air (or blocked by intervening geometry) in the
/// actual level -- one accepted portal with a 1.69 m XZ gap swept a live
/// agent clean off the mesh edge into the void (`y` still falling at
/// -348 m when observed). This runs once per candidate link at
/// archipelago-build time (`ensure_archipelago`, where the cooked BoxDDD
/// collision world is already available), mirroring where issue #154's
/// step-height check already moved to for the identical "no cooked
/// physics prepare-side" reason.
///
/// Two checks, both using the same capsule/filters ordinary agent movement
/// uses (`step_agent_kcc`'s own `mover`/`collision_filter`/
/// `support_filter`, constructed identically by the caller):
/// 1. Ground support (`player::try_step_down` -- the same step-height-
///    bounded downward probe the KCC itself uses when stepping down) must
///    exist within step height below both the crossing's midpoint and
///    `end`. This is what actually catches the void-fall case: a capsule
///    swept purely horizontally never contacts a floor that simply is not
///    there underneath it.
/// 2. A capsule slide from `start` to `end` (`player::move_mover`, the
///    same move-and-slide collision response ordinary agent/player
///    movement runs every tick -- deliberately *not* a single raw
///    `boxddd::World::cast_mover`) must actually arrive within a small
///    tolerance. A raw single sweep starting exactly at `start` routinely
///    reports "blocked immediately" for an otherwise walkable seam: `start`
///    is an un-eroded seam boundary point (`erosion_policy`'s protected-
///    edge rule deliberately never pulls a merge-triangle vertex inward,
///    so both sides keep agreeing on the same seam position), which in
///    real FO3 data commonly sits flush against the near-side wall -- a
///    capsule centred exactly there already touches that wall at the very
///    first query. `move_mover`'s plane-based sliding is what real
///    per-tick movement already relies on to handle a capsule touching a
///    wall without misreporting the whole crossing as impassable; a raw
///    cast has no such contact tolerance.
///
/// `start`/`end` are feet-level points (the same convention every other
/// nav-graph point in this module uses -- see `TRAVEL_ARRIVAL_DISTANCE`'s
/// doc comment); both are raised by `AGENT_HEIGHT / 2` to the capsule-
/// centre height `step_agent_kcc`'s own `origin` convention expects before
/// either check runs.
fn validate_merge_link_collision(
    world: &mut boxddd::World,
    mover: &boxddd::Capsule,
    collision_filter: boxddd::QueryFilter,
    support_filter: boxddd::QueryFilter,
    start: Vec3,
    end: Vec3,
) -> Result<(), MergeLinkRejection> {
    let to_capsule_center = Vec3::new(0.0, AGENT_HEIGHT * 0.5, 0.0);
    let start_origin = start + to_capsule_center;
    let end_origin = end + to_capsule_center;
    let mid_origin = start_origin.lerp(end_origin, 0.5);

    for probe in [mid_origin, end_origin] {
        if player::try_step_down(
            world,
            player::to_box_vec3(probe),
            mover,
            boxddd::Vec3::ZERO,
            collision_filter,
            support_filter,
        )
        .is_none()
        {
            return Err(MergeLinkRejection::NoGroundSupport);
        }
    }

    let delta = player::to_box_vec3(end_origin - start_origin);
    let (achieved_box, ..) = player::move_mover(
        world,
        player::to_box_vec3(start_origin),
        mover,
        delta,
        collision_filter,
        support_filter,
        true,
        false,
    );
    let achieved = player::from_box_vec3(achieved_box);
    if (achieved - end_origin).length() > MERGE_LINK_SWEEP_TOLERANCE {
        return Err(MergeLinkRejection::SweptBlocked);
    }
    Ok(())
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

/// Whether `agent_entity` currently holds an item stack whose base FormID is
/// `form_id`, in its own canonical inventory (issue #185).
///
/// A bound actor's canonical holder is always `HolderId::Actor {
/// reference_form_id }` (`viewer::actor::project_prepared_actors`), so this
/// resolves straight from the agent entity's own `ActorRuntime` -- an
/// unbound `tna spawn` debug capsule (no `ActorRuntime` at all) has no
/// inventory of its own and therefore never holds any key, which is the
/// same conservative default `door_open_and_locked` fell back to pre-#185
/// when no inventory resource was available.
fn agent_holds_item(world: &World, agent_entity: Entity, form_id: u32) -> bool {
    let Some(actor) = world.get::<crate::viewer::actor::ActorRuntime>(agent_entity) else {
        return false;
    };
    let holder = bevyout_core::item_transaction::HolderId::Actor {
        reference_form_id: actor.reference_form_id,
    };
    world
        .get_resource::<interaction::CanonicalItemLedger>()
        .and_then(|ledger| ledger.ledger.holders().get(&holder))
        .is_some_and(|state| state.items.iter().any(|item| item.base_form_id == form_id))
}

/// The live `(open, locked)` observation for `door_form_id`, for the
/// specific `agent` asking (issue #185: locked-with-a-key is a fact about
/// the *pair* of door and actor, not the door alone -- mirroring OpenMW's
/// `AiPackage::openDoors()`, which searches the routing actor's own
/// inventory, never the player's). `open` reads the runtime
/// `InteractionState.open` set (guarded on `RefRegistry` being present --
/// `resolve_reference` panics without one, which minimal test worlds may
/// not have); `locked` runs `openmw_doors::door_openable` against the
/// door's prepared lock/key/trap data and whether `agent` (when given) holds
/// the key. `agent: None` is the conservative, actor-independent baseline
/// `ensure_archipelago`'s initial build and `door_availability_system`'s
/// change-detection poll use -- no specific actor to check a key against,
/// so a keyed lock is never lifted. A door with no prepared lock info is
/// never locked.
fn door_open_and_locked(
    world: &World,
    agent: Option<Entity>,
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
        let holder_has_key = info.key_form_id.is_some_and(|key_form_id| {
            agent.is_some_and(|agent| agent_holds_item(world, agent, key_form_id))
        });
        !openmw_doors::door_openable(openmw_doors::DoorAccessObservation {
            lock_level: info.lock_level,
            trapped: info.trapped,
            key_form_id: info.key_form_id,
            holder_has_key,
        })
    });
    (open, locked)
}

/// Whether `door_form_id` is currently usable for route planning, for the
/// specific `agent` asking (see [`door_open_and_locked`]): already open, or
/// not locked (`repath::door_usable`'s rule). A door with no prepared lock
/// info is usable.
fn door_usable_now(
    world: &World,
    agent: Option<Entity>,
    door_form_id: u32,
    door_lock_info: &HashMap<u32, DoorLockInfo>,
) -> bool {
    let (open, locked) = door_open_and_locked(world, agent, door_form_id, door_lock_info);
    repath::door_usable(repath::DoorObservation { locked, open })
}

/// Rebuilds `agent_entity`'s `AgentTypeIndexCostOverrides` wholesale from
/// `NavArchipelagoState::door_usable`/`door_type_indices` (issue #155
/// feature 2): every currently-unusable (locked, closed) door with a
/// resolved type index gets [`LOCKED_DOOR_TYPE_INDEX_COST`]; everything
/// else -- including a door that *was* locked and just became usable --
/// gets no entry. Replacing the whole component rather than patching it is
/// deliberate: `bevy_landmass::AgentTypeIndexCostOverrides` only exposes
/// `set_type_index_cost` (insert/overwrite) publicly, with no matching
/// "remove one override" call, so the only way to actually *clear* a
/// stale locked-door entry from outside `bevy_landmass` is to insert a
/// fresh component that never had it. `bevy_landmass`'s own sync system
/// only re-applies this component to the underlying `landmass::Agent` when
/// it is `Changed<_>` -- inserting a fresh value every call always
/// satisfies that, so this is safe to call unconditionally (at spawn) or
/// on every door-usability flip (`door_availability_system`) without
/// needing its own separate change-tracking.
///
/// Called both at spawn time (`spawn_test_agent`, so a freshly spawned
/// agent's very first path query already respects whatever is locked) and
/// on every `door_availability_system` flip (so an agent that is idle, or
/// already `NoPath`/`Unreachable`, picks up the change on its next solve --
/// `landmass`'s own `does_agent_need_repath` retries every tick whenever
/// `current_path` is `None`, with no explicit retarget needed for that
/// case). An agent already mid-transit through a door at the exact instant
/// it locks keeps following its already-computed, structurally still-valid
/// path until its next genuine repath -- see this file's module doc
/// comment for why that narrower case is a documented scope cut rather
/// than fixed here.
fn apply_door_lock_overrides(world: &mut World, agent_entity: Entity) {
    let mut overrides = AgentTypeIndexCostOverrides::default();
    // Issue #185: cloned out up front (rather than held as a live borrow of
    // `NavArchipelagoState` for the whole function) so the per-agent
    // re-checks below (`door_usable_now`, which needs `&World`) are free to
    // read `world` without fighting this borrow.
    // No archipelago means no prepared door graph (e.g. a minimal-`App` route
    // test, or a cell with no navmesh yet): there is nothing to override, so
    // leave the agent's costs untouched rather than panicking on a missing
    // resource.
    let Some((
        door_usable,
        door_type_indices,
        closed_door_type_indices,
        openable_blockers,
        door_open,
        door_lock_info,
    )) = world.get_resource::<NavArchipelagoState>().map(|state| {
        (
            state.door_usable.clone(),
            state.door_type_indices.clone(),
            state.closed_door_type_indices.clone(),
            state.openable_blockers.clone(),
            state.door_open.clone(),
            state.door_lock_info.clone(),
        )
    })
    else {
        return;
    };
    // Issue #185, the main gap: the shared cache above answers "usable by an
    // actor with no particular key" (see `door_open_and_locked`'s `agent:
    // None` case) -- but OpenMW's `AiPackage::openDoors()` tries the routing
    // actor's *own* inventory before giving up, so a door the shared cache
    // calls unusable may still be usable for THIS agent specifically. Only
    // worth re-checking when the shared default already says no; if it is
    // already usable for nobody-in-particular, it is usable for everyone.
    //
    // Deliberately consults `door_lock_info` directly (`openmw_doors::
    // door_openable`) rather than the more general `door_usable_now`/
    // `door_open_and_locked`: those treat "no prepared lock info at all" as
    // "never locked" (harmlessly true for a real `PreparedSemantic::Door`
    // with no `XLOC`, but this same `door_usable` cache is shared with the
    // #177 derived-blocker class below, e.g. a vault gear activator, which
    // is closed/open-gated with no lock or key concept and so *never* has a
    // `door_lock_info` entry). Re-deriving "usable" for those from an absent
    // entry would spuriously read as "openable" and lift an override that
    // has nothing to do with a key. A door with no lock info therefore
    // simply is not re-examined here at all -- the shared cache's `false`
    // stands, exactly as it did before this issue.
    let agent_may_open_with_key = |world: &World, door_form_id: u32| -> bool {
        let Some(&info) = door_lock_info.get(&door_form_id) else {
            return false;
        };
        let holder_has_key = info
            .key_form_id
            .is_some_and(|key_form_id| agent_holds_item(world, agent_entity, key_form_id));
        openmw_doors::door_openable(openmw_doors::DoorAccessObservation {
            lock_level: info.lock_level,
            trapped: info.trapped,
            key_form_id: info.key_form_id,
            holder_has_key,
        })
    };
    for (&door_form_id, &usable) in &door_usable {
        if usable || agent_may_open_with_key(world, door_form_id) {
            continue;
        }
        if let Some(&type_index) = door_type_indices.get(&door_form_id) {
            overrides.set_type_index_cost(type_index, LOCKED_DOOR_TYPE_INDEX_COST);
        }
    }
    // Issue #177: a blocker's *interior* polygons (derived associations that
    // lie wholly inside its collision volume) are never freely traversable
    // while it is closed -- pricing them only on `door_usable` (lock) is what
    // let an agent plan straight through a shut door and wedge against it.
    //
    // How expensive depends on whether the blocker can be opened at all,
    // which is the correction the first cut of this issue needed:
    //
    // - **Openable and unlocked** -> [`CLOSED_DOOR_TYPE_INDEX_COST`], a
    //   strong but finite penalty. The route stays plannable, the agent walks
    //   to the doorway, and the mid-route crossing gate runs the existing
    //   pause -> request open -> wait -> traverse -> resume lifecycle. An
    //   unbounded cost here would stop the agent ever reaching the door it is
    //   supposed to open.
    // - **Locked, or not openable at all** (the ungated kinematic-activator
    //   class, e.g. a vault gear door with no open/close FSM) ->
    //   [`LOCKED_DOOR_TYPE_INDEX_COST`]. There is no sanctioned crossing, so
    //   the route must fail fast rather than walk the agent into a solid.
    //
    // Opening the blocker clears the entry entirely, through the same
    // rebuild-the-whole-component path a lock change takes.
    for (&blocker_form_id, &type_index) in &closed_door_type_indices {
        if door_open.get(&blocker_form_id).copied().unwrap_or(false) {
            continue;
        }
        let openable = openable_blockers.contains(&blocker_form_id);
        // Issue #185: same per-agent key exception as the loop above --
        // `usable` here means "not locked", the identical fact
        // `agent_may_open_with_key` re-derives for this specific agent (a
        // no-op for a lock-less activator blocker, which has no
        // `door_lock_info` entry to re-check in the first place).
        let usable = door_usable.get(&blocker_form_id).copied().unwrap_or(true)
            || agent_may_open_with_key(world, blocker_form_id);
        let cost = if openable && usable {
            CLOSED_DOOR_TYPE_INDEX_COST
        } else {
            LOCKED_DOOR_TYPE_INDEX_COST
        };
        overrides.set_type_index_cost(type_index, cost);
    }
    if let Ok(mut entity) = world.get_entity_mut(agent_entity) {
        entity.insert(overrides);
    }
}

/// Issue #163 (`setlock`): the narrow external mutation point for a door's
/// prepared lock level, callable from `console::world_commands` without
/// exposing `NavArchipelagoState` itself. Inserts/replaces the door's
/// `door_lock_info` entry -- the exact shape `ensure_archipelago` populates
/// from the manifest above -- preserving whatever `key_form_id` was already
/// recorded (a runtime lock change never invents a new key requirement). A
/// missing resource (no archipelago built yet for this cell, or a console
/// harness without the nav plugin) is a no-op: there is no `door_usable`
/// entry to flip either in that case, and the interaction-side write in the
/// same console command is still the ultimate consistent state for anything
/// reading only `PlacementRoot`. Once the resource exists,
/// `door_availability_system`'s next poll (`door_usable_now` ->
/// `door_open_and_locked`) reads this updated map and treats a runtime lock
/// exactly like an authored one -- no separate repath plumbing needed.
pub(crate) fn set_door_lock_level(world: &mut World, door_form_id: u32, lock_level: Option<i8>) {
    let Some(mut state) = world.get_resource_mut::<NavArchipelagoState>() else {
        return;
    };
    let (key_form_id, trapped) = state
        .door_lock_info
        .get(&door_form_id)
        .map(|info| (info.key_form_id, info.trapped))
        .unwrap_or_default();
    state.door_lock_info.insert(
        door_form_id,
        DoorLockInfo {
            lock_level,
            key_form_id,
            trapped,
        },
    );
}

/// Issue #185: the nav-side mirror of [`set_door_lock_level`] for a door's
/// key requirement -- `console::world_commands::setlock`'s optional key
/// argument writes both the interaction-side `PlacementRoot` (the player's
/// own activation check) and this `door_lock_info` entry (nav route
/// planning/door-open requests) the same way a lock-level change already
/// does. Preserves whatever `lock_level`/`trapped` was already recorded.
pub(crate) fn set_door_key_form_id(world: &mut World, door_form_id: u32, key_form_id: Option<u32>) {
    let Some(mut state) = world.get_resource_mut::<NavArchipelagoState>() else {
        return;
    };
    let (lock_level, trapped) = state
        .door_lock_info
        .get(&door_form_id)
        .map(|info| (info.lock_level, info.trapped))
        .unwrap_or_default();
    state.door_lock_info.insert(
        door_form_id,
        DoorLockInfo {
            lock_level,
            key_form_id,
            trapped,
        },
    );
}

/// Test-only support for `console::world_commands`'s `setlock` tests, which
/// run in the lighter console harness (`test_app` in
/// `console::tests`) that never builds a real archipelago. Neither
/// `NavArchipelagoState` nor `DoorLockInfo` is nameable outside this module,
/// so a console test cannot construct or inspect them directly.
#[cfg(test)]
pub(crate) fn init_test_archipelago_state(world: &mut World) {
    world.init_resource::<NavArchipelagoState>();
}

/// Test-only companion to [`init_test_archipelago_state`]: the locked-level
/// currently recorded for `door_form_id`, or `None` if it is absent or
/// recorded unlocked -- either way, "not locked" for route planning.
#[cfg(test)]
pub(crate) fn door_lock_level_for_test(world: &World, door_form_id: u32) -> Option<i8> {
    world
        .get_resource::<NavArchipelagoState>()
        .and_then(|state| state.door_lock_info.get(&door_form_id))
        .and_then(|info| info.lock_level)
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
        "bind" => bind_agent(world, rest),
        "goto" => goto_agent(world, rest),
        "travel" => travel_agent(world, rest),
        "status" => agent_status(world, rest),
        "despawn" => despawn_agent(world, rest),
        "solverate" => solve_rate_command(world, rest),
        other => Err(ConsoleError::new(
            "unknown_subcommand",
            format!(
                "unknown tna subcommand '{other}'; expected spawn, bind, goto, travel, status, despawn, or solverate"
            ),
        )),
    }
}

fn usage_reply() -> ConsoleCommandResult {
    let usage = "usage: tna spawn [<index>]|bind [<index>] <actor-reference-formid>|goto [<index>] <x> <y> <z>|goto [<index>] player|travel [<index>] <door-formid>|status [<index>]|despawn [<index>]|solverate [<n>]";
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

/// Parses an agent index argument. Every `tna` subcommand that used to
/// address the single spike agent now takes this as an optional leading
/// token; omitting it defaults to agent 0 (issue #114 feature 4's
/// back-compat requirement). Issue #215 removed the four-slot cap; the only
/// remaining ceiling is the defensive dense-allocation bound.
fn parse_agent_index(value: &str) -> Result<usize, ConsoleError> {
    value
        .parse::<usize>()
        .ok()
        .filter(|index| *index <= MAX_AGENT_INDEX)
        .ok_or_else(|| {
            ConsoleError::new(
                "bad_agent_index",
                format!("agent index must be an integer 0..={MAX_AGENT_INDEX}"),
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
            Transform::from_translation(position),
            Visibility::Inherited,
            // Offset zero: a capsule entity's own transform already is the
            // capsule centre (issue #188 introduced the offset for bound
            // actors only, leaving this path bit-for-bit as it was).
            agent_components(archipelago_entity, 0.0),
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
    // Issue #155 feature 2: this agent's very first path query must already
    // respect whatever is locked in the active cell -- see
    // `apply_door_lock_overrides`'s doc comment.
    apply_door_lock_overrides(world, agent_entity);
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
    if world.resource::<TestNavAgentState>().is_occupied(index) {
        return Err(ConsoleError::new(
            "already_spawned",
            "a test nav agent is already spawned at this index; use tna despawn first",
        ));
    }
    let position = player_transform_query(world)
        .ok_or_else(|| ConsoleError::new("player_unavailable", "the FPS player does not exist"))?;
    let agent_entity = spawn_test_agent(world, position);
    world
        .resource_mut::<TestNavAgentState>()
        .set(index, Some(agent_entity));
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

/// The nav-agent component set, minus the marker and the debug capsule
/// mesh: everything an entity needs to be routed and physically moved.
/// Shared by `spawn_test_agent` (a fresh capsule) and `bind_agent` (an
/// already-live projected actor), so the two paths cannot drift into
/// disagreeing about what an agent is.
fn agent_components(
    archipelago_entity: Entity,
    capsule_centre_offset_y: f32,
) -> impl Bundle + use<> {
    (
        AgentRuntime::default(),
        AgentKcc {
            capsule_centre_offset_y,
            ..default()
        },
        AgentDesiredVelocityBlend::default(),
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
    )
}

/// Strips everything `agent_components` (plus the marker and target) added,
/// returning the entity to whatever it was before it owned an agent. Used
/// only to release a *bound actor* -- a `tna` capsule is despawned whole
/// instead, since the capsule exists for nothing else.
fn remove_agent_components(entity: &mut EntityWorldMut<'_>) {
    entity.remove::<(
        TestNavAgentMarker,
        AgentRuntime,
        AgentKcc,
        AgentDesiredVelocityBlend,
        Agent3dBundle,
        TargetReachedCondition,
        AgentTarget3d,
    )>();
    // Any traversal in flight belongs to the agent, not the actor.
    entity.remove::<(DoorTraversal, MergeTraversal, PendingMergeRepath)>();
    // Nav-owned per-agent AI-facing state (door policy + facing ownership)
    // belongs to the agent too: a released actor opens doors and steers its own
    // facing again.
    entity.remove::<(AgentRefusesDoors, actor_binding::FacingAuthority)>();
}

// ---------------------------------------------------------------------
// Bound-actor release contract (nav-owned; issues: package leak + layering)
// ---------------------------------------------------------------------

/// Nav-owned teardown hooks other slices register so
/// [`actor_binding::release_bound_actor`] can dismantle their per-actor state
/// (e.g. the AI slice's running package controller and any interaction point it
/// claimed) when nav releases a bound actor via the #164 fall guard or `tna
/// despawn`. Nav owns and invokes the contract; the AI slice satisfies it -- so
/// nav never imports a concrete AI type, and a released actor can no longer be
/// ticked by an AI system on an agent it no longer has. Empty (and free) until a
/// slice registers a hook.
/// A single registered bound-actor teardown callback.
type BoundActorReleaseHook = Box<dyn Fn(&mut World, Entity) + Send + Sync>;

#[derive(Resource, Default)]
pub(crate) struct BoundActorReleaseHooks(Vec<BoundActorReleaseHook>);

impl BoundActorReleaseHooks {
    fn push(&mut self, hook: impl Fn(&mut World, Entity) + Send + Sync + 'static) {
        self.0.push(Box::new(hook));
    }
}

/// Registers a teardown `hook` invoked (in registration order) by
/// [`actor_binding::release_bound_actor`] for the entity being released. The
/// nav-owned inverse of an AI slice reaching into nav's release path: the AI
/// plugin calls this on build (via `world_mut()`), so releasing a bound actor
/// tears its package controller and occupancy down without nav ever naming an
/// AI type. Takes `&mut World` (not `&mut App`) so tests can register the same
/// hook against a bare world.
pub(crate) fn register_bound_actor_release_hook(
    world: &mut World,
    hook: impl Fn(&mut World, Entity) + Send + Sync + 'static,
) {
    world
        .get_resource_or_insert_with(BoundActorReleaseHooks::default)
        .push(hook);
}

/// Runs every registered release hook against `entity`. Takes the resource out
/// of the world while invoking so each hook receives `&mut World` without
/// aliasing the resource; a no-op when no slice has registered a hook.
fn run_bound_actor_release_hooks(world: &mut World, entity: Entity) {
    let Some(hooks) = world.remove_resource::<BoundActorReleaseHooks>() else {
        return;
    };
    for hook in &hooks.0 {
        hook(world, entity);
    }
    world.insert_resource(hooks);
}

/// The bound-actor release entry point delegated to by the #164 fall guard and
/// `tna despawn`: strips the nav agent set and runs every registered teardown
/// hook. Re-exported at `pub(crate)` so the AI slice can exercise the full
/// release contract (including its own hook) in tests.
pub(crate) fn release_bound_actor(world: &mut World, entity: Entity) {
    actor_binding::release_bound_actor(world, entity);
}

/// Nav-owned marker set on a bound actor whose active AI package must not open
/// doors (Sandbox/Wander, #198 -- OpenMW's `AiWander` never calls
/// `openDoors()`). Nav reads only this marker at the door-open seam; the AI
/// slice sets it via [`set_agent_refuses_doors`], so nav no longer imports an
/// AI type to learn a package's door policy. Absent == opens doors normally
/// (every `tna`-driven agent and every door-opening family).
#[derive(Component, Default)]
pub(crate) struct AgentRefusesDoors;

/// Sets or clears the nav-owned [`AgentRefusesDoors`] marker on `entity`. The AI
/// package adapter calls this when it starts/stops a family, passing
/// `!family.opens_doors()`; nav never learns the family type itself.
pub(crate) fn set_agent_refuses_doors(world: &mut World, entity: Entity, refuses: bool) {
    let Ok(mut entity) = world.get_entity_mut(entity) else {
        return;
    };
    if refuses {
        entity.insert(AgentRefusesDoors);
    } else {
        entity.remove::<AgentRefusesDoors>();
    }
}

/// Sets a bound actor's facing authority (rotation double-writer contract). When
/// `pose_authored` is true, the nav-derived facing writer
/// ([`actor_binding::face_bound_actors`]) yields, so the AI adapter's authored
/// idle-marker yaw is the only rotation written that frame; false hands facing
/// back to navigation. The AI adapter sets it; nav reads only its own component
/// -- so the two rotation writers can no longer race across a Stop/arrival
/// transition (pose takes precedence over a still-decaying desired velocity).
pub(crate) fn set_facing_authority(world: &mut World, entity: Entity, pose_authored: bool) {
    let Ok(mut entity) = world.get_entity_mut(entity) else {
        return;
    };
    entity.insert(if pose_authored {
        actor_binding::FacingAuthority::PoseAuthored
    } else {
        actor_binding::FacingAuthority::NavDerived
    });
}

/// Core, index-free bind (issue #215's extraction for the autonomous
/// package driver, #218): gives an already-projected actor entity a nav
/// agent -- the same component set `bind_agent` (the `tna bind` console
/// wrapper) inserts, with none of the console index/`ConsoleCommandResult`
/// layer. Callers that do not need a debug index (the autonomous driver)
/// call this directly; `bind_agent` calls it too and only adds the roster
/// bookkeeping on top, so there is exactly one bind implementation.
pub(crate) fn bind_agent_entity(world: &mut World, entity: Entity) -> Result<(), ConsoleError> {
    ensure_archipelago(world)?;
    if world.get::<AgentKcc>(entity).is_some() {
        return Err(ConsoleError::new(
            "already_bound",
            "that actor already owns a nav agent",
        ));
    }
    let archipelago_entity = world
        .resource::<NavArchipelagoState>()
        .archipelago
        .expect("ensure_archipelago populated the archipelago");
    world.entity_mut(entity).insert((
        TestNavAgentMarker,
        actor_binding::NavBoundActor::default(),
        agent_components(
            archipelago_entity,
            actor_binding::BOUND_ACTOR_CAPSULE_OFFSET_Y,
        ),
    ));
    Ok(())
}

/// `tna bind [<index>] <actor-reference-formid>` (issue #188): gives an
/// already-projected actor a nav agent in roster slot `index`, so every
/// existing `tna goto`/`travel`/`status`/`despawn` form drives a real NPC
/// with a skeleton and locomotion clips instead of a debug capsule. The
/// capsule path is deliberately not migrated onto this -- it is the harness
/// every nav wave has relied on -- so both populations coexist in the same
/// roster. A thin wrapper over [`bind_agent_entity`]: resolves the actor
/// entity and records the debug index, nothing more.
fn bind_agent(world: &mut World, rest: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    let (index, form_id) = match rest {
        [form_id] => (0, form_id),
        [index, form_id] => (parse_agent_index(index)?, form_id),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna bind requires [<index>] <actor-reference-formid>",
            ));
        }
    };
    let reference_form_id = parse_form_id(form_id)
        .ok_or_else(|| ConsoleError::new("bad_type", "tna bind actor FormID must be hex"))?;
    if world.resource::<TestNavAgentState>().is_occupied(index) {
        return Err(ConsoleError::new(
            "already_spawned",
            "a nav agent already occupies this index; use tna despawn first",
        ));
    }
    let entity =
        actor_binding::actor_entity_by_reference(world, reference_form_id).ok_or_else(|| {
            ConsoleError::new(
                "no_actor",
                format!("no projected actor with reference FormID {reference_form_id:08x}"),
            )
        })?;
    bind_agent_entity(world, entity)?;
    world
        .resource_mut::<TestNavAgentState>()
        .set(index, Some(entity));
    let position = world
        .get::<Transform>(entity)
        .map_or(Vec3::ZERO, |transform| transform.translation);
    info!(
        "nav agent {index} bound actor {reference_form_id:08x} position=({:.2},{:.2},{:.2})",
        position.x, position.y, position.z
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "index": index,
            "reference_form_id": reference_form_id,
            "position": [position.x, position.y, position.z],
        }),
        vec![format!(
            "nav agent {index} bound to actor {reference_form_id:08x} at ({:.2}, {:.2}, {:.2})",
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
    let Some(agent_entity) = world.resource::<TestNavAgentState>().get(index) else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned at this index; use tna spawn first",
        ));
    };
    route_agent_to_target(world, agent_entity, target);
    Ok(ConsoleCommandResult::new(
        json!({ "index": index, "target": description }),
        vec![format!("nav agent {index} target set to {description}")],
    ))
}

/// Sets `target` on `agent_entity` and resets the per-route bookkeeping: the
/// merge-portal quarantine (issue #162), the path-latency timer, and the pure
/// stuck-tracking window. The single routing seam every caller that hands an
/// agent a new destination goes through -- `goto_agent` (the `tna goto`
/// console command) and the AI package families (`ai::family_runtime`,
/// #196/#197) both call this so neither can drift into a different notion of
/// "a fresh route intent".
pub(crate) fn route_agent_to_target(
    world: &mut World,
    agent_entity: Entity,
    target: AgentTarget3d,
) {
    world.entity_mut(agent_entity).insert(target);
    // Issue #162 feature 2: a fresh target is a new routing intent -- any
    // merge-portal quarantine from a previous route no longer applies.
    clear_merge_link_quarantine(world, agent_entity);
    // Issue #185: a fresh target is also a fresh chance for this agent's
    // key-aware door-lock overrides to reflect whatever the agent is
    // holding right now (e.g. a key granted since the last route) rather
    // than whatever was last computed at spawn or the last door flip.
    apply_door_lock_overrides(world, agent_entity);
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
}

/// Routes `agent_entity` to a fixed world point (the AI package families'
/// `FamilyRequest::Route`). Thin wrapper over [`route_agent_to_target`].
pub(crate) fn route_agent_to_point(world: &mut World, agent_entity: Entity, point: Vec3) {
    route_agent_to_target(world, agent_entity, AgentTarget3d::Point(point));
}

/// Clears any nav route on `agent_entity` (the families' `FamilyRequest::Stop`
/// on travel arrival / package completion), so landmass stops steering it and
/// the locomotion policy idles the now-stationary actor.
pub(crate) fn clear_agent_target(world: &mut World, agent_entity: Entity) {
    if let Ok(mut entity) = world.get_entity_mut(agent_entity) {
        entity.insert(AgentTarget3d::None);
    }
}

/// Whether `entity` currently owns a nav agent (`tna bind`/`tna spawn`
/// inserted the `AgentKcc`). The AI package families route only nav-bound
/// actors; the `runpackage` console command checks this before starting one.
pub(crate) fn is_nav_bound(world: &World, entity: Entity) -> bool {
    world.get::<AgentKcc>(entity).is_some()
}

/// Whether `agent_entity` has reached its current route target -- landmass's
/// own authoritative arrival latch, the families' `nav_reached` input.
pub(crate) fn agent_reached_target(world: &World, agent_entity: Entity) -> bool {
    matches!(
        world.get::<AgentState>(agent_entity),
        Some(AgentState::ReachedTarget)
    )
}

/// Whether `agent_entity`'s current route cannot be pathed (no path, or the
/// agent/target is off the nav mesh) -- the families' `route_failed` input.
pub(crate) fn agent_route_failed(world: &World, agent_entity: Entity) -> bool {
    matches!(
        world.get::<AgentState>(agent_entity),
        Some(AgentState::NoPath | AgentState::AgentNotOnNavMesh | AgentState::TargetNotOnNavMesh)
    )
}

/// The FormID of a door `agent_entity`'s route has given up on -- #185's
/// `DoorLinkState::Failed` terminal, which names the blocking door after the
/// deterministic open-wait bound expires. The AI follow family (#198) reads
/// this to distinguish "blocked by a door it cannot open" (name it, abandon)
/// from a plain no-path. `None` while the agent is not stuck at a failed door.
pub(crate) fn agent_blocking_door(world: &World, agent_entity: Entity) -> Option<u32> {
    match world.get::<AgentRuntime>(agent_entity)?.door_link {
        door_link::DoorLinkState::Failed { door_form_id } => Some(door_form_id),
        _ => None,
    }
}

/// Test-only: seeds the empty [`NavArchipelagoState`] a minimal-`World` needs
/// before [`route_agent_to_target`] (its `apply_door_lock_overrides` reads it).
/// Lets the AI-family adapter tests exercise the real routing seam without
/// standing up the whole `NavBackendPlugin`.
#[cfg(test)]
pub(crate) fn insert_test_archipelago_state(world: &mut World) {
    world.init_resource::<NavArchipelagoState>();
}

/// Test-only cross-module support (issue #218's autonomous package driver
/// tests): marks a `NavArchipelagoState` already current for `cell_form_id`
/// with `archipelago` as its entity, so [`ensure_archipelago`]'s
/// already-current check short-circuits without a real nav-graph file --
/// exactly [`bind_agent_entity`]'s own test harness, exposed for other
/// modules' tests since neither `NavArchipelagoState` nor its fields are
/// nameable outside this module.
#[cfg(test)]
pub(crate) fn mark_test_archipelago_current(
    world: &mut World,
    cell_form_id: u32,
    archipelago: Entity,
) {
    let mut state = world.get_resource_or_insert_with(NavArchipelagoState::default);
    state.cell_form_id = Some(cell_form_id);
    state.archipelago = Some(archipelago);
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
    let Some(agent_entity) = world.resource::<TestNavAgentState>().get(index) else {
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
    // Issue #155 feature 4: a door lifecycle that gave up waiting
    // (`MAX_WAIT_TICKS` exhausted, whether from a locked mid-route crossing
    // or a two-sided link) is a distinct, terminal failure -- not the same
    // "temporarily waiting" status a `Paused` agent still recovering from is
    // in. `Unreachable` (not a brand-new variant) is deliberate: it is the
    // exact word both call sites' `nav agent unreachable` log line already
    // uses (AGENTS.md: stable log wording), so `tna status`/the HUD finally
    // agree with the log instead of contradicting it.
    if door_link::is_failed(door_link_state) {
        return landmass_graph::NavAgentStatus::Unreachable;
    }
    if door_link::is_paused(door_link_state) {
        return landmass_graph::NavAgentStatus::Paused;
    }
    landmass_graph::map_agent_state(landmass_state)
}

// ponytail: fixed cap, no console knob. The `tdi` HUD shares a screen corner
// with the rest of the debug block, and a real cell holds dozens of
// autonomously-bound actors -- printing them all would bury the player/cell
// lines above it. Eight lines plus a `+N more` tail is enough to see that the
// population is being driven at all; `tna status <index>` remains the way to
// interrogate one specific agent. Raise the constant if that ever stops being
// true rather than adding a setting for it.
const HUD_AGENT_LINE_LIMIT: usize = 8;

/// Issue #151: one deterministic line per live nav agent for the console
/// debug-info HUD, reusing the exact same status/grounded/stuck/blocked
/// fields `tna status` (`agent_status` above) reports.
///
/// Issue #241: driven off the agent component set rather than the console
/// roster, so an autonomously-bound actor is visible here too -- which also
/// means it needs `&mut World` (a query, not a resource read) and a
/// [`HUD_AGENT_LINE_LIMIT`] cap, because a real cell can hold dozens of them.
pub(crate) fn hud_agent_status_lines(world: &mut World) -> Vec<String> {
    if world.get_resource::<TestNavAgentState>().is_none() {
        return Vec::new();
    }
    let mut agents = all_agent_entities(world);
    // Console-addressed agents first, in index order (what the operator is
    // usually debugging, and the order this HUD block has always printed);
    // every other bound actor follows in entity order. The cap therefore
    // truncates the anonymous tail, not the agent someone just spawned.
    agents.sort_by_key(|&entity| {
        (
            world
                .resource::<TestNavAgentState>()
                .index_of(entity)
                .unwrap_or(usize::MAX),
            entity.index_u32(),
        )
    });
    let hidden = agents.len().saturating_sub(HUD_AGENT_LINE_LIMIT);
    let mut lines: Vec<String> = agents
        .into_iter()
        .take(HUD_AGENT_LINE_LIMIT)
        .map(|entity| {
            let id = agent_log_id(world, entity);
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
            format!(
                "nav agent {id} status={} position=({:.2},{:.2},{:.2}) grounded={grounded} stuck={stuck} blocked={collision_blocked}",
                status.as_str(),
                position.x,
                position.y,
                position.z,
            )
        })
        .collect();
    if hidden > 0 {
        lines.push(format!("nav agent +{hidden} more"));
    }
    lines
}

/// The `link=` suffix for `tna status` (issue #113 feature 5): the active
/// link kind while interacting with one (`merge` while crossing a merge
/// seam, `door <formid>` through a door lifecycle), else `None`.
fn active_link_description(runtime: &AgentRuntime) -> Option<String> {
    match runtime.active_link {
        Some(LinkKind::Merge { .. }) => Some("merge".to_string()),
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
    let Some(agent_entity) = world.resource::<TestNavAgentState>().get(index) else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned at this index; use tna spawn first",
        ));
    };
    // Issue #188: a bound actor is *released*, never despawned. The capsule
    // exists only to be an agent, so despawning it is right; a projected NPC
    // exists independently of navigation and the world/actor slice owns its
    // lifetime.
    let bound = world
        .get::<actor_binding::NavBoundActor>(agent_entity)
        .is_some();
    if bound {
        release_bound_actor(world, agent_entity);
    } else if let Ok(entity) = world.get_entity_mut(agent_entity) {
        entity.despawn();
    }
    world.resource_mut::<TestNavAgentState>().set(index, None);
    let verb = if bound { "released" } else { "despawned" };
    Ok(ConsoleCommandResult::new(
        json!({ "index": index, "despawned": true, "bound_actor": bound }),
        vec![format!("nav agent {index} {verb}")],
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

/// Issue #148 diagnostic: summarises what a blocked agent's capsule is
/// actually touching, as a single stable log field. Emits every
/// non-walkable contact plane (normal + world contact point) plus the
/// fraction of this tick's desired motion the sweep could achieve, so a
/// wedge can be attributed to a real surface instead of guessed at from a
/// scene-manifest footprint scan. Footprint scans keyed on the capsule
/// *centre* miss the blocker by exactly `AGENT_RADIUS`, which is how
/// #148's metro wedge stayed misattributed across four builds.
fn world_contact_report(
    world: &mut boxddd::World,
    mover: &boxddd::Capsule,
    position: Vec3,
    collision_filter: boxddd::QueryFilter,
    desired_horizontal: Vec2,
) -> String {
    let origin = player::to_box_vec3(position);
    let planes = world
        .collide_mover(origin, mover, collision_filter)
        .unwrap_or_default();
    let mut blocking = planes
        .iter()
        .filter(|plane| plane.plane.normal.y < WALKABLE_CONTACT_NORMAL_Y)
        .map(|plane| {
            format!(
                "n=({:.2},{:.2},{:.2})@({:.2},{:.2},{:.2})",
                plane.plane.normal.x,
                plane.plane.normal.y,
                plane.plane.normal.z,
                plane.point.x,
                plane.point.y,
                plane.point.z,
            )
        })
        .collect::<Vec<_>>();
    blocking.sort();
    blocking.dedup();
    let step = desired_horizontal / 60.0;
    let fraction = world
        .cast_mover(
            origin,
            mover,
            boxddd::Vec3::new(step.x, 0.0, step.y),
            collision_filter,
        )
        .unwrap_or(1.0);
    // Issue #177: the reason is now stated, not left to be inferred. A
    // shortfall in *achieved* speed is what latches `collision_blocked`
    // (`movement_policy::decide_collision_outcome` compares desired against
    // achieved, and never consults contact geometry), so the flag fires just
    // as readily for an agent whose steering produced no motion as for one
    // wedged against a wall. Reporting both cases as "collision-blocked" sent
    // acceptance chasing colliders that were not there.
    let reason = if blocking.is_empty() && fraction >= 0.999 {
        "no_contact_no_progress"
    } else {
        "obstructed"
    };
    format!(
        "reason={reason} sweep_fraction={fraction:.3} blocking_planes=[{}]",
        blocking.join(" ")
    )
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
    // Issue #241: the diagnostics below identify a non-roster agent by its
    // bound actor's reference FormID, so they need the actor identity
    // alongside the roster. Read-only and disjoint from `agents`.
    actor_identities: Query<&ActorRuntime>,
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
    // Issue #241: every agent gets telemetry, identified by roster index when
    // it has one and by bound-actor FormID otherwise -- the diagnostics used
    // to be gated on `roster.index_of(entity)` and therefore never fired for
    // an autonomously-bound actor, the exact population #148 is about.
    let agent_id = |entity: Entity| {
        format_agent_id(
            roster.index_of(entity),
            actor_identities
                .get(entity)
                .ok()
                .map(|actor| actor.reference_form_id),
            entity,
        )
    };

    for (entity, mut transform, mut velocity, blend, mut kcc, target, agent_state) in &mut agents {
        let desired_velocity = blend.previous.lerp(blend.latest, solve_blend_fraction);
        let desired_horizontal = Vec2::new(desired_velocity.x, desired_velocity.z);
        // Issue #188: one capsule-centre convention reaches the KCC whatever
        // the entity transform means -- zero for `tna` capsules, half a
        // capsule height for a feet-level bound actor root. See
        // `AgentKcc::capsule_centre_offset_y`.
        let centre_offset = Vec3::new(0.0, kcc.capsule_centre_offset_y, 0.0);
        let centre = transform.translation + centre_offset;
        let (new_centre, new_kcc_velocity, grounded) = step_agent_kcc(
            world,
            &mover,
            collision_filter,
            support_filter,
            centre,
            kcc.velocity,
            kcc.grounded,
            desired_horizontal,
            dt,
        );
        let new_position = new_centre - centre_offset;
        let achieved = (new_centre - centre) / dt;
        transform.translation = new_position;
        kcc.velocity = new_kcc_velocity;
        kcc.grounded = grounded;
        velocity.velocity = achieved;
        // Issue #188: hand the locomotion consumer the very pair this
        // system just computed, rather than letting it derive a second one.
        kcc.last_desired_horizontal = desired_horizontal;
        kcc.last_achieved_horizontal = Vec2::new(achieved.x, achieved.z);

        let outcome =
            movement_policy::decide_collision_outcome(movement_policy::VelocityObservation {
                desired_horizontal_speed: desired_horizontal.length(),
                achieved_horizontal_speed: Vec2::new(achieved.x, achieved.z).length(),
            });
        let was_blocked = kcc.collision_blocked;
        kcc.collision_blocked = matches!(outcome, movement_policy::CollisionOutcome::Blocked);
        if kcc.collision_blocked && !was_blocked {
            let id = agent_id(entity);
            info!("nav agent collision-blocked {id}");
            // Issue #148: "blocked" alone never said *what* blocked, which
            // sent two waves chasing the wrong collider. Report the
            // obstructing contact geometry on the rising edge: the
            // non-walkable contact normals and world points locate the
            // offending surface directly (a wall face at
            // `point +/- AGENT_RADIUS` along its normal). Rising edge only,
            // so a permanently wedged agent logs this once, not per tick.
            // `new_centre`, not `transform.translation`: the contact probe
            // must query the capsule where the capsule actually is (issue
            // #188 added the feet-level bound-actor convention).
            let contacts = world_contact_report(
                world,
                &mover,
                new_centre,
                collision_filter,
                desired_horizontal,
            );
            info!("nav agent collision-blocked {id} contacts {contacts}");
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
                info!("nav agent stuck-recovery {}", agent_id(entity));
            }
            movement_policy::StuckDecision::RecoveryPending => {}
            movement_policy::StuckDecision::Stuck => {
                let was_stuck = kcc.stuck;
                kcc.stuck = true;
                if !was_stuck {
                    info!("nav agent stuck {}", agent_id(entity));
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
                Some(LinkKind::Merge { .. }) | None => {
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
                    // Issue #241: the intercell ledger is keyed by console
                    // index (`agent_ledger_id`), so only a roster agent can
                    // actually be handed off to another cell -- an
                    // autonomously-bound actor belongs to the AI/actor slice,
                    // which respawns it with its own cell. It must still
                    // *finish* the lifecycle rather than being skipped: the
                    // old `continue` here left it latched in `Traversing`
                    // forever with its travel intent intact, i.e. a
                    // permanently wedged actor -- exactly the failure mode
                    // this issue is about. Fall through to the same "left at
                    // the travel door" terminal the missing-metadata branch
                    // below already used.
                    let ledger_target = roster.index_of(entity).and_then(|index| {
                        archipelago_state
                            .travel_doors
                            .get(&door_form_id)
                            .map(|link| {
                                (agent_ledger_id(index), index, link.destination_door_form_id)
                            })
                    });
                    // Issue #113's terminal travel seam: the agent stopped at
                    // the traversed door. Issue #134 owns what happens next:
                    // the agent leaves the active cell entirely, ledgered for
                    // the destination cell at that door's own paired marker.
                    info!(
                        "nav agent travel reached {door_form_id:08x} -> cell {destination_cell_form_id:08x}"
                    );
                    match ledger_target {
                        Some((agent_id, index, destination_door_form_id)) => {
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
                            roster.set(index, None);
                        }
                        None => {
                            // Either no destination-door metadata (defensive:
                            // `travel_doors` always carries it once
                            // `TravelReached` fired through it) or a
                            // non-roster, autonomously-bound actor that has
                            // no ledger identity -- both end the lifecycle
                            // here, standing at the door, rather than losing
                            // or wedging the agent.
                            warn!(
                                "nav agent handoff {door_form_id:08x}: not ledgered; agent left at the travel door"
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
        Option<&'static AgentTarget3d>,
    ),
    With<TestNavAgentMarker>,
>;

/// Rebuilds an agent's `PermittedAnimationLinks` from its current
/// `AgentRuntime::quarantined_merge_link_kinds` (issue #162): thin wrapper
/// over the pure `landmass_graph::permitted_animation_link_kinds` that
/// converts its `Option<BTreeSet<usize>>` into the actual `bevy_landmass`
/// component -- `None` (nothing quarantined) becomes the cheap `All`
/// default rather than materializing an equivalent full allow-list.
fn permitted_animation_links_for(
    quarantined: &BTreeSet<usize>,
    merge_link_kind_count: usize,
) -> PermittedAnimationLinks {
    match landmass_graph::permitted_animation_link_kinds(quarantined, merge_link_kind_count) {
        None => PermittedAnimationLinks::All,
        Some(kinds) => PermittedAnimationLinks::Kinds(Arc::new(kinds.into_iter().collect())),
    }
}

/// Issue #162 feature 2: resets `agent_entity`'s merge-portal quarantine to
/// empty, called whenever the agent gets a genuinely new destination
/// (`goto_agent`/`request_travel`) -- a fresh `tna goto`/`tna travel` is a
/// new routing intent, so whatever previously blocked links this agent
/// steered around no longer apply to it. Despawn/hand-off need no
/// equivalent call: `AgentRuntime`/`PermittedAnimationLinks` are ordinary
/// components on the agent entity, gone the moment it despawns.
fn clear_merge_link_quarantine(world: &mut World, agent_entity: Entity) {
    if let Some(mut runtime) = world.get_mut::<AgentRuntime>(agent_entity) {
        runtime.quarantined_merge_link_kinds.clear();
    }
    if let Ok(mut entity) = world.get_entity_mut(agent_entity) {
        entity.insert(PermittedAnimationLinks::All);
    }
}

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
    archipelago_state: Res<NavArchipelagoState>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    mut agents: MergeTraversalQuery<'_, '_>,
    // Issue #241: same identity sources as `apply_agent_physics_movement`'s
    // telemetry, so a non-roster agent's quarantine is reported too.
    actor_identities: Query<&ActorRuntime>,
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

    for (entity, mut transform, mut kcc, mut traversal, mut runtime, current_target) in &mut agents
    {
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

            // Issue #162: per-agent portal quarantine, not a wholesale
            // route clear. Excludes just this link's kind
            // (`landmass_graph::merge_link_kind`, assigned per validated
            // merge candidate at archipelago build) from this agent's own
            // `PermittedAnimationLinks` -- kind 0 (every door link) is
            // never touched, so a blocked merge seam can never lock an
            // unrelated door. The real destination is kept: `AgentTarget3d`
            // is blanked for exactly one tick (`PendingMergeRepath`,
            // `resume_pending_merge_repath_system`) to force landmass's own
            // solver to discard the now-stale corridor (which still points
            // through the blocked link) and search again with the updated
            // exclusion, instead of resuming the identical path. An absent
            // or already-`None` target has nothing worth restoring, so it
            // is left as-is.
            runtime
                .quarantined_merge_link_kinds
                .insert(traversal.link_kind);
            let permitted = permitted_animation_links_for(
                &runtime.quarantined_merge_link_kinds,
                archipelago_state.merge_link_kind_count,
            );
            commands.entity(entity).insert(permitted);
            if let Some(snapshot) = current_target.and_then(AgentTargetSnapshot::capture) {
                commands
                    .entity(entity)
                    .insert(AgentTarget3d::None)
                    .insert(PendingMergeRepath { target: snapshot });
            }
            if !was_reported {
                let id = format_agent_id(
                    roster.index_of(entity),
                    actor_identities
                        .get(entity)
                        .ok()
                        .map(|actor| actor.reference_form_id),
                    entity,
                );
                warn!(
                    "nav agent portal blocked: swept crossing did not reach the far side within {:.1}s",
                    traversal.timeout
                );
                info!(
                    "nav agent portal quarantined {id} link={}",
                    traversal.link_kind
                );
                info!("nav agent collision-blocked {id}");
                info!("nav agent stuck {id}");
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

/// Issue #162: restores the real `AgentTarget3d` one fixed tick after
/// `merge_traversal_system`'s timeout branch deliberately blanked it to
/// `AgentTarget3d::None`.
///
/// Why the blank tick is needed at all: `landmass`'s own repath decision
/// (`landmass::agent::does_agent_need_repath`) only recomputes a path when
/// either the target transitions from absent to present, or the existing
/// corridor is structurally invalidated (an island/link actually
/// added/removed from the graph). Merely swapping this agent's
/// `PermittedAnimationLinks` does neither -- the just-failed portal step
/// was already behind the corridor's tracked progress, so the *existing*
/// path would simply be resumed unchanged and the agent would walk straight
/// back into the same blocked link. Blanking the target for exactly one
/// tick forces `RepathResult::ClearPathNoTarget` that tick (observed by
/// `LandmassSystems::Update` in `FixedPreUpdate`, which this system runs
/// `.after`); restoring it here lets the *next* tick's `Update` see
/// `current_path: None` plus a real target again, which is
/// `does_agent_need_repath`'s unconditional `NeedsRepath` case -- a genuine
/// fresh solve that honours the just-updated quarantine.
///
/// Skips the restore (but still removes the marker) when `AgentTarget3d` is
/// no longer `None`: a `tna goto`/`tna travel` issued during the one-tick
/// gap already retargeted the agent (and, via `clear_merge_link_
/// quarantine`, reset its quarantine too), so there is nothing stale left
/// to restore -- overwriting the fresh target with the stale captured one
/// would silently discard that newer command.
fn resume_pending_merge_repath_system(
    mut commands: Commands,
    agents: Query<(Entity, &PendingMergeRepath, Option<&AgentTarget3d>)>,
) {
    for (entity, pending, current_target) in &agents {
        commands.entity(entity).remove::<PendingMergeRepath>();
        if matches!(current_target, Some(AgentTarget3d::None) | None) {
            commands
                .entity(entity)
                .insert(pending.target.to_agent_target());
        }
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
/// `activate` console command uses, on behalf of `agent_entity` (issue
/// #185: usability is now a per-agent fact -- see `door_open_and_locked`).
/// A door currently unusable *for this agent* is deliberately *not*
/// scripted open (issue #113 feature 3: no teleporting through closed
/// doors, and a locked door resolves to the deterministic `Failed` outcome
/// via the wait bound) -- `scripted_door_open` bypasses locks by design
/// (dev tooling), so the lock gate lives here.
fn request_door_open(world: &mut World, agent_entity: Entity, door_form_id: u32) {
    // OpenMW parity (`AiWander::execute` never calls `openDoors()`,
    // `if getTypeId() == TypeIdWander return;`): a wander/sandbox package does
    // not open doors. #185 deliberately left this gate for when the package
    // family became available (#198). A sandboxing actor treats a closed door
    // as a wall and roams elsewhere. This is the minimal hook -- gating the one
    // door-open request site rather than threading the package type through the
    // whole door-link state machine; the pause/wait cycle still runs and
    // terminates at the documented `Failed` bound, it just never opens the door.
    if agent_family_refuses_doors(world, agent_entity) {
        info!("nav agent door {door_form_id:08x}: wander package does not open doors");
        return;
    }
    let lock_info = world
        .resource::<NavArchipelagoState>()
        .door_lock_info
        .clone();
    if !door_usable_now(world, Some(agent_entity), door_form_id, &lock_info) {
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

/// Whether the active AI package on `agent_entity` refuses to open doors --
/// true only when the AI slice has flagged this agent with [`AgentRefusesDoors`]
/// (its Sandbox/Wander family; see `PackageFamily::opens_doors`). Reads a
/// nav-owned marker the AI slice sets, not an AI type -- so nav does not depend
/// on `ai`. Actors with no running package (the common `tna`-driven agent) are
/// unflagged and open doors as before.
fn agent_family_refuses_doors(world: &World, agent_entity: Entity) -> bool {
    world.get::<AgentRefusesDoors>(agent_entity).is_some()
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
    // Issue #241: driven off the agent component set, not the console roster
    // -- an autonomously-bound actor whose route crosses a door used to get
    // no door lifecycle at all (no pause, no open request, no traversal, and
    // no travel arrival), which is a mid-route wedge with zero diagnostics.
    for agent_entity in all_agent_entities(world) {
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
                // Issue #165: consult the same lock/open source of truth the
                // mid-route crossing gate (`crossing_gate`, just below) and
                // `request_door_open`'s own internal `door_usable_now` check
                // already use, instead of relying solely on the latter's
                // implicit refusal -- the decision is now explicit and
                // symmetric with the mid-route gate rather than something a
                // reader has to trust `request_door_open` enforces
                // correctly. A locked door still routes through the normal
                // `Paused` -> `MAX_WAIT_TICKS` -> `Failed` terminal (`gate`
                // only changes whether `request_door_open` is even worth
                // calling this tick, mirroring the mid-route gate's own
                // `gate != Pass` guard), never an immediate short-circuit --
                // see `door_link::MAX_WAIT_TICKS`'s doc comment for why that
                // deterministic wait bound is the contract, not a fast
                // fail.
                let lock_info = world
                    .resource::<NavArchipelagoState>()
                    .door_lock_info
                    .clone();
                let (door_open, door_locked) =
                    door_open_and_locked(world, Some(agent_entity), door_form_id, &lock_info);
                let gate = door_link::crossing_gate(door_link::CrossingObservation {
                    door_open,
                    door_locked,
                });
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
                if gate != door_link::CrossingGate::Pass {
                    request_door_open(world, agent_entity, door_form_id);
                }
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
            // Issue #155 feature 3: corridor-based containment
            // (`landmass_graph::point_in_door_triangle` against the door's
            // own un-eroded triangle vertices), not the earlier
            // `MID_ROUTE_DOOR_GATE_DISTANCE` centroid-proximity scan -- a
            // route that merely passes near a doorway without its corridor
            // ever crossing it must not gate.
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
                                && landmass_graph::point_in_door_triangle(
                                    position.to_array(),
                                    door.vertices.map(|vertex| vertex.to_array()),
                                    AGENT_HEIGHT,
                                )
                        })
                        .copied()
                });
            // Issue #177: containment is a trigger that can be starved.
            // Real data (Vault 101, `VDoor01`) showed an agent routed at a
            // closed in-cell door halting ~2 m short of its crossing with a
            // completely free collision sweep -- never entering the polygon,
            // so never gating, never requesting the open, and never
            // continuing. When the agent has stopped making progress, fall
            // back to the nearest crossing its own route continues through
            // (`door_link::approach_gate`), which is the door it is stalled
            // against. Only consulted when containment found nothing, so the
            // normal case keeps issue #155's exact corridor semantics.
            let mid_route_crossing = mid_route_crossing.or_else(|| {
                let stalled = world
                    .get::<AgentKcc>(agent_entity)
                    .is_some_and(|kcc| kcc.collision_blocked || kcc.stuck);
                if !stalled || !has_target {
                    return None;
                }
                let position = world.get::<Transform>(agent_entity)?.translation;
                let target = match world.get::<AgentTarget3d>(agent_entity)? {
                    AgentTarget3d::Point(point) => *point,
                    AgentTarget3d::Entity(entity) => {
                        world.get::<GlobalTransform>(*entity)?.translation()
                    }
                    AgentTarget3d::None => return None,
                };
                let flat = |a: Vec3, b: Vec3| Vec2::new(a.x - b.x, a.z - b.z).length();
                let agent_distance_to_target = flat(position, target);
                let mut best: Option<(f32, MidRouteDoor)> = None;
                for door in &world.resource::<NavArchipelagoState>().mid_route_doors {
                    if Some(door.door_form_id) == travel_target_door {
                        continue;
                    }
                    let vertices = door.vertices.map(|vertex| vertex.to_array());
                    let Some(distance) = landmass_graph::distance_to_door_triangle(
                        position.to_array(),
                        vertices,
                        AGENT_HEIGHT,
                    ) else {
                        continue;
                    };
                    let centroid = (door.vertices[0] + door.vertices[1] + door.vertices[2]) / 3.0;
                    if !door_link::approach_gate(door_link::ApproachObservation {
                        distance_to_crossing: distance,
                        agent_distance_to_target,
                        crossing_distance_to_target: flat(centroid, target),
                        stalled,
                    }) {
                        continue;
                    }
                    if best.as_ref().is_none_or(|(best, _)| distance < *best) {
                        best = Some((distance, *door));
                    }
                }
                best.map(|(_, door)| door)
            });

            if let Some(door) = mid_route_crossing {
                let lock_info = world
                    .resource::<NavArchipelagoState>()
                    .door_lock_info
                    .clone();
                let (door_open, door_locked) =
                    door_open_and_locked(world, Some(agent_entity), door.door_form_id, &lock_info);
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
                    request_door_open(world, agent_entity, door.door_form_id);
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
                LinkKind::Merge { kind } => {
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
                            link_kind: kind,
                        },
                    ));
                    world
                        .get_mut::<AgentRuntime>(agent_entity)
                        .unwrap()
                        .active_link = Some(LinkKind::Merge { kind });
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
                    request_door_open(world, agent_entity, door_form_id);
                    info!("nav agent door wait {door_form_id:08x}");
                    let mut runtime = world.get_mut::<AgentRuntime>(agent_entity).unwrap();
                    runtime.door_link = new_state;
                    runtime.active_link = Some(link_kind);
                    runtime.pending_traversal = Some((start_point, end_point));
                }
            }
        }
        door_link::DoorLinkState::Paused {
            door_form_id,
            destination,
            ..
        } => {
            let physically_open = world
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
            // Issue #165 (real-data acceptance, contaminated-leg-B
            // measurement): the door's raw physical open flag alone is not
            // enough for a `Travel` destination -- a prior successful
            // hand-off through this exact door leaves it physically open
            // forever, and a later `setlock` + reissued `tna travel` must
            // not let that stale physical state complete a scripted
            // hand-off through what is now a locked door. See
            // `door_link::effective_door_open`'s doc comment for the full
            // rationale (why `IntraCell` keeps the plain physical-open
            // rule and `Travel` does not).
            let lock_info = world
                .resource::<NavArchipelagoState>()
                .door_lock_info
                .clone();
            let (_, door_locked) =
                door_open_and_locked(world, Some(agent_entity), door_form_id, &lock_info);
            let door_open =
                door_link::effective_door_open(destination, physically_open, door_locked);
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
                // Issue #165: clearing `travel_intent` alone left
                // `AgentTarget3d` still pointed at this exact door's
                // triangle. The very next tick, `drive_door_link_for_agent`
                // re-enters this `Idle | Failed | TravelReached` match arm
                // (current state is now `Failed`), finds no travel arrival
                // (intent is gone), but the mid-route crossing gate no
                // longer excludes this door either -- its exclusion is keyed
                // on `travel_intent`, which is now `None` -- so it
                // "re-discovers" the agent standing in the door's own
                // triangle and restarts the *whole* pause -> wait ->
                // `Failed` cycle via the `IntraCell` destination, forever:
                // `tna status` observed alternating between `Paused` and
                // `Unreachable` on a live locked travel door instead of
                // settling at the documented deterministic terminal
                // (confirmed with a real `NavBackendPlugin` schedule, not
                // just the FSM in isolation -- see this file's
                // `locked_travel_arrival_settles_at_a_stable_unreachable_
                // terminal_not_an_oscillation` test below). A door-link
                // failure has nowhere useful left to walk regardless of
                // which destination type it failed as,
                // so clearing the target here (not just the intent) is the
                // fix: `has_target` in the mid-route check below then
                // reads false and the gate leaves a failed agent alone.
                //
                // Real-data correction (M4 wave 10 post-#153 verification):
                // `PauseAgent` -- inserted when this door-link cycle first
                // paused the agent (`is_traversing(new_state)`'s own arm
                // above is the *only* other place this component is
                // touched) -- was never removed on this `Failed` terminal.
                // `landmass` treats a `PauseAgent`-carrying entity as
                // permanently `AgentState::Paused` (skips its own
                // path/movement solving every tick, `landmass::lib::update`),
                // so a fresh `tna goto`/`tna travel` reissued after the
                // failure (e.g. once a `setlock` unblocks this exact door)
                // set a brand-new `AgentTarget3d` that `landmass` then never
                // even looked at -- confirmed live on FranklinMetro02
                // (0001a273): the agent physically froze at the door's own
                // triangle and stayed `paused` forever after `setlock
                // 0007f7e3 0` + a reissued `tna goto`, despite the door-link
                // FSM itself correctly reaching `Idle`/`Traversing` on later
                // cycles. `PauseAgent` must be removed here too, mirroring
                // the `is_traversing` arm's own removal.
                world.entity_mut(agent_entity).remove::<PauseAgent>();
                world.entity_mut(agent_entity).insert(AgentTarget3d::None);
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

    // Issue #177: open-state poll, tracked separately from usability. An
    // unlocked door that merely opens or shuts never flips `door_usable`, so
    // the loop below would miss it entirely -- but it is exactly the state
    // the closed-blocker cost override keys on. Flips rebuild every active
    // agent's overrides and re-insert its target, the same one-repath-per-
    // flip shape the usability loop uses.
    let open_flips: Vec<(u32, bool)> = tracked
        .iter()
        .filter_map(|&(door_form_id, _)| {
            let (open, _) = door_open_and_locked(world, None, door_form_id, &lock_info);
            let was_open = world
                .resource::<NavArchipelagoState>()
                .door_open
                .get(&door_form_id)
                .copied();
            (was_open != Some(open)).then_some((door_form_id, open))
        })
        .collect();
    if !open_flips.is_empty() {
        for (door_form_id, open) in &open_flips {
            world
                .resource_mut::<NavArchipelagoState>()
                .door_open
                .insert(*door_form_id, *open);
        }
        // Issue #241: every agent, not just roster-indexed ones -- an
        // autonomously-bound actor's route must replan on a door open/close
        // flip exactly like a `tna`-driven one.
        let active_agents = all_agent_entities(world);
        for agent_entity in &active_agents {
            apply_door_lock_overrides(world, *agent_entity);
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
    }

    for (door_form_id, was_usable) in tracked {
        let now_usable = door_usable_now(world, None, door_form_id, &lock_info);
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
                    spawn_link_pair(world, archipelago_entity, link.start, link.end, 1.0, 0)
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
        let active_agents = all_agent_entities(world);
        // Issue #155 feature 2: every active agent's lock-cost overrides
        // must reflect this exact flip before landmass's next solve --
        // `NavArchipelagoState::door_usable` was already updated above, so
        // this rebuild picks up `door_form_id`'s new state along with any
        // other door's existing one.
        for agent_entity in &active_agents {
            apply_door_lock_overrides(world, *agent_entity);
        }
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
        // Issue #185: this door may have become usable *for the shared,
        // no-particular-actor baseline* (an ordinary unlock), or it may
        // still show unusable there while being usable for one specific
        // paused agent that holds its key -- so every paused agent gets its
        // own `request_door_open` attempt rather than one shared check.
        for agent_entity in active_agents.iter().copied() {
            let paused_on_this_door = matches!(
                world.get::<AgentRuntime>(agent_entity).map(|r| r.door_link),
                Some(door_link::DoorLinkState::Paused { door_form_id: paused, .. }) if paused == door_form_id
            );
            if paused_on_this_door {
                request_door_open(world, agent_entity, door_form_id);
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
    let Some(agent_entity) = world.resource::<TestNavAgentState>().get(index) else {
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
    // Issue #162 feature 2: a fresh travel request is a new routing intent
    // -- any merge-portal quarantine from a previous route no longer
    // applies.
    clear_merge_link_quarantine(world, agent_entity);
    // Issue #185: same rationale as `goto_agent` -- re-evaluate this
    // agent's key-aware door-lock overrides for the fresh travel intent.
    apply_door_lock_overrides(world, agent_entity);
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
///
/// Issue #241: an agent with no roster index used to be `continue`d over
/// here, so an autonomously-bound actor never reported `reached`,
/// `off-navmesh` or `unreachable` -- see [`format_agent_id`].
fn log_agent_state_changes(
    mut agents: Query<
        (
            Entity,
            &AgentState,
            &mut AgentRuntime,
            Option<&ActorRuntime>,
        ),
        With<TestNavAgentMarker>,
    >,
    roster: Res<TestNavAgentState>,
) {
    for (entity, agent_state, mut runtime, actor) in &mut agents {
        if runtime.last_logged_state == Some(*agent_state) {
            continue;
        }
        runtime.last_logged_state = Some(*agent_state);
        let id = format_agent_id(
            roster.index_of(entity),
            actor.map(|actor| actor.reference_form_id),
            entity,
        );
        match agent_state {
            AgentState::ReachedTarget => info!("nav agent {id} reached"),
            AgentState::AgentNotOnNavMesh => info!("nav agent off-navmesh {id}"),
            AgentState::TargetNotOnNavMesh | AgentState::NoPath => {
                info!("nav agent {id} unreachable state={agent_state:?}");
            }
            _ => {}
        }
    }
}

fn log_path_latency(
    time: Res<Time>,
    mut agents: Query<
        (
            Entity,
            &AgentState,
            &mut AgentRuntime,
            Option<&ActorRuntime>,
        ),
        With<TestNavAgentMarker>,
    >,
    roster: Res<TestNavAgentState>,
) {
    for (entity, agent_state, mut runtime, actor) in &mut agents {
        if runtime.latency_logged {
            continue;
        }
        let Some(started_at) = runtime.goto_started_at else {
            continue;
        };
        if matches!(agent_state, AgentState::Moving | AgentState::ReachedTarget) {
            let id = format_agent_id(
                roster.index_of(entity),
                actor.map(|actor| actor.reference_form_id),
                entity,
            );
            let latency_ms = (time.elapsed_secs() - started_at) * 1000.0;
            info!("nav agent {id} path latency_ms={latency_ms:.1}");
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
    if !world
        .resource::<NavAgentLedger>()
        .0
        .has_entry_for_cell(current_cell)
    {
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
        // Guard both the "no id" sentinel and the dense roster's defensive
        // allocation ceiling before restoring an external ledger value.
        let Some(index) = entry
            .agent_id
            .checked_sub(1)
            .map(|zero_based| zero_based as usize)
            .filter(|index| *index <= MAX_AGENT_INDEX)
        else {
            warn!(
                "nav agent restore {:08x} cell {:08x}: agent id outside the supported roster; entry dropped",
                entry.agent_id, entry.cell_form_id
            );
            continue;
        };
        // An entry is only ever ledgered while no entity exists at that
        // index, so a live entity here means restoration already happened
        // this activation (or `tna spawn` ran first) -- do not double-spawn.
        if world.resource::<TestNavAgentState>().is_occupied(index) {
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
    world
        .resource_mut::<TestNavAgentState>()
        .set(index, Some(agent_entity));
    info!(
        "nav agent restore {:08x} cell {:08x}",
        entry.agent_id, entry.cell_form_id
    );
}


#[cfg(test)]
#[path = "tests/agent.rs"]
mod tests;
