//! Pure navmesh erosion policy (issue #136): shrinks the walkable polygon
//! boundary inward by the agent radius so `bevy_landmass`'s path smoothing
//! does not string-pull a route within less than a capsule-width of
//! wall/prop geometry.
//!
//! Context: FO3 `NAVM` meshes are authored without any agent-radius margin,
//! and since M4 wave 5 made agent movement physics-authoritative (a real
//! 0.35 m kinematic capsule character controller, `nav::agent`'s
//! `AGENT_RADIUS`/`player::CAPSULE_RADIUS`), a route that hugs the
//! un-eroded navmesh boundary can put the capsule's collider inside a wall
//! or prop, wedging it (`collision-blocked` -> `stuck`). This module is the
//! fix: it moves each navmesh vertex on the walkable-region boundary inward
//! by up to the agent radius before the mesh reaches `bevy_landmass`.
//!
//! **Landmass-native option checked first, per the wave plan (F136.1):**
//! `landmass` 0.9.1 / `bevy_landmass` 0.12.0 (this project's pinned
//! versions, `Cargo.toml`) were read before writing this module.
//! `landmass::Agent::radius` (`ArchipelagoOptions::from_agent_radius`, used
//! by `nav::agent::ensure_archipelago`) feeds only local avoidance
//! (RVO-style agent/agent separation, `landmass::avoidance`) and
//! `PointSampleDistance3d::from_agent_radius`'s off-navmesh sampling
//! envelope -- neither erodes navmesh polygon boundaries or offsets path
//! smoothing away from static geometry. `landmass::nav_mesh`/`pathfinding`
//! have no wall-clearance or polygon-offset option at all. So there is no
//! lazy landmass-side switch to flip; this module does the erosion itself.
//!
//! **Why erosion moves vertex positions, not polygon topology:** the
//! prepared navmesh is a triangle soup where adjacent triangles literally
//! share a vertex *index* into a common vertex array -- that shared index
//! is exactly what `bevy_landmass`'s `NavigationMesh3d::validate()` uses to
//! detect two polygons as connected. Eroding each polygon independently
//! (producing per-polygon private vertices) would sever every triangle from
//! its neighbours and disconnect nearly the whole mesh. Instead this module
//! only ever *moves* existing vertex positions -- indices, and therefore
//! connectivity, are untouched by construction, which is what makes
//! F136.2's "must not disconnect a previously connected walkable region"
//! trivially true rather than something the algorithm has to prove per
//! call.
//!
//! **Algorithm.** An edge shared by exactly one polygon is a walkable-region
//! boundary edge (a wall, prop cutout, or non-walkable/water exclusion --
//! `landmass_graph::build_navigation_mesh` already excludes water before
//! calling this, so those edges show up as boundary edges here too). Each
//! boundary edge contributes an inward-facing unit normal (2D, horizontal
//! X/Z plane -- matching wave 5's horizontal-plane treatment of nav-point
//! distance, since FO3 floors are near-flat and vertical erosion is not
//! this issue's problem) to both of its endpoint vertices. Interior
//! vertices (touching no boundary edge) get a zero offset and never move.
//!
//! **Miter offset, not a fixed-length average (issue #136 follow-up,
//! external review).** A vertex touching exactly one boundary edge (a
//! straight wall run, or two exactly-collinear boundary edges) moves along
//! that edge's normal by exactly `radius`. A vertex touching two boundary
//! edges whose unit inward normals `n1`, `n2` subtend angle `theta`
//! (`cos(theta) = n1 . n2`) is a corner: moving it by `radius` along the
//! averaged normal only gives each wall `radius * cos(theta / 2)`
//! clearance, not `radius` -- at a 90-degree corner that is `r / sqrt(2)`,
//! about 30% short of the agent's actual capsule radius. The correct move
//! is the classic polygon-offset miter: scale the displacement along the
//! averaged normal by `radius / cos(theta / 2)` (`cos(theta / 2) =
//! sqrt((1 + n1 . n2) / 2)`, the half-angle identity), which is exactly the
//! amount that puts each wall back at `radius` clearance. A vertex
//! touching more than two boundary edges uses the *widest* such pair (the
//! smallest `n_i . n_j`) to set the scale, since that pair needs the most
//! aggressive miter to keep every incident edge at or above `radius`; the
//! displacement direction is still the renormalized sum of all incident
//! normals. [`MITER_LIMIT_FACTOR`] caps how far a near-reversed pair (a
//! thin spike) can push a vertex, matching the order of magnitude
//! Recast/Clipper-style polygon offsetters use for their own miter limits.
//!
//! **Corridor-pinch safety (F136.2), two phases over the WHOLE mesh at
//! once.** Applying every vertex's raw offset at full strength can invert
//! or degenerate a polygon whose available width is less than `2 *
//! radius` (a corridor narrower than the agent) -- and because boundary
//! vertices are shared between triangles, fixing one polygon's shape by
//! moving its vertices can change a *different*, neighbouring polygon
//! that shares one of those same vertices. A first version of this module
//! checked each polygon's validity only against its own three vertices'
//! *current* factors and stopped as soon as one pass made no changes;
//! that missed a real case (issue #136 follow-up, Vault 101 Entrance mesh
//! `0001862b`, polygon 164): a vertex's factor reached this module's old
//! per-vertex floor and got silently clamped to exactly zero without the
//! pass being marked as "changed", so the loop exited one pass too early
//! -- before a neighbouring polygon sharing that vertex was re-checked
//! against the vertex's *final* (just-zeroed) position. That polygon's
//! shape had actually flipped, but nothing re-validated it, and
//! `bevy_landmass`'s `validate()` rejected the whole mesh.
//!
//! The fix is two unconditional phases over every polygon, both driven by
//! one shared `factors` array (so a change to a vertex from checking one
//! polygon is immediately visible to every other polygon that shares it,
//! not just its own three vertices):
//!
//! 1. **Proportional relaxation.** Each vertex carries a `[0, 1]` erosion
//!    factor, starting at `1.0`. Every polygon whose *current*
//!    (factor-scaled) shape has flipped winding sign or dropped below a
//!    minimum area relative to its original shape has its three
//!    vertices' factors halved -- unconditionally, no per-vertex floor
//!    that could stop short of a real change -- and the pass is marked
//!    changed. Repeats until a full pass makes no changes, or a generous
//!    fixed pass cap is hit. This alone does not have to reach an exactly
//!    valid state (halving a nonzero `f32` never reaches exactly zero in
//!    finitely many steps), so it hands off to phase 2 rather than
//!    asserting success.
//! 2. **Hard fallback.** A second, unconditional pass over every polygon:
//!    anything *still* invalid after phase 1 has its vertices' factors
//!    forced to exactly `0.0` (their original, pre-erosion position --
//!    already guaranteed valid, since `landmass_graph` only calls this
//!    with polygons it has already filtered for degeneracy). This can
//!    only ever *reduce* the set of nonzero-factor vertices (never
//!    increase it), so it is bounded by the vertex count and always
//!    terminates at a fully valid mesh -- the true "final fallback is
//!    zero displacement for the vertices that can't move safely".
//!
//! Both phases are deterministic (fixed polygon/vertex iteration order,
//! no hashing) and their combined pass count is returned as
//! [`ErosionResult::relax_passes`] so the diagnostic log line makes this
//! relaxation activity visible (F136.3 follow-up).
//!
//! **Protected (seam/portal) edges, second real-data regression.** A
//! multi-mesh cell's meshes never share coincident boundary vertices --
//! `nav/agent.rs`'s own module doc comment on `merge_link_descriptors`
//! notes real FO3 meshes are joined by generated animation links between
//! matched triangle midpoints, not landmass's native geometric adjacency.
//! From a single mesh's own triangle-soup topology, the boundary edge of
//! a cross-mesh merge triangle or a door-link/travel-door triangle looks
//! *exactly* like a wall (referenced by exactly one polygon within this
//! mesh) -- eroding it inward by the agent radius on both sides of the
//! seam independently opened a real ~0.7 m gap between the two meshes'
//! islands (Franklin Metro 02, mesh `0001a273`'s cell) where a link used
//! to connect them, and could equally push a door-link/travel-door
//! attachment triangle far enough that `bevy_landmass`'s own on-mesh
//! sampling tolerance no longer recognizes it.
//!
//! The fix is the same "final fallback is zero displacement" idea phase 2
//! already uses, applied unconditionally rather than only on a validation
//! failure: any vertex touching an `ErosionMeshInput::protected_edges`
//! edge has its raw offset forced to `(0.0, 0.0)` before phase 1 even
//! runs, so it can never move, full stop -- deliberately the documented
//! *lazy* rule (a vertex shared between a protected edge and a genuine
//! wall edge still loses the wall's push too, rather than the more
//! precise "exclude only the protected edge's own normal contribution"),
//! chosen because it guarantees the exact same seam vertex position on
//! both sides of a merge (each mesh is eroded independently, so only
//! *never moving* the seam at all guarantees both sides still agree
//! afterward) with no risk of the two sides drifting apart by different,
//! independently-computed amounts.
//!
//! Std-only (no `bevy`/`bevy_landmass`/`glam`, not even `serde`): this file
//! is included verbatim by `tests/features.rs` via `#[path]` -- see
//! `src/viewer/world/policy.rs`'s module doc comment for why modules driven
//! from cucumber take this shape, and `landmass_graph.rs`'s module doc
//! comment for the sibling flat top-level `#[path]` inclusion this depends
//! on lining up against.
//!
//! **Agent radius constant.** `nav::agent::AGENT_RADIUS` (0.35 m) is
//! private to `agent.rs`. This module holds its own copy rather than
//! importing it, so `erosion_policy.rs`/`landmass_graph.rs` share exactly
//! one value between themselves; unifying it with `nav::agent::
//! AGENT_RADIUS` / `player::CAPSULE_RADIUS` (also 0.35 m) is left to a
//! separate follow-up, since it is purely a private-visibility
//! housekeeping change orthogonal to this module's actual behaviour.
use std::collections::{BTreeMap, BTreeSet};

