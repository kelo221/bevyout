use super::*;

fn synthetic_animation_glb(input_end: f32) -> Vec<u8> {
    let document = serde_json::json!({
        "asset": {"version": "2.0"},
        "scene": 0,
        "scenes": [{"nodes": [0]}],
        "nodes": [
            {"name": "Skeleton", "children": [1]},
            {"name": "Bip01 Spine"}
        ],
        "buffers": [{"byteLength": 32}],
        "bufferViews": [
            {"buffer": 0, "byteOffset": 0, "byteLength": 8},
            {"buffer": 0, "byteOffset": 8, "byteLength": 24}
        ],
        "accessors": [
            {"bufferView": 0, "componentType": 5126, "count": 2, "type": "SCALAR", "min": [0.0], "max": [1.0]},
            {"bufferView": 1, "componentType": 5126, "count": 2, "type": "VEC3"}
        ],
        "animations": [{
            "name": "idle",
            "samplers": [{"input": 0, "output": 1, "interpolation": "LINEAR"}],
            "channels": [{"sampler": 0, "target": {"node": 1, "path": "translation"}}]
        }]
    });
    let mut json = serde_json::to_vec(&document).unwrap();
    while !json.len().is_multiple_of(4) {
        json.push(b' ');
    }
    let mut binary = Vec::new();
    for value in [0.0_f32, input_end, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0] {
        binary.extend_from_slice(&value.to_le_bytes());
    }
    let total_length = 12 + 8 + json.len() + 8 + binary.len();
    let mut glb = Vec::new();
    glb.extend_from_slice(b"glTF");
    glb.extend_from_slice(&2_u32.to_le_bytes());
    glb.extend_from_slice(&(total_length as u32).to_le_bytes());
    glb.extend_from_slice(&(json.len() as u32).to_le_bytes());
    glb.extend_from_slice(b"JSON");
    glb.extend_from_slice(&json);
    glb.extend_from_slice(&(binary.len() as u32).to_le_bytes());
    glb.extend_from_slice(b"BIN\0");
    glb.extend_from_slice(&binary);
    glb
}

#[test]
fn fingerprint_changes_with_skeleton_clip_bytes_and_policy() {
    let clips = [("idle", "meshes/actors/idle.kf", b"clip".as_slice())];
    let baseline =
        actor_animation_pack_fingerprint("v1", "meshes/actors/skeleton.nif", b"skeleton", &clips);
    assert_ne!(
        baseline,
        actor_animation_pack_fingerprint("v2", "meshes/actors/skeleton.nif", b"skeleton", &clips,)
    );
    assert_ne!(
        baseline,
        actor_animation_pack_fingerprint("v1", "meshes/actors/skeleton.nif", b"changed", &clips,)
    );
    let changed = [("idle", "meshes/actors/idle.kf", b"changed".as_slice())];
    assert_ne!(
        baseline,
        actor_animation_pack_fingerprint("v1", "meshes/actors/skeleton.nif", b"skeleton", &changed,)
    );
}

#[test]
fn synthetic_skeleton_clip_pack_validates_channels_targets_and_times() {
    let path =
        std::env::temp_dir().join(format!("bevyout-animation-pack-{}.glb", std::process::id()));
    fs::write(&path, synthetic_animation_glb(1.0)).unwrap();
    validate_actor_animation_glb(&path, &HashSet::from(["idle".to_owned()])).unwrap();
    fs::write(&path, synthetic_animation_glb(f32::NAN)).unwrap();
    assert!(
        validate_actor_animation_glb(&path, &HashSet::from(["idle".to_owned()]))
            .unwrap_err()
            .to_string()
            .contains("non-finite")
    );
    let _ = fs::remove_file(path);
}

#[test]
fn converter_report_retains_source_sequence_contract() {
    let report: ActorAnimationPackReport = serde_json::from_value(serde_json::json!({
        "revision": "v1",
        "skeleton_path": "meshes/characters/_male/skeleton.nif",
        "clips": [{
            "name": "equip",
            "source_path": "meshes/characters/_male/1hpequip.kf",
            "success": true,
            "duration_seconds": 0.5,
            "source_sequence_name": "Equip",
            "source_start_seconds": 0.25,
            "source_end_seconds": 0.75,
            "source_frequency": 1.0,
            "source_phase": 0.0,
            "loop_mode": "clamp",
            "root_motion_policy": "preserve_authored",
            "accumulation_root": "Bip01",
            "animated_channel_count": 3,
            "animated_target_count": 1,
            "required_targets": ["Bip01 R Hand", "Weapon"],
            "animated_targets": ["Bip01 R Hand"],
            "missing_targets": ["Weapon"],
            "controller_types": ["NiTransformController"],
            "interpolator_types": ["NiTransformInterpolator"],
            "text_keys": [{"time_seconds": 0.5, "value": "Attach"}],
            "error": null
        }],
        "pack_error": null
    }))
    .unwrap();

    let clip = &report.clips[0];
    assert_eq!(clip.source_sequence_name.as_deref(), Some("Equip"));
    assert_eq!(clip.loop_mode, PreparedActorAnimationLoopMode::Clamp);
    assert_eq!(clip.accumulation_root.as_deref(), Some("Bip01"));
    assert_eq!(clip.required_targets, ["Bip01 R Hand", "Weapon"]);
    assert_eq!(clip.text_keys[0].value, "Attach");
}

