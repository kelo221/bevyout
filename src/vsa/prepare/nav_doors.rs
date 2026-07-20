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

/// Slack (metres) allowed when testing whether a polygon vertex lies inside a
/// blocker footprint, so a vertex sitting numerically *on* the footprint
/// boundary still counts as inside. Same robustness role as
/// [`OVERLAP_EPSILON`].
const CONTAINMENT_EPSILON: f32 = 1.0e-4;

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
                    && triangle
                        .iter()
                        .all(|point| point_in_convex_polygon(*point, &blocker.footprint));
                if !blocker.gated && !contained {
                    continue;
                }
                associations.push(DerivedDoorAssociation {
                    mesh_form_id: mesh.form_id,
                    door_reference_form_id: blocker.reference_form_id,
                    triangle_index: polygon.index,
                    blocks_when_closed: contained,
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

fn cross(origin: [f32; 2], a: [f32; 2], b: [f32; 2]) -> f32 {
    (a[0] - origin[0]) * (b[1] - origin[1]) - (a[1] - origin[1]) * (b[0] - origin[0])
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

/// Whether `point` lies inside the counter-clockwise convex `polygon`
/// (boundary counts as inside, per [`CONTAINMENT_EPSILON`]).
fn point_in_convex_polygon(point: [f32; 2], polygon: &[[f32; 2]]) -> bool {
    for index in 0..polygon.len() {
        let start = polygon[index];
        let end = polygon[(index + 1) % polygon.len()];
        if cross(start, end, point) < -CONTAINMENT_EPSILON {
            return false;
        }
    }
    true
}

/// Every walkable polygon that lies wholly inside a blocker's collision
/// volume without a matching `blocks_when_closed` association -- the
/// deterministic invariant check `navmesh.rs` reports at prepare time and the
/// unit/cucumber suites assert is empty. Returns `(mesh_form_id,
/// triangle_index, reference_form_id)` triples, sorted.
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
                    && polygon.vertices.iter().all(|vertex| {
                        point_in_convex_polygon([vertex[0], vertex[2]], &blocker.footprint)
                    });
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
mod tests {
    use super::*;

    fn square(min_x: f32, min_z: f32, max_x: f32, max_z: f32) -> Vec<[f32; 2]> {
        vec![
            [min_x, min_z],
            [max_x, min_z],
            [max_x, max_z],
            [min_x, max_z],
        ]
    }

    fn blocker(footprint: Vec<[f32; 2]>, gated: bool) -> BlockerVolume {
        BlockerVolume {
            reference_form_id: 0x99,
            footprint,
            min_y: 0.0,
            max_y: 2.0,
            gated,
        }
    }

    fn mesh(polygons: Vec<BlockerPolygonInput>) -> BlockerMeshInput {
        BlockerMeshInput {
            form_id: 0x10,
            polygons,
            authored_door_polygons: std::collections::BTreeSet::new(),
        }
    }

    #[test]
    fn an_authored_door_polygon_is_never_classified_blocking() {
        let mut input = mesh(vec![triangle(3, [[0.1, 0.1], [0.9, 0.1], [0.5, 0.9]], 0.0)]);
        input.authored_door_polygons.insert(3);
        let blockers = [blocker(square(0.0, 0.0, 1.0, 1.0), true)];
        let associations = derive_door_associations(&[input.clone()], &blockers);
        assert_eq!(associations.len(), 1);
        assert!(!associations[0].blocks_when_closed);
        assert!(unreported_interior_polygons(&[input], &blockers, &associations).is_empty());
    }

    fn triangle(index: u32, points: [[f32; 2]; 3], y: f32) -> BlockerPolygonInput {
        BlockerPolygonInput {
            index,
            vertices: points.map(|point| [point[0], y, point[1]]),
        }
    }

    #[test]
    fn overlapping_polygon_is_a_gate_association() {
        let associations = derive_door_associations(
            &[mesh(vec![triangle(
                7,
                [[-1.0, 0.0], [1.0, 0.0], [0.0, 1.0]],
                0.0,
            )])],
            &[blocker(square(-0.1, 0.2, 0.1, 0.6), true)],
        );
        assert_eq!(associations.len(), 1);
        assert_eq!(associations[0].triangle_index, 7);
        assert!(!associations[0].blocks_when_closed);
    }

    #[test]
    fn polygon_wholly_inside_the_volume_blocks_when_closed() {
        let associations = derive_door_associations(
            &[mesh(vec![triangle(
                3,
                [[0.1, 0.1], [0.9, 0.1], [0.5, 0.9]],
                0.0,
            )])],
            &[blocker(square(0.0, 0.0, 1.0, 1.0), true)],
        );
        assert_eq!(associations.len(), 1);
        assert!(associations[0].blocks_when_closed);
    }

    #[test]
    fn a_non_gated_blocker_only_reports_contained_polygons() {
        let meshes = [mesh(vec![
            triangle(1, [[0.1, 0.1], [0.9, 0.1], [0.5, 0.9]], 0.0),
            triangle(2, [[0.5, 0.5], [3.0, 0.5], [3.0, 3.0]], 0.0),
        ])];
        let gated = derive_door_associations(&meshes, &[blocker(square(0.0, 0.0, 1.0, 1.0), true)]);
        assert_eq!(gated.len(), 2);
        let ungated =
            derive_door_associations(&meshes, &[blocker(square(0.0, 0.0, 1.0, 1.0), false)]);
        assert_eq!(ungated.len(), 1);
        assert_eq!(ungated[0].triangle_index, 1);
        assert!(ungated[0].blocks_when_closed);
    }

    #[test]
    fn a_polygon_on_another_storey_is_not_associated() {
        let associations = derive_door_associations(
            &[mesh(vec![triangle(
                1,
                [[0.1, 0.1], [0.9, 0.1], [0.5, 0.9]],
                8.0,
            )])],
            &[blocker(square(0.0, 0.0, 1.0, 1.0), true)],
        );
        assert!(associations.is_empty());
    }

    #[test]
    fn a_polygon_merely_touching_the_footprint_edge_is_not_associated() {
        let associations = derive_door_associations(
            &[mesh(vec![triangle(
                1,
                [[1.0, 0.0], [3.0, 0.0], [3.0, 2.0]],
                0.0,
            )])],
            &[blocker(square(0.0, 0.0, 1.0, 1.0), true)],
        );
        assert!(associations.is_empty());
    }

    #[test]
    fn associations_are_deterministic_and_sorted() {
        let meshes = [mesh(vec![
            triangle(5, [[0.1, 0.1], [0.9, 0.1], [0.5, 0.9]], 0.0),
            triangle(2, [[0.2, 0.2], [0.8, 0.2], [0.5, 0.8]], 0.0),
        ])];
        let blockers = [
            BlockerVolume {
                reference_form_id: 0x40,
                ..blocker(square(0.0, 0.0, 1.0, 1.0), true)
            },
            BlockerVolume {
                reference_form_id: 0x20,
                ..blocker(square(0.0, 0.0, 1.0, 1.0), true)
            },
        ];
        let first = derive_door_associations(&meshes, &blockers);
        let second = derive_door_associations(&meshes, &blockers);
        assert_eq!(first, second);
        let keys: Vec<(u32, u32)> = first
            .iter()
            .map(|association| {
                (
                    association.door_reference_form_id,
                    association.triangle_index,
                )
            })
            .collect();
        assert_eq!(keys, vec![(0x20, 2), (0x20, 5), (0x40, 2), (0x40, 5)]);
    }

    /// The invariant this issue is measured by: once a closed blocker's
    /// contained polygons are reported, no *unreported* walkable polygon is
    /// left wholly inside its collision volume.
    #[test]
    fn no_walkable_polygon_is_left_unreported_inside_a_blocker() {
        let meshes = [mesh(vec![
            triangle(1, [[0.1, 0.1], [0.9, 0.1], [0.5, 0.9]], 0.0),
            triangle(2, [[0.2, 0.2], [0.4, 0.2], [0.3, 0.4]], 0.0),
            triangle(3, [[5.0, 5.0], [6.0, 5.0], [5.5, 6.0]], 0.0),
        ])];
        let blockers = [blocker(square(0.0, 0.0, 1.0, 1.0), true)];
        let associations = derive_door_associations(&meshes, &blockers);
        let unreported = unreported_interior_polygons(&meshes, &blockers, &associations);
        assert!(unreported.is_empty(), "{unreported:?}");
        assert_eq!(
            associations
                .iter()
                .filter(|association| association.blocks_when_closed)
                .count(),
            2
        );
    }
}
