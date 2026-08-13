//! Atomic active navigation-world state. The nested registries remain one
//! resource authority so topology, links, doors, and cell identity cannot
//! drift as independent Bevy resources.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use bevy::prelude::*;
use bevyout_core::manifest::exterior::GridCoordinate;

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
pub(crate) enum LinkKind {
    Merge { kind: usize },
    Door { form_id: u32 },
}

/// A two-sided intra-cell door link currently excluded from route planning
/// because its door is locked (issue #113 feature 3: "blocked until
/// usable"). The geometry is retained so `door_availability_system` can
/// spawn the real animation link the moment the door becomes usable.
#[derive(Debug, Clone, Copy)]
pub(crate) struct BlockedDoorLink {
    pub(crate) door_form_id: u32,
    pub(crate) start: Vec3,
    pub(crate) end: Vec3,
}

/// A travel door reachable from this cell's nav mesh (issue #113 feature
/// 3): its single-sided triangle midpoint (the routing target), the door
/// placement's own position (the traversal end point -- the agent walks
/// *to* the door, never through into the unloaded destination cell), and
/// the destination cell the existing world-transition metadata (#51/#52)
/// resolves it to.
#[derive(Debug, Clone, Copy)]
pub(crate) struct TravelDoorLink {
    pub(crate) triangle_midpoint: Vec3,
    pub(crate) door_position: Vec3,
    pub(crate) destination_cell_form_id: u32,
    /// The door reference FormID in the destination cell this travel door
    /// pairs with (issue #134): the ledger's `DoorMarker` spawn kind
    /// resolves the agent's restore position from this door's own placed
    /// position once the destination cell is active.
    pub(crate) destination_door_form_id: u32,
}

/// One archipelago + its islands/links for the currently loaded cell,
/// built lazily by `ensure_archipelago` and torn down by
/// `despawn_stale_navmesh_archipelago` on cell swap (mirrors
/// `nav_overlay::despawn_stale_nav_overlay`'s pattern).
#[derive(Resource, Default)]
pub(crate) struct NavArchipelagoState {
    pub(crate) cell_form_id: Option<u32>,
    /// Resident exterior package grids included in the current archipelago.
    /// A new streamed cell changes this signature and forces the ownership
    /// set to rebuild before a subsequent navigation command uses it.
    pub(crate) exterior_resident_grids: Vec<GridCoordinate>,
    /// W3-B's resident NAVM topology for the current resident set (M6 W3-C).
    /// Rebuilt with the archipelago and consulted before any live agent is
    /// re-pointed at it, so a stale or evicting side can never be used.
    pub(crate) resident_nav_topology: crate::viewer::nav::landmass_graph::ResidentNavTopology,
    pub(crate) archipelago: Option<Entity>,
    /// The landmass `Character3d` mirroring the FPS player (issue #114
    /// added scope, wave 5): a non-agent RVO obstacle agents steer around
    /// but that landmass never moves. Lives exactly as long as
    /// `archipelago` -- spawned alongside it in `ensure_archipelago`,
    /// despawned with everything else in `teardown_archipelago` -- so a
    /// cell swap re-associates it with the freshly rebuilt archipelago the
    /// same way agents themselves do.
    pub(crate) player_character: Option<Entity>,
    pub(crate) islands: Vec<Entity>,
    pub(crate) links: Vec<Entity>,
    /// Animation-link entity -> what it represents, so `door_link_system`
    /// can map a `ReachedAnimationLink3d.link_entity` back to either a door
    /// reference to activate or a merge seam to cross directly.
    pub(crate) link_kinds: HashMap<Entity, LinkKind>,
    /// Two-sided door links currently excluded as blocked (locked door).
    pub(crate) blocked_door_links: Vec<BlockedDoorLink>,
    /// Door reference FormID -> terminal travel-link data.
    pub(crate) travel_doors: HashMap<u32, TravelDoorLink>,
    /// Every single-sided door's triangle (issue #137) -- a crossing-gate
    /// candidate regardless of whether it also resolves to a travel
    /// destination (real data: nearly all of them do; see the module doc),
    /// ordered the same way `landmass_graph::single_sided_doors` returns
    /// them (deterministic). `drive_door_link_for_agent` excludes an
    /// agent's own active `travel_intent` door from this set at check time
    /// -- that one door stays owned by the travel-arrival lifecycle.
    pub(crate) mid_route_doors: Vec<MidRouteDoor>,
    /// Last observed per-door usability (open, or not locked), for
    /// `door_availability_system`'s change detection -- exactly one repath
    /// per actual flip.
    pub(crate) door_usable: HashMap<u32, bool>,
    /// Doors' prepared lock/key data + placement entity resolution inputs,
    /// captured from the manifest at build time so the availability poll
    /// does not re-borrow the manifest every frame.
    pub(crate) door_lock_info: HashMap<u32, DoorLockInfo>,
    /// Door FormID -> `landmass` polygon type index (issue #155 feature 1),
    /// the same archipelago-wide mapping `landmass_graph::door_type_indices`
    /// computed for this build's `build_navigation_mesh` calls -- kept here
    /// so `door_availability_system`/`spawn_test_agent` can translate a
    /// door's lock state into the matching `AgentTypeIndexCostOverrides`
    /// entry without recomputing it from the raw mesh inputs every time.
    pub(crate) door_type_indices: BTreeMap<u32, usize>,
    /// Blocker FormID -> `landmass` polygon type index for the *blocking*
    /// derived association class (issue #177, `landmass_graph::
    /// closed_door_type_indices`): the polygons that lie wholly inside the
    /// blocker's collision volume. Priced [`LOCKED_DOOR_TYPE_INDEX_COST`]
    /// whenever the blocker is *closed* -- lock or no lock -- so no route
    /// can ever be planned through the inside of a closed door slab, which
    /// is what let an agent walk in and wedge against it in physics.
    pub(crate) closed_door_type_indices: BTreeMap<u32, usize>,
    /// Blockers that own a runtime open/close FSM (`landmass_graph::
    /// openable_blockers`). Decides whether a closed blocker's interior is
    /// merely expensive ([`CLOSED_DOOR_TYPE_INDEX_COST`]) or impassable.
    pub(crate) openable_blockers: BTreeSet<u32>,
    /// Last observed per-door *open* state, the change detector for the
    /// closed-blocker override above (`door_usable` cannot serve: an
    /// unlocked door is usable whether it is open or shut).
    pub(crate) door_open: HashMap<u32, bool>,
    /// How many distinct merge-portal `landmass` animation-link kinds this
    /// build assigned (issue #162 feature 1, `landmass_graph::
    /// merge_link_kind`): every validated merge candidate this build
    /// spawned a link pair for got kind `1..=merge_link_kind_count`, in
    /// spawn order. `permitted_animation_links_for` needs this to build
    /// the "everything except the quarantined kinds" allow-list
    /// `landmass::PermittedAnimationLinks::Kinds` requires. `0` when this
    /// cell has no merge portals at all.
    pub(crate) merge_link_kind_count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct DoorLockInfo {
    pub(crate) lock_level: Option<i8>,
    pub(crate) key_form_id: Option<u32>,
    /// Issue #185: `PreparedDoor::trapped`, captured the same way
    /// `lock_level`/`key_form_id` are.
    pub(crate) trapped: bool,
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
pub(crate) struct MidRouteDoor {
    pub(crate) door_form_id: u32,
    pub(crate) vertices: [Vec3; 3],
}
