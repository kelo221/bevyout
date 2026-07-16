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

use super::super::{animation, interaction, player};
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
        && placement.asset_path.is_some()
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
    if cell_state.references.is_empty() && cell_state.dropped_items.is_empty() {
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
    if cell_state.references.is_empty() && cell_state.dropped_items.is_empty() {
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
    if cell_state.references.is_empty() && cell_state.dropped_items.is_empty() {
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

    let Some(manifest) = world.get_resource::<PreparedSceneManifest>() else {
        anyhow::bail!("no prepared scene manifest loaded");
    };
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
        current_cell: active,
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
        canonical: world
            .get_resource::<interaction::CanonicalItemLedger>()
            .map(interaction::CanonicalItemLedger::snapshot),
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
mod tests {
    use super::*;
    use crate::item_transaction::{
        HolderId, ItemExtraEntry, ItemHolderState, ItemInstance, ItemInstanceId, ItemLedger,
        ItemState, OwnershipProvenance,
    };

    fn placement(reference_form_id: u32, translation: [f32; 3]) -> PreparedPlacement {
        PreparedPlacement {
            reference_form_id,
            base_form_id: 0x1,
            asset_path: Some("meshes/test.glb".into()),
            translation,
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            error: None,
            physics_asset_path: None,
            physics_source: None,
            physics_classification: Default::default(),
            step_support: false,
            mutability: Default::default(),
            mutability_root_form_id: None,
            reference_kind: "REFR".into(),
            base_kind: "STAT".into(),
            editor_id: None,
            display_name: None,
            count: 1,
            semantic: Default::default(),
            initially_enabled: true,
            enable_parent: None,
            owner_form_id: None,
            owner_faction_rank: None,
            inventory: Vec::new(),
            audio: Default::default(),
            ao_mode: "ao-none".into(),
        }
    }

    fn test_world() -> World {
        let mut world = World::new();
        world.init_resource::<ActiveSaveState>();
        world.init_resource::<PersistRestores>();
        world.init_resource::<interaction::InteractionState>();
        world.init_resource::<bevy::ecs::message::Messages<animation::PlayPlacementAnimation>>();
        world.init_resource::<interaction::ContainerStates>();
        world
    }

    fn container_placement(
        reference_form_id: u32,
        entries: Vec<PreparedInventoryEntry>,
    ) -> PreparedPlacement {
        PreparedPlacement {
            semantic: PreparedSemantic::Container,
            inventory: entries,
            ..placement(reference_form_id, [0.0, 0.0, 0.0])
        }
    }

    fn inventory_entry(base_form_id: u32, count: i32, leveled: bool) -> PreparedInventoryEntry {
        PreparedInventoryEntry {
            base_form_id,
            count,
            record_kind: "MISC".into(),
            editor_id: None,
            display_name: None,
            leveled,
        }
    }

    fn spawn_cell(world: &mut World, placements: &[PreparedPlacement]) -> (Entity, Vec<Entity>) {
        let root = world.spawn((Transform::default(), Visibility::Hidden)).id();
        let children = placements
            .iter()
            .map(|placement| {
                world
                    .spawn((
                        interaction::PlacementRoot::new(placement.clone()),
                        Transform {
                            translation: Vec3::from_array(placement.translation),
                            rotation: Quat::from_array(placement.rotation_xyzw),
                            scale: Vec3::splat(placement.scale),
                        },
                        Visibility::Inherited,
                        ChildOf(root),
                    ))
                    .id()
            })
            .collect();
        (root, children)
    }

    // T60.4: a moved placement's transform survives capture -> despawn ->
    // respawn -> apply (the swap-away-and-back shape, minus the swap driver).
    #[test]
    fn capture_and_apply_restore_a_moved_placement_transform() {
        let mut world = test_world();
        let placements = [
            placement(0x10, [0.0, 0.0, 0.0]),
            placement(0x20, [5.0, 0.0, 0.0]),
        ];
        let (root, children) = spawn_cell(&mut world, &placements);
        world
            .entity_mut(children[0])
            .insert(Transform::from_xyz(1.0, 2.0, 3.0));

        capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
        let state = world.resource::<ActiveSaveState>();
        let cell = state.0.cells.get(&0xC0DE).expect("cell state captured");
        assert!(cell.references.contains_key(&0x10));
        assert!(
            !cell.references.contains_key(&0x20),
            "untouched ref must produce no delta"
        );

        // Simulate eviction + fresh respawn at baseline pose.
        world.entity_mut(root).despawn();
        let (root, children) = spawn_cell(&mut world, &placements);
        apply_cell_placements(&mut world, 0xC0DE, root, &placements);

        let restored = world.get::<Transform>(children[0]).unwrap();
        assert_eq!(restored.translation, Vec3::new(1.0, 2.0, 3.0));
        let untouched = world.get::<Transform>(children[1]).unwrap();
        assert_eq!(untouched.translation, Vec3::new(5.0, 0.0, 0.0));
        // The restore is staged for the collider build (no physics here).
        assert!(
            world
                .resource::<PersistRestores>()
                .bodies
                .contains_key(&0x10)
        );
    }

    // T60.4: a taken pickup (despawned entity) is captured deleted, and a
    // later apply despawns the freshly-spawned entity and suppresses its
    // collider build.
    #[test]
    fn a_taken_pickup_stays_deleted_across_capture_and_apply() {
        let mut world = test_world();
        let placements = [placement(0x10, [0.0, 0.0, 0.0])];
        let (root, children) = spawn_cell(&mut world, &placements);
        world.entity_mut(children[0]).despawn();

        capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
        assert!(world.resource::<ActiveSaveState>().0.cells[&0xC0DE].references[&0x10].deleted);

        world.entity_mut(root).despawn();
        let (root, children) = spawn_cell(&mut world, &placements);
        apply_cell_placements(&mut world, 0xC0DE, root, &placements);
        assert!(
            world.get_entity(children[0]).is_err(),
            "deleted ref must despawn"
        );
        assert!(
            world
                .resource::<PersistRestores>()
                .suppressed
                .contains(&0x10)
        );
    }

    // T60.4: an open container is captured activated and re-opened on apply
    // (open set + Opening clip replayed exactly once).
    #[test]
    fn an_open_container_reopens_on_apply() {
        let mut world = test_world();
        let placements = [placement(0x10, [0.0, 0.0, 0.0])];
        let (root, children) = spawn_cell(&mut world, &placements);
        world
            .resource_mut::<interaction::InteractionState>()
            .open
            .insert(children[0]);

        capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
        assert_eq!(
            world.resource::<ActiveSaveState>().0.cells[&0xC0DE].references[&0x10].activated,
            Some(true)
        );

        world.entity_mut(root).despawn();
        world
            .resource_mut::<interaction::InteractionState>()
            .open
            .clear();
        let (root, children) = spawn_cell(&mut world, &placements);
        apply_cell_placements(&mut world, 0xC0DE, root, &placements);
        assert!(
            world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&children[0])
        );
        let messages =
            world.resource::<bevy::ecs::message::Messages<animation::PlayPlacementAnimation>>();
        assert_eq!(messages.iter_current_update_messages().count(), 1);

        // Re-applying while already open must not replay the clip.
        apply_cell_placements(&mut world, 0xC0DE, root, &placements);
        let messages =
            world.resource::<bevy::ecs::message::Messages<animation::PlayPlacementAnimation>>();
        assert_eq!(messages.iter_current_update_messages().count(), 1);
    }

    // A loaded save's enabled override survives a capture that cannot
    // observe enable state (nothing toggles it at runtime yet).
    #[test]
    fn capture_preserves_a_loaded_enabled_override() {
        let mut world = test_world();
        let placements = [placement(0x10, [0.0, 0.0, 0.0])];
        let (root, _children) = spawn_cell(&mut world, &placements);
        world
            .resource_mut::<ActiveSaveState>()
            .0
            .cells
            .entry(0xC0DE)
            .or_default()
            .references
            .insert(
                0x10,
                PersistentReferenceDelta {
                    enabled: Some(false),
                    ..Default::default()
                },
            );

        capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
        assert_eq!(
            world.resource::<ActiveSaveState>().0.cells[&0xC0DE].references[&0x10].enabled,
            Some(false)
        );
    }

    // F60.2: a disabled-by-save reference is hidden on apply.
    #[test]
    fn apply_hides_a_reference_disabled_by_the_save() {
        let mut world = test_world();
        let placements = [placement(0x10, [0.0, 0.0, 0.0])];
        let (root, children) = spawn_cell(&mut world, &placements);
        world
            .resource_mut::<ActiveSaveState>()
            .0
            .cells
            .entry(0xC0DE)
            .or_default()
            .references
            .insert(
                0x10,
                PersistentReferenceDelta {
                    enabled: Some(false),
                    ..Default::default()
                },
            );
        apply_cell_placements(&mut world, 0xC0DE, root, &placements);
        assert_eq!(
            world.get::<Visibility>(children[0]),
            Some(&Visibility::Hidden)
        );
    }

    fn minimal_manifest(cell_form_id: u32) -> PreparedSceneManifest {
        PreparedSceneManifest {
            schema_version: 13,
            prepare_revision: None,
            converter_revision: None,
            physics_schema_version: None,
            asset_root: ".".into(),
            source_plugin: "Fallout3.esm".into(),
            source_fingerprint: "content-hash".into(),
            item_catalog_path: None,
            item_catalog_revision: None,
            item_catalog_hash: None,
            recipe_catalog_path: None,
            recipe_catalog_revision: None,
            recipe_catalog_hash: None,
            // `PreparedPluginSource` is not re-exported from `crate::vsa`
            // and widening that surface for a test is not worth it; an
            // empty plugin list exercises the same identity plumbing.
            source_plugins: Vec::new(),
            visual_issues: Vec::new(),
            cell: crate::vsa::CellInfo {
                form_id: cell_form_id,
                editor_id: None,
                name: None,
                interior: true,
                ambient_rgba: [0.0; 4],
                directional_rgba: [0.0; 4],
                image_space_form_id: None,
                image_space: None,
                lighting_template_form_id: None,
                lighting_template_flags: 0,
                lighting_template: None,
                raw_lighting: None,
                effective_lighting: None,
                water_form_id: None,
                water_height: None,
                grid: None,
                worldspace_form_id: None,
            },
            placements: Vec::new(),
            lights: Vec::new(),
            diagnostics: Vec::new(),
            navmeshes: Vec::new(),
            cell_audio: Default::default(),
            audio_clips: Vec::new(),
            footstep_sets: Vec::new(),
            hard_landing_clips: Vec::new(),
            bake: None,
            static_point_shadows: None,
            mutability_summary: Default::default(),
            leveled_lists: Default::default(),
        }
    }

    // F60.3/F60.4: the console `save` path captures the active cell, stamps
    // the manifest's content identity, includes the player inventory, and
    // the written slot round-trips through `SaveStore`.
    #[test]
    fn write_save_slot_round_trips_world_state_and_player_inventory() {
        let save_dir = std::env::temp_dir().join(format!(
            "bevyout-persist-test-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut world = test_world();
        world.init_resource::<super::super::preload::ResidentCells>();
        world.insert_resource(ActiveCell(0xC0DE));
        world.insert_resource(minimal_manifest(0xC0DE));
        world.insert_resource(SaveDirectory(save_dir.clone()));
        world.insert_resource(interaction::PlayerInventory::from_stacks([(0x42, 2)]));
        world
            .resource_mut::<ActiveSaveState>()
            .0
            .cells
            .entry(0xC0DE)
            .or_default()
            .references
            .insert(
                0x10,
                PersistentReferenceDelta {
                    deleted: true,
                    ..Default::default()
                },
            );

        let path = write_save_slot(&mut world, "slot1").expect("save must write");
        assert!(path.ends_with("slot1.bevyoutsave"));

        let outcome = SaveStore::from_save_dir(&save_dir)
            .read_slot("slot1")
            .expect("slot must read back");
        assert_eq!(outcome.save.header.content_fingerprint, "content-hash");
        assert_eq!(outcome.save.header.current_cell, 0xC0DE);
        assert!(outcome.save.header.plugins.is_empty());
        assert!(outcome.save.world.cells[&0xC0DE].references[&0x10].deleted);
        assert_eq!(
            outcome.save.player.as_ref().unwrap().inventory,
            vec![ItemStack {
                base_form_id: 0x42,
                count: 2,
                condition: None,
            }]
        );
        assert!(
            outcome
                .save
                .canonical
                .as_ref()
                .and_then(|snapshot| snapshot.holders.get(&HolderId::Player))
                .is_some()
        );
        let _ = std::fs::remove_dir_all(save_dir);
    }

    #[test]
    fn save_uses_canonical_player_items_and_preserves_opaque_state_and_revision() {
        let save_dir = std::env::temp_dir().join(format!(
            "bevyout-canonical-save-test-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut world = test_world();
        world.init_resource::<super::super::preload::ResidentCells>();
        world.insert_resource(ActiveCell(0xC0DE));
        world.insert_resource(minimal_manifest(0xC0DE));
        world.insert_resource(SaveDirectory(save_dir.clone()));
        world.insert_resource(interaction::PlayerInventory::from_stacks([(0x99, 99)]));

        let item = ItemInstance::new(
            ItemInstanceId(42),
            0x42,
            2,
            ItemState {
                condition: Some(80),
                ownership: OwnershipProvenance {
                    origin_owner_form_id: Some(0x1234),
                    origin_faction_rank: Some(2),
                    stolen: true,
                },
                extras: vec![ItemExtraEntry {
                    namespace_form_id: 0x77,
                    tag: *b"TEST",
                    payload: vec![1, 2, 3],
                }],
            },
        )
        .unwrap();
        let state = ItemHolderState {
            items: vec![item.clone()],
            caps: 12,
            revision: 7,
        };
        let mut canonical = ItemLedger::new();
        canonical.insert_holder(HolderId::Player, state).unwrap();
        canonical.bind_hotkey(HolderId::Player, 0, item.id).unwrap();
        canonical.equip(HolderId::Player, item.id).unwrap();
        let before = canonical.snapshot();
        world.insert_resource(interaction::CanonicalItemLedger { ledger: canonical });

        write_save_slot(&mut world, "canonical").expect("canonical save must write");

        let after = world
            .resource::<interaction::CanonicalItemLedger>()
            .snapshot();
        assert_eq!(
            after.holders[&HolderId::Player].revision,
            before.holders[&HolderId::Player].revision
        );
        assert_eq!(after, before);
        let outcome = SaveStore::from_save_dir(&save_dir)
            .read_slot("canonical")
            .expect("canonical slot must read back");
        let saved = outcome.save.canonical.expect("v3 canonical state");
        assert_eq!(saved, before);
        assert_eq!(
            outcome.save.player.unwrap().inventory,
            vec![ItemStack {
                base_form_id: 0x42,
                count: 2,
                condition: Some(80),
            }]
        );
        let _ = std::fs::remove_dir_all(save_dir);
    }

    #[test]
    fn save_does_not_bootstrap_over_an_intentionally_empty_canonical_player() {
        let save_dir = std::env::temp_dir().join(format!(
            "bevyout-empty-canonical-save-test-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut world = test_world();
        world.init_resource::<super::super::preload::ResidentCells>();
        world.insert_resource(ActiveCell(0xC0DE));
        world.insert_resource(minimal_manifest(0xC0DE));
        world.insert_resource(SaveDirectory(save_dir.clone()));
        world.insert_resource(interaction::PlayerInventory::from_stacks([(0x99, 99)]));

        let mut canonical = ItemLedger::new();
        canonical
            .insert_holder(
                HolderId::Player,
                ItemHolderState {
                    revision: 11,
                    ..Default::default()
                },
            )
            .unwrap();
        world.insert_resource(interaction::CanonicalItemLedger { ledger: canonical });

        write_save_slot(&mut world, "empty").expect("empty canonical save must write");

        let outcome = SaveStore::from_save_dir(&save_dir)
            .read_slot("empty")
            .expect("empty canonical slot must read back");
        let snapshot = outcome.save.canonical.expect("v3 canonical state");
        let player = &snapshot.holders[&HolderId::Player];
        assert!(player.items.is_empty());
        assert_eq!(player.revision, 11);
        assert!(outcome.save.player.unwrap().inventory.is_empty());
        let _ = std::fs::remove_dir_all(save_dir);
    }

    // Issue #76 (F76.2/F76.3): a looted, leveled-resolved container survives
    // capture -> despawn/evict -> respawn -> apply, seeding
    // `interaction::ContainerStates` with the exact stacks and resolved marker it had
    // when the cell was left, the same swap-away-and-back shape
    // `capture_and_apply_restore_a_moved_placement_transform` exercises for
    // transforms.
    #[test]
    fn capture_and_apply_restore_looted_container_state() {
        let mut world = test_world();
        let placements = [container_placement(
            0x900,
            vec![inventory_entry(0x10, 3, false)],
        )];
        let (root, _children) = spawn_cell(&mut world, &placements);
        world
            .resource_mut::<interaction::ContainerStates>()
            .0
            .insert(
                0x900,
                interaction::container_policy::ContainerState {
                    stacks: vec![(0x10, 1)],
                    resolved: true,
                },
            );

        capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
        let saved = &world.resource::<ActiveSaveState>().0.cells[&0xC0DE].references[&0x900];
        assert_eq!(
            saved.inventory,
            Some(vec![ItemStack {
                base_form_id: 0x10,
                count: 1,
                condition: None,
            }])
        );
        assert_eq!(saved.leveled_resolved, Some(true));

        // Simulate eviction + fresh respawn with a cleared runtime state.
        world.entity_mut(root).despawn();
        world
            .resource_mut::<interaction::ContainerStates>()
            .0
            .clear();
        let (root, _children) = spawn_cell(&mut world, &placements);
        apply_cell_placements(&mut world, 0xC0DE, root, &placements);

        let restored = &world.resource::<interaction::ContainerStates>().0[&0x900];
        assert_eq!(restored.stacks, vec![(0x10, 1)]);
        assert!(restored.resolved);
    }

    // F118.3: corpse contents reuse the FormID-keyed ContainerStates and the
    // existing OBJE inventory delta, so leaving and re-entering a cell does
    // not lose or duplicate a looted stack.
    #[test]
    fn capture_and_apply_restore_looted_corpse_state() {
        let mut world = test_world();
        let mut corpse = container_placement(0x902, vec![inventory_entry(0x10, 3, false)]);
        corpse.semantic = PreparedSemantic::Corpse;
        corpse.base_kind = "ACHR".into();
        corpse.display_name = Some("Corpse".into());
        let placements = [corpse];
        let (root, _children) = spawn_cell(&mut world, &placements);
        world
            .resource_mut::<interaction::ContainerStates>()
            .0
            .insert(
                0x902,
                interaction::container_policy::ContainerState {
                    stacks: vec![(0x10, 1)],
                    resolved: true,
                },
            );

        capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
        let saved = &world.resource::<ActiveSaveState>().0.cells[&0xC0DE].references[&0x902];
        assert_eq!(
            saved.inventory,
            Some(vec![ItemStack {
                base_form_id: 0x10,
                count: 1,
                condition: None,
            }])
        );
        assert_eq!(saved.leveled_resolved, Some(true));

        world.entity_mut(root).despawn();
        world
            .resource_mut::<interaction::ContainerStates>()
            .0
            .clear();
        let (root, _children) = spawn_cell(&mut world, &placements);
        apply_cell_placements(&mut world, 0xC0DE, root, &placements);

        let restored = &world.resource::<interaction::ContainerStates>().0[&0x902];
        assert_eq!(restored.stacks, vec![(0x10, 1)]);
        assert!(restored.resolved);
    }

    // F118.3 compatibility: an old save with no corpse section does not
    // synthesize a corpse runtime holder during apply.
    #[test]
    fn apply_old_save_without_corpse_delta_leaves_corpse_unseeded() {
        let mut world = test_world();
        let mut corpse = container_placement(0x903, vec![inventory_entry(0x10, 3, false)]);
        corpse.semantic = PreparedSemantic::Corpse;
        corpse.base_kind = "ACHR".into();
        let placements = [corpse];
        let (root, _children) = spawn_cell(&mut world, &placements);

        apply_cell_placements(&mut world, 0xC0DE, root, &placements);

        assert!(
            !world
                .resource::<interaction::ContainerStates>()
                .0
                .contains_key(&0x903)
        );
    }

    // F76.2: a container whose stacks and resolved marker never diverge from
    // the manifest baseline produces no delta and no seed on the next apply
    // (it still rolls on first open, per F76.3).
    #[test]
    fn an_untouched_container_captures_and_seeds_nothing() {
        let mut world = test_world();
        let placements = [container_placement(
            0x901,
            vec![inventory_entry(0x10, 3, false)],
        )];
        let (root, _children) = spawn_cell(&mut world, &placements);
        world
            .resource_mut::<interaction::ContainerStates>()
            .0
            .insert(
                0x901,
                interaction::container_policy::ContainerState {
                    stacks: vec![(0x10, 3)],
                    resolved: false,
                },
            );

        capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
        assert!(
            !world
                .resource::<ActiveSaveState>()
                .0
                .cells
                .get(&0xC0DE)
                .is_some_and(|cell| cell.references.contains_key(&0x901))
        );

        world.entity_mut(root).despawn();
        world
            .resource_mut::<interaction::ContainerStates>()
            .0
            .clear();
        let (root, _children) = spawn_cell(&mut world, &placements);
        apply_cell_placements(&mut world, 0xC0DE, root, &placements);
        assert!(
            !world
                .resource::<interaction::ContainerStates>()
                .0
                .contains_key(&0x901)
        );
    }
}
