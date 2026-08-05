use super::super::manifest::PreparedRuntimeMutability;
use super::lightmap::{LightmapAtlas, LightmapPage};
use super::rust_scene::synthetic_lightmap_scene_for_test;
use super::*;

#[test]
fn backend_defaults_keep_cpu_quality_and_make_solari_fast() {
    assert_eq!(
        default_lightmap_texels_per_meter_for_backend(SelectedLightmapBackend::Cpu),
        16.0
    );
    assert_eq!(
        default_lightmap_tile_size_for_backend(SelectedLightmapBackend::Cpu),
        128
    );
    assert_eq!(
        default_static_batch_chunk_meters_for_backend(SelectedLightmapBackend::Cpu),
        64.0
    );
    assert_eq!(
        default_lightmap_texels_per_meter_for_backend(SelectedLightmapBackend::Solari),
        4.0
    );
    assert_eq!(
        default_lightmap_tile_size_for_backend(SelectedLightmapBackend::Solari),
        512
    );
    assert_eq!(
        default_static_batch_chunk_meters_for_backend(SelectedLightmapBackend::Solari),
        32.0
    );
}

#[test]
fn bake_progress_label_uses_a_concrete_backend_name() {
    assert_eq!(
        bake_operation_label(LightmapBackendPreference::Cpu),
        "CPU bake"
    );
    assert_eq!(
        bake_operation_label(LightmapBackendPreference::Solari),
        "GPU bake"
    );
    #[cfg(feature = "lightmap-gpu-solari")]
    assert_eq!(
        bake_operation_label(LightmapBackendPreference::Auto),
        "GPU bake"
    );
    #[cfg(not(feature = "lightmap-gpu-solari"))]
    assert_eq!(
        bake_operation_label(LightmapBackendPreference::Auto),
        "CPU bake"
    );
}

#[test]
fn flat_overlays_are_not_folded_into_the_lightmapped_static_scene() {
    let mut placement = PreparedPlacement {
        reference_form_id: 1,
        base_form_id: 2,
        asset_path: Some("assets/stain.glb".into()),
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
        editor_id: Some("Stain03".into()),
        display_name: None,
        count: 1,
        semantic: PreparedSemantic::Static,
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        linked_reference_form_id: None,
        inventory: Vec::new(),
        audio: Default::default(),
        ao_mode: "ao-none".into(),
    };
    assert!(!is_bake_static(&placement));
    placement.editor_id = Some("VaultWall01".into());
    assert!(is_bake_static(&placement));
}

#[test]
fn oversized_lightmap_pages_fail_before_backend_dispatch() {
    let mut scene = synthetic_lightmap_scene_for_test();
    scene.primitives[0].lightmap_dimensions = [4093, 8];

    let scale = super::lightmap::page_density_scale_to_fit(&scene, 4096)
        .unwrap()
        .unwrap();
    assert!((scale - 4092.0 / 4093.0).abs() < 1e-6);

    let error = super::lightmap::validate_page_dimensions(&scene, 4096).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("reduce --lightmap-texels-per-meter")
    );
}

#[test]
fn lightmap_binding_projection_preserves_primitive_identity_and_uv_rect() {
    let scene = synthetic_lightmap_scene_for_test();
    let pages = vec![LightmapPage {
        primitive_index: 0,
        width: 8,
        height: 6,
        raw_path: "synthetic.raw".into(),
        covered_texels: 12,
        dilated_texels: 9,
        atlas_index: 0,
        atlas_offset: [2, 3],
    }];
    let atlases = vec![LightmapAtlas {
        width: 16,
        height: 20,
        raw_path: "synthetic.ktx2".into(),
        content_hash: "hash".into(),
    }];

    let bindings = build_lightmap_bindings(&scene, &pages, &atlases).unwrap();

    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings[0].binding_id, 1);
    assert_eq!(bindings[0].primitive_key, "fixture/synthetic_triangle");
    assert_eq!(bindings[0].atlas_index, 0);
    assert_eq!(bindings[0].uv_rect, [0.125, 0.15, 0.625, 0.45]);
    assert_eq!(bindings[0].texels_per_meter, 4.0);
}

#[test]
fn duplicate_lightmap_density_overrides_are_rejected() {
    let overrides = vec![
        crate::cli::LightmapDensityOverrideArg {
            reference_form_id: 0x151e3,
            texels_per_meter: 16.0,
        },
        crate::cli::LightmapDensityOverrideArg {
            reference_form_id: 0x151e3,
            texels_per_meter: 32.0,
        },
    ];
    let error = validate_lightmap_density_overrides(&overrides).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("duplicate lightmap density override")
    );
}

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

#[path = "transport.rs"]
mod transport;

#[path = "denoise.rs"]
mod denoise;

#[path = "ktx2.rs"]
mod ktx2;

#[test]
fn only_static_semantics_are_batchable() {
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
            linked_reference_form_id: None,
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

    for mutability in [
        PreparedRuntimeMutability::EnableGroup,
        PreparedRuntimeMutability::ScriptAddressable,
        PreparedRuntimeMutability::Unknown,
    ] {
        let mut mutable_placement = placement(PreparedSemantic::Static);
        mutable_placement.mutability = mutability;
        assert!(!is_bake_static(&mutable_placement));
        assert!(!is_batchable_static(&mutable_placement));
    }

    let mut kinematic_placement = placement(PreparedSemantic::Static);
    kinematic_placement.physics_classification = PreparedPhysicsClassification::Kinematic;
    assert!(!is_bake_static(&kinematic_placement));
    assert!(!is_batchable_static(&kinematic_placement));
}

#[test]
fn item_record_kinds_never_enter_the_static_bake() {
    fn placement(kind: &str) -> PreparedPlacement {
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
            base_kind: kind.into(),
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
            audio: Default::default(),
            ao_mode: "ao-none".into(),
        }
    }

    for kind in [
        "WEAP", "AMMO", "ARMO", "ALCH", "MISC", "BOOK", "NOTE", "KEYM",
    ] {
        assert!(
            !is_bake_static(&placement(kind)),
            "{kind} should remain spawnable instead of entering the static bake"
        );
    }
    assert!(!is_bake_static(&placement("weap")));
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
fn non_static_semantics_are_excluded_from_the_bake() {
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
            linked_reference_form_id: None,
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
            trapped: false,
            destination: None,
        }),
        PreparedSemantic::Activator,
        PreparedSemantic::Furniture,
        PreparedSemantic::Npc(super::super::manifest::PreparedActor {
            base_template_form_id: None,
            ..Default::default()
        }),
        PreparedSemantic::Creature(super::super::manifest::PreparedActor {
            base_template_form_id: None,
            ..Default::default()
        }),
        PreparedSemantic::Unsupported,
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
fn job_path_strips_extended_length_prefix_only() {
    assert_eq!(
        job_path(Path::new(r"\\?\C:\cache\assets\scene.glb")),
        r"C:\cache\assets\scene.glb"
    );
    // Paths without the prefix pass through unchanged.
    assert_eq!(
        job_path(Path::new(r"C:\cache\assets\scene.glb")),
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
