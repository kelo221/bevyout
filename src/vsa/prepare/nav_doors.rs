//! Pure derivation of blocker -> nav-polygon associations (issue #177, M4
//! wave 11).
//!
//! The authored Bethesda `NAVM` only associates *load/travel* doors with
//! triangles (`NVDP`): measured on the prepared catalog, an ordinary in-cell
//! door has no association at all, so it gets no polygon typing, no crossing
//! gate, and no door link -- the navmesh runs straight through its closed
//! slab and the agent wedges against it in physics. This module recovers the
//! missing input geometrically: given each mesh's walkable triangles and each
//! *blocking placement*'s collision footprint, it reports which polygons that
//! blocker overlaps, and which lie wholly inside it.
//!
//! Two association classes, because they mean different things at query time:
//!
//! - **Gate** (`blocks_when_closed == false`): the polygon merely *overlaps*
//!   the blocker's footprint -- the doorway crossing itself. It stays
//!   routable so the runtime's existing crossing gate can fire on it (pause
//!   -> request open -> wait -> traverse -> resume) exactly the way an
//!   authored travel-door triangle already does. Making these impassable
//!   would turn every closed interior door into a wall, which the issue
//!   explicitly rules out.
//! - **Blocking** (`blocks_when_closed == true`): the polygon lies *wholly
//!   inside* the blocker's collision volume. There is no legitimate way to
//!   stand there while the blocker is closed, so the runtime prices it
//!   `f32::INFINITY` until the blocker opens. This is the `MetroGateLoad`
//!   case, where `tna spawn` placed an agent 0.041 m from the gate surface,
//!   i.e. inside the closed gate.
//!
//! Nothing here is keyed on a FormID, an EditorID, a cell, or a coordinate:
//! the rule is the same geometry applied to every walkable polygon and every
//! blocking placement's footprint. The record-type decision (which placements
//! are blockers, and which of them own a runtime open/close FSM) lives in
//! `navmesh.rs`'s boundary conversion, not here.
//!
//! The blocker footprints themselves are convex XZ hulls built by
//! `navmesh.rs`'s existing `convex_hull_xz` (issue #171's footprint prism
//! machinery), reused rather than reimplemented here.
//!
//! Std-only (no `bevy`/`glam`/`serde`): this file is included verbatim by
//! `tests/features.rs` via `#[path]`, the same way `nav_graph.rs` and
//! `nav_clearance.rs` are -- see `AGENTS.md`'s testing section.

/// How far *below* a blocker's collision volume a polygon's floor may sit and
/// still be considered to be at that blocker's storey (metres). Sized to the
/// agent's step height, matching `nav_clearance::STEP_HEIGHT`; held locally
/// rather than imported, keeping this module std-only for the cucumber
/// `#[path]` include (the same precedent `nav_clearance::AGENT_RADIUS` sets).
pub(crate) const BLOCKER_FLOOR_TOLERANCE: f32 = 0.5;

/// Minimum XZ overlap (metres, along a separating-axis projection) before a
/// polygon counts as overlapping a blocker footprint. Purely a
/// numerical-robustness tolerance: two shapes that merely *touch* along an
/// edge -- a nav polygon whose boundary was clipped exactly onto a collider
/// face by the clearance pass (issue #171) -- are not overlapping.
const OVERLAP_EPSILON: f32 = 1.0e-4;

