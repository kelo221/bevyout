//! T39.1-T39.8: hermetic synthetic ESM4 fixtures for the load-order-wide
//! ContentIndex. No Bethesda-derived data; bytes are built in-test, mirroring
//! `openmw_esm4/tests/mod.rs`'s `record`/`subrecord`/`tes4` fixture writer.

use super::*;
use crate::vsa::record_stream::{RecordPayload, walk_resolved_records, winners::WinningRecords};
use flate2::{Compression, write::ZlibEncoder};
use std::io::Write;

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

fn tes4(masters: &[&str]) -> Vec<u8> {
    let mut data = Vec::new();
    for master in masters {
        data.extend(subrecord(b"MAST", format!("{master}\0").as_bytes()));
        data.extend(subrecord(b"DATA", &[0; 8]));
    }
    record(b"TES4", 0, 0, &data)
}

fn edid_record(signature: &[u8; 4], form_id: u32, editor_id: &str) -> Vec<u8> {
    record(
        signature,
        0,
        form_id,
        &subrecord(b"EDID", format!("{editor_id}\0").as_bytes()),
    )
}

fn compressed_record(signature: &[u8; 4], form_id: u32, data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    let compressed = encoder.finish().unwrap();
    let mut payload = (data.len() as u32).to_le_bytes().to_vec();
    payload.extend(compressed);
    record(signature, 0x0004_0000, form_id, &payload)
}

#[test]
fn t39_1_two_plugin_fixture_indexes_both_and_remaps_form_ids() {
    // Three plugins so a genuine remap is observable: dependent.esp's own
    // master list only names "master.esm", but master.esm is *not* at
    // global position 0 because unrelated.esm loads before it. Resolving
    // dependent's local master index (0) must land on master.esm's true
    // global position (1), not 0.
    let mut unrelated = tes4(&[]);
    unrelated.extend(edid_record(b"GLOB", 0x000001, "UnrelatedGlobal"));

    let mut master = tes4(&[]);
    master.extend(edid_record(b"MISC", 0x400, "MasterWidget"));

    let mut dependent = tes4(&["master.esm"]);
    // Raw top byte 0 = dependent's own master-list index 0 ("master.esm"),
    // object index 0x777; should resolve to global file index 1.
    dependent.extend(edid_record(b"STAT", 0x0000_0777, "DependentProp"));

    let sources = [
        PluginSource {
            name: "unrelated.esm",
            bytes: &unrelated,
        },
        PluginSource {
            name: "master.esm",
            bytes: &master,
        },
        PluginSource {
            name: "dependent.esp",
            bytes: &dependent,
        },
    ];
    let index = ContentIndex::build(&sources).unwrap();

    let unrelated_record = index.get(FormId(0x000001)).unwrap();
    assert_eq!(unrelated_record.winning_plugin, "unrelated.esm");

    // master.esm has no masters of its own, so its raw top byte (0) falls
    // back to its own *global* position (1, since unrelated.esm loads
    // first) rather than staying 0.
    let master_record = index.get(FormId(0x0100_0400)).unwrap();
    assert_eq!(master_record.record_type, "MISC");
    assert_eq!(master_record.editor_id.as_deref(), Some("MasterWidget"));
    assert_eq!(master_record.winning_plugin, "master.esm");
    assert!(index.get(FormId(0x400)).is_none());

    let remapped = index.get(FormId(0x0100_0777)).expect(
        "dependent.esp's record should resolve into master.esm's global file index (1), not 0",
    );
    assert_eq!(remapped.record_type, "STAT");
    assert_eq!(remapped.editor_id.as_deref(), Some("DependentProp"));
    assert_eq!(remapped.winning_plugin, "dependent.esp");
    assert!(index.get(FormId(0x0000_0777)).is_none());

    assert_eq!(
        index.load_order(),
        ["unrelated.esm", "master.esm", "dependent.esp"]
    );
}

#[test]
fn t39_2_override_fixture_dependent_wins_with_full_provenance() {
    let mut master = tes4(&[]);
    master.extend(edid_record(b"MISC", 0x400, "Original"));

    let mut dependent = tes4(&["master.esm"]);
    // Raw top byte 0 = dependent's sole master ("master.esm", global
    // position 0 here), same object index -> overrides the master record.
    dependent.extend(edid_record(b"MISC", 0x400, "Overridden"));

    let sources = [
        PluginSource {
            name: "master.esm",
            bytes: &master,
        },
        PluginSource {
            name: "dependent.esp",
            bytes: &dependent,
        },
    ];
    let index = ContentIndex::build(&sources).unwrap();

    let winner = index.get(FormId(0x400)).unwrap();
    assert_eq!(winner.editor_id.as_deref(), Some("Overridden"));
    assert_eq!(winner.winning_plugin, "dependent.esp");
    assert_eq!(winner.provenance, ["master.esm", "dependent.esp"]);
}

