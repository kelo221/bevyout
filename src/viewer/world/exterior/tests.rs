//! Runtime exterior tests live beside the pure policy adapter.

use bevy::camera::visibility::VisibleEntities;
use bevy::ecs::system::RunSystemOnce;
use bevy::mesh::{Indices, Mesh};
use bevy::prelude::{
    Camera3d, GlobalTransform, Mesh3d, Resource, Transform, Vec3, Visibility, World,
};
use bevy_boxddd::prelude::BoxdddPhysicsContext;
use bevyout_core::manifest::exterior::{
    ExteriorCellLifecycle, ExteriorCellState, ExteriorCoordinatePolicy, ExteriorLoadAction,
    ExteriorResidencyAction, ExteriorWorldspaceLodAsset, GridCoordinate, PreparedTerrain,
    PreparedWater, TerrainLod,
};
use std::any::TypeId;
use std::collections::BTreeMap;

use super::{
    ExteriorCellRoot, ExteriorObjectLod, ExteriorPresentationStats, ExteriorWaterState,
    ExteriorWaterSurface, ExteriorWorldspaceLodCatalog, ExteriorWorldspaceLodVisual, FpsPlayer,
    apply_action, clamp_adjacent_terrain_lods, exterior_package_header_has_current_revision,
    exterior_presentation_json, finalize_evictions, mark_collision_ready, terrain_center,
    terrain_mesh_with_stride, terrain_mesh_with_subdivisions, update_water_state,
    worldspace_lod_distance,
};
use super::{diagnostics, lifecycle};

#[derive(Resource)]
struct TestAction(ExteriorResidencyAction);

fn apply_test_action(
    mut commands: bevy::prelude::Commands,
    mut state: bevy::prelude::ResMut<lifecycle::ExteriorStreamState>,
    tasks: bevy::prelude::Query<&super::loading::ExteriorPackageTask>,
    action: bevy::prelude::Res<TestAction>,
) {
    apply_action(&mut commands, &mut state, action.0.clone(), &tasks, "");
}

fn run_test_action(world: &mut World, action: ExteriorResidencyAction) {
    world.insert_resource(TestAction(action));
    world
        .run_system_once(apply_test_action)
        .expect("exterior action system runs");
    world.flush();
}

#[test]
fn terrain_render_winding_faces_upward() {
    let terrain = PreparedTerrain {
        width: 3,
        height: 3,
        positions: (0..9)
            .map(|index| {
                let x = (index % 3) as f32;
                let row = (index / 3) as f32;
                [x, 0.0, -row]
            })
            .collect(),
        normals: vec![[0.0, 1.0, 0.0]; 9],
        colors: vec![[255, 255, 255, 255]; 9],
        blend_weights: vec![[255, 0, 0, 0]; 9],
        texture_layers: Vec::new(),
        albedo_asset_path: None,
        normal_asset_path: None,
        collision_heights: vec![0.0; 9],
    };
    let mesh = terrain_mesh_with_stride(&terrain, 1).expect("well-formed terrain mesh");
    let Indices::U32(indices) = mesh.indices().expect("indexed terrain mesh") else {
        panic!("terrain mesh indices must use u32");
    };
    assert_eq!(&indices[..6], &[0, 1, 3, 1, 4, 3]);

    let [a, b, c] = [
        terrain.positions[indices[0] as usize],
        terrain.positions[indices[1] as usize],
        terrain.positions[indices[2] as usize],
    ];
    let edge_a = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let edge_b = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let geometric_normal_y = edge_a[2] * edge_b[0] - edge_a[0] * edge_b[2];
    assert!(geometric_normal_y > 0.0);
}

#[test]
fn near_terrain_subdivision_keeps_source_borders_and_adds_visual_detail() {
    let terrain = PreparedTerrain {
        width: 3,
        height: 3,
        positions: (0..9)
            .map(|index| {
                let x = (index % 3) as f32;
                let row = (index / 3) as f32;
                [x, 0.0, -row]
            })
            .collect(),
        normals: vec![[0.0, 1.0, 0.0]; 9],
        colors: vec![[255, 255, 255, 255]; 9],
        blend_weights: vec![[255, 0, 0, 0]; 9],
        texture_layers: Vec::new(),
        albedo_asset_path: None,
        normal_asset_path: None,
        collision_heights: vec![0.0; 9],
    };
    let mesh = terrain_mesh_with_subdivisions(&terrain, 2).expect("subdivided terrain mesh");
    assert_eq!(mesh.count_vertices(), 57);
    let positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("subdivided mesh positions");
    let bevy::mesh::VertexAttributeValues::Float32x3(positions) = positions else {
        panic!("subdivided positions must be Float32x3");
    };
    assert_eq!(positions[0], terrain.positions[0]);
    assert_eq!(positions[4], terrain.positions[2]);
    assert_eq!(positions[20], terrain.positions[6]);
}

