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

/// Number of samples used to approximate a round shape's XZ footprint.
/// Purely a discretization constant: higher is a closer approximation of the
/// same circle, and it is not derived from any placement or cell.
const ROUND_FOOTPRINT_SAMPLES: usize = 12;

/// Appends one prepared physics shape's world-space triangles under
/// `transform`. `Box`/`TriangleMesh` are emitted exactly.
///
/// `Sphere`/`Capsule`/`ConvexHull` have no triangles of their own, so they are
/// emitted as the vertical prism over their true XZ footprint (issue #171):
/// the convex hull of the shape's world-space sample points projected to XZ,
/// extruded over the shape's world height range. The previous conservative
/// world-space AABB was adequate while clearance judged whole authored
/// triangles, but sub-triangle clipping erodes the agent radius from every
/// collider footprint, and an AABB around a *diagonal* capsule or hull covers
/// an enormous area the collider does not occupy -- a long railing or pipe
/// would erode the platform it runs along. Over half this cell class's static
/// shapes are capsules and hulls, so the footprint has to be the real one.
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
            append_footprint_prism(&round_shape_points(&[*center], *radius, transform), out);
        }
        PreparedPhysicsShape::Capsule {
            point1,
            point2,
            radius,
        } => {
            append_footprint_prism(
                &round_shape_points(&[*point1, *point2], *radius, transform),
                out,
            );
        }
        PreparedPhysicsShape::ConvexHull { points } => {
            append_footprint_prism(&points.iter().map(|&p| xf(p)).collect::<Vec<_>>(), out);
        }
    }
}

/// World-space sample points describing a sphere/capsule: a horizontal ring of
/// [`ROUND_FOOTPRINT_SAMPLES`] points at each centre, plus the poles, all at
/// the world radius. Hulling these in XZ reproduces the shape's true footprint
/// (a disc for a sphere, a stadium for a capsule) instead of its bounding box.
fn round_shape_points(centres: &[[f32; 3]], radius: f32, transform: &Mat4) -> Vec<[f32; 3]> {
    // The placement transform is a uniform scale + rotation + translation, so
    // a local radius maps to the world by that single scale factor.
    let scale = transform.x_axis.truncate().length().max(1.0e-6);
    let world_radius = radius * scale;
    let mut points = Vec::with_capacity(centres.len() * (ROUND_FOOTPRINT_SAMPLES + 2));
    for centre in centres {
        let c = transform.transform_point3(Vec3::from_array(*centre));
        for i in 0..ROUND_FOOTPRINT_SAMPLES {
            let angle = std::f32::consts::TAU * (i as f32) / (ROUND_FOOTPRINT_SAMPLES as f32);
            points.push([
                c.x + world_radius * angle.cos(),
                c.y,
                c.z + world_radius * angle.sin(),
            ]);
        }
        points.push([c.x, c.y + world_radius, c.z]);
        points.push([c.x, c.y - world_radius, c.z]);
    }
    points
}

/// Emits the vertical prism over `points`' XZ convex hull, spanning their
/// world height range: side quads (the wall-like surfaces an agent must clear)
/// plus a top and bottom fan (the surfaces that can support one).
fn append_footprint_prism(points: &[[f32; 3]], out: &mut Vec<CollisionTriangle>) {
    let hull = convex_hull_xz(points);
    if hull.len() < 3 {
        return;
    }
    let mut y_min = f32::INFINITY;
    let mut y_max = f32::NEG_INFINITY;
    for point in points {
        y_min = y_min.min(point[1]);
        y_max = y_max.max(point[1]);
    }
    for i in 0..hull.len() {
        let a = hull[i];
        let b = hull[(i + 1) % hull.len()];
        out.push(CollisionTriangle {
            vertices: [
                [a[0], y_min, a[1]],
                [b[0], y_min, b[1]],
                [b[0], y_max, b[1]],
            ],
        });
        out.push(CollisionTriangle {
            vertices: [
                [a[0], y_min, a[1]],
                [b[0], y_max, b[1]],
                [a[0], y_max, a[1]],
            ],
        });
    }
    for y in [y_min, y_max] {
        for i in 1..hull.len() - 1 {
            out.push(CollisionTriangle {
                vertices: [
                    [hull[0][0], y, hull[0][1]],
                    [hull[i][0], y, hull[i][1]],
                    [hull[i + 1][0], y, hull[i + 1][1]],
                ],
            });
        }
    }
}

