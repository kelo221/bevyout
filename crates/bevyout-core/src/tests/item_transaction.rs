use super::*;
use crate::combat::{BASIS_POINT_DENOMINATOR, MAX_JAM_CHANCE_BASIS_POINTS};
use proptest::prelude::*;

fn state(condition: Option<u32>) -> ItemState {
    ItemState {
        condition,
        ..Default::default()
    }
}

fn holder(items: Vec<ItemInstance>, caps: u64) -> ItemHolderState {
    ItemHolderState {
        items,
        caps,
        revision: 0,
    }
}

fn item(id: u64, form: u32, count: u32, condition: Option<u32>) -> ItemInstance {
    ItemInstance::new(ItemInstanceId(id), form, count, state(condition)).unwrap()
}

#[test]
fn atomic_reload_consumes_missing_rounds_and_is_idempotent() {
    let mut weapon = item(1, 0x434f, 1, Some(100));
    weapon.state.combat.magazine.ammo_form_id = Some(0x4241);
    weapon.state.combat.magazine.loaded = 7;
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(
            HolderId::Player,
            holder(vec![weapon, item(2, 0x4241, 20, None)], 0),
        )
        .unwrap();
    ledger.equip(HolderId::Player, ItemInstanceId(1)).unwrap();

    let first = ledger
        .reload_weapon_with_id(
            TransactionId(10),
            HolderId::Player,
            ItemInstanceId(1),
            0x4241,
            12,
        )
        .unwrap();
    let second = ledger
        .reload_weapon_with_id(
            TransactionId(10),
            HolderId::Player,
            ItemInstanceId(1),
            0x4241,
            12,
        )
        .unwrap();

    assert_eq!(first, second);
    assert_eq!(first.consumed, 5);
    assert_eq!(first.loaded, 12);
    assert_eq!(ledger.holders()[&HolderId::Player].items[1].count, 15);
}

#[test]
fn failed_ammo_switch_rolls_back_weapon_and_reserve() {
    let mut weapon = item(1, 0x434f, 1, Some(100));
    weapon.state.combat.magazine.ammo_form_id = Some(0x4241);
    weapon.state.combat.magazine.loaded = 7;
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(
            HolderId::Player,
            holder(vec![weapon, item(2, 0x4241, 20, None)], 0),
        )
        .unwrap();
    let before = ledger.snapshot();

    assert_eq!(
        ledger.reload_weapon_with_id(
            TransactionId(11),
            HolderId::Player,
            ItemInstanceId(1),
            0x9999,
            12,
        ),
        Err(TransactionError::InsufficientItems)
    );
    assert_eq!(ledger.snapshot(), before);
}

#[test]
fn mutable_weapon_stack_is_split_and_active_binding_is_remapped() {
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(
            HolderId::Player,
            holder(
                vec![item(1, 0x434f, 2, Some(100)), item(2, 0x4241, 12, None)],
                0,
            ),
        )
        .unwrap();
    ledger.equip(HolderId::Player, ItemInstanceId(1)).unwrap();

    let receipt = ledger
        .reload_weapon_with_id(
            TransactionId(12),
            HolderId::Player,
            ItemInstanceId(1),
            0x4241,
            12,
        )
        .unwrap();

    assert_ne!(receipt.weapon_id, ItemInstanceId(1));
    assert_eq!(
        ledger.bindings()[&HolderId::Player].equipped,
        Some(receipt.weapon_id)
    );
    assert_eq!(
        ledger.holders()[&HolderId::Player]
            .find(ItemInstanceId(1))
            .unwrap()
            .count,
        1
    );
}

#[test]
fn actor_holder_identity_is_the_stable_reference_form_id() {
    let holder = HolderId::Actor {
        reference_form_id: 0x0004_1600,
    };
    let reconstructed = HolderId::Actor {
        reference_form_id: 0x0004_1600,
    };
    let other_placement = HolderId::Actor {
        reference_form_id: 0x0004_1601,
    };

    assert_eq!(holder, reconstructed);
    assert_ne!(holder, other_placement);

    let encoded = ron::to_string(&holder).unwrap();
    assert_eq!(ron::from_str::<HolderId>(&encoded).unwrap(), holder);
}

