//! Bevy-free structural extraction of direct and embedded script attachments.

use bevyout_core::form_id::FormId;

use super::record::{
    DecodedScriptRecord, EmbeddedScriptSlot, PackageScriptSlot, ScriptAssetId,
    ScriptRecordDiagnostic, ScriptSubrecordInput, decode_script_asset,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ScriptAttachmentSlot {
    Direct(u32),
    Embedded(EmbeddedScriptSlot),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExtractedScriptAttachment {
    pub(crate) owner: FormId,
    pub(crate) owner_signature: String,
    pub(crate) slot: ScriptAttachmentSlot,
    pub(crate) script: ScriptAssetId,
    pub(crate) source_offset: usize,
}

#[derive(Debug, Default)]
pub(crate) struct ExtractedOwnerScripts {
    pub(crate) attachments: Vec<ExtractedScriptAttachment>,
    pub(crate) embedded: Vec<DecodedScriptRecord>,
    pub(crate) diagnostics: Vec<ScriptRecordDiagnostic>,
}

pub(crate) fn extract_owner_scripts(
    owner: FormId,
    owner_signature: &str,
    source_plugin: &str,
    subrecords: &[ScriptSubrecordInput<'_>],
    resolve_form_id: impl Fn(u32) -> FormId,
) -> ExtractedOwnerScripts {
    let mut extracted = ExtractedOwnerScripts::default();
    for (index, subrecord) in subrecords
        .iter()
        .filter(|subrecord| subrecord.signature == "SCRI")
        .enumerate()
    {
        if subrecord.data.len() != 4 {
            extracted.diagnostics.push(ScriptRecordDiagnostic {
                script: ScriptAssetId::Record(owner),
                source_plugin: source_plugin.into(),
                subrecord: Some("SCRI".into()),
                offset: Some(subrecord.offset),
                message: format!(
                    "{owner_signature} {owner} SCRI must be exactly 4 bytes, found {}",
                    subrecord.data.len()
                ),
            });
            continue;
        }
        let raw = u32::from_le_bytes(subrecord.data.try_into().unwrap());
        extracted.attachments.push(ExtractedScriptAttachment {
            owner,
            owner_signature: owner_signature.into(),
            slot: ScriptAttachmentSlot::Direct(index as u32),
            script: ScriptAssetId::Record(resolve_form_id(raw)),
            source_offset: subrecord.offset,
        });
    }

    if owner_signature == "PACK" {
        extract_package_scripts(
            owner,
            source_plugin,
            subrecords,
            &resolve_form_id,
            &mut extracted,
        );
    }
    extracted
}

fn extract_package_scripts(
    owner: FormId,
    source_plugin: &str,
    subrecords: &[ScriptSubrecordInput<'_>],
    resolve_form_id: &impl Fn(u32) -> FormId,
    extracted: &mut ExtractedOwnerScripts,
) {
    let mut marker = None;
    let mut after_idle = false;
    let mut script_subrecords = Vec::new();

    for subrecord in subrecords {
        if let Some(next_marker) = package_marker(subrecord.signature) {
            finish_package_script(
                owner,
                source_plugin,
                marker,
                &script_subrecords,
                resolve_form_id,
                extracted,
            );
            marker = Some((next_marker, subrecord.offset));
            after_idle = false;
            script_subrecords.clear();
            continue;
        }
        if marker.is_some() && subrecord.signature == "INAM" && script_subrecords.is_empty() {
            after_idle = true;
            continue;
        }
        if subrecord.signature == "TNAM" {
            finish_package_script(
                owner,
                source_plugin,
                marker,
                &script_subrecords,
                resolve_form_id,
                extracted,
            );
            marker = None;
            after_idle = false;
            script_subrecords.clear();
            continue;
        }
        if marker.is_some()
            && (after_idle
                || !script_subrecords.is_empty()
                || is_script_subrecord(subrecord.signature))
        {
            script_subrecords.push(*subrecord);
        }
    }
    finish_package_script(
        owner,
        source_plugin,
        marker,
        &script_subrecords,
        resolve_form_id,
        extracted,
    );
}

fn finish_package_script(
    owner: FormId,
    source_plugin: &str,
    marker: Option<(EmbeddedScriptSlot, usize)>,
    subrecords: &[ScriptSubrecordInput<'_>],
    resolve_form_id: &impl Fn(u32) -> FormId,
    extracted: &mut ExtractedOwnerScripts,
) {
    let Some((slot, source_offset)) = marker else {
        return;
    };
    let id = ScriptAssetId::Embedded { owner, slot };
    let decoded = decode_script_asset(id, source_plugin, subrecords, resolve_form_id);
    extracted
        .diagnostics
        .extend(decoded.diagnostics.iter().cloned());
    extracted.attachments.push(ExtractedScriptAttachment {
        owner,
        owner_signature: "PACK".into(),
        slot: ScriptAttachmentSlot::Embedded(slot),
        script: id,
        source_offset,
    });
    extracted.embedded.push(decoded);
}

fn package_marker(signature: &str) -> Option<EmbeddedScriptSlot> {
    match signature {
        "POBA" => Some(EmbeddedScriptSlot::Package(PackageScriptSlot::Begin)),
        "POCA" => Some(EmbeddedScriptSlot::Package(PackageScriptSlot::Change)),
        "POEA" => Some(EmbeddedScriptSlot::Package(PackageScriptSlot::End)),
        _ => None,
    }
}

fn is_script_subrecord(signature: &str) -> bool {
    matches!(
        signature,
        "SCHR" | "SCDA" | "SCTX" | "SLSD" | "SCVR" | "SCRO" | "SCRV"
    )
}
