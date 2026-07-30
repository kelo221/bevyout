use super::*;

/// A minimal valid TES4 header record (24-byte record header, no
/// subrecords): enough for `load_plugin_chain` (no MAST subrecords, so
/// no masters to recurse into) and esplugin's header-only parse to run
/// without any real Fallout data. Mirrors the `record()` helper in
/// `prepare/tests/mod.rs`.
fn write_minimal_plugin(dir: &Path, name: &str) -> PathBuf {
    let mut bytes = b"TES4".to_vec();
    bytes.extend_from_slice(&0u32.to_le_bytes()); // data size
    bytes.extend_from_slice(&0u32.to_le_bytes()); // flags
    bytes.extend_from_slice(&0u32.to_le_bytes()); // form id
    bytes.extend_from_slice(&[0; 8]); // unknown/version control
    let path = dir.join(name);
    fs::write(&path, &bytes).unwrap();
    path
}

// T47.2: `BatchSession::new` takes the already-loaded plugin chain -- it
// never receives a path to (re)read it from, so nothing inside it, and
// nothing in `prepare_cell` (which only ever sees `&mut BatchSession`),
// can reload the chain even by accident. Exercised end-to-end against a
// temp Data directory with no BSAs and a minimal synthetic plugin (no
// real Fallout data): the loaded chain and its fingerprint must come
// through unchanged, and every per-session cache starts empty.
#[test]
fn session_construction_takes_the_already_loaded_chain_once() {
    let dir = std::env::temp_dir().join(format!(
        "bevyout-batch-session-test-{}-{}",
        std::process::id(),
        line!()
    ));
    fs::create_dir_all(&dir).unwrap();
    let plugin_path = write_minimal_plugin(&dir, "Test.esm");
    let loaded_plugins = load_plugin_chain(&plugin_path, &dir).unwrap();
    let fingerprint = content_set_fingerprint(&loaded_plugins);
    let expected_len = loaded_plugins.len();

    let session = BatchSession::new(
        &plugin_path,
        &dir,
        &dir.join("cache"),
        loaded_plugins,
        fingerprint.clone(),
    )
    .unwrap();

    assert_eq!(session.loaded_plugins.len(), expected_len);
    assert_eq!(session.fingerprint, fingerprint);
    assert!(session.archives.is_empty(), "no BSAs in the temp Data dir");
    assert!(session.audio_archives.is_empty());
    assert_eq!(session.physics_cache.lock().unwrap().accesses(), 0);
    assert_eq!(
        *session.asset_totals.lock().unwrap(),
        BatchAssetTotals::default()
    );

    fs::remove_dir_all(&dir).ok();
}
