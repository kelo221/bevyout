//! Bevy-free contracts for prepared exterior worlds.
//!
//! Exterior preparation and streaming deliberately share these types.  The
//! application crate may attach filesystem and Bevy adapters, but it cannot
//! invent a second coordinate system or lifecycle authority.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use serde::{Deserialize, Serialize};

pub const EXTERIOR_INDEX_REVISION: &str = "exterior-index-v4-shared-weather-catalog";
pub const EXTERIOR_CELL_PACKAGE_REVISION: &str =
    "exterior-cell-package-v9-shared-weather-catalog-object-store-terrain";
pub const EXTERIOR_COORDINATE_POLICY_REVISION: &str = "fo3-exterior-coordinates-v1";
pub const EXTERIOR_LOD_POLICY_REVISION: &str = "exterior-lod-v3-worldspace-assets";
pub const EXTERIOR_ENVIRONMENT_REVISION: &str = "exterior-environment-v3-shared-weather-catalog";
pub const EXTERIOR_NAVIGATION_REVISION: &str = "exterior-nav-v3";

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
    /// Content-set weather colors stored once per worldspace instead of once
    /// per exterior cell package. Cell-local climate timings remain on the
    /// environment and are combined with these colors at runtime.
    #[serde(default)]
    pub weather_profiles: Vec<PreparedWeatherCatalogEntry>,
    pub cells: Vec<ExteriorCellIndexEntry>,
    pub persistent_references: Vec<ExteriorPersistentReference>,
    /// Prepared vanilla worldspace terrain/landmark LOD tiles. These are
    /// presentation-only and never participate in cell residency, collision,
    /// navigation, or persistence.
    #[serde(default)]
    pub worldspace_lod: Vec<ExteriorWorldspaceLodAsset>,
    pub diagnostics: Vec<ExteriorDiagnostic>,
}

