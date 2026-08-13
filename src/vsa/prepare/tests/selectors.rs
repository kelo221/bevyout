use super::*;

fn cell(form_id: u32, editor_id: &str, interior: bool) -> CellSummary {
    CellSummary {
        form_id,
        editor_id: Some(editor_id.to_owned()),
        name: None,
        interior,
        worldspace_form_id: None,
        grid: None,
    }
}

fn cell_in_worldspace(form_id: u32, editor_id: &str, worldspace_form_id: u32) -> CellSummary {
    CellSummary {
        form_id,
        editor_id: Some(editor_id.to_owned()),
        name: None,
        interior: false,
        worldspace_form_id: Some(worldspace_form_id),
        grid: None,
    }
}

fn cell_at_grid(
    form_id: u32,
    editor_id: &str,
    worldspace_form_id: u32,
    grid: (i32, i32),
) -> CellSummary {
    CellSummary {
        form_id,
        editor_id: Some(editor_id.to_owned()),
        name: None,
        interior: false,
        worldspace_form_id: Some(worldspace_form_id),
        grid: Some(grid),
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

#[test]
fn all_exteriors_yields_exactly_the_exterior_subset_sorted_and_deduplicated() {
    let cells = vec![
        cell(0x00000005, "ExtB", false),
        cell(0x00000001, "IntA", true),
        cell(0x00000003, "ExtC", false),
        cell(0x00000003, "ExtCOverride", false),
    ];
    let spec = SelectionSpec {
        all_exteriors: true,
        ..Default::default()
    };

    let resolved = resolve_selection(&cells, &[], &spec).expect("resolves");

    assert_eq!(resolved, vec![0x00000003, 0x00000005]);
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

#[test]
fn exterior_radius_zero_selects_only_the_anchor() {
    let cells = vec![
        cell_at_grid(0x05, "Center", 0x100, (10, -4)),
        cell_at_grid(0x01, "Neighbor", 0x100, (9, -4)),
    ];
    let spec = SelectionSpec {
        exterior_radius: Some(0),
        explicit: vec!["Center".into()],
        ..Default::default()
    };

    let resolved = resolve_selection(&cells, &[], &spec).expect("resolves");

    assert_eq!(resolved, vec![0x05]);
}

#[test]
fn exterior_radius_selects_same_worldspace_chebyshev_square_sorted() {
    let cells = vec![
        cell_at_grid(0x05, "Center", 0x100, (10, -4)),
        cell_at_grid(0x01, "West", 0x100, (9, -4)),
        cell_at_grid(0x03, "Diagonal", 0x100, (11, -3)),
        cell_at_grid(0x02, "Outside", 0x100, (12, -4)),
        cell_at_grid(0x04, "OtherWorld", 0x200, (10, -4)),
    ];
    let spec = SelectionSpec {
        exterior_radius: Some(1),
        explicit: vec!["Center".into()],
        ..Default::default()
    };

    let resolved = resolve_selection(&cells, &[], &spec).expect("resolves");

    assert_eq!(resolved, vec![0x01, 0x03, 0x05]);
}

#[test]
fn exterior_radius_rejects_an_interior_anchor() {
    let cells = vec![cell(0x01, "Interior", true)];
    let spec = SelectionSpec {
        exterior_radius: Some(1),
        explicit: vec!["Interior".into()],
        ..Default::default()
    };

    let error = resolve_selection(&cells, &[], &spec)
        .unwrap_err()
        .to_string();

    assert!(error.contains("interior"), "{error}");
}

#[test]
fn exterior_radius_rejects_an_anchor_without_grid_metadata() {
    let cells = vec![cell_in_worldspace(0x01, "NoGrid", 0x100)];
    let spec = SelectionSpec {
        exterior_radius: Some(1),
        explicit: vec!["NoGrid".into()],
        ..Default::default()
    };

    let error = resolve_selection(&cells, &[], &spec)
        .unwrap_err()
        .to_string();

    assert!(error.contains("grid"), "{error}");
}

#[test]
fn exterior_radius_rejects_an_anchor_without_worldspace_metadata() {
    let mut anchor = cell(0x01, "NoWorldspace", false);
    anchor.grid = Some((0, 0));
    let spec = SelectionSpec {
        exterior_radius: Some(1),
        explicit: vec!["NoWorldspace".into()],
        ..Default::default()
    };

    let error = resolve_selection(&[anchor], &[], &spec)
        .unwrap_err()
        .to_string();

    assert!(error.contains("worldspace"), "{error}");
}

#[test]
fn exterior_radius_requires_exactly_one_anchor() {
    for explicit in [Vec::new(), vec!["One".into(), "Two".into()]] {
        let spec = SelectionSpec {
            exterior_radius: Some(1),
            explicit,
            ..Default::default()
        };

        let error = resolve_selection(&[], &[], &spec).unwrap_err().to_string();

        assert!(error.contains("exactly one"), "{error}");
    }
}
