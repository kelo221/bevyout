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
use bevy::tasks::futures::check_ready;
use bevy::tasks::{AsyncComputeTaskPool, Task, TaskPool};
use bevyout_core::actor_state::ActorInstanceState;
use bevyout_core::manifest::exterior::{
    ExteriorCellLifecycle, ExteriorCoordinatePolicy, GridCoordinate, PreparedExteriorActor,
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
    /// Cells with an in-flight background catalog read (issue #305 review):
    /// guards [`ensure_cell_catalogs`] against spawning a second task for the
    /// same cell while the first is still resolving.
    pending_catalog_tasks: BTreeSet<u32>,
    pub(crate) binds: u64,
    pub(crate) restores: u64,
    pub(crate) handoffs: u64,
    pub(crate) unloads: u64,
    pub(crate) rejections: u64,
    pub(crate) last_rejection: BTreeMap<u32, &'static str>,
    /// References this module has ever observed nav-bound (a `tna bind` or a
    /// successful autonomous-driver bind). Once a reference enters this set it
    /// stays until the process exits -- deliberately never cleared on unload,
    /// so a bind survives the full evict/restore round trip, not just a
    /// handoff. [`reconcile_actor_nav_bindings`] is the sole reader/writer.
    pub(crate) bound_references: BTreeSet<u32>,
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

/// Snapshot of one cell's identity taken while `ExteriorStreamState` is
/// borrowed, so the exclusive phases below never re-borrow it. Prepared actor
/// data is deliberately *not* copied here: an assembly blueprint is large and
/// this runs every frame, so it is read once, at the moment a bind actually
/// spawns ([`prepared_actor_for`]).
struct CellSnapshot {
    cell_form_id: u32,
    grid: GridCoordinate,
    root: Option<Entity>,
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
    // Observed and retried before this frame's own transitions run: a
    // reference about to be unloaded this same frame is still a live,
    // valid entity right here, so its bound state is captured before
    // `apply_request`'s `Unload` arm despawns it a few lines down.
    reconcile_actor_nav_bindings(world, &live);
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

/// Recovers a previously nav-bound actor's binding once its post-transition
/// state has settled, polled every frame alongside residency itself.
///
/// Neither `Handoff` nor `Restore` respawns a bound actor's nav agent
/// itself (issue #303): `Handoff` never touches the nav components at all
/// (the same `Entity` keeps them across the border), and `Restore` reuses
/// the ordinary spawn chain (`project_prepared_actors` ->
/// `seed_actor_states` -> `ai::autonomous`) verbatim, so it binds exactly
/// when a fresh `Bind` would and no more -- an actor whose package fails to
/// start (e.g. no linked-reference location resolves) is deliberately left
/// unbound by that chain, same as on first load. Watching `is_bound` here
/// catches the actual failure mode live re-verification found: a bind can
/// be lost *after* a transition already committed, most visibly when a
/// freshly-handed-off actor free-falls through not-yet-settled destination
/// collision and `nav_fall_guard_system` releases it. Retrying next poll
/// (rather than inside the transition arm) is what makes that recoverable
/// instead of a permanent loss, and costs nothing when nothing is amiss:
/// `nav::api::bind_actor` is idempotent-guarded (`already_bound`) for an
/// actor that is still fine.
fn reconcile_actor_nav_bindings(world: &mut World, live: &[(Entity, PlannedLiveActor)]) {
    let mut newly_bound = Vec::new();
    let mut unbound = Vec::new();
    for (entity, actor) in live {
        // `live` was snapshotted before this frame's own transitions ran; an
        // actor this same poll just unloaded is despawned by the time this
        // runs. `World::entity_mut` (inside `bind_actor`) panics on a dead
        // entity, so a stale entry is skipped outright rather than treated
        // as merely unbound.
        if !world.entities().contains(*entity) {
            continue;
        }
        if api::is_bound(world, *entity) {
            newly_bound.push(actor.reference_form_id);
        } else {
            unbound.push((*entity, actor.reference_form_id));
        }
    }
    if !newly_bound.is_empty() {
        world
            .resource_mut::<ExteriorActorResidency>()
            .bound_references
            .extend(newly_bound);
    }
    for (entity, reference_form_id) in unbound {
        let should_retry = world
            .resource::<ExteriorActorResidency>()
            .bound_references
            .contains(&reference_form_id);
        if !should_retry {
            continue;
        }
        match api::bind_actor(world, entity) {
            Ok(()) => {
                info!("exterior actor nav rebind {reference_form_id:08x}");
            }
            Err(error) => {
                warn!(
                    "exterior actor nav rebind {reference_form_id:08x} deferred: {} ({})",
                    error.message(),
                    error.code()
                );
            }
        }
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
        planned.push(PlannedCell {
            cell_form_id: cell.state.cell_form_id,
            generation: cell.state.generation,
            grid: (grid.x, grid.y),
            projectable,
            evicting: cell.state.lifecycle == ExteriorCellLifecycle::Evicting,
            actors: cell
                .package
                .as_ref()
                .map(|package| {
                    package
                        .actors
                        .iter()
                        .filter(|actor| actor.initially_enabled && actor.reference_form_id != 0)
                        .map(|actor| PlannedActorEntry {
                            reference_form_id: actor.reference_form_id,
                            has_saved_state: saved.contains(&actor.reference_form_id),
                        })
                        .collect()
                })
                .unwrap_or_default(),
        });
        snapshots.insert(
            cell.state.cell_form_id,
            CellSnapshot {
                cell_form_id: cell.state.cell_form_id,
                grid: *grid,
                root: cell.root,
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
        if note_rejection(world, reference_form_id, "InvalidIdentity") {
            warn!("exterior actor reject {reference_form_id:08x} InvalidIdentity");
        }
        return;
    };

    let policy_request = match request {
        PlannedRequest::Duplicate { owners, .. } => {
            if note_rejection(world, reference_form_id, "DuplicateOwner") {
                warn!(
                    "exterior actor reject {reference_form_id:08x} DuplicateOwner owners={owners}"
                );
            }
            return;
        }
        PlannedRequest::Bind { destination, .. } => {
            owner_token(destination).map(|destination| ActorResidencyRequest::Bind { destination })
        }
        PlannedRequest::Restore { destination, .. } => owner_token(destination)
            .map(|destination| ActorResidencyRequest::Restore { destination }),
        PlannedRequest::Handoff {
            source,
            destination,
            ..
        } => owner_token(source)
            .zip(owner_token(destination))
            .map(|(source, destination)| ActorResidencyRequest::Handoff {
                source,
                destination,
            }),
        PlannedRequest::Unload { source, .. } => {
            owner_token(source).map(|source| ActorResidencyRequest::Unload { source })
        }
        PlannedRequest::Retain { owner, .. } => {
            owner_token(owner).map(|owner| ActorResidencyRequest::Retain { owner })
        }
    };
    let Some(policy_request) = policy_request else {
        if note_rejection(world, reference_form_id, "InvalidOwner") {
            warn!("exterior actor reject {reference_form_id:08x} InvalidOwner");
        }
        return;
    };

    match decide_actor_residency(identity, &owners, policy_request) {
        ActorResidencyDecision::Reject(rejection) => {
            let code = rejection_code(rejection);
            if note_rejection(world, reference_form_id, code) {
                warn!("exterior actor reject {reference_form_id:08x} {code}");
            }
        }
        ActorResidencyDecision::Apply(plan) => {
            apply_transition(world, identity, plan.transition(), snapshots, entity);
        }
    }
}

/// A planned owner only becomes a policy owner token if its cell FormID is
/// valid; an unidentifiable cell is refused rather than substituted, so no
/// request is ever decided against an invented owner.
fn owner_token(owner: PlannedOwner) -> Option<ActorResidencyOwner> {
    ActorResidencyOwner::new(owner.cell_form_id, owner.generation).ok()
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
            let Some(prepared) = prepared_actor_for(world, snapshot.grid, reference_form_id) else {
                return;
            };
            ensure_cell_catalogs(world, snapshot.cell_form_id);
            apply_canonical_state(world, reference_form_id, state);
            spawn_exterior_actor(world, &prepared, root, owner);
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
            if let Some(mut references) = world.get_resource_mut::<crate::console::RefRegistry>() {
                references.unregister(entity);
            }
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

/// Reads one prepared actor entry out of the live package, at bind time only.
fn prepared_actor_for(
    world: &World,
    grid: GridCoordinate,
    reference_form_id: u32,
) -> Option<PreparedExteriorActor> {
    world
        .get_resource::<ExteriorStreamState>()?
        .cells
        .get(&grid)?
        .package
        .as_ref()?
        .actors
        .iter()
        .find(|actor| actor.reference_form_id == reference_form_id)
        .cloned()
}

/// Records a rejection and reports whether it is a genuine *transition* --
/// the first rejection for this reference, or a different code than last
/// time. A live actor stuck on a persistent rejection (e.g. `StaleSource`
/// every frame until its cell catches up) produces a `PlannedRequest::Retain`
/// every frame, so without this check `rejections` would count frames, not
/// events, and the log would flood for as long as the mismatch persists
/// (issue #305 review). Only a real transition -- including the transition
/// *into* a persistent rejection -- bumps the counter and returns `true`.
fn note_rejection(world: &mut World, reference_form_id: u32, code: &'static str) -> bool {
    let mut residency = world.resource_mut::<ExteriorActorResidency>();
    let changed = residency.last_rejection.get(&reference_form_id) != Some(&code);
    if changed {
        residency.rejections = residency.rejections.saturating_add(1);
        residency.last_rejection.insert(reference_form_id, code);
    }
    changed
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
    let entity = entity.id();
    // Without this a streamed actor is invisible to every reference-addressed
    // console command (`prid`, `actorinspect`, `actorstate`, `setpos`), the
    // same registration `scene::spawn_cell_placements_chunk` performs for an
    // interior placement.
    if let Some(mut references) = world.get_resource_mut::<crate::console::RefRegistry>() {
        references.register(entity, prepared.reference_form_id, None);
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

/// Background task carrying one streamed cell's prepared actor/animation
/// catalog read (issue #305 review): `read_cell_catalogs` does a blocking
/// file read plus a full `PreparedSceneManifest` RON parse, which must not
/// stall the frame it first runs on. Mirrors the `ExteriorPackageTask`
/// pattern in `loading.rs`.
#[derive(Component)]
pub(crate) struct ExteriorCatalogTask {
    cell_form_id: u32,
    task: Task<anyhow::Result<CellCatalogs>>,
}

/// Requests a streamed cell's prepared actor/animation catalogs, once, and
/// hands them to the existing multi-cell registries when the background read
/// resolves. Both are keyed by cell FormID already; this module only decides
/// *when* an exterior cell's catalogs enter and leave them. Non-blocking: see
/// [`poll_cell_catalog_tasks`] for the completion side.
fn ensure_cell_catalogs(world: &mut World, cell_form_id: u32) {
    {
        let residency = world.resource::<ExteriorActorResidency>();
        if residency.inserted_catalogs.contains(&cell_form_id)
            || residency.failed_catalogs.contains(&cell_form_id)
            || residency.pending_catalog_tasks.contains(&cell_form_id)
        {
            return;
        }
    }
    let Some(asset_root) = world.resource::<ExteriorStreamState>().asset_root.clone() else {
        return;
    };
    let pool = AsyncComputeTaskPool::get_or_init(TaskPool::default);
    let task = pool.spawn(async move { read_cell_catalogs(&asset_root, cell_form_id) });
    world.spawn(ExteriorCatalogTask { cell_form_id, task });
    world
        .resource_mut::<ExteriorActorResidency>()
        .pending_catalog_tasks
        .insert(cell_form_id);
}

/// Polls in-flight [`ExteriorCatalogTask`]s and applies a resolved read to
/// the same registries [`ensure_cell_catalogs`] used to update synchronously.
/// Registered alongside [`sync_exterior_actor_residency`] so a completed read
/// is applied the same frame it resolves.
pub(crate) fn poll_cell_catalog_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ExteriorCatalogTask)>,
    mut residency: ResMut<ExteriorActorResidency>,
    mut definitions: ResMut<ActorDefinitionCatalogs>,
    mut animations: ResMut<crate::viewer::actor_animation::ActorAnimationCatalogs>,
) {
    for (entity, mut pending) in &mut tasks {
        let Some(result) = check_ready(&mut pending.task) else {
            continue;
        };
        commands.entity(entity).despawn();
        residency
            .pending_catalog_tasks
            .remove(&pending.cell_form_id);
        match result {
            Ok((defs, anims)) => {
                if let Some(catalog) = defs {
                    definitions.insert(pending.cell_form_id, catalog);
                }
                if let Some(catalog) = anims {
                    animations.insert(pending.cell_form_id, catalog);
                }
                residency.inserted_catalogs.insert(pending.cell_form_id);
            }
            Err(error) => {
                warn!(
                    "exterior actor catalog unavailable for cell {:08x}: {error:#}",
                    pending.cell_form_id
                );
                residency.failed_catalogs.insert(pending.cell_form_id);
            }
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
    // The resident NAVM topology the nav build gated agent retargeting on
    // (W3-B/W3-C), so a border crossing can be inspected from one command.
    let topology = world
        .get_resource::<crate::viewer::nav::world::state::NavArchipelagoState>()
        .and_then(|state| state.resident_nav_topology.archipelago.as_ref())
        .map(|archipelago| {
            serde_json::json!({
                "resident_cells": archipelago
                    .resident_cells
                    .iter()
                    .map(|key| serde_json::json!({
                        "cell": format!("{:08x}", key.cell_form_id),
                        "generation": key.generation,
                    }))
                    .collect::<Vec<_>>(),
                "merge_links": archipelago.merge_links.len(),
            })
        });
    let residency = world.get_resource::<ExteriorActorResidency>();
    serde_json::json!({
        "actors": rows,
        "nav_topology": topology,
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

/// Clears a reference's saved package checkpoint once the autonomous driver
/// has applied it (issue #305 review): [`capture_package_checkpoint`] is the
/// only writer, so a resumed checkpoint that is never cleared is found again
/// by [`saved_package_checkpoint`] on every later start of the same package
/// in the same session, rewinding it a second time.
pub(crate) fn clear_saved_package_checkpoint(world: &mut World, reference_form_id: u32) {
    let Some(mut save) = world.get_resource_mut::<ActiveSaveState>() else {
        return;
    };
    if let Some(state) = save
        .0
        .cells
        .values_mut()
        .find_map(|cell| cell.actors.get_mut(&reference_form_id))
    {
        state.package = None;
    }
}

#[cfg(test)]
#[path = "actors_tests.rs"]
mod tests;
