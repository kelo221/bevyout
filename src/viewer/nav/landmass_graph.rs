//! Pure `bevy_landmass` conversion (issue #112, M4 wave 3 spike): prepared
//! per-cell nav-graph data (issue #111's `PreparedNavGraph`) -> a validated
//! `bevy_landmass` 3D navigation mesh, plus a deterministic door-link
//! descriptor list and a `landmass` agent-state -> project-native status
//! mapping.
//!
//! Deliberately decoupled from `vsa::prepare::nav_graph::PreparedNavGraph`:
//! this file is included verbatim by `tests/features.rs` via `#[path]` (see
//! that file's module doc comment -- `src/lib.rs` keeps `vsa`/`viewer`
//! private, so the integration-test binary cannot `use bevyout::viewer::...`
//! and instead compiles the exact same source in via `#[path]`), and that
//! `#[path]` inclusion has no access to the `vsa` module tree from this
//! file's real directory depth (`src/viewer/nav/`, three levels from crate
//! root, versus `vsa::prepare::nav_graph`'s two -- there is no shared
//! relative-`super::` path that resolves in both trees, unlike
//! `vsa::prepare::nav_graph` itself reusing `vsa::paths` via `super::super`).
//! So this module takes its own small plain-data `MeshInput`/`PolygonInput`/
//! `DoorInput` shape instead of `PreparedNavMesh`/`PreparedNavPolygon`/
//! `PreparedNavDoor` directly; `nav/mod.rs` (Bevy-visible, not
//! cucumber-included) does the trivial field-for-field boundary conversion,
//! exactly the shape `vsa::prepare::nav_graph.rs` itself uses to decouple
//! from `openmw_esm4`'s raw record types.
//!
//! No `bevy` (full engine) import: `bevy_landmass`/`glam` types are used
//! directly instead, per the wave plan's explicit allowance ("Add `landmass`
//! directly only if the pure conversion module needs types `bevy_landmass`
//! does not re-export"). In practice this module needs `bevy_landmass`
//! itself, not the plain `landmass` crate: `bevy_landmass::NavigationMesh3d`
//! (a `landmass::NavigationMesh<bevy_landmass::coords::ThreeD>` alias) is
//! the type the runtime side (`nav/agent.rs`) needs back out of this
//! conversion to build an `Archipelago3d`/`Island3dBundle`, and there is no
//! way to convert a `landmass::NavigationMesh<MyOwnCoordinateSystem>` into
//! that type after the fact -- `ThreeD` is a concrete marker type, not just
//! a shape. `bevy_landmass`'s `coords`/`nav_mesh`/`agent` split keeps this
//! import free of any ECS/`Component`/`System`/`World` type, so it stays
//! pure data-in-data-out and testable without spinning up a `World`.

use std::collections::BTreeSet;

use bevy_landmass::{AgentState, NavigationMesh3d, ValidNavigationMesh3d, ValidationError};
use glam::Vec3;

use super::erosion_policy;

