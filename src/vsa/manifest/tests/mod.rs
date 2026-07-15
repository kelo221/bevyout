use super::*;

#[test]
fn schema_one_through_four_manifests_remain_readable_without_new_metadata() {
    let template = r#"(
            schema_version: SCHEMA,
            asset_root: "cache",
            source_plugin: "Fallout3.esm",
            source_fingerprint: "fingerprint",
            cell: (
                form_id: 1,
                editor_id: None,
                name: None,
                interior: true,
                ambient_rgba: (0.0, 0.0, 0.0, 0.0),
                directional_rgba: (0.0, 0.0, 0.0, 0.0),
            ),
            placements: [],
            lights: [],
            diagnostics: [],
        )"#;
    for version in [1, 2, 3, 4] {
        let text = template.replace("SCHEMA", &version.to_string());
        let manifest: PreparedSceneManifest = ron::de::from_str(&text).unwrap();
        assert_eq!(manifest.schema_version, version);
        assert!(manifest.bake.is_none());
        assert!(manifest.source_plugins.is_empty());
        assert!(manifest.navmeshes.is_empty());
        assert!(manifest.footstep_sets.is_empty());
        assert!(manifest.hard_landing_clips.is_empty());
    }
}

#[test]
fn prepared_artifact_versions_require_exact_pipeline_identity() {
    let manifest: PreparedSceneManifest = ron::de::from_str(&format!(
        r#"(
                schema_version: {},
                prepare_revision: Some("{}"),
                converter_revision: Some("converter-v1"),
                physics_schema_version: Some(1),
                asset_root: "cache",
                source_plugin: "Fallout3.esm",
                source_fingerprint: "fingerprint",
                cell: (
                    form_id: 1,
                    editor_id: None,
                    name: None,
                    interior: true,
                    ambient_rgba: (0.0, 0.0, 0.0, 0.0),
                    directional_rgba: (0.0, 0.0, 0.0, 0.0),
                ),
                placements: [],
                lights: [],
                diagnostics: [],
            )"#,
        CURRENT_MANIFEST_SCHEMA_VERSION, CURRENT_PREPARE_REVISION
    ))
    .unwrap();
    ensure_prepared_manifest_compatible(&manifest, "converter-v1", 1).unwrap();

    let mut future = manifest.clone();
    future.schema_version += 1;
    let error = ensure_prepared_manifest_compatible(&future, "converter-v1", 1)
        .unwrap_err()
        .to_string();
    assert!(error.contains("newer than"));

    let mut stale = manifest.clone();
    stale.prepare_revision = Some("prepare-old".into());
    let error = ensure_prepared_manifest_compatible(&stale, "converter-v1", 1)
        .unwrap_err()
        .to_string();
    assert!(error.contains("prepare revision"));

    let mut stale_bake = manifest;
    stale_bake.bake = Some(PreparedBake {
        bake_revision: Some("bake-old".into()),
        source_fingerprint: "fingerprint".into(),
        scene_path: "scenes/00000001/baked/scene.glb".into(),
        irradiance_volume: None,
    });
    let error = ensure_baked_scene_compatible(&stale_bake)
        .unwrap_err()
        .to_string();
    assert!(error.contains("bake revision"));
}

#[test]
fn legacy_footstep_set_defaults_landing_clips() {
    let set: PreparedFootstepSet = ron::de::from_str(
        r#"(
                surface: "concrete",
                left: [],
                right: [],
            )"#,
    )
    .unwrap();
    assert!(set.land.is_empty());
}

#[test]
fn legacy_placement_defaults_to_static_semantics() {
    let text = r#"(
            reference_form_id: 1,
            base_form_id: 2,
            asset_path: None,
            translation: (0.0, 0.0, 0.0),
            rotation_xyzw: (0.0, 0.0, 0.0, 1.0),
            scale: 1.0,
            error: None,
        )"#;
    let placement: PreparedPlacement = ron::de::from_str(text).unwrap();
    assert_eq!(placement.semantic, PreparedSemantic::Static);
    assert_eq!(placement.count, 1);
    assert!(placement.initially_enabled);
    assert!(placement.inventory.is_empty());
    assert_eq!(placement.audio, PreparedPlacementAudio::default());
    assert!(!placement.step_support);
    // A legacy placement predates PreparedRuntimeMutability entirely; it
    // must default to Unknown, never be treated as Immutable scenery.
    assert_eq!(placement.mutability, PreparedRuntimeMutability::Unknown);
    assert!(placement.mutability_root_form_id.is_none());
}

