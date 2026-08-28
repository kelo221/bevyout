//! Two-item field repair over the canonical item ledger (M9 wave 5).
//!
//! Condition math is integer-only. Fallout 3 GOTY:
//! `NewCond = CondA + CondB + (MaxCond * 0.25 * Repair / 100)`,
//! capped at `max(0.5, Repair/100) * MaxCond`. Compatibility is the same
//! `base_form_id` (no NAM2 repair list is present in Fallout3.esm).

use serde::{Deserialize, Serialize};

use crate::item_transaction::{
    HolderId, ItemInstanceId, ItemLedger, TransactionError, TransactionId,
};

pub const REPAIR_SETTINGS_REVISION: &str = "fo3-repair-v1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepairRequest {
    pub transaction_id: TransactionId,
    pub holder: HolderId,
    pub target: ItemInstanceId,
    pub donor: ItemInstanceId,
    pub repair_skill: u8,
    pub max_condition: u32,
    pub expected_holder_revision: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepairReceipt {
    pub id: TransactionId,
    pub target: ItemInstanceId,
    pub donor: ItemInstanceId,
    pub condition_before: u32,
    pub condition_after: u32,
    pub donor_condition: u32,
    pub cap: u32,
    pub donor_consumed: u32,
    pub holder_revision_before: u64,
    pub holder_revision_after: u64,
    pub settings_revision: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepairError {
    Transaction(TransactionError),
    SameItem,
    Incompatible,
    EquippedDonor,
    StaleRevision,
    MissingMaxCondition,
}

impl std::fmt::Display for RepairError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transaction(error) => write!(formatter, "{error}"),
            Self::SameItem => formatter.write_str("repair target and donor are the same item"),
            Self::Incompatible => formatter.write_str("repair items are incompatible"),
            Self::EquippedDonor => formatter.write_str("repair donor is equipped"),
            Self::StaleRevision => formatter.write_str("repair holder revision is stale"),
            Self::MissingMaxCondition => formatter.write_str("repair max condition is missing"),
        }
    }
}

impl std::error::Error for RepairError {}

impl From<TransactionError> for RepairError {
    fn from(error: TransactionError) -> Self {
        match error {
            TransactionError::StaleRevision => Self::StaleRevision,
            TransactionError::EquippedItem(_) => Self::EquippedDonor,
            other => Self::Transaction(other),
        }
    }
}

#[must_use]
pub fn repair_bonus(max_condition: u32, skill: u8) -> u32 {
    (u64::from(max_condition) * 25 * u64::from(skill) / 10_000) as u32
}

#[must_use]
pub fn repair_cap(max_condition: u32, skill: u8) -> u32 {
    let factor = u32::from(skill.max(50));
    ((u64::from(factor) * u64::from(max_condition)) / 100) as u32
}

#[must_use]
pub fn planned_condition(cond_a: u32, cond_b: u32, max_condition: u32, skill: u8) -> u32 {
    let uncapped = cond_a
        .saturating_add(cond_b)
        .saturating_add(repair_bonus(max_condition, skill));
    uncapped
        .min(repair_cap(max_condition, skill))
        .min(max_condition)
}

pub fn repair(
    ledger: &mut ItemLedger,
    request: RepairRequest,
) -> Result<RepairReceipt, RepairError> {
    if let Some(receipt) = ledger.repair_receipt(request.transaction_id) {
        return Ok(receipt);
    }
    if request.target == request.donor {
        return Err(RepairError::SameItem);
    }
    if request.max_condition == 0 {
        return Err(RepairError::MissingMaxCondition);
    }
    ledger.repair_with_id(request)
}

impl ItemLedger {
    pub(crate) fn repair_receipt(&self, id: TransactionId) -> Option<RepairReceipt> {
        self.repair_finalized.get(&id).cloned()
    }

