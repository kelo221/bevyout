//! Prepared navmesh staging (raw sources + decoded nav graph, issue #111),
//! plus the collision-derived validation/clearance wiring (issue #153, M4
//! wave 10): this module owns the boundary conversion from prepared
//! placements + cooked physics shapes into the pure `nav_clearance` module's
//! world-space triangle inputs, and writes the clearance results back into
//! the `PreparedNavGraph` before the manifest is finalized.

use std::collections::HashMap;

use bevy::math::{Mat4, Quat, Vec3};

use super::super::physics::{PreparedPhysicsAsset, PreparedPhysicsShape, body_blocks_player};
use super::*;
use crate::vsa::{PreparedPhysicsClassification, PreparedPlacement, PreparedSemantic};

/// `stage_navmeshes`' return: per-record source metadata, the manifest graph
/// pointer, the built graph itself (kept so `apply_nav_clearance` can mutate
/// and rewrite it after physics classification, issue #153), and the
/// deterministic report line. The last two are `None` when the cell has no
/// navmeshes. Named to keep `stage_navmeshes` under clippy's type-complexity
/// limit.
pub(crate) type StagedNavmeshes = (
    Vec<PreparedNavMeshSource>,
    Option<PreparedNavGraphSource>,
    Option<PreparedNavGraph>,
    String,
);

/// Stages this cell's NAVM records: writes the retained raw `*.navm.bin`
/// sources beside the scene manifest (unchanged from the pre-#111
/// behaviour), then builds the decoded polygon navigation graph via the pure
/// `nav_graph` module and writes it as `scenes/<cell>/navmesh/navgraph.ron`.
/// See [`StagedNavmeshes`] for the returned tuple.
pub(crate) fn stage_navmeshes(
    cache_dir: &Path,
    scene_dir: &Path,
    cell_form_id: u32,
    diagnostics: &mut Vec<Diagnostic>,
    navmeshes: &[crate::vsa::openmw_esm4::NavMeshRecord],
    navigation: Option<&crate::vsa::openmw_esm4::NaviRecord>,
) -> Result<StagedNavmeshes> {
    let graph_inputs = nav_graph_inputs(cell_form_id, navmeshes, navigation);
    let graph = build_nav_graph(&graph_inputs);
    let summary = format!(
        "nav graph: meshes {}, polygons {}, vertices {}, doors {}, external {}, merges {} (rejected {}, authored {} geometric {}, candidates authored {} geometric {}), diagnostics warn {} error {}, nvex correlation (outside-cell {} inside-cell {}), nvci correlation (subrecords {} entries {} door-matches {} navmesh-matches {})",
        graph.counters.meshes,
        graph.counters.polygons,
        graph.counters.vertices,
        graph.counters.doors,
        graph.counters.external_connections,
        graph.counters.mesh_merges,
        graph.counters.mesh_merges_rejected,
        graph.counters.mesh_merges_authored,
        graph.counters.mesh_merges_geometric,
        graph.counters.merge_candidates_authored,
        graph.counters.merge_candidates_geometric,
        graph.counters.diagnostics_warning,
        graph.counters.diagnostics_error,
        graph.counters.nvex_targets_outside_cell,
        graph.counters.nvex_targets_inside_cell,
        graph.counters.nvci_subrecords,
        graph.counters.nvci_entries,
        graph.counters.nvci_door_matches,
        graph.counters.nvci_navmesh_matches,
    );

    if navmeshes.is_empty() {
        return Ok((Vec::new(), None, None, summary));
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
    let graph_source = nav_graph_source(artifact.relative_path, artifact.hash, &graph);
    Ok((sources, Some(graph_source), Some(graph), summary))
}

/// Builds the manifest-level `PreparedNavGraphSource` pointer from a written
/// artifact and its graph counters. Shared by `stage_navmeshes` (initial
/// write) and `apply_nav_clearance` (rewrite after clearance) so both agree
/// on exactly which counters the pointer carries.
fn nav_graph_source(
    asset_path: String,
    hash: String,
    graph: &PreparedNavGraph,
) -> PreparedNavGraphSource {
    PreparedNavGraphSource {
        asset_path,
        revision: NAV_GRAPH_REVISION.into(),
        hash,
        mesh_count: graph.counters.meshes,
        polygon_count: graph.counters.polygons,
        vertex_count: graph.counters.vertices,
        door_count: graph.counters.doors,
        external_connection_count: graph.counters.external_connections,
        mesh_merge_count: graph.counters.mesh_merges,
        diagnostics_warning: graph.counters.diagnostics_warning,
        diagnostics_error: graph.counters.diagnostics_error,
    }
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
        // Issue #156 feature 3: correlation-only, boundary-converted 1:1
        // from `openmw_esm4::navmesh::NaviCorrelation`/`NaviCorrelationEntry`.
        navi_correlations: navigation
            .map(|navi| {
                navi.correlations
                    .iter()
                    .map(|correlation| NavGraphNaviCorrelationInput {
                        leading_navmesh_form_id: correlation.leading_navmesh_form_id,
                        entries: correlation
                            .entries
                            .iter()
                            .map(|entry| NavGraphNaviCorrelationEntryInput {
                                navmesh_form_id: entry.navmesh_form_id,
                                other_navmesh_form_id: entry.other_navmesh_form_id,
                                door_form_id: entry.door_form_id,
                            })
                            .collect(),
                    })
                    .collect()
            })
            .unwrap_or_default(),
    }
}

