//! Versioned deterministic randomness for combat decisions.
//!
//! Combat does not use the viewer frame clock, ECS entity IDs, or a process
//! global random generator. A draw is identified by its domain and the
//! serialized draw index, so the same seed and accepted-intent sequence gives
//! the same result after save/load.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

pub const COMBAT_RNG_REVISION: &str = "m5-combat-rng-v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CombatRngDomain {
    FireJam,
    ReloadJam,
}

impl CombatRngDomain {
    const fn salt(self) -> u64 {
        match self {
            Self::FireJam => 0x4649_5245_4a41_4d01,
            Self::ReloadJam => 0x5245_4c44_4a41_4d02,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CombatRngDraw {
    pub domain: CombatRngDomain,
    pub index: u64,
    pub value: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CombatRngState {
    pub revision: String,
    pub seed: u64,
    pub draw_index: u64,
}

impl Default for CombatRngState {
    fn default() -> Self {
        Self::from_seed(0)
    }
}

impl CombatRngState {
    #[must_use]
    pub fn from_seed(seed: u64) -> Self {
        Self {
            revision: COMBAT_RNG_REVISION.into(),
            seed,
            draw_index: 0,
        }
    }

    pub fn validate(&self) -> Result<(), CombatRngError> {
        if self.revision != COMBAT_RNG_REVISION {
            return Err(CombatRngError::UnsupportedRevision(self.revision.clone()));
        }
        Ok(())
    }

    /// Reserves one draw. Callers must invoke this only after validating that
    /// the action is otherwise accepted; rejected intents never advance the
    /// serialized index.
    pub fn draw(&mut self, domain: CombatRngDomain) -> Result<CombatRngDraw, CombatRngError> {
        self.validate()?;
        let index = self.draw_index;
        let mixed =
            splitmix64(self.seed ^ domain.salt() ^ index.wrapping_mul(0x9e37_79b9_7f4a_7c15));
        self.draw_index = self.draw_index.saturating_add(1);
        Ok(CombatRngDraw {
            domain,
            index,
            value: (mixed >> 32) as u32,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CombatRngError {
    UnsupportedRevision(String),
}

impl fmt::Display for CombatRngError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedRevision(revision) => {
                write!(formatter, "unsupported combat RNG revision {revision}")
            }
        }
    }
}

impl Error for CombatRngError {}

const fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[cfg(test)]
#[path = "tests/rng.rs"]
mod tests;