/// XZ convex hull (Andrew's monotone chain) of `points`. Deterministic: the
/// input is sorted by `(x, z)` and near-duplicate points are collapsed first.
fn convex_hull_xz(points: &[[f32; 3]]) -> Vec<[f32; 2]> {
    let mut sorted: Vec<[f32; 2]> = points.iter().map(|p| [p[0], p[2]]).collect();
    sorted.sort_by(|a, b| a[0].total_cmp(&b[0]).then(a[1].total_cmp(&b[1])));
    sorted.dedup_by(|a, b| (a[0] - b[0]).abs() < 1.0e-6 && (a[1] - b[1]).abs() < 1.0e-6);
    if sorted.len() < 3 {
        return sorted;
    }
    let cross = |o: [f32; 2], a: [f32; 2], b: [f32; 2]| {
        (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    };
    let chain = |input: &mut dyn Iterator<Item = [f32; 2]>| -> Vec<[f32; 2]> {
        let mut out: Vec<[f32; 2]> = Vec::new();
        for point in input {
            while out.len() >= 2 && cross(out[out.len() - 2], out[out.len() - 1], point) <= 0.0 {
                out.pop();
            }
            out.push(point);
        }
        out.pop();
        out
    };
    let mut hull = chain(&mut sorted.iter().copied());
    hull.extend(chain(&mut sorted.iter().rev().copied()));
    hull
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

/// Runs `landmass`' own mesh validation over a prepared mesh exactly as the
/// runtime will (issue #171): the same walkable/water/index filtering
/// `viewer::nav::mesh_inputs` plus `landmass_graph::build_navigation_mesh`
/// apply, and the same both-windings retry.
///
/// This exists because a single invalid polygon makes `landmass` reject an
/// *entire* mesh, leaving a cell with no navigation at all -- and prepare-side
/// connectivity metrics cannot see it, because they measure the graph this
/// pass built rather than the graph the runtime is willing to accept. Vault
/// 101 shipped at a healthy 98% component share while having zero usable
/// navigation. Replicating landmass's convexity rule prepare-side (which
/// `nav_clearance::reject_invalid_geometry` does, so bad polygons are dropped
/// with a named reason) is necessary but not sufficient: only running the real
/// validator proves the runtime will accept the result.
///
/// `bevy_landmass` is imported directly rather than through `viewer`, which
/// `tests/architecture.rs` forbids and which would invert the dependency.
fn verify_landmass_acceptance(mesh: &PreparedNavMesh) -> std::result::Result<(), String> {
    use bevy_landmass::NavigationMesh3d;

    let vertex_count = mesh.vertices.len();
    let polygons: Vec<Vec<usize>> = mesh
        .polygons
        .iter()
        .filter(|polygon| polygon.walkable && !polygon.is_water)
        .filter(|polygon| {
            let [a, b, c] = polygon.vertex_indices;
            a != b
                && b != c
                && c != a
                && polygon
                    .vertex_indices
                    .iter()
                    .all(|&index| (index as usize) < vertex_count)
        })
        .map(|polygon| {
            polygon
                .vertex_indices
                .iter()
                .map(|&index| index as usize)
                .collect()
        })
        .collect();
    if polygons.is_empty() {
        return Ok(()); // an empty mesh is the runtime's own documented case.
    }
    let vertices: Vec<Vec3> = mesh
        .vertices
        .iter()
        .map(|v| Vec3::new(v[0], v[1], v[2]))
        .collect();

    let type_indices = vec![0; polygons.len()];
    let mut last = String::new();
    for reversed in [false, true] {
        let wound: Vec<Vec<usize>> = polygons
            .iter()
            .map(|polygon| {
                let mut indices = polygon.clone();
                if reversed {
                    indices.reverse();
                }
                indices
            })
            .collect();
        let candidate = NavigationMesh3d {
            vertices: vertices.clone(),
            polygons: wound,
            polygon_type_indices: type_indices.clone(),
            height_mesh: None,
        };
        match candidate.validate() {
            Ok(_) => return Ok(()),
            Err(error) => last = error.to_string(),
        }
    }
    Err(last)
}

/// Per-mesh cap on individually listed clearance drops (issue #171). Wave 10's
/// whole-triangle verdicts produced a handful per cell; sub-triangle clipping
/// produces far more, far smaller ones, and listing every one would dominate
/// the prepared manifest. The remainder is reported as a single count line.
const DROP_DIAGNOSTIC_CAP: usize = 32;

/// Environment variable naming world points the clearance pass should explain
/// itself at (issue #171). Debug affordance: unset in every normal run, and it
/// only ever adds `println!` lines -- it never changes a verdict.
///
/// Format: `;`-separated entries, each either a point `x,y,z` or a sampled
/// segment `x0,y0,z0>x1,y1,z1@count`. Example:
///
/// ```text
/// BEVYOUT_NAV_PROBE='9.6,106,-73.1>0,106,-73.1@40' cargo run-dev -- prepare --cell 0001a273
/// ```
const NAV_PROBE_ENV: &str = "BEVYOUT_NAV_PROBE";

/// Parses [`NAV_PROBE_ENV`] into world points. Malformed entries are skipped
/// silently -- this is a debug affordance, never a prepare input.
fn nav_probe_points() -> Vec<[f32; 3]> {
    let Ok(spec) = std::env::var(NAV_PROBE_ENV) else {
        return Vec::new();
    };
    let parse = |text: &str| -> Option<[f32; 3]> {
        let mut parts = text.split(',').map(|value| value.trim().parse::<f32>());
        match (parts.next(), parts.next(), parts.next()) {
            (Some(Ok(x)), Some(Ok(y)), Some(Ok(z))) => Some([x, y, z]),
            _ => None,
        }
    };
    let mut points = Vec::new();
    for entry in spec.split(';').map(str::trim).filter(|e| !e.is_empty()) {
        let Some((from, rest)) = entry.split_once('>') else {
            points.extend(parse(entry));
            continue;
        };
        let (to, count) = rest.split_once('@').unwrap_or((rest, "20"));
        let (Some(a), Some(b), Ok(count)) = (parse(from), parse(to), count.trim().parse::<usize>())
        else {
            continue;
        };
        for step in 0..=count.max(1) {
            let t = step as f32 / count.max(1) as f32;
            points.push([
                a[0] + (b[0] - a[0]) * t,
                a[1] + (b[1] - a[1]) * t,
                a[2] + (b[2] - a[2]) * t,
            ]);
        }
    }
    points
}

/// Reports, for every probed point, which walkability term rejected it and
/// what the clipped mesh finally decided -- the covering polygon, its drop
/// reason, and which connected component it ended up in, plus whether any
/// output polygon covers the point at all (which is how a sliver-filtering
/// hole shows up). The predicate terms are evaluated at the covering
/// polygon's own surface height, not at the probe's `y`, since that is the
/// height the clearance pass itself judged the point at.
/// Deterministic `println!` lines, in the CLI logging style.
fn report_nav_probe(
    mesh: &PreparedNavMesh,
    result: &NavClearanceResult,
    collision: &[CollisionTriangle],
    points: &[[f32; 3]],
) {
    let covering = |x: f32, z: f32| -> Option<(usize, f32)> {
        result.polygons.iter().enumerate().find_map(|(index, tri)| {
            let (Some(&a), Some(&b), Some(&c)) = (
                result.vertices.get(tri[0] as usize),
                result.vertices.get(tri[1] as usize),
                result.vertices.get(tri[2] as usize),
            ) else {
                return None;
            };
            let det = (b[0] - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (b[2] - a[2]);
            if det.abs() < 1.0e-9 {
                return None;
            }
            let beta = ((x - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (z - a[2])) / det;
            let gamma = ((b[0] - a[0]) * (z - a[2]) - (x - a[0]) * (b[2] - a[2])) / det;
            let alpha = 1.0 - beta - gamma;
            (alpha >= -1.0e-4 && beta >= -1.0e-4 && gamma >= -1.0e-4)
                .then(|| (index, alpha * a[1] + beta * b[1] + gamma * c[1]))
        })
    };

    // Component labels over the surviving walkable set, so the probe can say
    // whether two reachable-looking points are actually routable to each other.
    let components = walkable_component_labels(result);
    let mut component_sizes: HashMap<usize, usize> = HashMap::new();
    for root in components.iter().flatten() {
        *component_sizes.entry(*root).or_insert(0) += 1;
    }
    let largest = component_sizes.values().copied().max().unwrap_or(0);

    let samples: Vec<[f32; 3]> = points
        .iter()
        .map(|&[x, y, z]| [x, covering(x, z).map(|(_, sy)| sy).unwrap_or(y), z])
        .collect();
    let probes = probe_points(collision, NavClearanceParams::default(), &samples);

    for (probe, original) in probes.iter().zip(points) {
        let [x, y, z] = probe.point;
        let verdict = match covering(x, z) {
            None => "uncovered".to_string(),
            Some((index, _)) => {
                let component = components
                    .get(index)
                    .copied()
                    .flatten()
                    .map(|root| root.to_string())
                    .unwrap_or_else(|| "-".into());
                match result.reasons.get(index).copied().flatten() {
                    None => {
                        let size = components
                            .get(index)
                            .copied()
                            .flatten()
                            .and_then(|root| component_sizes.get(&root).copied())
                            .unwrap_or(0);
                        format!(
                            "walkable polygon {index} component {component} size {size}{}",
                            if size == largest { " (MAIN)" } else { "" }
                        )
                    }
                    Some(reason) => format!("polygon {index} {}", reason.label()),
                }
            }
        };
        let wall = match probe.nearest_wall {
            Some([a, b, c]) => format!(
                " nearest-wall ({:.2},{:.2},{:.2})/({:.2},{:.2},{:.2})/({:.2},{:.2},{:.2})",
                a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]
            ),
            None => String::new(),
        };
        println!(
            "nav clearance probe mesh {:08x} ({x:.2}, {z:.2}) at surface y {y:.2} (probed {:.2}): supported {} (exact {}), obstructed {}, wall-distance {:.3}, {verdict}{wall}",
            mesh.form_id,
            original[1],
            probe.supported,
            probe.supported_exact,
            probe.obstructed,
            probe.nearest_wall_distance,
        );
    }
}

/// Connected-component root per output polygon (`None` when not walkable),
/// over shared vertex-index edges -- exactly the adjacency landmass derives.
fn walkable_component_labels(result: &NavClearanceResult) -> Vec<Option<usize>> {
    let count = result.polygons.len();
    let mut parent: Vec<usize> = (0..count).collect();
    fn find(parent: &mut [usize], mut node: usize) -> usize {
        while parent[node] != node {
            parent[node] = parent[parent[node]];
            node = parent[node];
        }
        node
    }
    let mut owners: HashMap<(u32, u32), usize> = HashMap::new();
    for (index, tri) in result.polygons.iter().enumerate() {
        if !result.walkable[index] {
            continue;
        }
        for &(a, b) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
            let key = if a <= b { (a, b) } else { (b, a) };
            match owners.get(&key) {
                Some(&other) => {
                    let (ra, rb) = (find(&mut parent, index), find(&mut parent, other));
                    if ra != rb {
                        parent[ra] = rb;
                    }
                }
                None => {
                    owners.insert(key, index);
                }
            }
        }
    }
    (0..count)
        .map(|index| result.walkable[index].then(|| find(&mut parent, index)))
        .collect()
}

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
    let mut min_authored_share = 100usize;
    let mut clipped_polygons = 0usize;
    let mut added_vertices = 0usize;
    let mut degenerate_discarded = 0usize;
    let mut collapsed_welds = 0usize;
    let mut invalid_geometry = 0usize;
    let mut predicate_evaluations = 0usize;
    let probe_points_spec = nav_probe_points();

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

        if !probe_points_spec.is_empty() {
            report_nav_probe(mesh, &result, collision, &probe_points_spec);
        }

        apply_clipped_geometry(mesh, &result);

        // F171.5: prove the runtime will accept what we just wrote, before it
        // ships. A failure here is a bug in this pass, not in the cell data.
        if let Err(error) = verify_landmass_acceptance(mesh) {
            diagnostics.push(Diagnostic {
                severity: "error".into(),
                message: format!(
                    "nav clearance mesh {:08x}: landmass would reject this mesh ({error})",
                    mesh.form_id
                ),
            });
            anyhow::bail!(
                "nav clearance produced a mesh landmass rejects (mesh {:08x}: {error}); \
                 the prepared nav graph would leave this cell with no navigation",
                mesh.form_id
            );
        }

        removed += result.removed_unsupported;
        cut += result.cut_obstructed;
        dropped += result.dropped_unfit;
        walkable_total += result.walkable_count;
        clipped_polygons += result.clipped_polygons;
        added_vertices += result.added_vertices;
        degenerate_discarded += result.degenerate_discarded;
        collapsed_welds += result.collapsed_welds;
        invalid_geometry += result.invalid_geometry;
        if result.invalid_geometry > 0 {
            diagnostics.push(Diagnostic {
                severity: "error".into(),
                message: format!(
                    "nav clearance mesh {:08x}: {} polygon(s) rejected as invalid navigation geometry (degenerate, or wound against the mesh)",
                    mesh.form_id, result.invalid_geometry,
                ),
            });
        }
        predicate_evaluations += result.predicate_evaluations;
        let share = component_share_pct(result.largest_component, result.walkable_count);
        min_component_share = min_component_share.min(share);
        let authored_share = component_share_pct(
            result.authored_in_main_component,
            result.authored_polygon_count,
        );
        min_authored_share = min_authored_share.min(authored_share);

        let baseline_share = component_share_pct(
            result.baseline_largest_component,
            result.authored_polygon_count,
        );
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
                "nav clearance clip mesh {:08x}: authored polygons {} re-triangulated to {} ({} clipped), vertices +{}, slivers welded away {} ({} vertex weld(s)), invalid geometry rejected {}, authored polygons reachable in the main component {} = {}%",
                mesh.form_id,
                authored_polygon_count,
                result.polygon_count,
                result.clipped_polygons,
                result.added_vertices,
                result.degenerate_discarded,
                result.collapsed_welds,
                result.invalid_geometry,
                result.authored_in_main_component,
                authored_share,
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
        "nav clearance: collision triangles {}, meshes {}, polygons {} (clipped {}, vertices +{}, slivers welded {} via {} weld(s), invalid rejected {}), removed unsupported {}, cut obstructed {}, dropped unfit {}, walkable {}, smallest largest-component share {}%, smallest authored-reachable share {}%, predicate evaluations {}",
        collision.len(),
        graph.counters.meshes,
        graph.counters.polygons,
        clipped_polygons,
        added_vertices,
        degenerate_discarded,
        collapsed_welds,
        invalid_geometry,
        removed,
        cut,
        dropped,
        walkable_total,
        min_component_share,
        min_authored_share,
        predicate_evaluations,
    );
    let graph_source = nav_graph_source(artifact.relative_path, artifact.hash, graph);
    Ok((graph_source, summary))
}

