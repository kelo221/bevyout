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

pub(crate) fn install(app: &mut App) {
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
                .map(|polygon| landmass_graph::PolygonInput {
                    index: polygon.index,
                    vertex_indices: polygon.vertex_indices,
                    is_water: polygon.is_water,
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
        })
        .collect()
}
