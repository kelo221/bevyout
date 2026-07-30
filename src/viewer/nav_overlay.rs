//! `tnm`/`togglenavmesh` console command (issue #128): visualizes the
//! prepared per-cell navigation graph (`vsa::prepare::nav_graph`'s
//! `navgraph.ron`, issue #111) as one flat, distinct color per triangle.
//!
//! First activation in a cell reads `PreparedSceneManifest::nav_graph`'s
//! `asset_path` (relative to `manifest.asset_root`, resolved the same way
//! `player::PreparedPhysicsAssets` resolves `physics_asset_path` -- a plain
//! `fs::read_to_string` + `ron::de::from_str`, not the `AssetServer`, since
//! this is QA tooling reached once per toggle rather than a renderable
//! asset), builds one `Mesh` per `PreparedNavMesh` with per-triangle flat
//! vertex colors, and spawns it hidden-or-shown under one root entity.
//! Later `tnm` calls just flip that root's `Visibility` (`toggle_nav_mesh`
//! never rebuilds an already-current overlay). `despawn_stale_nav_overlay`
//! -- installed alongside the console command in `console::install` --
//! notices when the active cell (`PreparedSceneManifest::cell::form_id`)
//! no longer matches the overlay's, and despawns it so a swap never leaves
//! a stale overlay floating in the newly active cell's geometry; the next
//! `tnm` there rebuilds from scratch.
//!
//! Nav-mesh vertices are already Bevy metres (`vsa::prepare::nav_graph`'s
//! `to_bevy_position` applies `FO3_SCALE` once, at prepare time) -- no
//! coordinate conversion happens here.
//!
//! Issue #138 adds two things on top of the #128 triangle overlay:
//!
//! - The active test agent's current route, drawn as a bright white
//!   `LineList` polyline child of the same overlay root (so it toggles and
//!   despawns with the rest of the overlay). The route is read straight off
//!   `bevy_landmass`'s own public debug-draw API
//!   (`bevy_landmass::debug::draw_archipelago_debug` filtered down to
//!   `LineType::AgentCorridor` lines for the lowest-`Entity` agent found --
//!   "agent 0", per the issue's spec) rather than any hook into
//!   `nav/agent.rs`'s private state, so this module stays fully decoupled
//!   from that parallel-wave rewrite. `despawn_stale_nav_overlay` was
//!   already polled every `Update` for cell swaps; it now also refreshes
//!   this polyline in place (same `Mesh` handle, no entity churn) whenever
//!   the collected corridor segments differ from last frame's -- i.e. on
//!   repath, not on every frame, even though the check itself runs every
//!   frame the overlay is visible.
//! - `OVERLAY_ALPHA`/`TRIANGLE_LIGHTNESS` are both dimmed from their #128
//!   values. The un-dimmed overlay (0.55 alpha, full HSL lightness) could
//!   cover most of the screen with saturated, unlit color and was bright
//!   enough to push the scene's `AutoExposure` histogram up, crushing a
//!   dark interior to black the instant `tnm` was toggled on. `AutoExposure`
//!   only offers a screen-space `metering_mask` (see `scene.rs`'s
//!   `camera_post_processing`), which can't track a 3D overlay's
//!   ever-changing screen footprint without a new per-frame render system.
//!   Dimming the material constants alone (#138's first pass at 0.28/0.22,
//!   then this follow-up's 0.12/0.12) turned out not to be fixable by a
//!   constant at all: `AutoExposure`'s adaptation is applied to the
//!   *rendered* image after the overlay's colors are already resolved, and
//!   Bevy 0.19's implementation is GPU-side histogram metering with no
//!   CPU-readable adapted value -- in a dark interior the exposure gain can
//!   be 8x or more, so any fixed material color eventually saturates on
//!   screen regardless of how low it is set. The actual fix
//!   (`lock_exposure_for_overlay`/`unlock_exposure_for_overlay`) neutralizes
//!   exposure entirely while the overlay is visible: `AutoExposure` is
//!   removed from the camera and `Exposure` is pinned to the same fixed
//!   baseline the camera spawns with (`scene.rs`'s `Exposure { ev100: 12.0
//!   }`), restoring the camera's exact prior `Exposure`/`AutoExposure` the
//!   moment the overlay is hidden again (`NavOverlayExposureLock`). This is
//!   a deliberate debug-overlay UX tradeoff: the rest of the scene stops
//!   adapting to brightness changes for as long as `tnm` is on, in exchange
//!   for the overlay's own colors finally rendering at a predictable,
//!   fixed brightness instead of at whatever the current auto-exposure
//!   gain happens to be. The fixed-white route polyline (`PATH_LINE_COLOR`)
//!   is untouched by any of this -- it is the thing actually being read and
//!   can stay brighter than the triangle fill beneath it.
//! - Even with exposure locked, a real-data screenshot still showed the
//!   fill fully opaque and saturated at the 0.12/0.12 constants above --
//!   floor texture not visible through it at all. The transparency had
//!   been carried entirely by `Mesh::ATTRIBUTE_COLOR`'s per-vertex alpha
//!   channel, which was not reliably blending on screen for this unlit
//!   material (per-vertex *hue* clearly was working -- distinct colors per
//!   triangle rendered correctly -- so this was specifically an
//!   alpha-channel pitfall, not a missing `VERTEX_COLORS` pipeline
//!   feature). The fix moves the actual transparency into the *material's*
//!   `base_color` alpha instead (`spawn_nav_mesh_overlay`'s fill material
//!   is now `Color::WHITE.with_alpha(OVERLAY_ALPHA)`, with
//!   `build_triangle_mesh`'s vertex colors always fully opaque) --
//!   `bevy_pbr`'s fragment shader (`pbr_input.material.base_color *=
//!   base_color`) unconditionally multiplies the material's own base color
//!   into every pixel regardless of vertex-color pipeline behavior, so
//!   this is the one alpha path guaranteed to actually apply. With that
//!   real bug fixed, real-data screenshots (agent bridge + a window
//!   capture bypassing this sandbox's screen-occlusion quirk) drove
//!   `OVERLAY_ALPHA`/`TRIANGLE_LIGHTNESS` down further still (0.1/0.015):
//!   even a "dim" 0.12 lightness unlit color reads as a strong, saturated
//!   fill once blended over an already near-black floor and put through
//!   `AcesFitted` tonemapping's shadow lift, so the constants needed
//!   another real reduction, not just the exposure fix, to leave the floor
//!   texture actually visible through the fill.

