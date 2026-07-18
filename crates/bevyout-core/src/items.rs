//! Pure caps, ownership, and quest-item rules (issue #81).
//!
//! Dependency-free (std only, no Bevy) exactly like `container_policy` so
//! `tests/features.rs` can include it verbatim. Callers check these rules
//! before invoking the #75 transfer ops; the ops themselves stay unchanged.

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

// ponytail: any non-player owner counts as theft — there is no faction
// membership model yet; add `XRNK` rank checks when factions land.
pub fn classify_take(owner_form_id: Option<u32>) -> TakeClassification {
    match owner_form_id {
        Some(owner) if owner != PLAYER_FORM_ID => TakeClassification::Steal {
            owner_form_id: owner,
        },
        _ => TakeClassification::Take,
    }
}
