//! Walks a plugin's record/subrecord inventory into a [`CompatibilityReport`].

use anyhow::Result;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

use super::registry::{asset_format_status, record_status, subrecord_status};
use super::schema::{
    CURRENT_REPORT_SCHEMA_VERSION, CompatibilityReport, ReportClass, ReportEntry,
    ScriptAttachmentEntry, ScriptDiagnosticEntry, ScriptInventoryEntry, ScriptInventoryReport,
    ScriptInventoryTotals, SupportStatus,
};
use crate::vsa::content_index::PluginSource;
use crate::vsa::openmw_esm4::{cstring, inventory_records};
use crate::vsa::scripts::{
    EmbeddedScriptSlot, PackageScriptSlot, ScriptAssetId, ScriptAttachmentSlot, ScriptCatalog,
    ScriptKind,
};

struct Accumulator {
    status: SupportStatus,
    save_affecting: bool,
    provenance: BTreeSet<String>,
}

/// Builds a deterministic compatibility report for one plugin's bytes.
///
/// `plugin_name` should be a bare file name (no directory component) so the
/// report is portable and stays byte-identical across machines (F37.3).
#[cfg(test)]
pub(crate) fn generate_report(plugin_name: &str, bytes: &[u8]) -> Result<CompatibilityReport> {
    let sources = [PluginSource {
        name: plugin_name,
        bytes,
    }];
    generate_report_for_sources(plugin_name, bytes, &sources)
}

