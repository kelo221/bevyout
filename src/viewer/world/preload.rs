//! Bevy-side driver for the predictive door-graph neighbor preloader
//! (issue #51). Reads `<asset_root>/cellmap.ron` into a `policy::CellGraph`
//! once at startup, tracks the active cell and which cells are resident, and
//! -- whenever the active cell changes (including once at startup) --
//! background-parses newly planned neighbor manifests and spawns their
//! content under a hidden per-cell root, or despawns evicted ones.
//!
//! Preloaded cells are spawned via `scene::spawn_cell_content` with
//! `references: None` and `Visibility::Hidden`: they never register in
//! `crate::console::RefRegistry` and (since `player::build_prepared_colliders`
//! only ever reads the single startup `PreparedSceneManifest` resource, not
//! spawned entities) never get physics colliders either. Wiring a
//! newly-active preloaded cell into both is left to issue #52.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bevy::prelude::*;
use bevy::tasks::futures::check_ready;
use bevy::tasks::{AsyncComputeTaskPool, Task};

use crate::app_state::AppState;
use crate::vsa::{CellMap, PreparedSceneManifest};

use super::super::LightingScale;
use super::super::scene::spawn_cell_content;
use super::policy;

/// Parsed door-graph adjacency, or inert if `cellmap.ron` was absent or
/// failed to parse (F51.1).
#[derive(Resource, Default)]
pub(crate) struct CellMapIndex {
    graph: Option<policy::CellGraph>,
}

impl CellMapIndex {
    pub(crate) fn graph(&self) -> Option<&policy::CellGraph> {
        self.graph.as_ref()
    }
}

/// FormID of the cell the player currently occupies. Seeded from the startup
/// manifest; later agents (issue #52) update it on cell transitions.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ActiveCell(pub(crate) u32);

#[derive(Resource, Clone, Copy, Debug)]
pub(crate) struct ResidentCellLimit(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResidentState {
    /// Manifest parsed and content spawned, but scene GLB handles are still
    /// loading through the `AssetServer`.
    Loading,
    /// Spawned and every scene handle has finished loading.
    Ready,
}

pub(crate) struct ResidentCell {
    pub(crate) root: Entity,
    pub(crate) state: ResidentState,
    // Not read yet -- issue #52's activation systems will use this to wire
    // up `RefRegistry`/physics colliders when a preloaded cell becomes
    // active, without needing to re-parse `scene.ron`.
    #[allow(dead_code)]
    pub(crate) manifest: Arc<PreparedSceneManifest>,
    // `Handle<WorldAsset>`, not `Handle<Scene>` -- see `scene::SpawnedCellContent`.
    pub(crate) scene_handles: Vec<Handle<WorldAsset>>,
    pub(crate) placement_count: usize,
}

/// Cells with spawned content, keyed by cell FormID. The active/startup
/// cell is always present and `Ready`.
#[derive(Resource, Default)]
pub(crate) struct ResidentCells(pub(crate) HashMap<u32, ResidentCell>);

/// Tracks a neighbor manifest being parsed off the main thread.
#[derive(Component)]
struct PendingPreloadParse {
    form_id: u32,
    task: Task<Result<PreparedSceneManifest, String>>,
}

pub(crate) fn install(app: &mut App, resident_cell_limit: usize) {
    app.insert_resource(ResidentCellLimit(resident_cell_limit))
        .init_resource::<ResidentCells>()
        .add_systems(Startup, seed_world_state)
        .add_systems(
            Update,
            (
                evaluate_preload_plan.run_if(in_state(AppState::InGame)),
                poll_preload_parse_tasks,
                check_preload_ready,
            ),
        );
}

fn seed_world_state(mut commands: Commands, manifest: Res<PreparedSceneManifest>) {
    commands.insert_resource(ActiveCell(manifest.cell.form_id));
    commands.insert_resource(build_cell_map_index(&manifest.asset_root));
}

/// F51.1: `CellMapIndex` from `<asset_root>/cellmap.ron`; an absent or
/// corrupt file leaves the preloader inert with exactly one `warn!`.
fn build_cell_map_index(asset_root: &str) -> CellMapIndex {
    let path = Path::new(asset_root).join("cellmap.ron");
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(_) => {
            warn!(
                "no cellmap.ron at {}; neighbor preloader inert",
                path.display()
            );
            return CellMapIndex::default();
        }
    };
    match ron::de::from_str::<CellMap>(&text) {
        Ok(map) => {
            let edges: Vec<policy::DoorLink> = map
                .doors
                .iter()
                .map(|door| policy::DoorLink {
                    source_cell_form_id: door.source_cell_form_id,
                    destination_cell_form_id: door.destination_cell_form_id,
                })
                .collect();
            CellMapIndex {
                graph: Some(policy::CellGraph::build(&edges)),
            }
        }
        Err(error) => {
            warn!(
                "cellmap.ron at {} is corrupt ({error}); neighbor preloader inert",
                path.display()
            );
            CellMapIndex::default()
        }
    }
}

