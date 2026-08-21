//! Pure SPECIAL, skill, derived-attribute, and leveling kernels (M9 wave 1).
//!
//! Everything here is deterministic Bevy-free math driven by named GMST
//! settings with Fallout 3 GOTY defaults. Probabilities are expressed in
//! basis points (`0..=10_000`) and collections are ordered so serialized
//! state never depends on iteration order.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::actor_state::{ActorSkill, SpecialAttribute};

/// The thirteen Fallout 3 player skills. `ActorSkill::Throwing` appears in
/// imported record data but is not a Fallout 3 player skill.
pub const FO3_SKILLS: &[ActorSkill] = &[
    ActorSkill::Barter,
    ActorSkill::BigGuns,
    ActorSkill::EnergyWeapons,
    ActorSkill::Explosives,
    ActorSkill::Lockpick,
    ActorSkill::Medicine,
    ActorSkill::MeleeWeapons,
    ActorSkill::Repair,
    ActorSkill::Science,
    ActorSkill::SmallGuns,
    ActorSkill::Sneak,
    ActorSkill::Speech,
    ActorSkill::Unarmed,
];

/// Primary SPECIAL attribute governing a Fallout 3 skill; `None` for skills
/// outside the Fallout 3 thirteen.
#[must_use]
pub fn skill_governing_attribute(skill: ActorSkill) -> Option<SpecialAttribute> {
    match skill {
        ActorSkill::Barter | ActorSkill::Speech => Some(SpecialAttribute::Charisma),
        ActorSkill::BigGuns | ActorSkill::Unarmed => Some(SpecialAttribute::Endurance),
        ActorSkill::EnergyWeapons | ActorSkill::Explosives | ActorSkill::Lockpick => {
            Some(SpecialAttribute::Perception)
        }
        ActorSkill::Medicine | ActorSkill::Repair | ActorSkill::Science => {
            Some(SpecialAttribute::Intelligence)
        }
        ActorSkill::MeleeWeapons => Some(SpecialAttribute::Strength),
        ActorSkill::SmallGuns | ActorSkill::Sneak => Some(SpecialAttribute::Agility),
        ActorSkill::Throwing => None,
    }
}

pub const SPECIAL_MIN: u8 = 1;
pub const SPECIAL_MAX: u8 = 10;
pub const SKILL_MIN: u8 = 0;
pub const SKILL_MAX: u8 = 100;
pub const TAG_SKILL_BONUS: u8 = 15;
/// Luck contributes `ceil(luck / 2)` to every skill base.
#[must_use]
pub fn luck_skill_bonus(luck: u8) -> u8 {
    (luck.clamp(SPECIAL_MIN, SPECIAL_MAX) + 1) / 2
}

/// A decoded `GMST` record value. The variant follows the setting's
/// EditorID prefix (`f`, `i`, `b`, `s`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GmstValue {
    Float(f32),
    Int(i32),
    Bool(bool),
    Str(String),
}

impl GmstValue {
    #[must_use]
    pub fn as_f32(&self) -> Option<f32> {
        match *self {
            Self::Float(value) => Some(value),
            Self::Int(value) => Some(value as f32),
            Self::Bool(_) | Self::Str(_) => None,
        }
    }

    #[must_use]
    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            Self::Int(value) => Some(value),
            Self::Float(value) if value.fract() == 0.0 => Some(value as i32),
            Self::Float(_) | Self::Bool(_) | Self::Str(_) => None,
        }
    }
}

// Canonical GMST setting names consumed by the kernels. The defaults below
// match Fallout 3 GOTY; prepared catalogs override them per content set.
pub const GMST_HEALTH_BASE: &str = "iAVDHealthBase";
pub const GMST_HEALTH_ENDURANCE_MULT: &str = "fAVDHealthEnduranceMult";
pub const GMST_HEALTH_LEVEL_MULT: &str = "iAVDHealthLevelMult";
pub const GMST_ACTION_POINTS_BASE: &str = "iBaseActionPoints";
pub const GMST_ACTION_POINTS_AGILITY_MULT: &str = "fAVDActionPointsAgilityMult";
pub const GMST_CARRY_WEIGHT_BASE: &str = "iAVDCarryWeightBase";
pub const GMST_CARRY_WEIGHT_STRENGTH_MULT: &str = "fAVDCarryWeightStrengthMult";
pub const GMST_MAX_PLAYER_LEVEL: &str = "iMaxPlayerLevel";
pub const GMST_LEVEL_UP_SKILL_POINTS: &str = "iLevelUpSkillPoints";
pub const GMST_XP_BASE: &str = "iXPBase";