/// One blocking placement's collision footprint in Bevy-metre world space:
/// the XZ convex hull of every player-blocking shape triangle it contributes,
/// plus that geometry's vertical extent.
///
/// A convex hull rather than an axis-aligned box on purpose: real doors are
/// rotated to their doorway, and an AABB around a diagonal slab covers a
/// large area the door does not occupy -- the same reason issue #171 replaced
/// world-space AABBs with true footprint prisms in `navmesh.rs`.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct BlockerVolume {
    /// The blocker's reference FormID -- the identity the runtime tracks
    /// open/locked state under.
    pub(crate) reference_form_id: u32,
    /// Counter-clockwise XZ convex hull of the blocker's collision geometry.
    /// Fewer than three points means the blocker has no usable footprint and
    /// is skipped.
    pub(crate) footprint: Vec<[f32; 2]>,
    pub(crate) min_y: f32,
    pub(crate) max_y: f32,
    /// Whether this blocker owns a runtime open/close crossing FSM (a real
    /// `DOOR` record). Only a gated blocker emits *gate* associations: a
    /// blocker with no way to be opened must never become a crossing-gate
    /// candidate, or an agent would pause in front of it forever waiting for
    /// an open that can never happen. Non-gated blockers contribute blocking
    /// associations only.
    pub(crate) gated: bool,
    /// The blocker's player-blocking collision triangles in Bevy-metre world
    /// space -- the raw geometry `footprint` above is the convex hull *of*
    /// (issue #189 feature 3).
    ///
    /// Carried separately, and deliberately redundant, because
    /// [`unreported_interior_polygons`] is the invariant check on
    /// [`derive_door_associations`] and must not share the derivation's
    /// input or its containment primitive. Before this, both called the same
    /// `point_in_convex_polygon` over the same `footprint`, so the invariant
    /// could not catch a bug in the thing they shared -- it would simply
    /// agree with it. That is the exact failure shape this project shipped
    /// four times (`docs/postmortem/VERDICT.md` §1).
    pub(crate) collision_triangles: Vec<[[f32; 3]; 3]>,
}

/// One walkable polygon of one prepared nav mesh, in Bevy-metre world space.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct BlockerPolygonInput {
    pub(crate) index: u32,
    pub(crate) vertices: [[f32; 3]; 3],
}

/// One prepared nav mesh reduced to what association needs.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct BlockerMeshInput {
    pub(crate) form_id: u32,
    /// Walkable polygons only -- the boundary conversion filters
    /// `PreparedNavPolygon::walkable` before building this.
    pub(crate) polygons: Vec<BlockerPolygonInput>,
    /// Polygon indices this mesh's *authored* `NVDP` door list already
    /// associates with a door. Such a polygon is never classified blocking:
    /// it is the authored doorway crossing itself -- the exact triangle the
    /// travel/crossing lifecycle routes *to* -- and a real door leaf's
    /// collision hull naturally contains it, so a purely geometric
    /// containment test would price the sanctioned crossing impassable and
    /// make every travel door unusable while shut. Authored evidence wins
    /// over derived geometry, the same precedence
    /// `landmass_graph::resolve_polygon_type_index` applies between door and
    /// preferred-pathing typing.
    pub(crate) authored_door_polygons: std::collections::BTreeSet<u32>,
}

/// One derived blocker -> polygon association.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct DerivedDoorAssociation {
    pub(crate) mesh_form_id: u32,
    pub(crate) door_reference_form_id: u32,
    pub(crate) triangle_index: u32,
    /// `true` when the polygon lies wholly inside the blocker's collision
    /// volume (see the module doc comment's two classes).
    pub(crate) blocks_when_closed: bool,
    /// Mirrors [`BlockerVolume::gated`]: whether this blocker owns a runtime
    /// open/close FSM. Carried through to the runtime because the two cases
    /// need different *costs*, not just different classifications -- a door
    /// that can be opened must stay passable-but-expensive while shut so the
    /// solver routes through it and the crossing gate can open it, whereas a
    /// blocker with no open mechanism is genuinely impassable.
    pub(crate) openable: bool,
}