fn scene_manifest_path(asset_root: &Path, form_id: u32) -> PathBuf {
    asset_root
        .join("scenes")
        .join(format!("{form_id:08x}"))
        .join("scene.ron")
}

/// F51.2/F51.5 glue: runs once at startup and again whenever `ActiveCell`
/// changes, computes the prepared-neighbor set from which `scenes/<id>/scene.ron`
/// files exist on disk, runs the pure policy, and starts/stops background
/// loads accordingly.
fn evaluate_preload_plan(
    mut commands: Commands,
    active_cell: Res<ActiveCell>,
    cell_map_index: Res<CellMapIndex>,
    mut resident_cells: ResMut<ResidentCells>,
    resident_cell_limit: Res<ResidentCellLimit>,
    manifest: Res<PreparedSceneManifest>,
    mut last_planned: Local<Option<u32>>,
) {
    if *last_planned == Some(active_cell.0) {
        return;
    }
    *last_planned = Some(active_cell.0);
    let Some(graph) = cell_map_index.graph() else {
        return;
    };
    let asset_root = Path::new(&manifest.asset_root);
    let distances = graph.distances_from(active_cell.0);
    let prepared: HashSet<u32> = distances
        .iter()
        .filter(|&(_, &distance)| distance == 1)
        .filter(|&(&form_id, _)| scene_manifest_path(asset_root, form_id).is_file())
        .map(|(&form_id, _)| form_id)
        .collect();
    let resident: Vec<u32> = resident_cells.0.keys().copied().collect();
    let plan = graph.plan(active_cell.0, &resident, &prepared, resident_cell_limit.0);

    for form_id in plan.load {
        info!("preload start {form_id:08x}");
        let path = scene_manifest_path(asset_root, form_id);
        let pool = AsyncComputeTaskPool::get();
        let task = pool.spawn(async move {
            let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
            ron::de::from_str::<PreparedSceneManifest>(&text).map_err(|error| error.to_string())
        });
        commands.spawn(PendingPreloadParse { form_id, task });
    }

    for form_id in plan.evict {
        if let Some(resident) = resident_cells.0.remove(&form_id) {
            commands.entity(resident.root).despawn();
        }
        let distance = distances
            .get(&form_id)
            .map_or_else(|| "unknown".to_string(), usize::to_string);
        info!("preload evict {form_id:08x} (graph distance {distance})");
    }
}

/// F51.3: polls background manifest parses; on success, spawns the cell's
/// placements and point lights under a new hidden per-cell root (not yet
/// registered as `Ready` until every scene handle finishes loading, see
/// `check_preload_ready`).
fn poll_preload_parse_tasks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    lighting: Res<LightingScale>,
    mut pending: Query<(Entity, &mut PendingPreloadParse)>,
    mut resident_cells: ResMut<ResidentCells>,
) {
    for (entity, mut pending_parse) in &mut pending {
        let Some(result) = check_ready(&mut pending_parse.task) else {
            continue;
        };
        let form_id = pending_parse.form_id;
        commands.entity(entity).despawn();
        match result {
            Ok(manifest) => {
                let root = commands
                    .spawn((Transform::default(), Visibility::Hidden))
                    .id();
                let content = spawn_cell_content(
                    &mut commands,
                    &asset_server,
                    &manifest,
                    root,
                    lighting.0,
                    None,
                );
                resident_cells.0.insert(
                    form_id,
                    ResidentCell {
                        root,
                        state: ResidentState::Loading,
                        manifest: Arc::new(manifest),
                        scene_handles: content.scene_handles,
                        placement_count: content.placement_count,
                    },
                );
            }
            Err(error) => {
                warn!("preload parse failed for {form_id:08x}: {error}");
            }
        }
    }
}

/// F51.3: promotes `Loading` resident cells to `Ready` once every spawned
/// scene handle has finished loading, logging `preload ready` exactly once.
fn check_preload_ready(asset_server: Res<AssetServer>, mut resident_cells: ResMut<ResidentCells>) {
    for (form_id, resident) in resident_cells.0.iter_mut() {
        if resident.state != ResidentState::Loading {
            continue;
        }
        let all_loaded = resident
            .scene_handles
            .iter()
            .all(|handle| asset_server.is_loaded_with_dependencies(handle.id()));
        if all_loaded {
            resident.state = ResidentState::Ready;
            info!(
                "preload ready {form_id:08x} ({} placements)",
                resident.placement_count
            );
        }
    }
}
