//! Resumable batch bake job selection (issue #62).
//!
//! `bake --all-interiors` walks the cell catalogue exactly like `prepare
//! --all-interiors` -- the same `resolve_selection` grammar over the same
//! catalogue, read back from the `cellmap.ron` snapshot a batch `prepare`
//! writes into the cache dir (F47.4) so `bake` needs no game data of its
//! own -- and reuses the wave-2 resumable job-manifest machinery from issue
//! #48 (`prepare::jobs::JobManifest`/`JobStatus`/`filter_resume`)
//! completely unchanged: this module never touches `prepare/jobs.rs`, it
//! only calls the `pub(crate)` API that module already exposes. A separate
//! `<cache_dir>/bake_jobs.ron` file (see `bake_jobs_path` in `batch.rs`)
//! tracks each selected cell's pending/done/failed(reason) status the same
//! way `prepare_jobs.ron` does for `prepare`.
//!
//! Issue #49's `prepare::fingerprints` module is the *model* for the second
//! layer this module adds on top of `filter_resume`, not a type it reuses
//! directly: prepare records four independently-versioned components
//! (plugin content-set, converter, physics, prepare pipeline) because any
//! of them can change without altering the manifest bytes. A bake's
//! validity is simpler -- it depends only on (a) the bake pipeline's own
//! revision (`CURRENT_BAKE_REVISION`) and (b) the exact job-parameter
//! fingerprint `bake()` already computes and records in the cell's own
//! prepared scene manifest (`PreparedBake.source_fingerprint`, see
//! `bake_job_fingerprint` in `mod.rs`). That job fingerprint already folds in
//! the prepared cell's own `source_fingerprint`, so a converter, physics,
//! or prepare-pipeline change that altered the manifest's placements
//! changes the job fingerprint automatically -- there is no need to
//! separately track those three components the way `CellFingerprints`
//! does. `bake_is_valid` below is that two-part check; `filter_bake_resume`
//! extends `filter_resume` with it the same way `filter_resume_checked`
//! (#49) extends it for prepare (T62.1-T62.3).
//!
//! Pure std/serde, like its siblings `prepare::jobs` and
//! `prepare::fingerprints`: reachable from `tests/features.rs` via
//! `#[path]` (nested to land its `super::super::` imports on the already-
//! included `manifest`/`prepare` modules), so it must stay free of Bevy and
//! of any I/O of its own -- reading a cell's prepared scene manifest and
//! recomputing its current job fingerprint is the caller's job (`batch.rs`'s
//! `recorded_bake_validity`), not this module's.

use std::collections::{BTreeMap, HashSet};

use super::super::manifest::PreparedBake;
use super::super::prepare::{JobManifest, filter_resume};

/// True when a previously recorded bake is still valid against the current
/// bake pipeline revision and job-parameter fingerprint (F62.1). `bake` is
/// `None` for a cell that was never baked (or whose scene manifest failed
/// to load/parse/validate) -- always invalid.
pub(crate) fn bake_is_valid(
    bake: Option<&PreparedBake>,
    current_bake_revision: &str,
    current_job_fingerprint: &str,
) -> bool {
    bake.is_some_and(|bake| {
        bake.bake_revision.as_deref() == Some(current_bake_revision)
            && bake.source_fingerprint == current_job_fingerprint
    })
}

/// Extends `filter_resume` (#48, reused unchanged) the same way #49's
/// `filter_resume_checked` extends it for prepare: every cell `filter_resume`
/// would already run (never recorded `Done`, or `force`) is kept as-is
/// (T62.1: pending/failed/never-recorded cells are governed entirely by
/// status). Only a `Done` cell is given a second look, via `valid` --
/// precomputed per cell by the caller (`recorded_bake_validity` in
/// `batch.rs`) from the cell's real prepared scene manifest -- and is
/// skipped only when
/// `valid` says its bake is still current; otherwise it is moved back into
/// `to_run` and reported in the returned stale list (T62.3). `force`
/// bypasses both checks exactly as `filter_resume` does.
pub(crate) fn filter_bake_resume(
    manifest: &JobManifest,
    selection: &[u32],
    force: bool,
    valid: &BTreeMap<u32, bool>,
) -> (Vec<u32>, usize, Vec<u32>) {
    let (status_to_run, _status_skipped) = filter_resume(manifest, selection, force);
    if force {
        return (status_to_run, 0, Vec::new());
    }
    let status_to_run: HashSet<u32> = status_to_run.into_iter().collect();

    let mut to_run = Vec::with_capacity(selection.len());
    let mut skipped = 0usize;
    let mut stale = Vec::new();
    for &form_id in selection {
        if status_to_run.contains(&form_id) {
            to_run.push(form_id);
            continue;
        }
        if valid.get(&form_id).copied().unwrap_or(false) {
            skipped += 1;
        } else {
            stale.push(form_id);
            to_run.push(form_id);
        }
    }
    (to_run, skipped, stale)
}

/// `bake fingerprint: cell <formid:08x> stale` (F62.1, one line per
/// re-queued `Done` cell), mirroring #49's `fingerprint: cell ... stale
/// (...)` lines. Bake validity is a single yes/no (see `bake_is_valid`), so
/// there is no per-component list to append.
pub(crate) fn stale_bake_line(form_id: u32) -> String {
    format!("bake fingerprint: cell {form_id:08x} stale")
}

/// The deterministic end-of-batch summary line (F62.1, the CLI logging
/// convention): `bake batch: <n> baked, <m> skipped (valid), <k> failed`.
/// Always printed once per batch run, mirroring prepare's always-printed
/// `<n> done, <m> failed` summary. The wording is a stable contract; do not
/// reword without checking for callers/tooling matching on it verbatim.
pub(crate) fn bake_batch_summary_line(baked: usize, skipped: usize, failed: usize) -> String {
    format!("bake batch: {baked} baked, {skipped} skipped (valid), {failed} failed")
}

#[cfg(test)]
mod tests {
    use super::super::super::prepare::JobStatus;
    use super::*;

    fn bake(revision: &str, job_fingerprint: &str) -> PreparedBake {
        PreparedBake {
            bake_revision: Some(revision.into()),
            source_fingerprint: job_fingerprint.into(),
            scene_path: "scenes/00000001/baked/scene.glb".into(),
            irradiance_volume: None,
            dynamic_lighting: None,
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
}
