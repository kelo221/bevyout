//! Pure collision-derived navmesh validation + clearance (issue #153, M4
//! wave 10). Given one prepared nav mesh's walkable triangle soup plus this
//! cell's cooked static collision as world-space triangles, it validates the
//! authored FO3 NAVM against the cooked collision and drops triangles the
//! agent capsule cannot legitimately stand on or fit through:
//!
//! 1. **Collision-support validation (F153.1).** Removes any walkable
//!    triangle with no cooked collision surface under it within the agent's
//!    step height -- the authored NAVM paves over voids the faithfully cooked
//!    collision does not fill (issue #164 restroom strip). A route into a
//!    removed area is then `unreachable` at query time.
//! 2. **Interior-obstruction cutting (F153.2).** Cuts an *interior* walkable
//!    triangle when a wall-like static collider that the agent cannot step
//!    over (rises above the step height, into the agent body band) sits within
//!    the agent radius of it -- the #148 entrance frame the authored NAVM ran
//!    straight through. Step-overable colliders (stair risers, low ledges) and
//!    a room's own perimeter walls are deliberately excluded.
//! 3. **Sub-diameter disconnection (F153.3).** Drops a walkable triangle only
//!    when the *authored passage width* through it -- near-wall distance +
//!    far-wall distance, measured across the triangle against the
//!    (post-F153.1/F153.2) walkable boundary -- is below `2 * radius`
//!    everywhere in it: a genuinely sub-diameter throat the agent capsule
//!    cannot pass. Crucially it measures the passage, not per-triangle
//!    clearance, so a wall-adjacent triangle in a wide corridor keeps the
//!    full passage width (`0 + width`) and is never dropped -- the walkable
//!    region stays connected instead of fragmenting into an eroded center
//!    band. It never moves a vertex, so a ~1 m doorway with 90-degree jambs
//!    stays connected (its passage is `1.0 m > 2 * radius`) instead of
//!    collapsing under the reverted wave-6 miter's two-sided offset
//!    arithmetic, while a genuine 0.5 m gap disconnects. Seam/door protected
//!    triangles are exempt so authored doorways stay traversable.
//!
//! The pass also reports, per mesh, the connected-component structure of the
//! surviving walkable set (over shared triangle edges) so the fragmentation
//! this replaces is visible in `prepare` output without a viewer.
//!
//! Std-only (no `bevy`/`glam`/`serde`): this file is included verbatim by
//! `tests/features.rs` via `#[path]`, the same way `nav_graph.rs` is -- see
//! `AGENTS.md`'s testing section. The boundary conversion from
//! `PreparedNavMesh`/`PreparedPhysicsShape` into the plain world-space
//! triangle inputs below lives in `navmesh.rs`, not here.

use std::collections::{BTreeMap, BTreeSet};

/// Agent capsule radius (metres). Matches `nav::agent::AGENT_RADIUS` and
/// `player::CAPSULE_RADIUS` (0.35 m); held locally rather than imported,
/// keeping this module std-only for the cucumber `#[path]` include.
pub(crate) const AGENT_RADIUS: f32 = 0.35;
/// Agent capsule height (metres). Matches `nav::agent::AGENT_HEIGHT` (1.8 m).
pub(crate) const AGENT_HEIGHT: f32 = 1.8;
/// How far below a walkable triangle a cooked collision surface may sit and
/// still count as support, and how tall a collider may rise above the floor
/// and still be *stepped over* rather than obstruct (metres). Sized to the
/// agent's step-up capability.
pub(crate) const STEP_HEIGHT: f32 = 0.5;
/// How far a supporting collision surface may sit *above* the authored nav
/// height and still count (metres).
pub(crate) const SUPPORT_ABOVE_MARGIN: f32 = 0.3;
/// A collision triangle whose unit normal's vertical component magnitude is
/// at or above this is floor/ceiling-like and never counts as an interior
/// obstruction (only as support). `0.5` is 60 degrees from horizontal.
pub(crate) const WALL_NORMAL_Y_MAX: f32 = 0.5;
/// A non-main connected component at or above this many polygons is a "large"
/// reachable region: validation stranding it means a drop misread the
/// geometry, and the connectivity-preserving finalization reconnects it.
/// Below it, a component is a legitimate small island (the removed restroom
/// strip, a genuine sub-diameter alcove) left disconnected.
const LARGE_ISLAND: usize = 6;

/// Why a triangle was dropped (`None` = walkable). Tracked so the
/// connectivity-preserving finalization can un-drop a triangle stranding a
/// large region, and so counts reflect only committed drops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DropReason {
    Unsupported,
    Obstructed,
    SubDiameter,
}

impl DropReason {
    /// Stable one-word label for prepare diagnostics.
    pub(crate) fn label(self) -> &'static str {
        match self {
            DropReason::Unsupported => "unsupported",
            DropReason::Obstructed => "obstructed",
            DropReason::SubDiameter => "sub-diameter",
        }
    }
}

// ---------------------------------------------------------------------
// Inputs / outputs
// ---------------------------------------------------------------------

/// One cooked static collision triangle in Bevy-metre world space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CollisionTriangle {
    pub(crate) vertices: [[f32; 3]; 3],
}

/// One nav mesh's walkable triangle soup, plus its seam/door protected edges
/// (unordered vertex-index pairs) -- a plain vertex-array + index-triple
/// shape, the boundary conversion filling it in `navmesh.rs`.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct NavClearanceMeshInput {
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) polygons: Vec<[u32; 3]>,
    pub(crate) protected_edges: Vec<(u32, u32)>,
}

/// Tunable agent geometry for one clearance pass (single-sourced from
/// `navmesh.rs`; defaults mirror the module constants).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct NavClearanceParams {
    pub(crate) agent_radius: f32,
    pub(crate) agent_height: f32,
    pub(crate) step_height: f32,
    pub(crate) support_above_margin: f32,
}

