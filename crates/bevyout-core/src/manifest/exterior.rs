//! Bevy-free contracts for prepared exterior worlds.
//!
//! Exterior preparation and streaming deliberately share these types.  The
//! application crate may attach filesystem and Bevy adapters, but it cannot
//! invent a second coordinate system or lifecycle authority.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use serde::{Deserialize, Serialize};

pub const EXTERIOR_INDEX_REVISION: &str = "exterior-index-v1";
pub const EXTERIOR_CELL_PACKAGE_REVISION: &str = "exterior-cell-package-v3";
pub const EXTERIOR_COORDINATE_POLICY_REVISION: &str = "fo3-exterior-coordinates-v1";
pub const EXTERIOR_LOD_POLICY_REVISION: &str = "exterior-lod-v1";
pub const EXTERIOR_ENVIRONMENT_REVISION: &str = "exterior-environment-v1";

/// Fallout's exterior grid is 4096 plugin units wide.  Existing bevyout
/// placement conversion uses 70 plugin units per Bevy metre.
pub const FALLOUT_GRID_UNITS: f64 = 4096.0;
pub const FALLOUT_UNITS_PER_METRE: f64 = 70.0;

#[derive(
    Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct GridCoordinate {
    pub x: i32,
    pub y: i32,
}

impl GridCoordinate {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExteriorCoordinatePolicy {
    pub revision: String,
    pub plugin_units_per_metre: f64,
    pub cell_span_plugin_units: f64,
    /// Small boundary tolerance in Bevy metres.  A point exactly on a border
    /// belongs to the lower grid coordinate unless it is beyond this epsilon.
    pub border_epsilon_metres: f64,
    /// True only when measured route/world bounds fit the selected f32 policy.
    pub global_f32_stable: bool,
}

impl Default for ExteriorCoordinatePolicy {
    fn default() -> Self {
        Self {
            revision: EXTERIOR_COORDINATE_POLICY_REVISION.into(),
            plugin_units_per_metre: FALLOUT_UNITS_PER_METRE,
            cell_span_plugin_units: FALLOUT_GRID_UNITS,
            border_epsilon_metres: 0.0001,
            global_f32_stable: true,
        }
    }
}

impl ExteriorCoordinatePolicy {
    pub fn cell_span_metres(&self) -> f64 {
        self.cell_span_plugin_units / self.plugin_units_per_metre
    }

    /// Converts Fallout XYZ plugin coordinates into the existing Bevy
    /// coordinate convention: X stays X, Fallout Y becomes -Bevy Z, and
    /// Fallout Z becomes Bevy Y.
    pub fn plugin_to_bevy(&self, point: [f64; 3]) -> [f64; 3] {
        [
            point[0] / self.plugin_units_per_metre,
            point[2] / self.plugin_units_per_metre,
            -point[1] / self.plugin_units_per_metre,
        ]
    }

    pub fn bevy_to_plugin(&self, point: [f64; 3]) -> [f64; 3] {
        [
            point[0] * self.plugin_units_per_metre,
            -point[2] * self.plugin_units_per_metre,
            point[1] * self.plugin_units_per_metre,
        ]
    }

    pub fn grid_origin(&self, grid: GridCoordinate) -> [f64; 3] {
        self.plugin_to_bevy([
            f64::from(grid.x) * self.cell_span_plugin_units,
            f64::from(grid.y) * self.cell_span_plugin_units,
            0.0,
        ])
    }

    pub fn grid_for_bevy(&self, point: [f64; 3]) -> GridCoordinate {
        let plugin = self.bevy_to_plugin(point);
        let span = self.cell_span_plugin_units;
        let epsilon = self.border_epsilon_metres * self.plugin_units_per_metre;
        GridCoordinate::new(
            floor_border_coordinate(plugin[0], span, epsilon),
            floor_border_coordinate(plugin[1], span, epsilon),
        )
    }

