use super::*;

#[test]
fn kill_z_is_a_fixed_margin_below_the_bounds_minimum() {
    assert_eq!(fall_kill_z(94.168), 94.168 - FALL_GUARD_MARGIN_METRES);
    assert_eq!(fall_kill_z(0.0), -FALL_GUARD_MARGIN_METRES);
}

#[test]
fn an_agent_above_the_kill_plane_is_in_bounds() {
    // Real FranklinMetro02 (0001a273) whole-graph bounds minimum Y is
    // ~94.168; a capsule resting on the lowest walkable surface sits
    // above it, never a whole margin below.
    assert_eq!(evaluate_fall(94.168, 95.0), FallVerdict::InBounds);
}

#[test]
fn an_agent_resting_exactly_at_the_kill_plane_is_still_in_bounds() {
    let min_y = 94.168;
    assert_eq!(
        evaluate_fall(min_y, fall_kill_z(min_y)),
        FallVerdict::InBounds
    );
}

#[test]
fn an_agent_below_the_kill_plane_has_fallen_out_of_the_world() {
    let min_y = 94.168;
    assert_eq!(
        evaluate_fall(min_y, fall_kill_z(min_y) - 0.01),
        FallVerdict::FellOutOfWorld
    );
}

#[test]
fn an_agent_that_descended_far_below_the_cell_is_out_of_the_world() {
    // The exact failure mode #164 describes: an agent that fell through
    // a missing floor keeps descending without bound.
    assert_eq!(evaluate_fall(94.168, -1000.0), FallVerdict::FellOutOfWorld);
}
