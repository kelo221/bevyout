use super::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn temporary_cache() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("bevyout-scene-resolver-{stamp}"))
}

fn write_manifest(cache_dir: &Path) {
    let scene_dir = cache_dir.join("scenes").join("00017f37");
    fs::create_dir_all(&scene_dir).unwrap();
    fs::write(
        scene_dir.join("scene.ron"),
        r#"(
                schema_version: 8,
                asset_root: "cache",
                source_plugin: "Fallout3.esm",
                source_fingerprint: "fingerprint",
                cell: (
                    form_id: 98103,
                    editor_id: Some("SuperDuperMart"),
                    name: Some("Super-Duper Mart"),
                    interior: true,
                    ambient_rgba: (0.0, 0.0, 0.0, 0.0),
                    directional_rgba: (0.0, 0.0, 0.0, 0.0),
                ),
                placements: [],
                lights: [],
                diagnostics: [],
            )"#,
    )
    .unwrap();
}

#[test]
fn resolves_editor_id_from_cached_manifest() {
    let cache_dir = temporary_cache();
    write_manifest(&cache_dir);

    let path = resolve_cached_manifest(&cache_dir, "superdupermart").unwrap();
    assert_eq!(
        path,
        fs::canonicalize(cache_dir.join("scenes/00017f37/scene.ron")).unwrap()
    );

    fs::remove_dir_all(cache_dir).unwrap();
}

#[test]
fn reports_missing_cached_manifest_without_error_for_recovery() {
    let cache_dir = temporary_cache();
    assert_eq!(
        find_cached_manifest(&cache_dir, "RooseveltHS04").unwrap(),
        None
    );
}
