use super::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::Write;

const BLENDER_CONVERSION_SCRIPT: &str = include_str!("../blender_script.py");

#[test]
fn directx_normal_conversion_flips_only_green() {
    let mut texel = [12, 34, 56, 78];
    flip_directx_normal_y_texel(&mut texel);
    assert_eq!(texel, [12, 221, 56, 78]);
}

#[test]
fn blender_uses_the_shared_glossiness_formula_and_diffuse_path_annotation() {
    assert!(BLENDER_CONVERSION_SCRIPT.contains("1.5 * (2.0 / (exponent + 2.0)) ** 0.25"));
    assert!(
        BLENDER_CONVERSION_SCRIPT
            .contains("Material.import_material_gloss = staticmethod(import_material_gloss_ggx)")
    );
    assert!(BLENDER_CONVERSION_SCRIPT.contains("actor_shape_glossiness(nifnode)"));
    assert!(BLENDER_CONVERSION_SCRIPT.contains("bevyout_diffuse_texture_path"));
    assert!(BLENDER_CONVERSION_SCRIPT.contains("bevyout_perceptual_roughness"));
}

#[test]
fn blender_normal_conversion_is_green_only_and_rebuilds_stale_pngs() {
    let dds = Path::new(r"textures\architecture\Wall_N.DDS");
    let output = Path::new(r"textures\architecture\Wall.normal-y.tmp.png");
    let arguments = imagemagick_texture_arguments(dds, output, true);
    let arguments = arguments
        .iter()
        .map(|value| value.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    assert_eq!(
        arguments,
        vec![
            dds.to_string_lossy(),
            "-channel".into(),
            "G".into(),
            "-negate".into(),
            "+channel".into(),
            "-strip".into(),
            output.to_string_lossy(),
        ]
    );
    assert!(staged_texture_conversion_required(dds, true));
    assert!(!staged_texture_conversion_required(
        Path::new("textures/architecture/wall.dds"),
        true
    ));
}

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
fn material_policy_content_participates_in_converter_identity() {
    let identity = material_policy_identity("converter-v1");
    assert!(identity.starts_with("converter-v1+material-policy-"));
    assert_ne!(identity, "converter-v1");
    assert_eq!(identity, material_policy_identity("converter-v1"));
    assert_ne!(
        identity,
        material_policy_identity_with_csv(
            "converter-v1",
            "diffuse_texture,object_name,metallic\ntextures/fixtures/metal.dds,Metal Fixture,1\n"
        )
    );
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

fn glb_with_json_document(document: serde_json::Value) -> Vec<u8> {
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

#[test]
fn reads_and_deduplicates_animation_sound_cues_from_glb_extras() {
    let encoded = serde_json::to_string(&vec![
        AnimationSoundCue {
            sequence: "Open".into(),
            time: 0.2,
            editor_id: "DRSLate".into(),
        },
        AnimationSoundCue {
            sequence: "Open".into(),
            time: 0.01,
            editor_id: "DRSEarly".into(),
        },
    ])
    .unwrap();
    let document = serde_json::json!({
        "nodes": [
            {"extras": {"bevyout_animation_sound_cues": encoded}},
            {"extras": {"bevyout_animation_sound_cues": encoded}},
        ]
    });
    let path = std::env::temp_dir().join(format!(
        "bevyout-animation-audio-{}.glb",
        std::process::id()
    ));
    fs::write(&path, glb_with_json_document(document)).unwrap();
    let cues = read_glb_animation_sound_cues(&path).unwrap();
    assert_eq!(cues.len(), 2);
    assert_eq!(cues[0].editor_id, "DRSEarly");
    assert_eq!(cues[1].editor_id, "DRSLate");
    let _ = fs::remove_file(path);
}

#[test]
fn rejects_malformed_animation_sound_cue_metadata() {
    let document = serde_json::json!({
        "nodes": [{"extras": {"bevyout_animation_sound_cues": "not-json"}}]
    });
    let path = std::env::temp_dir().join(format!(
        "bevyout-invalid-animation-audio-{}.glb",
        std::process::id()
    ));
    fs::write(&path, glb_with_json_document(document)).unwrap();
    let error = read_glb_animation_sound_cues(&path).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("invalid animation sound cue metadata")
    );
    let _ = fs::remove_file(path);
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
    let valid = br#"{"schema_version":3,"source":"GeneratedRender","bodies":[],"joints":[]}"#;
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
fn authored_emission_policy_exports_nonzero_colors_and_rejects_zero() {
    let authored = [0.8, 0.4, 0.1];
    assert_eq!(authored_emission_color(authored), Some(authored));
    assert_eq!(authored_emission_color([0.0, 0.0, 0.0]), None);
    assert_eq!(
        authored_emission(authored, 2.5),
        Some(AuthoredEmission {
            color: authored,
            strength: 2.5,
        })
    );
    assert_eq!(authored_emission(authored, f32::NAN).unwrap().strength, 1.0);
    assert_eq!(authored_emission(authored, -1.0).unwrap().strength, 1.0);
}

#[test]
fn blender_script_reads_authored_emission_before_existing_overrides() {
    for source_field in [
        "nif_emission_multiplier(prop, 'emit_multi', 'emissive_mult')",
        "nif_emission_multiplier(n_mat_prop, 'emit_multi', 'emissive_mult')",
        "getattr(prop, 'emissive_multiple', None)",
        "getattr(prop, 'base_color_scale', None)",
    ] {
        assert!(
            BLENDER_CONVERSION_SCRIPT.contains(source_field),
            "missing NIFTools source field: {source_field}"
        );
    }
    assert!(BLENDER_CONVERSION_SCRIPT.contains("bevyout_emissive_strength"));
    assert!(
        BLENDER_CONVERSION_SCRIPT
            .contains("candidates = [keys[shader_type], 'bevyout_emissive_strength']")
    );
    assert!(BLENDER_CONVERSION_SCRIPT.contains("value < 0.0"));
    assert!(BLENDER_CONVERSION_SCRIPT.contains("return 1.0, False"));
    assert!(BLENDER_CONVERSION_SCRIPT.contains("getattr(niftools, 'emissive_color', None)"));
    assert!(
        BLENDER_CONVERSION_SCRIPT.contains("new_emission.default_value = (0.0, 0.0, 0.0, 1.0)")
    );
    assert!(BLENDER_CONVERSION_SCRIPT.contains("elif authored_emission is not None:"));
    assert!(
        BLENDER_CONVERSION_SCRIPT.contains("if new_emission_strength and source_strength_applies:")
    );
    let authored = BLENDER_CONVERSION_SCRIPT
        .find("elif authored_emission is not None:")
        .expect("authored emission fallback is missing");
    let strength = BLENDER_CONVERSION_SCRIPT
        .find("if new_emission_strength and source_strength_applies:")
        .expect("source emission strength fallback is missing");
    let bulb = BLENDER_CONVERSION_SCRIPT
        .find("if bulb_override and new_emission:")
        .expect("emissive bulb override is missing");
    let glow = BLENDER_CONVERSION_SCRIPT
        .find("if glow and new_emission:")
        .expect("glow texture override is missing");
    assert!(authored < strength && strength < bulb && bulb < glow);
}

#[test]
fn blender_script_recovers_authored_emission_from_zero_imported_strength() {
    let normalized_script = BLENDER_CONVERSION_SCRIPT
        .replace("\r\n", "\n")
        .replace('\r', "\n");
    assert!(BLENDER_CONVERSION_SCRIPT.contains("authored_emission_fallback = False"));
    assert!(BLENDER_CONVERSION_SCRIPT.contains("authored_emission_fallback = True"));
    assert!(
        normalized_script
            .contains("source_strength_applies = (\n            authored_emission_fallback and\n            has_emission_multiplier")
    );
    assert!(BLENDER_CONVERSION_SCRIPT.contains("emission_strength <= 0.0"));
    assert!(BLENDER_CONVERSION_SCRIPT.contains("emission_strength == 1.0"));
    assert!(
        BLENDER_CONVERSION_SCRIPT
            .contains("new_emission_strength.default_value = emission_multiplier")
    );
    assert!(BLENDER_CONVERSION_SCRIPT.contains("new_emission_strength.default_value = 1.0"));
    assert!(
        BLENDER_CONVERSION_SCRIPT.contains("if source_strength_applies and not bulb_override:")
    );
    assert!(BLENDER_CONVERSION_SCRIPT.contains("original_ni_material_import"));
    assert!(BLENDER_CONVERSION_SCRIPT.contains("NifData.data.blocks"));
    assert!(BLENDER_CONVERSION_SCRIPT.contains("BSLightingShaderProperty"));
    assert!(BLENDER_CONVERSION_SCRIPT.contains("BSEffectShaderProperty"));
}

#[test]
fn material_emission_policy_preserves_source_strength_and_override_precedence() {
    let authored = [0.8, 0.4, 0.1];
    assert_eq!(
        material_emission_policy(authored, 2.5, false, false, false),
        MaterialEmissionPolicy::Authored(AuthoredEmission {
            color: authored,
            strength: 2.5,
        })
    );
    assert_eq!(
        material_emission_policy(authored, 2.5, true, false, false),
        MaterialEmissionPolicy::Explicit
    );
    assert_eq!(
        material_emission_policy(authored, 2.5, true, true, false),
        MaterialEmissionPolicy::Bulb
    );
    assert_eq!(
        material_emission_policy(authored, 2.5, true, true, true),
        MaterialEmissionPolicy::Glow
    );
    assert_eq!(
        material_emission_policy([0.0, 0.0, 0.0], 2.5, false, false, false),
        MaterialEmissionPolicy::None
    );
}

#[test]
fn blender_job_json_carries_quick_ao_profile() {
    let json = blender_jobs_json(&[BlenderAssetJob {
        kind: AssetJobKind::StaticNif,
        input: PathBuf::from("C:\\staging\\mesh.nif"),
        output: PathBuf::from("C:\\cache\\mesh.glb"),
        physics_output: PathBuf::from("C:\\cache\\mesh.physics.json.gz"),
        model: "architecture/test.nif".into(),
        conversion: AssetConversion::QuickAo,
        root_transform_policy: RootTransformPolicy::PreserveReviewRequired,
    }]);
    assert!(json.contains("\"conversion\":\"ao-quick-v1\""));
    assert!(json.contains("mesh.physics.json.gz"));
    assert!(json.contains("architecture/test.nif"));
    assert!(json.contains("preserve_review_required"));
}

#[test]
fn root_transform_policy_is_normalized_and_limited_to_verified_models() {
    assert_eq!(
        root_transform_policy(r"MESHES\Dungeons\Vault\Room\VRmWallScreen01.NIF"),
        RootTransformPolicy::DiscardVerified
    );
    assert_eq!(
        root_transform_policy("/dungeons/vault/room/vdnwallendcoroutr01.nif"),
        RootTransformPolicy::PreserveVerified
    );
    assert_eq!(
        root_transform_policy("dungeons/vault/room/vdnwallendcorinr01.nif"),
        RootTransformPolicy::PreserveVerified
    );
    assert_eq!(
        root_transform_policy("dungeons/rivetcity/roomsmall/rcsmdoor01.nif"),
        RootTransformPolicy::PreserveReviewRequired
    );
}

#[test]
fn texture_references_matches_forward_slashes_and_mid_string_textures() {
    // Forward-slash separated path, plus "textures" appearing mid-token
    // (inside "nontextures") which the scanner still recognizes because it
    // searches for the substring "textures" rather than a path segment.
    // A trailing 2-byte printable run ("ab") between control bytes is
    // shorter than the 5-byte inspection threshold and must contribute
    // nothing.
    let bytes = b"meshes/foo/textures/floor.dds materials/nontextures/decal.tga \x01ab\x01";
    let refs = texture_references(bytes)
        .into_iter()
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        refs,
        std::collections::HashSet::from([
            "textures/floor.dds".to_string(),
            "textures/decal.tga".to_string(),
        ])
    );
}

#[test]
fn texture_references_ignores_runs_shorter_than_five_bytes() {
    // ".dds" alone is 4 bytes: shorter than the inspection threshold, so
    // it must never be considered even though it contains an extension.
    let refs = texture_references(b"\x01.dds\x01");
    assert!(refs.is_empty());
}

#[test]
fn placeholder_png_images_are_rejected_and_real_ones_pass() {
    fn glb_with_png(width: u32, height: u32) -> Vec<u8> {
        let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
        png.extend_from_slice(&13_u32.to_be_bytes());
        png.extend_from_slice(b"IHDR");
        png.extend_from_slice(&width.to_be_bytes());
        png.extend_from_slice(&height.to_be_bytes());
        let json = format!(
            "{{\"bufferViews\":[{{\"byteOffset\":0,\"byteLength\":{}}}],\
             \"images\":[{{\"bufferView\":0,\"name\":\"probe\"}}]}}",
            png.len()
        );
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"glTF");
        bytes.extend_from_slice(&2_u32.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(&(json.len() as u32).to_le_bytes());
        bytes.extend_from_slice(b"JSON");
        bytes.extend_from_slice(json.as_bytes());
        bytes.extend_from_slice(&(png.len() as u32).to_le_bytes());
        bytes.extend_from_slice(b"BIN\0");
        bytes.extend_from_slice(&png);
        bytes
    }
    let dir = std::env::temp_dir();
    let placeholder = dir.join(format!("bevyout-placeholder-{}.glb", std::process::id()));
    std::fs::write(&placeholder, glb_with_png(1, 1)).unwrap();
    let error = validate_glb_images(&placeholder).unwrap_err();
    assert!(error.to_string().contains("1x1 placeholder"));
    let real = dir.join(format!("bevyout-real-{}.glb", std::process::id()));
    std::fs::write(&real, glb_with_png(2, 2)).unwrap();
    assert!(validate_glb_images(&real).is_ok());
    let _ = std::fs::remove_file(placeholder);
    let _ = std::fs::remove_file(real);
}

fn glb_with_json(json: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"glTF");
    bytes.extend_from_slice(&2_u32.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&(json.len() as u32).to_le_bytes());
    bytes.extend_from_slice(b"JSON");
    bytes.extend_from_slice(json.as_bytes());
    bytes
}

#[test]
fn visual_audit_counts_non_collision_primitives_and_reads_root_metadata() {
    let json = r#"{
        "accessors":[{"count":24},{"count":24}],
        "meshes":[{"primitives":[{"attributes":{"POSITION":0},"indices":1}]}],
        "nodes":[
            {"name":"root","extras":{
                "bevyout_source_model":"architecture/test.nif",
                "bevyout_root_transform_policy":"preserve_review_required",
                "bevyout_record_zero_non_identity":true,
                "bevyout_source_render_meshes":1,
                "bevyout_source_render_vertices":16,
                "bevyout_source_render_triangles":8,
                "bevyout_spatial_audit_version":1,
                "bevyout_expected_spatial_corrections":0,
                "bevyout_verified_spatial_corrections":0
                ,"bevyout_expected_collision_corrections":0
                ,"bevyout_verified_collision_corrections":0
            }},
            {"name":"visual","mesh":0}
        ]
    }"#;
    let path =
        std::env::temp_dir().join(format!("bevyout-visual-audit-{}.glb", std::process::id()));
    fs::write(&path, glb_with_json(json)).unwrap();
    let audit = audit_glb_visuals(&path).unwrap();
    assert_eq!(audit.renderable_primitives, 1);
    assert_eq!(audit.renderable_vertices, 24);
    assert_eq!(audit.renderable_triangles, 8);
    assert_eq!(audit.source_render_meshes, Some(1));
    assert_eq!(audit.source_render_vertices, Some(16));
    assert_eq!(audit.source_render_triangles, Some(8));
    assert_eq!(audit.spatial_audit_version, Some(1));
    assert_eq!(audit.expected_spatial_corrections, Some(0));
    assert_eq!(audit.verified_spatial_corrections, Some(0));
    assert_eq!(audit.expected_collision_corrections, Some(0));
    assert_eq!(audit.verified_collision_corrections, Some(0));
    assert_eq!(audit.source_model.as_deref(), Some("architecture/test.nif"));
    assert_eq!(
        audit.root_transform_policy.as_deref(),
        Some("preserve_review_required")
    );
    assert!(audit.record_zero_non_identity);
    let _ = fs::remove_file(path);
}

