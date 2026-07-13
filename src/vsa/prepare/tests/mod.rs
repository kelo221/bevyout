use super::super::manifest::CellInfo;
use super::*;

#[test]
fn valid_cached_assets_are_reused_even_when_manifest_is_forced() {
    assert_eq!(
        asset_cache_decision(true, true, false),
        AssetCacheDecision::Reuse
    );
    assert_eq!(
        asset_cache_decision(true, true, true),
        AssetCacheDecision::RebuildRequested
    );
}

#[test]
fn missing_and_invalid_cached_assets_are_rebuilt() {
    assert_eq!(
        asset_cache_decision(false, false, false),
        AssetCacheDecision::BuildMissing
    );
    assert_eq!(
        asset_cache_decision(true, false, false),
        AssetCacheDecision::RebuildInvalid
    );
    assert_eq!(
        asset_cache_decision(true, false, true),
        AssetCacheDecision::RebuildInvalid
    );
}

fn plugin(name: &str, bytes: &[u8]) -> LoadedPlugin {
    LoadedPlugin {
        name: name.to_string(),
        path: PathBuf::from(name),
        bytes: bytes.to_vec(),
    }
}

#[test]
fn content_set_fingerprint_is_stable_and_changes_with_content() {
    let a = vec![plugin("Fallout3.esm", b"one"), plugin("Update.esp", b"two")];
    let a_again = vec![plugin("Fallout3.esm", b"one"), plugin("Update.esp", b"two")];
    assert_eq!(
        content_set_fingerprint(&a),
        content_set_fingerprint(&a_again)
    );

    let changed_bytes = vec![plugin("Fallout3.esm", b"one"), plugin("Update.esp", b"TWO")];
    assert_ne!(
        content_set_fingerprint(&a),
        content_set_fingerprint(&changed_bytes)
    );
}

#[test]
fn content_set_fingerprint_is_sensitive_to_plugin_order() {
    // `load_plugin_chain` always feeds plugins in master-first load
    // order, and load order changes which records win when two plugins
    // conflict. A fingerprint that changes when the same two plugins are
    // fed in a different order is therefore intended cache-invalidation
    // behavior (a different order can produce a different scene), not a
    // bug. Pin the current behavior rather than change it.
    let forward = vec![plugin("A.esm", b"aaa"), plugin("B.esp", b"bbb")];
    let reversed = vec![plugin("B.esp", b"bbb"), plugin("A.esm", b"aaa")];
    assert_ne!(
        content_set_fingerprint(&forward),
        content_set_fingerprint(&reversed)
    );
}

#[test]
fn relative_cache_path_keeps_the_root_directory_name_when_inside_root() {
    let root = Path::new("/cache/audio");
    let path = Path::new("/cache/audio/sound/test.wav");
    assert_eq!(relative_cache_path(root, path), "audio/sound/test.wav");
}

#[test]
fn relative_cache_path_falls_back_to_the_original_path_outside_root() {
    let root = Path::new("/cache/audio");
    let path = Path::new("/elsewhere/test.wav");
    assert_eq!(relative_cache_path(root, path), "/elsewhere/test.wav");
}

#[test]
fn legacy_lighting_falls_back_to_cell_ambient_and_directional_colors_only() {
    let cell = CellInfo {
        form_id: 1,
        editor_id: None,
        name: None,
        interior: true,
        ambient_rgba: [0.1, 0.2, 0.3, 1.0],
        directional_rgba: [0.4, 0.5, 0.6, 1.0],
        image_space_form_id: None,
        image_space: None,
        lighting_template_form_id: None,
        lighting_template_flags: 0,
        lighting_template: None,
        raw_lighting: None,
        effective_lighting: None,
        water_form_id: None,
        water_height: None,
    };
    let lighting = legacy_lighting(&cell);
    assert_eq!(lighting.ambient_rgba, cell.ambient_rgba);
    assert_eq!(lighting.directional_rgba, cell.directional_rgba);
    assert_eq!(lighting.fog_rgba, [0.0; 4]);
    assert_eq!(lighting.fog_near, 0.0);
    assert_eq!(lighting.fog_far, 0.0);
    assert_eq!(lighting.rotation_xy, 0);
    assert_eq!(lighting.rotation_z, 0);
    assert_eq!(lighting.fog_directional_fade, 0.0);
    assert_eq!(lighting.fog_clip_distance, 0.0);
    assert_eq!(lighting.fog_power, 1.0);
}

