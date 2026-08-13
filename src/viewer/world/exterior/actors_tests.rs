//! Exterior gameplay-actor residency tests (M6 W3-C).
//!
//! These drive the real system against a bare `World` holding synthetic
//! resident cells, the same shape `tests.rs` uses for lifecycle actions. The
//! downstream projection chain (`project_prepared_actors` ->
//! `seed_actor_states` -> autonomous start) is not installed here; what is
//! asserted is exactly what this module owns: which entities exist, which
//! cell owns them, and where the canonical `ActorInstanceState` lives.

use bevy::prelude::*;
use bevyout_core::actor::ActorKind;
use bevyout_core::actor_state::{
    ActorInstanceState, ActorLifeState, ActorPackageCheckpoint, ActorValue,
};
use bevyout_core::manifest::exterior::{
    EXTERIOR_CELL_PACKAGE_REVISION, ExteriorCellLifecycle, ExteriorCellPackage, ExteriorCellState,
    ExteriorCoordinatePolicy, GridCoordinate, PreparedExteriorActor, PreparedExteriorEnvironment,
};

use super::super::lifecycle::{ExteriorStreamState, RuntimeCell};
use super::super::{ExteriorCellRoot, finalize_evictions};
use super::{ExteriorActorResidency, ExteriorResidentActor, sync_exterior_actor_residency};
use crate::viewer::LoadedSceneManifest;
use crate::viewer::actor_state::ActorDefinitionCatalogs;
use crate::viewer::interaction::PlacementRoot;
use crate::viewer::nav::api as nav_api;
use crate::viewer::world::ActiveSaveState;
use crate::vsa::{CellInfo, PreparedSceneManifest};

const EYEBOT: u32 = 0x0006_38e8;
const CELL_A: u32 = 0x0000_0c67;
const CELL_B: u32 = 0x0000_0c68;

fn grid_a() -> GridCoordinate {
    GridCoordinate::new(5, -6)
}

fn grid_b() -> GridCoordinate {
    GridCoordinate::new(4, -6)
}

fn prepared_actor(reference_form_id: u32, position: [f32; 3]) -> PreparedExteriorActor {
    PreparedExteriorActor {
        reference_form_id,
        base_form_id: 0x0001_cf73,
        kind: ActorKind::Creature,
        asset_path: None,
        physics_asset_path: None,
        assembly: None,
        position,
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        initially_enabled: true,
    }
}

fn package(
    cell_form_id: u32,
    grid: GridCoordinate,
    actors: Vec<PreparedExteriorActor>,
) -> ExteriorCellPackage {
    ExteriorCellPackage {
        revision: EXTERIOR_CELL_PACKAGE_REVISION.into(),
        content_fingerprint: "synthetic".into(),
        cell_form_id,
        worldspace_form_id: 0x0000_003c,
        grid,
        origin: [0.0; 3],
        terrain: None,
        water: None,
        static_objects: Vec::new(),
        dynamic_objects: Vec::new(),
        distant_objects: Vec::new(),
        actors,
        local_lights: Vec::new(),
        navigation: None,
        environment: PreparedExteriorEnvironment::default(),
        diagnostics: Vec::new(),
    }
}

/// Bevy position of the centre of `grid`, so a projected actor's live
/// transform really does fall inside the cell the test claims owns it.
fn grid_centre(grid: GridCoordinate) -> [f32; 3] {
    let policy = ExteriorCoordinatePolicy::default();
    let origin = policy.grid_origin(grid);
    let half = policy.cell_span_metres() * 0.5;
    [(origin[0] + half) as f32, 0.0, (origin[2] - half) as f32]
}

/// Duplicated from `nav::tests::support::minimal_manifest` (private to that
/// module, and to a `#[cfg(test)]` module tree this file cannot reach) --
/// only the shape `ensure_archipelago`'s already-current short-circuit
/// reads: `manifest.cell.form_id` and `manifest.exterior` (kept `None` so
/// the nav build treats this as an ordinary interior-shaped cell, entirely
/// independent of the *gameplay* exterior cells this module owns; nav's own
/// archipelago identity is a fiction this harness only needs to hold still).
fn minimal_manifest(cell_form_id: u32) -> PreparedSceneManifest {
    PreparedSceneManifest {
        schema_version: 17,
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
        actor_catalog_path: None,
        actor_catalog_revision: None,
        actor_catalog_hash: None,
        actor_animation_catalog_path: None,
        actor_animation_catalog_revision: None,
        actor_animation_catalog_hash: None,
        image_space_modifier_catalog_path: None,
        image_space_modifier_catalog_revision: None,
        image_space_modifier_catalog_hash: None,
        source_plugins: Vec::new(),
        visual_issues: Vec::new(),
        cell: CellInfo {
            form_id: cell_form_id,
            editor_id: None,
            name: None,
            interior: true,
            behave_like_exterior: false,
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
            day_night_profile: None,
            day_night_preview_profile: None,
        },
        placements: Vec::new(),
        lights: Vec::new(),
        diagnostics: Vec::new(),
        navmeshes: Vec::new(),
        nav_graph: None,
        cell_audio: Default::default(),
        audio_clips: Vec::new(),
        footstep_sets: Vec::new(),
        hard_landing_clips: Vec::new(),
        bake: None,
        static_point_shadows: None,
        reflection_probes: None,
        mutability_summary: Default::default(),
        leveled_lists: Default::default(),
        dialogue: None,
        exterior: None,
    }
}

