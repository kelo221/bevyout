use super::*;

fn cell(form_id: u32, editor_id: &str, interior: bool) -> CellSummary {
    CellSummary {
        form_id,
        editor_id: Some(editor_id.to_owned()),
        name: None,
        interior,
        worldspace_form_id: None,
    }
}

fn cell_in_worldspace(form_id: u32, editor_id: &str, worldspace_form_id: u32) -> CellSummary {
    CellSummary {
        form_id,
        editor_id: Some(editor_id.to_owned()),
        name: None,
        interior: false,
        worldspace_form_id: Some(worldspace_form_id),
    }
}

// T46.1
#[test]
fn all_interiors_yields_exactly_the_interior_subset_sorted() {
    let cells = vec![
        cell(0x00000005, "ExtB", false),
        cell(0x00000001, "IntA", true),
        cell(0x00000003, "IntC", true),
    ];
    let spec = SelectionSpec {
        all_interiors: true,
        ..Default::default()
    };
    let resolved = resolve_selection(&cells, &[], &spec).expect("resolves");
    assert_eq!(resolved, vec![0x00000001, 0x00000003]);
}

// T46.2
#[test]
fn explicit_list_mixing_editor_id_and_form_id_resolves_dedupes_and_sorts() {
    let cells = vec![
        cell(0x00000010, "VaultDoor", true),
        cell(0x00000002, "Wasteland", false),
    ];
    let spec = SelectionSpec {
        explicit: vec![
            "Wasteland".to_string(),
            "00000010".to_string(),
            "wasteland".to_string(),
        ],
        ..Default::default()
    };
    let resolved = resolve_selection(&cells, &[], &spec).expect("resolves");
    assert_eq!(resolved, vec![0x00000002, 0x00000010]);
}

// T46.3
#[test]
fn unknown_worldspace_errors_and_names_available_worldspaces() {
    let cells = vec![cell_in_worldspace(0x00000001, "Cell1", 0x00000100)];
    let worldspace_names = vec![(0x00000100, "Capital Wasteland".to_string())];
    let spec = SelectionSpec {
        worldspace: Some("Nowhere".to_string()),
        ..Default::default()
    };
    let error = resolve_selection(&cells, &worldspace_names, &spec).unwrap_err();
    let message = error.to_string();
    assert!(message.contains("Nowhere"), "{message}");
    assert!(message.contains("Capital Wasteland"), "{message}");
}

#[test]
fn all_subsumes_every_other_selector_and_is_sorted() {
    let cells = vec![
        cell(0x00000005, "ExtB", false),
        cell(0x00000001, "IntA", true),
    ];
    let spec = SelectionSpec {
        all: true,
        explicit: vec!["nonexistent".to_string()],
        ..Default::default()
    };
    let resolved = resolve_selection(&cells, &[], &spec).expect("resolves");
    assert_eq!(resolved, vec![0x00000001, 0x00000005]);
}

#[test]
fn all_interiors_and_worldspace_and_explicit_selectors_union() {
    let cells = vec![
        cell(0x00000001, "IntA", true),
        cell_in_worldspace(0x00000002, "ExtInWorld", 0x00000100),
        cell(0x00000003, "Explicit", false),
    ];
    let worldspace_names = vec![(0x00000100, "Capital Wasteland".to_string())];
    let spec = SelectionSpec {
        all_interiors: true,
        worldspace: Some("Capital Wasteland".to_string()),
        explicit: vec!["Explicit".to_string()],
        ..Default::default()
    };
    let resolved = resolve_selection(&cells, &worldspace_names, &spec).expect("resolves");
    assert_eq!(resolved, vec![0x00000001, 0x00000002, 0x00000003]);
}

#[test]
fn unknown_explicit_selector_names_near_candidates() {
    let cells = vec![cell(0x00000001, "SuperDuperMart", false)];
    let spec = SelectionSpec {
        explicit: vec!["SuperDuper".to_string()],
        ..Default::default()
    };
    let error = resolve_selection(&cells, &[], &spec).unwrap_err();
    let message = error.to_string();
    assert!(message.contains("SuperDuper"), "{message}");
    assert!(message.contains("SuperDuperMart"), "{message}");
}

#[test]
fn empty_selection_spec_is_an_error() {
    assert!(resolve_selection(&[], &[], &SelectionSpec::default()).is_err());
}
