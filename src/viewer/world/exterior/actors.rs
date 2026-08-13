//! Gameplay-actor residency for streamed exterior cells (M6 W3-C).
//!
//! This is the runtime adapter between three existing authorities and no new
//! one:
//!
//! * `ExteriorStreamState` owns cell residency and generations.
//! * `viewer::actor_residency` (pure, W3-A) decides every canonical identity
//!   and holder transition; nothing here invents one.
//! * `ActorDefinitionCatalogs` owns prepared definitions per cell and
//!   `ActiveSaveState` owns mutable actor state; both already exist and both
//!   stay the single writer of their own data.
//!
//! Projection itself reuses the interior chain verbatim: an actor is spawned
//! as an ordinary [`PlacementRoot`] under the cell root, which makes
//! `actor::project_prepared_actors` -> `actor_state::seed_actor_states` ->
//! `ai::autonomous` bind its nav agent and start its package with no
//! exterior-specific code. Because the entity is a `ChildOf` the cell root,
//! `finalize_evictions`' root despawn also tears the actor down; this module
//! only has to checkpoint and release *before* that happens.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use bevy::prelude::*;
use bevyout_core::actor_state::ActorInstanceState;
use bevyout_core::manifest::exterior::{
    ExteriorCellLifecycle, ExteriorCoordinatePolicy, PreparedExteriorActor,
};
use bevyout_core::manifest::{PreparedActor, PreparedPlacement, PreparedSemantic};

use super::actor_residency_plan::{
    PlannedActorEntry, PlannedCell, PlannedLiveActor, PlannedOwner, PlannedRequest,
    plan_actor_residency,
};
use super::lifecycle::ExteriorStreamState;
use crate::viewer::WorldAssetRoot;
use crate::viewer::actor_residency::{
    ActorIdentity, ActorResidencyDecision, ActorResidencyOwner, ActorResidencyRejection,
    ActorResidencyRequest, ActorResidencyTransition, CanonicalStateTransition,
    decide_actor_residency, resolve_actor_identity,
};
use crate::viewer::actor_state::{ActorDefinitionCatalogs, ActorStateRuntime};
use crate::viewer::ai::family_runtime::ActorPackageController;
use crate::viewer::nav::api;
use crate::viewer::world::ActiveSaveState;

/// The live ECS projection of one canonically owned exterior actor. It is a
/// projection of the accepted [`ActorResidencyTransition`], not a store: the
/// canonical record stays in `ActiveSaveState` and the definition stays in
/// `ActorDefinitionCatalogs`.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ExteriorResidentActor {
    pub(crate) reference_form_id: u32,
    pub(crate) cell_form_id: u32,
    pub(crate) generation: u64,
}

/// Bookkeeping for catalogs this module inserted, plus the counters and last
/// rejection code the `actorresidency` console command reports.
#[derive(Resource, Default, Debug)]
pub(crate) struct ExteriorActorResidency {
    /// Cells whose prepared catalogs this module inserted, so eviction only
    /// removes what exterior streaming owns (never the startup cell's).
    inserted_catalogs: BTreeSet<u32>,
    /// Cells whose catalog could not be read, so the warning is logged once.
    failed_catalogs: BTreeSet<u32>,
    pub(crate) binds: u64,
    pub(crate) restores: u64,
    pub(crate) handoffs: u64,
    pub(crate) unloads: u64,
    pub(crate) rejections: u64,
    pub(crate) last_rejection: BTreeMap<u32, &'static str>,
}

fn rejection_code(rejection: ActorResidencyRejection) -> &'static str {
    match rejection {
        ActorResidencyRejection::InvalidOwner { .. } => "InvalidOwner",
        ActorResidencyRejection::InvalidHandoff { .. } => "InvalidHandoff",
        ActorResidencyRejection::DuplicateOwner { .. } => "DuplicateOwner",
        ActorResidencyRejection::CompetingOwner { .. } => "CompetingOwner",
        ActorResidencyRejection::StaleSource { .. } => "StaleSource",
    }
}

/// Snapshot of one projectable cell taken while `ExteriorStreamState` is
/// borrowed, so the exclusive phases below never re-borrow it.
struct CellSnapshot {
    cell_form_id: u32,
    root: Option<Entity>,
    actors: Vec<PreparedExteriorActor>,
}

