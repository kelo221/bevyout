//! Pure backend-neutral navigation-graph resolution (issue #111, M4 wave 2).
//!
//! Given decoded `NAVM` records for one cell (plus the optional content-set
//! `NAVI` singleton), builds a `PreparedNavGraph`: Bevy-metre vertices,
//! polygons with per-edge adjacency, door/off-mesh associations, and
//! external (cross-cell) connections, plus per-mesh/whole-graph AABB bounds
//! and severity-tagged validation diagnostics.
//!
//! Deliberately std/serde-only (no `openmw_esm4`/Bevy imports) so it is
//! includable verbatim by `tests/features.rs` via `#[path]`, the same way
//! `actor_catalog.rs`/`selectors.rs`/`fingerprints.rs` are -- see those
//! modules' doc comments and `AGENTS.md`'s testing section for the pattern.
//! Boundary conversion from `openmw_esm4::{NavMeshRecord, NaviRecord}` into
//! the plain input types below happens in `navmesh.rs`'s `stage_navmeshes`
//! (not `orchestrator.rs`, since that is itself a `vsa::prepare` module free
//! to import `openmw_esm4` types directly, unlike this one).
//!
//! Coordinate conversion reuses `vsa::paths::FO3_SCALE` and the single
//! established `[x, z, -y] * FO3_SCALE` convention (`paths.rs`'s
//! `placement_transform_parts`) -- applied exactly once, here, to every
//! `NVVX` vertex.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::super::paths::FO3_SCALE;

/// Bump whenever the graph asset shape changes, even when new fields are
/// serde-defaulted, per the `ACTOR_CATALOG_REVISION`/`ITEM_CATALOG_REVISION`
/// precedent. Bumped to v8 for issue #177 acceptance: derived associations
/// carry `openable`, which decides whether a closed blocker's interior is
/// merely expensive or genuinely impassable.
/// Bumped to v7 for issue #177 (M4 wave 11): every mesh carries
/// the new `PreparedNavMesh::derived_doors` list of collision-derived
/// blocker -> polygon associations, which the authored `NVDP` data does not
/// provide for ordinary in-cell doors.
/// Bumped to v6 for issue #171 (M4 wave 11): the clearance pass now
/// re-triangulates each mesh locally against the collision-derived walkability
/// boundary, so a graph carries clip-introduced vertices and sub-polygons
/// (authored vertices and unsplit polygons keep their indices), plus the new
/// `clearance_clipped_polygons`/`clearance_added_vertices` counters.
/// Bumped to v5 for issue #153 (M4 wave 10): collision-derived
/// validation + clearance bakes a per-polygon `PreparedNavPolygon::walkable`
/// flag and clearance-offset vertex positions into the graph, plus new
/// `NavGraphCounters` clearance fields (removed/cut/disconnected/offset).
/// Bumped to v4 for issue #156: `PreparedNavMeshMerge::authored_evidence`,
/// `PreparedNavPolygon::authored_external`, and the merge-candidate
/// authored/geometric split + NVEX/NVCI correlation counters.
pub(crate) const NAV_GRAPH_REVISION: &str = "nav-graph-v8";

/// `NVTR` per-edge "external" flag bits (fopdoc FalloutNV `Records/NAVM.html`,
/// "Flag Values"), restated locally per this module's no-`openmw_esm4`-import
/// rule -- the same precedent as `is_water`/`is_preferred_pathing`/
/// `contains_door` below (see `openmw_esm4::navmesh::NavMeshTriangle`'s own
/// constants, the format-side source of truth). Edge slot `n` runs from
/// `vertex_indices[n]` to `vertex_indices[(n + 1) % 3]`, the same slot
/// convention `adjacency` already uses. Issue #156 feature 2: an edge with
/// this bit set is *authored* evidence that the edge is meant to connect
/// outside this mesh (a doorway/seam), as opposed to `adjacency`'s purely
/// geometric "no same-mesh neighbour" inference, which cannot distinguish an
/// authored seam from a genuine dead-end wall.
const EDGE_EXTERNAL_FLAGS: [u32; 3] = [0x0000_0001, 0x0000_0002, 0x0000_0004];

