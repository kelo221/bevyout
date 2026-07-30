use super::*;

fn fixture() -> CellMap {
    CellMap::build(
        "fingerprint".into(),
        vec![WorldspaceEntry {
            form_id: 0x10,
            editor_id: Some("Wasteland".into()),
            name: Some("The Wasteland".into()),
        }],
        vec![
            CellMapEntry {
                form_id: 0x200,
                editor_id: Some("Wasteland01".into()),
                interior: false,
                worldspace_form_id: Some(0x10),
                grid: Some((-2, 5)),
            },
            CellMapEntry {
                form_id: 0x100,
                editor_id: Some("VaultInterior".into()),
                interior: true,
                worldspace_form_id: None,
                grid: None,
            },
        ],
        vec![DoorEdge {
            source_cell_form_id: 0x100,
            door_reference_form_id: 0x101,
            destination_cell_form_id: 0x200,
            destination_door_reference_form_id: 0x201,
            position: [1.0, 2.0, 3.0],
            rotation: [0.0, 0.0, 0.0],
        }],
        1,
    )
}

#[test]
fn build_sorts_every_collection_by_form_id() {
    let map = fixture();
    assert_eq!(
        map.cells
            .iter()
            .map(|cell| cell.form_id)
            .collect::<Vec<_>>(),
        vec![0x100, 0x200]
    );
}

#[test]
fn interior_cells_have_no_worldspace_exterior_cells_do() {
    let map = fixture();
    let interior = map.cells.iter().find(|cell| cell.form_id == 0x100).unwrap();
    let exterior = map.cells.iter().find(|cell| cell.form_id == 0x200).unwrap();
    assert_eq!(interior.worldspace_form_id, None);
    assert_eq!(interior.grid, None);
    assert_eq!(exterior.worldspace_form_id, Some(0x10));
    assert_eq!(exterior.grid, Some((-2, 5)));
}

#[test]
fn dangling_teleports_are_counted_unresolved_not_fatal() {
    let map = fixture();
    assert_eq!(map.doors.len(), 1);
    assert_eq!(map.unresolved_door_count, 1);
}

// T45.4: same input, built twice, produces byte-identical RON.
#[test]
fn ron_output_is_byte_identical_across_two_builds() {
    let first = fixture().to_ron().unwrap();
    let second = fixture().to_ron().unwrap();
    assert_eq!(first, second);
}

#[test]
fn committed_golden_matches_or_regenerates_explicitly() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let golden = root.join("tests/goldens/cell_map.ron");
    let actual = fixture().to_ron().unwrap();
    if std::env::var_os("UPDATE_GOLDENS").is_some() {
        std::fs::write(&golden, &actual).unwrap();
    } else {
        // See src/console/script.rs's identical golden test for why the
        // CRLF normalization is needed: Git may check text out with
        // CRLF on Windows even though the RON we emit is always LF.
        let actual = actual.replace("\r\n", "\n");
        let expected = std::fs::read_to_string(&golden)
            .unwrap_or_else(|error| panic!("reading golden {golden:?}: {error}"))
            .replace("\r\n", "\n");
        assert_eq!(actual, expected);
    }
}
