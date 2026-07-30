use super::*;

fn door_marker(agent_id: u32, cell: u32, door: u32) -> LedgerEntry {
    LedgerEntry {
        agent_id,
        cell_form_id: cell,
        spawn_kind: SpawnKind::DoorMarker {
            destination_door_form_id: door,
        },
        remaining_target: None,
    }
}

fn frozen(agent_id: u32, cell: u32, position: [f32; 3]) -> LedgerEntry {
    LedgerEntry {
        agent_id,
        cell_form_id: cell,
        spawn_kind: SpawnKind::FrozenPosition { position },
        remaining_target: None,
    }
}

#[test]
fn recording_replaces_any_existing_entry_for_the_same_agent() {
    let mut ledger = Ledger::default();
    ledger.record(door_marker(1, 0x100, 0xD00));
    ledger.record(frozen(1, 0x200, [1.0, 2.0, 3.0]));
    assert_eq!(ledger.len(), 1);
    assert_eq!(ledger.entry_for(1), Some(frozen(1, 0x200, [1.0, 2.0, 3.0])));
}

#[test]
fn claim_for_activation_only_claims_the_matching_cell_in_agent_id_order() {
    let mut ledger = Ledger::default();
    ledger.record(door_marker(2, 0x100, 0xD00));
    ledger.record(frozen(1, 0x100, [1.0, 0.0, 0.0]));
    ledger.record(frozen(9, 0x200, [9.0, 0.0, 0.0]));

    let known = HashSet::from([0xD00]);
    let result = ledger.claim_for_activation(0x100, &known);

    assert_eq!(
        result.restored,
        vec![
            frozen(1, 0x100, [1.0, 0.0, 0.0]),
            door_marker(2, 0x100, 0xD00)
        ],
        "claimed entries must be ordered deterministically by agent_id"
    );
    assert!(result.stale.is_empty());
    // The other cell's entry stays ledgered, untouched.
    assert_eq!(ledger.len(), 1);
    assert_eq!(ledger.entry_for(9), Some(frozen(9, 0x200, [9.0, 0.0, 0.0])));
}

#[test]
fn claiming_a_cell_with_no_matching_entries_restores_nothing() {
    let mut ledger = Ledger::default();
    ledger.record(frozen(1, 0x100, [0.0, 0.0, 0.0]));
    let result = ledger.claim_for_activation(0x999, &HashSet::new());
    assert!(result.restored.is_empty());
    assert!(result.stale.is_empty());
    assert_eq!(ledger.len(), 1, "the unrelated entry must remain ledgered");
}

#[test]
fn a_door_marker_entry_missing_its_destination_door_is_diagnosed_as_stale() {
    let mut ledger = Ledger::default();
    ledger.record(door_marker(1, 0x100, 0xD00));
    let result = ledger.claim_for_activation(0x100, &HashSet::new());
    assert!(result.restored.is_empty());
    assert_eq!(
        result.stale,
        vec![StaleEntry {
            agent_id: 1,
            cell_form_id: 0x100,
            missing_door_form_id: 0xD00,
        }]
    );
    assert_eq!(
        ledger.len(),
        0,
        "a stale entry is still consumed, never retried"
    );
}

#[test]
fn a_frozen_position_entry_never_needs_a_known_door() {
    let mut ledger = Ledger::default();
    ledger.record(frozen(1, 0x100, [1.0, 2.0, 3.0]));
    let result = ledger.claim_for_activation(0x100, &HashSet::new());
    assert_eq!(result.restored, vec![frozen(1, 0x100, [1.0, 2.0, 3.0])]);
    assert!(result.stale.is_empty());
}

#[test]
fn follow_through_requires_the_exact_door_the_player_used() {
    assert_eq!(
        decide_swap_eligibility(Some(0xD00), 0xD00),
        SwapEligibility::FollowThrough
    );
}

#[test]
fn a_different_active_route_door_freezes() {
    assert_eq!(
        decide_swap_eligibility(Some(0xD50), 0xD00),
        SwapEligibility::Freeze
    );
}

#[test]
fn no_active_route_freezes() {
    assert_eq!(
        decide_swap_eligibility(None, 0xD00),
        SwapEligibility::Freeze
    );
}