/// Deterministic association list, ordered by `(door_reference_form_id,
/// mesh_form_id, triangle_index)` -- the same ordering rule
/// `landmass_graph::door_sides` already uses for authored associations, so
/// repeated calls on the same graph produce byte-identical output.
///
/// A polygon is associated with a blocker when its XZ footprint genuinely
/// overlaps the blocker's XZ footprint *and* its floor sits at that blocker's
/// storey (within [`BLOCKER_FLOOR_TOLERANCE`] below the collision volume, and
/// no higher than its top) -- the vertical guard keeps a doorway from
/// claiming the polygon stacked directly above it on the next floor, the same
/// concern `landmass_graph::point_in_door_triangle`'s vertical guard answers.
/// It is `blocks_when_closed` when *every* vertex additionally lies inside
/// the footprint, i.e. the whole polygon is inside the solid.
pub(crate) fn derive_door_associations(
    meshes: &[BlockerMeshInput],
    blockers: &[BlockerVolume],
) -> Vec<DerivedDoorAssociation> {
    let mut associations = Vec::new();
    for blocker in blockers {
        if blocker.footprint.len() < 3 {
            continue;
        }
        for mesh in meshes {
            for polygon in &mesh.polygons {
                let floor = polygon
                    .vertices
                    .iter()
                    .fold(f32::INFINITY, |acc, vertex| acc.min(vertex[1]));
                if floor < blocker.min_y - BLOCKER_FLOOR_TOLERANCE || floor > blocker.max_y {
                    continue;
                }
                let triangle = polygon.vertices.map(|vertex| [vertex[0], vertex[2]]);
                if !convex_shapes_overlap(&triangle, &blocker.footprint) {
                    continue;
                }
                let contained = !mesh.authored_door_polygons.contains(&polygon.index)
                    && triangle.iter().all(|point| {
                        bevyout_core::geometry::point_in_convex_polygon_xz(
                            *point,
                            &blocker.footprint,
                        )
                    });
                if !blocker.gated && !contained {
                    continue;
                }
                associations.push(DerivedDoorAssociation {
                    mesh_form_id: mesh.form_id,
                    door_reference_form_id: blocker.reference_form_id,
                    triangle_index: polygon.index,
                    blocks_when_closed: contained,
                    openable: blocker.gated,
                });
            }
        }
    }
    associations.sort_by_key(|association| {
        (
            association.door_reference_form_id,
            association.mesh_form_id,
            association.triangle_index,
        )
    });
    associations.dedup();
    associations
}

/// Whether two convex XZ polygons genuinely overlap (separating-axis test
/// over both shapes' edge normals). Shapes that merely touch along an edge or
/// a vertex do not overlap, per [`OVERLAP_EPSILON`].
fn convex_shapes_overlap(a: &[[f32; 2]], b: &[[f32; 2]]) -> bool {
    !has_separating_axis(a, b) && !has_separating_axis(b, a)
}

fn has_separating_axis(shape: &[[f32; 2]], other: &[[f32; 2]]) -> bool {
    for index in 0..shape.len() {
        let start = shape[index];
        let end = shape[(index + 1) % shape.len()];
        let axis = [-(end[1] - start[1]), end[0] - start[0]];
        let length = (axis[0] * axis[0] + axis[1] * axis[1]).sqrt();
        if length < 1.0e-9 {
            continue;
        }
        let axis = [axis[0] / length, axis[1] / length];
        let (min_a, max_a) = project(shape, axis);
        let (min_b, max_b) = project(other, axis);
        if max_a <= min_b + OVERLAP_EPSILON || max_b <= min_a + OVERLAP_EPSILON {
            return true;
        }
    }
    false
}

fn project(shape: &[[f32; 2]], axis: [f32; 2]) -> (f32, f32) {
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    for point in shape {
        let value = point[0] * axis[0] + point[1] * axis[1];
        min = min.min(value);
        max = max.max(value);
    }
    (min, max)
}

// ---------------------------------------------------------------------
// The invariant check -- an INDEPENDENT verification path (issue #189
// feature 3). Nothing below this line may call the shared convex-polygon
// primitive `bevyout_core::geometry::point_in_convex_polygon_xz`,
// `convex_shapes_overlap`, or read `BlockerVolume::footprint` for a
// containment decision: those are `derive_door_associations`' primitives and
// its input, i.e. the subject under test.
// ---------------------------------------------------------------------

