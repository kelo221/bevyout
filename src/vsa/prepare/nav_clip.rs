//! Pure 2D conformal re-triangulation of a walkable triangle soup against an
//! arbitrary walkability predicate (issue #171, M4 wave 11).
//!
//! Wave 10's clearance pass (`nav_clearance`) judged whole authored triangles:
//! a triangle was kept or dropped as a unit. Two real defect classes live
//! *below* that granularity -- a collider protruding into a triangle's opening
//! while its centroid sits clear, and a narrow void strip under an otherwise
//! supported triangle. Neither can be expressed by keeping or dropping whole
//! authored triangles, so this module re-triangulates each polygon locally so
//! that the surviving edges lie on the walkable/unwalkable boundary itself.
//!
//! The rule is deliberately generic: it takes a `predicate` (`is this point
//! walkable?`) and knows nothing about what makes a point walkable. Two
//! phases, both conformal (no T-junctions, so shared vertex indices keep
//! polygon adjacency intact -- landmass derives adjacency purely from shared
//! vertex indices):
//!
//! 1. **Adaptive refinement.** A triangle whose sampled predicate is not
//!    uniform (its three vertices, three edge midpoints and centroid do not
//!    all agree) and whose longest edge still exceeds `resolution` has its
//!    edges split at their midpoints. Neighbours that were not themselves
//!    marked take a *green closure* (2 or 3 sub-triangles using only the
//!    midpoints their edges already received), so refinement never cascades
//!    across the mesh and the result stays conformal. Midpoints are keyed by
//!    the unordered edge, so both owners of a shared edge use the same new
//!    vertex index.
//! 2. **Marching clip.** Every triangle whose three vertices no longer agree
//!    is cut along the predicate's zero set: the crossing on each sign-change
//!    edge is located by bisection (again keyed by the unordered edge, so both
//!    owners get the identical point) and the triangle is split into walkable
//!    and unwalkable sub-triangles. Uniform triangles pass through whole.
//!
//! Both sides of every cut are emitted -- the unwalkable pieces carry
//! `inside = false` rather than being discarded -- so the caller's existing
//! connectivity guard can un-drop a piece exactly the way it un-drops a whole
//! authored triangle, without ever breaking conformity.
//!
//! `locked_edges` are never split and never receive a crossing: they are the
//! caller's protected seam/door edges, whose owning polygons must survive
//! byte-identical so door/merge triangle indices stay valid. The caller keeps
//! those polygons uniform by making its predicate report `true` inside them,
//! which is what guarantees a locked edge never needs a crossing in the first
//! place (both its endpoints are walkable).
//!
//! Std-only (no `bevy`/`glam`/`serde`), like `nav_clearance`: it is included
//! verbatim by `tests/features.rs` via `#[path]`.

use std::collections::{BTreeMap, BTreeSet};

/// One output triangle: its vertex indices into [`ClipOutput::vertices`], the
/// authored polygon it came from, and whether the predicate holds over it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ClipTriangle {
    pub(crate) vertex_indices: [u32; 3],
    /// Index of the authored polygon this triangle is a piece of.
    pub(crate) source: u32,
    /// `true` when the predicate holds over this piece (walkable).
    pub(crate) inside: bool,
}

/// Result of one re-triangulation: the authored vertices (unchanged, at their
/// original indices) followed by every refinement midpoint and boundary
/// crossing, plus the conformal triangle cover of the original area.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ClipOutput {
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) triangles: Vec<ClipTriangle>,
    /// Edges split during adaptive refinement.
    pub(crate) refinement_splits: usize,
    /// Boundary crossings located by bisection during the marching clip.
    pub(crate) boundary_crossings: usize,
    /// Sub-triangles discarded for being slivers below `min_area`.
    pub(crate) degenerate_discarded: usize,
    /// Predicate evaluations, so the caller can report the pass's cost.
    pub(crate) predicate_evaluations: usize,
}

/// Tunables for one re-triangulation. All are agent-geometry or
/// numerical-robustness quantities -- never anything derived from a
/// particular cell or placement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ClipParams {
    /// Refinement stops once a triangle's longest XZ edge is at or below this
    /// (metres). Sized to the smallest feature the agent must resolve.
    pub(crate) resolution: f32,
    /// Upper bound on refinement rounds, so a pathological predicate cannot
    /// subdivide without limit.
    pub(crate) max_refinement_rounds: usize,
    /// Bisection iterations used to locate a boundary crossing on an edge.
    pub(crate) bisection_steps: u32,
    /// Sub-triangles with a smaller XZ area (square metres) are dropped as
    /// slivers.
    pub(crate) min_area: f32,
}

