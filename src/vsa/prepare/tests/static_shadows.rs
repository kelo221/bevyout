use super::*;
use crate::vsa::manifest::{
    PreparedActor, PreparedDoor, PreparedPhysicsClassification, PreparedPickup,
    PreparedPlacementAudio, PreparedRuntimeMutability,
};
use std::sync::atomic::AtomicUsize;

static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn temp_directory(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "bevyout-static-shadows-{label}-{}-{}",
        std::process::id(),
        TEMP_COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    if path.exists() {
        fs::remove_dir_all(&path).unwrap();
    }
    fs::create_dir_all(&path).unwrap();
    path
}

fn placement(asset_path: &str) -> PreparedPlacement {
    PreparedPlacement {
        reference_form_id: 20,
        base_form_id: 10,
        asset_path: Some(asset_path.into()),
        translation: [0.0, 0.0, 0.0],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        error: None,
        physics_asset_path: None,
        physics_source: None,
        physics_classification: PreparedPhysicsClassification::Static,
        step_support: true,
        mutability: PreparedRuntimeMutability::Immutable,
        mutability_root_form_id: None,
        reference_kind: "REFR".into(),
        base_kind: "STAT".into(),
        editor_id: None,
        display_name: None,
        count: 1,
        semantic: PreparedSemantic::Static,
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        linked_reference_form_id: None,
        inventory: Vec::new(),
        audio: PreparedPlacementAudio::default(),
        ao_mode: "ao-none".into(),
    }
}

fn light(reference_form_id: u32) -> PreparedLight {
    PreparedLight {
        reference_form_id,
        base_form_id: reference_form_id + 100,
        translation: [1.0, 2.0, 3.0],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        color_rgba: [1.0, 0.5, 0.25, 1.0],
        radius: 8.0,
        intensity_lumens: 1_000.0,
        kind: "point".into(),
        flags: 0,
        initially_enabled: true,
    }
}

fn triangle(winding_reversed: bool) -> ShadowTriangle {
    let mut vertices = [
        Point3::new(-1.0, -1.0, -2.0),
        Point3::new(1.0, -1.0, -2.0),
        Point3::new(0.0, 1.0, -2.0),
    ];
    if winding_reversed {
        vertices.swap(1, 2);
    }
    ShadowTriangle {
        vertices,
        node_index: 0,
    }
}

#[test]
fn triangle_intersection_is_two_sided() {
    let origin = Point3::origin();
    let direction = Vec3::NEG_Z;
    assert_eq!(
        ray_triangle_distance(origin, direction, &triangle(false)),
        Some(2.0)
    );
    assert_eq!(
        ray_triangle_distance(origin, direction, &triangle(true)),
        Some(2.0)
    );
}

#[test]
fn cubemap_centers_match_bevy_z_conversion() {
    let expected = [
        Vec3::X,
        Vec3::NEG_X,
        Vec3::Y,
        Vec3::NEG_Y,
        Vec3::NEG_Z,
        Vec3::Z,
    ];
    for (face, expected) in expected.into_iter().enumerate() {
        assert!(cubemap_texel_direction(face, 0, 0, 1).abs_diff_eq(expected, 1e-6));
    }
}

#[test]
fn reverse_z_encodes_near_and_clear_depths() {
    assert_eq!(reverse_z_depth(Vec3::NEG_Z, 0.1, 0.1), 1.0);
    assert!((reverse_z_depth(Vec3::NEG_Z, 2.0, 0.1) - 0.05).abs() < 1e-6);
    assert!(
        (reverse_z_depth(Vec3::new(1.0, 1.0, 0.0).normalize(), 2.0, 0.1) - 0.07071068).abs() < 1e-6
    );
}

