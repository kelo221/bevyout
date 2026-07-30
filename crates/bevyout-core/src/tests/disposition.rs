use super::*;
use crate::faction::{FactionRelation, PreparedFaction};

fn faction(form_id: u32, relations: Vec<FactionRelation>) -> PreparedFaction {
    PreparedFaction {
        form_id,
        editor_id: None,
        name: None,
        relations,
    }
}

fn table_with(factions: Vec<PreparedFaction>) -> FactionRelationTable {
    let mut table = FactionRelationTable::new();
    for f in factions {
        table.insert(f);
    }
    table
}

fn member(form_id: u32) -> FactionMembership {
    FactionMembership {
        faction_form_id: form_id,
        rank: 0,
    }
}

fn observer(factions: Vec<u32>, base: i32, aggression: Aggression) -> DispositionActor {
    DispositionActor {
        factions: factions.into_iter().map(member).collect(),
        base_disposition: base,
        aggression,
        race_disposition_adjust: 0,
    }
}

fn target(factions: Vec<u32>) -> DispositionTarget {
    DispositionTarget {
        factions: factions.into_iter().map(member).collect(),
        is_self: false,
    }
}

#[test]
fn self_target_is_always_friendly() {
    let result = resolve_disposition(
        &observer(vec![], 0, Aggression::Frenzied),
        &DispositionTarget {
            is_self: true,
            ..Default::default()
        },
        &FactionRelationTable::new(),
        &DispositionThresholds::default(),
    );
    assert_eq!(result.hostility, Hostility::Friendly);
    assert_eq!(result.decided_by, HostilityRule::SameActor);
}

#[test]
fn frenzied_is_hostile_to_an_ally() {
    // Even a faction ally is attacked by a frenzied actor.
    let table = table_with(vec![
        faction(
            0x10,
            vec![FactionRelation {
                faction_form_id: 0x20,
                modifier: 50,
                reaction: GroupCombatReaction::Ally,
            }],
        ),
        faction(0x20, vec![]),
    ]);
    let result = resolve_disposition(
        &observer(vec![0x10], 50, Aggression::Frenzied),
        &target(vec![0x20]),
        &table,
        &DispositionThresholds::default(),
    );
    assert_eq!(result.hostility, Hostility::Hostile);
    assert_eq!(result.decided_by, HostilityRule::Frenzied);
}

#[test]
fn faction_enemy_relation_forces_hostility_despite_high_disposition() {
    let table = table_with(vec![
        faction(
            0x10,
            vec![FactionRelation {
                faction_form_id: 0x20,
                modifier: -100,
                reaction: GroupCombatReaction::Enemy,
            }],
        ),
        faction(0x20, vec![]),
    ]);
    let result = resolve_disposition(
        &observer(vec![0x10], 100, Aggression::Unaggressive),
        &target(vec![0x20]),
        &table,
        &DispositionThresholds::default(),
    );
    assert_eq!(result.hostility, Hostility::Hostile);
    assert_eq!(result.decided_by, HostilityRule::FactionEnemy);
}

#[test]
fn shared_faction_is_friendly_and_outranks_an_enemy_relation() {
    // Observer and target both in 0x10; 0x10 also lists an enemy relation
    // to a faction only the observer holds. Shared membership wins.
    let table = table_with(vec![
        faction(
            0x10,
            vec![FactionRelation {
                faction_form_id: 0x99,
                modifier: -80,
                reaction: GroupCombatReaction::Enemy,
            }],
        ),
        faction(0x99, vec![]),
    ]);
    let result = resolve_disposition(
        &observer(vec![0x10, 0x99], 20, Aggression::Aggressive),
        &target(vec![0x10]),
        &table,
        &DispositionThresholds::default(),
    );
    assert_eq!(result.hostility, Hostility::Friendly);
    assert_eq!(result.decided_by, HostilityRule::SharedFaction);
}

#[test]
fn aggressive_actor_attacks_low_disposition_stranger() {
    let result = resolve_disposition(
        &observer(vec![], 40, Aggression::Aggressive),
        &target(vec![]),
        &FactionRelationTable::new(),
        &DispositionThresholds::default(),
    );
    assert_eq!(result.hostility, Hostility::Hostile);
    assert_eq!(result.decided_by, HostilityRule::Aggressive);
}

#[test]
fn unaggressive_stranger_falls_through_to_disposition_bucket() {
    let neutral = resolve_disposition(
        &observer(vec![], 50, Aggression::Unaggressive),
        &target(vec![]),
        &FactionRelationTable::new(),
        &DispositionThresholds::default(),
    );
    assert_eq!(neutral.hostility, Hostility::Neutral);
    assert_eq!(neutral.decided_by, HostilityRule::DispositionThreshold);

    let friendly = resolve_disposition(
        &observer(vec![], 80, Aggression::Unaggressive),
        &target(vec![]),
        &FactionRelationTable::new(),
        &DispositionThresholds::default(),
    );
    assert_eq!(friendly.hostility, Hostility::Friendly);

    let hostile = resolve_disposition(
        &observer(vec![], 10, Aggression::Unaggressive),
        &target(vec![]),
        &FactionRelationTable::new(),
        &DispositionThresholds::default(),
    );
    assert_eq!(hostile.hostility, Hostility::Hostile);
}

#[test]
fn faction_modifiers_move_the_disposition_value_and_are_clamped() {
    let table = table_with(vec![
        faction(
            0x10,
            vec![FactionRelation {
                faction_form_id: 0x20,
                modifier: -40,
                reaction: GroupCombatReaction::Neutral,
            }],
        ),
        faction(0x20, vec![]),
    ]);
    let result = resolve_disposition(
        &observer(vec![0x10], 50, Aggression::Unaggressive),
        &target(vec![0x20]),
        &table,
        &DispositionThresholds::default(),
    );
    assert_eq!(result.disposition, 10);
    assert_eq!(result.hostility, Hostility::Hostile);
}

#[test]
fn unresolved_faction_is_diagnosed_not_guessed() {
    let result = resolve_disposition(
        &observer(vec![0xABCD], 50, Aggression::Unaggressive),
        &target(vec![]),
        &FactionRelationTable::new(),
        &DispositionThresholds::default(),
    );
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.contains("unresolved observer faction 0000abcd"))
    );
}

#[test]
fn resolution_is_deterministic_regardless_of_faction_authoring_order() {
    let table = table_with(vec![
        faction(
            0x10,
            vec![FactionRelation {
                faction_form_id: 0x20,
                modifier: -40,
                reaction: GroupCombatReaction::Enemy,
            }],
        ),
        faction(0x20, vec![]),
        faction(0x30, vec![]),
    ]);
    let ordered = observer(vec![0x10, 0x30], 50, Aggression::Unaggressive);
    let reversed = DispositionActor {
        factions: vec![member(0x30), member(0x10)],
        ..ordered.clone()
    };
    let a = resolve_disposition(&ordered, &target(vec![0x20]), &table, &Default::default());
    let b = resolve_disposition(&reversed, &target(vec![0x20]), &table, &Default::default());
    assert_eq!(a, b);
}
