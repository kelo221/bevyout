use super::*;

#[test]
fn variable_names_choose_deterministic_lifetimes() {
    let mut state = NarrativeVariableState::default();
    state.set("$global_reputation", NarrativeValue::Number(3));
    state.set("$session_met", NarrativeValue::Bool(true));
    state.set("$temp_hint", NarrativeValue::Text("look".into()));

    assert_eq!(
        state.persistent["$global_reputation"],
        NarrativeValue::Number(3)
    );
    assert_eq!(state.session["$session_met"], NarrativeValue::Bool(true));
    assert_eq!(
        state.temporary["$temp_hint"],
        NarrativeValue::Text("look".into())
    );
    assert_eq!(
        state.get("$temp_hint"),
        Some(&NarrativeValue::Text("look".into()))
    );

    state.clear_session_boundary();
    assert!(state.session.is_empty());
    assert!(state.temporary.is_empty());
    assert_eq!(
        state.persistent["$global_reputation"],
        NarrativeValue::Number(3)
    );
}

#[test]
fn boundary_snapshot_never_contains_runner_checkpoint() {
    let snapshot = DialogueSnapshot::boundary(
        "hash",
        NarrativeVariableState {
            persistent: [("$global_done".into(), NarrativeValue::Bool(true))]
                .into_iter()
                .collect(),
            ..Default::default()
        },
    );
    assert_eq!(snapshot.schema_version, DIALOGUE_SNAPSHOT_SCHEMA_VERSION);
    assert!(snapshot.active.is_none());
}

#[test]
fn presentation_timing_prefers_voice_and_is_deterministic_without_it() {
    let policy = DialoguePresentationPolicy {
        language: "en-US".into(),
        subtitles_enabled: true,
        typewriter_enabled: true,
        skip_requires_second_press: true,
        accessible_choice_numbers: true,
    };
    assert_eq!(policy.reveal_duration_seconds("hello", Some(1250)), 1.25);
    assert_eq!(policy.reveal_duration_seconds("hello", None), 0.125);
    assert_eq!(policy.auto_advance_duration_seconds("hello", None), 0.5);
    assert_eq!(
        policy.auto_advance_duration_seconds("hello", Some(1250)),
        1.25
    );
}
