//! Resumable prepare job manifest (issue #48).
//!
//! `prepare --all` (and friends) can take a long time over many cells; if the
//! process is interrupted, restarting used to mean re-preparing every cell
//! from scratch. This module tracks each selected cell's `pending` / `done`
//! / `failed(reason)` status in a small RON file
//! (`<cache_dir>/prepare_jobs.ron`, F48.1) so a later run can skip cells
//! already `done` (F48.2) or retry only the ones that previously `failed`
//! (F48.3).
//!
//! Pure std/serde/ron, like `selectors.rs` (#46) and `batch_cache.rs` (#47):
//! no plugin parsing, no BSA/Blender I/O beyond reading and writing its own
//! RON file, so it is pulled into `tests/features.rs` verbatim via
//! `#[path]`. Unlike those two it has no relative `super::super::` imports,
//! so it needs no special nesting to compile there.
//!
//! Issue #49 adds fingerprint validation on top of the F48 pending/done/
//! failed status this module already tracks: a `fingerprints` map (keyed by
//! the same FormID as `jobs`, `#[serde(default)]` so a manifest written
//! before this issue loads with an empty map rather than a parse error --
//! F49.4) records each `Done` cell's `fingerprints::CellFingerprints`, and
//! `filter_resume_checked` extends `filter_resume` below with a fingerprint
//! check: a `Done` cell whose recorded fingerprints no longer match the
//! current toolchain is treated as needing to re-run, not skipped (F49.2).

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::fingerprints::{CellFingerprints, StaleCells, stale_components};

/// One cell's status in the job manifest (F48.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum JobStatus {
    Pending,
    Done,
    Failed(String),
}

/// The on-disk resumable job manifest: the content set it was built
/// against, plus each selected cell's current status. `jobs` is a
/// `BTreeMap` keyed by FormID (not a `HashMap`) so its RON serialization is
/// byte-identical across runs regardless of insertion order (F48.1).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct JobManifest {
    pub(crate) content_fingerprint: String,
    pub(crate) jobs: BTreeMap<u32, JobStatus>,
    /// F49.1/F49.4: each `Done` cell's recorded fingerprints, keyed by the
    /// same FormID as `jobs`. `#[serde(default)]` so a manifest written by
    /// the pre-#49 format (no `fingerprints` field at all) still parses --
    /// every cell in it simply has no entry here, which `stale_components`
    /// treats as stale in every component rather than a parse error.
    #[serde(default)]
    pub(crate) fingerprints: BTreeMap<u32, CellFingerprints>,
}

impl JobManifest {
    pub(crate) fn new(content_fingerprint: impl Into<String>) -> Self {
        Self {
            content_fingerprint: content_fingerprint.into(),
            jobs: BTreeMap::new(),
            fingerprints: BTreeMap::new(),
        }
    }

    /// Loads the manifest at `path`. Starts fresh (empty `jobs`) if the file
    /// does not exist yet, or if it exists but was built against a
    /// different content fingerprint -- a plugin chain edit invalidates
    /// every recorded status, so nothing under the old fingerprint is
    /// trusted (F48.1).
    pub(crate) fn load_or_new(path: &Path, content_fingerprint: &str) -> Result<Self> {
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::new(content_fingerprint));
            }
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("reading job manifest {}", path.display()));
            }
        };
        let manifest: Self = ron::de::from_str(&text)
            .with_context(|| format!("parsing job manifest {}", path.display()))?;
        if manifest.content_fingerprint != content_fingerprint {
            return Ok(Self::new(content_fingerprint));
        }
        Ok(manifest)
    }

    /// Inserts `Pending` for every FormID in `form_ids` that has no existing
    /// entry; an existing `done`/`failed`/`pending` status is left alone
    /// (F48.2: newly selected cells default to pending).
    pub(crate) fn ensure_pending(&mut self, form_ids: &[u32]) {
        for &form_id in form_ids {
            self.jobs.entry(form_id).or_insert(JobStatus::Pending);
        }
    }

    pub(crate) fn set_status(&mut self, form_id: u32, status: JobStatus) {
        self.jobs.insert(form_id, status);
    }

    pub(crate) fn status(&self, form_id: u32) -> Option<&JobStatus> {
        self.jobs.get(&form_id)
    }

    /// Records `fingerprints` for a `Done` cell (F49.1). Called once the
    /// cell's status is set to `Done`; overwrites any previously recorded
    /// fingerprints for the same FormID.
    pub(crate) fn record_fingerprints(&mut self, form_id: u32, fingerprints: CellFingerprints) {
        self.fingerprints.insert(form_id, fingerprints);
    }

    /// The fingerprints recorded for `form_id`, or `None` for a cell never
    /// recorded under this issue -- either never prepared, or a legacy
    /// entry from before F49.1 (F49.4).
    pub(crate) fn fingerprints_for(&self, form_id: u32) -> Option<&CellFingerprints> {
        self.fingerprints.get(&form_id)
    }

    /// Every cell currently recorded `failed`, FormID-sorted (F48.3; a
    /// `BTreeMap` iterates in key order already, so no extra sort is
    /// needed).
    pub(crate) fn failed_form_ids(&self) -> Vec<u32> {
        self.jobs
            .iter()
            .filter(|(_, status)| matches!(status, JobStatus::Failed(_)))
            .map(|(form_id, _)| *form_id)
            .collect()
    }

    pub(crate) fn to_ron(&self) -> Result<String> {
        Ok(ron::ser::to_string_pretty(
            self,
            ron::ser::PrettyConfig::default(),
        )?)
    }

    /// Writes the manifest to `path` atomically (F48.4): a temp file next
    /// to `path` is written in full, then renamed over `path`. A rename is
    /// a single filesystem operation, so a process interrupted at any point
    /// -- including mid-write of the temp file -- leaves `path` holding
    /// either the previous manifest or the complete new one, never a
    /// half-written file. Called after every cell completion so interrupting
    /// a batch at any point loses at most the in-flight cell's result.
    pub(crate) fn write_atomic(&self, path: &Path) -> Result<()> {
        write_atomic(path, &self.to_ron()?)
    }
}

