//! Bevy-side save-layer glue for issues #60 (apply persistent state on every
//! load path) and #61 (capture state on the way out). The decisions live in
//! the pure `persist_policy` module; this file only reads/writes the ECS,
//! `PreparedCollisionWorld`, and the `save` slice's types.
//!
//! Capture (F61.1) runs when a cell is swapped away from
//! (`swap::activate_resident_cell`) and when the preloader evicts a resident
//! cell (`preload::evaluate_preload_plan` pushes into
//! `PendingEvictionCaptures`; `drain_eviction_captures` snapshots the cell
//! and only then despawns its root). Apply (F60.2) runs on both swap
//! activation paths (same call site in `activate_resident_cell`) and at
//! viewer startup for the launch cell (`apply_save_state_at_startup`,
//! chained between `spawn_prepared_scene` and `build_prepared_colliders`).
//!
//! Restored dynamic bodies flow through the regular collider build: apply
//! stashes each saved pose/velocity in `PersistRestores`, and
//! `player::collision::build_colliders_for_placement` re-applies it the
//! moment `create_dynamic_body` makes the body (or immediately here, if the
//! cell's body survived as a live resident). Deleted references are
//! despawned and their FormIDs suppressed so no collider is ever rebuilt
//! for them.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_boxddd::prelude::BoxdddPhysicsContext;

use crate::save::{
    DroppedItemState, EquippedItem, EquippedKind, HotkeyBinding, ItemStack,
    PersistentReferenceDelta, PlayerState, SaveGame, SaveGameHeader, SavePlugin, SaveStore,
    SavedBodyState, SavedTransform,
};
#[cfg(test)]
use crate::vsa::PreparedInventoryEntry;
#[cfg(test)]
use crate::vsa::PreparedSemantic;
use crate::vsa::{PreparedPlacement, PreparedSceneManifest, is_bake_static};

use super::super::{actor, animation, interaction, player, weapon};
use super::persist_policy;
use super::preload::{ActiveCell, ResidentCells};

/// F52.3's live-save seam, moved here from `swap.rs` (issue #60): `src/save`
/// is deliberately Bevy-free, so this thin wrapper is the viewer's own
/// resource type. `run_view` inserts a loaded slot's world state at startup
/// (`--save-slot`); capture keeps it current; the console `save` command
/// writes it back to disk.
#[derive(Resource, Default)]
pub(crate) struct ActiveSaveState(pub(crate) crate::save::PersistentWorldState);

/// Where console `save <slot>` writes slots. Defaults to the project-local
/// `.bevyout/saves`; tests override it with a temporary directory.
#[derive(Resource, Clone, Debug)]
pub(crate) struct SaveDirectory(pub(crate) PathBuf);

impl Default for SaveDirectory {
    fn default() -> Self {
        Self(PathBuf::from(".bevyout/saves"))
    }
}

/// One saved dynamic-body state waiting to be re-applied when the
/// placement's body is built (see `collision::build_colliders_for_placement`).
#[derive(Clone, Copy, Debug)]
pub(crate) struct DynamicBodyRestore {
    pub(crate) translation: Vec3,
    pub(crate) rotation: Quat,
    pub(crate) linear_velocity: Vec3,
    pub(crate) angular_velocity: Vec3,
    pub(crate) sleeping: bool,
}

/// Cross-module seam consumed by the collider build (F60.2): `bodies` are
/// consume-once pose/velocity restores keyed by reference FormID;
/// `suppressed` FormIDs (deleted references) never get colliders built.
#[derive(Resource, Default)]
pub(crate) struct PersistRestores {
    pub(crate) bodies: HashMap<u32, DynamicBodyRestore>,
    pub(crate) suppressed: HashSet<u32>,
}

/// A cell evicted by `preload::evaluate_preload_plan`, staged for capture
/// before its root is despawned. The manifest rides along because the
/// resident entry has already been removed by the time capture runs.
pub(crate) struct EvictionCapture {
    pub(crate) form_id: u32,
    pub(crate) root: Entity,
    pub(crate) manifest: Arc<PreparedSceneManifest>,
}

#[derive(Resource, Default)]
pub(crate) struct PendingEvictionCaptures(pub(crate) Vec<EvictionCapture>);

/// Playthrough-stable RNG seed for deterministic leveled-list resolution
/// (#74): saved as `SaveGame.rng_state`, restored on `--save-slot` load. A
/// fresh playthrough currently always starts at 0; a new-game flow that
/// randomizes it slots in here without touching the resolver.
#[derive(Resource, Clone, Copy, Default)]
pub(crate) struct PlaythroughSeed(pub(crate) u64);

pub(crate) fn install(app: &mut App) {
    app.init_resource::<PlaythroughSeed>()
        .init_resource::<ActiveSaveState>()
        .init_resource::<SaveDirectory>()
        .init_resource::<PersistRestores>()
        .init_resource::<PendingEvictionCaptures>()
        .init_resource::<interaction::ContainerStates>()
        .add_systems(Update, drain_eviction_captures);
}

