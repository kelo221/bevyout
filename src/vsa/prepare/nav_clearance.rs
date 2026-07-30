//! Pure collision-derived navmesh validation + clearance (issue #153, M4
//! wave 10; sub-triangle re-triangulation added by issue #171, M4 wave 11).
//! Given one prepared nav mesh's walkable triangle soup plus this cell's
//! cooked static collision as world-space triangles, it validates the authored
//! FO3 NAVM against the cooked collision and removes the area the agent
//! capsule cannot legitimately stand on or fit through.
//!
//! 1. **Local re-triangulation against the walkability boundary (F171.1 /
//!    F171.2).** A single pointwise predicate decides walkability anywhere in
//!    the cell: a point is walkable when cooked collision supports it within
//!    the agent's step height (F153.1's support test, F171.2's void boundary)
//!    *and* no wall-like collider the agent cannot step over rises into the
//!    agent capsule within the agent radius of it (F153.2's obstruction test,
//!    F171.1's footprint expanded by the agent radius). `nav_clip` then
//!    re-triangulates every authored polygon conformally against that
//!    predicate, so the surviving edges lie on the obstruction/void boundary
//!    itself rather than on authored triangle edges. This is what wave 10's
//!    whole-triangle tests provably could not express: a collider protruding
//!    into a triangle's *opening* while its centroid sits clear, and a narrow
//!    void strip under an otherwise supported triangle.
//! 2. **Sub-diameter disconnection (F153.3).** Drops a walkable triangle only
//!    when the *authored passage width* through it -- near-wall distance +
//!    far-wall distance, measured across the triangle against the walkable
//!    boundary before any obstruction clip -- is below `2 * radius`
//!    everywhere in it: a genuinely sub-diameter throat the agent capsule
//!    cannot pass. Crucially it measures the passage, not per-triangle
//!    clearance, so a wall-adjacent triangle in a wide corridor keeps the
//!    full passage width (`0 + width`) and is never dropped -- the walkable
//!    region stays connected instead of fragmenting into an eroded center
//!    band. It never moves a vertex, so a ~1 m doorway with 90-degree jambs
//!    stays connected (its passage is `1.0 m > 2 * radius`) instead of
//!    collapsing under the reverted wave-6 miter's two-sided offset
//!    arithmetic, while a genuine 0.5 m gap disconnects. Seam/door protected
//!    triangles are exempt so authored doorways stay traversable. It measures
//!    against the *pre-obstruction* boundary on purpose: the clip has already
//!    accounted for the agent radius once, and charging it twice would
//!    collapse every authored doorway.
//! 3. **Connectivity-preserving finalization.** Unchanged from wave 10, and
//!    now operating on the clipped sub-polygons: a drop that strands a large
//!    or seam/door-bearing component is un-dropped. This is why the clip emits
//!    *both* sides of every cut (the unwalkable side flagged, not discarded) --
//!    un-dropping is a flag flip that can never break the re-triangulation's
//!    conformity.
//!
//! Nothing here is keyed on a placement, a cell, or a coordinate: the rule is
//! the same geometry applied to every polygon and every static collider.
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

use super::nav_clip::{ClipParams, refine_and_clip};

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
/// How far (metres, XZ) from a sample a cooked collision surface may sit and
/// still support it. Cooked static collision is assembled from independently
/// placed meshes that abut without welding, so hairline seams and T-junctions
/// between two floor placements are void by the letter of the geometry. This
/// dilates the supported region enough to close those seams while leaving any
/// genuine void (an overhang, a missing floor strip) intact -- it is a
/// numerical-robustness tolerance, not a tuned per-cell value.
const SUPPORT_SEAM_TOLERANCE: f32 = 0.15;
/// Refinement rounds the re-triangulation may take before it stops halving
/// edges, regardless of how much predicate detail remains. Bounds prepare cost
/// on pathological geometry.
const MAX_REFINEMENT_ROUNDS: usize = 4;
/// Target XZ cell size (metres) of the collision broadphase grid. Purely a
/// query-cost tuning knob: results are identical at any cell size.
const COLLISION_GRID_CELL: f32 = 1.0;
/// Upper bound on grid cells per axis, so an unusually large collision extent
/// coarsens the grid instead of allocating without limit.
const COLLISION_GRID_MAX_DIM: usize = 512;
/// A collision triangle covering more grid cells than this is held in a
/// shared oversized list checked by every query, instead of being written into
/// every cell it spans.
const COLLISION_GRID_OVERSIZED_CELLS: usize = 1024;

/// Why a triangle was dropped (`None` = walkable). Tracked so the
/// connectivity-preserving finalization can un-drop a triangle stranding a
/// large region, and so counts reflect only committed drops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DropReason {
    Unsupported,
    Obstructed,
    SubDiameter,
    /// F171.5: the polygon is not a geometrically valid navigation polygon
    /// (repeated or out-of-range vertices, too ill-conditioned to have a
    /// reliable winding, or wound against the rest of its mesh). Never
    /// restored by the connectivity guard -- shipping one invalidates the
    /// whole mesh at runtime.
    InvalidGeometry,
}