// ---------------------------------------------------------------------
// Plain input types (boundary conversion happens in navmesh.rs)
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct NavGraphVertexInput {
    /// Raw ESM source coordinates (not yet converted to Bevy metres).
    pub(crate) source: [f32; 3],
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct NavGraphTriangleInput {
    /// Widened from `NavMeshTriangle::vertex_indices` (`i16`); negative or
    /// out-of-range values are retained and diagnosed here.
    pub(crate) vertex_indices: [i32; 3],
    /// Widened from `NavMeshTriangle::edge_neighbors` (`i16`); negative
    /// means "no same-mesh neighbour on this edge".
    pub(crate) edge_neighbors: [i32; 3],
    pub(crate) flags: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct NavGraphDoorInput {
    pub(crate) door_reference_form_id: Option<u32>,
    pub(crate) triangle_index: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct NavGraphExternalInput {
    pub(crate) target_navmesh_form_id: Option<u32>,
    pub(crate) triangle_index: u32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct NavGraphMeshInput {
    pub(crate) form_id: u32,
    pub(crate) cell_form_id: Option<u32>,
    pub(crate) vertices: Vec<NavGraphVertexInput>,
    pub(crate) triangles: Vec<NavGraphTriangleInput>,
    pub(crate) doors: Vec<NavGraphDoorInput>,
    pub(crate) external_connections: Vec<NavGraphExternalInput>,
    /// Widened `NVCA` triangle-ID candidates (see `NavMeshRecord::cover_triangle_ids`).
    pub(crate) cover_triangle_ids: Vec<i32>,
}

/// One `NAVI` `NVMI` entry, reduced to the fields this module cares about
/// (grid coordinates, and the owning-cell cross-check).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct NavGraphNaviEntryInput {
    pub(crate) navmesh_form_id: Option<u32>,
    pub(crate) location_form_id: Option<u32>,
    pub(crate) grid_x: i16,
    pub(crate) grid_y: i16,
}

/// One decoded `NAVI` `NVCI` correlation entry (issue #156 feature 3;
/// boundary conversion from `openmw_esm4::navmesh::NaviCorrelationEntry`
/// happens in `navmesh.rs`'s `stage_navmeshes`). Correlation evidence only --
/// never consumed for runtime pathing, see [`navi_correlation`]'s doc
/// comment.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct NavGraphNaviCorrelationEntryInput {
    pub(crate) navmesh_form_id: Option<u32>,
    pub(crate) other_navmesh_form_id: Option<u32>,
    pub(crate) door_form_id: Option<u32>,
}

/// One decoded `NAVI` `NVCI` subrecord (issue #156 feature 3); boundary
/// conversion from `openmw_esm4::navmesh::NaviCorrelation`.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct NavGraphNaviCorrelationInput {
    pub(crate) leading_navmesh_form_id: Option<u32>,
    pub(crate) entries: Vec<NavGraphNaviCorrelationEntryInput>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct NavGraphInputs {
    pub(crate) cell_form_id: u32,
    pub(crate) meshes: Vec<NavGraphMeshInput>,
    pub(crate) navi_entries: Vec<NavGraphNaviEntryInput>,
    /// Decoded `NAVI` `NVCI` subrecords for this cell's content set (issue
    /// #156 feature 3), independent of `navi_entries` (`NVMI`'s reduced
    /// grid-coordinate fields).
    pub(crate) navi_correlations: Vec<NavGraphNaviCorrelationInput>,
}

// ---------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct PreparedNavAabb {
    pub(crate) min: [f32; 3],
    pub(crate) max: [f32; 3],
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct PreparedNavGrid {
    pub(crate) x: i16,
    pub(crate) y: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct PreparedNavPolygon {
    /// Index within this mesh's `polygons` (matches the source `NVTR`
    /// triangle index, so doors/external connections/cover ids -- all keyed
    /// by triangle index -- line up directly).
    pub(crate) index: u32,
    /// Per-vertex index into this mesh's `vertices`. `u32::MAX` is an
    /// explicit invalid-index sentinel for a slot whose source `NVTR` index
    /// was negative or out of range -- already diagnosed as an error at
    /// construction, so the polygon is kept in place (its `index` must stay
    /// stable for doors/external connections/cover ids) rather than dropped.
    /// Consumers must check `index >= vertices.len()` (equivalently, `index
    /// == u32::MAX`) before indexing.
    pub(crate) vertex_indices: [u32; 3],
    /// Per-edge same-mesh neighbour polygon index (`NVTR`'s edge fields);
    /// `None` when the edge has no same-mesh neighbour (mesh boundary, or
    /// externally linked -- see `PreparedNavMesh::external_connections`,
    /// looked up by this polygon's `index`).
    pub(crate) adjacency: [Option<u32>; 3],
    pub(crate) flags: u32,
    pub(crate) is_water: bool,
    pub(crate) is_preferred_pathing: bool,
    pub(crate) contains_door: bool,
    /// Per-edge `NVTR` "external" flag bits (issue #156 feature 2), aligned
    /// with `adjacency`'s slot convention: `authored_external[n]` is the edge
    /// from `vertex_indices[n]` to `vertex_indices[(n + 1) % 3]`. Authored
    /// evidence that the edge is meant to connect outside this mesh,
    /// independent of (and not implied by) `adjacency[n].is_none()`, which
    /// only says this mesh has no same-mesh neighbour on that edge.
    pub(crate) authored_external: [bool; 3],
    /// Issue #153 (M4 wave 10): `false` when the collision-derived
    /// validation/clearance pass (`nav_clearance`, run prepare-side after
    /// physics classification) dropped this polygon -- removed for lacking
    /// collision support (F153.1), cut by an interior wall-like collider
    /// (F153.2), or disconnected as a sub-diameter corridor throat under
    /// agent-radius clearance (F153.3). The runtime seam (`viewer::nav::
    /// mesh_inputs`) excludes `!walkable` polygons from the landmass
    /// navigation mesh, so a route into a dropped region is `unreachable` at
    /// query time. Defaults `true` (serde-missing = walkable) so the flag is
    /// additive; the revision bump above rejects stale caches regardless.
    #[serde(default = "default_walkable")]
    pub(crate) walkable: bool,
}

/// Manual `Default` (not derived) so the Rust-side default of `walkable`
/// matches its serde default (`default_walkable`, `true`). A derived `Default`
/// would give `walkable: false`, so a synthetic `PreparedNavPolygon { ..,
/// ..Default::default() }` would be silently filtered out by
/// `viewer::nav::mesh_inputs`'s walkable filter -- keep the two aligned.
impl Default for PreparedNavPolygon {
    fn default() -> Self {
        Self {
            index: 0,
            vertex_indices: [0; 3],
            adjacency: [None; 3],
            flags: 0,
            is_water: false,
            is_preferred_pathing: false,
            contains_door: false,
            authored_external: [false; 3],
            walkable: default_walkable(),
        }
    }
}

fn default_walkable() -> bool {
    true
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct PreparedNavDoor {
    pub(crate) triangle_index: u32,
    pub(crate) door_reference_form_id: Option<u32>,
}

/// One derived blocker -> polygon association (issue #177). See
/// `PreparedNavMesh::derived_doors` and `prepare::nav_doors` for the
/// geometric rule and the two association classes.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct PreparedNavDerivedDoor {
    pub(crate) triangle_index: u32,
    /// Always populated: a derived association exists only because a
    /// blocking placement's reference FormID produced it.
    pub(crate) door_reference_form_id: u32,
    /// `true` when the polygon lies wholly inside the blocker's collision
    /// volume, so it must not be freely traversable while the blocker is
    /// closed.
    pub(crate) blocks_when_closed: bool,
    /// `true` when this blocker owns a runtime open/close FSM (a real `DOOR`
    /// record). Decides the runtime *cost* of a closed blocker's interior:
    /// passable-but-expensive when it can be opened, impassable when it
    /// cannot -- see `viewer::nav::agent::CLOSED_DOOR_TYPE_INDEX_COST`.
    pub(crate) openable: bool,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct PreparedNavExternalConnection {
    pub(crate) triangle_index: u32,
    /// Retained for M6 exterior-tile stitching (#13/#87); `None` when the
    /// source `NVEX` FormID field was null (diagnosed as an invalid link).
    pub(crate) target_navmesh_form_id: Option<u32>,
}

/// A same-cell cross-mesh connection between two of this graph's own
/// `PreparedNavMesh`es (issue #113, M4 wave 4 feature 2; validated portal
/// geometry added issue #154, M4 wave 8). `NAVI`'s `NVMI` tail (decoded in
/// `openmw_esm4::navmesh`) does not carry OpenMW's TES4/TES5-style
/// merged-navmesh FormID arrays for real FO3 data (see that module's doc
/// comment and `NOTICE.md` for the byte-level verification), so this
/// connection is derived purely from geometry instead: `triangle_a`'s
/// boundary edge (an edge with no same-mesh neighbour) within
/// [`MESH_MERGE_DISTANCE`] of `triangle_b`'s, matched by
/// `compute_mesh_merges`'s reciprocal, non-overlapping-interval resolution
/// (issue #154 feature 2 -- see that function's doc comment for why this is
/// not simple mutual-nearest matching: one long boundary edge can
/// legitimately face several shorter tessellated edges on the other mesh)
/// and passing `validate_portal_candidate`'s opposing-direction/
/// overlapping-interval checks. `edge_a`/`edge_b` are the matched edges'
/// own vertex-index identity (stable across clearance: `nav_clearance`'s
/// protected-edge rule pins these exact vertices in place);
/// `interval_a`/`interval_b` are the two edges' overlapping world-space
/// portal interval, clamped to the shared overlap and positionally
/// corresponding (`interval_a[0]`/`interval_b[0]` are the same point along
/// the shared axis, likewise index `1`) -- note this module deliberately
/// does *not* reject a candidate for excessive vertical drop between the
/// two sides; that is an agent-class-specific check `viewer::nav::
/// landmass_graph` makes at runtime instead (see `BoundaryEdge`'s doc
/// comment). Consumed by
/// `viewer::nav::landmass_graph` to build a walk-through animation link
/// between the interval midpoints (not triangle centroids, issue #154
/// feature 3) when landmass's own island-boundary linking does not connect
/// the two meshes (real FO3 seams do not share exact vertex positions -- see
/// that module's doc comment).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct PreparedNavMeshMerge {
    pub(crate) mesh_a_form_id: u32,
    pub(crate) triangle_a: u32,
    /// `[start_index, end_index]` into `mesh_a`'s own `vertices`, in the
    /// triangle's own winding order.
    pub(crate) edge_a: [u32; 2],
    pub(crate) mesh_b_form_id: u32,
    pub(crate) triangle_b: u32,
    /// `[start_index, end_index]` into `mesh_b`'s own `vertices`.
    pub(crate) edge_b: [u32; 2],
    /// Bevy-metre portal interval on `mesh_a`'s side of the seam, after the
    /// mutual overlap clamp.
    pub(crate) interval_a: [[f32; 3]; 2],
    /// Bevy-metre portal interval on `mesh_b`'s side of the seam.
    pub(crate) interval_b: [[f32; 3]; 2],
    /// Issue #156 feature 2: `true` when `edge_a` or `edge_b` (or both)
    /// carries the authored `NVTR` external-edge flag (`PreparedNavPolygon::
    /// authored_external`), rather than being inferred purely from
    /// `adjacency` geometry. `compute_mesh_merges` also prioritizes authored
    /// candidates over purely geometric ones when resolving conflicting
    /// portal claims on the same edge -- see that function's doc comment.
    pub(crate) authored_evidence: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct PreparedNavMesh {
    pub(crate) form_id: u32,
    pub(crate) cell_form_id: Option<u32>,
    /// `NAVI` `NVMI` grid coordinates for this mesh, when a matching entry
    /// was found.
    pub(crate) grid: Option<PreparedNavGrid>,
    /// Bevy metres, converted exactly once via `FO3_SCALE`.
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) polygons: Vec<PreparedNavPolygon>,
    pub(crate) doors: Vec<PreparedNavDoor>,
    /// Issue #177 (M4 wave 11): blocker -> polygon associations *derived*
    /// from collision footprints rather than read from the authored `NVDP`
    /// door list above. The authored data only associates load/travel doors
    /// with triangles, so an ordinary in-cell door otherwise gets no polygon
    /// typing, no crossing gate, and no door link at all -- the navmesh runs
    /// straight through its closed slab. Kept in its own list, never merged
    /// into `doors`: the authored list's per-door triangle *count* is load
    /// bearing at runtime (`landmass_graph::door_link_descriptors` treats
    /// exactly two as a two-sided link, `single_sided_doors` exactly one as a
    /// travel/crossing candidate), and folding derived associations in would
    /// silently reclassify authored doors.
    ///
    /// Populated by `navmesh::apply_derived_door_associations`, after the
    /// clearance pass -- `triangle_index` refers to post-clip polygons.
    pub(crate) derived_doors: Vec<PreparedNavDerivedDoor>,
    pub(crate) external_connections: Vec<PreparedNavExternalConnection>,
    /// Range-checked `NVCA` triangle-index candidates (see
    /// `NavGraphMeshInput::cover_triangle_ids`'s doc comment on its source
    /// field for why the semantics stay unclaimed beyond "triangle index").
    pub(crate) cover_triangle_indices: Vec<u32>,
    pub(crate) bounds: PreparedNavAabb,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct NavGraphDiagnostic {
    /// `"warning"` or `"error"`.
    pub(crate) severity: String,
    pub(crate) message: String,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct NavGraphCounters {
    pub(crate) meshes: usize,
    pub(crate) polygons: usize,
    pub(crate) vertices: usize,
    pub(crate) doors: usize,
    pub(crate) external_connections: usize,
    pub(crate) mesh_merges: usize,
    /// Mutual-nearest-within-threshold boundary-edge pairs issue #154
    /// feature 2's validation rejected (non-opposing direction, no
    /// overlapping interval, step too high, or a repeated-destination
    /// candidate that lost its mutual-nearest tie) -- one per rejection
    /// diagnostic `compute_mesh_merges` emits. Kept separate from
    /// `mesh_merges` itself so that counter's existing "accepted merges"
    /// meaning stays exactly what it was before this issue.
    pub(crate) mesh_merges_rejected: usize,
    pub(crate) diagnostics_warning: usize,
    pub(crate) diagnostics_error: usize,
    /// Issue #156 feature 2: of `mesh_merges` (accepted merges), how many
    /// carry authored `NVTR` external-edge evidence on at least one side
    /// (`PreparedNavMeshMerge::authored_evidence`) versus how many were
    /// accepted purely on geometric proximity. `mesh_merges_authored +
    /// mesh_merges_geometric == mesh_merges` always.
    pub(crate) mesh_merges_authored: usize,
    pub(crate) mesh_merges_geometric: usize,
    /// Same authored/geometric split, but over every portal *candidate*
    /// `compute_mesh_merges` generated before the reciprocal-overlap
    /// resolution pass accepted or rejected it (a superset of the accepted
    /// `mesh_merges_authored`/`mesh_merges_geometric` counts above).
    pub(crate) merge_candidates_authored: usize,
    pub(crate) merge_candidates_geometric: usize,
    /// Issue #156 feature 3: `NVEX` external-connection correlation counts
    /// (never consumed for runtime pathing -- see `navi_correlation`'s doc
    /// comment). `nvex_targets_inside_cell` is unexpected (a supposedly
    /// "external" connection resolving to a NAVM already in this cell's own
    /// graph); `nvex_targets_outside_cell` is the expected shape (an
    /// exterior-tile stitching candidate for a future milestone).
    pub(crate) nvex_targets_inside_cell: usize,
    pub(crate) nvex_targets_outside_cell: usize,
    /// Issue #156 feature 3: `NAVI` `NVCI` correlation counts (see
    /// `navi_correlation`'s doc comment for the decode's verification
    /// caveat).
    pub(crate) nvci_subrecords: usize,
    pub(crate) nvci_entries: usize,
    pub(crate) nvci_door_matches: usize,
    pub(crate) nvci_navmesh_matches: usize,
    /// Issue #153 (M4 wave 10) collision-derived clearance counters, summed
    /// over every mesh once `apply_nav_clearance` has run. All zero on a
    /// freshly built graph (before clearance); `removed`/`cut` also zero on
    /// cells with no cooked static collision.
    pub(crate) clearance_removed_unsupported: usize,
    pub(crate) clearance_cut_obstructed: usize,
    /// Polygons dropped by the agent-radius clearance-fit test (the capsule
    /// fits nowhere in them -- wall-adjacent slivers and sub-diameter throats).
    pub(crate) clearance_dropped_unfit: usize,
    /// Walkable polygons surviving every clearance phase, whole-graph.
    pub(crate) clearance_walkable_total: usize,
    /// Smallest per-mesh largest-connected-component share (percent of that
    /// mesh's surviving walkable polygons), 100 when every mesh is one
    /// dominant component; the fragmentation health signal.
    pub(crate) clearance_min_component_share_pct: usize,
    pub(crate) clearance_collision_triangles: usize,
    /// Issue #171 (M4 wave 11) local re-triangulation counters: authored
    /// polygons the clip split into more than one piece, and the vertices it
    /// appended (refinement midpoints plus boundary crossings). Both zero on
    /// cells with no cooked static collision.
    pub(crate) clearance_clipped_polygons: usize,
    pub(crate) clearance_added_vertices: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct PreparedNavGraph {
    pub(crate) revision: String,
    pub(crate) cell_form_id: u32,
    /// Sorted by `form_id` (deterministic ordering per the plan).
    pub(crate) meshes: Vec<PreparedNavMesh>,
    pub(crate) bounds: PreparedNavAabb,
    /// Ordered by (mesh `form_id`, polygon/entry index) via construction
    /// order -- never re-sorted by message text, which would scramble that.
    pub(crate) diagnostics: Vec<NavGraphDiagnostic>,
    pub(crate) counters: NavGraphCounters,
    /// Deterministically ordered by `(mesh_a_form_id, triangle_a,
    /// mesh_b_form_id, triangle_b)`. See [`PreparedNavMeshMerge`].
    pub(crate) mesh_merges: Vec<PreparedNavMeshMerge>,
}

// ---------------------------------------------------------------------
// Graph construction
// ---------------------------------------------------------------------

fn to_bevy_position(source: [f32; 3]) -> [f32; 3] {
    [
        source[0] * FO3_SCALE,
        source[2] * FO3_SCALE,
        -source[1] * FO3_SCALE,
    ]
}

fn warning(message: String) -> NavGraphDiagnostic {
    NavGraphDiagnostic {
        severity: "warning".into(),
        message,
    }
}

fn error(message: String) -> NavGraphDiagnostic {
    NavGraphDiagnostic {
        severity: "error".into(),
        message,
    }
}

fn info(message: String) -> NavGraphDiagnostic {
    NavGraphDiagnostic {
        severity: "info".into(),
        message,
    }
}

/// Builds the full graph from `inputs`, in deterministic
/// (mesh `form_id`, polygon/entry index) order.
pub(crate) fn build_nav_graph(inputs: &NavGraphInputs) -> PreparedNavGraph {
    let mut sorted_meshes: Vec<&NavGraphMeshInput> = inputs.meshes.iter().collect();
    sorted_meshes.sort_by_key(|mesh| mesh.form_id);

    let mut meshes = Vec::with_capacity(sorted_meshes.len());
    let mut diagnostics = Vec::new();
    for mesh_input in sorted_meshes.iter().copied() {
        let (mesh, mesh_diagnostics) = build_mesh(mesh_input, &inputs.navi_entries);
        diagnostics.extend(mesh_diagnostics);
        meshes.push(mesh);
    }

    let bounds = whole_graph_bounds(&meshes);
    let (mesh_merges, merge_diagnostics, merge_candidate_counts) = compute_mesh_merges(&meshes);
    let mesh_merges_rejected = merge_diagnostics.len();
    diagnostics.extend(merge_diagnostics);
    let mesh_merges_authored = mesh_merges.iter().filter(|m| m.authored_evidence).count();
    let mesh_merges_geometric = mesh_merges.len() - mesh_merges_authored;

    let (navi_correlation_counts, navi_correlation_diagnostics) =
        navi_correlation(&meshes, &inputs.navi_correlations);
    diagnostics.extend(navi_correlation_diagnostics);

    let diagnostics_warning = diagnostics
        .iter()
        .filter(|d| d.severity == "warning")
        .count();
    let diagnostics_error = diagnostics.iter().filter(|d| d.severity == "error").count();
    let counters = NavGraphCounters {
        meshes: meshes.len(),
        polygons: meshes.iter().map(|m| m.polygons.len()).sum(),
        vertices: meshes.iter().map(|m| m.vertices.len()).sum(),
        doors: meshes.iter().map(|m| m.doors.len()).sum(),
        external_connections: meshes.iter().map(|m| m.external_connections.len()).sum(),
        mesh_merges: mesh_merges.len(),
        mesh_merges_rejected,
        diagnostics_warning,
        diagnostics_error,
        mesh_merges_authored,
        mesh_merges_geometric,
        merge_candidates_authored: merge_candidate_counts.authored,
        merge_candidates_geometric: merge_candidate_counts.geometric,
        nvex_targets_inside_cell: navi_correlation_counts.nvex_targets_inside_cell,
        nvex_targets_outside_cell: navi_correlation_counts.nvex_targets_outside_cell,
        nvci_subrecords: navi_correlation_counts.nvci_subrecords,
        nvci_entries: navi_correlation_counts.nvci_entries,
        nvci_door_matches: navi_correlation_counts.nvci_door_matches,
        nvci_navmesh_matches: navi_correlation_counts.nvci_navmesh_matches,
        // Populated later by `navmesh::apply_nav_clearance` (issue #153),
        // which runs after physics classification; zero on the freshly built
        // graph. `clearance_min_component_share_pct` is left 0 here and set to
        // its real value (100 when unfragmented) by that pass.
        clearance_removed_unsupported: 0,
        clearance_cut_obstructed: 0,
        clearance_dropped_unfit: 0,
        clearance_walkable_total: 0,
        clearance_min_component_share_pct: 0,
        clearance_collision_triangles: 0,
        clearance_clipped_polygons: 0,
        clearance_added_vertices: 0,
    };

    PreparedNavGraph {
        revision: NAV_GRAPH_REVISION.into(),
        cell_form_id: inputs.cell_form_id,
        meshes,
        bounds,
        diagnostics,
        counters,
        mesh_merges,
    }
}

/// Result of [`navi_correlation`]'s FormID cross-referencing.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct NaviCorrelationCounts {
    nvex_targets_inside_cell: usize,
    nvex_targets_outside_cell: usize,
    nvci_subrecords: usize,
    nvci_entries: usize,
    nvci_door_matches: usize,
    nvci_navmesh_matches: usize,
}

/// Issue #156 feature 3: correlates already-decoded `NVEX` external
/// connections (`meshes[..].external_connections`, unchanged from issue
/// #111) and `NAVI` `NVCI` correlation subrecords (new this issue, see
/// `openmw_esm4::navmesh::decode_navi_correlation`'s doc comment for the
/// decode's verification caveat) against this cell's own known NAVM/door
/// FormIDs. Correlation only -- neither output feeds pathing or mesh-merge
/// resolution; this is evidence for a future exterior-stitching milestone
/// (see the module doc comment's `PreparedNavMeshMerge` reference and issue
/// #13/#87), surfaced today only as counters and diagnostics.
///
/// A `target`/`navmesh`/`other_navmesh` FormID resolving to one of this
/// cell's own NAVM `form_id`s is *inside* this cell already (an NVEX pointing
/// back into the same cell is unexpected -- NVEX's whole purpose is
/// presumably cross-cell/cross-worldspace linkage); anything else is
/// *outside* (the expected shape for both fields, since this cell's graph
/// has no visibility into other cells' NAVM FormIDs to positively confirm
/// them). Never panics: every lookup is a plain FormID set membership check,
/// no indexing.
fn navi_correlation(
    meshes: &[PreparedNavMesh],
    navi_correlations: &[NavGraphNaviCorrelationInput],
) -> (NaviCorrelationCounts, Vec<NavGraphDiagnostic>) {
    let known_navmesh_form_ids: BTreeSet<u32> = meshes.iter().map(|mesh| mesh.form_id).collect();
    let known_door_form_ids: BTreeSet<u32> = meshes
        .iter()
        .flat_map(|mesh| {
            mesh.doors
                .iter()
                .filter_map(|door| door.door_reference_form_id)
        })
        .collect();

    let mut counts = NaviCorrelationCounts::default();
    for mesh in meshes {
        for connection in &mesh.external_connections {
            let Some(target) = connection.target_navmesh_form_id else {
                continue;
            };
            if known_navmesh_form_ids.contains(&target) {
                counts.nvex_targets_inside_cell += 1;
            } else {
                counts.nvex_targets_outside_cell += 1;
            }
        }
    }

    for correlation in navi_correlations {
        counts.nvci_subrecords += 1;
        let mut navmesh_candidates: Vec<u32> =
            correlation.leading_navmesh_form_id.into_iter().collect();
        for entry in &correlation.entries {
            counts.nvci_entries += 1;
            navmesh_candidates.extend(entry.navmesh_form_id);
            navmesh_candidates.extend(entry.other_navmesh_form_id);
            if let Some(door) = entry.door_form_id
                && known_door_form_ids.contains(&door)
            {
                counts.nvci_door_matches += 1;
            }
        }
        counts.nvci_navmesh_matches += navmesh_candidates
            .iter()
            .filter(|form_id| known_navmesh_form_ids.contains(form_id))
            .count();
    }

    let mut diagnostics = Vec::new();
    if counts.nvex_targets_inside_cell > 0 || counts.nvex_targets_outside_cell > 0 {
        diagnostics.push(info(format!(
            "NVEX correlation: {} target(s) outside this cell's own NAVM set (exterior stitching evidence), {} target(s) already inside this cell (unexpected)",
            counts.nvex_targets_outside_cell, counts.nvex_targets_inside_cell
        )));
    }
    if counts.nvci_subrecords > 0 {
        diagnostics.push(info(format!(
            "NVCI correlation: {} subrecord(s), {} entrie(s), {} door FormID match(es) against this cell's own NAVM doors, {} NAVM FormID match(es) against this cell's own NAVM records",
            counts.nvci_subrecords,
            counts.nvci_entries,
            counts.nvci_door_matches,
            counts.nvci_navmesh_matches
        )));
    }

    (counts, diagnostics)
}

/// bevy-metre distance below which two different meshes' unconnected
/// boundary edges (an edge with no same-mesh `adjacency` neighbour) are
/// considered the same seam and linked as a [`PreparedNavMeshMerge`] (issue
/// #113, M4 wave 4). Real FO3 NAVM data does not share exact vertex
/// positions across separate records at a seam: FranklinMetro02's
/// (`0001a273`) two real meshes measured 0.09-0.9 m gaps between their
/// nearest boundary vertices at the actual connecting corridor, confirmed by
/// direct inspection of the prepared graph. `landmass`'s native
/// island-boundary linking needs coincident positions and does not connect
/// them (see `viewer::nav::landmass_graph`'s doc comment), which is exactly
/// the gap this generates explicit links for. The threshold is generous
/// relative to that measured real gap so other cells' seams (with slightly
/// larger float drift) still match, while staying well under typical
/// interior room dimensions so unrelated boundary edges across a large
/// navmesh are not matched.
const MESH_MERGE_DISTANCE: f32 = 2.0;

/// Minimum cosine of the angle between two candidate portal edges'
/// direction vectors for them to count as "facing" each other across a
/// seam (issue #154 feature 2, "near-opposing edge directions"). `-1.0` is
/// perfectly opposite (180°); this project accepts down to `-0.5` (an angle
/// of at least 120°) -- generous enough for FO3's independently-authored
/// per-mesh boundary edges (the seams `MESH_MERGE_DISTANCE`'s doc comment
/// cites already measure 0.09-0.9 m of position drift, so some angular
/// drift between the two edges is expected too) while still rejecting a
/// near-perpendicular or same-direction pairing, which cannot be two sides
/// of one doorway/seam.
const PORTAL_DIRECTION_COSINE_MAX: f32 = -0.5;

/// Minimum length (bevy metres) of the two candidate edges' overlapping
/// projection onto their shared axis for it to count as a real portal
/// (issue #154 feature 2). A hair above zero: large enough to reject two
/// edges that merely touch at a single point (no actual walkable crossing
/// between them), small enough not to reject a genuine narrow real-data
/// seam.
const PORTAL_MIN_OVERLAP: f32 = 0.01;

/// One mesh-boundary edge candidate for [`compute_mesh_merges`] (issue
/// #154): an edge referenced by exactly one polygon (`adjacency` slot
/// `None`), carrying both its stable vertex-index identity (feeding
/// [`PreparedNavMeshMerge::edge_a`]/`edge_b`) and its resolved world
/// positions (feeding `validate_portal_candidate`'s direction/overlap
/// geometry). Deliberately agent-class-agnostic: earlier revisions of this
/// module also rejected a candidate whose matched interval had too much
/// vertical drop for a *specific* (hardcoded, unnamed) agent step height --
/// an external review correctly flagged that as baking an agent-class
/// assumption into the universal prepared graph. That check now lives at
/// runtime in `viewer::nav::landmass_graph`, where the actual agent
/// definition (`nav::agent::AGENT_RADIUS` and friends) already lives; this
/// module still records the full (possibly vertically offset)
/// `interval_a`/`interval_b` on every accepted [`PreparedNavMeshMerge`] so
/// that runtime check has the data to work with.
struct BoundaryEdge {
    triangle_index: u32,
    /// `[start_index, end_index]` into the owning mesh's `vertices`, in the
    /// triangle's own winding order (`vertex_indices[slot]` ->
    /// `vertex_indices[(slot + 1) % 3]`).
    vertex_indices: [u32; 2],
    start: [f32; 3],
    end: [f32; 3],
    /// Issue #156 feature 2: this edge's own `PreparedNavPolygon::
    /// authored_external` slot -- authored `NVTR` evidence that this
    /// boundary edge is meant to connect outside this mesh, independent of
    /// the purely geometric "no same-mesh neighbour" fact that put it in
    /// `boundary_edges` at all.
    authored_external: bool,
}

fn boundary_edges(mesh: &PreparedNavMesh) -> Vec<BoundaryEdge> {
    let mut edges = Vec::new();
    for polygon in &mesh.polygons {
        for (slot, neighbor) in polygon.adjacency.iter().enumerate() {
            if neighbor.is_some() {
                continue;
            }
            let a = polygon.vertex_indices[slot];
            let b = polygon.vertex_indices[(slot + 1) % 3];
            if a as usize >= mesh.vertices.len() || b as usize >= mesh.vertices.len() {
                // Already diagnosed as an invalid-index error when the
                // polygon was built; skip rather than index out of bounds.
                continue;
            }
            let va = mesh.vertices[a as usize];
            let vb = mesh.vertices[b as usize];
            edges.push(BoundaryEdge {
                triangle_index: polygon.index,
                vertex_indices: [a, b],
                start: va,
                end: vb,
                authored_external: polygon.authored_external[slot],
            });
        }
    }
    edges
}

fn distance_sq(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx * dx + dy * dy + dz * dz
}

fn vec_sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn vec_dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn vec_length(a: [f32; 3]) -> f32 {
    vec_dot(a, a).sqrt()
}

/// `None` for a (near-)zero-length vector -- callers treat that as a
/// degenerate edge, never a division by (near) zero.
fn vec_normalize(a: [f32; 3]) -> Option<[f32; 3]> {
    let length = vec_length(a);
    if length < 1.0e-6 {
        None
    } else {
        Some([a[0] / length, a[1] / length, a[2] / length])
    }
}

fn vec_add_scaled(origin: [f32; 3], direction: [f32; 3], t: f32) -> [f32; 3] {
    [
        origin[0] + direction[0] * t,
        origin[1] + direction[1] * t,
        origin[2] + direction[2] * t,
    ]
}

fn vec_lerp(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

/// Squared distance from `point` to the segment `start..end`.
fn point_segment_distance_sq(point: [f32; 3], start: [f32; 3], end: [f32; 3]) -> f32 {
    let segment = vec_sub(end, start);
    let length_sq = vec_dot(segment, segment);
    if length_sq < 1.0e-12 {
        return distance_sq(point, start);
    }
    let t = (vec_dot(vec_sub(point, start), segment) / length_sq).clamp(0.0, 1.0);
    distance_sq(point, vec_add_scaled(start, segment, t))
}

/// A conservative closeness pre-filter between two boundary edges, robust
/// to very different edge lengths (issue #154 review correction: one long
/// boundary edge may legitimately face several much shorter tessellated
/// edges on the other mesh, whose *midpoints* can sit far from the long
/// edge's own midpoint even though the edges genuinely overlap along part
/// of their length) -- the minimum of each edge's endpoints' squared
/// distance to the *other* edge's segment, not just midpoint-to-midpoint.
fn boundary_edge_distance_sq(edge_a: &BoundaryEdge, edge_b: &BoundaryEdge) -> f32 {
    point_segment_distance_sq(edge_a.start, edge_b.start, edge_b.end)
        .min(point_segment_distance_sq(
            edge_a.end,
            edge_b.start,
            edge_b.end,
        ))
        .min(point_segment_distance_sq(
            edge_b.start,
            edge_a.start,
            edge_a.end,
        ))
        .min(point_segment_distance_sq(
            edge_b.end,
            edge_a.start,
            edge_a.end,
        ))
}

/// Why a portal candidate failed issue #154 feature 2's per-pair geometric
/// checks; each variant carries its own stable diagnostic wording
/// (`describe`) so a `prepare` operator can tell which rule fired without
/// re-deriving the geometry by hand. A candidate that passes both of these
/// can still be rejected later by `compute_mesh_merges`'s own interval-
/// overlap resolution -- that rejection is reported inline there, not
/// through this enum, since it depends on *other* candidates, not just this
/// pair.
#[derive(Debug, Clone, Copy)]
enum PortalRejection {
    /// The two edges' direction vectors are not opposing enough to be two
    /// sides of the same seam/doorway (see [`PORTAL_DIRECTION_COSINE_MAX`]).
    NonOpposingDirection { cosine: f32 },
    /// One (or both) edges are degenerate (near-zero length), so no
    /// direction can be derived.
    DegenerateEdge,
    /// The two edges' projections onto their shared axis do not overlap
    /// (see [`PORTAL_MIN_OVERLAP`]).
    NoIntervalOverlap,
}

impl PortalRejection {
    fn describe(self) -> String {
        match self {
            PortalRejection::NonOpposingDirection { cosine } => format!(
                "edge directions not opposing enough (cosine {cosine:.3}, need <= {PORTAL_DIRECTION_COSINE_MAX})"
            ),
            PortalRejection::DegenerateEdge => "a degenerate (near-zero-length) edge".to_string(),
            PortalRejection::NoIntervalOverlap => {
                "no overlapping portal interval along the shared edge axis".to_string()
            }
        }
    }
}

/// A validated portal candidate between one `BoundaryEdge` in each mesh:
/// the matched world-space interval on both sides (positionally
/// corresponding -- `interval_a[0]`/`interval_b[0]` are the same point
/// along the shared axis, likewise index `1`), plus each side's own
/// interval expressed as a `[min, max]` distance-along-the-edge range (from
/// `edge_a.start`/`edge_b.start` respectively) -- the common unit
/// `compute_mesh_merges`'s overlap-resolution phase compares different
/// candidates on the *same* edge against each other in, regardless of which
/// opposing edge (and therefore which shared axis) produced each one.
struct PortalCandidate {
    interval_a: [[f32; 3]; 2],
    interval_b: [[f32; 3]; 2],
    a_range: (f32, f32),
    b_range: (f32, f32),
}

/// Issue #154 feature 2's per-pair geometric validation: opposing
/// directions and an overlapping projected interval. Reciprocal, non-
/// overlapping interval correspondence *across every candidate on an edge*
/// (no repeated/conflicting destination coverage) is a global property the
/// caller (`compute_mesh_merges`) resolves afterward, once every pair's
/// candidate interval is known -- a single pair can never determine that on
/// its own.
fn validate_portal_candidate(
    edge_a: &BoundaryEdge,
    edge_b: &BoundaryEdge,
) -> Result<PortalCandidate, PortalRejection> {
    let raw_direction_a = vec_sub(edge_a.end, edge_a.start);
    let raw_direction_b = vec_sub(edge_b.end, edge_b.start);
    let Some(direction_a) = vec_normalize(raw_direction_a) else {
        return Err(PortalRejection::DegenerateEdge);
    };
    let Some(direction_b) = vec_normalize(raw_direction_b) else {
        return Err(PortalRejection::DegenerateEdge);
    };
    let cosine = vec_dot(direction_a, direction_b);
    if cosine > PORTAL_DIRECTION_COSINE_MAX {
        return Err(PortalRejection::NonOpposingDirection { cosine });
    }

    // Project both edges onto edge_a's own axis -- the direction check just
    // above already confirmed edge_b's direction is substantially opposed
    // to it, so it is a reasonable shared axis for both sides.
    let origin = edge_a.start;
    let t_a0 = 0.0_f32;
    let t_a1 = vec_dot(vec_sub(edge_a.end, origin), direction_a);
    let (a_min, a_max) = (t_a0.min(t_a1), t_a0.max(t_a1));
    let t_b0 = vec_dot(vec_sub(edge_b.start, origin), direction_a);
    let t_b1 = vec_dot(vec_sub(edge_b.end, origin), direction_a);
    let (b_min, b_max) = (t_b0.min(t_b1), t_b0.max(t_b1));

    let overlap_min = a_min.max(b_min);
    let overlap_max = a_max.min(b_max);
    if overlap_max - overlap_min < PORTAL_MIN_OVERLAP {
        return Err(PortalRejection::NoIntervalOverlap);
    }

    let interval_a = [
        vec_add_scaled(origin, direction_a, overlap_min),
        vec_add_scaled(origin, direction_a, overlap_max),
    ];
    // Map the same two axis parameters back onto edge_b's own segment via
    // its local `t_b0..t_b1` parametrization. `t_b1 - t_b0` cannot be (near)
    // zero here: `cosine <= PORTAL_DIRECTION_COSINE_MAX` above already
    // guarantees edge_b's direction has a component of magnitude >= 0.5
    // along `direction_a`, and `vec_normalize` above already ruled out a
    // zero-length edge_b.
    let denom = t_b1 - t_b0;
    let s_min = (overlap_min - t_b0) / denom;
    let s_max = (overlap_max - t_b0) / denom;
    let interval_b = [
        vec_lerp(edge_b.start, edge_b.end, s_min),
        vec_lerp(edge_b.start, edge_b.end, s_max),
    ];

    // edge_b's own distance-along-edge_b range, independent of whichever
    // edge_a produced this candidate -- `length_b` un-normalizes `s_min`/
    // `s_max` (each in `[0, 1]` along edge_b) back to bevy metres from
    // `edge_b.start`.
    let length_b = vec_length(raw_direction_b);
    let (b_range_min, b_range_max) = (
        (s_min * length_b).min(s_max * length_b),
        (s_min * length_b).max(s_max * length_b),
    );

    Ok(PortalCandidate {
        interval_a,
        interval_b,
        a_range: (overlap_min, overlap_max),
        b_range: (b_range_min, b_range_max),
    })
}

/// Whether two `[min, max]` ranges on the same edge overlap by more than a
/// hair (floating-point slack so two intervals that merely touch at a
/// shared boundary point are not treated as conflicting).
fn ranges_overlap(a: (f32, f32), b: (f32, f32)) -> bool {
    const EPSILON: f32 = 1.0e-4;
    a.0 < b.1 - EPSILON && b.0 < a.1 - EPSILON
}

/// Derives same-cell cross-mesh connections between every pair of `meshes`
/// (already sorted by `form_id`) using issue #154's validated-portal
/// pipeline (reworked after external review: an earlier revision required
/// each boundary edge to be the *other's* single nearest match, which
/// cannot represent a long boundary edge legitimately facing several
/// shorter tessellated edges on the other mesh -- real FO3 NAVM data does
/// this):
///
/// 1. **Candidate generation.** Every boundary-edge pair within
///    [`MESH_MERGE_DISTANCE`] of each other (`boundary_edge_distance_sq`,
///    robust to very different edge lengths) is geometrically validated
///    (`validate_portal_candidate`: opposing directions, an overlapping
///    projected interval) independently -- an edge can appear in any
///    number of candidates, not just one.
/// 2. **Reciprocal, non-overlapping resolution.** Candidates are visited in
///    a deterministic priority order -- authored `NVTR` external-edge
///    evidence first (issue #156 feature 2: a candidate where `edge_a` or
///    `edge_b` carries the flag outranks one where neither does, and a
///    candidate flagged on *both* sides outranks one flagged on only one),
///    then longest overlap, then tie-broken by edge index -- and greedily
///    accepted only when neither side's own `[min, max]` distance-along-the-
///    edge range overlaps a range already accepted on that *same* edge
///    (`ranges_overlap`) -- this is what replaces mutual-nearest-only
///    matching: it allows disjoint intervals on one long edge to each pair
///    with a different shorter opposing edge (one-to-many), while still
///    rejecting two candidates that would double-claim the same stretch of
///    an edge (the real-data defect an external review found in the
///    original one-directional nearest-midpoint matching, where every edge
///    of a facing triangle independently claimed the same destination
///    edge).
///
/// Deterministic (`meshes` is already form_id-sorted, boundary edges are
/// visited in stable polygon/vertex order, candidates are sorted by a
/// total order before the greedy resolution pass, and the result is
/// explicitly sorted+deduped by full mesh/triangle/edge identity) and never
/// panics (invalid vertex indices are skipped by `boundary_edges`, never
/// indexed). Returns the accepted merges, one diagnostic for each candidate
/// rejected by a per-pair geometric check (direction/overlap) or by the
/// overlap-resolution pass, and the authored-vs-geometric split over every
/// candidate generated (issue #156 feature 2's evidence for the prepare
/// summary line); a boundary-edge pair that never became a candidate at all
/// (outside `MESH_MERGE_DISTANCE`, or geometrically non-overlapping) is not
/// diagnosed and not counted -- most nearby boundary-edge pairs on a busy
/// seam are not the matching edge, and logging every one of them would drown
/// the signal a real rejection (a near-miss that *did* pass the distance
/// gate) is meant to surface.
fn compute_mesh_merges(
    meshes: &[PreparedNavMesh],
) -> (
    Vec<PreparedNavMeshMerge>,
    Vec<NavGraphDiagnostic>,
    MergeCandidateCounts,
) {
    let threshold_sq = MESH_MERGE_DISTANCE * MESH_MERGE_DISTANCE;
    let mut merges = Vec::new();
    let mut diagnostics = Vec::new();
    let mut candidate_counts = MergeCandidateCounts::default();
    for a_index in 0..meshes.len() {
        for b_index in (a_index + 1)..meshes.len() {
            let mesh_a = &meshes[a_index];
            let mesh_b = &meshes[b_index];
            let edges_a = boundary_edges(mesh_a);
            let edges_b = boundary_edges(mesh_b);

            struct IndexedCandidate {
                a_edge_index: usize,
                b_edge_index: usize,
                candidate: PortalCandidate,
                /// Issue #156 feature 2: how many of this candidate's two
                /// edges carry the authored `NVTR` external-edge flag (`0`,
                /// `1`, or `2`) -- the priority key `compute_mesh_merges`
                /// sorts candidates by before overlap length.
                authored_tier: u8,
            }
            let mut candidates: Vec<IndexedCandidate> = Vec::new();
            for (a_edge_index, edge_a) in edges_a.iter().enumerate() {
                for (b_edge_index, edge_b) in edges_b.iter().enumerate() {
                    if boundary_edge_distance_sq(edge_a, edge_b) > threshold_sq {
                        continue;
                    }
                    match validate_portal_candidate(edge_a, edge_b) {
                        Ok(candidate) => {
                            let authored_tier = u8::from(edge_a.authored_external)
                                + u8::from(edge_b.authored_external);
                            if authored_tier > 0 {
                                candidate_counts.authored += 1;
                            } else {
                                candidate_counts.geometric += 1;
                            }
                            candidates.push(IndexedCandidate {
                                a_edge_index,
                                b_edge_index,
                                candidate,
                                authored_tier,
                            });
                        }
                        // A pure "these two nearby edges don't actually
                        // overlap along their length" outcome is the
                        // expected common case, not a defect -- only a
                        // direction/degeneracy failure (a genuine near-miss
                        // shape) is worth an operator's attention. See this
                        // function's doc comment.
                        Err(PortalRejection::NoIntervalOverlap) => {}
                        Err(reason) => diagnostics.push(warning(format!(
                            "mesh {:08x} triangle {} <-> mesh {:08x} triangle {}: rejected portal candidate ({})",
                            mesh_a.form_id,
                            edge_a.triangle_index,
                            mesh_b.form_id,
                            edge_b.triangle_index,
                            reason.describe(),
                        ))),
                    }
                }
            }

            // Authored evidence first (issue #156 feature 2), then longest
            // overlap -- the most unambiguous portal match on any contested
            // stretch of an edge should win it among equally-authored
            // candidates. Ties broken by edge index for full determinism.
            candidates.sort_by(|left, right| {
                let left_len = left.candidate.a_range.1 - left.candidate.a_range.0;
                let right_len = right.candidate.a_range.1 - right.candidate.a_range.0;
                right.authored_tier.cmp(&left.authored_tier).then(
                    right_len
                        .partial_cmp(&left_len)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then(left.a_edge_index.cmp(&right.a_edge_index))
                        .then(left.b_edge_index.cmp(&right.b_edge_index)),
                )
            });

            let mut accepted_a_ranges: Vec<Vec<(f32, f32)>> = vec![Vec::new(); edges_a.len()];
            let mut accepted_b_ranges: Vec<Vec<(f32, f32)>> = vec![Vec::new(); edges_b.len()];
            for indexed in candidates {
                let edge_a = &edges_a[indexed.a_edge_index];
                let edge_b = &edges_b[indexed.b_edge_index];
                let a_conflict = accepted_a_ranges[indexed.a_edge_index]
                    .iter()
                    .any(|range| ranges_overlap(*range, indexed.candidate.a_range));
                let b_conflict = accepted_b_ranges[indexed.b_edge_index]
                    .iter()
                    .any(|range| ranges_overlap(*range, indexed.candidate.b_range));
                if a_conflict || b_conflict {
                    diagnostics.push(warning(format!(
                        "mesh {:08x} triangle {} <-> mesh {:08x} triangle {}: rejected portal candidate (overlaps another accepted portal interval on the {})",
                        mesh_a.form_id,
                        edge_a.triangle_index,
                        mesh_b.form_id,
                        edge_b.triangle_index,
                        if a_conflict { "mesh_a edge" } else { "mesh_b edge" },
                    )));
                    continue;
                }
                accepted_a_ranges[indexed.a_edge_index].push(indexed.candidate.a_range);
                accepted_b_ranges[indexed.b_edge_index].push(indexed.candidate.b_range);
                merges.push(PreparedNavMeshMerge {
                    mesh_a_form_id: mesh_a.form_id,
                    triangle_a: edge_a.triangle_index,
                    edge_a: edge_a.vertex_indices,
                    mesh_b_form_id: mesh_b.form_id,
                    triangle_b: edge_b.triangle_index,
                    edge_b: edge_b.vertex_indices,
                    interval_a: indexed.candidate.interval_a,
                    interval_b: indexed.candidate.interval_b,
                    authored_evidence: indexed.authored_tier > 0,
                });
            }
        }
    }
    merges.sort_by(|left, right| {
        (
            left.mesh_a_form_id,
            left.triangle_a,
            left.edge_a,
            left.mesh_b_form_id,
            left.triangle_b,
            left.edge_b,
        )
            .cmp(&(
                right.mesh_a_form_id,
                right.triangle_a,
                right.edge_a,
                right.mesh_b_form_id,
                right.triangle_b,
                right.edge_b,
            ))
    });
    // Defensive only at this point (each `(a_edge_index, b_edge_index)`
    // pair is visited at most once above, so structurally-identical
    // duplicates should not arise) -- kept so a future refactor bug fails
    // safe (a dropped duplicate) rather than shipping a doubled link.
    merges.dedup();
    (merges, diagnostics, candidate_counts)
}

/// Issue #156 feature 2: authored-vs-geometric split over every portal
/// candidate [`compute_mesh_merges`] generated (accepted or rejected) --
/// `authored` counts a candidate where at least one of its two edges carries
/// the `NVTR` external-edge flag, `geometric` counts a candidate backed by
/// pure proximity/opposing-direction/overlap geometry alone.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct MergeCandidateCounts {
    authored: usize,
    geometric: usize,
}

fn build_mesh(
    input: &NavGraphMeshInput,
    navi_entries: &[NavGraphNaviEntryInput],
) -> (PreparedNavMesh, Vec<NavGraphDiagnostic>) {
    let mut diagnostics = Vec::new();
    let vertex_count = input.vertices.len();
    let triangle_count = input.triangles.len();

    let vertices = input
        .vertices
        .iter()
        .map(|vertex| to_bevy_position(vertex.source))
        .collect::<Vec<_>>();

    let mut polygons = Vec::with_capacity(triangle_count);
    let mut incoming = vec![0_u32; triangle_count];
    for (index, triangle) in input.triangles.iter().enumerate() {
        let mut vertex_indices = [0_u32; 3];
        let mut has_invalid_vertex_index = false;
        for (slot, raw) in triangle.vertex_indices.iter().enumerate() {
            if *raw < 0 || *raw as usize >= vertex_count {
                diagnostics.push(error(format!(
                    "mesh {:08x} polygon {index}: vertex index {raw} out of range (0..{vertex_count})",
                    input.form_id
                )));
                // Explicit invalid-index sentinel (see
                // `PreparedNavPolygon::vertex_indices`'s doc comment) --
                // never silently collapse to a valid-looking `0`, which
                // would both hide the defect from consumers and risk a
                // spurious "degenerate triangle" warning below for the same
                // triangle.
                vertex_indices[slot] = u32::MAX;
                has_invalid_vertex_index = true;
            } else {
                vertex_indices[slot] = *raw as u32;
            }
        }
        if !has_invalid_vertex_index
            && (vertex_indices[0] == vertex_indices[1]
                || vertex_indices[1] == vertex_indices[2]
                || vertex_indices[0] == vertex_indices[2])
        {
            diagnostics.push(warning(format!(
                "mesh {:08x} polygon {index}: degenerate triangle (repeated vertex index)",
                input.form_id
            )));
        }

        let mut adjacency: [Option<u32>; 3] = [None; 3];
        for (slot, raw) in triangle.edge_neighbors.iter().enumerate() {
            if *raw < 0 {
                continue;
            }
            let neighbor = *raw as usize;
            if neighbor >= triangle_count {
                diagnostics.push(error(format!(
                    "mesh {:08x} polygon {index}: neighbor index {raw} out of range (0..{triangle_count})",
                    input.form_id
                )));
                continue;
            }
            adjacency[slot] = Some(neighbor as u32);
            incoming[neighbor] += 1;
        }

        polygons.push(PreparedNavPolygon {
            index: index as u32,
            vertex_indices,
            adjacency,
            flags: triangle.flags,
            is_water: triangle.flags & 0x0000_0200 != 0,
            is_preferred_pathing: triangle.flags & 0x0000_0040 != 0,
            contains_door: triangle.flags & 0x0000_0400 != 0,
            authored_external: EDGE_EXTERNAL_FLAGS.map(|bit| triangle.flags & bit != 0),
            // Every authored polygon starts walkable; the collision-derived
            // clearance pass (issue #153) flips this off prepare-side after
            // physics classification (see `navmesh::apply_nav_clearance`).
            walkable: true,
        });
    }

    for (index, count) in incoming.iter().enumerate() {
        if *count > 3 {
            diagnostics.push(warning(format!(
                "mesh {:08x} polygon {index}: non-manifold, referenced as a neighbour by {count} edge(s) (a triangle has only 3)",
                input.form_id
            )));
        }
    }

    for polygon in &polygons {
        for neighbor in polygon.adjacency.iter().flatten() {
            let neighbor_polygon = &polygons[*neighbor as usize];
            let symmetric = neighbor_polygon
                .adjacency
                .iter()
                .flatten()
                .any(|back| *back == polygon.index);
            if !symmetric {
                diagnostics.push(warning(format!(
                    "mesh {:08x} polygon {}: asymmetric adjacency to polygon {neighbor} (no reverse link)",
                    input.form_id, polygon.index
                )));
            }
        }
    }

    let islands = count_islands(&polygons);
    if islands > 1 {
        diagnostics.push(warning(format!(
            "mesh {:08x}: {islands} disconnected islands across {} polygon(s)",
            input.form_id,
            polygons.len()
        )));
    }

    let doors = input
        .doors
        .iter()
        .enumerate()
        .map(|(door_index, door)| {
            if door.triangle_index as usize >= triangle_count {
                diagnostics.push(error(format!(
                    "mesh {:08x} door {door_index}: triangle index {} out of range (0..{triangle_count})",
                    input.form_id, door.triangle_index
                )));
            }
            PreparedNavDoor {
                triangle_index: door.triangle_index,
                door_reference_form_id: door.door_reference_form_id,
            }
        })
        .collect::<Vec<_>>();

    let external_connections = input
        .external_connections
        .iter()
        .enumerate()
        .map(|(ext_index, connection)| {
            if connection.triangle_index as usize >= triangle_count {
                diagnostics.push(error(format!(
                    "mesh {:08x} external connection {ext_index}: triangle index {} out of range (0..{triangle_count})",
                    input.form_id, connection.triangle_index
                )));
            }
            if connection.target_navmesh_form_id.is_none() {
                diagnostics.push(error(format!(
                    "mesh {:08x} external connection {ext_index}: missing target NAVM FormID",
                    input.form_id
                )));
            }
            PreparedNavExternalConnection {
                triangle_index: connection.triangle_index,
                target_navmesh_form_id: connection.target_navmesh_form_id,
            }
        })
        .collect::<Vec<_>>();

    let cover_triangle_indices = input
        .cover_triangle_ids
        .iter()
        .enumerate()
        .filter_map(|(cover_index, raw)| {
            if *raw < 0 || *raw as usize >= triangle_count {
                diagnostics.push(warning(format!(
                    "mesh {:08x} cover entry {cover_index}: triangle index {raw} out of range (0..{triangle_count})",
                    input.form_id
                )));
                None
            } else {
                Some(*raw as u32)
            }
        })
        .collect::<Vec<_>>();

    let grid = navi_entries
        .iter()
        .find(|entry| entry.navmesh_form_id == Some(input.form_id))
        .map(|entry| PreparedNavGrid {
            x: entry.grid_x,
            y: entry.grid_y,
        });
    for entry in navi_entries
        .iter()
        .filter(|entry| entry.navmesh_form_id == Some(input.form_id))
    {
        if let Some(location) = entry.location_form_id
            && Some(location) != input.cell_form_id
        {
            diagnostics.push(warning(format!(
                "mesh {:08x}: NAVI NVMI location {location:08x} does not match this NAVM's owning cell",
                input.form_id
            )));
        }
    }

    let bounds = mesh_bounds(&vertices);

    (
        PreparedNavMesh {
            form_id: input.form_id,
            cell_form_id: input.cell_form_id,
            grid,
            vertices,
            polygons,
            doors,
            // Populated later by `navmesh::apply_derived_door_associations`
            // (issue #177), after the clearance pass has settled polygon
            // indices.
            derived_doors: Vec::new(),
            external_connections,
            cover_triangle_indices,
            bounds,
        },
        diagnostics,
    )
}

fn count_islands(polygons: &[PreparedNavPolygon]) -> usize {
    let n = polygons.len();
    if n == 0 {
        return 0;
    }
    let mut visited = vec![false; n];
    let mut islands = 0;
    for start in 0..n {
        if visited[start] {
            continue;
        }
        islands += 1;
        let mut stack = vec![start];
        visited[start] = true;
        while let Some(node) = stack.pop() {
            for neighbor in polygons[node].adjacency.iter().flatten() {
                let neighbor = *neighbor as usize;
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }
    }
    islands
}

fn mesh_bounds(vertices: &[[f32; 3]]) -> PreparedNavAabb {
    let Some(first) = vertices.first() else {
        return PreparedNavAabb::default();
    };
    let mut min = *first;
    let mut max = *first;
    for vertex in &vertices[1..] {
        for axis in 0..3 {
            min[axis] = min[axis].min(vertex[axis]);
            max[axis] = max[axis].max(vertex[axis]);
        }
    }
    PreparedNavAabb { min, max }
}

fn whole_graph_bounds(meshes: &[PreparedNavMesh]) -> PreparedNavAabb {
    let mut result: Option<PreparedNavAabb> = None;
    for mesh in meshes {
        if mesh.vertices.is_empty() {
            continue;
        }
        result = Some(match result {
            None => mesh.bounds,
            Some(acc) => PreparedNavAabb {
                min: [
                    acc.min[0].min(mesh.bounds.min[0]),
                    acc.min[1].min(mesh.bounds.min[1]),
                    acc.min[2].min(mesh.bounds.min[2]),
                ],
                max: [
                    acc.max[0].max(mesh.bounds.max[0]),
                    acc.max[1].max(mesh.bounds.max[1]),
                    acc.max[2].max(mesh.bounds.max[2]),
                ],
            },
        });
    }
    result.unwrap_or_default()
}

// ---------------------------------------------------------------------
// Artifact I/O
// ---------------------------------------------------------------------

use std::path::{Path, PathBuf};

use anyhow::Result;

use super::super::paths::fingerprint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NavGraphArtifact {
    pub(crate) relative_path: String,
    pub(crate) hash: String,
    pub(crate) reused: bool,
}

/// Writes the deterministic nav-graph artifact into the cell's own scene
/// directory (`scenes/<cell>/navmesh/navgraph.ron`), next to the retained
/// raw `*.navm.bin` sources. Per-cell like `write_actor_catalog` (not
/// fingerprint-keyed under `catalogs/`), since the graph embeds this cell's
/// NAVM data. A byte-identical existing file is left untouched and reported
/// as reused.
pub(crate) fn write_nav_graph(
    cache_dir: &Path,
    cell_form_id: u32,
    graph: &PreparedNavGraph,
) -> Result<NavGraphArtifact> {
    let relative = PathBuf::from("scenes")
        .join(format!("{cell_form_id:08x}"))
        .join("navmesh")
        .join("navgraph.ron");
    let path = cache_dir.join(&relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let serialized = ron::ser::to_string_pretty(graph, ron::ser::PrettyConfig::default())
        .map_err(|error| anyhow::anyhow!("failed to serialize nav graph: {error}"))?;
    let hash = fingerprint(serialized.as_bytes());
    let reused = std::fs::read(&path)
        .map(|existing| existing == serialized.as_bytes())
        .unwrap_or(false);
    if !reused {
        std::fs::write(&path, serialized)?;
    }
    Ok(NavGraphArtifact {
        relative_path: relative.to_string_lossy().replace('\\', "/"),
        hash,
        reused,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mesh(form_id: u32) -> NavGraphMeshInput {
        NavGraphMeshInput {
            form_id,
            ..NavGraphMeshInput::default()
        }
    }

    fn triangle(vertex_indices: [i32; 3], edge_neighbors: [i32; 3]) -> NavGraphTriangleInput {
        NavGraphTriangleInput {
            vertex_indices,
            edge_neighbors,
            flags: 0,
        }
    }

    /// Inverse of `to_bevy_position`: bevy metres -> raw source units.
    fn bevy_to_source(p: [f32; 3]) -> [f32; 3] {
        [p[0] / FO3_SCALE, -p[2] / FO3_SCALE, p[1] / FO3_SCALE]
    }

    /// A two-triangle rectangle (bevy metres, `corners` in `p0, p1, p2, p3`
    /// winding order) with the internal diagonal `p0-p2` correctly marked
    /// as a same-mesh neighbour, so only the outer four sides are boundary
    /// edges -- unlike a single right triangle (this module's portal tests'
    /// original shape), a quad's three *other* sides are each perpendicular
    /// to the pair of opposite long sides, so none of them can accidentally
    /// satisfy `validate_portal_candidate`'s opposing-direction check
    /// against an edge that faces one of those long sides, regardless of
    /// distance. `p0-p1` is boundary edge 0 of triangle 0; `p2-p3` is
    /// boundary edge 1 of triangle 1 -- the two long sides portal tests
    /// pick one of to face another mesh.
    fn quad_mesh(form_id: u32, corners: [[f32; 3]; 4]) -> NavGraphMeshInput {
        let mut m = mesh(form_id);
        m.vertices = corners
            .into_iter()
            .map(|bevy| NavGraphVertexInput {
                source: bevy_to_source(bevy),
            })
            .collect();
        m.triangles = vec![
            triangle([0, 1, 2], [-1, -1, 1]),
            triangle([0, 2, 3], [0, -1, -1]),
        ];
        m
    }

    /// Combines several same-`form_id` mesh pieces (typically `quad_mesh`
    /// outputs) into one `NavGraphMeshInput`, remapping each later piece's
    /// vertex/triangle indices past the earlier pieces' -- lets a test
    /// build one mesh out of several geometrically-separated quads (e.g.
    /// two candidate edges competing to face the same edge on another
    /// mesh) without their vertex/triangle indices colliding.
    fn combine_mesh_pieces(form_id: u32, pieces: Vec<NavGraphMeshInput>) -> NavGraphMeshInput {
        let mut combined = mesh(form_id);
        for piece in pieces {
            let vertex_offset = combined.vertices.len() as i32;
            let triangle_offset = combined.triangles.len() as i32;
            combined.vertices.extend(piece.vertices);
            combined
                .triangles
                .extend(piece.triangles.into_iter().map(|t| {
                    NavGraphTriangleInput {
                        vertex_indices: t
                            .vertex_indices
                            .map(|i| if i < 0 { i } else { i + vertex_offset }),
                        edge_neighbors: t
                            .edge_neighbors
                            .map(|i| if i < 0 { i } else { i + triangle_offset }),
                        flags: t.flags,
                    }
                }));
        }
        combined
    }

    #[test]
    fn revision_is_pinned() {
        assert_eq!(NAV_GRAPH_REVISION, "nav-graph-v8");
    }

    #[test]
    fn a_default_polygon_is_walkable() {
        // Rust-side `Default` must agree with the serde default: a synthetic
        // `PreparedNavPolygon { .., ..Default::default() }` (how other slices
        // build test graphs) must be walkable, or `mesh_inputs`'s walkable
        // filter would silently drop it.
        assert!(PreparedNavPolygon::default().walkable);
    }

    #[test]
    fn converts_a_known_vertex_to_bevy_metres() {
        let mut mesh = mesh(1);
        mesh.vertices.push(NavGraphVertexInput {
            source: [70.0, 140.0, -210.0],
        });
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.meshes[0].vertices[0], [1.0, -3.0, -2.0]);
    }

    #[test]
    fn meshes_are_sorted_by_form_id() {
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh(0x20), mesh(0x10)],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(
            graph.meshes.iter().map(|m| m.form_id).collect::<Vec<_>>(),
            vec![0x10, 0x20]
        );
    }

    #[test]
    fn builds_symmetric_adjacency_without_diagnostics() {
        let mut mesh = mesh(1);
        mesh.vertices = vec![
            NavGraphVertexInput { source: [0.0; 3] },
            NavGraphVertexInput {
                source: [70.0, 0.0, 0.0],
            },
            NavGraphVertexInput {
                source: [0.0, 70.0, 0.0],
            },
            NavGraphVertexInput {
                source: [70.0, 70.0, 0.0],
            },
        ];
        mesh.triangles = vec![
            triangle([0, 1, 2], [-1, 1, -1]),
            triangle([1, 3, 2], [-1, -1, 0]),
        ];
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert!(graph.diagnostics.is_empty(), "{:?}", graph.diagnostics);
        assert_eq!(graph.meshes[0].polygons[0].adjacency[1], Some(1));
        assert_eq!(graph.meshes[0].polygons[1].adjacency[2], Some(0));
    }

    #[test]
    fn detects_asymmetric_adjacency() {
        let mut mesh = mesh(1);
        mesh.vertices = vec![
            NavGraphVertexInput { source: [0.0; 3] },
            NavGraphVertexInput {
                source: [70.0, 0.0, 0.0],
            },
            NavGraphVertexInput {
                source: [0.0, 70.0, 0.0],
            },
            NavGraphVertexInput {
                source: [70.0, 70.0, 0.0],
            },
        ];
        // Triangle 0 claims triangle 1 as a neighbour; triangle 1 does not
        // claim triangle 0 back.
        mesh.triangles = vec![
            triangle([0, 1, 2], [-1, 1, -1]),
            triangle([1, 3, 2], [-1, -1, -1]),
        ];
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert!(
            graph
                .diagnostics
                .iter()
                .any(|d| d.message.contains("asymmetric adjacency"))
        );
    }

    #[test]
    fn detects_disconnected_islands() {
        let mut mesh = mesh(1);
        mesh.vertices = vec![
            NavGraphVertexInput { source: [0.0; 3] },
            NavGraphVertexInput {
                source: [70.0, 0.0, 0.0],
            },
            NavGraphVertexInput {
                source: [0.0, 70.0, 0.0],
            },
            NavGraphVertexInput {
                source: [700.0, 700.0, 0.0],
            },
            NavGraphVertexInput {
                source: [770.0, 700.0, 0.0],
            },
            NavGraphVertexInput {
                source: [700.0, 770.0, 0.0],
            },
        ];
        mesh.triangles = vec![
            triangle([0, 1, 2], [-1, -1, -1]),
            triangle([3, 4, 5], [-1, -1, -1]),
        ];
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert!(
            graph
                .diagnostics
                .iter()
                .any(|d| d.message.contains("disconnected islands"))
        );
    }

    #[test]
    fn detects_non_manifold_reverse_references() {
        let mut mesh = mesh(1);
        mesh.vertices = (0..8)
            .map(|i| NavGraphVertexInput {
                source: [i as f32 * 70.0, 0.0, 0.0],
            })
            .collect();
        // Four different triangles (1..=4) all claim triangle 0 as their
        // neighbour -- more references than triangle 0 has edges (3).
        mesh.triangles = vec![
            triangle([0, 1, 2], [-1, -1, -1]),
            triangle([1, 2, 3], [0, -1, -1]),
            triangle([2, 3, 4], [0, -1, -1]),
            triangle([3, 4, 5], [0, -1, -1]),
            triangle([4, 5, 6], [0, -1, -1]),
        ];
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert!(
            graph
                .diagnostics
                .iter()
                .any(|d| d.message.contains("non-manifold"))
        );
    }

    #[test]
    fn out_of_range_vertex_and_neighbor_indices_are_diagnosed_as_errors() {
        let mut mesh = mesh(1);
        mesh.vertices = vec![NavGraphVertexInput { source: [0.0; 3] }];
        mesh.triangles = vec![triangle([0, 5, 0], [9, -1, -1])];
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert!(
            graph
                .diagnostics
                .iter()
                .any(|d| d.severity == "error" && d.message.contains("vertex index 5"))
        );
        assert!(
            graph
                .diagnostics
                .iter()
                .any(|d| d.severity == "error" && d.message.contains("neighbor index 9"))
        );
        assert_eq!(graph.counters.diagnostics_error, 2);
    }

    #[test]
    fn invalid_vertex_indices_become_sentinel_without_degenerate_warning() {
        let mut mesh = mesh(1);
        mesh.vertices = vec![
            NavGraphVertexInput { source: [0.0; 3] },
            NavGraphVertexInput {
                source: [70.0, 0.0, 0.0],
            },
        ];
        // Slot 0 is valid; slot 1 is negative; slot 2 is out of range.
        mesh.triangles = vec![triangle([0, -1, 9], [-1, -1, -1])];
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);

        assert_eq!(
            graph.meshes[0].polygons[0].vertex_indices,
            [0, u32::MAX, u32::MAX]
        );

        let vertex_index_errors = graph
            .diagnostics
            .iter()
            .filter(|d| d.severity == "error" && d.message.contains("vertex index"))
            .count();
        assert_eq!(vertex_index_errors, 2, "{:?}", graph.diagnostics);
        assert_eq!(graph.counters.diagnostics_error, 2);

        assert!(
            !graph
                .diagnostics
                .iter()
                .any(|d| d.message.contains("degenerate triangle")),
            "{:?}",
            graph.diagnostics
        );
    }

    #[test]
    fn door_and_external_links_out_of_range_are_diagnosed() {
        let mut mesh = mesh(1);
        mesh.vertices = vec![
            NavGraphVertexInput { source: [0.0; 3] },
            NavGraphVertexInput {
                source: [70.0, 0.0, 0.0],
            },
            NavGraphVertexInput {
                source: [0.0, 70.0, 0.0],
            },
        ];
        mesh.triangles = vec![triangle([0, 1, 2], [-1, -1, -1])];
        mesh.doors.push(NavGraphDoorInput {
            door_reference_form_id: Some(0x99),
            triangle_index: 5,
        });
        mesh.external_connections.push(NavGraphExternalInput {
            target_navmesh_form_id: None,
            triangle_index: 0,
        });
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert!(
            graph
                .diagnostics
                .iter()
                .any(|d| d.message.contains("door 0") && d.message.contains("out of range"))
        );
        assert!(
            graph
                .diagnostics
                .iter()
                .any(|d| d.message.contains("missing target NAVM FormID"))
        );
    }

    #[test]
    fn navi_grid_is_attached_and_location_mismatch_is_diagnosed() {
        let mut mesh = mesh(0x500);
        mesh.cell_form_id = Some(0x10);
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh],
            navi_entries: vec![NavGraphNaviEntryInput {
                navmesh_form_id: Some(0x500),
                location_form_id: Some(0x99),
                grid_x: 3,
                grid_y: -4,
            }],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.meshes[0].grid, Some(PreparedNavGrid { x: 3, y: -4 }));
        assert!(
            graph
                .diagnostics
                .iter()
                .any(|d| d.message.contains("does not match this NAVM's owning cell"))
        );
    }

    /// Two quad meshes (`quad_mesh`, bevy metres), offset so their nearest
    /// long side is `gap` metres apart along Z -- the shape of a real FO3
    /// NAVM seam (see `MESH_MERGE_DISTANCE`'s doc comment), and (unlike a
    /// single right triangle) unambiguous under the full pairwise-candidate
    /// algorithm: every other boundary-edge pair between the two meshes is
    /// excluded by direction, distance, or lack of overlap -- verified by
    /// hand for exactly this geometry -- leaving exactly one accepted
    /// portal, `mesh_a`'s `p0-p1` (triangle 0) against `mesh_b`'s `q0-q1`
    /// (triangle 0).
    fn seam_meshes(gap: f32) -> Vec<NavGraphMeshInput> {
        let mesh_a = quad_mesh(
            0x10,
            [
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, -3.0],
                [0.0, 0.0, -3.0],
            ],
        );
        let mesh_b = quad_mesh(
            0x20,
            [
                [1.0, 0.0, gap],
                [0.0, 0.0, gap],
                [0.0, 0.0, gap + 3.0],
                [1.0, 0.0, gap + 3.0],
            ],
        );
        vec![mesh_a, mesh_b]
    }

    #[test]
    fn same_cell_meshes_with_a_near_boundary_seam_are_merged() {
        // 0.5 m gap -- well inside `MESH_MERGE_DISTANCE`, matching the real
        // FranklinMetro02 (0.09-0.9 m) measured gap this constant is sized
        // for.
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: seam_meshes(0.5),
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.mesh_merges.len(), 1, "{:?}", graph.mesh_merges);
        let merge = graph.mesh_merges[0];
        assert_eq!(merge.mesh_a_form_id, 0x10);
        assert_eq!(merge.mesh_b_form_id, 0x20);
        assert_eq!(graph.counters.mesh_merges, 1);
        // Issue #154 feature 1: edge identity and a real (non-degenerate,
        // non-zero-length) matched world-space interval on both sides.
        assert_ne!(merge.edge_a[0], merge.edge_a[1]);
        assert_ne!(merge.edge_b[0], merge.edge_b[1]);
        assert!(distance_sq(merge.interval_a[0], merge.interval_a[1]) > 1.0e-6);
        assert!(distance_sq(merge.interval_b[0], merge.interval_b[1]) > 1.0e-6);
        // No vertical drop for this flat synthetic seam.
        assert!((merge.interval_a[0][1] - merge.interval_b[0][1]).abs() < 1.0e-4);
        assert!((merge.interval_a[1][1] - merge.interval_b[1][1]).abs() < 1.0e-4);
    }

    #[test]
    fn a_conflicting_candidate_is_rejected_leaving_one_accepted_interval() {
        // Real FO3 data shape (the review finding this issue originally
        // fixed): mesh_b offers *two* separate edges (`winner`, `loser`)
        // both facing mesh_a's single long edge; `loser`'s candidate
        // interval on mesh_a's edge is a strict *subset* of `winner`'s
        // (shorter overlap, so it sorts after `winner`), so the resolution
        // pass (`compute_mesh_merges`'s doc comment) rejects it as
        // conflicting rather than silently producing a second (or,
        // pre-#154, an accidentally-deduplicated-away) merge. This is
        // deliberately *not* framed as "not the nearest edge" (a review
        // correction replaced single-nearest-only matching with
        // reciprocal, non-overlapping *interval* matching so one long edge
        // can legitimately face several short ones -- see
        // `a_long_edge_legitimately_matches_two_short_collinear_edges`
        // below for that one-to-many case).
        let mesh_a = quad_mesh(
            0x10,
            [
                [0.0, 0.0, 0.0],
                [2.0, 0.0, 0.0],
                [2.0, 0.0, -3.0],
                [0.0, 0.0, -3.0],
            ],
        );
        let gap = 0.5;
        let winner = quad_mesh(
            0x20,
            [
                [2.0, 0.0, gap],
                [0.0, 0.0, gap],
                [0.0, 0.0, gap + 3.0],
                [2.0, 0.0, gap + 3.0],
            ],
        );
        let loser = quad_mesh(
            0x20,
            [
                [1.5, 0.0, gap],
                [0.5, 0.0, gap],
                [0.5, 0.0, gap + 3.0],
                [1.5, 0.0, gap + 3.0],
            ],
        );
        let mesh_b = combine_mesh_pieces(0x20, vec![winner, loser]);

        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh_a, mesh_b],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.mesh_merges.len(), 1, "{:?}", graph.mesh_merges);
        // The winner's full-length interval, not the loser's shorter one.
        assert!(
            distance_sq(
                graph.mesh_merges[0].interval_a[0],
                graph.mesh_merges[0].interval_a[1]
            ) > 3.0,
            "{:?}",
            graph.mesh_merges
        );
        assert!(
            graph.diagnostics.iter().any(|d| d
                .message
                .contains("overlaps another accepted portal interval")),
            "{:?}",
            graph.diagnostics
        );
        assert!(graph.counters.mesh_merges_rejected >= 1);
    }

    #[test]
    fn a_long_edge_legitimately_matches_two_short_collinear_edges() {
        // Review correction (issue #154): one long boundary edge may face
        // several shorter tessellated edges on the other mesh -- a real
        // FO3 shape single-nearest-only matching could not represent. Two
        // short mesh_b edges, collinear and each covering half of one long
        // mesh_a edge, must both become accepted (non-overlapping)
        // portals, not just the "nearest" one.
        let mesh_a = quad_mesh(
            0x10,
            [
                [0.0, 0.0, 0.0],
                [2.0, 0.0, 0.0],
                [2.0, 0.0, -3.0],
                [0.0, 0.0, -3.0],
            ],
        );
        let gap = 0.3;
        // The far half of mesh_a's edge (x in [1, 2]).
        let far_half = quad_mesh(
            0x20,
            [
                [2.0, 0.0, gap],
                [1.0, 0.0, gap],
                [1.0, 0.0, gap + 3.0],
                [2.0, 0.0, gap + 3.0],
            ],
        );
        // The near half (x in [0, 1]).
        let near_half = quad_mesh(
            0x20,
            [
                [1.0, 0.0, gap],
                [0.0, 0.0, gap],
                [0.0, 0.0, gap + 3.0],
                [1.0, 0.0, gap + 3.0],
            ],
        );
        let mesh_b = combine_mesh_pieces(0x20, vec![far_half, near_half]);

        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh_a, mesh_b],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.mesh_merges.len(), 2, "{:?}", graph.mesh_merges);
        assert!(
            graph
                .mesh_merges
                .iter()
                .all(|merge| merge.mesh_a_form_id == 0x10 && merge.triangle_a == 0),
            "both portals share the same long mesh_a edge: {:?}",
            graph.mesh_merges
        );
        let mut triangle_bs: Vec<u32> = graph
            .mesh_merges
            .iter()
            .map(|merge| merge.triangle_b)
            .collect();
        triangle_bs.sort_unstable();
        triangle_bs.dedup();
        assert_eq!(
            triangle_bs.len(),
            2,
            "the two portals must come from two distinct mesh_b triangles: {:?}",
            graph.mesh_merges
        );
    }

    #[test]
    fn adversarial_close_parallel_walls_do_not_portal() {
        // Adversarial fixture: two close, *parallel* (not opposing) walls
        // -- the shape of two independently-authored corridor edges that
        // merely happen to run near and alongside each other, never a real
        // doorway/seam. `seam_meshes`' own geometry, but mesh_b's facing
        // edge is authored to run the *same* +X direction as mesh_a's
        // instead of opposing it (its two long sides are swapped so the
        // near one -- not the far one -- runs +X).
        let mesh_a = quad_mesh(
            0x10,
            [
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, -3.0],
                [0.0, 0.0, -3.0],
            ],
        );
        let gap = 0.5;
        let mesh_b = quad_mesh(
            0x20,
            [
                [0.0, 0.0, gap],
                [1.0, 0.0, gap],
                [1.0, 0.0, gap + 3.0],
                [0.0, 0.0, gap + 3.0],
            ],
        );

        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh_a, mesh_b],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert!(graph.mesh_merges.is_empty(), "{:?}", graph.mesh_merges);
        assert!(
            graph
                .diagnostics
                .iter()
                .any(|d| d.message.contains("not opposing enough")),
            "{:?}",
            graph.diagnostics
        );
    }

    #[test]
    fn a_vertically_offset_pair_is_still_accepted_prepare_side_with_the_drop_recorded() {
        // Review correction (issue #154): prepare-time validation must not
        // bake an agent-class assumption (step height) into the universal
        // prepared graph. `seam_meshes`' own geometry, but mesh_b's quad is
        // additionally raised 1 m in bevy Y -- adversarial fixture
        // "vertically stacked floors whose edges overlap in XZ". The pair
        // is still perfectly opposing and perfectly overlapping in XZ, so
        // it is geometrically a valid *portal candidate* prepare-side; the
        // interval it records simply carries that 1 m drop for
        // `viewer::nav::landmass_graph`'s runtime, agent-aware check to act
        // on (see that module's `MERGE_PORTAL_STEP_HEIGHT`).
        let mesh_a = quad_mesh(
            0x10,
            [
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, -3.0],
                [0.0, 0.0, -3.0],
            ],
        );
        let gap = 0.5;
        let mesh_b = quad_mesh(
            0x20,
            [
                [1.0, 1.0, gap],
                [0.0, 1.0, gap],
                [0.0, 1.0, gap + 3.0],
                [1.0, 1.0, gap + 3.0],
            ],
        );

        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh_a, mesh_b],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.mesh_merges.len(), 1, "{:?}", graph.mesh_merges);
        let merge = graph.mesh_merges[0];
        let drop = (merge.interval_a[0][1] - merge.interval_b[0][1]).abs();
        assert!(
            (drop - 1.0).abs() < 1.0e-4,
            "expected a 1 m drop, got {drop}"
        );
    }

    #[test]
    fn adversarial_reversed_winding_that_would_otherwise_match_is_rejected() {
        // Adversarial fixture: `seam_meshes`' own geometry, but mesh_b's
        // quad is built with its corners in the reverse order a mis-wound
        // source triangle would produce (`p1, p0, p3, p2` instead of `p0,
        // p1, p2, p3`) -- the same four physical corners and the same
        // internal-diagonal adjacency shape, just with every edge's
        // start/end (and therefore direction) flipped. What would
        // correctly oppose mesh_a's edge instead runs the same direction
        // and must be rejected exactly like any other non-opposing pair,
        // not silently "corrected" by inferring a canonical winding.
        let mesh_a = quad_mesh(
            0x10,
            [
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, -3.0],
                [0.0, 0.0, -3.0],
            ],
        );
        let gap = 0.5;
        let mesh_b = quad_mesh(
            0x20,
            [
                [0.0, 0.0, gap],
                [1.0, 0.0, gap],
                [1.0, 0.0, gap + 3.0],
                [0.0, 0.0, gap + 3.0],
            ],
        );

        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh_a, mesh_b],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert!(graph.mesh_merges.is_empty(), "{:?}", graph.mesh_merges);
    }

    #[test]
    fn same_cell_meshes_far_apart_are_not_merged() {
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: seam_meshes(1_000.0),
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert!(graph.mesh_merges.is_empty(), "{:?}", graph.mesh_merges);
        assert_eq!(graph.counters.mesh_merges, 0);
    }

    #[test]
    fn mesh_merges_are_deterministic_across_calls() {
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: seam_meshes(0.5),
            ..NavGraphInputs::default()
        };
        let first = build_nav_graph(&inputs).mesh_merges;
        let second = build_nav_graph(&inputs).mesh_merges;
        assert_eq!(first, second);
    }

    #[test]
    fn a_single_mesh_never_merges_with_itself() {
        let mut single = mesh(0x10);
        single.vertices = vec![
            NavGraphVertexInput { source: [0.0; 3] },
            NavGraphVertexInput {
                source: [70.0, 0.0, 0.0],
            },
            NavGraphVertexInput {
                source: [0.0, 0.0, 70.0],
            },
        ];
        single.triangles = vec![triangle([0, 1, 2], [-1, -1, -1])];
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![single],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert!(graph.mesh_merges.is_empty());
    }

    #[test]
    fn whole_graph_bounds_cover_every_mesh() {
        let mut mesh_a = mesh(0x10);
        mesh_a.vertices = vec![NavGraphVertexInput {
            source: [0.0, 0.0, 0.0],
        }];
        let mut mesh_b = mesh(0x20);
        mesh_b.vertices = vec![NavGraphVertexInput {
            source: [700.0, 700.0, -700.0],
        }];
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh_a, mesh_b],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.bounds.min, [0.0, -10.0, -10.0]);
        assert_eq!(graph.bounds.max, [10.0, 0.0, 0.0]);
    }

    // -------------------------------------------------------------
    // Authored NVTR external-edge evidence for portals (issue #156
    // feature 2).
    // -------------------------------------------------------------

    #[test]
    fn a_merge_authored_on_both_sides_is_marked_authored_evidence() {
        let mut meshes = seam_meshes(0.5);
        meshes[0].triangles[0].flags |= EDGE_EXTERNAL_FLAGS[0];
        meshes[1].triangles[0].flags |= EDGE_EXTERNAL_FLAGS[0];
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes,
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.mesh_merges.len(), 1, "{:?}", graph.mesh_merges);
        assert!(
            graph.mesh_merges[0].authored_evidence,
            "{:?}",
            graph.mesh_merges
        );
        assert_eq!(graph.counters.mesh_merges_authored, 1);
        assert_eq!(graph.counters.mesh_merges_geometric, 0);
        assert_eq!(graph.counters.merge_candidates_authored, 1);
        assert_eq!(graph.counters.merge_candidates_geometric, 0);
    }

    #[test]
    fn a_merge_authored_on_only_one_side_is_still_marked_authored_evidence() {
        // OR semantics: the two NAVM records are independently authored, so
        // there is no guarantee both sides of a real seam carry the flag.
        let mut meshes = seam_meshes(0.5);
        meshes[0].triangles[0].flags |= EDGE_EXTERNAL_FLAGS[0];
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes,
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.mesh_merges.len(), 1, "{:?}", graph.mesh_merges);
        assert!(
            graph.mesh_merges[0].authored_evidence,
            "{:?}",
            graph.mesh_merges
        );
        assert_eq!(graph.counters.mesh_merges_authored, 1);
    }

    #[test]
    fn a_purely_geometric_merge_is_not_marked_authored_evidence() {
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: seam_meshes(0.5),
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.mesh_merges.len(), 1, "{:?}", graph.mesh_merges);
        assert!(
            !graph.mesh_merges[0].authored_evidence,
            "{:?}",
            graph.mesh_merges
        );
        assert_eq!(graph.counters.mesh_merges_authored, 0);
        assert_eq!(graph.counters.mesh_merges_geometric, 1);
        assert_eq!(graph.counters.merge_candidates_authored, 0);
        assert_eq!(graph.counters.merge_candidates_geometric, 1);
    }

    #[test]
    fn an_authored_candidate_is_prioritized_over_a_longer_purely_geometric_conflicting_candidate() {
        // Issue #156 feature 2: authored `NVTR` external-edge evidence
        // outranks pure geometric overlap length when two candidates
        // conflict on the same edge -- the mirror image of
        // `a_conflicting_candidate_is_rejected_leaving_one_accepted_interval`
        // (identical geometry: a shorter `loser` candidate and a longer
        // `winner` candidate both facing `mesh_a`'s single long edge), but
        // this time `loser` carries the authored flag and `winner` does
        // not, so `loser` wins despite its shorter overlap.
        let mesh_a = quad_mesh(
            0x10,
            [
                [0.0, 0.0, 0.0],
                [2.0, 0.0, 0.0],
                [2.0, 0.0, -3.0],
                [0.0, 0.0, -3.0],
            ],
        );
        let gap = 0.5;
        let winner = quad_mesh(
            0x20,
            [
                [2.0, 0.0, gap],
                [0.0, 0.0, gap],
                [0.0, 0.0, gap + 3.0],
                [2.0, 0.0, gap + 3.0],
            ],
        );
        let mut loser = quad_mesh(
            0x20,
            [
                [1.5, 0.0, gap],
                [0.5, 0.0, gap],
                [0.5, 0.0, gap + 3.0],
                [1.5, 0.0, gap + 3.0],
            ],
        );
        loser.triangles[0].flags |= EDGE_EXTERNAL_FLAGS[0];
        let mesh_b = combine_mesh_pieces(0x20, vec![winner, loser]);

        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh_a, mesh_b],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.mesh_merges.len(), 1, "{:?}", graph.mesh_merges);
        let merge = graph.mesh_merges[0];
        assert!(merge.authored_evidence, "{merge:?}");
        // `loser`'s own triangle (combined index 2, since `winner`
        // contributed triangles 0/1 first) is the one accepted, not
        // `winner`'s (0), despite its shorter overlap.
        assert_eq!(merge.triangle_b, 2, "{:?}", graph.mesh_merges);
        assert_eq!(graph.counters.mesh_merges_authored, 1);
        assert_eq!(graph.counters.mesh_merges_geometric, 0);
        assert!(
            graph.diagnostics.iter().any(|d| d
                .message
                .contains("overlaps another accepted portal interval")),
            "{:?}",
            graph.diagnostics
        );
    }

    // -------------------------------------------------------------
    // NVEX/NVCI correlation (issue #156 feature 3): correlation-only, no
    // runtime behavior.
    // -------------------------------------------------------------

    fn single_triangle_mesh(form_id: u32) -> NavGraphMeshInput {
        let mut m = mesh(form_id);
        m.vertices = vec![
            NavGraphVertexInput { source: [0.0; 3] },
            NavGraphVertexInput {
                source: [70.0, 0.0, 0.0],
            },
            NavGraphVertexInput {
                source: [0.0, 70.0, 0.0],
            },
        ];
        m.triangles = vec![triangle([0, 1, 2], [-1, -1, -1])];
        m
    }

    #[test]
    fn nvex_targets_are_split_between_inside_and_outside_this_cells_navm_set() {
        let mut mesh_a = single_triangle_mesh(0x10);
        mesh_a.external_connections = vec![
            NavGraphExternalInput {
                target_navmesh_form_id: Some(0x10), // this cell's own NAVM
                triangle_index: 0,
            },
            NavGraphExternalInput {
                target_navmesh_form_id: Some(0x999), // not in this cell
                triangle_index: 0,
            },
        ];
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh_a],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.counters.nvex_targets_inside_cell, 1);
        assert_eq!(graph.counters.nvex_targets_outside_cell, 1);
        assert!(
            graph
                .diagnostics
                .iter()
                .any(|d| d.severity == "info" && d.message.contains("NVEX correlation")),
            "{:?}",
            graph.diagnostics
        );
    }

    #[test]
    fn nvci_entries_are_correlated_against_this_cells_doors_and_navmeshes() {
        let mut mesh_a = single_triangle_mesh(0x10);
        mesh_a.doors.push(NavGraphDoorInput {
            door_reference_form_id: Some(0x99),
            triangle_index: 0,
        });

        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![mesh_a],
            navi_correlations: vec![NavGraphNaviCorrelationInput {
                leading_navmesh_form_id: Some(0x10), // matches this cell's own NAVM
                entries: vec![
                    NavGraphNaviCorrelationEntryInput {
                        navmesh_form_id: Some(0x10),        // matches
                        other_navmesh_form_id: Some(0x999), // does not match
                        door_form_id: Some(0x99),           // matches this cell's own door
                    },
                    NavGraphNaviCorrelationEntryInput {
                        navmesh_form_id: Some(0x999),
                        other_navmesh_form_id: Some(0x888),
                        door_form_id: Some(0x111), // does not match
                    },
                ],
            }],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.counters.nvci_subrecords, 1);
        assert_eq!(graph.counters.nvci_entries, 2);
        assert_eq!(graph.counters.nvci_door_matches, 1);
        // leading (0x10, matches) + entry1.navmesh_form_id (0x10, matches) =
        // 2 matches; entry1.other_navmesh_form_id (0x999),
        // entry2.navmesh_form_id (0x999), entry2.other_navmesh_form_id
        // (0x888) do not.
        assert_eq!(graph.counters.nvci_navmesh_matches, 2);
        assert!(
            graph
                .diagnostics
                .iter()
                .any(|d| d.severity == "info" && d.message.contains("NVCI correlation")),
            "{:?}",
            graph.diagnostics
        );
    }

    #[test]
    fn no_nvex_or_nvci_data_produces_no_correlation_counts_or_diagnostics() {
        let inputs = NavGraphInputs {
            cell_form_id: 0x10,
            meshes: vec![single_triangle_mesh(0x10)],
            ..NavGraphInputs::default()
        };
        let graph = build_nav_graph(&inputs);
        assert_eq!(graph.counters.nvex_targets_inside_cell, 0);
        assert_eq!(graph.counters.nvex_targets_outside_cell, 0);
        assert_eq!(graph.counters.nvci_subrecords, 0);
        assert_eq!(graph.counters.nvci_entries, 0);
        assert!(
            !graph
                .diagnostics
                .iter()
                .any(|d| d.message.contains("correlation")),
            "{:?}",
            graph.diagnostics
        );
    }
}
