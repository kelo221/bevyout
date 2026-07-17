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

use serde::{Deserialize, Serialize};

use super::super::paths::FO3_SCALE;

/// Bump whenever the graph asset shape changes, even when new fields are
/// serde-defaulted, per the `ACTOR_CATALOG_REVISION`/`ITEM_CATALOG_REVISION`
/// precedent.
pub(crate) const NAV_GRAPH_REVISION: &str = "nav-graph-v2";

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

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct NavGraphInputs {
    pub(crate) cell_form_id: u32,
    pub(crate) meshes: Vec<NavGraphMeshInput>,
    pub(crate) navi_entries: Vec<NavGraphNaviEntryInput>,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
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
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct PreparedNavDoor {
    pub(crate) triangle_index: u32,
    pub(crate) door_reference_form_id: Option<u32>,
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
/// `PreparedNavMesh`es (issue #113, M4 wave 4 feature 2). `NAVI`'s `NVMI`
/// tail (decoded in `openmw_esm4::navmesh`) does not carry OpenMW's
/// TES4/TES5-style merged-navmesh FormID arrays for real FO3 data (see that
/// module's doc comment and `NOTICE.md` for the byte-level verification), so
/// this connection is derived purely from geometry instead: `triangle_a`'s
/// boundary edge (an edge with no same-mesh neighbour) sits within
/// [`MESH_MERGE_DISTANCE`] of `triangle_b`'s boundary edge. Consumed by
/// `viewer::nav::landmass_graph` to build a walk-through animation link when
/// landmass's own island-boundary linking does not connect the two meshes
/// (real FO3 seams do not share exact vertex positions -- see that module's
/// doc comment).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct PreparedNavMeshMerge {
    pub(crate) mesh_a_form_id: u32,
    pub(crate) triangle_a: u32,
    pub(crate) mesh_b_form_id: u32,
    pub(crate) triangle_b: u32,
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
    pub(crate) diagnostics_warning: usize,
    pub(crate) diagnostics_error: usize,
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
    let mesh_merges = compute_mesh_merges(&meshes);
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
        diagnostics_warning,
        diagnostics_error,
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

struct BoundaryEdge {
    triangle_index: u32,
    midpoint: [f32; 3],
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
                midpoint: [
                    (va[0] + vb[0]) / 2.0,
                    (va[1] + vb[1]) / 2.0,
                    (va[2] + vb[2]) / 2.0,
                ],
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

/// Derives same-cell cross-mesh connections between every pair of `meshes`
/// (already sorted by `form_id`) by matching each boundary edge in the
/// lower-`form_id` mesh of a pair to its nearest boundary edge in the
/// higher-`form_id` mesh, keeping the match only when within
/// [`MESH_MERGE_DISTANCE`]. One-directional matching (from the lower
/// `form_id` mesh) keeps the connection count bounded by that mesh's own
/// boundary-edge count rather than exploding combinatorially; deterministic
/// (`meshes` is already form_id-sorted, `dedup` removes exact repeats), and
/// never panics (invalid vertex indices are skipped, not indexed).
fn compute_mesh_merges(meshes: &[PreparedNavMesh]) -> Vec<PreparedNavMeshMerge> {
    let threshold_sq = MESH_MERGE_DISTANCE * MESH_MERGE_DISTANCE;
    let mut merges = Vec::new();
    for a_index in 0..meshes.len() {
        for b_index in (a_index + 1)..meshes.len() {
            let mesh_a = &meshes[a_index];
            let mesh_b = &meshes[b_index];
            let edges_a = boundary_edges(mesh_a);
            let edges_b = boundary_edges(mesh_b);
            for edge_a in &edges_a {
                let mut best: Option<(f32, &BoundaryEdge)> = None;
                for edge_b in &edges_b {
                    let d = distance_sq(edge_a.midpoint, edge_b.midpoint);
                    if best.is_none_or(|(best_d, _)| d < best_d) {
                        best = Some((d, edge_b));
                    }
                }
                if let Some((d, edge_b)) = best
                    && d <= threshold_sq
                {
                    merges.push(PreparedNavMeshMerge {
                        mesh_a_form_id: mesh_a.form_id,
                        triangle_a: edge_a.triangle_index,
                        mesh_b_form_id: mesh_b.form_id,
                        triangle_b: edge_b.triangle_index,
                    });
                }
            }
        }
    }
    merges.sort_by_key(|merge| {
        (
            merge.mesh_a_form_id,
            merge.triangle_a,
            merge.mesh_b_form_id,
            merge.triangle_b,
        )
    });
    merges.dedup();
    merges
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

    #[test]
    fn revision_is_pinned() {
        assert_eq!(NAV_GRAPH_REVISION, "nav-graph-v2");
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

    /// Two meshes with a one-triangle-each square, offset so their nearest
    /// edge is `gap` metres apart along x -- the shape of a real FO3 NAVM
    /// seam (see `MESH_MERGE_DISTANCE`'s doc comment).
    fn seam_meshes(gap: f32) -> Vec<NavGraphMeshInput> {
        let mut mesh_a = mesh(0x10);
        mesh_a.vertices = vec![
            NavGraphVertexInput { source: [0.0; 3] },
            NavGraphVertexInput {
                source: [70.0, 0.0, 0.0],
            },
            NavGraphVertexInput {
                source: [0.0, 0.0, 70.0],
            },
        ];
        mesh_a.triangles = vec![triangle([0, 1, 2], [-1, -1, -1])];

        let offset = 70.0 + gap / FO3_SCALE;
        let mut mesh_b = mesh(0x20);
        mesh_b.vertices = vec![
            NavGraphVertexInput {
                source: [offset, 0.0, 0.0],
            },
            NavGraphVertexInput {
                source: [offset + 70.0, 0.0, 0.0],
            },
            NavGraphVertexInput {
                source: [offset, 0.0, 70.0],
            },
        ];
        mesh_b.triangles = vec![triangle([0, 1, 2], [-1, -1, -1])];
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
}
