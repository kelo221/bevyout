//! Prepared exterior package streaming.
//!
//! Runtime accepts only `.ron` packages and GLB asset paths already produced
//! by preparation. No converter, Blender process, or source-plugin parser is
//! reachable from this module.

mod diagnostics;
mod lifecycle;
mod loading;
mod policy;

use std::any::TypeId;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use avian3d::prelude::Collider;
use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::VisibleEntities;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy_boxddd::prelude::BoxdddPhysicsContext;
use bevyout_core::manifest::exterior::{
    EXTERIOR_CELL_PACKAGE_REVISION, EXTERIOR_INDEX_REVISION, ExteriorCellLifecycle,
    ExteriorCellPackage, ExteriorCoordinatePolicy, ExteriorLoadAction, ExteriorResidencyAction,
    ExteriorWaterContact, ExteriorWorldspaceIndex, ExteriorWorldspaceLodAsset, GridCoordinate,
    PreparedTerrain, PreparedWater, TerrainLod, resolve_water_contact, select_terrain_lod,
};
use serde::Serialize;

use super::super::LoadedSceneManifest;
use super::super::player::{
    CellPhysicsReadiness, CollisionRuntimeStats, FpsPlayer, PhysicsDisabled,
    PreparedCollisionWorld, PreparedPhysicsAssets,
};
use crate::app_state::AppState;
use crate::viewer::day_night::GameClock;

pub(crate) use diagnostics::{cells as exterior_cells_json, status as exterior_status_json};
pub(crate) use lifecycle::ExteriorStreamState;

#[derive(Component)]
pub(crate) struct ExteriorCellRoot {
    #[allow(dead_code)]
    pub(crate) form_id: u32,
}

/// Root for visuals owned by the worldspace index rather than a streamed
/// cell.  Its children intentionally survive cell eviction so persistent
/// landmarks do not disappear when their authored cell leaves the residency
/// ring.
#[derive(Component)]
struct ExteriorWorldspaceRoot {
    #[allow(dead_code)]
    worldspace_form_id: u32,
}

#[derive(Resource, Debug, Default, Clone)]
struct ExteriorWorldspaceLodCatalog {
    root: Option<Entity>,
    descriptors: Vec<ExteriorWorldspaceLodAsset>,
}

/// Far-worldspace LOD is presentation-only and opt-in because the authored
/// Fallout archive contains many separate imports. Per-cell terrain LOD is
/// independent and remains enabled by default.
#[derive(Resource, Debug, Clone, Copy)]
pub(crate) struct ExteriorWorldspaceLodSettings {
    pub(crate) enabled: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ExteriorWorldspaceLodVisual {
    level: u8,
    grid: GridCoordinate,
    blocks: bool,
}

#[derive(Component)]
struct ExteriorTerrain;

#[derive(Component)]
struct ExteriorTerrainLod {
    grid: GridCoordinate,
    near: Handle<Mesh>,
    middle: Handle<Mesh>,
    distant: Handle<Mesh>,
    current: TerrainLod,
    center: Vec3,
}

#[derive(Component)]
struct ExteriorWaterSurface {
    descriptor: PreparedWater,
}

#[derive(Component)]
struct ExteriorLocalLight {
    reference_form_id: u32,
    base_intensity: f32,
}

#[derive(Resource, Debug, Clone, Copy)]
struct ExteriorLightBudget {
    max_active: usize,
}

impl Default for ExteriorLightBudget {
    fn default() -> Self {
        Self { max_active: 64 }
    }
}

#[derive(Resource, Debug, Default, Clone, Copy)]
pub(crate) struct ExteriorWaterState {
    pub(crate) contact: Option<ExteriorWaterContact>,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Serialize)]
pub(crate) struct SwimmingState {
    pub(crate) submerged: bool,
    pub(crate) breath_seconds: f32,
    pub(crate) max_breath_seconds: f32,
}

impl Default for SwimmingState {
    fn default() -> Self {
        Self {
            submerged: false,
            breath_seconds: 20.0,
            max_breath_seconds: 20.0,
        }
    }
}

#[derive(Component)]
pub(crate) struct ExteriorReference {
    #[allow(dead_code)]
    pub(crate) reference_form_id: u32,
}

#[derive(Component)]
struct ExteriorObjectLod {
    distant: bool,
    persistent: bool,
    visible: bool,
}

/// Presentation-only counters. The authoritative terrain collision and
/// streamed gameplay state intentionally do not use the selected render LOD
/// or visibility result.
#[derive(Resource, Debug, Default, Clone, Copy, Serialize)]
pub(crate) struct ExteriorPresentationStats {
    pub(crate) terrain_lod_transitions: u64,
    pub(crate) object_lod_transitions: u64,
    pub(crate) worldspace_lod_asset_loads_staged_total: u64,
    pub(crate) worldspace_lod_asset_loads_staged_last_frame: u64,
    pub(crate) worldspace_lod_peak_asset_loads_staged_per_frame: u64,
    pub(crate) worldspace_lod_despawns_total: u64,
}

const TERRAIN_SKIRT_DEPTH: f32 = 8.0;
const NEAR_TERRAIN_SUBDIVISIONS: usize = 4;
// Worldspace LOD NIFs are separate GLB imports. Keep the runtime set bounded
// and spread initial imports over frames so a large LOD archive cannot flood
// Bevy's IO task pool during the first presentation update.
const WORLDSPACE_LOD_MAX_ACTIVE: usize = 48;
const WORLDSPACE_LOD_MAX_SPAWN_PER_FRAME: usize = 8;
const WORLDSPACE_LOD_TERRAIN_BUDGETS: [(u8, usize); 4] = [(4, 16), (8, 12), (16, 8), (32, 4)];
const WORLDSPACE_LOD_BLOCK_BUDGET: usize = 8;

pub(crate) struct ExteriorWorldPlugin {
    pub(crate) resident_cell_limit: usize,
    pub(crate) worldspace_lod: bool,
}

impl Plugin for ExteriorWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExteriorStreamState>()
            .init_resource::<ExteriorWorldspaceLodCatalog>()
            .insert_resource(ExteriorWorldspaceLodSettings {
                enabled: self.worldspace_lod,
            })
            .init_resource::<ExteriorWaterState>()
            .init_resource::<SwimmingState>()
            .init_resource::<ExteriorPresentationStats>()
            .init_resource::<ExteriorLightBudget>()
            .insert_resource(ExteriorStreamBudget {
                resident_cells: self.resident_cell_limit,
                bytes: 128 * 1024 * 1024,
            })
            .add_systems(
                Update,
                (
                    initialize,
                    place_player,
                    update_residency,
                    loading::poll,
                    apply_exterior_persistence,
                    attach_streamed_colliders,
                    finalize_evictions,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );
        app.add_systems(
            Update,
            (update_local_lights, update_water_state)
                .after(super::super::plugins::ViewerSet::WorldSync)
                .run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            Update,
            (
                update_terrain_lod,
                update_exterior_object_lod,
                update_worldspace_lod,
            )
                .after(super::super::plugins::ViewerSet::WorldSync)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Resource, Clone, Copy)]
struct ExteriorStreamBudget {
    resident_cells: usize,
    bytes: u64,
}

