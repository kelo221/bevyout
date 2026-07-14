use super::super::manifest::PreparedRuntimeMutability;
use super::*;

#[test]
fn bake_job_emits_resolved_cell_directional_light() {
    let lighting = PreparedCellLighting {
        directional_rgba: [0.5, 0.5, 0.5, 1.0],
        directional_fade: 2.0,
        ..Default::default()
    };
    assert_eq!(cell_directional_illuminance(&lighting), 10_000.0);
    assert_eq!(
        cell_directional_illuminance(&PreparedCellLighting::default()),
        0.0
    );
}

#[test]
fn only_static_semantics_are_batchable_without_changing_bake_inclusion() {
    fn placement(semantic: PreparedSemantic) -> PreparedPlacement {
        PreparedPlacement {
            reference_form_id: 1,
            base_form_id: 2,
            asset_path: Some("assets/test.glb".into()),
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            error: None,
            physics_asset_path: None,
            physics_source: None,
            physics_classification: PreparedPhysicsClassification::Static,
            step_support: false,
            mutability: PreparedRuntimeMutability::Immutable,
            mutability_root_form_id: None,
            reference_kind: "REFR".into(),
            base_kind: "STAT".into(),
            editor_id: None,
            display_name: None,
            count: 1,
            semantic,
            initially_enabled: true,
            enable_parent: None,
            owner_form_id: None,
            owner_faction_rank: None,
            inventory: Vec::new(),
            audio: Default::default(),
            ao_mode: "ao-none".into(),
        }
    }

    let static_placement = placement(PreparedSemantic::Static);
    assert!(is_bake_static(&static_placement));
    assert!(is_batchable_static(&static_placement));

    let mut dynamic_placement = placement(PreparedSemantic::Static);
    dynamic_placement.physics_classification = PreparedPhysicsClassification::Dynamic;
    assert!(!is_bake_static(&dynamic_placement));
    assert!(!is_batchable_static(&dynamic_placement));

    for semantic in [
        PreparedSemantic::Furniture,
        PreparedSemantic::Npc(super::super::manifest::PreparedActor {
            base_template_form_id: None,
        }),
        PreparedSemantic::Creature(super::super::manifest::PreparedActor {
            base_template_form_id: None,
        }),
        PreparedSemantic::Unsupported,
    ] {
        let placement = placement(semantic);
        assert!(is_bake_static(&placement));
        assert!(!is_batchable_static(&placement));
    }
}

#[test]
fn cell_directional_illuminance_clamps_non_finite_and_negative_luminance() {
    fn lighting_with(directional_rgba: [f32; 4]) -> PreparedCellLighting {
        PreparedCellLighting {
            directional_rgba,
            ..Default::default()
        }
    }
    // Sum exactly at the epsilon threshold is still treated as dark.
    assert_eq!(
        cell_directional_illuminance(&lighting_with([f32::EPSILON, 0.0, 0.0, 1.0])),
        0.0
    );
    // Just above the threshold is bright.
    assert_eq!(
        cell_directional_illuminance(&lighting_with([f32::EPSILON * 2.0, 0.0, 0.0, 1.0])),
        10_000.0
    );
    // Negative color channels summing below zero are treated as dark.
    assert_eq!(
        cell_directional_illuminance(&lighting_with([-1.0, 0.2, 0.2, 1.0])),
        0.0
    );
    // NaN/infinite sums must not propagate into the bake job.
    assert_eq!(
        cell_directional_illuminance(&lighting_with([f32::NAN, 0.0, 0.0, 1.0])),
        0.0
    );
    assert_eq!(
        cell_directional_illuminance(&lighting_with([f32::INFINITY, 0.0, 0.0, 1.0])),
        0.0
    );
}

#[test]
fn pickup_container_door_and_activator_are_excluded_from_the_bake() {
    fn placement(semantic: PreparedSemantic) -> PreparedPlacement {
        PreparedPlacement {
            reference_form_id: 1,
            base_form_id: 2,
            asset_path: Some("assets/test.glb".into()),
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            error: None,
            physics_asset_path: None,
            physics_source: None,
            physics_classification: PreparedPhysicsClassification::Static,
            step_support: false,
            reference_kind: "REFR".into(),
            base_kind: "STAT".into(),
            editor_id: None,
            display_name: None,
            count: 1,
            semantic,
            initially_enabled: true,
            enable_parent: None,
            owner_form_id: None,
            owner_faction_rank: None,
            inventory: Vec::new(),
            audio: Default::default(),
            ao_mode: "ao-none".into(),
            mutability: PreparedRuntimeMutability::Unknown,
            mutability_root_form_id: None,
        }
    }
    for semantic in [
        PreparedSemantic::Pickup(super::super::manifest::PreparedPickup {
            category: "MISC".into(),
            value: Some(10),
            weight: Some(1.0),
        }),
        PreparedSemantic::Container,
        PreparedSemantic::Door(super::super::manifest::PreparedDoor {
            lock_level: None,
            key_form_id: None,
            destination: None,
        }),
        PreparedSemantic::Activator,
    ] {
        let placement = placement(semantic);
        assert!(!is_bake_static(&placement));
        assert!(!is_batchable_static(&placement));
    }
}

