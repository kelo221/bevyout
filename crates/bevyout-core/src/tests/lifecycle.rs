use crate::barter::MERCHANT_RESTOCK_INTERVAL_MS;
use crate::effects::{ActiveEffect, EffectSource};
use crate::item_transaction::{HolderId, ItemHolderState, ItemLedger};
use crate::lifecycle::{
    FastTravelEvidence, LifecycleKind, LifecycleTask, LifecycleWorld, player_owned_item,
    unowned_item,
};
use crate::time::{GameTime, TimeAdvanceReason};

fn world() -> LifecycleWorld {
    let mut world = LifecycleWorld::default();
    world.schedule_defaults();
    world
}

#[test]
fn restock_boundary_is_exclusive_then_inclusive() {
    let mut world = world();
    world.schedule_restock(1, MERCHANT_RESTOCK_INTERVAL_MS);
    world
        .advance(
            MERCHANT_RESTOCK_INTERVAL_MS - 1,
            TimeAdvanceReason::Wait,
            None,
        )
        .unwrap();
    assert_eq!(world.restocks[&1].generation, 0);
    world.advance(1, TimeAdvanceReason::Wait, None).unwrap();
    assert_eq!(world.restocks[&1].generation, 1);
}

#[test]
fn large_jump_processes_two_restock_deadlines() {
    let mut world = world();
    world.schedule_restock(1, MERCHANT_RESTOCK_INTERVAL_MS);
    world
        .advance(
            MERCHANT_RESTOCK_INTERVAL_MS * 2,
            TimeAdvanceReason::Wait,
            None,
        )
        .unwrap();
    assert_eq!(world.restocks[&1].generation, 2);
}

#[test]
fn same_timestamp_sorts_by_kind_then_owner() {
    let mut world = world();
    world.schedule_restock(2, 1_000);
    world.schedule_restock(1, 1_000);
    world.scheduler.schedule(LifecycleTask {
        kind: LifecycleKind::CellReset,
        owner: 9,
        due_game_ms: 1_000,
    });
    world.advance(1_000, TimeAdvanceReason::Wait, None).unwrap();
    let restock_then_reset: Vec<_> = world
        .last_owners
        .iter()
        .copied()
        .zip(world.last_kinds.iter().copied())
        .filter(|(_, kind)| matches!(kind, LifecycleKind::Restock | LifecycleKind::CellReset))
        .collect();
    assert_eq!(
        restock_then_reset,
        vec![
            (1, LifecycleKind::Restock),
            (2, LifecycleKind::Restock),
            (9, LifecycleKind::CellReset),
        ]
    );
}

#[test]
fn occupied_cells_do_not_reset() {
    let mut world = world();
    world.register_cell(0x0001_51e3, true);
    world
        .advance(MERCHANT_RESTOCK_INTERVAL_MS, TimeAdvanceReason::Wait, None)
        .unwrap();
    assert_eq!(world.cells[&0x0001_51e3].reset_generation, 0);
    world.vacate_cell(0x0001_51e3, world.clock.absolute_game_ms);
    world
        .advance(MERCHANT_RESTOCK_INTERVAL_MS, TimeAdvanceReason::Wait, None)
        .unwrap();
    assert_eq!(world.cells[&0x0001_51e3].reset_generation, 1);
}

#[test]
fn unique_and_player_owned_holders_survive_reset() {
    let mut world = world();
    world.register_cell(0x0001_51e3, false);
    {
        let cell = world.cells.get_mut(&0x0001_51e3).unwrap();
        cell.unique_refs.insert(0x10);
        cell.containers.insert(0x20);
        cell.containers.insert(0x21);
        cell.actors.insert(0x30);
        cell.actors.insert(0x31);
        cell.unique_actors.insert(0x31);
        cell.corpses.insert(0x40);
    }
    let mut ledger = ItemLedger::new();
    ledger.holders_mut().insert(
        HolderId::FixtureContainer {
            reference_form_id: 0x20,
        },
        ItemHolderState {
            items: vec![player_owned_item(1, 0x51)],
            caps: 0,
            revision: 1,
        },
    );
    ledger.holders_mut().insert(
        HolderId::FixtureContainer {
            reference_form_id: 0x21,
        },
        ItemHolderState {
            items: vec![unowned_item(2, 0x52)],
            caps: 0,
            revision: 1,
        },
    );
    ledger.holders_mut().insert(
        HolderId::Corpse {
            actor_reference_form_id: 0x40,
        },
        ItemHolderState::default(),
    );
    world
        .advance(
            MERCHANT_RESTOCK_INTERVAL_MS,
            TimeAdvanceReason::Wait,
            Some(&mut ledger),
        )
        .unwrap();
    assert_eq!(world.cells[&0x0001_51e3].reset_generation, 1);
    assert_eq!(
        ledger.holders()[&HolderId::FixtureContainer {
            reference_form_id: 0x20
        }]
            .items
            .len(),
        1
    );
    assert_eq!(
        ledger.holders()[&HolderId::FixtureContainer {
            reference_form_id: 0x21
        }]
            .items
            .len(),
        1,
        "unowned containers keep contents until reset templates exist"
    );
    assert!(!ledger.holders().contains_key(&HolderId::Corpse {
        actor_reference_form_id: 0x40
    }));
    assert!(
        world
            .apply_cell_reset(0x0001_51e3, MERCHANT_RESTOCK_INTERVAL_MS, Some(&mut ledger))
            .is_err()
    );
}

