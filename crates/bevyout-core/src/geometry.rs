//! Shared XZ point-in-polygon containment (issue #189 feature 4).
//!
//! The navigation pipeline previously carried *four* hand-rolled point-in-
//! polygon primitives, with three different tolerances (exact zero, 1e-4,
//! 1e-9):
//!
//! - `viewer::nav::landmass_graph::point_in_triangle_xz` (exact zero),
//! - `vsa::prepare::nav_clearance::barycentric_xz` (1e-4),
//! - `vsa::prepare::nav_doors::point_in_convex_polygon` (1e-4),
//! - an inline copy inside `vsa::prepare::navmesh`'s probe report (1e-9 for
//!   the degeneracy guard, 1e-4 for containment).
//!
//! Four implementations are four chances to be inconsistent at a seam, and
//! this exact family has already produced two shipped defects: an epsilon
//! acting as a shape filter at 1e-4, then retaining unwindable slivers at
//! 1e-9 (`docs/postmortem/VERDICT.md` §2.5). This module is the single source
//! of truth they now all defer to.
//!
//! It deliberately does **not** absorb `nav_doors::unreported_interior_polygons`'
//! containment test. That check is the invariant on `derive_door_associations`,
//! and issue #189 feature 3 requires it to verify by an *independent* means --
//! sampling the blocker's raw collision triangles with its own even-odd
//! crossing count. A check that shared this primitive with the code it
//! validates could only ever agree with it, which is the failure this whole
//! issue exists to remove. The consolidation here serves the *production*
//! paths; the invariant keeps its separate path on purpose.
//!
//! Engine-independent (std + `glam`-free): plain `[f32; 2]`/`[f32; 3]` arrays,
//! so both the prepare pipeline and the Bevy-free `viewer::nav` boundary
//! modules can share it, and the cucumber `#[path]` includes reach it as an
//! ordinary crate dependency.

/// Boundary-inclusion slack (metres) for every containment test in this
/// module.
///
/// Sign and magnitude both matter here, and both were wrong somewhere in the
/// four primitives this replaces:
///
/// - **Positive**, so a point sitting numerically *on* a shared edge counts as
///   inside both neighbouring polygons. The clearance pass (issue #171) clips
///   nav-polygon boundaries directly onto collider faces and shares edges
///   between adjacent triangles, so "exactly on an edge" is the common case,
///   not an oddity -- an exact-zero test (the old `landmass_graph` primitive)
///   lets a point on a seam fall through the crack between two polygons that
///   both should contain it.
/// - **1e-4 m (0.1 mm)**, comfortably above f32 coordinate noise and below any
///   real geometric feature. World coordinates reach the tens of metres, where
///   an f32 ulp is on the order of 1e-5 m; 1e-9 (the old probe-report
///   degeneracy epsilon, misapplied as a containment slack elsewhere) sits
///   *below* that noise floor, so it retains slivers f32 rounding cannot even
///   represent -- the second shipped defect. 1e-4 is the smallest slack that
///   is reliably above the noise while still far tighter than the ~0.3 m agent
///   radius, so it can never turn a genuinely-outside point into a false
///   containment.
///
/// [`crate::geometry::tests::the_containment_epsilon_boundary_is_pinned`]
/// pins this value's boundary behaviour: a point 1e-5 m outside an edge is
/// contained, one 1e-3 m outside is not.
pub const CONTAINMENT_EPSILON: f32 = 1.0e-4;

/// Area-degeneracy floor (metres²·2) below which a triangle's XZ projection is
/// treated as having no interior (a wall triangle projects to a line, a
/// zero-area triangle is authoring noise). Separate from
/// [`CONTAINMENT_EPSILON`] because it guards a *division*, not a boundary: it
/// is compared against `2 * signed area`, which is `O(edge_length²)`, so it
/// lives far below the linear containment slack. 1e-9 is the value all three
/// barycentric variants already used for exactly this guard; unifying it here
/// keeps that agreement rather than reintroducing a fourth threshold.
pub const DEGENERACY_EPSILON: f32 = 1.0e-9;

/// Barycentric weights `[alpha, beta, gamma]` of `(px, pz)` within the XZ
/// projection of triangle `a, b, c` (`alpha` weights `a`, `beta` weights `b`,
/// `gamma` weights `c`), or `None` when the point is outside (beyond
/// [`CONTAINMENT_EPSILON`]) or the projection is degenerate (below
/// [`DEGENERACY_EPSILON`] -- a vertical wall triangle projects to a line and
/// so never contains a point, which is exactly why walls never falsely
/// support a nav point).
///
/// Winding-independent: the signed area `det` carries the winding's sign and
/// divides through, so a point inside is reported inside under either winding.
/// The weights are the caller's own affordance -- `nav_clearance` and the
/// `navmesh` probe interpolate surface height from them; a caller needing only
/// containment uses [`point_in_triangle_xz`].
pub fn barycentric_xz(px: f32, pz: f32, a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> Option<[f32; 3]> {
    let v0x = b[0] - a[0];
    let v0z = b[2] - a[2];
    let v1x = c[0] - a[0];
    let v1z = c[2] - a[2];
    let det = v0x * v1z - v1x * v0z;
    if det.abs() < DEGENERACY_EPSILON {
        return None;
    }
    let v2x = px - a[0];
    let v2z = pz - a[2];
    let beta = (v2x * v1z - v1x * v2z) / det;
    let gamma = (v0x * v2z - v2x * v0z) / det;
    let alpha = 1.0 - beta - gamma;
    if alpha < -CONTAINMENT_EPSILON || beta < -CONTAINMENT_EPSILON || gamma < -CONTAINMENT_EPSILON {
        None
    } else {
        Some([alpha, beta, gamma])
    }
}

/// Whether `point` lies inside `triangle`'s XZ projection (height ignored).
/// The containment half of [`barycentric_xz`], for callers that do not need
/// the weights.
pub fn point_in_triangle_xz(point: [f32; 3], triangle: [[f32; 3]; 3]) -> bool {
    barycentric_xz(point[0], point[2], triangle[0], triangle[1], triangle[2]).is_some()
}

/// Whether `point` lies inside the counter-clockwise convex XZ `polygon`
/// (boundary counts as inside, per [`CONTAINMENT_EPSILON`]). A triangle is a
/// convex polygon, but this half-plane form takes an arbitrary-arity convex
/// hull -- the shape `nav_doors`' blocker footprints are.
pub fn point_in_convex_polygon_xz(point: [f32; 2], polygon: &[[f32; 2]]) -> bool {
    for index in 0..polygon.len() {
        let start = polygon[index];
        let end = polygon[(index + 1) % polygon.len()];
        let cross = (end[0] - start[0]) * (point[1] - start[1])
            - (end[1] - start[1]) * (point[0] - start[0]);
        if cross < -CONTAINMENT_EPSILON {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[path = "tests/geometry.rs"]
mod tests;