use std::path::Path;

use anyhow::Context;
use bevy::asset::RenderAssetUsages;
use bevy::camera::Exposure;
use bevy::material::AlphaMode;
use bevy::mesh::PrimitiveTopology;
use bevy::post_process::auto_exposure::AutoExposure;
use bevy::prelude::*;
use bevy_landmass::coords::ThreeD;
use bevy_landmass::debug::{
    DebugDrawer, LineType, PointType, TriangleType, draw_archipelago_debug,
};
use bevy_landmass::{Agent3d, Archipelago3d};
use serde_json::json;

use crate::console::{ConsoleCommandResult, ConsoleError, ConsoleInvocation};
#[cfg(test)]
use crate::vsa::PreparedSceneManifest;
use crate::vsa::{PreparedNavGraph, PreparedNavMesh};

/// Golden-ratio hue step (issue #128's spec): consecutive polygon indices
/// land far apart on the hue wheel instead of drifting slowly, so adjacent
/// triangles read as visually distinct even though they're numbered
/// sequentially.
const HUE_STEP: f32 = 0.618_034;
/// Some transparency so the geometry underneath the overlay stays readable.
/// Lowered from #128's 0.55 (issue #138 feature 2), then twice more in the
/// #138 follow-up: first to 0.28 (still "way too bright" on human
/// acceptance), then -- after fixing the real bug (see the module doc
/// comment: this is now the *material's* `base_color` alpha, not vertex
/// alpha) and locking exposure -- to this value, chosen by iterating
/// against real agent-bridge screenshots of Vault 101 Entrance until the
/// floor texture was clearly visible through the fill.
const OVERLAY_ALPHA: f32 = 0.1;
/// HSL lightness used for every triangle's hue (issue #138 feature 2,
/// down from #128's fixed 0.5, then dimmed twice more in the #138
/// follow-up -- same real-screenshot iteration as
/// `OVERLAY_ALPHA`). Saturation stays at 1.0 so polygons remain visually
/// distinct; only the overall brightness is dimmed.
const TRIANGLE_LIGHTNESS: f32 = 0.015;
/// Offset against z-fighting with the floor/nav-mesh-adjacent geometry.
const OVERLAY_Y_OFFSET: f32 = 0.02;
/// Additional height (relative to the triangle mesh, i.e. on top of
/// `OVERLAY_Y_OFFSET`) the active-agent route polyline is drawn at, so it
/// never z-fights with the triangle overlay sitting directly beneath it.
const PATH_Y_OFFSET: f32 = 0.03;
/// Fully opaque white (issue #138 feature 1): deliberately not routed
/// through `triangle_color`'s hue wheel, so the route always reads as "the
/// path" regardless of which polygon hue it happens to cross. Full opacity
/// is fine here -- feature 2's auto-exposure concern is about the
/// triangle overlay's large screen-covering area, not a handful of 1px
/// line pixels.
const PATH_LINE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
/// Fixed camera exposure the `tnm` overlay locks to while visible (#138
/// follow-up): the same baseline `scene.rs`'s `spawn_scene_camera` gives the
/// camera at spawn time, before any `AutoExposure` adaptation
/// (`Exposure { ev100: 12.0 }`). Locking to this specific value (rather
/// than, say, a brighter or darker fixed EV chosen just for the overlay)
/// keeps the rest of the scene's baseline look consistent with what it
/// would render as if `AutoExposure` were simply absent.
const OVERLAY_LOCKED_EV100: f32 = 12.0;

