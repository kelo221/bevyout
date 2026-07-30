use super::*;

#[test]
fn native_normal_conversion_separates_normal_from_shared_specular_source() {
    let source_path = "textures/shared_payload.dds".to_string();
    let mut source = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
        1,
        1,
        image::Rgba([12, 34, 56, 78]),
    ))
    .write_to(&mut source, image::ImageFormat::Png)
    .unwrap();
    let source = source.into_inner();
    let mut textures = BTreeMap::from([(source_path.clone(), source.clone())]);
    let mut materials = vec![nif::fo3::SceneMaterial {
        name: "shared normal/specular".into(),
        base_color: [1.0; 4],
        emissive: [0.0; 3],
        emissive_multiplier: 1.0,
        roughness: 0.5,
        alpha_mode: nif::fo3::SceneAlphaMode::Opaque,
        alpha_cutoff: None,
        double_sided: false,
        unlit: false,
        diffuse_texture: Some(source_path.clone()),
        normal_texture: Some(source_path.clone()),
        specular_texture: Some(source_path.clone()),
        glow_texture: None,
        height_texture: None,
        environment_texture: None,
        environment_mask: None,
        shader_type: 0,
        shader_flags_1: 0,
        shader_flags_2: 0,
    }];

    prepare_native_normal_textures(&mut materials, &mut textures).unwrap();

    let derived_path = materials[0]
        .normal_texture
        .as_deref()
        .expect("normal path is retained");
    assert_ne!(derived_path, source_path);
    assert_eq!(
        materials[0].diffuse_texture.as_deref(),
        Some(source_path.as_str())
    );
    assert_eq!(materials[0].specular_texture.as_deref(), Some(derived_path));
    assert_eq!(textures.get(&source_path), Some(&source));
    let converted = image::load_from_memory(
        textures
            .get(derived_path)
            .expect("derived normal image was inserted"),
    )
    .unwrap()
    .to_rgba8();
    assert_eq!(converted.get_pixel(0, 0).0, [12, 221, 56, 78]);
}

#[test]
fn native_glb_reuses_the_alpha_preserving_normal_for_specular_strength() {
    let source_path = "textures/furniture/chair03_n.dds".to_string();
    let mut source = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
        1,
        1,
        image::Rgba([12, 34, 56, 61]),
    ))
    .write_to(&mut source, image::ImageFormat::Png)
    .unwrap();
    let mut textures = BTreeMap::from([(source_path.clone(), source.into_inner())]);
    let mut materials = vec![nif::fo3::SceneMaterial {
        name: "Chair03F".into(),
        base_color: [1.0; 4],
        emissive: [0.0; 3],
        emissive_multiplier: 1.0,
        roughness: 0.817_491_5,
        alpha_mode: nif::fo3::SceneAlphaMode::Opaque,
        alpha_cutoff: None,
        double_sided: false,
        unlit: false,
        diffuse_texture: None,
        normal_texture: Some(source_path),
        specular_texture: None,
        glow_texture: None,
        height_texture: None,
        environment_texture: None,
        environment_mask: None,
        shader_type: nif::fo3::SHADER_TYPE_DEFAULT,
        shader_flags_1: nif::fo3::SHADER_FLAG1_SPECULAR,
        shader_flags_2: 0,
    }];

    apply_fallout_specular_texture_policy(&mut materials);
    prepare_native_normal_textures(&mut materials, &mut textures).unwrap();
    let scene = nif::fo3::Scene {
        nodes: Vec::new(),
        roots: Vec::new(),
        materials,
        skins: Vec::new(),
        issues: Vec::new(),
        statistics: nif::fo3::SceneStatistics::default(),
        animations: Vec::new(),
        animation_sound_cues: Vec::new(),
    };
    let output = nif::fo3::encode_glb(&scene, &textures, &nif::fo3::GlbOptions::default()).unwrap();
    let output =
        patch_glb_material_policy(&output.bytes, &MetallicMaterialTable::built_in().unwrap())
            .unwrap();
    let json_length = u32::from_le_bytes(output[12..16].try_into().unwrap()) as usize;
    let document: serde_json::Value =
        serde_json::from_slice(&output[20..20 + json_length]).unwrap();
    let material = &document["materials"][0];

    assert_eq!(
        material["normalTexture"]["index"],
        material["extensions"]["KHR_materials_specular"]["specularTexture"]["index"]
    );
    assert_eq!(
        material["pbrMetallicRoughness"]["roughnessFactor"],
        serde_json::json!(0.8174915)
    );
    assert_eq!(
        material["pbrMetallicRoughness"]["metallicFactor"],
        serde_json::json!(0.0)
    );
    assert!(
        document["extensionsUsed"]
            .as_array()
            .unwrap()
            .iter()
            .any(|extension| extension == "KHR_materials_specular")
    );
    assert_eq!(document["images"].as_array().unwrap().len(), 1);
    assert_eq!(document["textures"].as_array().unwrap().len(), 1);

    let gltf = gltf::Gltf::from_slice(&output).unwrap();
    let blob = gltf.blob.as_deref().unwrap();
    let source = gltf.document.images().next().unwrap().source();
    let gltf::image::Source::View { view, .. } = source else {
        panic!("embedded image must use a buffer view");
    };
    let image = image::load_from_memory(&blob[view.offset()..view.offset() + view.length()])
        .unwrap()
        .to_rgba8();
    assert_eq!(image.get_pixel(0, 0).0, [12, 221, 56, 61]);
}

