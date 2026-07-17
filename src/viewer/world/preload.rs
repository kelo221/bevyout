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

use std::collections::VecDeque;

use super::super::LightingScale;
use super::super::scene::{spawn_cell_lights, spawn_cell_placements_chunk};
use super::policy;

/// Raw `manifest.placements` entries a background preload spawns per frame.
/// One-shot spawning of a large cell (Vault101d is 1,371 placements) cost a
/// 130 ms frame in acceptance testing; draining through this budget keeps
/// preload spawning invisible next to the 33 ms transition budget.
/// Wave 4 (#55, A15): 128 -> 64. Each spawn chunk kicks off its GLB loads,
/// whose completions cluster into GPU-upload bursts a few frames later; at
/// 128 those bursts measured a 35.7 ms frame inside the post-swap telemetry
/// window while the next cell preloaded. 64 halves the burst size for the
/// same total work (a global upload throttle was tried instead and reverted
/// -- see `app.rs`).
const PRELOAD_SPAWN_BUDGET_PER_FRAME: usize = 64;

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

/// A physics sidecar read and parsed inside the background preload task, so
/// the door swap's staggered collider build (issue #52) never does sidecar
/// file I/O inside the transition window.
pub(crate) struct PreloadedSidecar {
    relative_path: String,
    byte_len: u64,
    asset: crate::vsa::PreparedPhysicsAsset,
}

type PreloadParseResult = Result<(PreparedSceneManifest, Vec<PreloadedSidecar>), String>;

/// Tracks a neighbor manifest being parsed off the main thread.
#[derive(Component)]
struct PendingPreloadParse {
    form_id: u32,
    task: Task<PreloadParseResult>,
}

/// Cells whose manifest has parsed but whose placements are still being
/// spawned, a bounded chunk per frame (front of the queue first).
#[derive(Resource, Default)]
pub(crate) struct PendingCellSpawns(VecDeque<PendingCellSpawn>);

impl PendingCellSpawns {
    pub(crate) fn contains(&self, form_id: u32) -> bool {
        self.0.iter().any(|pending| pending.form_id == form_id)
    }
}

struct PendingCellSpawn {
    form_id: u32,
    root: Entity,
    next_index: usize,
}

