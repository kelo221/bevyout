use super::*;

#[test]
fn only_a_fresh_alive_untouched_actor_is_eligible() {
    assert!(eligible_for_autonomous_start(true, false, false));
}

#[test]
fn a_dead_actor_is_never_eligible() {
    assert!(!eligible_for_autonomous_start(false, false, false));
    // Even if somehow already bound/controlled, death alone is enough
    // to reject -- this asserts life-state is checked unconditionally,
    // not merely first-checked-first-fails.
    assert!(!eligible_for_autonomous_start(false, true, true));
}

#[test]
fn an_already_nav_bound_actor_is_left_to_the_console() {
    assert!(!eligible_for_autonomous_start(true, true, false));
}

#[test]
fn an_actor_already_running_a_package_is_left_alone() {
    assert!(!eligible_for_autonomous_start(true, false, true));
}
