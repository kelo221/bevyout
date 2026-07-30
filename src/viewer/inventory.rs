//! OpenMW-inspired authoritative player inventory policy (M3 wave 1).
//!
//! OpenMW keeps stack identity and item-view presentation separate from world
//! objects. This pure module follows that split: Bevy interaction and UI code
//! consume these operations, but cannot mutate a second shadow count map.

use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct StackKey {
    pub(crate) base_form_id: u32,
    /// Current condition in record health units. `None` is used by items that
    /// do not have condition; fully and partially damaged items never merge.
    pub(crate) condition: Option<u32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct InventoryStack {
    pub(crate) base_form_id: u32,
    pub(crate) count: i32,
    pub(crate) condition: Option<u32>,
}

impl InventoryStack {
    pub(crate) fn key(self) -> StackKey {
        StackKey {
            base_form_id: self.base_form_id,
            condition: self.condition,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TransferResult {
    Applied { remaining: i32 },
    InvalidCount,
    InsufficientItems,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DropAction {
    DropOne,
    ChooseQuantity { min: i32, max: i32, default: i32 },
}

pub(crate) fn drop_action(count: i32) -> Option<DropAction> {
    match count {
        ..=0 => None,
        1..=3 => Some(DropAction::DropOne),
        _ => Some(DropAction::ChooseQuantity {
            min: 1,
            max: count,
            default: 1,
        }),
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Inventory {
    stacks: BTreeMap<StackKey, i32>,
}

impl Inventory {
    pub(crate) fn count(&self, base_form_id: u32) -> i32 {
        self.stacks
            .iter()
            .filter(|(key, _)| key.base_form_id == base_form_id)
            .map(|(_, count)| *count)
            .sum()
    }

    pub(crate) fn contains(&self, base_form_id: u32) -> bool {
        self.count(base_form_id) > 0
    }

    pub(crate) fn available(&self, key: StackKey) -> i32 {
        self.stacks.get(&key).copied().unwrap_or_default()
    }

    #[cfg(test)]
    pub(crate) fn stack_count(&self, key: StackKey) -> i32 {
        self.stacks.get(&key).copied().unwrap_or(0)
    }

    pub(crate) fn add(&mut self, stack: InventoryStack) -> TransferResult {
        if stack.count <= 0 {
            return TransferResult::InvalidCount;
        }
        let count = self.stacks.entry(stack.key()).or_default();
        *count = count.saturating_add(stack.count);
        TransferResult::Applied { remaining: *count }
    }

    pub(crate) fn remove(&mut self, key: StackKey, count: i32) -> TransferResult {
        if count <= 0 {
            return TransferResult::InvalidCount;
        }
        let Some(current) = self.stacks.get_mut(&key) else {
            return TransferResult::InsufficientItems;
        };
        if *current < count {
            return TransferResult::InsufficientItems;
        }
        *current -= count;
        let remaining = *current;
        if remaining == 0 {
            self.stacks.remove(&key);
        }
        TransferResult::Applied { remaining }
    }

    pub(crate) fn stacks(&self) -> Vec<InventoryStack> {
        self.stacks
            .iter()
            .map(|(key, count)| InventoryStack {
                base_form_id: key.base_form_id,
                count: *count,
                condition: key.condition,
            })
            .collect()
    }

    pub(crate) fn from_stacks(stacks: impl IntoIterator<Item = InventoryStack>) -> Self {
        let mut inventory = Self::default();
        for stack in stacks {
            let _ = inventory.add(stack);
        }
        inventory
    }

    pub(crate) fn total_weight(&self, mut weight: impl FnMut(u32) -> Option<f32>) -> f32 {
        self.stacks
            .iter()
            .filter_map(|(key, count)| {
                weight(key.base_form_id).map(|item_weight| item_weight * *count as f32)
            })
            .sum()
    }
}

#[cfg(test)]
#[path = "tests/inventory.rs"]
mod tests;
