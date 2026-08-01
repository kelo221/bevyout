use super::*;

#[test]
fn discovering_an_eligible_mesh_queues_one_processing_pass() {
    let mut tracker = AoEligibilityTracker::<u32, u32>::default();
    assert!(!tracker.has_pending());
    tracker.discover(1, 10, true);
    assert!(tracker.is_eligible(10));
    assert!(tracker.is_pending(10));
    tracker.resolve_pending(10);
    assert!(!tracker.has_pending());
    assert!(tracker.is_eligible(10));
    // Rediscovery of the same entity/outcome is idempotent after resolution.
    tracker.discover(1, 10, true);
    assert!(!tracker.has_pending());
}

#[test]
fn an_ineligible_discovery_tracks_nothing() {
    let mut tracker = AoEligibilityTracker::<u32, u32>::default();
    tracker.discover(1, 10, false);
    assert!(!tracker.is_eligible(10));
    assert!(!tracker.has_pending());
}

#[test]
fn a_remove_and_add_pair_with_equal_totals_queues_the_new_mesh() {
    // The #270 regression: a despawn plus a spawn inside one interval left
    // the entity/asset counts unchanged, so the count sentinel never fired.
    let mut tracker = AoEligibilityTracker::<u32, u32>::default();
    tracker.discover(1, 10, true);
    tracker.resolve_pending(10);

    tracker.release(1);
    tracker.discover(2, 11, true);

    assert!(!tracker.is_eligible(10));
    assert!(!tracker.is_pending(10));
    assert!(tracker.is_eligible(11));
    assert!(tracker.is_pending(11));
}

#[test]
fn shared_meshes_stay_eligible_until_the_last_referrer_leaves() {
    let mut tracker = AoEligibilityTracker::<u32, u32>::default();
    tracker.discover(1, 10, true);
    tracker.discover(2, 10, true);
    tracker.release(1);
    assert!(tracker.is_eligible(10));
    assert!(tracker.is_pending(10));
    tracker.release(2);
    assert!(!tracker.is_eligible(10));
    assert!(!tracker.has_pending());
}

#[test]
fn a_handle_swap_retires_the_old_mesh() {
    let mut tracker = AoEligibilityTracker::<u32, u32>::default();
    tracker.discover(1, 10, true);
    tracker.resolve_pending(10);

    tracker.discover(1, 11, true);

    assert!(!tracker.is_eligible(10));
    assert!(tracker.is_eligible(11));
    assert!(tracker.is_pending(11));
}

#[test]
fn reclassifying_a_tracked_entity_as_ineligible_retires_its_mesh() {
    let mut tracker = AoEligibilityTracker::<u32, u32>::default();
    tracker.discover(1, 10, true);
    tracker.discover(1, 10, false);
    assert!(!tracker.is_eligible(10));
    assert!(!tracker.has_pending());
}

#[test]
fn a_reloaded_mesh_is_requeued_only_while_still_referenced() {
    let mut tracker = AoEligibilityTracker::<u32, u32>::default();
    tracker.asset_added(10);
    assert!(!tracker.has_pending());

    tracker.discover(1, 10, true);
    tracker.resolve_pending(10);

    tracker.asset_added(10);
    assert!(tracker.is_pending(10));

    tracker.resolve_pending(10);
    tracker.release(1);
    tracker.asset_added(10);
    assert!(!tracker.has_pending());
}
