use super::*;

fn quad() -> (Vec<[f32; 3]>, Vec<[u32; 3]>) {
    (
        vec![
            [0.0, 0.0, 0.0],
            [4.0, 0.0, 0.0],
            [4.0, 0.0, 4.0],
            [0.0, 0.0, 4.0],
        ],
        vec![[0, 1, 2], [0, 2, 3]],
    )
}

/// Every output triangle's edges either bound the region or are shared by
/// exactly two triangles, and no vertex lies strictly inside another
/// triangle's edge (the T-junction test): conformity, which is what keeps
/// landmass's shared-index adjacency intact.
fn assert_conformal(output: &ClipOutput) {
    let mut counts: BTreeMap<(u32, u32), usize> = BTreeMap::new();
    for tri in &output.triangles {
        let [a, b, c] = tri.vertex_indices;
        for edge in [edge_key(a, b), edge_key(b, c), edge_key(c, a)] {
            *counts.entry(edge).or_insert(0) += 1;
        }
    }
    for &(a, b) in counts.keys() {
        let (pa, pb) = (output.vertices[a as usize], output.vertices[b as usize]);
        for (index, point) in output.vertices.iter().enumerate() {
            if index as u32 == a || index as u32 == b {
                continue;
            }
            // Only vertices actually referenced by a triangle matter.
            if !output
                .triangles
                .iter()
                .any(|tri| tri.vertex_indices.contains(&(index as u32)))
            {
                continue;
            }
            let ex = pb[0] - pa[0];
            let ez = pb[2] - pa[2];
            let len_sq = ex * ex + ez * ez;
            if len_sq < 1.0e-12 {
                continue;
            }
            let t = ((point[0] - pa[0]) * ex + (point[2] - pa[2]) * ez) / len_sq;
            if !(0.01..=0.99).contains(&t) {
                continue;
            }
            let cx = pa[0] + ex * t;
            let cz = pa[2] + ez * t;
            let d = ((point[0] - cx).powi(2) + (point[2] - cz).powi(2)).sqrt();
            assert!(
                d > 1.0e-3,
                "T-junction: vertex {index} lies on edge ({a}, {b})"
            );
        }
    }
}

#[test]
fn a_uniform_predicate_leaves_the_mesh_untouched() {
    let (vertices, polygons) = quad();
    let output = refine_and_clip(
        &vertices,
        &polygons,
        &BTreeSet::new(),
        &|_| true,
        ClipParams::default(),
    );
    assert_eq!(output.vertices, vertices);
    assert_eq!(output.triangles.len(), 2);
    assert!(output.triangles.iter().all(|tri| tri.inside));
    assert_eq!(output.refinement_splits, 0);
}

#[test]
fn a_half_plane_cut_lands_on_the_boundary_and_stays_conformal() {
    // Walkable only where x < 1.7: every surviving vertex must be at or
    // left of that line, and the cut must introduce no T-junction.
    let (vertices, polygons) = quad();
    let output = refine_and_clip(
        &vertices,
        &polygons,
        &BTreeSet::new(),
        &|p| p[0] < 1.7,
        ClipParams::default(),
    );
    assert!(output.boundary_crossings > 0);
    assert_conformal(&output);
    for tri in &output.triangles {
        if !tri.inside {
            continue;
        }
        for index in tri.vertex_indices {
            assert!(
                output.vertices[index as usize][0] <= 1.7 + 1.0e-2,
                "walkable vertex past the boundary: {:?}",
                output.vertices[index as usize]
            );
        }
    }
}

#[test]
fn an_interior_hole_smaller_than_a_triangle_is_resolved_by_refinement() {
    // A 0.6 m obstruction disc sitting in the middle of a 4 m quad: no
    // authored vertex is inside it, so only refinement can find it.
    let (vertices, polygons) = quad();
    let hole = |p: [f32; 3]| {
        let dx = p[0] - 2.0;
        let dz = p[2] - 2.0;
        (dx * dx + dz * dz).sqrt() > 0.6
    };
    let output = refine_and_clip(
        &vertices,
        &polygons,
        &BTreeSet::new(),
        &hole,
        ClipParams::default(),
    );
    assert!(
        output.triangles.iter().any(|tri| !tri.inside),
        "the interior hole must be cut out"
    );
    assert!(
        output.triangles.iter().any(|tri| tri.inside),
        "the surrounding floor must survive"
    );
    assert_conformal(&output);
}

#[test]
fn both_sides_of_every_cut_are_emitted() {
    // Area is conserved: the walkable and unwalkable pieces together still
    // cover the authored quad, so the caller's connectivity guard can
    // un-drop a piece without breaking conformity.
    let (vertices, polygons) = quad();
    let output = refine_and_clip(
        &vertices,
        &polygons,
        &BTreeSet::new(),
        &|p| p[0] < 1.7,
        ClipParams::default(),
    );
    let total: f32 = output
        .triangles
        .iter()
        .map(|tri| {
            area_xz(
                output.vertices[tri.vertex_indices[0] as usize],
                output.vertices[tri.vertex_indices[1] as usize],
                output.vertices[tri.vertex_indices[2] as usize],
            )
        })
        .sum();
    assert!((total - 16.0).abs() < 0.01, "area {total} != 16");
}