/// Agent capsule radius (metres) navmesh polygons are eroded by. See the
/// module doc comment for why this is not imported from `nav::agent`.
pub(crate) const AGENT_RADIUS: f32 = 0.35;

/// A triangle-soup navmesh, in the same plain shape
/// `landmass_graph::MeshInput`/`PolygonInput` already use (vertex array +
/// index triples), but without any of that module's water/door/form-id
/// fields this erosion pass has no use for.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ErosionMeshInput {
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) polygons: Vec<[u32; 3]>,
    /// Boundary edges (unordered vertex-index pairs) that are seam/portal
    /// connections to another mesh or an off-mesh link attachment (door
    /// links, travel doors, cross-mesh merges -- issue #136 follow-up),
    /// not real walls. A vertex touching a protected edge never moves
    /// (see the module doc comment's "protected edges" section) --
    /// erosion would otherwise treat these exactly like a wall, since
    /// within a single mesh's own triangle-soup topology they look
    /// identical (referenced by exactly one polygon).
    pub(crate) protected_edges: Vec<(u32, u32)>,
}

/// Result of eroding one [`ErosionMeshInput`]: the eroded vertex positions
/// (same length/order as the input -- indices are untouched, only
/// positions move), plus the F136.3 diagnostic counters.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ErosionResult {
    pub(crate) vertices: Vec<[f32; 3]>,
    /// Total polygon count considered (the walkable polygons fed in).
    pub(crate) polygon_count: usize,
    /// Polygons with at least one vertex that actually moved.
    pub(crate) eroded_count: usize,
    /// Polygons where the corridor-pinch guard reduced erosion below full
    /// strength for at least one of their vertices, in either phase.
    pub(crate) pinch_guard_count: usize,
    /// Total number of corrective passes actually run (phase 1 +
    /// phase 2 combined) to reach a valid mesh -- 0 when every polygon
    /// validated at full erosion strength on the first check. Makes the
    /// relaxation activity described in the module doc comment visible
    /// in the F136.3 diagnostic line, distinct from `pinch_guard_count`
    /// (how many polygons needed adjustment at all).
    pub(crate) relax_passes: usize,
    /// Distinct vertices left unmoved because they touch a
    /// `ErosionMeshInput::protected_edges` seam/portal edge (issue #136
    /// follow-up), regardless of what a wall push would otherwise have
    /// computed for them.
    pub(crate) protected_count: usize,
}