pub(crate) fn generate_report_for_sources(
    plugin_name: &str,
    bytes: &[u8],
    sources: &[PluginSource<'_>],
) -> Result<CompatibilityReport> {
    let records = inventory_records(bytes)?;
    let mut accum: BTreeMap<(ReportClass, String), Accumulator> = BTreeMap::new();

    for record in &records {
        let location = format!("{plugin_name}:{:08x}", record.form_id);
        note(
            &mut accum,
            ReportClass::Record,
            record.kind.clone(),
            record_status(&record.kind),
            location.clone(),
        );

        if record.kind == "QUST" {
            note(
                &mut accum,
                ReportClass::Quest,
                format!("{:08x}", record.form_id),
                record_status("QUST"),
                location.clone(),
            );
        }

        for (signature, data) in &record.subrecords {
            if signature == "CTDA" {
                note(
                    &mut accum,
                    ReportClass::Condition,
                    "CTDA".to_string(),
                    Some((SupportStatus::Unsupported, true)),
                    location.clone(),
                );
                continue;
            }
            if record.kind == "SCPT" && (signature == "SCTX" || signature == "SCDA") {
                note(
                    &mut accum,
                    ReportClass::ScriptFunction,
                    format!("SCPT.{signature}"),
                    Some((SupportStatus::Unsupported, true)),
                    location.clone(),
                );
                continue;
            }
            if signature == "MODL" || signature == "MOD2" {
                let Some(extension) = asset_extension(&cstring(data)) else {
                    continue;
                };
                note(
                    &mut accum,
                    ReportClass::AssetFormat,
                    extension.clone(),
                    asset_format_status(&extension),
                    location.clone(),
                );
                continue;
            }
            note(
                &mut accum,
                ReportClass::Subrecord,
                format!("{}.{signature}", record.kind),
                subrecord_status(signature),
                location.clone(),
            );
        }
    }

    let mut entries = accum
        .into_iter()
        .map(|((class, key), value)| ReportEntry {
            class,
            key,
            status: value.status,
            provenance: value.provenance.into_iter().collect(),
            save_affecting: value.save_affecting,
        })
        .collect::<Vec<_>>();
    entries.sort();

    let (_index, script_catalog) = ScriptCatalog::build(sources)?;

    Ok(CompatibilityReport {
        schema_version: CURRENT_REPORT_SCHEMA_VERSION,
        source_plugin: plugin_name.to_string(),
        source_fingerprint: fingerprint(bytes),
        entries,
        script_inventory: script_inventory(&script_catalog),
    })
}

fn script_inventory(catalog: &ScriptCatalog) -> ScriptInventoryReport {
    let mut totals = ScriptInventoryTotals::default();
    let mut by_kind = BTreeMap::new();
    let mut by_representation = BTreeMap::new();
    let mut scripts = catalog
        .scripts()
        .map(|script| {
            let id = script_id(script.record.id);
            match script.record.id {
                ScriptAssetId::Record(_) => totals.top_level += 1,
                ScriptAssetId::Embedded { .. } => totals.embedded += 1,
            }
            let kind = script_kind(script.record.kind);
            *by_kind.entry(kind.clone()).or_insert(0) += 1;
            let has_scda = script.record.compiled_data.is_some();
            let has_sctx = script.record.source_text.is_some();
            let representation = match (has_scda, has_sctx) {
                (true, true) => "scda_sctx",
                (true, false) => "scda_only",
                (false, true) => "sctx_only",
                (false, false) => "neither",
            };
            *by_representation.entry(representation.into()).or_insert(0) += 1;
            let compiled_bytes = script.record.compiled_data.as_ref().map_or(0, Vec::len);
            let variables = script
                .record
                .header
                .map_or(script.record.locals.len(), |header| {
                    header.variable_count as usize
                });
            let references = script
                .record
                .header
                .map_or(script.record.references.len(), |header| {
                    header.reference_count as usize
                });
            totals.compiled_bytes += compiled_bytes;
            totals.variables += variables;
            totals.references += references;
            ScriptInventoryEntry {
                id,
                kind,
                has_scda,
                has_sctx,
                compiled_bytes,
                variables,
                references,
                winning_plugin: script.winning_plugin.clone(),
                provenance: script.provenance.clone(),
            }
        })
        .collect::<Vec<_>>();
    scripts.sort();

    let mut attachment_owner_signatures = BTreeMap::new();
    let mut attachments = catalog
        .attachments()
        .map(|attachment| {
            *attachment_owner_signatures
                .entry(attachment.owner_signature.clone())
                .or_insert(0) += 1;
            ScriptAttachmentEntry {
                owner: attachment.owner.to_string(),
                owner_signature: attachment.owner_signature.clone(),
                slot: attachment_slot(attachment.slot),
                script: script_id(attachment.script),
                winning_plugin: attachment.winning_plugin.clone(),
                provenance: attachment.provenance.clone(),
            }
        })
        .collect::<Vec<_>>();
    attachments.sort();
    totals.attachments = attachments.len();

    let mut diagnostics = catalog
        .diagnostics()
        .iter()
        .map(|diagnostic| ScriptDiagnosticEntry {
            script: script_id(diagnostic.script),
            source_plugin: diagnostic.source_plugin.clone(),
            subrecord: diagnostic.subrecord.clone(),
            offset: diagnostic.offset,
            message: diagnostic.message.clone(),
        })
        .collect::<Vec<_>>();
    diagnostics.sort();
    totals.diagnostics = diagnostics.len();

    ScriptInventoryReport {
        content_fingerprint: catalog.content_fingerprint().into(),
        totals,
        by_kind,
        by_representation,
        attachment_owner_signatures,
        scripts,
        attachments,
        diagnostics,
    }
}

fn script_id(id: ScriptAssetId) -> String {
    match id {
        ScriptAssetId::Record(form_id) => format!("record:{form_id}"),
        ScriptAssetId::Embedded { owner, slot } => {
            format!("embedded:{owner}:{}", embedded_slot(slot))
        }
    }
}

fn script_kind(kind: Option<ScriptKind>) -> String {
    match kind {
        Some(ScriptKind::Object) => "object".into(),
        Some(ScriptKind::Quest) => "quest".into(),
        Some(ScriptKind::Effect) => "effect".into(),
        Some(ScriptKind::Unknown(value)) => format!("unknown_{value:04x}"),
        None => "missing".into(),
    }
}

fn attachment_slot(slot: ScriptAttachmentSlot) -> String {
    match slot {
        ScriptAttachmentSlot::Direct(index) => format!("direct:{index}"),
        ScriptAttachmentSlot::Embedded(slot) => embedded_slot(slot).into(),
    }
}

fn embedded_slot(slot: EmbeddedScriptSlot) -> &'static str {
    match slot {
        EmbeddedScriptSlot::Package(PackageScriptSlot::Begin) => "package_begin",
        EmbeddedScriptSlot::Package(PackageScriptSlot::Change) => "package_change",
        EmbeddedScriptSlot::Package(PackageScriptSlot::End) => "package_end",
    }
}

fn note(
    accum: &mut BTreeMap<(ReportClass, String), Accumulator>,
    class: ReportClass,
    key: String,
    declared: Option<(SupportStatus, bool)>,
    location: String,
) {
    let (status, save_affecting) = declared.unwrap_or((SupportStatus::Unknown, true));
    accum
        .entry((class, key))
        .or_insert_with(|| Accumulator {
            status,
            save_affecting,
            provenance: BTreeSet::new(),
        })
        .provenance
        .insert(location);
}

fn asset_extension(path: &str) -> Option<String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return None;
    }
    let dot = trimmed.rfind('.')?;
    Some(trimmed[dot..].to_ascii_lowercase())
}

fn fingerprint(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
