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

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_boxddd::prelude::BoxdddPhysicsContext;

use crate::save::{
    ItemStack, PersistentReferenceDelta, PlayerState, SaveGame, SaveGameHeader, SavePlugin,
    SaveStore, SavedBodyState, SavedTransform,
};
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

pub(crate) fn install(app: &mut App) {
    app.init_resource::<ActiveSaveState>()
        .init_resource::<SaveDirectory>()
        .init_resource::<PersistRestores>()
        .init_resource::<PendingEvictionCaptures>()
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
    info!("save capture {cell:08x} deltas={delta_count}");
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
/// overrides, enable roots, locks, container inventories) from any
/// previously loaded save delta. All-default deltas are dropped.
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
    if cell_state.references.is_empty() {
        state.0.cells.remove(&cell);
    }
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
    info!("save apply {cell:08x} deltas={delta_count}");
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
    let player = PlayerState {
        inventory: world
            .get_resource::<interaction::PlayerInventory>()
            .map(|inventory| {
                inventory
                    .stacks()
                    .into_iter()
                    .map(|(base_form_id, count)| ItemStack {
                        base_form_id,
                        count,
                    })
                    .collect()
            })
            .unwrap_or_default(),
    };
    let save = SaveGame {
        header,
        world: world
            .get_resource::<ActiveSaveState>()
            .map(|state| state.0.clone())
            .unwrap_or_default(),
        player: Some(player),
        rng_state: 0,
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

#[cfg(test)]
mod tests {
    use super::*;

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
        world
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
            // `PreparedPluginSource` is not re-exported from `crate::vsa`
            // and widening that surface for a test is not worth it; an
            // empty plugin list exercises the same identity plumbing.
            source_plugins: Vec::new(),
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
            }]
        );
        let _ = std::fs::remove_dir_all(save_dir);
    }
}