struct TestWorld {
    world: World,
}

impl TestWorld {
    fn new() -> Self {
        let mut world = World::new();
        world.init_resource::<ExteriorStreamState>();
        world.init_resource::<ExteriorActorResidency>();
        world.init_resource::<ActiveSaveState>();
        world.init_resource::<ActorDefinitionCatalogs>();
        world.init_resource::<crate::viewer::actor_animation::ActorAnimationCatalogs>();
        Self { world }
    }

    /// Makes `nav::api::bind_actor` succeed in this bare `World`, through
    /// `ensure_archipelago`'s already-current short-circuit
    /// (`nav::api::mark_test_archipelago_current`) rather than a real
    /// prepared nav graph on disk. `cell_form_id` here is nav's own
    /// bookkeeping key, independent of any `ExteriorStreamState` cell.
    fn make_nav_bindable(&mut self, cell_form_id: u32) {
        self.world
            .insert_resource(LoadedSceneManifest(minimal_manifest(cell_form_id)));
        nav_api::insert_test_archipelago_state(&mut self.world);
        let archipelago = self.world.spawn_empty().id();
        nav_api::mark_test_archipelago_current(&mut self.world, cell_form_id, archipelago);
    }

    fn insert_cell(
        &mut self,
        cell_form_id: u32,
        grid: GridCoordinate,
        generation: u64,
        actors: Vec<PreparedExteriorActor>,
    ) -> Entity {
        let root = self
            .world
            .spawn((
                ExteriorCellRoot {
                    form_id: cell_form_id,
                },
                Transform::default(),
                Visibility::Inherited,
            ))
            .id();
        let mut state = self.world.resource_mut::<ExteriorStreamState>();
        state.cells.insert(
            grid,
            RuntimeCell {
                state: ExteriorCellState {
                    cell_form_id,
                    grid,
                    lifecycle: ExteriorCellLifecycle::Resident,
                    generation,
                    pinned: false,
                    estimated_bytes: 1,
                    failed_attempts: 0,
                },
                root: Some(root),
                task: None,
                package: Some(package(cell_form_id, grid, actors)),
                collision_ready: true,
                eviction_restore: None,
            },
        );
        root
    }

    fn begin_eviction(&mut self, grid: GridCoordinate) {
        let mut state = self.world.resource_mut::<ExteriorStreamState>();
        state
            .cells
            .get_mut(&grid)
            .expect("cell exists")
            .begin_eviction();
    }

    fn sync(&mut self) {
        sync_exterior_actor_residency(&mut self.world);
        self.world.flush();
    }

    fn projected(&mut self) -> Vec<(Entity, ExteriorResidentActor)> {
        let mut actors = self
            .world
            .query::<(Entity, &ExteriorResidentActor)>()
            .iter(&self.world)
            .map(|(entity, resident)| (entity, *resident))
            .collect::<Vec<_>>();
        actors.sort_by_key(|(entity, _)| entity.index_u32());
        actors
    }

    fn placement_roots(&mut self, reference_form_id: u32) -> usize {
        self.world
            .query::<&PlacementRoot>()
            .iter(&self.world)
            .filter(|root| root.placement().reference_form_id == reference_form_id)
            .count()
    }

    fn saved_cell(&self, reference_form_id: u32) -> Option<u32> {
        self.world
            .resource::<ActiveSaveState>()
            .0
            .cells
            .iter()
            .find_map(|(cell_form_id, cell)| {
                cell.actors
                    .contains_key(&reference_form_id)
                    .then_some(*cell_form_id)
            })
    }

