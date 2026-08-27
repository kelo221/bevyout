//! Engine-independent actor definition, value resolution, and mutable state.
//!
//! Prepared records provide immutable definitions. Runtime systems persist only
//! mutations keyed by a stable ACHR/ACRE reference; presentation, inventory,
//! equipment, transforms, and hostility remain separate projections.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::actor::ActorKind;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpecialAttribute {
    Strength,
    Perception,
    Endurance,
    Charisma,
    Intelligence,
    Agility,
    Luck,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActorSkill {
    Barter,
    BigGuns,
    EnergyWeapons,
    Explosives,
    Lockpick,
    Medicine,
    MeleeWeapons,
    Repair,
    Science,
    SmallGuns,
    Sneak,
    Speech,
    Throwing,
    Unarmed,
}

/// Authored Fallout 3 values currently exposed by the prepared actor catalog.
/// Unknown engine actor-value IDs are not guessed into this enum; the M9
/// wave 3 additions (ActionPoints through Rads) are verified against the
/// real ESM's `MGEF` primary actor-value indices and the GECK Actor Value
/// Codes table.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActorValue {
    Health,
    Fatigue,
    SpeedMultiplier,
    Karma,
    Disposition,
    ActionPoints,
    CarryWeight,
    DamageResist,
    PoisonResist,
    RadResist,
    Rads,
    Special(SpecialAttribute),
    Skill(ActorSkill),
    CreatureCombatSkill,
    CreatureMagicSkill,
    CreatureStealthSkill,
    CreatureDamage,
}

impl ActorValue {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Health => "health",
            Self::Fatigue => "fatigue",
            Self::SpeedMultiplier => "speed_multiplier",
            Self::Karma => "karma",
            Self::Disposition => "disposition",
            Self::ActionPoints => "action_points",
            Self::CarryWeight => "carry_weight",
            Self::DamageResist => "damage_resist",
            Self::PoisonResist => "poison_resist",
            Self::RadResist => "rad_resist",
            Self::Rads => "rads",
            Self::Special(SpecialAttribute::Strength) => "strength",
            Self::Special(SpecialAttribute::Perception) => "perception",
            Self::Special(SpecialAttribute::Endurance) => "endurance",
            Self::Special(SpecialAttribute::Charisma) => "charisma",
            Self::Special(SpecialAttribute::Intelligence) => "intelligence",
            Self::Special(SpecialAttribute::Agility) => "agility",
            Self::Special(SpecialAttribute::Luck) => "luck",
            Self::Skill(ActorSkill::Barter) => "barter",
            Self::Skill(ActorSkill::BigGuns) => "big_guns",
            Self::Skill(ActorSkill::EnergyWeapons) => "energy_weapons",
            Self::Skill(ActorSkill::Explosives) => "explosives",
            Self::Skill(ActorSkill::Lockpick) => "lockpick",
            Self::Skill(ActorSkill::Medicine) => "medicine",
            Self::Skill(ActorSkill::MeleeWeapons) => "melee_weapons",
            Self::Skill(ActorSkill::Repair) => "repair",
            Self::Skill(ActorSkill::Science) => "science",
            Self::Skill(ActorSkill::SmallGuns) => "small_guns",
            Self::Skill(ActorSkill::Sneak) => "sneak",
            Self::Skill(ActorSkill::Speech) => "speech",
            Self::Skill(ActorSkill::Throwing) => "throwing",
            Self::Skill(ActorSkill::Unarmed) => "unarmed",
            Self::CreatureCombatSkill => "creature_combat_skill",
            Self::CreatureMagicSkill => "creature_magic_skill",
            Self::CreatureStealthSkill => "creature_stealth_skill",
            Self::CreatureDamage => "creature_damage",
        }
    }

    #[must_use]
    pub fn parse(label: &str) -> Option<Self> {
        ALL_ACTOR_VALUES
            .iter()
            .copied()
            .find(|value| value.label().eq_ignore_ascii_case(label))
    }
}