// ---------------------------------------------------------------------
// Derived blocker -> polygon associations (issue #177, M4 wave 11)
// ---------------------------------------------------------------------

/// Whether `placement` is a *blocking* placement for nav-topology purposes:
/// an initially-enabled reference with a solid, player-blocking body that the
/// static nav-collision shell deliberately excludes because it moves
/// (`static_collision_placement`), and that is not an actor. In practice that
/// is doors (kinematic slabs) plus kinematic activators -- vault gear doors
/// and the like, which are `ACTI` records but block a corridor exactly the
/// way a `DOOR` does.
///
/// Returns `(is_blocker, gated)`; `gated` is `true` only for real `DOOR`
/// records, which own a runtime open/close FSM. A blocker with no way to be
/// opened must never become a crossing-gate candidate (an agent would pause
/// in front of it forever), so it contributes blocking associations only --
/// see `nav_doors::BlockerVolume::gated`.
fn blocker_placement(placement: &PreparedPlacement) -> Option<bool> {
    if !placement.initially_enabled || placement.physics_asset_path.is_none() {
        return None;
    }
    match (&placement.semantic, placement.physics_classification) {
        (PreparedSemantic::Door(_), _) => Some(true),
        (PreparedSemantic::Npc(_) | PreparedSemantic::Creature(_), _) => None,
        (_, PreparedPhysicsClassification::Kinematic) => Some(false),
        _ => None,
    }
}

