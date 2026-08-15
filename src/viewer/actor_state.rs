//! Authoritative mutable actor-state projection for normal game flow.
//!
//! Prepared catalogs own immutable definitions; `ActiveSaveState` owns
//! per-reference mutations even while a cell has no entity. ECS components
//! are disposable projections and canonical item holders remain the sole
//! inventory/equipment authority.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result, bail};
use bevy::prelude::*;
use bevyout_core::actor_state::{ActorDefinition, ActorInstanceState, ActorLifeState};

use super::actor::ActorRuntime;
use super::world::ActiveSaveState;
use super::{PreparedSceneManifest, fingerprint};
use crate::vsa::{ACTOR_CATALOG_REVISION, ActorCatalogEntry, PreparedActorCatalog};

pub(crate) struct ActorStatePlugin;

impl Plugin for ActorStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorDefinitionCatalogs>().add_systems(
            Update,
            (seed_actor_states, sync_actor_lifecycle)
                .chain()
                .in_set(super::plugins::ViewerSet::WorldSync),
        );
    }
}

/// Prepared actor catalogs keyed by resident cell. Mutable state deliberately
/// does not live here: catalogs may be evicted while save state must survive.
#[derive(Resource, Default)]
pub(crate) struct ActorDefinitionCatalogs(HashMap<u32, Arc<PreparedActorCatalog>>);

impl ActorDefinitionCatalogs {
    pub(crate) fn insert(&mut self, cell_form_id: u32, catalog: PreparedActorCatalog) {
        self.0.insert(cell_form_id, Arc::new(catalog));
    }

    pub(crate) fn remove(&mut self, cell_form_id: u32) {
        self.0.remove(&cell_form_id);
    }

    /// Every resident cell's catalog in deterministic cell order. Streamed
    /// exterior cells insert their catalogs here too (M6 W3-C), so a caller
    /// that needs a blueprint must search this registry rather than the
    /// startup manifest's single catalog.
    pub(crate) fn catalogs(&self) -> Vec<(u32, Arc<PreparedActorCatalog>)> {
        let mut catalogs = self
            .0
            .iter()
            .map(|(cell_form_id, catalog)| (*cell_form_id, Arc::clone(catalog)))
            .collect::<Vec<_>>();
        catalogs.sort_by_key(|(cell_form_id, _)| *cell_form_id);
        catalogs
    }

    pub(crate) fn definition(
        &self,
        reference_form_id: u32,
        base_form_id: u32,
    ) -> Option<(u32, ActorDefinition)> {
        self.0.iter().find_map(|(cell, catalog)| {
            catalog.entries.iter().find_map(|entry| match entry {
                ActorCatalogEntry::Prepared(blueprint)
                    if blueprint.reference_form_id == reference_form_id
                        || (blueprint.reference_form_id == 0
                            && blueprint.base_form_id == base_form_id) =>
                {
                    Some((*cell, blueprint.runtime_definition()))
                }
                _ => None,
            })
        })
    }
}

/// Disposable ECS view of one immutable definition plus current lifecycle.
/// The corresponding `ActorInstanceState` stays in `ActiveSaveState`.
#[derive(Component, Clone, Debug)]
pub(crate) struct ActorStateRuntime {
    pub(crate) cell_form_id: u32,
    pub(crate) definition: Arc<ActorDefinition>,
    pub(crate) life_state: ActorLifeState,
}

/// Reads and validates the manifest-linked per-cell actor catalog. An absent
/// link is valid for actor-free cells; a stale/mismatched link is not.
pub(crate) fn load_catalog_for_manifest(
    manifest: &PreparedSceneManifest,
    asset_root: &Path,
) -> Result<Option<PreparedActorCatalog>> {
    let Some(relative) = manifest.actor_catalog_path.as_deref() else {
        return Ok(None);
    };
    let path = asset_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    let bytes =
        fs::read(&path).with_context(|| format!("reading actor catalog {}", path.display()))?;
    if manifest.actor_catalog_hash.as_deref() != Some(fingerprint(&bytes).as_str()) {
        bail!("actor catalog hash does not match scene manifest");
    }
    let catalog: PreparedActorCatalog = ron::de::from_bytes(&bytes)
        .with_context(|| format!("invalid actor catalog {}", path.display()))?;
    if manifest.actor_catalog_revision.as_deref() != Some(catalog.revision.as_str()) {
        bail!("actor catalog revision does not match scene manifest");
    }
    if catalog.revision != ACTOR_CATALOG_REVISION {
        bail!(
            "actor catalog revision {} is stale, expected {ACTOR_CATALOG_REVISION}; run `prepare` again",
            catalog.revision
        );
    }
    if catalog.source_fingerprint != manifest.source_fingerprint {
        bail!(
            "actor catalog fingerprint {} does not match scene {}",
            catalog.source_fingerprint,
            manifest.source_fingerprint
        );
    }
    Ok(Some(catalog))
}

pub(crate) fn seed_actor_states(
    mut commands: Commands,
    actors: Query<(Entity, &ActorRuntime), Added<ActorRuntime>>,
    catalogs: Res<ActorDefinitionCatalogs>,
    mut save: ResMut<ActiveSaveState>,
) {
    for (entity, runtime) in &actors {
        let Some((cell_form_id, definition)) =
            catalogs.definition(runtime.reference_form_id, runtime.base_form_id)
        else {
            warn!(
                "actor state unresolved {:08x}: no prepared actor definition",
                runtime.reference_form_id
            );
            continue;
        };
        if let Err(error) = definition.validate() {
            warn!(
                "actor state invalid {:08x}: {error}",
                runtime.reference_form_id
            );
            continue;
        }
        let actors = &mut save.0.cells.entry(cell_form_id).or_default().actors;
        let (life_state, outcome) = match actors.entry(runtime.reference_form_id) {
            std::collections::btree_map::Entry::Occupied(entry) => {
                (entry.get().life_state, "restored")
            }
            std::collections::btree_map::Entry::Vacant(entry) => {
                let state =
                    ActorInstanceState::new(runtime.reference_form_id, ActorLifeState::Alive);
                entry.insert(state);
                (ActorLifeState::Alive, "seeded")
            }
        };
        commands.entity(entity).insert(ActorStateRuntime {
            cell_form_id,
            definition: Arc::new(definition),
            life_state,
        });
        info!(
            "actor state {outcome} {:08x} cell={cell_form_id:08x} life={}",
            runtime.reference_form_id,
            life_state.label()
        );
    }
}

fn sync_actor_lifecycle(
    save: Res<ActiveSaveState>,
    mut actors: Query<(&ActorRuntime, &mut ActorStateRuntime)>,
) {
    if !save.is_changed() {
        return;
    }
    for (runtime, mut projected) in &mut actors {
        let Some(state) = save
            .0
            .cells
            .get(&projected.cell_form_id)
            .and_then(|cell| cell.actors.get(&runtime.reference_form_id))
        else {
            continue;
        };
        projected.life_state = state.life_state;
    }
}

#[cfg(test)]
#[path = "tests/actor_state.rs"]
mod tests;