#[test]
fn t39_3_missing_master_names_missing_master_and_requiring_plugin() {
    let dependent = tes4(&["phantom.esm"]);
    let sources = [PluginSource {
        name: "dependent.esp",
        bytes: &dependent,
    }];

    let error = ContentIndex::build(&sources).unwrap_err().to_string();
    assert!(error.contains("phantom.esm"), "error was: {error}");
    assert!(error.contains("dependent.esp"), "error was: {error}");
}

#[test]
fn t39_4_invalid_order_names_both_plugins() {
    let mut master = tes4(&[]);
    master.extend(edid_record(b"MISC", 0x400, "Original"));
    let dependent = tes4(&["master.esm"]);

    // dependent.esp listed *before* the master it requires.
    let sources = [
        PluginSource {
            name: "dependent.esp",
            bytes: &dependent,
        },
        PluginSource {
            name: "master.esm",
            bytes: &master,
        },
    ];

    let error = ContentIndex::build(&sources).unwrap_err().to_string();
    assert!(error.contains("dependent.esp"), "error was: {error}");
    assert!(error.contains("master.esm"), "error was: {error}");
}

#[test]
fn t39_5_duplicate_plugin_entry_errors() {
    let master = tes4(&[]);
    let sources = [
        PluginSource {
            name: "master.esm",
            bytes: &master,
        },
        PluginSource {
            name: "master.esm",
            bytes: &master,
        },
    ];

    let error = ContentIndex::build(&sources).unwrap_err().to_string();
    assert!(error.contains("master.esm"), "error was: {error}");
    assert!(error.contains("duplicate"), "error was: {error}");
}

#[test]
fn t39_6_editor_id_and_record_type_queries() {
    let mut plugin = tes4(&[]);
    plugin.extend(edid_record(b"CELL", 0x100, "TestCell"));
    plugin.extend(edid_record(b"MISC", 0x400, "Widget"));
    plugin.extend(edid_record(b"MISC", 0x401, "OtherWidget"));

    let sources = [PluginSource {
        name: "plugin.esp",
        bytes: &plugin,
    }];
    let index = ContentIndex::build(&sources).unwrap();

    assert_eq!(index.records_of_type("MISC").count(), 2);
    assert_eq!(index.records_of_type("NOPE").count(), 0);

    let cells: Vec<_> = index.cells().collect();
    assert_eq!(cells.len(), 1);
    assert_eq!(cells[0].form_id, FormId(0x100));
    assert_eq!(cells[0].editor_id.as_deref(), Some("TestCell"));

    let matches = index.by_editor_id("testcell");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].record_type, "CELL");

    let resolver: &dyn ContentRecordResolver = &index;
    let resolved = resolver.resolve_form_id(FormId(0x100)).unwrap();
    assert_eq!(resolved.editor_id.as_deref(), Some("TestCell"));
    assert_eq!(resolver.resolve_editor_id("TESTCELL"), [resolved]);

    assert!(index.by_editor_id("does-not-exist").is_empty());
}

#[test]
fn t39_7_fingerprint_stable_and_change_sensitive() {
    let mut plugin = tes4(&[]);
    plugin.extend(edid_record(b"MISC", 0x400, "Widget"));

    let sources = [PluginSource {
        name: "plugin.esp",
        bytes: &plugin,
    }];
    let first = ContentIndex::build(&sources).unwrap();
    let second = ContentIndex::build(&sources).unwrap();
    assert_eq!(first.fingerprint(), second.fingerprint());

    let mut changed_plugin = tes4(&[]);
    changed_plugin.extend(edid_record(b"MISC", 0x400, "ChangedWidget"));
    let changed_sources = [PluginSource {
        name: "plugin.esp",
        bytes: &changed_plugin,
    }];
    let third = ContentIndex::build(&changed_sources).unwrap();
    assert_ne!(first.fingerprint(), third.fingerprint());
}

#[test]
fn t39_8_summary_is_byte_identical_across_runs() {
    let mut master = tes4(&[]);
    master.extend(edid_record(b"MISC", 0x400, "Widget"));
    master.extend(edid_record(b"MISC", 0x401, "OtherWidget"));

    let mut dependent = tes4(&["master.esm"]);
    dependent.extend(edid_record(b"STAT", 0x0000_0777, "Prop"));

    let sources = [
        PluginSource {
            name: "master.esm",
            bytes: &master,
        },
        PluginSource {
            name: "dependent.esp",
            bytes: &dependent,
        },
    ];

    let first = ContentIndex::build(&sources).unwrap().summary();
    let second = ContentIndex::build(&sources).unwrap().summary();
    assert_eq!(first, second);
    assert_eq!(
        first, "dependent.esp\tSTAT\t1\nmaster.esm\tMISC\t2",
        "summary should be sorted by plugin then record type"
    );
}

