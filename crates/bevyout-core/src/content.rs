//! Parser-independent content identity and lookup contracts.

use serde::{Deserialize, Serialize};

use crate::form_id::FormId;

/// Owned snapshot of one winning content record.
///
/// The view owns its strings so callers never borrow parser arenas or mapped
/// plugin bytes. `winning_source` and `provenance` describe load-order
/// override provenance when the backing source can provide it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentRecordView {
    pub form_id: FormId,
    pub record_type: String,
    pub editor_id: Option<String>,
    pub winning_source: Option<String>,
    pub provenance: Vec<String>,
}

/// Thread-safe, object-safe lookup boundary for indexed game content.
///
/// Implementations must compare EditorIDs case-insensitively and return
/// EditorID matches in ascending [`FormId`] order. A missing FormID is `None`;
/// a missing EditorID is an empty vector, while multiple entries preserve
/// ambiguity for the caller to resolve. Returned views are owned snapshots,
/// so they remain valid independently of the resolver's lifetime.
pub trait ContentRecordResolver: Send + Sync {
    fn resolve_form_id(&self, form_id: FormId) -> Option<ContentRecordView>;

    fn resolve_editor_id(&self, editor_id: &str) -> Vec<ContentRecordView>;
}

#[cfg(test)]
#[path = "tests/content.rs"]
mod tests;
