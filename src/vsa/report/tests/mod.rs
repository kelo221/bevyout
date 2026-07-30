use super::schema::{ReportClass, SupportStatus};
use super::{generate_report, generate_report_for_sources};
use crate::vsa::content_index::PluginSource;

fn subrecord(signature: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut result = signature.to_vec();
    result.extend_from_slice(&(data.len() as u16).to_le_bytes());
    result.extend_from_slice(data);
    result
}

fn record(signature: &[u8; 4], flags: u32, form_id: u32, data: &[u8]) -> Vec<u8> {
    let mut result = signature.to_vec();
    result.extend_from_slice(&(data.len() as u32).to_le_bytes());
    result.extend_from_slice(&flags.to_le_bytes());
    result.extend_from_slice(&form_id.to_le_bytes());
    result.extend_from_slice(&[0; 8]);
    result.extend_from_slice(data);
    result
}

fn tes4() -> Vec<u8> {
    record(b"TES4", 0, 0, &[])
}

fn tes4_with_masters(masters: &[&str]) -> Vec<u8> {
    let payload = masters
        .iter()
        .flat_map(|master| {
            [
                subrecord(b"MAST", format!("{master}\0").as_bytes()),
                subrecord(b"DATA", &[0; 8]),
            ]
            .concat()
        })
        .collect::<Vec<_>>();
    record(b"TES4", 0, 0, &payload)
}

fn script_payload(editor_id: &str) -> Vec<u8> {
    let mut header = Vec::new();
    header.extend(0_u32.to_le_bytes());
    header.extend(0_u32.to_le_bytes());
    header.extend(1_u32.to_le_bytes());
    header.extend(0_u32.to_le_bytes());
    header.extend(0_u16.to_le_bytes());
    header.extend(1_u16.to_le_bytes());
    [
        subrecord(b"EDID", format!("{editor_id}\0").as_bytes()),
        subrecord(b"SCHR", &header),
        subrecord(b"SCDA", &[0xaa]),
        subrecord(b"SCTX", b"begin GameMode\0"),
    ]
    .concat()
}

/// Entirely synthetic ESM4 byte stream (no Bethesda-derived content)
/// exercising one record of every support status, plus condition,
/// script-function, and asset-format signals.
fn fixture_plugin() -> Vec<u8> {
    let mut plugin = tes4();

    // Supported record with a supported EDID/FULL/asset and one undeclared
    // (Unknown) subrecord.
    plugin.extend(record(
        b"STAT",
        0,
        0x001,
        &[
            subrecord(b"EDID", b"TestStatic\0"),
            subrecord(b"FULL", b"Test Static\0"),
            subrecord(b"MODL", b"rubble.nif\0"),
            subrecord(b"UNKN", &[1, 2, 3]),
        ]
        .concat(),
    ));

    // Supported record referencing a declared-unsupported asset extension.
    plugin.extend(record(
        b"STAT",
        0,
        0x002,
        &[
            subrecord(b"EDID", b"TestAnimStatic\0"),
            subrecord(b"MODL", b"creature.kf\0"),
        ]
        .concat(),
    ));

    // Partial-support record kind.
    plugin.extend(record(
        b"NPC_",
        0,
        0x100,
        &[
            subrecord(b"EDID", b"TestNpc\0"),
            subrecord(b"FULL", b"Test Npc\0"),
        ]
        .concat(),
    ));

    // Declared-unsupported, save-affecting quest with a condition block.
    plugin.extend(record(
        b"QUST",
        0,
        0x200,
        &[
            subrecord(b"EDID", b"TestQuest\0"),
            subrecord(b"CTDA", &[0; 20]),
        ]
        .concat(),
    ));

    // Declared-unsupported script with script-function data.
    plugin.extend(record(
        b"SCPT",
        0,
        0x300,
        &[
            subrecord(b"EDID", b"TestScript\0"),
            subrecord(b"SCTX", b"; test\0"),
        ]
        .concat(),
    ));

    // Entirely undeclared record kind -> Unknown, never Supported.
    plugin.extend(record(
        b"FAKE",
        0,
        0x400,
        &[subrecord(b"EDID", b"TestUnknownRecord\0")].concat(),
    ));

    plugin
}