/// F61.1 (eviction half): captures every staged evicted cell while its
/// entities still exist, then despawns the cell root exactly like the
/// eviction loop used to.
pub(crate) fn drain_eviction_captures(world: &mut World) {
    let pending = std::mem::take(&mut world.resource_mut::<PendingEvictionCaptures>().0);
    for capture in pending {
        capture_cell_placements(
            world,
            capture.form_id,
            capture.root,
            &capture.manifest.placements,
            capture.manifest.bake.is_some(),
        );
        // Issue #63: an evicted cell's colliders go with it (capture above
        // already snapshotted the dynamic poses).
        player::teardown_cell_colliders(world, capture.form_id);
        if let Ok(root) = world.get_entity_mut(capture.root) {
            root.despawn();
        }
    }
}

/// F61.1 (swap-away half): captures `cell` if it is still resident. Called
/// from `swap::activate_resident_cell` before the source root is hidden.
pub(crate) fn capture_cell_state(world: &mut World, cell: u32) {
    let Some((root, manifest)) = world
        .resource::<ResidentCells>()
        .0
        .get(&cell)
        .map(|resident| (resident.root, Arc::clone(&resident.manifest)))
    else {
        return;
    };
    capture_cell_placements(
        world,
        cell,
        root,
        &manifest.placements,
        manifest.bake.is_some(),
    );
}

/// F60.2: applies the active save state to `cell`'s spawned placements.
/// Called from both swap activation paths and viewer startup.
pub(crate) fn apply_cell_state(world: &mut World, cell: u32, root: Entity) {
    let Some(manifest) = world
        .resource::<ResidentCells>()
        .0
        .get(&cell)
        .map(|resident| Arc::clone(&resident.manifest))
    else {
        return;
    };
    apply_cell_placements(world, cell, root, &manifest.placements);
    super::super::world_items::restore_dropped_items(world, cell, root);
}

/// Applies the saved transform/deletion projection to an exterior package's
/// dynamic reference roots after their package commands have materialized.
/// Exterior rendering remains presentation-only; canonical deltas stay in the
/// same `ActiveSaveState` map used by interior cells.
pub(crate) fn apply_exterior_cell_state(
    world: &mut World,
    cell: u32,
    root: Entity,
    package: &bevyout_core::manifest::exterior::ExteriorCellPackage,
) {
    let deltas = world
        .get_resource::<ActiveSaveState>()
        .and_then(|state| state.0.cells.get(&cell))
        .map(|cell| cell.references.clone())
        .unwrap_or_default();
    if deltas.is_empty() {
        return;
    }
    let entities = {
        let mut query = world.query::<(Entity, &super::exterior::ExteriorReference, &ChildOf)>();
        query
            .iter(world)
            .filter(|(_, _, parent)| parent.parent() == root)
            .map(|(entity, reference, _)| (entity, reference.reference_form_id))
            .collect::<Vec<_>>()
    };
    for (entity, reference_form_id) in entities {
        let Some(delta) = deltas.get(&reference_form_id) else {
            continue;
        };
        if delta.deleted {
            if let Ok(mut entity) = world.get_entity_mut(entity) {
                entity.insert(Visibility::Hidden);
            }
            continue;
        }
        if let Some(transform) = delta.transform {
            world.entity_mut(entity).insert(Transform {
                translation: Vec3::from_array(transform.translation),
                rotation: Quat::from_xyzw(
                    transform.rotation_xyzw[0],
                    transform.rotation_xyzw[1],
                    transform.rotation_xyzw[2],
                    transform.rotation_xyzw[3],
                ),
                scale: Vec3::from_array(transform.scale),
            });
        }
    }
    let dynamic_ids = package
        .dynamic_objects
        .iter()
        .map(|object| object.reference_form_id)
        .collect::<HashSet<_>>();
    if deltas.keys().any(|form_id| dynamic_ids.contains(form_id)) {
        info!("save apply exterior {cell:08x} dynamic references");
    }
}

