use super::*;
use crate::viewer::nav::world::state::LinkKind;

/// Runtime ownership marker shared by autonomous and console-addressed
/// navigation actors. It is deliberately independent from debug indexing.
#[derive(Component)]
pub(crate) struct NavAgent;

/// Console/debug identity attached only to an agent occupying a `tna` roster
/// slot. Autonomous actors remain ordinary `NavAgent`s without this marker.
#[derive(Component)]
pub(crate) struct DebugNavAgent {
    pub(crate) index: usize,
}

/// Growable roster used only to resolve debug command indices to entities.
#[derive(Resource)]
pub(crate) struct DebugAgentRoster {
    pub(crate) entities: Vec<Option<Entity>>,
}

impl Default for DebugAgentRoster {
    fn default() -> Self {
        Self {
            entities: vec![None; INITIAL_AGENT_SLOTS],
        }
    }
}

impl DebugAgentRoster {
    pub(crate) fn index_of(&self, entity: Entity) -> Option<usize> {
        self.entities.iter().position(|slot| *slot == Some(entity))
    }

    /// Every currently-spawned `(index, entity)` pair, in index order.
    pub(crate) fn active(&self) -> impl Iterator<Item = (usize, Entity)> + '_ {
        self.entities
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| slot.map(|entity| (index, entity)))
    }

    /// The entity occupying `index`, if any.
    pub(crate) fn get(&self, index: usize) -> Option<Entity> {
        self.entities.get(index).copied().flatten()
    }

    pub(crate) fn is_occupied(&self, index: usize) -> bool {
        self.get(index).is_some()
    }

    /// Sets a slot, growing the dense debug roster on demand.
    pub(crate) fn set(&mut self, index: usize, value: Option<Entity>) {
        debug_assert!(index <= MAX_AGENT_INDEX);
        if index >= self.entities.len() {
            self.entities.resize(index + 1, None);
        }
        self.entities[index] = value;
    }
}

