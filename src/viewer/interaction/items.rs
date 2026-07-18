//! Runtime item, equipment, and canonical-ledger resource adapters.
//!
//! Pure transaction rules live in `bevyout-core`; this module owns only the
//! Bevy resources and legacy viewer projections at that boundary.

use super::*;

#[derive(Resource, Default, Debug)]
pub(crate) struct PlayerInventory(Inventory);

impl PlayerInventory {
    pub(crate) fn count(&self, form_id: u32) -> i32 {
        self.0.count(form_id)
    }

    pub(crate) fn contains(&self, form_id: u32) -> bool {
        self.0.contains(form_id)
    }

    pub(crate) fn add_stack(&mut self, stack: InventoryStack) -> TransferResult {
        self.0.add(stack)
    }

    pub(crate) fn remove(&mut self, key: StackKey, count: i32) -> TransferResult {
        self.0.remove(key, count)
    }

    pub(crate) fn available(&self, key: StackKey) -> i32 {
        self.0.available(key)
    }

    pub(crate) fn stack_states(&self) -> Vec<InventoryStack> {
        self.0.stacks()
    }

    pub(crate) fn legacy_snapshot(&self) -> Inventory {
        self.0.clone()
    }

    pub(crate) fn total_weight(&self, weight: impl FnMut(u32) -> Option<f32>) -> f32 {
        self.0.total_weight(weight)
    }

    /// Issue #60 (F60.4): rebuilds the inventory from a loaded save's
    /// player record.
    #[cfg(test)]
    pub(crate) fn from_stacks(stacks: impl IntoIterator<Item = (u32, i32)>) -> Self {
        Self(Inventory::from_stacks(stacks.into_iter().map(
            |(base_form_id, count)| InventoryStack {
                base_form_id,
                count,
                condition: None,
            },
        )))
    }

    pub(crate) fn from_stack_states(stacks: impl IntoIterator<Item = InventoryStack>) -> Self {
        Self(Inventory::from_stacks(stacks))
    }
}

/// Runtime adapter for the canonical #95 holder ledger. The legacy inventory
/// resource remains the Pip-Boy-facing projection during this migration; item
/// ownership and mutable state live in this ledger.
#[derive(Resource, Debug, Clone)]
pub(crate) struct CanonicalItemLedger {
    pub(crate) ledger: ItemLedger,
}

impl Default for CanonicalItemLedger {
    fn default() -> Self {
        Self {
            ledger: ItemLedger::new(),
        }
    }
}

/// Issue #98 (F98.2/F98.3): the authoritative equipped set. Thin `Resource`
/// wrapper around the pure `equipment::EquipmentState`, mirroring how
/// `PlayerInventory` wraps `Inventory`.
#[derive(Resource, Default, Debug, Clone)]
pub(crate) struct PlayerEquipment(EquipmentState);

impl PlayerEquipment {
    pub(crate) fn is_equipped(&self, key: StackKey) -> bool {
        self.0.is_equipped(key)
    }

    pub(crate) fn equipped_apparel(
        &self,
    ) -> impl Iterator<Item = (equipment::BipedSlot, StackKey)> + '_ {
        self.0.equipped_apparel()
    }

    pub(crate) fn equipped_weapon(&self) -> Option<StackKey> {
        self.0.equipped_weapon()
    }

    pub(crate) fn equipped_ammo(&self) -> Option<StackKey> {
        self.0.equipped_ammo()
    }

    pub(crate) fn toggle(
        &mut self,
        key: StackKey,
        kind: EquipKind,
    ) -> Result<EquipOutcome, EquipError> {
        self.0.toggle(key, kind)
    }

    pub(crate) fn equip(
        &mut self,
        key: StackKey,
        kind: EquipKind,
    ) -> Result<EquipOutcome, EquipError> {
        self.0.equip(key, kind)
    }

    pub(crate) fn unequip(&mut self, key: StackKey) -> EquipOutcome {
        self.0.unequip(key)
    }

    /// Issue #60/#98 (F98.4): rebuilds equipment from a loaded save's
    /// persisted entries.
    pub(crate) fn restore(
        apparel: impl IntoIterator<Item = (equipment::BipedSlot, StackKey)>,
        weapon: Option<(StackKey, Option<u32>)>,
        ammo: Option<StackKey>,
    ) -> Self {
        Self(EquipmentState::restore(apparel, weapon, ammo))
    }
}

/// F98.3: the biped-slot mask (apparel), required ammo form id (weapon), or
/// plain ammo-slot membership an item's `PreparedItemStats` maps to, if it
/// is equippable at all. Shared by the Pip-Boy equip toggle, hotkeys, and
/// the `player.equipitem` console command so all three equip through the
/// same classification.
pub(crate) fn equip_kind_for(item: &PreparedItemDefinition) -> Option<EquipKind> {
    match item.category {
        PreparedItemCategory::Weapons => {
            let ammo_form_id = match &item.stats {
                PreparedItemStats::Weapon { ammo_form_id, .. } => *ammo_form_id,
                _ => None,
            };
            Some(EquipKind::Weapon { ammo_form_id })
        }
        PreparedItemCategory::Apparel => {
            let biped_slot_mask = match &item.stats {
                PreparedItemStats::Apparel {
                    biped_slot_mask, ..
                } => biped_slot_mask.unwrap_or(0),
                _ => 0,
            };
            Some(EquipKind::Apparel { biped_slot_mask })
        }
        PreparedItemCategory::Ammo => Some(EquipKind::Ammo),
        _ => None,
    }
}

