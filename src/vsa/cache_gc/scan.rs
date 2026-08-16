use super::{
    model::{CACHE_GC_SCHEMA_VERSION, GcCandidate, GcReasonSummary, GcReport},
    policy::{CacheEntryClass, CacheEntryFacts, GcPolicy, gc_reason},
};
use crate::vsa::{
    cache_stats::scan::{allocated_file_size, collect_live_cache_files},
    cache_store::{PreparedObjectRef, PreparedRecipeRecord},
};
use anyhow::{Context, Result, bail};
use bevyout_core::manifest::exterior::EXTERIOR_CELL_PACKAGE_REVISION;
use rayon::prelude::*;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

pub(crate) fn plan_gc(
    cache_root: &Path,
    dry_run: bool,
    grace_hours: u64,
    include_rebuildable: bool,
) -> Result<GcReport> {
    let cache_root = fs::canonicalize(cache_root)
        .with_context(|| format!("could not resolve cache root {}", cache_root.display()))?;
    if !cache_root.is_dir() {
        bail!("cache root is not a directory: {}", cache_root.display());
    }
    let live = collect_live_cache_files(&cache_root)?;
    let policy = GcPolicy {
        grace_seconds: grace_hours.saturating_mul(3600),
        include_rebuildable,
    };
    let now = SystemTime::now();
    let mut files = Vec::new();
    collect_files(&cache_root, &cache_root, &mut files)?;
    let examined_file_count = files.len() as u64;
    let mut candidates = Vec::new();
    for path in files {
        let relative_path = normalized_relative(&cache_root, &path)?;
        if relative_path == super::lock::CACHE_GC_LOCK_NAME {
            continue;
        }
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            bail!(
                "cache GC does not follow symbolic links: {}",
                path.display()
            );
        }
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let age_seconds = now
            .duration_since(modified)
            .unwrap_or(Duration::ZERO)
            .as_secs();
        let (class, recipe_output) = classify_entry(&path, &relative_path)?;
        let mut reachable = live.contains(&path);
        if let Some(output) = recipe_output {
            reachable |= object_path(&cache_root, &output)
                .and_then(|path| fs::canonicalize(path).ok())
                .is_some_and(|path| live.contains(&path));
        }
        let Some(reason) = gc_reason(
            CacheEntryFacts {
                class,
                reachable,
                age_seconds,
            },
            policy,
        ) else {
            continue;
        };
        let logical_bytes = metadata.len();
        candidates.push(GcCandidate {
            relative_path,
            reason: reason.label().into(),
            logical_bytes,
            allocated_bytes: 0,
            age_seconds,
            absolute_path: path,
            modified,
        });
    }
    candidates.par_iter_mut().for_each(|candidate| {
        candidate.allocated_bytes =
            allocated_file_size(&candidate.absolute_path, candidate.logical_bytes);
    });
    candidates.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));

    let mut candidates_by_reason = BTreeMap::<String, GcReasonSummary>::new();
    for candidate in &candidates {
        let summary = candidates_by_reason
            .entry(candidate.reason.clone())
            .or_default();
        summary.file_count += 1;
        summary.logical_bytes = summary
            .logical_bytes
            .saturating_add(candidate.logical_bytes);
        summary.allocated_bytes = summary
            .allocated_bytes
            .saturating_add(candidate.allocated_bytes);
    }
    Ok(GcReport {
        schema_version: CACHE_GC_SCHEMA_VERSION.into(),
        cache_root: cache_root.to_string_lossy().replace('\\', "/"),
        dry_run,
        grace_hours,
        include_rebuildable,
        live_file_count: live.len() as u64,
        examined_file_count,
        candidate_file_count: candidates.len() as u64,
        candidate_logical_bytes: candidates
            .iter()
            .map(|candidate| candidate.logical_bytes)
            .sum(),
        candidate_allocated_bytes: candidates
            .iter()
            .map(|candidate| candidate.allocated_bytes)
            .sum(),
        candidates_by_reason,
        deleted_file_count: 0,
        deleted_logical_bytes: 0,
        deleted_allocated_bytes: 0,
        candidates,
    })
}