#[test]
fn canonical_transactions_can_transfer_and_equip_an_actor_item() {
    let actor = HolderId::Actor {
        reference_form_id: 0x0004_1600,
    };
    let weapon_id = ItemInstanceId(7);
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(
            HolderId::Player,
            holder(vec![item(7, 0x0000_4322, 1, None)], 0),
        )
        .unwrap();
    ledger.insert_holder(actor, holder(vec![], 0)).unwrap();

    ledger
        .execute(TransactionRequest::Transfer {
            source: HolderId::Player,
            destination: actor,
            item_id: weapon_id,
            count: 1,
        })
        .unwrap();
    ledger.equip(actor, weapon_id).unwrap();

    assert!(ledger.holders()[&HolderId::Player].items.is_empty());
    assert_eq!(ledger.holders()[&actor].items[0].base_form_id, 0x0000_4322);
    assert_eq!(ledger.bindings()[&actor].equipped, Some(weapon_id));
}

#[test]
fn partial_transfer_allocates_a_new_destination_id_and_conserves_quantity() {
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![item(7, 1, 5, Some(80))], 0))
        .unwrap();
    ledger
        .insert_holder(
            HolderId::FixtureContainer {
                reference_form_id: 9,
            },
            holder(vec![], 0),
        )
        .unwrap();
    let receipt = ledger
        .execute(TransactionRequest::Transfer {
            source: HolderId::Player,
            destination: HolderId::FixtureContainer {
                reference_form_id: 9,
            },
            item_id: ItemInstanceId(7),
            count: 2,
        })
        .unwrap();
    assert_eq!(receipt.moved, vec![(ItemInstanceId(8), 2)]);
    assert_eq!(
        ledger.holders()[&HolderId::Player].items[0].id,
        ItemInstanceId(7)
    );
    assert_eq!(ledger.holders()[&HolderId::Player].items[0].count, 3);
    assert_eq!(
        ledger.holders()[&HolderId::FixtureContainer {
            reference_form_id: 9
        }]
            .items[0]
            .count,
        2
    );
}

#[test]
fn full_transfer_prunes_source_hotkeys() {
    let container = HolderId::FixtureContainer {
        reference_form_id: 9,
    };
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![item(7, 1, 1, None)], 0))
        .unwrap();
    ledger.insert_holder(container, holder(vec![], 0)).unwrap();
    ledger
        .bind_hotkey(HolderId::Player, 0, ItemInstanceId(7))
        .unwrap();

    ledger
        .execute(TransactionRequest::Transfer {
            source: HolderId::Player,
            destination: container,
            item_id: ItemInstanceId(7),
            count: 1,
        })
        .unwrap();

    assert_eq!(ledger.bindings()[&HolderId::Player].hotkeys[0], None);
    assert_eq!(ledger.bindings()[&container].hotkeys[0], None);
}

#[test]
fn partial_transfer_keeps_source_hotkey_and_leaves_destination_unbound() {
    let container = HolderId::FixtureContainer {
        reference_form_id: 9,
    };
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![item(7, 1, 5, None)], 0))
        .unwrap();
    ledger.insert_holder(container, holder(vec![], 0)).unwrap();
    ledger
        .bind_hotkey(HolderId::Player, 0, ItemInstanceId(7))
        .unwrap();

    let receipt = ledger
        .execute(TransactionRequest::Transfer {
            source: HolderId::Player,
            destination: container,
            item_id: ItemInstanceId(7),
            count: 2,
        })
        .unwrap();

    assert_eq!(
        ledger.bindings()[&HolderId::Player].hotkeys[0],
        Some(ItemInstanceId(7))
    );
    assert_eq!(ledger.bindings()[&container].hotkeys[0], None);
    assert_eq!(receipt.moved, vec![(ItemInstanceId(8), 2)]);
}