#[allow(clippy::too_many_arguments)]
fn initialize(
    mut commands: Commands,
    manifest: Res<LoadedSceneManifest>,
    mut state: ResMut<ExteriorStreamState>,
    mut lod_catalog: ResMut<ExteriorWorldspaceLodCatalog>,
    budget: Res<ExteriorStreamBudget>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player: Query<&mut Transform, With<FpsPlayer>>,
    collision_world: Res<PreparedCollisionWorld>,
    physics_disabled: Res<PhysicsDisabled>,
) {
    if state.initialized {
        return;
    }
    state.resident_budget = budget.resident_cells;
    state.byte_budget = budget.bytes;
    let Some(package) = manifest.exterior.clone() else {
        state.initialized = true;
        return;
    };
    state.initialized = true;
    state.asset_root = Some(Path::new(&manifest.asset_root).to_path_buf());
    state.worldspace_form_id = Some(package.worldspace_form_id);
    state.current_grid = package.grid;
    if let Some(mut transform) = player.iter_mut().next() {
        let current_grid = ExteriorCoordinatePolicy::default().grid_for_bevy([
            f64::from(transform.translation.x),
            f64::from(transform.translation.y),
            f64::from(transform.translation.z),
        ]);
        if current_grid != package.grid {
            transform.translation = exterior_player_spawn(&package);
        }
        state.player_positioned = true;
    }
    let index_path = Path::new(&manifest.asset_root)
        .join("worldspaces")
        .join(format!("{:08x}", package.worldspace_form_id))
        .join("index.ron");
    match fs::read_to_string(&index_path).and_then(|text| {
        ron::from_str::<ExteriorWorldspaceIndex>(&text).map_err(std::io::Error::other)
    }) {
        Ok(mut index) if index.revision == EXTERIOR_INDEX_REVISION => {
            let indexed_cells = index.cells.len();
            index.cells.retain(|cell| {
                let path = Path::new(&manifest.asset_root).join(&cell.package_path);
                path.is_file() && exterior_package_has_current_revision(&path)
            });
            info!(
                "exterior package availability worldspace {:08x}: {}/{} indexed cells",
                index.worldspace_form_id,
                index.cells.len(),
                indexed_cells
            );
            state.index = Some(index);
        }
        Ok(index) => warn!(
            "exterior index stale {}: found {}, expected {}; run `prepare` again",
            index_path.display(),
            index.revision,
            EXTERIOR_INDEX_REVISION
        ),
        Err(error) => warn!(
            "exterior index unavailable {}: {error}",
            index_path.display()
        ),
    }
    let persistent_index = state.index.clone();
    if let Some(index) = persistent_index.as_ref() {
        lod_catalog.root = Some(spawn_worldspace_persistent_objects(
            &mut commands,
            &asset_server,
            index,
        ));
        lod_catalog.descriptors = index.worldspace_lod.clone();
    }
    if package.revision != EXTERIOR_CELL_PACKAGE_REVISION {
        warn!(
            "exterior startup package stale {:08x}: found {}, expected {}; run `prepare` again",
            package.cell_form_id, package.revision, EXTERIOR_CELL_PACKAGE_REVISION
        );
    }
    let root = spawn_package(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        &package,
    );
    let initial_grid = package.grid;
    let collision_ready =
        physics_disabled.0 || collision_world.has_cell_colliders(package.cell_form_id);
    state.cells.insert(
        package.grid,
        lifecycle::RuntimeCell {
            state: bevyout_core::manifest::exterior::ExteriorCellState {
                cell_form_id: package.cell_form_id,
                grid: package.grid,
                lifecycle: ExteriorCellLifecycle::Resident,
                generation: 1,
                pinned: true,
                estimated_bytes: estimate_package_bytes(&package),
                failed_attempts: 0,
            },
            root: Some(root),
            task: None,
            package: Some(package),
            collision_ready,
            eviction_restore: None,
        },
    );
    if collision_ready {
        state.collision_cells.insert(
            initial_grid,
            manifest.exterior.as_ref().unwrap().cell_form_id,
        );
    }
    state.resident_bytes = state
        .cells
        .get(&initial_grid)
        .map(|cell| cell.state.estimated_bytes)
        .unwrap_or_default();
    state.peak_resident_cells = 1;
    state.peak_memory = state.resident_bytes;
    if !collision_ready {
        warn!(
            "exterior startup collision not tracked for grid={},{}; residency remains gated",
            initial_grid.x, initial_grid.y
        );
    }
    info!("exterior resident {:08x}", manifest.cell.form_id);
}

fn exterior_package_has_current_revision(path: &Path) -> bool {
    let Ok(file) = fs::File::open(path) else {
        return false;
    };
    let Ok(lines) = BufReader::new(file)
        .lines()
        .take(4)
        .collect::<Result<Vec<_>, _>>()
    else {
        return false;
    };
    exterior_package_header_has_current_revision(&lines)
}

fn exterior_package_header_has_current_revision(lines: &[String]) -> bool {
    let expected = format!("revision: \"{}\"", EXTERIOR_CELL_PACKAGE_REVISION);
    lines
        .iter()
        .any(|line| line.trim_start().starts_with("revision:") && line.contains(&expected))
}

fn place_player(
    manifest: Res<LoadedSceneManifest>,
    mut state: ResMut<ExteriorStreamState>,
    mut player: Query<&mut Transform, With<FpsPlayer>>,
) {
    if !state.initialized || state.player_positioned {
        return;
    }
    let Some(package) = manifest.exterior.as_ref() else {
        state.player_positioned = true;
        return;
    };
    let Some(mut transform) = player.iter_mut().next() else {
        return;
    };
    let current_grid = ExteriorCoordinatePolicy::default().grid_for_bevy([
        f64::from(transform.translation.x),
        f64::from(transform.translation.y),
        f64::from(transform.translation.z),
    ]);
    if current_grid != package.grid {
        transform.translation = exterior_player_spawn(package);
    }
    state.current_grid = package.grid;
    state.player_positioned = true;
    info!(
        "exterior player placed grid={},{} position={:?}",
        package.grid.x, package.grid.y, transform.translation
    );
}

fn update_residency(
    mut commands: Commands,
    manifest: Res<LoadedSceneManifest>,
    mut state: ResMut<ExteriorStreamState>,
    budget: Res<ExteriorStreamBudget>,
    player: Query<&Transform, With<FpsPlayer>>,
    tasks: Query<&loading::ExteriorPackageTask>,
    mut cell_physics: ResMut<CellPhysicsReadiness>,
) {
    if !state.initialized || !state.player_positioned || state.index.is_none() {
        return;
    }
    let Some(index) = state.index.clone() else {
        return;
    };
    let policy = index.coordinate_policy.clone();
    let current = player
        .iter()
        .next()
        .map(|transform| policy::grid_for_translation(&policy, transform.translation.to_array()))
        .unwrap_or(state.current_grid);
    let target_ready = state
        .cells
        .get(&current)
        .is_some_and(|cell| cell.collision_ready);
    if current != state.current_grid && target_ready {
        state.previous_grid = Some(state.current_grid);
        state.current_grid = current;
        *cell_physics = CellPhysicsReadiness::Ready;
    } else if current != state.current_grid && index.cell_at(current).is_some() {
        // Keep the old logical active cell until the destination package has
        // entered BoxDDD. This freezes fixed-step movement at the seam rather
        // than allowing the player to cross into a render-only neighbor.
        *cell_physics = CellPhysicsReadiness::BuildingStatic;
    }
    // Keep the physical target as the planner's focus so a missing neighbor
    // is requested immediately, but temporarily pin the last collision-ready
    // cell. Otherwise the budget trim can evict the old active terrain before
    // the destination enters BoxDDD.
    let handoff =
        current != state.current_grid && !target_ready && index.cell_at(current).is_some();
    let plan_current = current;
    let velocity_grid = state
        .previous_grid
        .map(|previous| (plan_current.x - previous.x, plan_current.y - previous.y))
        .unwrap_or((0, 0));
    let mut planning_states = state.states();
    if handoff
        && let Some(old) = planning_states
            .iter_mut()
            .find(|cell| cell.grid == state.current_grid)
    {
        old.pinned = true;
    }
    let plan = policy::desired_plan(
        &index,
        plan_current,
        velocity_grid,
        &planning_states,
        budget.resident_cells,
        budget.bytes,
    );
    for action in plan.actions {
        apply_action(
            &mut commands,
            &mut state,
            action,
            &tasks,
            &manifest.asset_root,
        );
    }
}

