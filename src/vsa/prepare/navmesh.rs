//! Prepared navmesh staging (raw sources + decoded nav graph, issue #111).

use super::*;

/// Stages this cell's NAVM records: writes the retained raw `*.navm.bin`
/// sources beside the scene manifest (unchanged from the pre-#111
/// behaviour), then builds the decoded polygon navigation graph via the pure
/// `nav_graph` module and writes it as `scenes/<cell>/navmesh/navgraph.ron`.
/// Returns the per-record source metadata plus the manifest's graph pointer
/// (`None` when the cell has no navmeshes) and the deterministic report
/// line.
pub(crate) fn stage_navmeshes(
    cache_dir: &Path,
    scene_dir: &Path,
    cell_form_id: u32,
    diagnostics: &mut Vec<Diagnostic>,
    navmeshes: &[crate::vsa::openmw_esm4::NavMeshRecord],
    navigation: Option<&crate::vsa::openmw_esm4::NaviRecord>,
) -> Result<(
    Vec<PreparedNavMeshSource>,
    Option<PreparedNavGraphSource>,
    String,
)> {
    let graph_inputs = nav_graph_inputs(cell_form_id, navmeshes, navigation);
    let graph = build_nav_graph(&graph_inputs);
    let summary = format!(
        "nav graph: meshes {}, polygons {}, vertices {}, doors {}, external {}, diagnostics warn {} error {}",
        graph.counters.meshes,
        graph.counters.polygons,
        graph.counters.vertices,
        graph.counters.doors,
        graph.counters.external_connections,
        graph.counters.diagnostics_warning,
        graph.counters.diagnostics_error
    );

    if navmeshes.is_empty() {
        return Ok((Vec::new(), None, summary));
    }

    let navmesh_dir = scene_dir.join("navmesh");
    fs::create_dir_all(&navmesh_dir)?;
    let sources = navmeshes
        .iter()
        .map(|navmesh| {
            let filename = format!("{:08x}.navm.bin", navmesh.form_id);
            fs::write(navmesh_dir.join(&filename), &navmesh.payload)?;
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!(
                    "retained FO3 NAVM {:08x} raw source beside its decoded nav graph",
                    navmesh.form_id
                ),
            });
            Ok(PreparedNavMeshSource {
                form_id: navmesh.form_id,
                record_flags: navmesh.flags,
                version: navmesh.version,
                asset_path: format!(
                    "scenes/{}/navmesh/{filename}",
                    scene_dir
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or_default()
                ),
                chunks: navmesh
                    .chunks
                    .iter()
                    .map(|chunk| PreparedNavMeshChunk {
                        signature: chunk.signature.clone(),
                        byte_len: chunk.byte_len,
                    })
                    .collect(),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    for diagnostic in &graph.diagnostics {
        diagnostics.push(Diagnostic {
            severity: diagnostic.severity.clone(),
            message: format!("nav graph: {}", diagnostic.message),
        });
    }
    let artifact = write_nav_graph(cache_dir, cell_form_id, &graph)?;
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: summary.clone(),
    });
    let graph_source = PreparedNavGraphSource {
        asset_path: artifact.relative_path,
        revision: NAV_GRAPH_REVISION.into(),
        hash: artifact.hash,
        mesh_count: graph.counters.meshes,
        polygon_count: graph.counters.polygons,
        vertex_count: graph.counters.vertices,
        door_count: graph.counters.doors,
        external_connection_count: graph.counters.external_connections,
        diagnostics_warning: graph.counters.diagnostics_warning,
        diagnostics_error: graph.counters.diagnostics_error,
    };
    Ok((sources, Some(graph_source), summary))
}

/// Boundary conversion from the parser's `NavMeshRecord`/`NaviRecord` types
/// into the pure `nav_graph` input shapes. Lives here (not in `nav_graph.rs`)
/// so that module stays free of `openmw_esm4` imports and cucumber-includable
/// -- the same split `actor_catalog.rs` uses with `orchestrator.rs`.
fn nav_graph_inputs(
    cell_form_id: u32,
    navmeshes: &[crate::vsa::openmw_esm4::NavMeshRecord],
    navigation: Option<&crate::vsa::openmw_esm4::NaviRecord>,
) -> NavGraphInputs {
    NavGraphInputs {
        cell_form_id,
        meshes: navmeshes
            .iter()
            .map(|navmesh| NavGraphMeshInput {
                form_id: navmesh.form_id,
                cell_form_id: navmesh.cell_form_id,
                vertices: navmesh
                    .vertices
                    .iter()
                    .map(|vertex| NavGraphVertexInput { source: *vertex })
                    .collect(),
                triangles: navmesh
                    .triangles
                    .iter()
                    .map(|triangle| NavGraphTriangleInput {
                        vertex_indices: triangle.vertex_indices.map(i32::from),
                        edge_neighbors: triangle.edge_neighbors.map(i32::from),
                        flags: triangle.flags,
                    })
                    .collect(),
                doors: navmesh
                    .doors
                    .iter()
                    .map(|door| NavGraphDoorInput {
                        door_reference_form_id: door.door_reference_form_id,
                        triangle_index: u32::from(door.triangle),
                    })
                    .collect(),
                external_connections: navmesh
                    .external_connections
                    .iter()
                    .map(|connection| NavGraphExternalInput {
                        target_navmesh_form_id: connection.target_navmesh_form_id,
                        triangle_index: u32::from(connection.triangle),
                    })
                    .collect(),
                cover_triangle_ids: navmesh
                    .cover_triangle_ids
                    .iter()
                    .copied()
                    .map(i32::from)
                    .collect(),
            })
            .collect(),
        navi_entries: navigation
            .map(|navi| {
                navi.entries
                    .iter()
                    .map(|entry| NavGraphNaviEntryInput {
                        navmesh_form_id: entry.navmesh_form_id,
                        location_form_id: entry.location_form_id,
                        grid_x: entry.grid_x,
                        grid_y: entry.grid_y,
                    })
                    .collect()
            })
            .unwrap_or_default(),
    }
}