/// Below this scaled-triangle area (2D, horizontal plane, same units as
/// `vertices`), a candidate erosion is treated as degenerate.
const MIN_TRIANGLE_AREA: f32 = 1.0e-4;
/// Below this original-triangle area, the triangle's *sign* is not just
/// literally zero (that is `MIN_TRIANGLE_AREA`'s job) but too close to
/// the f32 noise floor to trust as an "expected orientation" for the
/// sign-flip comparison in `is_invalid_at`. Real FO3 NAVM data has been
/// observed to contain deliberately-tiny sliver triangles whose original
/// area sits right at that noise floor (issue #136 follow-up: Vault 101
/// Entrance mesh `0001862b` polygon 164, original area ~7.7e-6) while
/// erosion resurrects the same triangle into a large, well-defined shape
/// (~0.046) whose sign has no relationship to the meaningless original
/// one -- landmass rejected the resulting mesh because that resurrected
/// orientation did not match the rest of the mesh's winding convention,
/// and this module's old two-value (degenerate / reliable) split had no
/// way to flag it, since it isn't literally zero. This threshold sits
/// comfortably inside the four-order-of-magnitude gap real data showed
/// between that one pathological triangle and the next-smallest genuine
/// triangle in the same graph (~0.035): a full order of magnitude above
/// `MIN_TRIANGLE_AREA` and more than 3x below the smallest real triangle
/// observed, so it only catches the former, never the latter.
const MIN_RELIABLE_ORIGINAL_AREA: f32 = 1.0e-2;
/// Fixed pass limit for phase 1's proportional-relaxation loop (see
/// module doc comment). Generous rather than tight: phase 1 is an
/// optimization over phase 2's hard fallback, not the correctness
/// guarantee, so hitting this cap without full convergence is fine --
/// phase 2 always finishes the job.
const MAX_RELAX_PASSES: usize = 64;
/// Miter limit (issue #136 follow-up, external review): caps a corner
/// vertex's displacement at `MITER_LIMIT_FACTOR * radius` regardless of how
/// sharp (close to 180 degrees) the angle between its incident boundary
/// normals is. Without this, a thin spike (two nearly-reversed boundary
/// edges meeting at one vertex) would compute an unboundedly large
/// `radius / cos(theta / 2)` scale as `theta` approaches 180 degrees. `2.5`
/// matches the order of magnitude Recast/Clipper-style polygon offsetters
/// use for their own miter limits; the corridor-pinch guard (phase 1/2
/// below) still catches anything this clamp lets through that would invert
/// or degenerate a polygon.
const MITER_LIMIT_FACTOR: f32 = 2.5;
/// Floor for `cos(theta / 2)` in the miter-scale division, guarding against
/// a divide-by-(near)zero as the angle between two incident boundary
/// normals approaches 180 degrees (a near-total reversal).
/// `MITER_LIMIT_FACTOR` above is what actually bounds the result in that
/// regime; this constant just keeps the division itself finite.
const MIN_COS_HALF_ANGLE: f32 = 1.0e-4;

