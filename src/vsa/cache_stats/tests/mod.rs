use super::model::{CacheFileReport, CacheStatsReport};
use super::policy::{CacheFileFacts, summarize_cache_files};
use super::{output, scan};
use std::{fs, path::PathBuf};

fn temporary_directory(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "bevyout-cache-stats-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn minimal_glb() -> Vec<u8> {
    let json = br#"{"asset":{"version":"2.0"}} "#;
    let total = 20 + json.len();
    let mut bytes = Vec::with_capacity(total);
    bytes.extend_from_slice(b"glTF");
    bytes.extend_from_slice(&2u32.to_le_bytes());
    bytes.extend_from_slice(&(total as u32).to_le_bytes());
    bytes.extend_from_slice(&(json.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&0x4e4f_534au32.to_le_bytes());
    bytes.extend_from_slice(json);
    bytes
}

#[test]
fn scanner_hashes_only_possible_duplicates_and_keeps_paths_sorted() {
    let root = temporary_directory("scan");
    fs::create_dir_all(root.join("assets")).unwrap();
    fs::write(root.join("assets/b.glb"), b"same").unwrap();
    fs::write(root.join("assets/a.glb"), b"same").unwrap();
    fs::write(root.join("unique.ron"), b"different-size").unwrap();

    let scan = scan::scan_cache(&root, None).unwrap();
    assert_eq!(scan.directory_count, 2);
    assert_eq!(
        scan.files
            .iter()
            .map(|file| file.relative_path.as_str())
            .collect::<Vec<_>>(),
        ["assets/a.glb", "assets/b.glb", "unique.ron"]
    );
    assert_eq!(scan.files[0].sha256, scan.files[1].sha256);
    assert!(scan.files[2].sha256.is_none());
    for file in &scan.files {
        assert!(file.allocated_bytes >= file.logical_bytes);
    }
    assert!(
        scan.files
            .iter()
            .any(|file| file.allocated_bytes != file.logical_bytes),
        "allocated bytes must come from FILE_STANDARD_INFO, not logical length"
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn manifest_set_limits_the_scanned_roots() {
    let root = temporary_directory("manifest-set");
    let scene = root.join("scenes/000151e3");
    fs::create_dir_all(&scene).unwrap();
    fs::create_dir_all(root.join("assets")).unwrap();
    fs::write(scene.join("scene.ron"), "(asset: \"assets/used.glb\")").unwrap();
    fs::write(root.join("assets/used.glb"), minimal_glb()).unwrap();
    fs::write(root.join("assets/unused.glb"), b"unused").unwrap();
    let manifest_set = root.join("sample.ron");
    fs::write(
        &manifest_set,
        "(manifests: [\"scenes/000151e3/scene.ron\"])",
    )
    .unwrap();

    let scan = scan::scan_cache(&root, Some(&manifest_set)).unwrap();
    let paths = scan
        .files
        .iter()
        .map(|file| file.relative_path.as_str())
        .collect::<Vec<_>>();
    assert!(paths.contains(&"scenes/000151e3/scene.ron"));
    assert!(paths.contains(&"assets/used.glb"));
    assert!(!paths.contains(&"assets/unused.glb"));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn manifest_asset_root_directory_is_not_expanded_as_a_reference() {
    let root = temporary_directory("manifest-asset-root");
    let scene = root.join("scenes/000151e3");
    fs::create_dir_all(&scene).unwrap();
    fs::create_dir_all(root.join("assets")).unwrap();
    fs::write(
        scene.join("scene.ron"),
        format!(
            "(asset_root: {:?}, asset: \"assets/used.glb\")",
            root.to_string_lossy()
        ),
    )
    .unwrap();
    fs::write(root.join("assets/used.glb"), minimal_glb()).unwrap();
    fs::write(root.join("assets/unused.glb"), b"unused").unwrap();
    let manifest_set = root.join("sample.ron");
    fs::write(
        &manifest_set,
        "(manifests: [\"scenes/000151e3/scene.ron\"])",
    )
    .unwrap();

    let scan = scan::scan_cache(&root, Some(&manifest_set)).unwrap();
    let paths = scan
        .files
        .iter()
        .map(|file| file.relative_path.as_str())
        .collect::<Vec<_>>();
    assert!(paths.contains(&"assets/used.glb"));
    assert!(!paths.contains(&"assets/unused.glb"));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn selected_exterior_scene_does_not_expand_to_every_indexed_cell() {
    use bevyout_core::manifest::exterior::{
        EXTERIOR_INDEX_REVISION, ExteriorCellIndexEntry, ExteriorWorldspaceIndex, GridCoordinate,
    };

    let root = temporary_directory("exterior-index-scope");
    let worldspace = root.join("worldspaces/0000003c");
    fs::create_dir_all(worldspace.join("cells")).unwrap();
    fs::write(worldspace.join("cells/00000cb8.ron"), "(selected: true)").unwrap();
    fs::write(worldspace.join("cells/00000cb9.ron"), "(unselected: true)").unwrap();
    let index = ExteriorWorldspaceIndex {
        revision: EXTERIOR_INDEX_REVISION.into(),
        content_fingerprint: "f".into(),
        worldspace_form_id: 60,
        editor_id: None,
        name: None,
        climate_form_id: None,
        coordinate_policy: Default::default(),
        cells: vec![
            ExteriorCellIndexEntry {
                cell_form_id: 0x0cb8,
                grid: GridCoordinate::new(17, -9),
                origin: [0.0; 3],
                package_path: "worldspaces/0000003c/cells/00000cb8.ron".into(),
                land_form_id: None,
                road_count: 0,
                navm_count: 0,
                persistent_reference_count: 0,
                distant_reference_count: 0,
            },
            ExteriorCellIndexEntry {
                cell_form_id: 0x0cb9,
                grid: GridCoordinate::new(18, -9),
                origin: [0.0; 3],
                package_path: "worldspaces/0000003c/cells/00000cb9.ron".into(),
                land_form_id: None,
                road_count: 0,
                navm_count: 0,
                persistent_reference_count: 0,
                distant_reference_count: 0,
            },
        ],
        weather_profiles: Vec::new(),
        persistent_references: Vec::new(),
        worldspace_lod: Vec::new(),
        diagnostics: Vec::new(),
    };
    fs::write(
        worldspace.join("index.ron"),
        ron::ser::to_string(&index).unwrap(),
    )
    .unwrap();
    let canonical_root = fs::canonicalize(&root).unwrap();
    let selected_package = fs::canonicalize(worldspace.join("cells/00000cb8.ron")).unwrap();
    let unselected_package = fs::canonicalize(worldspace.join("cells/00000cb9.ron")).unwrap();
    let mut selected = std::collections::BTreeSet::from([selected_package.clone()]);
    scan::select_worldspace_index(
        &canonical_root,
        &worldspace.join("index.ron"),
        &mut selected,
    )
    .unwrap();
    assert!(selected.contains(&selected_package));
    assert!(selected.contains(&fs::canonicalize(worldspace.join("index.ron")).unwrap()));
    assert!(!selected.contains(&unselected_package));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn selected_glb_includes_its_external_texture_object() {
    let root = temporary_directory("external-glb-texture");
    let glb = root.join("objects/glb/aa/bb/model.glb");
    let texture = root.join("objects/texture/cc/dd/image.ktx2");
    fs::create_dir_all(glb.parent().unwrap()).unwrap();
    fs::create_dir_all(texture.parent().unwrap()).unwrap();
    fs::write(&texture, b"texture").unwrap();
    let mut json =
        br#"{"asset":{"version":"2.0"},"images":[{"uri":"/objects/texture/cc/dd/image.ktx2"}]}"#
            .to_vec();
    while !json.len().is_multiple_of(4) {
        json.push(b' ');
    }
    let total = 20 + json.len();
    let mut bytes = Vec::with_capacity(total);
    bytes.extend_from_slice(b"glTF");
    bytes.extend_from_slice(&2u32.to_le_bytes());
    bytes.extend_from_slice(&(total as u32).to_le_bytes());
    bytes.extend_from_slice(&(json.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&0x4e4f_534au32.to_le_bytes());
    bytes.extend_from_slice(&json);
    fs::write(&glb, bytes).unwrap();

    let canonical_root = fs::canonicalize(&root).unwrap();
    let mut selected = std::collections::BTreeSet::from([fs::canonicalize(&glb).unwrap()]);
    scan::expand_glb_external_references(&canonical_root, &mut selected).unwrap();
    assert!(selected.contains(&fs::canonicalize(&texture).unwrap()));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn csv_output_is_stable_and_quotes_paths() {
    let root = temporary_directory("csv");
    fs::create_dir_all(&root).unwrap();
    let path = root.join("objects.csv");
    let files = vec![CacheFileReport {
        relative_path: "assets/a,b.glb".into(),
        category: "glb".into(),
        logical_bytes: 4,
        allocated_bytes: 8,
        sha256: Some("abcd".into()),
        duplicate_copies: 2,
        glb: None,
    }];
    output::write_csv(&path, &files).unwrap();
    let csv = fs::read_to_string(&path).unwrap();
    assert!(csv.contains("\"assets/a,b.glb\",glb,4,8,abcd,2"));

    let facts = vec![CacheFileFacts {
        relative_path: "assets/a,b.glb".into(),
        logical_bytes: 4,
        allocated_bytes: 8,
        payload_id: "abcd".into(),
    }];
    let _report = CacheStatsReport {
        storage: summarize_cache_files(&facts),
        ..Default::default()
    };
    fs::remove_dir_all(root).unwrap();
}