fn apply_action(
    commands: &mut Commands,
    state: &mut ExteriorStreamState,
    action: ExteriorResidencyAction,
    tasks: &Query<&loading::ExteriorPackageTask>,
    _asset_root: &str,
) {
    if state.trace {
        info!(
            "exterior stream action {:?} grid={},{} form={:08x} generation={}",
            action.action, action.grid.x, action.grid.y, action.form_id, action.generation
        );
    }
    match action.action {
        ExteriorLoadAction::Request => {
            let collision_owned = state.collision_cells.contains_key(&action.grid);
            let can_request = match state.cells.get(&action.grid) {
                None => action.generation == 1 && !collision_owned,
                Some(cell) => {
                    cell.state.cell_form_id == action.form_id
                        && cell.state.lifecycle == ExteriorCellLifecycle::Unloaded
                        && cell.state.generation.saturating_add(1) == action.generation
                        && cell.task.is_none()
                        && !cell.owns_runtime_state(collision_owned)
                }
            };
            if !can_request {
                return;
            }
            if let Some(cell) = state.cells.get_mut(&action.grid) {
                state.resident_bytes = state
                    .resident_bytes
                    .saturating_sub(cell.state.estimated_bytes);
                cell.state.generation = action.generation;
                cell.state.lifecycle = ExteriorCellLifecycle::Queued;
                cell.state.estimated_bytes = 0;
                cell.collision_ready = false;
                cell.eviction_restore = None;
            } else {
                state.cells.insert(
                    action.grid,
                    lifecycle::RuntimeCell {
                        state: bevyout_core::manifest::exterior::ExteriorCellState {
                            cell_form_id: action.form_id,
                            grid: action.grid,
                            lifecycle: ExteriorCellLifecycle::Queued,
                            generation: action.generation,
                            pinned: false,
                            estimated_bytes: 0,
                            failed_attempts: 0,
                        },
                        root: None,
                        task: None,
                        package: None,
                        collision_ready: false,
                        eviction_restore: None,
                    },
                );
            }
            let _ = loading::request(
                commands,
                state,
                action.grid,
                action.form_id,
                action.generation,
            );
        }
        ExteriorLoadAction::Cancel => {
            let collision_owned = state.collision_cells.contains_key(&action.grid);
            let current_grid = state.current_grid;
            if let Some(cell) = state.cells.get_mut(&action.grid) {
                if !action_matches_cell(cell, &action) {
                    return;
                }
                if cell.state.lifecycle == ExteriorCellLifecycle::Evicting {
                    cell.cancel_eviction(current_grid, collision_owned);
                    return;
                }
                if !matches!(
                    cell.state.lifecycle,
                    ExteriorCellLifecycle::Queued
                        | ExteriorCellLifecycle::Loading
                        | ExteriorCellLifecycle::Ready
                        | ExteriorCellLifecycle::Resident
                ) {
                    return;
                }
                despawn_package_task(commands, tasks, cell);
                cell.state.generation = cell.state.generation.saturating_add(1);
                if cell.owns_runtime_state(collision_owned) {
                    cell.eviction_restore = Some(cell.state.lifecycle);
                    cell.state.lifecycle = ExteriorCellLifecycle::Evicting;
                } else {
                    cell.state.lifecycle = ExteriorCellLifecycle::Unloaded;
                    cell.eviction_restore = None;
                    state.resident_bytes = state
                        .resident_bytes
                        .saturating_sub(cell.state.estimated_bytes);
                    cell.state.estimated_bytes = 0;
                }
                state.cancellations += 1;
            }
        }
        ExteriorLoadAction::Evict => {
            if let Some(cell) = state.cells.get_mut(&action.grid) {
                if !action_matches_cell(cell, &action) {
                    return;
                }
                cell.begin_eviction();
                despawn_package_task(commands, tasks, cell);
            }
        }
        ExteriorLoadAction::Activate => {
            let collision_owned = state
                .collision_cells
                .get(&action.grid)
                .is_some_and(|form_id| *form_id == action.form_id);
            if let Some(cell) = state.cells.get_mut(&action.grid) {
                if !action_matches_cell(cell, &action) {
                    return;
                }
                if cell.state.lifecycle == ExteriorCellLifecycle::Resident {
                    return;
                }
                if cell.state.lifecycle == ExteriorCellLifecycle::Ready
                    && cell.collision_ready
                    && collision_owned
                {
                    cell.state.lifecycle = ExteriorCellLifecycle::Resident;
                }
            }
        }
        ExteriorLoadAction::RaisePriority | ExteriorLoadAction::Deactivate => {}
    }
}

fn action_matches_cell(cell: &lifecycle::RuntimeCell, action: &ExteriorResidencyAction) -> bool {
    cell.state.cell_form_id == action.form_id && cell.state.generation == action.generation
}

fn despawn_package_task(
    commands: &mut Commands,
    tasks: &Query<&loading::ExteriorPackageTask>,
    cell: &mut lifecycle::RuntimeCell,
) {
    if let Some(task) = cell.task.take()
        && tasks.get(task).is_ok()
    {
        commands.entity(task).despawn();
    }
}

#[allow(clippy::too_many_arguments)]
fn attach_streamed_colliders(
    mut commands: Commands,
    mut state: ResMut<ExteriorStreamState>,
    physics_disabled: Res<PhysicsDisabled>,
    mut physics_assets: ResMut<PreparedPhysicsAssets>,
    mut collision_world: ResMut<PreparedCollisionWorld>,
    mut stats: ResMut<CollisionRuntimeStats>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    mut restores: ResMut<super::super::world::PersistRestores>,
) {
    let pending = state
        .cells
        .iter()
        .filter(|(_, cell)| {
            cell.state.lifecycle == ExteriorCellLifecycle::Loading
                && cell.task.is_none()
                && cell.package.is_some()
                && !cell.collision_ready
        })
        .map(|(grid, cell)| {
            (
                *grid,
                cell.state.generation,
                cell.package.clone().expect("package checked above"),
            )
        })
        .collect::<Vec<_>>();
    if pending.is_empty() {
        return;
    }

    if physics_disabled.0 {
        for (grid, generation, package) in pending {
            mark_collision_ready(&mut state, grid, package.cell_form_id, generation);
        }
        return;
    }
    let Some(static_body) = collision_world.static_body() else {
        return;
    };
    let Some(boxddd_world) = context.world_mut() else {
        return;
    };
    let asset_root = state.asset_root.clone().unwrap_or_default();
    let mut changed_static_tree = false;
    let mut completed = Vec::with_capacity(pending.len());
    for (grid, generation, package) in pending {
        if !collision_world.has_cell_colliders(package.cell_form_id) {
            changed_static_tree |= super::super::player::build_exterior_static_colliders(
                boxddd_world,
                &mut commands,
                &package,
                &asset_root,
                &mut physics_assets,
                static_body,
                &mut collision_world,
                &mut stats,
                &mut restores,
            );
        }
        let collision_expected = package
            .terrain
            .as_ref()
            .is_some_and(|terrain| terrain.is_well_formed())
            || package
                .static_objects
                .iter()
                .any(|object| object.initially_enabled && object.physics_asset_path.is_some());
        if collision_expected && !collision_world.has_cell_colliders(package.cell_form_id) {
            continue;
        }
        completed.push((grid, package.cell_form_id, generation));
    }
    if changed_static_tree && let Err(error) = boxddd_world.try_rebuild_static_tree() {
        warn!("streamed exterior static tree rebuild returned error: {error:?}");
        return;
    }
    for (grid, form_id, generation) in completed {
        mark_collision_ready(&mut state, grid, form_id, generation);
    }
}

