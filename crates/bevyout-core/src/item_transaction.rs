//! Canonical item instances and atomic holder transactions (M3/#95).
//!
//! This module deliberately has no Bevy dependency.  It is the authoritative
//! decision layer for item movement; runtime adapters may stage these values in
//! ECS resources, but they must commit the resulting holder snapshots as one
//! operation.  The representation is intentionally extensible: game-specific
//! mutable state is carried as deterministic, namespaced opaque payloads until
//! a later slice needs to interpret it.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct ItemInstanceId(pub u64);

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct TransactionId(pub u64);

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum HolderId {
    Player,
    FixtureContainer {
        reference_form_id: u32,
    },
    FixtureMerchant {
        reference_form_id: u32,
    },
    WorldReference {
        cell_form_id: u32,
        reference_form_id: u32,
    },
    RuntimeWorld {
        cell_form_id: u32,
        runtime_id: u64,
    },
    Corpse {
        actor_reference_form_id: u32,
    },
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OwnershipProvenance {
    pub origin_owner_form_id: Option<u32>,
    pub origin_faction_rank: Option<i32>,
    pub stolen: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ItemExtraEntry {
    pub namespace_form_id: u32,
    pub tag: [u8; 4],
    pub payload: Vec<u8>,
}

impl ItemExtraEntry {
    fn key(&self) -> (u32, [u8; 4]) {
        (self.namespace_form_id, self.tag)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ItemState {
    pub condition: Option<u32>,
    pub ownership: OwnershipProvenance,
    pub extras: Vec<ItemExtraEntry>,
}

impl ItemState {
    pub fn normalized(mut self) -> Result<Self, TransactionError> {
        self.extras.sort_by_key(ItemExtraEntry::key);
        for pair in self.extras.windows(2) {
            if pair[0].key() == pair[1].key() {
                return Err(TransactionError::DuplicateExtraState {
                    namespace_form_id: pair[0].namespace_form_id,
                    tag: pair[0].tag,
                });
            }
        }
        Ok(self)
    }

    fn stack_compatible(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ItemInstance {
    pub id: ItemInstanceId,
    pub base_form_id: u32,
    pub count: u32,
    pub state: ItemState,
}

impl ItemInstance {
    pub fn new(
        id: ItemInstanceId,
        base_form_id: u32,
        count: u32,
        state: ItemState,
    ) -> Result<Self, TransactionError> {
        if count == 0 {
            return Err(TransactionError::InvalidCount);
        }
        Ok(Self {
            id,
            base_form_id,
            count,
            state: state.normalized()?,
        })
    }

    fn compatible_with(&self, other: &Self) -> bool {
        self.base_form_id == other.base_form_id && self.state.stack_compatible(&other.state)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BindingState {
    pub equipped: Option<ItemInstanceId>,
    pub hotkeys: [Option<ItemInstanceId>; 8],
}

impl BindingState {
    fn remap(&mut self, from: ItemInstanceId, to: ItemInstanceId) {
        if self.equipped == Some(from) {
            self.equipped = Some(to);
        }
        for binding in &mut self.hotkeys {
            if *binding == Some(from) {
                *binding = Some(to);
            }
        }
    }

    fn references(&self, id: ItemInstanceId) -> bool {
        self.equipped == Some(id)
    }

    fn prune_to(&mut self, items: &ItemHolderState) {
        self.equipped = self
            .equipped
            .filter(|item_id| items.find(*item_id).is_some());
        for hotkey in &mut self.hotkeys {
            *hotkey = hotkey.filter(|item_id| items.find(*item_id).is_some());
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ItemHolderState {
    pub items: Vec<ItemInstance>,
    pub caps: u64,
    pub revision: u64,
}

impl ItemHolderState {
    pub fn validate(&self) -> Result<(), TransactionError> {
        let mut ids = BTreeSet::new();
        for item in &self.items {
            if item.count == 0 {
                return Err(TransactionError::InvalidCount);
            }
            if !ids.insert(item.id) {
                return Err(TransactionError::DuplicateItemId(item.id));
            }
            item.state.clone().normalized()?;
        }
        if self.items.windows(2).any(|pair| pair[0].id >= pair[1].id) {
            return Err(TransactionError::UnsortedItems);
        }
        Ok(())
    }

    pub fn find(&self, id: ItemInstanceId) -> Option<&ItemInstance> {
        self.items.iter().find(|item| item.id == id)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TransactionReceipt {
    pub id: TransactionId,
    pub moved: Vec<(ItemInstanceId, u32)>,
    pub remaps: Vec<(ItemInstanceId, ItemInstanceId)>,
    pub caps_delta: BTreeMap<HolderId, i64>,
    pub source_revision: u64,
    pub destination_revision: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TransactionRequest {
    Transfer {
        source: HolderId,
        destination: HolderId,
        item_id: ItemInstanceId,
        count: u32,
    },
    Buy {
        merchant: HolderId,
        player: HolderId,
        item_id: ItemInstanceId,
        count: u32,
        unit_price: u64,
    },
    Sell {
        player: HolderId,
        merchant: HolderId,
        item_id: ItemInstanceId,
        count: u32,
        unit_price: u64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransactionError {
    DuplicateTransaction(TransactionId),
    UnknownHolder(HolderId),
    InvalidCount,
    InsufficientItems,
    InsufficientCaps,
    CapsOverflow,
    SameHolder,
    DuplicateItemId(ItemInstanceId),
    UnsortedItems,
    DuplicateExtraState {
        namespace_form_id: u32,
        tag: [u8; 4],
    },
    InvalidBinding {
        holder: HolderId,
        item_id: ItemInstanceId,
    },
    EquippedItem(ItemInstanceId),
    InvalidMerchant,
    EquipmentOccupied,
    InvalidHotkey,
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for TransactionError {}

#[derive(Clone, Debug, Default)]
pub struct ItemLedger {
    holders: BTreeMap<HolderId, ItemHolderState>,
    bindings: BTreeMap<HolderId, BindingState>,
    finalized: BTreeMap<TransactionId, TransactionReceipt>,
    used_transaction_ids: BTreeSet<TransactionId>,
    next_item_id: ItemInstanceId,
    next_transaction_id: TransactionId,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ItemLedgerSnapshot {
    pub holders: BTreeMap<HolderId, ItemHolderState>,
    pub bindings: BTreeMap<HolderId, BindingState>,
    pub finalized: BTreeMap<TransactionId, TransactionReceipt>,
    pub used_transaction_ids: BTreeSet<TransactionId>,
    pub next_item_id: ItemInstanceId,
    pub next_transaction_id: TransactionId,
}

impl ItemLedger {
    pub fn new() -> Self {
        Self {
            next_item_id: ItemInstanceId(1),
            next_transaction_id: TransactionId(1),
            ..Self::default()
        }
    }

    pub fn holders(&self) -> &BTreeMap<HolderId, ItemHolderState> {
        &self.holders
    }

    pub fn holders_mut(&mut self) -> &mut BTreeMap<HolderId, ItemHolderState> {
        &mut self.holders
    }

    pub fn bindings(&self) -> &BTreeMap<HolderId, BindingState> {
        &self.bindings
    }

    pub fn next_item_id(&self) -> ItemInstanceId {
        self.next_item_id
    }

    pub fn next_transaction_id(&self) -> TransactionId {
        self.next_transaction_id
    }

    pub fn snapshot(&self) -> ItemLedgerSnapshot {
        ItemLedgerSnapshot {
            holders: self.holders.clone(),
            bindings: self.bindings.clone(),
            finalized: self.finalized.clone(),
            used_transaction_ids: self.used_transaction_ids.clone(),
            next_item_id: self.next_item_id,
            next_transaction_id: self.next_transaction_id,
        }
    }

    pub fn from_snapshot(snapshot: ItemLedgerSnapshot) -> Result<Self, TransactionError> {
        for state in snapshot.holders.values() {
            state.validate()?;
        }
        for (holder, bindings) in &snapshot.bindings {
            let state = snapshot
                .holders
                .get(holder)
                .ok_or(TransactionError::UnknownHolder(*holder))?;
            for item_id in bindings
                .equipped
                .into_iter()
                .chain(bindings.hotkeys.into_iter().flatten())
            {
                if state.find(item_id).is_none() {
                    return Err(TransactionError::InvalidBinding {
                        holder: *holder,
                        item_id,
                    });
                }
            }
        }
        Ok(Self {
            holders: snapshot.holders,
            bindings: snapshot.bindings,
            finalized: snapshot.finalized,
            used_transaction_ids: snapshot.used_transaction_ids,
            next_item_id: snapshot.next_item_id,
            next_transaction_id: snapshot.next_transaction_id,
        })
    }

    pub fn equip(
        &mut self,
        holder: HolderId,
        item_id: ItemInstanceId,
    ) -> Result<(), TransactionError> {
        let state = self
            .holders
            .get(&holder)
            .ok_or(TransactionError::UnknownHolder(holder))?;
        if state.find(item_id).is_none() {
            return Err(TransactionError::InsufficientItems);
        }
        let bindings = self.bindings.entry(holder).or_default();
        if bindings.equipped.is_some() {
            return Err(TransactionError::EquipmentOccupied);
        }
        bindings.equipped = Some(item_id);
        Ok(())
    }

    pub fn unequip(
        &mut self,
        holder: HolderId,
    ) -> Result<Option<ItemInstanceId>, TransactionError> {
        let bindings = self.bindings.entry(holder).or_default();
        Ok(bindings.equipped.take())
    }

    pub fn bind_hotkey(
        &mut self,
        holder: HolderId,
        slot: usize,
        item_id: ItemInstanceId,
    ) -> Result<(), TransactionError> {
        let state = self
            .holders
            .get(&holder)
            .ok_or(TransactionError::UnknownHolder(holder))?;
        if slot >= 8 {
            return Err(TransactionError::InvalidHotkey);
        }
        if state.find(item_id).is_none() {
            return Err(TransactionError::InsufficientItems);
        }
        self.bindings.entry(holder).or_default().hotkeys[slot] = Some(item_id);
        Ok(())
    }

    /// Minimal #95 use seam: an explicit use consumes one unit and is the only
    /// generic operation besides create/destroy that changes quantity. Typed
    /// effects can attach to the returned instance in a later slice.
    pub fn use_item(
        &mut self,
        holder: HolderId,
        item_id: ItemInstanceId,
    ) -> Result<ItemInstance, TransactionError> {
        let (used, removed) = {
            let state = self
                .holders
                .get_mut(&holder)
                .ok_or(TransactionError::UnknownHolder(holder))?;
            let index = state
                .items
                .iter()
                .position(|item| item.id == item_id)
                .ok_or(TransactionError::InsufficientItems)?;
            let used = ItemInstance {
                count: 1,
                ..state.items[index].clone()
            };
            let removed = state.items[index].count == 1;
            if removed {
                state.items.remove(index);
            } else {
                state.items[index].count -= 1;
            }
            state.revision = state.revision.saturating_add(1);
            (used, removed)
        };
        if removed && let Some(bindings) = self.bindings.get_mut(&holder) {
            if bindings.equipped == Some(item_id) {
                bindings.equipped = None;
            }
            for hotkey in &mut bindings.hotkeys {
                if *hotkey == Some(item_id) {
                    *hotkey = None;
                }
            }
        }
        Ok(used)
    }

    pub fn insert_holder(
        &mut self,
        id: HolderId,
        mut state: ItemHolderState,
    ) -> Result<(), TransactionError> {
        state.items.sort_by_key(|item| item.id);
        state.validate()?;
        if let Some(max_id) = state.items.iter().map(|item| item.id.0).max() {
            self.next_item_id = ItemInstanceId(self.next_item_id.0.max(max_id.saturating_add(1)));
        }
        self.holders.insert(id, state);
        self.bindings.entry(id).or_default();
        Ok(())
    }

    pub fn insert_new_item(
        &mut self,
        holder: HolderId,
        base_form_id: u32,
        count: u32,
        state: ItemState,
    ) -> Result<ItemInstanceId, TransactionError> {
        let state = state.normalized()?;
        if count == 0 {
            return Err(TransactionError::InvalidCount);
        }
        let existing_index = self
            .holders
            .get(&holder)
            .ok_or(TransactionError::UnknownHolder(holder))?
            .items
            .iter()
            .position(|item| item.base_form_id == base_form_id && item.state == state);
        if let Some(existing_index) = existing_index {
            let holder_state = self.holders.get_mut(&holder).expect("holder checked above");
            let existing = &mut holder_state.items[existing_index];
            existing.count = existing
                .count
                .checked_add(count)
                .ok_or(TransactionError::CapsOverflow)?;
            holder_state.revision = holder_state.revision.saturating_add(1);
            return Ok(existing.id);
        }
        let id = self.next_item_id;
        self.next_item_id = ItemInstanceId(id.0.saturating_add(1));
        let item = ItemInstance::new(id, base_form_id, count, state)?;
        let holder_state = self.holders.get_mut(&holder).expect("holder checked above");
        holder_state.items.push(item);
        holder_state.items.sort_by_key(|item| item.id);
        holder_state.revision = holder_state.revision.saturating_add(1);
        Ok(id)
    }

    pub fn execute(
        &mut self,
        request: TransactionRequest,
    ) -> Result<TransactionReceipt, TransactionError> {
        let id = self.next_transaction_id;
        self.next_transaction_id = TransactionId(id.0.saturating_add(1));
        self.execute_with_id(id, request)
    }

    pub fn execute_with_id(
        &mut self,
        id: TransactionId,
        request: TransactionRequest,
    ) -> Result<TransactionReceipt, TransactionError> {
        if let Some(receipt) = self.finalized.get(&id) {
            return Ok(receipt.clone());
        }
        if !self.used_transaction_ids.insert(id) {
            return Err(TransactionError::DuplicateTransaction(id));
        }
        self.next_transaction_id =
            TransactionId(self.next_transaction_id.0.max(id.0.saturating_add(1)));

        let (source, destination, item_id, count, caps_payer, caps_receiver, unit_price) =
            match request {
                TransactionRequest::Transfer {
                    source,
                    destination,
                    item_id,
                    count,
                } => (source, destination, item_id, count, None, None, 0),
                TransactionRequest::Buy {
                    merchant,
                    player,
                    item_id,
                    count,
                    unit_price,
                } => (
                    merchant,
                    player,
                    item_id,
                    count,
                    Some(player),
                    Some(merchant),
                    unit_price,
                ),
                TransactionRequest::Sell {
                    player,
                    merchant,
                    item_id,
                    count,
                    unit_price,
                } => (
                    player,
                    merchant,
                    item_id,
                    count,
                    Some(merchant),
                    Some(player),
                    unit_price,
                ),
            };
        if source == destination {
            return Err(TransactionError::SameHolder);
        }
        if count == 0 {
            return Err(TransactionError::InvalidCount);
        }

        let mut source_state = self
            .holders
            .get(&source)
            .cloned()
            .ok_or(TransactionError::UnknownHolder(source))?;
        let mut destination_state = self
            .holders
            .get(&destination)
            .cloned()
            .ok_or(TransactionError::UnknownHolder(destination))?;
        let mut source_bindings = self.bindings.get(&source).cloned().unwrap_or_default();
        let mut destination_bindings = self.bindings.get(&destination).cloned().unwrap_or_default();
        let source_index = source_state
            .items
            .iter()
            .position(|item| item.id == item_id)
            .ok_or(TransactionError::InsufficientItems)?;
        if source_bindings.references(item_id) {
            return Err(TransactionError::EquippedItem(item_id));
        }
        if source_state.items[source_index].count < count {
            return Err(TransactionError::InsufficientItems);
        }

        let total_price = unit_price
            .checked_mul(u64::from(count))
            .ok_or(TransactionError::CapsOverflow)?;
        if let (Some(payer), Some(receiver)) = (caps_payer, caps_receiver) {
            if payer == receiver {
                return Err(TransactionError::SameHolder);
            }
            let payer_state = if payer == source {
                &mut source_state
            } else if payer == destination {
                &mut destination_state
            } else {
                return Err(TransactionError::UnknownHolder(payer));
            };
            if payer_state.caps < total_price {
                return Err(TransactionError::InsufficientCaps);
            }
            payer_state.caps -= total_price;
            let receiver_state = if receiver == source {
                &mut source_state
            } else if receiver == destination {
                &mut destination_state
            } else {
                return Err(TransactionError::UnknownHolder(receiver));
            };
            receiver_state.caps = receiver_state
                .caps
                .checked_add(total_price)
                .ok_or(TransactionError::CapsOverflow)?;
        }

        let mut moved_item = source_state.items[source_index].clone();
        if count == moved_item.count {
            source_state.items.remove(source_index);
        } else {
            source_state.items[source_index].count -= count;
            let new_id = self.allocate_item_id();
            moved_item.id = new_id;
            moved_item.count = count;
        }
        let moved_item_id = moved_item.id;
        let mut remaps = Vec::new();
        if let Some(destination_index) = destination_state
            .items
            .iter()
            .position(|item| item.compatible_with(&moved_item))
        {
            let survivor = destination_state.items[destination_index]
                .id
                .min(moved_item.id);
            let loser = destination_state.items[destination_index]
                .id
                .max(moved_item.id);
            destination_state.items[destination_index].count = destination_state.items
                [destination_index]
                .count
                .checked_add(moved_item.count)
                .ok_or(TransactionError::CapsOverflow)?;
            destination_state.items[destination_index].id = survivor;
            if loser == moved_item.id {
                remaps.push((moved_item.id, survivor));
            } else {
                let merged_count = destination_state.items[destination_index].count;
                destination_state.items.remove(destination_index);
                destination_state.items.push(ItemInstance {
                    id: survivor,
                    count: merged_count,
                    ..moved_item
                });
                remaps.push((loser, survivor));
            }
        } else {
            destination_state.items.push(moved_item.clone());
        }
        destination_state.items.sort_by_key(|item| item.id);
        for (from, to) in remaps.iter().copied() {
            source_bindings.remap(from, to);
            destination_bindings.remap(from, to);
        }
        source_bindings.prune_to(&source_state);
        destination_bindings.prune_to(&destination_state);
        source_state.revision = source_state.revision.saturating_add(1);
        destination_state.revision = destination_state.revision.saturating_add(1);
        source_state.validate()?;
        destination_state.validate()?;
        let mut caps_delta = BTreeMap::new();
        if total_price != 0
            && let (Some(payer), Some(receiver)) = (caps_payer, caps_receiver)
        {
            caps_delta.insert(payer, -(total_price as i64));
            caps_delta.insert(receiver, total_price as i64);
        }
        let receipt = TransactionReceipt {
            id,
            moved: vec![(moved_item_id, count)],
            remaps,
            caps_delta,
            source_revision: source_state.revision,
            destination_revision: destination_state.revision,
        };
        self.holders.insert(source, source_state);
        self.holders.insert(destination, destination_state);
        self.bindings.insert(source, source_bindings);
        self.bindings.insert(destination, destination_bindings);
        self.finalized.insert(id, receipt.clone());
        Ok(receipt)
    }

    fn allocate_item_id(&mut self) -> ItemInstanceId {
        let id = self.next_item_id;
        self.next_item_id = ItemInstanceId(id.0.saturating_add(1));
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn state(condition: Option<u32>) -> ItemState {
        ItemState {
            condition,
            ..Default::default()
        }
    }

    fn holder(items: Vec<ItemInstance>, caps: u64) -> ItemHolderState {
        ItemHolderState {
            items,
            caps,
            revision: 0,
        }
    }

    fn item(id: u64, form: u32, count: u32, condition: Option<u32>) -> ItemInstance {
        ItemInstance::new(ItemInstanceId(id), form, count, state(condition)).unwrap()
    }

    #[test]
    fn partial_transfer_allocates_a_new_destination_id_and_conserves_quantity() {
        let mut ledger = ItemLedger::new();
        ledger
            .insert_holder(HolderId::Player, holder(vec![item(7, 1, 5, Some(80))], 0))
            .unwrap();
        ledger
            .insert_holder(
                HolderId::FixtureContainer {
                    reference_form_id: 9,
                },
                holder(vec![], 0),
            )
            .unwrap();
        let receipt = ledger
            .execute(TransactionRequest::Transfer {
                source: HolderId::Player,
                destination: HolderId::FixtureContainer {
                    reference_form_id: 9,
                },
                item_id: ItemInstanceId(7),
                count: 2,
            })
            .unwrap();
        assert_eq!(receipt.moved, vec![(ItemInstanceId(8), 2)]);
        assert_eq!(
            ledger.holders()[&HolderId::Player].items[0].id,
            ItemInstanceId(7)
        );
        assert_eq!(ledger.holders()[&HolderId::Player].items[0].count, 3);
        assert_eq!(
            ledger.holders()[&HolderId::FixtureContainer {
                reference_form_id: 9
            }]
                .items[0]
                .count,
            2
        );
    }

    #[test]
    fn full_transfer_prunes_source_hotkeys() {
        let container = HolderId::FixtureContainer {
            reference_form_id: 9,
        };
        let mut ledger = ItemLedger::new();
        ledger
            .insert_holder(HolderId::Player, holder(vec![item(7, 1, 1, None)], 0))
            .unwrap();
        ledger.insert_holder(container, holder(vec![], 0)).unwrap();
        ledger
            .bind_hotkey(HolderId::Player, 0, ItemInstanceId(7))
            .unwrap();

        ledger
            .execute(TransactionRequest::Transfer {
                source: HolderId::Player,
                destination: container,
                item_id: ItemInstanceId(7),
                count: 1,
            })
            .unwrap();

        assert_eq!(ledger.bindings()[&HolderId::Player].hotkeys[0], None);
        assert_eq!(ledger.bindings()[&container].hotkeys[0], None);
    }

    #[test]
    fn partial_transfer_keeps_source_hotkey_and_leaves_destination_unbound() {
        let container = HolderId::FixtureContainer {
            reference_form_id: 9,
        };
        let mut ledger = ItemLedger::new();
        ledger
            .insert_holder(HolderId::Player, holder(vec![item(7, 1, 5, None)], 0))
            .unwrap();
        ledger.insert_holder(container, holder(vec![], 0)).unwrap();
        ledger
            .bind_hotkey(HolderId::Player, 0, ItemInstanceId(7))
            .unwrap();

        let receipt = ledger
            .execute(TransactionRequest::Transfer {
                source: HolderId::Player,
                destination: container,
                item_id: ItemInstanceId(7),
                count: 2,
            })
            .unwrap();

        assert_eq!(
            ledger.bindings()[&HolderId::Player].hotkeys[0],
            Some(ItemInstanceId(7))
        );
        assert_eq!(ledger.bindings()[&container].hotkeys[0], None);
        assert_eq!(receipt.moved, vec![(ItemInstanceId(8), 2)]);
    }

    #[test]
    fn compatible_merge_keeps_lowest_id_and_reports_remap() {
        let mut ledger = ItemLedger::new();
        ledger
            .insert_holder(HolderId::Player, holder(vec![item(2, 1, 1, None)], 0))
            .unwrap();
        ledger
            .insert_holder(
                HolderId::FixtureContainer {
                    reference_form_id: 9,
                },
                holder(vec![item(4, 1, 3, None)], 0),
            )
            .unwrap();
        let receipt = ledger
            .execute(TransactionRequest::Transfer {
                source: HolderId::Player,
                destination: HolderId::FixtureContainer {
                    reference_form_id: 9,
                },
                item_id: ItemInstanceId(2),
                count: 1,
            })
            .unwrap();
        assert_eq!(receipt.remaps, vec![(ItemInstanceId(4), ItemInstanceId(2))]);
        assert_eq!(
            ledger.holders()[&HolderId::FixtureContainer {
                reference_form_id: 9
            }]
                .items[0]
                .id,
            ItemInstanceId(2)
        );
        assert_eq!(
            ledger.holders()[&HolderId::FixtureContainer {
                reference_form_id: 9
            }]
                .items[0]
                .count,
            4
        );
    }

    #[test]
    fn merge_remaps_destination_binding_and_prunes_source_binding() {
        let container = HolderId::FixtureContainer {
            reference_form_id: 9,
        };
        let mut ledger = ItemLedger::new();
        ledger
            .insert_holder(HolderId::Player, holder(vec![item(2, 1, 1, None)], 0))
            .unwrap();
        ledger
            .insert_holder(container, holder(vec![item(4, 1, 3, None)], 0))
            .unwrap();
        ledger.bind_hotkey(container, 0, ItemInstanceId(4)).unwrap();

        let receipt = ledger
            .execute(TransactionRequest::Transfer {
                source: HolderId::Player,
                destination: container,
                item_id: ItemInstanceId(2),
                count: 1,
            })
            .unwrap();

        assert_eq!(receipt.remaps, vec![(ItemInstanceId(4), ItemInstanceId(2))]);
        assert_eq!(
            ledger.bindings()[&container].hotkeys[0],
            Some(ItemInstanceId(2))
        );
        assert_eq!(ledger.bindings()[&HolderId::Player].hotkeys[0], None);
    }

    #[test]
    fn snapshot_rejects_cross_holder_binding() {
        let container = HolderId::FixtureContainer {
            reference_form_id: 9,
        };
        let mut snapshot = ItemLedgerSnapshot {
            next_item_id: ItemInstanceId(4),
            next_transaction_id: TransactionId(1),
            ..Default::default()
        };
        snapshot
            .holders
            .insert(HolderId::Player, holder(vec![item(2, 1, 1, None)], 0));
        snapshot
            .holders
            .insert(container, holder(vec![item(3, 1, 1, None)], 0));
        snapshot.bindings.insert(
            HolderId::Player,
            BindingState {
                hotkeys: [
                    Some(ItemInstanceId(3)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                ..Default::default()
            },
        );

        assert!(matches!(
            ItemLedger::from_snapshot(snapshot),
            Err(TransactionError::InvalidBinding {
                holder: HolderId::Player,
                item_id: ItemInstanceId(3),
            })
        ));
    }

    #[test]
    fn failed_transaction_leaves_both_holders_unchanged() {
        let mut ledger = ItemLedger::new();
        ledger
            .insert_holder(HolderId::Player, holder(vec![item(2, 1, 1, None)], 0))
            .unwrap();
        ledger
            .insert_holder(
                HolderId::FixtureContainer {
                    reference_form_id: 9,
                },
                holder(vec![], 0),
            )
            .unwrap();
        let before = ledger.clone();
        assert_eq!(
            ledger.execute(TransactionRequest::Transfer {
                source: HolderId::Player,
                destination: HolderId::FixtureContainer {
                    reference_form_id: 9
                },
                item_id: ItemInstanceId(2),
                count: 2,
            }),
            Err(TransactionError::InsufficientItems)
        );
        assert_eq!(ledger.holders(), before.holders());
    }

    #[test]
    fn duplicate_transaction_id_is_a_no_op_after_success() {
        let mut ledger = ItemLedger::new();
        ledger
            .insert_holder(HolderId::Player, holder(vec![item(2, 1, 1, None)], 0))
            .unwrap();
        ledger
            .insert_holder(
                HolderId::FixtureContainer {
                    reference_form_id: 9,
                },
                holder(vec![], 0),
            )
            .unwrap();
        let request = TransactionRequest::Transfer {
            source: HolderId::Player,
            destination: HolderId::FixtureContainer {
                reference_form_id: 9,
            },
            item_id: ItemInstanceId(2),
            count: 1,
        };
        let first = ledger
            .execute_with_id(TransactionId(44), request.clone())
            .unwrap();
        let second = ledger.execute_with_id(TransactionId(44), request).unwrap();
        assert_eq!(first, second);
        assert_eq!(
            ledger.holders()[&HolderId::FixtureContainer {
                reference_form_id: 9
            }]
                .items[0]
                .count,
            1
        );
    }

    #[test]
    fn opaque_state_must_have_deterministic_unique_tags() {
        let mut state = state(None);
        state.extras = vec![
            ItemExtraEntry {
                namespace_form_id: 2,
                tag: *b"TEST",
                payload: vec![2],
            },
            ItemExtraEntry {
                namespace_form_id: 1,
                tag: *b"TEST",
                payload: vec![1],
            },
        ];
        assert_eq!(
            state.clone().normalized().unwrap().extras[0].namespace_form_id,
            1
        );
        state.extras.push(ItemExtraEntry {
            namespace_form_id: 1,
            tag: *b"TEST",
            payload: vec![3],
        });
        assert!(matches!(
            state.normalized(),
            Err(TransactionError::DuplicateExtraState { .. })
        ));
    }

    #[test]
    fn static_merchant_buy_and_sell_are_atomic_and_use_unit_price() {
        let merchant = HolderId::FixtureMerchant {
            reference_form_id: 9,
        };
        let mut ledger = ItemLedger::new();
        ledger
            .insert_holder(HolderId::Player, holder(vec![], 100))
            .unwrap();
        ledger
            .insert_holder(merchant, holder(vec![item(2, 1, 3, None)], 50))
            .unwrap();
        let buy = ledger
            .execute(TransactionRequest::Buy {
                merchant,
                player: HolderId::Player,
                item_id: ItemInstanceId(2),
                count: 2,
                unit_price: 10,
            })
            .unwrap();
        assert_eq!(buy.caps_delta[&HolderId::Player], -20);
        assert_eq!(ledger.holders()[&HolderId::Player].caps, 80);
        assert_eq!(ledger.holders()[&merchant].caps, 70);
        let bought_id = ledger.holders()[&HolderId::Player].items[0].id;
        let sell = ledger
            .execute(TransactionRequest::Sell {
                player: HolderId::Player,
                merchant,
                item_id: bought_id,
                count: 1,
                unit_price: 10,
            })
            .unwrap();
        assert_eq!(sell.caps_delta[&HolderId::Player], 10);
        assert_eq!(ledger.holders()[&HolderId::Player].caps, 90);
        assert_eq!(ledger.holders()[&merchant].caps, 60);
    }

    proptest::proptest! {
        #[test]
        fn random_partial_transfers_conserve_quantity(source_count in 1u32..200, requested in 1u32..200) {
            let moved = requested.min(source_count);
            let mut ledger = ItemLedger::new();
            ledger.insert_holder(HolderId::Player, holder(vec![item(7, 1, source_count, Some(80))], 0)).unwrap();
            let container = HolderId::FixtureContainer { reference_form_id: 9 };
            ledger.insert_holder(container, holder(vec![], 0)).unwrap();
            let result = ledger.execute(TransactionRequest::Transfer {
                source: HolderId::Player,
                destination: container,
                item_id: ItemInstanceId(7),
                count: moved,
            });
            prop_assert!(result.is_ok());
            let player_count = ledger.holders()[&HolderId::Player].items.iter().map(|item| item.count).sum::<u32>();
            let container_count = ledger.holders()[&container].items.iter().map(|item| item.count).sum::<u32>();
            prop_assert_eq!(player_count + container_count, source_count);
        }
    }
}
