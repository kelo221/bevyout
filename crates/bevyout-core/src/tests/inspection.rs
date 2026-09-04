use crate::actor_state::{ActorSkill, ActorValue, SpecialAttribute};
use crate::chems::{AddictionPhase, Addictions};
use crate::combat::body::BodyPartId;
use crate::combat::limbs::{LIMB_MAX_MILLI, LimbState};
use crate::crime::CrimeLedger;
use crate::effects::{ActiveEffect, ActiveEffectsLedger, EffectSource};
use crate::inspection::{
    RPG_INSPECTION_SCHEMA_REVISION, RpgInspectionInput, VATS_PLANNED_WAVE, inspect_rpg,
    radiation_stage_line, world_clock_line,
};
use crate::perks::PerkProgression;
use crate::radiation::RadiationPool;
use crate::stats::{CharacterSheet, GmstSettings, xp_threshold};
use crate::time::{GAME_CALENDAR_REVISION, GameClockState, MS_PER_HOUR};

fn inspect_default() -> crate::inspection::RpgInspectionSnapshot {
    inspect_rpg(RpgInspectionInput {
        name: "Player",
        sheet: &CharacterSheet::default(),
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
        settings: &GmstSettings::default(),
        perk_catalog_revision: "perks-v1",
        gmst_catalog_revision: "gmst-v1",
    })
}

#[test]
fn default_sheet_projects_goty_vitals_without_current_ap() {
    let snapshot = inspect_default();
    assert_eq!(snapshot.schema_revision, RPG_INSPECTION_SCHEMA_REVISION);
    assert_eq!(snapshot.player.level, 1);
    assert_eq!(snapshot.player.hp_current, 200);
    assert_eq!(snapshot.player.hp_max, 200);
    assert_eq!(snapshot.player.ap_max, 75);
    assert!(!snapshot.player.ap_available);
    assert_eq!(snapshot.player.ap_current, None);
    assert_eq!(snapshot.player.xp_current, 0);
    assert_eq!(snapshot.player.xp_into_level, 0);
    assert_eq!(
        snapshot.player.xp_next,
        xp_threshold(2, &GmstSettings::default())
    );
    assert_eq!(snapshot.player.carry_weight, 200);
    assert_eq!(snapshot.player.rads, 0);
    assert_eq!(snapshot.player.radiation_stage, 0);
    assert!(!snapshot.player.radiation_fatal);
    assert_eq!(snapshot.player.special.len(), 7);
    assert!(snapshot.player.special.iter().all(|row| row.value == 5));
    assert_eq!(snapshot.player.skills.len(), 13);
    assert_eq!(snapshot.vats.available, false);
    assert_eq!(snapshot.vats.reason, "unavailable");
    assert_eq!(snapshot.vats.planned_wave, VATS_PLANNED_WAVE);
    assert_eq!(snapshot.world.year, 2277);
    assert_eq!(snapshot.world.month, 10);
    assert_eq!(snapshot.world.day, 23);
    assert_eq!(snapshot.world.calendar_revision, GAME_CALENDAR_REVISION);
}

