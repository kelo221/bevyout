//! Player RPG stats runtime (M9 wave 1, #310).
//!
//! Thin Bevy adapter over the pure `bevyout_core::stats` kernels (#309):
//! `PlayerProgression` is the authoritative state that survives transient FPS
//! player entities. `ActorStats`, `DerivedAttributes`, and `Experience` are
//! entity projections, and `StatsSettings` carries the prepared GMST settings
//! (GOTY defaults until a catalog overrides them). `PerkCatalog` is the
//! prepared perk definitions for console commands (M9 wave 2, #314). Wave 3
//! radiation, active effects, addictions, and current health live on the same
//! resource so camera-mode despawn cannot drop them.

use bevy::prelude::*;
use bevyout_core::chems::Addictions as CoreAddictions;
use bevyout_core::combat::body::BodyPartId;
use bevyout_core::combat::limbs::LimbState;
use bevyout_core::combat::medical::{MedicalSource, restore_limbs};
use bevyout_core::effects::{ActiveEffectsLedger, projected_derived_with_limbs};
use bevyout_core::perks::{PerkDefinition, PerkProgression};
use bevyout_core::radiation::RadiationPool;
use bevyout_core::stats::{
    CharacterSheet, DerivedAttributes as CoreDerivedAttributes, GmstSettings, xp_threshold,
};
use bevyout_core::time::GameTime;
use std::collections::BTreeMap;
use std::path::Path;

use super::effects::{ActiveEffectsList, Addictions, PlayerRadiation, PlayerVitals};
use super::player::FpsPlayer;
use super::plugins::ViewerSet;
use crate::vsa::{
    GMST_CATALOG_REVISION, PERK_CATALOG_REVISION, PreparedGmstCatalog, PreparedPerkCatalog,
    PreparedSceneManifest,
};

/// GMST settings resolved at startup; kernels read them from here.
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct StatsSettings(pub(crate) GmstSettings);

/// Prepared perk definitions keyed by FormID (M9 wave 2, #314), loaded
/// from `catalogs/<source_fingerprint>/perks.ron` at startup. An absent
/// catalog degrades to "no perks known": the console's `addperk` errors
/// with `unknown_perk` rather than crashing startup.
#[derive(Resource, Debug, Clone, Default, PartialEq)]
pub(crate) struct PerkCatalog(pub(crate) BTreeMap<u32, PerkDefinition>);

/// Persistent player-authored progression state. The FPS player entity is
/// transient across camera-mode changes, so console and gameplay systems
/// mutate this resource rather than an entity projection.
#[derive(Resource, Debug, Clone, Default, PartialEq)]
pub(crate) struct PlayerProgression {
    pub(crate) stats: CharacterSheet,
    pub(crate) perks: PerkProgression,
    pub(crate) unspent_skill_points: u16,
    pub(crate) total_skill_points: u16,
    pub(crate) radiation: RadiationPool,
    pub(crate) effects: ActiveEffectsLedger,
    pub(crate) chem_doses_ms: BTreeMap<u32, u32>,
    pub(crate) addictions: CoreAddictions,
    pub(crate) current_health: Option<f32>,
    pub(crate) limbs: LimbState,
}

/// Player-selected limb for targeted Stimpak restoration. Defaults to torso.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PlayerLimbTarget(pub(crate) BodyPartId);

impl Default for PlayerLimbTarget {
    fn default() -> Self {
        Self(BodyPartId::Torso)
    }
}

pub(crate) fn restore_targeted_stimpak(progression: &mut PlayerProgression, part: BodyPartId) {
    restore_limbs(
        &mut progression.limbs,
        MedicalSource::TargetedStimpak,
        Some(part),
        GameTime::from_ms(0),
    );
}

/// Player-authored progression sheet projected onto the transient FPS entity.
#[derive(Component, Debug, Clone, Default, PartialEq)]
pub(crate) struct ActorStats(pub(crate) CharacterSheet);

/// Recomputed projection of the sheet's derived attributes.
#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct DerivedAttributes(pub(crate) CoreDerivedAttributes);