/// Splits `selection` into cells to run and a skipped count (F48.2). Cells
/// recorded `done` in `manifest` are skipped; `pending`, `failed`, and
/// never-recorded cells are kept. `force = true` bypasses the filter
/// entirely and reruns everything in `selection`.
pub(crate) fn filter_resume(
    manifest: &JobManifest,
    selection: &[u32],
    force: bool,
) -> (Vec<u32>, usize) {
    if force {
        return (selection.to_vec(), 0);
    }
    let mut to_run = Vec::with_capacity(selection.len());
    let mut skipped = 0usize;
    for &form_id in selection {
        if manifest.status(form_id) == Some(&JobStatus::Done) {
            skipped += 1;
        } else {
            to_run.push(form_id);
        }
    }
    (to_run, skipped)
}

/// F49.2: extends `filter_resume` with fingerprint validation, built on top
/// of it rather than duplicating its status logic: `filter_resume` first
/// decides, by status alone, which cells would run/skip; every cell it
/// already decided to run is kept as-is (never previously `Done`, so there
/// is nothing recorded to validate). Only the cells it decided to *skip*
/// (recorded `Done`) get a second look: skipped only when its recorded
/// fingerprints (`manifest.fingerprints_for`) match `current` in every
/// component; a `Done` cell with any stale component is moved back into
/// `to_run` and reported in the returned stale list instead of being
/// counted as skipped (T49.2, T49.3). A legacy `Done` cell with no
/// recorded fingerprints at all is stale in every component (F49.4), via
/// `stale_components(None, ...)`. `force` bypasses both checks exactly as
/// `filter_resume` does (every selected cell reruns regardless, so nothing
/// is "stale" -- the third element is empty).
pub(crate) fn filter_resume_checked(
    manifest: &JobManifest,
    selection: &[u32],
    force: bool,
    current: &CellFingerprints,
) -> (Vec<u32>, usize, StaleCells) {
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
        let components = stale_components(manifest.fingerprints_for(form_id), current);
        if components.is_empty() {
            skipped += 1;
        } else {
            stale.push((form_id, components));
            to_run.push(form_id);
        }
    }
    (to_run, skipped, stale)
}

/// `<cache_dir>/prepare_jobs.ron` (F48.1).
pub(crate) fn manifest_path(cache_dir: &Path) -> PathBuf {
    cache_dir.join("prepare_jobs.ron")
}

fn write_atomic(path: &Path, contents: &str) -> Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)
        .with_context(|| format!("creating directory {}", parent.display()))?;
    let unique = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    let temp_path = parent.join(format!(".prepare_jobs.{}.{unique}.tmp", std::process::id()));
    fs::write(&temp_path, contents)
        .with_context(|| format!("writing temp job manifest {}", temp_path.display()))?;
    fs::rename(&temp_path, path).with_context(|| {
        format!(
            "renaming temp job manifest {} into place at {}",
            temp_path.display(),
            path.display()
        )
    })?;
    Ok(())
}

#[cfg(test)]
#[path = "tests/jobs.rs"]
mod tests;
