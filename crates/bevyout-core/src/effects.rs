//! Pure active-effects engine (M9 wave 3, #316/#317).
//!
//! Two concerns live here, both Bevy-free and deterministic:
//!
//! - *Definitions*: `EffectDefinition` (a decoded `MGEF` base effect) and
//!   `IngestibleDefinition` (a decoded `ALCH` ingestible) are what the
//!   prepared effect catalog (#316) persists, exactly like
//!   `perks::PerkDefinition` for wave 2.
//! - *Runtime kernel*: `ActiveEffectsLedger` tracks timed value-modifier
//!   effects (chem buffs), ticks them in whole milliseconds, and projects
//!   effective SPECIAL through `projected_special` together with the
//!   radiation penalties (`radiation::radiation_penalties`).
//!
//! Timescale note: authored `EFIT` durations are game **seconds**; this
//! wave ticks real frame milliseconds, so the runtime adapter converts
//! once at application time (`duration_s * 1000`). The engine's scaled
//! game clock is wave 9's `GameTime` and will replace that conversion.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::actor_state::{ActorSkill, ActorValue, SpecialAttribute};
use crate::radiation::radiation_penalties;
use crate::stats::CharacterSheet;

/// Maps the engine's internal actor-value index found in `MGEF.DATA`'s
/// primary actor-value field onto the domain `ActorValue` (M9 wave 3).
///
/// The index family is the GECK "Actor Value Codes" table (0 Aggression,
/// 5..=11 SPECIAL, 12 ActionPoints, 16 Health, 32..=45 skills, 54 Rads),
/// verified against the real GOTY `Fallout3.esm`: every `ChemInc*`/`AddictRed*`
/// effect carries the index its EditorID names (`ChemIncSTBuffout` = 5,
/// `ChemIncAPJet` = 12) and the `IncreaseSkill*`/`ReduceSkill*` families
/// cover 32..=45 exactly like the wave-2 perk condition mapping. Indices
/// with no domain variant yet (limbs are wave 4) stay unresolved rather
/// than guessed.
#[must_use]
pub fn actor_value_from_effect_index(index: i32) -> Option<ActorValue> {
    match index {
        12 => Some(ActorValue::ActionPoints),
        16 => Some(ActorValue::Health),
        21 => Some(ActorValue::SpeedMultiplier),
        22 => Some(ActorValue::Fatigue),
        23 => Some(ActorValue::Karma),
        5 => Some(ActorValue::Special(SpecialAttribute::Strength)),
        6 => Some(ActorValue::Special(SpecialAttribute::Perception)),
        7 => Some(ActorValue::Special(SpecialAttribute::Endurance)),
        8 => Some(ActorValue::Special(SpecialAttribute::Charisma)),
        9 => Some(ActorValue::Special(SpecialAttribute::Intelligence)),
        10 => Some(ActorValue::Special(SpecialAttribute::Agility)),
        11 => Some(ActorValue::Special(SpecialAttribute::Luck)),
        13 => Some(ActorValue::CarryWeight),
        18 => Some(ActorValue::DamageResist),
        19 => Some(ActorValue::PoisonResist),
        20 => Some(ActorValue::RadResist),
        54 => Some(ActorValue::Rads),
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
        45 => Some(ActorValue::Skill(ActorSkill::Unarmed)),
        _ => None,
    }
}

/// `MGEF.DATA` archetype 0 (Value Modifier): the magnitude shifts the
/// associated actor value for the effect's duration.
pub const ARCHETYPE_VALUE_MODIFIER: u32 = 0;
/// `MGEF.DATA` archetype 34 (Value And Parts): value modifier plus limb
/// parts (Stimpak's `RestoreHealthStimpak`); the value part applies, limb
/// restoration itself is wave 4.
pub const ARCHETYPE_VALUE_AND_PARTS: u32 = 34;

/// True when an MGEF archetype's magnitude shifts its associated actor
/// value (the only archetypes the wave-3 ledger interprets).
#[must_use]
pub fn archetype_modifies_value(archetype: u32) -> bool {
    matches!(
        archetype,
        ARCHETYPE_VALUE_MODIFIER | ARCHETYPE_VALUE_AND_PARTS
    )
}

