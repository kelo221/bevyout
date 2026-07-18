//! Load-order-wide `ContentIndex`: a single-pass index over a resolved
//! plugin load order, keyed by resolved FormID.
//!
//! This reuses the attributed ESM4 byte-decoding primitives already tested
//! in `vsa::openmw_esm4` (subrecord/record/GRUP layout parsing, `EDID`
//! extraction and master-name reading) rather than writing a new parser.
//! FormID identity/remapping comes from `bevyout-core`, shared with the ESM
//! reader.
//!
//! Only `ContentIndex`, `FormId`, and `IndexedRecord` are exposed. No
//! `openmw_esm4` parser type leaves this module.
//!
//! The index implements the core `ContentRecordResolver` boundary. The live
//! catalogue path uses the same trait through an adapter over its already
//! parsed content set, avoiding a second full plugin parse during prepare.
//! Remaining index reports are still future CLI surface, so their unused
//! accessors stay explicitly allowed here.
#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use bevyout_core::content::{ContentRecordResolver, ContentRecordView};
pub(crate) use bevyout_core::form_id::FormId;

pub(crate) use super::plugin::PluginSource;

mod builder;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

/// One record's resolved state after winning-override resolution, plus the
/// full provenance chain of plugins that contributed a version of it.
#[derive(Debug, Clone)]
pub(crate) struct IndexedRecord {
    pub(crate) form_id: FormId,
    pub(crate) record_type: String,
    pub(crate) editor_id: Option<String>,
    pub(crate) winning_plugin: String,
    pub(crate) provenance: Vec<String>,
}

/// A load-order-wide index over every record contributed by a configured
/// plugin list, keyed by resolved FormID.
#[derive(Debug, Default)]
pub(crate) struct ContentIndex {
    records: HashMap<u32, IndexedRecord>,
    load_order: Vec<String>,
    type_counts: BTreeMap<(String, String), usize>,
    diagnostics: Vec<String>,
    fingerprint: String,
}

impl ContentIndex {
    /// Build one index over `sources` in the given (configured) load order.
    ///
    /// Validates master-before-dependent ordering, missing masters, and
    /// duplicate plugin entries before parsing anything (F39.1/F39.6); see
    /// `builder::validate_load_order`.
    pub(crate) fn build(sources: &[PluginSource<'_>]) -> Result<Self> {
        builder::build(sources)
    }

    /// Look up a record by its resolved FormID.
    pub(crate) fn get(&self, form_id: FormId) -> Option<&IndexedRecord> {
        self.records.get(&form_id.0)
    }

    /// All winning records of a given ESM4 record-type signature (e.g. `"CELL"`).
    pub(crate) fn records_of_type<'a>(
        &'a self,
        record_type: &'a str,
    ) -> impl Iterator<Item = &'a IndexedRecord> + 'a {
        self.records
            .values()
            .filter(move |record| record.record_type == record_type)
    }

    /// Convenience wrapper over `records_of_type("CELL")`.
    pub(crate) fn cells(&self) -> impl Iterator<Item = &IndexedRecord> + '_ {
        self.records_of_type("CELL")
    }

    /// All winning records whose EditorID matches, case-insensitively.
    pub(crate) fn by_editor_id(&self, editor_id: &str) -> Vec<&IndexedRecord> {
        let mut records = self
            .records
            .values()
            .filter(|record| {
                record
                    .editor_id
                    .as_deref()
                    .is_some_and(|value| value.eq_ignore_ascii_case(editor_id))
            })
            .collect::<Vec<_>>();
        records.sort_by_key(|record| record.form_id);
        records
    }

    /// The configured plugin names, in resolved load order.
    pub(crate) fn load_order(&self) -> &[String] {
        &self.load_order
    }

    /// Stable hash over plugin names + sizes + content, exposed for later
    /// save-compatibility checks (F39.7).
    pub(crate) fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    /// Non-fatal diagnostics collected while indexing (e.g. malformed
    /// record signatures); missing-master/order/duplicate problems are
    /// returned as `build()` errors instead, not diagnostics.
    pub(crate) fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }

    pub(crate) fn len(&self) -> usize {
        self.records.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Deterministic, sorted `plugin\trecord-type\tcount` lines suitable for
    /// byte-identical regression comparison across runs (F39.8).
    pub(crate) fn summary(&self) -> String {
        self.type_counts
            .iter()
            .map(|((plugin, record_type), count)| format!("{plugin}\t{record_type}\t{count}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl ContentRecordResolver for ContentIndex {
    fn resolve_form_id(&self, form_id: FormId) -> Option<ContentRecordView> {
        self.get(form_id).map(record_view)
    }

    fn resolve_editor_id(&self, editor_id: &str) -> Vec<ContentRecordView> {
        self.by_editor_id(editor_id)
            .into_iter()
            .map(record_view)
            .collect()
    }
}

fn record_view(record: &IndexedRecord) -> ContentRecordView {
    ContentRecordView {
        form_id: record.form_id,
        record_type: record.record_type.clone(),
        editor_id: record.editor_id.clone(),
        winning_source: Some(record.winning_plugin.clone()),
        provenance: record.provenance.clone(),
    }
}
