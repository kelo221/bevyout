use std::path::Path;
use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::mesh::Mesh;
use bevy::prelude::{App, Assets, MinimalPlugins, StandardMaterial, Update};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevyout_core::manifest::exterior::{
    EXTERIOR_CELL_PACKAGE_REVISION, ExteriorCellPackage, GridCoordinate,
    PreparedExteriorEnvironment,
};

use super::{ExteriorPackageTask, LoadedExteriorPackage, decode_package, poll};

fn empty_package() -> ExteriorCellPackage {
    ExteriorCellPackage {
        revision: EXTERIOR_CELL_PACKAGE_REVISION.into(),
        content_fingerprint: "synthetic".into(),
        cell_form_id: 0x10,
        worldspace_form_id: 0x20,
        grid: GridCoordinate::new(-1, 2),
        origin: [0.0; 3],
        terrain: None,
        water: None,
        static_objects: Vec::new(),
        dynamic_objects: Vec::new(),
        distant_objects: Vec::new(),
        local_lights: Vec::new(),
        navigation: None,
        environment: PreparedExteriorEnvironment::default(),
        diagnostics: Vec::new(),
    }
}

fn task(
    result: Result<LoadedExteriorPackage, String>,
) -> Task<Result<LoadedExteriorPackage, String>> {
    AsyncComputeTaskPool::get().spawn(async move { result })
}

fn base_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .add_systems(Update, poll);
    app
}

fn test_wait_for_task_pool() {
    std::thread::sleep(Duration::from_millis(10));
}

#[test]
fn decoded_package_preserves_the_exact_serialized_byte_length() {
    let package = empty_package();
    let canonical = ron::ser::to_string(&package).expect("synthetic package serializes");
    let bytes = format!("\n  {canonical}\n").into_bytes();

    let loaded = decode_package(Path::new("synthetic-package.ron"), &bytes)
        .expect("synthetic package parses");

    assert_eq!(loaded.serialized_bytes, bytes.len() as u64);
    assert_ne!(loaded.serialized_bytes, canonical.len() as u64);
    assert_eq!(loaded.package, package);
}

#[test]
fn stale_completion_cannot_spawn_a_root_or_leave_a_task_owner() {
    let mut app = base_app();
    let grid = GridCoordinate::new(4, -3);
    let form_id = 0x10;
    let task_entity = app
        .world_mut()
        .spawn(ExteriorPackageTask {
            form_id,
            grid,
            generation: 1,
            task: task(Ok(LoadedExteriorPackage {
                package: empty_package(),
                serialized_bytes: 32,
            })),
        })
        .id();
    let mut state = super::super::lifecycle::ExteriorStreamState::default();
    state.cells.insert(
        grid,
        super::super::lifecycle::RuntimeCell {
            state: bevyout_core::manifest::exterior::ExteriorCellState {
                cell_form_id: form_id,
                grid,
                lifecycle: bevyout_core::manifest::exterior::ExteriorCellLifecycle::Loading,
                generation: 2,
                pinned: false,
                estimated_bytes: 0,
                failed_attempts: 0,
            },
            root: None,
            task: Some(task_entity),
            package: None,
            collision_ready: false,
            eviction_restore: None,
        },
    );
    state.resident_budget = 1;
    app.insert_resource(state);

    test_wait_for_task_pool();
    app.update();

    let state = app
        .world()
        .resource::<super::super::lifecycle::ExteriorStreamState>();
    assert_eq!(state.stale_completions, 1);
    assert!(state.cells[&grid].root.is_none());
    assert!(state.cells[&grid].task.is_none());
    let mut roots = app.world_mut().query::<&super::super::ExteriorCellRoot>();
    assert_eq!(roots.iter(app.world()).count(), 0);
}

#[test]
fn duplicate_same_generation_tasks_can_commit_only_one_package_root() {
    let mut app = base_app();
    let grid = GridCoordinate::new(-1, 2);
    let form_id = 0x10;
    let first = app
        .world_mut()
        .spawn(ExteriorPackageTask {
            form_id,
            grid,
            generation: 3,
            task: task(Ok(LoadedExteriorPackage {
                package: empty_package(),
                serialized_bytes: 48,
            })),
        })
        .id();
    let second = app
        .world_mut()
        .spawn(ExteriorPackageTask {
            form_id,
            grid,
            generation: 3,
            task: task(Ok(LoadedExteriorPackage {
                package: empty_package(),
                serialized_bytes: 48,
            })),
        })
        .id();
    let mut state = super::super::lifecycle::ExteriorStreamState::default();
    state.cells.insert(
        grid,
        super::super::lifecycle::RuntimeCell {
            state: bevyout_core::manifest::exterior::ExteriorCellState {
                cell_form_id: form_id,
                grid,
                lifecycle: bevyout_core::manifest::exterior::ExteriorCellLifecycle::Loading,
                generation: 3,
                pinned: false,
                estimated_bytes: 0,
                failed_attempts: 0,
            },
            root: None,
            task: Some(second),
            package: None,
            collision_ready: false,
            eviction_restore: None,
        },
    );
    state.resident_budget = 1;
    app.insert_resource(state);

    let _ = first;
    test_wait_for_task_pool();
    app.update();

    let state = app
        .world()
        .resource::<super::super::lifecycle::ExteriorStreamState>();
    assert!(state.cells[&grid].root.is_some());
    assert!(state.cells[&grid].package.is_some());
    assert_eq!(state.resident_bytes, 48);
    assert_eq!(state.stale_completions, 1);
    let mut roots = app.world_mut().query::<&super::super::ExteriorCellRoot>();
    assert_eq!(roots.iter(app.world()).count(), 1);
}