fn mark_collision_ready(
    state: &mut ExteriorStreamState,
    grid: GridCoordinate,
    form_id: u32,
    generation: u64,
) {
    let current = state.current_grid;
    let collision_owner = state.collision_cells.get(&grid).copied();
    if let Some(cell) = state.cells.get_mut(&grid) {
        if cell.state.generation != generation
            || cell.state.cell_form_id != form_id
            || cell.state.lifecycle != ExteriorCellLifecycle::Loading
            || cell.task.is_some()
            || cell.root.is_none()
            || cell.collision_ready
            || collision_owner.is_some_and(|owner| owner != form_id)
        {
            return;
        }
        cell.collision_ready = true;
        cell.state.lifecycle = if grid == current {
            ExteriorCellLifecycle::Resident
        } else {
            ExteriorCellLifecycle::Ready
        };
        state.collision_cells.insert(grid, form_id);
        state.ready += 1;
        state.record_peaks();
        if state.trace {
            info!(
                "exterior preload ready {:08x} grid={},{} collision_ready=1",
                form_id, grid.x, grid.y
            );
        }
    }
}

fn apply_exterior_persistence(world: &mut World) {
    let pending = {
        let state = world.resource::<ExteriorStreamState>();
        state
            .cells
            .iter()
            .filter_map(|(grid, cell)| {
                (!state.persistence_applied.contains(grid)).then_some((
                    *grid,
                    cell.state.cell_form_id,
                    cell.root,
                    cell.package.clone(),
                ))
            })
            .collect::<Vec<_>>()
    };
    for (grid, cell_form_id, root, package) in pending {
        let Some(root) = root else {
            continue;
        };
        let Some(package) = package else {
            continue;
        };
        super::super::world::apply_exterior_cell_state(world, cell_form_id, root, &package);
        world
            .resource_mut::<ExteriorStreamState>()
            .persistence_applied
            .insert(grid);
    }
}

/// Eviction deliberately has a final exclusive phase. BoxDDD shapes are
/// released before the package root and ownership record disappear, so an
/// old terrain cell cannot remain in the shared static tree after streaming.
fn finalize_evictions(world: &mut World) {
    let pending = world
        .resource::<ExteriorStreamState>()
        .cells
        .iter()
        .filter(|(_, cell)| cell.state.lifecycle == ExteriorCellLifecycle::Evicting)
        .map(|(grid, cell)| {
            (
                *grid,
                cell.state.cell_form_id,
                cell.root,
                cell.state.estimated_bytes,
            )
        })
        .collect::<Vec<_>>();
    if pending.is_empty() {
        return;
    }
    let mut rebuilt_static_tree = false;
    for (grid, form_id, root, _) in &pending {
        let package = world
            .resource::<ExteriorStreamState>()
            .cells
            .get(grid)
            .and_then(|cell| cell.package.clone());
        if let (Some(root), Some(package)) = (root, package) {
            super::super::world::capture_exterior_cell_state(world, *form_id, *root, &package);
        }
        let owned_form_id = world
            .resource::<ExteriorStreamState>()
            .collision_cells
            .get(grid)
            .copied();
        if let Some(owned_form_id) = owned_form_id {
            super::super::player::teardown_cell_colliders(world, owned_form_id);
            world
                .resource_mut::<ExteriorStreamState>()
                .collision_cells
                .remove(grid);
            rebuilt_static_tree = true;
        }
        if let Some(root) = root.and_then(|entity| world.get_entity_mut(entity).ok()) {
            root.despawn();
        }
        let mut state = world.resource_mut::<ExteriorStreamState>();
        if let Some(cell) = state.cells.remove(grid) {
            state.resident_bytes = state
                .resident_bytes
                .saturating_sub(cell.state.estimated_bytes);
            state.evictions += 1;
            state.persistence_applied.remove(grid);
        } else {
            warn!("exterior eviction lost cell {:08x}", form_id);
        }
    }
    if rebuilt_static_tree
        && let Some(boxddd_world) = world.non_send_mut::<BoxdddPhysicsContext>().world_mut()
        && let Err(error) = boxddd_world.try_rebuild_static_tree()
    {
        warn!("exterior eviction static tree rebuild returned error: {error:?}");
    }
}

pub(crate) fn spawn_package(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    package: &ExteriorCellPackage,
) -> Entity {
    let root = commands
        .spawn((
            ExteriorCellRoot {
                form_id: package.cell_form_id,
            },
            Transform::default(),
            Visibility::Inherited,
        ))
        .id();
    if let Some(terrain_data) = package.terrain.as_ref()
        && let Some(terrain) = terrain_mesh_with_stride(terrain_data, 1)
    {
        let collider = terrain_collider(terrain_data);
        let near_handle = meshes.add(
            terrain_mesh_with_subdivisions(terrain_data, NEAR_TERRAIN_SUBDIVISIONS)
                .unwrap_or(terrain),
        );
        let middle_handle = terrain_mesh_with_stride(terrain_data, 1)
            .map(|mesh| meshes.add(mesh))
            .unwrap_or_else(|| near_handle.clone());
        let distant_handle = terrain_mesh_with_stride(terrain_data, 2)
            .map(|mesh| meshes.add(mesh))
            .unwrap_or_else(|| middle_handle.clone());
        let normal_map = terrain_data.normal_asset_path.as_deref().map(|path| {
            asset_server
                .load_builder()
                .with_settings(|settings: &mut bevy::image::ImageLoaderSettings| {
                    settings.is_srgb = false;
                })
                .load(path.to_owned())
        });
        let entity = commands
            .spawn((
                Mesh3d(near_handle.clone()),
                MeshMaterial3d(
                    materials.add(StandardMaterial {
                        base_color: Color::WHITE,
                        base_color_texture: terrain_data
                            .albedo_asset_path
                            .as_deref()
                            .map(|path| asset_server.load(path.to_owned())),
                        normal_map_texture: normal_map.clone(),
                        specular_texture: normal_map,
                        flip_normal_map_y: false,
                        // LAND normal alpha carries source specular data, but
                        // the terrain is still a matte dielectric surface.
                        // Maxing the dielectric F0 makes the high-frequency
                        // normal detail sparkle like polished metal.
                        reflectance: 0.25,
                        perceptual_roughness: 1.0,
                        ..default()
                    }),
                ),
                ExteriorTerrain,
                ExteriorTerrainLod {
                    grid: package.grid,
                    near: near_handle,
                    middle: middle_handle,
                    distant: distant_handle,
                    current: TerrainLod::Near,
                    center: terrain_center(package.terrain.as_ref(), package.origin),
                },
                ChildOf(root),
            ))
            .id();
        if let Some(collider) = collider {
            commands.entity(entity).insert(collider);
        }
    }
    for object in package
        .static_objects
        .iter()
        .chain(package.dynamic_objects.iter())
        .chain(package.distant_objects.iter())
        .filter(|object| object.initially_enabled)
    {
        let Some(path) = object.asset_path.clone() else {
            continue;
        };
        let handle = asset_server.load::<WorldAsset>(GltfAssetLabel::Scene(0).from_asset(path));
        commands.spawn((
            WorldAssetRoot(handle),
            ExteriorReference {
                reference_form_id: object.reference_form_id,
            },
            ExteriorObjectLod {
                distant: object.distant,
                persistent: false,
                visible: true,
            },
            Transform {
                translation: Vec3::from_array(object.position),
                rotation: Quat::from_xyzw(
                    object.rotation_xyzw[0],
                    object.rotation_xyzw[1],
                    object.rotation_xyzw[2],
                    object.rotation_xyzw[3],
                ),
                scale: Vec3::splat(object.scale),
            },
            Visibility::Inherited,
            ChildOf(root),
        ));
    }
    for light in &package.local_lights {
        commands.spawn((
            PointLight {
                color: Color::srgba(
                    light.color_rgba[0],
                    light.color_rgba[1],
                    light.color_rgba[2],
                    light.color_rgba[3],
                ),
                range: light.range,
                intensity: light.range * light.range * 2.0,
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_translation(Vec3::from_array(light.position)),
            ExteriorLocalLight {
                reference_form_id: light.reference_form_id,
                base_intensity: light.range * light.range * 2.0,
            },
            ChildOf(root),
        ));
    }
    if let Some(water) = package.water.as_ref() {
        let span = ExteriorCoordinatePolicy::default().cell_span_metres() as f32;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(span, 0.05, span))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.06, 0.18, 0.26, 0.55),
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 0.1,
                ..default()
            })),
            ExteriorWaterSurface {
                descriptor: water.clone(),
            },
            Transform::from_xyz(
                package.origin[0] + span * 0.5,
                water.height,
                package.origin[2] - span * 0.5,
            ),
            ChildOf(root),
        ));
    }
    root
}