/// Marks the overlay's root entity (and its per-mesh children) for the
/// benefit of tests and any future diagnostics query; not otherwise queried
/// at runtime.
#[derive(Component)]
pub(crate) struct NavMeshOverlayRoot;

/// Marks the active-agent route polyline child (issue #138) for the same
/// reason `NavMeshOverlayRoot` marks the root.
#[derive(Component)]
pub(crate) struct NavAgentPathOverlay;

#[derive(Clone, Copy)]
struct NavMeshOverlay {
    entity: Entity,
    cell_form_id: u32,
    mesh_count: usize,
    triangle_count: usize,
    visible: bool,
}

/// Rust-side bookkeeping for the route polyline (issue #138): the stable
/// `Mesh` handle it's rebuilt in place under, and the last collected
/// corridor segments (so `refresh_active_agent_path_overlay` only touches
/// the `Assets<Mesh>` entry when a repath actually changed the route).
struct NavAgentPathState {
    mesh: Handle<Mesh>,
    last_segments: Vec<[Vec3; 2]>,
}

#[derive(Resource, Default)]
pub(crate) struct NavMeshOverlayState {
    overlay: Option<NavMeshOverlay>,
    path: Option<NavAgentPathState>,
}

/// The camera's exposure state as it was the moment the `tnm` overlay last
/// locked it (issue #138 follow-up). `None` means exposure is not currently
/// locked -- either the overlay has never been toggled on, or it was toggled
/// off again and the saved state already restored.
#[derive(Resource, Default)]
pub(crate) struct NavOverlayExposureLock {
    saved: Option<SavedExposure>,
}

struct SavedExposure {
    camera: Entity,
    exposure: Exposure,
    auto_exposure: Option<AutoExposure>,
}

/// Locks the camera's exposure to a fixed baseline while the overlay is
/// visible: see the module doc comment for why constant material dimming
/// alone cannot compensate for `AutoExposure`'s GPU-side adaptation. Removes
/// `AutoExposure` from the (lowest-`Entity`, matching the rest of this
/// module's "agent 0"/"first camera found" convention) `Camera3d` entity and
/// pins `Exposure` to `OVERLAY_LOCKED_EV100`, saving whatever was there
/// before so `unlock_exposure_for_overlay` can restore it exactly. A no-op
/// if exposure is already locked (repeated visibility toggles within the
/// same overlay lifetime must not clobber the originally-saved state) or no
/// camera entity exists yet (e.g. a test `World` with no camera spawned).
fn lock_exposure_for_overlay(world: &mut World) {
    if world.resource::<NavOverlayExposureLock>().saved.is_some() {
        return;
    }
    let Some(camera) = world
        .query_filtered::<Entity, With<Camera3d>>()
        .iter(world)
        .next()
    else {
        return;
    };
    let Ok(mut entity) = world.get_entity_mut(camera) else {
        return;
    };
    let exposure = entity.get::<Exposure>().copied().unwrap_or(Exposure {
        ev100: OVERLAY_LOCKED_EV100,
    });
    let auto_exposure = entity.get::<AutoExposure>().cloned();
    entity.remove::<AutoExposure>();
    entity.insert(Exposure {
        ev100: OVERLAY_LOCKED_EV100,
    });
    world.resource_mut::<NavOverlayExposureLock>().saved = Some(SavedExposure {
        camera,
        exposure,
        auto_exposure,
    });
}

