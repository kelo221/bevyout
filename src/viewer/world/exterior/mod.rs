//! Prepared exterior package streaming.
//!
//! Runtime accepts only `.ron` packages and GLB asset paths already produced
//! by preparation. No converter, Blender process, or source-plugin parser is
//! reachable from this module.

mod diagnostics;
mod lifecycle;
mod loading;
mod policy;

use std::fs;
use std::path::Path;

use avian3d::prelude::Collider;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevyout_core::manifest::exterior::{
    ExteriorCellLifecycle, ExteriorCellPackage, ExteriorCoordinatePolicy, ExteriorLoadAction,
    ExteriorWaterContact, PreparedTerrain, PreparedWater, TerrainLod, resolve_water_contact,
    select_terrain_lod,
};

use super::super::LoadedSceneManifest;
use super::super::player::FpsPlayer;
use crate::app_state::AppState;
use crate::viewer::day_night::GameClock;

pub(crate) use diagnostics::{cells as exterior_cells_json, status as exterior_status_json};
pub(crate) use lifecycle::ExteriorStreamState;

#[derive(Component)]
pub(crate) struct ExteriorCellRoot {
    #[allow(dead_code)]
    pub(crate) form_id: u32,
}

#[derive(Component)]
struct ExteriorTerrain;

#[derive(Component)]
struct ExteriorTerrainLod {
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
    base_intensity: f32,
}

#[derive(Resource, Debug, Default, Clone, Copy)]
pub(crate) struct ExteriorWaterState {
    pub(crate) contact: Option<ExteriorWaterContact>,
}

#[derive(Component)]
struct ExteriorReference {
    #[allow(dead_code)]
    reference_form_id: u32,
}

pub(crate) struct ExteriorWorldPlugin {
    pub(crate) resident_cell_limit: usize,
}