/// Named GMST view used by every kernel. Values are copied from a prepared
/// catalog or left at the Fallout 3 GOTY defaults.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct GmstSettings {
    pub health_base: f32,
    pub health_endurance_mult: f32,
    pub health_level_mult: f32,
    pub action_points_base: f32,
    pub action_points_agility_mult: f32,
    pub carry_weight_base: f32,
    pub carry_weight_strength_mult: f32,
    pub max_player_level: u8,
    pub level_up_skill_points: u8,
    pub xp_base: u32,
}

impl Default for GmstSettings {
    fn default() -> Self {
        Self {
            health_base: 100.0,
            health_endurance_mult: 20.0,
            health_level_mult: 10.0,
            action_points_base: 65.0,
            action_points_agility_mult: 3.0,
            carry_weight_base: 150.0,
            carry_weight_strength_mult: 10.0,
            max_player_level: 30,
            level_up_skill_points: 10,
            xp_base: 150,
        }
    }
}

impl GmstSettings {
    /// Builds settings from `(setting name, value)` pairs, keeping the GOTY
    /// default for absent, mistyped, or non-finite entries. Matching against
    /// setting names is ASCII case-insensitive.
    #[must_use]
    pub fn from_pairs<'a>(pairs: impl Iterator<Item = (&'a str, GmstValue)>) -> Self {
        let mut settings = Self::default();
        for (name, value) in pairs {
            let matches = |expected: &str| name.eq_ignore_ascii_case(expected);
            if matches(GMST_HEALTH_BASE) {
                if let Some(v) = finite(value.as_f32()) {
                    settings.health_base = v;
                }
            } else if matches(GMST_HEALTH_ENDURANCE_MULT) {
                if let Some(v) = finite(value.as_f32()) {
                    settings.health_endurance_mult = v;
                }
            } else if matches(GMST_HEALTH_LEVEL_MULT) {
                if let Some(v) = finite(value.as_f32()) {
                    settings.health_level_mult = v;
                }
            } else if matches(GMST_ACTION_POINTS_BASE) {
                if let Some(v) = finite(value.as_f32()) {
                    settings.action_points_base = v;
                }
            } else if matches(GMST_ACTION_POINTS_AGILITY_MULT) {
                if let Some(v) = finite(value.as_f32()) {
                    settings.action_points_agility_mult = v;
                }
            } else if matches(GMST_CARRY_WEIGHT_BASE) {
                if let Some(v) = finite(value.as_f32()) {
                    settings.carry_weight_base = v;
                }
            } else if matches(GMST_CARRY_WEIGHT_STRENGTH_MULT) {
                if let Some(v) = finite(value.as_f32()) {
                    settings.carry_weight_strength_mult = v;
                }
            } else if matches(GMST_MAX_PLAYER_LEVEL) {
                if let Some(v) = value.as_i32() {
                    settings.max_player_level = v.clamp(1, 99) as u8;
                }
            } else if matches(GMST_LEVEL_UP_SKILL_POINTS) {
                if let Some(v) = value.as_i32() {
                    settings.level_up_skill_points = v.clamp(0, 100) as u8;
                }
            } else if matches(GMST_XP_BASE) {
                if let Some(v) = value.as_i32() {
                    settings.xp_base = v.clamp(1, 1_000_000) as u32;
                }
            }
        }
        settings
    }

    /// Rejects non-finite multipliers and out-of-range discrete settings at
    /// catalog-load boundaries. Kernels themselves stay total.
    pub fn validate(&self) -> Result<(), StatsError> {
        let finite = [
            ("health_base", self.health_base),
            ("health_endurance_mult", self.health_endurance_mult),
            ("health_level_mult", self.health_level_mult),
            ("action_points_base", self.action_points_base),
            (
                "action_points_agility_mult",
                self.action_points_agility_mult,
            ),
            ("carry_weight_base", self.carry_weight_base),
            (
                "carry_weight_strength_mult",
                self.carry_weight_strength_mult,
            ),
        ];
        for (name, value) in finite {
            if !value.is_finite() {
                return Err(StatsError::NonFiniteSetting(name));
            }
            if value < 0.0 {
                return Err(StatsError::NegativeSetting(name));
            }
        }
        if self.max_player_level == 0 || self.max_player_level > 99 {
            return Err(StatsError::LevelCapOutOfRange(self.max_player_level));
        }
        if self.xp_base == 0 {
            return Err(StatsError::XpBaseOutOfRange(self.xp_base));
        }
        Ok(())
    }
}