/// Restores whatever `lock_exposure_for_overlay` saved. A no-op if exposure
/// was never locked (already restored, or never locked in the first place),
/// or the camera entity has since been despawned.
fn unlock_exposure_for_overlay(world: &mut World) {
    let Some(saved) = world.resource_mut::<NavOverlayExposureLock>().saved.take() else {
        return;
    };
    let Ok(mut entity) = world.get_entity_mut(saved.camera) else {
        return;
    };
    entity.insert(saved.exposure);
    if let Some(auto_exposure) = saved.auto_exposure {
        entity.insert(auto_exposure);
    }
}

/// Deterministic per-polygon hue fraction in `[0, 1)` (issue #128's spec:
/// `hue = (polygon.index as f32 * 0.618_034) % 1.0`). Pure so it can be unit
/// tested without spinning up a `World`.
pub(crate) fn polygon_hue_fraction(polygon_index: u32) -> f32 {
    (polygon_index as f32 * HUE_STEP) % 1.0
}

fn triangle_color(polygon_index: u32, alpha: f32) -> [f32; 4] {
    let hue_degrees = polygon_hue_fraction(polygon_index) * 360.0;
    LinearRgba::from(Hsla::hsl(hue_degrees, 1.0, TRIANGLE_LIGHTNESS).with_alpha(alpha))
        .to_f32_array()
}

/// Resolves `manifest.nav_graph`'s `asset_path` against `asset_root` and
/// reads+parses `navgraph.ron`, the same way `player::PreparedPhysicsAssets`
/// resolves and reads a `physics_asset_path` sidecar.
fn read_nav_graph(path: &Path) -> anyhow::Result<PreparedNavGraph> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("nav graph does not exist: {}", path.display()))?;
    ron::de::from_str(&text).with_context(|| format!("invalid nav graph RON: {}", path.display()))
}

/// Builds one `Mesh` per `PreparedNavMesh` that has at least one polygon
/// (an empty mesh would just waste a draw call), positions duplicated per
/// triangle (3 unique vertices per polygon, no shared indices) so each
/// triangle can carry its own flat `ATTRIBUTE_COLOR`. A polygon carrying the
/// `nav_graph::build_mesh` `u32::MAX` invalid-vertex-index sentinel (or any
/// other index past `mesh.vertices`) is skipped entirely rather than
/// rendered as a garbage triangle at the origin -- the out-of-range error
/// diagnostic at prepare time already covers it. Returns the mesh and the
/// number of triangles actually emitted (excluding skipped polygons), so
/// callers report an accurate triangle count.
fn build_triangle_mesh(mesh: &PreparedNavMesh) -> (Mesh, usize) {
    let mut positions = Vec::with_capacity(mesh.polygons.len() * 3);
    let mut colors = Vec::with_capacity(mesh.polygons.len() * 3);
    let mut triangle_count = 0usize;
    for polygon in &mesh.polygons {
        if polygon
            .vertex_indices
            .iter()
            .any(|&index| index as usize >= mesh.vertices.len())
        {
            continue;
        }
        // Issue #138 follow-up (visual verification found the fill still
        // rendering fully opaque/saturated even at OVERLAY_ALPHA 0.12):
        // vertex-color alpha alone was not reliably blending on screen for
        // this unlit material/mesh combination. The transparency now lives
        // in the *material's* `base_color` alpha instead (see
        // `spawn_nav_mesh_overlay`), which `pbr_fragment.wgsl` always
        // multiplies into the final pixel unconditionally
        // (`pbr_input.material.base_color *= base_color`) regardless of
        // any vertex-color pipeline behavior -- so per-vertex color here
        // only ever needs to carry full opacity.
        let color = triangle_color(polygon.index, 1.0);
        for vertex_index in polygon.vertex_indices {
            positions.push(mesh.vertices[vertex_index as usize]);
            colors.push(color);
        }
        triangle_count += 1;
    }
    (
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors),
        triangle_count,
    )
}