#[test]
fn schema_five_lighting_round_trip_and_legacy_defaults() {
    let text = r#"(
            schema_version: 5,
            asset_root: "cache",
            source_plugin: "Fallout3.esm",
            source_fingerprint: "fingerprint",
            cell: (
                form_id: 1,
                editor_id: None,
                name: Some("Lit Cell"),
                interior: true,
                ambient_rgba: (0.1, 0.2, 0.3, 1.0),
                directional_rgba: (0.4, 0.5, 0.6, 1.0),
                raw_lighting: None,
                effective_lighting: Some((
                    ambient_rgba: (0.1, 0.2, 0.3, 1.0),
                    directional_rgba: (0.4, 0.5, 0.6, 1.0),
                    fog_rgba: (0.01, 0.02, 0.03, 1.0),
                    fog_near: 10.0,
                    fog_far: 100.0,
                    directional_rotation_xy: 5,
                    directional_rotation_z: 15,
                    directional_fade: 0.75,
                    fog_clip_distance: 80.0,
                    fog_power: 2.0,
                )),
            ),
            placements: [],
            lights: [],
            diagnostics: [],
        )"#;
    let manifest: PreparedSceneManifest = ron::de::from_str(text).unwrap();
    assert_eq!(manifest.schema_version, 5);
    assert_eq!(
        manifest.cell.effective_lighting.as_ref().unwrap().fog_far,
        100.0
    );
    let encoded = ron::ser::to_string(&manifest).unwrap();
    let decoded: PreparedSceneManifest = ron::de::from_str(&encoded).unwrap();
    assert_eq!(
        decoded.cell.effective_lighting,
        manifest.cell.effective_lighting
    );

    let legacy = r#"(
            form_id: 1,
            editor_id: None,
            name: None,
            interior: true,
            ambient_rgba: (0.0, 0.0, 0.0, 0.0),
            directional_rgba: (0.0, 0.0, 0.0, 0.0),
        )"#;
    let legacy_cell: CellInfo = ron::de::from_str(legacy).unwrap();
    assert!(legacy_cell.raw_lighting.is_none());
    assert!(legacy_cell.effective_lighting.is_none());
}

// --- Issue #38: PreparedRuntimeMutability schema -------------------------

// T38.4: golden schema test. Serialize a minimal manifest (one placement of
// each interesting mutability class) and assert the current schema version
// plus the new fields are present and survive an RON round trip.
#[test]
fn current_schema_mutability_and_static_shadows_round_trip_through_ron() {
    fn placement(
        reference_form_id: u32,
        mutability: PreparedRuntimeMutability,
        mutability_root_form_id: Option<u32>,
    ) -> PreparedPlacement {
        PreparedPlacement {
            reference_form_id,
            base_form_id: 1,
            asset_path: None,
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            error: None,
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
            semantic: PreparedSemantic::Static,
            initially_enabled: true,
            enable_parent: None,
            owner_form_id: None,
            owner_faction_rank: None,
            inventory: Vec::new(),
            audio: PreparedPlacementAudio::default(),
            ao_mode: "ao-none".into(),
        }
    }

    let manifest = PreparedSceneManifest {
        schema_version: CURRENT_MANIFEST_SCHEMA_VERSION,
        prepare_revision: Some(CURRENT_PREPARE_REVISION.into()),
        converter_revision: Some("converter-v1".into()),
        physics_schema_version: Some(1),
        asset_root: "cache".into(),
        source_plugin: "Fallout3.esm".into(),
        source_fingerprint: "fingerprint".into(),
        item_catalog_path: None,
        item_catalog_revision: None,
        item_catalog_hash: None,
        source_plugins: Vec::new(),
        cell: ron::de::from_str(
            r#"(
                form_id: 1,
                editor_id: None,
                name: None,
                interior: true,
                ambient_rgba: (0.0, 0.0, 0.0, 0.0),
                directional_rgba: (0.0, 0.0, 0.0, 0.0),
            )"#,
        )
        .unwrap(),
        placements: vec![
            placement(0x1, PreparedRuntimeMutability::Immutable, None),
            placement(0x2, PreparedRuntimeMutability::EnableGroup, Some(0x9)),
            placement(0x3, PreparedRuntimeMutability::ScriptAddressable, None),
            placement(0x4, PreparedRuntimeMutability::Unknown, None),
        ],
        lights: Vec::new(),
        diagnostics: Vec::new(),
        visual_issues: vec![PreparedVisualIssue {
            code: "unreviewed_root_transform".into(),
            severity: "warning".into(),
            model_path: "architecture/test.nif".into(),
            base_form_ids: vec![0x10],
            reference_form_ids: vec![0x20],
            message: "review root".into(),
        }],
        navmeshes: Vec::new(),
        cell_audio: PreparedCellAudio::default(),
        audio_clips: Vec::new(),
        footstep_sets: Vec::new(),
        hard_landing_clips: Vec::new(),
        bake: None,
        static_point_shadows: Some(PreparedStaticPointShadows {
            revision: STATIC_POINT_SHADOW_REVISION.into(),
            source_fingerprint: "shadow-fingerprint".into(),
            asset_path: "scenes/00000001/shadows/shadow-fingerprint.ktx2".into(),
            resolution: 256,
            near_z: 0.1,
            lights: vec![PreparedStaticPointShadowLight {
                reference_form_id: 0x1234,
                layer: 0,
                translation: [1.0, 2.0, 3.0],
                range: 8.0,
            }],
        }),
        mutability_summary: PreparedMutabilitySummary {
            immutable: 1,
            enable_group: 1,
            script_addressable: 1,
            unknown: 1,
        },
    };

    assert_eq!(manifest.schema_version, CURRENT_MANIFEST_SCHEMA_VERSION);

    let encoded = ron::ser::to_string(&manifest).unwrap();
    assert!(encoded.contains("mutability"));
    assert!(encoded.contains("mutability_summary"));
    assert!(encoded.contains("EnableGroup"));
    assert!(encoded.contains("static_point_shadows"));
    assert!(encoded.contains("unreviewed_root_transform"));

    let decoded: PreparedSceneManifest = ron::de::from_str(&encoded).unwrap();
    assert_eq!(decoded.schema_version, CURRENT_MANIFEST_SCHEMA_VERSION);
    assert_eq!(decoded.mutability_summary, manifest.mutability_summary);
    assert_eq!(
        decoded.placements[1].mutability,
        PreparedRuntimeMutability::EnableGroup
    );
    assert_eq!(decoded.placements[1].mutability_root_form_id, Some(0x9));
    assert_eq!(decoded.static_point_shadows, manifest.static_point_shadows);
    assert_eq!(decoded.visual_issues, manifest.visual_issues);
}

