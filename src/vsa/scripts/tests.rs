use super::attachments::{ScriptAttachmentSlot, extract_owner_scripts};
use super::catalog::ScriptCatalog;
use super::record::{
    EmbeddedScriptSlot, PackageScriptSlot, ScriptAssetId, ScriptKind, ScriptReference,
    ScriptSubrecordInput, decode_script_record,
};
use crate::vsa::content_index::{FormId, PluginSource};
use crate::vsa::openmw_esm4::parse_subrecords_with_offsets;

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

fn header(references: u32, compiled_size: u32, variables: u32, kind: u16) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend(0_u32.to_le_bytes());
    data.extend(references.to_le_bytes());
    data.extend(compiled_size.to_le_bytes());
    data.extend(variables.to_le_bytes());
    data.extend(kind.to_le_bytes());
    data.extend(1_u16.to_le_bytes());
    data
}

fn script_payload(editor_id: &str, reference: u32) -> Vec<u8> {
    let mut local = vec![0; 24];
    local[0..4].copy_from_slice(&3_u32.to_le_bytes());
    local[16] = 1;
    [
        subrecord(b"EDID", format!("{editor_id}\0").as_bytes()),
        subrecord(b"SCHR", &header(1, 3, 1, 0)),
        subrecord(b"SCDA", &[1, 2, 3]),
        subrecord(b"SCTX", b"begin GameMode\0"),
        subrecord(b"SLSD", &local),
        subrecord(b"SCVR", b"Counter\0"),
        subrecord(b"SCRO", &reference.to_le_bytes()),
        subrecord(b"UNKN", &[9, 8, 7]),
    ]
    .concat()
}

#[test]
fn catalog_decodes_the_winning_script_and_resolves_payload_form_ids() {
    let mut master = tes4(&[]);
    master.extend(record(
        b"SCPT",
        0,
        0x400,
        &script_payload("Original", 0x0000_1234),
    ));
    let mut patch = tes4(&["master.esm"]);
    patch.extend(record(
        b"SCPT",
        0,
        0x400,
        &script_payload("Patched", 0x0100_5678),
    ));
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

    let (index, catalog) = ScriptCatalog::build(&sources).unwrap();
    let script = catalog.get(ScriptAssetId::Record(FormId(0x400))).unwrap();
    assert_eq!(script.record.editor_id.as_deref(), Some("Patched"));
    assert_eq!(script.record.kind, Some(ScriptKind::Object));
    assert_eq!(script.record.compiled_data.as_deref(), Some(&[1, 2, 3][..]));
    assert_eq!(
        script.record.source_text.as_deref(),
        Some(&b"begin GameMode\0"[..])
    );
    assert_eq!(script.record.locals[0].slot, 3);
    assert_eq!(script.record.locals[0].name.as_deref(), Some("Counter"));
    assert_eq!(
        script.record.references,
        [ScriptReference::Form(FormId(0x0100_5678))]
    );
    assert_eq!(script.record.unknown_subrecords[0].signature, "UNKN");
    assert_eq!(script.winning_plugin, "patch.esp");
    assert_eq!(script.provenance, ["master.esm", "patch.esp"]);
    assert_eq!(catalog.content_fingerprint(), index.fingerprint());
    assert!(catalog.diagnostics().is_empty());
}

#[test]
fn deleted_override_removes_the_script() {
    let mut master = tes4(&[]);
    master.extend(record(
        b"SCPT",
        0,
        0x400,
        &script_payload("Original", 0x0000_1234),
    ));
    let mut patch = tes4(&["master.esm"]);
    patch.extend(record(b"SCPT", 0x20, 0x400, &[]));
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

    let (_index, catalog) = ScriptCatalog::build(&sources).unwrap();
    assert_eq!(catalog.scripts().count(), 0);
}

#[test]
fn malformed_header_and_unknown_subrecord_are_reported_without_panicking() {
    let payload = [subrecord(b"SCHR", &[0; 3]), subrecord(b"ZZZZ", &[4, 5, 6])].concat();
    let mut plugin = tes4(&[]);
    plugin.extend(record(b"SCPT", 0, 0x400, &payload));
    let sources = [PluginSource {
        name: "broken.esp",
        bytes: &plugin,
    }];

    let (_index, catalog) = ScriptCatalog::build(&sources).unwrap();
    let script = catalog.get(ScriptAssetId::Record(FormId(0x400))).unwrap();
    assert!(script.record.header.is_none());
    assert_eq!(script.record.header_raw.as_deref(), Some(&[0; 3][..]));
    assert_eq!(script.record.unknown_subrecords[0].signature, "ZZZZ");
    assert!(catalog.diagnostics().iter().any(|diagnostic| {
        diagnostic.source_plugin == "broken.esp"
            && diagnostic.subrecord.as_deref() == Some("SCHR")
            && diagnostic.offset == Some(0)
            && diagnostic.message.contains("20 bytes")
    }));
}

