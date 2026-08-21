//! Pure perk definition, eligibility, and active-modifier kernels (M9
//! wave 2, #313).
//!
//! `PerkDefinition` is what the prepared perk catalog (#312) persists:
//! level/rank gates, resolved `GetActorValue` conditions, and typed perk
//! entries. Everything here is deterministic and Bevy-free: eligibility is
//! evaluated against a `stats::CharacterSheet`, and owned perk ranks
//! project onto the wave-1 leveling kernels through `PerkModifiers`
//! (basis points, no floats in decision state beyond authored entry
//! values).

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::actor_state::{ActorSkill, ActorValue, SpecialAttribute};
use crate::stats::CharacterSheet;

/// `CTDA` condition function `GetActorValue`, probed from the real
/// `Fallout3.esm` perk conditions.
pub const CONDITION_FUNCTION_GET_ACTOR_VALUE: u32 = 0x1EF;
/// `CTDA` oper byte of the greater-or-equal comparison every perk
/// `GetActorValue` gate uses in `Fallout3.esm`.
pub const CONDITION_OPER_GREATER_OR_EQUAL: u8 = 0x60;

/// Entry-point code that multiplies awarded XP (Swift Learner's 1.1/1.2/
/// 1.3 by rank).
pub const ENTRY_CODE_XP_AWARD_MULTIPLIER: u8 = 0x09;
/// Entry-point code that adds bonus skill points per level (Educated's
/// +3.0).
pub const ENTRY_CODE_BONUS_SKILL_POINTS: u8 = 0x0A;

/// The neutral XP award multiplier: 10 000 basis points = 1.0x.
pub const NEUTRAL_XP_MULTIPLIER_BPS: u32 = 10_000;

/// Maps the engine-internal actor-value index found in `GetActorValue`
/// perk conditions onto the domain `ActorValue`.
///
/// The index is neither the `AVIF` FormID (`AVStrength` is `0x3E8`) nor
/// the FormID order position; it was verified empirically against the
/// installed GOTY `Fallout3.esm` by pairing every perk `CTDA` with its
/// published requirement: SPECIAL lives at 5..=11 (Strong Back gates on
/// indices 5 and 7 = Strength/Endurance, Swift Learner and Educated on
/// index 9 = Intelligence, Thief on 6 and 10 = Perception/Agility,
/// Better Criticals on 6 and 11 = Perception/Luck) and the thirteen-plus
/// skills at 32..=45 (Master Trader 32 = Barter, Ninja 38/42 =
/// Melee Weapons/Sneak, Computer Whiz 40 = Science, Paralyzing Palm 45 =
/// Unarmed). Indices outside those blocks stay unresolved rather than
/// guessed.
#[must_use]
pub fn actor_value_from_condition_index(index: u32) -> Option<ActorValue> {
    match index {
        5 => Some(ActorValue::Special(SpecialAttribute::Strength)),
        6 => Some(ActorValue::Special(SpecialAttribute::Perception)),
        7 => Some(ActorValue::Special(SpecialAttribute::Endurance)),
        8 => Some(ActorValue::Special(SpecialAttribute::Charisma)),
        9 => Some(ActorValue::Special(SpecialAttribute::Intelligence)),
        10 => Some(ActorValue::Special(SpecialAttribute::Agility)),
        11 => Some(ActorValue::Special(SpecialAttribute::Luck)),
        32 => Some(ActorValue::Skill(ActorSkill::Barter)),
        33 => Some(ActorValue::Skill(ActorSkill::BigGuns)),
        34 => Some(ActorValue::Skill(ActorSkill::EnergyWeapons)),
        35 => Some(ActorValue::Skill(ActorSkill::Explosives)),
        36 => Some(ActorValue::Skill(ActorSkill::Lockpick)),
        37 => Some(ActorValue::Skill(ActorSkill::Medicine)),
        38 => Some(ActorValue::Skill(ActorSkill::MeleeWeapons)),
        39 => Some(ActorValue::Skill(ActorSkill::Repair)),
        40 => Some(ActorValue::Skill(ActorSkill::Science)),
        41 => Some(ActorValue::Skill(ActorSkill::SmallGuns)),
        42 => Some(ActorValue::Skill(ActorSkill::Sneak)),
        43 => Some(ActorValue::Skill(ActorSkill::Speech)),
        44 => Some(ActorValue::Skill(ActorSkill::Throwing)),
        45 => Some(ActorValue::Skill(ActorSkill::Unarmed)),
        _ => None,
    }
}

