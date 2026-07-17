use super::super::manifest::CellInfo;
use super::super::openmw_esm4::EnableParentRecord;
use super::super::recipe::{
    PreparedRecipe, PreparedRecipeItem, RecipeValidationError, validate_recipe,
};
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

#[test]
fn structural_static_paths_are_the_only_stair_support_candidates() {
    for path in [
        "architecture/megaton/interior/stairs.nif",
        "dungeons/vault/stairwell.nif",
        "landscape/rocks/cliffstep.nif",
    ] {
        assert!(is_structural_step_support(&PreparedSemantic::Static, path));
    }

    for path in [
        "furniture/chair01.nif",
        "clutter/office/shelf01.nif",
        "vehicles/car01.nif",
        "terminals/terminal01.nif",
    ] {
        assert!(!is_structural_step_support(&PreparedSemantic::Static, path));
    }
}

#[test]
fn non_static_semantics_never_become_stair_support() {
    for semantic in [
        PreparedSemantic::Furniture,
        PreparedSemantic::Door(PreparedDoor {
            lock_level: None,
            key_form_id: None,
            destination: None,
        }),
        PreparedSemantic::Activator,
        PreparedSemantic::Container,
        PreparedSemantic::Unsupported,
    ] {
        assert!(!is_structural_step_support(
            &semantic,
            "architecture/megaton/interior/stairs.nif"
        ));
    }
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
        grid: None,
        worldspace_form_id: None,
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
    record_with_flags(signature, form_id, 0, data)
}

