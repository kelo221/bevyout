use super::super::openmw_esm4::{LandRecord, ParsedPlugin};
use super::terrain_from_land;
use bevyout_core::manifest::exterior::{ExteriorCoordinatePolicy, GridCoordinate};

#[test]
fn terrain_fixture_has_shared_shape_and_cell_origin() {
    let mut land = LandRecord {
        form_id: 1,
        cell_form_id: 2,
        heights: vec![0.0; 33 * 33],
        ..Default::default()
    };
    land.heights[32] = 70.0;
    let terrain = terrain_from_land(&land, GridCoordinate::new(-2, 3));
    assert!(terrain.is_well_formed());
    let origin = ExteriorCoordinatePolicy::default().grid_origin(GridCoordinate::new(-2, 3));
    assert_eq!(terrain.positions[0][0], origin[0] as f32);
    assert_eq!(terrain.positions[32][1], 1.0);
    assert_eq!(ParsedPlugin::default().road_count, 0);
}

#[test]
fn terrain_fixture_keeps_source_colors_and_has_deterministic_blend_weights() {
    let mut land = LandRecord {
        heights: vec![0.0; 33 * 33],
        colors: vec![[10, 20, 30]; 33 * 33],
        ..Default::default()
    };
    land.texture_layers = vec![1, 2];
    let terrain = terrain_from_land(&land, GridCoordinate::new(0, 0));
    assert_eq!(terrain.colors[0], [10, 20, 30, 255]);
    assert_eq!(
        terrain.blend_weights[0]
            .iter()
            .map(|v| u16::from(*v))
            .sum::<u16>(),
        255
    );
}
