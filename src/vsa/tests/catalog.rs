use super::*;

fn subrecord(signature: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut result = signature.to_vec();
    result.extend_from_slice(&(data.len() as u16).to_le_bytes());
    result.extend_from_slice(data);
    result
}

fn record(signature: &[u8; 4], form_id: u32, data: &[u8]) -> Vec<u8> {
    let mut result = signature.to_vec();
    result.extend_from_slice(&(data.len() as u32).to_le_bytes());
    result.extend_from_slice(&0_u32.to_le_bytes());
    result.extend_from_slice(&form_id.to_le_bytes());
    result.extend_from_slice(&[0; 8]);
    result.extend_from_slice(data);
    result
}

fn plugin_with_cell(form_id: u32, editor_id: &str) -> Vec<u8> {
    let mut bytes = record(b"TES4", 0, &[]);
    bytes.extend(record(
        b"CELL",
        form_id,
        &subrecord(b"EDID", format!("{editor_id}\0").as_bytes()),
    ));
    bytes
}

#[test]
fn output_is_deterministic_and_sanitizes_tsv_fields() {
    let catalog = CellCatalog {
        content_fingerprint: "fingerprint".into(),
        entries: vec![
            CellCatalogEntry {
                form_id: 2,
                editor_id: Some("B\nCell".into()),
                name: Some("Second".into()),
                interior: false,
                winning_plugin: "Patch.esp".into(),
                provenance: vec!["Fallout3.esm".into(), "Patch.esp".into()],
                worldspace_form_id: Some(60),
            },
            CellCatalogEntry {
                form_id: 1,
                editor_id: Some("A".into()),
                name: Some("First".into()),
                interior: true,
                winning_plugin: "Fallout3.esm".into(),
                provenance: vec!["Fallout3.esm".into()],
                worldspace_form_id: None,
            },
        ],
        worldspaces: vec![(60, "Wasteland".into())],
    };
    assert!(
        catalog
            .output(true)
            .contains("00000001\tinterior\tA\tFirst")
    );
    assert!(!catalog.output(true).contains("00000002"));
    assert_eq!(display_field(Some("line\nvalue")), "line value");
}

#[test]
fn catalog_build_routes_record_identity_through_the_core_resolver_contract() {
    let plugin = plugin_with_cell(0x100, "ResolverCell");
    let sources = [PluginSource {
        name: "synthetic.esp",
        bytes: &plugin,
    }];

    let catalog = CellCatalog::build(&sources, "fixture".into()).unwrap();
    assert_eq!(catalog.entries.len(), 1);
    assert_eq!(catalog.entries[0].form_id, 0x100);
    assert_eq!(
        catalog.entries[0].editor_id.as_deref(),
        Some("ResolverCell")
    );
    assert_eq!(catalog.entries[0].winning_plugin, "synthetic.esp");
    assert_eq!(catalog.entries[0].provenance, ["synthetic.esp"]);
}
