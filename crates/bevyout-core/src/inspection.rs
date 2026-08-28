//! Shared RPG inspection snapshots for Pip-Boy, console, and BRP.
//!
//! One serializable family is the only place derived HP, radiation stages,
//! cripple flags, and calendar fields are computed. Presentation layers
//! format these values; they must not recalculate them.

use serde::{Deserialize, Serialize};

use crate::actor_state::{ActorSkill, ActorValue, SpecialAttribute};
use crate::chems::{AddictionPhase, Addictions};
use crate::combat::body::{ALL_BODY_PARTS, BodyPartId};
use crate::combat::limbs::LimbState;
use crate::crime::CrimeLedger;
use crate::effects::{
    ActiveEffectsLedger, projected_derived_with_limbs, projected_special_with_limbs,
};
use crate::lifecycle::{LIFECYCLE_SNAPSHOT_REVISION, LifecycleSnapshot};
use crate::perks::{PerkDefinition, PerkProgression};
use crate::radiation::RadiationPool;
use crate::stats::{
    CharacterSheet, FO3_SKILLS, GmstSettings, SPECIAL_MAX, SPECIAL_MIN, xp_threshold,
};
use crate::time::{GAME_CALENDAR_REVISION, GameClockState};

/// Inspection schema consumed by Pip-Boy, `showstats`, and BRP probes.
pub const RPG_INSPECTION_SCHEMA_REVISION: u32 = 1;
/// Planned V.A.T.S. wave while M5 ballistics/armor remain unfinished.
pub const VATS_PLANNED_WAVE: u32 = 8;

/// Input for [`inspect_rpg`]. Callers pass live or stored canonical state.
#[derive(Debug, Clone)]
pub struct RpgInspectionInput<'a> {
    pub name: &'a str,
    pub sheet: &'a CharacterSheet,
    pub perks: &'a PerkProgression,
    pub perk_names: &'a [(u32, String)],
    pub unspent_skill_points: u16,
    pub total_skill_points: u16,
    pub radiation: RadiationPool,
    pub effects: &'a ActiveEffectsLedger,
    pub addictions: &'a Addictions,
    pub current_health: Option<f32>,
    pub current_action_points: Option<f32>,
    pub limbs: &'a LimbState,
    pub crime: &'a CrimeLedger,
    pub clock: GameClockState,
    pub lifecycle: Option<&'a LifecycleSnapshot>,
    pub player_cell: Option<u32>,
    pub settings: &'a GmstSettings,
    pub perk_catalog_revision: &'a str,
    pub gmst_catalog_revision: &'a str,
}

