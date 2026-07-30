use super::*;

const TRI: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];

#[test]
fn a_point_in_the_interior_is_contained_under_either_winding() {
    assert!(point_in_triangle_xz([0.25, 0.0, 0.25], TRI));
    let reversed = [TRI[0], TRI[2], TRI[1]];
    assert!(point_in_triangle_xz([0.25, 5.0, 0.25], reversed));
}

#[test]
fn barycentric_weights_reconstruct_the_point() {
    let w = barycentric_xz(0.25, 0.25, TRI[0], TRI[1], TRI[2]).expect("inside");
    let x = w[0] * TRI[0][0] + w[1] * TRI[1][0] + w[2] * TRI[2][0];
    let z = w[0] * TRI[0][2] + w[1] * TRI[1][2] + w[2] * TRI[2][2];
    assert!((x - 0.25).abs() < 1.0e-6 && (z - 0.25).abs() < 1.0e-6);
}

#[test]
fn a_degenerate_triangle_never_contains_a_point() {
    let line = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
    assert!(barycentric_xz(0.5, 0.0, line[0], line[1], line[2]).is_none());
}

/// The whole point of feature 4: one epsilon, and its boundary is pinned
/// so a future edit cannot silently widen it into a shape filter (the
/// 1e-4 defect) or tighten it into sliver retention (the 1e-9 defect)
/// without turning this red.
#[test]
fn the_containment_epsilon_boundary_is_pinned() {
    // Just outside edge b->c (the hypotenuse x + z = 1). A point at
    // (0.5 + d, 0.5 + d) is d*sqrt(2) beyond the edge.
    let just_outside = |d: f32| point_in_triangle_xz([0.5 + d, 0.0, 0.5 + d], TRI);
    // Well within the slack: contained.
    assert!(just_outside(1.0e-5));
    // Well beyond the slack: excluded.
    assert!(!just_outside(1.0e-3));
    // Convex-polygon form agrees on the same boundary.
    let square = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    assert!(point_in_convex_polygon_xz([1.0 + 1.0e-5, 0.5], &square));
    assert!(!point_in_convex_polygon_xz([1.0 + 1.0e-3, 0.5], &square));
}
