use super::super::cache::{TileCache, TileCacheStats, TileKey};
use std::fs;

fn temp_root(name: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!(
        "bevyout-lightmap-cache-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    root
}

#[test]
fn round_trip_is_deterministic_and_tracks_hits_and_writes() {
    let root = temp_root("round-trip");
    let mut cache = TileCache::open(&root, "fingerprint-a", false).unwrap();
    cache
        .write(
            TileKey {
                primitive: 3,
                tile_x: 4,
                tile_y: 5,
            },
            "tile-fingerprint-a",
            2,
            1,
            b"payload",
        )
        .unwrap();
    assert_eq!(
        cache
            .read(
                TileKey {
                    primitive: 3,
                    tile_x: 4,
                    tile_y: 5,
                },
                "tile-fingerprint-a",
            )
            .unwrap(),
        Some(super::super::cache::TileRecord {
            width: 2,
            height: 1,
            payload: b"payload".to_vec(),
        })
    );
    assert_eq!(
        cache.stats(),
        TileCacheStats {
            hits: 1,
            misses: 0,
            writes: 1,
        }
    );
    assert!(
        cache
            .tile_path(TileKey {
                primitive: 3,
                tile_x: 4,
                tile_y: 5,
            })
            .ends_with("page_0003_tile_0004_0005.bin")
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn missing_tile_is_a_miss_and_stale_fingerprint_invalidates_entries() {
    let root = temp_root("stale");
    let mut first = TileCache::open(&root, "fingerprint-a", false).unwrap();
    let key = TileKey {
        primitive: 0,
        tile_x: 0,
        tile_y: 0,
    };
    assert_eq!(first.read(key, "tile-a").unwrap(), None);
    first.write(key, "tile-a", 1, 1, b"old").unwrap();
    drop(first);

    let mut second = TileCache::open(&root, "fingerprint-b", false).unwrap();
    assert_eq!(second.read(key, "tile-a").unwrap(), None);
    let files = fs::read_dir(&root).unwrap().count();
    assert_eq!(files, 1); // cache.meta only
    let _ = fs::remove_dir_all(root);
}

#[test]
fn corruption_and_truncation_are_rejected() {
    let root = temp_root("corrupt");
    let mut cache = TileCache::open(&root, "fingerprint", false).unwrap();
    let key = TileKey {
        primitive: 0,
        tile_x: 0,
        tile_y: 0,
    };
    cache
        .write(key, "tile-fingerprint", 1, 1, b"payload")
        .unwrap();
    let path = cache.tile_path(key);
    let mut bytes = fs::read(&path).unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 0xff;
    fs::write(&path, &bytes).unwrap();
    assert!(cache.read(key, "tile-fingerprint").is_err());
    fs::write(&path, &bytes[..bytes.len() - 2]).unwrap();
    assert!(cache.read(key, "tile-fingerprint").is_err());
    let _ = fs::remove_dir_all(root);
}

#[test]
fn force_clear_leaves_a_fresh_metadata_root() {
    let root = temp_root("clear");
    let mut cache = TileCache::open(&root, "fingerprint-a", false).unwrap();
    let key = TileKey {
        primitive: 0,
        tile_x: 0,
        tile_y: 0,
    };
    cache
        .write(key, "tile-fingerprint", 1, 1, b"payload")
        .unwrap();
    drop(cache);
    let mut cleared = TileCache::open(&root, "fingerprint-a", true).unwrap();
    assert_eq!(cleared.read(key, "tile-fingerprint").unwrap(), None);
    assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
    let _ = fs::remove_dir_all(root);
}

#[test]
fn stale_tile_fingerprints_are_misses_without_clearing_unrelated_tiles() {
    let root = temp_root("partial");
    let mut first = TileCache::open(&root, "scene-fingerprint", false).unwrap();
    first
        .write(
            TileKey {
                primitive: 0,
                tile_x: 0,
                tile_y: 0,
            },
            "light-a",
            1,
            1,
            b"a",
        )
        .unwrap();
    first
        .write(
            TileKey {
                primitive: 1,
                tile_x: 0,
                tile_y: 0,
            },
            "light-b",
            1,
            1,
            b"b",
        )
        .unwrap();
    drop(first);

    let mut second = TileCache::open(&root, "scene-fingerprint", false).unwrap();
    assert_eq!(
        second
            .read(
                TileKey {
                    primitive: 0,
                    tile_x: 0,
                    tile_y: 0,
                },
                "light-a",
            )
            .unwrap()
            .unwrap()
            .payload,
        b"a"
    );
    assert_eq!(
        second
            .read(
                TileKey {
                    primitive: 1,
                    tile_x: 0,
                    tile_y: 0,
                },
                "light-c",
            )
            .unwrap(),
        None
    );
    assert_eq!(second.stats().hits, 1);
    assert_eq!(second.stats().misses, 1);
    let _ = fs::remove_dir_all(root);
}
