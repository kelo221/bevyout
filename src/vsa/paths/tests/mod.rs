use super::super::plugin::ReferenceRecord;

#[test]
fn parses_editor_id_and_form_id_selectors() {
    assert_eq!(
        parse_cell_selector("SuperDuperMart").unwrap(),
        CellSelector::EditorId("SuperDuperMart".into())
    );
    assert_eq!(
        parse_cell_selector("00017f37").unwrap(),
        CellSelector::FormId(0x0001_7f37)
    );
    assert_eq!(
        parse_cell_selector("0x151e3").unwrap(),
        CellSelector::FormId(0x0001_51e3)
    );
    assert_eq!(
        parse_cell_selector("151e3").unwrap(),
        CellSelector::FormId(0x0001_51e3)
    );
}

#[test]
fn rejects_empty_cell_selectors() {
    assert!(parse_cell_selector(" ").is_err());
}
use super::*;

#[test]
fn parses_form_ids_with_optional_hex_prefix() {
    assert_eq!(parse_form_id("0x000151e3").unwrap(), 0x0001_51e3);
    assert_eq!(parse_form_id("151E3").unwrap(), 0x0001_51e3);
}

#[test]
fn placement_transform_uses_metric_fallout_scale() {
    let reference = ReferenceRecord {
        form_id: 1,
        base_form_id: 2,
        position: [70.0, 0.0, 0.0],
        rotation: [0.0, 0.0, 0.0],
        scale: 1.0,
        flags: 0,
        ..Default::default()
    };
    let (translation, _, _) = placement_transform(&reference);
    assert!((translation[0] - 1.0).abs() < f32::EPSILON);
    assert!((translation[1]).abs() < f32::EPSILON);
    assert!((translation[2]).abs() < f32::EPSILON);
}

#[test]
fn normalizes_game_asset_paths() {
    assert_eq!(
        normalize_asset_path("\\Textures\\Foo\\BAR.DDS"),
        "textures/foo/bar.dds"
    );
}

#[test]
fn identifies_non_rendering_editor_markers() {
    assert!(is_editor_marker("meshes/markerx.nif"));
    assert!(is_editor_marker("meshes/markerxheading.nif"));
    assert!(is_editor_marker("marker_north.nif"));
    assert!(!is_editor_marker("meshes/furniture/table01.nif"));
}

#[test]
fn identifies_non_rendering_effects() {
    assert!(is_non_rendering_effect(
        "effects/ambient/fxglowsimplefill.nif"
    ));
    assert!(is_non_rendering_effect(
        "effects/ambient/fxdustsimple01.nif"
    ));
    assert!(is_non_rendering_effect("effects/ambient/fxlightbeam05.nif"));
    assert!(is_non_rendering_effect(
        "effects/ambient/spraymeshconnect.nif"
    ));
    assert!(!is_non_rendering_effect("meshes/clutter/lampgeneric01.nif"));
}