fn finite(value: Option<f32>) -> Option<f32> {
    value.filter(|v| v.is_finite() && *v >= 0.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatsError {
    NonFiniteSetting(&'static str),
    NegativeSetting(&'static str),
    LevelCapOutOfRange(u8),
    XpBaseOutOfRange(u32),
}

impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteSetting(name) => write!(f, "gmst setting {name} is not finite"),
            Self::NegativeSetting(name) => write!(f, "gmst setting {name} is negative"),
            Self::LevelCapOutOfRange(level) => {
                write!(f, "max player level {level} outside 1..=99")
            }
            Self::XpBaseOutOfRange(base) => write!(f, "xp base {base} outside 1..=1_000_000"),
        }
    }
}

impl Error for StatsError {}

/// Authoritative player-authored progression state. Derived values are never
/// stored here; they are recomputed from this sheet plus active modifiers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CharacterSheet {
    pub special: BTreeMap<SpecialAttribute, u8>,
    pub tagged_skills: BTreeSet<ActorSkill>,
    pub skill_increases: BTreeMap<ActorSkill, i16>,
    pub level: u8,
    pub xp: u32,
}

impl Default for CharacterSheet {
    fn default() -> Self {
        Self {
            special: [
                SpecialAttribute::Strength,
                SpecialAttribute::Perception,
                SpecialAttribute::Endurance,
                SpecialAttribute::Charisma,
                SpecialAttribute::Intelligence,
                SpecialAttribute::Agility,
                SpecialAttribute::Luck,
            ]
            .into_iter()
            .map(|attribute| (attribute, 5))
            .collect(),
            tagged_skills: BTreeSet::new(),
            skill_increases: BTreeMap::new(),
            level: 1,
            xp: 0,
        }
    }
}

impl CharacterSheet {
    /// Effective SPECIAL, clamped to `1..=10`.
    #[must_use]
    pub fn effective_special(&self, attribute: SpecialAttribute) -> u8 {
        self.special
            .get(&attribute)
            .copied()
            .unwrap_or(5)
            .clamp(SPECIAL_MIN, SPECIAL_MAX)
    }

    /// Sets a base SPECIAL value, returning the clamped stored value.
    pub fn set_special(&mut self, attribute: SpecialAttribute, value: u8) -> u8 {
        let clamped = value.clamp(SPECIAL_MIN, SPECIAL_MAX);
        self.special.insert(attribute, clamped);
        clamped
    }

    /// Shifts a base SPECIAL value by `delta`, returning the clamped stored
    /// value.
    pub fn mod_special(&mut self, attribute: SpecialAttribute, delta: i16) -> u8 {
        let current = i16::from(self.effective_special(attribute));
        let next = (current + delta).clamp(i16::from(SPECIAL_MIN), i16::from(SPECIAL_MAX));
        self.set_special(attribute, next as u8)
    }

    /// Skill base `2 + 2 * primary + ceil(luck / 2)` plus the tag bonus.
    #[must_use]
    pub fn skill_base(&self, skill: ActorSkill) -> u8 {
        let luck = luck_skill_bonus(self.effective_special(SpecialAttribute::Luck));
        let primary = skill_governing_attribute(skill)
            .map(|attribute| self.effective_special(attribute))
            .unwrap_or(0);
        let tagged = u8::from(self.tagged_skills.contains(&skill)) * TAG_SKILL_BONUS;
        2 + (2 * primary) + luck + tagged
    }

    /// Effective skill value, clamped to `0..=100`.
    #[must_use]
    pub fn skill_value(&self, skill: ActorSkill) -> u8 {
        let base = i16::from(self.skill_base(skill));
        let increases = self.skill_increases.get(&skill).copied().unwrap_or(0);
        (base + increases).clamp(i16::from(SKILL_MIN), i16::from(SKILL_MAX)) as u8
    }

    /// Spends `points` on a skill (negative values refund), keeping the
    /// stored increase within `0..=100`. The effective value is clamped on
    /// read by `skill_value`.
    pub fn add_skill_points(&mut self, skill: ActorSkill, points: i16) -> i16 {
        let current = self.skill_increases.entry(skill).or_insert(0);
        *current = (*current + points).clamp(0, i16::from(SKILL_MAX));
        *current
    }