impl DropReason {
    /// Stable one-word label for prepare diagnostics.
    pub(crate) fn label(self) -> &'static str {
        match self {
            DropReason::Unsupported => "unsupported",
            DropReason::Obstructed => "obstructed",
            DropReason::SubDiameter => "sub-diameter",
            DropReason::InvalidGeometry => "invalid-geometry",
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

/// Result of one clearance pass: the re-triangulated mesh (F171.1/F171.2) plus
/// a per-output-polygon walkable flag (`false` = clipped away for lacking
/// collision support, clipped away as inside an obstruction's agent-radius
/// footprint, or dropped for a sub-diameter authored passage) and
/// deterministic diagnostic counters, including the connectivity structure of
/// the surviving walkable set.
///
/// Authored vertices keep their original indices and authored polygons that
/// the clip left whole keep their original polygon index, so caller-side data
/// keyed by either (door/merge triangle indices, merge edge vertex indices)
/// stays valid. New vertices and additional sub-polygons are appended.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NavClearanceResult {
    /// Authored vertices followed by every refinement midpoint and boundary
    /// crossing the re-triangulation introduced.
    pub(crate) vertices: Vec<[f32; 3]>,
    /// The re-triangulated polygon cover, aligned with `walkable`/`reasons`.
    pub(crate) polygons: Vec<[u32; 3]>,
    /// For each output polygon, the authored polygon index it came from.
    pub(crate) sources: Vec<u32>,
    /// Authored polygons the re-triangulation split into more than one piece.
    pub(crate) clipped_polygons: usize,
    /// Vertices the re-triangulation appended.
    pub(crate) added_vertices: usize,
    /// Sliver sub-triangles the re-triangulation removed by welding (F171.5).
    pub(crate) degenerate_discarded: usize,
    /// Vertex welds the degenerate-collapse phase performed.
    pub(crate) collapsed_welds: usize,
    /// Walkable polygons the geometry gate rejected as invalid navigation
    /// polygons. Must be zero: a single one invalidates the whole mesh at
    /// runtime, so any non-zero count is reported as an error by the caller.
    pub(crate) invalid_geometry: usize,
    /// Walkability-predicate evaluations, reported so the pass's cost is
    /// visible in `prepare` output.
    pub(crate) predicate_evaluations: usize,
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
    /// Authored polygons in this mesh, before re-triangulation.
    pub(crate) authored_polygon_count: usize,
    /// Authored polygons with at least one walkable piece in the main
    /// (largest) surviving component. Sub-triangle clipping makes the raw
    /// polygon-level component share incomparable with wave 10's (a clipped
    /// mesh has ~40x the polygons, all much smaller), so this reports the same
    /// health question at the authored granularity: what share of the mesh the
    /// runtime can actually reach from its dominant region.
    pub(crate) authored_in_main_component: usize,
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
///
/// Issue #189 feature 4: the single shared containment primitive
/// (`bevyout_core::geometry`), reached here through a thin alias so this
/// module's many call sites keep their local name. Previously one of four
/// hand-rolled copies with three different tolerances; see that module for the
/// consolidated epsilon rationale.
fn barycentric_xz(px: f32, pz: f32, a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> Option<[f32; 3]> {
    bevyout_core::geometry::barycentric_xz(px, pz, a, b, c)
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

// ---------------------------------------------------------------------
// Collision broadphase (issue #171): the re-triangulation evaluates the
// walkability predicate orders of magnitude more often than wave 10's
// per-triangle tests did, so the linear scans those used are replaced by a
// uniform XZ grid. Purely a cost structure -- it changes no verdict.
// ---------------------------------------------------------------------

/// Uniform XZ grid over one cell's cooked static collision, split into the
/// surfaces that can *support* a point (anything with a non-degenerate XZ
/// projection) and the wall-like surfaces that can *obstruct* one (binned with
/// their footprint already expanded by the agent radius, so an obstruction
/// query only ever reads the sample's own cell).
struct CollisionIndex<'a> {
    triangles: &'a [CollisionTriangle],
    aabbs: Vec<TriAabb>,
    origin: [f32; 2],
    cell: f32,
    dims: [usize; 2],
    support_cells: Vec<Vec<u32>>,
    support_oversized: Vec<u32>,
    wall_cells: Vec<Vec<u32>>,
    wall_oversized: Vec<u32>,
}

impl<'a> CollisionIndex<'a> {
    fn build(triangles: &'a [CollisionTriangle], params: NavClearanceParams) -> Self {
        let aabbs: Vec<TriAabb> = triangles.iter().map(tri_aabb).collect();
        let mut min = [f32::INFINITY; 2];
        let mut max = [f32::NEG_INFINITY; 2];
        for aabb in &aabbs {
            min[0] = min[0].min(aabb.min[0]);
            min[1] = min[1].min(aabb.min[2]);
            max[0] = max[0].max(aabb.max[0]);
            max[1] = max[1].max(aabb.max[2]);
        }
        if !min[0].is_finite() {
            min = [0.0, 0.0];
            max = [0.0, 0.0];
        }
        let span = [
            (max[0] - min[0]).max(0.0) + 1.0,
            (max[1] - min[1]).max(0.0) + 1.0,
        ];
        let cell = COLLISION_GRID_CELL
            .max(span[0] / COLLISION_GRID_MAX_DIM as f32)
            .max(span[1] / COLLISION_GRID_MAX_DIM as f32);
        let dims = [
            ((span[0] / cell).ceil() as usize).max(1),
            ((span[1] / cell).ceil() as usize).max(1),
        ];
        let mut index = Self {
            triangles,
            aabbs,
            origin: min,
            cell,
            dims,
            support_cells: vec![Vec::new(); dims[0] * dims[1]],
            support_oversized: Vec::new(),
            wall_cells: vec![Vec::new(); dims[0] * dims[1]],
            wall_oversized: Vec::new(),
        };
        for (i, triangle) in triangles.iter().enumerate() {
            let aabb = index.aabbs[i];
            let [a, b, c] = triangle.vertices;
            // Support candidate: any surface with a non-degenerate XZ
            // projection (a vertical wall projects to a line and can never
            // support, so it is not binned here).
            if ((b[0] - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (b[2] - a[2])).abs() > 1.0e-9 {
                index.insert(i as u32, aabb, 0.0, true);
            }
            // Obstruction candidate: wall-like normal, binned with the
            // agent-radius expansion baked into its footprint.
            if let Some(normal) = triangle_normal(triangle)
                && normal[1].abs() < WALL_NORMAL_Y_MAX
            {
                index.insert(i as u32, aabb, params.agent_radius, false);
            }
        }
        index
    }

    fn cell_range(&self, aabb: TriAabb, pad: f32) -> ([usize; 2], [usize; 2]) {
        let to_cell = |value: f32, origin: f32, dim: usize| {
            (((value - origin) / self.cell).floor().max(0.0) as usize).min(dim - 1)
        };
        (
            [
                to_cell(aabb.min[0] - pad, self.origin[0], self.dims[0]),
                to_cell(aabb.min[2] - pad, self.origin[1], self.dims[1]),
            ],
            [
                to_cell(aabb.max[0] + pad, self.origin[0], self.dims[0]),
                to_cell(aabb.max[2] + pad, self.origin[1], self.dims[1]),
            ],
        )
    }

    fn insert(&mut self, triangle: u32, aabb: TriAabb, pad: f32, support: bool) {
        let (lo, hi) = self.cell_range(aabb, pad);
        let covered = (hi[0] - lo[0] + 1) * (hi[1] - lo[1] + 1);
        if covered > COLLISION_GRID_OVERSIZED_CELLS {
            if support {
                self.support_oversized.push(triangle);
            } else {
                self.wall_oversized.push(triangle);
            }
            return;
        }
        for gz in lo[1]..=hi[1] {
            for gx in lo[0]..=hi[0] {
                let slot = gz * self.dims[0] + gx;
                if support {
                    self.support_cells[slot].push(triangle);
                } else {
                    self.wall_cells[slot].push(triangle);
                }
            }
        }
    }

    /// Highest collision surface height at `(x, z)` that is at or below
    /// `ceiling`, or `None` when nothing supports that spot within reach.
    /// This is the *footing* an obstruction candidate stands on.
    fn highest_support_at(&self, x: f32, z: f32, ceiling: f32) -> Option<f32> {
        let (cell, oversized) = self.candidates(x, z, true);
        let mut best: Option<f32> = None;
        for &i in cell.iter().chain(oversized) {
            let aabb = self.aabbs[i as usize];
            if x < aabb.min[0] || x > aabb.max[0] || z < aabb.min[2] || z > aabb.max[2] {
                continue;
            }
            if aabb.min[1] > ceiling {
                continue;
            }
            let [a, b, c] = self.triangles[i as usize].vertices;
            let Some(w) = barycentric_xz(x, z, a, b, c) else {
                continue;
            };
            let height = w[0] * a[1] + w[1] * b[1] + w[2] * c[1];
            if height <= ceiling {
                best = Some(best.map_or(height, |current: f32| current.max(height)));
            }
        }
        best
    }

    /// Candidate triangle indices for a point query, as the point's own grid
    /// cell plus the oversized list.
    fn candidates(&self, x: f32, z: f32, support: bool) -> (&[u32], &[u32]) {
        let (cells, oversized) = if support {
            (&self.support_cells, &self.support_oversized)
        } else {
            (&self.wall_cells, &self.wall_oversized)
        };
        let gx = ((x - self.origin[0]) / self.cell).floor();
        let gz = ((z - self.origin[1]) / self.cell).floor();
        if gx < 0.0 || gz < 0.0 {
            return (&[], oversized);
        }
        let (gx, gz) = (gx as usize, gz as usize);
        if gx >= self.dims[0] || gz >= self.dims[1] {
            return (&[], oversized);
        }
        (&cells[gz * self.dims[0] + gx], oversized)
    }
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
            vertices: mesh.vertices.clone(),
            polygons: mesh.polygons.clone(),
            sources: (0..polygon_count as u32).collect(),
            clipped_polygons: 0,
            added_vertices: 0,
            degenerate_discarded: 0,
            collapsed_welds: 0,
            invalid_geometry: 0,
            predicate_evaluations: 0,
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
            authored_polygon_count: polygon_count,
            authored_in_main_component: polygon_count,
            reasons: vec![None; polygon_count],
            nonmain_components: Vec::new(),
        };
    }

    let authored_polygon_count = polygon_count;
    let protected: BTreeSet<u32> = protected_triangle_indices(mesh);
    let protected_count = protected.len();

    // F171.1/F171.2: re-triangulate the authored mesh against the pointwise
    // walkability predicate, so the surviving edges lie on the
    // obstruction/void boundary rather than on authored triangle edges. With
    // no cooked collision to judge against there is nothing to clip: the
    // authored mesh passes through untouched (never remove walkable area on
    // no evidence).
    let clipped = clip_against_collision(mesh, collision, &protected, params);
    let clipped_mesh = NavClearanceMeshInput {
        vertices: clipped.vertices,
        polygons: clipped.polygons,
        protected_edges: mesh.protected_edges.clone(),
    };
    let mesh = &clipped_mesh;
    let polygon_count = mesh.polygons.len();
    let protected: BTreeSet<u32> = protected_triangle_indices(mesh);

    // Per-polygon drop reason (`None` = walkable). Tracked so the
    // connectivity-preserving finalization below can un-drop a piece that
    // turns out to strand a large reachable region, and so the final counts
    // reflect only committed drops.
    let mut reason: Vec<Option<DropReason>> = clipped.reasons;

    // F153.3: sub-diameter disconnection, measured against the *authored*
    // passage. A polygon is dropped only when the passage width through it
    // (near-wall distance + far-wall distance, measured across it) is below
    // `2 * radius` everywhere in it -- a genuinely sub-diameter throat.
    // Wall-adjacent polygons in a wide passage keep the full passage width
    // (near-wall 0 + far-wall = width) and are never dropped, so the walkable
    // region stays connected (no erosion-style fragmentation). The boundary
    // deliberately excludes the obstruction clip's own cuts: that clip has
    // already charged the agent radius once, and charging it a second time
    // here would collapse every authored doorway. Void removal *is* included,
    // because it changes where the floor genuinely ends. Protected polygons
    // are exempt so authored doorways/seams stay traversable.
    {
        let authored_extent: Vec<bool> = reason
            .iter()
            .map(|r| !matches!(r, Some(DropReason::Unsupported)))
            .collect();
        let boundary = boundary_segments(mesh, &authored_extent);
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
    // gated solely by an unsupported/obstructed/sub-diameter polygon, so a
    // large stranded component means a drop misread the geometry. Any drop
    // adjacent to a large non-main component is un-dropped, iterated until only
    // small islands remain disconnected. Determinism: fixed index order.
    restore_large_strands(mesh, &mut reason, &protected);

    // F171.5 geometry gate. Runs last and is never undone: the connectivity
    // guard may not rescue a polygon that is not a valid navigation polygon,
    // because landmass rejects an entire mesh over a single bad one -- a cell
    // then has no navigation at all, which prepare-side connectivity metrics
    // cannot see (they measure the graph this pass built, not the graph the
    // runtime can accept). This gate is what closes that hole.
    reject_invalid_geometry(mesh, &mut reason);

    let mut walkable = vec![true; polygon_count];
    let mut removed_unsupported = 0usize;
    let mut cut_obstructed = 0usize;
    let mut dropped_unfit = 0usize;
    let mut invalid_geometry = 0usize;
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
            Some(DropReason::InvalidGeometry) => {
                walkable[index] = false;
                invalid_geometry += 1;
            }
        }
    }

    let walkable_count = walkable.iter().filter(|&&w| w).count();
    let (roots, sizes) = label_components(mesh, &walkable);
    let component_count = sizes.len();
    let largest_component = sizes.values().copied().max().unwrap_or(0);
    let nonmain_components = nonmain_component_report(mesh, &roots, &sizes);
    let authored_in_main_component = match main_component_root(&sizes) {
        Some(main) => {
            let mut reached: BTreeSet<u32> = BTreeSet::new();
            for (index, &source) in clipped.sources.iter().enumerate() {
                if walkable.get(index).copied().unwrap_or(false) && roots[index] == main {
                    reached.insert(source);
                }
            }
            reached.len()
        }
        None => 0,
    };

    NavClearanceResult {
        vertices: clipped_mesh.vertices,
        polygons: clipped_mesh.polygons,
        sources: clipped.sources,
        clipped_polygons: clipped.clipped_polygons,
        added_vertices: clipped.added_vertices,
        degenerate_discarded: clipped.degenerate_discarded,
        collapsed_welds: clipped.collapsed_welds,
        invalid_geometry,
        predicate_evaluations: clipped.predicate_evaluations,
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
        authored_polygon_count,
        authored_in_main_component,
        reasons: reason,
        nonmain_components,
    }
}

/// Why the walkability predicate accepted or rejected one world point, for the
/// `prepare`-time probe (issue #171 acceptance): the pointwise question
/// "why is this spot not walkable?" answered term by term, so an
/// over-blocking term can be identified from `prepare` output alone rather
/// than inferred from a severed route.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct NavClearanceProbe {
    pub(crate) point: [f32; 3],
    /// Collision support directly under the point, before the seam tolerance.
    pub(crate) supported_exact: bool,
    /// Collision support including [`SUPPORT_SEAM_TOLERANCE`] dilation.
    pub(crate) supported: bool,
    pub(crate) obstructed: bool,
    /// XZ distance to the nearest non-step-overable wall-like collider
    /// triangle that reaches into the agent band here (`f32::MAX` when none
    /// does). Below the agent radius means this point is inside an expanded
    /// obstruction footprint.
    pub(crate) nearest_wall_distance: f32,
    /// That triangle's world-space vertices, so the offending collider can be
    /// identified against the prepared placement list.
    pub(crate) nearest_wall: Option<[[f32; 3]; 3]>,
}