#[test]
fn terrain_lod_center_uses_authored_elevation() {
    let terrain = PreparedTerrain {
        width: 3,
        height: 3,
        positions: vec![
            [0.0, 159.0, 0.0],
            [1.0, 160.0, 0.0],
            [2.0, 159.0, 0.0],
            [0.0, 160.0, -1.0],
            [1.0, 161.0, -1.0],
            [2.0, 160.0, -1.0],
            [0.0, 159.0, -2.0],
            [1.0, 160.0, -2.0],
            [2.0, 159.0, -2.0],
        ],
        normals: vec![[0.0, 1.0, 0.0]; 9],
        colors: vec![[255, 255, 255, 255]; 9],
        blend_weights: vec![[255, 0, 0, 0]; 9],
        texture_layers: Vec::new(),
        albedo_asset_path: None,
        normal_asset_path: None,
        collision_heights: vec![0.0; 9],
    };

    assert_eq!(
        terrain_center(Some(&terrain), [0.0, 0.0, 0.0]),
        Vec3::new(1.0, 161.0, -1.0)
    );
    assert_eq!(terrain_center(None, [0.0, 0.0, 0.0]).y, 0.0);
}

#[test]
fn terrain_lod_clamp_reaches_a_fixed_point_across_a_strip() {
    let mut selected = BTreeMap::from([
        (GridCoordinate::new(0, 0), TerrainLod::Near),
        (GridCoordinate::new(1, 0), TerrainLod::Distant),
        (GridCoordinate::new(2, 0), TerrainLod::Near),
    ]);

    clamp_adjacent_terrain_lods(&mut selected);

    let ranks = [
        selected[&GridCoordinate::new(0, 0)],
        selected[&GridCoordinate::new(1, 0)],
        selected[&GridCoordinate::new(2, 0)],
    ];
    assert_eq!(
        ranks,
        [TerrainLod::Near, TerrainLod::Middle, TerrainLod::Near]
    );
}

#[test]
fn worldspace_lod_distance_uses_level_ranges_and_block_policy() {
    let terrain = ExteriorWorldspaceLodAsset {
        asset_path: "assets/terrain.glb".into(),
        level: 4,
        grid: GridCoordinate::new(4, -8),
        blocks: false,
    };
    let policy = ExteriorCoordinatePolicy::default();
    let origin = policy.grid_origin(terrain.grid);
    let span = policy.cell_span_metres() * f64::from(terrain.level);
    let center = Vec3::new(
        (origin[0] + span * 0.5) as f32,
        0.0,
        (origin[2] - span * 0.5) as f32,
    );
    assert_eq!(
        worldspace_lod_distance(&terrain, center + Vec3::X * 120.0),
        Some(120.0)
    );
    assert!(worldspace_lod_distance(&terrain, center + Vec3::X * 720.1).is_none());

    let blocks = ExteriorWorldspaceLodAsset {
        blocks: true,
        ..terrain
    };
    assert_eq!(
        worldspace_lod_distance(&blocks, center + Vec3::X * 1_200.0),
        Some(1_200.0)
    );
    assert!(worldspace_lod_distance(&blocks, center + Vec3::X * 1_200.1).is_none());
}

#[test]
fn stale_exterior_package_headers_are_not_resident_candidates() {
    assert!(exterior_package_header_has_current_revision(&[
        "(".into(),
        "    revision: \"exterior-cell-package-v7-terrain-normal-map\",".into(),
    ]));
    assert!(!exterior_package_header_has_current_revision(&[
        "(".into(),
        "    revision: \"exterior-cell-package-v6\",".into(),
    ]));
}

