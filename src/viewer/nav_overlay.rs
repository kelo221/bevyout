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

use std::path::Path;

use anyhow::Context;
use bevy::asset::RenderAssetUsages;
use bevy::material::AlphaMode;
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;
use serde_json::json;

use crate::console::{ConsoleCommandResult, ConsoleError, ConsoleInvocation};
use crate::vsa::{PreparedNavGraph, PreparedNavMesh, PreparedSceneManifest};

/// Golden-ratio hue step (issue #128's spec): consecutive polygon indices
/// land far apart on the hue wheel instead of drifting slowly, so adjacent
/// triangles read as visually distinct even though they're numbered
/// sequentially.
const HUE_STEP: f32 = 0.618_034;
/// Some transparency so the geometry underneath the overlay stays readable.
const OVERLAY_ALPHA: f32 = 0.55;
/// Offset against z-fighting with the floor/nav-mesh-adjacent geometry.
const OVERLAY_Y_OFFSET: f32 = 0.02;

/// Marks the overlay's root entity (and its per-mesh children) for the
/// benefit of tests and any future diagnostics query; not otherwise queried
/// at runtime.
#[derive(Component)]
pub(crate) struct NavMeshOverlayRoot;

#[derive(Clone, Copy)]
struct NavMeshOverlay {
    entity: Entity,
    cell_form_id: u32,
    mesh_count: usize,
    triangle_count: usize,
    visible: bool,
}

#[derive(Resource, Default)]
pub(crate) struct NavMeshOverlayState {
    overlay: Option<NavMeshOverlay>,
}

/// Deterministic per-polygon hue fraction in `[0, 1)` (issue #128's spec:
/// `hue = (polygon.index as f32 * 0.618_034) % 1.0`). Pure so it can be unit
/// tested without spinning up a `World`.
pub(crate) fn polygon_hue_fraction(polygon_index: u32) -> f32 {
    (polygon_index as f32 * HUE_STEP) % 1.0
}

