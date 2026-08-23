//! Player RPG stats runtime (M9 wave 1, #310).
//!
//! Thin Bevy adapter over the pure `bevyout_core::stats` kernels (#309):
//! `PlayerProgression` is the authoritative state that survives transient FPS
//! player entities. `ActorStats`, `DerivedAttributes`, and `Experience` are
//! entity projections, and `StatsSettings` carries the prepared GMST settings
//! (GOTY defaults until a catalog overrides them).

use bevy::prelude::*;
use bevyout_core::stats::{
    CharacterSheet, DerivedAttributes as CoreDerivedAttributes, GmstSettings, xp_threshold,
};
use std::path::Path;

use super::player::FpsPlayer;
use super::plugins::ViewerSet;
use crate::vsa::{GMST_CATALOG_REVISION, PreparedGmstCatalog, PreparedSceneManifest};

/// GMST settings resolved at startup; kernels read them from here.
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct StatsSettings(pub(crate) GmstSettings);

/// Persistent player-authored progression state. The FPS player entity is
/// transient across camera-mode changes, so console and gameplay systems
/// mutate this resource rather than an entity projection.
#[derive(Resource, Debug, Clone, Default, PartialEq)]
pub(crate) struct PlayerProgression {
    pub(crate) stats: CharacterSheet,
    pub(crate) unspent_skill_points: u16,
    pub(crate) total_skill_points: u16,
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
            .init_resource::<PlayerProgression>()
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

/// Keeps the active FPS entity as a projection of the persistent progression
/// resource. Console commands mutate the resource directly, so this also
/// repairs the projection after a command batch without relying on entity
/// change detection.
fn recalculate_derived_stats(
    settings: Res<StatsSettings>,
    progression: Res<PlayerProgression>,
    mut players: Query<(&mut ActorStats, &mut DerivedAttributes, &mut Experience)>,
) {
    for (mut stats, mut derived, mut experience) in &mut players {
        stats.0 = progression.stats.clone();
        derived.0 = stats.0.derived(&settings.0);
        experience.xp = stats.0.xp;
        experience.level = stats.0.level;
        experience.xp_into_level = stats.0.xp_into_level(&settings.0);
        experience.next_threshold = xp_threshold(stats.0.level.saturating_add(1), &settings.0);
        experience.unspent_skill_points = progression.unspent_skill_points;
        experience.total_skill_points = progression.total_skill_points;
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