impl ExteriorWorldspaceIndex {
    pub fn sort_deterministically(&mut self) {
        self.cells
            .sort_by_key(|cell| (cell.grid, cell.cell_form_id));
        self.weather_profiles.sort_by_key(|profile| profile.form_id);
        self.persistent_references
            .sort_by_key(|reference| reference.reference_form_id);
        self.worldspace_lod.sort_by_key(|asset| {
            (
                asset.level,
                asset.grid,
                asset.blocks,
                asset.asset_path.clone(),
            )
        });
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExteriorWorldspaceLodAsset {
    /// Relative prepared GLB path below the manifest asset root.
    pub asset_path: String,
    /// Vanilla terrain LOD tier: 4, 8, 16, or 32 cells per tile.
    pub level: u8,
    /// Fallout cell coordinate of the tile's southwest origin.
    pub grid: GridCoordinate,
    /// `true` for the separate `landscape/lod/<worldspace>/blocks` landmark
    /// mesh family; false for the terrain surface family.
    pub blocks: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExteriorPersistentReference {
    pub reference_form_id: u32,
    pub cell_form_id: u32,
    pub base_form_id: u32,
    pub semantic: String,
    /// Prepared world-space visual data. Persistent references are owned by
    /// the worldspace index rather than a streaming cell, so their visual
    /// placement has to survive package eviction too.
    #[serde(default)]
    pub asset_path: Option<String>,
    #[serde(default)]
    pub position: [f32; 3],
    #[serde(default)]
    pub rotation_xyzw: [f32; 4],
    #[serde(default)]
    pub scale: f32,
    #[serde(default)]
    pub initially_enabled: bool,
    #[serde(default)]
    pub distant: bool,
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
    /// Prepared, cell-local albedo produced from LAND layer weights and the
    /// resolved LTEX diffuse sources.  The runtime only loads this finished
    /// image; it never resolves Fallout source assets.
    #[serde(default)]
    pub albedo_asset_path: Option<String>,
    /// Prepared, cell-local tangent-space normal map blended from the same
    /// LAND layer weights. The runtime never resolves TXST source records.
    #[serde(default)]
    pub normal_asset_path: Option<String>,
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
        // `swim_depth` is descriptive prepared metadata, not a maximum
        // contact depth. A player cannot become dry by moving farther below
        // the same water surface.
        submerged: depth > 0.0,
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
    /// Authored arrival translation in Bevy coordinates. Runtime capsule or
    /// camera offsets are applied by the viewer adapter, never serialized here.
    pub position: [f32; 3],
    /// Authored arrival rotation, preserved without normalization.
    pub rotation_xyzw: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedExteriorLight {
    pub reference_form_id: u32,
    pub position: [f32; 3],
    pub color_rgba: [f32; 4],
    pub range: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct PreparedExteriorNavigation {
    pub revision: String,
    /// The complete prepared graph, including mesh identity, adjacency,
    /// authored flags, doors, NVEX targets, merge records, and clearance
    /// verdicts. The flattened buffers below remain a cheap diagnostic view
    /// for the exterior console, but are not the production navigation input.
    pub graph_asset_path: Option<String>,
    pub graph_hash: Option<String>,
    pub mesh_count: usize,
    pub polygon_count: usize,
    pub vertex_count: usize,
    pub door_count: usize,
    pub external_connection_count: usize,
    pub mesh_merge_count: usize,
    pub clearance_ready: bool,
    pub vertices: Vec<[f32; 3]>,
    pub triangles: Vec<[u32; 3]>,
    pub border_portals: Vec<ExteriorBorderPortal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExteriorBorderPortal {
    pub edge: u8,
    /// World-space Bevy metres. These points already include the owning
    /// cell's grid origin; they are not local-to-cell coordinates.
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
    /// Climate timings are cell-local because an exterior cell can inherit a
    /// different climate while using colors from the shared weather catalog.
    #[serde(default)]
    pub timings: crate::time_of_day::DayNightTimings,
    /// Compatibility input for packages written before the shared worldspace
    /// weather catalog. New packages leave this empty and omit it from RON.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub weather_profiles: Vec<PreparedWeatherProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedWeatherCatalogEntry {
    pub form_id: u32,
    pub editor_id: Option<String>,
    pub sky_upper: crate::time_of_day::ColorKeyframes,
    pub sky_lower: crate::time_of_day::ColorKeyframes,
    pub ambient: crate::time_of_day::ColorKeyframes,
    pub sunlight: crate::time_of_day::ColorKeyframes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedWeatherProfile {
    pub form_id: u32,
    pub editor_id: Option<String>,
    pub timings: crate::time_of_day::DayNightTimings,
    pub sky_upper: crate::time_of_day::ColorKeyframes,
    pub sky_lower: crate::time_of_day::ColorKeyframes,
    pub ambient: crate::time_of_day::ColorKeyframes,
    pub sunlight: crate::time_of_day::ColorKeyframes,
}

pub fn resolve_prepared_weather_profile(
    environment: &PreparedExteriorEnvironment,
    catalog: &[PreparedWeatherCatalogEntry],
    form_id: u32,
) -> Option<PreparedWeatherProfile> {
    catalog
        .iter()
        .find(|profile| profile.form_id == form_id)
        .map(|profile| PreparedWeatherProfile {
            form_id: profile.form_id,
            editor_id: profile.editor_id.clone(),
            timings: environment.timings,
            sky_upper: profile.sky_upper,
            sky_lower: profile.sky_lower,
            ambient: profile.ambient,
            sunlight: profile.sunlight,
        })
        .or_else(|| {
            environment
                .weather_profiles
                .iter()
                .find(|profile| profile.form_id == form_id)
                .cloned()
        })
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
            Some(state) if state.lifecycle == ExteriorCellLifecycle::Evicting => {
                // Eviction is finalized by the runtime after the planner
                // returns.  If the target re-enters the ring first, cancel
                // that teardown at the same generation instead of allowing
                // the package to disappear and be re-requested.
                actions.push(ExteriorResidencyAction {
                    form_id,
                    grid: *grid,
                    action: ExteriorLoadAction::Cancel,
                    generation: state.generation,
                });
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

fn residency_priority(grid: GridCoordinate, input: ExteriorResidencyInput) -> (u8, i32, i32, i32) {
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
        grid.x,
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
        // Keep a nearby tile at its current high-detail representation until
        // it reaches the far side of the transition band. This prevents a
        // one-frame downgrade in the middle of a tile's near/middle overlap;
        // the middle state uses the reciprocal band below.
        Some(TerrainLod::Near) if distance_metres <= middle_end - h => TerrainLod::Near,
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
    /// Authored arrival translation in Bevy coordinates. Runtime capsule or
    /// camera offsets are applied by the viewer adapter, never serialized here.
    pub position: [f32; 3],
    /// Authored arrival rotation, preserved without normalization.
    pub rotation_xyzw: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorldLocationInterior {
    pub cell_form_id: u32,
    /// Authored arrival translation in Bevy coordinates. Runtime capsule or
    /// camera offsets are applied by the viewer adapter, never serialized here.
    pub position: [f32; 3],
    /// Authored arrival rotation, preserved without normalization.
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

    /// WLOC is intentionally a small exact contract: one non-zero identity
    /// key and finite authored translation/rotation values. The quaternion is
    /// not normalized so a save/load round trip cannot change authored data.
    pub fn is_well_formed(&self) -> bool {
        match self {
            Self::Interior(value) => {
                value.cell_form_id != 0
                    && finite_values(&value.position)
                    && finite_values(&value.rotation_xyzw)
                    && non_zero_values(&value.rotation_xyzw)
            }
            Self::Exterior(value) => {
                value.worldspace_form_id != 0
                    && finite_values(&value.position)
                    && finite_values(&value.rotation_xyzw)
                    && non_zero_values(&value.rotation_xyzw)
            }
        }
    }
}

fn finite_values<const N: usize>(values: &[f32; N]) -> bool {
    values.iter().all(|value| value.is_finite())
}

fn non_zero_values<const N: usize>(values: &[f32; N]) -> bool {
    values.iter().any(|value| *value != 0.0)
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
    _left_grid: GridCoordinate,
    left: &ExteriorBorderPortal,
    _right_grid: GridCoordinate,
    right: &ExteriorBorderPortal,
) -> bool {
    // The package producer emits absolute Bevy coordinates. Translating them
    // by the grid origin again moves every non-zero cell twice and prevents
    // both positive and negative grid neighbors from stitching.
    let l0 = left.start.map(f64::from);
    let l1 = left.end.map(f64::from);
    let r0 = right.start.map(f64::from);
    let r1 = right.end.map(f64::from);
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
