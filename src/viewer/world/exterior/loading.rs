//! Background package reads and generation-checked completion.

use bevy::prelude::*;
use bevy::tasks::futures::check_ready;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevyout_core::manifest::exterior::{EXTERIOR_CELL_PACKAGE_REVISION, ExteriorCellPackage};
use std::fs;

use super::lifecycle::ExteriorStreamState;
use super::spawn_package;

#[derive(Component)]
pub(crate) struct ExteriorPackageTask {
    pub(crate) form_id: u32,
    pub(crate) grid: bevyout_core::manifest::exterior::GridCoordinate,
    pub(crate) generation: u64,
    pub(crate) task: Task<Result<ExteriorCellPackage, String>>,
}

pub(crate) fn request(
    commands: &mut Commands,
    state: &mut ExteriorStreamState,
    grid: bevyout_core::manifest::exterior::GridCoordinate,
    form_id: u32,
    generation: u64,
) {
    let Some(asset_root) = state.asset_root.clone() else {
        return;
    };
    let Some(index) = state.index.as_ref() else {
        return;
    };
    let Some(entry) = index.cell_at(grid) else {
        return;
    };
    let path = asset_root.join(
        entry
            .package_path
            .replace('/', std::path::MAIN_SEPARATOR_STR),
    );
    let task = AsyncComputeTaskPool::get().spawn(async move {
        let bytes =
            fs::read(&path).map_err(|error| format!("reading {}: {error}", path.display()))?;
        ron::de::from_bytes(&bytes).map_err(|error| format!("parsing {}: {error}", path.display()))
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
        let Some(cell) = state.cells.get_mut(&pending.grid) else {
            state.stale_completions += 1;
            continue;
        };
        if cell.state.generation != pending.generation
            || cell.state.cell_form_id != pending.form_id
            || cell.state.lifecycle
                != bevyout_core::manifest::exterior::ExteriorCellLifecycle::Loading
        {
            state.stale_completions += 1;
            continue;
        }
        match result {
            Ok(package) if package.revision == EXTERIOR_CELL_PACKAGE_REVISION => {
                let root = spawn_package(
                    &mut commands,
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                    &package,
                );
                let estimated_bytes = estimate_bytes(&package);
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
                }
                state.resident_bytes = state.resident_bytes.saturating_add(estimated_bytes);
                state.peak_resident_cells = state.peak_resident_cells.max(
                    state
                        .cells
                        .values()
                        .filter(|cell| {
                            matches!(
                                cell.state.lifecycle,
                                bevyout_core::manifest::exterior::ExteriorCellLifecycle::Ready
                                    | bevyout_core::manifest::exterior::ExteriorCellLifecycle::Resident
                            )
                        })
                        .count(),
                );
                state.peak_memory = state.peak_memory.max(state.resident_bytes);
                if state.trace {
                    info!(
                        "exterior preload generated {:08x} grid={},{} bytes={} awaiting_collision",
                        pending.form_id, pending.grid.x, pending.grid.y, estimated_bytes
                    );
                }
            }
            Ok(package) => {
                cell.state.lifecycle =
                    bevyout_core::manifest::exterior::ExteriorCellLifecycle::Failed;
                cell.state.failed_attempts = cell.state.failed_attempts.saturating_add(1);
                cell.task = None;
                state.failures += 1;
                warn!(
                    "exterior package failed {:08x}: stale revision {}, expected {}",
                    pending.form_id, package.revision, EXTERIOR_CELL_PACKAGE_REVISION
                );
            }
            Err(error) => {
                cell.state.lifecycle =
                    bevyout_core::manifest::exterior::ExteriorCellLifecycle::Failed;
                cell.state.failed_attempts = cell.state.failed_attempts.saturating_add(1);
                cell.task = None;
                state.failures += 1;
                warn!("exterior package failed {:08x}: {error}", pending.form_id);
            }
        }
    }
}

fn estimate_bytes(package: &ExteriorCellPackage) -> u64 {
    u64::try_from(ron::ser::to_string(package).map_or(0, |text| text.len())).unwrap_or(u64::MAX)
}