#[test]
fn malformed_framing_preserves_the_complete_payload_and_failure_offset() {
    let mut payload = subrecord(b"SCHR", &header(0, 0, 0, 0));
    let failure_offset = payload.len();
    payload.extend([b'B', b'A']);
    let mut plugin = tes4(&[]);
    plugin.extend(record(b"SCPT", 0, 0x400, &payload));
    let sources = [PluginSource {
        name: "broken.esp",
        bytes: &plugin,
    }];

    let (_index, catalog) = ScriptCatalog::build(&sources).unwrap();
    let script = catalog.get(ScriptAssetId::Record(FormId(0x400))).unwrap();
    let unparsed = script.record.unparsed_payload.as_ref().unwrap();
    assert_eq!(unparsed.data, payload);
    assert_eq!(unparsed.error_offset, failure_offset);
    assert!(catalog.diagnostics().iter().any(|diagnostic| {
        diagnostic.source_plugin == "broken.esp"
            && diagnostic.offset == Some(failure_offset)
            && diagnostic.message.contains("truncated subrecord header")
    }));
}

#[test]
fn source_only_zero_byte_script_does_not_report_a_false_scda_mismatch() {
    let header = header(0, 0, 0, 0);
    let inputs = [
        ScriptSubrecordInput {
            signature: "SCHR",
            data: &header,
            offset: 0,
        },
        ScriptSubrecordInput {
            signature: "SCTX",
            data: b"begin GameMode\0",
            offset: 26,
        },
    ];
    let decoded = decode_script_record(FormId(0x400), "source.esp", &inputs, FormId);
    assert!(
        decoded
            .diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.message.contains("compiled size"))
    );
}

#[test]
fn missing_header_and_local_name_are_diagnosed() {
    let mut local = vec![0; 24];
    local[0..4].copy_from_slice(&7_u32.to_le_bytes());
    let inputs = [ScriptSubrecordInput {
        signature: "SLSD",
        data: &local,
        offset: 12,
    }];
    let decoded = decode_script_record(FormId(0x400), "broken.esp", &inputs, FormId);
    assert!(
        decoded
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("missing required SCHR"))
    );
    assert!(decoded.diagnostics.iter().any(|diagnostic| {
        diagnostic.offset == Some(12) && diagnostic.message.contains("no paired SCVR")
    }));
}

#[test]
fn duplicate_source_and_compiled_subrecords_are_preserved_raw() {
    let header = header(0, 1, 0, 0);
    let inputs = [
        ScriptSubrecordInput {
            signature: "SCHR",
            data: &header,
            offset: 0,
        },
        ScriptSubrecordInput {
            signature: "SCDA",
            data: &[1],
            offset: 26,
        },
        ScriptSubrecordInput {
            signature: "SCDA",
            data: &[2],
            offset: 33,
        },
        ScriptSubrecordInput {
            signature: "SCTX",
            data: b"first",
            offset: 40,
        },
        ScriptSubrecordInput {
            signature: "SCTX",
            data: b"second",
            offset: 51,
        },
    ];
    let decoded = decode_script_record(FormId(0x400), "duplicate.esp", &inputs, FormId);
    assert_eq!(decoded.record.compiled_data.as_deref(), Some(&[1][..]));
    assert_eq!(decoded.record.source_text.as_deref(), Some(&b"first"[..]));
    assert!(
        decoded
            .record
            .unknown_subrecords
            .iter()
            .any(|subrecord| { subrecord.signature == "SCDA" && subrecord.data == [2] })
    );
    assert!(
        decoded
            .record
            .unknown_subrecords
            .iter()
            .any(|subrecord| { subrecord.signature == "SCTX" && subrecord.data == b"second" })
    );
}

#[test]
fn dangling_extended_size_marker_is_rejected_at_its_offset() {
    let payload = subrecord(b"XXXX", &12_u32.to_le_bytes());
    let error = parse_subrecords_with_offsets(&payload).unwrap_err();
    assert_eq!(error.offset, 0);
    assert_eq!(error.signature.as_deref(), Some("XXXX"));
    assert!(error.message.contains("no following subrecord"));
}

