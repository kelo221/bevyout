//! Pure Fallout 3 biped-slot equip/unequip rules (issue #98).
//!
//! Dependency-free except for `viewer::inventory::StackKey` (the same
//! stack-identity key `viewer::inventory::Inventory` already uses), so
//! `tests/features.rs` can include it verbatim -- see the module-level
//! nesting comment there for how the relative `super::super::inventory`
//! import lands. Fallout 3's `ARMO` records mark which biped body parts
//! they cover with a bitmask (`BMDT`, decoded by
//! `openmw_esm4::records::parse_item_stats`'s `ARMO` arm); equipping an
//! apparel item claims every slot its mask sets, evicting whatever
//! currently occupies any of them. Weapons and ammo are not part of the
//! biped-slot model -- FO3 tracks a single equipped weapon and a single
//! loaded ammo type, paired by the weapon's `NAM0` ammo base form id.

use super::super::inventory::StackKey;

/// FO3 `ARMO.BMDT` biped-slot bits (`ESM4::Armor::ArmorFlags`'s `FO3_*`
/// variants in OpenMW's `components/esm4/loadarmo.hpp`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum BipedSlot {
    Head,
    Hair,
    UpperBody,
    LeftHand,
    RightHand,
    Weapon,
    PipBoy,
    Backpack,
    Necklace,
    Headband,
    Hat,
    EyeGlasses,
    NoseRing,
    Earrings,
    Mask,
    Choker,
    MouthObject,
    BodyAddOn1,
    BodyAddOn2,
    BodyAddOn3,
}

impl BipedSlot {
    pub(crate) const ALL: [BipedSlot; 20] = [
        Self::Head,
        Self::Hair,
        Self::UpperBody,
        Self::LeftHand,
        Self::RightHand,
        Self::Weapon,
        Self::PipBoy,
        Self::Backpack,
        Self::Necklace,
        Self::Headband,
        Self::Hat,
        Self::EyeGlasses,
        Self::NoseRing,
        Self::Earrings,
        Self::Mask,
        Self::Choker,
        Self::MouthObject,
        Self::BodyAddOn1,
        Self::BodyAddOn2,
        Self::BodyAddOn3,
    ];

    pub(crate) fn bit(self) -> u32 {
        match self {
            Self::Head => 0x0000_0001,
            Self::Hair => 0x0000_0002,
            Self::UpperBody => 0x0000_0004,
            Self::LeftHand => 0x0000_0008,
            Self::RightHand => 0x0000_0010,
            Self::Weapon => 0x0000_0020,
            Self::PipBoy => 0x0000_0040,
            Self::Backpack => 0x0000_0080,
            Self::Necklace => 0x0000_0100,
            Self::Headband => 0x0000_0200,
            Self::Hat => 0x0000_0400,
            Self::EyeGlasses => 0x0000_0800,
            Self::NoseRing => 0x0000_1000,
            Self::Earrings => 0x0000_2000,
            Self::Mask => 0x0000_4000,
            Self::Choker => 0x0000_8000,
            Self::MouthObject => 0x0001_0000,
            Self::BodyAddOn1 => 0x0002_0000,
            Self::BodyAddOn2 => 0x0004_0000,
            Self::BodyAddOn3 => 0x0008_0000,
        }
    }
}

/// Every biped slot an apparel `BMDT` mask covers, in `BipedSlot::ALL` order.
pub(crate) fn slots_from_mask(mask: u32) -> Vec<BipedSlot> {
    BipedSlot::ALL
        .into_iter()
        .filter(|slot| mask & slot.bit() != 0)
        .collect()
}

/// Which kind of equip slot a `StackKey` is being equipped into. Apparel
/// carries its `BMDT` mask, and a weapon carries the ammo base form id it
/// requires (`WEAP.NAM0`) so equipping it can evict incompatible ammo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EquipKind {
    Apparel { biped_slot_mask: u32 },
    Weapon { ammo_form_id: Option<u32> },
    Ammo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EquipError {
    /// Apparel with an empty/zero biped-slot mask cannot be equipped.
    NotEquippable,
    /// Ammo does not match the currently equipped weapon's ammo type.
    IncompatibleAmmo,
    /// Ammo was equipped with no weapon equipped to pair it with.
    NoWeaponEquipped,
}

/// Stacks displaced by an equip/unequip -- callers return these to ordinary
/// carried inventory.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct EquipOutcome {
    pub(crate) evicted: Vec<StackKey>,
}

/// The FO3 biped-slot/weapon/ammo equip state. Every occupant is tracked by
/// `StackKey`, so a different condition of the same base item is a distinct
/// equipped identity (see `viewer::inventory::StackKey`'s doc comment).
#[derive(Debug, Clone, Default)]
pub(crate) struct EquipmentState {
    apparel: std::collections::BTreeMap<BipedSlot, StackKey>,
    weapon: Option<(StackKey, Option<u32>)>,
    ammo: Option<StackKey>,
}

