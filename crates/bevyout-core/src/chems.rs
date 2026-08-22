//! Pure chem-addiction kernel and deterministic PRNG (M9 wave 3, #317).
//!
//! No `rand`: `RpgRngState` is a serializable splitmix64 stream whose draw
//! index makes every addiction roll reproducible from a seed. The
//! addiction machine is a per-withdrawal-effect state machine
//! (Clean -> Addicted -> Withdrawing -> cured).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Neutral resist and full-certainty bps anchors.
pub const BPS_MIN: u32 = 0;
pub const BPS_MAX: u32 = 10_000;

/// Core-owned deterministic PRNG state (splitmix64). `draw_index` counts
/// every `draw_bps` call so results can cite their position in the stream
/// for reproducibility evidence.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct RpgRngState {
    pub state: u64,
    pub draw_index: u32,
}

/// The documented startup seed: viewers construct
/// `RpgRngState::new(RPG_RNG_DEFAULT_SEED)` so acceptance runs are
/// reproducible from launch.
pub const RPG_RNG_DEFAULT_SEED: u64 = 0;

impl Default for RpgRngState {
    fn default() -> Self {
        Self::new(RPG_RNG_DEFAULT_SEED)
    }
}

impl RpgRngState {
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self {
            state: seed,
            draw_index: 0,
        }
    }

    /// One splitmix64 step; the full 64-bit stream is what `draw_bps`
    /// samples from.
    #[must_use]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Draws a basis-point value in `0..limit_bps` (limit 10 000 = whole
    /// percent scale). Increments `draw_index` on every draw.
    #[must_use]
    pub fn draw_bps(&mut self, limit_bps: u32) -> u32 {
        let limit = limit_bps.clamp(1, BPS_MAX) as u64;
        let value = (self.next_u64() % limit) as u32;
        self.draw_index = self.draw_index.saturating_add(1);
        value
    }
}

/// Rolls one addiction check: succeeds when the draw lands below the
/// effective chance `chance * (1 - resist)`, all in basis points
/// (`chance_bps` from `IngestibleDefinition::addiction_chance_bps`,
/// `chem_resist_bps` from perks, 0 = none). A zero chance never addicts,
/// matching the engine's requirement of a non-zero Addiction Chance.
pub fn roll_addiction(chance_bps: u32, chem_resist_bps: u32, rng: &mut RpgRngState) -> bool {
    if chance_bps == 0 {
        return false;
    }
    let resist = chem_resist_bps.min(BPS_MAX);
    let effective =
        (u64::from(chance_bps.min(BPS_MAX)) * u64::from(BPS_MAX - resist)) / u64::from(BPS_MAX);
    rng.draw_bps(BPS_MAX) < effective as u32
}

/// One chem's addiction lifecycle, keyed by the withdrawal SPEL FormID the
/// ingestible's `ENIT` names (that FormID is the addiction's identity
/// across chems sharing an effect, e.g. all mentats flavors).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AddictionPhase {
    /// No addiction.
    Clean,
    /// Addicted; the chem's buff is still running.
    Addicted,
    /// Addicted; the chem wore off and withdrawal penalties apply until
    /// cured or re-dosed.
    Withdrawing,
}

impl AddictionPhase {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::Addicted => "addicted",
            Self::Withdrawing => "withdrawing",
        }
    }
}

/// Player addiction state: withdrawal FormID -> phase. Empty = clean.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Addictions(pub BTreeMap<u32, AddictionPhase>);

impl Addictions {
    /// Records a won addiction roll for `withdrawal_form_id`.
    pub fn addict(&mut self, withdrawal_form_id: u32) {
        if withdrawal_form_id != 0 {
            self.0.insert(withdrawal_form_id, AddictionPhase::Addicted);
        }
    }

    /// True while `withdrawal_form_id` is addicted or withdrawing.
    #[must_use]
    pub fn is_addicted(&self, withdrawal_form_id: u32) -> bool {
        matches!(
            self.0.get(&withdrawal_form_id),
            Some(AddictionPhase::Addicted) | Some(AddictionPhase::Withdrawing)
        )
    }

    /// Moves an addicted entry into withdrawal (the chem's effects just
    /// expired). Returns true when the transition happened.
    pub fn begin_withdrawal(&mut self, withdrawal_form_id: u32) -> bool {
        if matches!(
            self.0.get(&withdrawal_form_id),
            Some(AddictionPhase::Addicted)
        ) {
            self.0
                .insert(withdrawal_form_id, AddictionPhase::Withdrawing);
            true
        } else {
            false
        }
    }

    /// Cures one addiction. Returns true when an entry was removed.
    pub fn cure(&mut self, withdrawal_form_id: u32) -> bool {
        self.0.remove(&withdrawal_form_id).is_some()
    }

    /// Cures every addiction; returns how many were removed.
    pub fn cure_all(&mut self) -> usize {
        let count = self.0.len();
        self.0.clear();
        count
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
#[path = "tests/chems.rs"]
mod tests;
