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

#[test]
fn cache_stats_writes_deterministic_inventory_reports() {
    let root = std::env::temp_dir().join(format!(
        "bevyout-cache-command-smoke-{}",
        std::process::id()
    ));
    let cache = root.join("cache");
    std::fs::create_dir_all(cache.join("assets")).unwrap();
    std::fs::write(cache.join("assets/a.glb"), b"duplicate").unwrap();
    std::fs::write(cache.join("assets/b.glb"), b"duplicate").unwrap();
    std::fs::write(cache.join("cellmap.ron"), b"unique-index").unwrap();
    let json = root.join("cache.json");
    let csv = root.join("cache.csv");

    let output = Command::new(env!("CARGO_BIN_EXE_bevyout"))
        .args([
            "cache",
            "stats",
            "--cache",
            cache.to_str().unwrap(),
            "--json",
            json.to_str().unwrap(),
            "--csv",
            csv.to_str().unwrap(),
        ])
        .output()
        .expect("bevyout cache stats should start");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cache stats: files 3"));
    assert!(stdout.contains("duplicate logical 9"));
    let report: serde_json::Value = serde_json::from_slice(&std::fs::read(&json).unwrap()).unwrap();
    assert_eq!(report["schema_version"], "cache-stats-v1");
    assert_eq!(report["storage"]["duplicate_logical_bytes"], 9);
    assert!(
        std::fs::read_to_string(csv)
            .unwrap()
            .contains("assets/a.glb")
    );

    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn cache_gc_dry_run_reports_without_deleting() {
    let root = std::env::temp_dir().join(format!(
        "bevyout-cache-gc-command-smoke-{}",
        std::process::id()
    ));
    let cache = root.join("cache");
    let dead = cache.join("objects/catalog/aa/bb/dead.ron");
    std::fs::create_dir_all(dead.parent().unwrap()).unwrap();
    std::fs::write(&dead, b"(dead:true)").unwrap();
    let json = root.join("gc.json");

    let output = Command::new(env!("CARGO_BIN_EXE_bevyout"))
        .args([
            "cache",
            "gc",
            "--cache",
            cache.to_str().unwrap(),
            "--dry-run",
            "--grace-hours",
            "0",
            "--json",
            json.to_str().unwrap(),
        ])
        .output()
        .expect("bevyout cache gc should start");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(dead.is_file());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cache gc: mode dry-run"));
    let report: serde_json::Value = serde_json::from_slice(&std::fs::read(&json).unwrap()).unwrap();
    assert_eq!(report["schema_version"], "cache-gc-v1");
    assert_eq!(report["candidate_file_count"], 1);
    assert_eq!(report["deleted_file_count"], 0);

    std::fs::remove_dir_all(root).unwrap();
}