#[test]
fn presentation_diagnostics_keep_distance_culling_separate_from_occlusion() {
    let mut world = World::new();
    world.insert_resource(ExteriorPresentationStats {
        terrain_lod_transitions: 4,
        ..Default::default()
    });
    world.spawn((
        ExteriorObjectLod {
            distant: false,
            persistent: false,
            visible: true,
        },
        Visibility::Inherited,
    ));
    world.spawn((
        ExteriorObjectLod {
            distant: false,
            persistent: false,
            visible: false,
        },
        Visibility::Hidden,
    ));
    world.spawn((
        ExteriorObjectLod {
            distant: true,
            persistent: true,
            visible: true,
        },
        Visibility::Inherited,
    ));

    let report = exterior_presentation_json(&mut world);
    assert_eq!(report["terrain"]["lod_transitions"], 4);
    assert_eq!(report["objects"]["distance_culled"], 1);
    assert_eq!(report["culling"]["distance"]["culled"], 1);
    assert_eq!(report["culling"]["frustum"]["measured"], false);
    assert_eq!(report["culling"]["occlusion"]["measured"], false);
    assert_eq!(
        report["culling"]["occlusion"]["culled"],
        serde_json::Value::Null
    );
    assert_eq!(report["gameplay"]["collision_and_navigation_culled"], false);
}

#[test]
fn presentation_diagnostics_reports_cpu_visibility_and_lod_identity_counters() {
    let mut world = World::new();
    let visible_mesh = world.spawn(Mesh3d::default()).id();
    world.spawn(Mesh3d::default());
    let mut visible_entities = VisibleEntities::default();
    visible_entities
        .get_mut(TypeId::of::<Mesh3d>())
        .push(visible_mesh);
    world.spawn((Camera3d::default(), visible_entities));

    let duplicate_key = ExteriorWorldspaceLodVisual {
        level: 4,
        grid: GridCoordinate::new(4, -5),
        blocks: false,
    };
    world.spawn(duplicate_key);
    world.spawn(duplicate_key);
    world.insert_resource(ExteriorWorldspaceLodCatalog {
        descriptors: vec![
            ExteriorWorldspaceLodAsset {
                asset_path: "assets/near-a.glb".into(),
                level: 4,
                grid: GridCoordinate::new(4, -5),
                blocks: false,
            },
            ExteriorWorldspaceLodAsset {
                asset_path: "assets/near-b.glb".into(),
                level: 4,
                grid: GridCoordinate::new(4, -5),
                blocks: false,
            },
        ],
        ..Default::default()
    });
    world.insert_resource(ExteriorPresentationStats {
        worldspace_lod_asset_loads_staged_total: 10,
        worldspace_lod_asset_loads_staged_last_frame: 3,
        worldspace_lod_peak_asset_loads_staged_per_frame: 8,
        worldspace_lod_despawns_total: 2,
        ..Default::default()
    });

    let report = exterior_presentation_json(&mut world);
    assert_eq!(report["culling"]["frustum"]["measured"], true);
    assert_eq!(report["culling"]["frustum"]["candidate_meshes"], 2);
    assert_eq!(report["culling"]["frustum"]["visible_meshes"], 1);
    assert_eq!(report["culling"]["frustum"]["culled"], 1);
    assert_eq!(report["worldspace_lod"]["catalog_duplicate_instances"], 1);
    assert_eq!(report["worldspace_lod"]["active_duplicate_instances"], 1);
    assert_eq!(report["worldspace_lod"]["asset_loads_staged_total"], 10);
    assert_eq!(report["worldspace_lod"]["asset_loads_staged_last_frame"], 3);
    assert_eq!(
        report["worldspace_lod"]["peak_asset_loads_staged_per_frame"],
        8
    );
    assert_eq!(report["worldspace_lod"]["despawns_total"], 2);
    assert_eq!(report["worldspace_lod"]["selection_transitions"], 12);
}

#[test]
fn cancelling_after_package_spawn_uses_the_owned_eviction_teardown() {
    fn cancel_loaded_cell(
        mut commands: bevy::prelude::Commands,
        mut state: bevy::prelude::ResMut<lifecycle::ExteriorStreamState>,
        tasks: bevy::prelude::Query<&super::loading::ExteriorPackageTask>,
    ) {
        apply_action(
            &mut commands,
            &mut state,
            ExteriorResidencyAction {
                action: ExteriorLoadAction::Cancel,
                grid: GridCoordinate::new(1, -2),
                form_id: 0x1234,
                generation: 1,
            },
            &tasks,
            "",
        );
    }

    let mut world = World::new();
    let root = world.spawn_empty().id();
    let grid = GridCoordinate::new(1, -2);
    let mut state = lifecycle::ExteriorStreamState {
        resident_bytes: 64,
        ..Default::default()
    };
    state.cells.insert(
        grid,
        lifecycle::RuntimeCell {
            state: ExteriorCellState {
                cell_form_id: 0x1234,
                grid,
                lifecycle: ExteriorCellLifecycle::Loading,
                generation: 1,
                pinned: false,
                estimated_bytes: 64,
                failed_attempts: 0,
            },
            root: Some(root),
            task: None,
            package: None,
            collision_ready: false,
            eviction_restore: None,
        },
    );
    world.insert_resource(state);

    world
        .run_system_once(cancel_loaded_cell)
        .expect("cancel system runs");
    assert_eq!(
        world.resource::<lifecycle::ExteriorStreamState>().cells[&grid]
            .state
            .lifecycle,
        ExteriorCellLifecycle::Evicting,
        "a spawned root is owned state and must use the full eviction path"
    );

    finalize_evictions(&mut world);
    assert!(
        world.get_entity(root).is_err(),
        "the root must be despawned"
    );
    let state = world.resource::<lifecycle::ExteriorStreamState>();
    assert!(!state.cells.contains_key(&grid));
    assert_eq!(state.resident_bytes, 0);
    assert_eq!(state.cancellations, 1);
}