#[test]
fn compatible_merge_keeps_lowest_id_and_reports_remap() {
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![item(2, 1, 1, None)], 0))
        .unwrap();
    ledger
        .insert_holder(
            HolderId::FixtureContainer {
                reference_form_id: 9,
            },
            holder(vec![item(4, 1, 3, None)], 0),
        )
        .unwrap();
    let receipt = ledger
        .execute(TransactionRequest::Transfer {
            source: HolderId::Player,
            destination: HolderId::FixtureContainer {
                reference_form_id: 9,
            },
            item_id: ItemInstanceId(2),
            count: 1,
        })
        .unwrap();
    assert_eq!(receipt.remaps, vec![(ItemInstanceId(4), ItemInstanceId(2))]);
    assert_eq!(
        ledger.holders()[&HolderId::FixtureContainer {
            reference_form_id: 9
        }]
            .items[0]
            .id,
        ItemInstanceId(2)
    );
    assert_eq!(
        ledger.holders()[&HolderId::FixtureContainer {
            reference_form_id: 9
        }]
            .items[0]
            .count,
        4
    );
}

#[test]
fn merge_remaps_destination_binding_and_prunes_source_binding() {
    let container = HolderId::FixtureContainer {
        reference_form_id: 9,
    };
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![item(2, 1, 1, None)], 0))
        .unwrap();
    ledger
        .insert_holder(container, holder(vec![item(4, 1, 3, None)], 0))
        .unwrap();
    ledger.bind_hotkey(container, 0, ItemInstanceId(4)).unwrap();

    let receipt = ledger
        .execute(TransactionRequest::Transfer {
            source: HolderId::Player,
            destination: container,
            item_id: ItemInstanceId(2),
            count: 1,
        })
        .unwrap();

    assert_eq!(receipt.remaps, vec![(ItemInstanceId(4), ItemInstanceId(2))]);
    assert_eq!(
        ledger.bindings()[&container].hotkeys[0],
        Some(ItemInstanceId(2))
    );
    assert_eq!(ledger.bindings()[&HolderId::Player].hotkeys[0], None);
}

#[test]
fn snapshot_rejects_cross_holder_binding() {
    let container = HolderId::FixtureContainer {
        reference_form_id: 9,
    };
    let mut snapshot = ItemLedgerSnapshot {
        next_item_id: ItemInstanceId(4),
        next_transaction_id: TransactionId(1),
        ..Default::default()
    };
    snapshot
        .holders
        .insert(HolderId::Player, holder(vec![item(2, 1, 1, None)], 0));
    snapshot
        .holders
        .insert(container, holder(vec![item(3, 1, 1, None)], 0));
    snapshot.bindings.insert(
        HolderId::Player,
        BindingState {
            hotkeys: [
                Some(ItemInstanceId(3)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            ..Default::default()
        },
    );

    assert!(matches!(
        ItemLedger::from_snapshot(snapshot),
        Err(TransactionError::InvalidBinding {
            holder: HolderId::Player,
            item_id: ItemInstanceId(3),
        })
    ));
}

#[test]
fn failed_transaction_leaves_both_holders_unchanged() {
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![item(2, 1, 1, None)], 0))
        .unwrap();
    ledger
        .insert_holder(
            HolderId::FixtureContainer {
                reference_form_id: 9,
            },
            holder(vec![], 0),
        )
        .unwrap();
    let before = ledger.clone();
    assert_eq!(
        ledger.execute(TransactionRequest::Transfer {
            source: HolderId::Player,
            destination: HolderId::FixtureContainer {
                reference_form_id: 9
            },
            item_id: ItemInstanceId(2),
            count: 2,
        }),
        Err(TransactionError::InsufficientItems)
    );
    assert_eq!(ledger.holders(), before.holders());
}