    fn seed_saved_state(&mut self, cell_form_id: u32, reference_form_id: u32) {
        self.world
            .resource_mut::<ActiveSaveState>()
            .0
            .cells
            .entry(cell_form_id)
            .or_default()
            .actors
            .insert(
                reference_form_id,
                ActorInstanceState::new(reference_form_id, ActorLifeState::Alive),
            );
    }

    fn set_actor_position(&mut self, entity: Entity, position: [f32; 3]) {
        if let Some(mut transform) = self.world.get_mut::<Transform>(entity) {
            transform.translation = Vec3::from_array(position);
        }
        if let Some(mut global) = self.world.get_mut::<GlobalTransform>(entity) {
            *global = GlobalTransform::from_translation(Vec3::from_array(position));
        }
    }
}

#[test]
fn resident_exterior_cell_projects_its_prepared_actor_once() {
    let mut test = TestWorld::new();
    let grid = grid_a();
    test.insert_cell(
        CELL_A,
        grid,
        1,
        vec![prepared_actor(EYEBOT, grid_centre(grid))],
    );

    test.sync();
    test.sync();

    let projected = test.projected();
    assert_eq!(projected.len(), 1);
    assert_eq!(projected[0].1.reference_form_id, EYEBOT);
    assert_eq!(projected[0].1.cell_form_id, CELL_A);
    assert_eq!(projected[0].1.generation, 1);
    assert_eq!(test.placement_roots(EYEBOT), 1);
}

#[test]
fn a_second_resident_generation_cannot_project_a_duplicate_actor_entity() {
    let mut test = TestWorld::new();
    let grid = grid_a();
    test.insert_cell(
        CELL_A,
        grid,
        1,
        vec![prepared_actor(EYEBOT, grid_centre(grid))],
    );
    test.sync();
    let first = test.projected()[0].0;

    // A reload bumped the generation while the previous projection is still
    // live: the policy must refuse a second projection, not create one.
    {
        let mut state = test.world.resource_mut::<ExteriorStreamState>();
        state.cells.get_mut(&grid).expect("cell").state.generation = 2;
    }
    test.sync();

    let projected = test.projected();
    assert_eq!(projected.len(), 1, "exactly one canonical actor entity");
    assert_eq!(projected[0].0, first);
    assert_eq!(test.placement_roots(EYEBOT), 1);
}

#[test]
fn border_crossing_moves_canonical_state_without_respawning_the_entity() {
    let mut test = TestWorld::new();
    let (grid, destination) = (grid_a(), grid_b());
    test.insert_cell(
        CELL_A,
        grid,
        1,
        vec![prepared_actor(EYEBOT, grid_centre(grid))],
    );
    test.insert_cell(CELL_B, destination, 1, Vec::new());
    test.sync();
    let entity = test.projected()[0].0;
    test.seed_saved_state(CELL_A, EYEBOT);

    test.set_actor_position(entity, grid_centre(destination));
    test.sync();

    let projected = test.projected();
    assert_eq!(projected.len(), 1);
    assert_eq!(projected[0].0, entity, "handoff never respawns the entity");
    assert_eq!(projected[0].1.cell_form_id, CELL_B);
    assert_eq!(test.saved_cell(EYEBOT), Some(CELL_B));
    let destination_root = test
        .world
        .resource::<ExteriorStreamState>()
        .cells
        .get(&destination)
        .and_then(|cell| cell.root);
    assert_eq!(
        test.world
            .get::<ChildOf>(entity)
            .map(|child| child.parent()),
        destination_root
    );
}

#[test]
fn eviction_snapshots_the_package_checkpoint_before_the_root_despawns() {
    let mut test = TestWorld::new();
    let grid = grid_a();
    let root = test.insert_cell(
        CELL_A,
        grid,
        1,
        vec![prepared_actor(EYEBOT, grid_centre(grid))],
    );
    test.sync();
    let entity = test.projected()[0].0;
    test.seed_saved_state(CELL_A, EYEBOT);
    test.world
        .resource_mut::<ActiveSaveState>()
        .0
        .cells
        .get_mut(&CELL_A)
        .expect("cell state")
        .actors
        .get_mut(&EYEBOT)
        .expect("actor state")
        .set_value_mutation(ActorValue::Health, 3.0)
        .expect("valid mutation");

    test.begin_eviction(grid);
    test.sync();

    assert!(
        test.world.get::<ExteriorResidentActor>(entity).is_none(),
        "the unloaded actor is no longer a live owner"
    );
    assert_eq!(test.saved_cell(EYEBOT), Some(CELL_A));
    assert!(
        test.world.get_entity(root).is_ok(),
        "the unload runs while the cell root is still alive"
    );

    finalize_evictions(&mut test.world);
    test.world.flush();
    assert!(test.world.get_entity(root).is_err());
    assert!(
        test.world.get_entity(entity).is_err(),
        "the cell root's despawn tears down its actor child"
    );
    assert_eq!(test.saved_cell(EYEBOT), Some(CELL_A));
    assert_eq!(
        test.world
            .resource::<ActiveSaveState>()
            .0
            .cells
            .get(&CELL_A)
            .and_then(|cell| cell.actors.get(&EYEBOT))
            .map(|state| state.value_mutations.len()),
        Some(1),
        "canonical mutations survive the cell's teardown"
    );
}