// ---------------------------------------------------------------------
// Conversion inputs (boundary conversion from `PreparedNavGraph` happens in
// `nav/mod.rs`)
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct MeshInput {
    pub(crate) form_id: u32,
    /// Bevy metres (already converted at prepare time by #111's
    /// `to_bevy_position`); no further coordinate conversion happens here.
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) polygons: Vec<PolygonInput>,
    pub(crate) doors: Vec<DoorInput>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct PolygonInput {
    pub(crate) index: u32,
    pub(crate) vertex_indices: [u32; 3],
    pub(crate) is_water: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct DoorInput {
    pub(crate) triangle_index: u32,
    pub(crate) door_reference_form_id: Option<u32>,
}

// ---------------------------------------------------------------------
// Conversion diagnostics
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConversionDiagnostic {
    pub(crate) severity: Severity,
    pub(crate) message: String,
}

impl ConversionDiagnostic {
    fn warning(message: String) -> Self {
        Self {
            severity: Severity::Warning,
            message,
        }
    }

    fn error(message: String) -> Self {
        Self {
            severity: Severity::Error,
            message,
        }
    }
}

/// Result of converting one [`MeshInput`] into a validated landmass mesh.
/// `nav_mesh` is `None` when every polygon was excluded (all water, all
/// invalid/degenerate, or validation failed both winding attempts) --
/// `nav/agent.rs` simply skips spawning an island for that mesh rather than
/// treating it as fatal, mirroring `nav_overlay.rs`'s empty-mesh skip.
#[derive(Debug, Default)]
pub(crate) struct BuildResult {
    pub(crate) nav_mesh: Option<ValidNavigationMesh3d>,
    pub(crate) diagnostics: Vec<ConversionDiagnostic>,
}

/// Protected (seam/portal) edges for `mesh`'s own erosion pass (issue #136
/// follow-up): every edge of a triangle that is either a door-link/
/// travel-door attachment (`mesh.doors`, always per-mesh already) or a
/// cross-mesh merge participant naming this mesh's own `form_id`
/// (`merges`, graph-wide -- only entries touching `mesh.form_id`
/// contribute `triangle_a`/`triangle_b` respectively). See
/// `erosion_policy`'s module doc comment for why these edges must never
/// move during erosion, and why the whole triangle's three edges are
/// protected rather than trying to isolate exactly one "seam" edge (this
/// project has no data that says which single edge of a merge/door
/// triangle actually touches the other mesh, only that the triangle as a
/// whole is the attachment point).
pub(crate) fn protected_edges_for_mesh(mesh: &MeshInput, merges: &[MergeInput]) -> Vec<(u32, u32)> {
    let mut protected_triangle_indices: BTreeSet<u32> = BTreeSet::new();
    for door in &mesh.doors {
        protected_triangle_indices.insert(door.triangle_index);
    }
    for merge in merges {
        if merge.mesh_a_form_id == mesh.form_id {
            protected_triangle_indices.insert(merge.triangle_a);
        }
        if merge.mesh_b_form_id == mesh.form_id {
            protected_triangle_indices.insert(merge.triangle_b);
        }
    }
    let mut protected_edges: Vec<(u32, u32)> = Vec::new();
    for polygon in &mesh.polygons {
        if protected_triangle_indices.contains(&polygon.index) {
            let [a, b, c] = polygon.vertex_indices;
            protected_edges.push((a, b));
            protected_edges.push((b, c));
            protected_edges.push((c, a));
        }
    }
    protected_edges
}

/// Converts `mesh` into a validated `bevy_landmass` navigation mesh.
///
/// Non-walkable exclusion: `is_water` polygons are dropped rather than
/// mapped to a distinct type index. NAVM triangle flags decoded by #111
/// carry no separate "disabled"/ground bit at this layer (only
/// `is_water`/`is_preferred_pathing`/`contains_door` are extracted) -- water
/// is the only flag clearly meaning "a ground-walking humanoid should not
/// path across this triangle" in Fallout 3, so it stands in for
/// "non-walkable" for this spike's polygon-type-index question. A distinct
/// type index (rather than exclusion) would let a future wave give water a
/// non-infinite cost for wading, which this spike does not need.
///
/// Winding: `bevy_landmass`'s `ThreeD` coordinate system always reverses
/// polygon vertex order internally before storing (`FLIP_POLYGONS = true`,
/// compensating for the chirality difference between Bevy's Y-up and
/// landmass's internal Z-up "standard" coordinates) -- so this function does
/// not need to guess a fixed convention. It tries the polygon order exactly
/// as authored by #111's `PreparedNavPolygon::vertex_indices` first; if
/// `validate()` rejects it (concave/clockwise), it retries once with every
/// included polygon's vertex order reversed, and only then gives up. This
/// keeps both real Fallout NAVM winding conventions supported without
/// requiring advance knowledge of which one the source data uses, fixed
/// once per mesh (a mix of the two within one mesh would corrupt
/// `validate()`'s doubly-connected-edge adjacency detection).
///
/// Erosion (issue #136): after the water/invalid/degenerate exclusions
/// above, walkable-boundary vertices are moved inward by the agent radius
/// via `erosion_policy::erode` before either winding attempt -- see that
/// module's doc comment for why moving positions (not polygon topology)
/// keeps this safe against disconnecting the mesh, and for the
/// corridor-pinch fallback that keeps narrow corridors from inverting.
///
/// `merges` (issue #136 follow-up, real-data regression on a two-mesh
/// cell): every `MergeInput` touching `mesh.form_id` identifies one of
/// this mesh's own triangles as a seam/portal to another mesh, not a
/// wall -- eroding it independently on both sides of the seam opened a
/// real gap where a generated animation link used to connect the two
/// islands. Door-triangle vertices (`mesh.doors`, off-mesh link
/// attachment points for door links/travel doors) get the same
/// protection. See `erosion_policy`'s module doc comment for the
/// "protected edges" rule this feeds.
pub(crate) fn build_navigation_mesh(mesh: &MeshInput, merges: &[MergeInput]) -> BuildResult {
    let mut diagnostics = Vec::new();
    let vertex_count = mesh.vertices.len();

    let mut included_polygons: Vec<&PolygonInput> = Vec::new();
    for polygon in &mesh.polygons {
        if polygon.is_water {
            continue;
        }
        let invalid = polygon
            .vertex_indices
            .iter()
            .any(|&index| index as usize >= vertex_count);
        if invalid {
            diagnostics.push(ConversionDiagnostic::error(format!(
                "mesh {:08x} polygon {}: invalid vertex index, excluded from navigation mesh",
                mesh.form_id, polygon.index
            )));
            continue;
        }
        let [a, b, c] = polygon.vertex_indices;
        if a == b || b == c || a == c {
            diagnostics.push(ConversionDiagnostic::warning(format!(
                "mesh {:08x} polygon {}: degenerate triangle, excluded from navigation mesh",
                mesh.form_id, polygon.index
            )));
            continue;
        }
        included_polygons.push(polygon);
    }

    if included_polygons.is_empty() {
        diagnostics.push(ConversionDiagnostic::warning(format!(
            "mesh {:08x}: no walkable polygons after conversion",
            mesh.form_id
        )));
        return BuildResult {
            nav_mesh: None,
            diagnostics,
        };
    }

    // Issue #136: erode the walkable boundary inward by the agent radius
    // before handing vertices to `bevy_landmass`, so its path smoothing
    // cannot string-pull a route within less than a capsule-width of a
    // wall/prop collider. Runs once per mesh (not per winding attempt
    // below) -- erosion only moves vertex *positions*, so it does not
    // interact with which polygon vertex order validates.
    let erosion_input = erosion_policy::ErosionMeshInput {
        vertices: mesh.vertices.clone(),
        polygons: included_polygons
            .iter()
            .map(|polygon| polygon.vertex_indices)
            .collect(),
        protected_edges: protected_edges_for_mesh(mesh, merges),
    };
    let erosion_result = erosion_policy::erode(&erosion_input, erosion_policy::AGENT_RADIUS);
    tracing::info!(
        "nav erosion: polys {} eroded {} pinch-guard {} relax-passes {} protected {}",
        erosion_result.polygon_count,
        erosion_result.eroded_count,
        erosion_result.pinch_guard_count,
        erosion_result.relax_passes,
        erosion_result.protected_count,
    );

    let vertices: Vec<Vec3> = erosion_result
        .vertices
        .iter()
        .map(|v| Vec3::new(v[0], v[1], v[2]))
        .collect();

    let mut last_error: Option<ValidationError> = None;
    for reversed in [false, true] {
        let polygons: Vec<Vec<usize>> = included_polygons
            .iter()
            .map(|polygon| {
                let mut indices: Vec<usize> = polygon
                    .vertex_indices
                    .iter()
                    .map(|&index| index as usize)
                    .collect();
                if reversed {
                    indices.reverse();
                }
                indices
            })
            .collect();
        let polygon_type_indices = vec![0usize; polygons.len()];
        let candidate = NavigationMesh3d {
            vertices: vertices.clone(),
            polygons,
            polygon_type_indices,
            height_mesh: None,
        };
        match candidate.validate() {
            Ok(valid) => {
                if reversed {
                    diagnostics.push(ConversionDiagnostic::warning(format!(
                        "mesh {:08x}: source polygon winding required reversal to validate",
                        mesh.form_id
                    )));
                }
                return BuildResult {
                    nav_mesh: Some(valid),
                    diagnostics,
                };
            }
            Err(error) => last_error = Some(error),
        }
    }

    diagnostics.push(ConversionDiagnostic::error(format!(
        "mesh {:08x}: landmass validation failed with both polygon windings: {}",
        mesh.form_id,
        last_error.expect("both winding attempts ran and recorded an error"),
    )));
    BuildResult {
        nav_mesh: None,
        diagnostics,
    }
}

// ---------------------------------------------------------------------
// Door-link descriptors
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DoorLinkSide {
    pub(crate) mesh_form_id: u32,
    pub(crate) polygon_index: u32,
    /// Centroid of the door triangle, standing in for an exact edge crossing
    /// point -- #111's `PreparedNavDoor` carries only a triangle index, not
    /// which edge of that triangle faces the door, so there is no exact
    /// edge-crossing geometry to derive here. #113 can refine this once a
    /// per-edge door association exists.
    pub(crate) midpoint: [f32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DoorLinkDescriptor {
    pub(crate) door_form_id: u32,
    pub(crate) side_a: DoorLinkSide,
    pub(crate) side_b: DoorLinkSide,
}

fn polygon_centroid(mesh: &MeshInput, polygon: &PolygonInput) -> Option<[f32; 3]> {
    if polygon
        .vertex_indices
        .iter()
        .any(|&index| index as usize >= mesh.vertices.len())
    {
        return None;
    }
    let mut sum = [0.0f32; 3];
    for &index in &polygon.vertex_indices {
        let vertex = mesh.vertices[index as usize];
        sum[0] += vertex[0];
        sum[1] += vertex[1];
        sum[2] += vertex[2];
    }
    Some([sum[0] / 3.0, sum[1] / 3.0, sum[2] / 3.0])
}

/// Builds `(door_form_id, side)` for every door-triangle association across
/// `meshes`, ordered by `(door_form_id, mesh_form_id, polygon_index)` --
/// the shared grouping key `door_link_descriptors` and `single_sided_doors`
/// both partition, so a door's real FO3 triangle count (usually exactly 1;
/// see wave 3's finding that every real door triangle is single-sided) only
/// needs computing once.
fn door_sides(meshes: &[MeshInput]) -> Vec<(u32, DoorLinkSide)> {
    let mut sides: Vec<(u32, DoorLinkSide)> = Vec::new();
    for mesh in meshes {
        for door in &mesh.doors {
            let Some(door_form_id) = door.door_reference_form_id else {
                continue;
            };
            let Some(polygon) = mesh
                .polygons
                .iter()
                .find(|polygon| polygon.index == door.triangle_index)
            else {
                continue;
            };
            let Some(midpoint) = polygon_centroid(mesh, polygon) else {
                continue;
            };
            sides.push((
                door_form_id,
                DoorLinkSide {
                    mesh_form_id: mesh.form_id,
                    polygon_index: polygon.index,
                    midpoint,
                },
            ));
        }
    }
    sides
        .sort_by_key(|(door_form_id, side)| (*door_form_id, side.mesh_form_id, side.polygon_index));
    sides
}

/// Deterministic door-link descriptor list (issue #112 feature 4): a door
/// FormID becomes a descriptor only when it is associated with *exactly*
/// two triangles across the graph's meshes -- "both sides resolve to
/// polygons in the loaded cell's graph" per the wave plan. A door
/// associated with zero, one, or more than two triangles (a travel door
/// touching only this cell's side, a decode oddity) is skipped rather than
/// guessed at. Ordered by `(door_form_id, mesh_form_id, polygon_index)` so
/// repeated calls on the same graph produce byte-identical output.
pub(crate) fn door_link_descriptors(meshes: &[MeshInput]) -> Vec<DoorLinkDescriptor> {
    let sides = door_sides(meshes);
    let mut descriptors = Vec::new();
    let mut index = 0;
    while index < sides.len() {
        let door_form_id = sides[index].0;
        let mut group_end = index + 1;
        while group_end < sides.len() && sides[group_end].0 == door_form_id {
            group_end += 1;
        }
        if group_end - index == 2 {
            descriptors.push(DoorLinkDescriptor {
                door_form_id,
                side_a: sides[index].1,
                side_b: sides[index + 1].1,
            });
        }
        index = group_end;
    }
    descriptors
}

/// One door associated with exactly one triangle across the graph's meshes
/// -- the shape every real FO3 door triangle takes (wave 3's finding).
/// Distinct from `door_link_descriptors`'s two-sided group: `nav/agent.rs`
/// treats a single-sided door as a travel-door candidate when the manifest
/// resolves it to a destination cell (issue #113 feature 3).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SingleSidedDoor {
    pub(crate) door_form_id: u32,
    pub(crate) side: DoorLinkSide,
}

/// Deterministic single-sided-door list, ordered by `(door_form_id,
/// mesh_form_id, polygon_index)`. A door with more than one triangle
/// association is not single-sided and is left to `door_link_descriptors`
/// (exactly two) or skipped entirely (three or more, a decode oddity).
pub(crate) fn single_sided_doors(meshes: &[MeshInput]) -> Vec<SingleSidedDoor> {
    let sides = door_sides(meshes);
    let mut result = Vec::new();
    let mut index = 0;
    while index < sides.len() {
        let door_form_id = sides[index].0;
        let mut group_end = index + 1;
        while group_end < sides.len() && sides[group_end].0 == door_form_id {
            group_end += 1;
        }
        if group_end - index == 1 {
            result.push(SingleSidedDoor {
                door_form_id,
                side: sides[index].1,
            });
        }
        index = group_end;
    }
    result
}

// ---------------------------------------------------------------------
// Same-cell cross-mesh merge links (issue #113, M4 wave 4 feature 2)
// ---------------------------------------------------------------------

/// One same-cell cross-mesh connection (boundary conversion from
/// `vsa::prepare::nav_graph::PreparedNavMeshMerge`, done in `nav/mod.rs`
/// per this module's usual `vsa`-free boundary-conversion split).
/// `interval_a`/`interval_b` are issue #154's validated, positionally-
/// corresponding portal-interval endpoints (already clamped to the two
/// edges' geometric overlap, prepare-side) -- `merge_link_descriptors`
/// below links their midpoints rather than triangle centroids.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MergeInput {
    pub(crate) mesh_a_form_id: u32,
    pub(crate) triangle_a: u32,
    pub(crate) mesh_b_form_id: u32,
    pub(crate) triangle_b: u32,
    pub(crate) interval_a: [[f32; 3]; 2],
    pub(crate) interval_b: [[f32; 3]; 2],
}

/// A resolved cross-mesh merge link, the same two-sided shape as
/// [`DoorLinkDescriptor`] minus the door FormID (a merge link is always
/// open -- there is no door to activate). `distance` is the straight-line
/// traversal distance between `side_a.midpoint` and `side_b.midpoint`
/// (issue #154 feature 3): the animation-link cost `nav/agent.rs` spawns
/// the link with, in place of the previous flat `1.0` every link (door or
/// merge) used.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MergeLinkDescriptor {
    pub(crate) side_a: DoorLinkSide,
    pub(crate) side_b: DoorLinkSide,
    pub(crate) distance: f32,
}