fn subrecord(signature: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut result = signature.to_vec();
    result.extend_from_slice(&(data.len() as u16).to_le_bytes());
    result.extend_from_slice(data);
    result
}

fn record(signature: &[u8; 4], form_id: u32, data: &[u8]) -> Vec<u8> {
    let mut result = signature.to_vec();
    result.extend_from_slice(&(data.len() as u32).to_le_bytes());
    result.extend_from_slice(&0u32.to_le_bytes()); // flags
    result.extend_from_slice(&form_id.to_le_bytes());
    result.extend_from_slice(&[0; 8]);
    result.extend_from_slice(data);
    result
}

/// `BaseRecord`/`SoundRecord` carry a module-private field
/// (`ignored_subrecords`) inside `openmw_esm4`, so they cannot be built
/// with a struct literal from this module. `parse_content_set` is the
/// only reachable seam for producing real instances of them here: build
/// a minimal synthetic ESM4 plugin (no game data, just bare records) and
/// parse it through the same production entry point `prepare()` uses.
fn fabricated_content() -> ParsedPlugin {
    let mut bytes = record(b"TES4", 0, &[]);
    bytes.extend(record(b"STAT", 1, &[]));
    let misc_value_weight = {
        let mut data = 10_i32.to_le_bytes().to_vec();
        data.extend_from_slice(&2.5_f32.to_le_bytes());
        data
    };
    bytes.extend(record(b"MISC", 2, &subrecord(b"DATA", &misc_value_weight)));
    bytes.extend(record(b"CONT", 3, &[]));
    bytes.extend(record(b"DOOR", 4, &[]));
    bytes.extend(record(b"ACTI", 5, &[]));
    bytes.extend(record(b"FURN", 6, &[]));
    bytes.extend(record(b"LIGH", 7, &[]));
    let model = subrecord(b"MODL", b"meshes\\test\\prop.nif\0");
    bytes.extend(record(b"STAT", 8, &model));
    bytes.extend(record(b"ACTI", 9, &model));

    bytes.extend(record(b"SOUN", 100, &[])); // no FNAM -> missing file
    let clip = subrecord(b"FNAM", b"sound\\test\\clip.wav\0");
    bytes.extend(record(b"SOUN", 101, &clip)); // file present, no parameters
    let looping_clip = {
        let mut data = subrecord(b"FNAM", b"sound\\test\\clip.xwm\0");
        let sndx = [5u8, 200, 253, 0, 0x50, 0x00, 0, 0, 0xB0, 0x04, 0, 0];
        data.extend(subrecord(b"SNDX", &sndx));
        data
    };
    bytes.extend(record(b"SOUN", 102, &looping_clip));

    parse_content_set(
        &[PluginSource {
            name: "Test.esm",
            bytes: &bytes,
        }],
        &parse_cell_selector("0").unwrap(),
    )
    .expect("fabricated plugin bytes must parse")
}

fn object_reference(base_form_id: u32) -> ReferenceRecord {
    ReferenceRecord {
        kind: ReferenceKind::Object,
        base_form_id,
        ..Default::default()
    }
}