#[test]
fn reload_at_a_new_generation_restores_the_saved_actor_state_and_resumes_its_package() {
    let mut test = TestWorld::new();
    let grid = grid_a();
    let root = test.insert_cell(
        CELL_A,
        grid,
        1,
        vec![prepared_actor(EYEBOT, grid_centre(grid))],
    );
    test.sync();
    test.seed_saved_state(CELL_A, EYEBOT);
    test.world
        .resource_mut::<ActiveSaveState>()
        .0
        .cells
        .get_mut(&CELL_A)
        .expect("cell state")
        .actors
        .get_mut(&EYEBOT)
        .expect("actor state")
        .package = Some(ActorPackageCheckpoint {
        package_form_id: 0x0004_1600,
        procedure_index: 2,
        elapsed_seconds: 4.5,
    });
    test.begin_eviction(grid);
    test.sync();
    finalize_evictions(&mut test.world);
    test.world.flush();
    assert!(test.world.get_entity(root).is_err());
    assert!(test.projected().is_empty());

    test.insert_cell(
        CELL_A,
        grid,
        2,
        vec![prepared_actor(EYEBOT, grid_centre(grid))],
    );
    test.sync();

    let projected = test.projected();
    assert_eq!(projected.len(), 1);
    assert_eq!(projected[0].1.generation, 2);
    let restored = test
        .world
        .resource::<ActiveSaveState>()
        .0
        .cells
        .get(&CELL_A)
        .and_then(|cell| cell.actors.get(&EYEBOT))
        .expect("canonical actor state survived the reload");
    assert_eq!(
        restored
            .package
            .map(|checkpoint| checkpoint.procedure_index),
        Some(2),
        "the checkpoint is available for the autonomous driver to resume"
    );
    assert_eq!(
        super::saved_package_checkpoint(&test.world, EYEBOT)
            .map(|checkpoint| checkpoint.package_form_id),
        Some(0x0004_1600)
    );
}

#[test]
fn stale_generation_unload_is_rejected_and_counted_without_tearing_down_the_live_actor() {
    let mut test = TestWorld::new();
    let grid = grid_a();
    test.insert_cell(
        CELL_A,
        grid,
        1,
        vec![prepared_actor(EYEBOT, grid_centre(grid))],
    );
    test.sync();
    let entity = test.projected()[0].0;

    // The live projection still claims generation 1 while the cell has moved
    // on to generation 3. Retaining it is put to the policy, which refuses a
    // generation the projection never bound to; the live actor is neither
    // adopted by the new generation nor torn down behind the policy's back.
    {
        let mut state = test.world.resource_mut::<ExteriorStreamState>();
        state.cells.get_mut(&grid).expect("cell").state.generation = 3;
    }
    test.sync();

    assert_eq!(
        test.world.resource::<ExteriorActorResidency>().rejections,
        1
    );
    assert_eq!(
        test.world
            .resource::<ExteriorActorResidency>()
            .last_rejection
            .get(&EYEBOT)
            .copied(),
        Some("StaleSource")
    );
    assert!(test.world.get::<ExteriorResidentActor>(entity).is_some());
}

#[test]
fn cancelled_eviction_reversal_leaves_exactly_one_projected_actor() {
    let mut test = TestWorld::new();
    let grid = grid_a();
    test.insert_cell(
        CELL_A,
        grid,
        1,
        vec![prepared_actor(EYEBOT, grid_centre(grid))],
    );
    test.sync();
    let entity = test.projected()[0].0;

    test.begin_eviction(grid);
    test.sync();
    assert!(test.world.get::<ExteriorResidentActor>(entity).is_none());

    // `cancel_eviction` restores the lifecycle at the bumped generation; the
    // cell root and its actor child were never despawned.
    {
        let mut state = test.world.resource_mut::<ExteriorStreamState>();
        let cell = state.cells.get_mut(&grid).expect("cell");
        cell.cancel_eviction(grid, true);
        cell.state.lifecycle = ExteriorCellLifecycle::Resident;
    }
    test.sync();
    test.sync();

    let projected = test.projected();
    assert_eq!(projected.len(), 1, "no duplicate projection after reversal");
    assert_eq!(projected[0].1.generation, 2);
    assert_eq!(test.placement_roots(EYEBOT), 1);
}

