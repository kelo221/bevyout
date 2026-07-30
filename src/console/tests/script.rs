use super::*;
use crate::console::ConsoleFrame;

fn temp_script(name: &str, contents: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "bevyout-console-{name}-{}.bscript",
        std::process::id()
    ));
    fs::write(&path, contents).unwrap();
    path
}

#[test]
fn expect_covers_pass_failure_and_tolerance_edges() {
    let mut harness = ConsoleHarness::synthetic();
    assert!(harness.exec("prid TestCrate").ok);
    assert!(harness.exec("expect (getpos x) == 1").ok);
    assert!(harness.exec("expect (getpos x) == 1.001 tol 0.0011").ok);
    let failed = harness.exec("expect (getpos x) == 2");
    assert_eq!(failed.error.unwrap().code, "assertion_failed");
    let mutating = harness.exec("expect (setpos x 4) == 4");
    assert_eq!(mutating.error.unwrap().code, "expect_mutating_command");
    assert_eq!(harness.exec("getpos x").value, json!(1.0));
}

#[test]
fn advance_runs_exactly_the_requested_fixed_frames() {
    let mut harness = ConsoleHarness::synthetic();
    let before = harness.world().resource::<ConsoleFrame>().0;
    assert!(harness.exec("advance 7").ok);
    let after = harness.world().resource::<ConsoleFrame>().0;
    assert_eq!(after - before, 7);
}

#[test]
fn transcripts_are_byte_deterministic_and_keep_going_is_observable() {
    let path = temp_script(
        "determinism",
        "prid TestCrate\ngetpos\nexpect (getpos x) == 9\ngetpos z\n",
    );
    let mut first = ConsoleHarness::synthetic();
    let stopped = first.run_script(&path, false).unwrap();
    assert_eq!(stopped.records.len(), 3);
    let mut second = ConsoleHarness::synthetic();
    let kept = second.run_script(&path, true).unwrap();
    assert_eq!(kept.records.len(), 4);
    let mut third = ConsoleHarness::synthetic();
    let kept_again = third.run_script(&path, true).unwrap();
    assert_eq!(kept.to_jsonl().unwrap(), kept_again.to_jsonl().unwrap());
    let _ = fs::remove_file(path);
}

#[test]
fn committed_golden_matches_or_regenerates_explicitly() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let script = root.join("tests/console_scripts/basic.bscript");
    let golden = root.join("tests/goldens/basic.jsonl");
    let mut harness = ConsoleHarness::synthetic();
    let actual = harness
        .run_script(&script, false)
        .unwrap()
        .to_jsonl()
        .unwrap();
    if std::env::var_os("UPDATE_GOLDENS").is_some() {
        fs::write(&golden, actual).unwrap();
    } else {
        // Git may materialize tracked text files with CRLF on Windows even
        // though the runner emits deterministic LF JSONL. Compare the
        // logical transcript bytes so the golden remains portable across
        // checkout line-ending policies.
        let expected = fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
            .into_bytes();
        assert_eq!(actual, expected);
    }
}