/// Recomputed XP/level projection; `unspent_skill_points` accumulates what
/// level-ups granted minus what later slices spend.
#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Experience {
    pub(crate) xp: u32,
    pub(crate) level: u8,
    pub(crate) xp_into_level: u32,
    pub(crate) next_threshold: u32,
    pub(crate) unspent_skill_points: u16,
    pub(crate) total_skill_points: u16,
}

pub(crate) struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StatsSettings>()
            .init_resource::<PerkCatalog>()
            .init_resource::<PlayerProgression>()
            .init_resource::<PlayerLimbTarget>()
            .add_systems(
                Update,
                (attach_stats_to_player, recalculate_derived_stats)
                    .chain()
                    .in_set(ViewerSet::WorldSync),
            );
    }
}

fn attach_stats_to_player(
    players: Query<Entity, Added<FpsPlayer>>,
    progression: Res<PlayerProgression>,
    mut commands: Commands,
) {
    for entity in &players {
        commands
            .entity(entity)
            .insert(ProjectionBundle::from(&progression));
    }
}

#[derive(Bundle)]
struct ProjectionBundle {
    stats: ActorStats,
    derived: DerivedAttributes,
    experience: Experience,
}

impl ProjectionBundle {
    fn from(progression: &PlayerProgression) -> Self {
        Self {
            stats: ActorStats(progression.stats.clone()),
            derived: DerivedAttributes::default(),
            experience: Experience {
                unspent_skill_points: progression.unspent_skill_points,
                total_skill_points: progression.total_skill_points,
                ..default()
            },
        }
    }
}

pub(crate) fn restore_player_progression(world: &mut World, entity: Entity) {
    let Some(progression) = world.get_resource::<PlayerProgression>().cloned() else {
        return;
    };
    world
        .entity_mut(entity)
        .insert(ProjectionBundle::from(&progression));
}

type DerivedStatsQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut ActorStats,
        &'static mut DerivedAttributes,
        &'static mut Experience,
        Option<&'static ActiveEffectsList>,
        Option<&'static PlayerRadiation>,
    ),
>;

/// Keeps the active FPS entity as a projection of the persistent progression
/// resource. Console commands mutate the resource directly, so this also
/// repairs the projection after a command batch without relying on entity
/// change detection. Derived max health includes active effects and radiation
/// when those components are present.
fn recalculate_derived_stats(
    settings: Res<StatsSettings>,
    progression: Res<PlayerProgression>,
    mut players: DerivedStatsQuery,
) {
    for (mut stats, mut derived, mut experience, effects, radiation) in &mut players {
        stats.0 = progression.stats.clone();
        let rads = radiation.map_or(progression.radiation.rads, |radiation| radiation.0.rads);
        derived.0 = match effects {
            Some(effects) => projected_derived_with_limbs(
                &stats.0,
                &effects.ledger,
                rads,
                &settings.0,
                Some(&progression.limbs),
            ),
            None => projected_derived_with_limbs(
                &stats.0,
                &progression.effects,
                rads,
                &settings.0,
                Some(&progression.limbs),
            ),
        };
        experience.xp = stats.0.xp;
        experience.level = stats.0.level;
        experience.xp_into_level = stats.0.xp_into_level(&settings.0);
        experience.next_threshold = xp_threshold(stats.0.level.saturating_add(1), &settings.0);
        experience.unspent_skill_points = progression.unspent_skill_points;
        experience.total_skill_points = progression.total_skill_points;
    }
}

pub(crate) fn persist_player_effects(world: &mut World) {
    let Some(entity) = world
        .query_filtered::<Entity, With<FpsPlayer>>()
        .iter(world)
        .next()
    else {
        return;
    };
    let radiation = world
        .get::<PlayerRadiation>(entity)
        .map(|radiation| radiation.0);
    let effects = world.get::<ActiveEffectsList>(entity).cloned();
    let addictions = world
        .get::<Addictions>(entity)
        .map(|addictions| addictions.0.clone());
    let current_health = world
        .get::<PlayerVitals>(entity)
        .map(|vitals| vitals.current_health);
    let Some(mut progression) = world.get_resource_mut::<PlayerProgression>() else {
        return;
    };
    if let Some(radiation) = radiation {
        progression.radiation = radiation;
    }
    if let Some(effects) = effects {
        progression.effects = effects.ledger;
        progression.chem_doses_ms = effects.chem_doses_ms;
    }
    if let Some(addictions) = addictions {
        progression.addictions = addictions;
    }
    if let Some(current_health) = current_health {
        progression.current_health = Some(current_health);
    }
}