// ---------------------------------------------------------------------
// Collision-derived validation + clearance wiring (issue #153, M4 wave 10)
// ---------------------------------------------------------------------

/// Extracts this cell's cooked *static* collision as Bevy-metre world-space
/// triangles for the pure `nav_clearance` pass. Consumes the already-cooked
/// physics sidecars (never re-cooks); mirrors the runtime static-collider
/// build (`viewer::player::collision::create_prepared_shape` with
/// `local_space = false`): a shape point in model-local space maps to world
/// as `placement_rotation * (point * placement_scale) + placement_translation`.
///
/// Only initially-enabled `Static`-classified placements with a solid,
/// player-blocking body contribute -- doors (kinematic), actors, and dynamic
/// props are excluded, so the nav graph is validated against the same fixed
/// world shell the player's own capsule collides with. Deterministic:
/// placements are visited in `(reference_form_id, base_form_id)` order.
pub(crate) fn cell_static_collision_triangles(
    placements: &[PreparedPlacement],
    physics_assets: &HashMap<String, PreparedPhysicsAsset>,
) -> Vec<CollisionTriangle> {
    let mut ordered: Vec<&PreparedPlacement> = placements
        .iter()
        .filter(|placement| static_collision_placement(placement))
        .collect();
    ordered.sort_by_key(|placement| (placement.reference_form_id, placement.base_form_id));

    let mut triangles = Vec::new();
    for placement in ordered {
        let Some(path) = placement.physics_asset_path.as_ref() else {
            continue;
        };
        let Some(asset) = physics_assets.get(path) else {
            continue;
        };
        let scale = placement.scale.abs().max(0.0001);
        let transform = Mat4::from_scale_rotation_translation(
            Vec3::splat(scale),
            Quat::from_array(placement.rotation_xyzw).normalize(),
            Vec3::from_array(placement.translation),
        );
        for body in &asset.bodies {
            if !body_blocks_player(body) {
                continue;
            }
            for shape in &body.shapes {
                append_shape_world_triangles(&transform, shape, &mut triangles);
            }
        }
    }
    triangles
}

/// Whether `placement` contributes to the static nav-collision shell: an
/// initially-enabled, `Static`-classified world scenery reference that is not
/// a door/actor. Kinematic (animated door) and dynamic bodies are excluded --
/// they are not part of the fixed shell the authored NAVM should agree with.
fn static_collision_placement(placement: &PreparedPlacement) -> bool {
    placement.initially_enabled
        && placement.physics_asset_path.is_some()
        && placement.physics_classification == PreparedPhysicsClassification::Static
        && !matches!(
            placement.semantic,
            PreparedSemantic::Door(_) | PreparedSemantic::Npc(_) | PreparedSemantic::Creature(_)
        )
}