/// A resolvable perk requirement: the actor value must meet or exceed the
/// threshold. Thresholds are whole units (every observed `Fallout3.esm`
/// gate is integral).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PerkCondition {
    pub actor_value: ActorValue,
    pub threshold: u8,
}

/// The `EPFD` payload of an entry-point perk entry. Only `EPFT == 1`
/// floats are interpreted (XP multipliers, skill-point bonuses, damage
/// multipliers); every other shape is carried raw.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EntryPointPayload {
    Value(f32),
    Raw(u32),
    None,
}

/// One `PRKE`..`PRKF` perk entry. `rank` is the 0-based perk rank the
/// entry belongs to (Swift Learner's three entries sit at ranks 0..2).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PerkEntry {
    /// Quest perk: the quest FormID plus one undecoded wire word. Quest
    /// scripts are out of scope for wave 2 (wave 3+ material).
    Quest {
        rank: u8,
        quest_form_id: u32,
        unknown: u32,
    },
    /// Ability perk: the `SPEL` FormID granted while owned. Active
    /// effects execute in wave 3.
    Ability { rank: u8, spell_form_id: u32 },
    /// Entry point: the engine entry code plus its `EPFT`/`EPFD`
    /// parameter. Only codes `0x09`/`0x0A` are interpreted by the
    /// modifier projection; others are stored for later waves.
    EntryPoint {
        rank: u8,
        code: u8,
        param_count: u8,
        priority: u8,
        payload: EntryPointPayload,
    },
}

/// A decoded perk, exactly what the prepared perk catalog persists.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct PerkDefinition {
    pub form_id: u32,
    pub editor_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub min_level: u8,
    pub ranks: u8,
    pub playable: bool,
    pub hidden: bool,
    /// Resolved `GetActorValue` conditions; all must pass.
    pub conditions: Vec<PerkCondition>,
    /// `CTDA` conditions the decoder could not resolve (other functions,
    /// operators, or unmapped actor-value indices). Any nonzero count
    /// blocks eligibility: the engine would run them, so guessing pass
    /// would fabricate requirements.
    pub unknown_conditions: u32,
    pub entries: Vec<PerkEntry>,
}

/// Owned perk ranks keyed by perk FormID. This is the authoritative
/// progression state; `active_perk_modifiers` projects it.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PerkProgression(pub BTreeMap<u32, u8>);

impl PerkProgression {
    /// Currently owned rank of a perk (0 = not owned).
    #[must_use]
    pub fn rank(&self, form_id: u32) -> u8 {
        self.0.get(&form_id).copied().unwrap_or(0)
    }

    /// Sets the owned rank; rank 0 removes the perk.
    pub fn set_rank(&mut self, form_id: u32, rank: u8) {
        if rank == 0 {
            self.0.remove(&form_id);
        } else {
            self.0.insert(form_id, rank);
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Why a perk cannot be taken right now.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PerkBlockReason {
    MinLevel {
        required: u8,
        current: u8,
    },
    MaxRanksReached {
        ranks: u8,
    },
    ConditionNotMet {
        actor_value: ActorValue,
        required: u8,
        actual: u8,
    },
    /// A condition the evaluator cannot run (unsupported function, oper,
    /// or actor value); conservative block, matching engine behavior of
    /// failing unknown conditions.
    UnknownCondition,
}

impl PerkBlockReason {
    /// Stable machine identifier for console/JSON surfaces.
    #[must_use]
    pub fn kind(&self) -> &'static str {
        match self {
            Self::MinLevel { .. } => "min_level",
            Self::MaxRanksReached { .. } => "max_ranks",
            Self::ConditionNotMet { .. } => "condition",
            Self::UnknownCondition => "unknown_condition",
        }
    }
}

impl fmt::Display for PerkBlockReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MinLevel { required, current } => {
                write!(f, "requires level {required} (player is {current})")
            }
            Self::MaxRanksReached { ranks } => write!(f, "all {ranks} ranks already owned"),
            Self::ConditionNotMet {
                actor_value,
                required,
                actual,
            } => write!(
                f,
                "requires {} {required} (player has {actual})",
                actor_value.label()
            ),
            Self::UnknownCondition => write!(f, "has a condition this build cannot evaluate"),
        }
    }
}

/// Result of the perk eligibility evaluator.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PerkEligibility {
    Eligible,
    Blocked(Vec<PerkBlockReason>),
}

impl PerkEligibility {
    #[must_use]
    pub fn is_eligible(&self) -> bool {
        matches!(self, Self::Eligible)
    }

