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
mod tests {
    use super::*;

    // T47.1: a batch counter type fed two cells sharing an asset/physics key
    // reports the second occurrence as a hit. Synthetic keys, no I/O.
    #[test]
    fn shared_key_across_two_cells_is_a_hit_on_the_second_occurrence() {
        let mut cache: KeyedBatchCache<u32> = KeyedBatchCache::default();
        let first = cache
            .get_or_insert_with("assets/shared.physics.json.gz", || Ok(7))
            .unwrap();
        let second = cache
            .get_or_insert_with("assets/shared.physics.json.gz", || {
                panic!("must not rebuild a key already cached this batch")
            })
            .unwrap();
        assert_eq!(first, 7);
        assert_eq!(second, 7);
        assert_eq!(cache.hits, 1);
        assert_eq!(cache.misses, 1);
        assert_eq!(cache.accesses(), 2);
    }

    #[test]
    fn distinct_keys_never_count_as_hits() {
        let mut cache: KeyedBatchCache<u32> = KeyedBatchCache::default();
        cache.get_or_insert_with("a", || Ok(1)).unwrap();
        cache.get_or_insert_with("b", || Ok(2)).unwrap();
        assert_eq!(cache.hits, 0);
        assert_eq!(cache.misses, 2);
    }

    #[test]
    fn failed_build_is_not_cached_and_is_retried() {
        let mut cache: KeyedBatchCache<u32> = KeyedBatchCache::default();
        assert!(
            cache
                .get_or_insert_with("x", || Err(anyhow::anyhow!("boom")))
                .is_err()
        );
        let value = cache.get_or_insert_with("x", || Ok(9)).unwrap();
        assert_eq!(value, 9);
        assert_eq!(cache.misses, 2);
        assert_eq!(cache.hits, 0);
    }

    #[test]
    fn batch_asset_totals_sum_across_cells() {
        let mut totals = BatchAssetTotals::default();
        totals.add(3, 1, 0, 0); // cell A: 3 reused, 1 built
        totals.add(2, 0, 1, 1); // cell B: 2 reused, 1 invalid + 1 explicit rebuild
        assert_eq!(
            totals,
            BatchAssetTotals {
                reused: 5,
                built: 1,
                rebuilt: 2,
            }
        );
    }

    #[test]
    fn summary_line_has_stable_wording() {
        let totals = BatchAssetTotals {
            reused: 5,
            built: 1,
            rebuilt: 2,
        };
        assert_eq!(
            batch_cache_summary_line(totals, 4, 3),
            "batch cache: assets reused 5, built 1, rebuilt 2, physics reads 4, physics hits 3"
        );
    }

    // T47.3: the batch flow writes `cellmap.ron` under a temp cache dir,
    // factored so it is testable without real game data.
    #[test]
    fn write_cell_map_creates_the_file_under_the_cache_dir_root() {
        let dir = std::env::temp_dir().join(format!(
            "bevyout-batch-cache-test-{}-{}",
            std::process::id(),
            line!()
        ));
        let map = CellMap::build(
            "fingerprint".into(),
            Vec::new(),
            vec![CellMapEntry {
                form_id: 1,
                editor_id: Some("A".into()),
                interior: true,
                worldspace_form_id: None,
                grid: None,
            }],
            Vec::new(),
            0,
        );
        let path = write_cell_map(&dir, &map).unwrap();
        assert_eq!(path, dir.join("cellmap.ron"));
        let written = std::fs::read_to_string(&path).unwrap();
        assert_eq!(written, map.to_ron().unwrap());
        std::fs::remove_dir_all(&dir).ok();
    }
}
