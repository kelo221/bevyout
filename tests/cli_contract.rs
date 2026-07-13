use std::process::Command;

fn run_cli(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_bevyout"))
        .args(arguments)
        .output()
        .expect("bevyout binary should start")
}

#[test]
fn help_lists_the_supported_commands() {
    let output = run_cli(&["--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for command in [
        "prepare", "bake", "render", "view", "report", "cells", "script",
    ] {
        assert!(stdout.contains(command), "help should mention {command}");
    }
}

#[test]
fn script_runner_reports_success_failure_and_writes_transcripts() {
    let directory = std::env::temp_dir().join(format!("bevyout-script-cli-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let success = directory.join("success.bscript");
    let failure = directory.join("failure.bscript");
    let transcript = directory.join("failure.jsonl");
    std::fs::write(&success, "prid TestCrate\nexpect (getpos x) == 1\n").unwrap();
    std::fs::write(
        &failure,
        "prid TestCrate\nexpect (getpos x) == 9\ngetpos z\n",
    )
    .unwrap();

    let success_output = run_cli(&["script", "run", success.to_str().unwrap(), "--headless"]);
    assert!(success_output.status.success());

    let failure_output = run_cli(&[
        "script",
        "run",
        failure.to_str().unwrap(),
        "--keep-going",
        "--transcript",
        transcript.to_str().unwrap(),
    ]);
    assert!(!failure_output.status.success());
    let lines = std::fs::read_to_string(&transcript).unwrap();
    assert_eq!(lines.lines().count(), 3);
    assert!(lines.contains("assertion_failed"));
    let _ = std::fs::remove_dir_all(directory);
}

#[test]
fn report_subcommand_parses() {
    let output = run_cli(&["report", "--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for flag in ["--game-root", "--plugin", "--out-dir"] {
        assert!(stdout.contains(flag), "report --help should mention {flag}");
    }
}

#[test]
fn malformed_bake_arguments_are_rejected_by_clap() {
    let output = run_cli(&[
        "bake",
        "--manifest",
        "scene.ron",
        "--static-batch-chunk-meters",
        "7.99",
    ]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("batch chunk size"));
}

#[test]
fn agent_port_requires_the_opt_in_bridge() {
    let output = run_cli(&["render", "SmokeCell", "--agent-port", "15703"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("agent-bridge"), "stderr was: {stderr}");
}

#[test]
fn viewer_help_describes_the_agent_bridge() {
    let output = run_cli(&["view", "--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("agent-bridge"));
    assert!(stdout.contains("agent-port"));
}