#[test]
fn every_status_variant_appears_via_crafted_fixtures() {
    let report = generate_report("Fixture.esm", &fixture_plugin()).unwrap();
    for status in SupportStatus::ALL {
        assert!(
            report.entries.iter().any(|entry| entry.status == status),
            "expected at least one entry with status {status:?}"
        );
    }
    // TES4 is the declared-ignored example.
    assert!(
        report
            .entries
            .iter()
            .any(|entry| entry.class == ReportClass::Record
                && entry.key == "TES4"
                && entry.status == SupportStatus::IgnoredByDesign)
    );
    // STAT is the declared-supported example.
    assert!(
        report
            .entries
            .iter()
            .any(|entry| entry.class == ReportClass::Record
                && entry.key == "STAT"
                && entry.status == SupportStatus::Supported)
    );
    // NPC_ is the declared-partial example.
    assert!(
        report
            .entries
            .iter()
            .any(|entry| entry.class == ReportClass::Record
                && entry.key == "NPC_"
                && entry.status == SupportStatus::Partial)
    );
    // QUST is the declared-unsupported example.
    assert!(
        report
            .entries
            .iter()
            .any(|entry| entry.class == ReportClass::Record
                && entry.key == "QUST"
                && entry.status == SupportStatus::Unsupported)
    );
    // FAKE is undeclared -> Unknown.
    assert!(
        report
            .entries
            .iter()
            .any(|entry| entry.class == ReportClass::Record
                && entry.key == "FAKE"
                && entry.status == SupportStatus::Unknown)
    );
}

#[test]
fn generation_is_deterministic_across_runs() {
    let bytes = fixture_plugin();
    let first = generate_report("Fixture.esm", &bytes).unwrap();
    let second = generate_report("Fixture.esm", &bytes).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.to_json(), second.to_json());
}

#[test]
fn script_inventory_reports_structural_totals_and_diagnostics() {
    let report = generate_report("Fixture.esm", &fixture_plugin()).unwrap();
    let scripts = &report.script_inventory;
    assert_eq!(scripts.totals.top_level, 1);
    assert_eq!(scripts.totals.embedded, 0);
    assert_eq!(scripts.totals.attachments, 0);
    assert_eq!(scripts.totals.compiled_bytes, 0);
    assert_eq!(scripts.by_kind.get("missing"), Some(&1));
    assert_eq!(scripts.by_representation.get("sctx_only"), Some(&1));
    assert_eq!(scripts.scripts[0].id, "record:00000300");
    assert!(
        scripts
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("missing required SCHR"))
    );
}

#[test]
fn load_order_report_uses_winners_deletions_and_complete_owner_provenance() {
    let mut master = tes4();
    master.extend(record(b"SCPT", 0, 0x400, &script_payload("Old")));
    master.extend(record(b"SCPT", 0, 0x401, &script_payload("New")));
    master.extend(record(b"SCPT", 0, 0x402, &script_payload("Deleted")));
    master.extend(record(
        b"ACTI",
        0,
        0x500,
        &subrecord(b"SCRI", &0x400_u32.to_le_bytes()),
    ));
    let mut patch = tes4_with_masters(&["Master.esm"]);
    patch.extend(record(b"SCPT", 0x20, 0x402, &[]));
    patch.extend(record(
        b"ACTI",
        0,
        0x500,
        &subrecord(b"SCRI", &0x0000_0401_u32.to_le_bytes()),
    ));
    let sources = [
        PluginSource {
            name: "Master.esm",
            bytes: &master,
        },
        PluginSource {
            name: "Patch.esp",
            bytes: &patch,
        },
    ];

    let first = generate_report_for_sources("Patch.esp", &patch, &sources).unwrap();
    let second = generate_report_for_sources("Patch.esp", &patch, &sources).unwrap();
    assert_eq!(first.to_json(), second.to_json());
    assert_eq!(first.script_inventory.totals.top_level, 2);
    assert!(
        first
            .script_inventory
            .scripts
            .iter()
            .all(|script| script.id != "record:00000402")
    );
    let attachment = &first.script_inventory.attachments[0];
    assert_eq!(attachment.script, "record:00000401");
    assert_eq!(attachment.winning_plugin, "Patch.esp");
    assert_eq!(attachment.provenance, ["Master.esm", "Patch.esp"]);
}

