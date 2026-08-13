use super::model::PREPARED_RECIPE_RECORD_REVISION;
use super::{
    CandidateObject, FsPreparedObjectStore, PreparedObjectKind, PreparedObjectStore,
    PreparedRecipeInputs, normalize_source_path,
};
use std::{fs, path::PathBuf, sync::Arc, thread};

fn temporary_directory(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "bevyout-cache-store-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn recipe(label: &str) -> PreparedRecipeInputs {
    PreparedRecipeInputs {
        recipe_version: 1,
        kind: PreparedObjectKind::Other,
        source_identity: normalize_source_path(&format!("Data/meshes/{label}.nif")).unwrap(),
        input_hashes: vec![crate::vsa::fingerprint(label.as_bytes())],
        converter_revision: "converter-v1".into(),
        format_policy_revision: "format-v1".into(),
        canonical_settings: vec![1, 2, 3],
    }
}

fn only_recipe_record(cache: &std::path::Path) -> PathBuf {
    let mut pending = vec![cache.join("recipes")];
    let mut records = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                pending.push(entry.path());
            } else if entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "ron")
            {
                records.push(entry.path());
            }
        }
    }
    assert_eq!(records.len(), 1);
    records.pop().unwrap()
}

#[test]
fn two_recipes_with_identical_payloads_resolve_to_one_object() {
    let root = temporary_directory("shared-payload");
    let candidates = root.join("candidates");
    fs::create_dir_all(&candidates).unwrap();
    let first = candidates.join("first.bin");
    let second = candidates.join("second.bin");
    fs::write(&first, b"identical final bytes").unwrap();
    fs::write(&second, b"identical final bytes").unwrap();
    let store = FsPreparedObjectStore::open(root.join("cache")).unwrap();

    let first_recipe = recipe("first");
    let second_recipe = recipe("second");
    let first_object = store
        .publish(
            &first_recipe,
            CandidateObject::new(PreparedObjectKind::Other, "bin", &first),
        )
        .unwrap();
    let second_object = store
        .publish(
            &second_recipe,
            CandidateObject::new(PreparedObjectKind::Other, "bin", &second),
        )
        .unwrap();

    assert_ne!(first_recipe.id(), second_recipe.id());
    assert_eq!(first_object, second_object);
    assert_eq!(
        store.resolve_recipe(&first_recipe.id()).unwrap(),
        Some(first_object.clone())
    );
    assert_eq!(
        store.resolve_recipe(&second_recipe.id()).unwrap(),
        Some(first_object.clone())
    );
    assert_eq!(
        fs::read(store.object_path(&first_object)).unwrap(),
        b"identical final bytes"
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn rejected_candidate_leaves_no_visible_object_or_recipe() {
    let root = temporary_directory("rejected");
    fs::create_dir_all(&root).unwrap();
    let malformed = root.join("bad.ktx2");
    fs::write(&malformed, b"not a ktx2 payload").unwrap();
    let store = FsPreparedObjectStore::open(root.join("cache")).unwrap();
    let mut recipe = recipe("bad-texture");
    recipe.kind = PreparedObjectKind::Texture;

    assert!(
        store
            .publish(
                &recipe,
                CandidateObject::new(PreparedObjectKind::Texture, "ktx2", &malformed),
            )
            .is_err()
    );
    assert_eq!(store.resolve_recipe(&recipe.id()).unwrap(), None);
    assert_eq!(store.object_count().unwrap(), 0);
    assert_eq!(store.temporary_file_count().unwrap(), 0);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn stale_or_undecodable_recipe_records_regenerate_as_cache_misses() {
    let root = temporary_directory("stale-recipe-record");
    fs::create_dir_all(&root).unwrap();
    let cache = root.join("cache");
    let candidate = root.join("candidate.bin");
    fs::write(&candidate, b"durable payload").unwrap();
    let store = FsPreparedObjectStore::open(&cache).unwrap();
    let recipe = recipe("stale-recipe-record");
    let object = store
        .publish(
            &recipe,
            CandidateObject::new(PreparedObjectKind::Other, "bin", &candidate),
        )
        .unwrap();
    let record_path = only_recipe_record(&cache);
    let valid = fs::read_to_string(&record_path).unwrap();
    let missing_revision = valid
        .lines()
        .filter(|line| !line.trim_start().starts_with("revision:"))
        .collect::<Vec<_>>()
        .join("\n");
    let mismatched_revision =
        valid.replace(PREPARED_RECIPE_RECORD_REVISION, "prepared-recipe-record-v0");

    for stale in [missing_revision, mismatched_revision, "not RON".into()] {
        fs::write(&record_path, stale).unwrap();
        assert_eq!(store.resolve_recipe(&recipe.id()).unwrap(), None);
        assert_eq!(
            store
                .publish(
                    &recipe,
                    CandidateObject::new(PreparedObjectKind::Other, "bin", &candidate),
                )
                .unwrap(),
            object
        );
        assert_eq!(
            store.resolve_recipe(&recipe.id()).unwrap(),
            Some(object.clone())
        );
    }

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn concurrent_publishers_share_one_durable_payload() {
    let root = temporary_directory("concurrent");
    let candidates = root.join("candidates");
    fs::create_dir_all(&candidates).unwrap();
    let mut jobs = Vec::new();
    for index in 0..8 {
        let candidate = candidates.join(format!("{index}.bin"));
        fs::write(&candidate, b"one concurrently published payload").unwrap();
        jobs.push((recipe(&format!("worker-{index}")), candidate));
    }
    let store = Arc::new(FsPreparedObjectStore::open(root.join("cache")).unwrap());
    let handles = jobs
        .into_iter()
        .map(|(recipe, candidate)| {
            let store = Arc::clone(&store);
            thread::spawn(move || {
                store
                    .publish(
                        &recipe,
                        CandidateObject::new(PreparedObjectKind::Other, "bin", candidate),
                    )
                    .unwrap()
            })
        })
        .collect::<Vec<_>>();
    let objects = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect::<Vec<_>>();

    assert!(objects.windows(2).all(|pair| pair[0] == pair[1]));
    assert_eq!(store.object_count().unwrap(), 1);
    assert_eq!(store.temporary_file_count().unwrap(), 0);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn corrupt_existing_object_is_quarantined_and_replaced() {
    let root = temporary_directory("corrupt");
    fs::create_dir_all(&root).unwrap();
    let candidate = root.join("candidate.bin");
    fs::write(&candidate, b"durable payload").unwrap();
    let store = FsPreparedObjectStore::open(root.join("cache")).unwrap();
    let recipe = recipe("corrupt");
    let object = store
        .publish(
            &recipe,
            CandidateObject::new(PreparedObjectKind::Other, "bin", &candidate),
        )
        .unwrap();
    fs::write(store.object_path(&object), b"corrupt").unwrap();

    let repaired = store
        .publish(
            &recipe,
            CandidateObject::new(PreparedObjectKind::Other, "bin", &candidate),
        )
        .unwrap();

    assert_eq!(repaired, object);
    assert_eq!(
        fs::read(store.object_path(&object)).unwrap(),
        b"durable payload"
    );
    assert_eq!(store.quarantined_file_count().unwrap(), 1);
    assert!(store.verify(&object).unwrap().valid);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn recipe_identity_changes_at_each_declared_invalidation_boundary() {
    let baseline = recipe("identity");
    let mut source = baseline.clone();
    source.input_hashes = vec![crate::vsa::fingerprint(b"changed")];
    let mut converter = baseline.clone();
    converter.converter_revision = "converter-v2".into();
    let mut format = baseline.clone();
    format.format_policy_revision = "format-v2".into();

    assert_ne!(baseline.id(), source.id());
    assert_ne!(baseline.id(), converter.id());
    assert_ne!(baseline.id(), format.id());
}

#[test]
fn source_path_normalization_is_host_independent_and_rejects_escape() {
    assert_eq!(
        normalize_source_path(r"Data\Meshes\\Clutter\Chair.NIF").unwrap(),
        "meshes/clutter/chair.nif"
    );
    assert_eq!(
        normalize_source_path("data/meshes/clutter/chair.nif").unwrap(),
        "meshes/clutter/chair.nif"
    );
    assert!(normalize_source_path("C:/Fallout/Data/mesh.nif").is_err());
    assert!(normalize_source_path("Data/meshes/../secret.nif").is_err());
    assert!(normalize_source_path("/Data/meshes/secret.nif").is_err());
}