/// A decoded `MGEF` base effect, exactly what the prepared effect catalog
/// persists (M9 wave 3, #316).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct EffectDefinition {
    pub form_id: u32,
    pub editor_id: String,
    pub name: Option<String>,
    /// Raw `MGEF.DATA` flags dword (0x02 Recover, 0x04 Detrimental, delivery
    /// bits 0x10..=0x40, ...).
    pub flags: u32,
    pub base_cost: f32,
    pub archetype: u32,
    /// The raw engine AV index as authored (-1 = none); kept next to the
    /// resolved value so unmapped indices stay diagnosable.
    pub actor_value_index: i32,
    /// Domain actor value when the index maps; `None` leaves the effect
    /// cataloged but unapplied by the runtime.
    pub actor_value: Option<ActorValue>,
}

/// One effect item of an ingestible: the decoded `EFID`/`EFIT` pair.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct IngestibleEffect {
    pub mgef_form_id: u32,
    pub editor_id: String,
    pub magnitude: f32,
    /// Authored duration in game seconds (0 = instant application).
    pub duration_s: u32,
    /// Resolved target when both the MGEF archetype modifies a value and
    /// its AV index maps onto the domain enum.
    pub actor_value: Option<ActorValue>,
    /// True when a `CTDA` condition gates this effect item; conditioned
    /// items stay cataloged but are not run (same conservative contract as
    /// the wave-2 perk `unknown_conditions`).
    pub conditioned: bool,
}

impl IngestibleEffect {
    /// Duration in real-time milliseconds for the wave-3 ledger (game
    /// seconds times 1000; see the module timescale note).
    #[must_use]
    pub fn duration_ms(&self) -> u32 {
        self.duration_s.saturating_mul(1000)
    }
}

/// A decoded `ALCH` ingestible, exactly what the prepared effect catalog
/// persists (M9 wave 3, #316).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct IngestibleDefinition {
    pub form_id: u32,
    pub editor_id: String,
    pub name: Option<String>,
    /// `ENIT` bottle value in caps.
    pub value_caps: u32,
    /// `ENIT` flags byte: 0x01 no auto-calc, 0x02 food item, 0x04 medicine.
    pub flags: u8,
    /// `DATA` weight.
    pub weight: f32,
    /// `ENIT` withdrawal SPEL FormID (0 = none). Stored, not run: ENCH/SPEL
    /// execution stays deferred, so withdrawal penalties are a documented
    /// wave-3 parameterization keyed on this id.
    pub withdrawal_form_id: u32,
    /// `ENIT` base addiction chance as authored, percent (0.0..=100.0;
    /// Jet is 20.0 on real data).
    pub addiction_chance_percent: f32,
    pub effects: Vec<IngestibleEffect>,
}

impl IngestibleDefinition {
    /// True when consuming this ingestible can addict (a non-zero chance
    /// and a withdrawal effect, exactly the engine's two requirements).
    #[must_use]
    pub fn addictive(&self) -> bool {
        self.addiction_chance_percent > 0.0 && self.withdrawal_form_id != 0
    }

    /// Addiction chance in basis points for `chems::roll_addiction`.
    #[must_use]
    pub fn addiction_chance_bps(&self) -> u32 {
        (self.addiction_chance_percent.clamp(0.0, 100.0) * 100.0).round() as u32
    }
}

/// What granted an active effect; withdrawal entries are what the ledger
/// emits when a chem buff expires while addicted (#317).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EffectSource {
    Item,
    Chem,
    Environment,
    Withdrawal,
    Perk,
}

impl EffectSource {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Item => "item",
            Self::Chem => "chem",
            Self::Environment => "environment",
            Self::Withdrawal => "withdrawal",
            Self::Perk => "perk",
        }
    }
}

/// One active timed value-modifier effect. Instant applications (health
/// restore, rad removal) never enter the ledger; only timed modifiers do.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ActiveEffect {
    pub source: EffectSource,
    pub actor_value: ActorValue,
    /// Signed magnitude applied to the actor value while active.
    pub magnitude: f32,
    /// Whole milliseconds left; `u32::MAX` marks a permanent entry
    /// (withdrawal until cured).
    pub remaining_ms: u32,
}