#[test]
fn cancellation_reversal_keeps_a_collision_pending_root_owned() {
    let mut world = World::new();
    let grid = GridCoordinate::new(2, -1);
    let root = world.spawn(ExteriorCellRoot { form_id: 0x2233 }).id();
    let mut state = lifecycle::ExteriorStreamState {
        resident_bytes: 32,
        ..Default::default()
    };
    state.cells.insert(
        grid,
        lifecycle::RuntimeCell {
            state: ExteriorCellState {
                cell_form_id: 0x2233,
                grid,
                lifecycle: ExteriorCellLifecycle::Loading,
                generation: 4,
                pinned: false,
                estimated_bytes: 32,
                failed_attempts: 0,
            },
            root: Some(root),
            task: None,
            package: None,
            collision_ready: false,
            eviction_restore: None,
        },
    );
    world.insert_resource(state);

    run_test_action(
        &mut world,
        ExteriorResidencyAction {
            action: ExteriorLoadAction::Cancel,
            grid,
            form_id: 0x2233,
            generation: 4,
        },
    );
    run_test_action(
        &mut world,
        ExteriorResidencyAction {
            action: ExteriorLoadAction::Cancel,
            grid,
            form_id: 0x2233,
            generation: 5,
        },
    );
    let state = world.resource::<lifecycle::ExteriorStreamState>();
    assert_eq!(
        state.cells[&grid].state.lifecycle,
        ExteriorCellLifecycle::Loading
    );
    assert_eq!(state.cells[&grid].root, Some(root));
    assert_eq!(state.cells[&grid].state.generation, 5);
}

#[test]
fn collision_ready_records_all_spawned_resident_roots() {
    let mut state = lifecycle::ExteriorStreamState {
        current_grid: GridCoordinate::new(0, 0),
        ..Default::default()
    };
    for (grid, form_id, root) in [
        (GridCoordinate::new(0, 0), 0x10, 1_u32),
        (GridCoordinate::new(1, 0), 0x11, 2_u32),
    ] {
        state.cells.insert(
            grid,
            lifecycle::RuntimeCell {
                state: ExteriorCellState {
                    cell_form_id: form_id,
                    grid,
                    lifecycle: ExteriorCellLifecycle::Loading,
                    generation: 1,
                    pinned: false,
                    estimated_bytes: 10,
                    failed_attempts: 0,
                },
                root: Some(bevy::prelude::Entity::from_raw_u32(root).expect("test entity")),
                task: None,
                package: None,
                collision_ready: false,
                eviction_restore: None,
            },
        );
    }

    mark_collision_ready(&mut state, GridCoordinate::new(0, 0), 0x10, 2);
    assert!(
        state.collision_cells.is_empty(),
        "a stale collision completion must not claim the grid"
    );
    mark_collision_ready(&mut state, GridCoordinate::new(0, 0), 0x10, 1);
    assert_eq!(
        state.peak_resident_cells, 2,
        "peak residency counts spawned package roots, including collision-pending roots"
    );
}