fn interval_midpoint(interval: [[f32; 3]; 2]) -> [f32; 3] {
    [
        (interval[0][0] + interval[1][0]) / 2.0,
        (interval[0][1] + interval[1][1]) / 2.0,
        (interval[0][2] + interval[1][2]) / 2.0,
    ]
}

fn point_distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    Vec3::from_array(a).distance(Vec3::from_array(b))
}

/// Vertical clearance budget (bevy metres) a merge portal's matched
/// interval may differ by in elevation between its two sides before this
/// module treats it as impassable rather than building an animation link
/// for it (issue #154 review correction). `vsa::prepare::nav_graph`'s own
/// portal validation is deliberately agent-class-agnostic pure geometry
/// (opposing directions, an overlapping interval -- see that module's
/// `PreparedNavMeshMerge` doc comment) and does *not* reject a candidate
/// for excessive vertical drop; this runtime layer is where the actual
/// agent definition (`nav::agent::AGENT_RADIUS` and friends) already lives,
/// so it is the correct place for an agent-aware "can this agent even step
/// up/down this portal" check instead. Duplicate of `viewer::openmw_player::
/// DEFAULT_STEP_HEIGHT`'s value (`34.0 / 69.991_25`), not an import: this
/// module is Bevy-engine-free (see the module doc comment) and that
/// constant's owning module pulls in `bevy::prelude::*`. Same duplication
/// precedent as `erosion_policy::AGENT_RADIUS`.
const MERGE_PORTAL_STEP_HEIGHT: f32 = 34.0 / 69.991_25;