#[test]
fn visual_audit_rejects_collision_only_and_empty_position_accessors() {
    let json = r#"{
        "accessors":[{"count":0}],
        "meshes":[{"primitives":[{"attributes":{"POSITION":0}}]}],
        "nodes":[{"mesh":0,"extras":{"bevyout_collision":true}}]
    }"#;
    let path = std::env::temp_dir().join(format!(
        "bevyout-empty-visual-audit-{}.glb",
        std::process::id()
    ));
    fs::write(&path, glb_with_json(json)).unwrap();
    assert_eq!(audit_glb_visuals(&path).unwrap().renderable_primitives, 0);
    let _ = fs::remove_file(path);
}

#[test]
fn image_referencing_out_of_range_buffer_view_is_rejected() {
    let json = r#"{"bufferViews":[],"images":[{"bufferView":0,"name":"probe"}]}"#;
    let path =
        std::env::temp_dir().join(format!("bevyout-missing-view-{}.glb", std::process::id()));
    std::fs::write(&path, glb_with_json(json)).unwrap();
    let error = validate_glb_images(&path).unwrap_err();
    assert!(error.to_string().contains("missing bufferView"));
    let _ = std::fs::remove_file(path);
}

#[test]
fn buffer_view_extending_past_the_glb_is_rejected() {
    let json = r#"{"bufferViews":[{"byteOffset":0,"byteLength":999999}],"images":[{"bufferView":0,"name":"probe"}]}"#;
    let path =
        std::env::temp_dir().join(format!("bevyout-view-overflow-{}.glb", std::process::id()));
    std::fs::write(&path, glb_with_json(json)).unwrap();
    let error = validate_glb_images(&path).unwrap_err();
    assert!(error.to_string().contains("extends beyond GLB"));
    let _ = std::fs::remove_file(path);
}