pub const ALL_ACTOR_VALUES: &[ActorValue] = &[
    ActorValue::Health,
    ActorValue::Fatigue,
    ActorValue::SpeedMultiplier,
    ActorValue::Karma,
    ActorValue::Disposition,
    ActorValue::ActionPoints,
    ActorValue::CarryWeight,
    ActorValue::DamageResist,
    ActorValue::PoisonResist,
    ActorValue::RadResist,
    ActorValue::Rads,
    ActorValue::Special(SpecialAttribute::Strength),
    ActorValue::Special(SpecialAttribute::Perception),
    ActorValue::Special(SpecialAttribute::Endurance),
    ActorValue::Special(SpecialAttribute::Charisma),
    ActorValue::Special(SpecialAttribute::Intelligence),
    ActorValue::Special(SpecialAttribute::Agility),
    ActorValue::Special(SpecialAttribute::Luck),
    ActorValue::Skill(ActorSkill::Barter),
    ActorValue::Skill(ActorSkill::BigGuns),
    ActorValue::Skill(ActorSkill::EnergyWeapons),
    ActorValue::Skill(ActorSkill::Explosives),
    ActorValue::Skill(ActorSkill::Lockpick),
    ActorValue::Skill(ActorSkill::Medicine),
    ActorValue::Skill(ActorSkill::MeleeWeapons),
    ActorValue::Skill(ActorSkill::Repair),
    ActorValue::Skill(ActorSkill::Science),
    ActorValue::Skill(ActorSkill::SmallGuns),
    ActorValue::Skill(ActorSkill::Sneak),
    ActorValue::Skill(ActorSkill::Speech),
    ActorValue::Skill(ActorSkill::Throwing),
    ActorValue::Skill(ActorSkill::Unarmed),
    ActorValue::CreatureCombatSkill,
    ActorValue::CreatureMagicSkill,
    ActorValue::CreatureStealthSkill,
    ActorValue::CreatureDamage,
];

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ActorFactionMembership {
    pub faction_form_id: u32,
    pub rank: i8,
    pub title: Option<String>,
}

/// Immutable prepared actor data. Modifier maps are additive; base values
/// override template values rather than being added to them.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ActorDefinition {
    pub base_form_id: u32,
    pub reference_form_id: u32,
    pub kind: ActorKind,
    pub race_form_id: Option<u32>,
    pub class_form_id: Option<u32>,
    pub factions: Vec<ActorFactionMembership>,
    pub package_form_ids: Vec<u32>,
    pub template_values: BTreeMap<ActorValue, f32>,
    pub base_values: BTreeMap<ActorValue, f32>,
    pub race_modifiers: BTreeMap<ActorValue, f32>,
    pub class_modifiers: BTreeMap<ActorValue, f32>,
    pub faction_modifiers: BTreeMap<ActorValue, f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolvedActorValue {
    pub inherited_or_base: f32,
    pub authored_modifier: f32,
    pub runtime_mutation: f32,
    pub effective: f32,
}

impl ActorDefinition {
    #[must_use]
    pub fn resolve_value(
        &self,
        state: &ActorInstanceState,
        value: ActorValue,
    ) -> ResolvedActorValue {
        let inherited_or_base = self
            .base_values
            .get(&value)
            .or_else(|| self.template_values.get(&value))
            .copied()
            .unwrap_or_default();
        let authored_modifier = [
            &self.race_modifiers,
            &self.class_modifiers,
            &self.faction_modifiers,
        ]
        .into_iter()
        .filter_map(|layer| layer.get(&value))
        .fold(0.0, |total, modifier| total + modifier);
        let runtime_mutation = state
            .value_mutations
            .get(&value)
            .copied()
            .unwrap_or_default();
        ResolvedActorValue {
            inherited_or_base,
            authored_modifier,
            runtime_mutation,
            effective: inherited_or_base + authored_modifier + runtime_mutation,
        }
    }

