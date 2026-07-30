use super::*;

fn square(min_x: f32, min_z: f32, max_x: f32, max_z: f32) -> Vec<[f32; 2]> {
    vec![
        [min_x, min_z],
        [max_x, min_z],
        [max_x, max_z],
        [min_x, max_z],
    ]
}

/// A solid blocker: the footprint plus the collision geometry it is the
/// hull of, as a triangle fan over the same outline. The two are supplied
/// separately on purpose -- `derive_door_associations` reads only the
/// former and `unreported_interior_polygons` only the latter (issue #189
/// feature 3), and [`fake_solid_geometry`] below exploits exactly that to
/// prove the invariant is not merely echoing the derivation.
fn blocker(footprint: Vec<[f32; 2]>, gated: bool) -> BlockerVolume {
    let collision_triangles = fan(&footprint, 0.0);
    BlockerVolume {
        reference_form_id: 0x99,
        footprint,
        min_y: 0.0,
        max_y: 2.0,
        gated,
        collision_triangles,
    }
}

/// Triangle fan over a closed XZ outline, at height `y`.
fn fan(outline: &[[f32; 2]], y: f32) -> Vec<[[f32; 3]; 3]> {
    (1..outline.len().saturating_sub(1))
        .map(|index| {
            [
                [outline[0][0], y, outline[0][1]],
                [outline[index][0], y, outline[index][1]],
                [outline[index + 1][0], y, outline[index + 1][1]],
            ]
        })
        .collect()
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
    let ungated = derive_door_associations(&meshes, &[blocker(square(0.0, 0.0, 1.0, 1.0), false)]);
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

/// Issue #189 feature 3, and the test the whole feature is for: the
/// invariant must be able to *disagree* with the derivation.
///
/// The blocker below is given a footprint that excludes the polygon while
/// its collision solid encloses it -- the observable signature of a bug in
/// the derivation's own containment primitive or footprint construction.
/// The derivation therefore reports nothing, and the invariant must still
/// fire. Restore the pre-#189 shape (a check reading
/// `point_in_convex_polygon` over `BlockerVolume::footprint`, the
/// derivation's own primitive and input) and this goes red, because such a
/// check can only ever agree with the code it is validating.
#[test]
fn the_invariant_reads_the_collision_solid_not_the_derivations_footprint() {
    let meshes = [mesh(vec![triangle(
        1,
        [[0.1, 0.1], [0.9, 0.1], [0.5, 0.9]],
        0.0,
    )])];
    let blockers = [BlockerVolume {
        // A footprint nowhere near the polygon: whatever produced it is
        // wrong, and the derivation has no way to know that.
        footprint: square(10.0, 10.0, 11.0, 11.0),
        ..blocker(square(0.0, 0.0, 1.0, 1.0), true)
    }];
    let associations = derive_door_associations(&meshes, &blockers);
    assert!(associations.is_empty(), "{associations:?}");
    assert_eq!(
        unreported_interior_polygons(&meshes, &blockers, &associations),
        vec![(0x10, 1, 0x99)],
    );
}

/// The invariant must not be vacuous for the blocker class that motivated
/// it. Real authored Havok door collision is routinely a single
/// zero-thickness plane (`MetroGateLoad`), whose XZ projection has no
/// interior at all -- so a strict inside-the-projection test would report
/// nothing forever while reading perfectly healthy.
#[test]
fn a_zero_thickness_collision_plane_still_claims_the_ground_on_it() {
    let meshes = [mesh(vec![triangle(
        1,
        [[0.5, 0.49], [0.6, 0.5], [0.5, 0.51]],
        0.0,
    )])];
    let plane = [
        [[0.0f32, 0.0, 0.5], [1.0, 0.0, 0.5], [1.0, 2.0, 0.5]],
        [[0.0, 0.0, 0.5], [1.0, 2.0, 0.5], [0.0, 2.0, 0.5]],
    ];
    let blockers = [BlockerVolume {
        collision_triangles: plane.to_vec(),
        ..blocker(square(0.0, 0.45, 1.0, 0.55), true)
    }];
    let associations = derive_door_associations(&meshes, &blockers);
    let unreported = unreported_interior_polygons(&meshes, &blockers, &associations);
    assert!(unreported.is_empty(), "{unreported:?}");
    // ... and it is a real verdict, not an empty one: with the derivation
    // silenced, the same polygon is reported.
    assert_eq!(
        unreported_interior_polygons(&meshes, &blockers, &[]),
        vec![(0x10, 1, 0x99)],
    );
}

/// The independent path must not be *more* permissive than the
/// derivation's hull, or it would fail `prepare` on ground the derivation
/// never had a chance to claim. Ground outside a solid blocker is outside
/// it under both paths.
#[test]
fn ground_outside_a_solid_blocker_is_never_reported() {
    let meshes = [mesh(vec![triangle(
        1,
        [[2.0, 2.0], [3.0, 2.0], [2.5, 3.0]],
        0.0,
    )])];
    let blockers = [blocker(square(0.0, 0.0, 1.0, 1.0), true)];
    assert!(unreported_interior_polygons(&meshes, &blockers, &[]).is_empty());
}
