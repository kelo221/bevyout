//! Walks a plugin's record/subrecord inventory into a [`CompatibilityReport`].

use anyhow::Result;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

use super::registry::{asset_format_status, record_status, subrecord_status};
use super::schema::{
    CURRENT_REPORT_SCHEMA_VERSION, CompatibilityReport, ReportClass, ReportEntry, SupportStatus,
};
use crate::vsa::openmw_esm4::{cstring, inventory_records};

struct Accumulator {
    status: SupportStatus,
    save_affecting: bool,
    provenance: BTreeSet<String>,
}

/// Builds a deterministic compatibility report for one plugin's bytes.
///
/// `plugin_name` should be a bare file name (no directory component) so the
/// report is portable and stays byte-identical across machines (F37.3).
pub(crate) fn generate_report(plugin_name: &str, bytes: &[u8]) -> Result<CompatibilityReport> {
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

    Ok(CompatibilityReport {
        schema_version: CURRENT_REPORT_SCHEMA_VERSION,
        source_plugin: plugin_name.to_string(),
        source_fingerprint: fingerprint(bytes),
        entries,
    })
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