/// Extracts every blocking placement's collision footprint as a
/// [`BlockerVolume`] in Bevy-metre world space, using the exact same cooked
/// sidecars and placement transform `cell_static_collision_triangles` uses
/// for the static shell. Deterministic: placements are visited in
/// `(reference_form_id, base_form_id)` order and a blocker with no usable
/// footprint (degenerate or missing collision) is skipped.
pub(crate) fn cell_blocker_volumes(
    placements: &[PreparedPlacement],
    physics_assets: &HashMap<String, PreparedPhysicsAsset>,
) -> Vec<BlockerVolume> {
    let mut ordered: Vec<(&PreparedPlacement, bool)> = placements
        .iter()
        .filter_map(|placement| blocker_placement(placement).map(|gated| (placement, gated)))
        .collect();
    ordered.sort_by_key(|(placement, _)| (placement.reference_form_id, placement.base_form_id));

    let mut volumes = Vec::new();
    for (placement, gated) in ordered {
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
        let mut triangles = Vec::new();
        for body in &asset.bodies {
            if !body_blocks_player(body) {
                continue;
            }
            for shape in &body.shapes {
                append_shape_world_triangles(&transform, shape, &mut triangles);
            }
        }
        if triangles.is_empty() {
            continue;
        }
        let points: Vec<[f32; 3]> = triangles
            .iter()
            .flat_map(|triangle| triangle.vertices)
            .collect();
        let footprint = blocker_footprint(&points);
        if footprint.len() < 3 {
            continue;
        }
        let min_y = points
            .iter()
            .fold(f32::INFINITY, |acc, point| acc.min(point[1]));
        let max_y = points
            .iter()
            .fold(f32::NEG_INFINITY, |acc, point| acc.max(point[1]));
        volumes.push(BlockerVolume {
            reference_form_id: placement.reference_form_id,
            footprint,
            min_y,
            max_y,
            gated,
        });
    }
    volumes
}