    pub fn adjacent_origin_delta(&self, axis: i32) -> f64 {
        f64::from(axis) * self.cell_span_metres()
    }
}

fn floor_border_coordinate(value: f64, span: f64, epsilon: f64) -> i32 {
    let quotient = value / span;
    let rounded = quotient.round();
    if (value - rounded * span).abs() <= epsilon {
        rounded as i32
    } else {
        quotient.floor() as i32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExteriorWorldspaceIndex {
    pub revision: String,
    pub content_fingerprint: String,
    pub worldspace_form_id: u32,
    pub editor_id: Option<String>,
    pub name: Option<String>,
    pub climate_form_id: Option<u32>,
    pub coordinate_policy: ExteriorCoordinatePolicy,
    pub cells: Vec<ExteriorCellIndexEntry>,
    pub persistent_references: Vec<ExteriorPersistentReference>,
    pub diagnostics: Vec<ExteriorDiagnostic>,
}

impl ExteriorWorldspaceIndex {
    pub fn sort_deterministically(&mut self) {
        self.cells
            .sort_by_key(|cell| (cell.grid, cell.cell_form_id));
        self.persistent_references
            .sort_by_key(|reference| reference.reference_form_id);
        self.diagnostics.sort_by(|left, right| {
            left.code
                .cmp(&right.code)
                .then_with(|| left.form_id.cmp(&right.form_id))
                .then_with(|| left.message.cmp(&right.message))
        });
    }

    pub fn cell_at(&self, grid: GridCoordinate) -> Option<&ExteriorCellIndexEntry> {
        self.cells.iter().find(|cell| cell.grid == grid)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExteriorCellIndexEntry {
    pub cell_form_id: u32,
    pub grid: GridCoordinate,
    pub origin: [f32; 3],
    pub package_path: String,
    pub land_form_id: Option<u32>,
    pub road_count: usize,
    pub navm_count: usize,
    pub persistent_reference_count: usize,
    pub distant_reference_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExteriorPersistentReference {
    pub reference_form_id: u32,
    pub cell_form_id: u32,
    pub base_form_id: u32,
    pub semantic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExteriorCellPackage {
    pub revision: String,
    pub content_fingerprint: String,
    pub cell_form_id: u32,
    pub worldspace_form_id: u32,
    pub grid: GridCoordinate,
    pub origin: [f32; 3],
    pub terrain: Option<PreparedTerrain>,
    pub water: Option<PreparedWater>,
    pub static_objects: Vec<PreparedExteriorObject>,
    pub dynamic_objects: Vec<PreparedExteriorObject>,
    pub distant_objects: Vec<PreparedExteriorObject>,
    pub local_lights: Vec<PreparedExteriorLight>,
    pub navigation: Option<PreparedExteriorNavigation>,
    pub environment: PreparedExteriorEnvironment,
    pub diagnostics: Vec<ExteriorDiagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedTerrain {
    pub width: u16,
    pub height: u16,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[u8; 4]>,
    pub blend_weights: Vec<[u8; 4]>,
    pub texture_layers: Vec<u32>,
    pub collision_heights: Vec<f32>,
}

impl PreparedTerrain {
    pub fn sample_count(&self) -> usize {
        usize::from(self.width) * usize::from(self.height)
    }

    pub fn is_well_formed(&self) -> bool {
        let count = self.sample_count();
        self.positions.len() == count
            && self.normals.len() == count
            && self.colors.len() == count
            && self.blend_weights.len() == count
            && self.collision_heights.len() == count
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedWater {
    pub form_id: Option<u32>,
    pub height: f32,
    pub water_type_form_id: Option<u32>,
    pub swim_depth: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ExteriorWaterContact {
    pub submerged: bool,
    pub depth: f32,
    pub surface_height: f32,
}

pub fn resolve_water_contact(
    water: Option<&PreparedWater>,
    player_height: f32,
) -> Option<ExteriorWaterContact> {
    let water = water?;
    if !water.height.is_finite() || !player_height.is_finite() {
        return None;
    }
    let depth = (water.height - player_height).max(0.0);
    Some(ExteriorWaterContact {
        submerged: depth > 0.0 && depth <= water.swim_depth.max(0.0),
        depth,
        surface_height: water.height,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedExteriorObject {
    pub reference_form_id: u32,
    pub base_form_id: u32,
    pub asset_path: Option<String>,
    #[serde(default)]
    pub physics_asset_path: Option<String>,
    #[serde(default)]
    pub door_destination: Option<PreparedExteriorDoorDestination>,
    pub position: [f32; 3],
    pub rotation_xyzw: [f32; 4],
    pub scale: f32,
    pub initially_enabled: bool,
    pub persistent: bool,
    pub dynamic: bool,
    pub distant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedExteriorDoorDestination {
    pub door_reference_form_id: u32,
    pub cell_form_id: u32,
    pub position: [f32; 3],
    pub rotation_xyzw: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedExteriorLight {
    pub reference_form_id: u32,
    pub position: [f32; 3],
    pub color_rgba: [f32; 4],
    pub range: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedExteriorNavigation {
    pub revision: String,
    pub vertices: Vec<[f32; 3]>,
    pub triangles: Vec<[u32; 3]>,
    pub border_portals: Vec<ExteriorBorderPortal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExteriorBorderPortal {
    pub edge: u8,
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub tolerance: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PreparedExteriorEnvironment {
    pub revision: String,
    pub climate_form_id: Option<u32>,
    pub weather_form_id: Option<u32>,
    pub image_space_form_id: Option<u32>,
    pub water_type_form_id: Option<u32>,
    pub fog_near: f32,
    pub fog_far: f32,
    pub dynamic_lighting_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExteriorDiagnostic {
    pub code: String,
    pub form_id: Option<u32>,
    pub severity: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExteriorCellLifecycle {
    Unloaded,
    Queued,
    Loading,
    Ready,
    Resident,
    Evicting,
    Failed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExteriorLoadAction {
    Request,
    RaisePriority,
    Cancel,
    Activate,
    Deactivate,
    Evict,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExteriorCellState {
    pub cell_form_id: u32,
    pub grid: GridCoordinate,
    pub lifecycle: ExteriorCellLifecycle,
    pub generation: u64,
    pub pinned: bool,
    pub estimated_bytes: u64,
    pub failed_attempts: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ExteriorResidencyInput {
    pub current_grid: GridCoordinate,
    pub velocity_grid: (i32, i32),
    pub resident_budget: usize,
    pub byte_budget: u64,
    pub near_radius: i32,
    pub prefetch_radius: i32,
    pub distant_radius: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExteriorResidencyAction {
    pub form_id: u32,
    pub grid: GridCoordinate,
    pub action: ExteriorLoadAction,
    pub generation: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ExteriorResidencyPlan {
    pub desired: Vec<GridCoordinate>,
    pub actions: Vec<ExteriorResidencyAction>,
}

/// Pure, deterministic residency planner.  A package index maps each grid to
/// a FormID; missing grids are simply not requestable.
pub fn plan_residency(
    input: ExteriorResidencyInput,
    indexed_cells: &BTreeMap<GridCoordinate, u32>,
    states: &[ExteriorCellState],
) -> ExteriorResidencyPlan {
    let mut desired = indexed_cells
        .keys()
        .copied()
        .filter(|grid| {
            let dx = grid.x - input.current_grid.x;
            let dy = grid.y - input.current_grid.y;
            let distance = dx.abs().max(dy.abs());
            distance <= input.prefetch_radius
                || input.distant_radius.is_some_and(|radius| {
                    distance <= radius && distance == input.prefetch_radius + 1
                })
        })
        .collect::<Vec<_>>();
    desired.sort_by_key(|grid| residency_priority(*grid, input));

    let mut selected = desired.clone();
    let mut state_by_grid = states
        .iter()
        .map(|state| (state.grid, state))
        .collect::<HashMap<_, _>>();
    let mut bytes = selected
        .iter()
        .filter_map(|grid| state_by_grid.get(grid).map(|state| state.estimated_bytes))
        .sum::<u64>();
    while selected.len() > input.resident_budget || bytes > input.byte_budget {
        let Some(index) = selected
            .iter()
            .enumerate()
            .rev()
            .find(|(_, grid)| {
                **grid != input.current_grid
                    && !states
                        .iter()
                        .find(|state| state.grid == **grid)
                        .is_some_and(|state| state.pinned)
            })
            .map(|(index, _)| index)
        else {
            break;
        };
        if let Some(state) = state_by_grid.remove(&selected[index]) {
            bytes = bytes.saturating_sub(state.estimated_bytes);
        }
        selected.remove(index);
    }

    let selected_set = selected.iter().copied().collect::<BTreeSet<_>>();
    let mut actions = Vec::new();
    for grid in &selected {
        let Some(&form_id) = indexed_cells.get(grid) else {
            continue;
        };
        match states.iter().find(|state| state.grid == *grid) {
            None => actions.push(ExteriorResidencyAction {
                form_id,
                grid: *grid,
                action: ExteriorLoadAction::Request,
                generation: 1,
            }),
            Some(state) if matches!(state.lifecycle, ExteriorCellLifecycle::Unloaded) => {
                actions.push(ExteriorResidencyAction {
                    form_id,
                    grid: *grid,
                    action: ExteriorLoadAction::Request,
                    generation: state.generation.saturating_add(1),
                });
            }
            Some(state)
                if state.lifecycle == ExteriorCellLifecycle::Ready
                    && *grid == input.current_grid =>
            {
                actions.push(ExteriorResidencyAction {
                    form_id,
                    grid: *grid,
                    action: ExteriorLoadAction::Activate,
                    generation: state.generation,
                })
            }
            _ => {}
        }
    }
    for state in states {
        if !selected_set.contains(&state.grid)
            && state.grid != input.current_grid
            && !state.pinned
            && matches!(
                state.lifecycle,
                ExteriorCellLifecycle::Queued
                    | ExteriorCellLifecycle::Loading
                    | ExteriorCellLifecycle::Ready
                    | ExteriorCellLifecycle::Resident
            )
        {
            actions.push(ExteriorResidencyAction {
                form_id: *indexed_cells
                    .get(&state.grid)
                    .unwrap_or(&state.cell_form_id),
                grid: state.grid,
                action: if matches!(
                    state.lifecycle,
                    ExteriorCellLifecycle::Queued | ExteriorCellLifecycle::Loading
                ) {
                    ExteriorLoadAction::Cancel
                } else {
                    ExteriorLoadAction::Evict
                },
                generation: state.generation,
            });
        }
    }
    actions.sort_by(|left, right| {
        residency_priority(left.grid, input)
            .cmp(&residency_priority(right.grid, input))
            .then_with(|| left.form_id.cmp(&right.form_id))
    });
    ExteriorResidencyPlan {
        desired: selected,
        actions,
    }
}

fn residency_priority(grid: GridCoordinate, input: ExteriorResidencyInput) -> (u8, i32, i32, u32) {
    let dx = grid.x - input.current_grid.x;
    let dy = grid.y - input.current_grid.y;
    let distance = dx.abs().max(dy.abs());
    let ahead = dx.signum() == input.velocity_grid.0.signum()
        && dy.signum() == input.velocity_grid.1.signum()
        && (input.velocity_grid.0 != 0 || input.velocity_grid.1 != 0);
    (
        if distance == 0 {
            0
        } else if distance <= input.near_radius {
            1
        } else if ahead {
            2
        } else {
            3
        },
        distance,
        grid.y,
        grid.x as u32,
    )
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TerrainLod {
    Near,
    Middle,
    Distant,
}

pub fn select_terrain_lod(
    distance_metres: f32,
    previous: Option<TerrainLod>,
    near_end: f32,
    middle_end: f32,
    hysteresis: f32,
) -> TerrainLod {
    let h = hysteresis.max(0.0);
    match previous {
        Some(TerrainLod::Near) if distance_metres <= near_end + h => TerrainLod::Near,
        Some(TerrainLod::Middle)
            if distance_metres > near_end - h && distance_metres <= middle_end + h =>
        {
            TerrainLod::Middle
        }
        Some(TerrainLod::Distant) if distance_metres > middle_end - h => TerrainLod::Distant,
        _ if distance_metres <= near_end => TerrainLod::Near,
        _ if distance_metres <= middle_end => TerrainLod::Middle,
        _ => TerrainLod::Distant,
    }
}

pub fn clamp_lod_delta(left: TerrainLod, right: TerrainLod) -> (TerrainLod, TerrainLod) {
    let left_rank = lod_rank(left);
    let right_rank = lod_rank(right);
    if (left_rank - right_rank).abs() <= 1 {
        return (left, right);
    }
    if left_rank < right_rank {
        (left, rank_lod(left_rank + 1))
    } else {
        (rank_lod(right_rank + 1), right)
    }
}

fn lod_rank(lod: TerrainLod) -> i8 {
    match lod {
        TerrainLod::Near => 0,
        TerrainLod::Middle => 1,
        TerrainLod::Distant => 2,
    }
}

fn rank_lod(rank: i8) -> TerrainLod {
    match rank.clamp(0, 2) {
        0 => TerrainLod::Near,
        1 => TerrainLod::Middle,
        _ => TerrainLod::Distant,
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct WeatherBlend {
    pub source_weather_form_id: Option<u32>,
    pub target_weather_form_id: Option<u32>,
    pub progress: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentInput {
    pub interior: bool,
    pub behave_like_exterior: bool,
    pub worldspace_form_id: Option<u32>,
    pub climate_form_id: Option<u32>,
    pub weather: WeatherBlend,
    pub image_space_form_id: Option<u32>,
    pub time_hours: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentSnapshot {
    pub source: String,
    pub worldspace_form_id: Option<u32>,
    pub climate_form_id: Option<u32>,
    pub weather_form_id: Option<u32>,
    pub target_weather_form_id: Option<u32>,
    pub weather_progress: f32,
    pub image_space_form_id: Option<u32>,
    pub dynamic_lighting_allowed: bool,
    pub time_hours: f32,
}

pub fn resolve_environment(input: EnvironmentInput) -> EnvironmentSnapshot {
    let dynamic = !input.interior || input.behave_like_exterior;
    let source = if input.interior && !input.behave_like_exterior {
        "interior-cell"
    } else if input.weather.target_weather_form_id.is_some() {
        "worldspace-weather-transition"
    } else {
        "worldspace-climate"
    };
    EnvironmentSnapshot {
        source: source.into(),
        worldspace_form_id: input.worldspace_form_id,
        climate_form_id: input.climate_form_id,
        weather_form_id: input.weather.source_weather_form_id,
        target_weather_form_id: input.weather.target_weather_form_id,
        weather_progress: input.weather.progress.clamp(0.0, 1.0),
        image_space_form_id: input.image_space_form_id,
        dynamic_lighting_allowed: dynamic,
        time_hours: input.time_hours.rem_euclid(24.0),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorldLocationExterior {
    pub worldspace_form_id: u32,
    pub position: [f32; 3],
    pub rotation_xyzw: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorldLocationInterior {
    pub cell_form_id: u32,
    pub position: [f32; 3],
    pub rotation_xyzw: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorldLocation {
    Interior(WorldLocationInterior),
    Exterior(WorldLocationExterior),
}

impl WorldLocation {
    pub fn cell_key(&self) -> u32 {
        match self {
            Self::Interior(value) => value.cell_form_id,
            Self::Exterior(value) => value.worldspace_form_id,
        }
    }
}

/// Deterministic pairwise portal matching used by exterior NAVM stitching.
pub fn matching_portals(
    left_grid: GridCoordinate,
    left: &[ExteriorBorderPortal],
    right_grid: GridCoordinate,
    right: &[ExteriorBorderPortal],
) -> Vec<(usize, usize)> {
    let mut matches = Vec::new();
    for (left_index, left_portal) in left.iter().enumerate() {
        for (right_index, right_portal) in right.iter().enumerate() {
            if portal_edges_match(left_grid, left_portal, right_grid, right_portal) {
                matches.push((left_index, right_index));
            }
        }
    }
    matches.sort_unstable();
    matches.dedup();
    matches
}

fn portal_edges_match(
    left_grid: GridCoordinate,
    left: &ExteriorBorderPortal,
    right_grid: GridCoordinate,
    right: &ExteriorBorderPortal,
) -> bool {
    let policy = ExteriorCoordinatePolicy::default();
    let left_offset = policy.grid_origin(left_grid);
    let right_offset = policy.grid_origin(right_grid);
    let translate = |offset: [f64; 3], point: [f32; 3]| {
        [
            offset[0] + f64::from(point[0]),
            offset[1] + f64::from(point[1]),
            offset[2] + f64::from(point[2]),
        ]
    };
    let l0 = translate(left_offset, left.start);
    let l1 = translate(left_offset, left.end);
    let r0 = translate(right_offset, right.start);
    let r1 = translate(right_offset, right.end);
    close_points(l0, r1, f64::from(left.tolerance.max(right.tolerance)))
        && close_points(l1, r0, f64::from(left.tolerance.max(right.tolerance)))
}

fn close_points(left: [f64; 3], right: [f64; 3], tolerance: f64) -> bool {
    left.into_iter()
        .zip(right)
        .all(|(left, right)| (left - right).abs() <= tolerance)
}

impl Ord for TerrainLod {
    fn cmp(&self, other: &Self) -> Ordering {
        lod_rank(*self).cmp(&lod_rank(*other))
    }
}

impl PartialOrd for TerrainLod {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
#[path = "exterior_tests.rs"]
mod tests;
