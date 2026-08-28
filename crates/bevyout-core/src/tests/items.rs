use super::*;
use crate::disposition::FactionMembership;
use crate::faction::{FactionRelationTable, PreparedFaction};

fn known_faction(form_id: u32) -> FactionRelationTable {
    let mut table = FactionRelationTable::new();
    table.insert(PreparedFaction {
        form_id,
        ..Default::default()
    });
    table
}

#[test]
fn unowned_and_player_owned_are_legal() {
    let table = FactionRelationTable::default();
    let taker = TakerFactions::default();
    assert_eq!(
        classify_ownership(OwnershipClaim::default(), &taker, &table),
        TakeClassification::Take
    );
    assert_eq!(
        classify_ownership(OwnershipClaim::actor(PLAYER_FORM_ID), &taker, &table),
        TakeClassification::Take
    );
}

#[test]
fn actor_owned_property_is_theft() {
    assert_eq!(
        classify_take(Some(0x0001_A2B3)),
        TakeClassification::Steal {
            owner_form_id: 0x0001_A2B3
        }
    );
}

#[test]
fn faction_rank_policy() {
    let table = known_faction(0x0002_2457);
    let member = TakerFactions {
        memberships: vec![FactionMembership {
            faction_form_id: 0x0002_2457,
            rank: 1,
        }],
    };
    let junior = TakerFactions {
        memberships: vec![FactionMembership {
            faction_form_id: 0x0002_2457,
            rank: 0,
        }],
    };
    let claim = OwnershipClaim {
        owner_form_id: Some(0x0002_2457),
        owner_faction_rank: Some(1),
    };
    assert_eq!(
        classify_ownership(claim, &member, &table),
        TakeClassification::Take
    );
    assert_eq!(
        classify_ownership(claim, &junior, &table),
        TakeClassification::Steal {
            owner_form_id: 0x0002_2457
        }
    );
    assert_eq!(
        classify_ownership(claim, &TakerFactions::default(), &table),
        TakeClassification::Steal {
            owner_form_id: 0x0002_2457
        }
    );
}