fn spawn_worldspace_persistent_objects(
    commands: &mut Commands,
    asset_server: &AssetServer,
    index: &ExteriorWorldspaceIndex,
) -> Entity {
    let root = commands
        .spawn((
            ExteriorWorldspaceRoot {
                worldspace_form_id: index.worldspace_form_id,
            },
            Transform::default(),
            Visibility::Inherited,
        ))
        .id();
    let mut spawned = 0_usize;
    for reference in &index.persistent_references {
        if !reference.initially_enabled {
            continue;
        }
        let Some(path) = reference.asset_path.clone() else {
            continue;
        };
        let handle = asset_server.load::<WorldAsset>(GltfAssetLabel::Scene(0).from_asset(path));
        commands.spawn((
            WorldAssetRoot(handle),
            ExteriorReference {
                reference_form_id: reference.reference_form_id,
            },
            ExteriorObjectLod {
                distant: reference.distant,
                persistent: true,
                visible: true,
            },
            Transform {
                translation: Vec3::from_array(reference.position),
                rotation: Quat::from_xyzw(
                    reference.rotation_xyzw[0],
                    reference.rotation_xyzw[1],
                    reference.rotation_xyzw[2],
                    reference.rotation_xyzw[3],
                ),
                scale: Vec3::splat(reference.scale),
            },
            Visibility::Inherited,
            ChildOf(root),
        ));
        spawned += 1;
    }
    info!(
        "exterior worldspace persistent visuals {:08x} spawned={spawned}",
        index.worldspace_form_id
    );
    root
}

fn terrain_mesh_with_stride(terrain: &PreparedTerrain, stride: usize) -> Option<Mesh> {
    if !terrain.is_well_formed() || terrain.width < 2 || terrain.height < 2 {
        return None;
    }
    let width = usize::from(terrain.width);
    let height = usize::from(terrain.height);
    let stride = stride.max(1);
    let columns = (width - 1) / stride + 1;
    let rows = (height - 1) / stride + 1;
    let mut positions = Vec::with_capacity(columns * rows);
    let mut normals = Vec::with_capacity(columns * rows);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(columns * rows);
    let mut tangents = Vec::with_capacity(columns * rows);
    let mut uvs = Vec::with_capacity(columns * rows);
    for y in (0..height).step_by(stride) {
        for x in (0..width).step_by(stride) {
            let source = y * width + x;
            positions.push(terrain.positions[source]);
            normals.push(terrain.normals[source]);
            let color = terrain.colors[source];
            colors.push([
                f32::from(color[0]) / 255.0,
                f32::from(color[1]) / 255.0,
                f32::from(color[2]) / 255.0,
                f32::from(color[3]) / 255.0,
            ]);
            // LAND UVs advance in +X and -Bevy-Z, so +X is the tangent and
            // its handed bit reconstructs the +Y surface bitangent.
            tangents.push([1.0, 0.0, 0.0, 1.0]);
            uvs.push([
                x as f32 / (width - 1) as f32,
                y as f32 / (height - 1) as f32,
            ]);
        }
    }
    let mut indices = Vec::with_capacity((columns - 1) * (rows - 1) * 6);
    for y in 0..rows - 1 {
        for x in 0..columns - 1 {
            let i = (y * columns + x) as u32;
            let next = i + columns as u32;
            // Rows advance toward negative Bevy Z.  This order keeps the
            // rendered LAND faces counter-clockwise from above, matching the
            // upward normals stored by `terrain_from_land`.
            indices.extend_from_slice(&[i, i + 1, next, i + 1, next + 1, next]);
        }
    }
    let mut buffers = TerrainMeshBuffers {
        positions: &mut positions,
        normals: &mut normals,
        colors: &mut colors,
        tangents: &mut tangents,
        uvs: &mut uvs,
        indices: &mut indices,
    };
    append_terrain_skirts(&mut buffers, columns, rows);
    Some(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, tangents)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices)),
    )
}

/// Build the near presentation from the authored LAND samples without
/// changing the gameplay surface. FO3 LAND stores 33x33 height samples per
/// cell; bilinear subdivision removes the large visual facets while keeping
/// every source sample and every cell border at the same world position.
fn terrain_mesh_with_subdivisions(terrain: &PreparedTerrain, subdivisions: usize) -> Option<Mesh> {
    if !terrain.is_well_formed() || terrain.width < 2 || terrain.height < 2 {
        return None;
    }
    let width = usize::from(terrain.width);
    let height = usize::from(terrain.height);
    let subdivisions = subdivisions.max(1);
    let columns = (width - 1) * subdivisions + 1;
    let rows = (height - 1) * subdivisions + 1;
    let mut positions = Vec::with_capacity(columns * rows);
    let mut normals = Vec::with_capacity(columns * rows);
    let mut colors = Vec::with_capacity(columns * rows);
    let mut tangents = Vec::with_capacity(columns * rows);
    let mut uvs = Vec::with_capacity(columns * rows);
    for y in 0..rows {
        let source_y = y as f32 / subdivisions as f32;
        let y0 = source_y.floor() as usize;
        let y1 = (y0 + 1).min(height - 1);
        let ty = source_y.fract();
        for x in 0..columns {
            let source_x = x as f32 / subdivisions as f32;
            let x0 = source_x.floor() as usize;
            let x1 = (x0 + 1).min(width - 1);
            let tx = source_x.fract();
            let sample = |values: &[[f32; 3]]| {
                let at = |x: usize, y: usize| values[y * width + x];
                let a = at(x0, y0);
                let b = at(x1, y0);
                let c = at(x0, y1);
                let d = at(x1, y1);
                std::array::from_fn(|channel| {
                    let top = a[channel] * (1.0 - tx) + b[channel] * tx;
                    let bottom = c[channel] * (1.0 - tx) + d[channel] * tx;
                    top * (1.0 - ty) + bottom * ty
                })
            };
            let sample_color = |values: &[[u8; 4]]| -> [u8; 4] {
                let at = |x: usize, y: usize| values[y * width + x];
                let a = at(x0, y0);
                let b = at(x1, y0);
                let c = at(x0, y1);
                let d = at(x1, y1);
                std::array::from_fn(|channel| {
                    let top = f32::from(a[channel]) * (1.0 - tx) + f32::from(b[channel]) * tx;
                    let bottom = f32::from(c[channel]) * (1.0 - tx) + f32::from(d[channel]) * tx;
                    (top * (1.0 - ty) + bottom * ty).round().clamp(0.0, 255.0) as u8
                })
            };
            positions.push(sample(&terrain.positions));
            normals.push(normalize_mesh_vector(sample(&terrain.normals)));
            let color = sample_color(&terrain.colors);
            colors.push(std::array::from_fn(|channel| {
                f32::from(color[channel]) / 255.0
            }));
            tangents.push([1.0, 0.0, 0.0, 1.0]);
            uvs.push([
                x as f32 / (columns - 1) as f32,
                y as f32 / (rows - 1) as f32,
            ]);
        }
    }
    let mut indices = Vec::with_capacity((columns - 1) * (rows - 1) * 6);
    for y in 0..rows - 1 {
        for x in 0..columns - 1 {
            let i = (y * columns + x) as u32;
            let next = i + columns as u32;
            indices.extend_from_slice(&[i, i + 1, next, i + 1, next + 1, next]);
        }
    }
    let mut buffers = TerrainMeshBuffers {
        positions: &mut positions,
        normals: &mut normals,
        colors: &mut colors,
        tangents: &mut tangents,
        uvs: &mut uvs,
        indices: &mut indices,
    };
    append_terrain_skirts(&mut buffers, columns, rows);
    Some(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, tangents)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices)),
    )
}