#[test]
fn duplicate_transaction_id_is_a_no_op_after_success() {
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![item(2, 1, 1, None)], 0))
        .unwrap();
    ledger
        .insert_holder(
            HolderId::FixtureContainer {
                reference_form_id: 9,
            },
            holder(vec![], 0),
        )
        .unwrap();
    let request = TransactionRequest::Transfer {
        source: HolderId::Player,
        destination: HolderId::FixtureContainer {
            reference_form_id: 9,
        },
        item_id: ItemInstanceId(2),
        count: 1,
    };
    let first = ledger
        .execute_with_id(TransactionId(44), request.clone())
        .unwrap();
    let second = ledger.execute_with_id(TransactionId(44), request).unwrap();
    assert_eq!(first, second);
    assert_eq!(
        ledger.holders()[&HolderId::FixtureContainer {
            reference_form_id: 9
        }]
            .items[0]
            .count,
        1
    );
}

#[test]
fn opaque_state_must_have_deterministic_unique_tags() {
    let mut state = state(None);
    state.extras = vec![
        ItemExtraEntry {
            namespace_form_id: 2,
            tag: *b"TEST",
            payload: vec![2],
        },
        ItemExtraEntry {
            namespace_form_id: 1,
            tag: *b"TEST",
            payload: vec![1],
        },
    ];
    assert_eq!(
        state.clone().normalized().unwrap().extras[0].namespace_form_id,
        1
    );
    state.extras.push(ItemExtraEntry {
        namespace_form_id: 1,
        tag: *b"TEST",
        payload: vec![3],
    });
    assert!(matches!(
        state.normalized(),
        Err(TransactionError::DuplicateExtraState { .. })
    ));
}

#[test]
fn static_merchant_buy_and_sell_are_atomic_and_use_unit_price() {
    let merchant = HolderId::FixtureMerchant {
        reference_form_id: 9,
    };
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![], 100))
        .unwrap();
    ledger
        .insert_holder(merchant, holder(vec![item(2, 1, 3, None)], 50))
        .unwrap();
    let buy = ledger
        .execute(TransactionRequest::Buy {
            merchant,
            player: HolderId::Player,
            item_id: ItemInstanceId(2),
            count: 2,
            unit_price: 10,
        })
        .unwrap();
    assert_eq!(buy.caps_delta[&HolderId::Player], -20);
    assert_eq!(ledger.holders()[&HolderId::Player].caps, 80);
    assert_eq!(ledger.holders()[&merchant].caps, 70);
    let bought_id = ledger.holders()[&HolderId::Player].items[0].id;
    let sell = ledger
        .execute(TransactionRequest::Sell {
            player: HolderId::Player,
            merchant,
            item_id: bought_id,
            count: 1,
            unit_price: 10,
        })
        .unwrap();
    assert_eq!(sell.caps_delta[&HolderId::Player], 10);
    assert_eq!(ledger.holders()[&HolderId::Player].caps, 90);
    assert_eq!(ledger.holders()[&merchant].caps, 60);
}

#[test]
fn fire_transaction_degrades_canonical_weapon_and_is_idempotent() {
    let mut weapon = item(1, 0x434f, 1, Some(100));
    weapon.state.combat.magazine.ammo_form_id = Some(0x4241);
    weapon.state.combat.magazine.loaded = 2;
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(
            HolderId::Player,
            holder(vec![weapon, item(2, 0x4241, 20, None)], 0),
        )
        .unwrap();
    let mut rng = CombatRngState::from_seed(42);
    let policy = WeaponConditionPolicy::with_degradation(Some(100), 5);

    let first = ledger
        .fire_weapon_with_policy(
            TransactionId(20),
            HolderId::Player,
            ItemInstanceId(1),
            10.0,
            policy,
            &mut rng,
        )
        .unwrap();
    let second = ledger
        .fire_weapon_with_policy(
            TransactionId(20),
            HolderId::Player,
            ItemInstanceId(1),
            10.0,
            policy,
            &mut rng,
        )
        .unwrap();

    assert_eq!(first, second);
    assert_eq!(first.outcome, CombatTransactionOutcome::Fired);
    assert_eq!(first.condition_before, Some(100));
    assert_eq!(first.condition_after, Some(95));
    assert_eq!(first.damage_milli, Some(10_000));
    assert_eq!(first.weapon_id, ItemInstanceId(1));
    assert_eq!(rng.draw_index, 1);
    let weapon = ledger.holders()[&HolderId::Player]
        .find(ItemInstanceId(1))
        .unwrap();
    assert_eq!(weapon.state.condition, Some(95));
    assert_eq!(weapon.state.combat.magazine.loaded, 1);
}