fn signed_area_xz(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> f32 {
    0.5 * ((b[0] - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (b[2] - a[2]))
}

/// Erodes `mesh`'s walkable-region boundary inward by `radius` (metres),
/// with the corridor-pinch guard described in the module doc comment.
/// `radius <= 0.0` or an empty mesh is a no-op (original vertices
/// returned, all counters zero).
pub(crate) fn erode(mesh: &ErosionMeshInput, radius: f32) -> ErosionResult {
    let vertex_count = mesh.vertices.len();
    let polygon_count = mesh.polygons.len();
    if radius <= 0.0 || vertex_count == 0 || polygon_count == 0 {
        return ErosionResult {
            vertices: mesh.vertices.clone(),
            polygon_count,
            eroded_count: 0,
            pinch_guard_count: 0,
            relax_passes: 0,
            protected_count: 0,
        };
    }

    // 1. Boundary-edge detection: an edge (unordered vertex-index pair)
    // referenced by exactly one polygon. `BTreeMap` (not `HashMap`) keeps
    // iteration order fixed by key regardless of process hash-seed, so the
    // per-vertex normal accumulation below sums in the same order on every
    // call -- matching this codebase's existing "deterministic across
    // calls" testing convention (see `landmass_graph::door_link_descriptors`
    // and its tests) rather than relying on incidental hash stability.
    let mut edge_polygons: BTreeMap<(u32, u32), Vec<usize>> = BTreeMap::new();
    for (poly_index, triangle) in mesh.polygons.iter().enumerate() {
        for &(a, b) in &[
            (triangle[0], triangle[1]),
            (triangle[1], triangle[2]),
            (triangle[2], triangle[0]),
        ] {
            let key = if a <= b { (a, b) } else { (b, a) };
            edge_polygons.entry(key).or_default().push(poly_index);
        }
    }

    // 2. Per-vertex raw inward offset (horizontal X/Z plane only -- Y is
    // left untouched, see module doc comment). Collect each boundary edge's
    // contributed unit normal per endpoint vertex (rather than only a
    // running sum) so the miter scale below can inspect the actual angle
    // between a vertex's incident normals, not just their average
    // direction -- see the module doc comment's "Miter offset" section.
    let mut vertex_normals: Vec<Vec<(f32, f32)>> = vec![Vec::new(); vertex_count];
    for (&(a, b), polys) in &edge_polygons {
        if polys.len() != 1 {
            continue; // interior edge: shared by two polygons, not a wall.
        }
        let Some(&poly_index) = polys.first() else {
            continue;
        };
        let Some(triangle) = mesh.polygons.get(poly_index) else {
            continue;
        };
        let Some(&third) = triangle.iter().find(|&&v| v != a && v != b) else {
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
        // Perpendicular-to-edge candidate; pick whichever of the two signs
        // points toward the triangle's third vertex (i.e. into the
        // polygon's interior, away from the wall).
        let (n1x, n1z) = (-ez, ex);
        let mid_x = (pa[0] + pb[0]) * 0.5;
        let mid_z = (pa[2] + pb[2]) * 0.5;
        let to_third_x = pthird[0] - mid_x;
        let to_third_z = pthird[2] - mid_z;
        let dot = n1x * to_third_x + n1z * to_third_z;
        let (nx, nz) = if dot >= 0.0 { (n1x, n1z) } else { (-n1x, -n1z) };
        let len = (nx * nx + nz * nz).sqrt();
        if len <= f32::EPSILON {
            continue; // degenerate edge (zero length in the X/Z plane).
        }
        let (ux, uz) = (nx / len, nz / len);
        if let Some(list) = vertex_normals.get_mut(a as usize) {
            list.push((ux, uz));
        }
        if let Some(list) = vertex_normals.get_mut(b as usize) {
            list.push((ux, uz));
        }
    }

    // Miter scale for a vertex's incident boundary normals: `radius` for
    // zero or one incident edge (or exactly-collinear edges, where the
    // widest pair's angle is zero); otherwise `radius / cos(theta / 2)`
    // for the widest (largest-angle, i.e. smallest dot product) pair, per
    // the module doc comment's "Miter offset" section, clamped to
    // `MITER_LIMIT_FACTOR * radius` so a near-reversed (spike) pair cannot
    // push the vertex arbitrarily far.
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
        let cos_half_angle = ((1.0 + min_dot) * 0.5).max(0.0).sqrt();
        radius / cos_half_angle.max(MIN_COS_HALF_ANGLE)
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
            continue; // incident normals cancel out; no safe single direction.
        }
        let scale = miter_scale(normals).min(radius * MITER_LIMIT_FACTOR);
        offsets[index] = (sx / len * scale, sz / len * scale);
    }

    // 2b. Protected (seam/portal) edges: never move a vertex that touches
    // one, unconditionally -- see the module doc comment's "protected
    // edges" section for why this is a deliberately lazy full stop
    // rather than only excluding the protected edge's own normal
    // contribution.
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

    // 3. Corridor-pinch guard: shrink a polygon's vertex factors whenever
    // its current (factor-scaled) shape would invert or degenerate,
    // deterministically (fixed polygon/vertex order, no randomness).
    let original_areas: Vec<f32> = mesh
        .polygons
        .iter()
        .map(|triangle| {
            let a = mesh.vertices.get(triangle[0] as usize).copied();
            let b = mesh.vertices.get(triangle[1] as usize).copied();
            let c = mesh.vertices.get(triangle[2] as usize).copied();
            match (a, b, c) {
                (Some(a), Some(b), Some(c)) => signed_area_xz(a, b, c),
                _ => 0.0,
            }
        })
        .collect();

    let mut factors = vec![1.0f32; vertex_count];
    let mut pinch_guard_marked = vec![false; polygon_count];

    let eroded_position = |mesh: &ErosionMeshInput,
                           offsets: &[(f32, f32)],
                           factors: &[f32],
                           index: u32|
     -> Option<[f32; 3]> {
        let base = *mesh.vertices.get(index as usize)?;
        let &(ox, oz) = offsets.get(index as usize)?;
        let f = *factors.get(index as usize)?;
        Some([base[0] + ox * f, base[1], base[2] + oz * f])
    };

    // Returns `true` when the polygon at `poly_index`'s CURRENT
    // (factor-scaled) shape has flipped winding sign or dropped below the
    // minimum area relative to its own original shape.
    //
    // An original area below `MIN_RELIABLE_ORIGINAL_AREA` (see that
    // constant's doc comment -- this includes, and is strictly wider
    // than, literally-degenerate/near-zero original shapes, so there is
    // no separate `MIN_TRIANGLE_AREA`-only early return here: that
    // stricter threshold is a subset of this one and checking it first
    // would make this branch unreachable exactly for the pathological
    // case it exists to catch) has an unreliable sign, never used as the
    // "expected orientation" to compare against. Such a polygon is
    // instead conservatively treated as invalid for as long as any of
    // its vertices still carries a nonzero erosion factor, driving it
    // toward full zero displacement (its original, already-valid shape)
    // rather than trusting whatever orientation erosion happens to give
    // it. An out-of-range index falls through to `false` below, guarding
    // a divide-by-zero-style sign comparison on a shape with no
    // meaningful sign -- upstream (`landmass_graph`) already filters
    // invalid polygons before calling erosion, so that should never
    // actually trigger on real input.
    let is_invalid_at = |poly_index: usize, triangle: &[u32; 3], factors: &[f32]| -> bool {
        let Some(&original_area) = original_areas.get(poly_index) else {
            return false;
        };
        if original_area.abs() < MIN_RELIABLE_ORIGINAL_AREA {
            return triangle
                .iter()
                .any(|&v| factors.get(v as usize).copied().unwrap_or(0.0) != 0.0);
        }
        let (Some(a), Some(b), Some(c)) = (
            eroded_position(mesh, &offsets, factors, triangle[0]),
            eroded_position(mesh, &offsets, factors, triangle[1]),
            eroded_position(mesh, &offsets, factors, triangle[2]),
        ) else {
            return false;
        };
        let area = signed_area_xz(a, b, c);
        (area > 0.0) != (original_area > 0.0) || area.abs() < MIN_TRIANGLE_AREA
    };

    let mut relax_passes = 0usize;

    // Phase 1: proportional relaxation. See the module doc comment for why
    // this alone is not the correctness guarantee (halving never reaches
    // exactly zero) -- phase 2 below is what makes the result provably
    // valid.
    for _pass in 0..MAX_RELAX_PASSES {
        let mut changed = false;
        for (poly_index, triangle) in mesh.polygons.iter().enumerate() {
            if is_invalid_at(poly_index, triangle, &factors) {
                pinch_guard_marked[poly_index] = true;
                for &v in triangle {
                    if let Some(factor) = factors.get_mut(v as usize) {
                        *factor *= 0.5;
                    }
                }
                changed = true;
            }
        }
        if changed {
            relax_passes += 1;
        } else {
            break;
        }
    }

    // Phase 2: hard fallback. Bounded by `vertex_count + 1` passes -- each
    // productive pass zeroes at least one vertex factor that was still
    // nonzero, a strictly shrinking finite set, so this always terminates,
    // and an all-zero configuration is definitionally the original mesh
    // (already valid). This is what guarantees `erode()` never returns an
    // inverted or degenerate triangle, regardless of how phase 1 fared.
    for _pass in 0..=vertex_count {
        let mut changed = false;
        for (poly_index, triangle) in mesh.polygons.iter().enumerate() {
            if is_invalid_at(poly_index, triangle, &factors) {
                pinch_guard_marked[poly_index] = true;
                for &v in triangle {
                    if let Some(factor) = factors.get_mut(v as usize)
                        && *factor != 0.0
                    {
                        *factor = 0.0;
                        changed = true;
                    }
                }
            }
        }
        if changed {
            relax_passes += 1;
        } else {
            break;
        }
    }

    // Finalize eroded vertex positions.
    let mut eroded_vertices = mesh.vertices.clone();
    for (index, eroded) in eroded_vertices.iter_mut().enumerate() {
        let (ox, oz) = offsets[index];
        let factor = factors[index];
        if ox != 0.0 || oz != 0.0 {
            eroded[0] += ox * factor;
            eroded[2] += oz * factor;
        }
    }

    let mut eroded_count = 0usize;
    let mut pinch_guard_count = 0usize;
    for (poly_index, triangle) in mesh.polygons.iter().enumerate() {
        let moved = triangle.iter().any(|&v| {
            let Some(&(ox, oz)) = offsets.get(v as usize) else {
                return false;
            };
            let Some(&factor) = factors.get(v as usize) else {
                return false;
            };
            (ox * factor).abs() > 1.0e-6 || (oz * factor).abs() > 1.0e-6
        });
        if moved {
            eroded_count += 1;
        }
        if pinch_guard_marked[poly_index] {
            pinch_guard_count += 1;
        }
    }

    ErosionResult {
        vertices: eroded_vertices,
        polygon_count,
        eroded_count,
        pinch_guard_count,
        relax_passes,
        protected_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A single right-angle room corner: wall along X (A-B) and wall along
    /// Z (D-A) meeting at vertex A, split by diagonal A-C so the room is
    /// two triangles. Vertex A is the corner under test; B/C/D make the
    /// room's other three sides boundary too (an isolated room, not a
    /// corridor continuing further -- fine, this test only asserts A's
    /// movement).
    fn corner_room() -> ErosionMeshInput {
        ErosionMeshInput {
            vertices: vec![
                [0.0, 0.0, 0.0], // 0: A, the corner
                [4.0, 0.0, 0.0], // 1: B
                [4.0, 0.0, 4.0], // 2: C
                [0.0, 0.0, 4.0], // 3: D
            ],
            polygons: vec![[0, 1, 2], [0, 2, 3]],
            protected_edges: Vec::new(),
        }
    }

    /// Perpendicular distance (2D, horizontal X/Z plane) from `point` to the
    /// infinite line through `line_a`/`line_b`. Used to check the REAL
    /// erosion invariant -- clearance to each original wall line -- rather
    /// than displacement magnitude, which a miter corner deliberately
    /// exceeds `radius` (issue #136 follow-up, external review).
    fn distance_point_to_line_xz(point: [f32; 3], line_a: [f32; 3], line_b: [f32; 3]) -> f32 {
        let ex = line_b[0] - line_a[0];
        let ez = line_b[2] - line_a[2];
        let len = (ex * ex + ez * ez).sqrt();
        assert!(
            len > f32::EPSILON,
            "degenerate line: {line_a:?} == {line_b:?}"
        );
        let px = point[0] - line_a[0];
        let pz = point[2] - line_a[2];
        (ex * pz - ez * px).abs() / len
    }

    #[test]
    fn l_corner_vertex_keeps_radius_clearance_from_each_wall() {
        // Regression (issue #136 follow-up, external review): the old
        // implementation displaced the corner vertex by exactly `radius`
        // along the averaged wall normal, which only gives each wall
        // `radius * cos(45 deg)` = `radius / sqrt(2)` clearance at a
        // 90-degree corner -- about 0.247 m instead of the intended
        // 0.35 m agent radius. The miter fix scales the displacement by
        // `radius / cos(45 deg)` instead, so each wall keeps the full
        // `radius` clearance; the real invariant to check is exactly that
        // per-wall clearance, not the (now intentionally larger,
        // radius * sqrt(2)) displacement magnitude.
        let mesh = corner_room();
        let original = mesh.vertices[0];
        let wall_a_b = mesh.vertices[1]; // original A-B wall (Z = 0)
        let wall_d_a = mesh.vertices[3]; // original D-A wall (X = 0)
        let result = erode(&mesh, AGENT_RADIUS);
        let eroded = result.vertices[0];

        // A large room at this radius should not trip the pinch guard, so
        // the corner should get its full-strength miter offset.
        assert_eq!(result.pinch_guard_count, 0);
        assert!(result.eroded_count > 0);

        // A sits at the meeting point of the X=0 and Z=0 walls; clearance
        // means moving strictly into the room on both axes.
        assert!(
            eroded[0] > original[0] && eroded[2] > original[2],
            "corner vertex must move into the room on both axes: {eroded:?}"
        );

        let clearance_to_a_b = distance_point_to_line_xz(eroded, original, wall_a_b);
        let clearance_to_d_a = distance_point_to_line_xz(eroded, wall_d_a, original);
        assert!(
            (clearance_to_a_b - AGENT_RADIUS).abs() < 1.0e-4,
            "expected radius clearance from wall A-B, got {clearance_to_a_b}"
        );
        assert!(
            (clearance_to_d_a - AGENT_RADIUS).abs() < 1.0e-4,
            "expected radius clearance from wall D-A, got {clearance_to_d_a}"
        );

        // The displacement itself is the miter magnitude `radius /
        // cos(45 deg)` = `radius * sqrt(2)` for a 90-degree corner, not
        // `radius` -- this is the defect's fingerprint made explicit.
        let dx = eroded[0] - original[0];
        let dz = eroded[2] - original[2];
        let displacement = (dx * dx + dz * dz).sqrt();
        let expected_displacement = AGENT_RADIUS * std::f32::consts::SQRT_2;
        assert!(
            (displacement - expected_displacement).abs() < 1.0e-4,
            "expected miter displacement radius*sqrt(2) = {expected_displacement}, got {displacement}"
        );
    }

    #[test]
    fn obtuse_corner_vertex_keeps_radius_clearance_from_each_wall() {
        // A second corner-clearance regression case at a shallower, obtuse
        // 135-degree interior angle (turn angle 45 degrees, so the two
        // incident wall normals subtend 45 degrees rather than the square
        // corner's 90 degrees) -- constructed by chamfering one corner off
        // a large 6x6 room so every other vertex keeps a plain 90-degree
        // angle and nothing here is anywhere near narrow enough to trip
        // the corridor-pinch guard. Pentagon, fanned from vertex 0 (the
        // chamfered corner under test):
        //   0: (1, 0, 0) -- on the original Z=0 wall, 1 m from the corner
        //   1: (6, 0, 0) -- far corner along the Z=0 wall
        //   2: (6, 0, 6) -- opposite corner
        //   3: (0, 0, 6) -- far corner along the X=0 wall
        //   4: (0, 0, 1) -- on the original X=0 wall, 1 m from the corner
        // Vertex 0's only two boundary edges are 0-1 (the remaining Z=0
        // wall run) and 4-0 (the chamfer cut), whose interior angle at 0
        // is 135 degrees by construction (cutting a right angle at equal
        // distance along both original walls always leaves 135 degrees on
        // each new corner).
        let mesh = ErosionMeshInput {
            vertices: vec![
                [1.0, 0.0, 0.0], // 0: the chamfered corner under test
                [6.0, 0.0, 0.0], // 1
                [6.0, 0.0, 6.0], // 2
                [0.0, 0.0, 6.0], // 3
                [0.0, 0.0, 1.0], // 4
            ],
            polygons: vec![[0, 1, 2], [0, 2, 3], [0, 3, 4]],
            protected_edges: Vec::new(),
        };
        let original = mesh.vertices[0];
        let wall_0_1 = mesh.vertices[1];
        let wall_4_0 = mesh.vertices[4];
        let result = erode(&mesh, AGENT_RADIUS);
        let eroded = result.vertices[0];

        assert_eq!(result.pinch_guard_count, 0);
        assert!(result.eroded_count > 0);

        let clearance_to_wall = distance_point_to_line_xz(eroded, original, wall_0_1);
        let clearance_to_chamfer = distance_point_to_line_xz(eroded, wall_4_0, original);
        assert!(
            (clearance_to_wall - AGENT_RADIUS).abs() < 1.0e-4,
            "expected radius clearance from wall 0-1, got {clearance_to_wall}"
        );
        assert!(
            (clearance_to_chamfer - AGENT_RADIUS).abs() < 1.0e-4,
            "expected radius clearance from the chamfer wall 4-0, got {clearance_to_chamfer}"
        );

        // Sanity check the construction really is a 135-degree corner: the
        // miter displacement should be radius / cos(22.5 deg), noticeably
        // less than the 90-degree corner's radius * sqrt(2) but still more
        // than a bare `radius`.
        let dx = eroded[0] - original[0];
        let dz = eroded[2] - original[2];
        let displacement = (dx * dx + dz * dz).sqrt();
        let expected_displacement = AGENT_RADIUS / (std::f32::consts::FRAC_PI_8).cos();
        assert!(
            (displacement - expected_displacement).abs() < 1.0e-4,
            "expected miter displacement radius/cos(22.5 deg) = {expected_displacement}, got {displacement}"
        );
        assert!(displacement > AGENT_RADIUS);
        assert!(displacement < AGENT_RADIUS * std::f32::consts::SQRT_2);
    }

    #[test]
    fn interior_only_vertex_never_moves() {
        // The diagonal A-C is an interior edge shared by both triangles;
        // neither of its endpoints is boundary-only, but they are also
        // corners here so they do move via the *other* boundary edges they
        // touch. To isolate a genuinely interior-only vertex, add a fan of
        // four triangles around a single non-boundary centre vertex.
        let mesh = ErosionMeshInput {
            vertices: vec![
                [1.0, 0.0, 0.0],  // 0: centre-adjacent ring vertex (right)
                [0.0, 0.0, 1.0],  // 1: ring (top)
                [-1.0, 0.0, 0.0], // 2: ring (left)
                [0.0, 0.0, -1.0], // 3: ring (bottom)
                [0.0, 0.0, 0.0],  // 4: centre -- touches only interior edges
            ],
            polygons: vec![[4, 0, 1], [4, 1, 2], [4, 2, 3], [4, 3, 0]],
            protected_edges: Vec::new(),
        };
        let result = erode(&mesh, AGENT_RADIUS);
        assert_eq!(result.vertices[4], mesh.vertices[4], "centre must not move");
    }

    #[test]
    fn zero_or_negative_radius_is_a_no_op() {
        let mesh = corner_room();
        for radius in [0.0, -1.0] {
            let result = erode(&mesh, radius);
            assert_eq!(result.vertices, mesh.vertices);
            assert_eq!(result.eroded_count, 0);
            assert_eq!(result.pinch_guard_count, 0);
        }
    }

    #[test]
    fn erosion_is_deterministic_across_calls() {
        let mesh = corner_room();
        let first = erode(&mesh, AGENT_RADIUS);
        let second = erode(&mesh, AGENT_RADIUS);
        assert_eq!(first, second);
    }

    /// A chain of corridor segments along X between two long walls at
    /// Z=0 and Z=width, each segment split by a diagonal. The segment
    /// seams (shared bottom/top vertices between consecutive segments)
    /// are interior edges; only the long walls and the two open ends are
    /// boundary.
    fn corridor(width: f32, segments: u32) -> ErosionMeshInput {
        let mut vertices = Vec::new();
        let mut polygons = Vec::new();
        for i in 0..=segments {
            let x = i as f32 * 2.0;
            vertices.push([x, 0.0, 0.0]); // bottom row, index 2*i
            vertices.push([x, 0.0, width]); // top row, index 2*i + 1
        }
        for i in 0..segments {
            let bottom_a = 2 * i;
            let top_a = 2 * i + 1;
            let bottom_b = 2 * (i + 1);
            let top_b = 2 * (i + 1) + 1;
            polygons.push([bottom_a, bottom_b, top_b]);
            polygons.push([bottom_a, top_b, top_a]);
        }
        ErosionMeshInput {
            vertices,
            polygons,
            protected_edges: Vec::new(),
        }
    }

    /// Asserts every polygon in `mesh` keeps its original XZ winding sign
    /// and a non-degenerate area after `result`'s erosion -- the
    /// never-inverts-or-degenerates contract `erode()` must hold
    /// unconditionally (phase 2's hard fallback in the module doc comment
    /// is what guarantees this regardless of how adversarial the input
    /// is), shared across every regression test that needs it.
    fn assert_no_polygon_inverted_or_degenerated(mesh: &ErosionMeshInput, result: &ErosionResult) {
        for (poly_index, triangle) in mesh.polygons.iter().enumerate() {
            let original_area = signed_area_xz(
                mesh.vertices[triangle[0] as usize],
                mesh.vertices[triangle[1] as usize],
                mesh.vertices[triangle[2] as usize],
            );
            let eroded_area = signed_area_xz(
                result.vertices[triangle[0] as usize],
                result.vertices[triangle[1] as usize],
                result.vertices[triangle[2] as usize],
            );
            assert!(
                (eroded_area > 0.0) == (original_area > 0.0),
                "polygon {poly_index} inverted after erosion"
            );
            assert!(
                eroded_area.abs() >= MIN_TRIANGLE_AREA,
                "polygon {poly_index} degenerated after erosion"
            );
        }
    }

    #[test]
    fn narrow_corridor_engages_the_pinch_guard_and_never_inverts() {
        // Width 0.3 m, radius 0.35 m: full erosion from both walls (0.35 m
        // each) would overlap by 0.4 m, well past inversion.
        let mesh = corridor(0.3, 3);
        let result = erode(&mesh, AGENT_RADIUS);
        assert!(
            result.pinch_guard_count > 0,
            "narrow corridor must engage the pinch guard"
        );
        // Connectivity is untouched by construction (see module doc
        // comment), so the only thing left to prove is that no polygon
        // inverted or degenerated.
        assert_no_polygon_inverted_or_degenerated(&mesh, &result);
        // Same vertex count/order, indices unchanged: topology preserved.
        assert_eq!(result.vertices.len(), mesh.vertices.len());
        assert_eq!(result.polygon_count, mesh.polygons.len());
    }

    /// Regression test (issue #136 follow-up: real-data failure on Vault
    /// 101 Entrance mesh `0001862b`, polygon 164 -- "concave or has edges
    /// in clockwise order"). A narrow corridor corner (vertex A) shares
    /// its vertex with a THIRD, unrelated thin triangle sticking out to
    /// one side. The corridor's own two triangles are the ones with
    /// boundary edges that push A; the thin triangle has no boundary
    /// edges of its own; A's combined push (corridor walls + the thin
    /// triangle's own two boundary edges) still inverts it at full
    /// erosion strength if unguarded -- exactly the "a neighbouring
    /// triangle inverts from a shared vertex's combined displacement"
    /// class the old per-polygon-isolated check missed. Hand-verified: at
    /// factor 1.0 the thin triangle's signed area flips from +0.05 to
    /// about -0.202.
    #[test]
    fn shared_vertex_push_never_inverts_a_neighboring_triangle() {
        let mesh = ErosionMeshInput {
            vertices: vec![
                [0.0, 0.0, 0.0],    // 0: A -- shared corner
                [2.0, 0.0, 0.0],    // 1: B -- corridor bottom-right
                [2.0, 0.0, 0.1],    // 2: C -- corridor top-right
                [0.0, 0.0, 0.1],    // 3: D -- corridor top-left
                [-1.0, 0.0, 0.05],  // 4: E -- thin triangle tip 1
                [-1.0, 0.0, -0.05], // 5: F -- thin triangle tip 2
            ],
            polygons: vec![
                [0, 1, 2], // corridor triangle 1 (A,B,C)
                [0, 2, 3], // corridor triangle 2 (A,C,D)
                [0, 4, 5], // unrelated thin triangle sharing only vertex A
            ],
            protected_edges: Vec::new(),
        };
        let result = erode(&mesh, AGENT_RADIUS);
        assert!(
            result.pinch_guard_count > 0,
            "this construction must engage the pinch guard: {result:?}"
        );
        assert_no_polygon_inverted_or_degenerated(&mesh, &result);
        assert_eq!(result.vertices.len(), mesh.vertices.len());
        assert_eq!(result.polygon_count, mesh.polygons.len());
    }

    #[test]
    fn wide_corridor_erodes_fully_without_the_pinch_guard() {
        // Width 3.0 m is far wider than 2 * radius: both walls should
        // erode at full strength with no fallback needed.
        let mesh = corridor(3.0, 2);
        let result = erode(&mesh, AGENT_RADIUS);
        assert_eq!(result.pinch_guard_count, 0);
        assert!(result.eroded_count > 0);
        for (index, vertex) in mesh.vertices.iter().enumerate() {
            let eroded = result.vertices[index];
            let is_bottom = vertex[2] == 0.0;
            let is_top = vertex[2] == 3.0;
            if is_bottom {
                assert!(
                    eroded[2] > vertex[2],
                    "bottom-wall vertex {index} must move inward"
                );
            }
            if is_top {
                assert!(
                    eroded[2] < vertex[2],
                    "top-wall vertex {index} must move inward"
                );
            }
        }
    }

    #[test]
    fn protected_seam_edge_vertices_stay_fixed_while_wall_vertices_erode() {
        // Regression (issue #136 follow-up, Franklin Metro 02 mesh
        // 0001a273): a seam/portal edge to another mesh looks *exactly*
        // like a wall from this single mesh's own topology (an edge
        // referenced by exactly one polygon) -- without protection,
        // erosion pulled both sides of a real cross-mesh merge seam
        // inward independently, opening a ~0.7 m gap where a generated
        // animation link used to connect the two islands. D-A here
        // stands in for that seam (as if this mesh continued past x=0
        // into a neighbour); A-B, B-C, C-D are real walls that must
        // still erode normally.
        let mesh = ErosionMeshInput {
            vertices: vec![
                [0.0, 0.0, 0.0], // 0: A -- on the protected seam
                [4.0, 0.0, 0.0], // 1: B -- real wall corner
                [4.0, 0.0, 4.0], // 2: C -- real wall corner
                [0.0, 0.0, 4.0], // 3: D -- on the protected seam
            ],
            polygons: vec![[0, 1, 2], [0, 2, 3]],
            protected_edges: vec![(3, 0)],
        };
        let result = erode(&mesh, AGENT_RADIUS);
        assert_eq!(
            result.vertices[0], mesh.vertices[0],
            "seam vertex A must not move"
        );
        assert_eq!(
            result.vertices[3], mesh.vertices[3],
            "seam vertex D must not move"
        );
        assert_ne!(
            result.vertices[1], mesh.vertices[1],
            "wall vertex B must still erode"
        );
        assert_ne!(
            result.vertices[2], mesh.vertices[2],
            "wall vertex C must still erode"
        );
        assert_eq!(result.protected_count, 2);
        assert_no_polygon_inverted_or_degenerated(&mesh, &result);
    }
}
