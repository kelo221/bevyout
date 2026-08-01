//! Runtime exterior tests live beside the pure policy adapter.

use bevy::mesh::{Indices, Mesh};
use bevy::prelude::{Vec3, Visibility, World};
use bevyout_core::manifest::exterior::{
    ExteriorCoordinatePolicy, ExteriorWorldspaceLodAsset, GridCoordinate, PreparedTerrain,
    TerrainLod,
};
use std::collections::BTreeMap;

use super::{
    ExteriorObjectLod, ExteriorPresentationStats, clamp_adjacent_terrain_lods,
    exterior_package_header_has_current_revision, exterior_presentation_json, terrain_center,
    terrain_mesh_with_stride, terrain_mesh_with_subdivisions, worldspace_lod_distance,
};

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
    assert_eq!(report["culling"]["occlusion"]["measured"], false);
    assert_eq!(
        report["culling"]["occlusion"]["culled"],
        serde_json::Value::Null
    );
    assert_eq!(report["gameplay"]["collision_and_navigation_culled"], false);
}
