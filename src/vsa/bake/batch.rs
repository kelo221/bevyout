//! Resumable `bake --all-interiors` batch orchestration (issue #62).
//!
//! The I/O half of the batch bake: `plan.rs` owns the pure job-selection/
//! skip logic (and is what `tests/features.rs` exercises); this module walks
//! the cell catalogue, drives the reused #48 `JobManifest` on disk, and runs
//! the existing single-cell bake per selected cell.
//!
//! The catalogue is the `cellmap.ron` snapshot a batch `prepare` writes into
//! the cache dir (F47.4): baking presupposes prepared `scene.ron` manifests,
//! and `BakeArgs` deliberately has no game-root/plugin plumbing, so `bake`
//! reads the catalogue `prepare --all-interiors` already serialized instead
//! of re-parsing the plugin chain. Selection then goes through the same
//! `resolve_selection` (#46) grammar `prepare` uses, and resume/retry state
//! lives in `<cache_dir>/bake_jobs.ron` -- the same `JobManifest` type as
//! `prepare_jobs.ron`, in a separate file so bake and prepare progress never
//! overwrite each other. The manifest is keyed by the cell map's content
//! fingerprint, so a plugin-chain change discards recorded bake statuses
//! exactly as it discards prepare's (F48.1).
//!
//! Cells run sequentially: each bake performs a full CPU scene composition
//! and irradiance pass that saturates the machine on its own, unlike
//! prepare's parse/stage phases, so there is no `--jobs` worker pool here.

use std::collections::{BTreeMap, HashSet};

use super::*;
use crate::cli::progress::ProgressReporter;

use super::super::cell_map::CellMap;
use super::super::paths::absolutize;
use super::super::prepare::{
    CellSummary, JobManifest, JobStatus, SelectionSpec, resolve_selection,
};

/// `<cache_dir>/bake_jobs.ron`: the resumable bake job manifest, sibling of
/// prepare's `<cache_dir>/prepare_jobs.ron` (F62.1).
pub(crate) fn bake_jobs_path(cache_dir: &Path) -> PathBuf {
    cache_dir.join("bake_jobs.ron")
}

/// `<cache_dir>/scenes/<formid>/scene.ron`: where a batch `prepare` puts the
/// prepared manifest this batch bakes and writes its result back into.
fn scene_manifest_path(cache_dir: &Path, form_id: u32) -> PathBuf {
    cache_dir
        .join("scenes")
        .join(format!("{form_id:08x}"))
        .join("scene.ron")
}

/// Reads the cell catalogue snapshot written by `prepare --all-interiors`
/// (F47.4). Its absence means no batch prepare ever ran against this cache
/// dir, so there is nothing to bake yet -- the error says so instead of
/// reporting a bare missing file.
fn read_cell_map(cache_dir: &Path) -> Result<CellMap> {
    let path = cache_dir.join("cellmap.ron");
    let text = fs::read_to_string(&path).with_context(|| {
        format!(
            "could not read the cell catalogue {}; run `prepare --all-interiors` first to prepare the cells and write it",
            path.display()
        )
    })?;
    ron::de::from_str(&text).with_context(|| format!("invalid cell map {}", path.display()))
}

/// Whether the bake recorded in this cell's prepared scene manifest is still
/// valid against the current bake pipeline revision and this run's bake
/// parameters (F62.1). Any failure to load the manifest or rebuild its job
/// -- missing file, parse error, incompatible revisions, no renderable
/// placements -- means there is nothing trustworthy recorded, so the cell
/// counts as not-valid and re-runs (where the real bake will surface the
/// same error as a recorded failure).
fn recorded_bake_validity(args: &BakeArgs, scene_manifest: &Path) -> bool {
    let Ok(manifest) = load_prepared_manifest(scene_manifest) else {
        return false;
    };
    let Ok(outputs) = bake_outputs(&manifest) else {
        return false;
    };
    let mut job = build_bake_job_for_backend(
        &manifest,
        args,
        &outputs,
        backend_for_existing_bake(args, &manifest),
    );
    if exclude_animated_static_assets(&outputs.asset_root, &mut job).is_err() {
        return false;
    }
    let Ok(current_job_fingerprint) = bake_job_fingerprint(&manifest, &job) else {
        return false;
    };
    bake_is_valid(
        manifest.bake.as_ref(),
        CURRENT_BAKE_REVISION,
        &current_job_fingerprint,
    )
}

