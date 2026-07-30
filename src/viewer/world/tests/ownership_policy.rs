use super::*;

#[test]
fn release_returns_exactly_what_was_recorded_and_untracks_the_cell() {
    let mut ledger = CellColliderLedger::<u64, u64>::default();
    ledger.record_static_shape(0x100, 1);
    ledger.record_static_shape(0x100, 2);
    ledger.record_keyframed_body(0x100, 10);
    ledger.record_dynamic_body(0x100, 20);
    let released = ledger.release(0x100).expect("cell was tracked");
    assert_eq!(released.static_shapes, vec![1, 2]);
    assert_eq!(released.keyframed_bodies, vec![10]);
    assert_eq!(released.dynamic_bodies, vec![20]);
    assert_eq!(released.shape_count(), 2);
    assert_eq!(released.body_count(), 2);
    assert!(!ledger.is_tracked(0x100));
}

#[test]
fn releasing_an_untracked_cell_returns_none() {
    let mut ledger = CellColliderLedger::<u64, u64>::default();
    assert!(ledger.release(0x999).is_none());
}

#[test]
fn cells_are_independent() {
    let mut ledger = CellColliderLedger::<u64, u64>::default();
    ledger.record_static_shape(0x100, 1);
    ledger.record_static_shape(0x200, 2);
    let released = ledger.release(0x100).expect("cell was tracked");
    assert_eq!(released.static_shapes, vec![1]);
    assert!(ledger.is_tracked(0x200));
}

#[test]
fn re_recording_after_release_starts_fresh() {
    let mut ledger = CellColliderLedger::<u64, u64>::default();
    ledger.record_static_shape(0x100, 1);
    ledger.release(0x100);
    ledger.record_static_shape(0x100, 7);
    let released = ledger.release(0x100).expect("cell was re-tracked");
    assert_eq!(released.static_shapes, vec![7]);
}
