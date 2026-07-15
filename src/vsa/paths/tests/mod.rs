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

fn assert_same_rotation(actual: Quat, expected: Quat) {
    let agreement = actual.dot(expected).abs();
    assert!(
        (agreement - 1.0).abs() < 1e-5,
        "rotation mismatch: actual={actual:?}, expected={expected:?}"
    );
}

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
fn fallout_rotation_identity_is_identity() {
    assert_same_rotation(fallout_rotation_to_bevy([0.0, 0.0, 0.0]), Quat::IDENTITY);
}

#[test]
fn fallout_rotation_single_axes_match_golden_bevy_quaternions() {
    for (rotation, expected_xyzw) in [
        ([0.41, 0.0, 0.0], [0.20356716, 0.0, 0.0, 0.979061]),
        ([0.0, -0.73, 0.0], [0.0, 0.0, -0.3569493, 0.93412375]),
        ([0.0, 0.0, 1.17], [0.0, -0.55219936, 0.0, 0.8337121]),
    ] {
        assert_same_rotation(
            fallout_rotation_to_bevy(rotation),
            Quat::from_array(expected_xyzw),
        );
    }
}

#[test]
fn fallout_rotation_mixed_axes_matches_golden_bevy_quaternion() {
    assert_same_rotation(
        fallout_rotation_to_bevy([0.37, -0.51, 0.83]),
        Quat::from_array([0.06292178, -0.34103355, -0.29866213, 0.889122]),
    );
}

#[test]
fn wall_screen_source_rotation_remains_upright_after_basis_conversion() {
    let source_rotation = [-std::f32::consts::PI, -0.0000009834766, 6.2831845];
    let (_, rotation, _) = placement_transform_parts([0.0; 3], source_rotation, 1.0);
    assert_same_rotation(
        Quat::from_array(rotation),
        Quat::from_array([1.0, 0.000000492, 0.000000404, 0.0]),
    );
}

#[test]
fn rivet_city_chair_and_wall_rotations_match_pre_regression_goldens() {
    // RCHangar reference 00036135, NavalChair02R.
    assert_same_rotation(
        fallout_rotation_to_bevy([0.0, 0.0, 1.95]),
        Quat::from_array([0.0, -0.82770187, 0.0, 0.5611681]),
    );
    // RCHangar reference 00053d6f, RCRmLgWallEndMidInC09.
    assert_same_rotation(
        fallout_rotation_to_bevy([0.0, 0.0, -std::f32::consts::FRAC_PI_2]),
        Quat::from_array([0.0, 0.70710677, 0.0, 0.70710677]),
    );
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
    assert!(is_non_rendering_effect("clutter/fakefog01.nif"));
    assert!(!is_non_rendering_effect("meshes/clutter/lampgeneric01.nif"));
}