pub(crate) fn sweep(report: &mut GcReport) -> Result<()> {
    if report.dry_run {
        return Ok(());
    }
    let cache_root = fs::canonicalize(Path::new(&report.cache_root))?;
    for candidate in &report.candidates {
        let metadata = match fs::symlink_metadata(&candidate.absolute_path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error.into()),
        };
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            bail!(
                "cache GC candidate changed type after planning: {}",
                candidate.absolute_path.display()
            );
        }
        let canonical = fs::canonicalize(&candidate.absolute_path)?;
        if !canonical.starts_with(&cache_root)
            || metadata.len() != candidate.logical_bytes
            || metadata.modified().ok() != Some(candidate.modified)
        {
            bail!(
                "cache GC candidate changed after planning: {}",
                candidate.absolute_path.display()
            );
        }
        fs::remove_file(&canonical)
            .with_context(|| format!("deleting cache GC candidate {}", canonical.display()))?;
        report.deleted_file_count += 1;
        report.deleted_logical_bytes = report
            .deleted_logical_bytes
            .saturating_add(candidate.logical_bytes);
        report.deleted_allocated_bytes = report
            .deleted_allocated_bytes
            .saturating_add(candidate.allocated_bytes);
        remove_empty_parents(canonical.parent(), &cache_root);
    }
    Ok(())
}

fn collect_files(root: &Path, directory: &Path, output: &mut Vec<PathBuf>) -> Result<()> {
    let mut entries = fs::read_dir(directory)
        .with_context(|| format!("reading cache directory {}", directory.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            bail!(
                "cache GC does not follow symbolic links: {}",
                path.display()
            );
        }
        if metadata.is_dir() {
            collect_files(root, &path, output)?;
        } else if metadata.is_file() {
            let canonical = fs::canonicalize(&path)?;
            if !canonical.starts_with(root) {
                bail!("cache entry escaped cache root: {}", path.display());
            }
            output.push(canonical);
        }
    }
    Ok(())
}

fn normalized_relative(root: &Path, path: &Path) -> Result<String> {
    Ok(path
        .strip_prefix(root)?
        .to_string_lossy()
        .replace('\\', "/"))
}

fn classify_entry(
    path: &Path,
    relative_path: &str,
) -> Result<(CacheEntryClass, Option<PreparedObjectRef>)> {
    if relative_path.starts_with("quarantine/") {
        return Ok((CacheEntryClass::Quarantine, None));
    }
    if relative_path.starts_with("objects/") {
        return Ok((CacheEntryClass::Object, None));
    }
    if relative_path.starts_with("recipes/") {
        let record = fs::read_to_string(path)
            .ok()
            .and_then(|source| ron::from_str::<PreparedRecipeRecord>(&source).ok());
        return Ok(match record {
            Some(record) => (CacheEntryClass::Recipe, Some(record.output)),
            None => (CacheEntryClass::Other, None),
        });
    }
    if relative_path.starts_with("staging/") {
        return Ok((CacheEntryClass::Staging, None));
    }
    if relative_path.starts_with("assets/terrain/") {
        return Ok((CacheEntryClass::LegacyTerrain, None));
    }
    if is_exterior_package_path(relative_path) {
        return Ok((
            CacheEntryClass::LegacyExteriorPackage {
                current_revision: file_prefix_contains(path, EXTERIOR_CELL_PACKAGE_REVISION)?,
            },
            None,
        ));
    }
    if relative_path.starts_with("assets/") {
        return Ok((CacheEntryClass::RebuildableAsset, None));
    }
    Ok((CacheEntryClass::Other, None))
}

fn is_exterior_package_path(path: &str) -> bool {
    let parts = path.split('/').collect::<Vec<_>>();
    parts.len() == 4
        && parts[0] == "worldspaces"
        && parts[2] == "cells"
        && parts[3].ends_with(".ron")
}

fn file_prefix_contains(path: &Path, needle: &str) -> Result<bool> {
    let mut bytes = Vec::with_capacity(1024);
    BufReader::new(File::open(path)?)
        .take(1024)
        .read_to_end(&mut bytes)?;
    Ok(bytes
        .windows(needle.len())
        .any(|window| window == needle.as_bytes()))
}

fn object_path(cache_root: &Path, object: &PreparedObjectRef) -> Option<PathBuf> {
    if object.sha256.len() != 64 {
        return None;
    }
    Some(
        cache_root
            .join("objects")
            .join(object.kind.tag())
            .join(&object.sha256[0..2])
            .join(&object.sha256[2..4])
            .join(format!("{}.{}", object.sha256, object.extension)),
    )
}

fn remove_empty_parents(mut parent: Option<&Path>, root: &Path) {
    while let Some(directory) = parent {
        if directory == root || !directory.starts_with(root) {
            break;
        }
        if fs::remove_dir(directory).is_err() {
            break;
        }
        parent = directory.parent();
    }
}

#[cfg(test)]
pub(crate) fn candidate_paths(report: &GcReport) -> std::collections::BTreeSet<&str> {
    report
        .candidates
        .iter()
        .map(|candidate| candidate.relative_path.as_str())
        .collect()
}