#[test]
fn direct_attachment_is_resolved_and_winning_owner_replaces_prior_attachment() {
    let mut master = tes4(&[]);
    master.extend(record(
        b"ACTI",
        0,
        0x500,
        &subrecord(b"SCRI", &0x400_u32.to_le_bytes()),
    ));
    master.extend(record(b"SCPT", 0, 0x400, &script_payload("Old", 0)));
    master.extend(record(b"SCPT", 0, 0x401, &script_payload("New", 0)));
    let mut patch = tes4(&["master.esm"]);
    patch.extend(record(
        b"ACTI",
        0,
        0x500,
        &subrecord(b"SCRI", &0x0000_0401_u32.to_le_bytes()),
    ));
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

    let (_index, catalog) = ScriptCatalog::build(&sources).unwrap();
    let attachments = catalog.attachments().collect::<Vec<_>>();
    assert_eq!(attachments.len(), 1);
    assert_eq!(attachments[0].owner, FormId(0x500));
    assert_eq!(attachments[0].owner_signature, "ACTI");
    assert_eq!(attachments[0].slot, ScriptAttachmentSlot::Direct(0));
    assert_eq!(attachments[0].script, ScriptAssetId::Record(FormId(0x401)));
    assert_eq!(attachments[0].winning_plugin, "patch.esp");
    assert_eq!(attachments[0].provenance, ["master.esm", "patch.esp"]);
}

#[test]
fn missing_direct_attachment_target_is_contextual_but_retained() {
    let mut plugin = tes4(&[]);
    plugin.extend(record(
        b"MISC",
        0,
        0x500,
        &subrecord(b"SCRI", &0x999_u32.to_le_bytes()),
    ));
    let sources = [PluginSource {
        name: "missing.esp",
        bytes: &plugin,
    }];

    let (_index, catalog) = ScriptCatalog::build(&sources).unwrap();
    assert_eq!(catalog.attachments().count(), 1);
    assert!(catalog.diagnostics().iter().any(|diagnostic| {
        diagnostic.source_plugin == "missing.esp"
            && diagnostic.subrecord.as_deref() == Some("SCRI")
            && diagnostic.message.contains("00000999")
    }));
}

#[test]
fn package_embedded_scripts_use_named_slots_and_preserve_unknown_content() {
    let payload = [
        subrecord(b"EDID", b"Package\0"),
        subrecord(b"POBA", &[]),
        subrecord(b"SCHR", &header(0, 1, 0, 0)),
        subrecord(b"SCDA", &[0xaa]),
        subrecord(b"XTRA", &[1, 2]),
        subrecord(b"TNAM", &0_u32.to_le_bytes()),
        subrecord(b"POEA", &[]),
        subrecord(b"SCHR", &header(0, 1, 0, 0)),
        subrecord(b"SCDA", &[0xbb]),
    ]
    .concat();
    let mut plugin = tes4(&[]);
    plugin.extend(record(b"PACK", 0, 0x600, &payload));
    let sources = [PluginSource {
        name: "package.esp",
        bytes: &plugin,
    }];

    let (_index, catalog) = ScriptCatalog::build(&sources).unwrap();
    let begin = catalog
        .get(ScriptAssetId::Embedded {
            owner: FormId(0x600),
            slot: EmbeddedScriptSlot::Package(PackageScriptSlot::Begin),
        })
        .unwrap();
    let end = catalog
        .get(ScriptAssetId::Embedded {
            owner: FormId(0x600),
            slot: EmbeddedScriptSlot::Package(PackageScriptSlot::End),
        })
        .unwrap();
    assert_eq!(begin.record.compiled_data.as_deref(), Some(&[0xaa][..]));
    assert_eq!(begin.record.unknown_subrecords[0].signature, "XTRA");
    assert_eq!(end.record.compiled_data.as_deref(), Some(&[0xbb][..]));
    assert_eq!(catalog.attachments().count(), 2);
}

#[test]
fn deleted_owner_removes_direct_and_embedded_attachments() {
    let payload = [
        subrecord(b"POCA", &[]),
        subrecord(b"SCHR", &header(0, 0, 0, 0)),
    ]
    .concat();
    let mut master = tes4(&[]);
    master.extend(record(b"PACK", 0, 0x600, &payload));
    let mut patch = tes4(&["master.esm"]);
    patch.extend(record(b"PACK", 0x20, 0x600, &[]));
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

    let (_index, catalog) = ScriptCatalog::build(&sources).unwrap();
    assert_eq!(catalog.attachments().count(), 0);
    assert!(
        catalog
            .get(ScriptAssetId::Embedded {
                owner: FormId(0x600),
                slot: EmbeddedScriptSlot::Package(PackageScriptSlot::Change),
            })
            .is_none()
    );
}