/// Appends one prepared physics shape's world-space triangles under
/// `transform`. `Box`/`TriangleMesh` are emitted exactly; `Sphere`/`Capsule`/
/// `ConvexHull` fall back to their conservative world-space AABB box (rare for
/// static room geometry -- floors, walls, and frames are boxes/meshes).
fn append_shape_world_triangles(
    transform: &Mat4,
    shape: &PreparedPhysicsShape,
    out: &mut Vec<CollisionTriangle>,
) {
    let xf = |p: [f32; 3]| transform.transform_point3(Vec3::from_array(p)).to_array();
    match shape {
        PreparedPhysicsShape::Box {
            center,
            half_extents,
            rotation_xyzw,
        } => {
            let center = Vec3::from_array(*center);
            let rot = Quat::from_array(*rotation_xyzw).normalize();
            let he = Vec3::from_array(*half_extents);
            let mut corners = [[0.0f32; 3]; 8];
            for (i, corner) in corners.iter_mut().enumerate() {
                let sign = Vec3::new(
                    if i & 1 == 0 { -1.0 } else { 1.0 },
                    if i & 2 == 0 { -1.0 } else { 1.0 },
                    if i & 4 == 0 { -1.0 } else { 1.0 },
                );
                let local = center + rot * (he * sign);
                *corner = transform.transform_point3(local).to_array();
            }
            append_box_faces(&corners, out);
        }
        PreparedPhysicsShape::TriangleMesh { vertices, indices } => {
            for triangle in indices.chunks_exact(3) {
                let (Some(&a), Some(&b), Some(&c)) = (
                    vertices.get(triangle[0] as usize),
                    vertices.get(triangle[1] as usize),
                    vertices.get(triangle[2] as usize),
                ) else {
                    continue;
                };
                out.push(CollisionTriangle {
                    vertices: [xf(a), xf(b), xf(c)],
                });
            }
        }
        PreparedPhysicsShape::Sphere { center, radius } => {
            append_aabb_box(&aabb_corners(&[*center], *radius, transform), out);
        }
        PreparedPhysicsShape::Capsule {
            point1,
            point2,
            radius,
        } => {
            append_aabb_box(&aabb_corners(&[*point1, *point2], *radius, transform), out);
        }
        PreparedPhysicsShape::ConvexHull { points } => {
            append_aabb_box(&aabb_corners(points, 0.0, transform), out);
        }
    }
}

/// World-space AABB corners enclosing `points` (each inflated by `radius`),
/// transformed by `transform`. Corners are ordered by the same
/// `(x, y, z)` sign bit convention `append_box_faces` expects.
fn aabb_corners(points: &[[f32; 3]], radius: f32, transform: &Mat4) -> [[f32; 3]; 8] {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for point in points {
        for axis in 0..3 {
            min[axis] = min[axis].min(point[axis] - radius);
            max[axis] = max[axis].max(point[axis] + radius);
        }
    }
    let mut corners = [[0.0f32; 3]; 8];
    for (i, corner) in corners.iter_mut().enumerate() {
        let local = Vec3::new(
            if i & 1 == 0 { min[0] } else { max[0] },
            if i & 2 == 0 { min[1] } else { max[1] },
            if i & 4 == 0 { min[2] } else { max[2] },
        );
        *corner = transform.transform_point3(local).to_array();
    }
    corners
}

fn append_aabb_box(corners: &[[f32; 3]; 8], out: &mut Vec<CollisionTriangle>) {
    append_box_faces(corners, out);
}

/// Emits the 12 triangles of a box given its 8 corners (indexed by the
/// `(x, y, z)` low/high sign bits: bit 0 = x, bit 1 = y, bit 2 = z).
fn append_box_faces(corners: &[[f32; 3]; 8], out: &mut Vec<CollisionTriangle>) {
    // Six faces, each two triangles, referencing corner indices.
    const FACES: [[usize; 4]; 6] = [
        [0, 1, 3, 2], // y-min (bottom)
        [4, 6, 7, 5], // y-max (top)
        [0, 4, 5, 1], // z-min
        [2, 3, 7, 6], // z-max
        [0, 2, 6, 4], // x-min
        [1, 5, 7, 3], // x-max
    ];
    for face in FACES {
        out.push(CollisionTriangle {
            vertices: [corners[face[0]], corners[face[1]], corners[face[2]]],
        });
        out.push(CollisionTriangle {
            vertices: [corners[face[0]], corners[face[2]], corners[face[3]]],
        });
    }
}

