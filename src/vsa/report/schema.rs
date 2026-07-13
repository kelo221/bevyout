//! Machine and human-readable compatibility report schema.
//!
//! The schema is intentionally small and fully sorted so two reports
//! generated from the same input are byte-identical (F37.3): no timestamps,
//! no absolute paths, no hash-map iteration order.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// Current report schema version. Bump when the JSON shape changes.
pub(crate) const CURRENT_REPORT_SCHEMA_VERSION: u32 = 1;

/// The kind of thing a [`ReportEntry`] describes.
///
/// Only `Record` and `Subrecord` are currently populated from the reader;
/// `Condition`, `ScriptFunction`, `AssetFormat`, and `Quest` are surfaced
/// where the raw byte stream makes them cheaply observable (CTDA condition
/// blocks, SCPT script data, MODL/MOD2 asset extensions, QUST records).
/// Full condition/script/quest interpretation is out of scope for this
/// issue (see M1_PLAN.md, Issue #37 skip note).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReportClass {
    Record,
    Subrecord,
    Condition,
    ScriptFunction,
    AssetFormat,
    Quest,
}

impl ReportClass {
    pub(crate) const ALL: [Self; 6] = [
        Self::Record,
        Self::Subrecord,
        Self::Condition,
        Self::ScriptFunction,
        Self::AssetFormat,
        Self::Quest,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Record => "record",
            Self::Subrecord => "subrecord",
            Self::Condition => "condition",
            Self::ScriptFunction => "script-function",
            Self::AssetFormat => "asset-format",
            Self::Quest => "quest",
        }
    }
}

/// Support status of one entry. `Unknown` is the default for anything
/// encountered but not declared in the support registry (F37.4); it is
/// never assigned `Supported` implicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SupportStatus {
    Supported,
    Partial,
    Unsupported,
    IgnoredByDesign,
    Unknown,
}

impl SupportStatus {
    pub(crate) const ALL: [Self; 5] = [
        Self::Supported,
        Self::Partial,
        Self::Unsupported,
        Self::IgnoredByDesign,
        Self::Unknown,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Partial => "partial",
            Self::Unsupported => "unsupported",
            Self::IgnoredByDesign => "ignored_by_design",
            Self::Unknown => "unknown",
        }
    }
}

/// One compatibility fact: "this class/key combination has this status,
/// observed at these locations."
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) struct ReportEntry {
    pub(crate) class: ReportClass,
    pub(crate) key: String,
    pub(crate) status: SupportStatus,
    /// Sorted, deduplicated `"PluginName:FormID"` (or bare plugin name for
    /// plugin-wide facts) strings identifying every observed occurrence.
    pub(crate) provenance: Vec<String>,
    /// Conservative save-compatibility risk flag (F37.7): true unless the
    /// support registry explicitly declares the entry safe.
    pub(crate) save_affecting: bool,
}

/// A full compatibility report for one plugin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CompatibilityReport {
    pub(crate) schema_version: u32,
    pub(crate) source_plugin: String,
    pub(crate) source_fingerprint: String,
    /// Fully sorted by (class, key); deterministic across runs (F37.3).
    pub(crate) entries: Vec<ReportEntry>,
}

impl CompatibilityReport {
    /// Canonical, deterministic JSON rendering (pretty-printed, trailing
    /// newline). Two calls on an equal report are byte-identical.
    pub(crate) fn to_json(&self) -> String {
        let mut json = serde_json::to_string_pretty(self).expect("report schema is serializable");
        json.push('\n');
        json
    }

    /// Counts of entries per (class, status), used by [`Self::summary`] and
    /// by tests asserting `Unknown` is never folded into `Supported`
    /// totals (T37.4).
    pub(crate) fn counts(&self) -> BTreeMap<(ReportClass, SupportStatus), usize> {
        let mut counts = BTreeMap::new();
        for entry in &self.entries {
            *counts.entry((entry.class, entry.status)).or_insert(0) += 1;
        }
        counts
    }

    /// Plain-text, sorted-by-class-then-status summary suitable for
    /// terminal output and diffing (F37.2).
    pub(crate) fn summary(&self) -> String {
        let counts = self.counts();
        let mut out = String::new();
        let _ = writeln!(out, "Compatibility report for {}", self.source_plugin);
        let _ = writeln!(out, "fingerprint: {}", self.source_fingerprint);
        for class in ReportClass::ALL {
            let class_total: usize = SupportStatus::ALL
                .iter()
                .map(|status| counts.get(&(class, *status)).copied().unwrap_or(0))
                .sum();
            if class_total == 0 {
                continue;
            }
            let _ = write!(out, "  {}:", class.label());
            for status in SupportStatus::ALL {
                let count = counts.get(&(class, status)).copied().unwrap_or(0);
                if count > 0 {
                    let _ = write!(out, " {}={count}", status.label());
                }
            }
            let _ = writeln!(out);
        }
        let save_affecting_unresolved = self
            .entries
            .iter()
            .filter(|entry| {
                entry.save_affecting
                    && matches!(
                        entry.status,
                        SupportStatus::Unsupported | SupportStatus::Unknown
                    )
            })
            .count();
        let _ = writeln!(
            out,
            "save-affecting unresolved entries: {save_affecting_unresolved}"
        );
        out
    }
}
