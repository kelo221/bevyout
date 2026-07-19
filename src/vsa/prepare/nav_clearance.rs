//! Pure collision-derived navmesh validation + clearance (issue #153, M4
//! wave 10). Given one prepared nav mesh's walkable triangle soup plus this
//! cell's cooked static collision as world-space triangles, it:
//!
//! 1. **Collision-support validation (F153.1).** Removes any walkable
//!    triangle with no cooked collision surface under it within the agent's
//!    step height -- the authored FO3 NAVM paves over voids the faithfully
//!    cooked collision does not fill (issue #164: Franklin Metro 02's
//!    `0001a273` restroom strip runs to x~-14.2 over empty-collision clutter
//!    while the room shell ends at x~-15.6). A route into a removed area is
//!    then `unreachable` at query time rather than relying on the runtime
//!    fall guard.
//! 2. **Interior-obstruction cutting (F153.2).** Marks a walkable triangle
//!    unwalkable when a wall-like static collider rises into the agent
//!    capsule over that triangle within the agent radius -- the entrance
//!    frame the authored NAVM ran straight through (issue #148,
//!    `MetHallEntrance01` 370250 wedging the r=0.35 capsule at x~9.55). The
//!    solver then refuses the routes physics would refuse.
//! 3. **Clearance with miter-corrected corners (F153.3).** Offsets the
//!    boundary of the validated walkable region inward by the agent radius,
//!    scaling a corner vertex's displacement by `radius / cos(theta/2)`
//!    (clamped to [`MITER_LIMIT_FACTOR`] * radius) so each incident wall
//!    keeps the full `radius` clearance -- the reverted wave-6 miter
//!    (`6cb4c3a`), whose fixed-length average only gave a 90-degree corner
//!    `r/sqrt(2)`. **Sub-diameter corridors disconnect:** unlike wave-6's
//!    pinch guard, a triangle whose offset shape inverts or degenerates is
//!    dropped (not relaxed back), so a corridor narrower than `2 * radius`
//!    produces no route at all. New islands the disconnection creates are
//!    legitimate output -- the runtime landmass island handling copes.
//!
//! Seam/door **protected** vertices (a merge or door triangle's vertices)
//! never move and are never dropped -- the same rule the retired erosion pass
//! used, keeping both sides of a cross-mesh seam agreeing exactly.
//!
//! Std-only (no `bevy`/`glam`/`serde`): this file is included verbatim by
//! `tests/features.rs` via `#[path]`, the same way `erosion_policy.rs` and
//! `nav_graph.rs` are -- see `AGENTS.md`'s testing section. The boundary
//! conversion from `PreparedNavMesh`/`PreparedPhysicsShape` into the plain
//! world-space triangle inputs below lives in `navmesh.rs` (which is free to
//! import `glam`/`vsa` types), not here.

use std::collections::{BTreeMap, BTreeSet};

