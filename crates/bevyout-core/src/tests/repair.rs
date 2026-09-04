use super::*;
use crate::item_transaction::{
    HolderId, ItemHolderState, ItemInstance, ItemInstanceId, ItemLedger, ItemState, TransactionId,
};

fn holder(items: Vec<ItemInstance>, caps: u64) -> ItemHolderState {
    ItemHolderState {
        items,
        caps,
        revision: 0,
    }
}

fn item(id: u64, form: u32, count: u32, condition: Option<u32>) -> ItemInstance {
    ItemInstance::new(
        ItemInstanceId(id),
        form,
        count,
        ItemState {
            condition,
            ..Default::default()
        },
    )
    .unwrap()
}

fn request(
    ledger: &ItemLedger,
    target: u64,
    donor: u64,
    skill: u8,
    max_condition: u32,
) -> RepairRequest {
    RepairRequest {
        transaction_id: ledger.next_transaction_id(),
        holder: HolderId::Player,
        target: ItemInstanceId(target),
        donor: ItemInstanceId(donor),
        repair_skill: skill,
        max_condition,
        expected_holder_revision: ledger.holders()[&HolderId::Player].revision,
    }
}

#[test]
fn repair_combines_two_items_and_consumes_one_donor() {
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(
            HolderId::Player,
            holder(
                vec![item(1, 0x10, 1, Some(40)), item(2, 0x10, 3, Some(40))],
                0,
            ),
        )
        .unwrap();
    let request = request(&ledger, 1, 2, 50, 100);
    let receipt = repair(&mut ledger, request).unwrap();
    assert_eq!(receipt.condition_after, 50);
    assert_eq!(receipt.donor_consumed, 1);
    assert_eq!(
        ledger.holders()[&HolderId::Player]
            .find(ItemInstanceId(1))
            .unwrap()
            .state
            .condition,
        Some(50)
    );
    assert_eq!(
        ledger.holders()[&HolderId::Player]
            .find(ItemInstanceId(2))
            .unwrap()
            .count,
        2
    );
}

#[test]
fn repair_never_exceeds_the_skill_cap() {
    assert_eq!(planned_condition(10, 10, 100, 100), 45);
    assert_eq!(repair_cap(100, 100), 100);
    assert_eq!(repair_cap(100, 40), 50);
}

#[test]
fn repair_rejects_same_item_and_equipped_donor() {
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(
            HolderId::Player,
            holder(
                vec![item(1, 0x10, 2, Some(40)), item(2, 0x10, 1, Some(40))],
                0,
            ),
        )
        .unwrap();
    let same = request(&ledger, 1, 1, 50, 100);
    assert_eq!(
        repair(&mut ledger, same).unwrap_err(),
        RepairError::SameItem
    );
    ledger.equip(HolderId::Player, ItemInstanceId(2)).unwrap();
    let equipped = request(&ledger, 1, 2, 50, 100);
    assert_eq!(
        repair(&mut ledger, equipped).unwrap_err(),
        RepairError::EquippedDonor
    );
    assert_eq!(
        ledger.holders()[&HolderId::Player]
            .find(ItemInstanceId(2))
            .unwrap()
            .count,
        1
    );
}

#[test]
fn repair_replay_does_not_consume_another_donor() {
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(
            HolderId::Player,
            holder(
                vec![item(1, 0x10, 1, Some(40)), item(2, 0x10, 3, Some(40))],
                0,
            ),
        )
        .unwrap();
    let mut first = request(&ledger, 1, 2, 50, 100);
    first.transaction_id = TransactionId(7);
    let once = repair(&mut ledger, first.clone()).unwrap();
    let twice = repair(&mut ledger, first).unwrap();
    assert_eq!(once, twice);
    assert_eq!(
        ledger.holders()[&HolderId::Player]
            .find(ItemInstanceId(2))
            .unwrap()
            .count,
        2
    );
}