/// Minimum XZ half-thickness (metres) a blocker footprint is given when its
/// collision geometry is flat. Real authored Havok door collision is
/// routinely a single zero-thickness plane -- `MetroGateLoad` in
/// FranklinMetro02 is exactly that -- whose XZ convex hull is a degenerate
/// line segment with no interior, so every overlap and containment test would
/// silently report nothing and the door would keep its pre-#177 behaviour of
/// not existing at all as far as route topology is concerned. Purely a
/// numerical-robustness floor, not a tuned value: it is the smallest
/// thickness that gives a plane an interior at f32 precision with room to
/// spare, far below anything a nav polygon or agent capsule is measured in,
/// and it is applied identically to every blocker.
const BLOCKER_MIN_HALF_THICKNESS: f32 = 0.05;

/// The XZ footprint of a blocker's collision `points`: its convex hull,
/// thickened to a rectangle of minimum thickness whenever that hull is flat
/// (see [`BLOCKER_MIN_HALF_THICKNESS`]). Flatness is judged by the hull's
/// actual width, not its vertex count: a zero-thickness authored plane
/// survives `convex_hull_xz` as three or four *collinear* points once f32
/// rounding perturbs them off the line, and such a hull has no interior for
/// any overlap or containment test to find.
fn blocker_footprint(points: &[[f32; 3]]) -> Vec<[f32; 2]> {
    let hull = convex_hull_xz(points);
    if hull.len() < 2 {
        return Vec::new();
    }
    // Shoelace area and longest chord: width ~= 2 * area / chord for a thin
    // sliver, which is exactly the shape being detected.
    let mut area = 0.0f32;
    for index in 0..hull.len() {
        let a = hull[index];
        let b = hull[(index + 1) % hull.len()];
        area += a[0] * b[1] - b[0] * a[1];
    }
    let area = (area / 2.0).abs();
    let mut chord = ([0.0f32; 2], [0.0f32; 2]);
    let mut chord_length = 0.0f32;
    for (index, &a) in hull.iter().enumerate() {
        for &b in &hull[index + 1..] {
            let length = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt();
            if length > chord_length {
                chord_length = length;
                chord = (a, b);
            }
        }
    }
    if chord_length < 1.0e-6 {
        return Vec::new();
    }
    if hull.len() >= 3 && 2.0 * area / chord_length >= 2.0 * BLOCKER_MIN_HALF_THICKNESS {
        return hull;
    }
    // Flat: extend the longest chord sideways by the minimum half-thickness,
    // wound counter-clockwise like the hull.
    let (first, last) = chord;
    let direction = [
        (last[0] - first[0]) / chord_length,
        (last[1] - first[1]) / chord_length,
    ];
    let normal = [
        -direction[1] * BLOCKER_MIN_HALF_THICKNESS,
        direction[0] * BLOCKER_MIN_HALF_THICKNESS,
    ];
    vec![
        [first[0] - normal[0], first[1] - normal[1]],
        [last[0] - normal[0], last[1] - normal[1]],
        [last[0] + normal[0], last[1] + normal[1]],
        [first[0] + normal[0], first[1] + normal[1]],
    ]
}

