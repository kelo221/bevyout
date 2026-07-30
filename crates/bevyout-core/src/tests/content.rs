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
