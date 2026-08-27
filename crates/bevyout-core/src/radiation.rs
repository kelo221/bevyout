//! Pure radiation pool and threshold-penalty kernel (M9 wave 3, #317).
//!
//! Radiation accumulates in whole rads `0..=1000`. The vanilla threshold
//! table (roadmap `M9_Start.md` wave 3) maps accumulated rads onto SPECIAL
//! penalties; 1000 rads is fatal. All decisions are integer rads; the rad
//! *resistance* math lives at the caller in basis points through
//! `stats::clamp_resistance_bps`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::actor_state::SpecialAttribute;

/// Hard radiation cap; also the fatal threshold.
pub const RADIATION_MAX: u16 = 1000;

/// Threshold table entry: penalties begin at `threshold_rads`.
struct RadiationThreshold {
    threshold_rads: u16,
    endurance: i8,
    agility: i8,
    strength: i8,
    intelligence: i8,
}

/// Vanilla Fallout 3 thresholds: Minor 200 (-1 END), Advanced 400
/// (-2 END, -1 AGI), Critical 600 (-3 END, -2 AGI, -1 STR), Deadly 800
/// (-3 END, -2 AGI, -2 STR, -1 INT). 1000 is fatal (see `is_fatal`).
const THRESHOLDS: [RadiationThreshold; 4] = [
    RadiationThreshold {
        threshold_rads: 200,
        endurance: -1,
        agility: 0,
        strength: 0,
        intelligence: 0,
    },
    RadiationThreshold {
        threshold_rads: 400,
        endurance: -2,
        agility: -1,
        strength: 0,
        intelligence: 0,
    },
    RadiationThreshold {
        threshold_rads: 600,
        endurance: -3,
        agility: -2,
        strength: -1,
        intelligence: 0,
    },
    RadiationThreshold {
        threshold_rads: 800,
        endurance: -3,
        agility: -2,
        strength: -2,
        intelligence: -1,
    },
];

/// The player's accumulated radiation dose. Clamped to `0..=RADIATION_MAX`.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct RadiationPool {
    pub rads: u16,
}

impl RadiationPool {
    #[must_use]
    pub fn new(rads: u16) -> Self {
        Self {
            rads: rads.min(RADIATION_MAX),
        }
    }

    /// Highest reached threshold in rads, or 0 while below Minor.
    #[must_use]
    pub fn threshold_reached(&self) -> u16 {
        threshold_reached(self.rads)
    }

    /// True at the fatal 1000-rads dose.
    #[must_use]
    pub fn is_fatal(&self) -> bool {
        is_fatal(self.rads)
    }
}

/// Highest reached threshold in rads (0, 200, 400, 600, or 800).
#[must_use]
pub fn threshold_reached(rads: u16) -> u16 {
    THRESHOLDS
        .iter()
        .rev()
        .find(|threshold| rads >= threshold.threshold_rads)
        .map_or(0, |threshold| threshold.threshold_rads)
}

/// True at the fatal dose of exactly `RADIATION_MAX` rads.
#[must_use]
pub fn is_fatal(rads: u16) -> bool {
    rads >= RADIATION_MAX
}

/// SPECIAL penalties for an accumulated dose: the highest reached
/// threshold's penalties (thresholds replace, not stack). Zero entries are
/// omitted from the map.
#[must_use]
pub fn radiation_penalties(rads: u16) -> BTreeMap<SpecialAttribute, i8> {
    let mut penalties = BTreeMap::new();
    let Some(threshold) = THRESHOLDS
        .iter()
        .rev()
        .find(|threshold| rads >= threshold.threshold_rads)
    else {
        return penalties;
    };
    let mut insert = |attribute: SpecialAttribute, penalty: i8| {
        if penalty != 0 {
            penalties.insert(attribute, penalty);
        }
    };
    insert(SpecialAttribute::Endurance, threshold.endurance);
    insert(SpecialAttribute::Agility, threshold.agility);
    insert(SpecialAttribute::Strength, threshold.strength);
    insert(SpecialAttribute::Intelligence, threshold.intelligence);
    penalties
}

/// Outcome of one dose application.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct RadiationDoseOutcome {
    /// Dose actually absorbed after resistance (whole rads).
    pub absorbed_rads: u16,
    pub rads: u16,
    /// True when this application crossed into the fatal dose.
    pub fatal: bool,
}

/// Applies one radiation dose reduced by resistance. `resistance_bps` is
/// the actor's total rad resistance in basis points (armor + perks +
/// chems, typically already clamped by `stats::clamp_resistance_bps`):
/// absorbed = dose * (1 - resist/10000), rounded to the nearest whole rad.
/// Cure doses (RadAway) call `remove_rads` instead.
pub fn apply_radiation(
    pool: &mut RadiationPool,
    dose_rads: u16,
    resistance_bps: u32,
) -> RadiationDoseOutcome {
    let resisted = (u32::from(dose_rads).saturating_mul(resistance_bps.min(10_000))) / 10_000;
    let absorbed = u32::from(dose_rads).saturating_sub(resisted) as u16;
    pool.rads = pool.rads.saturating_add(absorbed).min(RADIATION_MAX);
    RadiationDoseOutcome {
        absorbed_rads: absorbed,
        rads: pool.rads,
        fatal: pool.is_fatal(),
    }
}

/// Removes up to `rads` from the pool (RadAway, `removerads`); never below
/// zero. Returns the dose actually removed.
pub fn remove_rads(pool: &mut RadiationPool, rads: u16) -> u16 {
    let removed = pool.rads.min(rads);
    pool.rads -= removed;
    removed
}

#[cfg(test)]
#[path = "tests/radiation.rs"]
mod tests;
