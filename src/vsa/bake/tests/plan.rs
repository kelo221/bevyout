use super::super::super::prepare::JobStatus;
use super::*;

fn bake(revision: &str, job_fingerprint: &str) -> PreparedBake {
    PreparedBake {
        bake_revision: Some(revision.into()),
        source_fingerprint: job_fingerprint.into(),
        scene_path: "scenes/00000001/baked/scene.glb".into(),
        irradiance_volume: None,
    }
}

// Mirrors T49.2's per-component invalidation shape, but for bake's two
// components (bake pipeline revision, job fingerprint).
#[test]
fn bake_is_valid_requires_both_revision_and_job_fingerprint_to_match() {
    let recorded = bake("bake-v1", "job-abc");
    assert!(bake_is_valid(Some(&recorded), "bake-v1", "job-abc"));
    assert!(!bake_is_valid(Some(&recorded), "bake-v2", "job-abc"));
    assert!(!bake_is_valid(Some(&recorded), "bake-v1", "job-xyz"));
    assert!(!bake_is_valid(None, "bake-v1", "job-abc"));
}

// T62.1: resume filtering skips a done-and-valid cell, keeps pending and
// failed cells, exactly like prepare's `filter_resume`.
#[test]
fn resume_filter_skips_done_valid_keeps_pending_and_failed() {
    let mut manifest = JobManifest::new("fp-1");
    manifest.set_status(1, JobStatus::Done);
    manifest.set_status(2, JobStatus::Pending);
    manifest.set_status(3, JobStatus::Failed("boom".into()));
    // FormID 4 was never recorded -- newly selected, treated as pending.

    let mut valid = BTreeMap::new();
    valid.insert(1, true);

    let (to_run, skipped, stale) = filter_bake_resume(&manifest, &[1, 2, 3, 4], false, &valid);
    assert_eq!(to_run, vec![2, 3, 4]);
    assert_eq!(skipped, 1);
    assert!(stale.is_empty());
}

// T62.2: `--retry-failed` selection (via `JobManifest::failed_form_ids`,
// reused unchanged) picks exactly the failed set; unaffected by bake
// validity, which this function never even consults for non-`Done`
// cells.
#[test]
fn retry_failed_selection_is_exactly_the_failed_set() {
    let mut manifest = JobManifest::new("fp-1");
    manifest.set_status(1, JobStatus::Done);
    manifest.set_status(2, JobStatus::Pending);
    manifest.set_status(3, JobStatus::Failed("boom".into()));
    manifest.set_status(4, JobStatus::Failed("bang".into()));

    assert_eq!(manifest.failed_form_ids(), vec![3, 4]);
}

#[test]
fn force_reruns_everything_regardless_of_status_or_validity() {
    let mut manifest = JobManifest::new("fp-1");
    manifest.set_status(1, JobStatus::Done);
    manifest.set_status(2, JobStatus::Failed("boom".into()));

    let mut valid = BTreeMap::new();
    valid.insert(1, true);

    let (to_run, skipped, stale) = filter_bake_resume(&manifest, &[1, 2], true, &valid);
    assert_eq!(to_run, vec![1, 2]);
    assert_eq!(skipped, 0);
    assert!(stale.is_empty());
}

// T62.3: a `Done` cell whose recorded bake is no longer valid is
// requeued and reported stale; a mixed manifest requeues exactly the
// stale/incomplete subset, matching #49's `mixed_manifest_requeues_
// exactly_the_stale_subset` shape.
#[test]
fn mixed_manifest_requeues_exactly_the_stale_subset() {
    let mut manifest = JobManifest::new("fp-1");
    manifest.set_status(1, JobStatus::Done);
    manifest.set_status(2, JobStatus::Done);
    manifest.set_status(3, JobStatus::Pending);

    let mut valid = BTreeMap::new();
    valid.insert(1, true);
    valid.insert(2, false);

    let (to_run, skipped, stale) = filter_bake_resume(&manifest, &[1, 2, 3], false, &valid);
    assert_eq!(to_run, vec![2, 3]);
    assert_eq!(skipped, 1);
    assert_eq!(stale, vec![2]);
}

// A `Done` cell with no recorded validity entry at all (the caller only
// populates `valid` for cells it actually checked) counts as stale
// rather than panicking or silently skipping.
#[test]
fn done_cell_missing_from_the_validity_map_counts_as_stale() {
    let mut manifest = JobManifest::new("fp-1");
    manifest.set_status(1, JobStatus::Done);

    let valid = BTreeMap::new();
    let (to_run, skipped, stale) = filter_bake_resume(&manifest, &[1], false, &valid);
    assert_eq!(to_run, vec![1]);
    assert_eq!(skipped, 0);
    assert_eq!(stale, vec![1]);
}

#[test]
fn stale_bake_line_is_deterministic() {
    assert_eq!(
        stale_bake_line(0x0001_2345),
        "bake fingerprint: cell 00012345 stale"
    );
}

#[test]
fn bake_batch_summary_line_reports_baked_skipped_and_failed_counts() {
    assert_eq!(
        bake_batch_summary_line(2, 5, 1),
        "bake batch: 2 baked, 5 skipped (valid), 1 failed"
    );
}
