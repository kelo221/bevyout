use super::*;

fn fingerprints(
    plugin: &str,
    converter: &str,
    physics: &str,
    prepare_pipeline: &str,
) -> CellFingerprints {
    CellFingerprints {
        plugin_content_set: plugin.into(),
        converter: converter.into(),
        physics: physics.into(),
        prepare_pipeline: prepare_pipeline.into(),
    }
}

// T49.1 (component): `CellFingerprints::current` records all four
// components from the constants/session fingerprint it is given.
#[test]
fn current_fingerprints_record_all_four_components() {
    let current = CellFingerprints::current("plugin-fp");
    assert_eq!(current.plugin_content_set, "plugin-fp");
    assert_eq!(
        current.converter,
        material_policy_identity(PREPARED_CONVERTER_REVISION)
    );
    assert_eq!(current.physics, PHYSICS_PIPELINE_REVISION);
    assert_eq!(current.prepare_pipeline, PREPARE_PIPELINE_REVISION);
}

#[test]
fn selected_converter_revision_is_recorded_independently() {
    let current = CellFingerprints::current_with_converter("plugin-fp", "native-v1");
    assert_eq!(current.converter, material_policy_identity("native-v1"));
}

// T49.2: an unchanged set of four fingerprints is not stale.
#[test]
fn matching_fingerprints_are_not_stale() {
    let recorded = fingerprints("p", "c", "phys", "prep");
    let current = fingerprints("p", "c", "phys", "prep");
    assert!(stale_components(Some(&recorded), &current).is_empty());
}

// T49.2: each component, changed alone, invalidates the cell (four
// cases) -- and no other component is reported stale alongside it.
#[test]
fn plugin_change_alone_invalidates_the_cell() {
    let recorded = fingerprints("p-old", "c", "phys", "prep");
    let current = fingerprints("p-new", "c", "phys", "prep");
    assert_eq!(
        stale_components(Some(&recorded), &current),
        vec![FingerprintComponent::Plugin]
    );
}

#[test]
fn converter_change_alone_invalidates_the_cell() {
    let recorded = fingerprints("p", "c-old", "phys", "prep");
    let current = fingerprints("p", "c-new", "phys", "prep");
    assert_eq!(
        stale_components(Some(&recorded), &current),
        vec![FingerprintComponent::Converter]
    );
}

#[test]
fn physics_change_alone_invalidates_the_cell() {
    let recorded = fingerprints("p", "c", "phys-old", "prep");
    let current = fingerprints("p", "c", "phys-new", "prep");
    assert_eq!(
        stale_components(Some(&recorded), &current),
        vec![FingerprintComponent::Physics]
    );
}

#[test]
fn prepare_pipeline_change_alone_invalidates_the_cell() {
    let recorded = fingerprints("p", "c", "phys", "prep-old");
    let current = fingerprints("p", "c", "phys", "prep-new");
    assert_eq!(
        stale_components(Some(&recorded), &current),
        vec![FingerprintComponent::PreparePipeline]
    );
}

// T49.4: a legacy entry with no recorded fingerprints is stale in every
// component, not a parse error.
#[test]
fn missing_recorded_fingerprints_are_stale_in_every_component() {
    let current = fingerprints("p", "c", "phys", "prep");
    assert_eq!(
        stale_components(None, &current),
        vec![
            FingerprintComponent::Plugin,
            FingerprintComponent::Converter,
            FingerprintComponent::Physics,
            FingerprintComponent::PreparePipeline,
        ]
    );
}

#[test]
fn stale_cell_line_joins_multiple_components_deterministically() {
    let line = stale_cell_line(
        0x0001_2345,
        &[
            FingerprintComponent::Converter,
            FingerprintComponent::Physics,
        ],
    );
    assert_eq!(
        line,
        "fingerprint: cell 00012345 stale (converter, physics)"
    );
}

#[test]
fn summary_line_reports_valid_and_stale_counts() {
    assert_eq!(summary_line(3, 1), "fingerprint: 3 cells valid, 1 stale");
}
