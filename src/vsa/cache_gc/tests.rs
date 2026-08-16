use super::{lock::CacheGcLock, scan};
use bevyout_core::manifest::exterior::EXTERIOR_CELL_PACKAGE_REVISION;
use std::{fs, path::PathBuf};

fn temporary_cache(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "bevyout-cache-gc-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn write(path: &std::path::Path, bytes: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

#[test]
fn planner_preserves_reachable_current_and_quarantine_files() {
    let cache = temporary_cache("retention");
    write(
        &cache.join("scenes/00000001/scene.ron"),
        b"(asset: \"catalogs/live.ron\")",
    );
    write(
        &cache.join("catalogs/live.ron"),
        b"(asset: \"objects/catalog/aa/bb/live.ron\")",
    );
    write(
        &cache.join("objects/catalog/aa/bb/live.ron"),
        b"(live:true)",
    );
    write(
        &cache.join("objects/catalog/cc/dd/dead.ron"),
        b"(dead:true)",
    );
    write(&cache.join("assets/terrain/legacy.png"), b"legacy");
    write(&cache.join("staging/old.tmp"), b"temporary");
    write(&cache.join("quarantine/corrupt/object.bin"), b"quarantined");
    write(
        &cache.join("worldspaces/0000003c/cells/00000001.ron"),
        format!("(revision:\"{EXTERIOR_CELL_PACKAGE_REVISION}\")").as_bytes(),
    );
    write(
        &cache.join("worldspaces/0000003c/cells/00000002.ron"),
        b"(revision:\"exterior-cell-package-v1\")",
    );

    let report = scan::plan_gc(&cache, true, 0, false).unwrap();
    let candidates = scan::candidate_paths(&report);
    assert!(!candidates.contains("objects/catalog/aa/bb/live.ron"));
    assert!(!candidates.contains("quarantine/corrupt/object.bin"));
    assert!(!candidates.contains("worldspaces/0000003c/cells/00000001.ron"));
    assert!(candidates.contains("objects/catalog/cc/dd/dead.ron"));
    assert!(candidates.contains("assets/terrain/legacy.png"));
    assert!(candidates.contains("staging/old.tmp"));
    assert!(candidates.contains("worldspaces/0000003c/cells/00000002.ron"));

    fs::remove_dir_all(cache).unwrap();
}

#[test]
fn dry_run_is_non_mutating_and_sweep_revalidates_then_deletes() {
    let cache = temporary_cache("sweep");
    let dead = cache.join("objects/catalog/cc/dd/dead.ron");
    write(&dead, b"(dead:true)");

    let mut dry_run = scan::plan_gc(&cache, true, 0, false).unwrap();
    scan::sweep(&mut dry_run).unwrap();
    assert!(dead.is_file());
    assert_eq!(dry_run.deleted_file_count, 0);

    let mut apply = scan::plan_gc(&cache, false, 0, false).unwrap();
    scan::sweep(&mut apply).unwrap();
    assert!(!dead.exists());
    assert_eq!(apply.deleted_file_count, 1);

    fs::remove_dir_all(cache).unwrap();
}

#[test]
fn rebuildable_assets_require_opt_in() {
    let cache = temporary_cache("rebuildable");
    write(&cache.join("assets/nif/unreferenced.glb"), b"rebuildable");

    let retained = scan::plan_gc(&cache, true, 0, false).unwrap();
    assert!(retained.candidates.is_empty());
    let selected = scan::plan_gc(&cache, true, 0, true).unwrap();
    assert_eq!(
        scan::candidate_paths(&selected),
        std::collections::BTreeSet::from(["assets/nif/unreferenced.glb"])
    );

    fs::remove_dir_all(cache).unwrap();
}

#[test]
fn cache_gc_lock_is_exclusive_and_removed_on_drop() {
    let cache = temporary_cache("lock");
    fs::create_dir_all(&cache).unwrap();
    let first = CacheGcLock::acquire(&cache).unwrap();
    let error = CacheGcLock::acquire(&cache).err().unwrap().to_string();
    assert!(error.contains("lock is held by another process"));
    drop(first);
    let second = CacheGcLock::acquire(&cache).unwrap();
    drop(second);

    fs::remove_dir_all(cache).unwrap();
}