#[test]
fn ktx_tool_kind_is_unified_only_for_ktx_named_executables() {
    assert!(matches!(
        ktx_tool_kind(Path::new("/usr/local/bin/ktx")),
        KtxToolKind::UnifiedKtx
    ));
    assert!(matches!(
        ktx_tool_kind(Path::new("C:/Program Files/KTX-Software/bin/ktx.exe")),
        KtxToolKind::UnifiedKtx
    ));
    // Case-insensitive on the file stem.
    assert!(matches!(
        ktx_tool_kind(Path::new("KTX.EXE")),
        KtxToolKind::UnifiedKtx
    ));
    assert!(matches!(
        ktx_tool_kind(Path::new("/usr/local/bin/toktx")),
        KtxToolKind::LegacyToktx
    ));
    // Anything else falls back to the legacy tool, even if it isn't
    // actually toktx by name.
    assert!(matches!(
        ktx_tool_kind(Path::new("/usr/local/bin/some-other-tool")),
        KtxToolKind::LegacyToktx
    ));
}

#[test]
fn bake_contribution_requires_every_reference_exactly_once() {
    fn job_placement(reference_form_id: u32) -> JobPlacement {
        JobPlacement {
            reference_form_id,
            asset_path: "assets/test.glb".into(),
            ao_mode: "ao-none".into(),
            batchable_static: true,
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
        }
    }
    let expected = vec![job_placement(0x10), job_placement(0x20)];
    validate_placement_contribution(
        &expected,
        &BlenderPlacementContribution {
            expected_placements: 2,
            contributed_placements: 2,
            reference_form_ids: vec![0x20, 0x10],
        },
    )
    .unwrap();

    let error = validate_placement_contribution(
        &expected,
        &BlenderPlacementContribution {
            expected_placements: 2,
            contributed_placements: 1,
            reference_form_ids: vec![0x10],
        },
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("00000020"));
}

#[test]
fn relative_asset_path_requires_the_path_to_be_inside_root() {
    let root = Path::new("/cache/assets");
    assert_eq!(
        relative_asset_path(root, Path::new("/cache/assets/scenes/00000001/scene.glb")).unwrap(),
        "scenes/00000001/scene.glb"
    );
    let error = relative_asset_path(root, Path::new("/elsewhere/scene.glb")).unwrap_err();
    assert!(error.to_string().contains("is outside asset root"));
}

#[test]
fn blender_path_strips_extended_length_prefix_only() {
    assert_eq!(
        blender_path(Path::new(r"\\?\C:\cache\assets\scene.glb")),
        r"C:\cache\assets\scene.glb"
    );
    // Paths without the prefix pass through unchanged.
    assert_eq!(
        blender_path(Path::new(r"C:\cache\assets\scene.glb")),
        r"C:\cache\assets\scene.glb"
    );
}

#[test]
fn tail_keeps_short_output_verbatim_and_truncates_long_output() {
    let short = "line1\nline2\nline3";
    assert_eq!(tail(short.as_bytes()), short);

    let long = (1..=50)
        .map(|index| format!("line{index}"))
        .collect::<Vec<_>>()
        .join("\n");
    let expected = (11..=50)
        .map(|index| format!("line{index}"))
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(tail(long.as_bytes()), expected);
}

#[test]
fn find_ktx_tool_errors_on_a_nonexistent_explicit_path() {
    let missing = std::env::temp_dir().join(format!(
        "bevyout-missing-ktx-{}-does-not-exist",
        std::process::id()
    ));
    let error = find_ktx_tool(Some(missing)).unwrap_err();
    assert!(error.to_string().contains("does not exist"));
}

#[test]
fn find_irradiance_ktx_tool_rejects_legacy_toktx_before_checking_existence() {
    // A path named "toktx" is rejected for being the wrong tool even
    // though the path itself does not exist; this must not depend on
    // any real KTX-Software install.
    let missing = std::env::temp_dir().join(format!(
        "bevyout-missing-toktx-{}-does-not-exist",
        std::process::id()
    ));
    let error = find_irradiance_ktx_tool(Some(missing)).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("requires the unified KTX executable")
    );
}

#[test]
fn find_irradiance_ktx_tool_errors_when_unified_path_is_missing() {
    // The file stem must be exactly "ktx" for `ktx_tool_kind` to classify
    // it as the unified tool; nest it under a directory that does not
    // exist so the path itself is guaranteed missing.
    let named_ktx = std::env::temp_dir()
        .join(format!("bevyout-missing-ktx-dir-{}", std::process::id()))
        .join("ktx.exe");
    let error = find_irradiance_ktx_tool(Some(named_ktx)).unwrap_err();
    assert!(error.to_string().contains("does not exist"));
}

#[test]
fn find_irradiance_blender_errors_on_a_nonexistent_explicit_path() {
    let missing = std::env::temp_dir().join(format!(
        "bevyout-missing-blender-{}-does-not-exist",
        std::process::id()
    ));
    let error = find_irradiance_blender(Some(&missing), None).unwrap_err();
    assert!(error.to_string().contains("does not exist"));
}

#[test]
fn validate_blender_45_errors_on_a_nonexistent_path() {
    let missing = std::env::temp_dir().join(format!(
        "bevyout-missing-blender45-{}-does-not-exist",
        std::process::id()
    ));
    let error = validate_blender_45(&missing).unwrap_err();
    assert!(error.to_string().contains("does not exist"));
}
