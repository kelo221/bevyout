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

#[test]
fn interaction_prompts_use_e_in_fps_mode() {
    let placement = PreparedPlacement {
        reference_form_id: 1,
        base_form_id: 2,
        asset_path: None,
        translation: [0.0; 3],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        error: None,
        physics_asset_path: None,
        physics_source: None,
        physics_classification: Default::default(),
        step_support: false,
        mutability: Default::default(),
        mutability_root_form_id: None,
        reference_kind: "CONT".into(),
        base_kind: "CONT".into(),
        editor_id: Some("TestContainer".into()),
        display_name: Some("Test Container".into()),
        count: 1,
        semantic: PreparedSemantic::Container,
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        inventory: Vec::new(),
        audio: Default::default(),
        ao_mode: "ao-none".into(),
    };

    let prompt = interaction_prompt(&placement, false, &PlayerInventory::default())
        .expect("containers should have an interaction prompt");
    assert!(prompt.starts_with("[E]"));
    assert!(!prompt.contains("Enter"));
}

#[test]
fn probe_status_distinguishes_reference_static_and_no_target() {
    assert_eq!(
        probe_status_message(true, Some("VaultDoorRef (0007b240)")),
        "probe: VaultDoorRef (0007b240)"
    );
    assert_eq!(
        probe_status_message(true, None),
        "probe: NOT_IMPLEMENTED (static-batched geometry)"
    );
    assert_eq!(probe_status_message(false, None), "probe: no target");
}
