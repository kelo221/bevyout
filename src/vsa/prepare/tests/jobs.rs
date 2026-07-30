use super::*;

fn temp_dir(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "bevyout-jobs-test-{label}-{}-{}",
        std::process::id(),
        line!()
    ))
}

// T48.1: pending -> done and pending -> failed(reason) persist to RON
// and reload byte-identically.
#[test]
fn status_transitions_persist_across_a_reload() {
    let dir = temp_dir("status-transitions");
    fs::create_dir_all(&dir).unwrap();
    let path = manifest_path(&dir);

    let mut manifest = JobManifest::new("fp-1");
    manifest.ensure_pending(&[1, 2, 3]);
    manifest.set_status(1, JobStatus::Done);
    manifest.set_status(2, JobStatus::Failed("missing model foo.nif".into()));
    manifest.write_atomic(&path).unwrap();

    let reloaded = JobManifest::load_or_new(&path, "fp-1").unwrap();
    assert_eq!(reloaded, manifest);
    assert_eq!(reloaded.status(1), Some(&JobStatus::Done));
    assert_eq!(
        reloaded.status(2),
        Some(&JobStatus::Failed("missing model foo.nif".into()))
    );
    assert_eq!(reloaded.status(3), Some(&JobStatus::Pending));

    // Byte-identical: writing the reloaded manifest back out reproduces
    // exactly the RON this test already wrote.
    assert_eq!(reloaded.to_ron().unwrap(), manifest.to_ron().unwrap());

    fs::remove_dir_all(&dir).ok();
}

// T48.2: resume filtering skips `done`, keeps `pending` and `failed`;
// `--retry-failed` selection (via `failed_form_ids`) picks exactly the
// failed set.
#[test]
fn resume_filter_skips_done_keeps_pending_and_failed() {
    let mut manifest = JobManifest::new("fp-1");
    manifest.set_status(1, JobStatus::Done);
    manifest.set_status(2, JobStatus::Pending);
    manifest.set_status(3, JobStatus::Failed("boom".into()));
    // FormID 4 was never recorded -- newly selected, treated as pending.

    let (to_run, skipped) = filter_resume(&manifest, &[1, 2, 3, 4], false);
    assert_eq!(to_run, vec![2, 3, 4]);
    assert_eq!(skipped, 1);
}

#[test]
fn force_reruns_everything_regardless_of_status() {
    let mut manifest = JobManifest::new("fp-1");
    manifest.set_status(1, JobStatus::Done);
    manifest.set_status(2, JobStatus::Failed("boom".into()));

    let (to_run, skipped) = filter_resume(&manifest, &[1, 2], true);
    assert_eq!(to_run, vec![1, 2]);
    assert_eq!(skipped, 0);
}

#[test]
fn retry_failed_selection_is_exactly_the_failed_set() {
    let mut manifest = JobManifest::new("fp-1");
    manifest.set_status(1, JobStatus::Done);
    manifest.set_status(2, JobStatus::Pending);
    manifest.set_status(3, JobStatus::Failed("boom".into()));
    manifest.set_status(4, JobStatus::Failed("bang".into()));

    assert_eq!(manifest.failed_form_ids(), vec![3, 4]);
}

// T48.3: a manifest on disk with a different content fingerprint is
// discarded -- reloading under a new fingerprint starts empty, so every
// cell is pending again once the caller re-selects it.
#[test]
fn manifest_with_different_fingerprint_is_discarded() {
    let dir = temp_dir("fingerprint-mismatch");
    fs::create_dir_all(&dir).unwrap();
    let path = manifest_path(&dir);

    let mut manifest = JobManifest::new("fp-old");
    manifest.set_status(1, JobStatus::Done);
    manifest.set_status(2, JobStatus::Failed("boom".into()));
    manifest.write_atomic(&path).unwrap();

    let reloaded = JobManifest::load_or_new(&path, "fp-new").unwrap();
    assert_eq!(reloaded.content_fingerprint, "fp-new");
    assert!(reloaded.jobs.is_empty());
    assert_eq!(reloaded.status(1), None);

    fs::remove_dir_all(&dir).ok();
}

fn synthetic_fingerprints(tag: &str) -> CellFingerprints {
    CellFingerprints {
        plugin_content_set: format!("plugin-{tag}"),
        converter: format!("converter-{tag}"),
        physics: format!("physics-{tag}"),
        prepare_pipeline: format!("prepare-{tag}"),
    }
}