fn normalize_mesh_vector(value: [f32; 3]) -> [f32; 3] {
    let length = (value[0] * value[0] + value[1] * value[1] + value[2] * value[2]).sqrt();
    if length.is_finite() && length > f32::EPSILON {
        [value[0] / length, value[1] / length, value[2] / length]
    } else {
        [0.0, 1.0, 0.0]
    }
}

struct TerrainMeshBuffers<'a> {
    positions: &'a mut Vec<[f32; 3]>,
    normals: &'a mut Vec<[f32; 3]>,
    colors: &'a mut Vec<[f32; 4]>,
    tangents: &'a mut Vec<[f32; 4]>,
    uvs: &'a mut Vec<[f32; 2]>,
    indices: &'a mut Vec<u32>,
}

fn append_terrain_skirts(buffers: &mut TerrainMeshBuffers<'_>, columns: usize, rows: usize) {
    let TerrainMeshBuffers {
        positions,
        normals,
        colors,
        tangents,
        uvs,
        indices,
    } = buffers;
    let mut add_segment = |top_a: usize, top_b: usize| {
        let lower_a = positions[top_a];
        let lower_b = positions[top_b];
        let lower_a_index = positions.len() as u32;
        positions.push([lower_a[0], lower_a[1] - TERRAIN_SKIRT_DEPTH, lower_a[2]]);
        positions.push([lower_b[0], lower_b[1] - TERRAIN_SKIRT_DEPTH, lower_b[2]]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        colors.push(colors[top_a]);
        colors.push(colors[top_b]);
        tangents.push([1.0, 0.0, 0.0, 1.0]);
        tangents.push([1.0, 0.0, 0.0, 1.0]);
        uvs.push(uvs[top_a]);
        uvs.push(uvs[top_b]);
        indices.extend_from_slice(&[
            top_a as u32,
            top_b as u32,
            lower_a_index + 1,
            top_a as u32,
            lower_a_index + 1,
            lower_a_index,
        ]);
    };

    for x in 0..columns.saturating_sub(1) {
        add_segment(x, x + 1);
        let bottom = (rows - 1) * columns + x;
        add_segment(bottom + 1, bottom);
    }
    for y in 0..rows.saturating_sub(1) {
        let top = y * columns;
        add_segment(top + columns, top);
        let right = y * columns + columns - 1;
        add_segment(right, right + columns);
    }
}

fn terrain_collider(terrain: &PreparedTerrain) -> Option<Collider> {
    if !terrain.is_well_formed() || terrain.width < 2 || terrain.height < 2 {
        return None;
    }
    let width = usize::from(terrain.width);
    let height = usize::from(terrain.height);
    let mut indices = Vec::with_capacity((width - 1) * (height - 1) * 2);
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let i = (y * width + x) as u32;
            let next = i + width as u32;
            indices.extend_from_slice(&[[i, i + 1, next], [i + 1, next + 1, next]]);
        }
    }
    Some(Collider::trimesh(
        terrain
            .positions
            .iter()
            .map(|position| Vec3::from_array(*position))
            .collect(),
        indices,
    ))
}

fn estimate_package_bytes(package: &ExteriorCellPackage) -> u64 {
    u64::try_from(ron::ser::to_string(package).map_or(0, |text| text.len())).unwrap_or(u64::MAX)
}

fn exterior_player_spawn(package: &ExteriorCellPackage) -> Vec3 {
    terrain_center(package.terrain.as_ref(), package.origin)
        + Vec3::Y * super::super::player::CAPSULE_HEIGHT.mul_add(0.5, 0.2)
}

fn terrain_center(terrain: Option<&PreparedTerrain>, origin: [f32; 3]) -> Vec3 {
    terrain
        .filter(|terrain| terrain.is_well_formed())
        .and_then(|terrain| {
            let x = usize::from(terrain.width) / 2;
            let y = usize::from(terrain.height) / 2;
            terrain.positions.get(y * usize::from(terrain.width) + x)
        })
        .copied()
        .map(Vec3::from_array)
        .unwrap_or_else(|| {
            let span = ExteriorCoordinatePolicy::default().cell_span_metres() as f32;
            Vec3::new(origin[0] + span * 0.5, origin[1], origin[2] - span * 0.5)
        })
}

#[allow(clippy::type_complexity)]
fn update_local_lights(
    clock: Res<GameClock>,
    player: Query<&Transform, With<FpsPlayer>>,
    budget: Res<ExteriorLightBudget>,
    mut queries: ParamSet<(
        Query<(Entity, &Transform, &ExteriorLocalLight)>,
        Query<(
            Entity,
            &mut PointLight,
            &mut Visibility,
            &ExteriorLocalLight,
        )>,
    )>,
) {
    let daylight = (((clock.hour - 6.0) / 12.0) * std::f32::consts::PI)
        .sin()
        .clamp(0.0, 1.0);
    let factor = 0.35 + daylight * 0.65;
    let player_position = player
        .iter()
        .next()
        .map_or(Vec3::ZERO, |transform| transform.translation);
    let mut ranked = {
        let lights = queries.p0();
        lights
            .iter()
            .map(|(entity, transform, authored)| {
                (
                    entity,
                    player_position.distance_squared(transform.translation),
                    authored.reference_form_id,
                )
            })
            .collect::<Vec<_>>()
    };
    ranked.sort_by(|left, right| {
        left.1
            .total_cmp(&right.1)
            .then_with(|| left.2.cmp(&right.2))
            .then_with(|| left.0.index().cmp(&right.0.index()))
    });
    let active = ranked
        .iter()
        .take(budget.max_active)
        .map(|(entity, _, _)| *entity)
        .collect::<HashSet<_>>();
    let mut lights = queries.p1();
    for (entity, mut light, mut visibility, authored) in &mut lights {
        if active.contains(&entity) {
            light.intensity = authored.base_intensity * (1.15 - factor * 0.55);
            *visibility = Visibility::Inherited;
        } else {
            light.intensity = 0.0;
            *visibility = Visibility::Hidden;
        }
    }
}

pub(crate) fn streamed_lights_json(world: &mut World) -> serde_json::Value {
    let budget = world
        .get_resource::<ExteriorLightBudget>()
        .map_or(64, |budget| budget.max_active);
    let mut query = world.query::<(&ExteriorLocalLight, Option<&Visibility>)>();
    let mut references = Vec::new();
    let mut active = 0_usize;
    for (light, visibility) in query.iter(world) {
        references.push(light.reference_form_id);
        if !matches!(visibility, Some(Visibility::Hidden)) {
            active += 1;
        }
    }
    references.sort_unstable();
    serde_json::json!({
        "active": active,
        "total": references.len(),
        "budget": budget,
        "references": references,
    })
}