/// F98.3: written by `viewer::pipboy`'s equip-toggle row action and by
/// `viewer::bindings`' outside-Pip-Boy hotkey press; `apply_equip_toggle_requests`
/// is the single choke point that resolves the catalog entry, decides the
/// `EquipKind`, and mutates `PlayerEquipment`, so both callers get identical
/// behavior and notices.
#[derive(Message, Clone, Copy, Debug)]
pub(crate) struct EquipToggleRequested(pub(crate) StackKey);

pub(super) fn apply_equip_toggle_requests(
    mut requests: MessageReader<EquipToggleRequested>,
    catalog: Res<PreparedItemCatalog>,
    inventory: Res<PlayerInventory>,
    mut equipment_state: ResMut<PlayerEquipment>,
    mut notice: ResMut<InteractionNotice>,
) {
    for request in requests.read() {
        let key = request.0;
        if !inventory
            .stack_states()
            .iter()
            .any(|stack| stack.key() == key)
        {
            notice.show("That item is not in your inventory");
            continue;
        }
        let Some(item) = catalog
            .items
            .iter()
            .find(|item| item.base_form_id == key.base_form_id)
        else {
            notice.show("That item has no prepared definition");
            continue;
        };
        let Some(kind) = equip_kind_for(item) else {
            notice.show("That item cannot be equipped");
            continue;
        };
        let label = item
            .display_name
            .clone()
            .or_else(|| item.editor_id.clone())
            .unwrap_or_else(|| format!("{:08X}", item.base_form_id));
        match equipment_state.toggle(key, kind) {
            Ok(_outcome) => {
                let now_equipped = equipment_state.is_equipped(key);
                notice.show(if now_equipped {
                    format!("Equipped {label}")
                } else {
                    format!("Unequipped {label}")
                });
            }
            Err(EquipError::NotEquippable) => notice.show("That item cannot be equipped"),
            Err(EquipError::IncompatibleAmmo) => {
                notice.show("That ammo does not fit the equipped weapon")
            }
            Err(EquipError::NoWeaponEquipped) => notice.show("Equip a weapon before loading ammo"),
        }
    }
}

impl CanonicalItemLedger {
    pub(crate) fn from_snapshot(snapshot: ItemLedgerSnapshot) -> Result<Self, TransactionError> {
        Ok(Self {
            ledger: ItemLedger::from_snapshot(snapshot)?,
        })
    }

    pub(crate) fn snapshot(&self) -> ItemLedgerSnapshot {
        self.ledger.snapshot()
    }

    pub(crate) fn player_stack_key(&self, item_id: ItemInstanceId) -> Option<StackKey> {
        self.ledger
            .holders()
            .get(&HolderId::Player)
            .and_then(|state| state.find(item_id))
            .map(|item| StackKey {
                base_form_id: item.base_form_id,
                condition: item.state.condition,
            })
    }

    pub(crate) fn player_legacy_snapshot(&self) -> Option<Inventory> {
        let state = self.ledger.holders().get(&HolderId::Player)?;
        let mut stacks = BTreeMap::<(u32, Option<u32>), i32>::new();
        for item in &state.items {
            let count = i32::try_from(item.count).unwrap_or(i32::MAX);
            let entry = stacks
                .entry((item.base_form_id, item.state.condition))
                .or_default();
            *entry = entry.saturating_add(count);
        }
        Some(Inventory::from_stacks(stacks.into_iter().map(
            |((base_form_id, condition), count)| InventoryStack {
                base_form_id,
                count,
                condition,
            },
        )))
    }

    pub(crate) fn sync_player(&mut self, inventory: &Inventory) -> Result<(), TransactionError> {
        if !self.ledger.holders().contains_key(&HolderId::Player) {
            self.ledger
                .insert_holder(HolderId::Player, ItemHolderState::default())?;
        }
        let mut desired = BTreeMap::new();
        for stack in inventory.stacks() {
            if stack.count > 0 {
                desired.insert(
                    (stack.base_form_id, stack.condition),
                    u32::try_from(stack.count).map_err(|_| TransactionError::InvalidCount)?,
                );
            }
        }
        let mut missing = desired.clone();
        if let Some(state) = self.ledger.holders_mut().get_mut(&HolderId::Player) {
            for item in &mut state.items {
                let remaining = missing
                    .entry((item.base_form_id, item.state.condition))
                    .or_default();
                let kept = item.count.min(*remaining);
                item.count = kept;
                *remaining -= kept;
            }
            state.items.retain(|item| item.count > 0);
            state.items.sort_by_key(|item| item.id);
            state.revision = state.revision.saturating_add(1);
        }
        for ((base_form_id, condition), count) in missing {
            if count > 0 {
                self.ledger.insert_new_item(
                    HolderId::Player,
                    base_form_id,
                    count,
                    ItemState {
                        condition,
                        ..Default::default()
                    },
                )?;
            }
        }
        Ok(())
    }

