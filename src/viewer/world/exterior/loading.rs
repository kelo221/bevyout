//! Background package reads and generation-checked completion.

use bevy::prelude::*;
use bevy::tasks::futures::check_ready;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevyout_core::manifest::exterior::{EXTERIOR_CELL_PACKAGE_REVISION, ExteriorCellPackage};
use std::fs;
use std::path::Path;

use super::lifecycle::ExteriorStreamState;
use super::spawn_package;

#[derive(Component)]
pub(crate) struct ExteriorPackageTask {
    form_id: u32,
    grid: bevyout_core::manifest::exterior::GridCoordinate,
    generation: u64,
    task: Task<Result<LoadedExteriorPackage, String>>,
}

#[derive(Debug)]
struct LoadedExteriorPackage {
    package: ExteriorCellPackage,
    serialized_bytes: u64,
}

pub(crate) fn request(
    commands: &mut Commands,
    state: &mut ExteriorStreamState,
    grid: bevyout_core::manifest::exterior::GridCoordinate,
    form_id: u32,
    generation: u64,
) -> bool {
    let collision_owned = state.collision_cells.contains_key(&grid);
    let Some(cell) = state.cells.get(&grid) else {
        return false;
    };
    if cell.state.cell_form_id != form_id
        || cell.state.generation != generation
        || cell.state.lifecycle != bevyout_core::manifest::exterior::ExteriorCellLifecycle::Queued
        || cell.task.is_some()
        || cell.root.is_some()
        || cell.package.is_some()
        || cell.collision_ready
        || collision_owned
    {
        return false;
    }
    let Some(asset_root) = state.asset_root.clone() else {
        return false;
    };
    let Some(index) = state.index.as_ref() else {
        return false;
    };
    let Some(entry) = index.cell_at(grid) else {
        return false;
    };
    if entry.cell_form_id != form_id {
        return false;
    }
    let path = asset_root.join(
        entry
            .package_path
            .replace('/', std::path::MAIN_SEPARATOR_STR),
    );
    let task = AsyncComputeTaskPool::get().spawn(async move {
        let bytes =
            fs::read(&path).map_err(|error| format!("reading {}: {error}", path.display()))?;
        decode_package(&path, &bytes)
    });
    let task_entity = commands
        .spawn(ExteriorPackageTask {
            form_id,
            grid,
            generation,
            task,
        })
        .id();
    if let Some(cell) = state.cells.get_mut(&grid) {
        cell.task = Some(task_entity);
        cell.state.lifecycle = bevyout_core::manifest::exterior::ExteriorCellLifecycle::Loading;
        cell.state.generation = generation;
    }
    state.requests += 1;
    if state.trace {
        info!(
            "exterior preload start {:08x} grid={},{} generation={}",
            form_id, grid.x, grid.y, generation
        );
    }
    true
}

pub(crate) fn poll(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ExteriorPackageTask)>,
    mut state: ResMut<ExteriorStreamState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for (task_entity, mut pending) in &mut tasks {
        let Some(result) = check_ready(&mut pending.task) else {
            continue;
        };
        commands.entity(task_entity).despawn();
        let pending_is_current = state
            .cells
            .get(&pending.grid)
            .is_some_and(|cell| cell.task == Some(task_entity));
        let collision_owned = state.collision_cells.contains_key(&pending.grid);
        let Some(cell) = state.cells.get_mut(&pending.grid) else {
            state.stale_completions += 1;
            continue;
        };
        if !pending_is_current
            || cell.state.generation != pending.generation
            || cell.state.cell_form_id != pending.form_id
            || cell.state.lifecycle
                != bevyout_core::manifest::exterior::ExteriorCellLifecycle::Loading
        {
            if pending_is_current {
                cell.task = None;
            }
            state.stale_completions += 1;
            continue;
        }
        if cell.root.is_some() || cell.package.is_some() || cell.collision_ready || collision_owned
        {
            cell.task = None;
            state.stale_completions += 1;
            continue;
        }
        match result {
            Ok(loaded)
                if loaded.package.revision == EXTERIOR_CELL_PACKAGE_REVISION
                    && loaded.package.cell_form_id == pending.form_id
                    && loaded.package.grid == pending.grid =>
            {
                let package = loaded.package;
                let estimated_bytes = loaded.serialized_bytes;
                let root = spawn_package(
                    &mut commands,
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                    &package,
                );
                let previous_bytes = cell.state.estimated_bytes;
                {
                    cell.root = Some(root);
                    // Rendering is spawned now, but the package is not
                    // resident until the BoxDDD ownership system attaches
                    // its terrain/object collision on the next chained step.
                    cell.state.lifecycle =
                        bevyout_core::manifest::exterior::ExteriorCellLifecycle::Loading;
                    cell.state.estimated_bytes = estimated_bytes;
                    cell.package = Some(package);
                    cell.task = None;
                    cell.collision_ready = false;
                    cell.eviction_restore = None;
                }
                state.resident_bytes = state
                    .resident_bytes
                    .saturating_sub(previous_bytes)
                    .saturating_add(estimated_bytes);
                state.record_peaks();
                if state.trace {
                    info!(
                        "exterior preload generated {:08x} grid={},{} bytes={} awaiting_collision",
                        pending.form_id, pending.grid.x, pending.grid.y, estimated_bytes
                    );
                }
            }
            Ok(loaded)
                if loaded.package.revision == EXTERIOR_CELL_PACKAGE_REVISION
                    && (loaded.package.cell_form_id != pending.form_id
                        || loaded.package.grid != pending.grid) =>
            {
                cell.state.lifecycle =
                    bevyout_core::manifest::exterior::ExteriorCellLifecycle::Failed;
                cell.state.failed_attempts = cell.state.failed_attempts.saturating_add(1);
                cell.task = None;
                cell.eviction_restore = None;
                state.stale_completions += 1;
                state.failures += 1;
                warn!(
                    "exterior package failed {:08x}: ownership mismatch for grid={},{}",
                    pending.form_id, pending.grid.x, pending.grid.y
                );
            }
            Ok(loaded) => {
                cell.state.lifecycle =
                    bevyout_core::manifest::exterior::ExteriorCellLifecycle::Failed;
                cell.state.failed_attempts = cell.state.failed_attempts.saturating_add(1);
                cell.task = None;
                cell.eviction_restore = None;
                state.failures += 1;
                warn!(
                    "exterior package failed {:08x}: stale revision {}, expected {}",
                    pending.form_id, loaded.package.revision, EXTERIOR_CELL_PACKAGE_REVISION
                );
            }
            Err(error) => {
                cell.state.lifecycle =
                    bevyout_core::manifest::exterior::ExteriorCellLifecycle::Failed;
                cell.state.failed_attempts = cell.state.failed_attempts.saturating_add(1);
                cell.task = None;
                cell.eviction_restore = None;
                state.failures += 1;
                warn!("exterior package failed {:08x}: {error}", pending.form_id);
            }
        }
    }
}

fn decode_package(path: &Path, bytes: &[u8]) -> Result<LoadedExteriorPackage, String> {
    let package = ron::de::from_bytes(bytes)
        .map_err(|error| format!("parsing {}: {error}", path.display()))?;
    Ok(LoadedExteriorPackage {
        package,
        serialized_bytes: u64::try_from(bytes.len()).unwrap_or(u64::MAX),
    })
}

#[cfg(test)]
#[path = "loading_tests.rs"]
mod tests;
