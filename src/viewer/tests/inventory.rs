use super::*;

#[test]
fn condition_is_part_of_stack_identity() {
    let mut inventory = Inventory::default();
    inventory.add(InventoryStack {
        base_form_id: 1,
        count: 2,
        condition: Some(100),
    });
    inventory.add(InventoryStack {
        base_form_id: 1,
        count: 1,
        condition: Some(80),
    });
    assert_eq!(inventory.stacks().len(), 2);
    assert_eq!(inventory.count(1), 3);
}

#[test]
fn failed_remove_is_atomic() {
    let key = StackKey {
        base_form_id: 1,
        condition: None,
    };
    let mut inventory = Inventory::from_stacks([InventoryStack {
        base_form_id: 1,
        count: 2,
        condition: None,
    }]);
    assert_eq!(inventory.remove(key, 3), TransferResult::InsufficientItems);
    assert_eq!(inventory.stack_count(key), 2);
}

#[test]
fn quantity_picker_is_reserved_for_stacks_above_three() {
    assert_eq!(drop_action(3), Some(DropAction::DropOne));
    assert_eq!(
        drop_action(4),
        Some(DropAction::ChooseQuantity {
            min: 1,
            max: 4,
            default: 1
        })
    );
}