#[test]
fn future_due_reset_is_not_due() {
    let mut world = world();
    world.register_cell(0x0001_51e3, false);
    let due = world.cells[&0x0001_51e3]
        .reset_due_game_ms
        .expect("vacant cells schedule a due");
    assert!(due > world.clock.absolute_game_ms);
    assert_eq!(
        world.apply_cell_reset(0x0001_51e3, due, None),
        Err(crate::lifecycle::CellResetError::NotDue)
    );
}

#[test]
fn encounter_zone_locks_once() {
    let mut world = world();
    let first = world.enter_encounter_zone(0x0002_a4a0, 6, 2, 10);
    let second = world.enter_encounter_zone(0x0002_a4a0, 10, 2, 10);
    assert_eq!(first.locked_level, 6);
    assert_eq!(second.locked_level, 6);
}

#[test]
fn fast_travel_commit_advances_effects_and_arrival() {
    let mut world = world();
    world.effects.apply(ActiveEffect {
        source: EffectSource::Chem,
        actor_value: crate::actor_state::ActorValue::ActionPoints,
        magnitude: 1.0,
        remaining_ms: 1_000,
    });
    world.schedule_restock(1, 1_000);
    let commit = world
        .commit_fast_travel(
            FastTravelEvidence {
                destination_cell: 0x0001_a000,
                travel_ms: 3_600_000,
                discovered: true,
                danger: false,
                interior: false,
                encumbered: false,
                combat: false,
                radiation: false,
            },
            None,
        )
        .unwrap();
    assert_eq!(world.clock.now(), GameTime::from_ms(3_600_000));
    assert!(world.effects.is_empty());
    assert_eq!(world.restocks[&1].generation, 1);
    assert_eq!(commit.destination_cell, 0x0001_a000);
    assert!(commit.arrival_requested);
}

#[test]
fn fast_travel_blocks_are_independent() {
    let mut world = world();
    let base = FastTravelEvidence {
        destination_cell: 0x0001_a000,
        travel_ms: 1,
        discovered: true,
        danger: false,
        interior: false,
        encumbered: false,
        combat: false,
        radiation: false,
    };
    assert!(
        world
            .commit_fast_travel(
                FastTravelEvidence {
                    danger: true,
                    ..base
                },
                None
            )
            .is_err()
    );
    assert!(
        world
            .commit_fast_travel(
                FastTravelEvidence {
                    interior: true,
                    ..base
                },
                None
            )
            .is_err()
    );
    assert!(
        world
            .commit_fast_travel(
                FastTravelEvidence {
                    encumbered: true,
                    ..base
                },
                None
            )
            .is_err()
    );
    assert!(
        world
            .commit_fast_travel(
                FastTravelEvidence {
                    combat: true,
                    ..base
                },
                None
            )
            .is_err()
    );
    assert!(
        world
            .commit_fast_travel(
                FastTravelEvidence {
                    radiation: true,
                    ..base
                },
                None
            )
            .is_err()
    );
    assert!(
        world
            .commit_fast_travel(
                FastTravelEvidence {
                    discovered: false,
                    ..base
                },
                None
            )
            .is_err()
    );
}

#[test]
fn sleep_restores_owned_bed_limbs() {
    let mut world = world();
    world
        .limbs
        .part_mut(crate::combat::BodyPartId::Head)
        .current_milli = 0;
    world
        .limbs
        .part_mut(crate::combat::BodyPartId::Head)
        .crippled = true;
    world
        .advance(1_000, TimeAdvanceReason::Sleep, None)
        .unwrap();
    assert!(!world.limbs.part(crate::combat::BodyPartId::Head).crippled);
    assert!(
        world
            .limbs
            .part(crate::combat::BodyPartId::Head)
            .current_milli
            > 0
    );
}