/// Captures all resident exterior dynamic roots before an explicit save.
/// Eviction uses the single-cell variant below so unload/reload and save use
/// the same baseline comparison.
pub(crate) fn capture_exterior_resident_state(world: &mut World) {
    let residents = world
        .get_resource::<super::exterior::ExteriorStreamState>()
        .map(|stream| {
            stream
                .cells
                .values()
                .filter_map(|cell| {
                    Some((cell.state.cell_form_id, cell.root?, cell.package.clone()?))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    for (cell, root, package) in residents {
        capture_exterior_cell_state(world, cell, root, &package);
    }
}

/// Captures mutable transforms for an exterior package's dynamic references
/// while the package root is still alive. Static presentation is never added
/// to save state, and a missing asset root is not treated as a deletion.
pub(crate) fn capture_exterior_cell_state(
    world: &mut World,
    cell: u32,
    root: Entity,
    package: &bevyout_core::manifest::exterior::ExteriorCellPackage,
) {
    let live = {
        let mut query =
            world.query::<(&super::exterior::ExteriorReference, &ChildOf, &Transform)>();
        query
            .iter(world)
            .filter(|(_, parent, _)| parent.parent() == root)
            .map(|(reference, _, transform)| (reference.reference_form_id, *transform))
            .collect::<HashMap<_, _>>()
    };
    let mut state = world.get_resource_or_insert_with(ActiveSaveState::default);
    let cell_state = state.0.cells.entry(cell).or_default();
    for object in &package.dynamic_objects {
        if !object.initially_enabled || object.asset_path.is_none() {
            continue;
        }
        let Some(transform) = live.get(&object.reference_form_id) else {
            continue;
        };
        let baseline = SavedTransform {
            translation: object.position,
            rotation_xyzw: object.rotation_xyzw,
            scale: [object.scale; 3],
        };
        let current = SavedTransform {
            translation: transform.translation.to_array(),
            rotation_xyzw: transform.rotation.to_array(),
            scale: transform.scale.to_array(),
        };
        let changed = baseline
            .translation
            .into_iter()
            .zip(current.translation)
            .chain(
                baseline
                    .rotation_xyzw
                    .into_iter()
                    .zip(current.rotation_xyzw),
            )
            .chain(baseline.scale.into_iter().zip(current.scale))
            .any(|(left, right)| (left - right).abs() > 0.0001);
        let delta = cell_state
            .references
            .entry(object.reference_form_id)
            .or_default();
        delta.transform = changed.then_some(current);
        if *delta == PersistentReferenceDelta::default() {
            cell_state.references.remove(&object.reference_form_id);
        }
    }
    if cell_state.references.is_empty()
        && cell_state.dropped_items.is_empty()
        && cell_state.actors.is_empty()
    {
        state.0.cells.remove(&cell);
    }
}

/// Startup wiring for F60.2/F60.3: applies the (possibly `--save-slot`
/// loaded) save state to the launch cell, chained after
/// `spawn_prepared_scene` and before `build_prepared_colliders` so deleted
/// references are suppressed and dynamic restores are staged before any
/// collider exists.
pub(crate) fn apply_save_state_at_startup(world: &mut World) {
    let Some(active) = world.get_resource::<ActiveCell>().map(|cell| cell.0) else {
        return;
    };
    let Some(root) = world
        .resource::<ResidentCells>()
        .0
        .get(&active)
        .map(|resident| resident.root)
    else {
        return;
    };
    apply_cell_state(world, active, root);
}

/// Mirrors `scene::spawn_cell_placements_chunk`'s spawn filter: only these
/// placements ever get an entity, so only these can be captured as taken
/// (`present == false`) rather than merely never-spawned.
fn placement_is_spawnable(placement: &PreparedPlacement, exclude_bake_static: bool) -> bool {
    placement.initially_enabled
        && (placement.asset_path.is_some() || actor::is_actor_semantic(&placement.semantic))
        && (!exclude_bake_static || !is_bake_static(placement))
}

fn spawned_placements(
    world: &mut World,
    root: Entity,
) -> HashMap<u32, (Entity, persist_policy::TransformDelta)> {
    let mut query = world.query::<(Entity, &interaction::PlacementRoot, &ChildOf, &Transform)>();
    query
        .iter(world)
        .filter(|(_, _, child_of, _)| child_of.parent() == root)
        .map(|(entity, placement_root, _, transform)| {
            (
                placement_root.placement().reference_form_id,
                (
                    entity,
                    persist_policy::TransformDelta {
                        translation: transform.translation.to_array(),
                        rotation_xyzw: transform.rotation.to_array(),
                        scale: transform.scale.to_array(),
                    },
                ),
            )
        })
        .collect()
}

fn capture_cell_placements(
    world: &mut World,
    cell: u32,
    root: Entity,
    placements: &[PreparedPlacement],
    exclude_bake_static: bool,
) {
    let spawned = spawned_placements(world, root);
    let open: HashSet<Entity> = world
        .get_resource::<interaction::InteractionState>()
        .map(|state| state.open.clone())
        .unwrap_or_default();

    let mut baselines = Vec::new();
    let mut snapshots = Vec::new();
    for placement in placements {
        if !placement_is_spawnable(placement, exclude_bake_static) {
            continue;
        }
        baselines.push(persist_policy::BaselinePlacement {
            reference_form_id: placement.reference_form_id,
            transform: persist_policy::TransformDelta {
                translation: placement.translation,
                rotation_xyzw: placement.rotation_xyzw,
                scale: [placement.scale, placement.scale, placement.scale],
            },
        });
        let snapshot = match spawned.get(&placement.reference_form_id) {
            Some((entity, transform)) => persist_policy::RuntimeSnapshot {
                reference_form_id: placement.reference_form_id,
                present: true,
                transform: Some(*transform),
                activated: open.contains(entity).then_some(true),
                body: dynamic_body_snapshot(world, *entity),
            },
            None => persist_policy::RuntimeSnapshot {
                reference_form_id: placement.reference_form_id,
                present: false,
                transform: None,
                activated: None,
                body: None,
            },
        };
        snapshots.push(snapshot);
    }

    let captured = persist_policy::diff_capture(&baselines, &snapshots);
    let observed: Vec<u32> = snapshots
        .iter()
        .map(|snapshot| snapshot.reference_form_id)
        .collect();
    let delta_count = captured.len();
    merge_captured_deltas(world, cell, &observed, &captured);

    // Issue #76 (F76.2): container stacks/resolved-marker capture, mirroring
    // the placement capture above but scoped to `PreparedSemantic::Container`
    // references observed in `interaction::ContainerStates`.
    let container_baselines = container_baselines(placements);
    let container_snapshots = container_snapshots(world, placements);
    let captured_containers =
        persist_policy::diff_capture_containers(&container_baselines, &container_snapshots);
    let container_observed: Vec<u32> = container_snapshots.keys().copied().collect();
    let container_delta_count = captured_containers.len();
    merge_captured_container_deltas(world, cell, &container_observed, &captured_containers);

    capture_runtime_items(world, cell, root);
    info!("save capture {cell:08x} deltas={delta_count} container_deltas={container_delta_count}");
}

/// F76.2: every container placement's manifest baseline -- its fixed,
/// non-leveled inventory entries as normalized stacks. Built for every
/// container regardless of whether it was ever opened, so
/// `diff_capture_containers` has a baseline to diff against the moment one
/// is.
fn container_baselines(
    placements: &[PreparedPlacement],
) -> HashMap<u32, persist_policy::ContainerBaseline> {
    placements
        .iter()
        .filter_map(|placement| {
            container_baseline(placement).map(|baseline| (placement.reference_form_id, baseline))
        })
        .collect()
}

fn container_baseline(placement: &PreparedPlacement) -> Option<persist_policy::ContainerBaseline> {
    if !placement.semantic.is_loot_holder() {
        return None;
    }
    let stacks: Vec<(u32, i32)> = placement
        .inventory
        .iter()
        .filter(|entry| !entry.leveled)
        .map(|entry| (entry.base_form_id, entry.count))
        .collect();
    Some(persist_policy::ContainerBaseline {
        stacks: persist_policy::normalize_stacks(&stacks),
    })
}

/// F76.2: every container reference's live state, read from
/// `interaction::ContainerStates` (issue #75's runtime resource, wired at
/// wave-2 integration). A container absent from the resource was never
/// opened this session and is excluded, so capture never clobbers a
/// previously loaded delta it cannot currently observe.
fn container_snapshots(
    world: &World,
    placements: &[PreparedPlacement],
) -> HashMap<u32, persist_policy::ContainerSnapshot> {
    let Some(states) = world.get_resource::<interaction::ContainerStates>() else {
        return HashMap::new();
    };
    placements
        .iter()
        .filter(|placement| placement.semantic.is_loot_holder())
        .filter_map(|placement| {
            states.0.get(&placement.reference_form_id).map(|state| {
                (
                    placement.reference_form_id,
                    persist_policy::ContainerSnapshot {
                        stacks: state.stacks.clone(),
                        resolved: state.resolved,
                    },
                )
            })
        })
        .collect()
}

fn capture_runtime_items(world: &mut World, cell: u32, root: Entity) {
    let snapshots = {
        let mut query = world.query::<(
            Entity,
            &super::super::world_items::RuntimeWorldItem,
            &ChildOf,
            &Transform,
        )>();
        query
            .iter(world)
            .filter(|(_, item, parent, _)| item.cell_form_id == cell && parent.parent() == root)
            .map(|(entity, item, _, transform)| {
                (
                    entity,
                    *item,
                    transform.translation,
                    transform.rotation,
                    transform.scale,
                )
            })
            .collect::<Vec<_>>()
    };
    let mut dropped_items = BTreeMap::new();
    for (entity, item, translation, rotation, scale) in snapshots {
        let body = dynamic_body_snapshot(world, entity)
            .map(to_saved_body)
            .unwrap_or(SavedBodyState {
                linear_velocity: [0.0; 3],
                angular_velocity: [0.0; 3],
                sleeping: true,
            });
        dropped_items.insert(
            item.runtime_id,
            DroppedItemState {
                runtime_id: item.runtime_id,
                stack: ItemStack {
                    base_form_id: item.stack.base_form_id,
                    count: item.stack.count,
                    condition: item.stack.condition,
                },
                transform: SavedTransform {
                    translation: translation.to_array(),
                    rotation_xyzw: rotation.to_array(),
                    scale: scale.to_array(),
                },
                body,
            },
        );
    }
    let mut state = world.get_resource_or_insert_with(ActiveSaveState::default);
    let cell_state = state.0.cells.entry(cell).or_default();
    cell_state.dropped_items = dropped_items;
    if cell_state.references.is_empty()
        && cell_state.dropped_items.is_empty()
        && cell_state.actors.is_empty()
    {
        state.0.cells.remove(&cell);
    }
}

/// Reads one placement entity's live dynamic-body velocities, if it has a
/// body. Missing physics context (tests, `--disable-physics`) reads as no
/// body state.
fn dynamic_body_snapshot(world: &World, entity: Entity) -> Option<persist_policy::BodyDelta> {
    let body = world
        .get_resource::<player::PreparedCollisionWorld>()?
        .dynamic_body_of(entity)?;
    let physics = world.get_non_send::<BoxdddPhysicsContext>()?;
    let physics = physics.world()?;
    let linear = physics.try_body_linear_velocity(body).ok()?;
    let angular = physics.try_body_angular_velocity(body).ok()?;
    let sleeping = !physics.try_body_awake(body).unwrap_or(true);
    Some(persist_policy::BodyDelta {
        linear_velocity: [linear.x, linear.y, linear.z],
        angular_velocity: [angular.x, angular.y, angular.z],
        sleeping,
    })
}

/// Folds a fresh capture into `ActiveSaveState`, overwriting only the
/// capture-observable fields (deleted/activated/transform/body) of each
/// observed reference and preserving fields capture cannot see (enabled
/// overrides, enable roots, locks) from any previously loaded save delta.
/// All-default deltas are dropped. Container `inventory`/`leveled_resolved`
/// are folded in separately by `merge_captured_container_deltas` below
/// (issue #76) since they are observed through a different resource scoped
/// to a different reference subset (containers only).
fn merge_captured_deltas(
    world: &mut World,
    cell: u32,
    observed: &[u32],
    captured: &HashMap<u32, persist_policy::ReferenceDelta>,
) {
    let mut state = world.get_resource_or_insert_with(ActiveSaveState::default);
    let cell_state = state.0.cells.entry(cell).or_default();
    for &form_id in observed {
        let new = captured.get(&form_id);
        let existing = cell_state.references.entry(form_id).or_default();
        existing.deleted = new.is_some_and(|delta| delta.deleted);
        existing.activated = new.and_then(|delta| delta.activated);
        existing.transform = new
            .and_then(|delta| delta.transform)
            .map(to_saved_transform);
        existing.body = new.and_then(|delta| delta.body).map(to_saved_body);
        if *existing == PersistentReferenceDelta::default() {
            cell_state.references.remove(&form_id);
        }
    }
    if cell_state.references.is_empty()
        && cell_state.dropped_items.is_empty()
        && cell_state.actors.is_empty()
    {
        state.0.cells.remove(&cell);
    }
}

/// F76.2's counterpart to `merge_captured_deltas`, scoped to
/// `inventory`/`leveled_resolved`: overwrites those two fields for every
/// observed container reference (clearing them when the fresh capture has no
/// delta for it, e.g. it reverted exactly to baseline) and leaves every
/// other reference's delta untouched. All-default deltas are dropped.
fn merge_captured_container_deltas(
    world: &mut World,
    cell: u32,
    observed: &[u32],
    captured: &HashMap<u32, persist_policy::ContainerDelta>,
) {
    if observed.is_empty() {
        return;
    }
    let mut state = world.get_resource_or_insert_with(ActiveSaveState::default);
    let cell_state = state.0.cells.entry(cell).or_default();
    for &form_id in observed {
        let new = captured.get(&form_id);
        let existing = cell_state.references.entry(form_id).or_default();
        existing.inventory = new
            .and_then(|delta| delta.inventory.clone())
            .map(|stacks| to_item_stacks(&stacks));
        existing.leveled_resolved = new.and_then(|delta| delta.leveled_resolved);
        if *existing == PersistentReferenceDelta::default() {
            cell_state.references.remove(&form_id);
        }
    }
    if cell_state.references.is_empty()
        && cell_state.dropped_items.is_empty()
        && cell_state.actors.is_empty()
    {
        state.0.cells.remove(&cell);
    }
}

/// Container-delta stacks always carry `condition: None`: `ContainerState`
/// (issue #75) is condition-less (`Vec<(u32, i32)>`), so a stack captured
/// from it has no condition to persist. Condition-aware container stacks
/// are a follow-up (see `interaction::transfer_ui::take`/`store`'s doc
/// comments for the same ceiling on the runtime side).
fn to_item_stacks(stacks: &[(u32, i32)]) -> Vec<ItemStack> {
    stacks
        .iter()
        .map(|&(base_form_id, count)| ItemStack {
            base_form_id,
            count,
            condition: None,
        })
        .collect()
}

fn to_container_delta(delta: &PersistentReferenceDelta) -> Option<persist_policy::ContainerDelta> {
    if delta.inventory.is_none() && delta.leveled_resolved.is_none() {
        return None;
    }
    Some(persist_policy::ContainerDelta {
        inventory: delta.inventory.as_ref().map(|stacks| {
            stacks
                .iter()
                .map(|item| (item.base_form_id, item.count))
                .collect()
        }),
        leveled_resolved: delta.leveled_resolved,
    })
}

fn apply_cell_placements(
    world: &mut World,
    cell: u32,
    root: Entity,
    placements: &[PreparedPlacement],
) {
    let deltas: HashMap<u32, persist_policy::ReferenceDelta> = world
        .get_resource::<ActiveSaveState>()
        .and_then(|state| state.0.cells.get(&cell))
        .map(|cell_state| {
            cell_state
                .references
                .iter()
                .map(|(form_id, delta)| (*form_id, to_policy_delta(delta)))
                .collect()
        })
        .unwrap_or_default();

    let infos: Vec<persist_policy::PlacementInfo> = placements
        .iter()
        .map(|placement| persist_policy::PlacementInfo {
            reference_form_id: placement.reference_form_id,
            initially_enabled: placement.initially_enabled,
            enable_parent: placement.enable_parent.as_ref().map(|parent| {
                persist_policy::EnableParentLink {
                    reference_form_id: parent.reference_form_id,
                    inverted: parent.inverted,
                }
            }),
        })
        .collect();
    let applications = persist_policy::plan_apply(&infos, &deltas);
    let spawned = spawned_placements(world, root);
    let delta_count = deltas.len();

    for application in applications {
        let form_id = application.reference_form_id;
        let delta = deltas.get(&form_id);
        let deleted = delta.is_some_and(|delta| delta.deleted);
        // Suppression must be recorded even when the entity is already gone,
        // so the collider build never resurrects a deleted reference.
        let mut restores = world.resource_mut::<PersistRestores>();
        if deleted {
            restores.suppressed.insert(form_id);
        } else {
            restores.suppressed.remove(&form_id);
        }
        let Some((entity, _)) = spawned.get(&form_id).copied() else {
            continue;
        };
        if deleted {
            if let Ok(entity_mut) = world.get_entity_mut(entity) {
                entity_mut.despawn();
            }
            continue;
        }
        let visibility = match application.visibility {
            persist_policy::VisibilityDecision::Visible => Visibility::Inherited,
            persist_policy::VisibilityDecision::Hidden => Visibility::Hidden,
        };
        world.entity_mut(entity).insert(visibility);
        let mut restored_pose = None;
        if let Some(transform) = application.transform {
            let pose = (
                Vec3::from_array(transform.translation),
                Quat::from_xyzw(
                    transform.rotation_xyzw[0],
                    transform.rotation_xyzw[1],
                    transform.rotation_xyzw[2],
                    transform.rotation_xyzw[3],
                ),
            );
            world.entity_mut(entity).insert(Transform {
                translation: pose.0,
                rotation: pose.1,
                scale: Vec3::from_array(transform.scale),
            });
            restored_pose = Some(pose);
        }
        if application.activated == Some(true) {
            let newly_opened = world
                .get_resource_or_insert_with(interaction::InteractionState::default)
                .open
                .insert(entity);
            if newly_opened {
                world.write_message(animation::PlayPlacementAnimation {
                    root: entity,
                    transition: animation::ClipTransition::Opening,
                    lead_ms: 0.0,
                });
            }
        }
        if let Some(pose) = restored_pose {
            let body = application.body;
            let restore = DynamicBodyRestore {
                translation: pose.0,
                rotation: pose.1,
                linear_velocity: body
                    .map(|body| Vec3::from_array(body.linear_velocity))
                    .unwrap_or(Vec3::ZERO),
                angular_velocity: body
                    .map(|body| Vec3::from_array(body.angular_velocity))
                    .unwrap_or(Vec3::ZERO),
                sleeping: body.is_none_or(|body| body.sleeping),
            };
            if !apply_body_restore_now(world, entity, &restore) {
                world
                    .resource_mut::<PersistRestores>()
                    .bodies
                    .insert(form_id, restore);
            }
        }
    }

    // Issue #76 (F76.3): rebuild every delta-carrying container's runtime
    // state before first activation. A container with no delta is left
    // absent from `interaction::ContainerStates`, so it still rolls on first open.
    let container_baselines = container_baselines(placements);
    let container_deltas: HashMap<u32, persist_policy::ContainerDelta> = world
        .get_resource::<ActiveSaveState>()
        .and_then(|state| state.0.cells.get(&cell))
        .map(|cell_state| {
            cell_state
                .references
                .iter()
                .filter_map(|(form_id, delta)| {
                    to_container_delta(delta).map(|container_delta| (*form_id, container_delta))
                })
                .collect()
        })
        .unwrap_or_default();
    let container_seed =
        persist_policy::plan_apply_containers(&container_baselines, &container_deltas);
    let container_seed_count = container_seed.len();
    if !container_seed.is_empty() {
        let mut states = world.resource_mut::<interaction::ContainerStates>();
        for (form_id, snapshot) in container_seed {
            states.0.insert(
                form_id,
                interaction::container_policy::ContainerState {
                    stacks: snapshot.stacks,
                    resolved: snapshot.resolved,
                },
            );
        }
    }

    info!("save apply {cell:08x} deltas={delta_count} container_seeded={container_seed_count}");
}

/// Applies a restore to an already-live dynamic body (a cell that stayed
/// resident since it was captured). Returns `false` when the entity has no
/// body yet -- the caller stashes the restore for the collider build instead.
fn apply_body_restore_now(world: &mut World, entity: Entity, restore: &DynamicBodyRestore) -> bool {
    let Some(body) = world
        .get_resource::<player::PreparedCollisionWorld>()
        .and_then(|collision| collision.dynamic_body_of(entity))
    else {
        return false;
    };
    let Some(mut context) = world.get_non_send_mut::<BoxdddPhysicsContext>() else {
        return false;
    };
    let Some(physics) = context.world_mut() else {
        return false;
    };
    player::apply_dynamic_body_restore(physics, body, restore);
    true
}

fn to_saved_transform(transform: persist_policy::TransformDelta) -> SavedTransform {
    SavedTransform {
        translation: transform.translation,
        rotation_xyzw: transform.rotation_xyzw,
        scale: transform.scale,
    }
}

fn to_saved_body(body: persist_policy::BodyDelta) -> SavedBodyState {
    SavedBodyState {
        linear_velocity: body.linear_velocity,
        angular_velocity: body.angular_velocity,
        sleeping: body.sleeping,
    }
}

fn to_policy_delta(delta: &PersistentReferenceDelta) -> persist_policy::ReferenceDelta {
    persist_policy::ReferenceDelta {
        enabled: delta.enabled,
        deleted: delta.deleted,
        activated: delta.activated,
        enable_root_form_id: delta.enable_root_form_id,
        transform: delta
            .transform
            .map(|transform| persist_policy::TransformDelta {
                translation: transform.translation,
                rotation_xyzw: transform.rotation_xyzw,
                scale: transform.scale,
            }),
        body: delta.body.map(|body| persist_policy::BodyDelta {
            linear_velocity: body.linear_velocity,
            angular_velocity: body.angular_velocity,
            sleeping: body.sleeping,
        }),
    }
}

/// F60.3: captures the active cell, assembles a `SaveGame` from the loaded
/// content identity (`PreparedSceneManifest`'s source fingerprint and
/// plugins -- the same identity `run_view` checks with `ensure_compatible`
/// on `--save-slot` load), the live `ActiveSaveState`, and the player
/// inventory, and writes it to `slot`. Returns the written primary path.
pub(crate) fn write_save_slot(world: &mut World, slot: &str) -> anyhow::Result<PathBuf> {
    let Some(active) = world.get_resource::<ActiveCell>().map(|cell| cell.0) else {
        anyhow::bail!("no active cell to save");
    };
    capture_cell_state(world, active);
    capture_exterior_resident_state(world);

    let legacy_inventory = world
        .get_resource::<interaction::PlayerInventory>()
        .map(|inventory| inventory.legacy_snapshot());
    let canonical_player_present = world
        .get_resource::<interaction::CanonicalItemLedger>()
        .is_some_and(|ledger| {
            ledger
                .ledger
                .holders()
                .contains_key(&crate::item_transaction::HolderId::Player)
        });
    if !canonical_player_present {
        let legacy_inventory = legacy_inventory.unwrap_or_default();
        world.get_resource_or_insert_with(interaction::CanonicalItemLedger::default);
        let mut ledger = world.resource_mut::<interaction::CanonicalItemLedger>();
        if !ledger
            .ledger
            .holders()
            .contains_key(&crate::item_transaction::HolderId::Player)
        {
            ledger
                .sync_player(&legacy_inventory)
                .map_err(|error| anyhow::anyhow!("canonical item bootstrap failed: {error}"))?;
        }
    }

    let Some(manifest) = world.get_resource::<crate::viewer::LoadedSceneManifest>() else {
        anyhow::bail!("no prepared scene manifest loaded");
    };
    let save_cell = world
        .get_resource::<super::exterior::ExteriorStreamState>()
        .and_then(|state| state.cells.get(&state.current_grid))
        .map(|cell| cell.state.cell_form_id)
        .unwrap_or(active);
    let header = SaveGameHeader {
        content_fingerprint: manifest.source_fingerprint.clone(),
        plugins: manifest
            .source_plugins
            .iter()
            .map(|plugin| SavePlugin {
                name: plugin.name.clone(),
                fingerprint: plugin.fingerprint.clone(),
            })
            .collect(),
        current_cell: save_cell,
        play_time_seconds: world
            .get_resource::<Time>()
            .map(|time| time.elapsed_secs_f64())
            .unwrap_or(0.0),
        description: format!("cell {active:08x}"),
        ..Default::default()
    };
    let canonical_inventory = world
        .get_resource::<interaction::CanonicalItemLedger>()
        .and_then(interaction::CanonicalItemLedger::player_legacy_snapshot)
        .map(|inventory| {
            inventory
                .stacks()
                .into_iter()
                .map(|stack| ItemStack {
                    base_form_id: stack.base_form_id,
                    count: stack.count,
                    condition: stack.condition,
                })
                .collect()
        })
        .unwrap_or_default();
    let player = PlayerState {
        inventory: canonical_inventory,
        equipped: capture_equipped(world),
        hotkeys: capture_hotkeys(world),
    };
    let dialogue = if let Some(mut runtime) =
        world.get_resource_mut::<crate::viewer::dialogue::DialogueRuntime>()
    {
        if runtime.is_active() {
            anyhow::bail!(
                "dialogue save deferred: an active dialogue must reach a boundary before saving"
            );
        }
        runtime.boundary_snapshot()
    } else {
        Default::default()
    };
    let save = SaveGame {
        header,
        world: world
            .get_resource::<ActiveSaveState>()
            .map(|state| state.0.clone())
            .unwrap_or_default(),
        player: Some(player),
        next_runtime_item_id: world
            .get_resource::<super::super::world_items::NextRuntimeItemId>()
            .map_or(1, |next| next.0),
        rng_state: world
            .get_resource::<PlaythroughSeed>()
            .map(|seed| seed.0)
            .unwrap_or_default(),
        combat_rng: world
            .get_resource::<weapon::CombatRngRuntime>()
            .map(|rng| rng.0.clone())
            .unwrap_or_default(),
        canonical: world
            .get_resource::<interaction::CanonicalItemLedger>()
            .map(interaction::CanonicalItemLedger::snapshot),
        dialogue,
        location: world
            .get_resource::<super::CurrentWorldLocation>()
            .and_then(|location| location.0.clone()),
    };
    let save_dir = world
        .get_resource::<SaveDirectory>()
        .cloned()
        .unwrap_or_default()
        .0;
    let store = SaveStore::from_save_dir(save_dir);
    store.write_slot(slot, &save)?;
    let path = store.primary_path(slot);
    info!("save write {slot} path={}", path.display());
    Ok(path)
}

/// Issue #98 (F98.4): flattens `PlayerEquipment` into the sorted save shape
/// `validate_equipped` requires. Apparel is deduplicated by `StackKey` since
/// one equipped item can occupy several biped slots at once (see
/// `player::equipment::EquipmentState::equip_apparel`).
fn capture_equipped(world: &World) -> Vec<EquippedItem> {
    let Some(equipment) = world.get_resource::<interaction::PlayerEquipment>() else {
        return Vec::new();
    };
    let mut equipped = Vec::new();
    let mut seen_apparel = HashSet::new();
    for (_, key) in equipment.equipped_apparel() {
        if seen_apparel.insert(key) {
            equipped.push(EquippedItem {
                kind: EquippedKind::Apparel,
                base_form_id: key.base_form_id,
                condition: key.condition,
            });
        }
    }
    if let Some(key) = equipment.equipped_weapon() {
        equipped.push(EquippedItem {
            kind: EquippedKind::Weapon,
            base_form_id: key.base_form_id,
            condition: key.condition,
        });
    }
    if let Some(key) = equipment.equipped_ammo() {
        equipped.push(EquippedItem {
            kind: EquippedKind::Ammo,
            base_form_id: key.base_form_id,
            condition: key.condition,
        });
    }
    equipped.sort_by_key(|item| (item.kind, item.base_form_id, item.condition));
    equipped
}

/// Issue #98 (F98.4): flattens the `HotkeyBindings` resource into the fixed
/// 8-slot save shape.
fn capture_hotkeys(world: &World) -> [Option<HotkeyBinding>; 8] {
    let Some(bindings) = world.get_resource::<super::super::bindings::HotkeyBindings>() else {
        return Default::default();
    };
    let mut hotkeys: [Option<HotkeyBinding>; 8] = Default::default();
    for (index, slot) in hotkeys.iter_mut().enumerate() {
        *slot = bindings.get((index + 1) as u8).map(|key| HotkeyBinding {
            base_form_id: key.base_form_id,
            condition: key.condition,
        });
    }
    hotkeys
}

#[cfg(test)]
#[path = "tests/persist.rs"]
mod tests;
