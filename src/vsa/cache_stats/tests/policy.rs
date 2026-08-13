use super::policy::{CacheFileFacts, classify_cache_path, summarize_cache_files};

#[test]
fn classifies_windows_and_unix_paths_identically() {
    assert_eq!(
        classify_cache_path("scenes\\000151e3\\shadows\\point.ktx2"),
        "shadow"
    );
    assert_eq!(
        classify_cache_path("scenes/000151e3/shadows/point.ktx2"),
        "shadow"
    );
}

#[test]
fn keeps_the_smallest_physical_copy_when_estimating_recoverable_bytes() {
    let summary = summarize_cache_files(&[
        CacheFileFacts {
            relative_path: "assets/a.glb".into(),
            logical_bytes: 10,
            allocated_bytes: 16,
            payload_id: "same".into(),
        },
        CacheFileFacts {
            relative_path: "assets/b.glb".into(),
            logical_bytes: 10,
            allocated_bytes: 24,
            payload_id: "same".into(),
        },
    ]);

    assert_eq!(summary.unique_payload_bytes, 10);
    assert_eq!(summary.duplicate_logical_bytes, 10);
    assert_eq!(summary.duplicate_allocated_bytes, 24);
}