#[test]
fn t252_1_stream_exposes_decompressed_payload_and_resolved_identity() {
    let mut unrelated = tes4(&[]);
    unrelated.extend(edid_record(b"GLOB", 0x000001, "Unrelated"));
    let master = tes4(&[]);
    let mut script_payload = subrecord(b"EDID", b"CompressedScript\0");
    script_payload.extend(subrecord(b"SCRO", &0x0000_1234_u32.to_le_bytes()));
    script_payload.extend(subrecord(b"SCRO", &0x0100_5678_u32.to_le_bytes()));
    let mut dependent = tes4(&["master.esm"]);
    dependent.extend(compressed_record(b"SCPT", 0x0000_0400, &script_payload));

    let sources = [
        PluginSource {
            name: "unrelated.esm",
            bytes: &unrelated,
        },
        PluginSource {
            name: "master.esm",
            bytes: &master,
        },
        PluginSource {
            name: "dependent.esp",
            bytes: &dependent,
        },
    ];
    let mut observed = Vec::new();
    walk_resolved_records(&sources, |record| {
        if record.signature == "SCPT" {
            let payload = match record.payload {
                RecordPayload::Decoded(payload) => payload.to_vec(),
                RecordPayload::Unavailable(error) => panic!("unexpected payload error: {error}"),
            };
            observed.push((
                record.form_id,
                record.source_plugin.to_string(),
                record.flags,
                payload,
                record.resolve_form_id(0x0000_1234),
                record.resolve_form_id(0x0100_5678),
            ));
        }
    })
    .unwrap();

    assert_eq!(
        observed,
        [(
            FormId(0x0100_0400),
            "dependent.esp".to_string(),
            0x0004_0000,
            script_payload,
            FormId(0x0100_1234),
            FormId(0x0200_5678),
        )]
    );
}

#[test]
fn t252_2_stream_fans_out_override_and_deletion_events_in_load_order() {
    let mut master = tes4(&[]);
    master.extend(edid_record(b"SCPT", 0x400, "Original"));
    master.extend(edid_record(b"SCPT", 0x401, "Deleted"));
    let mut patch = tes4(&["master.esm"]);
    patch.extend(edid_record(b"SCPT", 0x400, "Overridden"));
    patch.extend(record(b"SCPT", 0x0000_0020, 0x401, &[]));
    let sources = [
        PluginSource {
            name: "master.esm",
            bytes: &master,
        },
        PluginSource {
            name: "patch.esp",
            bytes: &patch,
        },
    ];
    let mut all_record_count = 0;
    let mut scripts = Vec::new();
    let mut payload_winners = WinningRecords::default();
    let index = ContentIndex::build_with(&sources, |record| {
        all_record_count += 1;
        if record.signature == "SCPT" {
            scripts.push((
                record.form_id,
                record.source_plugin.to_string(),
                record.deleted,
            ));
            if record.deleted {
                payload_winners.delete(record.form_id.0);
            } else if let RecordPayload::Decoded(payload) = record.payload {
                payload_winners.upsert(
                    record.form_id.0,
                    record.source_plugin.to_string(),
                    payload.to_vec(),
                );
            }
        }
    })
    .unwrap();

    assert_eq!(all_record_count, 4);
    assert_eq!(
        scripts,
        [
            (FormId(0x400), "master.esm".to_string(), false),
            (FormId(0x401), "master.esm".to_string(), false),
            (FormId(0x400), "patch.esp".to_string(), false),
            (FormId(0x401), "patch.esp".to_string(), true),
        ]
    );
    let indexed_winner = index.get(FormId(0x400)).unwrap();
    assert_eq!(indexed_winner.editor_id.as_deref(), Some("Overridden"));
    assert_eq!(indexed_winner.provenance, ["master.esm", "patch.esp"]);
    assert!(index.get(FormId(0x401)).is_none());
    let payload_winner = payload_winners.get(0x400).unwrap();
    assert_eq!(payload_winner.value, subrecord(b"EDID", b"Overridden\0"));
    assert_eq!(payload_winner.provenance, ["master.esm", "patch.esp"]);
    assert!(payload_winners.get(0x401).is_none());
}

#[test]
fn t252_3_unavailable_payload_is_diagnostic_but_record_remains_indexed() {
    let mut plugin = tes4(&[]);
    plugin.extend(record(b"SCPT", 0x0004_0000, 0x400, &[4, 0, 0, 0, 0xff]));
    let sources = [PluginSource {
        name: "broken.esp",
        bytes: &plugin,
    }];

    let index = ContentIndex::build(&sources).unwrap();
    assert!(index.get(FormId(0x400)).is_some());
    assert!(
        index.diagnostics().iter().any(|diagnostic| {
            diagnostic.contains("broken.esp")
                && diagnostic.contains("SCPT")
                && diagnostic.contains("00000400")
        }),
        "diagnostics were {:?}",
        index.diagnostics()
    );
}