/// Issue #303 regression. Live re-verification found a handoff itself never
/// touches an actor's nav components -- the same `Entity` keeps them across
/// the border -- but the actor can still lose its bind moments later (the
/// KCC free-falling through not-yet-settled destination collision, caught by
/// `nav_fall_guard_system::release_bound_actor`), and nothing used to notice.
/// `release_actor` reproduces that release directly rather than depending on
/// a physics harness this module's own test doc says is deliberately not
/// installed here.
#[test]
fn handoff_preserves_a_live_nav_bind_and_recovers_one_lost_after_the_crossing() {
    let mut test = TestWorld::new();
    test.make_nav_bindable(0xFFFF_0001);
    let (grid, destination) = (grid_a(), grid_b());
    test.insert_cell(
        CELL_A,
        grid,
        1,
        vec![prepared_actor(EYEBOT, grid_centre(grid))],
    );
    test.insert_cell(CELL_B, destination, 1, Vec::new());
    test.sync();
    let entity = test.projected()[0].0;
    test.seed_saved_state(CELL_A, EYEBOT);

    // A `tna bind` (or a successful autonomous package start) already gave
    // this actor a live nav agent before the border crossing.
    nav_api::insert_test_nav_agent(&mut test.world, entity);
    assert!(nav_api::is_bound(&test.world, entity));

    test.set_actor_position(entity, grid_centre(destination));
    test.sync();

    let projected = test.projected();
    assert_eq!(projected.len(), 1);
    assert_eq!(projected[0].0, entity, "handoff never respawns the entity");
    assert_eq!(projected[0].1.cell_form_id, CELL_B);
    assert!(
        nav_api::is_bound(&test.world, entity),
        "a handoff must not drop the actor's live nav agent"
    );

    nav_api::release_actor(&mut test.world, entity);
    assert!(!nav_api::is_bound(&test.world, entity));

    test.sync();

    assert!(
        nav_api::is_bound(&test.world, entity),
        "a nav bind lost after a handoff must be recovered on the residency module's next poll"
    );
}

/// Issue #303 regression, restore side. A restore respawns a fresh `Entity`
/// through the same chain a first bind uses (`project_prepared_actors` ->
/// `seed_actor_states` -> `ai::autonomous`), which is not installed in this
/// bare-`World` harness (see this file's module doc) -- so the fresh entity
/// is, correctly, unbound the instant it spawns. What must not happen is the
/// bind staying lost forever: a reference this module once observed bound
/// stays tracked across the eviction, and the very next residency poll after
/// restore recovers it.
#[test]
fn restore_recovers_a_nav_bind_the_spawn_chain_left_unbound() {
    let mut test = TestWorld::new();
    test.make_nav_bindable(0xFFFF_0002);
    let grid = grid_a();
    let root = test.insert_cell(
        CELL_A,
        grid,
        1,
        vec![prepared_actor(EYEBOT, grid_centre(grid))],
    );
    test.sync();
    let first_entity = test.projected()[0].0;
    test.seed_saved_state(CELL_A, EYEBOT);
    nav_api::insert_test_nav_agent(&mut test.world, first_entity);
    assert!(nav_api::is_bound(&test.world, first_entity));

    test.begin_eviction(grid);
    test.sync();
    finalize_evictions(&mut test.world);
    test.world.flush();
    assert!(test.world.get_entity(root).is_err());
    assert!(test.projected().is_empty());

    test.insert_cell(
        CELL_A,
        grid,
        2,
        vec![prepared_actor(EYEBOT, grid_centre(grid))],
    );
    test.sync();

    let projected = test.projected();
    assert_eq!(projected.len(), 1);
    let restored_entity = projected[0].0;
    assert_ne!(
        restored_entity, first_entity,
        "a restore spawns a fresh entity, unlike a handoff"
    );
    assert!(
        !nav_api::is_bound(&test.world, restored_entity),
        "the fresh entity has no nav agent on the frame it spawns in this harness"
    );

    test.sync();

    assert!(
        nav_api::is_bound(&test.world, restored_entity),
        "a nav bind established before an eviction must be recovered after restore"
    );
}