/// Permanent entries (withdrawal until cured) use this sentinel duration.
pub const PERMANENT_MS: u32 = u32::MAX;

/// Ordered ledger of active timed effects (#317). Entries append in
/// application order; re-applying the same `(source, actor_value)` pair
/// merges by taking the larger remaining time and the newer magnitude
/// (a second Buffout refreshes the timer rather than stacking).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ActiveEffectsLedger {
    pub entries: Vec<ActiveEffect>,
}

impl ActiveEffectsLedger {
    /// Applies (merges or appends) one timed effect.
    pub fn apply(&mut self, effect: ActiveEffect) {
        if let Some(existing) = self
            .entries
            .iter_mut()
            .find(|entry| entry.source == effect.source && entry.actor_value == effect.actor_value)
        {
            existing.remaining_ms = existing.remaining_ms.max(effect.remaining_ms);
            existing.magnitude = effect.magnitude;
        } else {
            self.entries.push(effect);
        }
    }

    /// Removes every entry targeting `actor_value` from one source
    /// (`cureaddiction` clearing withdrawal, re-dosing a chem).
    pub fn clear_source_value(&mut self, source: EffectSource, actor_value: ActorValue) {
        self.entries
            .retain(|entry| !(entry.source == source && entry.actor_value == actor_value));
    }

    /// Removes every entry from one source (e.g. all withdrawal entries).
    pub fn clear_source(&mut self, source: EffectSource) {
        self.entries.retain(|entry| entry.source != source);
    }

    /// Advances every entry by `delta_ms` whole milliseconds, returning the
    /// entries that expired so the caller can start withdrawal (#317).
    /// Permanent entries never expire.
    pub fn tick(&mut self, delta_ms: u32) -> Vec<ActiveEffect> {
        let mut expired = Vec::new();
        let mut remaining = Vec::with_capacity(self.entries.len());
        for mut entry in self.entries.drain(..) {
            if entry.remaining_ms == PERMANENT_MS {
                remaining.push(entry);
                continue;
            }
            entry.remaining_ms = entry.remaining_ms.saturating_sub(delta_ms);
            if entry.remaining_ms == 0 {
                expired.push(entry);
            } else {
                remaining.push(entry);
            }
        }
        self.entries = remaining;
        expired
    }

    /// Summed signed magnitude currently applying to `actor_value`.
    #[must_use]
    pub fn modifier_for(&self, actor_value: ActorValue) -> f32 {
        self.entries
            .iter()
            .filter(|entry| entry.actor_value == actor_value)
            .map(|entry| entry.magnitude)
            .sum()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

/// Effective SPECIAL projection: base sheet values plus active effect
/// modifiers minus radiation penalties, clamped to `1..=10` (#317). The
/// stored sheet stays authoritative; consumers read through this function.
#[must_use]
pub fn projected_special(
    sheet: &CharacterSheet,
    ledger: &ActiveEffectsLedger,
    rads: u16,
) -> BTreeMap<SpecialAttribute, u8> {
    let penalties = radiation_penalties(rads);
    let all = [
        SpecialAttribute::Strength,
        SpecialAttribute::Perception,
        SpecialAttribute::Endurance,
        SpecialAttribute::Charisma,
        SpecialAttribute::Intelligence,
        SpecialAttribute::Agility,
        SpecialAttribute::Luck,
    ];
    all.into_iter()
        .map(|attribute| {
            let base = i16::from(sheet.effective_special(attribute));
            let modifier = ledger.modifier_for(ActorValue::Special(attribute));
            let penalty = i16::from(penalties.get(&attribute).copied().unwrap_or(0));
            let effective = (base as f32 + modifier + f32::from(penalty))
                .round()
                .clamp(1.0, 10.0) as u8;
            (attribute, effective)
        })
        .collect()
}

#[cfg(test)]
#[path = "tests/effects.rs"]
mod tests;
