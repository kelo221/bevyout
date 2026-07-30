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