/// Slack (metres) the independent check allows around a blocker whose
/// collision geometry projects to *zero* XZ area.
///
/// Real authored Havok door collision is routinely a single zero-thickness
/// plane (`MetroGateLoad`), whose XZ projection is a line segment with no
/// interior -- so a strict inside-the-projection test would find nothing and
/// the invariant would be vacuous for exactly the blocker class that motivated
/// it, which is the disease this issue exists to cure rather than a tolerable
/// gap. Matches `navmesh::BLOCKER_MIN_HALF_THICKNESS`, the half-thickness the
/// footprint builder gives the same flat geometry, so the two paths judge the
/// same *region* while computing it by different means. Held locally rather
/// than imported, keeping this module std-only for the cucumber `#[path]`
/// include (the same precedent [`BLOCKER_FLOOR_TOLERANCE`] sets).
const FLAT_SOLID_TOLERANCE: f32 = 0.05;

/// Total projected XZ area (m²) below which a blocker's collision geometry
/// counts as flat and earns [`FLAT_SOLID_TOLERANCE`]. Purely a
/// numerical-robustness floor: 1 mm² is orders of magnitude below any real
/// door leaf's footprint and above f32 rounding on world-space coordinates.
const FLAT_SOLID_AREA: f32 = 1.0e-6;

/// Slack (metres) the independent check allows around a *solid* blocker's
/// projected boundary.
///
/// An even-odd crossing count is exact but undecided exactly *on* an edge, and
/// a triangle soup is full of interior edges: a point on a collision
/// triangulation's internal diagonal belongs to both neighbours and is
/// reported by neither. The clearance pass (issue #171) clips nav polygon
/// boundaries directly onto collider faces, so lying exactly on a collision
/// edge is the common case here, not an oddity.
///
/// Numerically equal to `bevyout_core::geometry::CONTAINMENT_EPSILON`, the
/// slack the derivation's shared convex-polygon primitive gives its boundary
/// (issue #189 feature 4), so the two paths agree on how much
/// f32 noise counts as "on the surface" while still deciding containment by
/// different means. That is a shared *convention*, not a shared primitive: at
/// 0.1 mm it cannot turn ground the derivation could not have claimed into a
/// prepare failure.
const SOLID_BOUNDARY_TOLERANCE: f32 = 1.0e-4;

