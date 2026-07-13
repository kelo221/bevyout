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