    pub(crate) fn add_player_item(
        &mut self,
        inventory: &Inventory,
        stack: InventoryStack,
    ) -> Result<ItemInstanceId, TransactionError> {
        self.sync_player(inventory)?;
        self.ledger.insert_new_item(
            HolderId::Player,
            stack.base_form_id,
            u32::try_from(stack.count).map_err(|_| TransactionError::InvalidCount)?,
            ItemState {
                condition: stack.condition,
                ..Default::default()
            },
        )
    }

    pub(crate) fn move_player_to_runtime(
        &mut self,
        inventory: &Inventory,
        runtime_id: u64,
        cell_form_id: u32,
        stack: InventoryStack,
    ) -> Result<(), TransactionError> {
        self.sync_player(inventory)?;
        let holder = HolderId::RuntimeWorld {
            cell_form_id,
            runtime_id,
        };
        self.ledger
            .insert_holder(holder, ItemHolderState::default())?;
        let item_id = self
            .ledger
            .holders()
            .get(&HolderId::Player)
            .and_then(|state| {
                state.items.iter().find(|item| {
                    item.base_form_id == stack.base_form_id
                        && item.state.condition == stack.condition
                        && item.count >= u32::try_from(stack.count).unwrap_or_default()
                })
            })
            .map(|item| item.id)
            .ok_or(TransactionError::InsufficientItems)?;
        self.ledger.execute(TransactionRequest::Transfer {
            source: HolderId::Player,
            destination: holder,
            item_id,
            count: u32::try_from(stack.count).map_err(|_| TransactionError::InvalidCount)?,
        })?;
        Ok(())
    }

    pub(crate) fn move_runtime_to_player(
        &mut self,
        inventory: &Inventory,
        runtime_id: u64,
        cell_form_id: u32,
    ) -> Result<i32, TransactionError> {
        self.sync_player(inventory)?;
        let source = HolderId::RuntimeWorld {
            cell_form_id,
            runtime_id,
        };
        let item = self
            .ledger
            .holders()
            .get(&source)
            .and_then(|state| state.items.first())
            .cloned()
            .ok_or(TransactionError::InsufficientItems)?;
        self.ledger.execute(TransactionRequest::Transfer {
            source,
            destination: HolderId::Player,
            item_id: item.id,
            count: item.count,
        })?;
        Ok(i32::try_from(item.count).unwrap_or(i32::MAX))
    }

    pub(crate) fn ensure_runtime_item(
        &mut self,
        runtime_id: u64,
        cell_form_id: u32,
        stack: InventoryStack,
    ) -> Result<(), TransactionError> {
        let holder = HolderId::RuntimeWorld {
            cell_form_id,
            runtime_id,
        };
        if !self.ledger.holders().contains_key(&holder) {
            self.ledger
                .insert_holder(holder, ItemHolderState::default())?;
            self.ledger.insert_new_item(
                holder,
                stack.base_form_id,
                u32::try_from(stack.count).map_err(|_| TransactionError::InvalidCount)?,
                ItemState {
                    condition: stack.condition,
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }

    pub(crate) fn ensure_container(
        &mut self,
        reference_form_id: u32,
        stacks: &[(u32, i32)],
    ) -> Result<(), TransactionError> {
        let holder = HolderId::FixtureContainer { reference_form_id };
        if self.ledger.holders().contains_key(&holder) {
            return Ok(());
        }
        self.ledger
            .insert_holder(holder, ItemHolderState::default())?;
        for &(base_form_id, count) in stacks {
            if count > 0 {
                self.ledger.insert_new_item(
                    holder,
                    base_form_id,
                    u32::try_from(count).map_err(|_| TransactionError::InvalidCount)?,
                    ItemState::default(),
                )?;
            }
        }
        Ok(())
    }

    pub(crate) fn set_merchant(
        &mut self,
        reference_form_id: u32,
        stacks: &[(u32, i32)],
        caps: u64,
    ) -> Result<(), TransactionError> {
        let holder = HolderId::FixtureMerchant { reference_form_id };
        if self.ledger.holders().contains_key(&holder) {
            return Err(TransactionError::InvalidMerchant);
        }
        self.ledger.insert_holder(
            holder,
            ItemHolderState {
                caps,
                ..Default::default()
            },
        )?;
        for &(base_form_id, count) in stacks {
            if count > 0 {
                self.ledger.insert_new_item(
                    holder,
                    base_form_id,
                    u32::try_from(count).map_err(|_| TransactionError::InvalidCount)?,
                    ItemState::default(),
                )?;
            }
        }
        Ok(())
    }
}