#[test]
fn unknown_is_never_counted_as_supported_in_summary_totals() {
    let report = generate_report("Fixture.esm", &fixture_plugin()).unwrap();
    let counts = report.counts();

    let fake_entry = report
        .entries
        .iter()
        .find(|entry| entry.class == ReportClass::Record && entry.key == "FAKE")
        .expect("FAKE record entry should be present");
    assert_eq!(fake_entry.status, SupportStatus::Unknown);

    let supported_record_keys = report
        .entries
        .iter()
        .filter(|entry| {
            entry.class == ReportClass::Record && entry.status == SupportStatus::Supported
        })
        .map(|entry| entry.key.as_str())
        .collect::<Vec<_>>();
    assert!(!supported_record_keys.contains(&"FAKE"));

    let supported_total = counts
        .get(&(ReportClass::Record, SupportStatus::Supported))
        .copied()
        .unwrap_or(0);
    let unknown_total = counts
        .get(&(ReportClass::Record, SupportStatus::Unknown))
        .copied()
        .unwrap_or(0);
    assert_eq!(supported_total, 1, "only STAT should count as supported");
    assert_eq!(unknown_total, 1, "only FAKE should count as unknown");
}

#[test]
fn save_affecting_is_set_for_a_state_carrying_record_class() {
    let report = generate_report("Fixture.esm", &fixture_plugin()).unwrap();
    let quest_entry = report
        .entries
        .iter()
        .find(|entry| entry.class == ReportClass::Record && entry.key == "QUST")
        .expect("QUST record entry should be present");
    assert_eq!(quest_entry.status, SupportStatus::Unsupported);
    assert!(
        quest_entry.save_affecting,
        "quest progress is exactly the kind of state a save file depends on"
    );

    // A structural/definition-only unsupported example stays false.
    let plugin = {
        let mut bytes = tes4();
        bytes.extend(record(
            b"WRLD",
            0,
            0x500,
            &[subrecord(b"EDID", b"TestWorld\0")].concat(),
        ));
        bytes
    };
    let world_report = generate_report("Fixture.esm", &plugin).unwrap();
    let world_entry = world_report
        .entries
        .iter()
        .find(|entry| entry.class == ReportClass::Record && entry.key == "WRLD")
        .unwrap();
    assert_eq!(world_entry.status, SupportStatus::Unsupported);
    assert!(!world_entry.save_affecting);
}

#[test]
fn report_json_matches_golden_snapshot() {
    let report = generate_report("Fixture.esm", &fixture_plugin()).unwrap();
    let golden = include_str!("fixtures/golden_report.json").replace("\r\n", "\n");
    assert_eq!(
        report.to_json(),
        golden,
        "compatibility report JSON drifted from the golden snapshot"
    );
}

#[test]
fn provenance_records_plugin_and_form_id() {
    let report = generate_report("Fixture.esm", &fixture_plugin()).unwrap();
    let stat_entry = report
        .entries
        .iter()
        .find(|entry| entry.class == ReportClass::Record && entry.key == "STAT")
        .unwrap();
    assert_eq!(
        stat_entry.provenance,
        vec![
            "Fixture.esm:00000001".to_string(),
            "Fixture.esm:00000002".to_string()
        ]
    );
}