/// Evaluates the walkability predicate's individual terms at each of `points`.
/// Debug-only: the clearance pass itself never calls this.
pub(crate) fn probe_points(
    collision: &[CollisionTriangle],
    params: NavClearanceParams,
    points: &[[f32; 3]],
) -> Vec<NavClearanceProbe> {
    let index = CollisionIndex::build(collision, params);
    points
        .iter()
        .map(|&point| {
            let mut nearest_wall_distance = f32::MAX;
            let mut nearest_wall = None;
            let step_top = point[1] + params.step_height;
            let band_high = point[1] + params.agent_height;
            let (cell, oversized) = index.candidates(point[0], point[2], false);
            for &i in cell.iter().chain(oversized) {
                let aabb = index.aabbs[i as usize];
                if aabb.max[1] <= step_top || aabb.min[1] >= band_high {
                    continue;
                }
                let triangle = &index.triangles[i as usize];
                let distance = point_triangle_dist_xz(point[0], point[2], triangle);
                if distance < nearest_wall_distance {
                    nearest_wall_distance = distance;
                    nearest_wall = Some(triangle.vertices);
                }
            }
            NavClearanceProbe {
                point,
                supported_exact: is_supported_exact(point, &index, params),
                supported: is_supported(point, &index, params),
                obstructed: is_obstructed(point, &index, params),
                nearest_wall_distance,
                nearest_wall,
            }
        })
        .collect()
}