    #[must_use]
    pub fn reasons(&self) -> &[PerkBlockReason] {
        match self {
            Self::Eligible => &[],
            Self::Blocked(reasons) => reasons,
        }
    }
}

/// Evaluates whether the sheet's owner may take the next rank of `def`:
/// level gate, rank gate, every resolved condition, and no unknown
/// conditions.
#[must_use]
pub fn can_take_perk(
    sheet: &CharacterSheet,
    def: &PerkDefinition,
    progression: &PerkProgression,
) -> PerkEligibility {
    let mut reasons = Vec::new();
    if def.min_level > sheet.level {
        reasons.push(PerkBlockReason::MinLevel {
            required: def.min_level,
            current: sheet.level,
        });
    }
    let owned = progression.rank(def.form_id);
    if def.ranks == 0 || owned >= def.ranks {
        reasons.push(PerkBlockReason::MaxRanksReached { ranks: def.ranks });
    }
    for condition in &def.conditions {
        let Some(actual) = sheet_actor_value(sheet, condition.actor_value) else {
            reasons.push(PerkBlockReason::UnknownCondition);
            continue;
        };
        if actual < condition.threshold {
            reasons.push(PerkBlockReason::ConditionNotMet {
                actor_value: condition.actor_value,
                required: condition.threshold,
                actual,
            });
        }
    }
    if def.unknown_conditions > 0 {
        reasons.push(PerkBlockReason::UnknownCondition);
    }
    if reasons.is_empty() {
        PerkEligibility::Eligible
    } else {
        PerkEligibility::Blocked(reasons)
    }
}

/// Reads a condition actor value off the sheet; SPECIAL through effective
/// values and skills through skill totals.
fn sheet_actor_value(sheet: &CharacterSheet, value: ActorValue) -> Option<u8> {
    match value {
        ActorValue::Special(attribute) => Some(sheet.effective_special(attribute)),
        ActorValue::Skill(skill) => Some(sheet.skill_value(skill)),
        _ => None,
    }
}

/// Active perk effects the wave-1 leveling kernels consume. Defaults are
/// neutral (multiplier 1.0x, no bonus points).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PerkModifiers {
    /// Multiplier applied to awarded XP in basis points; 10 000 = 1.0x.
    pub xp_award_multiplier_bps: u32,
    /// Extra skill points granted per level-up (Educated).
    pub bonus_skill_points: u16,
}

impl Default for PerkModifiers {
    fn default() -> Self {
        Self {
            xp_award_multiplier_bps: NEUTRAL_XP_MULTIPLIER_BPS,
            bonus_skill_points: 0,
        }
    }
}

/// Projects owned perk ranks onto the leveling modifiers. A perk grants
/// exactly the entries of its OWNED rank (0-based entry rank = owned rank
/// minus one), so Swift Learner rank 2 yields the 1.2x entry, never
/// 1.1x * 1.2x. Multiple distinct perks multiply; bonus skill points add.
#[must_use]
pub fn active_perk_modifiers(
    progression: &PerkProgression,
    defs: &BTreeMap<u32, PerkDefinition>,
) -> PerkModifiers {
    let mut xp_product = 1.0_f64;
    let mut bonus_points = 0_u16;
    for (&form_id, &owned) in &progression.0 {
        let Some(def) = defs.get(&form_id) else {
            continue;
        };
        // Rank N (1-based ownership) grants entries authored at 0-based
        // rank N-1.
        let Some(entry_rank) = owned.checked_sub(1) else {
            continue;
        };
        for entry in &def.entries {
            let PerkEntry::EntryPoint {
                code,
                rank,
                payload,
                ..
            } = entry
            else {
                continue;
            };
            if *rank != entry_rank {
                continue;
            }
            let EntryPointPayload::Value(value) = payload else {
                continue;
            };
            if *code == ENTRY_CODE_XP_AWARD_MULTIPLIER {
                xp_product *= f64::from(*value);
            } else if *code == ENTRY_CODE_BONUS_SKILL_POINTS {
                bonus_points = bonus_points.saturating_add(value.round().max(0.0) as u16);
            }
        }
    }
    let scaled = (xp_product * f64::from(NEUTRAL_XP_MULTIPLIER_BPS)).round();
    PerkModifiers {
        xp_award_multiplier_bps: scaled.clamp(0.0, u32::MAX as f64) as u32,
        bonus_skill_points: bonus_points,
    }
}

#[cfg(test)]
#[path = "tests/perks.rs"]
mod tests;
