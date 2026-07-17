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
pub(crate) fn build_navigation_mesh(mesh: &MeshInput) -> BuildResult {
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
    };
    let erosion_result = erosion_policy::erode(&erosion_input, erosion_policy::AGENT_RADIUS);
    tracing::info!(
        "nav erosion: polys {} eroded {} pinch-guard {}",
        erosion_result.polygon_count,
        erosion_result.eroded_count,
        erosion_result.pinch_guard_count,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MergeInput {
    pub(crate) mesh_a_form_id: u32,
    pub(crate) triangle_a: u32,
    pub(crate) mesh_b_form_id: u32,
    pub(crate) triangle_b: u32,
}

/// A resolved cross-mesh merge link, the same two-sided shape as
/// [`DoorLinkDescriptor`] minus the door FormID (a merge link is always
/// open -- there is no door to activate).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MergeLinkDescriptor {
    pub(crate) side_a: DoorLinkSide,
    pub(crate) side_b: DoorLinkSide,
}

/// Resolves each `MergeInput` (mesh/triangle pairs from the prepared graph)
/// against the loaded `meshes` into world-space link endpoints. A merge
/// referencing a mesh or triangle no longer present in `meshes` is skipped
/// (never panics) -- the prepared graph and the loaded manifest are
/// expected to agree, but this stays defensive the same way
/// `door_link_descriptors` is. Order follows `merges`' own order, which
/// `vsa::prepare::nav_graph::compute_mesh_merges` already produces
/// deterministically.
pub(crate) fn merge_link_descriptors(
    meshes: &[MeshInput],
    merges: &[MergeInput],
) -> Vec<MergeLinkDescriptor> {
    let mut descriptors = Vec::new();
    for merge in merges {
        let Some(mesh_a) = meshes
            .iter()
            .find(|mesh| mesh.form_id == merge.mesh_a_form_id)
        else {
            continue;
        };
        let Some(mesh_b) = meshes
            .iter()
            .find(|mesh| mesh.form_id == merge.mesh_b_form_id)
        else {
            continue;
        };
        let Some(polygon_a) = mesh_a
            .polygons
            .iter()
            .find(|polygon| polygon.index == merge.triangle_a)
        else {
            continue;
        };
        let Some(polygon_b) = mesh_b
            .polygons
            .iter()
            .find(|polygon| polygon.index == merge.triangle_b)
        else {
            continue;
        };
        let (Some(midpoint_a), Some(midpoint_b)) = (
            polygon_centroid(mesh_a, polygon_a),
            polygon_centroid(mesh_b, polygon_b),
        ) else {
            continue;
        };
        descriptors.push(MergeLinkDescriptor {
            side_a: DoorLinkSide {
                mesh_form_id: mesh_a.form_id,
                polygon_index: polygon_a.index,
                midpoint: midpoint_a,
            },
            side_b: DoorLinkSide {
                mesh_form_id: mesh_b.form_id,
                polygon_index: polygon_b.index,
                midpoint: midpoint_b,
            },
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
        let result = build_navigation_mesh(&mesh);
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
        let result = build_navigation_mesh(&mesh);
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
        let result = build_navigation_mesh(&mesh);
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
        let result = build_navigation_mesh(&mesh);
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
        let result = build_navigation_mesh(&mesh);
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
        let result = build_navigation_mesh(&mesh);
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
    fn merge_link_descriptor_resolves_both_sides_from_the_prepared_connection() {
        let mesh_a = square_mesh([0, 1, 2], [1, 3, 2]);
        let mut mesh_b = square_mesh([0, 1, 2], [1, 3, 2]);
        mesh_b.form_id = 0x20;
        let merges = vec![MergeInput {
            mesh_a_form_id: 0x10,
            triangle_a: 0,
            mesh_b_form_id: 0x20,
            triangle_b: 1,
        }];
        let descriptors = merge_link_descriptors(&[mesh_a, mesh_b], &merges);
        assert_eq!(descriptors.len(), 1);
        assert_eq!(descriptors[0].side_a.mesh_form_id, 0x10);
        assert_eq!(descriptors[0].side_a.polygon_index, 0);
        assert_eq!(descriptors[0].side_b.mesh_form_id, 0x20);
        assert_eq!(descriptors[0].side_b.polygon_index, 1);
    }

    #[test]
    fn merge_referencing_a_missing_mesh_or_triangle_is_skipped_not_panicked() {
        let mesh_a = square_mesh([0, 1, 2], [1, 3, 2]);
        let merges = vec![
            MergeInput {
                mesh_a_form_id: 0x10,
                triangle_a: 0,
                mesh_b_form_id: 0x9999, // no such mesh
                triangle_b: 0,
            },
            MergeInput {
                mesh_a_form_id: 0x10,
                triangle_a: 999, // no such triangle
                mesh_b_form_id: 0x10,
                triangle_b: 1,
            },
        ];
        let descriptors = merge_link_descriptors(&[mesh_a], &merges);
        assert!(descriptors.is_empty());
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
