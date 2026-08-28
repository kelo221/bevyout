use super::*;
use crate::item_transaction::ItemState;
use crate::perception::{TargetClass, TargetId};

fn player() -> TargetId {
    TargetId::player()
}

fn actor(form_id: u32) -> TargetId {
    TargetId {
        class: TargetClass::Actor,
        form_id,
    }
}

fn eligible(form_id: u32) -> WitnessEvidence {
    WitnessEvidence {
        witness: actor(form_id),
        has_line_of_sight: true,
        distance_mm: 5_000,
        alive: true,
        enabled: true,
        hostile_to_victim: false,
    }
}

#[test]
fn unwitnessed_theft_marks_stolen_without_report() {
    let mut ledger = CrimeLedger::default();
    let mut item = ItemState::default();
    let id = CrimeLedger::allocate(player(), &mut ledger);
    let report = resolve_crime(
        &mut ledger,
        CrimeEvent {
            id,
            kind: CrimeKind::Theft,
            victim: actor(0x1A2B3),
            item_id: None,
            owner_form_id: Some(0x1A2B3),
        },
        &mut [],
        Some(&mut item),
    );
    assert!(report.is_none());
    assert!(item.ownership.stolen);
    assert_eq!(item.ownership.origin_owner_form_id, Some(0x1A2B3));
    assert_eq!(ledger.bounty, 0);
    assert_eq!(ledger.karma, 0);
}

#[test]
fn two_witnesses_do_not_double_bounty_and_replay_is_noop() {
    let mut ledger = CrimeLedger::default();
    let id = CrimeLedger::allocate(player(), &mut ledger);
    let mut witnesses = [eligible(0x41601), eligible(0x41600)];
    let event = CrimeEvent {
        id,
        kind: CrimeKind::Theft,
        victim: actor(0x1A2B3),
        item_id: None,
        owner_form_id: Some(0x1A2B3),
    };
    let report = resolve_crime(&mut ledger, event, &mut witnesses, None).unwrap();
    assert_eq!(report.witnesses, vec![actor(0x41600), actor(0x41601)]);
    assert_eq!(ledger.bounty, THEFT_BOUNTY);
    assert_eq!(ledger.karma, THEFT_KARMA);
    assert!(resolve_crime(&mut ledger, event, &mut witnesses, None).is_none());
    assert_eq!(ledger.bounty, THEFT_BOUNTY);
}

#[test]
fn ineligible_witnesses_are_rejected() {
    let mut ledger = CrimeLedger::default();
    let id = CrimeLedger::allocate(player(), &mut ledger);
    let mut witnesses = [
        WitnessEvidence {
            has_line_of_sight: false,
            ..eligible(1)
        },
        WitnessEvidence {
            distance_mm: CRIME_ALARM_RANGE_MM + 1,
            ..eligible(2)
        },
        WitnessEvidence {
            alive: false,
            ..eligible(3)
        },
        WitnessEvidence {
            enabled: false,
            ..eligible(4)
        },
        WitnessEvidence {
            hostile_to_victim: true,
            ..eligible(5)
        },
    ];
    assert!(
        resolve_crime(
            &mut ledger,
            CrimeEvent {
                id,
                kind: CrimeKind::Theft,
                victim: actor(0x1A2B3),
                item_id: None,
                owner_form_id: Some(0x1A2B3),
            },
            &mut witnesses,
            None,
        )
        .is_none()
    );
}

#[test]
fn murder_escalation_does_not_bill_assault() {
    let mut ledger = CrimeLedger::default();
    let id = CrimeLedger::allocate(player(), &mut ledger);
    let assault = CrimeEvent {
        id,
        kind: CrimeKind::Assault,
        victim: actor(0x1A2B3),
        item_id: None,
        owner_form_id: Some(0x1A2B3),
    };
    let mut witnesses = [eligible(0x41600)];
    let report = escalate_assault_to_murder(&mut ledger, assault, &mut witnesses).unwrap();
    assert_eq!(report.kind, CrimeKind::Murder);
    assert_eq!(ledger.bounty, MURDER_BOUNTY);
    assert_eq!(ledger.karma, MURDER_KARMA);
    assert!(!ledger.reported.is_empty());
}
