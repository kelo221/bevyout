use super::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::Write;

#[test]
fn finds_length_adjacent_texture_names_in_nif_bytes() {
    let references = texture_references(b"textures\\clutter\\machine\\panel.dds4");
    assert!(references.contains(&"textures/clutter/machine/panel.dds".to_string()));
}

#[test]
fn keeps_texture_paths_with_spaces() {
    let references = texture_references(b"textures\\dungeons\\wasteland homes\\Wastehome01.dds3");
    assert!(references.contains(&"textures/dungeons/wasteland homes/wastehome01.dds".to_string()));
}

#[test]
fn content_addressed_glb_names_are_stable_and_revision_sensitive() {
    let first = content_addressed_glb_name("converter-v1", b"nif-bytes");
    assert_eq!(
        first,
        content_addressed_glb_name("converter-v1", b"nif-bytes")
    );
    assert_ne!(
        first,
        content_addressed_glb_name("converter-v2", b"nif-bytes")
    );
    assert_ne!(
        first,
        content_addressed_glb_name("converter-v1", b"changed-nif")
    );
    assert!(first.ends_with(".glb"));
}

#[test]
fn truncated_cached_glb_is_rejected_without_panicking() {
    let path =
        std::env::temp_dir().join(format!("bevyout-invalid-cache-{}.glb", std::process::id()));
    let mut bytes = vec![0_u8; 20];
    bytes[0..4].copy_from_slice(b"glTF");
    bytes[12..16].copy_from_slice(&1024_u32.to_le_bytes());
    std::fs::write(&path, bytes).unwrap();
    assert!(validate_glb_images(&path).is_err());
    let _ = std::fs::remove_file(path);
}

#[test]
fn cache_pair_rebuilds_when_sidecar_is_missing_or_invalid() {
    let stem = format!("bevyout-cache-pair-{}", std::process::id());
    let glb = std::env::temp_dir().join(format!("{stem}.glb"));
    let physics = std::env::temp_dir().join(format!("{stem}.physics.json.gz"));
    let mut glb_bytes = vec![0_u8; 24];
    glb_bytes[0..4].copy_from_slice(b"glTF");
    glb_bytes[4..8].copy_from_slice(&2_u32.to_le_bytes());
    glb_bytes[8..12].copy_from_slice(&24_u32.to_le_bytes());
    glb_bytes[12..16].copy_from_slice(&4_u32.to_le_bytes());
    glb_bytes[16..20].copy_from_slice(&0x4e4f534a_u32.to_le_bytes());
    glb_bytes[20..24].copy_from_slice(b"{}  ");
    fs::write(&glb, glb_bytes).unwrap();

    assert!(validate_asset_cache_pair(&glb, &physics).is_err());
    let valid = br#"{"schema_version":1,"source":"GeneratedRender","bodies":[]}"#;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(valid).unwrap();
    fs::write(&physics, encoder.finish().unwrap()).unwrap();
    validate_asset_cache_pair(&glb, &physics).unwrap();

    fs::write(&physics, b"not gzip").unwrap();
    assert!(validate_asset_cache_pair(&glb, &physics).is_err());
    let _ = fs::remove_file(glb);
    let _ = fs::remove_file(physics);
}

#[test]
fn static_assets_use_quick_ao_and_dynamic_assets_preserve_materials() {
    assert_eq!(asset_conversion(true), AssetConversion::QuickAo);
    assert_eq!(asset_conversion(false), AssetConversion::Preserve);
}

#[test]
fn blender_job_json_carries_quick_ao_profile() {
    let json = blender_jobs_json(&[BlenderAssetJob {
        input: PathBuf::from("C:\\staging\\mesh.nif"),
        output: PathBuf::from("C:\\cache\\mesh.glb"),
        physics_output: PathBuf::from("C:\\cache\\mesh.physics.json.gz"),
        model: "architecture/test.nif".into(),
        conversion: AssetConversion::QuickAo,
    }]);
    assert!(json.contains("\"conversion\":\"ao-quick-v1\""));
    assert!(json.contains("mesh.physics.json.gz"));
    assert!(json.contains("architecture/test.nif"));
}