#[test]
fn rejected_jammed_fire_does_not_advance_rng_or_mutate_snapshot() {
    let mut weapon = item(1, 0x434f, 1, Some(50));
    weapon.state.combat.magazine.ammo_form_id = Some(0x4241);
    weapon.state.combat.magazine.loaded = 1;
    weapon.state.combat.jam = Some(JamReason::Fire);
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![weapon], 0))
        .unwrap();
    let before = ledger.snapshot();
    let mut rng = CombatRngState::from_seed(7);

    assert_eq!(
        ledger.fire_weapon_with_policy(
            TransactionId(21),
            HolderId::Player,
            ItemInstanceId(1),
            10.0,
            WeaponConditionPolicy::new(Some(100)),
            &mut rng,
        ),
        Err(TransactionError::Jammed(JamReason::Fire))
    );
    assert_eq!(ledger.snapshot(), before);
    assert_eq!(rng.draw_index, 0);
}

#[test]
fn reload_can_jam_and_clear_preserves_instance_identity() {
    let seed = (0..10_000u64)
        .find(|seed| {
            let mut rng = CombatRngState::from_seed(*seed);
            let draw = rng.draw(CombatRngDomain::ReloadJam).unwrap();
            draw.value % BASIS_POINT_DENOMINATOR < MAX_JAM_CHANCE_BASIS_POINTS
        })
        .expect("a deterministic seed must produce a reload jam draw");
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(
            HolderId::Player,
            holder(
                vec![item(1, 0x434f, 1, Some(0)), item(2, 0x4241, 12, None)],
                0,
            ),
        )
        .unwrap();
    let mut rng = CombatRngState::from_seed(seed);
    let jammed = ledger
        .reload_weapon_with_policy(
            TransactionId(22),
            HolderId::Player,
            ItemInstanceId(1),
            WeaponReloadRequest {
                ammo_form_id: 0x4241,
                capacity: 12,
                policy: WeaponConditionPolicy::new(Some(100)),
            },
            &mut rng,
        )
        .unwrap();
    assert_eq!(jammed.outcome, CombatTransactionOutcome::Jammed);
    assert_eq!(jammed.jam, Some(JamReason::Reload));
    assert_eq!(jammed.weapon_id, ItemInstanceId(1));
    assert_eq!(rng.draw_index, 1);

    let cleared = ledger
        .clear_weapon_jam_with_id(TransactionId(23), HolderId::Player, jammed.weapon_id)
        .unwrap();
    assert_eq!(cleared.outcome, CombatTransactionOutcome::Cleared);
    assert_eq!(cleared.weapon_id, jammed.weapon_id);
    let weapon = ledger.holders()[&HolderId::Player]
        .find(jammed.weapon_id)
        .unwrap();
    assert_eq!(weapon.state.combat.jam, None);
    assert_eq!(weapon.state.combat.magazine.loaded, 12);
}

proptest::proptest! {
    #[test]
    fn random_partial_transfers_conserve_quantity(source_count in 1u32..200, requested in 1u32..200) {
        let moved = requested.min(source_count);
        let mut ledger = ItemLedger::new();
        ledger.insert_holder(HolderId::Player, holder(vec![item(7, 1, source_count, Some(80))], 0)).unwrap();
        let container = HolderId::FixtureContainer { reference_form_id: 9 };
        ledger.insert_holder(container, holder(vec![], 0)).unwrap();
        let result = ledger.execute(TransactionRequest::Transfer {
            source: HolderId::Player,
            destination: container,
            item_id: ItemInstanceId(7),
            count: moved,
        });
        prop_assert!(result.is_ok());
        let player_count = ledger.holders()[&HolderId::Player].items.iter().map(|item| item.count).sum::<u32>();
        let container_count = ledger.holders()[&container].items.iter().map(|item| item.count).sum::<u32>();
        prop_assert_eq!(player_count + container_count, source_count);
    }
}
