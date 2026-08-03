//! Synthetic IDLE record fixtures for Lane C's authored-idle decoder.

use super::*;

fn anam(parent: u32, previous: u32) -> Vec<u8> {
    [parent.to_le_bytes(), previous.to_le_bytes()].concat()
}

fn data8(group: u8, min: u8, max: u8, replay: i16, flags: u8) -> Vec<u8> {
    [
        vec![group, min, max, 0],
        replay.to_le_bytes().to_vec(),
        vec![flags, 0],
    ]
    .concat()
}

#[test]
fn idle_decoder_preserves_links_path_conditions_and_data_metadata() {
    let idle = parse_idle(
        &[
            direct_subrecord("EDID", b"Swatting\0".to_vec()),
            direct_subrecord(
                "MODL",
                b"Characters\\_Male\\IdleAnims\\Swatting.KF\0".to_vec(),
            ),
            direct_subrecord("CTDA", vec![1, 2, 3, 4]),
            direct_subrecord("CTDA", vec![5, 6]),
            direct_subrecord("ANAM", anam(0x100, 0x101)),
            direct_subrecord("DATA", data8(0x87, 2, 4, -3, 0x11)),
        ],
        0x200,
        0,
        &direct_resolver(),
    );

    assert_eq!(idle.form_id, 0x200);
    assert_eq!(idle.editor_id.as_deref(), Some("Swatting"));
    assert_eq!(
        idle.model_path.as_deref(),
        Some("Characters\\_Male\\IdleAnims\\Swatting.KF")
    );
    assert_eq!(idle.parent_form_id, Some(0x100));
    assert_eq!(idle.previous_sibling_form_id, Some(0x101));
    assert_eq!(idle.conditions, vec![vec![1, 2, 3, 4], vec![5, 6]]);
    assert_eq!(idle.group_section_raw, 0x87);
    assert_eq!(idle.group_section, 7);
    assert_eq!(idle.loop_min, 2);
    assert_eq!(idle.loop_max, 4);
    assert_eq!(idle.replay_delay_seconds, -3);
    assert_eq!(idle.flags, 0x11);
}

#[test]
fn idle_decoder_accepts_compact_six_byte_data_and_salvages_truncation() {
    let compact = parse_idle(
        &[direct_subrecord("DATA", vec![0x54, 1, 3, 0x05, 0, 0x22])],
        1,
        0,
        &direct_resolver(),
    );
    assert_eq!(compact.group_section_raw, 0x54);
    assert_eq!(compact.group_section, 20);
    assert_eq!(compact.loop_min, 1);
    assert_eq!(compact.loop_max, 3);
    assert_eq!(compact.replay_delay_seconds, 5);
    assert_eq!(compact.flags, 0x22);

    let truncated = parse_idle(
        &[
            direct_subrecord("DATA", vec![0x47, 1, 2]),
            direct_subrecord("WHAT", vec![9]),
        ],
        2,
        0,
        &direct_resolver(),
    );
    assert_eq!(truncated.form_id, 2);
    assert!(
        truncated
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.contains("DATA"))
    );
    assert!(
        truncated
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.contains("WHAT"))
    );
}

#[test]
fn idle_decoder_reports_duplicate_fields_without_dropping_record() {
    let idle = parse_idle(
        &[
            direct_subrecord("EDID", b"First\0".to_vec()),
            direct_subrecord("EDID", b"Second\0".to_vec()),
            direct_subrecord("DATA", data8(0x54, 0, 0, 0, 0)),
            direct_subrecord("DATA", data8(0x47, 0, 0, 0, 0)),
        ],
        3,
        0,
        &direct_resolver(),
    );
    assert_eq!(idle.editor_id.as_deref(), Some("First"));
    assert_eq!(idle.group_section_raw, 0x54);
    assert!(
        idle.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.contains("duplicate EDID"))
    );
    assert!(
        idle.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.contains("duplicate DATA"))
    );
}

#[test]
fn idle_winners_obey_override_and_deletion_across_plugins() {
    let cell_id = 0x0000_1000;
    let idle_id = 0x0000_0200;
    let base = [
        tes4(&[]),
        record(
            b"IDLE",
            0,
            idle_id,
            &[
                subrecord(b"EDID", b"Base\0"),
                subrecord(b"DATA", &data8(0x47, 0, 0, 0, 0)),
            ]
            .concat(),
        ),
        record(b"CELL", 0, cell_id, &[]),
    ]
    .concat();
    let override_plugin = [
        tes4(&["Fallout3.esm"]),
        record(
            b"IDLE",
            0,
            idle_id,
            &[
                subrecord(b"EDID", b"Override\0"),
                subrecord(b"DATA", &data8(0x54, 0, 0, 0, 0)),
            ]
            .concat(),
        ),
    ]
    .concat();

    let parsed = parse_content_set(
        &[
            PluginSource {
                name: "Fallout3.esm",
                bytes: &base,
            },
            PluginSource {
                name: "Override.esp",
                bytes: &override_plugin,
            },
        ],
        &CellSelector::FormId(cell_id),
    )
    .unwrap();
    assert_eq!(
        parsed.idles[&idle_id].editor_id.as_deref(),
        Some("Override")
    );

    let deletion = [
        tes4(&["Fallout3.esm"]),
        record(b"IDLE", RECORD_DELETED, idle_id, &[]),
    ]
    .concat();
    let deleted = parse_content_set(
        &[
            PluginSource {
                name: "Fallout3.esm",
                bytes: &base,
            },
            PluginSource {
                name: "Delete.esp",
                bytes: &deletion,
            },
        ],
        &CellSelector::FormId(cell_id),
    )
    .unwrap();
    assert!(!deleted.idles.contains_key(&idle_id));
}