#[test]
fn stale_activation_cannot_resurrect_an_eviction_and_reversal_keeps_one_root() {
    let mut world = World::new();
    let grid = GridCoordinate::new(1, 0);
    let root = world.spawn(ExteriorCellRoot { form_id: 0x1234 }).id();
    let mut state = lifecycle::ExteriorStreamState {
        current_grid: grid,
        ..Default::default()
    };
    state.cells.insert(
        grid,
        lifecycle::RuntimeCell {
            state: ExteriorCellState {
                cell_form_id: 0x1234,
                grid,
                lifecycle: ExteriorCellLifecycle::Resident,
                generation: 1,
                pinned: false,
                estimated_bytes: 64,
                failed_attempts: 0,
            },
            root: Some(root),
            task: None,
            package: None,
            collision_ready: false,
            eviction_restore: None,
        },
    );
    world.insert_resource(state);

    run_test_action(
        &mut world,
        ExteriorResidencyAction {
            action: ExteriorLoadAction::Evict,
            grid,
            form_id: 0x1234,
            generation: 1,
        },
    );
    run_test_action(
        &mut world,
        ExteriorResidencyAction {
            action: ExteriorLoadAction::Activate,
            grid,
            form_id: 0x1234,
            generation: 1,
        },
    );
    assert_eq!(
        world.resource::<lifecycle::ExteriorStreamState>().cells[&grid]
            .state
            .lifecycle,
        ExteriorCellLifecycle::Evicting,
        "an activation from before eviction must be stale"
    );

    run_test_action(
        &mut world,
        ExteriorResidencyAction {
            action: ExteriorLoadAction::Cancel,
            grid,
            form_id: 0x1234,
            generation: 2,
        },
    );
    let state = world.resource::<lifecycle::ExteriorStreamState>();
    assert_eq!(
        state.cells[&grid].state.lifecycle,
        ExteriorCellLifecycle::Resident
    );
    assert_eq!(state.cells[&grid].root, Some(root));
    let mut roots = world.query::<&ExteriorCellRoot>();
    assert_eq!(roots.iter(&world).count(), 1);
}

#[test]
fn final_eviction_tears_down_collision_ownership_before_removing_cell_state() {
    let mut world = World::new();
    let grid = GridCoordinate::new(-2, 3);
    let root = world.spawn(ExteriorCellRoot { form_id: 0x4321 }).id();
    let mut state = lifecycle::ExteriorStreamState {
        resident_bytes: 64,
        ..Default::default()
    };
    state.cells.insert(
        grid,
        lifecycle::RuntimeCell {
            state: ExteriorCellState {
                cell_form_id: 0x4321,
                grid,
                lifecycle: ExteriorCellLifecycle::Resident,
                generation: 1,
                pinned: false,
                estimated_bytes: 64,
                failed_attempts: 0,
            },
            root: Some(root),
            task: None,
            package: None,
            collision_ready: true,
            eviction_restore: None,
        },
    );
    state.collision_cells.insert(grid, 0x4321);
    world.insert_resource(state);
    world.insert_resource(crate::viewer::player::PreparedCollisionWorld::default());
    world.insert_resource(crate::viewer::player::PendingColliderBuild::default());
    world.insert_non_send(BoxdddPhysicsContext::disabled());

    run_test_action(
        &mut world,
        ExteriorResidencyAction {
            action: ExteriorLoadAction::Evict,
            grid,
            form_id: 0x4321,
            generation: 1,
        },
    );
    finalize_evictions(&mut world);

    let state = world.resource::<lifecycle::ExteriorStreamState>();
    assert!(!state.cells.contains_key(&grid));
    assert!(!state.collision_cells.contains_key(&grid));
    assert_eq!(state.resident_bytes, 0);
    assert!(world.get_entity(root).is_err());
}

#[test]
fn streaming_summary_labels_estimates_and_keeps_unmeasured_memory_null() {
    let state = lifecycle::ExteriorStreamState {
        resident_bytes: 256,
        peak_memory: 512,
        ..Default::default()
    };

    let report = diagnostics::status(&state);
    assert_eq!(report["resident_bytes"], serde_json::Value::Null);
    assert_eq!(report["peak_memory"], serde_json::Value::Null);
    assert_eq!(report["ending_memory"], serde_json::Value::Null);
    assert_eq!(report["memory_measurement"], "unmeasured");
    assert_eq!(report["resident_package_bytes_estimate"], 256);
    assert_eq!(report["peak_package_bytes_estimate"], 512);
}

#[test]
fn water_contact_ignores_surfaces_in_other_resident_cells() {
    let mut world = World::new();
    world.init_resource::<ExteriorWaterState>();
    world.spawn((
        FpsPlayer::default(),
        Transform::from_xyz(10_000.0, -2.0, 10_000.0),
        GlobalTransform::default(),
    ));
    world.spawn((
        ExteriorWaterSurface {
            descriptor: PreparedWater {
                form_id: None,
                height: 0.0,
                water_type_form_id: None,
                swim_depth: 1.0,
            },
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));

    world
        .run_system_once(update_water_state)
        .expect("water system runs");
    assert!(
        world.resource::<ExteriorWaterState>().contact.is_none(),
        "a water plane only applies inside its owning cell footprint"
    );
}
