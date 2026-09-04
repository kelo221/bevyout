//! Shared viewer adapter for [`bevyout_core::inspection`].
//!
//! Pip-Boy, `showstats`, and BRP probes all format this snapshot. Derived HP,
//! radiation stages, and calendar fields are not recalculated at those seams.

use bevy::prelude::*;
use bevyout_core::chems::Addictions;
use bevyout_core::combat::limbs::LimbState;
use bevyout_core::crime::CrimeLedger;
use bevyout_core::effects::ActiveEffectsLedger;
use bevyout_core::inspection::{
    RpgInspectionInput, RpgInspectionSnapshot, inspect_rpg, perk_display_name,
};
use bevyout_core::perks::PerkProgression;
use bevyout_core::radiation::RadiationPool;
use bevyout_core::stats::{CharacterSheet, GmstSettings};
use bevyout_core::time::GameClockState;

use super::effects::{ActiveEffectsList, PlayerRadiation, PlayerVitals};
use super::game_time::GameTimeRuntime;
use super::player::CameraModeState;
use super::stats::{PerkCatalog, PlayerProgression, StatsSettings};
use crate::vsa::{GMST_CATALOG_REVISION, PERK_CATALOG_REVISION};

fn fps_player(world: &World) -> Option<Entity> {
    world
        .get_resource::<CameraModeState>()
        .and_then(|state| state.player)
        .filter(|&entity| world.entities().contains(entity))
}

fn live_or_stored_effects(world: &World) -> (ActiveEffectsLedger, u16) {
    let stored = world.get_resource::<PlayerProgression>();
    if let Some(entity) = fps_player(world) {
        let ledger = world
            .get::<ActiveEffectsList>(entity)
            .map(|effects| effects.ledger.clone())
            .or_else(|| stored.map(|progression| progression.effects.clone()))
            .unwrap_or_default();
        let rads = world
            .get::<PlayerRadiation>(entity)
            .map(|radiation| radiation.0.rads)
            .or_else(|| stored.map(|progression| progression.radiation.rads))
            .unwrap_or(0);
        return (ledger, rads);
    }
    (
        stored
            .map(|progression| progression.effects.clone())
            .unwrap_or_default(),
        stored
            .map(|progression| progression.radiation.rads)
            .unwrap_or(0),
    )
}

fn live_or_stored_health(world: &World) -> Option<f32> {
    if let Some(entity) = fps_player(world)
        && let Some(vitals) = world.get::<PlayerVitals>(entity)
    {
        return Some(vitals.current_health);
    }
    world
        .get_resource::<PlayerProgression>()
        .and_then(|progression| progression.current_health)
}

/// Builds the shared RPG inspection snapshot from live viewer state.
#[must_use]
pub(crate) fn rpg_snapshot_from_world(world: &World) -> RpgInspectionSnapshot {
    let default_sheet = CharacterSheet::default();
    let default_perks = PerkProgression::default();
    let default_addictions = Addictions::default();
    let default_limbs = LimbState::healthy();
    let default_crime = CrimeLedger::default();
    let default_clock = GameClockState::default();
    let default_settings = GmstSettings::default();
    let progression = world.get_resource::<PlayerProgression>();
    let settings = world
        .get_resource::<StatsSettings>()
        .map(|settings| settings.0)
        .unwrap_or(default_settings);
    let perk_names: Vec<(u32, String)> = world
        .get_resource::<PerkCatalog>()
        .map(|catalog| {
            catalog
                .0
                .values()
                .map(|definition| (definition.form_id, perk_display_name(definition)))
                .collect()
        })
        .unwrap_or_default();
    let (effects, rads) = live_or_stored_effects(world);
    let radiation = progression
        .map(|progression| {
            let mut pool = progression.radiation;
            pool.rads = rads;
            pool
        })
        .unwrap_or_else(|| RadiationPool::new(rads));
    let runtime = world.get_resource::<GameTimeRuntime>();
    let lifecycle = runtime.map(|runtime| runtime.world.snapshot());
    inspect_rpg(RpgInspectionInput {
        name: "Player",
        sheet: progression
            .map(|progression| &progression.stats)
            .unwrap_or(&default_sheet),
        perks: progression
            .map(|progression| &progression.perks)
            .unwrap_or(&default_perks),
        perk_names: &perk_names,
        unspent_skill_points: progression
            .map(|progression| progression.unspent_skill_points)
            .unwrap_or(0),
        total_skill_points: progression
            .map(|progression| progression.total_skill_points)
            .unwrap_or(0),
        radiation,
        effects: &effects,
        addictions: progression
            .map(|progression| &progression.addictions)
            .unwrap_or(&default_addictions),
        current_health: live_or_stored_health(world),
        current_action_points: None,
        limbs: progression
            .map(|progression| &progression.limbs)
            .unwrap_or(&default_limbs),
        crime: progression
            .map(|progression| &progression.crime)
            .unwrap_or(&default_crime),
        clock: runtime
            .map(|runtime| runtime.world.clock)
            .unwrap_or(default_clock),
        lifecycle: lifecycle.as_ref(),
        player_cell: runtime.and_then(|runtime| runtime.world.player_cell),
        settings: &settings,
        perk_catalog_revision: PERK_CATALOG_REVISION,
        gmst_catalog_revision: GMST_CATALOG_REVISION,
    })
}