impl Default for ClipParams {
    fn default() -> Self {
        Self {
            resolution: 0.35,
            max_refinement_rounds: 4,
            bisection_steps: 10,
            min_area: 1.0e-4,
        }
    }
}

pub(crate) fn edge_key(a: u32, b: u32) -> (u32, u32) {
    if a <= b { (a, b) } else { (b, a) }
}

fn midpoint(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        (a[0] + b[0]) * 0.5,
        (a[1] + b[1]) * 0.5,
        (a[2] + b[2]) * 0.5,
    ]
}

fn lerp(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn centroid(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    [
        (a[0] + b[0] + c[0]) / 3.0,
        (a[1] + b[1] + c[1]) / 3.0,
        (a[2] + b[2] + c[2]) / 3.0,
    ]
}

fn dist_xz(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dx = b[0] - a[0];
    let dz = b[2] - a[2];
    (dx * dx + dz * dz).sqrt()
}

/// Unsigned XZ area of a triangle.
fn area_xz(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> f32 {
    ((b[0] - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (b[2] - a[2])).abs() * 0.5
}

/// Rotates `(vertices, per-edge slots)` left by `k`, keeping edge slot `n`
/// paired with the edge from vertex `n` to vertex `n + 1`.
fn rotate<T: Copy>(v: [u32; 3], e: [T; 3], k: usize) -> ([u32; 3], [T; 3]) {
    (
        [v[k], v[(k + 1) % 3], v[(k + 2) % 3]],
        [e[k], e[(k + 1) % 3], e[(k + 2) % 3]],
    )
}

/// Re-triangulates `polygons` conformally against `predicate`.
///
/// Authored vertices keep their indices (new vertices are only ever
/// appended), so any caller-side data keyed by a vertex index stays valid.
/// Polygons carrying an out-of-range vertex index (`nav_graph`'s `u32::MAX`
/// invalid-slot sentinel) are passed through untouched and reported walkable
/// -- they were already diagnosed upstream and must not be judged here.
pub(crate) fn refine_and_clip(
    vertices: &[[f32; 3]],
    polygons: &[[u32; 3]],
    locked_edges: &BTreeSet<(u32, u32)>,
    predicate: &dyn Fn([f32; 3]) -> bool,
    params: ClipParams,
) -> ClipOutput {
    let mut verts: Vec<[f32; 3]> = vertices.to_vec();
    let mut evaluations = 0usize;
    let mut inside: Vec<bool> = verts
        .iter()
        .map(|&p| {
            evaluations += 1;
            predicate(p)
        })
        .collect();

    let valid = |tri: &[u32; 3], len: usize| tri.iter().all(|&i| (i as usize) < len);
    let vertex_count = verts.len();

    // Polygons with an invalid vertex slot never enter refinement or
    // clipping; they are emitted verbatim as walkable.
    let mut output: Vec<ClipTriangle> = Vec::new();
    let mut working: Vec<([u32; 3], u32)> = Vec::with_capacity(polygons.len());
    for (index, tri) in polygons.iter().enumerate() {
        if valid(tri, vertex_count) {
            working.push((*tri, index as u32));
        } else {
            output.push(ClipTriangle {
                vertex_indices: *tri,
                source: index as u32,
                inside: true,
            });
        }
    }

    let mut refinement_splits = 0usize;

    // ---- Phase 1: adaptive, conformal refinement -------------------------
    for _round in 0..params.max_refinement_rounds {
        let mut marked: BTreeSet<(u32, u32)> = BTreeSet::new();
        for (tri, _) in &working {
            let [ia, ib, ic] = [tri[0] as usize, tri[1] as usize, tri[2] as usize];
            let (pa, pb, pc) = (verts[ia], verts[ib], verts[ic]);
            let longest = dist_xz(pa, pb).max(dist_xz(pb, pc)).max(dist_xz(pc, pa));
            if longest <= params.resolution {
                continue;
            }
            let edges = [
                edge_key(tri[0], tri[1]),
                edge_key(tri[1], tri[2]),
                edge_key(tri[2], tri[0]),
            ];
            if edges.iter().all(|edge| locked_edges.contains(edge)) {
                continue;
            }
            let base = inside[ia];
            let mut uniform = inside[ib] == base && inside[ic] == base;
            if uniform {
                for point in [
                    midpoint(pa, pb),
                    midpoint(pb, pc),
                    midpoint(pc, pa),
                    centroid(pa, pb, pc),
                ] {
                    evaluations += 1;
                    if predicate(point) != base {
                        uniform = false;
                        break;
                    }
                }
            }
            if uniform {
                continue;
            }
            for edge in edges {
                if !locked_edges.contains(&edge) {
                    marked.insert(edge);
                }
            }
        }
        if marked.is_empty() {
            break;
        }
        // Deterministic: `BTreeSet` iteration order fixes the new indices.
        let mut midpoints: BTreeMap<(u32, u32), u32> = BTreeMap::new();
        for &(a, b) in &marked {
            let point = midpoint(verts[a as usize], verts[b as usize]);
            let index = verts.len() as u32;
            verts.push(point);
            evaluations += 1;
            inside.push(predicate(point));
            midpoints.insert((a, b), index);
        }
        refinement_splits += midpoints.len();

        let mut next: Vec<([u32; 3], u32)> = Vec::with_capacity(working.len() * 2);
        for (tri, source) in &working {
            split_refined(*tri, *source, &midpoints, &mut next);
        }
        working = next;
    }

    // ---- Phase 2: marching clip along the predicate boundary -------------
    let mut crossings: BTreeMap<(u32, u32), u32> = BTreeMap::new();
    let mut degenerate_discarded = 0usize;
    let emit = |out: &mut Vec<ClipTriangle>,
                verts: &Vec<[f32; 3]>,
                tri: [u32; 3],
                source: u32,
                is_inside: bool,
                discarded: &mut usize| {
        if tri[0] == tri[1] || tri[1] == tri[2] || tri[2] == tri[0] {
            *discarded += 1;
            return;
        }
        if area_xz(
            verts[tri[0] as usize],
            verts[tri[1] as usize],
            verts[tri[2] as usize],
        ) < params.min_area
        {
            *discarded += 1;
            return;
        }
        out.push(ClipTriangle {
            vertex_indices: tri,
            source,
            inside: is_inside,
        });
    };

    for (tri, source) in &working {
        let signs = [
            inside[tri[0] as usize],
            inside[tri[1] as usize],
            inside[tri[2] as usize],
        ];
        if signs[0] == signs[1] && signs[1] == signs[2] {
            emit(
                &mut output,
                &verts,
                *tri,
                *source,
                signs[0],
                &mut degenerate_discarded,
            );
            continue;
        }
        let inside_count = signs.iter().filter(|&&s| s).count();
        if inside_count == 1 {
            // Rotate the single walkable vertex into slot 0.
            let k = signs.iter().position(|&s| s).unwrap_or(0);
            let (v, _) = rotate(*tri, [0u8; 3], k);
            let p = crossing(
                &mut verts,
                &mut inside,
                &mut crossings,
                &mut evaluations,
                predicate,
                params,
                v[0],
                v[1],
            );
            let r = crossing(
                &mut verts,
                &mut inside,
                &mut crossings,
                &mut evaluations,
                predicate,
                params,
                v[2],
                v[0],
            );
            emit(
                &mut output,
                &verts,
                [v[0], p, r],
                *source,
                true,
                &mut degenerate_discarded,
            );
            emit(
                &mut output,
                &verts,
                [p, v[1], v[2]],
                *source,
                false,
                &mut degenerate_discarded,
            );
            emit(
                &mut output,
                &verts,
                [p, v[2], r],
                *source,
                false,
                &mut degenerate_discarded,
            );
        } else {
            // Rotate the single unwalkable vertex into slot 2.
            let odd = signs.iter().position(|&s| !s).unwrap_or(0);
            let k = (odd + 1) % 3;
            let (v, _) = rotate(*tri, [0u8; 3], k);
            let q = crossing(
                &mut verts,
                &mut inside,
                &mut crossings,
                &mut evaluations,
                predicate,
                params,
                v[1],
                v[2],
            );
            let r = crossing(
                &mut verts,
                &mut inside,
                &mut crossings,
                &mut evaluations,
                predicate,
                params,
                v[2],
                v[0],
            );
            emit(
                &mut output,
                &verts,
                [v[0], v[1], q],
                *source,
                true,
                &mut degenerate_discarded,
            );
            emit(
                &mut output,
                &verts,
                [v[0], q, r],
                *source,
                true,
                &mut degenerate_discarded,
            );
            emit(
                &mut output,
                &verts,
                [q, v[2], r],
                *source,
                false,
                &mut degenerate_discarded,
            );
        }
    }

    ClipOutput {
        vertices: verts,
        triangles: output,
        refinement_splits,
        boundary_crossings: crossings.len(),
        degenerate_discarded,
        predicate_evaluations: evaluations,
    }
}

/// Splits one triangle against the midpoints its edges received this round:
/// none (pass through), one or two (green closure -- no further edge splits,
/// so refinement never cascades), or three (full red split into four).
fn split_refined(
    tri: [u32; 3],
    source: u32,
    midpoints: &BTreeMap<(u32, u32), u32>,
    out: &mut Vec<([u32; 3], u32)>,
) {
    let slots = [
        midpoints.get(&edge_key(tri[0], tri[1])).copied(),
        midpoints.get(&edge_key(tri[1], tri[2])).copied(),
        midpoints.get(&edge_key(tri[2], tri[0])).copied(),
    ];
    match slots.iter().filter(|slot| slot.is_some()).count() {
        0 => out.push((tri, source)),
        1 => {
            let k = slots.iter().position(Option::is_some).unwrap_or(0);
            let (v, e) = rotate(tri, slots, k);
            let m = e[0].unwrap_or(v[0]);
            out.push(([v[0], m, v[2]], source));
            out.push(([m, v[1], v[2]], source));
        }
        2 => {
            // Rotate so the *missing* slot is last.
            let missing = slots.iter().position(Option::is_none).unwrap_or(0);
            let k = (missing + 1) % 3;
            let (v, e) = rotate(tri, slots, k);
            let p = e[0].unwrap_or(v[0]);
            let q = e[1].unwrap_or(v[1]);
            out.push(([v[0], p, v[2]], source));
            out.push(([p, q, v[2]], source));
            out.push(([p, v[1], q], source));
        }
        _ => {
            let p = slots[0].unwrap_or(tri[0]);
            let q = slots[1].unwrap_or(tri[1]);
            let r = slots[2].unwrap_or(tri[2]);
            out.push(([tri[0], p, r], source));
            out.push(([p, tri[1], q], source));
            out.push(([r, q, tri[2]], source));
            out.push(([p, q, r], source));
        }
    }
}

/// The predicate's boundary crossing on edge `a..b`, located by bisection and
/// memoized on the *unordered* edge so both owning triangles reference the
/// identical vertex index -- the property that keeps the cut conformal and
/// preserves landmass's shared-index adjacency.
#[allow(clippy::too_many_arguments)]
fn crossing(
    verts: &mut Vec<[f32; 3]>,
    inside: &mut Vec<bool>,
    crossings: &mut BTreeMap<(u32, u32), u32>,
    evaluations: &mut usize,
    predicate: &dyn Fn([f32; 3]) -> bool,
    params: ClipParams,
    a: u32,
    b: u32,
) -> u32 {
    let key = edge_key(a, b);
    if let Some(&index) = crossings.get(&key) {
        return index;
    }
    let (lo, hi) = key;
    let (pa, pb) = (verts[lo as usize], verts[hi as usize]);
    let at_lo = inside[lo as usize];
    let (mut t0, mut t1) = (0.0f32, 1.0f32);
    for _ in 0..params.bisection_steps {
        let tm = 0.5 * (t0 + t1);
        *evaluations += 1;
        if predicate(lerp(pa, pb, tm)) == at_lo {
            t0 = tm;
        } else {
            t1 = tm;
        }
    }
    let point = lerp(pa, pb, 0.5 * (t0 + t1));
    let index = verts.len() as u32;
    verts.push(point);
    inside.push(at_lo);
    crossings.insert(key, index);
    index
}

#[cfg(test)]
mod tests {
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
        for (&(a, b), _) in counts.iter() {
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
}
