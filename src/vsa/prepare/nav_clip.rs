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
    /// Sub-triangles removed by welding an ill-conditioned edge (never by
    /// discarding: see [`ClipParams::min_area`]).
    pub(crate) degenerate_discarded: usize,
    /// Vertex welds the degenerate-collapse phase performed.
    pub(crate) collapsed_welds: usize,
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
    /// XZ area (square metres) below which a sub-triangle is too
    /// ill-conditioned to be a valid navigation polygon.
    ///
    /// Such a piece is *collapsed*, never discarded: its shortest edge is
    /// welded, which removes it without leaving a hole. Discarding instead
    /// punches a hole in the conformal cover, and a hole between two walkable
    /// pieces severs their adjacency exactly as if a wall stood there.
    /// Keeping them is not an option either -- a triangle whose vertices are
    /// nearly collinear reads to landmass as "concave or has edges in
    /// clockwise order" and invalidates the entire mesh. Welding is the only
    /// resolution that preserves both adjacency and validity.
    pub(crate) min_area: f32,
}

impl Default for ClipParams {
    fn default() -> Self {
        Self {
            resolution: 0.35,
            max_refinement_rounds: 4,
            bisection_steps: 10,
            min_area: 1.0e-6,
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
            let corners = [tri[0], tri[1], tri[2]];
            let edges = [(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])];
            let mut boundary_crossing_edge = false;
            // Split only the edges the boundary actually crosses. Splitting
            // all three (which is what a per-triangle "is this triangle near
            // the boundary" rule amounts to) refines the whole triangle down
            // to `resolution` including the parts nowhere near the boundary,
            // and the polygon count grows with *area* instead of with
            // boundary length.
            for (a, b) in edges {
                let key = edge_key(a, b);
                if locked_edges.contains(&key) {
                    continue;
                }
                let (pa, pb) = (verts[a as usize], verts[b as usize]);
                if dist_xz(pa, pb) <= params.resolution {
                    continue;
                }
                let crosses = if inside[a as usize] != inside[b as usize] {
                    true
                } else {
                    evaluations += 1;
                    predicate(midpoint(pa, pb)) != inside[a as usize]
                };
                if crosses {
                    marked.insert(key);
                    boundary_crossing_edge = true;
                }
            }
            if boundary_crossing_edge {
                continue;
            }
            // No edge straddles the boundary, so a feature entirely inside the
            // triangle (a freestanding post in a large authored polygon) is
            // still invisible. The centroid finds it; splitting all three
            // edges then exposes it to the edge test above next round.
            let (pa, pb, pc) = (
                verts[corners[0] as usize],
                verts[corners[1] as usize],
                verts[corners[2] as usize],
            );
            if dist_xz(pa, pb).max(dist_xz(pb, pc)).max(dist_xz(pc, pa)) <= params.resolution {
                continue;
            }
            evaluations += 1;
            if predicate(centroid(pa, pb, pc)) == inside[corners[0] as usize] {
                continue;
            }
            for (a, b) in edges {
                let key = edge_key(a, b);
                if !locked_edges.contains(&key) {
                    marked.insert(key);
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
                tri: [u32; 3],
                source: u32,
                is_inside: bool,
                discarded: &mut usize| {
        if tri[0] == tri[1] || tri[1] == tri[2] || tri[2] == tri[0] {
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
                [v[0], p, r],
                *source,
                true,
                &mut degenerate_discarded,
            );
            emit(
                &mut output,
                [p, v[1], v[2]],
                *source,
                false,
                &mut degenerate_discarded,
            );
            emit(
                &mut output,
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
                [v[0], v[1], q],
                *source,
                true,
                &mut degenerate_discarded,
            );
            emit(
                &mut output,
                [v[0], q, r],
                *source,
                true,
                &mut degenerate_discarded,
            );
            emit(
                &mut output,
                [q, v[2], r],
                *source,
                false,
                &mut degenerate_discarded,
            );
        }
    }

    let (welds, removed) = collapse_ill_conditioned(&verts, &mut output, locked_edges, params);
    degenerate_discarded += removed;

    ClipOutput {
        vertices: verts,
        triangles: output,
        refinement_splits,
        boundary_crossings: crossings.len(),
        degenerate_discarded,
        collapsed_welds: welds,
        predicate_evaluations: evaluations,
    }
}

/// Removes sub-triangles too ill-conditioned to be valid navigation polygons
/// by *welding* their shortest edge, never by discarding them.
///
/// A triangle whose vertices are nearly collinear has no reliable winding:
/// landmass rejects it as "concave or has edges in clockwise order" and throws
/// away the whole mesh with it, which is how a cell ends up with no navigation
/// at all. Discarding such a piece is not an alternative -- it punches a hole
/// in the conformal cover and severs adjacency across it. Welding removes the
/// piece while every other triangle that referenced the welded vertex simply
/// follows it to the survivor, so the cover stays conformal and shared-index
/// adjacency is preserved. The welded pieces have area below `min_area`, so
/// the geometry moves by less than that sliver's own extent.
///
/// Vertices belonging to a locked (protected seam/door) edge are never moved:
/// they always win a weld, and a weld between two locked vertices is refused.
/// Iterates because one weld can leave a neighbour ill-conditioned.
fn collapse_ill_conditioned(
    vertices: &[[f32; 3]],
    triangles: &mut Vec<ClipTriangle>,
    locked_edges: &BTreeSet<(u32, u32)>,
    params: ClipParams,
) -> (usize, usize) {
    let mut locked_vertices: BTreeSet<u32> = BTreeSet::new();
    for &(a, b) in locked_edges {
        locked_vertices.insert(a);
        locked_vertices.insert(b);
    }

    let mut parent: Vec<u32> = (0..vertices.len() as u32).collect();
    fn find(parent: &mut [u32], mut node: u32) -> u32 {
        while parent[node as usize] != node {
            parent[node as usize] = parent[parent[node as usize] as usize];
            node = parent[node as usize];
        }
        node
    }

    let before = triangles.len();
    let mut welds = 0usize;

    // Signed XZ area under the current representatives, or `None` when the
    // triangle references an invalid slot.
    let signed =
        |parent: &mut Vec<u32>, tri: [u32; 3], substitute: Option<(u32, u32)>| -> Option<f32> {
            let mut indices = [0u32; 3];
            for (slot, &index) in tri.iter().enumerate() {
                if index as usize >= vertices.len() {
                    return None;
                }
                let mut rep = find(parent, index);
                if let Some((drop, keep)) = substitute
                    && rep == drop
                {
                    rep = keep;
                }
                indices[slot] = rep;
            }
            let [a, b, c] = indices.map(|index| vertices[index as usize]);
            Some(((b[0] - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (b[2] - a[2])) * 0.5)
        };

    // One weld per round, so the incidence used to prove a weld safe is never
    // stale. Bounded by the number of vertices; breaks as soon as a round
    // finds nothing left to weld.
    for _ in 0..vertices.len().max(1) {
        let mut candidate: Option<(u32, u32)> = None;
        'triangles: for triangle in triangles.iter() {
            if triangle
                .vertex_indices
                .iter()
                .any(|&index| index as usize >= vertices.len())
            {
                continue;
            }
            let [a, b, c] = triangle
                .vertex_indices
                .map(|index| find(&mut parent, index));
            if a == b || b == c || c == a {
                continue;
            }
            let (pa, pb, pc) = (
                vertices[a as usize],
                vertices[b as usize],
                vertices[c as usize],
            );
            if area_xz(pa, pb, pc) >= params.min_area {
                continue;
            }
            // Weld the shortest edge: it perturbs the mesh the least.
            let mut ordered = [
                (a, b, dist_xz(pa, pb)),
                (b, c, dist_xz(pb, pc)),
                (c, a, dist_xz(pc, pa)),
            ];
            ordered.sort_by(|left, right| {
                left.2
                    .total_cmp(&right.2)
                    .then(left.0.cmp(&right.0))
                    .then(left.1.cmp(&right.1))
            });
            for (from, to, _) in ordered {
                let (keep, drop) = match (
                    locked_vertices.contains(&from),
                    locked_vertices.contains(&to),
                ) {
                    (true, true) => continue, // never move a protected seam
                    (true, false) => (from, to),
                    (false, true) => (to, from),
                    // Deterministic: the lower index survives.
                    (false, false) => (from.min(to), from.max(to)),
                };
                // Moving a vertex can drag a neighbouring triangle inside out,
                // and an inverted polygon is exactly what landmass rejects --
                // so a weld that would flip any triangle still carrying real
                // area is refused rather than left for the caller's gate.
                let inverts = triangles.iter().any(|other| {
                    let (Some(now), Some(after)) = (
                        signed(&mut parent, other.vertex_indices, None),
                        signed(&mut parent, other.vertex_indices, Some((drop, keep))),
                    ) else {
                        return false;
                    };
                    after.abs() >= params.min_area && (now > 0.0) != (after > 0.0)
                });
                if inverts {
                    continue;
                }
                candidate = Some((drop, keep));
                break 'triangles;
            }
        }
        let Some((drop, keep)) = candidate else {
            break;
        };
        parent[drop as usize] = keep;
        welds += 1;
    }

    if welds > 0 {
        for triangle in triangles.iter_mut() {
            if triangle
                .vertex_indices
                .iter()
                .any(|&index| index as usize >= vertices.len())
            {
                continue;
            }
            triangle.vertex_indices = triangle
                .vertex_indices
                .map(|index| find(&mut parent, index));
        }
    }
    triangles.retain(|triangle| {
        let [a, b, c] = triangle.vertex_indices;
        a != b && b != c && c != a
    });
    // A weld can make two pieces coincide; landmass requires each edge to be
    // shared by at most two polygons, so keep only the first of any duplicate.
    let mut seen: BTreeSet<[u32; 3]> = BTreeSet::new();
    triangles.retain(|triangle| {
        let mut key = triangle.vertex_indices;
        key.sort_unstable();
        seen.insert(key)
    });
    (welds, before - triangles.len())
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
#[path = "tests/nav_clip.rs"]
mod tests;