/// Populates every mesh's `PreparedNavMesh::derived_doors` from this cell's
/// blocking placements (issue #177), rewrites `navgraph.ron`, and returns the
/// refreshed manifest pointer plus a deterministic summary line.
///
/// Runs *after* `apply_nav_clearance`, so `triangle_index` refers to the
/// post-clip polygon set the runtime actually loads, and only walkable
/// polygons are considered (a polygon clearance already dropped is not
/// routable and needs no door typing).
///
/// The summary also reports the issue's invariant --
/// `nav_doors::unreported_interior_polygons` -- so a regression that leaves
/// routable ground inside a closed door is visible in `prepare` output
/// without a viewer.
pub(crate) fn apply_derived_door_associations(
    cache_dir: &Path,
    cell_form_id: u32,
    graph: &mut PreparedNavGraph,
    blockers: &[BlockerVolume],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(PreparedNavGraphSource, String)> {
    let mesh_inputs: Vec<BlockerMeshInput> = graph
        .meshes
        .iter()
        .map(|mesh| BlockerMeshInput {
            form_id: mesh.form_id,
            authored_door_polygons: mesh.doors.iter().map(|door| door.triangle_index).collect(),
            polygons: mesh
                .polygons
                .iter()
                .filter(|polygon| polygon.walkable)
                .filter_map(|polygon| {
                    let mut vertices = [[0.0f32; 3]; 3];
                    for (slot, &index) in polygon.vertex_indices.iter().enumerate() {
                        vertices[slot] = *mesh.vertices.get(index as usize)?;
                    }
                    Some(BlockerPolygonInput {
                        index: polygon.index,
                        vertices,
                    })
                })
                .collect(),
        })
        .collect();

    let associations = derive_door_associations(&mesh_inputs, blockers);
    let unreported = unreported_interior_polygons(&mesh_inputs, blockers, &associations);

    for mesh in &mut graph.meshes {
        mesh.derived_doors = associations
            .iter()
            .filter(|association| association.mesh_form_id == mesh.form_id)
            .map(|association| PreparedNavDerivedDoor {
                triangle_index: association.triangle_index,
                door_reference_form_id: association.door_reference_form_id,
                blocks_when_closed: association.blocks_when_closed,
            })
            .collect();
    }

    let blocking = associations
        .iter()
        .filter(|association| association.blocks_when_closed)
        .count();
    let associated_blockers: std::collections::BTreeSet<u32> = associations
        .iter()
        .map(|association| association.door_reference_form_id)
        .collect();

    for (mesh_form_id, triangle_index, reference_form_id) in &unreported {
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: format!(
                "nav doors: mesh {mesh_form_id:08x} polygon {triangle_index} is walkable inside closed blocker {reference_form_id:08x} with no blocking association"
            ),
        });
    }

    let artifact = write_nav_graph(cache_dir, cell_form_id, graph)?;
    let summary = format!(
        "nav doors: blockers {}, associations {} (blocking {}) across {} blocker(s), unreported interior polygons {}",
        blockers.len(),
        associations.len(),
        blocking,
        associated_blockers.len(),
        unreported.len(),
    );
    let graph_source = nav_graph_source(artifact.relative_path, artifact.hash, graph);
    Ok((graph_source, summary))
}