/// Spawns the overlay root (offset `+OVERLAY_Y_OFFSET` metres in Y) plus one
/// child entity per non-empty `PreparedNavMesh`, sharing a single unlit,
/// double-sided, alpha-blended material (vertex colors carry the actual
/// per-triangle hue), plus one more child (issue #138) for the active
/// agent's route polyline -- initially empty, filled in by the first
/// `refresh_active_agent_path_overlay` call. Returns the root entity, the
/// number of triangle meshes built, the total triangle count across them,
/// and the route polyline's `Mesh` handle (for `NavAgentPathState`).
fn spawn_nav_mesh_overlay(
    world: &mut World,
    graph: &PreparedNavGraph,
) -> (Entity, usize, usize, Handle<Mesh>) {
    let root = world
        .spawn((
            NavMeshOverlayRoot,
            Transform::from_xyz(0.0, OVERLAY_Y_OFFSET, 0.0),
            Visibility::Inherited,
        ))
        .id();
    // Issue #138 follow-up: the fill's transparency lives in the
    // *material's* `base_color` alpha, not per-vertex `ATTRIBUTE_COLOR`
    // alpha (see `build_triangle_mesh`'s doc comment for why) -- white RGB
    // (so it never tints the per-triangle hue, which vertex colors already
    // carry at full strength) at `OVERLAY_ALPHA`.
    let material = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: Color::WHITE.with_alpha(OVERLAY_ALPHA),
            unlit: true,
            cull_mode: None,
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

    let mut mesh_count = 0usize;
    let mut triangle_count = 0usize;
    for mesh in &graph.meshes {
        if mesh.polygons.is_empty() {
            continue;
        }
        let (bevy_mesh, mesh_triangle_count) = build_triangle_mesh(mesh);
        // Every polygon in this mesh carried an invalid vertex index (the
        // out-of-range error diagnostic already covers it) -- skip spawning
        // an empty mesh entity/draw call for it.
        if mesh_triangle_count == 0 {
            continue;
        }
        triangle_count += mesh_triangle_count;
        mesh_count += 1;
        let mesh_handle = world.resource_mut::<Assets<Mesh>>().add(bevy_mesh);
        let child = world
            .spawn((
                Mesh3d(mesh_handle),
                MeshMaterial3d(material.clone()),
                Transform::IDENTITY,
            ))
            .id();
        world.entity_mut(root).add_child(child);
    }

    let path_material = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            cull_mode: None,
            alpha_mode: AlphaMode::Opaque,
            ..default()
        });
    let path_mesh_handle = world
        .resource_mut::<Assets<Mesh>>()
        .add(build_path_mesh(&[]));
    let path_child = world
        .spawn((
            NavAgentPathOverlay,
            Mesh3d(path_mesh_handle.clone()),
            MeshMaterial3d(path_material),
            Transform::from_xyz(0.0, PATH_Y_OFFSET, 0.0),
        ))
        .id();
    world.entity_mut(root).add_child(path_child);

    (root, mesh_count, triangle_count, path_mesh_handle)
}

/// Builds a `LineList` mesh from disconnected route segments (issue #138):
/// every pair of positions is one independently-colored line, so segment
/// order/connectivity from the debug-draw source doesn't need to form a
/// single contiguous strip. An empty `segments` slice yields a valid,
/// zero-vertex mesh (the initial state before the first repath).
fn build_path_mesh(segments: &[[Vec3; 2]]) -> Mesh {
    let mut positions = Vec::with_capacity(segments.len() * 2);
    for [start, end] in segments {
        positions.push(start.to_array());
        positions.push(end.to_array());
    }
    let colors = vec![PATH_LINE_COLOR; positions.len()];
    Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
}

/// Filters `bevy_landmass`'s own public debug-draw output down to just the
/// chosen agent's route: the `AgentCorridor` lines the corridor follows
/// across polygon nodes (`bevy_landmass::debug::LineType`'s own doc: "the
/// corridor follows the path along nodes, not the actual path the agent
/// will travel" -- close enough for a debug overlay, and it's what makes
/// the route visible crossing a mesh seam per the issue's acceptance).
/// Everything else `draw_archipelago_debug` can report (boundary/
/// connectivity edges, other agents/animation links, nav-mesh triangles) is
/// dropped.
struct AgentCorridorCollector {
    agent: Entity,
    segments: Vec<[Vec3; 2]>,
}