impl Default for NavClearanceParams {
    fn default() -> Self {
        Self {
            agent_radius: AGENT_RADIUS,
            agent_height: AGENT_HEIGHT,
            step_height: STEP_HEIGHT,
            support_above_margin: SUPPORT_ABOVE_MARGIN,
        }
    }
}

/// Result of one clearance pass: a per-polygon walkable flag (`false` =
/// removed by validation, cut by an obstruction, or dropped for insufficient
/// agent-radius clearance) plus deterministic diagnostic counters, including
/// the connectivity structure of the surviving walkable set. Vertices are
/// never moved, so they are not returned.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NavClearanceResult {
    pub(crate) walkable: Vec<bool>,
    pub(crate) polygon_count: usize,
    /// Polygons dropped by F153.1 (no collision support under them).
    pub(crate) removed_unsupported: usize,
    /// Polygons dropped by F153.2 (a non-step-overable wall-like collider
    /// intrudes the agent capsule over their interior).
    pub(crate) cut_obstructed: usize,
    /// Polygons dropped by F153.3 (the authored passage through them is
    /// sub-diameter -- narrower than `2 * radius` everywhere in the triangle).
    pub(crate) dropped_unfit: usize,
    /// Protected (seam/door) triangles left walkable by the exemption from
    /// F153.2/F153.3 (they never carry an authored sub-diameter passage).
    pub(crate) protected_count: usize,
    /// Walkable polygons surviving every phase.
    pub(crate) walkable_count: usize,
    /// Connected components of the surviving walkable set over shared edges.
    pub(crate) component_count: usize,
    /// Polygon count of the largest such component (0 when none survive).
    pub(crate) largest_component: usize,
    /// Connected components of the *authored* mesh (every polygon walkable,
    /// before any validation) over shared edges -- the baseline this pass
    /// starts from. Real FO3 NAVM is often already multi-island within one
    /// mesh (islands the runtime reconnects with merge/door off-mesh links),
    /// so comparing `component_count` against this shows whether validation
    /// fragmented anything or merely inherited the authored structure.
    pub(crate) baseline_component_count: usize,
    pub(crate) baseline_largest_component: usize,
    /// Committed per-polygon drop reason (`None` = walkable), aligned with
    /// `walkable`. Lets the caller emit per-drop centroid diagnostics.
    pub(crate) reasons: Vec<Option<DropReason>>,
    /// Every non-main (stranded) walkable component: `(polygon_count,
    /// representative centroid)`. Lets the caller locate any disconnected
    /// island in world space without a viewer.
    pub(crate) nonmain_components: Vec<(usize, [f32; 3])>,
}

// ---------------------------------------------------------------------
// Geometry helpers (std-only)
// ---------------------------------------------------------------------