pub(crate) const AGENT_RADIUS: f32 = 0.35;
pub(crate) const AGENT_HEIGHT: f32 = 1.8;
pub(crate) const AGENT_DESIRED_SPEED: f32 = 2.5;
/// Contact normals at or above this Y are floor-like; below it they are
/// walls/steep faces, i.e. the things that actually block an agent.
pub(crate) const WALKABLE_CONTACT_NORMAL_Y: f32 = std::f32::consts::FRAC_1_SQRT_2;
pub(crate) const AGENT_MAX_SPEED: f32 = 3.5;
pub(crate) const AGENT_TARGET_REACHED_DISTANCE: f32 = 0.5;
/// Fixed kinematic crossing duration for a door-link traversal (spike
/// simplification -- #113 can derive this from the link's real length and
/// the agent's desired speed instead).
pub(crate) const DOOR_TRAVERSAL_SECONDS: f32 = 0.6;
/// How close (metres, horizontal -- see `movement_policy::nav_point_reached`)
/// the agent must get to a travel door's triangle midpoint before the door
/// lifecycle starts (issue #113 feature 3). Slightly wider than
/// `AGENT_TARGET_REACHED_DISTANCE` so landmass's own target-reached stop
/// always lands inside it.
pub(crate) const TRAVEL_ARRIVAL_DISTANCE: f32 = 0.75;
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
pub(crate) const LOCKED_DOOR_TYPE_INDEX_COST: f32 = f32::INFINITY;
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
pub(crate) const CLOSED_DOOR_TYPE_INDEX_COST: f32 = 1000.0;
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
pub(crate) const PREFERRED_PATHING_TYPE_INDEX_COST: f32 = 0.5;
/// How close (metres, horizontal) a swept merge-portal crossing (issue
/// #154 feature 4) must get to its far portal point before it counts as
/// complete. Same value/rationale as `AGENT_TARGET_REACHED_DISTANCE`.
pub(crate) const MERGE_TRAVERSAL_REACHED_DISTANCE: f32 = 0.5;
/// Horizontal arrival tolerance for the source-alignment leg of a merge
/// handoff. Landmass may emit `ReachedAnimationLink3d` before the capsule is
/// centred on the link's validated source point; aligning within less than
/// one capsule radius keeps the subsequent crossing on the collision-checked
/// segment without requiring an exact floating-point point hit.
pub(crate) const MERGE_SOURCE_ALIGNMENT_DISTANCE: f32 = AGENT_RADIUS * 0.75;
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
pub(crate) const MERGE_TRAVERSAL_TIMEOUT_FACTOR: f32 = 4.0;
/// Fixed floor (seconds) added to the computed timeout (issue #154 feature
/// 4) so a very short crossing still gets a sane minimum window instead of
/// a near-zero deadline.
pub(crate) const MERGE_TRAVERSAL_TIMEOUT_FLOOR_SECONDS: f32 = 1.0;
/// How close (metres, full 3D) `validate_merge_link_collision`'s one-shot
/// `player::move_mover` slide must land to a merge candidate's far portal
/// point to count as "arrived" (issue #154 real-data acceptance
/// correction). Deliberately looser than `MERGE_TRAVERSAL_REACHED_DISTANCE`
/// (which is horizontal-only and compares against a live, already-moving
/// agent): this is a single static slide budgeted at
/// `player::mod::MAX_SLIDE_PASSES` correction passes, not a full per-tick
/// crossing, so a small full-3D residual after sliding off one nearby
/// surface is expected on an otherwise-clear seam.
pub(crate) const MERGE_LINK_SWEEP_TOLERANCE: f32 = 0.6;

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
pub(crate) const AGENT_POINT_SAMPLE_DISTANCE: PointSampleDistance3d = PointSampleDistance3d {
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
pub(crate) const NAV_BORDER_AVOIDANCE_TIME_HORIZON: f32 = 1e-4;

/// The archipelago options every build shares (issue #184): landmass's
/// `from_agent_radius` avoidance defaults, with the point-sampling envelope
/// widened to humanoid scale (`AGENT_POINT_SAMPLE_DISTANCE`) and navmesh-border
/// ORCA avoidance clamped to one tick (`NAV_BORDER_AVOIDANCE_TIME_HORIZON`).
/// One helper rather than per-call-site literals so `ensure_archipelago` and
/// every test harness cannot drift apart on exactly the options a stall
/// regression depends on.
pub(crate) fn archipelago_options() -> ArchipelagoOptions<ThreeD> {
    let mut options = ArchipelagoOptions::from_agent_radius(AGENT_RADIUS);
    options.point_sample_distance = AGENT_POINT_SAMPLE_DISTANCE;
    options.obstacle_avoidance_time_horizon = NAV_BORDER_AVOIDANCE_TIME_HORIZON;
    options
}

/// `DebugAgentRoster`'s initial pre-allocated slot count (issue #215):
/// purely a small default capacity carried over from the original fixed
/// roster size, not a cap -- `DebugAgentRoster::set` grows the vector on
/// demand past this for any higher index.
pub(crate) const INITIAL_AGENT_SLOTS: usize = 4;

/// Defensive ceiling for the console-addressed debug roster. The roster is
/// growable well past the old four-agent limit, but it is still a dense `Vec`:
/// accepting an arbitrary `usize` would let input such as `usize::MAX` request
/// an impossible allocation. 65,536 simultaneous debug agents is far beyond a
/// viable scene while keeping worst-case growth small and ledger IDs unique.
pub(crate) const MAX_AGENT_INDEX: usize = u16::MAX as usize;

/// The ledger/tracing identity for agent `index`: stable, 1-based so it
/// never collides with the "no id" sentinel `0`, consistent with wave 3/4's
/// single `TEST_AGENT_ID = 1`. Formatted as a small decimal in tracing lines
/// (it identifies a spawn slot, not a FormID), but still handed to
/// `ledger_policy` as a plain `u32`.
pub(crate) fn agent_ledger_id(index: usize) -> u32 {
    debug_assert!(index <= MAX_AGENT_INDEX);
    index as u32 + 1
}

/// Present on the agent entity while it is kinematically crossing a
/// door-link edge (`start` -> `end`), holding `apply_agent_physics_movement`
/// off the transform until the crossing completes.
#[derive(Component)]
pub(crate) struct DoorTraversal {
    pub(crate) start: Vec3,
    pub(crate) end: Vec3,
    pub(crate) elapsed: f32,
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
pub(crate) struct MergeTraversal {
    /// Validated near-side portal point. A reached animation link can be
    /// reported while the capsule is still offset from this point, so the
    /// KCC owns a short alignment leg before it starts the actual crossing.
    pub(crate) source: Vec3,
    pub(crate) target: Vec3,
    /// False while moving to `source`; true once the capsule is aligned and
    /// the KCC is sweeping the validated `source -> target` segment.
    pub(crate) crossing_started: bool,
    /// Completion tolerance captured from the distance at which the agent
    /// entered the traversal. A short portal can end up closer than the
    /// normal `0.5 m` arrival tolerance while the agent is still on the
    /// source side; using the fixed tolerance would complete that crossing
    /// before the KCC gets a chance to move it.
    pub(crate) reached_distance: f32,
    /// Seconds elapsed since this crossing started.
    pub(crate) elapsed: f32,
    /// Absolute wall-clock deadline (seconds): computed once at traversal
    /// start from the source-alignment distance plus the portal length, not
    /// recomputed per tick. See [`MERGE_TRAVERSAL_TIMEOUT_FACTOR`]'s doc
    /// comment for why this is a fixed deadline rather than a resettable
    /// no-progress counter (`AgentKcc::best_distance`/
    /// `ticks_without_progress`, owned by the #157 stuck-progress issue and
    /// unsuitable here regardless -- a portal crossing is a distinct,
    /// much shorter-lived motion regime from ordinary route following).
    pub(crate) timeout: f32,
    /// This specific link's `landmass` animation-link kind (issue #162,
    /// `landmass_graph::merge_link_kind`), captured from the matched
    /// `LinkKind::Merge` at crossing start -- the identity
    /// `merge_traversal_system`'s timeout branch quarantines for this
    /// agent alone if the crossing fails, instead of clearing the whole
    /// route.
    pub(crate) link_kind: usize,
}

/// One-tick request to make `bevy_landmass` reconsider the agent's complete
/// input state in its next `FixedPreUpdate` sync. See
/// [`refresh_landmass_animation_link_input`] for the upstream ordering quirk
/// this bridges.
#[derive(Component)]
pub(crate) struct RefreshLandmassAnimationLinkInput;

/// Door-cost overrides temporarily removed only while Landmass reads an
/// animation-link transition. `None` preserves the fact that the agent did
/// not have an override component before the transition.
#[derive(Component)]
pub(crate) struct SuspendedLandmassTypeIndexCosts(pub(crate) Option<AgentTypeIndexCostOverrides>);

/// A capture of `AgentTarget3d`'s two meaningful variants (issue #162):
/// `AgentTarget3d` itself is not `Clone` (`bevy_landmass::AgentTarget`'s
/// derive is `Component, Default` only), so this is the plain-data stand-in
/// [`PendingMergeRepath`] holds across the one-tick target-blank window
/// `resume_pending_merge_repath_system` closes. `None` is deliberately not
/// representable here: `merge_traversal_system`'s timeout branch only ever
/// captures a target worth restoring, never a blank one (see its call
/// site).
#[derive(Debug, Clone, Copy)]
pub(crate) enum AgentTargetSnapshot {
    Point(Vec3),
    Entity(Entity),
}

impl AgentTargetSnapshot {
    pub(crate) fn capture(target: &AgentTarget3d) -> Option<Self> {
        match target {
            AgentTarget3d::Point(point) => Some(Self::Point(*point)),
            AgentTarget3d::Entity(entity) => Some(Self::Entity(*entity)),
            AgentTarget3d::None => None,
        }
    }

    pub(crate) fn to_agent_target(self) -> AgentTarget3d {
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
pub(crate) struct PendingMergeRepath {
    pub(crate) target: AgentTargetSnapshot,
}

/// Per-agent physics-authoritative KCC state (issue #114): the capsule
/// mover's own velocity (landmass's desired velocity is only ever this
/// tick's *input*), grounded state, and the deterministic stuck-tracking
/// counters `movement_policy::decide_stuck` consumes. One per agent entity,
/// inserted at spawn (`spawn_test_agent`) alongside `NavAgent`.
#[derive(Component, Default, Clone, Copy)]
pub(crate) struct AgentKcc {
    pub(crate) velocity: Vec3,
    pub(crate) grounded: bool,
    /// Smallest `movement_policy::StuckObservation::distance_to_target`
    /// observed so far along the current route (reset to `f32::MAX`
    /// whenever a new `tna goto`/`tna travel` target is set, or to the
    /// current value on arrival -- see `apply_agent_physics_movement`).
    /// Since issue #157 this is the negated `route_progress` below, not a
    /// literal distance to the final target; see `movement_policy`'s module
    /// doc comment for why.
    pub(crate) best_distance: f32,
    /// Running integral of `movement_policy::route_progress_delta` over the
    /// whole route (issue #157): metres of real, KCC-resolved motion
    /// achieved along whatever direction landmass was steering toward at
    /// the time, accumulated tick over tick. Never reset on its own --
    /// `best_distance`'s own reset-to-`f32::MAX` handling re-baselines every
    /// comparison to "progress since then" regardless of this field's
    /// absolute running total, so a fresh target does not need a fresh
    /// zero here.
    pub(crate) route_progress: f32,
    pub(crate) ticks_without_progress: u32,
    pub(crate) recovery_active: bool,
    /// This tick's `movement_policy::decide_collision_outcome` classification
    /// (`tna status`'s `blocked=` field; the stable `nav agent
    /// collision-blocked <id>` line fires on the rising edge only).
    pub(crate) collision_blocked: bool,
    /// Latched by `movement_policy::decide_stuck`'s `Stuck` outcome; cleared
    /// by the next `tna goto`/`tna travel` (`tna status`'s `stuck=` field;
    /// the stable `nav agent stuck <id>` line fires on the rising edge).
    pub(crate) stuck: bool,
    /// Y offset from this agent entity's `Transform.translation` to the KCC
    /// capsule *centre* (issue #188). Zero for `tna` capsules, whose entity
    /// transform already is the capsule centre; `AGENT_HEIGHT / 2.0` for a
    /// bound actor, whose placement root sits at feet level. Applied by
    /// `apply_agent_physics_movement` on the way into and back out of the
    /// sweep, so exactly one convention (capsule centre) reaches the KCC
    /// regardless of what the entity's own transform means.
    pub(crate) capsule_centre_offset_y: f32,
    /// This tick's desired horizontal velocity, exactly as
    /// `apply_agent_physics_movement` blended it before handing it to
    /// `step_agent_kcc` (issue #188). Stashed here purely so the locomotion
    /// consumer in `actor_binding` can *reuse* the `desired`/`achieved` pair
    /// this system already computes instead of recomputing a second,
    /// possibly disagreeing one.
    pub(crate) last_desired_horizontal: Vec2,
    /// The achieved half of that same pair: signed horizontal velocity in
    /// `[x, z]`. The sign lets the locomotion window cancel back-and-forth
    /// collision jitter instead of mistaking its magnitude for travel.
    pub(crate) last_achieved_horizontal: Vec2,
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
pub(crate) struct AgentDesiredVelocityBlend {
    pub(crate) previous: Vec3,
    pub(crate) latest: Vec3,
}

/// Per-agent bookkeeping that used to live in the single-agent
/// `DebugAgentRoster` (waves 3/4), now a `Component` on each agent entity
/// so any number of agents (issue #215) can each carry their own
/// door-link/travel/diagnostics state without a parallel resource-side
/// index.
#[derive(Component, Default)]
pub(crate) struct AgentRuntime {
    pub(crate) door_link: door_link::DoorLinkState,
    /// Set by `door_link_system` when a link is first reached, consumed by
    /// the same system once the door opens to start the `DoorTraversal`.
    pub(crate) pending_traversal: Option<(Vec3, Vec3)>,
    /// The link the agent is currently interacting with (for `tna status`'s
    /// `link=` report and for `door_traversal_system` to know whether a
    /// finished crossing should drive the door state machine).
    pub(crate) active_link: Option<LinkKind>,
    /// A pending travel-door route (issue #113 feature 3): the agent is
    /// heading to this door's triangle; arrival starts the door lifecycle
    /// with a `Travel` destination. Consumed by #134's `tna travel`.
    pub(crate) travel_intent: Option<u32>,
    /// `Time::elapsed_secs()` when the last `tna goto` ran, for the
    /// path-latency log line.
    pub(crate) goto_started_at: Option<f32>,
    pub(crate) latency_logged: bool,
    /// Last `AgentState` `log_agent_state_changes` reported, so the stable
    /// evidence lines fire once per actual change instead of every frame.
    pub(crate) last_logged_state: Option<AgentState>,
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
    pub(crate) quarantined_merge_link_kinds: BTreeSet<usize>,
}

/// Renders one nav agent's identity for the stable `nav agent ...` evidence
/// lines and the debug HUD (issue #241).
///
/// Those lines used to interpolate the [`DebugAgentRoster`] roster index and
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

#[derive(Resource, Default)]
pub(crate) struct NavAgentLedger(pub(crate) ledger_policy::Ledger);

/// The origin door reference the player just used to trigger a cell swap
/// (issue #134), noted by `note_player_swap_door` and consumed exactly
/// once by `despawn_stale_navmesh_archipelago` the next time it detects
/// the resulting stale archipelago. `None` when the swap-triggering cause
/// carried no door (there is currently no such path at runtime, but the
/// consumer treats an absent note as "no eligibility information" rather
/// than assuming a door, so any future non-door cell change still freezes
/// a live agent instead of losing it).
#[derive(Resource, Default)]
pub(crate) struct PendingPlayerSwapDoor(pub(crate) Option<u32>);

/// Console-configurable nav-solve interval (issue #114 added scope, wave 5):
/// `LandmassSystems::Update` (the pathfinding+avoidance solve) only runs
/// every `NavSolveRate`-th fixed tick, gated by `nav_solve_gate` against
/// `NavSolveStepCounter`; `apply_agent_physics_movement` still runs -- and
/// moves the agent -- every fixed tick regardless, blending toward whichever
/// desired velocity the last solve produced (`AgentDesiredVelocityBlend`).
/// `tna solverate [<n>]` is the console knob.
#[derive(Resource, Clone, Copy, Debug)]
pub(crate) struct NavSolveRate(pub(crate) u32);

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
pub(crate) struct NavSolveStepCounter(pub(crate) u64);

/// The active cell's minimum prepared geometry Y, kept independently from
/// the replaceable active-world resource for the fall guard.
#[derive(Resource, Default)]
pub(crate) struct NavCellFallBounds {
    pub(crate) min_y: Option<f32>,
}