#[test]
fn caster_filter_excludes_dynamic_physics_and_actor_objects() {
    let semantics = [
        PreparedSemantic::Static,
        PreparedSemantic::Pickup(PreparedPickup {
            category: "MISC".into(),
            value: Some(1),
            weight: Some(1.0),
        }),
        PreparedSemantic::Container,
        PreparedSemantic::Activator,
        PreparedSemantic::Furniture,
        PreparedSemantic::Npc(PreparedActor {
            base_template_form_id: None,
            ..Default::default()
        }),
        PreparedSemantic::Creature(PreparedActor {
            base_template_form_id: None,
            ..Default::default()
        }),
        PreparedSemantic::Unsupported,
    ];
    let physics_classes = [
        PreparedPhysicsClassification::Static,
        PreparedPhysicsClassification::Kinematic,
        PreparedPhysicsClassification::Dynamic,
    ];
    let mut placements = Vec::new();
    for (semantic_index, semantic) in semantics.into_iter().enumerate() {
        for (physics_index, physics_classification) in physics_classes.into_iter().enumerate() {
            let mut candidate =
                placement(&format!("candidate-{semantic_index}-{physics_index}.glb"));
            candidate.reference_form_id =
                (semantic_index * physics_classes.len() + physics_index + 1) as u32;
            candidate.semantic = semantic.clone();
            candidate.physics_classification = physics_classification;
            placements.push(candidate);
        }
    }

    let casters = sorted_shadow_casters(&placements);
    // Static and kinematic placements remain eligible except pickups and
    // actors; all dynamic placements are excluded as well.
    assert_eq!(casters.len(), 10);
    assert!(
        casters
            .iter()
            .all(|placement| !matches!(placement.semantic, PreparedSemantic::Pickup(_)))
    );
    assert!(casters.iter().all(|placement| {
        !matches!(
            placement.semantic,
            PreparedSemantic::Npc(_) | PreparedSemantic::Creature(_)
        )
    }));
    assert!(
        casters
            .iter()
            .any(|placement| placement.physics_classification
                == PreparedPhysicsClassification::Kinematic)
    );
    assert!(casters.iter().all(|placement| {
        placement.physics_classification != PreparedPhysicsClassification::Dynamic
    }));
}

#[test]
fn caster_filter_excludes_item_record_kinds_even_when_semantic_is_static() {
    let mut placements = Vec::new();
    for (index, kind) in [
        "WEAP", "AMMO", "ARMO", "ALCH", "MISC", "BOOK", "NOTE", "KEYM",
    ]
    .into_iter()
    .enumerate()
    {
        let mut candidate = placement(&format!("item-{index}.glb"));
        candidate.reference_form_id = index as u32 + 1;
        candidate.base_kind = kind.into();
        candidate.semantic = PreparedSemantic::Static;
        placements.push(candidate);
    }

    assert!(sorted_shadow_casters(&placements).is_empty());
}

#[test]
fn caster_filter_excludes_doors_dynamic_bodies_disabled_placements_and_unresolved_assets() {
    let eligible = placement("eligible.glb");
    let mut disabled = placement("disabled.glb");
    disabled.initially_enabled = false;
    let mut unresolved = placement("missing.glb");
    unresolved.asset_path = None;
    let mut doors = Vec::new();
    for (index, physics_classification) in [
        PreparedPhysicsClassification::Static,
        PreparedPhysicsClassification::Kinematic,
        PreparedPhysicsClassification::Dynamic,
    ]
    .into_iter()
    .enumerate()
    {
        let mut door = placement(&format!("door-{index}.glb"));
        door.reference_form_id = 100 + index as u32;
        door.semantic = PreparedSemantic::Door(PreparedDoor {
            lock_level: None,
            key_form_id: None,
            trapped: false,
            destination: None,
        });
        door.physics_classification = physics_classification;
        doors.push(door);
    }

    let mut dynamic = placement("dynamic.glb");
    dynamic.physics_classification = PreparedPhysicsClassification::Dynamic;
    let mut placements = vec![eligible, disabled, unresolved, dynamic];
    placements.extend(doors);
    let casters = sorted_shadow_casters(&placements);
    assert_eq!(casters.len(), 1);
    assert_eq!(casters[0].asset_path.as_deref(), Some("eligible.glb"));
}

