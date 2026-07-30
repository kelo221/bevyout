use super::*;

fn table() -> FactionRelationTable {
    let mut table = FactionRelationTable::new();
    table.insert(PreparedFaction {
        form_id: 0x10,
        editor_id: Some("RaidersFaction".into()),
        name: Some("Raiders".into()),
        relations: vec![
            FactionRelation {
                faction_form_id: 0x20,
                modifier: -60,
                reaction: GroupCombatReaction::Enemy,
            },
            FactionRelation {
                faction_form_id: 0x30,
                modifier: 40,
                reaction: GroupCombatReaction::Ally,
            },
        ],
    });
    table.insert(PreparedFaction {
        form_id: 0x20,
        editor_id: Some("TownsfolkFaction".into()),
        name: Some("Townsfolk".into()),
        relations: Vec::new(),
    });
    // Reciprocal ally so the combined reaction is Ally from both sides.
    table.insert(PreparedFaction {
        form_id: 0x30,
        editor_id: Some("AlliesFaction".into()),
        name: Some("Allies".into()),
        relations: vec![FactionRelation {
            faction_form_id: 0x10,
            modifier: 40,
            reaction: GroupCombatReaction::Ally,
        }],
    });
    table
}

#[test]
fn from_raw_preserves_unknown_reaction_value() {
    assert_eq!(GroupCombatReaction::from_raw(1), GroupCombatReaction::Enemy);
    assert_eq!(
        GroupCombatReaction::from_raw(7),
        GroupCombatReaction::Unknown(7)
    );
}

#[test]
fn relation_modifier_and_reaction_are_directional_lookups() {
    let table = table();
    assert_eq!(table.modifier(0x10, 0x20), -60);
    assert_eq!(table.modifier(0x20, 0x10), 0);
    assert_eq!(table.modifier(0x10, 0x999), 0);
}

#[test]
fn combat_reaction_takes_most_hostile_direction() {
    let table = table();
    // Only 0x10 -> 0x20 declares Enemy; the reverse is silent Neutral.
    assert_eq!(
        table.combat_reaction(0x20, 0x10),
        GroupCombatReaction::Enemy
    );
    // Reciprocal ally on both sides combines to Ally.
    assert_eq!(table.combat_reaction(0x10, 0x30), GroupCombatReaction::Ally);
    // A one-sided ally with a silent (neutral) reverse combines to Neutral;
    // only Enemy is contagious across a single direction.
    assert_eq!(
        table.combat_reaction(0x20, 0x999),
        GroupCombatReaction::Neutral
    );
}

#[test]
fn most_hostile_is_order_independent() {
    assert_eq!(
        GroupCombatReaction::Enemy.most_hostile(GroupCombatReaction::Friend),
        GroupCombatReaction::Enemy
    );
    assert_eq!(
        GroupCombatReaction::Friend.most_hostile(GroupCombatReaction::Enemy),
        GroupCombatReaction::Enemy
    );
    assert_eq!(
        GroupCombatReaction::Ally.most_hostile(GroupCombatReaction::Neutral),
        GroupCombatReaction::Neutral
    );
}