/// Unit normal of a world-space triangle, or `None` when degenerate.
fn triangle_normal(t: &CollisionTriangle) -> Option<[f32; 3]> {
    let [a, b, c] = t.vertices;
    let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let n = [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len < 1.0e-9 {
        None
    } else {
        Some([n[0] / len, n[1] / len, n[2] / len])
    }
}

/// Barycentric weights of `(px, pz)` within triangle `a,b,c` projected onto
/// the XZ plane, or `None` when outside or the projection is degenerate (a
/// vertical wall triangle projects to a line, so it never "contains" a point
/// -- exactly why walls never falsely support).
fn barycentric_xz(px: f32, pz: f32, a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> Option<[f32; 3]> {
    let v0x = b[0] - a[0];
    let v0z = b[2] - a[2];
    let v1x = c[0] - a[0];
    let v1z = c[2] - a[2];
    let det = v0x * v1z - v1x * v0z;
    if det.abs() < 1.0e-9 {
        return None;
    }
    let v2x = px - a[0];
    let v2z = pz - a[2];
    let beta = (v2x * v1z - v1x * v2z) / det;
    let gamma = (v0x * v2z - v2x * v0z) / det;
    let alpha = 1.0 - beta - gamma;
    const EPS: f32 = 1.0e-4;
    if alpha < -EPS || beta < -EPS || gamma < -EPS {
        None
    } else {
        Some([alpha, beta, gamma])
    }
}

/// Squared XZ distance from `(px, pz)` to segment `a..b`.
fn point_segment_dist_sq_xz(px: f32, pz: f32, a: [f32; 3], b: [f32; 3]) -> f32 {
    let ex = b[0] - a[0];
    let ez = b[2] - a[2];
    let len_sq = ex * ex + ez * ez;
    let (cx, cz) = if len_sq < 1.0e-12 {
        (a[0], a[2])
    } else {
        let t = (((px - a[0]) * ex + (pz - a[2]) * ez) / len_sq).clamp(0.0, 1.0);
        (a[0] + ex * t, a[2] + ez * t)
    };
    let dx = px - cx;
    let dz = pz - cz;
    dx * dx + dz * dz
}

/// XZ distance from `(px, pz)` to triangle `t` (0 when inside its XZ
/// projection).
fn point_triangle_dist_xz(px: f32, pz: f32, t: &CollisionTriangle) -> f32 {
    let [a, b, c] = t.vertices;
    if barycentric_xz(px, pz, a, b, c).is_some() {
        return 0.0;
    }
    point_segment_dist_sq_xz(px, pz, a, b)
        .min(point_segment_dist_sq_xz(px, pz, b, c))
        .min(point_segment_dist_sq_xz(px, pz, c, a))
        .sqrt()
}

/// Axis-aligned bounds of a collision triangle, precomputed for broadphase.
#[derive(Debug, Clone, Copy)]
struct TriAabb {
    min: [f32; 3],
    max: [f32; 3],
}

fn tri_aabb(t: &CollisionTriangle) -> TriAabb {
    let mut min = t.vertices[0];
    let mut max = t.vertices[0];
    for v in &t.vertices[1..] {
        for axis in 0..3 {
            min[axis] = min[axis].min(v[axis]);
            max[axis] = max[axis].max(v[axis]);
        }
    }
    TriAabb { min, max }
}

fn polygon_centroid(mesh: &NavClearanceMeshInput, tri: [u32; 3]) -> Option<[f32; 3]> {
    let a = *mesh.vertices.get(tri[0] as usize)?;
    let b = *mesh.vertices.get(tri[1] as usize)?;
    let c = *mesh.vertices.get(tri[2] as usize)?;
    Some([
        (a[0] + b[0] + c[0]) / 3.0,
        (a[1] + b[1] + c[1]) / 3.0,
        (a[2] + b[2] + c[2]) / 3.0,
    ])
}

fn edge_key(a: u32, b: u32) -> (u32, u32) {
    if a <= b { (a, b) } else { (b, a) }
}

/// Boundary edges of the currently-walkable region: an edge referenced by
/// exactly one walkable polygon.
fn walkable_boundary_edges(
    mesh: &NavClearanceMeshInput,
    walkable: &[bool],
) -> BTreeSet<(u32, u32)> {
    let mut counts: BTreeMap<(u32, u32), u32> = BTreeMap::new();
    for (index, tri) in mesh.polygons.iter().enumerate() {
        if !walkable.get(index).copied().unwrap_or(false) {
            continue;
        }
        for &(a, b) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
            *counts.entry(edge_key(a, b)).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .filter_map(|(edge, count)| (count == 1).then_some(edge))
        .collect()
}

/// Whether `tri` is an interior triangle: none of its three edges lies on the
/// walkable-region `boundary`.
fn is_interior_triangle(tri: [u32; 3], boundary: &BTreeSet<(u32, u32)>) -> bool {
    !boundary.contains(&edge_key(tri[0], tri[1]))
        && !boundary.contains(&edge_key(tri[1], tri[2]))
        && !boundary.contains(&edge_key(tri[2], tri[0]))
}

// ---------------------------------------------------------------------
// The pass
// ---------------------------------------------------------------------

/// Runs the full validation on one mesh. An empty `collision` list skips both
/// collision-driven phases (F153.1/F153.2) -- never remove walkable area when
/// there is no cooked collision to judge it against -- and still applies the
/// clearance-fit validation (F153.3). A zero/negative radius or empty mesh is
/// a no-op.
pub(crate) fn validate_and_clear(
    mesh: &NavClearanceMeshInput,
    collision: &[CollisionTriangle],
    params: NavClearanceParams,
) -> NavClearanceResult {
    let polygon_count = mesh.polygons.len();
    let walkable = vec![true; polygon_count];
    let (baseline_component_count, baseline_largest_component) =
        connected_components(mesh, &walkable);

    if params.agent_radius <= 0.0 || mesh.vertices.is_empty() || polygon_count == 0 {
        let (component_count, largest_component) = connected_components(mesh, &walkable);
        let walkable_count = walkable.iter().filter(|&&w| w).count();
        return NavClearanceResult {
            walkable,
            polygon_count,
            removed_unsupported: 0,
            cut_obstructed: 0,
            dropped_unfit: 0,
            protected_count: 0,
            walkable_count,
            component_count,
            largest_component,
            baseline_component_count,
            baseline_largest_component,
            reasons: vec![None; polygon_count],
            nonmain_components: Vec::new(),
        };
    }

    let protected: BTreeSet<u32> = protected_triangle_indices(mesh);
    let protected_count = protected.len();

    // Precompute collision-triangle bounds + normals for broadphase.
    let aabbs: Vec<TriAabb> = collision.iter().map(tri_aabb).collect();
    let normals: Vec<Option<[f32; 3]>> = collision.iter().map(triangle_normal).collect();

    // Per-triangle drop reason (`None` = walkable). Tracked so the
    // connectivity-preserving finalization below can un-drop a triangle that
    // turns out to strand a large reachable region, and so the final counts
    // reflect only committed drops.
    let mut reason: Vec<Option<DropReason>> = vec![None; polygon_count];

    // F153.1: collision-support validation (skipped when no collision).
    // Protected (seam/door) triangles are exempt so a runtime merge/door link
    // attachment is never removed out from under the off-mesh link. A triangle
    // is unsupported only when *every* sampled point (centroid + edge
    // midpoints) is over a void -- a single point landing in a crack/T-junction
    // of the cooked collision mesh must not sever a corridor, while a triangle
    // genuinely over a void (the #164 restroom strip) fails every sample.
    if !collision.is_empty() {
        for (index, tri) in mesh.polygons.iter().enumerate() {
            if protected.contains(&(index as u32)) {
                continue;
            }
            if !triangle_supported(mesh, *tri, collision, &aabbs, params) {
                reason[index] = Some(DropReason::Unsupported);
            }
        }
    }

    // F153.2: interior-obstruction cutting (skipped when no collision). A tall
    // (non-step-overable) wall-like collider within the agent radius of an
    // *interior* triangle cuts it -- a freestanding column/clutter the authored
    // NAVM ran straight through. Restricted to interior triangles (none of
    // whose edges is on the walkable boundary): cutting boundary triangles
    // within a radius of every perimeter wall erodes whole rooms, and (as the
    // #148 entrance frame showed) a centroid-distance test still misses a frame
    // whose posts flank a nav triangle's opening -- that class needs per-edge
    // clearance or local re-triangulation, out of scope this wave. Protected
    // (seam/door) triangles are exempt.
    if !collision.is_empty() {
        let walkable_now = reason.iter().map(Option::is_none).collect::<Vec<_>>();
        let boundary = walkable_boundary_edges(mesh, &walkable_now);
        for (index, tri) in mesh.polygons.iter().enumerate() {
            if reason[index].is_some()
                || protected.contains(&(index as u32))
                || !is_interior_triangle(*tri, &boundary)
            {
                continue;
            }
            let Some(centroid) = polygon_centroid(mesh, *tri) else {
                continue;
            };
            if is_obstructed(centroid, collision, &aabbs, &normals, params) {
                reason[index] = Some(DropReason::Obstructed);
            }
        }
    }

    // F153.3: sub-diameter disconnection on the validated walkable boundary.
    // A triangle is dropped only when the *authored passage width* through it
    // (near-wall distance + far-wall distance, measured across the triangle)
    // is below `2 * radius` everywhere in it -- a genuinely sub-diameter
    // throat. Wall-adjacent triangles in a wide passage keep the full passage
    // width (near-wall 0 + far-wall = width) and are never dropped, so the
    // walkable region stays connected (no erosion-style fragmentation).
    // Protected triangles are exempt so authored doorways/seams stay traversable.
    {
        let walkable_now = reason.iter().map(Option::is_none).collect::<Vec<_>>();
        let boundary = boundary_segments(mesh, &walkable_now);
        let min_width = 2.0 * params.agent_radius;
        for (index, tri) in mesh.polygons.iter().enumerate() {
            if reason[index].is_some() || protected.contains(&(index as u32)) {
                continue;
            }
            if max_passage_width(mesh, *tri, &boundary) < min_width {
                reason[index] = Some(DropReason::SubDiameter);
            }
        }
    }

    // Connectivity-preserving finalization. Validation must strand only small
    // dead-ends (the restroom strip, a genuine sub-diameter alcove), never
    // sever a large reachable region -- a real interior has no large area
    // gated solely by an unsupported/obstructed/sub-diameter triangle, so a
    // large stranded component means a drop misread the geometry. Any drop
    // adjacent to a large non-main component is un-dropped, iterated until only
    // small islands remain disconnected. Determinism: fixed index order.
    restore_large_strands(mesh, &mut reason, &protected);

    let mut walkable = walkable; // shadow the all-true init with committed drops.
    let mut removed_unsupported = 0usize;
    let mut cut_obstructed = 0usize;
    let mut dropped_unfit = 0usize;
    for (index, r) in reason.iter().enumerate() {
        match r {
            None => {}
            Some(DropReason::Unsupported) => {
                walkable[index] = false;
                removed_unsupported += 1;
            }
            Some(DropReason::Obstructed) => {
                walkable[index] = false;
                cut_obstructed += 1;
            }
            Some(DropReason::SubDiameter) => {
                walkable[index] = false;
                dropped_unfit += 1;
            }
        }
    }

    let walkable_count = walkable.iter().filter(|&&w| w).count();
    let (roots, sizes) = label_components(mesh, &walkable);
    let component_count = sizes.len();
    let largest_component = sizes.values().copied().max().unwrap_or(0);
    let nonmain_components = nonmain_component_report(mesh, &roots, &sizes);

    NavClearanceResult {
        walkable,
        polygon_count,
        removed_unsupported,
        cut_obstructed,
        dropped_unfit,
        protected_count,
        walkable_count,
        component_count,
        largest_component,
        baseline_component_count,
        baseline_largest_component,
        reasons: reason,
        nonmain_components,
    }
}

/// For each non-main component, its polygon count and the centroid of its
/// first (lowest-index) triangle. Deterministic (`BTreeMap` ordering).
fn nonmain_component_report(
    mesh: &NavClearanceMeshInput,
    roots: &[usize],
    sizes: &BTreeMap<usize, usize>,
) -> Vec<(usize, [f32; 3])> {
    let Some(main) = main_component_root(sizes) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (&root, &size) in sizes {
        if root == main {
            continue;
        }
        let centroid = roots
            .iter()
            .position(|&r| r == root)
            .and_then(|index| polygon_centroid(mesh, mesh.polygons[index]))
            .unwrap_or([0.0; 3]);
        out.push((size, centroid));
    }
    out
}

/// Protected triangle indices: any triangle owning at least one protected
/// (seam/door) edge.
fn protected_triangle_indices(mesh: &NavClearanceMeshInput) -> BTreeSet<u32> {
    let protected_edges: BTreeSet<(u32, u32)> = mesh
        .protected_edges
        .iter()
        .map(|&(a, b)| edge_key(a, b))
        .collect();
    let mut indices = BTreeSet::new();
    if protected_edges.is_empty() {
        return indices;
    }
    for (index, tri) in mesh.polygons.iter().enumerate() {
        let owns = [
            edge_key(tri[0], tri[1]),
            edge_key(tri[1], tri[2]),
            edge_key(tri[2], tri[0]),
        ]
        .iter()
        .any(|edge| protected_edges.contains(edge));
        if owns {
            indices.insert(index as u32);
        }
    }
    indices
}

/// F153.1: is `tri` supported? True when a majority of its sampled points
/// (centroid + three edge midpoints) have a cooked collision surface under
/// them -- crack/T-junction tolerant, so a single point falling through a seam
/// in the cooked collision mesh does not spuriously remove a triangle and
/// sever a corridor.
fn triangle_supported(
    mesh: &NavClearanceMeshInput,
    tri: [u32; 3],
    collision: &[CollisionTriangle],
    aabbs: &[TriAabb],
    params: NavClearanceParams,
) -> bool {
    let (Some(&a), Some(&b), Some(&c)) = (
        mesh.vertices.get(tri[0] as usize),
        mesh.vertices.get(tri[1] as usize),
        mesh.vertices.get(tri[2] as usize),
    ) else {
        return true; // invalid index already diagnosed upstream; do not drop.
    };
    let mid = |p: [f32; 3], q: [f32; 3]| {
        [
            (p[0] + q[0]) * 0.5,
            (p[1] + q[1]) * 0.5,
            (p[2] + q[2]) * 0.5,
        ]
    };
    let centroid = [
        (a[0] + b[0] + c[0]) / 3.0,
        (a[1] + b[1] + c[1]) / 3.0,
        (a[2] + b[2] + c[2]) / 3.0,
    ];
    let samples = [centroid, mid(a, b), mid(b, c), mid(c, a)];
    // Supported unless *every* sample is over a void: a triangle is only
    // removed when it is genuinely, wholly unsupported, never for a single
    // point falling through a collision-mesh crack/T-junction or a ramp/step
    // my sampling grazes -- over-removal severs through-corridors. (A narrow
    // void strip covered by triangles that also reach a supported floor is
    // deliberately kept: removing such a mostly-supported triangle would need
    // local re-triangulation, out of scope this wave -- the runtime fall guard
    // still catches a step onto the overhang.)
    samples
        .iter()
        .any(|&p| is_supported(p, collision, aabbs, params))
}

/// F153.1: is there a cooked collision surface under `point` within the
/// agent's step height (and a small margin above)?
fn is_supported(
    point: [f32; 3],
    collision: &[CollisionTriangle],
    aabbs: &[TriAabb],
    params: NavClearanceParams,
) -> bool {
    let low = point[1] - params.step_height;
    let high = point[1] + params.support_above_margin;
    for (tri, aabb) in collision.iter().zip(aabbs) {
        if point[0] < aabb.min[0] || point[0] > aabb.max[0] {
            continue;
        }
        if point[2] < aabb.min[2] || point[2] > aabb.max[2] {
            continue;
        }
        if aabb.max[1] < low || aabb.min[1] > high {
            continue;
        }
        let [a, b, c] = tri.vertices;
        let Some(w) = barycentric_xz(point[0], point[2], a, b, c) else {
            continue;
        };
        let height = w[0] * a[1] + w[1] * b[1] + w[2] * c[1];
        if height >= low && height <= high {
            return true;
        }
    }
    false
}

/// F153.2: does a wall-like collider the agent *cannot step over* rise into
/// the agent capsule over `point`, within the agent radius (XZ)? A collider
/// only obstructs if it occupies some height in `(floor + step_height, floor +
/// agent_height)` -- one entirely below `floor + step_height` is stepped over
/// (stair risers, low ledges), one entirely above `floor + agent_height` is
/// walked under (overheads). Floor/ceiling-like triangles never obstruct.
fn is_obstructed(
    point: [f32; 3],
    collision: &[CollisionTriangle],
    aabbs: &[TriAabb],
    normals: &[Option<[f32; 3]>],
    params: NavClearanceParams,
) -> bool {
    let radius = params.agent_radius;
    let step_top = point[1] + params.step_height;
    let band_high = point[1] + params.agent_height;
    for ((tri, aabb), normal) in collision.iter().zip(aabbs).zip(normals) {
        match normal {
            Some(n) if n[1].abs() < WALL_NORMAL_Y_MAX => {}
            _ => continue,
        }
        // Must rise above the step-over height and reach into the body band.
        if aabb.max[1] <= step_top || aabb.min[1] >= band_high {
            continue;
        }
        if point[0] < aabb.min[0] - radius || point[0] > aabb.max[0] + radius {
            continue;
        }
        if point[2] < aabb.min[2] - radius || point[2] > aabb.max[2] + radius {
            continue;
        }
        if point_triangle_dist_xz(point[0], point[2], tri) < radius {
            return true;
        }
    }
    false
}

/// A walkable-boundary edge as a world-space XZ segment plus the unit inward
/// normal (into the walkable region, from its one owning triangle's third
/// vertex). The inward normal is the axis the passage width is measured
/// along.
#[derive(Debug, Clone, Copy)]
struct BoundarySeg {
    ax: f32,
    az: f32,
    bx: f32,
    bz: f32,
    /// Unit inward normal (XZ).
    nx: f32,
    nz: f32,
}

/// Boundary edges of the walkable region as world-space XZ segments with
/// inward normals, for the F153.3 passage-width field. A boundary edge is one
/// referenced by exactly one walkable polygon; its inward normal points from
/// the edge toward that polygon's third vertex.
fn boundary_segments(mesh: &NavClearanceMeshInput, walkable: &[bool]) -> Vec<BoundarySeg> {
    let mut edge_polys: BTreeMap<(u32, u32), Vec<usize>> = BTreeMap::new();
    for (index, tri) in mesh.polygons.iter().enumerate() {
        if !walkable.get(index).copied().unwrap_or(false) {
            continue;
        }
        for &(a, b) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
            edge_polys.entry(edge_key(a, b)).or_default().push(index);
        }
    }
    let mut segs = Vec::new();
    for ((a, b), polys) in &edge_polys {
        if polys.len() != 1 {
            continue;
        }
        let tri = mesh.polygons[polys[0]];
        let Some(&third) = tri.iter().find(|&&v| v != *a && v != *b) else {
            continue;
        };
        let (Some(&pa), Some(&pb), Some(&pt)) = (
            mesh.vertices.get(*a as usize),
            mesh.vertices.get(*b as usize),
            mesh.vertices.get(third as usize),
        ) else {
            continue;
        };
        let ex = pb[0] - pa[0];
        let ez = pb[2] - pa[2];
        // Perpendicular candidate; orient toward the third vertex (interior).
        let (mut nx, mut nz) = (-ez, ex);
        let mid_x = (pa[0] + pb[0]) * 0.5;
        let mid_z = (pa[2] + pb[2]) * 0.5;
        if nx * (pt[0] - mid_x) + nz * (pt[2] - mid_z) < 0.0 {
            nx = -nx;
            nz = -nz;
        }
        let len = (nx * nx + nz * nz).sqrt();
        if len < 1.0e-9 {
            continue;
        }
        segs.push(BoundarySeg {
            ax: pa[0],
            az: pa[2],
            bx: pb[0],
            bz: pb[2],
            nx: nx / len,
            nz: nz / len,
        });
    }
    segs
}

/// Nearest positive distance along the ray `(ox, oz) + t*(dx, dz)` (dir
/// assumed unit) to any boundary segment, or `None` when the ray escapes
/// (an open passage end). Used to reach the *far* wall across a passage.
fn ray_boundary_distance(
    ox: f32,
    oz: f32,
    dx: f32,
    dz: f32,
    boundary: &[BoundarySeg],
) -> Option<f32> {
    const EPS: f32 = 1.0e-4;
    let mut best: Option<f32> = None;
    for seg in boundary {
        let ex = seg.bx - seg.ax;
        let ez = seg.bz - seg.az;
        let det = dx * (-ez) - (-ex) * dz;
        if det.abs() < 1.0e-9 {
            continue;
        }
        let rx = seg.ax - ox;
        let rz = seg.az - oz;
        // Solve [dx -ex; dz -ez] [t u]^T = [rx rz]^T.
        let t = (rx * (-ez) - (-ex) * rz) / det;
        let u = (dx * rz - dz * rx) / det;
        if t > EPS && (-EPS..=1.0 + EPS).contains(&u) {
            best = Some(best.map_or(t, |b: f32| b.min(t)));
        }
    }
    best
}

/// The maximum authored passage width sampled across `tri`: for the centroid
/// and three edge midpoints, `near-wall distance + far-wall distance` along
/// the nearest boundary edge's inward normal. A wall-adjacent triangle's
/// boundary-edge midpoint reports `0 + full width` = the full passage, so a
/// wide passage's wall strip is never mistaken for sub-diameter; a genuine
/// throat reports a small width at every sample. `f32::MAX` when there is no
/// boundary or the far ray escapes (an open, effectively unbounded passage).
fn max_passage_width(mesh: &NavClearanceMeshInput, tri: [u32; 3], boundary: &[BoundarySeg]) -> f32 {
    if boundary.is_empty() {
        return f32::MAX;
    }
    let (Some(&a), Some(&b), Some(&c)) = (
        mesh.vertices.get(tri[0] as usize),
        mesh.vertices.get(tri[1] as usize),
        mesh.vertices.get(tri[2] as usize),
    ) else {
        return f32::MAX; // invalid index already diagnosed upstream.
    };
    let mid = |p: [f32; 3], q: [f32; 3]| [(p[0] + q[0]) * 0.5, (p[2] + q[2]) * 0.5];
    let centroid = [(a[0] + b[0] + c[0]) / 3.0, (a[2] + b[2] + c[2]) / 3.0];
    let samples = [centroid, mid(a, b), mid(b, c), mid(c, a)];
    let mut best = 0.0f32;
    for [sx, sz] in samples {
        // Nearest boundary edge to the sample -> near-wall distance + axis.
        let mut near_dist_sq = f32::INFINITY;
        let mut axis = (0.0f32, 0.0f32);
        for seg in boundary {
            let d = point_segment_dist_sq_xz(sx, sz, [seg.ax, 0.0, seg.az], [seg.bx, 0.0, seg.bz]);
            if d < near_dist_sq {
                near_dist_sq = d;
                axis = (seg.nx, seg.nz);
            }
        }
        if axis == (0.0, 0.0) {
            continue;
        }
        let near = near_dist_sq.sqrt();
        // Reach the far wall along the inward normal (into the region).
        let width = match ray_boundary_distance(sx, sz, axis.0, axis.1, boundary) {
            Some(far) => near + far,
            None => f32::MAX, // open passage end -> effectively unbounded.
        };
        if width > best {
            best = width;
        }
        if best == f32::MAX {
            break;
        }
    }
    best
}

/// Per-triangle connected-component labels over shared edges of the walkable
/// set (union-find), plus each component root's polygon count. `roots[i] ==
/// usize::MAX` for a non-walkable triangle. `BTreeMap` keeps determinism.
fn label_components(
    mesh: &NavClearanceMeshInput,
    walkable: &[bool],
) -> (Vec<usize>, BTreeMap<usize, usize>) {
    let n = mesh.polygons.len();
    let mut parent: Vec<usize> = (0..n).collect();
    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    let mut edge_owner: BTreeMap<(u32, u32), usize> = BTreeMap::new();
    for (index, tri) in mesh.polygons.iter().enumerate() {
        if !walkable.get(index).copied().unwrap_or(false) {
            continue;
        }
        for &(a, b) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
            let key = edge_key(a, b);
            match edge_owner.get(&key) {
                Some(&other) => {
                    let ra = find(&mut parent, index);
                    let rb = find(&mut parent, other);
                    if ra != rb {
                        parent[ra] = rb;
                    }
                }
                None => {
                    edge_owner.insert(key, index);
                }
            }
        }
    }
    let roots: Vec<usize> = (0..n)
        .map(|index| {
            if walkable.get(index).copied().unwrap_or(false) {
                find(&mut parent, index)
            } else {
                usize::MAX
            }
        })
        .collect();
    let mut sizes: BTreeMap<usize, usize> = BTreeMap::new();
    for &root in &roots {
        if root != usize::MAX {
            *sizes.entry(root).or_insert(0) += 1;
        }
    }
    (roots, sizes)
}

/// Connected components of the walkable set over shared triangle edges.
/// Returns `(component_count, largest_component_polygon_count)`.
fn connected_components(mesh: &NavClearanceMeshInput, walkable: &[bool]) -> (usize, usize) {
    let (_, sizes) = label_components(mesh, walkable);
    (sizes.len(), sizes.values().copied().max().unwrap_or(0))
}

/// The main (largest) component root, ties broken by smallest root index for
/// determinism; `None` when nothing is walkable.
fn main_component_root(sizes: &BTreeMap<usize, usize>) -> Option<usize> {
    sizes
        .iter()
        .max_by(|(ra, sa), (rb, sb)| sa.cmp(sb).then(rb.cmp(ra)))
        .map(|(root, _)| *root)
}

/// Connectivity-preserving finalization: un-drops any triangle adjacent to a
/// non-main stranded component that is either *large* (>= [`LARGE_ISLAND`]
/// polygons) or contains a *protected* seam/door triangle, iterated until only
/// small, unprotected islands remain disconnected. A large stranded component
/// is a misread drop (a real interior has no large area gated by a single
/// unsupported/obstructed/sub-diameter triangle); a protected-containing
/// stranded component is an inter-mesh merge seam or door that must stay
/// reachable from the main region regardless of size (the #165/#169 travel
/// hand-off, and the door-corridor parity route). Protected triangles are
/// never themselves a drop, so they are not touched.
fn restore_large_strands(
    mesh: &NavClearanceMeshInput,
    reason: &mut [Option<DropReason>],
    protected: &BTreeSet<u32>,
) {
    let n = mesh.polygons.len();
    for _ in 0..=n {
        let walkable: Vec<bool> = reason.iter().map(Option::is_none).collect();
        let (roots, sizes) = label_components(mesh, &walkable);
        let Some(main) = main_component_root(&sizes) else {
            break;
        };
        // Component roots that contain at least one protected triangle.
        let protected_roots: BTreeSet<usize> = protected
            .iter()
            .filter_map(|&index| roots.get(index as usize).copied())
            .filter(|&root| root != usize::MAX)
            .collect();
        let large_stranded: BTreeSet<usize> = sizes
            .iter()
            .filter(|(root, size)| {
                **root != main && (**size >= LARGE_ISLAND || protected_roots.contains(*root))
            })
            .map(|(root, _)| *root)
            .collect();
        if large_stranded.is_empty() {
            break;
        }
        // Edge -> walkable polygon owners, to find a dropped triangle's
        // still-walkable neighbours and their components.
        let mut edge_owners: BTreeMap<(u32, u32), Vec<usize>> = BTreeMap::new();
        for (index, tri) in mesh.polygons.iter().enumerate() {
            if !walkable[index] {
                continue;
            }
            for &(a, b) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
                edge_owners.entry(edge_key(a, b)).or_default().push(index);
            }
        }
        let mut restored = false;
        for (index, tri) in mesh.polygons.iter().enumerate() {
            if reason[index].is_none() {
                continue;
            }
            let adjacent_to_large = [
                edge_key(tri[0], tri[1]),
                edge_key(tri[1], tri[2]),
                edge_key(tri[2], tri[0]),
            ]
            .iter()
            .flat_map(|edge| edge_owners.get(edge).into_iter().flatten())
            .any(|&neighbor| large_stranded.contains(&roots[neighbor]));
            if adjacent_to_large {
                reason[index] = None;
                restored = true;
            }
        }
        if !restored {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floor(x0: f32, x1: f32, z0: f32, z1: f32, y: f32) -> Vec<CollisionTriangle> {
        vec![
            CollisionTriangle {
                vertices: [[x0, y, z0], [x1, y, z0], [x1, y, z1]],
            },
            CollisionTriangle {
                vertices: [[x0, y, z0], [x1, y, z1], [x0, y, z1]],
            },
        ]
    }

    /// A vertical wall quad spanning `x=[x0,x1]` at fixed `z`, from `y0` up to
    /// `y1` -- a wall-like collider (normal in the XZ plane).
    fn wall(x0: f32, x1: f32, z: f32, y0: f32, y1: f32) -> Vec<CollisionTriangle> {
        vec![
            CollisionTriangle {
                vertices: [[x0, y0, z], [x1, y0, z], [x1, y1, z]],
            },
            CollisionTriangle {
                vertices: [[x0, y0, z], [x1, y1, z], [x0, y1, z]],
            },
        ]
    }

    fn nav_quad(x0: f32, x1: f32, z0: f32, z1: f32, y: f32) -> NavClearanceMeshInput {
        NavClearanceMeshInput {
            vertices: vec![[x0, y, z0], [x1, y, z0], [x1, y, z1], [x0, y, z1]],
            polygons: vec![[0, 1, 2], [0, 2, 3]],
            protected_edges: Vec::new(),
        }
    }

    #[test]
    fn empty_collision_never_removes_or_cuts() {
        let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
        let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
        assert_eq!(result.removed_unsupported, 0);
        assert_eq!(result.cut_obstructed, 0);
    }

    #[test]
    fn a_triangle_over_a_void_is_removed_as_unsupported() {
        // Floor only reaches x=1.5; polygon 0 = [0,1,2] = (0,0),(4,0),(4,2) has
        // its centroid and every edge midpoint beyond the floor (nearest at
        // x=2 > 1.5), so every sample is over the void -> removed. Polygon 1
        // keeps a supported centroid + corner.
        let mesh = nav_quad(0.0, 4.0, 0.0, 2.0, 0.0);
        let collision = floor(0.0, 1.5, -0.5, 2.5, 0.0);
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        assert_eq!(result.removed_unsupported, 1, "{result:?}");
        assert!(
            !result.walkable[0],
            "far triangle over void must be removed"
        );
    }

    #[test]
    fn a_single_missing_floor_triangle_does_not_remove_a_supported_polygon() {
        // Floor tiled as unit cells with one cell missing under polygon 0's
        // centroid (~2.67, 1.33): the other samples still land on a floor cell,
        // so the triangle is kept -- a collision-mesh gap must not sever the
        // mesh.
        let mut collision = Vec::new();
        for gx in -1..5 {
            for gz in -1..5 {
                if gx == 2 && gz == 1 {
                    continue; // the missing cell under polygon 0's centroid.
                }
                let (x0, z0) = (gx as f32, gz as f32);
                collision.extend(floor(x0, x0 + 1.0, z0, z0 + 1.0, 0.0));
            }
        }
        let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        assert_eq!(result.removed_unsupported, 0, "{result:?}");
    }

    #[test]
    fn a_floor_a_full_step_below_still_supports() {
        let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
        let collision = floor(-1.0, 5.0, -1.0, 5.0, -0.4);
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        assert_eq!(result.removed_unsupported, 0, "{result:?}");
    }

    #[test]
    fn a_floor_beyond_the_step_below_does_not_support() {
        let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
        let collision = floor(-1.0, 5.0, -1.0, 5.0, -1.0);
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        assert_eq!(result.removed_unsupported, 2, "{result:?}");
    }

    /// A big triangle midpoint-subdivided into four: the central triangle
    /// (index 3, vertices D,E,F) is the one fully interior triangle.
    fn subdivided_triangle() -> NavClearanceMeshInput {
        NavClearanceMeshInput {
            vertices: vec![
                [0.0, 0.0, 0.0], // 0 A
                [4.0, 0.0, 0.0], // 1 B
                [0.0, 0.0, 4.0], // 2 C
                [2.0, 0.0, 0.0], // 3 D
                [2.0, 0.0, 2.0], // 4 E
                [0.0, 0.0, 2.0], // 5 F
            ],
            polygons: vec![[0, 3, 5], [3, 1, 4], [5, 4, 2], [3, 4, 5]],
            protected_edges: Vec::new(),
        }
    }

    #[test]
    fn a_tall_interior_collider_cuts_only_the_interior_triangle_it_sits_on() {
        let mesh = subdivided_triangle();
        let mut collision = floor(-1.0, 5.0, -1.0, 5.0, 0.0);
        // A tall wall stub (rises well above the step height) on the centre.
        collision.extend(wall(1.2, 1.5, 1.33, 0.0, 2.0));
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        assert_eq!(result.removed_unsupported, 0, "{result:?}");
        assert_eq!(result.cut_obstructed, 1, "{result:?}");
        assert!(!result.walkable[3], "the interior triangle must be cut");
    }

    #[test]
    fn a_step_overable_riser_does_not_cut_the_stair_tread() {
        // A short riser (0.0..0.3 m, under the 0.5 m step height) on the same
        // interior triangle must NOT cut it -- stairs stay traversable.
        let mesh = subdivided_triangle();
        let mut collision = floor(-1.0, 5.0, -1.0, 5.0, 0.0);
        collision.extend(wall(1.2, 1.5, 1.33, 0.0, 0.3));
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        assert_eq!(result.cut_obstructed, 0, "{result:?}");
    }

    #[test]
    fn a_boundary_wall_does_not_cut_perimeter_triangles() {
        // A tall wall on the boundary of a small room is not cut: cutting
        // every perimeter-adjacent triangle would erode whole rooms. Only
        // interior obstructions (a freestanding column) are cut.
        let mesh = nav_quad(0.0, 4.0, 0.0, 1.0, 0.0);
        let mut collision = floor(-1.0, 5.0, -1.0, 2.0, 0.0);
        collision.extend(wall(1.1, 1.5, 0.5, 0.0, 2.0));
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        assert_eq!(result.cut_obstructed, 0, "{result:?}");
    }

    /// A corridor along X of `half_widths.len()` stations, centred on z=1,
    /// with a per-station half-width so it can be pinched in the middle.
    fn pinched_corridor(half_widths: &[f32]) -> NavClearanceMeshInput {
        let mut vertices = Vec::new();
        for (i, hw) in half_widths.iter().enumerate() {
            let x = i as f32 * 2.0;
            vertices.push([x, 0.0, 1.0 - hw]); // bottom row, index 2*i
            vertices.push([x, 0.0, 1.0 + hw]); // top row, index 2*i + 1
        }
        let mut polygons = Vec::new();
        for i in 0..half_widths.len() - 1 {
            let ba = (2 * i) as u32;
            let ta = (2 * i + 1) as u32;
            let bb = (2 * (i + 1)) as u32;
            let tb = (2 * (i + 1) + 1) as u32;
            polygons.push([ba, bb, tb]);
            polygons.push([ba, tb, ta]);
        }
        NavClearanceMeshInput {
            vertices,
            polygons,
            protected_edges: Vec::new(),
        }
    }

    #[test]
    fn a_one_metre_doorway_stays_connected_with_an_eroded_passage() {
        // A uniform 1.0 m corridor (half-width 0.5 > radius): the centre line
        // is 0.5 m > 0.35 m from each wall, so every triangle keeps a fitting
        // point -- one connected component, nothing dropped.
        let mesh = pinched_corridor(&[0.5, 0.5, 0.5, 0.5]);
        let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
        assert_eq!(result.dropped_unfit, 0, "{result:?}");
        assert_eq!(result.component_count, 1, "{result:?}");
        assert_eq!(result.largest_component, mesh.polygons.len(), "{result:?}");
    }

    #[test]
    fn a_sub_diameter_pinch_disconnects_the_two_wide_ends() {
        // Wide (half-width 1.0) at both ends, pinched to a 0.5 m gap
        // (half-width 0.25 < radius) in the middle: the neck fits nowhere and
        // drops, splitting the corridor into two components; the wide ends
        // survive.
        let mesh = pinched_corridor(&[1.0, 0.25, 0.25, 1.0]);
        let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
        assert!(result.dropped_unfit > 0, "the neck must drop: {result:?}");
        assert_eq!(
            result.component_count, 2,
            "the pinch must split the corridor: {result:?}"
        );
        assert!(
            result.walkable_count > 0,
            "the wide ends survive: {result:?}"
        );
    }

    #[test]
    fn protected_triangles_are_never_dropped_or_cut() {
        // A narrow (sub-diameter) quad whose z=0 edge is protected: the
        // protected triangle stays walkable even though the agent does not
        // fit, so an authored doorway/seam is never severed.
        let mut mesh = nav_quad(0.0, 4.0, 0.0, 0.3, 0.0);
        mesh.protected_edges = vec![(0, 1)];
        let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
        // Polygon 0 = [0,1,2] owns edge (0,1) -> protected; polygon 1 =
        // [0,2,3] does not own (0,1).
        assert!(result.walkable[0], "protected triangle stays walkable");
        assert_eq!(result.protected_count, 1, "{result:?}");
    }

    #[test]
    fn the_pass_is_deterministic_across_calls() {
        let mesh = pinched_corridor(&[1.0, 0.25, 0.25, 1.0]);
        let first = validate_and_clear(&mesh, &[], NavClearanceParams::default());
        let second = validate_and_clear(&mesh, &[], NavClearanceParams::default());
        assert_eq!(first, second);
    }

    #[test]
    fn a_zero_radius_pass_is_a_no_op() {
        let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
        let params = NavClearanceParams {
            agent_radius: 0.0,
            ..NavClearanceParams::default()
        };
        let result = validate_and_clear(&mesh, &[], params);
        assert!(result.walkable.iter().all(|&w| w));
    }
}
