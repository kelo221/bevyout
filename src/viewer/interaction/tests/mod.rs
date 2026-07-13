use super::*;

#[test]
fn inventory_accumulates_whole_stacks() {
    let mut inventory = PlayerInventory::default();
    inventory.add(0x1234, 3);
    inventory.add(0x1234, 2);
    assert_eq!(inventory.count(0x1234), 5);
    assert!(inventory.contains(0x1234));
}

#[test]
fn locked_door_requires_its_key() {
    let door = PreparedDoor {
        lock_level: Some(50),
        key_form_id: Some(0x42),
        destination: None,
    };
    let mut inventory = PlayerInventory::default();
    assert!(door_is_locked(&door, &inventory));
    inventory.add(0x42, 1);
    assert!(!door_is_locked(&door, &inventory));
}

#[test]
fn lock_without_a_key_remains_locked() {
    let door = PreparedDoor {
        lock_level: Some(1),
        key_form_id: None,
        destination: None,
    };
    assert!(door_is_locked(&door, &PlayerInventory::default()));
}

#[test]
fn zero_lock_level_is_unlocked() {
    let door = PreparedDoor {
        lock_level: Some(0),
        key_form_id: Some(0x42),
        destination: None,
    };
    assert!(!door_is_locked(&door, &PlayerInventory::default()));
}

#[test]
fn container_summary_is_bounded() {
    let entries = (0..10)
        .map(|index| PreparedInventoryEntry {
            base_form_id: index,
            count: 1,
            record_kind: "MISC".into(),
            editor_id: Some(format!("Item{index}")),
            display_name: None,
            leveled: false,
        })
        .collect::<Vec<_>>();
    let summary = inventory_summary(&entries);
    assert!(summary.contains("Item0 x1"));
    assert!(summary.contains("+2 more"));
    assert!(!summary.contains("Item8"));
}
