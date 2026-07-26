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
mod tests {
    use super::*;

    #[test]
    fn operational_reload_only_consumes_missing_rounds() {
        let decision = plan_reload(
            MagazineState {
                ammo_form_id: Some(10),
                loaded: 7,
            },
            10,
            12,
            20,
        )
        .unwrap();
        assert_eq!(decision.kind, ReloadKind::Operational);
        assert_eq!(decision.consume_reserve, 5);
        assert_eq!(decision.return_loaded, 0);
    }

    #[test]
    fn ammo_switch_returns_old_rounds_before_consuming_new_ones() {
        let decision = plan_reload(
            MagazineState {
                ammo_form_id: Some(10),
                loaded: 7,
            },
            20,
            12,
            20,
        )
        .unwrap();
        assert_eq!(decision.kind, ReloadKind::AmmoSwitch);
        assert_eq!(decision.return_loaded, 7);
        assert_eq!(decision.consume_reserve, 12);
    }

    #[test]
    fn dry_fire_does_not_mutate_the_magazine() {
        let mut magazine = MagazineState {
            ammo_form_id: Some(10),
            loaded: 0,
        };
        assert_eq!(consume_round(&mut magazine), Err(FireBlockReason::Empty));
        assert_eq!(magazine.loaded, 0);
    }
}