    pub(crate) fn repair_with_id(
        &mut self,
        request: RepairRequest,
    ) -> Result<RepairReceipt, RepairError> {
        if let Some(receipt) = self.repair_finalized.get(&request.transaction_id) {
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
                return Err(RepairError::StaleRevision);
            }
            state.revision
        };
        let target_index = candidate
            .holders
            .get(&request.holder)
            .ok_or(TransactionError::UnknownHolder(request.holder))?
            .items
            .iter()
            .position(|item| item.id == request.target)
            .ok_or(TransactionError::InsufficientItems)?;
        let donor_index = candidate
            .holders
            .get(&request.holder)
            .ok_or(TransactionError::UnknownHolder(request.holder))?
            .items
            .iter()
            .position(|item| item.id == request.donor)
            .ok_or(TransactionError::InsufficientItems)?;
        let (target_form, donor_form, target_count, donor_count, cond_a, cond_b) = {
            let state = candidate
                .holders
                .get(&request.holder)
                .ok_or(TransactionError::UnknownHolder(request.holder))?;
            (
                state.items[target_index].base_form_id,
                state.items[donor_index].base_form_id,
                state.items[target_index].count,
                state.items[donor_index].count,
                state.items[target_index].state.condition.unwrap_or(0),
                state.items[donor_index].state.condition.unwrap_or(0),
            )
        };
        if target_form != donor_form {
            return Err(RepairError::Incompatible);
        }
        if donor_count == 0 {
            return Err(TransactionError::InvalidCount.into());
        }
        if candidate
            .bindings
            .get(&request.holder)
            .is_some_and(|bindings| bindings.references(request.donor))
        {
            return Err(RepairError::EquippedDonor);
        }
        let condition_after =
            planned_condition(cond_a, cond_b, request.max_condition, request.repair_skill);

        let mut canonical_target = request.target;
        if target_count > 1 {
            canonical_target = candidate.allocate_item_id();
        }
        {
            let state = candidate
                .holders
                .get_mut(&request.holder)
                .ok_or(TransactionError::UnknownHolder(request.holder))?;
            if target_count > 1 {
                state.items[target_index].count -= 1;
                let mut singleton = state.items[target_index].clone();
                singleton.id = canonical_target;
                singleton.count = 1;
                singleton.state.condition = Some(condition_after);
                state.items.push(singleton);
            } else {
                state.items[target_index].state.condition = Some(condition_after);
            }
        }
        if target_count > 1
            && let Some(bindings) = candidate.bindings.get_mut(&request.holder)
        {
            bindings.remap(request.target, canonical_target);
        }
        {
            let state = candidate
                .holders
                .get_mut(&request.holder)
                .ok_or(TransactionError::UnknownHolder(request.holder))?;
            let donor_index = state
                .items
                .iter()
                .position(|item| item.id == request.donor)
                .ok_or(TransactionError::InsufficientItems)?;
            if state.items[donor_index].count == 1 {
                state.items.remove(donor_index);
            } else {
                state.items[donor_index].count -= 1;
            }
            state.items.sort_by_key(|item| item.id);
            state.revision = state.revision.saturating_add(1);
            state.validate()?;
        }
        if let Some(bindings) = candidate.bindings.get_mut(&request.holder) {
            let state = candidate
                .holders
                .get(&request.holder)
                .ok_or(TransactionError::UnknownHolder(request.holder))?;
            bindings.prune_to(state);
        }
        let holder_revision_after = candidate
            .holders
            .get(&request.holder)
            .ok_or(TransactionError::UnknownHolder(request.holder))?
            .revision;
        let receipt = RepairReceipt {
            id: request.transaction_id,
            target: canonical_target,
            donor: request.donor,
            condition_before: cond_a,
            condition_after,
            donor_condition: cond_b,
            cap: repair_cap(request.max_condition, request.repair_skill),
            donor_consumed: 1,
            holder_revision_before,
            holder_revision_after,
            settings_revision: REPAIR_SETTINGS_REVISION.into(),
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
            .repair_finalized
            .insert(request.transaction_id, receipt.clone());
        *self = candidate;
        Ok(receipt)
    }
}

#[cfg(test)]
#[path = "tests/repair.rs"]
mod tests;