#[test]
fn prepared_semantic_covers_every_supported_base_kind() {
    let content = fabricated_content();
    let bases = &content.bases;

    assert!(matches!(
        prepared_semantic(&object_reference(1), bases.get(&1)),
        PreparedSemantic::Static
    ));
    match prepared_semantic(&object_reference(2), bases.get(&2)) {
        PreparedSemantic::Pickup(pickup) => {
            assert_eq!(pickup.category, "MISC");
            assert_eq!(pickup.value, Some(10));
            assert_eq!(pickup.weight, Some(2.5));
        }
        other => panic!("expected Pickup, got {other:?}"),
    }
    assert!(matches!(
        prepared_semantic(&object_reference(3), bases.get(&3)),
        PreparedSemantic::Container
    ));
    assert!(matches!(
        prepared_semantic(&object_reference(4), bases.get(&4)),
        PreparedSemantic::Door(_)
    ));
    assert!(matches!(
        prepared_semantic(&object_reference(5), bases.get(&5)),
        PreparedSemantic::Activator
    ));
    assert!(matches!(
        prepared_semantic(&object_reference(6), bases.get(&6)),
        PreparedSemantic::Furniture
    ));
    // LIGH is a supported base record kind, but it is not one of the
    // semantic categories matched below, so an Object reference to it
    // still falls through to Unsupported even with a base present.
    assert!(matches!(
        prepared_semantic(&object_reference(7), bases.get(&7)),
        PreparedSemantic::Unsupported
    ));
}

#[test]
fn prepared_semantic_actor_kinds_ignore_the_base_record() {
    let npc = ReferenceRecord {
        kind: ReferenceKind::Npc,
        ..Default::default()
    };
    assert!(matches!(
        prepared_semantic(&npc, None),
        PreparedSemantic::Npc(_)
    ));
    let creature = ReferenceRecord {
        kind: ReferenceKind::Creature,
        ..Default::default()
    };
    assert!(matches!(
        prepared_semantic(&creature, None),
        PreparedSemantic::Creature(_)
    ));
}

#[test]
fn prepared_semantic_object_without_a_base_record_is_unsupported() {
    assert!(matches!(
        prepared_semantic(&object_reference(9999), None),
        PreparedSemantic::Unsupported
    ));
}

#[test]
fn model_static_usage_ands_the_static_flag_across_shared_models() {
    let content = fabricated_content();
    // form_id 8 is a STAT (static), form_id 9 is an ACTI (not static);
    // both share the same model, so the combined usage must be false.
    let refs = vec![object_reference(8), object_reference(9)];
    let usage = model_static_usage(&refs, &content.bases);
    assert_eq!(usage.get("meshes/test/prop.nif"), Some(&false));
}

#[test]
fn model_static_usage_is_true_when_every_reference_is_a_static_object() {
    let content = fabricated_content();
    let refs = vec![object_reference(8)];
    let usage = model_static_usage(&refs, &content.bases);
    assert_eq!(usage.get("meshes/test/prop.nif"), Some(&true));
}

#[test]
fn model_static_usage_skips_references_with_missing_base_records() {
    let refs = vec![object_reference(424_242)];
    let usage = model_static_usage(&refs, &HashMap::new());
    assert!(usage.is_empty());
}

#[test]
fn sound_descriptor_requires_a_file_path() {
    let content = fabricated_content();
    assert!(sound_descriptor(content.sounds.get(&100).unwrap()).is_none());
}

#[test]
fn sound_descriptor_defaults_when_parameters_are_absent() {
    let content = fabricated_content();
    let descriptor = sound_descriptor(content.sounds.get(&101).unwrap()).unwrap();
    assert_eq!(descriptor.source_path, "sound\\test\\clip.wav");
    assert_eq!(descriptor.flags, 0);
    assert!(!descriptor.looping);
    assert!(!descriptor.is_2d);
    assert_eq!(descriptor.static_attenuation_hundredths_db, 0);
}

#[test]
fn sound_descriptor_reads_loop_and_2d_flags_for_a_different_extension() {
    let content = fabricated_content();
    let descriptor = sound_descriptor(content.sounds.get(&102).unwrap()).unwrap();
    assert_eq!(descriptor.source_path, "sound\\test\\clip.xwm");
    assert!(descriptor.looping);
    assert!(descriptor.is_2d);
    assert_eq!(descriptor.min_attenuation, 5);
    assert_eq!(descriptor.max_attenuation, 200);
    assert_eq!(descriptor.frequency_adjustment, -3);
    assert_eq!(descriptor.static_attenuation_hundredths_db, 1200);
}