/// Smallest XZ area (square metres) a walkable polygon may have and still
/// carry a reliable winding. Below this, landmass reports it as "concave or
/// has edges in clockwise order" and discards the mesh it belongs to.
/// Deliberately larger than `nav_clip`'s collapse threshold so the gate can
/// never pass something the collapse phase should have removed.
const MIN_POLYGON_AREA: f32 = 1.0e-7;

/// F171.5: marks every still-walkable polygon that is not a geometrically
/// valid navigation polygon, so it can never reach the runtime.
///
/// Three ways a polygon fails: a repeated or out-of-range vertex index; too
/// little area to have a reliable winding; or a winding opposite to the rest
/// of its mesh. The last matters because the runtime validates a mesh under
/// one global winding (retrying reversed as a whole), so a single inverted
/// polygon can never be accommodated -- it invalidates every other polygon
/// with it.
fn reject_invalid_geometry(mesh: &NavClearanceMeshInput, reason: &mut [Option<DropReason>]) {
    let signed_area = |tri: [u32; 3]| -> Option<f32> {
        let (Some(&a), Some(&b), Some(&c)) = (
            mesh.vertices.get(tri[0] as usize),
            mesh.vertices.get(tri[1] as usize),
            mesh.vertices.get(tri[2] as usize),
        ) else {
            return None;
        };
        Some(((b[0] - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (b[2] - a[2])) * 0.5)
    };

    // The mesh's own winding convention, taken as the majority over the
    // polygons that have enough area to express one.
    let (mut positive, mut negative) = (0usize, 0usize);
    for (index, tri) in mesh.polygons.iter().enumerate() {
        if reason[index].is_some() {
            continue;
        }
        match signed_area(*tri) {
            Some(area) if area >= MIN_POLYGON_AREA => positive += 1,
            Some(area) if area <= -MIN_POLYGON_AREA => negative += 1,
            _ => {}
        }
    }
    let expect_positive = positive >= negative;

    for (index, tri) in mesh.polygons.iter().enumerate() {
        if reason[index].is_some() {
            continue;
        }
        let distinct = tri[0] != tri[1] && tri[1] != tri[2] && tri[2] != tri[0];
        let valid = distinct
            && match signed_area(*tri) {
                Some(area) => area.abs() >= MIN_POLYGON_AREA && (area > 0.0) == expect_positive,
                None => false,
            };
        if !valid {
            reason[index] = Some(DropReason::InvalidGeometry);
        }
    }
}

/// The re-triangulated mesh plus the clip's own per-polygon verdict.
struct ClippedMesh {
    vertices: Vec<[f32; 3]>,
    polygons: Vec<[u32; 3]>,
    sources: Vec<u32>,
    reasons: Vec<Option<DropReason>>,
    clipped_polygons: usize,
    added_vertices: usize,
    degenerate_discarded: usize,
    collapsed_welds: usize,
    predicate_evaluations: usize,
}

/// F171.1/F171.2: re-triangulates `mesh` conformally against the pointwise
/// walkability predicate -- supported by cooked collision within the agent's
/// step height, and outside every non-step-overable wall-like collider's
/// footprint expanded by the agent radius -- and labels each resulting
/// sub-polygon with the reason it failed, if it did.
///
/// The output polygon list is laid out so that authored polygon `i` keeps
/// index `i` (its first piece occupies that slot); any further pieces are
/// appended. Authored vertices likewise keep their indices. That is what lets
/// the caller rewrite a prepared mesh in place without invalidating anything
/// keyed by a triangle or vertex index.
fn clip_against_collision(
    mesh: &NavClearanceMeshInput,
    collision: &[CollisionTriangle],
    protected: &BTreeSet<u32>,
    params: NavClearanceParams,
) -> ClippedMesh {
    let polygon_count = mesh.polygons.len();
    if collision.is_empty() {
        return ClippedMesh {
            vertices: mesh.vertices.clone(),
            polygons: mesh.polygons.clone(),
            sources: (0..polygon_count as u32).collect(),
            reasons: vec![None; polygon_count],
            clipped_polygons: 0,
            added_vertices: 0,
            degenerate_discarded: 0,
            collapsed_welds: 0,
            predicate_evaluations: 0,
        };
    }

    let index = CollisionIndex::build(collision, params);
    let footprints = protected_footprints(mesh, protected);
    let supported = |point: [f32; 3]| is_supported(point, &index, params);
    let obstructed = |point: [f32; 3]| is_obstructed(point, &index, params);
    let predicate = |point: [f32; 3]| {
        in_protected_footprint(point, &footprints) || (supported(point) && !obstructed(point))
    };

    // Protected (seam/door) polygons' edges are locked: they are never split
    // and never carry a boundary crossing, so those polygons survive
    // byte-identical and the door/merge triangle indices keyed to them stay
    // valid.
    let mut locked: BTreeSet<(u32, u32)> = BTreeSet::new();
    for &polygon in protected {
        if let Some(tri) = mesh.polygons.get(polygon as usize) {
            locked.insert(edge_key(tri[0], tri[1]));
            locked.insert(edge_key(tri[1], tri[2]));
            locked.insert(edge_key(tri[2], tri[0]));
        }
    }

    let output = refine_and_clip(
        &mesh.vertices,
        &mesh.polygons,
        &locked,
        &predicate,
        ClipParams {
            resolution: params.agent_radius,
            max_refinement_rounds: MAX_REFINEMENT_ROUNDS,
            ..ClipParams::default()
        },
    );

    // Lay the pieces out so authored polygon `i` keeps slot `i`.
    let mut slots: Vec<Option<([u32; 3], u32, bool)>> = vec![None; polygon_count];
    let mut extra: Vec<([u32; 3], u32, bool)> = Vec::new();
    let mut pieces_per_source = vec![0usize; polygon_count];
    for triangle in &output.triangles {
        let source = triangle.source as usize;
        if source < polygon_count {
            pieces_per_source[source] += 1;
        }
        let entry = (triangle.vertex_indices, triangle.source, triangle.inside);
        match slots.get_mut(source) {
            Some(slot) if slot.is_none() => *slot = Some(entry),
            _ => extra.push(entry),
        }
    }

    let mut polygons = Vec::with_capacity(output.triangles.len());
    let mut sources = Vec::with_capacity(output.triangles.len());
    let mut inside_flags = Vec::with_capacity(output.triangles.len());
    for (slot_index, slot) in slots.into_iter().enumerate() {
        match slot {
            Some((tri, source, inside)) => {
                polygons.push(tri);
                sources.push(source);
                inside_flags.push(inside);
            }
            // Every piece of this authored polygon was a sliver; keep the slot
            // occupied by a degenerate placeholder marked unwalkable so the
            // index layout (and anything keyed by it) stays stable.
            None => {
                let tri = mesh.polygons[slot_index];
                polygons.push(tri);
                sources.push(slot_index as u32);
                inside_flags.push(false);
            }
        }
    }
    for (tri, source, inside) in extra {
        polygons.push(tri);
        sources.push(source);
        inside_flags.push(inside);
    }

    // Attribute each clipped-away piece to the predicate term that rejected
    // it, so `prepare`'s diagnostics keep distinguishing a void from an
    // obstruction.
    let reasons: Vec<Option<DropReason>> = polygons
        .iter()
        .zip(&inside_flags)
        .map(|(tri, &inside)| {
            if inside {
                return None;
            }
            let (Some(&a), Some(&b), Some(&c)) = (
                output.vertices.get(tri[0] as usize),
                output.vertices.get(tri[1] as usize),
                output.vertices.get(tri[2] as usize),
            ) else {
                return Some(DropReason::Unsupported);
            };
            let centroid = [
                (a[0] + b[0] + c[0]) / 3.0,
                (a[1] + b[1] + c[1]) / 3.0,
                (a[2] + b[2] + c[2]) / 3.0,
            ];
            if supported(centroid) {
                Some(DropReason::Obstructed)
            } else {
                Some(DropReason::Unsupported)
            }
        })
        .collect();

    let clipped_polygons = pieces_per_source.iter().filter(|&&n| n > 1).count();
    let added_vertices = output.vertices.len() - mesh.vertices.len();
    ClippedMesh {
        vertices: output.vertices,
        polygons,
        sources,
        reasons,
        clipped_polygons,
        added_vertices,
        degenerate_discarded: output.degenerate_discarded,
        collapsed_welds: output.collapsed_welds,
        predicate_evaluations: output.predicate_evaluations,
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

/// F153.1: is there a cooked collision surface under `point` within the
/// agent's step height (and a small margin above)?
fn is_supported_exact(point: [f32; 3], index: &CollisionIndex, params: NavClearanceParams) -> bool {
    let low = point[1] - params.step_height;
    let high = point[1] + params.support_above_margin;
    let (cell, oversized) = index.candidates(point[0], point[2], true);
    for &i in cell.iter().chain(oversized) {
        let aabb = index.aabbs[i as usize];
        if point[0] < aabb.min[0] || point[0] > aabb.max[0] {
            continue;
        }
        if point[2] < aabb.min[2] || point[2] > aabb.max[2] {
            continue;
        }
        if aabb.max[1] < low || aabb.min[1] > high {
            continue;
        }
        let [a, b, c] = index.triangles[i as usize].vertices;
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

/// F153.1/F171.2: is `point` supported, allowing for the hairline seams and
/// T-junctions that cooked collision assembled from independently placed
/// meshes carries? The supported region is dilated by
/// [`SUPPORT_SEAM_TOLERANCE`] -- sampled as the point itself plus its four
/// axis-aligned offsets -- so a seam between two abutting floor placements
/// never reads as a void, while a genuine void strip (wider than the
/// tolerance) still does.
fn is_supported(point: [f32; 3], index: &CollisionIndex, params: NavClearanceParams) -> bool {
    if is_supported_exact(point, index, params) {
        return true;
    }
    let t = SUPPORT_SEAM_TOLERANCE;
    [[t, 0.0], [-t, 0.0], [0.0, t], [0.0, -t]]
        .iter()
        .any(|offset| {
            is_supported_exact(
                [point[0] + offset[0], point[1], point[2] + offset[1]],
                index,
                params,
            )
        })
}

/// F153.2: does a wall-like collider the agent *cannot step over* rise into
/// the agent capsule over `point`, within the agent radius (XZ)? A collider
/// only obstructs if it occupies some height in `(floor + step_height, floor +
/// agent_height)` -- one entirely below `floor + step_height` is stepped over
/// (stair risers, low ledges), one entirely above `floor + agent_height` is
/// walked under (overheads). Floor/ceiling-like triangles never obstruct.
fn is_obstructed(point: [f32; 3], index: &CollisionIndex, params: NavClearanceParams) -> bool {
    let radius = params.agent_radius;
    let step_top = point[1] + params.step_height;
    let band_high = point[1] + params.agent_height;
    let (cell, oversized) = index.candidates(point[0], point[2], false);
    for &i in cell.iter().chain(oversized) {
        let aabb = index.aabbs[i as usize];
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
        let triangle = &index.triangles[i as usize];
        if point_triangle_dist_xz(point[0], point[2], triangle) >= radius {
            continue;
        }
        // Judge the collider against the walkable surface at *its own*
        // footprint, not at the query point. A step is only an obstruction if
        // the agent cannot stand on whatever is there: measured from the query
        // point instead, every riser more than a step above the tread the
        // agent is on reads as a wall, and since real stair runs are often
        // shorter than the agent radius, that classifies whole staircases as
        // walls and strands everything they serve. Measured from the riser's
        // own footing, the next tread is one step up and the staircase stays
        // walkable, while a wall still rises far above the floor it stands on
        // and a crate too tall to mount still obstructs (its own top is out of
        // reach, so its footing is the floor).
        let [a, b, c] = triangle.vertices;
        let footing = index
            .highest_support_at(
                (a[0] + b[0] + c[0]) / 3.0,
                (a[2] + b[2] + c[2]) / 3.0,
                step_top,
            )
            .unwrap_or(point[1]);
        if aabb.max[1] > footing + params.step_height {
            return true;
        }
    }
    false
}

/// The protected (seam/door) polygons' own XZ footprints. The walkability
/// predicate reports every point inside one of these as walkable, so an
/// authored doorway or inter-mesh seam is never clipped: its polygon survives
/// byte-identical (keeping the door/merge triangle index valid) and, because
/// both endpoints of every protected edge are then walkable, no boundary
/// crossing is ever needed on a protected edge -- which is what lets
/// `nav_clip` lock those edges without leaving a T-junction.
fn protected_footprints(
    mesh: &NavClearanceMeshInput,
    protected: &BTreeSet<u32>,
) -> Vec<[[f32; 3]; 3]> {
    protected
        .iter()
        .filter_map(|&index| {
            let tri = *mesh.polygons.get(index as usize)?;
            Some([
                *mesh.vertices.get(tri[0] as usize)?,
                *mesh.vertices.get(tri[1] as usize)?,
                *mesh.vertices.get(tri[2] as usize)?,
            ])
        })
        .collect()
}

fn in_protected_footprint(point: [f32; 3], footprints: &[[[f32; 3]; 3]]) -> bool {
    footprints
        .iter()
        .any(|&[a, b, c]| barycentric_xz(point[0], point[2], a, b, c).is_some())
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

/// Connectivity-preserving finalization: un-drops a dropped polygon adjacent
/// to a non-main stranded component when
///
/// - the component contains a *protected* seam/door polygon (any drop reason
///   may be undone): an inter-mesh merge seam or door must stay reachable from
///   the main region regardless of size -- the #165/#169 travel hand-off and
///   the door-corridor parity route; or
/// - the component is *large* (>= [`LARGE_ISLAND`] polygons) **and** the drop
///   was a [`DropReason::SubDiameter`] one.
///
/// The reason restriction is the issue #171 half of the rule. Wave 10 judged
/// whole authored triangles, so any drop could plausibly be a misread and the
/// guard undid all three reasons. The re-triangulation replaced the
/// support/obstruction verdicts with direct pointwise collision evidence: a
/// region stranded behind an obstruction clip is stranded because the agent
/// genuinely cannot fit through, and un-dropping that would put the agent
/// inside a wall (or over a void) purely to keep a graph connected -- the
/// steering-around-an-obstruction workaround #148 explicitly rules out. The
/// sub-diameter test is still an inference from authored width rather than
/// direct evidence, so it stays rescuable. Protected polygons are never
/// themselves a drop, so they are not touched.
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
        // Stranded components a *sub-diameter* drop may be undone for, and the
        // (protected-bearing) subset any drop reason may be undone for.
        let large_stranded: BTreeSet<usize> = sizes
            .iter()
            .filter(|(root, size)| {
                **root != main && (**size >= LARGE_ISLAND || protected_roots.contains(*root))
            })
            .map(|(root, _)| *root)
            .collect();
        let protected_stranded: BTreeSet<usize> = large_stranded
            .iter()
            .copied()
            .filter(|root| protected_roots.contains(root))
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
            let Some(drop_reason) = reason[index] else {
                continue;
            };
            // A drop backed by direct collision evidence may only be undone to
            // rescue a protected seam/door; an inferred sub-diameter drop may
            // also be undone to rescue any large stranded region.
            let rescuable = if matches!(drop_reason, DropReason::SubDiameter) {
                &large_stranded
            } else {
                &protected_stranded
            };
            if rescuable.is_empty() {
                continue;
            }
            let adjacent = [
                edge_key(tri[0], tri[1]),
                edge_key(tri[1], tri[2]),
                edge_key(tri[2], tri[0]),
            ]
            .iter()
            .flat_map(|edge| edge_owners.get(edge).into_iter().flatten())
            .any(|&neighbor| rescuable.contains(&roots[neighbor]));
            if adjacent {
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
#[path = "tests/nav_clearance.rs"]
mod tests;