/// Resolves each `MergeInput` into a world-space link, using its own
/// already-validated portal-interval midpoints (issue #154 feature 3) --
/// not, as before issue #154, a triangle-centroid lookup against `meshes`.
/// The prepare-side validation (`vsa::prepare::nav_graph::
/// validate_portal_candidate`) already confirmed both edges/triangles are
/// real and geometrically opposing/overlapping before a
/// `PreparedNavMeshMerge` was ever emitted, so the checks left to make here
/// are (a) both meshes are still present in the *loaded* `meshes` (a mesh
/// that failed landmass validation and so never became an island -- see
/// `build_navigation_mesh`'s doc comment -- would otherwise dangle a link
/// onto a non-existent island), never panicking on a missing one, and (b)
/// the agent-aware vertical-clearance check ([`MERGE_PORTAL_STEP_HEIGHT`])
/// prepare-time deliberately does not make. Order follows `merges`' own
/// order, which `vsa::prepare::nav_graph::compute_mesh_merges` already
/// produces deterministically.
pub(crate) fn merge_link_descriptors(
    meshes: &[MeshInput],
    merges: &[MergeInput],
) -> Vec<MergeLinkDescriptor> {
    let mut descriptors = Vec::new();
    for merge in merges {
        if !meshes
            .iter()
            .any(|mesh| mesh.form_id == merge.mesh_a_form_id)
        {
            continue;
        }
        if !meshes
            .iter()
            .any(|mesh| mesh.form_id == merge.mesh_b_form_id)
        {
            continue;
        }
        let drop = (merge.interval_a[0][1] - merge.interval_b[0][1])
            .abs()
            .max((merge.interval_a[1][1] - merge.interval_b[1][1]).abs());
        if drop > MERGE_PORTAL_STEP_HEIGHT {
            tracing::warn!(
                "nav merge portal mesh {:08x} triangle {} <-> mesh {:08x} triangle {}: skipped, vertical clearance {:.3} m exceeds the agent step height {:.3} m",
                merge.mesh_a_form_id,
                merge.triangle_a,
                merge.mesh_b_form_id,
                merge.triangle_b,
                drop,
                MERGE_PORTAL_STEP_HEIGHT,
            );
            continue;
        }
        let midpoint_a = interval_midpoint(merge.interval_a);
        let midpoint_b = interval_midpoint(merge.interval_b);
        descriptors.push(MergeLinkDescriptor {
            side_a: DoorLinkSide {
                mesh_form_id: merge.mesh_a_form_id,
                polygon_index: merge.triangle_a,
                midpoint: midpoint_a,
            },
            side_b: DoorLinkSide {
                mesh_form_id: merge.mesh_b_form_id,
                polygon_index: merge.triangle_b,
                midpoint: midpoint_b,
            },
            distance: point_distance(midpoint_a, midpoint_b),
        });
    }
    descriptors
}