impl EquipmentState {
    pub(crate) fn equipped_apparel(&self) -> impl Iterator<Item = (BipedSlot, StackKey)> + '_ {
        self.apparel.iter().map(|(slot, key)| (*slot, *key))
    }

    pub(crate) fn equipped_weapon(&self) -> Option<StackKey> {
        self.weapon.map(|(key, _)| key)
    }

    pub(crate) fn equipped_ammo(&self) -> Option<StackKey> {
        self.ammo
    }

    /// F98.2: the query the drop/transfer paths call -- an equipped item
    /// (apparel in any slot, the equipped weapon, or the loaded ammo) cannot
    /// be dropped or transferred while equipped.
    pub(crate) fn is_equipped(&self, key: StackKey) -> bool {
        self.apparel.values().any(|occupant| *occupant == key)
            || self.weapon.is_some_and(|(occupant, _)| occupant == key)
            || self.ammo == Some(key)
    }

    /// Equips `key` as `kind`, evicting whatever previously occupied the
    /// claimed slot(s). Does not check whether `key` is already equipped --
    /// see `toggle` for the equip-or-unequip convenience `viewer::pipboy`
    /// and hotkeys use.
    pub(crate) fn equip(
        &mut self,
        key: StackKey,
        kind: EquipKind,
    ) -> Result<EquipOutcome, EquipError> {
        match kind {
            EquipKind::Apparel { biped_slot_mask } => self.equip_apparel(key, biped_slot_mask),
            EquipKind::Weapon { ammo_form_id } => Ok(self.equip_weapon(key, ammo_form_id)),
            EquipKind::Ammo => self.equip_ammo(key),
        }
    }

    /// Unequips `key` from wherever it is currently equipped (a no-op,
    /// returning an empty outcome, if it is not equipped).
    pub(crate) fn unequip(&mut self, key: StackKey) -> EquipOutcome {
        if self.equipped_weapon() == Some(key) {
            return self.unequip_weapon();
        }
        if self.equipped_ammo() == Some(key) {
            return self.unequip_ammo();
        }
        self.unequip_apparel(key)
    }

    /// F98.3's Pip-Boy/hotkey entry point: unequips `key` if it is already
    /// equipped, otherwise equips it as `kind`.
    pub(crate) fn toggle(
        &mut self,
        key: StackKey,
        kind: EquipKind,
    ) -> Result<EquipOutcome, EquipError> {
        if self.is_equipped(key) {
            Ok(self.unequip(key))
        } else {
            self.equip(key, kind)
        }
    }

    fn equip_apparel(
        &mut self,
        key: StackKey,
        biped_slot_mask: u32,
    ) -> Result<EquipOutcome, EquipError> {
        let slots = slots_from_mask(biped_slot_mask);
        if slots.is_empty() {
            return Err(EquipError::NotEquippable);
        }
        let mut evicted = Vec::new();
        for slot in slots {
            if let Some(previous) = self.apparel.insert(slot, key)
                && previous != key
                && !evicted.contains(&previous)
            {
                evicted.push(previous);
            }
        }
        Ok(EquipOutcome { evicted })
    }

    fn unequip_apparel(&mut self, key: StackKey) -> EquipOutcome {
        let slots: Vec<BipedSlot> = self
            .apparel
            .iter()
            .filter(|(_, occupant)| **occupant == key)
            .map(|(slot, _)| *slot)
            .collect();
        if slots.is_empty() {
            return EquipOutcome::default();
        }
        for slot in slots {
            self.apparel.remove(&slot);
        }
        EquipOutcome { evicted: vec![key] }
    }

    /// F98.2 weapon+ammo pairing: switching to a weapon whose ammo type
    /// doesn't match the currently loaded ammo unequips that ammo too.
    fn equip_weapon(&mut self, key: StackKey, ammo_form_id: Option<u32>) -> EquipOutcome {
        let mut evicted = Vec::new();
        if let Some((previous, _)) = self.weapon.replace((key, ammo_form_id))
            && previous != key
        {
            evicted.push(previous);
        }
        if let Some(ammo_key) = self.ammo
            && ammo_form_id != Some(ammo_key.base_form_id)
        {
            self.ammo = None;
            evicted.push(ammo_key);
        }
        EquipOutcome { evicted }
    }

    fn unequip_weapon(&mut self) -> EquipOutcome {
        let mut evicted = Vec::new();
        if let Some((previous, _)) = self.weapon.take() {
            evicted.push(previous);
        }
        EquipOutcome { evicted }
    }

    /// Only the ammo type the equipped weapon requires (`WEAP.NAM0`) can be
    /// loaded; equipping ammo with no weapon equipped is rejected outright.
    fn equip_ammo(&mut self, key: StackKey) -> Result<EquipOutcome, EquipError> {
        let Some((_, required)) = self.weapon else {
            return Err(EquipError::NoWeaponEquipped);
        };
        if required != Some(key.base_form_id) {
            return Err(EquipError::IncompatibleAmmo);
        }
        let mut evicted = Vec::new();
        if let Some(previous) = self.ammo.replace(key)
            && previous != key
        {
            evicted.push(previous);
        }
        Ok(EquipOutcome { evicted })
    }

    fn unequip_ammo(&mut self) -> EquipOutcome {
        let mut evicted = Vec::new();
        if let Some(previous) = self.ammo.take() {
            evicted.push(previous);
        }
        EquipOutcome { evicted }
    }

    /// Save-restore constructor (issue #98, F98.4): rebuilds equip state
    /// directly from persisted entries, trusting the save (each entry was
    /// itself produced by `equip`'s invariants at write time) rather than
    /// replaying `equip` and having to re-derive catalog data at load time.
    pub(crate) fn restore(
        apparel: impl IntoIterator<Item = (BipedSlot, StackKey)>,
        weapon: Option<(StackKey, Option<u32>)>,
        ammo: Option<StackKey>,
    ) -> Self {
        Self {
            apparel: apparel.into_iter().collect(),
            weapon,
            ammo,
        }
    }
}

#[cfg(test)]
#[path = "tests/equipment.rs"]
mod tests;