#[test]
fn caster_filter_excludes_rclightbox01_across_representative_classes() {
    let semantics = [
        PreparedSemantic::Static,
        PreparedSemantic::Container,
        PreparedSemantic::Activator,
        PreparedSemantic::Furniture,
    ];
    let physics_classes = [
        PreparedPhysicsClassification::Static,
        PreparedPhysicsClassification::Kinematic,
        PreparedPhysicsClassification::Dynamic,
    ];
    let mut placements = Vec::new();
    for (semantic_index, semantic) in semantics.into_iter().enumerate() {
        for (physics_index, physics_classification) in physics_classes.into_iter().enumerate() {
            let mut candidate =
                placement(&format!("rclightbox-{semantic_index}-{physics_index}.glb"));
            candidate.reference_form_id =
                (semantic_index * physics_classes.len() + physics_index + 1) as u32;
            candidate.base_form_id = RCLIGHTBOX01_BASE_FORM_ID;
            candidate.semantic = semantic.clone();
            candidate.physics_classification = physics_classification;
            placements.push(candidate);
        }
    }

    let mut ordinary_activator = placement("ordinary-activator.glb");
    ordinary_activator.reference_form_id = 100;
    ordinary_activator.semantic = PreparedSemantic::Activator;
    ordinary_activator.physics_classification = PreparedPhysicsClassification::Kinematic;
    placements.push(ordinary_activator);

    let casters = sorted_shadow_casters(&placements);
    assert_eq!(casters.len(), 1);
    assert_eq!(casters[0].reference_form_id, 100);
    assert_eq!(casters[0].base_form_id, 10);
    assert_eq!(casters[0].semantic, PreparedSemantic::Activator);
    assert_eq!(
        casters[0].physics_classification,
        PreparedPhysicsClassification::Kinematic
    );
}

#[test]
fn light_layers_are_deterministic_and_include_disabled_lights() {
    let mut disabled = light(20);
    disabled.initially_enabled = false;
    let lights = [light(30), disabled, light(10)];
    let sorted = sorted_shadow_lights(&lights).unwrap();
    assert_eq!(
        sorted
            .iter()
            .map(|light| light.reference_form_id)
            .collect::<Vec<_>>(),
        [10, 20, 30]
    );

    let duplicates = [light(10), light(10)];
    assert!(sorted_shadow_lights(&duplicates).is_err());
}

#[test]
fn nearest_hit_respects_range_and_empty_rays() {
    let mut triangles = vec![triangle(false)];
    let bvh = Bvh::build(&mut triangles);
    let origin = Point3::origin();
    let negative_z = Vec3::NEG_Z;
    let ray = Ray::new(origin, vector3(negative_z));
    assert_eq!(
        nearest_hit_distance(&bvh, &triangles, &ray, origin, negative_z, 3.0),
        Some(2.0)
    );
    assert_eq!(
        nearest_hit_distance(&bvh, &triangles, &ray, origin, negative_z, 1.0),
        None
    );
    let positive_z = Vec3::Z;
    let empty_ray = Ray::new(origin, vector3(positive_z));
    assert_eq!(
        nearest_hit_distance(&bvh, &triangles, &empty_ray, origin, positive_z, 3.0),
        None
    );
}