#[test]
fn native_animation_names_normalize_blender_side_suffixes() {
    assert_eq!(animation_node_key("Bip01 Calf.L"), "bip01 l calf");
    assert_eq!(animation_node_key("Bip01 L Calf"), "bip01 l calf");
    assert_eq!(animation_node_key("Bip01 Calf.X"), "bip01 calf.x");
    assert_eq!(animation_node_key("Bip01 Head:0"), "bip01 head");
}

#[test]
fn native_sequence_parser_tolerates_bethesda_opaque_tail() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&0_i32.to_le_bytes()); // name
    bytes.extend_from_slice(&0_u32.to_le_bytes()); // controlled block count
    bytes.extend_from_slice(&0_u32.to_le_bytes()); // array grow by
    bytes.extend_from_slice(&1.0_f32.to_le_bytes()); // weight
    bytes.extend_from_slice(&(-1_i32).to_le_bytes()); // text keys
    bytes.extend_from_slice(&2_u32.to_le_bytes()); // clamp
    bytes.extend_from_slice(&1.0_f32.to_le_bytes()); // frequency
    bytes.extend_from_slice(&0.0_f32.to_le_bytes()); // start
    bytes.extend_from_slice(&0.5_f32.to_le_bytes()); // stop
    bytes.extend_from_slice(&(-1_i32).to_le_bytes()); // manager
    bytes.extend_from_slice(&1_i32.to_le_bytes()); // accumulation root
    bytes.extend_from_slice(&0_u16.to_le_bytes()); // no notes
    bytes.extend_from_slice(&[0xde, 0xad, 0xbe, 0xef]); // opaque tail
    let document = nif::fo3::Document {
        header: nif::fo3::Header {
            header_string: "Gamebryo File Format, Version 20.2.0.7".into(),
            version: nif::fo3::FILE_VERSION,
            user_version: nif::fo3::USER_VERSION,
            block_type_names: vec!["NiControllerSequence".into()],
            block_type_indices: vec![0],
            block_sizes: vec![bytes.len() as u32],
            strings: vec!["Idle".into(), "Bip01".into()],
            groups: Vec::new(),
            bethesda: nif::fo3::BethesdaHeader {
                version: 34,
                author: String::new(),
                process_script: String::new(),
                export_script: String::new(),
            },
        },
        blocks: vec![nif::fo3::RawBlock {
            index: 0,
            type_name: "NiControllerSequence".into(),
            bytes,
        }],
        roots: Vec::new(),
    };
    let sequence = native_controller_sequence(&document, 0).unwrap();
    assert_eq!(sequence.name, "Idle");
    assert_eq!(sequence.stop_time, 0.5);
    assert_eq!(sequence.loop_mode, PreparedActorAnimationLoopMode::Clamp);
    assert_eq!(sequence.accumulation_root.as_deref(), Some("Bip01"));
}

#[test]
fn native_compact_spline_values_stay_normalized_until_evaluation() {
    let data = NativeSplineData {
        floats: Vec::new(),
        compact: vec![-32767, 0, 32767],
    };
    let values = native_spline_values(
        &data,
        1,
        NativeSplineSpec {
            handle: 0,
            control_points: 3,
            compressed: true,
            offset: 10.0,
            half_range: 2.0,
            start: 0.0,
            stop: 1.0,
        },
    )
    .unwrap();
    assert_eq!(values, vec![vec![-1.0], vec![0.0], vec![1.0]]);
}

#[test]
fn native_open_uniform_spline_hits_end_controls() {
    let data = NativeSplineData {
        floats: vec![0.0, 1.0, 2.0, 3.0, 4.0],
        compact: Vec::new(),
    };
    let spec = NativeSplineSpec {
        handle: 0,
        control_points: 5,
        compressed: false,
        offset: 0.0,
        half_range: 1.0,
        start: 0.0,
        stop: 1.0,
    };
    let first = native_spline_sample(&data, 1, spec, 0.0).unwrap();
    let last = native_spline_sample(&data, 1, spec, 1.0).unwrap();
    assert!((first[0] - 0.0).abs() < 1.0e-5);
    assert!((last[0] - 4.0).abs() < 1.0e-5);
}

#[test]
fn native_open_uniform_basis_preserves_constant_channels() {
    let data = NativeSplineData {
        floats: vec![3.5; 8],
        compact: Vec::new(),
    };
    let spec = NativeSplineSpec {
        handle: 0,
        control_points: 8,
        compressed: false,
        offset: 0.0,
        half_range: 1.0,
        start: 0.0,
        stop: 1.0,
    };
    for time in [0.0, 0.13, 0.5, 0.87, 1.0] {
        let value = native_spline_sample(&data, 1, spec, time).unwrap();
        assert!((value[0] - 3.5).abs() < 1.0e-5, "time={time}: {value:?}");
    }
}

#[test]
fn native_compact_spline_applies_bias_after_weighting() {
    let data = NativeSplineData {
        floats: Vec::new(),
        compact: vec![0, 0, 0, 32767, 32767, 32767, 32767, 32767, 32767, 0, 0, 0],
    };
    let spec = NativeSplineSpec {
        handle: 0,
        control_points: 4,
        compressed: true,
        offset: 10.0,
        half_range: 2.0,
        start: 0.0,
        stop: 1.0,
    };
    let value = native_spline_sample(&data, 3, spec, 0.5).unwrap();
    assert!(value.iter().all(|value| (*value - 11.5).abs() < 1.0e-5));
}