// T49.1: a completed cell's recorded fingerprints round-trip through an
// atomic write and reload exactly as written (all four components).
#[test]
fn completed_cell_fingerprints_round_trip_across_a_reload() {
    let dir = temp_dir("fingerprints-round-trip");
    fs::create_dir_all(&dir).unwrap();
    let path = manifest_path(&dir);

    let mut manifest = JobManifest::new("fp-1");
    manifest.set_status(1, JobStatus::Done);
    manifest.record_fingerprints(1, synthetic_fingerprints("a"));
    manifest.write_atomic(&path).unwrap();

    let reloaded = JobManifest::load_or_new(&path, "fp-1").unwrap();
    assert_eq!(
        reloaded.fingerprints_for(1),
        Some(&synthetic_fingerprints("a"))
    );

    fs::remove_dir_all(&dir).ok();
}

// T49.3: a manifest mixing a valid Done cell, a stale Done cell, and a
// never-completed cell re-queues exactly the stale/incomplete subset;
// the skipped and stale counts match the actual split.
#[test]
fn mixed_manifest_requeues_exactly_the_stale_subset() {
    let mut manifest = JobManifest::new("fp-1");
    manifest.set_status(1, JobStatus::Done);
    manifest.record_fingerprints(1, synthetic_fingerprints("current"));
    manifest.set_status(2, JobStatus::Done);
    manifest.record_fingerprints(2, synthetic_fingerprints("stale"));
    manifest.set_status(3, JobStatus::Pending);

    let current = synthetic_fingerprints("current");
    let (to_run, skipped, stale) = filter_resume_checked(&manifest, &[1, 2, 3], false, &current);

    assert_eq!(to_run, vec![2, 3]);
    assert_eq!(skipped, 1);
    assert_eq!(stale.len(), 1);
    assert_eq!(stale[0].0, 2);
}

// T49.4: a manifest written before issue #49 (no `fingerprints` field
// at all) parses without error -- `#[serde(default)]` fills an empty
// map -- and its `Done` cells count as stale in every component rather
// than skipping or erroring.
#[test]
fn legacy_manifest_without_fingerprints_field_parses_and_counts_as_stale() {
    let legacy_ron = r#"(
            content_fingerprint: "fp-1",
            jobs: {
                1: Done,
                2: Pending,
            },
        )"#;
    let manifest: JobManifest =
        ron::de::from_str(legacy_ron).expect("legacy manifest without fingerprints must parse");
    assert!(manifest.fingerprints.is_empty());

    let current = synthetic_fingerprints("current");
    let (to_run, skipped, stale) = filter_resume_checked(&manifest, &[1, 2], false, &current);
    assert_eq!(to_run, vec![1, 2]);
    assert_eq!(skipped, 0);
    assert_eq!(stale.len(), 1);
    assert_eq!(stale[0].0, 1);
    assert_eq!(stale[0].1.len(), 4, "legacy entry stale in every component");
}

#[test]
fn missing_manifest_file_starts_fresh() {
    let dir = temp_dir("missing-file");
    let path = manifest_path(&dir);
    let manifest = JobManifest::load_or_new(&path, "fp-1").unwrap();
    assert_eq!(manifest, JobManifest::new("fp-1"));
}

// Atomic write: the temp-then-rename helper leaves exactly the new
// manifest at `path` and no `.tmp` residue in the directory, whether or
// not a previous manifest existed there.
#[test]
fn atomic_write_leaves_no_tmp_residue() {
    let dir = temp_dir("atomic-write");
    fs::create_dir_all(&dir).unwrap();
    let path = manifest_path(&dir);

    let first = JobManifest::new("fp-1");
    first.write_atomic(&path).unwrap();
    let second = {
        let mut manifest = JobManifest::new("fp-1");
        manifest.set_status(1, JobStatus::Done);
        manifest
    };
    second.write_atomic(&path).unwrap();

    let on_disk = fs::read_to_string(&path).unwrap();
    assert_eq!(on_disk, second.to_ron().unwrap());

    let tmp_residue: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().to_string_lossy().ends_with(".tmp"))
        .collect();
    assert!(
        tmp_residue.is_empty(),
        "leftover temp files: {tmp_residue:?}"
    );

    fs::remove_dir_all(&dir).ok();
}