pub(crate) fn install(app: &mut App, resident_cell_limit: usize) {
    app.insert_resource(ResidentCellLimit(resident_cell_limit))
        .init_resource::<ResidentCells>()
        .init_resource::<PendingCellSpawns>()
        .add_message::<PreloadParseFailed>()
        .add_systems(Startup, seed_world_state)
        .add_systems(
            Update,
            (
                evaluate_preload_plan.run_if(in_state(AppState::InGame)),
                poll_preload_parse_tasks,
                advance_pending_cell_spawns,
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

pub(crate) fn scene_manifest_path(asset_root: &Path, form_id: u32) -> PathBuf {
    asset_root
        .join("scenes")
        .join(format!("{form_id:08x}"))
        .join("scene.ron")
}

/// Issue #52: emitted by `poll_preload_parse_tasks` whenever a background
/// manifest parse fails, so `world::swap`'s fallback-swap driver can tell a
/// door-travel fallback load apart from an ambient #51 neighbor-preload
/// failure (both log a `warn!` here either way) and trigger
/// `SwapDecision`'s `ReturnToSource` outcome.
#[derive(Message, Clone, Debug)]
pub(crate) struct PreloadParseFailed {
    pub(crate) form_id: u32,
}

/// Issue #52: starts a background manifest parse for `form_id` exactly like
/// `evaluate_preload_plan`'s `plan.load` loop does, for use by the
/// fallback-swap driver to load a door's destination cell on demand even
/// when it is not a door-graph neighbor of the currently active cell (so it
/// would never be reached by the predictive preloader's own planning).
pub(crate) fn spawn_preload_parse_task(commands: &mut Commands, asset_root: &Path, form_id: u32) {
    info!("preload start {form_id:08x}");
    let path = scene_manifest_path(asset_root, form_id);
    let asset_root = asset_root.to_path_buf();
    let pool = AsyncComputeTaskPool::get();
    let task = pool.spawn(async move {
        let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        let manifest =
            ron::de::from_str::<PreparedSceneManifest>(&text).map_err(|error| error.to_string())?;
        // Read the cell's physics sidecars here too; failures are left for
        // the collider build's lazy `ensure_sidecar_loaded` to warn about.
        let mut seen = HashSet::new();
        let mut sidecars = Vec::new();
        for placement in &manifest.placements {
            if !placement.initially_enabled {
                continue;
            }
            let Some(relative_path) = placement.physics_asset_path.as_ref() else {
                continue;
            };
            if !seen.insert(relative_path.clone()) {
                continue;
            }
            let path = asset_root.join(relative_path.replace('/', std::path::MAIN_SEPARATOR_STR));
            let byte_len = fs::metadata(&path)
                .map(|metadata| metadata.len())
                .unwrap_or(0);
            if let Ok(asset) = crate::vsa::read_physics_asset(&path) {
                sidecars.push(PreloadedSidecar {
                    relative_path: relative_path.clone(),
                    byte_len,
                    asset,
                });
            }
        }
        Ok((manifest, sidecars))
    });
    commands.spawn(PendingPreloadParse { form_id, task });
}

/// F51.2/F51.5 glue: runs once at startup and again whenever `ActiveCell`
/// changes, computes the prepared-neighbor set from which `scenes/<id>/scene.ron`
/// files exist on disk, runs the pure policy, and starts/stops background
/// loads accordingly.
#[allow(clippy::too_many_arguments)]
fn evaluate_preload_plan(
    mut commands: Commands,
    active_cell: Res<ActiveCell>,
    cell_map_index: Res<CellMapIndex>,
    mut resident_cells: ResMut<ResidentCells>,
    resident_cell_limit: Res<ResidentCellLimit>,
    manifest: Res<PreparedSceneManifest>,
    mut pending_reveal: ResMut<super::reveal::PendingReveal>,
    mut pending_captures: ResMut<super::persist::PendingEvictionCaptures>,
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
        spawn_preload_parse_task(&mut commands, asset_root, form_id);
    }

    for form_id in plan.evict {
        if let Some(resident) = resident_cells.0.remove(&form_id) {
            // Issue #61 (F61.1): stage the departing cell for capture;
            // `persist::drain_eviction_captures` snapshots its dynamic/
            // open/taken state into `ActiveSaveState` and only then
            // despawns this root.
            pending_captures.0.push(super::persist::EvictionCapture {
                form_id,
                root: resident.root,
                manifest: Arc::clone(&resident.manifest),
            });
        }
        // Issue #55: the active cell is never evicted (see `policy::
        // CellGraph::plan`'s doc comment), so a mid-reveal cell can't
        // normally reach here -- defense-in-depth, see `PendingReveal::
        // clear_if`'s doc comment.
        pending_reveal.clear_if(form_id);
        let distance = distances
            .get(&form_id)
            .map_or_else(|| "unknown".to_string(), usize::to_string);
        info!("preload evict {form_id:08x} (graph distance {distance})");
    }
}

/// F51.3: polls background manifest parses; on success, spawns the cell's
/// hidden root and point lights immediately, then queues the placements for
/// `advance_pending_cell_spawns` to drain a bounded chunk per frame (not
/// registered as `Ready` until fully spawned and every scene handle finishes
/// loading, see `check_preload_ready`).
fn poll_preload_parse_tasks(
    mut commands: Commands,
    lighting: Res<LightingScale>,
    mut pending: Query<(Entity, &mut PendingPreloadParse)>,
    mut resident_cells: ResMut<ResidentCells>,
    mut pending_spawns: ResMut<PendingCellSpawns>,
    mut physics_assets: ResMut<super::super::player::PreparedPhysicsAssets>,
    mut parse_failed: MessageWriter<PreloadParseFailed>,
) {
    for (entity, mut pending_parse) in &mut pending {
        let Some(result) = check_ready(&mut pending_parse.task) else {
            continue;
        };
        let form_id = pending_parse.form_id;
        commands.entity(entity).despawn();
        match result {
            Ok((manifest, sidecars)) => {
                for sidecar in sidecars {
                    physics_assets.insert_preloaded(
                        sidecar.relative_path,
                        sidecar.byte_len,
                        sidecar.asset,
                    );
                }
                let root = commands
                    .spawn((Transform::default(), Visibility::Hidden))
                    .id();
                spawn_cell_lights(&mut commands, &manifest, root, lighting.0);
                resident_cells.0.insert(
                    form_id,
                    ResidentCell {
                        root,
                        state: ResidentState::Loading,
                        manifest: Arc::new(manifest),
                        scene_handles: Vec::new(),
                        placement_count: 0,
                    },
                );
                pending_spawns.0.push_back(PendingCellSpawn {
                    form_id,
                    root,
                    next_index: 0,
                });
            }
            Err(error) => {
                warn!("preload parse failed for {form_id:08x}: {error}");
                parse_failed.write(PreloadParseFailed { form_id });
            }
        }
    }
}

/// Drains the front of the pending-spawn queue at
/// `PRELOAD_SPAWN_BUDGET_PER_FRAME` raw placement entries per frame. Cells
/// evicted (or whose root was replaced) while still queued are dropped.
fn advance_pending_cell_spawns(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut resident_cells: ResMut<ResidentCells>,
    mut pending_spawns: ResMut<PendingCellSpawns>,
) {
    let Some(pending) = pending_spawns.0.front_mut() else {
        return;
    };
    let Some(resident) = resident_cells
        .0
        .get_mut(&pending.form_id)
        .filter(|resident| resident.root == pending.root)
    else {
        pending_spawns.0.pop_front();
        return;
    };
    let manifest = Arc::clone(&resident.manifest);
    let (content, next_index) = spawn_cell_placements_chunk(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        &manifest,
        pending.root,
        None,
        pending.next_index,
        PRELOAD_SPAWN_BUDGET_PER_FRAME,
    );
    resident.scene_handles.extend(content.scene_handles);
    resident.placement_count += content.placement_count;
    pending.next_index = next_index;
    if next_index >= manifest.placements.len() {
        pending_spawns.0.pop_front();
    }
}

/// F51.3: promotes `Loading` resident cells to `Ready` once fully spawned
/// and every spawned scene handle has finished loading, logging
/// `preload ready` exactly once.
fn check_preload_ready(
    asset_server: Res<AssetServer>,
    mut resident_cells: ResMut<ResidentCells>,
    pending_spawns: Res<PendingCellSpawns>,
) {
    for (form_id, resident) in resident_cells.0.iter_mut() {
        if resident.state != ResidentState::Loading || pending_spawns.contains(*form_id) {
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
