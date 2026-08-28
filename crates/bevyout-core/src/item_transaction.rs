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

use crate::combat::ammo::{self, ItemCombatState, ReloadKind};
use crate::combat::condition::{ConditionError, JamReason, WeaponConditionPolicy};
use crate::combat::rng::{CombatRngDomain, CombatRngDraw, CombatRngState};

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
    /// Canonical inventory owned by a living actor placement.
    ///
    /// The reference FormID, rather than the actor base FormID, keeps two
    /// placements of the same NPC or creature independent and stable across
    /// runtime respawns and save/load reconstruction.
    Actor {
        reference_form_id: u32,
    },
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
    #[serde(default)]
    pub combat: ItemCombatState,
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
    /// Active weapon. The serialized name is retained for v1-v4 compatibility.
    pub equipped: Option<ItemInstanceId>,
    #[serde(default)]
    pub equipped_apparel: BTreeSet<ItemInstanceId>,
    pub hotkeys: [Option<ItemInstanceId>; 8],
}

impl BindingState {
    pub(crate) fn remap(&mut self, from: ItemInstanceId, to: ItemInstanceId) {
        if self.equipped == Some(from) {
            self.equipped = Some(to);
        }
        if self.equipped_apparel.remove(&from) {
            self.equipped_apparel.insert(to);
        }
        for binding in &mut self.hotkeys {
            if *binding == Some(from) {
                *binding = Some(to);
            }
        }
    }

    pub(crate) fn references(&self, id: ItemInstanceId) -> bool {
        self.equipped == Some(id) || self.equipped_apparel.contains(&id)
    }