#[test]
fn native_glb_omits_specular_extension_when_disabled_or_normal_is_absent() {
    for (shader_flags_1, normal_texture) in [
        (0, Some("textures/furniture/chair03_n.dds")),
        (nif::fo3::SHADER_FLAG1_SPECULAR, None),
    ] {
        let mut textures = BTreeMap::new();
        if let Some(path) = normal_texture {
            let mut source = Cursor::new(Vec::new());
            image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
                1,
                1,
                image::Rgba([12, 34, 56, 61]),
            ))
            .write_to(&mut source, image::ImageFormat::Png)
            .unwrap();
            textures.insert(path.to_string(), source.into_inner());
        }
        let mut materials = vec![nif::fo3::SceneMaterial {
            name: "Chair03F".into(),
            base_color: [1.0; 4],
            emissive: [0.0; 3],
            emissive_multiplier: 1.0,
            roughness: 0.817_491_5,
            alpha_mode: nif::fo3::SceneAlphaMode::Opaque,
            alpha_cutoff: None,
            double_sided: false,
            unlit: false,
            diffuse_texture: None,
            normal_texture: normal_texture.map(str::to_owned),
            specular_texture: Some("textures/incorrect_slot7.dds".into()),
            glow_texture: None,
            height_texture: None,
            environment_texture: None,
            environment_mask: None,
            shader_type: nif::fo3::SHADER_TYPE_DEFAULT,
            shader_flags_1,
            shader_flags_2: 0,
        }];
        apply_fallout_specular_texture_policy(&mut materials);
        assert_eq!(materials[0].specular_texture, None);
        let scene = nif::fo3::Scene {
            nodes: Vec::new(),
            roots: Vec::new(),
            materials,
            skins: Vec::new(),
            issues: Vec::new(),
            statistics: nif::fo3::SceneStatistics::default(),
            animations: Vec::new(),
            animation_sound_cues: Vec::new(),
        };
        let output =
            nif::fo3::encode_glb(&scene, &textures, &nif::fo3::GlbOptions::default()).unwrap();
        let json_length = u32::from_le_bytes(output.bytes[12..16].try_into().unwrap()) as usize;
        let document: serde_json::Value =
            serde_json::from_slice(&output.bytes[20..20 + json_length]).unwrap();
        assert!(document["materials"][0]["extensions"]["KHR_materials_specular"].is_null());
        assert!(
            !document["extensionsUsed"]
                .as_array()
                .is_some_and(|extensions| extensions
                    .iter()
                    .any(|extension| extension == "KHR_materials_specular"))
        );
    }
}

#[test]
fn asset_paths_are_data_relative_and_portable() {
    assert_eq!(
        normalize_asset_path("Clutter\\Desk.NIF"),
        "meshes/clutter/desk.nif"
    );
    assert_eq!(
        normalize_asset_path("meshes/Ammo/10mm.nif"),
        "meshes/ammo/10mm.nif"
    );
}

#[test]
fn existing_outputs_require_force() {
    let path = std::env::temp_dir().join(format!(
        "bevyout-nif-convert-existing-{}.glb",
        std::process::id()
    ));
    fs::write(&path, b"old").unwrap();
    assert!(ensure_output_available(&path, false).is_err());
    assert!(ensure_output_available(&path, true).is_ok());
    let _ = fs::remove_file(path);
}

#[test]
fn reusable_conversion_rejects_malformed_nif_without_publishing_outputs() {
    let root = std::env::temp_dir().join(format!(
        "bevyout-nif-convert-malformed-{}",
        std::process::id()
    ));
    fs::create_dir_all(&root).unwrap();
    let output = root.join("malformed.glb");
    let physics = root.join("malformed.physics.json.gz");
    let error = convert_nif(NifConversionRequest {
        source_name: "malformed.nif",
        nif_bytes: b"not a nif",
        output: &output,
        physics_output: Some(&physics),
        report: None,
        conversion: NifConversionMode::Preserve,
        root_transform_policy: RootTransformPolicy::PreserveReviewRequired,
        allow_lossy: true,
        force: true,
        data_root: Some(&root),
        archives: &[],
    })
    .unwrap_err();
    assert!(error.to_string().contains("parsing FO3/FNV NIF"));
    assert!(!output.exists());
    assert!(!physics.exists());
    let _ = fs::remove_dir_all(root);
}

#[test]
fn native_joint_preserves_authored_frames_limits_and_strength() {
    let source = nif::fo3::PhysicsJoint {
        source_block: 42,
        kind: "spherical".into(),
        body_a: 3,
        body_b: 7,
        anchor_a: [1.0, 2.0, 3.0],
        anchor_b: [1.0, 2.0, 3.0],
        frame_a_rotation_xyzw: [0.0, 0.0, 0.70710677, 0.70710677],
        frame_b_rotation_xyzw: [0.0, 0.0, 0.70710677, 0.70710677],
        lower_limit: None,
        upper_limit: None,
        cone_limit: Some(1.2),
        plane_lower_limit: Some(-0.4),
        plane_upper_limit: Some(0.5),
        twist_lower_limit: Some(-0.7),
        twist_upper_limit: Some(0.8),
        malleable_strength: Some(0.9),
    };

    let converted = convert_physics_joint(source);

    assert_eq!(converted.kind, "spherical");
    assert_eq!((converted.body_a, converted.body_b), (3, 7));
    assert_eq!(converted.frame_a_rotation_xyzw[3], 0.70710677);
    assert_eq!(converted.cone_limit, Some(1.2));
    assert_eq!(converted.twist_upper_limit, Some(0.8));
    assert_eq!(converted.malleable_strength, Some(0.9));
    assert_eq!(converted.source, PreparedPhysicsJointSource::Authored);
}