/// Seam/door protected edges for one prepared mesh (issue #153): every edge
/// of a triangle that is a door attachment (`mesh.doors`) or a cross-mesh
/// merge participant naming this mesh's own `form_id` -- the same rule the
/// retired runtime erosion pass used (`landmass_graph::protected_edges_for_mesh`),
/// replicated prepare-side against `PreparedNavMesh`. These vertices never
/// move and are never dropped, keeping both sides of a seam agreeing exactly.
fn protected_edges_for_prepared_mesh(
    mesh: &PreparedNavMesh,
    merges: &[PreparedNavMeshMerge],
) -> Vec<(u32, u32)> {
    use std::collections::BTreeSet;
    let mut protected_triangles: BTreeSet<u32> = BTreeSet::new();
    for door in &mesh.doors {
        protected_triangles.insert(door.triangle_index);
    }
    for merge in merges {
        if merge.mesh_a_form_id == mesh.form_id {
            protected_triangles.insert(merge.triangle_a);
        }
        if merge.mesh_b_form_id == mesh.form_id {
            protected_triangles.insert(merge.triangle_b);
        }
    }
    let mut edges = Vec::new();
    for polygon in &mesh.polygons {
        if protected_triangles.contains(&polygon.index) {
            let [a, b, c] = polygon.vertex_indices;
            edges.push((a, b));
            edges.push((b, c));
            edges.push((c, a));
        }
    }
    edges
}

/// Largest-connected-component share as an integer percent of a mesh's
/// surviving walkable polygons (100 when unfragmented; 100 when the mesh has
/// no walkable polygons at all, so an empty mesh never drags the health
/// minimum down).
fn component_share_pct(largest_component: usize, walkable_count: usize) -> usize {
    // `checked_div` yields `None` (=> 100%) when there are no walkable
    // polygons, so an empty mesh never drags the health minimum down.
    (largest_component * 100)
        .checked_div(walkable_count)
        .unwrap_or(100)
}

/// Per-mesh cap on individually listed clearance drops (issue #171). Wave 10's
/// whole-triangle verdicts produced a handful per cell; sub-triangle clipping
/// produces far more, far smaller ones, and listing every one would dominate
/// the prepared manifest. The remainder is reported as a single count line.
const DROP_DIAGNOSTIC_CAP: usize = 32;

/// Writes one clearance pass's re-triangulated geometry (issue #171) back into
/// the prepared mesh.
///
/// The clip guarantees authored vertices keep their indices and that authored
/// polygon `i`'s first surviving piece occupies polygon slot `i`, so
/// everything keyed by either -- `PreparedNavDoor::triangle_index`,
/// `PreparedNavMeshMerge::triangle_a`/`edge_a`, `cover_triangle_indices` --
/// stays valid. Protected (door/merge) polygons are additionally never split
/// at all, so a door polygon is still exactly the authored triangle its
/// runtime link geometry was derived from.
///
/// Each piece inherits its authored polygon's flags. `authored_external` is
/// only inherited by a piece that *is* the authored triangle: a per-edge
/// authored flag has no meaning on an edge the authored data never described.
/// Same-mesh `adjacency` is recomputed from shared vertex indices over the new
/// polygon list, which is also exactly how the runtime derives it.
fn apply_clipped_geometry(mesh: &mut PreparedNavMesh, result: &NavClearanceResult) {
    let authored = std::mem::take(&mut mesh.polygons);
    mesh.vertices = result.vertices.clone();

    let mut edge_owners: HashMap<(u32, u32), Vec<usize>> = HashMap::new();
    let valid = |index: u32| (index as usize) < result.vertices.len();
    for (index, tri) in result.polygons.iter().enumerate() {
        for &(a, b) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
            if valid(a) && valid(b) {
                edge_owners
                    .entry(if a <= b { (a, b) } else { (b, a) })
                    .or_default()
                    .push(index);
            }
        }
    }

    mesh.polygons = result
        .polygons
        .iter()
        .enumerate()
        .map(|(index, tri)| {
            let template = result
                .sources
                .get(index)
                .and_then(|&source| authored.get(source as usize));
            let identical = template.is_some_and(|source| source.vertex_indices == *tri);
            let mut adjacency = [None; 3];
            for (slot, &(a, b)) in [(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])]
                .iter()
                .enumerate()
            {
                if !valid(a) || !valid(b) {
                    continue;
                }
                let key = if a <= b { (a, b) } else { (b, a) };
                adjacency[slot] = edge_owners.get(&key).and_then(|owners| {
                    owners
                        .iter()
                        .find(|&&owner| owner != index)
                        .map(|&owner| owner as u32)
                });
            }
            PreparedNavPolygon {
                index: index as u32,
                vertex_indices: *tri,
                adjacency,
                flags: template.map(|source| source.flags).unwrap_or_default(),
                is_water: template.is_some_and(|source| source.is_water),
                is_preferred_pathing: template.is_some_and(|source| source.is_preferred_pathing),
                contains_door: template.is_some_and(|source| source.contains_door),
                authored_external: match template {
                    Some(source) if identical => source.authored_external,
                    _ => [false; 3],
                },
                walkable: result.walkable.get(index).copied().unwrap_or(true),
            }
        })
        .collect();
}

