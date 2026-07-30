//! `ContentIndex` collector over the shared resolved ESM4 record stream.
//!
//! Load-order validation, GRUP traversal, decompression, and FormID remapping
//! live in `vsa::record_stream`; this module retains only index-specific
//! metadata, diagnostics, and summaries.

use std::collections::{BTreeMap, HashMap};

use anyhow::Result;

use crate::vsa::openmw_esm4::{cstring, parse_subrecords, sub};
use crate::vsa::record_stream::{
    RecordEnvelope, RecordPayload, content_fingerprint, walk_resolved_records,
    winners::WinningRecords,
};

use super::{ContentIndex, IndexedRecord, PluginSource};

pub(super) fn build(sources: &[PluginSource<'_>]) -> Result<ContentIndex> {
    build_with(sources, |_| {})
}

pub(super) fn build_with(
    sources: &[PluginSource<'_>],
    mut on_record: impl FnMut(RecordEnvelope<'_>),
) -> Result<ContentIndex> {
    let mut records = WinningRecords::default();
    let mut type_counts: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut diagnostics = Vec::new();

    walk_resolved_records(sources, |event| {
        on_record(event);
        let record_type = event.signature.to_string();
        *type_counts
            .entry((event.source_plugin.to_string(), record_type.clone()))
            .or_insert(0) += 1;
        if !is_well_formed_signature(&record_type) {
            diagnostics.push(format!(
                "unsupported record signature '{}' at FormID {} in {}",
                record_type, event.form_id, event.source_plugin
            ));
        }
        if event.deleted {
            records.delete(event.form_id.0);
            return;
        }
        let editor_id = match event.payload {
            RecordPayload::Decoded(payload) => match parse_subrecords(payload) {
                Ok(subrecords) => sub(&subrecords, "EDID").map(cstring),
                Err(error) => {
                    diagnostics.push(format!(
                        "parsing {} record {} payload in {}: {error}",
                        record_type, event.form_id, event.source_plugin
                    ));
                    None
                }
            },
            RecordPayload::Unavailable(error) => {
                diagnostics.push(error.to_string());
                None
            }
        };
        records.upsert(
            event.form_id.0,
            event.source_plugin.to_string(),
            IndexedRecord {
                form_id: event.form_id,
                record_type,
                editor_id,
                winning_plugin: event.source_plugin.to_string(),
                provenance: Vec::new(),
            },
        );
    })?;

    let records = records
        .into_iter()
        .map(|(form_id, winner)| {
            let mut record = winner.value;
            record.provenance = winner.provenance;
            (form_id, record)
        })
        .collect::<HashMap<_, _>>();

    Ok(ContentIndex {
        records,
        load_order: sources
            .iter()
            .map(|source| source.name.to_string())
            .collect(),
        type_counts,
        diagnostics,
        fingerprint: content_fingerprint(sources),
    })
}

fn is_well_formed_signature(signature: &str) -> bool {
    signature.len() == 4
        && signature
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
}
