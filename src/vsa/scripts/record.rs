//! Bevy-free structural representation of Fallout 3 script records.
//!
//! This layer preserves source and compiled bytes without interpreting either.
//! The catalog adapts the shared ESM4 subrecord parser into this input contract.

use bevyout_core::form_id::FormId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct EmbeddedScriptSlot(pub(crate) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ScriptAssetId {
    Record(FormId),
    Embedded {
        owner: FormId,
        slot: EmbeddedScriptSlot,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScriptKind {
    Object,
    Quest,
    Effect,
    Unknown(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ScriptHeader {
    pub(crate) reference_count: u32,
    pub(crate) compiled_size: u32,
    pub(crate) variable_count: u32,
    pub(crate) kind: ScriptKind,
    pub(crate) flags: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalDeclaration {
    pub(crate) slot: u32,
    pub(crate) flags: u8,
    pub(crate) name: Option<String>,
    pub(crate) raw: Vec<u8>,
    pub(crate) offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScriptReference {
    Form(FormId),
    LocalVariable(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RawScriptSubrecord {
    pub(crate) signature: String,
    pub(crate) data: Vec<u8>,
    pub(crate) offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RawScriptPayload {
    pub(crate) data: Vec<u8>,
    pub(crate) error_offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScriptRecord {
    pub(crate) id: ScriptAssetId,
    pub(crate) editor_id: Option<String>,
    pub(crate) header: Option<ScriptHeader>,
    pub(crate) header_raw: Option<Vec<u8>>,
    pub(crate) kind: Option<ScriptKind>,
    pub(crate) compiled_data: Option<Vec<u8>>,
    pub(crate) source_text: Option<Vec<u8>>,
    pub(crate) locals: Vec<LocalDeclaration>,
    pub(crate) references: Vec<ScriptReference>,
    pub(crate) unknown_subrecords: Vec<RawScriptSubrecord>,
    pub(crate) unparsed_payload: Option<RawScriptPayload>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScriptRecordDiagnostic {
    pub(crate) script: ScriptAssetId,
    pub(crate) source_plugin: String,
    pub(crate) subrecord: Option<String>,
    pub(crate) offset: Option<usize>,
    pub(crate) message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DecodedScriptRecord {
    pub(crate) record: ScriptRecord,
    pub(crate) diagnostics: Vec<ScriptRecordDiagnostic>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScriptSubrecordInput<'a> {
    pub(crate) signature: &'a str,
    pub(crate) data: &'a [u8],
    pub(crate) offset: usize,
}

pub(crate) fn decode_script_record(
    form_id: FormId,
    source_plugin: &str,
    subrecords: &[ScriptSubrecordInput<'_>],
    resolve_form_id: impl Fn(u32) -> FormId,
) -> DecodedScriptRecord {
    let id = ScriptAssetId::Record(form_id);
    let mut decoded = DecodedScriptRecord {
        record: ScriptRecord {
            id,
            editor_id: None,
            header: None,
            header_raw: None,
            kind: None,
            compiled_data: None,
            source_text: None,
            locals: Vec::new(),
            references: Vec::new(),
            unknown_subrecords: Vec::new(),
            unparsed_payload: None,
        },
        diagnostics: Vec::new(),
    };

    for subrecord in subrecords {
        match subrecord.signature {
            "EDID" => {
                if !set_bytes_once(
                    &mut decoded.record.editor_id,
                    cstring(subrecord.data),
                    id,
                    source_plugin,
                    subrecord,
                    &mut decoded.diagnostics,
                ) {
                    preserve_raw(&mut decoded.record, subrecord);
                }
            }
            "SCHR" => decode_header(&mut decoded, source_plugin, subrecord),
            "SCDA" => {
                if !set_bytes_once(
                    &mut decoded.record.compiled_data,
                    subrecord.data.to_vec(),
                    id,
                    source_plugin,
                    subrecord,
                    &mut decoded.diagnostics,
                ) {
                    preserve_raw(&mut decoded.record, subrecord);
                }
            }
            "SCTX" => {
                if !set_bytes_once(
                    &mut decoded.record.source_text,
                    subrecord.data.to_vec(),
                    id,
                    source_plugin,
                    subrecord,
                    &mut decoded.diagnostics,
                ) {
                    preserve_raw(&mut decoded.record, subrecord);
                }
            }
            "SLSD" => decode_local(&mut decoded, source_plugin, subrecord),
            "SCVR" => decode_local_name(&mut decoded, source_plugin, subrecord),
            "SCRO" => decode_reference(&mut decoded, source_plugin, subrecord, &resolve_form_id),
            "SCRV" => decode_local_reference(&mut decoded, source_plugin, subrecord),
            _ => preserve_raw(&mut decoded.record, subrecord),
        }
    }
    validate_header_counts(&mut decoded, source_plugin);
    diagnose_unnamed_locals(&mut decoded, source_plugin);
    decoded
}

fn decode_header(
    decoded: &mut DecodedScriptRecord,
    source_plugin: &str,
    subrecord: &ScriptSubrecordInput<'_>,
) {
    if decoded.record.header_raw.is_some() {
        duplicate_diagnostic(decoded, source_plugin, subrecord);
        preserve_raw(&mut decoded.record, subrecord);
        return;
    }
    decoded.record.header_raw = Some(subrecord.data.to_vec());
    if subrecord.data.len() != 20 {
        diagnostic(
            decoded,
            source_plugin,
            Some(subrecord),
            format!(
                "SCHR must be exactly 20 bytes, found {}",
                subrecord.data.len()
            ),
        );
        return;
    }
    let kind = match read_u16(subrecord.data, 16) {
        0 => ScriptKind::Object,
        1 => ScriptKind::Quest,
        0x100 => ScriptKind::Effect,
        value => ScriptKind::Unknown(value),
    };
    decoded.record.kind = Some(kind);
    decoded.record.header = Some(ScriptHeader {
        reference_count: read_u32(subrecord.data, 4),
        compiled_size: read_u32(subrecord.data, 8),
        variable_count: read_u32(subrecord.data, 12),
        kind,
        flags: read_u16(subrecord.data, 18),
    });
}

fn decode_local(
    decoded: &mut DecodedScriptRecord,
    source_plugin: &str,
    subrecord: &ScriptSubrecordInput<'_>,
) {
    if subrecord.data.len() != 24 {
        diagnostic(
            decoded,
            source_plugin,
            Some(subrecord),
            format!(
                "SLSD must be exactly 24 bytes, found {}",
                subrecord.data.len()
            ),
        );
        preserve_raw(&mut decoded.record, subrecord);
        return;
    }
    decoded.record.locals.push(LocalDeclaration {
        slot: read_u32(subrecord.data, 0),
        flags: subrecord.data[16],
        name: None,
        raw: subrecord.data.to_vec(),
        offset: subrecord.offset,
    });
}

fn decode_local_name(
    decoded: &mut DecodedScriptRecord,
    source_plugin: &str,
    subrecord: &ScriptSubrecordInput<'_>,
) {
    if let Some(local) = decoded
        .record
        .locals
        .iter_mut()
        .rev()
        .find(|local| local.name.is_none())
    {
        local.name = Some(cstring(subrecord.data));
    } else {
        diagnostic(
            decoded,
            source_plugin,
            Some(subrecord),
            "SCVR has no preceding unnamed SLSD".into(),
        );
        preserve_raw(&mut decoded.record, subrecord);
    }
}

fn decode_reference(
    decoded: &mut DecodedScriptRecord,
    source_plugin: &str,
    subrecord: &ScriptSubrecordInput<'_>,
    resolve_form_id: &impl Fn(u32) -> FormId,
) {
    if subrecord.data.len() != 4 {
        invalid_u32(decoded, source_plugin, subrecord);
        return;
    }
    decoded
        .record
        .references
        .push(ScriptReference::Form(resolve_form_id(read_u32(
            subrecord.data,
            0,
        ))));
}

fn decode_local_reference(
    decoded: &mut DecodedScriptRecord,
    source_plugin: &str,
    subrecord: &ScriptSubrecordInput<'_>,
) {
    if subrecord.data.len() != 4 {
        invalid_u32(decoded, source_plugin, subrecord);
        return;
    }
    decoded
        .record
        .references
        .push(ScriptReference::LocalVariable(read_u32(subrecord.data, 0)));
}

fn invalid_u32(
    decoded: &mut DecodedScriptRecord,
    source_plugin: &str,
    subrecord: &ScriptSubrecordInput<'_>,
) {
    diagnostic(
        decoded,
        source_plugin,
        Some(subrecord),
        format!(
            "{} must be exactly 4 bytes, found {}",
            subrecord.signature,
            subrecord.data.len()
        ),
    );
    preserve_raw(&mut decoded.record, subrecord);
}

fn validate_header_counts(decoded: &mut DecodedScriptRecord, source_plugin: &str) {
    let Some(header) = decoded.record.header else {
        if decoded.record.header_raw.is_none() {
            diagnostic(
                decoded,
                source_plugin,
                None,
                "missing required SCHR subrecord".into(),
            );
        }
        return;
    };
    let compiled_size = decoded.record.compiled_data.as_ref().map_or(0, Vec::len);
    if compiled_size != header.compiled_size as usize {
        diagnostic(
            decoded,
            source_plugin,
            None,
            format!(
                "SCHR compiled size {} does not match SCDA size {}",
                header.compiled_size, compiled_size
            ),
        );
    }
    if decoded.record.locals.len() != header.variable_count as usize {
        diagnostic(
            decoded,
            source_plugin,
            None,
            format!(
                "SCHR variable count {} does not match {} SLSD entries",
                header.variable_count,
                decoded.record.locals.len()
            ),
        );
    }
    if decoded.record.references.len() != header.reference_count as usize {
        diagnostic(
            decoded,
            source_plugin,
            None,
            format!(
                "SCHR reference count {} does not match {} SCRO/SCRV entries",
                header.reference_count,
                decoded.record.references.len()
            ),
        );
    }
}

fn diagnose_unnamed_locals(decoded: &mut DecodedScriptRecord, source_plugin: &str) {
    let unnamed = decoded
        .record
        .locals
        .iter()
        .filter(|local| local.name.is_none())
        .map(|local| (local.slot, local.offset))
        .collect::<Vec<_>>();
    for (slot, offset) in unnamed {
        decoded.diagnostics.push(ScriptRecordDiagnostic {
            script: decoded.record.id,
            source_plugin: source_plugin.into(),
            subrecord: Some("SLSD".into()),
            offset: Some(offset),
            message: format!("local slot {slot} has no paired SCVR name"),
        });
    }
}

fn set_bytes_once<T>(
    target: &mut Option<T>,
    value: T,
    id: ScriptAssetId,
    source_plugin: &str,
    subrecord: &ScriptSubrecordInput<'_>,
    diagnostics: &mut Vec<ScriptRecordDiagnostic>,
) -> bool {
    if target.is_none() {
        *target = Some(value);
        true
    } else {
        diagnostics.push(ScriptRecordDiagnostic {
            script: id,
            source_plugin: source_plugin.into(),
            subrecord: Some(subrecord.signature.into()),
            offset: Some(subrecord.offset),
            message: format!("duplicate {} subrecord", subrecord.signature),
        });
        false
    }
}

fn duplicate_diagnostic(
    decoded: &mut DecodedScriptRecord,
    source_plugin: &str,
    subrecord: &ScriptSubrecordInput<'_>,
) {
    diagnostic(
        decoded,
        source_plugin,
        Some(subrecord),
        format!("duplicate {} subrecord", subrecord.signature),
    );
}

fn diagnostic(
    decoded: &mut DecodedScriptRecord,
    source_plugin: &str,
    subrecord: Option<&ScriptSubrecordInput<'_>>,
    message: String,
) {
    decoded.diagnostics.push(ScriptRecordDiagnostic {
        script: decoded.record.id,
        source_plugin: source_plugin.into(),
        subrecord: subrecord.map(|subrecord| subrecord.signature.into()),
        offset: subrecord.map(|subrecord| subrecord.offset),
        message,
    });
}

fn preserve_raw(record: &mut ScriptRecord, subrecord: &ScriptSubrecordInput<'_>) {
    record.unknown_subrecords.push(RawScriptSubrecord {
        signature: subrecord.signature.into(),
        data: subrecord.data.to_vec(),
        offset: subrecord.offset,
    });
}

fn cstring(data: &[u8]) -> String {
    String::from_utf8_lossy(data)
        .trim_end_matches('\0')
        .to_string()
}

fn read_u16(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap())
}

fn read_u32(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap())
}
