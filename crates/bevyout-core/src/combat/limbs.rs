//! Limb health, cripple transitions, and derived combat/locomotion penalties.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::body::{ALL_BODY_PARTS, BodyPartId};
use crate::perception::TargetId;

/// Full limb condition in milli-units (`100_000` = 100.000).
pub const LIMB_MAX_MILLI: u32 = 100_000;
/// A limb is crippled once current condition reaches this value.
pub const LIMB_CRIPPLE_THRESHOLD_MILLI: u32 = 0;

pub const LOCOMOTION_FULL_BPS: u32 = 10_000;
pub const LOCOMOTION_ONE_LEG_BPS: u32 = 6_000;
pub const LOCOMOTION_TWO_LEG_BPS: u32 = 4_000;

pub const ARM_RELOAD_PER_CRIPPLE_BPS: u32 = 5_000;
pub const ARM_SPREAD_PER_CRIPPLE_BPS: u32 = 2_500;
pub const HEAD_PERCEPTION_PENALTY: i8 = -4;

/// Stable identity for one accepted weapon shot.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShotId(pub u64);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct LimbCondition {
    pub current_milli: u32,
    pub max_milli: u32,
    pub crippled: bool,
}

impl Default for LimbCondition {
    fn default() -> Self {
        Self {
            current_milli: LIMB_MAX_MILLI,
            max_milli: LIMB_MAX_MILLI,
            crippled: false,
        }
    }
}

impl LimbCondition {
    #[must_use]
    pub fn fraction(self) -> f32 {
        if self.max_milli == 0 {
            0.0
        } else {
            self.current_milli as f32 / self.max_milli as f32
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct LimbState {
    pub parts: BTreeMap<BodyPartId, LimbCondition>,
    #[serde(default)]
    pub applied_shots: BTreeSet<ShotId>,
}

impl Default for LimbState {
    fn default() -> Self {
        Self::healthy()
    }
}

impl LimbState {
    #[must_use]
    pub fn healthy() -> Self {
        Self {
            parts: ALL_BODY_PARTS
                .into_iter()
                .map(|part| (part, LimbCondition::default()))
                .collect(),
            applied_shots: BTreeSet::new(),
        }
    }

    #[must_use]
    pub fn part(&self, id: BodyPartId) -> LimbCondition {
        self.parts.get(&id).copied().unwrap_or_default()
    }

    pub fn part_mut(&mut self, id: BodyPartId) -> &mut LimbCondition {
        self.parts.entry(id).or_default()
    }

    #[must_use]
    pub fn crippled_count(&self, parts: impl IntoIterator<Item = BodyPartId>) -> u32 {
        parts
            .into_iter()
            .filter(|part| self.part(*part).crippled)
            .count() as u32
    }

    /// Ground locomotion projection: 100% / 60% / 40% for zero / one / two
    /// crippled legs.
    #[must_use]
    pub fn locomotion_speed_bps(&self) -> u32 {
        match self.crippled_count([BodyPartId::LeftLeg, BodyPartId::RightLeg]) {
            0 => LOCOMOTION_FULL_BPS,
            1 => LOCOMOTION_ONE_LEG_BPS,
            _ => LOCOMOTION_TWO_LEG_BPS,
        }
    }

    #[must_use]
    pub fn arm_reload_multiplier_bps(&self) -> u32 {
        LOCOMOTION_FULL_BPS
            + ARM_RELOAD_PER_CRIPPLE_BPS
                * self.crippled_count([BodyPartId::LeftArm, BodyPartId::RightArm])
    }

    #[must_use]
    pub fn arm_spread_penalty_bps(&self) -> u32 {
        ARM_SPREAD_PER_CRIPPLE_BPS
            * self.crippled_count([BodyPartId::LeftArm, BodyPartId::RightArm])
    }

    #[must_use]
    pub fn head_perception_penalty(&self) -> i8 {
        if self.part(BodyPartId::Head).crippled {
            HEAD_PERCEPTION_PENALTY
        } else {
            0
        }
    }

    #[must_use]
    pub fn requests_head_blur(&self) -> bool {
        self.part(BodyPartId::Head).crippled
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LimbImpact {
    pub shot_id: ShotId,
    pub target: TargetId,
    pub part: BodyPartId,
    pub final_damage_milli: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LimbImpactOutcome {
    pub part: BodyPartId,
    pub previous_milli: u32,
    pub remaining_milli: u32,
    pub newly_crippled: bool,
    pub already_crippled: bool,
    pub duplicate: bool,
    pub head_blur: bool,
}

impl LimbImpactOutcome {
    #[must_use]
    fn duplicate(part: BodyPartId, condition: LimbCondition) -> Self {
        Self {
            part,
            previous_milli: condition.current_milli,
            remaining_milli: condition.current_milli,
            newly_crippled: false,
            already_crippled: condition.crippled,
            duplicate: true,
            head_blur: false,
        }
    }
}

/// Applies limb damage after health damage has already been resolved. Duplicate
/// `ShotId`s are rejected without mutating condition.
pub fn apply_limb_impact(state: &mut LimbState, impact: LimbImpact) -> LimbImpactOutcome {
    let existing = state.part(impact.part);
    if !state.applied_shots.insert(impact.shot_id) {
        return LimbImpactOutcome::duplicate(impact.part, existing);
    }
    let previous_milli = existing.current_milli;
    let already_crippled = existing.crippled;
    let remaining = previous_milli.saturating_sub(impact.final_damage_milli);
    let newly_crippled = !already_crippled && remaining == LIMB_CRIPPLE_THRESHOLD_MILLI;
    let condition = LimbCondition {
        current_milli: remaining,
        max_milli: existing.max_milli.max(LIMB_MAX_MILLI),
        crippled: already_crippled || newly_crippled,
    };
    *state.part_mut(impact.part) = condition;
    LimbImpactOutcome {
        part: impact.part,
        previous_milli,
        remaining_milli: remaining,
        newly_crippled,
        already_crippled,
        duplicate: false,
        head_blur: newly_crippled && impact.part == BodyPartId::Head,
    }
}

#[cfg(test)]
#[path = "tests/limbs.rs"]
mod tests;
