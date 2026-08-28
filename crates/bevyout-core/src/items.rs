//! Pure caps, ownership, and quest-item rules (issue #81, M9 wave 6).
//!
//! Dependency-free (std only, no Bevy) exactly like `container_policy` so
//! `tests/features.rs` can include it verbatim. Callers check these rules
//! before invoking the #75 transfer ops; the ops themselves stay unchanged.

use crate::disposition::FactionMembership;
use crate::faction::FactionRelationTable;

/// FO3 bottle-cap currency base record (`Caps001`).
pub const CAPS_FORM_ID: u32 = 0x0000_000F;

/// Player base record (`Player`); references owned by the player are not
/// theft.
pub const PLAYER_FORM_ID: u32 = 0x0000_0007;

/// TES4-family record header flag marking a base record as a quest item.
const QUEST_ITEM_HEADER_FLAG: u32 = 0x0000_0400;

pub fn is_quest_item(record_flags: u32) -> bool {
    record_flags & QUEST_ITEM_HEADER_FLAG != 0
}

/// Why a drop/store request was refused before reaching the transfer ops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferRejection {
    QuestItem,
    Caps,
}

/// F81.2/F81.3: quest items and caps never leave the inventory as world
/// objects in FO3.
pub fn can_drop(base_form_id: u32, quest_item: bool) -> Result<(), TransferRejection> {
    if quest_item {
        Err(TransferRejection::QuestItem)
    } else if base_form_id == CAPS_FORM_ID {
        Err(TransferRejection::Caps)
    } else {
        Ok(())
    }
}

/// F81.2/F81.3: quest items cannot be stored into containers; caps can.
pub fn can_store(quest_item: bool) -> Result<(), TransferRejection> {
    if quest_item {
        Err(TransferRejection::QuestItem)
    } else {
        Ok(())
    }
}

/// F81.2: quest items are weightless for FO3 carry weight; everything else
/// keeps its record weight.
pub fn carried_weight(quest_item: bool, weight: Option<f32>) -> Option<f32> {
    if quest_item { None } else { weight }
}

/// F81.4: Take-vs-Steal for a reference with an optional `XOWN` owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TakeClassification {
    Take,
    Steal { owner_form_id: u32 },
}

/// Authored ownership on a reference or container (`XOWN` / `XRNK`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OwnershipClaim {
    pub owner_form_id: Option<u32>,
    pub owner_faction_rank: Option<i32>,
}

impl OwnershipClaim {
    #[must_use]
    pub const fn actor(owner_form_id: u32) -> Self {
        Self {
            owner_form_id: Some(owner_form_id),
            owner_faction_rank: None,
        }
    }
}

/// Membership snapshot used by take/steal classification.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TakerFactions {
    pub memberships: Vec<FactionMembership>,
}

impl TakerFactions {
    #[must_use]
    pub fn rank_in(&self, faction_form_id: u32) -> Option<i8> {
        self.memberships
            .iter()
            .filter(|membership| membership.faction_form_id == faction_form_id)
            .map(|membership| membership.rank)
            .max()
    }
}

/// Actor-owned property is theft unless the owner is the player. Faction-owned
/// property is legal when the taker holds at least the required rank in a
/// known faction. Unknown owners that are not the player remain theft so the
/// existing cucumber (`owned by 0x0001A2B3`) stays green.
pub fn classify_ownership(
    claim: OwnershipClaim,
    taker: &TakerFactions,
    factions: &FactionRelationTable,
) -> TakeClassification {
    let Some(owner) = claim.owner_form_id else {
        return TakeClassification::Take;
    };
    if owner == PLAYER_FORM_ID {
        return TakeClassification::Take;
    }
    if factions.is_known(owner) {
        let required = claim.owner_faction_rank.unwrap_or(0);
        if taker
            .rank_in(owner)
            .is_some_and(|rank| i32::from(rank) >= required)
        {
            return TakeClassification::Take;
        }
        return TakeClassification::Steal {
            owner_form_id: owner,
        };
    }
    TakeClassification::Steal {
        owner_form_id: owner,
    }
}

#[cfg(test)]
#[path = "tests/items.rs"]
mod tests;

/// Compatibility wrapper: owner FormID only, empty taker factions.
pub fn classify_take(owner_form_id: Option<u32>) -> TakeClassification {
    classify_ownership(
        OwnershipClaim {
            owner_form_id,
            owner_faction_rank: None,
        },
        &TakerFactions::default(),
        &FactionRelationTable::default(),
    )
}