/// Return the presentation state that can be measured without confusing it
/// with gameplay residency. Bevy's main-world `VisibleEntities` list is the
/// authoritative CPU visibility result for each active camera, so the report
/// can expose its mesh counts. GPU occlusion remains intentionally unmeasured:
/// the render-world/GPU path does not expose a stable count to this console
/// snapshot. The distance-cull count remains useful and conservative: hidden
/// presentation roots never remove collision, navigation, or persistent
/// simulation.
pub(crate) fn exterior_presentation_json(world: &mut World) -> serde_json::Value {
    let mut terrain_counts = [0usize; 3];
    let mut terrain_grids = Vec::new();
    {
        let mut query = world.query::<&ExteriorTerrainLod>();
        for lod in query.iter(world) {
            let index = match lod.current {
                TerrainLod::Near => 0,
                TerrainLod::Middle => 1,
                TerrainLod::Distant => 2,
            };
            terrain_counts[index] += 1;
            terrain_grids.push((lod.grid, format!("{:?}", lod.current).to_ascii_lowercase()));
        }
    }
    terrain_grids.sort_unstable_by_key(|(grid, _)| (grid.y, grid.x));

    let mut object_total = 0usize;
    let mut object_visible = 0usize;
    let mut object_hidden = 0usize;
    let mut object_distant = 0usize;
    let mut object_persistent = 0usize;
    let mut distance_culled = 0usize;
    {
        let mut query = world.query::<(&ExteriorObjectLod, &Visibility)>();
        for (lod, visibility) in query.iter(world) {
            object_total += 1;
            let visible = *visibility != Visibility::Hidden && lod.visible;
            if visible {
                object_visible += 1;
            } else {
                object_hidden += 1;
            }
            if lod.distant {
                object_distant += 1;
            }
            if lod.persistent {
                object_persistent += 1;
            }
            if !visible && !lod.distant && !lod.persistent {
                distance_culled += 1;
            }
        }
    }

    let occlusion_cameras = world
        .query_filtered::<Entity, (
            With<Camera3d>,
            With<bevy::render::occlusion_culling::OcclusionCulling>,
        )>()
        .iter(world)
        .count();
    let mut frustum_visible_meshes = HashSet::new();
    let mut frustum_cameras = 0usize;
    {
        let mut query = world.query_filtered::<(&Camera, &VisibleEntities), With<Camera3d>>();
        for (camera, visible_entities) in query.iter(world) {
            if !camera.is_active {
                continue;
            }
            frustum_cameras += 1;
            frustum_visible_meshes.extend(visible_entities.iter(TypeId::of::<Mesh3d>()).copied());
        }
    }
    let frustum_candidate_meshes = world
        .query_filtered::<Entity, With<Mesh3d>>()
        .iter(world)
        .count();
    let frustum_measured = frustum_cameras > 0;
    let frustum_visible_meshes = frustum_measured.then_some(frustum_visible_meshes.len());
    let frustum_culled_meshes =
        frustum_visible_meshes.map(|visible| frustum_candidate_meshes.saturating_sub(visible));
    let stats = world
        .get_resource::<ExteriorPresentationStats>()
        .copied()
        .unwrap_or_default();
    let catalog_duplicate_instances = world
        .get_resource::<ExteriorWorldspaceLodCatalog>()
        .map(|catalog| {
            let mut seen = HashSet::new();
            catalog
                .descriptors
                .iter()
                .map(|descriptor| ExteriorWorldspaceLodVisual {
                    level: descriptor.level,
                    grid: descriptor.grid,
                    blocks: descriptor.blocks,
                })
                .filter(|key| !seen.insert(*key))
                .count()
        })
        .unwrap_or_default();
    let mut worldspace_lod_active = 0usize;
    let mut worldspace_lod_terrain = 0usize;
    let mut worldspace_lod_blocks = 0usize;
    let mut worldspace_lod_levels = BTreeMap::<u8, usize>::new();
    let mut active_worldspace_lod_keys = HashSet::new();
    let mut active_duplicate_instances = 0usize;
    {
        let mut query = world.query::<&ExteriorWorldspaceLodVisual>();
        for lod in query.iter(world) {
            worldspace_lod_active += 1;
            if lod.blocks {
                worldspace_lod_blocks += 1;
            } else {
                worldspace_lod_terrain += 1;
            }
            *worldspace_lod_levels.entry(lod.level).or_default() += 1;
            if !active_worldspace_lod_keys.insert(*lod) {
                active_duplicate_instances += 1;
            }
        }
    }

    serde_json::json!({
        "terrain": {
            "resident": terrain_counts.iter().sum::<usize>(),
            "near": terrain_counts[0],
            "middle": terrain_counts[1],
            "distant": terrain_counts[2],
            "lod_transitions": stats.terrain_lod_transitions,
            "collision": "full_land_mesh",
            "grids": terrain_grids,
        },
        "objects": {
            "total": object_total,
            "visible": object_visible,
            "hidden": object_hidden,
            "distance_culled": distance_culled,
            "distant": object_distant,
            "persistent": object_persistent,
            "lod_transitions": stats.object_lod_transitions,
        },
        "worldspace_lod": {
            "active": worldspace_lod_active,
            "terrain": worldspace_lod_terrain,
            "blocks": worldspace_lod_blocks,
            "levels": worldspace_lod_levels,
            "catalog_duplicate_instances": catalog_duplicate_instances,
            "active_duplicate_instances": active_duplicate_instances,
            "asset_loads_staged_total": stats.worldspace_lod_asset_loads_staged_total,
            "asset_loads_staged_last_frame": stats.worldspace_lod_asset_loads_staged_last_frame,
            "peak_asset_loads_staged_per_frame": stats
                .worldspace_lod_peak_asset_loads_staged_per_frame,
            "asset_loads_staged_per_frame_cap": WORLDSPACE_LOD_MAX_SPAWN_PER_FRAME,
            "despawns_total": stats.worldspace_lod_despawns_total,
            "selection_transitions": stats
                .worldspace_lod_asset_loads_staged_total
                .saturating_add(stats.worldspace_lod_despawns_total),
            "presentation_only": true,
        },
        "culling": {
            "frustum": {
                "cameras": frustum_cameras,
                "measured": frustum_measured,
                "candidate_meshes": frustum_candidate_meshes,
                "visible_meshes": frustum_visible_meshes,
                "culled": frustum_culled_meshes,
            },
            "distance": {
                "measured": true,
                "culled": distance_culled,
            },
            "occlusion": {
                "enabled": occlusion_cameras > 0,
                "cameras": occlusion_cameras,
                "measured": false,
                "culled": serde_json::Value::Null,
                "fallback": "conservative_visibility",
            },
        },
        "gameplay": {
            "presentation_only": true,
            "collision_and_navigation_culled": false,
            "persistent_landmarks_survive_cell_eviction": true,
        },
    })
}

fn update_water_state(
    player: Query<&Transform, With<FpsPlayer>>,
    waters: Query<(&ExteriorWaterSurface, &Transform)>,
    mut state: ResMut<ExteriorWaterState>,
) {
    let Some(player) = player.iter().next() else {
        state.contact = None;
        return;
    };
    let half_span = ExteriorCoordinatePolicy::default().cell_span_metres() as f32 * 0.5;
    state.contact = waters
        .iter()
        .filter(|(_, transform)| {
            (player.translation.x - transform.translation.x).abs() <= half_span
                && (player.translation.z - transform.translation.z).abs() <= half_span
        })
        .filter_map(|(water, _)| {
            resolve_water_contact(Some(&water.descriptor), player.translation.y)
        })
        .max_by(|left, right| left.depth.total_cmp(&right.depth));
}

#[allow(clippy::type_complexity)]
fn update_terrain_lod(
    player: Query<&Transform, With<FpsPlayer>>,
    mut presentation: ResMut<ExteriorPresentationStats>,
    mut terrain: ParamSet<(
        Query<&ExteriorTerrainLod>,
        Query<(&mut Mesh3d, &mut ExteriorTerrainLod)>,
    )>,
) {
    let Some(player) = player.iter().next() else {
        return;
    };
    let mut selected_by_grid = BTreeMap::new();
    for lod in terrain.p0().iter() {
        let distance = player.translation.distance(lod.center);
        let selected = select_terrain_lod(distance, Some(lod.current), 80.0, 180.0, 8.0);
        selected_by_grid.insert(lod.grid, selected);
    }

    clamp_adjacent_terrain_lods(&mut selected_by_grid);

    for (mut mesh, mut lod) in terrain.p1().iter_mut() {
        let Some(selected) = selected_by_grid.get(&lod.grid).copied() else {
            continue;
        };
        if selected == lod.current {
            continue;
        }
        mesh.0 = match selected {
            TerrainLod::Near => lod.near.clone(),
            TerrainLod::Middle => lod.middle.clone(),
            TerrainLod::Distant => lod.distant.clone(),
        };
        lod.current = selected;
        presentation.terrain_lod_transitions =
            presentation.terrain_lod_transitions.saturating_add(1);
    }
}