impl DebugDrawer<ThreeD> for AgentCorridorCollector {
    fn add_point(&mut self, _point_type: PointType, _point: Vec3) {}

    fn add_line(&mut self, line_type: LineType, line: [Vec3; 2]) {
        if line_type == LineType::AgentCorridor(self.agent) {
            self.segments.push(line);
        }
    }

    fn add_triangle(&mut self, _triangle_type: TriangleType, _triangle: [Vec3; 3]) {}
}

/// The lowest-`Entity` `bevy_landmass` agent's current route ("agent 0" per
/// issue #138's spec -- multi-agent support may land in parallel via #114,
/// but drawing just one agent's path is sufficient for this wave), read
/// straight off `bevy_landmass::debug::draw_archipelago_debug`'s public
/// API. No agent or archipelago in the world yields an empty route rather
/// than an error -- this is a best-effort debug overlay, not something
/// that should ever panic or block `tnm`.
fn active_agent_corridor(world: &mut World) -> Vec<[Vec3; 2]> {
    let mut agents = world.query_filtered::<Entity, With<Agent3d>>();
    let Some(agent) = agents.iter(world).min() else {
        return Vec::new();
    };
    let mut archipelagos = world.query::<&Archipelago3d>();
    let Some(archipelago) = archipelagos.iter(world).next() else {
        return Vec::new();
    };
    let mut collector = AgentCorridorCollector {
        agent,
        segments: Vec::new(),
    };
    if let Err(error) = draw_archipelago_debug(archipelago, &mut collector) {
        warn!("nav agent path overlay debug draw failed: {error}");
        return Vec::new();
    }
    collector.segments
}

/// Rebuilds the route polyline's `Mesh` in place (same handle, no entity
/// churn) when the collected corridor differs from last frame's -- i.e. on
/// repath. A no-op (besides the cheap collection + comparison) on every
/// other frame the overlay is visible, and entirely skipped by the caller
/// while the overlay is absent or hidden.
fn refresh_active_agent_path_overlay(world: &mut World) {
    let Some(mesh_handle) = world
        .resource::<NavMeshOverlayState>()
        .path
        .as_ref()
        .map(|path| path.mesh.clone())
    else {
        return;
    };
    let segments = active_agent_corridor(world);
    let unchanged = world
        .resource::<NavMeshOverlayState>()
        .path
        .as_ref()
        .is_some_and(|path| path.last_segments == segments);
    if unchanged {
        return;
    }
    let mesh = build_path_mesh(&segments);
    // `insert` only errors on a stale/dead handle generation, which can't
    // happen here -- `mesh_handle` was cloned from live `NavMeshOverlayState`
    // moments ago and nothing else touches this handle's slot.
    if let Err(error) = world
        .resource_mut::<Assets<Mesh>>()
        .insert(mesh_handle.id(), mesh)
    {
        warn!("nav agent path overlay mesh update failed: {error}");
        return;
    }
    if let Some(path) = world.resource_mut::<NavMeshOverlayState>().path.as_mut() {
        path.last_segments = segments;
    }
}

fn no_nav_graph_error() -> ConsoleError {
    ConsoleError::new("no_nav_graph", "no nav graph prepared for this cell")
}

fn nav_mesh_toggle_reply(
    visible: bool,
    mesh_count: usize,
    triangle_count: usize,
) -> ConsoleCommandResult {
    let line = if visible {
        format!("nav mesh visualization on ({mesh_count} meshes, {triangle_count} triangles)")
    } else {
        "nav mesh visualization off".to_string()
    };
    ConsoleCommandResult::new(
        json!({
            "visible": visible,
            "meshes": mesh_count,
            "triangles": triangle_count,
        }),
        vec![line],
    )
}

