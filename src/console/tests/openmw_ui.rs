use super::*;

#[test]
fn history_restores_unfinished_draft() {
    let mut history = CommandHistory::from_entries(["help".into(), "getpos z".into()]);
    assert_eq!(history.up("setpos "), "getpos z");
    assert_eq!(history.up("ignored"), "help");
    assert_eq!(history.down("ignored"), "getpos z");
    assert_eq!(history.down("ignored"), "setpos ");
}

#[test]
fn history_caps_and_deduplicates_consecutive_entries() {
    let mut history = CommandHistory::default();
    for index in 0..=HISTORY_LIMIT {
        history.record(format!("command {index}"));
    }
    history.record(format!("command {HISTORY_LIMIT}"));
    let entries = history.entries().collect::<Vec<_>>();
    assert_eq!(entries.len(), HISTORY_LIMIT);
    assert_eq!(entries[0], "command 1");
    assert_eq!(entries.last(), Some(&"command 200"));
}

#[test]
fn completion_uses_longest_prefix_then_lists_on_repeated_tab() {
    let candidates = vec!["getpos".into(), "getformid".into(), "getangle".into()];
    let mut completion = CompletionState::default();
    let first = completion.complete("get", candidates.clone());
    assert_eq!(first.text, "get");
    assert!(!first.list_candidates);
    let second = completion.complete("get", candidates);
    assert!(second.list_candidates);
    assert_eq!(second.matches.len(), 3);
}

#[test]
fn clear_submission_preserves_recorded_history() {
    let mut history = CommandHistory::from_entries(["help".into(), "getpos z".into()]);
    let mut transcript = ConsoleTranscript::default();
    transcript.push("help");
    transcript.push("commands listed");

    history.record("clear");
    if is_clear_submission(" clear ") {
        transcript.clear();
    }

    assert!(transcript.is_empty());
    assert_eq!(
        history.entries().collect::<Vec<_>>(),
        ["help", "getpos z", "clear"]
    );
}

#[test]
fn transcript_keeps_only_the_recent_bounded_lines() {
    let mut transcript = ConsoleTranscript::default();
    for index in 0..=TRANSCRIPT_LIMIT {
        transcript.push(format!("line {index}"));
    }

    assert_eq!(transcript.len(), TRANSCRIPT_LIMIT);
    assert_eq!(transcript.lines().next(), Some("line 1"));
    assert_eq!(transcript.lines().last(), Some("line 200"));
}