#[test]
fn structural_extractor_rejects_malformed_scri_without_dropping_other_slots() {
    let inputs = [
        ScriptSubrecordInput {
            signature: "SCRI",
            data: &[1, 2],
            offset: 7,
        },
        ScriptSubrecordInput {
            signature: "SCRI",
            data: &0x400_u32.to_le_bytes(),
            offset: 15,
        },
    ];
    let extracted = extract_owner_scripts(FormId(0x500), "ACTI", "fixture.esp", &inputs, FormId);
    assert_eq!(extracted.attachments.len(), 1);
    assert_eq!(
        extracted.attachments[0].slot,
        ScriptAttachmentSlot::Direct(1)
    );
    assert!(extracted.diagnostics[0].message.contains("exactly 4 bytes"));
}

#[test]
fn package_group_without_header_is_retained_and_diagnosed() {
    let inputs = [
        ScriptSubrecordInput {
            signature: "POBA",
            data: &[],
            offset: 4,
        },
        ScriptSubrecordInput {
            signature: "INAM",
            data: &0_u32.to_le_bytes(),
            offset: 10,
        },
        ScriptSubrecordInput {
            signature: "SCDA",
            data: &[0xaa],
            offset: 20,
        },
        ScriptSubrecordInput {
            signature: "XTRA",
            data: &[1, 2],
            offset: 27,
        },
        ScriptSubrecordInput {
            signature: "TNAM",
            data: &0_u32.to_le_bytes(),
            offset: 35,
        },
    ];
    let extracted = extract_owner_scripts(FormId(0x600), "PACK", "fixture.esp", &inputs, FormId);
    assert_eq!(extracted.embedded.len(), 1);
    assert_eq!(
        extracted.embedded[0].record.compiled_data.as_deref(),
        Some(&[0xaa][..])
    );
    assert_eq!(
        extracted.embedded[0].record.unknown_subrecords[0].signature,
        "XTRA"
    );
    assert!(
        extracted
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("missing required SCHR"))
    );
}

#[test]
fn package_topic_closes_the_embedded_group() {
    let header = header(0, 0, 0, 0);
    let inputs = [
        ScriptSubrecordInput {
            signature: "POBA",
            data: &[],
            offset: 0,
        },
        ScriptSubrecordInput {
            signature: "SCHR",
            data: &header,
            offset: 6,
        },
        ScriptSubrecordInput {
            signature: "TNAM",
            data: &0_u32.to_le_bytes(),
            offset: 32,
        },
        ScriptSubrecordInput {
            signature: "SCHR",
            data: &header,
            offset: 42,
        },
    ];
    let extracted = extract_owner_scripts(FormId(0x600), "PACK", "fixture.esp", &inputs, FormId);
    assert_eq!(extracted.embedded.len(), 1);
    assert!(extracted.diagnostics.is_empty());
}

#[test]
fn empty_package_action_slot_is_not_counted_as_an_embedded_script() {
    let inputs = [
        ScriptSubrecordInput {
            signature: "POBA",
            data: &[],
            offset: 0,
        },
        ScriptSubrecordInput {
            signature: "INAM",
            data: &0_u32.to_le_bytes(),
            offset: 6,
        },
        ScriptSubrecordInput {
            signature: "TNAM",
            data: &0_u32.to_le_bytes(),
            offset: 16,
        },
    ];
    let extracted = extract_owner_scripts(FormId(0x600), "PACK", "fixture.esp", &inputs, FormId);
    assert!(extracted.embedded.is_empty());
    assert!(extracted.attachments.is_empty());
    assert!(extracted.diagnostics.is_empty());
}

#[test]
fn diagnostics_are_stable_by_winning_form_id() {
    let mut plugin = tes4(&[]);
    plugin.extend(record(b"SCPT", 0, 0x500, &[]));
    plugin.extend(record(b"SCPT", 0, 0x400, &[]));
    let sources = [PluginSource {
        name: "order.esp",
        bytes: &plugin,
    }];

    let (_index, catalog) = ScriptCatalog::build(&sources).unwrap();
    let scripts = catalog
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.script)
        .collect::<Vec<_>>();
    assert_eq!(
        scripts,
        [
            ScriptAssetId::Record(FormId(0x400)),
            ScriptAssetId::Record(FormId(0x500)),
        ]
    );
}
