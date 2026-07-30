use super::*;

fn key(base_form_id: u32, condition: Option<u32>) -> StackKey {
    StackKey {
        base_form_id,
        condition,
    }
}

#[test]
fn equipping_apparel_claims_every_slot_in_its_mask() {
    let mut state = EquipmentState::default();
    let helmet = key(0x1001, Some(100));
    state
        .equip(
            helmet,
            EquipKind::Apparel {
                biped_slot_mask: 0x5,
            },
        )
        .unwrap();
    assert!(state.is_equipped(helmet));
    assert_eq!(
        state.equipped_apparel().collect::<Vec<_>>(),
        vec![(BipedSlot::Head, helmet), (BipedSlot::UpperBody, helmet)]
    );
}

#[test]
fn equipping_into_an_occupied_slot_evicts_the_previous_occupant() {
    let mut state = EquipmentState::default();
    let hat = key(0x1001, Some(100));
    let helmet = key(0x2002, Some(100));
    state
        .equip(
            hat,
            EquipKind::Apparel {
                biped_slot_mask: 0x1,
            },
        )
        .unwrap();
    let outcome = state
        .equip(
            helmet,
            EquipKind::Apparel {
                biped_slot_mask: 0x1,
            },
        )
        .unwrap();
    assert_eq!(outcome.evicted, vec![hat]);
    assert!(!state.is_equipped(hat));
    assert!(state.is_equipped(helmet));
}

#[test]
fn apparel_with_an_empty_mask_cannot_be_equipped() {
    let mut state = EquipmentState::default();
    let junk = key(0x3003, None);
    assert_eq!(
        state.equip(junk, EquipKind::Apparel { biped_slot_mask: 0 }),
        Err(EquipError::NotEquippable)
    );
}

#[test]
fn a_different_condition_of_the_same_base_item_is_a_distinct_identity() {
    let mut state = EquipmentState::default();
    let pristine = key(0x1001, Some(100));
    let worn = key(0x1001, Some(50));
    state
        .equip(
            pristine,
            EquipKind::Apparel {
                biped_slot_mask: 0x1,
            },
        )
        .unwrap();
    let outcome = state
        .equip(
            worn,
            EquipKind::Apparel {
                biped_slot_mask: 0x1,
            },
        )
        .unwrap();
    assert_eq!(outcome.evicted, vec![pristine]);
    assert!(!state.is_equipped(pristine));
    assert!(state.is_equipped(worn));
}

#[test]
fn ammo_matching_the_equipped_weapon_can_be_loaded() {
    let mut state = EquipmentState::default();
    let rifle = key(0x3003, Some(100));
    let round = key(0xa, None);
    state
        .equip(
            rifle,
            EquipKind::Weapon {
                ammo_form_id: Some(0xa),
            },
        )
        .unwrap();
    state.equip(round, EquipKind::Ammo).unwrap();
    assert!(state.is_equipped(round));
}

#[test]
fn ammo_not_matching_the_equipped_weapon_is_rejected() {
    let mut state = EquipmentState::default();
    let rifle = key(0x3003, Some(100));
    let wrong_round = key(0xb, None);
    state
        .equip(
            rifle,
            EquipKind::Weapon {
                ammo_form_id: Some(0xa),
            },
        )
        .unwrap();
    assert_eq!(
        state.equip(wrong_round, EquipKind::Ammo),
        Err(EquipError::IncompatibleAmmo)
    );
}

#[test]
fn ammo_with_no_weapon_equipped_is_rejected() {
    let mut state = EquipmentState::default();
    let round = key(0xa, None);
    assert_eq!(
        state.equip(round, EquipKind::Ammo),
        Err(EquipError::NoWeaponEquipped)
    );
}

#[test]
fn switching_weapons_unequips_incompatible_ammo() {
    let mut state = EquipmentState::default();
    let rifle = key(0x3003, Some(100));
    let pistol = key(0x4004, Some(100));
    let round = key(0xa, None);
    state
        .equip(
            rifle,
            EquipKind::Weapon {
                ammo_form_id: Some(0xa),
            },
        )
        .unwrap();
    state.equip(round, EquipKind::Ammo).unwrap();
    let outcome = state
        .equip(
            pistol,
            EquipKind::Weapon {
                ammo_form_id: Some(0xb),
            },
        )
        .unwrap();
    // Both the previous weapon and its now-incompatible ammo come back
    // to carried inventory.
    assert_eq!(outcome.evicted, vec![rifle, round]);
    assert!(!state.is_equipped(rifle));
    assert!(!state.is_equipped(round));
}

#[test]
fn switching_weapons_keeps_still_compatible_ammo() {
    let mut state = EquipmentState::default();
    let rifle = key(0x3003, Some(100));
    let rifle_variant = key(0x3004, Some(100));
    let round = key(0xa, None);
    state
        .equip(
            rifle,
            EquipKind::Weapon {
                ammo_form_id: Some(0xa),
            },
        )
        .unwrap();
    state.equip(round, EquipKind::Ammo).unwrap();
    let outcome = state
        .equip(
            rifle_variant,
            EquipKind::Weapon {
                ammo_form_id: Some(0xa),
            },
        )
        .unwrap();
    assert_eq!(outcome.evicted, vec![rifle]);
    assert!(state.is_equipped(round));
}

#[test]
fn toggle_unequips_an_already_equipped_stack() {
    let mut state = EquipmentState::default();
    let hat = key(0x1001, Some(100));
    state
        .toggle(
            hat,
            EquipKind::Apparel {
                biped_slot_mask: 0x1,
            },
        )
        .unwrap();
    assert!(state.is_equipped(hat));
    let outcome = state
        .toggle(
            hat,
            EquipKind::Apparel {
                biped_slot_mask: 0x1,
            },
        )
        .unwrap();
    assert_eq!(outcome.evicted, vec![hat]);
    assert!(!state.is_equipped(hat));
}

#[test]
fn restore_rebuilds_equip_state_directly() {
    let hat = key(0x1001, Some(100));
    let rifle = key(0x3003, Some(100));
    let round = key(0xa, None);
    let state = EquipmentState::restore(
        [(BipedSlot::Hat, hat)],
        Some((rifle, Some(0xa))),
        Some(round),
    );
    assert!(state.is_equipped(hat));
    assert_eq!(state.equipped_weapon(), Some(rifle));
    assert_eq!(state.equipped_ammo(), Some(round));
}