// ---------------------------------------------------------------------
// Agent-state mapping
// ---------------------------------------------------------------------

/// Project-native agent status, decoupled from `bevy_landmass::AgentState`
/// so console output (`tna status`) and log lines have stable wording
/// independent of the backend's own enum naming.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NavAgentStatus {
    Idle,
    Moving,
    Reached,
    Unreachable,
    Paused,
    /// A travel-door traversal completed (issue #113 feature 3): the agent
    /// stopped at the traversed door; the destination cell is unloaded.
    /// #134 turns this terminal status into an actual cell handoff.
    TravelReached,
}

impl NavAgentStatus {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            NavAgentStatus::Idle => "idle",
            NavAgentStatus::Moving => "moving",
            NavAgentStatus::Reached => "reached",
            NavAgentStatus::Unreachable => "unreachable",
            NavAgentStatus::Paused => "paused",
            NavAgentStatus::TravelReached => "travel-reached",
        }
    }
}

/// Maps a `landmass` agent state to the project-native status. Animation
/// link states count as `Moving` at this layer -- the door-link state
/// machine (`nav::door_link`) is the authority on pause/wait/resume/failure
/// around a specific door, and overrides this mapping while active (see
/// `nav/agent.rs`'s status resolution).
pub(crate) fn map_agent_state(state: AgentState) -> NavAgentStatus {
    match state {
        AgentState::Idle => NavAgentStatus::Idle,
        AgentState::Moving | AgentState::ReachedAnimationLink | AgentState::UsingAnimationLink => {
            NavAgentStatus::Moving
        }
        AgentState::ReachedTarget => NavAgentStatus::Reached,
        AgentState::AgentNotOnNavMesh | AgentState::TargetNotOnNavMesh | AgentState::NoPath => {
            NavAgentStatus::Unreachable
        }
        AgentState::Paused => NavAgentStatus::Paused,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square_mesh(vertex_indices_a: [u32; 3], vertex_indices_b: [u32; 3]) -> MeshInput {
        MeshInput {
            form_id: 0x10,
            vertices: vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 1.0],
            ],
            polygons: vec![
                PolygonInput {
                    index: 0,
                    vertex_indices: vertex_indices_a,
                    is_water: false,
                },
                PolygonInput {
                    index: 1,
                    vertex_indices: vertex_indices_b,
                    is_water: false,
                },
            ],
            doors: Vec::new(),
        }
    }

    #[test]
    fn validates_a_known_good_square_mesh() {
        // Empirically the winding `NavigationMesh3d::validate()` accepts
        // without needing this function's reversal retry, for this
        // vertex layout (see `reversed_winding_still_validates_after_retry`
        // for the opposite winding, which does need the retry).
        let mesh = square_mesh([0, 1, 2], [1, 3, 2]);
        let result = build_navigation_mesh(&mesh, &[]);
        assert!(result.nav_mesh.is_some(), "{:?}", result.diagnostics);
        assert!(
            result.diagnostics.is_empty(),
            "expected no diagnostics for a known-good mesh: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn reversed_winding_still_validates_after_retry_with_a_warning() {
        // Same square, opposite winding: the first attempt must fail
        // internally and the retry with reversed order must succeed.
        let mesh = square_mesh([0, 2, 1], [1, 2, 3]);
        let result = build_navigation_mesh(&mesh, &[]);
        assert!(result.nav_mesh.is_some(), "{:?}", result.diagnostics);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("required reversal")),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn water_polygons_are_excluded_as_non_walkable() {
        let mut mesh = square_mesh([0, 2, 1], [1, 2, 3]);
        mesh.polygons[1].is_water = true;
        let result = build_navigation_mesh(&mesh, &[]);
        // One walkable polygon remains -- still a valid (smaller) mesh.
        // `ValidNavigationMesh`'s fields are private to `landmass`, so this
        // only asserts what's externally observable: conversion still
        // succeeds after excluding the water polygon.
        assert!(result.nav_mesh.is_some(), "{:?}", result.diagnostics);
    }

    #[test]
    fn all_water_mesh_produces_no_navigation_mesh_and_a_warning() {
        let mut mesh = square_mesh([0, 2, 1], [1, 2, 3]);
        mesh.polygons[0].is_water = true;
        mesh.polygons[1].is_water = true;
        let result = build_navigation_mesh(&mesh, &[]);
        assert!(result.nav_mesh.is_none());
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("no walkable polygons")),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn invalid_vertex_index_polygon_is_skipped_with_an_error_diagnostic_and_never_panics() {
        let mut mesh = square_mesh([0, 2, 1], [1, 2, 3]);
        mesh.polygons[1].vertex_indices = [1, 2, u32::MAX];
        let result = build_navigation_mesh(&mesh, &[]);
        assert!(result.nav_mesh.is_some(), "{:?}", result.diagnostics);
        assert!(
            result.diagnostics.iter().any(
                |d| d.severity == Severity::Error && d.message.contains("invalid vertex index")
            ),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn degenerate_polygon_is_skipped_with_a_warning_diagnostic() {
        let mut mesh = square_mesh([0, 2, 1], [1, 2, 3]);
        mesh.polygons[1].vertex_indices = [1, 1, 3];
        let result = build_navigation_mesh(&mesh, &[]);
        assert!(result.nav_mesh.is_some(), "{:?}", result.diagnostics);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.severity == Severity::Warning
                    && d.message.contains("degenerate triangle")),
            "{:?}",
            result.diagnostics
        );
    }

    fn mesh_with_door(form_id: u32, triangle_index: u32, door_form_id: u32) -> MeshInput {
        MeshInput {
            form_id,
            vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
            polygons: vec![PolygonInput {
                index: triangle_index,
                vertex_indices: [0, 1, 2],
                is_water: false,
            }],
            doors: vec![DoorInput {
                triangle_index,
                door_reference_form_id: Some(door_form_id),
            }],
        }
    }

    #[test]
    fn door_link_descriptor_extracted_when_both_sides_resolve() {
        let mesh_a = mesh_with_door(0x10, 0, 0x99);
        let mesh_b = mesh_with_door(0x20, 0, 0x99);
        let descriptors = door_link_descriptors(&[mesh_a, mesh_b]);
        assert_eq!(descriptors.len(), 1);
        assert_eq!(descriptors[0].door_form_id, 0x99);
        assert_eq!(descriptors[0].side_a.mesh_form_id, 0x10);
        assert_eq!(descriptors[0].side_b.mesh_form_id, 0x20);
    }

    #[test]
    fn door_link_descriptor_is_deterministic_across_calls() {
        let mesh_a = mesh_with_door(0x10, 0, 0x99);
        let mesh_b = mesh_with_door(0x20, 0, 0x99);
        let first = door_link_descriptors(&[mesh_a.clone(), mesh_b.clone()]);
        let second = door_link_descriptors(&[mesh_a, mesh_b]);
        assert_eq!(first, second);
    }

    #[test]
    fn door_with_only_one_side_is_skipped() {
        let mesh_a = mesh_with_door(0x10, 0, 0x99);
        let descriptors = door_link_descriptors(&[mesh_a]);
        assert!(descriptors.is_empty());
    }

    #[test]
    fn door_with_more_than_two_sides_is_skipped() {
        let mesh_a = mesh_with_door(0x10, 0, 0x99);
        let mesh_b = mesh_with_door(0x20, 0, 0x99);
        let mesh_c = mesh_with_door(0x30, 0, 0x99);
        let descriptors = door_link_descriptors(&[mesh_a, mesh_b, mesh_c]);
        assert!(descriptors.is_empty());
    }

    #[test]
    fn a_door_with_one_side_is_single_sided() {
        // Wave 3's real-data finding: every real FO3 door triangle is
        // single-sided.
        let mesh_a = mesh_with_door(0x10, 0, 0x99);
        let doors = single_sided_doors(&[mesh_a]);
        assert_eq!(doors.len(), 1);
        assert_eq!(doors[0].door_form_id, 0x99);
        assert_eq!(doors[0].side.mesh_form_id, 0x10);
    }

    #[test]
    fn a_door_with_two_sides_is_not_single_sided() {
        let mesh_a = mesh_with_door(0x10, 0, 0x99);
        let mesh_b = mesh_with_door(0x20, 0, 0x99);
        let doors = single_sided_doors(&[mesh_a, mesh_b]);
        assert!(doors.is_empty());
    }

    #[test]
    fn single_sided_doors_are_deterministic_and_ordered() {
        let mesh_a = mesh_with_door(0x10, 0, 0x99);
        let mesh_b = mesh_with_door(0x20, 0, 0x50);
        let first = single_sided_doors(&[mesh_a.clone(), mesh_b.clone()]);
        let second = single_sided_doors(&[mesh_a, mesh_b]);
        assert_eq!(first, second);
        assert_eq!(
            first.iter().map(|d| d.door_form_id).collect::<Vec<_>>(),
            vec![0x50, 0x99]
        );
    }

    #[test]
    fn merge_link_descriptor_resolves_both_sides_from_the_prepared_interval() {
        let mesh_a = square_mesh([0, 1, 2], [1, 3, 2]);
        let mut mesh_b = square_mesh([0, 1, 2], [1, 3, 2]);
        mesh_b.form_id = 0x20;
        let merges = vec![MergeInput {
            mesh_a_form_id: 0x10,
            triangle_a: 0,
            mesh_b_form_id: 0x20,
            triangle_b: 1,
            interval_a: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            interval_b: [[2.0, 0.0, 0.0], [3.0, 0.0, 0.0]],
        }];
        let descriptors = merge_link_descriptors(&[mesh_a, mesh_b], &merges);
        assert_eq!(descriptors.len(), 1);
        let descriptor = descriptors[0];
        assert_eq!(descriptor.side_a.mesh_form_id, 0x10);
        assert_eq!(descriptor.side_a.polygon_index, 0);
        // Issue #154 feature 3: the link point is the *interval* midpoint,
        // not a triangle centroid.
        assert_eq!(descriptor.side_a.midpoint, [0.5, 0.0, 0.0]);
        assert_eq!(descriptor.side_b.mesh_form_id, 0x20);
        assert_eq!(descriptor.side_b.polygon_index, 1);
        assert_eq!(descriptor.side_b.midpoint, [2.5, 0.0, 0.0]);
        // Cost is the real distance between the two interval midpoints.
        assert!((descriptor.distance - 2.0).abs() < 1.0e-6, "{descriptor:?}");
    }

    #[test]
    fn merge_referencing_a_missing_mesh_is_skipped_not_panicked() {
        // Issue #154: triangle/edge legitimacy is already validated
        // prepare-side before a `MergeInput` is ever produced, so this
        // module's own defensive check is reduced to "is the mesh still
        // present in the loaded manifest" -- see `merge_link_descriptors`'s
        // doc comment.
        let mesh_a = square_mesh([0, 1, 2], [1, 3, 2]);
        let merges = vec![MergeInput {
            mesh_a_form_id: 0x10,
            triangle_a: 0,
            mesh_b_form_id: 0x9999, // no such mesh
            triangle_b: 0,
            interval_a: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            interval_b: [[2.0, 0.0, 0.0], [3.0, 0.0, 0.0]],
        }];
        let descriptors = merge_link_descriptors(&[mesh_a], &merges);
        assert!(descriptors.is_empty());
    }

    #[test]
    fn a_merge_with_too_much_vertical_drop_is_skipped_at_runtime() {
        // Issue #154 review correction: prepare-side no longer rejects a
        // portal candidate for excessive vertical drop (that is an
        // agent-class assumption, moved here) -- so a `PreparedNavMeshMerge`
        // can legitimately carry an interval like this (adversarial
        // fixture: vertically stacked floors whose edges overlap in XZ).
        // This runtime layer, where the actual agent step-height
        // definition lives, is what must skip building a link for it.
        let mesh_a = square_mesh([0, 1, 2], [1, 3, 2]);
        let mut mesh_b = square_mesh([0, 1, 2], [1, 3, 2]);
        mesh_b.form_id = 0x20;
        let merges = vec![MergeInput {
            mesh_a_form_id: 0x10,
            triangle_a: 0,
            mesh_b_form_id: 0x20,
            triangle_b: 1,
            interval_a: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            // 1 m above mesh_a's interval -- well past a humanoid step.
            interval_b: [[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
        }];
        let descriptors = merge_link_descriptors(&[mesh_a, mesh_b], &merges);
        assert!(descriptors.is_empty(), "{descriptors:?}");
    }

    #[test]
    fn a_merge_within_step_height_still_resolves() {
        // The counterpart to the vertical-drop-is-skipped test above: a
        // small (well under step-height) elevation difference must still
        // resolve normally.
        let mesh_a = square_mesh([0, 1, 2], [1, 3, 2]);
        let mut mesh_b = square_mesh([0, 1, 2], [1, 3, 2]);
        mesh_b.form_id = 0x20;
        let merges = vec![MergeInput {
            mesh_a_form_id: 0x10,
            triangle_a: 0,
            mesh_b_form_id: 0x20,
            triangle_b: 1,
            interval_a: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            interval_b: [[0.0, 0.05, 0.0], [1.0, 0.05, 0.0]],
        }];
        let descriptors = merge_link_descriptors(&[mesh_a, mesh_b], &merges);
        assert_eq!(descriptors.len(), 1, "{descriptors:?}");
    }

    /// `MergeInput` with zeroed portal intervals -- the tests below only
    /// exercise `protected_edges_for_mesh`, which never reads
    /// `interval_a`/`interval_b`.
    fn merge_input(
        mesh_a_form_id: u32,
        triangle_a: u32,
        mesh_b_form_id: u32,
        triangle_b: u32,
    ) -> MergeInput {
        MergeInput {
            mesh_a_form_id,
            triangle_a,
            mesh_b_form_id,
            triangle_b,
            interval_a: [[0.0; 3]; 2],
            interval_b: [[0.0; 3]; 2],
        }
    }

    #[test]
    fn a_merge_naming_this_mesh_as_side_a_protects_its_triangles_edges() {
        let mesh = square_mesh([0, 1, 2], [1, 3, 2]);
        // polygon 0 = [0, 1, 2]; triangle_b (5) is some triangle on the
        // other mesh, irrelevant here.
        let merges = vec![merge_input(0x10, 0, 0x20, 5)];
        let edges = protected_edges_for_mesh(&mesh, &merges);
        assert_eq!(edges, vec![(0, 1), (1, 2), (2, 0)]);
    }

    #[test]
    fn a_merge_naming_this_mesh_as_side_b_protects_its_triangles_edges() {
        let mesh = square_mesh([0, 1, 2], [1, 3, 2]);
        // polygon 1 = [1, 3, 2].
        let merges = vec![merge_input(0x20, 5, 0x10, 1)];
        let edges = protected_edges_for_mesh(&mesh, &merges);
        assert_eq!(edges, vec![(1, 3), (3, 2), (2, 1)]);
    }

    #[test]
    fn a_door_triangle_protects_its_edges_the_same_way_as_a_merge() {
        let mut mesh = square_mesh([0, 1, 2], [1, 3, 2]);
        mesh.doors.push(DoorInput {
            triangle_index: 0,
            door_reference_form_id: Some(0x99),
        });
        let edges = protected_edges_for_mesh(&mesh, &[]);
        assert_eq!(edges, vec![(0, 1), (1, 2), (2, 0)]);
    }

    #[test]
    fn a_merge_not_touching_this_mesh_protects_nothing() {
        let mesh = square_mesh([0, 1, 2], [1, 3, 2]);
        let merges = vec![merge_input(0x30, 0, 0x40, 1)];
        assert!(protected_edges_for_mesh(&mesh, &merges).is_empty());
    }

    #[test]
    fn no_doors_or_merges_protects_nothing() {
        let mesh = square_mesh([0, 1, 2], [1, 3, 2]);
        assert!(protected_edges_for_mesh(&mesh, &[]).is_empty());
    }

    #[test]
    fn agent_state_maps_to_project_status() {
        assert_eq!(map_agent_state(AgentState::Idle), NavAgentStatus::Idle);
        assert_eq!(map_agent_state(AgentState::Moving), NavAgentStatus::Moving);
        assert_eq!(
            map_agent_state(AgentState::ReachedTarget),
            NavAgentStatus::Reached
        );
        assert_eq!(
            map_agent_state(AgentState::AgentNotOnNavMesh),
            NavAgentStatus::Unreachable
        );
        assert_eq!(
            map_agent_state(AgentState::TargetNotOnNavMesh),
            NavAgentStatus::Unreachable
        );
        assert_eq!(
            map_agent_state(AgentState::NoPath),
            NavAgentStatus::Unreachable
        );
        assert_eq!(map_agent_state(AgentState::Paused), NavAgentStatus::Paused);
    }
}