/// Agent capsule radius (metres). Matches `nav::agent::AGENT_RADIUS` and
/// `player::CAPSULE_RADIUS` (0.35 m); held locally per the same
/// no-cross-import rule `erosion_policy` documented.
pub(crate) const AGENT_RADIUS: f32 = 0.35;
/// Agent capsule height (metres). Matches `nav::agent::AGENT_HEIGHT` (1.8 m).
/// A collider must rise into `[floor, floor + AGENT_HEIGHT]` to count as an
/// interior obstruction; a knee-high ledge the agent steps over does not.
pub(crate) const AGENT_HEIGHT: f32 = 1.8;
/// How far below a walkable triangle a cooked collision surface may sit and
/// still count as support (metres). Sized to the agent's step-up capability:
/// a floor within a step of the authored nav height supports it.
pub(crate) const STEP_HEIGHT: f32 = 0.5;
/// How far a supporting collision surface may sit *above* the authored nav
/// height and still count (metres) -- small, since FO3 NAVM is authored on
/// the floor, but nonzero for float drift between the cooked collision top
/// and the authored nav vertex.
pub(crate) const SUPPORT_ABOVE_MARGIN: f32 = 0.3;
/// A collision triangle whose unit normal's vertical component magnitude is
/// at or above this is floor/ceiling-like and never counts as an interior
/// *obstruction* (only as *support*). Below it the triangle is wall-like.
/// `0.5` is 60 degrees from horizontal.
pub(crate) const WALL_NORMAL_Y_MAX: f32 = 0.5;
/// Miter limit: caps a corner vertex's displacement at
/// `MITER_LIMIT_FACTOR * radius` regardless of how sharp the angle between
/// its incident boundary normals is, matching the order of magnitude
/// Recast/Clipper polygon offsetters use (reused from wave-6 `6cb4c3a`).
pub(crate) const MITER_LIMIT_FACTOR: f32 = 2.5;
/// Floor for `cos(theta/2)` in the miter-scale division, keeping it finite as
/// the angle between two incident normals approaches 180 degrees;
/// `MITER_LIMIT_FACTOR` is what actually bounds the result there.
const MIN_COS_HALF_ANGLE: f32 = 1.0e-4;
/// Below this scaled-triangle area (2D horizontal plane) an offset triangle
/// is degenerate and dropped.
const MIN_TRIANGLE_AREA: f32 = 1.0e-4;
/// Below this original-triangle area a triangle's winding sign is too close
/// to the f32 noise floor to trust as the "expected orientation" (real FO3
/// NAVM carries deliberate sliver triangles) -- such a triangle is validated
/// against the mesh's dominant winding sign instead of its own. Reused from
/// wave-6 `erosion_policy`'s `MIN_RELIABLE_ORIGINAL_AREA`.
const MIN_RELIABLE_ORIGINAL_AREA: f32 = 1.0e-2;

// ---------------------------------------------------------------------
// Inputs / outputs
// ---------------------------------------------------------------------

/// One cooked static collision triangle in Bevy-metre world space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CollisionTriangle {
    pub(crate) vertices: [[f32; 3]; 3],
}

/// One nav mesh's walkable triangle soup, plus its seam/door protected edges
/// (unordered vertex-index pairs) -- the same plain shape
/// `erosion_policy::ErosionMeshInput` used.
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

/// Result of one clearance pass: offset vertex positions (same length/order
/// as the input -- indices untouched, only positions move, protected
/// vertices unmoved), a per-polygon walkable flag (`false` = removed by
/// validation, cut by an obstruction, or disconnected by clearance), and
/// deterministic diagnostic counters.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NavClearanceResult {
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) walkable: Vec<bool>,
    pub(crate) polygon_count: usize,
    /// Polygons dropped by F153.1 (no collision support under them).
    pub(crate) removed_unsupported: usize,
    /// Polygons dropped by F153.2 (a wall-like collider intrudes the agent
    /// capsule over them).
    pub(crate) cut_obstructed: usize,
    /// Polygons dropped by F153.3 because their offset shape inverted or
    /// degenerated -- a sub-diameter corridor throat that now disconnects.
    pub(crate) disconnected_narrow: usize,
    /// Polygons with at least one vertex that actually moved under clearance.
    pub(crate) offset_count: usize,
    /// Distinct vertices left unmoved because they touch a protected
    /// (seam/door) edge.
    pub(crate) protected_count: usize,
}

// ---------------------------------------------------------------------
// Geometry helpers (std-only)
// ---------------------------------------------------------------------