#[cfg(test)]
mod blocker_footprint_tests {
    use super::*;

    /// Real authored Havok door collision is routinely a single
    /// zero-thickness plane (`MetroGateLoad` in FranklinMetro02): f32
    /// rounding leaves `convex_hull_xz` returning three or four *collinear*
    /// points, which has no interior for any overlap or containment test to
    /// find. It must be thickened, not discarded.
    #[test]
    fn a_flat_collision_plane_is_thickened_into_a_footprint() {
        let points: Vec<[f32; 3]> = (0..8)
            .map(|step| {
                [
                    9.558576 + (step % 3) as f32 * 1.0e-5,
                    105.0 + step as f32 * 0.25,
                    -75.0 + step as f32 * 0.5,
                ]
            })
            .collect();
        let footprint = blocker_footprint(&points);
        assert_eq!(footprint.len(), 4, "{footprint:?}");
        let min_x = footprint.iter().fold(f32::INFINITY, |acc, p| acc.min(p[0]));
        let max_x = footprint
            .iter()
            .fold(f32::NEG_INFINITY, |acc, p| acc.max(p[0]));
        assert!(
            (max_x - min_x - 2.0 * BLOCKER_MIN_HALF_THICKNESS).abs() < 1.0e-3,
            "{footprint:?}"
        );
    }

    #[test]
    fn a_solid_box_keeps_its_own_hull() {
        let points = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 2.0, 1.0],
            [0.0, 2.0, 1.0],
        ];
        assert_eq!(blocker_footprint(&points).len(), 4);
    }

    #[test]
    fn a_single_point_has_no_footprint() {
        assert!(blocker_footprint(&[[1.0, 2.0, 3.0]]).is_empty());
    }
}
