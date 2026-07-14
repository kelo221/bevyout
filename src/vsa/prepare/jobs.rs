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

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

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
}

impl JobManifest {
    pub(crate) fn new(content_fingerprint: impl Into<String>) -> Self {
        Self {
            content_fingerprint: content_fingerprint.into(),
            jobs: BTreeMap::new(),
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
mod tests {
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
}
