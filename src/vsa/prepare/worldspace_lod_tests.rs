use super::parse_lod_source;
use bevyout_core::manifest::exterior::GridCoordinate;

#[test]
fn parses_terrain_and_block_lod_names() {
    let prefix = "meshes/landscape/lod/wasteland/";
    assert_eq!(
        parse_lod_source(
            "meshes/landscape/lod/wasteland/wasteland.level4.x4.y-8.nif",
            prefix,
        ),
        Some((
            "meshes/landscape/lod/wasteland/wasteland.level4.x4.y-8.nif".into(),
            4,
            GridCoordinate::new(4, -8),
            false,
        ))
    );
    assert_eq!(
        parse_lod_source(
            "meshes/landscape/lod/wasteland/blocks/wasteland.level4.x4.y-8.nif",
            prefix,
        )
        .map(|(_, level, grid, blocks)| (level, grid, blocks)),
        Some((4, GridCoordinate::new(4, -8), true))
    );
}

#[test]
fn rejects_non_lod_or_unsupported_level_names() {
    let prefix = "meshes/landscape/lod/wasteland/";
    assert!(parse_lod_source("meshes/landscape/lod/wasteland/foo.nif", prefix).is_none());
    assert!(
        parse_lod_source(
            "meshes/landscape/lod/wasteland/wasteland.level2.x4.y-8.nif",
            prefix,
        )
        .is_none()
    );
}
