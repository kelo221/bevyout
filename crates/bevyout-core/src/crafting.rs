//! Schematic crafting over prepared recipe definitions (M9 wave 5).
//!
//! Recipes are not decoded a second time. Any opaque CTDA payload is
//! [`CraftError::UnsupportedCondition`], never treated as true. Ingredient
//! consumption is ordered by `(base_form_id, ItemInstanceId)`.

use serde::{Deserialize, Serialize};

use crate::item_transaction::{
    HolderId, ItemInstanceId, ItemLedger, ItemState, TransactionError, TransactionId,
};

pub const CRAFT_SETTINGS_REVISION: &str = "fo3-craft-v1";

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum SchematicTier {
    #[default]
    V1,
    V2,
    V3,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeItem {
    pub item_form_id: u32,
    pub quantity: u32,
    pub order: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeDefinition {
    pub form_id: u32,
    pub skill: i32,
    pub level: u32,
    pub ingredients: Vec<RecipeItem>,
    pub outputs: Vec<RecipeItem>,
    pub has_conditions: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CraftingActorSnapshot {
    pub skill_value: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CraftRequest<'a> {
    pub transaction_id: TransactionId,
    pub holder: HolderId,
    pub recipe: &'a RecipeDefinition,
    pub count: u32,
    pub expected_holder_revision: u64,
    pub actor: CraftingActorSnapshot,
    pub schematic_tier: SchematicTier,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CraftReceipt {
    pub id: TransactionId,
    pub recipe_form_id: u32,
    pub count: u32,
    pub consumed: Vec<(ItemInstanceId, u32)>,
    pub created: Vec<(ItemInstanceId, u32, u32)>,
    pub holder_revision_before: u64,
    pub holder_revision_after: u64,
    pub schematic_tier: SchematicTier,
    pub settings_revision: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CraftError {
    Transaction(TransactionError),
    UnsupportedCondition,
    MissingIngredients,
    StaleRevision,
    InsufficientSkill,
    InvalidCount,
}

impl std::fmt::Display for CraftError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transaction(error) => write!(formatter, "{error}"),
            Self::UnsupportedCondition => {
                formatter.write_str("recipe has an unsupported GECK condition")
            }
            Self::MissingIngredients => formatter.write_str("missing crafting ingredients"),
            Self::StaleRevision => formatter.write_str("craft holder revision is stale"),
            Self::InsufficientSkill => formatter.write_str("crafting skill is too low"),
            Self::InvalidCount => formatter.write_str("craft count must be at least 1"),
        }
    }
}

impl std::error::Error for CraftError {}

impl From<TransactionError> for CraftError {
    fn from(error: TransactionError) -> Self {
        match error {
            TransactionError::StaleRevision => Self::StaleRevision,
            TransactionError::InsufficientItems => Self::MissingIngredients,
            TransactionError::InvalidCount => Self::InvalidCount,
            other => Self::Transaction(other),
        }
    }
}

pub fn craft(
    ledger: &mut ItemLedger,
    request: CraftRequest<'_>,
) -> Result<CraftReceipt, CraftError> {
    if let Some(receipt) = ledger.craft_receipt(request.transaction_id) {
        return Ok(receipt);
    }
    if request.count == 0 {
        return Err(CraftError::InvalidCount);
    }
    if request.recipe.has_conditions {
        return Err(CraftError::UnsupportedCondition);
    }
    if request.recipe.level > 0 && u32::from(request.actor.skill_value) < request.recipe.level {
        return Err(CraftError::InsufficientSkill);
    }
    ledger.craft_with_id(request)
}

impl ItemLedger {
    pub(crate) fn craft_receipt(&self, id: TransactionId) -> Option<CraftReceipt> {
        self.craft_finalized.get(&id).cloned()
    }

    pub(crate) fn craft_with_id(
        &mut self,
        request: CraftRequest<'_>,
    ) -> Result<CraftReceipt, CraftError> {
        if let Some(receipt) = self.craft_finalized.get(&request.transaction_id) {
            return Ok(receipt.clone());
        }
        if self.used_transaction_ids.contains(&request.transaction_id) {
            return Err(TransactionError::DuplicateTransaction(request.transaction_id).into());
        }

        let mut candidate = self.clone();
        let holder_revision_before = {
            let state = candidate
                .holders
                .get(&request.holder)
                .ok_or(TransactionError::UnknownHolder(request.holder))?;
            if state.revision != request.expected_holder_revision {
                return Err(CraftError::StaleRevision);
            }
            state.revision
        };
        let bindings = candidate
            .bindings
            .get(&request.holder)
            .cloned()
            .unwrap_or_default();

        let mut needed = std::collections::BTreeMap::<u32, u32>::new();
        for ingredient in &request.recipe.ingredients {
            let quantity = ingredient
                .quantity
                .checked_mul(request.count)
                .ok_or(TransactionError::CapsOverflow)?;
            *needed.entry(ingredient.item_form_id).or_insert(0) += quantity;
        }

        let mut consumed = Vec::new();
        {
            let state = candidate
                .holders
                .get_mut(&request.holder)
                .ok_or(TransactionError::UnknownHolder(request.holder))?;
            let mut ids: Vec<ItemInstanceId> = state.items.iter().map(|item| item.id).collect();
            ids.sort();
            for item_id in ids {
                let item = state
                    .find(item_id)
                    .ok_or(TransactionError::InsufficientItems)?;
                let Some(remaining) = needed.get_mut(&item.base_form_id) else {
                    continue;
                };
                if *remaining == 0 {
                    continue;
                }
                if bindings.references(item_id) {
                    return Err(TransactionError::EquippedItem(item_id).into());
                }
                let take = item.count.min(*remaining);
                *remaining -= take;
                consumed.push((item_id, take));
            }
            if needed.values().any(|remaining| *remaining > 0) {
                return Err(CraftError::MissingIngredients);
            }
            for (item_id, take) in &consumed {
                let item = state
                    .find_mut(*item_id)
                    .ok_or(TransactionError::InsufficientItems)?;
                item.count -= *take;
            }
            state.items.retain(|item| item.count > 0);
        }
        if let Some(bindings) = candidate.bindings.get_mut(&request.holder) {
            let state = candidate
                .holders
                .get(&request.holder)
                .ok_or(TransactionError::UnknownHolder(request.holder))?;
            bindings.prune_to(state);
        }

        let mut created = Vec::new();
        for output in &request.recipe.outputs {
            let quantity = output
                .quantity
                .checked_mul(request.count)
                .ok_or(TransactionError::CapsOverflow)?;
            let id = candidate.insert_new_item(
                request.holder,
                output.item_form_id,
                quantity,
                ItemState::default(),
            )?;
            created.push((id, output.item_form_id, quantity));
        }

        let holder_revision_after = {
            let state = candidate
                .holders
                .get_mut(&request.holder)
                .ok_or(TransactionError::UnknownHolder(request.holder))?;
            state.revision = holder_revision_before.saturating_add(1);
            state.validate()?;
            state.revision
        };
        let receipt = CraftReceipt {
            id: request.transaction_id,
            recipe_form_id: request.recipe.form_id,
            count: request.count,
            consumed,
            created,
            holder_revision_before,
            holder_revision_after,
            schematic_tier: request.schematic_tier,
            settings_revision: CRAFT_SETTINGS_REVISION.into(),
        };
        candidate
            .used_transaction_ids
            .insert(request.transaction_id);
        candidate.next_transaction_id = TransactionId(
            candidate
                .next_transaction_id
                .0
                .max(request.transaction_id.0.saturating_add(1)),
        );
        candidate
            .craft_finalized
            .insert(request.transaction_id, receipt.clone());
        *self = candidate;
        Ok(receipt)
    }
}

#[cfg(test)]
#[path = "tests/crafting.rs"]
mod tests;