#[test]
fn effects_and_perks_sort_by_stable_identity() {
    let mut effects = ActiveEffectsLedger::default();
    effects.apply(ActiveEffect {
        source: EffectSource::Chem,
        actor_value: ActorValue::Special(SpecialAttribute::Strength),
        magnitude: 1.0,
        remaining_ms: 2_000,
    });
    effects.apply(ActiveEffect {
        source: EffectSource::Chem,
        actor_value: ActorValue::ActionPoints,
        magnitude: 20.0,
        remaining_ms: 1_000,
    });
    let mut perks = PerkProgression::default();
    perks.set_rank(0x20, 1);
    perks.set_rank(0x10, 2);
    let mut addictions = Addictions::default();
    addictions.addict(0xB);
    addictions.addict(0xA);
    addictions.begin_withdrawal(0xA);
    let snapshot = inspect_rpg(RpgInspectionInput {
        name: "Player",
        sheet: &CharacterSheet::default(),
        perks: &perks,
        perk_names: &[(0x20, "Later".into()), (0x10, "Earlier".into())],
        unspent_skill_points: 0,
        total_skill_points: 0,
        radiation: RadiationPool::new(200),
        effects: &effects,
        addictions: &addictions,
        current_health: Some(150.0),
        current_action_points: None,
        limbs: &LimbState::healthy(),
        crime: &CrimeLedger {
            bounty: 40,
            karma: -5,
            ..Default::default()
        },
        clock: GameClockState {
            absolute_game_ms: MS_PER_HOUR,
            fractional_timescale_remainder: 0,
            timescale: 30,
        },
        lifecycle: None,
        player_cell: Some(0x0001_7f37),
        settings: &GmstSettings::default(),
        perk_catalog_revision: "",
        gmst_catalog_revision: "",
    });
    assert_eq!(snapshot.player.hp_current, 150);
    assert_eq!(snapshot.player.radiation_stage, 200);
    assert_eq!(
        snapshot
            .player
            .perks
            .iter()
            .map(|perk| perk.form_id)
            .collect::<Vec<_>>(),
        vec![0x10, 0x20]
    );
    assert_eq!(
        snapshot
            .effects
            .entries
            .iter()
            .map(|entry| entry.actor_value.as_str())
            .collect::<Vec<_>>(),
        vec!["action_points", "strength"]
    );
    assert_eq!(
        snapshot
            .effects
            .addictions
            .iter()
            .map(|entry| entry.withdrawal_form_id)
            .collect::<Vec<_>>(),
        vec![0xA, 0xB]
    );
    assert_eq!(
        snapshot.effects.addictions[0].phase,
        AddictionPhase::Withdrawing
    );
    assert_eq!(snapshot.crime.bounty, 40);
    assert_eq!(snapshot.crime.karma, -5);
    assert_eq!(snapshot.world.player_cell, Some(0x0001_7f37));
    assert_eq!(snapshot.world.hour, 1);
    assert_eq!(
        world_clock_line(&snapshot),
        "GAME TIME  2277-10-23 01:00  3600000 ms"
    );
    assert_eq!(radiation_stage_line(&snapshot), "RADS  200  STAGE 200");
}

#[test]
fn limb_parts_follow_body_part_order_and_cripple_flags() {
    let mut limbs = LimbState::healthy();
    limbs.part_mut(BodyPartId::LeftLeg).current_milli = 0;
    limbs.part_mut(BodyPartId::LeftLeg).crippled = true;
    let snapshot = inspect_rpg(RpgInspectionInput {
        name: "Player",
        sheet: &CharacterSheet::default(),
        perks: &PerkProgression::default(),
        perk_names: &[],
        unspent_skill_points: 0,
        total_skill_points: 0,
        radiation: RadiationPool::default(),
        effects: &ActiveEffectsLedger::default(),
        addictions: &Addictions::default(),
        current_health: None,
        current_action_points: None,
        limbs: &limbs,
        crime: &CrimeLedger::default(),
        clock: GameClockState::default(),
        lifecycle: None,
        player_cell: None,
        settings: &GmstSettings::default(),
        perk_catalog_revision: "",
        gmst_catalog_revision: "",
    });
    let labels: Vec<_> = snapshot
        .limbs
        .parts
        .iter()
        .map(|part| part.label.as_str())
        .collect();
    assert_eq!(
        labels,
        [
            "head",
            "torso",
            "left_arm",
            "right_arm",
            "left_leg",
            "right_leg"
        ]
    );
    let left_leg = snapshot
        .limbs
        .parts
        .iter()
        .find(|part| part.part == BodyPartId::LeftLeg)
        .unwrap();
    assert!(left_leg.crippled);
    assert_eq!(left_leg.current_milli, 0);
    assert_eq!(left_leg.max_milli, LIMB_MAX_MILLI);
}

#[test]
fn tagged_small_guns_skill_is_projected_not_recalculated_by_callers() {
    let mut sheet = CharacterSheet::default();
    sheet.tagged_skills.insert(ActorSkill::SmallGuns);
    let snapshot = inspect_rpg(RpgInspectionInput {
        name: "Player",
        sheet: &sheet,
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
        settings: &GmstSettings::default(),
        perk_catalog_revision: "",
        gmst_catalog_revision: "",
    });
    let small_guns = snapshot
        .player
        .skills
        .iter()
        .find(|skill| skill.skill == ActorSkill::SmallGuns)
        .unwrap();
    assert_eq!(small_guns.value, sheet.skill_value(ActorSkill::SmallGuns));
}