/// Loads GMST settings for a manifest's content set from the deterministic
/// `catalogs/<source_fingerprint>/gmst.ron` path (M9 wave 1, #308). Missing
/// or stale catalogs fall back to the GOTY defaults with a warning -- unlike
/// the item catalog, every setting has a safe default.
pub(crate) fn load_settings_for_manifest(
    manifest: &PreparedSceneManifest,
    asset_root: &Path,
) -> StatsSettings {
    let path = asset_root
        .join("catalogs")
        .join(&manifest.source_fingerprint)
        .join("gmst.ron");
    let Some(text) = std::fs::read_to_string(&path).ok() else {
        warn!("gmst settings: no gmst catalog for this content set, using GOTY defaults");
        return StatsSettings::default();
    };
    let catalog: PreparedGmstCatalog = match ron::from_str(&text) {
        Ok(catalog) => catalog,
        Err(error) => {
            warn!("gmst settings: gmst catalog unreadable ({error}), using GOTY defaults");
            return StatsSettings::default();
        }
    };
    if catalog.revision != GMST_CATALOG_REVISION {
        warn!(
            "gmst settings: gmst catalog revision {} is stale, expected {GMST_CATALOG_REVISION}; run `prepare` again (using GOTY defaults)",
            catalog.revision
        );
        return StatsSettings::default();
    }
    if catalog.source_fingerprint != manifest.source_fingerprint {
        warn!("gmst settings: gmst catalog fingerprint mismatch, using GOTY defaults");
        return StatsSettings::default();
    }
    if let Err(error) = catalog.settings.validate() {
        warn!("gmst settings: catalog settings invalid ({error}), using GOTY defaults");
        return StatsSettings::default();
    }
    info!(
        "gmst settings: loaded {} settings ({} consumed) from gmst catalog",
        catalog.counters.total, catalog.counters.consumed
    );
    StatsSettings(catalog.settings)
}

/// Loads the perk catalog for a manifest's content set from the
/// deterministic `catalogs/<source_fingerprint>/perks.ron` path (M9 wave 2,
/// #312/#314). Like the gmst catalog -- and unlike the item catalog -- a
/// missing, stale, or unreadable perk catalog degrades to an empty catalog
/// with a warning: perk commands report `unknown_perk` instead of failing
/// viewer startup.
pub(crate) fn load_perk_catalog_for_manifest(
    manifest: &PreparedSceneManifest,
    asset_root: &Path,
) -> PerkCatalog {
    let path = asset_root
        .join("catalogs")
        .join(&manifest.source_fingerprint)
        .join("perks.ron");
    let Some(text) = std::fs::read_to_string(&path).ok() else {
        warn!("perks: no perk catalog for this content set, perk commands disabled");
        return PerkCatalog::default();
    };
    let catalog: PreparedPerkCatalog = match ron::from_str(&text) {
        Ok(catalog) => catalog,
        Err(error) => {
            warn!("perks: perk catalog unreadable ({error}), perk commands disabled");
            return PerkCatalog::default();
        }
    };
    if catalog.revision != PERK_CATALOG_REVISION {
        warn!(
            "perks: perk catalog revision {} is stale, expected {PERK_CATALOG_REVISION}; run `prepare` again (perk commands disabled)",
            catalog.revision
        );
        return PerkCatalog::default();
    }
    if catalog.source_fingerprint != manifest.source_fingerprint {
        warn!("perks: perk catalog fingerprint mismatch, perk commands disabled");
        return PerkCatalog::default();
    }
    info!(
        "perks: loaded {} perks ({} playable) from perk catalog",
        catalog.counters.total, catalog.counters.playable
    );
    PerkCatalog(
        catalog
            .entries
            .into_iter()
            .map(|perk| (perk.form_id, perk))
            .collect(),
    )
}
