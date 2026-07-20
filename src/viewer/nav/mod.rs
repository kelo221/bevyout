//! M4 wave 3 (#112): `bevy_landmass` navigation-backend spike. Owns loading
//! the prepared per-cell nav graph (#111's `navgraph.ron`, the same way
//! `nav_overlay.rs` reads it), converting it into a validated
//! `bevy_landmass` navigation mesh via the pure `landmass_graph` module, and
//! running a crude kinematic test agent (`agent.rs`) driven by the `tna`
//! console command family. This is the runtime navigation slice seam #113
//! grows into the full Fallout adapter (travel doors, AI packages); this
//! wave stays a spike -- one archipelago per active cell (one island per
//! prepared nav mesh within it), built lazily, torn down on cell swap.

use std::path::{Path, PathBuf};

use anyhow::Context;
use bevy::prelude::*;

use crate::vsa::{PreparedNavGraph, PreparedSceneManifest};

pub(crate) mod agent;
pub(crate) mod door_link;
pub(crate) mod landmass_graph;
pub(crate) mod ledger_policy;
pub(crate) mod movement_policy;
pub(crate) mod repath;

pub(crate) struct NavPlugin;

impl Plugin for NavPlugin {
    fn build(&self, app: &mut App) {
        install(app);
    }
}

fn install(app: &mut App) {
    agent::install(app);
}

/// Reads+parses `navgraph.ron`. Duplicated from `nav_overlay::read_nav_graph`
/// rather than shared: that function is private to `nav_overlay.rs`, which
/// this wave's file-ownership boundary does not include.
pub(crate) fn read_nav_graph(path: &Path) -> anyhow::Result<PreparedNavGraph> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("nav graph does not exist: {}", path.display()))?;
    ron::de::from_str(&text).with_context(|| format!("invalid nav graph RON: {}", path.display()))
}

/// Resolves `manifest.nav_graph`'s `asset_path` against `asset_root`, the
/// same way `nav_overlay::toggle_nav_mesh` does. `None` when the cell has no
/// prepared nav graph.
pub(crate) fn nav_graph_path(manifest: &PreparedSceneManifest) -> Option<PathBuf> {
    let source = manifest.nav_graph.as_ref()?;
    Some(
        PathBuf::from(&manifest.asset_root).join(
            source
                .asset_path
                .replace('/', std::path::MAIN_SEPARATOR_STR),
        ),
    )
}

/// Boundary conversion: `PreparedNavGraph`/`PreparedNavMesh` (vsa's decode
/// output, issue #111) -> `landmass_graph`'s own plain input DTOs. Keeps
/// `landmass_graph` free of any `vsa`/Bevy import so it stays includable
/// verbatim by `tests/features.rs` via `#[path]` -- see that module's doc
/// comment for why this boundary conversion cannot live inside it.
///
/// Issue #153 (M4 wave 10): the prepared graph's vertices already carry the
/// collision-derived agent-radius clearance offset, and each polygon's
/// `walkable` flag records the prepare-side validation verdict (removed for
/// lacking collision support, cut by an interior collider, or disconnected as
/// a sub-diameter corridor throat). `!walkable` polygons are excluded here so
/// they never reach the landmass navigation mesh -- a route into a dropped
/// region is then `unreachable` at query time. The interim runtime erosion
/// pass is retired; clearance now lives entirely prepare-side.
pub(crate) fn mesh_inputs(graph: &PreparedNavGraph) -> Vec<landmass_graph::MeshInput> {
    graph
        .meshes
        .iter()
        .map(|mesh| landmass_graph::MeshInput {
            form_id: mesh.form_id,
            vertices: mesh.vertices.clone(),
            polygons: mesh
                .polygons
                .iter()
                .filter(|polygon| polygon.walkable)
                .map(|polygon| landmass_graph::PolygonInput {
                    index: polygon.index,
                    vertex_indices: polygon.vertex_indices,
                    is_water: polygon.is_water,
                    is_preferred_pathing: polygon.is_preferred_pathing,
                })
                .collect(),
            doors: mesh
                .doors
                .iter()
                .map(|door| landmass_graph::DoorInput {
                    triangle_index: door.triangle_index,
                    door_reference_form_id: door.door_reference_form_id,
                })
                .collect(),
            // Issue #177: collision-derived blocker associations, carried
            // separately from the authored `NVDP` list -- see
            // `landmass_graph::MeshInput::derived_doors` for why they must
            // never be merged into it.
            derived_doors: mesh
                .derived_doors
                .iter()
                .map(|door| landmass_graph::DerivedDoorInput {
                    triangle_index: door.triangle_index,
                    door_reference_form_id: door.door_reference_form_id,
                    blocks_when_closed: door.blocks_when_closed,
                    openable: door.openable,
                })
                .collect(),
        })
        .collect()
}

/// Boundary conversion: `PreparedNavGraph::mesh_merges` (prepare-time
/// spatial cross-mesh connections, issue #113 feature 2; validated portal
/// intervals added issue #154 feature 1) -> plain
/// `landmass_graph::MergeInput`s. Same split rationale as `mesh_inputs`.
/// `edge_a`/`edge_b` (the matched edges' vertex-index identity) stay
/// prepare-side only -- nothing at runtime needs to re-derive geometry from
/// them, only the already-resolved `interval_a`/`interval_b` world points.
pub(crate) fn merge_inputs(graph: &PreparedNavGraph) -> Vec<landmass_graph::MergeInput> {
    graph
        .mesh_merges
        .iter()
        .map(|merge| landmass_graph::MergeInput {
            mesh_a_form_id: merge.mesh_a_form_id,
            triangle_a: merge.triangle_a,
            mesh_b_form_id: merge.mesh_b_form_id,
            triangle_b: merge.triangle_b,
            interval_a: merge.interval_a,
            interval_b: merge.interval_b,
        })
        .collect()
}

/// A travel door's resolved far side (issue #113 feature 3 / #134): the
/// destination cell, and the door reference FormID *in that cell* the
/// destination metadata pairs this door with (`PreparedDoorDestination::
/// door_reference_form_id`) -- the same marker position the player's own
/// teleport uses, and what #134's ledger keys a door-marker spawn on.
#[derive(Debug, Clone, Copy)]
pub(crate) struct TravelDestination {
    pub(crate) cell_form_id: u32,
    pub(crate) door_reference_form_id: u32,
}

/// Door reference FormID -> resolved travel destination, for every
/// placement in `manifest` whose semantic is a travel door (issue #113
/// feature 3). Reuses the existing world-transition door-destination data
/// (`PreparedDoor`, #51/#52) rather than a new lookup -- a placement's
/// `reference_form_id` is exactly the FormID the prepared nav graph's door
/// array keys triangle associations by
/// (`PreparedNavDoor::door_reference_form_id`).
pub(crate) fn travel_door_destinations(
    manifest: &PreparedSceneManifest,
) -> std::collections::HashMap<u32, TravelDestination> {
    manifest
        .placements
        .iter()
        .filter_map(|placement| match &placement.semantic {
            crate::vsa::PreparedSemantic::Door(door) => {
                door.destination.as_ref().map(|destination| {
                    (
                        placement.reference_form_id,
                        TravelDestination {
                            cell_form_id: destination.cell_form_id,
                            door_reference_form_id: destination.door_reference_form_id,
                        },
                    )
                })
            }
            _ => None,
        })
        .collect()
}
