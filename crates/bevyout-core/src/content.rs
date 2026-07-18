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
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[derive(Default)]
    struct SyntheticResolver(BTreeMap<FormId, ContentRecordView>);

    impl ContentRecordResolver for SyntheticResolver {
        fn resolve_form_id(&self, form_id: FormId) -> Option<ContentRecordView> {
            self.0.get(&form_id).cloned()
        }

        fn resolve_editor_id(&self, editor_id: &str) -> Vec<ContentRecordView> {
            self.0
                .values()
                .filter(|record| {
                    record
                        .editor_id
                        .as_deref()
                        .is_some_and(|candidate| candidate.eq_ignore_ascii_case(editor_id))
                })
                .cloned()
                .collect()
        }
    }

    fn record(form_id: u32, editor_id: &str) -> ContentRecordView {
        ContentRecordView {
            form_id: FormId(form_id),
            record_type: "CELL".into(),
            editor_id: Some(editor_id.into()),
            winning_source: Some("synthetic.esp".into()),
            provenance: vec!["synthetic.esp".into()],
        }
    }

    #[test]
    fn trait_object_resolves_form_ids_and_case_insensitive_editor_ids() {
        let resolver = SyntheticResolver(BTreeMap::from([
            (FormId(2), record(2, "SharedCell")),
            (FormId(1), record(1, "SharedCell")),
        ]));
        let resolver: &dyn ContentRecordResolver = &resolver;

        assert_eq!(
            resolver.resolve_form_id(FormId(2)).unwrap().editor_id,
            Some("SharedCell".into())
        );
        assert_eq!(
            resolver
                .resolve_editor_id("sharedcell")
                .into_iter()
                .map(|record| record.form_id)
                .collect::<Vec<_>>(),
            [FormId(1), FormId(2)]
        );
        assert!(resolver.resolve_form_id(FormId(99)).is_none());
        assert!(resolver.resolve_editor_id("missing").is_empty());
    }
}