/// Twice the signed XZ area of a projected collision triangle.
fn projected_double_area(triangle: &[[f32; 3]; 3]) -> f32 {
    let [a, b, c] = triangle;
    (b[0] - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (b[2] - a[2])
}

/// Whether `point` is inside a collision triangle's XZ projection, by an
/// **even-odd ray-crossing** test.
///
/// Deliberately a different algorithm from the barycentric/half-plane family
/// the derivation and the rest of the nav pipeline use (issue #189 feature 4's
/// shared primitive): a crossing count makes no assumption about winding,
/// convexity or the sign of a determinant, so a sign, winding or epsilon
/// mistake in that family cannot reproduce itself here and silently make this
/// invariant agree with the bug it is meant to catch.
fn point_in_projected_triangle(point: [f32; 2], triangle: &[[f32; 3]; 3]) -> bool {
    let mut inside = false;
    for index in 0..3 {
        let a = triangle[index];
        let b = triangle[(index + 1) % 3];
        let (az, bz) = (a[2], b[2]);
        if (az > point[1]) != (bz > point[1]) {
            let x = a[0] + (point[1] - az) / (bz - az) * (b[0] - a[0]);
            if point[0] < x {
                inside = !inside;
            }
        }
    }
    inside
}

/// XZ distance from `point` to a projected collision triangle's boundary.
fn distance_to_projected_triangle(point: [f32; 2], triangle: &[[f32; 3]; 3]) -> f32 {
    let mut best = f32::INFINITY;
    for index in 0..3 {
        let a = triangle[index];
        let b = triangle[(index + 1) % 3];
        let (ex, ez) = (b[0] - a[0], b[2] - a[2]);
        let length_squared = ex * ex + ez * ez;
        let t = if length_squared < 1.0e-12 {
            0.0
        } else {
            (((point[0] - a[0]) * ex + (point[1] - a[2]) * ez) / length_squared).clamp(0.0, 1.0)
        };
        let (cx, cz) = (a[0] + ex * t, a[2] + ez * t);
        best = best.min(((point[0] - cx).powi(2) + (point[1] - cz).powi(2)).sqrt());
    }
    best
}

/// Whether `point` lies inside a blocker's *actual collision solid* in XZ,
/// sampled against the blocker's raw triangles rather than the convex hull
/// footprint the derivation uses.
///
/// The union of a blocker's projected triangles is always a subset of their
/// convex hull, so for solid geometry this predicate is strictly *no more*
/// permissive than the derivation's: it cannot invent a violation the
/// derivation had no chance to report. What it can do -- and the only reason
/// it exists -- is disagree when the derivation's own containment primitive is
/// wrong.
fn point_inside_solid_xz(point: [f32; 2], blocker: &BlockerVolume) -> bool {
    let flat = blocker
        .collision_triangles
        .iter()
        .map(|triangle| projected_double_area(triangle).abs() / 2.0)
        .sum::<f32>()
        < FLAT_SOLID_AREA;
    let tolerance = if flat {
        FLAT_SOLID_TOLERANCE
    } else {
        SOLID_BOUNDARY_TOLERANCE
    };
    blocker.collision_triangles.iter().any(|triangle| {
        point_in_projected_triangle(point, triangle)
            || distance_to_projected_triangle(point, triangle) <= tolerance
    })
}

/// Every walkable polygon that lies wholly inside a blocker's collision
/// volume without a matching `blocks_when_closed` association -- the
/// deterministic invariant `navmesh.rs` enforces as a hard `prepare` failure
/// (issue #189 feature 2) and the unit/cucumber suites assert is empty.
/// Returns `(mesh_form_id, triangle_index, reference_form_id)` triples,
/// sorted.
///
/// Verified independently of its subject (issue #189 feature 3): containment
/// is decided by [`point_inside_solid_xz`] against the blocker's raw collision
/// triangles, never by the shared `geometry::point_in_convex_polygon_xz` /
/// `BlockerVolume::footprint` pair `derive_door_associations` runs on. A check
/// that shares its primitive with the code it validates can only ever agree
/// with that code -- see this module's section header above.
pub(crate) fn unreported_interior_polygons(
    meshes: &[BlockerMeshInput],
    blockers: &[BlockerVolume],
    associations: &[DerivedDoorAssociation],
) -> Vec<(u32, u32, u32)> {
    let reported: std::collections::BTreeSet<(u32, u32, u32)> = associations
        .iter()
        .filter(|association| association.blocks_when_closed)
        .map(|association| {
            (
                association.mesh_form_id,
                association.triangle_index,
                association.door_reference_form_id,
            )
        })
        .collect();
    let mut unreported = Vec::new();
    for blocker in blockers {
        if blocker.footprint.len() < 3 {
            continue;
        }
        for mesh in meshes {
            for polygon in &mesh.polygons {
                let floor = polygon
                    .vertices
                    .iter()
                    .fold(f32::INFINITY, |acc, vertex| acc.min(vertex[1]));
                if floor < blocker.min_y - BLOCKER_FLOOR_TOLERANCE || floor > blocker.max_y {
                    continue;
                }
                // Authored doorway crossings are exempt for the same reason
                // they are never classified blocking -- see
                // `BlockerMeshInput::authored_door_polygons`.
                let inside = !mesh.authored_door_polygons.contains(&polygon.index)
                    && polygon
                        .vertices
                        .iter()
                        .all(|vertex| point_inside_solid_xz([vertex[0], vertex[2]], blocker));
                if !inside {
                    continue;
                }
                let key = (mesh.form_id, polygon.index, blocker.reference_form_id);
                if !reported.contains(&key) {
                    unreported.push(key);
                }
            }
        }
    }
    unreported.sort_unstable();
    unreported
}

#[cfg(test)]
#[path = "tests/nav_doors.rs"]
mod tests;