#[test]
fn actor_conversion_retries_creature_assemblies_through_niftools() {
    assert!(
        BLENDER_CONVERSION_SCRIPT
            .contains("actor PyNifly import failed; retrying NIFTools compatibility path")
    );
    assert!(BLENDER_CONVERSION_SCRIPT.contains("source_paths=assembly_inputs"));
    assert!(
        BLENDER_CONVERSION_SCRIPT
            .contains("assembly_inputs is not None and not assembly_used_niftools_fallback")
    );
}

#[test]
fn actor_conversion_applies_the_selected_eyes_texture_only_to_eye_sources() {
    assert!(BLENDER_CONVERSION_SCRIPT.contains("eye_sources = {actor_source_key(path)"));
    assert!(BLENDER_CONVERSION_SCRIPT.contains("is_selected_eye and eye_texture"));
    assert!(BLENDER_CONVERSION_SCRIPT.contains("selected, [eye_texture] + list(texture_values)"));
}

#[test]
fn actor_glb_audit_accepts_a_textured_weighted_skin() {
    let document = serde_json::json!({
        "accessors": [
            {"count": 3},
            {"count": 3},
            {"count": 3},
            {"count": 1}
        ],
        "images": [{"uri": "skin.png"}],
        "textures": [{"source": 0}],
        "materials": [{"pbrMetallicRoughness": {"baseColorTexture": {"index": 0}}}],
        "meshes": [{"primitives": [{
            "attributes": {"POSITION": 0, "JOINTS_0": 1, "WEIGHTS_0": 2},
            "material": 0
        }]}],
        "nodes": [{"mesh": 0, "skin": 0}, {"name": "Bip01"}],
        "skins": [{"joints": [1], "inverseBindMatrices": 3}]
    });
    let path = std::env::temp_dir().join(format!(
        "bevyout-valid-actor-audit-{}.glb",
        std::process::id()
    ));
    fs::write(&path, glb_with_json_document(document)).unwrap();
    let audit = validate_actor_glb(&path).unwrap();
    assert_eq!(audit.skins, 1);
    assert_eq!(audit.skinned_primitives, 1);
    assert_eq!(audit.textured_primitives, 1);
    let _ = fs::remove_file(path);
}

#[test]
fn actor_glb_audit_rejects_missing_weights_and_base_color_texture() {
    let document = serde_json::json!({
        "accessors": [{"count": 3}, {"count": 3}, {"count": 1}],
        "materials": [{}],
        "meshes": [{"primitives": [{
            "attributes": {"POSITION": 0, "JOINTS_0": 1},
            "material": 0
        }]}],
        "nodes": [{"mesh": 0, "skin": 0}, {"name": "Bip01"}],
        "skins": [{"joints": [1], "inverseBindMatrices": 2}]
    });
    let path = std::env::temp_dir().join(format!(
        "bevyout-invalid-actor-audit-{}.glb",
        std::process::id()
    ));
    fs::write(&path, glb_with_json_document(document)).unwrap();
    let error = validate_actor_glb(&path).unwrap_err();
    assert!(error.to_string().contains("WEIGHTS_0"));
    let _ = fs::remove_file(path);
}