impl Plugin for ExteriorWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExteriorStreamState>()
            .init_resource::<ExteriorWaterState>()
            .insert_resource(ExteriorStreamBudget {
                resident_cells: self.resident_cell_limit,
                bytes: 128 * 1024 * 1024,
            })
            .add_systems(
                Update,
                (initialize, place_player, update_residency, loading::poll)
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
            update_terrain_lod
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
    budget: Res<ExteriorStreamBudget>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player: Query<&mut Transform, With<FpsPlayer>>,
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
    match fs::read_to_string(&index_path)
        .and_then(|text| ron::from_str(&text).map_err(std::io::Error::other))
    {
        Ok(index) => state.index = Some(index),
        Err(error) => warn!(
            "exterior index unavailable {}: {error}",
            index_path.display()
        ),
    }
    let root = spawn_package(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        &package,
    );
    let initial_grid = package.grid;
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
        },
    );
    state.resident_bytes = state
        .cells
        .get(&initial_grid)
        .map(|cell| cell.state.estimated_bytes)
        .unwrap_or_default();
    state.peak_resident_cells = 1;
    state.peak_memory = state.resident_bytes;
    info!("exterior resident {:08x}", manifest.cell.form_id);
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
    if current != state.current_grid {
        state.previous_grid = Some(state.current_grid);
        state.current_grid = current;
    }
    let velocity_grid = state
        .previous_grid
        .map(|previous| (current.x - previous.x, current.y - previous.y))
        .unwrap_or((0, 0));
    let plan = policy::desired_plan(
        &index,
        current,
        velocity_grid,
        &state.states(),
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
    action: bevyout_core::manifest::exterior::ExteriorResidencyAction,
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
            state
                .cells
                .entry(action.grid)
                .or_insert_with(|| lifecycle::RuntimeCell {
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
                });
            loading::request(
                commands,
                state,
                action.grid,
                action.form_id,
                action.generation,
            );
        }
        ExteriorLoadAction::Cancel => {
            if let Some(cell) = state.cells.get_mut(&action.grid) {
                if let Some(task) = cell.task.take()
                    && tasks.get(task).is_ok()
                {
                    commands.entity(task).despawn();
                }
                cell.state.generation = cell.state.generation.saturating_add(1);
                cell.state.lifecycle = ExteriorCellLifecycle::Unloaded;
                state.cancellations += 1;
            }
        }
        ExteriorLoadAction::Evict => {
            if let Some(mut cell) = state.cells.remove(&action.grid) {
                if let Some(root) = cell.root.take() {
                    commands.entity(root).despawn();
                }
                if let Some(task) = cell.task.take()
                    && tasks.get(task).is_ok()
                {
                    commands.entity(task).despawn();
                }
                state.resident_bytes = state
                    .resident_bytes
                    .saturating_sub(cell.state.estimated_bytes);
                state.evictions += 1;
            }
        }
        ExteriorLoadAction::Activate => {
            state.set_lifecycle(action.grid, ExteriorCellLifecycle::Resident);
        }
        ExteriorLoadAction::RaisePriority | ExteriorLoadAction::Deactivate => {}
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
        let near_handle = meshes.add(terrain);
        let middle_handle = terrain_mesh_with_stride(terrain_data, 2)
            .map(|mesh| meshes.add(mesh))
            .unwrap_or_else(|| near_handle.clone());
        let distant_handle = terrain_mesh_with_stride(terrain_data, 4)
            .map(|mesh| meshes.add(mesh))
            .unwrap_or_else(|| middle_handle.clone());
        let entity = commands
            .spawn((
                Mesh3d(near_handle.clone()),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.22, 0.27, 0.16),
                    perceptual_roughness: 1.0,
                    ..default()
                })),
                ExteriorTerrain,
                ExteriorTerrainLod {
                    near: near_handle,
                    middle: middle_handle,
                    distant: distant_handle,
                    current: TerrainLod::Near,
                    center: Vec3::new(
                        package.origin[0]
                            + ExteriorCoordinatePolicy::default().cell_span_metres() as f32 * 0.5,
                        package.origin[1],
                        package.origin[2]
                            - ExteriorCoordinatePolicy::default().cell_span_metres() as f32 * 0.5,
                    ),
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
    let mut colors = Vec::with_capacity(columns * rows);
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
        }
    }
    let mut indices = Vec::with_capacity((columns - 1) * (rows - 1) * 6);
    for y in 0..rows - 1 {
        for x in 0..columns - 1 {
            let i = (y * columns + x) as u32;
            let next = i + columns as u32;
            indices.extend_from_slice(&[i, next, i + 1, i + 1, next, next + 1]);
        }
    }
    Some(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_inserted_indices(Indices::U32(indices)),
    )
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
            indices.extend_from_slice(&[[i, next, i + 1], [i + 1, next, next + 1]]);
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
    let terrain_center = package
        .terrain
        .as_ref()
        .filter(|terrain| terrain.is_well_formed())
        .and_then(|terrain| {
            let x = usize::from(terrain.width) / 2;
            let y = usize::from(terrain.height) / 2;
            terrain.positions.get(y * usize::from(terrain.width) + x)
        })
        .copied()
        .map(Vec3::from_array);
    terrain_center.unwrap_or_else(|| {
        let span = ExteriorCoordinatePolicy::default().cell_span_metres() as f32;
        Vec3::new(
            package.origin[0] + span * 0.5,
            package.origin[1],
            package.origin[2] - span * 0.5,
        )
    }) + Vec3::Y * super::super::player::CAPSULE_HEIGHT.mul_add(0.5, 0.2)
}

fn update_local_lights(
    clock: Res<GameClock>,
    mut lights: Query<(&mut PointLight, &ExteriorLocalLight)>,
) {
    let daylight = (((clock.hour - 6.0) / 12.0) * std::f32::consts::PI)
        .sin()
        .clamp(0.0, 1.0);
    let factor = 0.35 + daylight * 0.65;
    for (mut light, authored) in &mut lights {
        light.intensity = authored.base_intensity * (1.15 - factor * 0.55);
    }
}

fn update_water_state(
    player: Query<&Transform, With<FpsPlayer>>,
    waters: Query<&ExteriorWaterSurface>,
    mut state: ResMut<ExteriorWaterState>,
) {
    let Some(player) = player.iter().next() else {
        state.contact = None;
        return;
    };
    state.contact = waters
        .iter()
        .filter_map(|water| resolve_water_contact(Some(&water.descriptor), player.translation.y))
        .max_by(|left, right| left.depth.total_cmp(&right.depth));
}

fn update_terrain_lod(
    player: Query<&Transform, With<FpsPlayer>>,
    mut terrain: Query<(&mut Mesh3d, &mut ExteriorTerrainLod)>,
) {
    let Some(player) = player.iter().next() else {
        return;
    };
    for (mut mesh, mut lod) in &mut terrain {
        let distance = player.translation.distance(lod.center);
        let selected = select_terrain_lod(distance, Some(lod.current), 80.0, 180.0, 8.0);
        if selected == lod.current {
            continue;
        }
        mesh.0 = match selected {
            TerrainLod::Near => lod.near.clone(),
            TerrainLod::Middle => lod.middle.clone(),
            TerrainLod::Distant => lod.distant.clone(),
        };
        lod.current = selected;
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