#[test]
fn a_locked_edge_is_never_split_or_crossed() {
    let (vertices, polygons) = quad();
    let mut locked = BTreeSet::new();
    for tri in &polygons {
        for edge in [
            edge_key(tri[0], tri[1]),
            edge_key(tri[1], tri[2]),
            edge_key(tri[2], tri[0]),
        ] {
            locked.insert(edge);
        }
    }
    // The predicate reports the locked polygons walkable everywhere, the
    // way the caller's protected-region rule does.
    let output = refine_and_clip(
        &vertices,
        &polygons,
        &locked,
        &|_| true,
        ClipParams::default(),
    );
    assert_eq!(output.triangles.len(), 2);
    assert_eq!(output.vertices.len(), vertices.len());
}

#[test]
fn an_invalid_vertex_slot_passes_through_walkable() {
    let (vertices, mut polygons) = quad();
    polygons[1] = [0, 2, u32::MAX];
    let output = refine_and_clip(
        &vertices,
        &polygons,
        &BTreeSet::new(),
        &|p| p[0] < 1.7,
        ClipParams::default(),
    );
    let passthrough: Vec<_> = output
        .triangles
        .iter()
        .filter(|tri| tri.source == 1)
        .collect();
    assert_eq!(passthrough.len(), 1);
    assert!(passthrough[0].inside);
    assert_eq!(passthrough[0].vertex_indices, [0, 2, u32::MAX]);
}

/// The property the blocker was: landmass rejects an *entire* mesh over
/// one polygon that is degenerate or wound against the rest, so every
/// triangle this module emits must carry real area and a consistent
/// winding -- including under predicates whose boundary falls exactly on,
/// or a hair away from, an existing vertex, which is where near-collinear
/// pieces come from.
#[test]
fn every_emitted_triangle_is_a_valid_navigation_polygon() {
    let (vertices, polygons) = quad();
    type NamedPredicate<'a> = (&'a str, &'a dyn Fn([f32; 3]) -> bool);
    let predicates: [NamedPredicate; 6] = [
        ("crossing on a vertex", &|p: [f32; 3]| p[0] < 0.0),
        ("crossing on the far vertex", &|p: [f32; 3]| p[0] < 4.0),
        ("crossing a hair off a vertex", &|p: [f32; 3]| p[0] < 1.0e-6),
        ("crossing along the diagonal", &|p: [f32; 3]| p[0] < p[2]),
        ("crossing a hair off the diagonal", &|p: [f32; 3]| {
            p[0] < p[2] + 1.0e-6
        }),
        ("a tiny interior island", &|p: [f32; 3]| {
            (p[0] - 2.0).abs() > 1.0e-4 || (p[2] - 2.0).abs() > 1.0e-4
        }),
    ];
    for (name, predicate) in predicates {
        let params = ClipParams::default();
        let output = refine_and_clip(&vertices, &polygons, &BTreeSet::new(), predicate, params);
        let mut signs = Vec::new();
        for tri in &output.triangles {
            let [a, b, c] = tri.vertex_indices;
            assert!(
                a != b && b != c && c != a,
                "{name}: emitted a triangle with a repeated vertex"
            );
            let (pa, pb, pc) = (
                output.vertices[a as usize],
                output.vertices[b as usize],
                output.vertices[c as usize],
            );
            let signed =
                ((pb[0] - pa[0]) * (pc[2] - pa[2]) - (pc[0] - pa[0]) * (pb[2] - pa[2])) * 0.5;
            assert!(
                signed.abs() >= params.min_area,
                "{name}: emitted a degenerate triangle (area {signed:e})"
            );
            signs.push(signed > 0.0);
        }
        assert!(
            signs.iter().all(|&s| s == signs[0]),
            "{name}: emitted triangles with inconsistent winding"
        );
    }
}

#[test]
fn welding_a_sliver_removes_it_without_punching_a_hole() {
    // A predicate whose boundary sits a hair off the diagonal produces
    // slivers along it. They must be welded away, not discarded: the
    // covered area is conserved, so no adjacency is severed.
    let (vertices, polygons) = quad();
    let output = refine_and_clip(
        &vertices,
        &polygons,
        &BTreeSet::new(),
        &|p| p[0] < p[2] + 1.0e-6,
        ClipParams::default(),
    );
    let total: f32 = output
        .triangles
        .iter()
        .map(|tri| {
            area_xz(
                output.vertices[tri.vertex_indices[0] as usize],
                output.vertices[tri.vertex_indices[1] as usize],
                output.vertices[tri.vertex_indices[2] as usize],
            )
        })
        .sum();
    assert!(
        (total - 16.0).abs() < 0.01,
        "welding must conserve the covered area, got {total}"
    );
}

#[test]
fn the_pass_is_deterministic_across_calls() {
    let (vertices, polygons) = quad();
    let run = || {
        refine_and_clip(
            &vertices,
            &polygons,
            &BTreeSet::new(),
            &|p| p[0] * p[0] + p[2] * p[2] > 2.0,
            ClipParams::default(),
        )
    };
    assert_eq!(run(), run());
}
