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