/// One serializable RPG view for UI, console, and BRP.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RpgInspectionSnapshot {
    pub schema_revision: u32,
    pub perk_catalog_revision: String,
    pub gmst_catalog_revision: String,
    pub calendar_revision: String,
    pub lifecycle_revision: u32,
    pub player: PlayerRpgSnapshot,
    pub effects: ActiveEffectsSnapshot,
    pub limbs: LimbSnapshot,
    pub crime: CrimeSnapshot,
    pub vats: VatsInspectionSnapshot,
    pub world: WorldLifecycleSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerRpgSnapshot {
    pub name: String,
    pub level: u8,
    pub hp_current: u32,
    pub hp_max: u32,
    pub ap_current: Option<u32>,
    pub ap_max: u32,
    pub ap_available: bool,
    pub xp_current: u32,
    pub xp_into_level: u32,
    pub xp_next: u32,
    pub carry_weight: u32,
    pub critical_chance_bps: u32,
    pub unspent_skill_points: u16,
    pub total_skill_points: u16,
    pub rads: u16,
    pub radiation_stage: u16,
    pub radiation_fatal: bool,
    pub special: Vec<SpecialSnapshot>,
    pub skills: Vec<SkillSnapshot>,
    pub perks: Vec<PerkRankSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpecialSnapshot {
    pub attribute: SpecialAttribute,
    pub label: String,
    pub value: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillSnapshot {
    pub skill: ActorSkill,
    pub label: String,
    pub value: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PerkRankSnapshot {
    pub form_id: u32,
    pub rank: u8,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveEffectsSnapshot {
    pub entries: Vec<EffectEntrySnapshot>,
    pub addictions: Vec<AddictionSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EffectEntrySnapshot {
    pub source: String,
    pub actor_value: String,
    pub magnitude: f32,
    pub remaining_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddictionSnapshot {
    pub withdrawal_form_id: u32,
    pub phase: AddictionPhase,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LimbSnapshot {
    pub parts: Vec<LimbPartSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LimbPartSnapshot {
    pub part: BodyPartId,
    pub label: String,
    pub current_milli: u32,
    pub max_milli: u32,
    pub crippled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrimeSnapshot {
    pub bounty: u32,
    pub karma: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VatsInspectionSnapshot {
    pub available: bool,
    pub reason: String,
    pub planned_wave: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldLifecycleSnapshot {
    pub game_ms: u64,
    pub remainder: u32,
    pub timescale: u32,
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u16,
    pub calendar_revision: String,
    pub lifecycle_revision: u32,
    pub player_cell: Option<u32>,
}

/// Builds the shared RPG inspection snapshot from canonical core state.
#[must_use]
pub fn inspect_rpg(input: RpgInspectionInput<'_>) -> RpgInspectionSnapshot {
    let derived = projected_derived_with_limbs(
        input.sheet,
        input.effects,
        input.radiation.rads,
        input.settings,
        Some(input.limbs),
    );
    let special = projected_special_with_limbs(
        input.sheet,
        input.effects,
        input.radiation.rads,
        Some(input.limbs),
    );
    let hp_max = derived.max_health.max(0.0).round() as u32;
    let hp_current = input
        .current_health
        .unwrap_or(derived.max_health)
        .clamp(0.0, derived.max_health.max(0.0))
        .round() as u32;
    let ap_max = derived.max_action_points.max(0.0).round() as u32;
    let ap_available = input.current_action_points.is_some();
    let ap_current = input
        .current_action_points
        .map(|value| value.clamp(0.0, derived.max_action_points.max(0.0)).round() as u32);
    let xp_next = if input.sheet.level >= input.settings.max_player_level {
        input.sheet.xp
    } else {
        xp_threshold(input.sheet.level.saturating_add(1), input.settings)
    };
    let special_order = [
        SpecialAttribute::Strength,
        SpecialAttribute::Perception,
        SpecialAttribute::Endurance,
        SpecialAttribute::Charisma,
        SpecialAttribute::Intelligence,
        SpecialAttribute::Agility,
        SpecialAttribute::Luck,
    ];
    let special_rows = special_order
        .into_iter()
        .map(|attribute| SpecialSnapshot {
            attribute,
            label: ActorValue::Special(attribute).label().to_string(),
            value: special
                .get(&attribute)
                .copied()
                .unwrap_or(SPECIAL_MIN)
                .clamp(SPECIAL_MIN, SPECIAL_MAX),
        })
        .collect();
    let skills = FO3_SKILLS
        .iter()
        .copied()
        .map(|skill| SkillSnapshot {
            skill,
            label: ActorValue::Skill(skill).label().to_string(),
            value: input.sheet.skill_value(skill),
        })
        .collect();
    let mut perk_names: Vec<(u32, String)> = input.perk_names.to_vec();
    perk_names.sort_by_key(|(form_id, _)| *form_id);
    perk_names.dedup_by_key(|(form_id, _)| *form_id);
    let mut perks: Vec<PerkRankSnapshot> = input
        .perks
        .0
        .iter()
        .map(|(&form_id, &rank)| PerkRankSnapshot {
            form_id,
            rank,
            name: perk_names
                .iter()
                .find(|(id, _)| *id == form_id)
                .map(|(_, name)| name.clone())
                .unwrap_or_else(|| format!("{form_id:08x}")),
        })
        .collect();
    perks.sort_by_key(|perk| perk.form_id);

    let mut effects: Vec<EffectEntrySnapshot> = input
        .effects
        .entries
        .iter()
        .map(|entry| EffectEntrySnapshot {
            source: entry.source.label().to_string(),
            actor_value: entry.actor_value.label().to_string(),
            magnitude: entry.magnitude,
            remaining_ms: entry.remaining_ms,
        })
        .collect();
    effects.sort_by(|left, right| {
        left.source
            .cmp(&right.source)
            .then(left.actor_value.cmp(&right.actor_value))
            .then(left.remaining_ms.cmp(&right.remaining_ms))
    });
    let addictions: Vec<AddictionSnapshot> = input
        .addictions
        .0
        .iter()
        .filter(|(_, phase)| !matches!(phase, AddictionPhase::Clean))
        .map(|(&withdrawal_form_id, &phase)| AddictionSnapshot {
            withdrawal_form_id,
            phase,
            label: phase.label().to_string(),
        })
        .collect();

    let parts = ALL_BODY_PARTS
        .into_iter()
        .map(|part| {
            let condition = input.limbs.part(part);
            LimbPartSnapshot {
                part,
                label: part.label().to_string(),
                current_milli: condition.current_milli,
                max_milli: condition.max_milli,
                crippled: condition.crippled,
            }
        })
        .collect();

    let date = input.clock.calendar();
    let lifecycle_revision = input
        .lifecycle
        .map(|snapshot| snapshot.revision)
        .unwrap_or(LIFECYCLE_SNAPSHOT_REVISION);

    RpgInspectionSnapshot {
        schema_revision: RPG_INSPECTION_SCHEMA_REVISION,
        perk_catalog_revision: input.perk_catalog_revision.to_string(),
        gmst_catalog_revision: input.gmst_catalog_revision.to_string(),
        calendar_revision: GAME_CALENDAR_REVISION.to_string(),
        lifecycle_revision,
        player: PlayerRpgSnapshot {
            name: if input.name.is_empty() {
                "Player".into()
            } else {
                input.name.to_string()
            },
            level: input.sheet.level.max(1),
            hp_current,
            hp_max,
            ap_current,
            ap_max,
            ap_available,
            xp_current: input.sheet.xp,
            xp_into_level: input.sheet.xp_into_level(input.settings),
            xp_next,
            carry_weight: derived.carry_weight.max(0.0).round() as u32,
            critical_chance_bps: derived.critical_chance_bps,
            unspent_skill_points: input.unspent_skill_points,
            total_skill_points: input.total_skill_points,
            rads: input.radiation.rads,
            radiation_stage: input.radiation.threshold_reached(),
            radiation_fatal: input.radiation.is_fatal(),
            special: special_rows,
            skills,
            perks,
        },
        effects: ActiveEffectsSnapshot {
            entries: effects,
            addictions,
        },
        limbs: LimbSnapshot { parts },
        crime: CrimeSnapshot {
            bounty: input.crime.bounty,
            karma: input.crime.karma,
        },
        vats: VatsInspectionSnapshot {
            available: false,
            reason: "unavailable".into(),
            planned_wave: VATS_PLANNED_WAVE,
        },
        world: WorldLifecycleSnapshot {
            game_ms: input.clock.absolute_game_ms,
            remainder: input.clock.fractional_timescale_remainder,
            timescale: input.clock.timescale,
            year: date.year,
            month: date.month,
            day: date.day,
            hour: date.hour,
            minute: date.minute,
            second: date.second,
            millisecond: date.millisecond,
            calendar_revision: GAME_CALENDAR_REVISION.to_string(),
            lifecycle_revision,
            player_cell: input.player_cell,
        },
    }
}

/// Named default constructor for missing M9 sections on v1–v8 saves.
#[must_use]
pub fn default_inspection_from_sheet(
    sheet: &CharacterSheet,
    settings: &GmstSettings,
) -> RpgInspectionSnapshot {
    inspect_rpg(RpgInspectionInput {
        name: "Player",
        sheet,
        perks: &PerkProgression::default(),
        perk_names: &[],
        unspent_skill_points: 0,
        total_skill_points: 0,
        radiation: RadiationPool::default(),
        effects: &ActiveEffectsLedger::default(),
        addictions: &Addictions::default(),
        current_health: None,
        current_action_points: None,
        limbs: &LimbState::healthy(),
        crime: &CrimeLedger::default(),
        clock: GameClockState::default(),
        lifecycle: None,
        player_cell: None,
        settings,
        perk_catalog_revision: "",
        gmst_catalog_revision: "",
    })
}

/// Display helper for Pip-Boy SPECIAL rows from a frozen snapshot.
#[must_use]
pub fn special_lines(snapshot: &RpgInspectionSnapshot) -> Vec<String> {
    snapshot
        .player
        .special
        .iter()
        .map(|row| format!("{}  {}", row.label.to_ascii_uppercase(), row.value))
        .collect()
}

/// Display helper for Pip-Boy radiation stage from a frozen snapshot.
#[must_use]
pub fn radiation_stage_line(snapshot: &RpgInspectionSnapshot) -> String {
    if snapshot.player.radiation_fatal {
        format!("RADS  {}  FATAL", snapshot.player.rads)
    } else {
        format!(
            "RADS  {}  STAGE {}",
            snapshot.player.rads, snapshot.player.radiation_stage
        )
    }
}

/// Display helper for Pip-Boy Data World integer clock.
#[must_use]
pub fn world_clock_line(snapshot: &RpgInspectionSnapshot) -> String {
    let world = &snapshot.world;
    format!(
        "GAME TIME  {:04}-{:02}-{:02} {:02}:{:02}  {} ms",
        world.year, world.month, world.day, world.hour, world.minute, world.game_ms
    )
}

/// Perk name lookup from a prepared catalog without exposing Bevy types.
#[must_use]
pub fn perk_display_name(def: &PerkDefinition) -> String {
    def.name
        .clone()
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| {
            if def.editor_id.is_empty() {
                format!("{:08x}", def.form_id)
            } else {
                def.editor_id.clone()
            }
        })
}

#[cfg(test)]
#[path = "tests/inspection.rs"]
mod tests;
