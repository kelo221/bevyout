//! Pure weapon condition, degradation, and jam policy for M5 Wave 3.

use super::rng::CombatRngDraw;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

pub const COMBAT_POLICY_REVISION: &str = "m5-combat-v3";
pub const MIN_DAMAGE_EFFECTIVENESS: f32 = 0.25;
pub const MAX_JAM_CHANCE_BASIS_POINTS: u32 = 2_500;
pub const BASIS_POINT_DENOMINATOR: u32 = 10_000;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum JamReason {
    Fire,
    Reload,
}

impl JamReason {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fire => "fire",
            Self::Reload => "reload",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeaponConditionPolicy {
    max_condition: Option<u32>,
    degradation_per_shot: u32,
}

impl Default for WeaponConditionPolicy {
    fn default() -> Self {
        Self::new(None)
    }
}

impl WeaponConditionPolicy {
    #[must_use]
    pub const fn new(max_condition: Option<u32>) -> Self {
        let degradation_per_shot = match max_condition {
            Some(max) if max > 0 => {
                let scaled = max / 100;
                if scaled == 0 { 1 } else { scaled }
            }
            _ => 0,
        };
        Self {
            max_condition,
            degradation_per_shot,
        }
    }

    #[must_use]
    pub const fn with_degradation(max_condition: Option<u32>, degradation_per_shot: u32) -> Self {
        Self {
            max_condition,
            degradation_per_shot,
        }
    }

    #[must_use]
    pub const fn max_condition(self) -> Option<u32> {
        self.max_condition
    }

    #[must_use]
    pub const fn degradation_per_shot(self) -> u32 {
        self.degradation_per_shot
    }

    pub fn evaluate_fire(
        self,
        base_damage: f32,
        current_condition: Option<u32>,
        draw: CombatRngDraw,
    ) -> Result<ConditionDecision, ConditionError> {
        self.evaluate(base_damage, current_condition, draw, JamReason::Fire)
    }

    pub fn evaluate_reload(
        self,
        current_condition: Option<u32>,
        draw: CombatRngDraw,
    ) -> Result<JamDecision, ConditionError> {
        let condition = self.normalized_condition(current_condition)?;
        Ok(JamDecision {
            condition,
            jam_chance_basis_points: self.jam_chance_basis_points(condition),
            jammed: self.roll_jams(draw, condition),
        })
    }

    pub fn normalized_condition(
        self,
        current_condition: Option<u32>,
    ) -> Result<Option<u32>, ConditionError> {
        let Some(max) = self.max_condition else {
            return Ok(None);
        };
        if max == 0 {
            return Err(ConditionError::InvalidMaximum);
        }
        Ok(Some(current_condition.unwrap_or(max).min(max)))
    }

    fn evaluate(
        self,
        base_damage: f32,
        current_condition: Option<u32>,
        draw: CombatRngDraw,
        reason: JamReason,
    ) -> Result<ConditionDecision, ConditionError> {
        if !base_damage.is_finite() || base_damage <= 0.0 {
            return Err(ConditionError::InvalidDamage(base_damage));
        }
        let condition_before = self.normalized_condition(current_condition)?;
        let damage_multiplier = self.damage_multiplier(condition_before);
        let damage = base_damage * damage_multiplier;
        if !damage.is_finite() || damage <= 0.0 {
            return Err(ConditionError::InvalidDamage(damage));
        }
        let condition_after =
            condition_before.map(|condition| condition.saturating_sub(self.degradation_per_shot));
        let jam_chance_basis_points = self.jam_chance_basis_points(condition_before);
        Ok(ConditionDecision {
            reason,
            condition_before,
            condition_after,
            damage_multiplier,
            damage,
            degradation: self.degradation_per_shot,
            jam_chance_basis_points,
            jammed: self.roll_jams(draw, condition_before),
        })
    }

    #[must_use]
    pub fn damage_multiplier(self, condition: Option<u32>) -> f32 {
        let Some(max) = self.max_condition.filter(|max| *max > 0) else {
            return 1.0;
        };
        let current = condition.unwrap_or(max).min(max);
        MIN_DAMAGE_EFFECTIVENESS + (1.0 - MIN_DAMAGE_EFFECTIVENESS) * (current as f32 / max as f32)
    }

    #[must_use]
    pub fn jam_chance_basis_points(self, condition: Option<u32>) -> u32 {
        let Some(max) = self.max_condition.filter(|max| *max > 0) else {
            return 0;
        };
        let current = condition.unwrap_or(max).min(max);
        ((max - current) as u64 * u64::from(MAX_JAM_CHANCE_BASIS_POINTS) / u64::from(max)) as u32
    }

    #[must_use]
    pub fn roll_jams(self, draw: CombatRngDraw, condition: Option<u32>) -> bool {
        let threshold = self.jam_chance_basis_points(condition);
        threshold > 0 && draw.value % BASIS_POINT_DENOMINATOR < threshold
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConditionDecision {
    pub reason: JamReason,
    pub condition_before: Option<u32>,
    pub condition_after: Option<u32>,
    pub damage_multiplier: f32,
    pub damage: f32,
    pub degradation: u32,
    pub jam_chance_basis_points: u32,
    pub jammed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JamDecision {
    pub condition: Option<u32>,
    pub jam_chance_basis_points: u32,
    pub jammed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConditionError {
    InvalidMaximum,
    InvalidDamage(f32),
}

impl fmt::Display for ConditionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMaximum => write!(formatter, "weapon maximum condition must be non-zero"),
            Self::InvalidDamage(amount) => {
                write!(
                    formatter,
                    "weapon damage must be positive and finite, got {amount}"
                )
            }
        }
    }
}

impl Error for ConditionError {}

#[cfg(test)]
#[path = "tests/condition.rs"]
mod tests;