// T38.5: an old-schema manifest fails compatibility with a precise
// "re-run prepare for <cell>" instruction, extending the existing
// ensure_prepared_manifest_compatible mechanism rather than a new one.
#[test]
fn old_schema_manifest_fails_with_a_precise_reprepare_instruction() {
    let previous_schema = CURRENT_MANIFEST_SCHEMA_VERSION - 1;
    let text = format!(
        r#"(
            schema_version: {previous_schema},
            prepare_revision: Some("{}"),
            converter_revision: Some("converter-v1"),
            physics_schema_version: Some(1),
            asset_root: "cache",
            source_plugin: "Fallout3.esm",
            source_fingerprint: "fingerprint",
            cell: (
                form_id: 305,
                editor_id: Some("SuperDuperMart"),
                name: None,
                interior: true,
                ambient_rgba: (0.0, 0.0, 0.0, 0.0),
                directional_rgba: (0.0, 0.0, 0.0, 0.0),
            ),
            placements: [],
            lights: [],
            diagnostics: [],
        )"#,
        CURRENT_PREPARE_REVISION
    );
    let manifest: PreparedSceneManifest = ron::de::from_str(&text).unwrap();
    assert_eq!(manifest.schema_version, previous_schema);
    // Legacy manifests predating F38.1 must never be interpreted as if
    // their placements were classified Immutable.
    assert!(manifest.placements.is_empty());

    let error = ensure_prepared_manifest_compatible(&manifest, "converter-v1", 1)
        .unwrap_err()
        .to_string();
    assert!(error.contains("older than"));
    assert!(error.contains(&previous_schema.to_string()));
    assert!(error.contains(&CURRENT_MANIFEST_SCHEMA_VERSION.to_string()));
    assert!(error.contains("SuperDuperMart"));
    assert!(error.contains("run `prepare` again"));
}

#[test]
fn schema_three_semantics_round_trip_through_ron() {
    let placement = PreparedPlacement {
        reference_form_id: 1,
        base_form_id: 2,
        asset_path: Some("assets/door.glb".into()),
        translation: [1.0, 2.0, 3.0],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        error: None,
        physics_asset_path: None,
        physics_source: None,
        physics_classification: PreparedPhysicsClassification::Static,
        step_support: false,
        mutability: PreparedRuntimeMutability::ScriptAddressable,
        mutability_root_form_id: None,
        reference_kind: "REFR".into(),
        base_kind: "DOOR".into(),
        editor_id: Some("TestDoor".into()),
        display_name: Some("Test Door".into()),
        count: 1,
        semantic: PreparedSemantic::Door(PreparedDoor {
            lock_level: Some(75),
            key_form_id: Some(3),
            destination: Some(PreparedDoorDestination {
                door_reference_form_id: 4,
                cell_form_id: 5,
                translation: [4.0, 5.0, 6.0],
                rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            }),
        }),
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        inventory: Vec::new(),
        audio: PreparedPlacementAudio::default(),
        ao_mode: "ao-none".into(),
    };
    let text = ron::ser::to_string(&placement).unwrap();
    let decoded: PreparedPlacement = ron::de::from_str(&text).unwrap();
    assert_eq!(decoded.semantic, placement.semantic);
    assert_eq!(decoded.reference_kind, "REFR");
    assert_eq!(decoded.base_kind, "DOOR");
}
