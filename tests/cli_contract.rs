use std::process::Command;

fn run_cli(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_bevyout"))
        .args(arguments)
        .output()
        .expect("bevyout binary should start")
}

#[test]
fn help_lists_the_four_supported_commands() {
    let output = run_cli(&["--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for command in ["prepare", "bake", "render", "view"] {
        assert!(stdout.contains(command), "help should mention {command}");
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