/// Polls the exterior residency ring and keeps exactly one canonical actor
/// projection per prepared reference. Registered between `update_residency`
/// (which marks evictions) and `finalize_evictions` (which despawns the cell
/// root), so an unload always checkpoints before its entities disappear.
pub(crate) fn sync_exterior_actor_residency(world: &mut World) {
    if world.get_resource::<ExteriorStreamState>().is_none() {
        return;
    }
    let (planned_cells, snapshots) = observe_cells(world);
    let live = observe_live_actors(world);
    retire_evicted_catalogs(world, &planned_cells);
    if planned_cells.is_empty() && live.is_empty() {
        return;
    }
    let requests = plan_actor_residency(
        &planned_cells,
        &live.iter().map(|(_, live)| *live).collect::<Vec<_>>(),
    );
    for request in requests {
        apply_request(world, request, &snapshots, &live);
    }
}

fn observe_cells(world: &mut World) -> (Vec<PlannedCell>, BTreeMap<u32, CellSnapshot>) {
    let saved = saved_actor_references(world);
    let state = world.resource::<ExteriorStreamState>();
    let mut planned = Vec::new();
    let mut snapshots = BTreeMap::new();
    for (grid, cell) in &state.cells {
        let projectable = cell.collision_ready
            && matches!(
                cell.state.lifecycle,
                ExteriorCellLifecycle::Ready | ExteriorCellLifecycle::Resident
            )
            && cell.package.is_some()
            && cell.root.is_some();
        let actors = cell
            .package
            .as_ref()
            .map(|package| {
                package
                    .actors
                    .iter()
                    .filter(|actor| actor.initially_enabled && actor.reference_form_id != 0)
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        planned.push(PlannedCell {
            cell_form_id: cell.state.cell_form_id,
            generation: cell.state.generation,
            grid: (grid.x, grid.y),
            projectable,
            evicting: cell.state.lifecycle == ExteriorCellLifecycle::Evicting,
            actors: actors
                .iter()
                .map(|actor| PlannedActorEntry {
                    reference_form_id: actor.reference_form_id,
                    has_saved_state: saved.contains(&actor.reference_form_id),
                })
                .collect(),
        });
        snapshots.insert(
            cell.state.cell_form_id,
            CellSnapshot {
                cell_form_id: cell.state.cell_form_id,
                root: cell.root,
                actors,
            },
        );
    }
    (planned, snapshots)
}

fn saved_actor_references(world: &World) -> BTreeSet<u32> {
    world
        .get_resource::<ActiveSaveState>()
        .map(|save| {
            save.0
                .cells
                .values()
                .flat_map(|cell| cell.actors.keys().copied())
                .collect()
        })
        .unwrap_or_default()
}

fn observe_live_actors(world: &mut World) -> Vec<(Entity, PlannedLiveActor)> {
    let policy = ExteriorCoordinatePolicy::default();
    let mut live = world
        .query::<(Entity, &ExteriorResidentActor, &GlobalTransform)>()
        .iter(world)
        .map(|(entity, resident, transform)| {
            let translation = transform.translation();
            let grid = policy.grid_for_bevy([
                f64::from(translation.x),
                f64::from(translation.y),
                f64::from(translation.z),
            ]);
            (
                entity,
                PlannedLiveActor {
                    reference_form_id: resident.reference_form_id,
                    owner: PlannedOwner {
                        cell_form_id: resident.cell_form_id,
                        generation: resident.generation,
                    },
                    grid: (grid.x, grid.y),
                },
            )
        })
        .collect::<Vec<_>>();
    live.sort_by_key(|(entity, live)| (live.reference_form_id, entity.index_u32()));
    live
}

/// Drops prepared catalogs for cells this module projected that the stream no
/// longer reports at all (their eviction completed in an earlier frame).
fn retire_evicted_catalogs(world: &mut World, cells: &[PlannedCell]) {
    let present = cells
        .iter()
        .map(|cell| cell.cell_form_id)
        .collect::<BTreeSet<_>>();
    let stale = world
        .resource::<ExteriorActorResidency>()
        .inserted_catalogs
        .difference(&present)
        .copied()
        .collect::<Vec<_>>();
    for cell_form_id in stale {
        remove_cell_catalogs(world, cell_form_id);
    }
}

fn apply_request(
    world: &mut World,
    request: PlannedRequest,
    snapshots: &BTreeMap<u32, CellSnapshot>,
    live: &[(Entity, PlannedLiveActor)],
) {
    let reference_form_id = request.reference_form_id();
    let owners = live
        .iter()
        .filter(|(_, actor)| actor.reference_form_id == reference_form_id)
        .filter_map(|(_, actor)| {
            ActorResidencyOwner::new(actor.owner.cell_form_id, actor.owner.generation).ok()
        })
        .collect::<Vec<_>>();
    let entity = live
        .iter()
        .find(|(_, actor)| actor.reference_form_id == reference_form_id)
        .map(|(entity, _)| *entity);

    let saved_reference = saved_reference_form_id(world, reference_form_id);
    let Ok(identity) =
        resolve_actor_identity(reference_form_id, reference_form_id, saved_reference)
    else {
        warn!("exterior actor reject {reference_form_id:08x} InvalidIdentity");
        note_rejection(world, reference_form_id, "InvalidIdentity");
        return;
    };

    let policy_request = match request {
        PlannedRequest::Duplicate { owners, .. } => {
            warn!("exterior actor reject {reference_form_id:08x} DuplicateOwner owners={owners}");
            note_rejection(world, reference_form_id, "DuplicateOwner");
            return;
        }
        PlannedRequest::Bind { destination, .. } => ActorResidencyRequest::Bind {
            destination: owner_token(destination),
        },
        PlannedRequest::Restore { destination, .. } => ActorResidencyRequest::Restore {
            destination: owner_token(destination),
        },
        PlannedRequest::Handoff {
            source,
            destination,
            ..
        } => ActorResidencyRequest::Handoff {
            source: owner_token(source),
            destination: owner_token(destination),
        },
        PlannedRequest::Unload { source, .. } => ActorResidencyRequest::Unload {
            source: owner_token(source),
        },
        PlannedRequest::Retain { owner, .. } => ActorResidencyRequest::Retain {
            owner: owner_token(owner),
        },
    };

    match decide_actor_residency(identity, &owners, policy_request) {
        ActorResidencyDecision::Reject(rejection) => {
            let code = rejection_code(rejection);
            warn!("exterior actor reject {reference_form_id:08x} {code}");
            note_rejection(world, reference_form_id, code);
        }
        ActorResidencyDecision::Apply(plan) => {
            apply_transition(world, identity, plan.transition(), snapshots, entity);
        }
    }
}

fn owner_token(owner: PlannedOwner) -> ActorResidencyOwner {
    // The planner only ever produces owners copied out of a live cell state,
    // whose FormID is non-zero; a zero would be rejected by the policy as
    // `InvalidOwner` anyway, which is what this fallback preserves.
    ActorResidencyOwner::new(owner.cell_form_id, owner.generation)
        .unwrap_or_else(|_| ActorResidencyOwner::new(u32::MAX, owner.generation).expect("non-zero"))
}

fn apply_transition(
    world: &mut World,
    identity: ActorIdentity,
    transition: ActorResidencyTransition,
    snapshots: &BTreeMap<u32, CellSnapshot>,
    entity: Option<Entity>,
) {
    let reference_form_id = identity.reference_form_id();
    match transition {
        ActorResidencyTransition::Bind { owner, state }
        | ActorResidencyTransition::Restore { owner, state } => {
            let restore = matches!(transition, ActorResidencyTransition::Restore { .. });
            let Some(snapshot) = snapshots.get(&owner.cell_form_id()) else {
                return;
            };
            let Some(root) = snapshot.root else {
                return;
            };
            let Some(prepared) = snapshot
                .actors
                .iter()
                .find(|actor| actor.reference_form_id == reference_form_id)
            else {
                return;
            };
            ensure_cell_catalogs(world, snapshot.cell_form_id);
            apply_canonical_state(world, reference_form_id, state);
            spawn_exterior_actor(world, prepared, root, owner);
            let mut residency = world.resource_mut::<ExteriorActorResidency>();
            if restore {
                residency.restores = residency.restores.saturating_add(1);
            } else {
                residency.binds = residency.binds.saturating_add(1);
            }
            info!(
                "exterior actor {} {reference_form_id:08x} cell={:08x} generation={}",
                if restore { "restore" } else { "bind" },
                owner.cell_form_id(),
                owner.generation()
            );
        }
        ActorResidencyTransition::Handoff {
            source,
            destination,
            state,
        } => {
            let Some(entity) = entity else {
                return;
            };
            let Some(root) = snapshots
                .get(&destination.cell_form_id())
                .and_then(|snapshot| snapshot.root)
            else {
                return;
            };
            ensure_cell_catalogs(world, destination.cell_form_id());
            apply_canonical_state(world, reference_form_id, state);
            let Ok(mut actor) = world.get_entity_mut(entity) else {
                return;
            };
            // The entity itself is never respawned: one canonical identity
            // keeps one `Entity` id across the border.
            actor.insert((
                ChildOf(root),
                ExteriorResidentActor {
                    reference_form_id,
                    cell_form_id: destination.cell_form_id(),
                    generation: destination.generation(),
                },
            ));
            if let Some(mut projected) = actor.get_mut::<ActorStateRuntime>() {
                projected.cell_form_id = destination.cell_form_id();
            }
            let mut residency = world.resource_mut::<ExteriorActorResidency>();
            residency.handoffs = residency.handoffs.saturating_add(1);
            info!(
                "exterior actor handoff {reference_form_id:08x} {:08x}->{:08x}",
                source.cell_form_id(),
                destination.cell_form_id()
            );
        }
        ActorResidencyTransition::Unload { owner, state } => {
            let Some(entity) = entity else {
                return;
            };
            capture_package_checkpoint(world, entity, reference_form_id);
            apply_canonical_state(world, reference_form_id, state);
            api::release_actor(world, entity);
            // The cell root's own despawn would take this entity with it, but
            // a cancelled eviction can keep that root alive: despawning the
            // projection here is what makes the unload idempotent and
            // reversal-safe (a cancel then re-binds one fresh actor).
            if let Ok(actor) = world.get_entity_mut(entity) {
                actor.despawn();
            }
            remove_cell_catalogs(world, owner.cell_form_id());
            let mut residency = world.resource_mut::<ExteriorActorResidency>();
            residency.unloads = residency.unloads.saturating_add(1);
            info!(
                "exterior actor unload {reference_form_id:08x} cell={:08x} generation={}",
                owner.cell_form_id(),
                owner.generation()
            );
        }
        ActorResidencyTransition::Retain { .. } => {}
    }
}

fn note_rejection(world: &mut World, reference_form_id: u32, code: &'static str) {
    let mut residency = world.resource_mut::<ExteriorActorResidency>();
    residency.rejections = residency.rejections.saturating_add(1);
    residency.last_rejection.insert(reference_form_id, code);
}

fn saved_reference_form_id(world: &World, reference_form_id: u32) -> Option<u32> {
    world
        .get_resource::<ActiveSaveState>()
        .and_then(|save| {
            save.0
                .cells
                .values()
                .find_map(|cell| cell.actors.get(&reference_form_id))
        })
        .map(|state| state.reference_form_id)
}

/// Applies the canonical-state instruction returned by the policy. The single
/// `ActorInstanceState` record is relocated, never copied: two cells can never
/// hold a record for the same reference.
fn apply_canonical_state(
    world: &mut World,
    reference_form_id: u32,
    transition: CanonicalStateTransition,
) {
    let destination = match transition {
        CanonicalStateTransition::EnsureAt { cell_form_id }
        | CanonicalStateTransition::KeepAt { cell_form_id } => cell_form_id,
        CanonicalStateTransition::Move {
            to_cell_form_id, ..
        } => to_cell_form_id,
    };
    let Some(mut save) = world.get_resource_mut::<ActiveSaveState>() else {
        return;
    };
    let existing = save.0.cells.iter_mut().find_map(|(cell_form_id, cell)| {
        (*cell_form_id != destination)
            .then(|| cell.actors.remove(&reference_form_id))
            .flatten()
    });
    let Some(existing) = existing else {
        return;
    };
    save.0
        .cells
        .entry(destination)
        .or_default()
        .actors
        .insert(reference_form_id, existing);
}

fn capture_package_checkpoint(world: &mut World, entity: Entity, reference_form_id: u32) {
    let checkpoint = world
        .get::<ActorPackageController>(entity)
        .and_then(|controller| controller.lifecycle.to_checkpoint());
    let Some(checkpoint) = checkpoint else {
        return;
    };
    let Some(mut save) = world.get_resource_mut::<ActiveSaveState>() else {
        return;
    };
    let Some(state) = save
        .0
        .cells
        .values_mut()
        .find_map(|cell| cell.actors.get_mut(&reference_form_id))
    else {
        return;
    };
    state.package = Some(checkpoint);
}

fn spawn_exterior_actor(
    world: &mut World,
    prepared: &PreparedExteriorActor,
    root: Entity,
    owner: ActorResidencyOwner,
) {
    let placement = exterior_actor_placement(prepared);
    let transform = Transform {
        translation: Vec3::from_array(prepared.position),
        rotation: Quat::from_xyzw(
            prepared.rotation_xyzw[0],
            prepared.rotation_xyzw[1],
            prepared.rotation_xyzw[2],
            prepared.rotation_xyzw[3],
        ),
        scale: Vec3::splat(crate::viewer::actor::placement_root_scale(&placement)),
    };
    let asset = placement.asset_path.clone().and_then(|path| {
        world
            .get_resource::<AssetServer>()
            .map(|server| server.load(GltfAssetLabel::Scene(0).from_asset(path)))
    });
    let mut entity = world.spawn((
        crate::viewer::interaction::PlacementRoot::new(placement),
        transform,
        Visibility::Inherited,
        ChildOf(root),
        ExteriorResidentActor {
            reference_form_id: prepared.reference_form_id,
            cell_form_id: owner.cell_form_id(),
            generation: owner.generation(),
        },
    ));
    if let Some(handle) = asset {
        entity.insert(WorldAssetRoot(handle));
    }
}

/// Projects a prepared exterior actor onto the same [`PreparedPlacement`]
/// shape the interior scene spawner produces, so one projection chain serves
/// both worlds.
fn exterior_actor_placement(prepared: &PreparedExteriorActor) -> PreparedPlacement {
    let actor = PreparedActor {
        base_template_form_id: None,
        assembly: prepared.assembly.clone(),
    };
    PreparedPlacement {
        reference_form_id: prepared.reference_form_id,
        base_form_id: prepared.base_form_id,
        asset_path: prepared.asset_path.clone(),
        translation: prepared.position,
        rotation_xyzw: prepared.rotation_xyzw,
        scale: prepared.scale,
        error: None,
        physics_asset_path: prepared.physics_asset_path.clone(),
        physics_source: None,
        physics_classification: Default::default(),
        step_support: false,
        mutability: Default::default(),
        mutability_root_form_id: None,
        reference_kind: match prepared.kind {
            bevyout_core::actor::ActorKind::Humanoid => "ACHR".to_owned(),
            bevyout_core::actor::ActorKind::Creature => "ACRE".to_owned(),
        },
        base_kind: String::new(),
        editor_id: None,
        display_name: None,
        count: 1,
        semantic: match prepared.kind {
            bevyout_core::actor::ActorKind::Humanoid => PreparedSemantic::Npc(actor),
            bevyout_core::actor::ActorKind::Creature => PreparedSemantic::Creature(actor),
        },
        initially_enabled: prepared.initially_enabled,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        linked_reference_form_id: None,
        inventory: Vec::new(),
        audio: Default::default(),
        ao_mode: String::new(),
    }
}

/// Reads a streamed cell's prepared actor/animation catalogs once and hands
/// them to the existing multi-cell registries. Both are keyed by cell FormID
/// already; this module only decides *when* an exterior cell's catalogs enter
/// and leave them.
fn ensure_cell_catalogs(world: &mut World, cell_form_id: u32) {
    {
        let residency = world.resource::<ExteriorActorResidency>();
        if residency.inserted_catalogs.contains(&cell_form_id)
            || residency.failed_catalogs.contains(&cell_form_id)
        {
            return;
        }
    }
    let Some(asset_root) = world.resource::<ExteriorStreamState>().asset_root.clone() else {
        return;
    };
    match read_cell_catalogs(&asset_root, cell_form_id) {
        Ok((definitions, animations)) => {
            if let Some(catalog) = definitions {
                world
                    .resource_mut::<ActorDefinitionCatalogs>()
                    .insert(cell_form_id, catalog);
            }
            if let Some(catalog) = animations {
                world
                    .resource_mut::<crate::viewer::actor_animation::ActorAnimationCatalogs>()
                    .insert(cell_form_id, catalog);
            }
            world
                .resource_mut::<ExteriorActorResidency>()
                .inserted_catalogs
                .insert(cell_form_id);
        }
        Err(error) => {
            warn!("exterior actor catalog unavailable for cell {cell_form_id:08x}: {error:#}");
            world
                .resource_mut::<ExteriorActorResidency>()
                .failed_catalogs
                .insert(cell_form_id);
        }
    }
}

type CellCatalogs = (
    Option<crate::vsa::PreparedActorCatalog>,
    Option<bevyout_core::actor_animation::PreparedActorAnimationCatalog>,
);

fn read_cell_catalogs(asset_root: &Path, cell_form_id: u32) -> anyhow::Result<CellCatalogs> {
    let path = crate::viewer::world::preload::scene_manifest_path(asset_root, cell_form_id);
    let text = std::fs::read_to_string(&path)?;
    let manifest = ron::de::from_str::<crate::viewer::PreparedSceneManifest>(&text)?;
    let definitions = crate::viewer::actor_state::load_catalog_for_manifest(&manifest, asset_root)?;
    let animations =
        crate::viewer::actor_animation::load_catalog_for_manifest(&manifest, asset_root)?;
    Ok((definitions, animations))
}

fn remove_cell_catalogs(world: &mut World, cell_form_id: u32) {
    let owned = world
        .resource_mut::<ExteriorActorResidency>()
        .inserted_catalogs
        .remove(&cell_form_id);
    world
        .resource_mut::<ExteriorActorResidency>()
        .failed_catalogs
        .remove(&cell_form_id);
    if !owned {
        return;
    }
    world
        .resource_mut::<ActorDefinitionCatalogs>()
        .remove(cell_form_id);
    world
        .resource_mut::<crate::viewer::actor_animation::ActorAnimationCatalogs>()
        .remove(cell_form_id);
}

/// `actorresidency`: one line of state per currently projected exterior
/// actor. Read-only -- it reports the existing authorities, never mutates.
pub(crate) fn actor_residency_json(world: &mut World) -> serde_json::Value {
    let mut actors = world
        .query::<(Entity, &ExteriorResidentActor)>()
        .iter(world)
        .map(|(entity, resident)| (entity, *resident))
        .collect::<Vec<_>>();
    actors.sort_by_key(|(entity, resident)| (resident.reference_form_id, entity.index_u32()));
    let rows = actors
        .into_iter()
        .map(|(entity, resident)| {
            let nav_bound = api::is_bound(world, entity);
            let package = world.get::<ActorPackageController>(entity).map(|controller| {
                serde_json::json!({
                    "selected": format!("{:08x}", controller.selected_form_id),
                    "phase": format!("{:?}", controller.lifecycle.phase()),
                    "step": controller.lifecycle.to_checkpoint().map(|checkpoint| checkpoint.procedure_index),
                })
            });
            let life = world
                .get::<ActorStateRuntime>(entity)
                .map(|state| state.life_state.label().to_owned());
            let last_rejection = world
                .get_resource::<ExteriorActorResidency>()
                .and_then(|residency| {
                    residency
                        .last_rejection
                        .get(&resident.reference_form_id)
                        .copied()
                });
            serde_json::json!({
                "reference": format!("{:08x}", resident.reference_form_id),
                "cell": format!("{:08x}", resident.cell_form_id),
                "generation": resident.generation,
                "nav_bound": nav_bound,
                "life": life,
                "package": package,
                "last_rejection": last_rejection,
            })
        })
        .collect::<Vec<_>>();
    let residency = world.get_resource::<ExteriorActorResidency>();
    serde_json::json!({
        "actors": rows,
        "binds": residency.map_or(0, |residency| residency.binds),
        "restores": residency.map_or(0, |residency| residency.restores),
        "handoffs": residency.map_or(0, |residency| residency.handoffs),
        "unloads": residency.map_or(0, |residency| residency.unloads),
        "rejections": residency.map_or(0, |residency| residency.rejections),
    })
}

/// Reads the saved package checkpoint for a reference, used by the autonomous
/// driver to resume a package after a reload at a new generation.
pub(crate) fn saved_package_checkpoint(
    world: &World,
    reference_form_id: u32,
) -> Option<bevyout_core::actor_state::ActorPackageCheckpoint> {
    world
        .get_resource::<ActiveSaveState>()?
        .0
        .cells
        .values()
        .find_map(|cell| cell.actors.get(&reference_form_id))
        .and_then(|state: &ActorInstanceState| state.package)
}

#[cfg(test)]
#[path = "actors_tests.rs"]
mod tests;
