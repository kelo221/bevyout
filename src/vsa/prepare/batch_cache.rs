//! Pure batch-cache accounting for issue #47: hit/miss counters, the
//! aggregated end-of-batch cache summary line, and writing an already-built
//! `CellMap` to the cache dir. Depends only on `std`/`anyhow` plus
//! `vsa::cell_map` (itself std/serde-only, see its module doc comment), so
//! -- like `vsa::prepare::selectors` (issue #46) -- it is pulled into
//! `tests/features.rs` verbatim via `#[path]`, nested one module deep so its
//! relative `super::super::cell_map` import lands on that suite's own
//! `mod cell_map` include.
//!
//! The actual I/O (parsing the plugin chain, indexing BSAs, staging
//! footsteps) lives in the sibling `session.rs`, which is not included here
//! -- it pulls in `esplugin` and the rest of the preparation pipeline, so it
//! does not meet the std/serde-only bar for verbatim inclusion.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::super::cell_map::CellMap;
#[cfg(test)]
use super::super::cell_map::CellMapEntry;

/// A small keyed cache with hit/miss counters (F47.3). Callers supply the
/// actual work as a `build` closure, so this type stays I/O-free and can be
/// exercised with synthetic keys/values in tests (T47.1) even though its
/// real caller -- the session-level physics sidecar cache (F47.3) -- uses it
/// to avoid re-reading a sidecar file already read for an earlier cell in
/// the batch.
#[derive(Debug)]
pub(crate) struct KeyedBatchCache<T> {
    values: HashMap<String, T>,
    pub(crate) hits: usize,
    pub(crate) misses: usize,
}

impl<T> Default for KeyedBatchCache<T> {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
}

impl<T: Clone> KeyedBatchCache<T> {
    /// Returns the cached value for `key`, counting a hit; otherwise runs
    /// `build`, counts a miss, and remembers the result for later callers
    /// (e.g. a later cell in the same batch sharing the key). A failed
    /// `build` is not cached, so the next call with the same key retries it.
    pub(crate) fn get_or_insert_with<F>(&mut self, key: &str, build: F) -> Result<T>
    where
        F: FnOnce() -> Result<T>,
    {
        if let Some(value) = self.values.get(key) {
            self.hits += 1;
            return Ok(value.clone());
        }
        self.misses += 1;
        let value = build()?;
        self.values.insert(key.to_string(), value.clone());
        Ok(value)
    }

    /// Total lookups attempted through this cache, hit or miss.
    pub(crate) fn accesses(&self) -> usize {
        self.hits + self.misses
    }
}

/// Aggregated NIF-to-GLB asset cache counts across every cell in a batch
/// (F47.3). Each cell's own counts already reflect the on-disk
/// content-addressed cache (`vsa::assets`), which is naturally shared across
/// cells within a run; `BatchAssetTotals` just sums what each cell reported
/// so the batch can print one deterministic summary line.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BatchAssetTotals {
    pub(crate) reused: usize,
    pub(crate) built: usize,
    pub(crate) rebuilt: usize,
}

impl BatchAssetTotals {
    /// `cache_invalid` and `cache_explicit_rebuilds` are folded into
    /// `rebuilt`: both schedule a rebuild job rather than a first-time
    /// build, and the batch summary line only distinguishes "reused /
    /// built / rebuilt" (see `batch_cache_summary_line`).
    pub(crate) fn add(
        &mut self,
        cache_hits: usize,
        cache_missing: usize,
        cache_invalid: usize,
        cache_explicit_rebuilds: usize,
    ) {
        self.reused += cache_hits;
        self.built += cache_missing;
        self.rebuilt += cache_invalid + cache_explicit_rebuilds;
    }
}

/// One deterministic end-of-batch line (F47.3). The wording is a stable
/// contract; do not reword without checking for callers/tooling matching on
/// it verbatim.
pub(crate) fn batch_cache_summary_line(
    totals: BatchAssetTotals,
    physics_reads: usize,
    physics_hits: usize,
) -> String {
    format!(
        "batch cache: assets reused {}, built {}, rebuilt {}, physics reads {}, physics hits {}",
        totals.reused, totals.built, totals.rebuilt, physics_reads, physics_hits
    )
}

/// Writes an already-built `CellMap` to `<cache_dir>/cellmap.ron` (F47.4),
/// reusing the same `CellMap`/RON serialization `cells --map` uses
/// (`vsa::catalog::build_cell_map`) rather than inventing a second format.
/// Pure aside from the single write, so it is testable against a temp dir
/// with a synthetic `CellMap` (T47.3) -- no plugin or BSA parsing required.
pub(crate) fn write_cell_map(cache_dir: &Path, map: &CellMap) -> Result<PathBuf> {
    fs::create_dir_all(cache_dir)
        .with_context(|| format!("creating cache directory {}", cache_dir.display()))?;
    let path = cache_dir.join("cellmap.ron");
    let ron_text = map.to_ron().context("serializing cell map to RON")?;
    fs::write(&path, &ron_text)
        .with_context(|| format!("writing cell map to {}", path.display()))?;
    Ok(path)
}

#[cfg(test)]
#[path = "tests/batch_cache.rs"]
mod tests;