    pub fn validate(&self) -> Result<(), ActorStateError> {
        validate_form_id(self.base_form_id, "actor base")?;
        validate_form_id(self.reference_form_id, "actor reference")?;
        validate_value_layers([
            &self.template_values,
            &self.base_values,
            &self.race_modifiers,
            &self.class_modifiers,
            &self.faction_modifiers,
        ])?;
        let mut factions = BTreeSet::new();
        for membership in &self.factions {
            validate_form_id(membership.faction_form_id, "actor faction")?;
            if !factions.insert(membership.faction_form_id) {
                return Err(ActorStateError::DuplicateFaction(
                    membership.faction_form_id,
                ));
            }
        }
        if self.package_form_ids.contains(&0) {
            return Err(ActorStateError::ZeroFormId("actor package"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActorLifeState {
    #[default]
    Alive,
    Dead,
}

impl ActorLifeState {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Alive => "alive",
            Self::Dead => "dead",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ActorPackageCheckpoint {
    pub package_form_id: u32,
    pub procedure_index: u32,
    pub elapsed_seconds: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ActorInstanceState {
    pub reference_form_id: u32,
    pub life_state: ActorLifeState,
    pub value_mutations: BTreeMap<ActorValue, f32>,
    pub package: Option<ActorPackageCheckpoint>,
}

impl Default for ActorInstanceState {
    fn default() -> Self {
        Self {
            reference_form_id: 0,
            life_state: ActorLifeState::Alive,
            value_mutations: BTreeMap::new(),
            package: None,
        }
    }
}

impl ActorInstanceState {
    #[must_use]
    pub fn new(reference_form_id: u32, life_state: ActorLifeState) -> Self {
        Self {
            reference_form_id,
            life_state,
            ..Self::default()
        }
    }

    pub fn set_value_mutation(
        &mut self,
        value: ActorValue,
        mutation: f32,
    ) -> Result<(), ActorStateError> {
        validate_value(value, mutation)?;
        if mutation == 0.0 {
            self.value_mutations.remove(&value);
        } else {
            self.value_mutations.insert(value, mutation);
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<(), ActorStateError> {
        validate_form_id(self.reference_form_id, "actor reference")?;
        validate_value_layers([&self.value_mutations])?;
        if let Some(package) = self.package {
            validate_form_id(package.package_form_id, "actor package")?;
            if !package.elapsed_seconds.is_finite() || package.elapsed_seconds < 0.0 {
                return Err(ActorStateError::InvalidPackageElapsed(
                    package.elapsed_seconds,
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorSeedOutcome {
    Inserted,
    Existing,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ActorStateStore {
    pub cells: BTreeMap<u32, BTreeMap<u32, ActorInstanceState>>,
}

impl ActorStateStore {
    pub fn seed(
        &mut self,
        cell_form_id: u32,
        reference_form_id: u32,
        initial_life_state: ActorLifeState,
    ) -> Result<ActorSeedOutcome, ActorStateError> {
        validate_form_id(cell_form_id, "actor cell")?;
        validate_form_id(reference_form_id, "actor reference")?;
        let states = self.cells.entry(cell_form_id).or_default();
        if states.contains_key(&reference_form_id) {
            return Ok(ActorSeedOutcome::Existing);
        }
        states.insert(
            reference_form_id,
            ActorInstanceState::new(reference_form_id, initial_life_state),
        );
        Ok(ActorSeedOutcome::Inserted)
    }

    #[must_use]
    pub fn get(&self, cell_form_id: u32, reference_form_id: u32) -> Option<&ActorInstanceState> {
        self.cells.get(&cell_form_id)?.get(&reference_form_id)
    }

    pub fn get_mut(
        &mut self,
        cell_form_id: u32,
        reference_form_id: u32,
    ) -> Option<&mut ActorInstanceState> {
        self.cells
            .get_mut(&cell_form_id)?
            .get_mut(&reference_form_id)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.cells.values().map(BTreeMap::len).sum()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cells.values().all(BTreeMap::is_empty)
    }

    pub fn validate(&self) -> Result<(), ActorStateError> {
        for (cell_form_id, actors) in &self.cells {
            validate_form_id(*cell_form_id, "actor cell")?;
            for (reference_form_id, actor) in actors {
                if *reference_form_id != actor.reference_form_id {
                    return Err(ActorStateError::ReferenceKeyMismatch {
                        key: *reference_form_id,
                        state: actor.reference_form_id,
                    });
                }
                actor.validate()?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActorStateError {
    ZeroFormId(&'static str),
    DuplicateFaction(u32),
    NonFiniteValue { value: ActorValue, amount: f32 },
    InvalidPackageElapsed(f32),
    ReferenceKeyMismatch { key: u32, state: u32 },
}

impl fmt::Display for ActorStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroFormId(kind) => write!(formatter, "{kind} FormID must be non-zero"),
            Self::DuplicateFaction(form_id) => {
                write!(formatter, "duplicate actor faction {form_id:08x}")
            }
            Self::NonFiniteValue { value, amount } => write!(
                formatter,
                "actor value {} must be finite, got {amount}",
                value.label()
            ),
            Self::InvalidPackageElapsed(elapsed) => {
                write!(
                    formatter,
                    "actor package elapsed seconds must be finite and non-negative, got {elapsed}"
                )
            }
            Self::ReferenceKeyMismatch { key, state } => write!(
                formatter,
                "actor state key {key:08x} does not match reference {state:08x}"
            ),
        }
    }
}

impl std::error::Error for ActorStateError {}

fn validate_form_id(value: u32, kind: &'static str) -> Result<(), ActorStateError> {
    if value == 0 {
        return Err(ActorStateError::ZeroFormId(kind));
    }
    Ok(())
}

fn validate_value(value: ActorValue, amount: f32) -> Result<(), ActorStateError> {
    if !amount.is_finite() {
        return Err(ActorStateError::NonFiniteValue { value, amount });
    }
    Ok(())
}

fn validate_value_layers<'a>(
    layers: impl IntoIterator<Item = &'a BTreeMap<ActorValue, f32>>,
) -> Result<(), ActorStateError> {
    for layer in layers {
        for (&value, &amount) in layer {
            validate_value(value, amount)?;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests/actor_state.rs"]
mod tests;