fn record_with_flags(signature: &[u8; 4], form_id: u32, flags: u32, data: &[u8]) -> Vec<u8> {
    let mut result = signature.to_vec();
    result.extend_from_slice(&(data.len() as u32).to_le_bytes());
    result.extend_from_slice(&flags.to_le_bytes());
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
    let mut clip = subrecord(b"EDID", b"DrsTestOpen\0");
    clip.extend(subrecord(b"FNAM", b"sound\\test\\clip.wav\0"));
    bytes.extend(record(b"SOUN", 101, &clip)); // file present, no parameters
    let looping_clip = {
        let mut data = subrecord(b"EDID", b"DrsTestClose\0");
        data.extend(subrecord(b"FNAM", b"sound\\test\\clip.xwm\0"));
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

// Issue #120 (F119.1): a source-authored dead NPC reference (the ACHR
// record-header "starts dead" flag, `RECORD_STARTS_DEAD`) prepares as a
// lootable `PreparedSemantic::Corpse`, not `Npc`. A living actor -- the
// same reference with the flag unset -- must keep preparing as `Npc`.
#[test]
fn prepared_semantic_a_source_dead_npc_reference_prepares_as_a_corpse() {
    let dead = ReferenceRecord {
        kind: ReferenceKind::Npc,
        flags: RECORD_STARTS_DEAD,
        ..Default::default()
    };
    assert!(matches!(
        prepared_semantic(&dead, None),
        PreparedSemantic::Corpse
    ));
}

#[test]
fn prepared_semantic_a_living_npc_reference_still_prepares_as_npc() {
    let living = ReferenceRecord {
        kind: ReferenceKind::Npc,
        flags: 0,
        ..Default::default()
    };
    assert!(matches!(
        prepared_semantic(&living, None),
        PreparedSemantic::Npc(_)
    ));
}

// The "starts dead" bit is documented only for `ACHR` (NPC) references;
// `ACRE` (creature) references reusing the same header-flags bit for an
// undocumented meaning must not be reinterpreted as a corpse.
#[test]
fn prepared_semantic_the_starts_dead_bit_is_not_read_for_creature_references() {
    let creature = ReferenceRecord {
        kind: ReferenceKind::Creature,
        flags: RECORD_STARTS_DEAD,
        ..Default::default()
    };
    assert!(matches!(
        prepared_semantic(&creature, None),
        PreparedSemantic::Creature(_)
    ));
}

/// `BaseRecord` carries a module-private field (`ignored_subrecords`)
/// inside `openmw_esm4`, so -- like `fabricated_content` above -- it
/// cannot be built with a struct literal from this module. Builds a
/// minimal synthetic `NPC_` base (editor ID, display name, one `CNTO`
/// inventory item) plus its `MISC` item base, and parses them through the
/// real `parse_content_set` entry point.
fn fabricated_dead_actor_content(npc_form_id: u32, item_form_id: u32) -> ParsedPlugin {
    let mut bytes = record(b"TES4", 0, &[]);
    bytes.extend(record(
        b"MISC",
        item_form_id,
        &subrecord(b"FULL", b"Corpse Loot Item\0"),
    ));
    let mut npc_data = subrecord(b"EDID", b"CG04DeadOldLady\0");
    npc_data.extend(subrecord(b"FULL", b"Old Lady\0"));
    let mut cnto = item_form_id.to_le_bytes().to_vec();
    cnto.extend_from_slice(&3_u32.to_le_bytes());
    npc_data.extend(subrecord(b"CNTO", &cnto));
    bytes.extend(record(b"NPC_", npc_form_id, &npc_data));

    parse_content_set(
        &[PluginSource {
            name: "DeadActor.esm",
            bytes: &bytes,
        }],
        &parse_cell_selector("0").unwrap(),
    )
    .expect("fabricated dead-actor plugin bytes must parse")
}

// F119.2: the corpse's stable reference, transform, display identity, and
// inventory come straight from the same generic `prepared_placement`
// fields every other semantic uses -- nothing corpse-specific is dropped
// once classification flips to `Corpse`.
#[test]
fn prepared_placement_preserves_identity_transform_and_inventory_for_a_dead_actor() {
    let npc_form_id = 0x0001_2345;
    let item_form_id = 0x0000_2222;
    let content = fabricated_dead_actor_content(npc_form_id, item_form_id);
    let reference = ReferenceRecord {
        kind: ReferenceKind::Npc,
        form_id: 0x0005_4398,
        base_form_id: npc_form_id,
        flags: RECORD_STARTS_DEAD,
        position: [12.0, -4.5, 7.0],
        rotation: [0.0, 0.0, std::f32::consts::FRAC_PI_2],
        scale: 1.0,
        count: 1,
        initially_enabled: true,
        ..Default::default()
    };
    let base = content.bases.get(&npc_form_id);
    let placement = prepared_placement(&reference, base, None, None, &content.bases);

    assert!(matches!(placement.semantic, PreparedSemantic::Corpse));
    assert_eq!(placement.reference_form_id, reference.form_id);
    assert_eq!(placement.base_form_id, reference.base_form_id);
    // `placement_transform` applies the same FO3-units-to-metres/axis
    // conversion every other semantic gets; a corpse must not bypass it.
    assert_eq!(placement.translation, placement_transform(&reference).0);
    assert_eq!(placement.display_name.as_deref(), Some("Old Lady"));
    assert_eq!(placement.editor_id.as_deref(), Some("CG04DeadOldLady"));
    assert_eq!(placement.inventory.len(), 1);
    assert_eq!(placement.inventory[0].base_form_id, item_form_id);
    assert_eq!(placement.inventory[0].count, 3);
    assert_eq!(
        placement.inventory[0].display_name.as_deref(),
        Some("Corpse Loot Item")
    );
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

#[test]
fn sound_editor_ids_resolve_case_insensitively_and_deterministically() {
    let content = fabricated_content();
    assert_eq!(
        sound_form_ids_by_editor_id(&content, "drstestopen"),
        vec![101]
    );
    assert_eq!(
        sound_form_ids_by_editor_id(&content, "DRSTESTCLOSE"),
        vec![102]
    );
    assert!(sound_form_ids_by_editor_id(&content, "missing").is_empty());
}

#[test]
fn additional_audio_staging_is_deduplicated_and_form_id_sorted() {
    let content = fabricated_content();
    let mut diagnostics = Vec::new();
    let root = std::env::temp_dir().join(format!(
        "bevyout-container-audio-stage-{}",
        std::process::id()
    ));
    let clips = stage_audio_clips(
        &root,
        &[],
        &content,
        &mut diagnostics,
        &root.join("audio"),
        [102, 101, 102],
    )
    .unwrap();
    assert_eq!(
        clips.iter().map(|clip| clip.form_id).collect::<Vec<_>>(),
        vec![101, 102]
    );
    assert_eq!(diagnostics.len(), 2);
}

#[test]
fn glb_container_cues_fill_only_missing_record_audio() {
    fn glb_with_cues(cues: &[AnimationSoundCue]) -> Vec<u8> {
        let encoded = serde_json::to_string(cues).unwrap();
        let document = serde_json::json!({
            "nodes": [{"extras": {"bevyout_animation_sound_cues": encoded}}]
        });
        let mut json = serde_json::to_vec(&document).unwrap();
        while !json.len().is_multiple_of(4) {
            json.push(b' ');
        }
        let total_length = 20 + json.len();
        let mut bytes = Vec::with_capacity(total_length);
        bytes.extend_from_slice(b"glTF");
        bytes.extend_from_slice(&2_u32.to_le_bytes());
        bytes.extend_from_slice(&(total_length as u32).to_le_bytes());
        bytes.extend_from_slice(&(json.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&0x4e4f534a_u32.to_le_bytes());
        bytes.extend_from_slice(&json);
        bytes
    }

    let content = fabricated_content();
    let root = std::env::temp_dir().join(format!(
        "bevyout-container-audio-glb-{}",
        std::process::id()
    ));
    fs::create_dir_all(&root).unwrap();
    let asset_path = "container.glb";
    fs::write(
        root.join(asset_path),
        glb_with_cues(&[
            AnimationSoundCue {
                sequence: "Open".into(),
                time: 0.01,
                editor_id: "drstestopen".into(),
            },
            AnimationSoundCue {
                sequence: "Close".into(),
                time: 0.01,
                editor_id: "DRSTESTCLOSE".into(),
            },
        ]),
    )
    .unwrap();
    let reference = object_reference(3);
    let mut placement = prepared_placement(
        &reference,
        content.bases.get(&3),
        Some(asset_path.into()),
        None,
        &content.bases,
    );
    placement.audio.open_sound_form_id = Some(777);
    let mut placements = vec![placement];
    let mut diagnostics = Vec::new();
    let additional =
        apply_container_animation_audio(&root, &content, &mut placements, &mut diagnostics);
    assert_eq!(placements[0].audio.open_sound_form_id, Some(777));
    assert_eq!(placements[0].audio.close_sound_form_id, Some(102));
    assert_eq!(additional, vec![102]);
    assert!(diagnostics.is_empty());
    let _ = fs::remove_dir_all(root);
}

#[test]
fn non_static_physics_classifications_remove_stair_support() {
    assert!(retain_static_step_support(
        true,
        PreparedPhysicsClassification::Static
    ));
    assert!(!retain_static_step_support(
        true,
        PreparedPhysicsClassification::Kinematic
    ));
    assert!(!retain_static_step_support(
        true,
        PreparedPhysicsClassification::Dynamic
    ));
    assert!(!retain_static_step_support(
        false,
        PreparedPhysicsClassification::Static
    ));
}

// --- Issue #38: PreparedRuntimeMutability classification -----------------
//
// Fixtures below are synthetic ReferenceRecord/PreparedSemantic values
// built directly (no real ESM bytes, no Bethesda-derived data), matching
// the level classify_runtime_mutability actually operates at.

fn reference_fixture(form_id: u32, enable_parent: Option<EnableParentRecord>) -> ReferenceRecord {
    ReferenceRecord {
        form_id,
        enable_parent,
        ..Default::default()
    }
}

// T38.1: synthetic record fixtures producing each of the four
// classifications; assert one placement of each class after prepare-time
// classification.
#[test]
fn synthetic_records_classify_into_all_four_mutability_classes() {
    // Immutable: plain static scenery, no enable-parent, no error.
    let statics = reference_fixture(0x10, None);
    assert_eq!(
        classify_runtime_mutability(&statics, &PreparedSemantic::Static, false),
        (PreparedRuntimeMutability::Immutable, None)
    );

    // EnableGroup: reference toggled by an enable-parent chain whose root
    // resolved successfully.
    let mut enabled_child = reference_fixture(
        0x20,
        Some(EnableParentRecord {
            parent_reference_form_id: 0x21,
            flags: 0,
        }),
    );
    enabled_child.enable_root_form_id = Some(0x21);
    assert_eq!(
        classify_runtime_mutability(&enabled_child, &PreparedSemantic::Static, false),
        (PreparedRuntimeMutability::EnableGroup, Some(0x21))
    );

    // ScriptAddressable: a known script-reachable record kind (a door).
    let door = reference_fixture(0x30, None);
    let door_semantic = PreparedSemantic::Door(PreparedDoor {
        lock_level: None,
        key_form_id: None,
        destination: None,
    });
    assert_eq!(
        classify_runtime_mutability(&door, &door_semantic, false),
        (PreparedRuntimeMutability::ScriptAddressable, None)
    );

    // Unknown: an unresolved placement (error present) must never be
    // silently treated as Immutable.
    let broken = reference_fixture(0x40, None);
    assert_eq!(
        classify_runtime_mutability(&broken, &PreparedSemantic::Static, true),
        (PreparedRuntimeMutability::Unknown, None)
    );
}

// T38.2: enable-parent fixture: children (however deep in the chain) share
// the same resolved parent root FormID in their EnableGroup classification.
#[test]
fn enable_parent_children_share_the_resolved_chain_root() {
    // root (0x1) <- mid (0x2, enabled by root) <- leaf (0x3, enabled by mid)
    let mut mid = reference_fixture(
        0x2,
        Some(EnableParentRecord {
            parent_reference_form_id: 0x1,
            flags: 0,
        }),
    );
    mid.enable_root_form_id = Some(0x1);
    let mut leaf = reference_fixture(
        0x3,
        Some(EnableParentRecord {
            parent_reference_form_id: 0x2,
            flags: 0,
        }),
    );
    leaf.enable_root_form_id = Some(0x1);

    let mid_result = classify_runtime_mutability(&mid, &PreparedSemantic::Static, false);
    let leaf_result = classify_runtime_mutability(&leaf, &PreparedSemantic::Static, false);
    assert_eq!(
        mid_result,
        (PreparedRuntimeMutability::EnableGroup, Some(0x1))
    );
    assert_eq!(
        leaf_result,
        (PreparedRuntimeMutability::EnableGroup, Some(0x1))
    );
    assert_eq!(mid_result.1, leaf_result.1, "children must share one root");

    // An enable-parent reference whose chain root could not be resolved
    // (cycle/unresolved XESP) is conservatively Unknown, never a guessed
    // EnableGroup root.
    let unresolved = reference_fixture(
        0x4,
        Some(EnableParentRecord {
            parent_reference_form_id: 0x999,
            flags: 0,
        }),
    );
    assert_eq!(
        classify_runtime_mutability(&unresolved, &PreparedSemantic::Static, false),
        (PreparedRuntimeMutability::Unknown, None)
    );
}

// T38.3: ambiguous fixture (record type not in the known-safe set) ->
// Unknown, never Immutable.
#[test]
fn unsupported_record_kind_classifies_as_unknown_not_immutable() {
    let reference = reference_fixture(0x50, None);
    let (mutability, root) =
        classify_runtime_mutability(&reference, &PreparedSemantic::Unsupported, false);
    assert_eq!(mutability, PreparedRuntimeMutability::Unknown);
    assert_eq!(root, None);
    assert_ne!(mutability, PreparedRuntimeMutability::Immutable);
}

// T38.6: classifying the same fixture set twice yields identical results,
// including the derived per-class summary counts (F38.4).
#[test]
fn classification_and_summary_are_deterministic_across_repeated_runs() {
    struct MutabilityCase {
        form_id: u32,
        enable_parent: Option<EnableParentRecord>,
        enable_root: Option<u32>,
        semantic: PreparedSemantic,
        has_error: bool,
    }

    fn fixture_set() -> Vec<PreparedPlacement> {
        let cases = [
            MutabilityCase {
                form_id: 0x1,
                enable_parent: None,
                enable_root: None,
                semantic: PreparedSemantic::Static,
                has_error: false,
            },
            MutabilityCase {
                form_id: 0x2,
                enable_parent: Some(EnableParentRecord {
                    parent_reference_form_id: 0x1,
                    flags: 0,
                }),
                enable_root: Some(0x1),
                semantic: PreparedSemantic::Static,
                has_error: false,
            },
            MutabilityCase {
                form_id: 0x3,
                enable_parent: None,
                enable_root: None,
                semantic: PreparedSemantic::Activator,
                has_error: false,
            },
            MutabilityCase {
                form_id: 0x4,
                enable_parent: None,
                enable_root: None,
                semantic: PreparedSemantic::Unsupported,
                has_error: false,
            },
            MutabilityCase {
                form_id: 0x5,
                enable_parent: None,
                enable_root: None,
                semantic: PreparedSemantic::Static,
                has_error: true,
            },
        ];
        cases
            .into_iter()
            .map(|case| {
                let mut reference = reference_fixture(case.form_id, case.enable_parent);
                reference.enable_root_form_id = case.enable_root;
                let (mutability, mutability_root_form_id) =
                    classify_runtime_mutability(&reference, &case.semantic, case.has_error);
                PreparedPlacement {
                    reference_form_id: case.form_id,
                    base_form_id: 0,
                    asset_path: None,
                    translation: [0.0; 3],
                    rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                    scale: 1.0,
                    error: case.has_error.then(|| "synthetic error".to_string()),
                    physics_asset_path: None,
                    physics_source: None,
                    physics_classification: PreparedPhysicsClassification::Static,
                    step_support: false,
                    mutability,
                    mutability_root_form_id,
                    reference_kind: "REFR".into(),
                    base_kind: "STAT".into(),
                    editor_id: None,
                    display_name: None,
                    count: 1,
                    semantic: case.semantic,
                    initially_enabled: true,
                    enable_parent: None,
                    owner_form_id: None,
                    owner_faction_rank: None,
                    inventory: Vec::new(),
                    audio: Default::default(),
                    ao_mode: "ao-none".into(),
                }
            })
            .collect()
    }

    let first = fixture_set();
    let second = fixture_set();
    let first_classes = first
        .iter()
        .map(|p| (p.reference_form_id, p.mutability, p.mutability_root_form_id))
        .collect::<Vec<_>>();
    let second_classes = second
        .iter()
        .map(|p| (p.reference_form_id, p.mutability, p.mutability_root_form_id))
        .collect::<Vec<_>>();
    assert_eq!(first_classes, second_classes);

    let first_summary = summarize_mutability(&first);
    let second_summary = summarize_mutability(&second);
    assert_eq!(first_summary, second_summary);
    assert_eq!(
        first_summary,
        PreparedMutabilitySummary {
            immutable: 1,
            enable_group: 1,
            script_addressable: 1,
            unknown: 2,
        }
    );
}

// --- Issue #81: quest-item header flag flows into the item catalog ------

#[test]
fn quest_item_header_flag_flows_from_plugin_bytes_into_the_item_catalog() {
    const QUEST_ITEM_HEADER_FLAG: u32 = 0x0000_0400;

    let misc_value_weight = {
        let mut data = 5_i32.to_le_bytes().to_vec();
        data.extend_from_slice(&1.0_f32.to_le_bytes());
        data
    };
    let mut bytes = record(b"TES4", 0, &[]);
    bytes.extend(record_with_flags(
        b"MISC",
        1,
        QUEST_ITEM_HEADER_FLAG,
        &subrecord(b"DATA", &misc_value_weight),
    ));
    bytes.extend(record(b"MISC", 2, &subrecord(b"DATA", &misc_value_weight)));

    let parsed = parse_content_set(
        &[PluginSource {
            name: "Test.esm",
            bytes: &bytes,
        }],
        &parse_cell_selector("0").unwrap(),
    )
    .expect("synthetic quest-item plugin bytes must parse");

    let catalog = build_item_catalog(&parsed.bases, &HashMap::new(), &[], &HashMap::new(), "test");
    let quest_item = catalog
        .items
        .iter()
        .find(|item| item.base_form_id == 1)
        .expect("quest MISC item must be present in the catalog");
    let ordinary_item = catalog
        .items
        .iter()
        .find(|item| item.base_form_id == 2)
        .expect("ordinary MISC item must be present in the catalog");
    assert!(quest_item.quest_item);
    assert!(!ordinary_item.quest_item);
}

// --- Issue #117: prepared recipes and pure validation --------------------

fn recipe_item(item_form_id: u32, quantity: i32, order: u32) -> RecipeItemRecord {
    RecipeItemRecord {
        item_form_id,
        quantity,
        order,
    }
}

fn recipe_record(
    form_id: u32,
    ingredients: Vec<RecipeItemRecord>,
    outputs: Vec<RecipeItemRecord>,
) -> RecipeRecord {
    RecipeRecord {
        form_id,
        record_flags: 0,
        editor_id: Some(format!("Recipe{form_id:08x}")),
        name: Some(format!("Recipe {form_id:08x}")),
        skill: 25,
        level: 4,
        category_form_id: Some(0x500),
        sub_category_form_id: Some(0x501),
        ingredients,
        outputs,
        conditions: Vec::new(),
        ignored_subrecords: Vec::new(),
    }
}

fn recipe_bases(ids: &[u32]) -> HashMap<u32, BaseRecord> {
    ids.iter()
        .copied()
        .map(|form_id| {
            let mut base = BaseRecord::default();
            base.kind = "MISC".into();
            base.name = Some(format!("Item {form_id:08x}"));
            (form_id, base)
        })
        .collect()
}

#[test]
fn recipe_catalog_sorts_records_and_items_without_mutating_source_order() {
    let bases = recipe_bases(&[0x10, 0x20, 0x30]);
    let mut recipes = HashMap::new();
    recipes.insert(
        0x20,
        recipe_record(
            0x20,
            vec![recipe_item(0x30, 1, 0), recipe_item(0x10, 2, 1)],
            vec![recipe_item(0x30, 1, 0)],
        ),
    );
    recipes.insert(
        0x10,
        recipe_record(
            0x10,
            vec![recipe_item(0x20, 1, 0)],
            vec![recipe_item(0x30, 1, 0)],
        ),
    );

    let built = build_recipe_catalog(&recipes, &bases, "content-fingerprint");
    assert!(built.invalid.is_empty());
    assert_eq!(
        built
            .catalog
            .recipes
            .iter()
            .map(|recipe| recipe.form_id)
            .collect::<Vec<_>>(),
        vec![0x10, 0x20]
    );
    assert_eq!(
        built.catalog.recipes[1]
            .ingredients
            .iter()
            .map(|item| item.item_form_id)
            .collect::<Vec<_>>(),
        vec![0x10, 0x30]
    );
    assert_eq!(
        recipes[&0x20]
            .ingredients
            .iter()
            .map(|item| item.item_form_id)
            .collect::<Vec<_>>(),
        vec![0x30, 0x10],
        "catalog ordering must not mutate parsed records"
    );
}

#[test]
fn recipe_catalog_serialization_is_independent_of_hash_map_insertion_order() {
    let bases = recipe_bases(&[0x10, 0x20, 0x30]);
    let first_recipe = recipe_record(
        0x20,
        vec![recipe_item(0x30, 1, 0), recipe_item(0x10, 2, 1)],
        vec![recipe_item(0x30, 1, 0)],
    );
    let second_recipe = recipe_record(
        0x10,
        vec![recipe_item(0x20, 1, 0)],
        vec![recipe_item(0x30, 1, 0)],
    );
    let first = HashMap::from([(0x20, first_recipe.clone()), (0x10, second_recipe.clone())]);
    let second = HashMap::from([(0x10, second_recipe), (0x20, first_recipe)]);
    let first_catalog = build_recipe_catalog(&first, &bases, "content-fingerprint");
    let second_catalog = build_recipe_catalog(&second, &bases, "content-fingerprint");
    let ron_config = ron::ser::PrettyConfig::default();

    assert_eq!(
        ron::ser::to_string_pretty(&first_catalog.catalog, ron_config.clone()).unwrap(),
        ron::ser::to_string_pretty(&second_catalog.catalog, ron_config).unwrap()
    );
}

#[test]
fn invalid_recipe_records_are_diagnosed_and_excluded_as_a_unit() {
    let bases = recipe_bases(&[0x10, 0x20]);
    let mut recipes = HashMap::new();
    recipes.insert(
        0x30,
        recipe_record(0x30, vec![recipe_item(0x10, 1, 0)], Vec::new()),
    );
    recipes.insert(
        0x31,
        recipe_record(0x31, Vec::new(), vec![recipe_item(0x20, 1, 0)]),
    );
    recipes.insert(
        0x32,
        recipe_record(
            0x32,
            vec![recipe_item(0x10, 1, 0), recipe_item(0x10, 2, 1)],
            vec![recipe_item(0x20, 1, 0)],
        ),
    );
    recipes.insert(
        0x33,
        recipe_record(
            0x33,
            vec![recipe_item(0x10, 0, 0)],
            vec![recipe_item(0x20, 1, 0)],
        ),
    );
    recipes.insert(
        0x34,
        recipe_record(
            0x34,
            vec![recipe_item(0x999, 1, 0)],
            vec![recipe_item(0x20, 1, 0)],
        ),
    );

    let built = build_recipe_catalog(&recipes, &bases, "content-fingerprint");
    assert!(built.catalog.recipes.is_empty());
    assert_eq!(built.invalid.len(), 5);
    assert!(built.invalid[0].message.contains("no outputs"));
    assert!(built.invalid[1].message.contains("no ingredients"));
    assert!(built.invalid[2].message.contains("duplicate ingredient"));
    assert!(built.invalid[3].message.contains("non-positive quantity"));
    assert!(built.invalid[4].message.contains("missing ingredient item"));
}

#[test]
fn recipe_catalog_write_is_content_fingerprinted_and_reused() {
    let root = std::env::temp_dir().join(format!(
        "bevyout-recipe-catalog-{}-{}",
        std::process::id(),
        line!()
    ));
    let bases = recipe_bases(&[0x10, 0x20]);
    let recipes = HashMap::from([(
        0x30,
        recipe_record(
            0x30,
            vec![recipe_item(0x10, 1, 0)],
            vec![recipe_item(0x20, 1, 0)],
        ),
    )]);
    let catalog = build_recipe_catalog(&recipes, &bases, "content-fingerprint");

    let first = write_recipe_catalog(&root, &catalog.catalog).unwrap();
    let second = write_recipe_catalog(&root, &catalog.catalog).unwrap();
    assert!(!first.reused);
    assert!(second.reused);
    assert_eq!(first.relative_path, second.relative_path);
    assert_eq!(first.hash, second.hash);
    assert!(root.join(&first.relative_path).is_file());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn recipe_validation_rejects_non_positive_quantities_without_partial_state() {
    let bases = recipe_bases(&[0x10, 0x20]);
    let recipe = PreparedRecipe {
        form_id: 0x30,
        record_flags: 0,
        editor_id: None,
        display_name: None,
        skill: 0,
        level: 0,
        category_form_id: None,
        sub_category_form_id: None,
        ingredients: vec![PreparedRecipeItem {
            item_form_id: 0x10,
            quantity: -1,
            order: 0,
        }],
        outputs: vec![PreparedRecipeItem {
            item_form_id: 0x20,
            quantity: 1,
            order: 0,
        }],
        conditions: Vec::new(),
    };
    let available = bases
        .keys()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let error = validate_recipe(&recipe, &available).unwrap_err();
    assert!(matches!(
        error,
        RecipeValidationError::NonPositiveQuantity { .. }
    ));
    assert_eq!(recipe.ingredients[0].quantity, -1);
}