fn signed_area_xz(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> f32 {
    0.5 * ((b[0] - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (b[2] - a[2]))
}

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
/// the XZ plane, or `None` when the point is outside or the triangle's XZ
/// projection is degenerate (a vertical wall triangle projects to a line, so
/// it can never "contain" a point -- exactly why walls never falsely
/// support).
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

// ---------------------------------------------------------------------
// The pass
// ---------------------------------------------------------------------

/// Runs the full validation + clearance pass on one mesh. An empty
/// `collision` list skips both collision-driven phases (F153.1/F153.2) --
/// never remove walkable area when there is no cooked collision to judge it
/// against -- and still applies clearance (F153.3). A zero/negative radius or
/// empty mesh is a no-op.
pub(crate) fn validate_and_clear(
    mesh: &NavClearanceMeshInput,
    collision: &[CollisionTriangle],
    params: NavClearanceParams,
) -> NavClearanceResult {
    let polygon_count = mesh.polygons.len();
    let mut walkable = vec![true; polygon_count];
    if params.agent_radius <= 0.0 || mesh.vertices.is_empty() || polygon_count == 0 {
        return NavClearanceResult {
            vertices: mesh.vertices.clone(),
            walkable,
            polygon_count,
            removed_unsupported: 0,
            cut_obstructed: 0,
            disconnected_narrow: 0,
            offset_count: 0,
            protected_count: 0,
        };
    }

    // Precompute collision-triangle bounds + normals for broadphase.
    let aabbs: Vec<TriAabb> = collision.iter().map(tri_aabb).collect();
    let normals: Vec<Option<[f32; 3]>> = collision.iter().map(triangle_normal).collect();

    // F153.1: collision-support validation (skipped when no collision).
    let mut removed_unsupported = 0usize;
    if !collision.is_empty() {
        for (index, tri) in mesh.polygons.iter().enumerate() {
            let Some(centroid) = polygon_centroid(mesh, *tri) else {
                continue;
            };
            if !is_supported(centroid, collision, &aabbs, params) {
                walkable[index] = false;
                removed_unsupported += 1;
            }
        }
    }

    // F153.2: interior-obstruction cutting (skipped when no collision).
    let mut cut_obstructed = 0usize;
    if !collision.is_empty() {
        for (index, tri) in mesh.polygons.iter().enumerate() {
            if !walkable[index] {
                continue;
            }
            let Some(centroid) = polygon_centroid(mesh, *tri) else {
                continue;
            };
            if is_obstructed(centroid, collision, &aabbs, &normals, params) {
                walkable[index] = false;
                cut_obstructed += 1;
            }
        }
    }

    // F153.3: clearance on the validated (post-removal/cut) walkable set.
    let (vertices, offset_count, protected_count, disconnected_narrow) =
        apply_clearance(mesh, &mut walkable, params.agent_radius);

    NavClearanceResult {
        vertices,
        walkable,
        polygon_count,
        removed_unsupported,
        cut_obstructed,
        disconnected_narrow,
        offset_count,
        protected_count,
    }
}

/// F153.1: is there a cooked collision surface under `point` within the
/// agent's step height (and a small margin above)? A wall triangle's XZ
/// projection is degenerate, so `barycentric_xz` rejects it -- only genuine
/// floor/ledge surfaces the agent could stand on count.
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

/// F153.2: does a wall-like collider rise into the agent capsule over
/// `point`, within the agent radius (XZ)? Floor/ceiling-like triangles are
/// excluded (they support, never obstruct); a collider must overlap the
/// agent's vertical body band `[floor, floor + height]` to wedge it.
fn is_obstructed(
    point: [f32; 3],
    collision: &[CollisionTriangle],
    aabbs: &[TriAabb],
    normals: &[Option<[f32; 3]>],
    params: NavClearanceParams,
) -> bool {
    let radius = params.agent_radius;
    let band_low = point[1];
    let band_high = point[1] + params.agent_height;
    for ((tri, aabb), normal) in collision.iter().zip(aabbs).zip(normals) {
        // Only wall-like triangles obstruct.
        match normal {
            Some(n) if n[1].abs() < WALL_NORMAL_Y_MAX => {}
            _ => continue,
        }
        // Must rise into the agent's vertical body band.
        if aabb.max[1] < band_low || aabb.min[1] > band_high {
            continue;
        }
        // XZ broadphase: reject if the triangle's XZ box is more than a
        // radius away on either axis.
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

/// F153.3: miter-corrected inward offset of the validated walkable region's
/// boundary, with sub-diameter corridors disconnecting. Mutates `walkable`
/// (dropping any polygon whose offset shape inverts/degenerates) and returns
/// `(offset_vertices, offset_count, protected_count, disconnected_narrow)`.
fn apply_clearance(
    mesh: &NavClearanceMeshInput,
    walkable: &mut [bool],
    radius: f32,
) -> (Vec<[f32; 3]>, usize, usize, usize) {
    let vertex_count = mesh.vertices.len();

    // Boundary edges of the *validated* walkable region: an edge referenced
    // by exactly one still-walkable polygon. `BTreeMap` keeps a fixed
    // iteration order regardless of hash seed, matching the codebase's
    // deterministic convention.
    let mut edge_polygons: BTreeMap<(u32, u32), Vec<usize>> = BTreeMap::new();
    for (poly_index, tri) in mesh.polygons.iter().enumerate() {
        if !walkable[poly_index] {
            continue;
        }
        for &(a, b) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
            let key = if a <= b { (a, b) } else { (b, a) };
            edge_polygons.entry(key).or_default().push(poly_index);
        }
    }

    // Per-vertex incident boundary normals (horizontal XZ, inward-facing).
    let mut vertex_normals: Vec<Vec<(f32, f32)>> = vec![Vec::new(); vertex_count];
    for (&(a, b), polys) in &edge_polygons {
        if polys.len() != 1 {
            continue; // interior edge, not a wall/void boundary.
        }
        let Some(&poly_index) = polys.first() else {
            continue;
        };
        let Some(tri) = mesh.polygons.get(poly_index) else {
            continue;
        };
        let Some(&third) = tri.iter().find(|&&v| v != a && v != b) else {
            continue;
        };
        let (Some(&pa), Some(&pb), Some(&pthird)) = (
            mesh.vertices.get(a as usize),
            mesh.vertices.get(b as usize),
            mesh.vertices.get(third as usize),
        ) else {
            continue;
        };
        let ex = pb[0] - pa[0];
        let ez = pb[2] - pa[2];
        let (n1x, n1z) = (-ez, ex);
        let mid_x = (pa[0] + pb[0]) * 0.5;
        let mid_z = (pa[2] + pb[2]) * 0.5;
        let dot = n1x * (pthird[0] - mid_x) + n1z * (pthird[2] - mid_z);
        let (nx, nz) = if dot >= 0.0 { (n1x, n1z) } else { (-n1x, -n1z) };
        let len = (nx * nx + nz * nz).sqrt();
        if len <= f32::EPSILON {
            continue;
        }
        let (ux, uz) = (nx / len, nz / len);
        if let Some(list) = vertex_normals.get_mut(a as usize) {
            list.push((ux, uz));
        }
        if let Some(list) = vertex_normals.get_mut(b as usize) {
            list.push((ux, uz));
        }
    }

    // Miter scale for a vertex's incident normals: `radius` for 0/1 (or
    // exactly-collinear) edges; else `radius / cos(theta/2)` for the widest
    // (smallest-dot) incident pair, clamped to `MITER_LIMIT_FACTOR * radius`.
    let miter_scale = |normals: &[(f32, f32)]| -> f32 {
        if normals.len() <= 1 {
            return radius;
        }
        let mut min_dot = 1.0f32;
        for i in 0..normals.len() {
            for j in (i + 1)..normals.len() {
                let (ax, az) = normals[i];
                let (bx, bz) = normals[j];
                let dot = (ax * bx + az * bz).clamp(-1.0, 1.0);
                if dot < min_dot {
                    min_dot = dot;
                }
            }
        }
        let cos_half = ((1.0 + min_dot) * 0.5).max(0.0).sqrt();
        radius / cos_half.max(MIN_COS_HALF_ANGLE)
    };

    let mut offsets: Vec<(f32, f32)> = vec![(0.0, 0.0); vertex_count];
    for (index, normals) in vertex_normals.iter().enumerate() {
        if normals.is_empty() {
            continue;
        }
        let (mut sx, mut sz) = (0.0f32, 0.0f32);
        for &(nx, nz) in normals {
            sx += nx;
            sz += nz;
        }
        let len = (sx * sx + sz * sz).sqrt();
        if len <= f32::EPSILON {
            continue; // incident normals cancel; no safe single direction.
        }
        let scale = miter_scale(normals).min(radius * MITER_LIMIT_FACTOR);
        offsets[index] = (sx / len * scale, sz / len * scale);
    }

    // Protected (seam/door) vertices never move.
    let mut protected_vertices: BTreeSet<u32> = BTreeSet::new();
    for &(a, b) in &mesh.protected_edges {
        protected_vertices.insert(a);
        protected_vertices.insert(b);
    }
    for &vertex in &protected_vertices {
        if let Some(offset) = offsets.get_mut(vertex as usize) {
            *offset = (0.0, 0.0);
        }
    }
    let protected_count = protected_vertices.len();

    // Apply offsets (X/Z only; Y untouched -- FO3 floors near-flat).
    let mut vertices = mesh.vertices.clone();
    for (index, vertex) in vertices.iter_mut().enumerate() {
        let (ox, oz) = offsets[index];
        vertex[0] += ox;
        vertex[2] += oz;
    }

    // Mesh dominant winding sign, from reliable-area walkable polygons, so a
    // single offset triangle's sign can be judged consistent with the whole.
    let dominant_sign = dominant_winding_sign(mesh, walkable);

    // Disconnect: drop any walkable polygon whose offset shape inverted or
    // degenerated (a sub-diameter corridor throat). Reliable-area polygons
    // that flip are counted as narrow disconnections; unreliable slivers that
    // flip/degenerate are dropped too (they would otherwise make landmass
    // reject the whole mesh) but not counted as corridor throats.
    let mut disconnected_narrow = 0usize;
    for (poly_index, tri) in mesh.polygons.iter().enumerate() {
        if !walkable[poly_index] {
            continue;
        }
        let (Some(&a), Some(&b), Some(&c)) = (
            vertices.get(tri[0] as usize),
            vertices.get(tri[1] as usize),
            vertices.get(tri[2] as usize),
        ) else {
            continue;
        };
        let area = signed_area_xz(a, b, c);
        let original = original_area(mesh, *tri);
        let degenerate = area.abs() < MIN_TRIANGLE_AREA;
        let inverted = match dominant_sign {
            Some(sign) => (area > 0.0) != sign,
            None => false,
        };
        if degenerate || inverted {
            walkable[poly_index] = false;
            if inverted && original.abs() >= MIN_RELIABLE_ORIGINAL_AREA {
                disconnected_narrow += 1;
            }
        }
    }

    // offset_count: still-walkable polygons that actually moved.
    let mut offset_count = 0usize;
    for (poly_index, tri) in mesh.polygons.iter().enumerate() {
        if !walkable[poly_index] {
            continue;
        }
        let moved = tri.iter().any(|&v| {
            let Some(&(ox, oz)) = offsets.get(v as usize) else {
                return false;
            };
            ox.abs() > 1.0e-6 || oz.abs() > 1.0e-6
        });
        if moved {
            offset_count += 1;
        }
    }

    (vertices, offset_count, protected_count, disconnected_narrow)
}

fn original_area(mesh: &NavClearanceMeshInput, tri: [u32; 3]) -> f32 {
    match (
        mesh.vertices.get(tri[0] as usize),
        mesh.vertices.get(tri[1] as usize),
        mesh.vertices.get(tri[2] as usize),
    ) {
        (Some(&a), Some(&b), Some(&c)) => signed_area_xz(a, b, c),
        _ => 0.0,
    }
}

/// Dominant winding sign (`true` = positive area) across the walkable
/// polygons with a reliable original area, `None` when none qualify.
fn dominant_winding_sign(mesh: &NavClearanceMeshInput, walkable: &[bool]) -> Option<bool> {
    let mut positive = 0usize;
    let mut negative = 0usize;
    for (poly_index, tri) in mesh.polygons.iter().enumerate() {
        if !walkable[poly_index] {
            continue;
        }
        let area = original_area(mesh, *tri);
        if area.abs() < MIN_RELIABLE_ORIGINAL_AREA {
            continue;
        }
        if area > 0.0 {
            positive += 1;
        } else {
            negative += 1;
        }
    }
    if positive == 0 && negative == 0 {
        None
    } else {
        Some(positive >= negative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A flat floor quad (two triangles) covering `[x0,x1] x [z0,z1]` at
    /// height `y`, as world-space collision triangles. Its top surface
    /// supports nav triangles at `y`.
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
    fn empty_collision_never_removes_walkable_area() {
        let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
        let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
        assert!(result.walkable.iter().all(|&w| w));
        assert_eq!(result.removed_unsupported, 0);
        assert_eq!(result.cut_obstructed, 0);
    }

    #[test]
    fn a_triangle_over_a_void_is_removed_as_unsupported() {
        // Floor only covers x in [0, 2]; nav covers x in [0, 4], so the
        // far triangle's centroid (x ~ 2.67) sits over a void.
        let mesh = nav_quad(0.0, 4.0, 0.0, 2.0, 0.0);
        let collision = floor(0.0, 2.0, -0.5, 2.5, 0.0);
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        // Polygon 0 = [0,1,2] centroid x ~ 2.67 (void), polygon 1 = [0,2,3]
        // centroid x ~ 1.33 (supported).
        assert_eq!(result.removed_unsupported, 1, "{:?}", result);
        assert!(
            !result.walkable[0],
            "far triangle over void must be removed"
        );
        assert!(result.walkable[1], "near triangle over floor must survive");
    }

    #[test]
    fn a_fully_supported_quad_survives_validation() {
        let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
        let collision = floor(-1.0, 5.0, -1.0, 5.0, 0.0);
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        assert_eq!(result.removed_unsupported, 0, "{:?}", result);
        assert_eq!(result.cut_obstructed, 0);
    }

    #[test]
    fn a_floor_a_full_step_below_still_supports() {
        let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
        // Floor 0.4 m below, inside the 0.5 m step height.
        let collision = floor(-1.0, 5.0, -1.0, 5.0, -0.4);
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        assert_eq!(result.removed_unsupported, 0, "{:?}", result);
    }

    #[test]
    fn a_floor_beyond_the_step_below_does_not_support() {
        let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
        // Floor 1.0 m below, beyond the 0.5 m step height -> unsupported.
        let collision = floor(-1.0, 5.0, -1.0, 5.0, -1.0);
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        assert_eq!(result.removed_unsupported, 2, "{:?}", result);
        assert!(result.walkable.iter().all(|&w| !w));
    }

    #[test]
    fn an_interior_wall_cuts_the_overlapping_triangle_but_leaves_the_opening() {
        // Supporting floor everywhere; a short wall stub near x=1, z=0.5
        // rising into the agent band. The near triangle's centroid is within
        // a radius of it; the far one is not.
        let mesh = nav_quad(0.0, 4.0, 0.0, 1.0, 0.0);
        let mut collision = floor(-1.0, 5.0, -1.0, 2.0, 0.0);
        // Wall stub covering only x in [1.1, 1.5] at z=0.5, rising 2 m.
        collision.extend(wall(1.1, 1.5, 0.5, 0.0, 2.0));
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        assert_eq!(result.removed_unsupported, 0, "{:?}", result);
        // At least one triangle cut, not both (the opening survives).
        assert!(result.cut_obstructed >= 1, "{:?}", result);
        assert!(
            result.walkable.iter().any(|&w| w),
            "the doorway opening must stay walkable: {:?}",
            result
        );
    }

    #[test]
    fn a_low_ledge_below_agent_height_does_not_obstruct() {
        // A knee-high wall stub (0.3 m) does not reach into the agent band
        // enough... it starts at floor, so it DOES overlap [0, 1.8]. To model
        // a step-over ledge the collider must sit fully below the band: here
        // a ledge from y=-1 to y=-0.2, entirely below the floor band start.
        let mesh = nav_quad(0.0, 4.0, 0.0, 1.0, 0.0);
        let mut collision = floor(-1.0, 5.0, -1.0, 2.0, 0.0);
        collision.extend(wall(1.1, 1.5, 0.5, -1.0, -0.2));
        let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        assert_eq!(result.cut_obstructed, 0, "{:?}", result);
    }

    #[test]
    fn a_wide_room_corner_gets_full_radius_clearance_per_wall() {
        // Reuse of wave-6's L-corner invariant: perpendicular clearance to
        // each original wall is >= radius (the miter, not the fixed average).
        let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
        let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
        let original = mesh.vertices[0]; // corner at (0,0)
        let eroded = result.vertices[0];
        assert!(
            eroded[0] > original[0] && eroded[2] > original[2],
            "corner must move into the room: {eroded:?}"
        );
        // 90-degree corner -> displacement radius*sqrt(2), clearance radius.
        let dx = eroded[0] - original[0];
        let dz = eroded[2] - original[2];
        let displacement = (dx * dx + dz * dz).sqrt();
        let expected = AGENT_RADIUS * std::f32::consts::SQRT_2;
        assert!(
            (displacement - expected).abs() < 1.0e-4,
            "expected miter displacement {expected}, got {displacement}"
        );
    }

    /// A chain of `segments` corridor cells of the given `width` along X
    /// between two long walls at z=0 and z=width -- the same fixture shape
    /// wave-6 `erosion_policy` used for its pinch tests.
    fn corridor(width: f32, segments: u32) -> NavClearanceMeshInput {
        let mut vertices = Vec::new();
        let mut polygons = Vec::new();
        for i in 0..=segments {
            let x = i as f32 * 2.0;
            vertices.push([x, 0.0, 0.0]);
            vertices.push([x, 0.0, width]);
        }
        for i in 0..segments {
            let ba = 2 * i;
            let ta = 2 * i + 1;
            let bb = 2 * (i + 1);
            let tb = 2 * (i + 1) + 1;
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
    fn a_sub_diameter_corridor_disconnects_instead_of_being_preserved() {
        // Width 0.3 m, radius 0.35: full clearance from both walls (0.35 each)
        // overlaps by 0.4 m, well past inversion. Wave-6 preserved these
        // impassable; this issue disconnects them (no route).
        let mesh = corridor(0.3, 3);
        let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
        assert!(
            result.disconnected_narrow > 0,
            "a sub-diameter corridor must disconnect: {:?}",
            result
        );
        assert!(
            result.walkable.iter().any(|&w| !w),
            "at least one throat polygon must be dropped: {:?}",
            result
        );
    }

    #[test]
    fn a_wide_corridor_stays_connected_after_clearance() {
        // Width 3.0 m, far wider than 2*radius: no disconnection.
        let mesh = corridor(3.0, 3);
        let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
        assert_eq!(result.disconnected_narrow, 0, "{:?}", result);
        assert!(
            result.walkable.iter().all(|&w| w),
            "a wide corridor must stay fully walkable: {:?}",
            result
        );
        assert!(result.offset_count > 0);
    }

    #[test]
    fn protected_seam_vertices_never_move() {
        let mut mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
        // Protect the z=0 edge (vertices 0-1).
        mesh.protected_edges = vec![(0, 1)];
        let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
        assert_eq!(result.vertices[0], mesh.vertices[0], "seam vertex 0 fixed");
        assert_eq!(result.vertices[1], mesh.vertices[1], "seam vertex 1 fixed");
        assert!(result.protected_count == 2);
    }

    #[test]
    fn the_pass_is_deterministic_across_calls() {
        let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
        let collision = floor(-1.0, 5.0, -1.0, 5.0, 0.0);
        let first = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
        let second = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
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
        assert_eq!(result.vertices, mesh.vertices);
        assert!(result.walkable.iter().all(|&w| w));
    }
}