    pub(crate) fn prune_to(&mut self, items: &ItemHolderState) {
        self.equipped = self
            .equipped
            .filter(|item_id| items.find(*item_id).is_some());
        self.equipped_apparel
            .retain(|item_id| items.find(*item_id).is_some());
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
            item.state
                .combat
                .magazine
                .validate(u16::MAX)
                .map_err(|_| TransactionError::InvalidMagazine)?;
        }
        if self.items.windows(2).any(|pair| pair[0].id >= pair[1].id) {
            return Err(TransactionError::UnsortedItems);
        }
        Ok(())
    }

    pub fn find(&self, id: ItemInstanceId) -> Option<&ItemInstance> {
        self.items.iter().find(|item| item.id == id)
    }

    pub fn find_mut(&mut self, id: ItemInstanceId) -> Option<&mut ItemInstance> {
        self.items.iter_mut().find(|item| item.id == id)
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
pub struct AmmoTransactionReceipt {
    pub id: TransactionId,
    pub weapon_id: ItemInstanceId,
    pub ammo_form_id: u32,
    pub kind: ReloadKind,
    pub returned: u16,
    pub consumed: u16,
    pub loaded: u16,
    pub holder_revision: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CombatTransactionKind {
    Fire,
    Reload,
    ClearJam,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CombatTransactionOutcome {
    Fired,
    Jammed,
    Reloaded,
    Cleared,
    AlreadyClear,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeaponReloadRequest {
    pub ammo_form_id: u32,
    pub capacity: u16,
    pub policy: WeaponConditionPolicy,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CombatTransactionReceipt {
    pub id: TransactionId,
    pub weapon_id: ItemInstanceId,
    pub kind: CombatTransactionKind,
    pub outcome: CombatTransactionOutcome,
    pub condition_before: Option<u32>,
    pub condition_after: Option<u32>,
    /// Condition terms are persisted as milli-units so the serialized
    /// decision is independent of platform float formatting.
    pub damage_multiplier_milli: Option<u32>,
    pub damage_milli: Option<u32>,
    pub jam: Option<JamReason>,
    pub rng_draw: Option<CombatRngDraw>,
    pub loaded: u16,
    pub holder_revision: u64,
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
    InvalidMagazine,
    IncompatibleAmmo,
    MutableStack(ItemInstanceId),
    Jammed(JamReason),
    InvalidCombatRng,
    InvalidWeaponCondition,
    StaleRevision,
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for TransactionError {}

#[derive(Clone, Debug, Default)]
pub struct ItemLedger {
    pub(crate) holders: BTreeMap<HolderId, ItemHolderState>,
    pub(crate) bindings: BTreeMap<HolderId, BindingState>,
    finalized: BTreeMap<TransactionId, TransactionReceipt>,
    ammo_finalized: BTreeMap<TransactionId, AmmoTransactionReceipt>,
    combat_finalized: BTreeMap<TransactionId, CombatTransactionReceipt>,
    pub(crate) repair_finalized: BTreeMap<TransactionId, crate::repair::RepairReceipt>,
    pub(crate) craft_finalized: BTreeMap<TransactionId, crate::crafting::CraftReceipt>,
    pub(crate) used_transaction_ids: BTreeSet<TransactionId>,
    pub(crate) next_item_id: ItemInstanceId,
    pub(crate) next_transaction_id: TransactionId,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ItemLedgerSnapshot {
    pub holders: BTreeMap<HolderId, ItemHolderState>,
    pub bindings: BTreeMap<HolderId, BindingState>,
    pub finalized: BTreeMap<TransactionId, TransactionReceipt>,
    #[serde(default)]
    pub ammo_finalized: BTreeMap<TransactionId, AmmoTransactionReceipt>,
    #[serde(default)]
    pub combat_finalized: BTreeMap<TransactionId, CombatTransactionReceipt>,
    #[serde(default)]
    pub repair_finalized: BTreeMap<TransactionId, crate::repair::RepairReceipt>,
    #[serde(default)]
    pub craft_finalized: BTreeMap<TransactionId, crate::crafting::CraftReceipt>,
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
            ammo_finalized: self.ammo_finalized.clone(),
            combat_finalized: self.combat_finalized.clone(),
            repair_finalized: self.repair_finalized.clone(),
            craft_finalized: self.craft_finalized.clone(),
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
                .chain(bindings.equipped_apparel.iter().copied())
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
            ammo_finalized: snapshot.ammo_finalized,
            combat_finalized: snapshot.combat_finalized,
            repair_finalized: snapshot.repair_finalized,
            craft_finalized: snapshot.craft_finalized,
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

    pub fn equip_apparel(
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
        self.bindings
            .entry(holder)
            .or_default()
            .equipped_apparel
            .insert(item_id);
        Ok(())
    }

    /// Atomically returns switched rounds, consumes compatible reserve rounds,
    /// and mutates one canonical weapon instance.
    pub fn reload_weapon_with_id(
        &mut self,
        id: TransactionId,
        holder: HolderId,
        weapon_id: ItemInstanceId,
        ammo_form_id: u32,
        capacity: u16,
    ) -> Result<AmmoTransactionReceipt, TransactionError> {
        if let Some(receipt) = self.ammo_finalized.get(&id) {
            return Ok(receipt.clone());
        }
        if self.used_transaction_ids.contains(&id) {
            return Err(TransactionError::DuplicateTransaction(id));
        }

        let mut state = self
            .holders
            .get(&holder)
            .cloned()
            .ok_or(TransactionError::UnknownHolder(holder))?;
        let mut bindings = self.bindings.get(&holder).cloned().unwrap_or_default();
        let weapon_index = state
            .items
            .iter()
            .position(|item| item.id == weapon_id)
            .ok_or(TransactionError::InsufficientItems)?;
        let mut canonical_weapon_id = weapon_id;
        let mut next_item_id = self.next_item_id;
        if state.items[weapon_index].count > 1 {
            state.items[weapon_index].count -= 1;
            canonical_weapon_id = next_item_id;
            next_item_id = ItemInstanceId(next_item_id.0.saturating_add(1));
            let mut singleton = state.items[weapon_index].clone();
            singleton.id = canonical_weapon_id;
            singleton.count = 1;
            state.items.push(singleton);
            bindings.remap(weapon_id, canonical_weapon_id);
        }
        let weapon_index = state
            .items
            .iter()
            .position(|item| item.id == canonical_weapon_id)
            .expect("canonical weapon retained");
        let magazine = state.items[weapon_index].state.combat.magazine;
        let reserve = state
            .items
            .iter()
            .filter(|item| item.base_form_id == ammo_form_id)
            .map(|item| item.count)
            .sum();
        let decision =
            ammo::plan_reload(magazine, ammo_form_id, capacity, reserve).map_err(|error| {
                match error {
                    ammo::AmmoError::InvalidMagazine | ammo::AmmoError::InvalidCapacity => {
                        TransactionError::InvalidMagazine
                    }
                    ammo::AmmoError::InvalidAmmo | ammo::AmmoError::IncompatibleAmmo => {
                        TransactionError::IncompatibleAmmo
                    }
                    ammo::AmmoError::InsufficientReserve => TransactionError::InsufficientItems,
                }
            })?;

        let mut remaining = u32::from(decision.consume_reserve);
        for item in &mut state.items {
            if item.base_form_id == ammo_form_id && remaining > 0 {
                let consumed = item.count.min(remaining);
                item.count -= consumed;
                remaining -= consumed;
            }
        }
        state.items.retain(|item| item.count > 0);
        if decision.return_loaded > 0 {
            let old_ammo = magazine
                .ammo_form_id
                .ok_or(TransactionError::InvalidMagazine)?;
            if let Some(item) = state
                .items
                .iter_mut()
                .find(|item| item.base_form_id == old_ammo && item.state == ItemState::default())
            {
                item.count = item
                    .count
                    .checked_add(u32::from(decision.return_loaded))
                    .ok_or(TransactionError::CapsOverflow)?;
            } else {
                let returned_id = next_item_id;
                next_item_id = ItemInstanceId(next_item_id.0.saturating_add(1));
                state.items.push(ItemInstance::new(
                    returned_id,
                    old_ammo,
                    u32::from(decision.return_loaded),
                    ItemState::default(),
                )?);
            }
        }
        let weapon = state
            .items
            .iter_mut()
            .find(|item| item.id == canonical_weapon_id)
            .expect("canonical weapon retained");
        weapon.state.combat.magazine.ammo_form_id = Some(ammo_form_id);
        weapon.state.combat.magazine.loaded = if decision.kind == ReloadKind::AmmoSwitch {
            decision.consume_reserve
        } else {
            magazine.loaded.saturating_add(decision.consume_reserve)
        };
        let loaded = weapon.state.combat.magazine.loaded;
        state.items.sort_by_key(|item| item.id);
        state.revision = state.revision.saturating_add(1);
        let receipt = AmmoTransactionReceipt {
            id,
            weapon_id: canonical_weapon_id,
            ammo_form_id,
            kind: decision.kind,
            returned: decision.return_loaded,
            consumed: decision.consume_reserve,
            loaded,
            holder_revision: state.revision,
        };
        self.holders.insert(holder, state);
        self.bindings.insert(holder, bindings);
        self.next_item_id = next_item_id;
        self.next_transaction_id =
            TransactionId(self.next_transaction_id.0.max(id.0.saturating_add(1)));
        self.used_transaction_ids.insert(id);
        self.ammo_finalized.insert(id, receipt.clone());
        Ok(receipt)
    }

    pub fn consume_weapon_round(
        &mut self,
        holder: HolderId,
        weapon_id: ItemInstanceId,
    ) -> Result<(), TransactionError> {
        let state = self
            .holders
            .get_mut(&holder)
            .ok_or(TransactionError::UnknownHolder(holder))?;
        let weapon = state
            .items
            .iter_mut()
            .find(|item| item.id == weapon_id)
            .ok_or(TransactionError::InsufficientItems)?;
        if weapon.count != 1 {
            return Err(TransactionError::MutableStack(weapon_id));
        }
        ammo::consume_round(&mut weapon.state.combat.magazine)
            .map_err(|_| TransactionError::InsufficientItems)?;
        state.revision = state.revision.saturating_add(1);
        Ok(())
    }

    /// Atomically accepts one fire intent, consumes one loaded round, applies
    /// condition degradation, and records the deterministic jam draw. The
    /// affected weapon and a candidate RNG state are staged until every
    /// validation succeeds, so a rejected intent changes neither authority.
    pub fn fire_weapon_with_policy(
        &mut self,
        id: TransactionId,
        holder: HolderId,
        weapon_id: ItemInstanceId,
        base_damage: f32,
        policy: WeaponConditionPolicy,
        rng: &mut CombatRngState,
    ) -> Result<CombatTransactionReceipt, TransactionError> {
        if let Some(receipt) = self.combat_finalized.get(&id) {
            return Ok(receipt.clone());
        }
        if self.used_transaction_ids.contains(&id) {
            return Err(TransactionError::DuplicateTransaction(id));
        }

        let mut candidate_rng = rng.clone();
        candidate_rng
            .validate()
            .map_err(|_| TransactionError::InvalidCombatRng)?;
        let (condition_before, magazine) = {
            let state = self
                .holders
                .get(&holder)
                .ok_or(TransactionError::UnknownHolder(holder))?;
            let current = state
                .find(weapon_id)
                .ok_or(TransactionError::InsufficientItems)?;
            if current.count != 1 {
                return Err(TransactionError::MutableStack(weapon_id));
            }
            if let Some(reason) = current.state.combat.jam {
                return Err(TransactionError::Jammed(reason));
            }
            let condition_before = current.state.condition;
            let mut magazine = current.state.combat.magazine;
            ammo::consume_round(&mut magazine).map_err(|reason| match reason {
                ammo::FireBlockReason::Empty => TransactionError::InsufficientItems,
                ammo::FireBlockReason::InvalidMagazine => TransactionError::InvalidMagazine,
            })?;
            (condition_before, magazine)
        };
        let draw = candidate_rng
            .draw(CombatRngDomain::FireJam)
            .map_err(|_| TransactionError::InvalidCombatRng)?;
        let decision = policy
            .evaluate_fire(base_damage, condition_before, draw)
            .map_err(map_condition_error)?;

        let (jam, loaded, holder_revision) = {
            let state = self
                .holders
                .get_mut(&holder)
                .expect("holder was validated before fire mutation");
            let weapon = state
                .find_mut(weapon_id)
                .expect("weapon was validated before fire mutation");
            weapon.state.combat.magazine = magazine;
            if policy.max_condition().is_some() {
                weapon.state.condition = decision.condition_after;
            }
            weapon.state.combat.jam = decision.jammed.then_some(JamReason::Fire);
            let jam = weapon.state.combat.jam;
            let loaded = weapon.state.combat.magazine.loaded;
            state.revision = state.revision.saturating_add(1);
            (jam, loaded, state.revision)
        };
        let outcome = if decision.jammed {
            CombatTransactionOutcome::Jammed
        } else {
            CombatTransactionOutcome::Fired
        };
        let receipt = CombatTransactionReceipt {
            id,
            weapon_id,
            kind: CombatTransactionKind::Fire,
            outcome,
            condition_before: decision.condition_before,
            condition_after: if policy.max_condition().is_some() {
                decision.condition_after
            } else {
                condition_before
            },
            damage_multiplier_milli: Some(quantize_milli(decision.damage_multiplier)),
            damage_milli: Some(quantize_milli(decision.damage)),
            jam,
            rng_draw: Some(draw),
            loaded,
            holder_revision,
        };
        self.used_transaction_ids.insert(id);
        self.next_transaction_id =
            TransactionId(self.next_transaction_id.0.max(id.0.saturating_add(1)));
        self.combat_finalized.insert(id, receipt.clone());
        *rng = candidate_rng;
        Ok(receipt)
    }

    /// Atomically performs the existing ammo reload and then evaluates the
    /// reload jam policy against the same weapon instance. Ammo changes and a
    /// possible jam commit together; a rejected reload leaves both holders and
    /// the RNG untouched.
    pub fn reload_weapon_with_policy(
        &mut self,
        id: TransactionId,
        holder: HolderId,
        weapon_id: ItemInstanceId,
        request: WeaponReloadRequest,
        rng: &mut CombatRngState,
    ) -> Result<CombatTransactionReceipt, TransactionError> {
        if let Some(receipt) = self.combat_finalized.get(&id) {
            return Ok(receipt.clone());
        }
        if self.used_transaction_ids.contains(&id) {
            return Err(TransactionError::DuplicateTransaction(id));
        }
        let current = self
            .holders
            .get(&holder)
            .and_then(|state| state.find(weapon_id))
            .ok_or(TransactionError::InsufficientItems)?;
        if let Some(reason) = current.state.combat.jam {
            return Err(TransactionError::Jammed(reason));
        }
        if current.count != 1 {
            return Err(TransactionError::MutableStack(weapon_id));
        }
        let condition_before = current.state.condition;
        let mut candidate = self.clone();
        let mut candidate_rng = rng.clone();
        candidate_rng
            .validate()
            .map_err(|_| TransactionError::InvalidCombatRng)?;
        let ammo_receipt = candidate.reload_weapon_with_id(
            id,
            holder,
            weapon_id,
            request.ammo_form_id,
            request.capacity,
        )?;
        let draw = candidate_rng
            .draw(CombatRngDomain::ReloadJam)
            .map_err(|_| TransactionError::InvalidCombatRng)?;
        let jam_decision = request
            .policy
            .evaluate_reload(condition_before, draw)
            .map_err(map_condition_error)?;
        let (jam, loaded) = {
            let weapon = candidate
                .holders
                .get_mut(&holder)
                .and_then(|state| state.find_mut(ammo_receipt.weapon_id))
                .ok_or(TransactionError::InsufficientItems)?;
            weapon.state.combat.jam = jam_decision.jammed.then_some(JamReason::Reload);
            (weapon.state.combat.jam, weapon.state.combat.magazine.loaded)
        };
        if jam_decision.jammed {
            let state = candidate
                .holders
                .get_mut(&holder)
                .expect("holder was validated before reload mutation");
            state.revision = state.revision.saturating_add(1);
        }
        let holder_revision = candidate
            .holders
            .get(&holder)
            .map_or(ammo_receipt.holder_revision, |state| state.revision);
        let receipt = CombatTransactionReceipt {
            id,
            weapon_id: ammo_receipt.weapon_id,
            kind: CombatTransactionKind::Reload,
            outcome: if jam_decision.jammed {
                CombatTransactionOutcome::Jammed
            } else {
                CombatTransactionOutcome::Reloaded
            },
            condition_before: jam_decision.condition,
            condition_after: jam_decision.condition,
            damage_multiplier_milli: None,
            damage_milli: None,
            jam,
            rng_draw: Some(draw),
            loaded,
            holder_revision,
        };
        candidate.combat_finalized.insert(id, receipt.clone());
        *self = candidate;
        *rng = candidate_rng;
        Ok(receipt)
    }

    /// Clears the canonical jam on one weapon. This is an idempotent
    /// transaction and deliberately consumes no combat RNG draw.
    pub fn clear_weapon_jam_with_id(
        &mut self,
        id: TransactionId,
        holder: HolderId,
        weapon_id: ItemInstanceId,
    ) -> Result<CombatTransactionReceipt, TransactionError> {
        if let Some(receipt) = self.combat_finalized.get(&id) {
            return Ok(receipt.clone());
        }
        if self.used_transaction_ids.contains(&id) {
            return Err(TransactionError::DuplicateTransaction(id));
        }
        let mut candidate = self.clone();
        let state = candidate
            .holders
            .get_mut(&holder)
            .ok_or(TransactionError::UnknownHolder(holder))?;
        let weapon = state
            .items
            .iter_mut()
            .find(|item| item.id == weapon_id)
            .ok_or(TransactionError::InsufficientItems)?;
        if weapon.count != 1 {
            return Err(TransactionError::MutableStack(weapon_id));
        }
        let condition = weapon.state.condition;
        let previous_jam = weapon.state.combat.jam;
        if previous_jam.is_some() {
            weapon.state.combat.jam = None;
            state.revision = state.revision.saturating_add(1);
        }
        let receipt = CombatTransactionReceipt {
            id,
            weapon_id,
            kind: CombatTransactionKind::ClearJam,
            outcome: if previous_jam.is_some() {
                CombatTransactionOutcome::Cleared
            } else {
                CombatTransactionOutcome::AlreadyClear
            },
            condition_before: condition,
            condition_after: condition,
            damage_multiplier_milli: None,
            damage_milli: None,
            jam: None,
            rng_draw: None,
            loaded: weapon.state.combat.magazine.loaded,
            holder_revision: state.revision,
        };
        candidate.used_transaction_ids.insert(id);
        candidate.next_transaction_id =
            TransactionId(candidate.next_transaction_id.0.max(id.0.saturating_add(1)));
        candidate.combat_finalized.insert(id, receipt.clone());
        *self = candidate;
        Ok(receipt)
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

    pub fn execute_quoted(
        &mut self,
        id: TransactionId,
        request: TransactionRequest,
        expected_player_revision: u64,
        expected_merchant_revision: u64,
    ) -> Result<TransactionReceipt, TransactionError> {
        if let Some(receipt) = self.finalized.get(&id) {
            return Ok(receipt.clone());
        }
        let (player, merchant) = match &request {
            TransactionRequest::Buy {
                merchant, player, ..
            }
            | TransactionRequest::Sell {
                player, merchant, ..
            } => (*player, *merchant),
            TransactionRequest::Transfer { .. } => {
                return self.execute_with_id(id, request);
            }
        };
        let player_revision = self
            .holders
            .get(&player)
            .ok_or(TransactionError::UnknownHolder(player))?
            .revision;
        let merchant_revision = self
            .holders
            .get(&merchant)
            .ok_or(TransactionError::UnknownHolder(merchant))?
            .revision;
        if player_revision != expected_player_revision
            || merchant_revision != expected_merchant_revision
        {
            return Err(TransactionError::StaleRevision);
        }
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

    pub(crate) fn allocate_item_id(&mut self) -> ItemInstanceId {
        let id = self.next_item_id;
        self.next_item_id = ItemInstanceId(id.0.saturating_add(1));
        id
    }
}

fn map_condition_error(error: ConditionError) -> TransactionError {
    match error {
        ConditionError::InvalidMaximum | ConditionError::InvalidDamage(_) => {
            TransactionError::InvalidWeaponCondition
        }
    }
}

fn quantize_milli(value: f32) -> u32 {
    (value * 1_000.0).round().max(0.0) as u32
}

#[cfg(test)]
#[path = "tests/item_transaction.rs"]
mod tests;
