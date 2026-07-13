//! Load-order-wide `ContentIndex`: a single-pass index over a resolved
//! plugin load order, keyed by resolved FormID.
//!
//! This reuses the attributed ESM4 byte-decoding primitives already tested
//! in `vsa::openmw_esm4` (subrecord/record/GRUP layout parsing, `EDID`
//! extraction, master-name reading) rather than writing a new parser. The
//! FormID master-index remap arithmetic is duplicated in `builder` in
//! miniature because `openmw_esm4::FormIdResolver`'s fields are private to
//! its own module; see the comment there.
//!
//! Only `ContentIndex`, `FormId`, and `IndexedRecord` are exposed. No
//! `openmw_esm4` parser type leaves this module.
//!
//! Integrating this index into the `prepare` cell-selection path is a
//! follow-up once issue #38 lands (see `M1_PLAN.md`); nothing outside this
//! module's own tests calls it yet, so `dead_code` is relaxed here rather
//! than wired into the CLI prematurely.
#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};
use std::fmt;

use anyhow::Result;

pub(crate) use super::plugin::PluginSource;

mod builder;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

/// A resolved FormID: the load-order-wide, master-remapped 32-bit
/// identifier shared by every plugin that references the same record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct FormId(pub(crate) u32);

impl FormId {
    pub(crate) fn value(self) -> u32 {
        self.0
    }
}

impl From<u32> for FormId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl fmt::Display for FormId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:08x}", self.0)
    }
}

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
        self.records
            .values()
            .filter(|record| {
                record
                    .editor_id
                    .as_deref()
                    .is_some_and(|value| value.eq_ignore_ascii_case(editor_id))
            })
            .collect()
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