fn clamp_adjacent_terrain_lods(selected_by_grid: &mut BTreeMap<GridCoordinate, TerrainLod>) {
    // Clamp only cardinal neighbours.  Iterate to a fixed point because a
    // tile can be adjusted by one already-visited neighbour and thereby
    // expose a larger delta on another edge visited earlier in the sweep.
    loop {
        let mut changed = false;
        let grids = selected_by_grid.keys().copied().collect::<Vec<_>>();
        for grid in grids {
            for neighbour in [
                GridCoordinate::new(grid.x + 1, grid.y),
                GridCoordinate::new(grid.x, grid.y + 1),
            ] {
                let Some(left) = selected_by_grid.get(&grid).copied() else {
                    continue;
                };
                let Some(right) = selected_by_grid.get(&neighbour).copied() else {
                    continue;
                };
                let clamped = bevyout_core::manifest::exterior::clamp_lod_delta(left, right);
                if clamped != (left, right) {
                    changed = true;
                    selected_by_grid.insert(grid, clamped.0);
                    selected_by_grid.insert(neighbour, clamped.1);
                }
            }
        }
        if !changed {
            break;
        }
    }
}

fn update_exterior_object_lod(
    player: Query<&Transform, With<FpsPlayer>>,
    mut presentation: ResMut<ExteriorPresentationStats>,
    mut objects: Query<(&Transform, &mut ExteriorObjectLod, &mut Visibility)>,
) {
    let Some(player) = player.iter().next() else {
        return;
    };
    for (transform, mut lod, mut visibility) in &mut objects {
        // Distant records are the worldspace's explicit landmark
        // representation. Cell-owned objects are culled before the next
        // streaming ring would normally make them relevant.
        let was_visible = lod.visible;
        if lod.persistent || lod.distant {
            lod.visible = true;
        } else {
            let distance = player.translation.distance(transform.translation);
            lod.visible = if lod.visible {
                distance <= 340.0
            } else {
                distance <= 320.0
            };
        }
        if was_visible != lod.visible {
            presentation.object_lod_transitions =
                presentation.object_lod_transitions.saturating_add(1);
        }
        *visibility = if lod.visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

#[allow(clippy::too_many_arguments)]
fn update_worldspace_lod(
    mut commands: Commands,
    catalog: Res<ExteriorWorldspaceLodCatalog>,
    settings: Res<ExteriorWorldspaceLodSettings>,
    mut presentation: ResMut<ExteriorPresentationStats>,
    asset_server: Res<AssetServer>,
    player: Query<&Transform, With<FpsPlayer>>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    active: Query<(Entity, &ExteriorWorldspaceLodVisual)>,
) {
    presentation.worldspace_lod_asset_loads_staged_last_frame = 0;
    if !settings.enabled {
        let despawns = active.iter().count() as u64;
        for (entity, _) in &active {
            commands.entity(entity).despawn();
        }
        presentation.worldspace_lod_despawns_total = presentation
            .worldspace_lod_despawns_total
            .saturating_add(despawns);
        return;
    }
    let Some(root) = catalog.root else {
        return;
    };
    let Some(view_position) = cameras
        .iter()
        .next()
        .map(GlobalTransform::translation)
        .or_else(|| player.iter().next().map(|transform| transform.translation))
    else {
        return;
    };
    let mut candidates = catalog
        .descriptors
        .iter()
        .filter_map(|descriptor| {
            worldspace_lod_distance(descriptor, view_position).map(|distance| {
                (
                    ExteriorWorldspaceLodVisual {
                        level: descriptor.level,
                        grid: descriptor.grid,
                        blocks: descriptor.blocks,
                    },
                    descriptor,
                    distance,
                )
            })
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|(_, left, left_distance), (_, right, right_distance)| {
        left_distance
            .total_cmp(right_distance)
            .then_with(|| left.level.cmp(&right.level))
            .then_with(|| left.grid.x.cmp(&right.grid.x))
            .then_with(|| left.grid.y.cmp(&right.grid.y))
            .then_with(|| left.blocks.cmp(&right.blocks))
            .then_with(|| left.asset_path.cmp(&right.asset_path))
    });
    let mut desired = candidates
        .iter()
        .copied()
        .filter(|(_, descriptor, _)| descriptor.blocks)
        .take(WORLDSPACE_LOD_BLOCK_BUDGET)
        .collect::<Vec<_>>();
    for (level, budget) in WORLDSPACE_LOD_TERRAIN_BUDGETS {
        desired.extend(
            candidates
                .iter()
                .copied()
                .filter(|(_, descriptor, _)| !descriptor.blocks && descriptor.level == level)
                .take(budget),
        );
    }
    debug_assert!(desired.len() <= WORLDSPACE_LOD_MAX_ACTIVE);
    let active_keys = active.iter().map(|(_, key)| *key).collect::<HashSet<_>>();
    let mut despawned = 0u64;
    for (entity, key) in &active {
        if !desired
            .iter()
            .any(|(desired_key, _, _)| *desired_key == *key)
        {
            commands.entity(entity).despawn();
            despawned = despawned.saturating_add(1);
        }
    }
    let mut spawned = 0;
    for (key, descriptor, _) in desired {
        if active_keys.contains(&key) {
            continue;
        }
        if spawned >= WORLDSPACE_LOD_MAX_SPAWN_PER_FRAME {
            break;
        }
        let handle = asset_server
            .load::<WorldAsset>(GltfAssetLabel::Scene(0).from_asset(descriptor.asset_path.clone()));
        commands.spawn((
            WorldAssetRoot(handle),
            key,
            // The preserved NIF root already carries the authored worldspace
            // tile origin. Applying `grid_origin` here would translate every
            // LOD tile twice and create the detached slabs seen in far views.
            Transform::IDENTITY,
            Visibility::Inherited,
            ChildOf(root),
        ));
        spawned += 1;
    }
    let spawned = spawned as u64;
    presentation.worldspace_lod_asset_loads_staged_total = presentation
        .worldspace_lod_asset_loads_staged_total
        .saturating_add(spawned);
    presentation.worldspace_lod_asset_loads_staged_last_frame = spawned;
    presentation.worldspace_lod_peak_asset_loads_staged_per_frame = presentation
        .worldspace_lod_peak_asset_loads_staged_per_frame
        .max(spawned);
    presentation.worldspace_lod_despawns_total = presentation
        .worldspace_lod_despawns_total
        .saturating_add(despawned);
}

fn worldspace_lod_distance(
    descriptor: &ExteriorWorldspaceLodAsset,
    view_position: Vec3,
) -> Option<f32> {
    let policy = ExteriorCoordinatePolicy::default();
    let origin = policy.grid_origin(descriptor.grid);
    let span = policy.cell_span_metres() * f64::from(descriptor.level);
    let center = Vec3::new(
        origin[0] as f32 + span as f32 * 0.5,
        0.0,
        origin[2] as f32 - span as f32 * 0.5,
    );
    let distance = Vec2::new(view_position.x - center.x, view_position.z - center.z).length();
    if descriptor.blocks {
        return (descriptor.level == 4 && distance <= 1_200.0).then_some(distance);
    }
    match descriptor.level {
        4 if (120.0..=720.0).contains(&distance) => Some(distance),
        8 if (520.0..=1_500.0).contains(&distance) => Some(distance),
        16 if (1_200.0..=3_000.0).contains(&distance) => Some(distance),
        32 if (2_500.0..=7_000.0).contains(&distance) => Some(distance),
        _ => None,
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