#[test]
fn fingerprint_tracks_depth_inputs_but_not_light_appearance() {
    let root = temp_directory("fingerprint");
    fs::create_dir_all(root.join("assets")).unwrap();
    fs::write(root.join("assets/caster.glb"), b"geometry-a").unwrap();
    let caster = placement("assets/caster.glb");
    let source_light = light(40);
    let fingerprint =
        |revision: &str, caster: &PreparedPlacement, light: &PreparedLight, resolution| {
            shadow_fingerprint_with_revision(
                revision,
                &root,
                &[caster],
                &[light],
                resolution,
                STATIC_POINT_SHADOW_NEAR_Z,
            )
            .unwrap()
        };
    let baseline = fingerprint(STATIC_POINT_SHADOW_REVISION, &caster, &source_light, 256);

    fs::write(root.join("assets/second-caster.glb"), b"geometry-c").unwrap();
    let second_caster = placement("assets/second-caster.glb");
    let expanded_caster_set = shadow_fingerprint_with_revision(
        STATIC_POINT_SHADOW_REVISION,
        &root,
        &[&caster, &second_caster],
        &[&source_light],
        256,
        STATIC_POINT_SHADOW_NEAR_Z,
    )
    .unwrap();
    assert_ne!(baseline, expanded_caster_set);

    let mut appearance = source_light.clone();
    appearance.color_rgba = [0.0, 1.0, 0.0, 1.0];
    appearance.intensity_lumens = 99_000.0;
    assert_eq!(
        baseline,
        fingerprint(STATIC_POINT_SHADOW_REVISION, &caster, &appearance, 256)
    );

    let mut moved_light = source_light.clone();
    moved_light.translation[0] += 1.0;
    assert_ne!(
        baseline,
        fingerprint(STATIC_POINT_SHADOW_REVISION, &caster, &moved_light, 256)
    );
    let mut ranged_light = source_light.clone();
    ranged_light.radius += 1.0;
    assert_ne!(
        baseline,
        fingerprint(STATIC_POINT_SHADOW_REVISION, &caster, &ranged_light, 256)
    );
    let mut moved_caster = caster.clone();
    moved_caster.translation[1] += 1.0;
    assert_ne!(
        baseline,
        fingerprint(
            STATIC_POINT_SHADOW_REVISION,
            &moved_caster,
            &source_light,
            256
        )
    );
    assert_ne!(
        baseline,
        fingerprint(STATIC_POINT_SHADOW_REVISION, &caster, &source_light, 128)
    );
    assert_ne!(
        baseline,
        fingerprint("different-revision", &caster, &source_light, 256)
    );
    fs::write(root.join("assets/caster.glb"), b"geometry-b").unwrap();
    assert_ne!(
        baseline,
        fingerprint(STATIC_POINT_SHADOW_REVISION, &caster, &source_light, 256)
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn glb_parser_applies_recursive_nodes_and_reads_indexed_and_unindexed_triangles() {
    let root = temp_directory("glb");
    let path = root.join("triangles.glb");
    write_test_glb(&path);

    let triangles = load_glb_triangles(&path).unwrap();
    assert_eq!(triangles.len(), 2);
    assert!(triangles[0][0].abs_diff_eq(Vec3::new(1.0, 2.0, 0.0), 1e-6));
    assert!(triangles[0][2].abs_diff_eq(Vec3::new(1.0, 3.0, 0.0), 1e-6));
    assert!(triangles[1][0].abs_diff_eq(Vec3::new(1.0, 2.0, 1.0), 1e-6));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn ktx_arguments_match_the_prepared_depth_cube_array_contract() {
    let raw = (0..12)
        .map(|index| PathBuf::from(format!("face-{index}.raw")))
        .collect::<Vec<_>>();
    let arguments = ktx_create_arguments(256, 2, &raw, Path::new("output.ktx2"))
        .into_iter()
        .map(|argument| argument.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        &arguments[..17],
        [
            "create",
            "--raw",
            "--format",
            "D32_SFLOAT",
            "--width",
            "256",
            "--height",
            "256",
            "--layers",
            "2",
            "--cubemap",
            "--assign-tf",
            "linear",
            "--assign-texcoord-origin",
            "top-left",
            "--zstd",
            "3",
        ]
    );
    assert_eq!(arguments.len(), 30);
    assert_eq!(arguments.last().unwrap(), "output.ktx2");
}

#[test]
fn ktx_input_list_arguments_keep_face_paths_off_the_command_line() {
    let arguments = ktx_create_arguments_with_input(
        256,
        47,
        OsString::from("@raw-files.txt"),
        Path::new("output.ktx2"),
    )
    .into_iter()
    .map(|argument| argument.to_string_lossy().into_owned())
    .collect::<Vec<_>>();

    assert_eq!(arguments.len(), 19);
    assert_eq!(arguments[17], "@raw-files.txt");
    assert_eq!(arguments[18], "output.ktx2");
}

#[test]
fn atomic_replace_commits_complete_output() {
    let root = temp_directory("atomic");
    let source = root.join("source.tmp");
    let destination = root.join("artifact.ktx2");
    fs::write(&source, b"new").unwrap();
    fs::write(&destination, b"old").unwrap();
    atomic_replace(&source, &destination).unwrap();
    assert_eq!(fs::read(&destination).unwrap(), b"new");
    assert!(!source.exists());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn installed_ktx_software_validates_depth_cubemap_array_when_opted_in() {
    if std::env::var_os("BEVYOUT_TEST_KTX").is_none() {
        return;
    }
    let root = temp_directory("ktx-integration");
    let output = root.join("artifact.ktx2");
    let ktx = find_unified_ktx_tool(None).unwrap();
    write_ktx2(
        &ktx.path,
        &root,
        "integration",
        &output,
        1,
        &vec![vec![0.0_f32]; FACE_COUNT],
    )
    .unwrap();
    assert!(output.is_file());
    fs::remove_dir_all(root).unwrap();
}

fn write_test_glb(path: &Path) {
    let mut binary = Vec::new();
    for position in [[0.0_f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] {
        for value in position {
            binary.extend_from_slice(&value.to_le_bytes());
        }
    }
    for index in [0_u16, 1, 2] {
        binary.extend_from_slice(&index.to_le_bytes());
    }
    while binary.len() % 4 != 0 {
        binary.push(0);
    }
    let second_positions_offset = binary.len();
    for position in [[0.0_f32, 0.0, 1.0], [1.0, 0.0, 1.0], [0.0, 1.0, 1.0]] {
        for value in position {
            binary.extend_from_slice(&value.to_le_bytes());
        }
    }
    let mut json = format!(
            concat!(
                "{{\"asset\":{{\"version\":\"2.0\"}},",
                "\"scene\":0,\"scenes\":[{{\"nodes\":[0]}}],",
                "\"nodes\":[{{\"translation\":[1,0,0],\"children\":[1]}},",
                "{{\"translation\":[0,2,0],\"mesh\":0}}],",
                "\"meshes\":[{{\"primitives\":[",
                "{{\"attributes\":{{\"POSITION\":0}},\"indices\":1,\"mode\":4}},",
                "{{\"attributes\":{{\"POSITION\":2}},\"mode\":4}}]}}],",
                "\"buffers\":[{{\"byteLength\":{buffer_len}}}],",
                "\"bufferViews\":[",
                "{{\"buffer\":0,\"byteOffset\":0,\"byteLength\":36}},",
                "{{\"buffer\":0,\"byteOffset\":36,\"byteLength\":6}},",
                "{{\"buffer\":0,\"byteOffset\":{second_offset},\"byteLength\":36}}],",
                "\"accessors\":[",
                "{{\"bufferView\":0,\"componentType\":5126,\"count\":3,\"type\":\"VEC3\",\"min\":[0,0,0],\"max\":[1,1,0]}},",
                "{{\"bufferView\":1,\"componentType\":5123,\"count\":3,\"type\":\"SCALAR\"}},",
                "{{\"bufferView\":2,\"componentType\":5126,\"count\":3,\"type\":\"VEC3\",\"min\":[0,0,1],\"max\":[1,1,1]}}]}}"
            ),
            buffer_len = binary.len(),
            second_offset = second_positions_offset,
        )
        .into_bytes();
    while json.len() % 4 != 0 {
        json.push(b' ');
    }
    let total_len = 12 + 8 + json.len() + 8 + binary.len();
    let mut glb = Vec::with_capacity(total_len);
    glb.extend_from_slice(b"glTF");
    glb.extend_from_slice(&2_u32.to_le_bytes());
    glb.extend_from_slice(&(total_len as u32).to_le_bytes());
    glb.extend_from_slice(&(json.len() as u32).to_le_bytes());
    glb.extend_from_slice(&0x4E4F_534A_u32.to_le_bytes());
    glb.extend_from_slice(&json);
    glb.extend_from_slice(&(binary.len() as u32).to_le_bytes());
    glb.extend_from_slice(&0x004E_4942_u32.to_le_bytes());
    glb.extend_from_slice(&binary);
    fs::write(path, glb).unwrap();
}
