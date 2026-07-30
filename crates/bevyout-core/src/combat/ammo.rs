//! Canonical ammunition, magazine, and reload policy.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct MagazineState {
    pub ammo_form_id: Option<u32>,
    pub loaded: u16,
}

impl MagazineState {
    pub fn validate(self, capacity: u16) -> Result<Self, AmmoError> {
        if self.loaded > capacity || (self.loaded > 0 && self.ammo_form_id.is_none()) {
            return Err(AmmoError::InvalidMagazine);
        }
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ItemCombatState {
    pub magazine: MagazineState,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ReloadKind {
    Empty,
    Operational,
    AmmoSwitch,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReloadDecision {
    pub kind: ReloadKind,
    pub return_loaded: u16,
    pub consume_reserve: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FireBlockReason {
    Empty,
    InvalidMagazine,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AmmoError {
    InvalidAmmo,
    InvalidCapacity,
    InvalidMagazine,
    IncompatibleAmmo,
    InsufficientReserve,
}

pub fn plan_reload(
    magazine: MagazineState,
    requested_ammo_form_id: u32,
    capacity: u16,
    reserve: u32,
) -> Result<ReloadDecision, AmmoError> {
    if requested_ammo_form_id == 0 {
        return Err(AmmoError::InvalidAmmo);
    }
    if capacity == 0 {
        return Err(AmmoError::InvalidCapacity);
    }
    magazine.validate(capacity)?;
    let switching = magazine
        .ammo_form_id
        .is_some_and(|loaded| loaded != requested_ammo_form_id);
    let retained = if switching { 0 } else { magazine.loaded };
    let missing = capacity - retained;
    let consume_reserve = u16::try_from(reserve.min(u32::from(missing)))
        .map_err(|_| AmmoError::InsufficientReserve)?;
    if consume_reserve == 0 {
        return Err(AmmoError::InsufficientReserve);
    }
    Ok(ReloadDecision {
        kind: if switching {
            ReloadKind::AmmoSwitch
        } else if magazine.loaded == 0 {
            ReloadKind::Empty
        } else {
            ReloadKind::Operational
        },
        return_loaded: if switching { magazine.loaded } else { 0 },
        consume_reserve,
    })
}

pub fn consume_round(magazine: &mut MagazineState) -> Result<(), FireBlockReason> {
    if magazine.loaded == 0 {
        return Err(FireBlockReason::Empty);
    }
    if magazine.ammo_form_id.is_none() {
        return Err(FireBlockReason::InvalidMagazine);
    }
    magazine.loaded -= 1;
    Ok(())
}

#[cfg(test)]
#[path = "tests/ammo.rs"]
mod tests;