/// `bake --all-interiors` / `bake --retry-failed` (F62.1): resolves the cell
/// selection from the catalogue, filters it through the resumable job
/// manifest plus the per-cell bake validity check, bakes the remainder
/// sequentially, and persists progress after every cell so an interrupted
/// batch resumes without redoing valid work.
pub(crate) fn bake_batch(args: BakeArgs, progress: &ProgressReporter) -> Result<()> {
    if args.selector.is_some() || args.manifest.is_some() {
        bail!("--all-interiors/--retry-failed cannot be combined with a selector or --manifest");
    }
    let cache_dir = absolutize(
        args.cache_dir
            .as_deref()
            .unwrap_or_else(|| Path::new(".bevyout/cache")),
    )?;
    let cell_map = read_cell_map(&cache_dir)?;
    let fingerprint = cell_map.content_fingerprint.clone();
    let cells: Vec<CellSummary> = cell_map
        .cells
        .iter()
        .map(|entry| CellSummary {
            form_id: entry.form_id,
            editor_id: entry.editor_id.clone(),
            name: None,
            interior: entry.interior,
            worldspace_form_id: entry.worldspace_form_id,
            grid: entry.grid,
        })
        .collect();

    // F48.1 (reused): a manifest on disk built against a different content
    // fingerprint is discarded automatically by `load_or_new`.
    let manifest_path = bake_jobs_path(&cache_dir);
    let mut job_manifest = JobManifest::load_or_new(&manifest_path, &fingerprint)?;

    // F48.3 (reused): `--retry-failed` alone means "every failed cell in the
    // bake manifest"; combined with `--all-interiors`, the intersection of
    // that selection with the failed set (T62.2).
    let spec = SelectionSpec {
        all_interiors: args.all_interiors,
        ..Default::default()
    };
    let mut resolved = if args.retry_failed {
        if spec.is_empty() {
            job_manifest.failed_form_ids()
        } else {
            let selection = resolve_selection(&cells, &[], &spec)?;
            let failed: HashSet<u32> = job_manifest.failed_form_ids().into_iter().collect();
            selection
                .into_iter()
                .filter(|form_id| failed.contains(form_id))
                .collect()
        }
    } else {
        resolve_selection(&cells, &[], &spec)?
    };
    resolved.sort_unstable();
    resolved.dedup();

    // F48.2 (reused): every selected cell gets at least a `pending` entry,
    // and the manifest is written once up front -- before any cell runs --
    // so a crash before the first cell finishes still leaves a manifest
    // distinguishing "selected, not yet attempted" from "never selected".
    job_manifest.ensure_pending(&resolved);

    // F62.1: only cells `filter_resume` would skip on status (recorded
    // `Done`, and not forced) pay for a validity check, which re-reads the
    // cell's prepared scene manifest and recomputes its job fingerprint.
    let mut validity = BTreeMap::new();
    if !args.force {
        for &form_id in &resolved {
            if job_manifest.status(form_id) == Some(&JobStatus::Done) {
                validity.insert(
                    form_id,
                    recorded_bake_validity(&args, &scene_manifest_path(&cache_dir, form_id)),
                );
            }
        }
    }
    let (to_run, skipped, stale) =
        filter_bake_resume(&job_manifest, &resolved, args.force, &validity);
    for &form_id in &stale {
        println!("{}", stale_bake_line(form_id));
    }
    if skipped > 0 {
        println!("resuming: skipping {skipped} baked cell(s)");
    }
    job_manifest.write_atomic(&manifest_path)?;

    progress.started(
        super::bake_operation_label(args.lightmap_backend),
        Some(resolved.len() as u64),
    );
    progress.phase_started("cell", Some(resolved.len() as u64));
    for _ in 0..skipped {
        progress.unit_completed_in_phase("cell", Some(resolved.len() as u64), None);
    }

    for &form_id in &to_run {
        let scene_manifest = scene_manifest_path(&cache_dir, form_id);
        let result = bake_manifest(&args, &scene_manifest, progress);
        let status = match &result {
            Ok(()) => JobStatus::Done,
            Err(error) => JobStatus::Failed(format!("{error:#}")),
        };
        if let JobStatus::Failed(reason) = &status {
            eprintln!("cell {form_id:08x} failed: {reason}");
        }
        progress.unit_completed_in_phase("cell", Some(resolved.len() as u64), None);
        job_manifest.set_status(form_id, status);
        // F48.4 (reused): rewrite the manifest through after EVERY cell
        // completion (atomically) so interrupting the batch at any point is
        // safe to resume from.
        if let Err(error) = job_manifest.write_atomic(&manifest_path) {
            eprintln!("warning: failed to persist bake job manifest: {error:#}");
        }
    }

    let mut failed_entries: Vec<(u32, String)> = Vec::new();
    let mut baked_count = 0usize;
    for &form_id in &to_run {
        match job_manifest.status(form_id) {
            Some(JobStatus::Done) => baked_count += 1,
            Some(JobStatus::Failed(reason)) => failed_entries.push((form_id, reason.clone())),
            _ => {}
        }
    }
    failed_entries.sort_by_key(|(form_id, _)| *form_id);

    // F62.1: the deterministic end-of-batch summary -- always printed, even
    // with zero failures -- plus one sorted-by-FormID line per failure,
    // mirroring prepare's failure summary.
    println!(
        "{}",
        bake_batch_summary_line(baked_count, skipped, failed_entries.len())
    );
    for (form_id, reason) in &failed_entries {
        let first_line = reason.lines().next().unwrap_or("");
        println!("  {form_id:08x} {first_line}");
    }

    if !failed_entries.is_empty() {
        progress.finished(false);
        bail!(
            "{} of {} cell(s) failed to bake",
            failed_entries.len(),
            to_run.len()
        );
    }
    progress.finished(true);
    Ok(())
}
