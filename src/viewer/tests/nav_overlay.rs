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
        behave_like_exterior: false,
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
        day_night_profile: None,
        day_night_preview_profile: None,
    }
}

fn minimal_manifest(
    cell_form_id: u32,
    asset_root: String,
    nav_graph: Option<PreparedNavGraphSource>,
) -> PreparedSceneManifest {
    PreparedSceneManifest {
        schema_version: 17,
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
        actor_animation_catalog_path: None,
        actor_animation_catalog_revision: None,
        actor_animation_catalog_hash: None,
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
        reflection_probes: None,
        mutability_summary: Default::default(),
        leveled_lists: Default::default(),
        dialogue: None,
        exterior: None,
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
fn manifest_with_nav_graph(cell_form_id: u32, graph: &PreparedNavGraph) -> PreparedSceneManifest {
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
    world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));
    world.init_resource::<Assets<Mesh>>();
    world.init_resource::<Assets<StandardMaterial>>();
    world.init_resource::<NavMeshOverlayState>();
    world.init_resource::<NavOverlayExposureLock>();
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

// #138 follow-up: toggling the overlay on must lock the camera's
// exposure (remove `AutoExposure`, pin `Exposure` to the fixed
// baseline) since constant material dimming alone cannot compensate
// for `AutoExposure`'s GPU-side adaptation (see the module doc
// comment); toggling off must restore the exact prior `Exposure`/
// `AutoExposure` rather than some new default.
#[test]
fn toggling_on_locks_exposure_and_toggling_off_restores_it() {
    let graph = two_triangle_graph();
    let manifest = manifest_with_nav_graph(0xC0DE, &graph);
    let mut world = test_world_with_manifest(manifest);

    let original_auto_exposure = AutoExposure {
        speed_brighten: 1.23,
        ..default()
    };
    let camera = world
        .spawn((
            Camera3d::default(),
            Exposure { ev100: 9.5 },
            original_auto_exposure.clone(),
        ))
        .id();

    toggle_nav_mesh(&mut world, &invocation()).expect("first toggle locks exposure");
    assert_eq!(
        world.get::<Exposure>(camera).map(|e| e.ev100),
        Some(OVERLAY_LOCKED_EV100),
        "exposure must be pinned to the fixed baseline while the overlay is visible"
    );
    assert!(
        world.get::<AutoExposure>(camera).is_none(),
        "AutoExposure must be removed while the overlay is visible"
    );

    toggle_nav_mesh(&mut world, &invocation()).expect("second toggle unlocks exposure");
    assert_eq!(
        world.get::<Exposure>(camera).map(|e| e.ev100),
        Some(9.5),
        "the camera's exact prior Exposure must be restored, not some new default"
    );
    let restored = world
        .get::<AutoExposure>(camera)
        .expect("AutoExposure must be restored once the overlay is hidden again");
    assert_eq!(
        restored.speed_brighten,
        original_auto_exposure.speed_brighten
    );

    // A third toggle (back on) must lock again from the *current*
    // (restored) state, not leak the first lock's saved values.
    toggle_nav_mesh(&mut world, &invocation()).expect("third toggle re-locks exposure");
    assert_eq!(
        world.get::<Exposure>(camera).map(|e| e.ev100),
        Some(OVERLAY_LOCKED_EV100)
    );
    assert!(world.get::<AutoExposure>(camera).is_none());
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
    world.insert_resource(crate::viewer::LoadedSceneManifest(minimal_manifest(
        0xBEEF,
        ".".into(),
        None,
    )));
    world
        .run_system_once(despawn_stale_nav_overlay)
        .expect("teardown system runs");

    assert!(world.get_entity(entity).is_err());
    assert!(world.resource::<NavMeshOverlayState>().overlay.is_none());
}

// PR #127 review: a polygon carrying an invalid vertex index (the
// `nav_graph::build_mesh` `u32::MAX` sentinel, or any other index past
// `mesh.vertices`) must be skipped rather than rendered as a garbage
// triangle at the origin, and must not count toward the returned
// triangle count.
#[test]
fn build_triangle_mesh_skips_polygons_with_invalid_vertex_indices() {
    let mesh = PreparedNavMesh {
        form_id: 0x10,
        vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        polygons: vec![
            PreparedNavPolygon {
                index: 0,
                vertex_indices: [0, 1, 2],
                ..Default::default()
            },
            PreparedNavPolygon {
                index: 1,
                // Slot 2 carries the invalid-index sentinel.
                vertex_indices: [0, 1, u32::MAX],
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let (built, triangle_count) = build_triangle_mesh(&mesh);

    assert_eq!(triangle_count, 1);
    assert_eq!(built.count_vertices(), 3);
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

// Issue #138 feature 2: the triangle overlay's brightness constants
// must stay low enough that a floor-covering overlay doesn't push
// `AutoExposure`'s histogram bright enough to crush a dark interior --
// asserted directly on the constants (and the worst-case rendered
// color they produce) rather than requiring a real render to observe
// it, per the wave plan's testing section.
#[test]
fn overlay_material_constants_are_dimmed_to_avoid_driving_auto_exposure() {
    // Both bounds are `const`, so clippy (rightly) flags a runtime
    // `assert!` on them as dead weight -- a `const` block instead makes
    // any regression a compile error, which is stronger than a test
    // failure anyway.
    const _: () = assert!(
        OVERLAY_ALPHA <= 0.15,
        "overlay alpha regressed toward #138's own first pass (0.28) or #128's pre-#138 0.55"
    );
    const _: () = assert!(
        TRIANGLE_LIGHTNESS <= 0.15,
        "triangle HSL lightness regressed toward #138's own first pass (0.22) or #128's pre-#138 0.5"
    );

    // The brightest a triangle can render at is hue-independent (full
    // saturation, fixed lightness) -- Rec. 709 luma of that color,
    // weighted by the blend alpha, bounds how much any single triangle
    // can contribute to the metered scene brightness. The rendered
    // pixel's alpha is `vertex_alpha (always 1.0, see
    // `build_triangle_mesh`) * material base_color alpha
    // (`OVERLAY_ALPHA`, see `spawn_nav_mesh_overlay`)` -- multiplied
    // explicitly here rather than baked into `triangle_color`'s own
    // alpha parameter, since that call now always passes `1.0`.
    let brightest = triangle_color(0, 1.0);
    let luma = 0.2126 * brightest[0] + 0.7152 * brightest[1] + 0.0722 * brightest[2];
    let contribution = luma * OVERLAY_ALPHA;
    assert!(
        contribution <= 0.03,
        "dimmed overlay still contributes too much luminance ({contribution}) to be safe for auto-exposure metering"
    );
}

// Issue #138 feature 1: the route polyline's color is fixed, opaque
// white -- never routed through `triangle_color`'s saturated hue wheel
// -- so it always reads as visually distinct from every triangle,
// regardless of which polygon hue the route happens to cross.
#[test]
fn path_line_color_is_opaque_white_and_never_produced_by_triangle_color() {
    assert_eq!(PATH_LINE_COLOR, [1.0, 1.0, 1.0, 1.0]);
    // Vertex color alpha is always 1.0 now (transparency lives in the
    // fill material's `base_color`, see `build_triangle_mesh`'s doc
    // comment) -- so this compares the actual per-vertex RGBA value
    // against the route color, hue by hue.
    for index in 0..16_u32 {
        assert_ne!(
            triangle_color(index, 1.0),
            PATH_LINE_COLOR,
            "polygon {index}'s triangle color must never coincide with the route color"
        );
    }
}

// Issue #138 feature 1: `AgentCorridorCollector` keeps only the
// `AgentCorridor` lines belonging to the agent it was built for,
// dropping every other debug-draw kind `draw_archipelago_debug` can
// report (other agents' corridors, nav-mesh edges, targets, waypoints,
// triangles) -- driven directly against the `DebugDrawer` trait so this
// doesn't need a real archipelago.
#[test]
fn agent_corridor_collector_keeps_only_the_matching_agents_corridor_lines() {
    let mut world = World::new();
    let agent = world.spawn_empty().id();
    let other_agent = world.spawn_empty().id();

    let mut collector = AgentCorridorCollector {
        agent,
        segments: Vec::new(),
    };
    let a = Vec3::new(0.0, 0.0, 0.0);
    let b = Vec3::new(1.0, 0.0, 0.0);
    collector.add_line(LineType::AgentCorridor(agent), [a, b]);
    collector.add_line(LineType::AgentCorridor(other_agent), [b, a]);
    collector.add_line(LineType::BoundaryEdge, [a, b]);
    collector.add_line(LineType::Target(agent), [a, b]);
    collector.add_point(PointType::AgentPosition(agent), a);
    collector.add_triangle(TriangleType::Node, [a, b, a]);

    assert_eq!(collector.segments, vec![[a, b]]);
}

// Issue #138 feature 1: the route mesh is a `LineList` -- two vertices
// per segment, independently colored, positions carried through
// unchanged -- and an empty segment list yields a valid, zero-vertex
// mesh (the state before the first repath).
#[test]
fn build_path_mesh_emits_two_positions_per_segment() {
    let a = Vec3::new(0.0, 0.0, 0.0);
    let b = Vec3::new(1.0, 0.0, 0.0);
    let c = Vec3::new(1.0, 0.0, 1.0);

    let empty = build_path_mesh(&[]);
    assert_eq!(empty.count_vertices(), 0);

    let mesh = build_path_mesh(&[[a, b], [b, c]]);
    assert_eq!(mesh.count_vertices(), 4);
    let positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .and_then(|attribute| attribute.as_float3())
        .expect("path mesh always carries positions");
    assert_eq!(
        positions,
        [a.to_array(), b.to_array(), b.to_array(), c.to_array()]
    );
}

// Issue #138: `tnm` still reports the exact same triangle counts/log
// wording as #128 (the plan's "console test for unchanged tnm
// toggling") even though it now also spawns a route-polyline child and
// populates `NavMeshOverlayState::path` -- the new machinery is
// additive, not a behavior change to the existing surface.
#[test]
fn toggling_still_reports_the_same_counts_and_also_seeds_empty_path_state() {
    let graph = two_triangle_graph();
    let manifest = manifest_with_nav_graph(0xC0DE, &graph);
    let mut world = test_world_with_manifest(manifest);

    let on = toggle_nav_mesh(&mut world, &invocation()).expect("first toggle builds");
    assert_eq!(
        on.log,
        ["nav mesh visualization on (1 meshes, 2 triangles)"]
    );

    let path = world
        .resource::<NavMeshOverlayState>()
        .path
        .as_ref()
        .map(|state| (state.mesh.clone(), state.last_segments.clone()));
    let (path_mesh, last_segments) = path.expect("toggle seeds path state");
    assert!(last_segments.is_empty());
    assert_eq!(
        world
            .resource::<Assets<Mesh>>()
            .get(&path_mesh)
            .expect("path mesh handle resolves")
            .count_vertices(),
        0,
        "no agent exists yet, so the route starts empty"
    );

    let off = toggle_nav_mesh(&mut world, &invocation()).expect("second toggle hides");
    assert_eq!(off.log, ["nav mesh visualization off"]);
}

// Issue #138: a cell swap clears the route state alongside the overlay
// itself, so a `tnm` in the destination cell rebuilds a fresh route
// instead of reusing a stale mesh handle from the source cell.
#[test]
fn cell_swap_also_clears_the_path_state() {
    let graph = two_triangle_graph();
    let manifest = manifest_with_nav_graph(0xC0DE, &graph);
    let mut world = test_world_with_manifest(manifest);
    toggle_nav_mesh(&mut world, &invocation()).expect("builds for the source cell");
    assert!(world.resource::<NavMeshOverlayState>().path.is_some());

    world.insert_resource(crate::viewer::LoadedSceneManifest(minimal_manifest(
        0xBEEF,
        ".".into(),
        None,
    )));
    world
        .run_system_once(despawn_stale_nav_overlay)
        .expect("teardown system runs");

    assert!(world.resource::<NavMeshOverlayState>().path.is_none());
}

// Issue #138's core acceptance surface, driven against a real
// `bevy_landmass` archipelago rather than synthetic debug-draw calls
// (mirrors `agent.rs`'s own
// `kinematic_velocity_snaps_agent_y_to_the_sampled_navmesh_surface`
// harness): a target that forces the agent across both triangles of a
// two-triangle island produces a non-empty route (the corridor line
// between the two polygon nodes); clearing the target repaths the
// agent back to no path at all, collapsing the corridor to empty --
// driven purely through `bevy_landmass`'s own public API, with no hook
// into `nav/agent.rs`'s private state. (Retargeting to a different
// point *without* clearing first is not a valid way to force this:
// `landmass`'s own repath decision -- `does_agent_need_repath` --
// reuses the existing path whenever the new target's node is already
// part of it, which is true for any point in this 2-triangle island
// once a path has been found, so a bare retarget is a no-op here.)
#[test]
fn active_agent_corridor_reflects_a_real_path_and_updates_on_repath() {
    use bevy::app::App;
    use bevy_landmass::{
        Agent3dBundle, AgentSettings, AgentTarget3d, ArchipelagoOptions, ArchipelagoRef3d,
        FromAgentRadius, Island, Island3dBundle, Landmass3dPlugin, NavMesh3d, NavMeshHandle,
    };
    use std::sync::Arc;

    use crate::viewer::nav::landmass_graph;

    let mut app = App::new();
    app.add_plugins((
        bevy::MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        Landmass3dPlugin::default(),
    ));

    // Two adjoining flat triangles (a 4x4 square split along its
    // diagonal), same shape as `agent.rs`'s own harness fixture.
    let mesh_input = landmass_graph::MeshInput {
        form_id: 0x10,
        vertices: vec![
            [0.0, 0.0, 0.0],
            [4.0, 0.0, 0.0],
            [0.0, 0.0, 4.0],
            [4.0, 0.0, 4.0],
        ],
        polygons: vec![
            landmass_graph::PolygonInput {
                index: 0,
                vertex_indices: [0, 1, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
            landmass_graph::PolygonInput {
                index: 1,
                vertex_indices: [1, 3, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
        ],
        doors: Vec::new(),
        derived_doors: Vec::new(),
    };
    let valid = landmass_graph::build_navigation_mesh(
        &mesh_input,
        &[],
        &std::collections::BTreeMap::new(),
        &std::collections::BTreeMap::new(),
    )
    .nav_mesh
    .expect("synthetic square validates");
    let nav_mesh_handle = app
        .world_mut()
        .resource_mut::<Assets<NavMesh3d>>()
        .add(NavMesh3d {
            nav_mesh: Arc::new(valid),
        });

    let options = ArchipelagoOptions::from_agent_radius(0.35);
    let archipelago = app.world_mut().spawn(Archipelago3d::new(options)).id();
    app.world_mut().spawn(Island3dBundle {
        island: Island,
        archipelago_ref: ArchipelagoRef3d::new(archipelago),
        nav_mesh: NavMeshHandle::<ThreeD>(nav_mesh_handle),
    });
    // Island sync + agent existence/value/update/output all live in
    // `FixedPreUpdate` by default (`LandmassPlugin::in_schedule`'s
    // doc); running it directly (rather than `app.update()`) sidesteps
    // the fixed-timestep accumulator never crossing its threshold
    // across back-to-back calls with no real elapsed time, same as
    // `agent.rs`'s own harness.
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);

    let agent = app
        .world_mut()
        .spawn((
            Agent3dBundle {
                agent: default(),
                settings: AgentSettings {
                    radius: 0.35,
                    desired_speed: 1.0,
                    max_speed: 1.0,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago),
            },
            Transform::from_xyz(0.5, 0.0, 0.5),
        ))
        .id();

    app.world_mut()
        .entity_mut(agent)
        .insert(AgentTarget3d::Point(Vec3::new(3.5, 0.0, 3.5)));
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);

    let crossing_segments = active_agent_corridor(app.world_mut());
    assert!(
        !crossing_segments.is_empty(),
        "a target across the shared edge must produce at least one AgentCorridor line"
    );

    // Clear the target entirely: `does_agent_need_repath` returns
    // `ClearPathNoTarget`, dropping `agent.current_path` -- a genuine,
    // unambiguous repath that the debug-drawn corridor must reflect.
    app.world_mut()
        .entity_mut(agent)
        .insert(AgentTarget3d::None);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);

    let cleared_segments = active_agent_corridor(app.world_mut());
    assert!(
        cleared_segments.is_empty(),
        "clearing the target must drop the corridor entirely, got {cleared_segments:?}"
    );

    // Retarget across the seam again: a fresh path is found from
    // scratch (`current_path` was `None`, so `does_agent_need_repath`
    // unconditionally returns `NeedsRepath`), restoring a non-empty
    // corridor -- the overlay's route recovers after a repath, not
    // just collapses once.
    app.world_mut()
        .entity_mut(agent)
        .insert(AgentTarget3d::Point(Vec3::new(3.5, 0.0, 3.5)));
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);

    let restored_segments = active_agent_corridor(app.world_mut());
    assert!(
        !restored_segments.is_empty(),
        "retargeting from a cleared path must produce a fresh, non-empty corridor"
    );
}