/// Runs the collision-derived validation pass (issue #153) over every mesh in
/// `graph`, mutating each polygon's `walkable` flag in place: triangles with
/// no collision support are removed (F153.1), interior triangles a
/// non-step-overable collider intrudes are cut (F153.2), and triangles the
/// agent capsule fits nowhere in are dropped (F153.3) -- measured on the
/// authored geometry, so a wide passage keeps its connected center band while
/// a genuinely sub-diameter throat drops. Vertices are never moved. Rewrites
/// `navgraph.ron`, updates the graph's clearance counters (including the
/// per-mesh connectivity health signal), pushes per-mesh diagnostics, and
/// returns the refreshed manifest pointer and a deterministic summary line.
/// Runs after physics classification so `collision` reflects the same fixed
/// static shell the player collides with.
pub(crate) fn apply_nav_clearance(
    cache_dir: &Path,
    cell_form_id: u32,
    graph: &mut PreparedNavGraph,
    collision: &[CollisionTriangle],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(PreparedNavGraphSource, String)> {
    let params = NavClearanceParams::default();
    let merges = graph.mesh_merges.clone();

    let mut removed = 0usize;
    let mut cut = 0usize;
    let mut dropped = 0usize;
    let mut walkable_total = 0usize;
    let mut min_component_share = 100usize;
    let mut clipped_polygons = 0usize;
    let mut added_vertices = 0usize;
    let mut degenerate_discarded = 0usize;
    let mut predicate_evaluations = 0usize;

    for mesh in &mut graph.meshes {
        let authored_polygon_count = mesh.polygons.len();
        let protected_edges = protected_edges_for_prepared_mesh(mesh, &merges);
        let input = NavClearanceMeshInput {
            vertices: mesh.vertices.clone(),
            polygons: mesh
                .polygons
                .iter()
                .map(|polygon| polygon.vertex_indices)
                .collect(),
            protected_edges,
        };
        let result = validate_and_clear(&input, collision, params);

        // Per-drop centroid diagnostics (Bevy-metre world space): exactly
        // which polygons the pass dropped and why, so a corridor/route
        // regression is locatable from the prepared manifest without a viewer.
        // Capped per mesh (issue #171: sub-triangle clipping produces far more,
        // far smaller drops than wave 10's whole-triangle verdicts, and an
        // uncapped list would dominate the manifest) with a deterministic
        // remainder line.
        let mut emitted = 0usize;
        let mut suppressed = 0usize;
        for (index, reason) in result.reasons.iter().enumerate() {
            let Some(reason) = reason else { continue };
            if emitted >= DROP_DIAGNOSTIC_CAP {
                suppressed += 1;
                continue;
            }
            let tri = result.polygons[index];
            let (Some(&a), Some(&b), Some(&c)) = (
                result.vertices.get(tri[0] as usize),
                result.vertices.get(tri[1] as usize),
                result.vertices.get(tri[2] as usize),
            ) else {
                continue;
            };
            let centroid = [
                (a[0] + b[0] + c[0]) / 3.0,
                (a[1] + b[1] + c[1]) / 3.0,
                (a[2] + b[2] + c[2]) / 3.0,
            ];
            emitted += 1;
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!(
                    "nav clearance drop mesh {:08x} polygon {} {} at ({:.2}, {:.2}, {:.2})",
                    mesh.form_id,
                    index,
                    reason.label(),
                    centroid[0],
                    centroid[1],
                    centroid[2],
                ),
            });
        }
        if suppressed > 0 {
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!(
                    "nav clearance drop mesh {:08x}: {} further drop(s) not listed",
                    mesh.form_id, suppressed,
                ),
            });
        }

        // Non-main (stranded) walkable islands: size + a world-space centroid,
        // so a disconnected corridor/room is locatable from the manifest.
        for (size, centroid) in &result.nonmain_components {
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!(
                    "nav clearance island mesh {:08x}: {} polygon(s) near ({:.2}, {:.2}, {:.2})",
                    mesh.form_id, size, centroid[0], centroid[1], centroid[2],
                ),
            });
        }

        apply_clipped_geometry(mesh, &result);

        removed += result.removed_unsupported;
        cut += result.cut_obstructed;
        dropped += result.dropped_unfit;
        walkable_total += result.walkable_count;
        clipped_polygons += result.clipped_polygons;
        added_vertices += result.added_vertices;
        degenerate_discarded += result.degenerate_discarded;
        predicate_evaluations += result.predicate_evaluations;
        let share = component_share_pct(result.largest_component, result.walkable_count);
        min_component_share = min_component_share.min(share);

        let baseline_share =
            component_share_pct(result.baseline_largest_component, result.polygon_count);
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: format!(
                "nav clearance mesh {:08x}: removed {}, cut {}, dropped {} of {} polygon(s); walkable {}, components {} (largest {} = {}%); authored components {} (largest {} = {}%)",
                mesh.form_id,
                result.removed_unsupported,
                result.cut_obstructed,
                result.dropped_unfit,
                result.polygon_count,
                result.walkable_count,
                result.component_count,
                result.largest_component,
                share,
                result.baseline_component_count,
                result.baseline_largest_component,
                baseline_share,
            ),
        });
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: format!(
                "nav clearance clip mesh {:08x}: authored polygons {} re-triangulated to {} ({} clipped), vertices +{}, slivers discarded {}",
                mesh.form_id,
                authored_polygon_count,
                result.polygon_count,
                result.clipped_polygons,
                result.added_vertices,
                result.degenerate_discarded,
            ),
        });
    }

    graph.counters.clearance_removed_unsupported = removed;
    graph.counters.clearance_cut_obstructed = cut;
    graph.counters.clearance_dropped_unfit = dropped;
    graph.counters.clearance_walkable_total = walkable_total;
    graph.counters.clearance_min_component_share_pct = min_component_share;
    graph.counters.clearance_collision_triangles = collision.len();
    graph.counters.clearance_clipped_polygons = clipped_polygons;
    graph.counters.clearance_added_vertices = added_vertices;
    graph.counters.polygons = graph.meshes.iter().map(|mesh| mesh.polygons.len()).sum();
    graph.counters.vertices = graph.meshes.iter().map(|mesh| mesh.vertices.len()).sum();

    let artifact = write_nav_graph(cache_dir, cell_form_id, graph)?;
    let summary = format!(
        "nav clearance: collision triangles {}, meshes {}, polygons {} (clipped {}, vertices +{}, slivers {}), removed unsupported {}, cut obstructed {}, dropped unfit {}, walkable {}, smallest largest-component share {}%, predicate evaluations {}",
        collision.len(),
        graph.counters.meshes,
        graph.counters.polygons,
        clipped_polygons,
        added_vertices,
        degenerate_discarded,
        removed,
        cut,
        dropped,
        walkable_total,
        min_component_share,
        predicate_evaluations,
    );
    let graph_source = nav_graph_source(artifact.relative_path, artifact.hash, graph);
    Ok((graph_source, summary))
}
