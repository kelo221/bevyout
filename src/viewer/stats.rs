//! Player RPG stats runtime (M9 wave 1, #310).
//!
//! Thin Bevy adapter over the pure `bevyout_core::stats` kernels (#309):
//! `ActorStats` is the authoritative progression state, `DerivedAttributes`
//! and `Experience` are recomputed projections, and `StatsSettings` carries
//! the prepared GMST settings (GOTY defaults until a catalog overrides
//! them). The FPS player is despawned/respawned on camera-mode changes, so
//! components attach through `Added<FpsPlayer>` exactly like the actor-state
//! seeding pattern.

use bevy::prelude::*;
use bevyout_core::effects::projected_derived;
use bevyout_core::perks::PerkDefinition;
use bevyout_core::stats::{
    CharacterSheet, DerivedAttributes as CoreDerivedAttributes, GmstSettings, xp_threshold,
};
use std::collections::BTreeMap;
use std::path::Path;

use super::effects::{ActiveEffectsList, EffectsSet, PlayerRadiation};
use super::player::FpsPlayer;
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

/// Owned perk ranks; the authoritative perk progression state next to
/// `ActorStats` (#314). Mutated only by console commands; modifiers are
/// recomputed from it on demand.
#[derive(Component, Debug, Clone, Default, PartialEq)]
pub(crate) struct ActorPerks(pub(crate) bevyout_core::perks::PerkProgression);

/// Authoritative player-authored progression state. Only console and future
/// gameplay systems mutate this sheet; everything else reads projections.
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
            .add_systems(Update, attach_stats_to_player.in_set(EffectsSet::Attach))
            .add_systems(
                Update,
                recalculate_derived_stats.in_set(EffectsSet::Project),
            );
    }
}

fn attach_stats_to_player(players: Query<Entity, Added<FpsPlayer>>, mut commands: Commands) {
    for entity in &players {
        commands
            .entity(entity)
            .insert(ActorStats::default())
            .insert(ActorPerks::default())
            .insert(DerivedAttributes::default())
            .insert(Experience::default());
    }
}

type DerivedProjectionQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static ActorStats,
        &'static ActiveEffectsList,
        &'static PlayerRadiation,
        &'static mut DerivedAttributes,
        &'static mut Experience,
    ),
    Or<(
        Changed<ActorStats>,
        Changed<ActiveEffectsList>,
        Changed<PlayerRadiation>,
    )>,
>;

fn recalculate_derived_stats(settings: Res<StatsSettings>, mut players: DerivedProjectionQuery) {
    for (stats, effects, radiation, mut derived, mut experience) in &mut players {
        derived.0 = projected_derived(&stats.0, &effects.ledger, radiation.0.rads, &settings.0);
        experience.xp = stats.0.xp;
        experience.level = stats.0.level;
        experience.xp_into_level = stats.0.xp_into_level(&settings.0);
        experience.next_threshold = xp_threshold(stats.0.level.saturating_add(1), &settings.0);
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