/// `tnm`/`togglenavmesh` handler.
pub(crate) fn toggle_nav_mesh(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if !invocation.args.is_empty() {
        return Err(ConsoleError::new(
            "bad_arity",
            "tnm does not accept arguments",
        ));
    }

    let (current_cell, asset_root, nav_graph_source) = {
        let manifest = world.resource::<crate::viewer::LoadedSceneManifest>();
        (
            manifest.cell.form_id,
            std::path::PathBuf::from(&manifest.asset_root),
            manifest.nav_graph.clone(),
        )
    };

    if let Some(overlay) = world.resource::<NavMeshOverlayState>().overlay {
        if overlay.cell_form_id == current_cell {
            let visible = !overlay.visible;
            if let Ok(mut entity) = world.get_entity_mut(overlay.entity)
                && let Some(mut visibility) = entity.get_mut::<Visibility>()
            {
                *visibility = if visible {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
            world.resource_mut::<NavMeshOverlayState>().overlay =
                Some(NavMeshOverlay { visible, ..overlay });
            if visible {
                lock_exposure_for_overlay(world);
            } else {
                unlock_exposure_for_overlay(world);
            }
            return Ok(nav_mesh_toggle_reply(
                visible,
                overlay.mesh_count,
                overlay.triangle_count,
            ));
        }
        // Stale overlay left over from a cell we've since swapped away
        // from -- `despawn_stale_nav_overlay` normally clears this before
        // `tnm` runs again, but a same-frame race (swap and `tnm` landing
        // in the same Update) is defended against here too.
        if let Ok(entity) = world.get_entity_mut(overlay.entity) {
            entity.despawn();
        }
        let mut state = world.resource_mut::<NavMeshOverlayState>();
        state.overlay = None;
        state.path = None;
        unlock_exposure_for_overlay(world);
    }

    let Some(source) = nav_graph_source else {
        return Err(no_nav_graph_error());
    };
    let path = asset_root.join(
        source
            .asset_path
            .replace('/', std::path::MAIN_SEPARATOR_STR),
    );
    let graph = match read_nav_graph(&path) {
        Ok(graph) => graph,
        Err(error) => {
            warn!("nav graph read failed at {}: {error:#}", path.display());
            return Err(no_nav_graph_error());
        }
    };

    let (entity, mesh_count, triangle_count, path_mesh) = spawn_nav_mesh_overlay(world, &graph);
    let mut state = world.resource_mut::<NavMeshOverlayState>();
    state.overlay = Some(NavMeshOverlay {
        entity,
        cell_form_id: current_cell,
        mesh_count,
        triangle_count,
        visible: true,
    });
    state.path = Some(NavAgentPathState {
        mesh: path_mesh,
        last_segments: Vec::new(),
    });
    lock_exposure_for_overlay(world);
    Ok(nav_mesh_toggle_reply(true, mesh_count, triangle_count))
}

/// Despawns the overlay the moment the active cell no longer matches the
/// one it was built for (a door swap, instant or fallback, always
/// repoints `PreparedSceneManifest` -- see `world::swap::activate_resident_cell`),
/// and (issue #138) otherwise refreshes the active agent's route polyline
/// in place while the overlay is visible. Despawn is a cheap integer
/// compare every frame; the route refresh is skipped entirely while the
/// overlay is absent or hidden, and even while visible only touches
///// typed `Query`/`Res` params) so it can share `active_agent_corridor`'s
/// `World`-based query construction without a second, parallel parameter
/// list. Also restores the camera's exposure (`unlock_exposure_for_overlay`,
/// #138 follow-up) on a cell-swap despawn -- from the player's perspective
/// the overlay is now off, so its exposure lock must not outlive it.
pub(crate) fn despawn_stale_nav_overlay(world: &mut World) {
    let Some(overlay) = world.resource::<NavMeshOverlayState>().overlay else {
        return;
    };
    if let Some(manifest) = world.get_resource::<crate::viewer::LoadedSceneManifest>()
        && overlay.cell_form_id != manifest.cell.form_id
    {
        if let Ok(entity) = world.get_entity_mut(overlay.entity) {
            entity.despawn();
        }
        let mut state = world.resource_mut::<NavMeshOverlayState>();
        state.overlay = None;
        state.path = None;
        unlock_exposure_for_overlay(world);
        return;
    }
    if overlay.visible {
        refresh_active_agent_path_overlay(world);
    }
}

#[cfg(test)]
#[path = "tests/nav_overlay.rs"]
mod tests;
