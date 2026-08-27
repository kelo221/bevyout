//! Shared limb restoration policy for Stimpak, doctor, and owned-bed sources.

use serde::{Deserialize, Serialize};

use super::BodyPartId;
use super::body::ALL_BODY_PARTS;
use super::limbs::{LIMB_MAX_MILLI, LimbCondition, LimbState};
use crate::time::GameTime;

/// Instant restoration applied by a targeted Stimpak to one limb.
pub const STIMPAK_RESTORE_MILLI: u32 = 30_000;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MedicalSource {
    TargetedStimpak,
    Doctor,
    OwnedBed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestorationOutcome {
    pub source: MedicalSource,
    pub at: GameTime,
    pub restored_milli: u32,
}

/// Restores one or all limbs from a medical source. Owned-bed callers pass
/// explicit [`GameTime`]; Wave 9 owns when that source becomes legal.
pub fn restore_limbs(
    state: &mut LimbState,
    source: MedicalSource,
    part: Option<BodyPartId>,
    now: GameTime,
) -> RestorationOutcome {
    let targets: Vec<BodyPartId> = match (source, part) {
        (MedicalSource::TargetedStimpak, Some(part)) => vec![part],
        (MedicalSource::TargetedStimpak, None) => vec![BodyPartId::Torso],
        (_, Some(part)) => vec![part],
        (_, None) => ALL_BODY_PARTS.to_vec(),
    };
    let mut restored_milli = 0;
    for target in targets {
        restored_milli += restore_one(state.part_mut(target), source);
    }
    RestorationOutcome {
        source,
        at: now,
        restored_milli,
    }
}

fn restore_one(condition: &mut LimbCondition, source: MedicalSource) -> u32 {
    let before = condition.current_milli;
    let after = match source {
        MedicalSource::TargetedStimpak => before
            .saturating_add(STIMPAK_RESTORE_MILLI)
            .min(condition.max_milli.max(LIMB_MAX_MILLI)),
        MedicalSource::Doctor | MedicalSource::OwnedBed => condition.max_milli.max(LIMB_MAX_MILLI),
    };
    condition.current_milli = after;
    condition.max_milli = condition.max_milli.max(LIMB_MAX_MILLI);
    if after > LIMB_CRIPPLE_UNCRIPPLE {
        condition.crippled = false;
    }
    after.saturating_sub(before)
}

/// Any restored current value above the cripple threshold un-cripples.
const LIMB_CRIPPLE_UNCRIPPLE: u32 = 0;

#[cfg(test)]
#[path = "tests/medical.rs"]
mod tests;