fn triangle_color(polygon_index: u32, alpha: f32) -> [f32; 4] {
    let hue_degrees = polygon_hue_fraction(polygon_index) * 360.0;
    LinearRgba::from(Hsla::hsl(hue_degrees, 1.0, 0.5).with_alpha(alpha)).to_f32_array()
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
/// triangle can carry its own flat `ATTRIBUTE_COLOR`.
fn build_triangle_mesh(mesh: &PreparedNavMesh) -> Mesh {
    let mut positions = Vec::with_capacity(mesh.polygons.len() * 3);
    let mut colors = Vec::with_capacity(mesh.polygons.len() * 3);
    for polygon in &mesh.polygons {
        let color = triangle_color(polygon.index, OVERLAY_ALPHA);
        for vertex_index in polygon.vertex_indices {
            let position = mesh
                .vertices
                .get(vertex_index as usize)
                .copied()
                .unwrap_or_default();
            positions.push(position);
            colors.push(color);
        }
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
}

/// Spawns the overlay root (offset `+OVERLAY_Y_OFFSET` metres in Y) plus one
/// child entity per non-empty `PreparedNavMesh`, sharing a single unlit,
/// double-sided, alpha-blended material (vertex colors carry the actual
/// per-triangle hue). Returns the root entity, the number of meshes built,
/// and the total triangle count across them.
fn spawn_nav_mesh_overlay(world: &mut World, graph: &PreparedNavGraph) -> (Entity, usize, usize) {
    let root = world
        .spawn((
            NavMeshOverlayRoot,
            Transform::from_xyz(0.0, OVERLAY_Y_OFFSET, 0.0),
            Visibility::Inherited,
        ))
        .id();
    let material = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: Color::WHITE,
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
        let bevy_mesh = build_triangle_mesh(mesh);
        triangle_count += mesh.polygons.len();
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
    (root, mesh_count, triangle_count)
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
        let manifest = world.resource::<PreparedSceneManifest>();
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
        world.resource_mut::<NavMeshOverlayState>().overlay = None;
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

    let (entity, mesh_count, triangle_count) = spawn_nav_mesh_overlay(world, &graph);
    world.resource_mut::<NavMeshOverlayState>().overlay = Some(NavMeshOverlay {
        entity,
        cell_form_id: current_cell,
        mesh_count,
        triangle_count,
        visible: true,
    });
    Ok(nav_mesh_toggle_reply(true, mesh_count, triangle_count))
}

/// Despawns the overlay the moment the active cell no longer matches the
/// one it was built for (a door swap, instant or fallback, always
/// repoints `PreparedSceneManifest` -- see `world::swap::activate_resident_cell`).
/// A cheap integer compare every frame; does no mesh/material work, so it
/// costs nothing while the overlay is absent or hidden. `PreparedSceneManifest`
/// is `Option`-wrapped rather than required: this system is registered
/// unconditionally alongside the console command (like `sync_ui_visibility`),
/// and console-harness tests that never insert a manifest must not panic.
pub(crate) fn despawn_stale_nav_overlay(
    mut commands: Commands,
    manifest: Option<Res<PreparedSceneManifest>>,
    mut state: ResMut<NavMeshOverlayState>,
) {
    let Some(overlay) = state.overlay else {
        return;
    };
    let Some(manifest) = manifest else {
        return;
    };
    if overlay.cell_form_id == manifest.cell.form_id {
        return;
    }
    commands.entity(overlay.entity).despawn();
    state.overlay = None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::console::ConsoleSessionId;
    use crate::vsa::{CellInfo, PreparedNavGraphSource, PreparedNavPolygon};
    use bevy::ecs::system::RunSystemOnce;

    fn invocation() -> ConsoleInvocation {
        ConsoleInvocation {
            request_id: 1,
            frame: 1,
            session: ConsoleSessionId::new("test"),
            command: "tnm".into(),
            args: Vec::new(),
            target: None,
        }
    }

    fn minimal_cell(form_id: u32) -> CellInfo {
        CellInfo {
            form_id,
            editor_id: None,
            name: None,
            interior: true,
            ambient_rgba: [0.0; 4],
            directional_rgba: [0.0; 4],
            image_space_form_id: None,
            image_space: None,
            lighting_template_form_id: None,
            lighting_template_flags: 0,
            lighting_template: None,
            raw_lighting: None,
            effective_lighting: None,
            water_form_id: None,
            water_height: None,
            grid: None,
            worldspace_form_id: None,
        }
    }

    fn minimal_manifest(
        cell_form_id: u32,
        asset_root: String,
        nav_graph: Option<PreparedNavGraphSource>,
    ) -> PreparedSceneManifest {
        PreparedSceneManifest {
            schema_version: 16,
            prepare_revision: None,
            converter_revision: None,
            physics_schema_version: None,
            asset_root,
            source_plugin: "Fallout3.esm".into(),
            source_fingerprint: "content-hash".into(),
            item_catalog_path: None,
            item_catalog_revision: None,
            item_catalog_hash: None,
            recipe_catalog_path: None,
            recipe_catalog_revision: None,
            recipe_catalog_hash: None,
            actor_catalog_path: None,
            actor_catalog_revision: None,
            actor_catalog_hash: None,
            source_plugins: Vec::new(),
            visual_issues: Vec::new(),
            cell: minimal_cell(cell_form_id),
            placements: Vec::new(),
            lights: Vec::new(),
            diagnostics: Vec::new(),
            navmeshes: Vec::new(),
            nav_graph,
            cell_audio: Default::default(),
            audio_clips: Vec::new(),
            footstep_sets: Vec::new(),
            hard_landing_clips: Vec::new(),
            bake: None,
            static_point_shadows: None,
            mutability_summary: Default::default(),
            leveled_lists: Default::default(),
        }
    }

    fn two_triangle_graph() -> PreparedNavGraph {
        let mesh = PreparedNavMesh {
            form_id: 0x10,
            vertices: vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 1.0],
            ],
            polygons: vec![
                PreparedNavPolygon {
                    index: 0,
                    vertex_indices: [0, 1, 2],
                    ..Default::default()
                },
                PreparedNavPolygon {
                    index: 1,
                    vertex_indices: [1, 3, 2],
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        PreparedNavGraph {
            meshes: vec![mesh],
            ..Default::default()
        }
    }

    /// Writes a synthetic `navgraph.ron` (never Bethesda-derived, matching
    /// `AGENTS.md`'s git caution) under a scratch cache dir, and returns a
    /// manifest whose `nav_graph.asset_path`/`asset_root` resolve to it.
    fn manifest_with_nav_graph(
        cell_form_id: u32,
        graph: &PreparedNavGraph,
    ) -> PreparedSceneManifest {
        let dir = std::env::temp_dir().join(format!(
            "bevyout-nav-overlay-test-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let relative = "navgraph.ron";
        std::fs::write(
            dir.join(relative),
            ron::ser::to_string_pretty(graph, ron::ser::PrettyConfig::default()).unwrap(),
        )
        .unwrap();
        minimal_manifest(
            cell_form_id,
            dir.to_string_lossy().into_owned(),
            Some(PreparedNavGraphSource {
                asset_path: relative.into(),
                ..Default::default()
            }),
        )
    }

    fn test_world_with_manifest(manifest: PreparedSceneManifest) -> World {
        let mut world = World::new();
        world.insert_resource(manifest);
        world.init_resource::<Assets<Mesh>>();
        world.init_resource::<Assets<StandardMaterial>>();
        world.init_resource::<NavMeshOverlayState>();
        world
    }

    // T128.1: first `tnm` builds a visible overlay entity from the cell's
    // navgraph.ron; a second `tnm` hides it (toggles Visibility, no
    // rebuild -- same entity id both times); a third shows it again.
    #[test]
    fn toggling_creates_then_hides_then_shows_the_overlay_entity() {
        let graph = two_triangle_graph();
        let manifest = manifest_with_nav_graph(0xC0DE, &graph);
        let mut world = test_world_with_manifest(manifest);

        let on = toggle_nav_mesh(&mut world, &invocation()).expect("first toggle builds");
        assert_eq!(
            on.log,
            ["nav mesh visualization on (1 meshes, 2 triangles)"]
        );
        let overlay = world.resource::<NavMeshOverlayState>().overlay.unwrap();
        assert_eq!(overlay.mesh_count, 1);
        assert_eq!(overlay.triangle_count, 2);
        assert_eq!(
            world.get::<Visibility>(overlay.entity),
            Some(&Visibility::Inherited)
        );
        let built_entity = overlay.entity;

        let off = toggle_nav_mesh(&mut world, &invocation()).expect("second toggle hides");
        assert_eq!(off.log, ["nav mesh visualization off"]);
        assert_eq!(
            world.get::<Visibility>(built_entity),
            Some(&Visibility::Hidden)
        );
        // No rebuild: same entity id, unchanged counts.
        let overlay_after_hide = world.resource::<NavMeshOverlayState>().overlay.unwrap();
        assert_eq!(overlay_after_hide.entity, built_entity);

        let on_again = toggle_nav_mesh(&mut world, &invocation()).expect("third toggle shows");
        assert_eq!(
            on_again.log,
            ["nav mesh visualization on (1 meshes, 2 triangles)"]
        );
        assert_eq!(
            world.get::<Visibility>(built_entity),
            Some(&Visibility::Inherited)
        );
    }

    // T128.2: a cell whose manifest carries no `nav_graph` at all replies
    // the documented one-line error instead of panicking or building
    // anything.
    #[test]
    fn missing_nav_graph_replies_the_documented_error() {
        let manifest = minimal_manifest(0xC0DE, ".".into(), None);
        let mut world = test_world_with_manifest(manifest);

        let error = toggle_nav_mesh(&mut world, &invocation()).unwrap_err();
        assert_eq!(error.code, "no_nav_graph");
        assert_eq!(error.message, "no nav graph prepared for this cell");
        assert!(world.resource::<NavMeshOverlayState>().overlay.is_none());
    }

    // T128.2 (unreadable variant): a `nav_graph` entry whose asset file does
    // not exist replies the same documented error rather than panicking.
    #[test]
    fn unreadable_nav_graph_asset_replies_the_documented_error_without_panicking() {
        let manifest = minimal_manifest(
            0xC0DE,
            ".".into(),
            Some(PreparedNavGraphSource {
                asset_path: "does/not/exist/navgraph.ron".into(),
                ..Default::default()
            }),
        );
        let mut world = test_world_with_manifest(manifest);

        let error = toggle_nav_mesh(&mut world, &invocation()).unwrap_err();
        assert_eq!(error.code, "no_nav_graph");
    }

    // T128.3: a cell swap (the active manifest's cell FormID changing)
    // despawns the overlay via `despawn_stale_nav_overlay`, exactly like
    // the rest of the per-cell diagnostics -- and clears the tracked
    // state, so the next `tnm` in the new cell rebuilds instead of trying
    // (and failing) to toggle a despawned entity.
    #[test]
    fn cell_swap_despawns_the_overlay() {
        let graph = two_triangle_graph();
        let manifest = manifest_with_nav_graph(0xC0DE, &graph);
        let mut world = test_world_with_manifest(manifest);
        toggle_nav_mesh(&mut world, &invocation()).expect("builds for the source cell");
        let entity = world
            .resource::<NavMeshOverlayState>()
            .overlay
            .unwrap()
            .entity;
        assert!(world.get_entity(entity).is_ok());

        // Simulate `world::swap::activate_resident_cell` repointing the
        // active manifest to the destination cell.
        world.insert_resource(minimal_manifest(0xBEEF, ".".into(), None));
        world
            .run_system_once(despawn_stale_nav_overlay)
            .expect("teardown system runs");

        assert!(world.get_entity(entity).is_err());
        assert!(world.resource::<NavMeshOverlayState>().overlay.is_none());
    }

    // Pure unit test (issue #128's spec): the golden-ratio hue assignment
    // is deterministic per index, distinct for adjacent indices, and wraps
    // within `[0, 1)`.
    #[test]
    fn polygon_hue_is_deterministic_and_distinct_for_adjacent_indices() {
        assert_eq!(polygon_hue_fraction(0), 0.0);
        assert_eq!(polygon_hue_fraction(5), polygon_hue_fraction(5));
        for index in 0..64_u32 {
            let hue = polygon_hue_fraction(index);
            assert!((0.0..1.0).contains(&hue), "hue {hue} out of range");
            let next = polygon_hue_fraction(index + 1);
            let delta = (next - hue).abs();
            let wrapped_delta = delta.min(1.0 - delta);
            assert!(
                wrapped_delta > 0.3,
                "adjacent polygons {index}/{} should read as visually distinct, got hues {hue}/{next}",
                index + 1
            );
        }
    }
}
