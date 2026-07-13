use std::process::Command;

#[test]
fn noninteractive_render_reports_a_missing_cached_scene() {
    let cache_dir =
        std::env::temp_dir().join(format!("bevyout-command-smoke-{}", std::process::id()));
    let output = Command::new(env!("CARGO_BIN_EXE_bevyout"))
        .args([
            "render",
            "MissingSmokeCell",
            "--cache-dir",
            cache_dir.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("bevyout binary should start");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("prepared scene for GECK EditorID"));
}