    /// Derived attributes recomputed from the sheet.
    #[must_use]
    pub fn derived(&self, settings: &GmstSettings) -> DerivedAttributes {
        let endurance = f32::from(self.effective_special(SpecialAttribute::Endurance));
        let agility = f32::from(self.effective_special(SpecialAttribute::Agility));
        let strength = f32::from(self.effective_special(SpecialAttribute::Strength));
        let luck = self.effective_special(SpecialAttribute::Luck);
        DerivedAttributes {
            max_health: settings.health_base
                + endurance * settings.health_endurance_mult
                + f32::from(self.level) * settings.health_level_mult,
            max_action_points: settings.action_points_base
                + agility * settings.action_points_agility_mult,
            carry_weight: settings.carry_weight_base
                + strength * settings.carry_weight_strength_mult,
            critical_chance_bps: critical_chance_bps(luck, 0),
        }
    }

    /// Total accumulated XP spent inside the current level.
    #[must_use]
    pub fn xp_into_level(&self, settings: &GmstSettings) -> u32 {
        self.xp.saturating_sub(xp_threshold(self.level, settings))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct DerivedAttributes {
    pub max_health: f32,
    pub max_action_points: f32,
    pub carry_weight: f32,
    pub critical_chance_bps: u32,
}

/// Critical chance in basis points: `luck * 100 bps` plus perk bonuses.
#[must_use]
pub fn critical_chance_bps(luck: u8, perk_bonus_bps: u32) -> u32 {
    (u32::from(luck.clamp(SPECIAL_MIN, SPECIAL_MAX)) * 100).saturating_add(perk_bonus_bps)
}

pub const RESISTANCE_CAP_BPS: u32 = 8500;

/// Clamps a total resistance (armor + perks + chems) to the 85% hard cap.
#[must_use]
pub fn clamp_resistance_bps(total_bps: u32) -> u32 {
    total_bps.min(RESISTANCE_CAP_BPS)
}

/// Base poison/rad resistance from Endurance: `(END - 1) * 5%`.
#[must_use]
pub fn base_poison_rad_resistance_bps(endurance: u8) -> u32 {
    (u32::from(endurance.clamp(SPECIAL_MIN, SPECIAL_MAX)).saturating_sub(1)) * 500
}

/// Cumulative XP required to reach `level`: `(level - 1) * level / 2 * xp_base`.
#[must_use]
pub fn xp_threshold(level: u8, settings: &GmstSettings) -> u32 {
    if level <= 1 {
        return 0;
    }
    let level = u32::from(level);
    ((level - 1) * level / 2).saturating_mul(settings.xp_base)
}

/// Skill points granted by one level-up: base + effective Intelligence.
#[must_use]
pub fn skill_points_per_level(sheet: &CharacterSheet, settings: &GmstSettings) -> u16 {
    u16::from(settings.level_up_skill_points)
        + u16::from(sheet.effective_special(SpecialAttribute::Intelligence))
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AwardXpOutcome {
    pub xp: u32,
    pub level: u8,
    pub levels_gained: u8,
    pub skill_points_gained: u16,
}

/// Awards XP, advancing levels across every crossed threshold and clamping
/// at the configured level cap.
pub fn award_xp(
    sheet: &mut CharacterSheet,
    amount: u32,
    settings: &GmstSettings,
) -> AwardXpOutcome {
    let cap_level = settings.max_player_level.max(1);
    let cap_xp = xp_threshold(cap_level, settings);
    let total = sheet.xp.saturating_add(amount).min(cap_xp);
    let mut level = 1.max(sheet.level);
    while level < cap_level && xp_threshold(level.saturating_add(1), settings) <= total {
        level += 1;
    }
    let levels_gained = level.saturating_sub(sheet.level.max(1));
    let skill_points_gained = u16::from(levels_gained) * skill_points_per_level(sheet, settings);
    sheet.xp = total;
    sheet.level = level;
    AwardXpOutcome {
        xp: total,
        level,
        levels_gained,
        skill_points_gained,
    }
}

/// Hard skill gate: the value must meet or exceed the requirement.
#[must_use]
pub fn skill_gate_passes(value: u8, required: u8) -> bool {
    value >= required
}

#[cfg(test)]
#[path = "tests/stats.rs"]
mod tests;
