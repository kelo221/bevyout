use super::*;

#[test]
fn identical_player_projection_does_not_churn_holder_revision() {
    let inventory = PlayerInventory::from_stacks([(0x4241, 24), (0x434f, 1)]);
    let mut canonical = CanonicalItemLedger::default();
    canonical.sync_player(&inventory.legacy_snapshot()).unwrap();
    let before = canonical.ledger.holders()[&HolderId::Player].revision;

    canonical.sync_player(&inventory.legacy_snapshot()).unwrap();

    assert_eq!(
        canonical.ledger.holders()[&HolderId::Player].revision,
        before
    );
}
