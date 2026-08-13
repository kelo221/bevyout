use super::super::{CURRENT_MANIFEST_SCHEMA_VERSION, CURRENT_PREPARE_REVISION};
use super::*;
use bevyout_core::manifest::exterior::PreparedExteriorEnvironment;
use bevyout_core::manifest::{CellInfo, PreparedSceneManifest};

fn compact_manifest(root: &Path) -> PreparedSceneManifest {
    PreparedSceneManifest {
        schema_version: CURRENT_MANIFEST_SCHEMA_VERSION,
        prepare_revision: Some(CURRENT_PREPARE_REVISION.into()),
        converter_revision: None,
        physics_schema_version: None,
        asset_root: root.to_string_lossy().into_owned(),
        source_plugin: "Fallout3.esm".into(),
        source_fingerprint: "content".into(),
        item_catalog_path: None,
        item_catalog_revision: None,
        item_catalog_hash: None,
        recipe_catalog_path: None,
        recipe_catalog_revision: None,
        recipe_catalog_hash: None,
        actor_catalog_path: None,
        actor_catalog_revision: None,
        actor_catalog_hash: None,
        actor_animation_catalog_path: None,
        actor_animation_catalog_revision: None,
        actor_animation_catalog_hash: None,
        image_space_modifier_catalog_path: None,
        image_space_modifier_catalog_revision: None,
        image_space_modifier_catalog_hash: None,
        source_plugins: Vec::new(),
        cell: CellInfo {
            form_id: 0x0000_0cb8,
            editor_id: None,
            name: None,
            interior: false,
            behave_like_exterior: false,
            ambient_rgba: [0.0; 4],
            directional_rgba: [0.0; 4],
            image_space_form_id: None,
            image_space: None,
            lighting_template_form_id: None,
            lighting_template_flags: 0,
            lighting_template: None,
            raw_lighting: None,
            effective_lighting: None,
            water_form_id: None,
            water_height: None,
            grid: Some((17, -9)),
            worldspace_form_id: Some(0x3c),
            day_night_profile: None,
            day_night_preview_profile: None,
        },
        placements: Vec::new(),
        lights: Vec::new(),
        diagnostics: Vec::new(),
        visual_issues: Vec::new(),
        navmeshes: Vec::new(),
        nav_graph: None,
        cell_audio: Default::default(),
        audio_clips: Vec::new(),
        footstep_sets: Vec::new(),
        hard_landing_clips: Vec::new(),
        bake: None,
        static_point_shadows: None,
        reflection_probes: None,
        mutability_summary: Default::default(),
        leveled_lists: Default::default(),
        dialogue: None,
        exterior: None,
    }
}

#[test]
fn compact_exterior_scene_hydrates_the_canonical_package() {
    let root = std::env::temp_dir().join(format!(
        "bevyout-compact-exterior-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&root);
    let package_path = root.join("worldspaces/0000003c/cells/00000cb8.ron");
    fs::create_dir_all(package_path.parent().unwrap()).unwrap();
    let package = ExteriorCellPackage {
        revision: EXTERIOR_CELL_PACKAGE_REVISION.into(),
        content_fingerprint: "content".into(),
        cell_form_id: 0x0000_0cb8,
        worldspace_form_id: 0x3c,
        grid: bevyout_core::manifest::exterior::GridCoordinate::new(17, -9),
        origin: [0.0; 3],
        terrain: None,
        static_objects: Vec::new(),
        dynamic_objects: Vec::new(),
        distant_objects: Vec::new(),
        actors: Vec::new(),
        local_lights: Vec::new(),
        navigation: None,
        water: None,
        environment: PreparedExteriorEnvironment::default(),
        diagnostics: Vec::new(),
    };
    fs::write(&package_path, ron::ser::to_string(&package).unwrap()).unwrap();

    let mut manifest = compact_manifest(&root);
    hydrate_exterior_package(&mut manifest).unwrap();
    assert_eq!(manifest.exterior.unwrap(), package);
    let _ = fs::remove_dir_all(root);
}

/// Issue #299: a package written under the previous
/// `EXTERIOR_CELL_PACKAGE_REVISION` (before the `actors` field existed) must
/// be rejected rather than silently hydrated with a missing actor list. This
/// is the staleness gate the "Prepared asset revisions" policy exists for.
#[test]
fn stale_exterior_package_revision_is_rejected() {
    let root = std::env::temp_dir().join(format!(
        "bevyout-stale-exterior-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&root);
    let package_path = root.join("worldspaces/0000003c/cells/00000cb8.ron");
    fs::create_dir_all(package_path.parent().unwrap()).unwrap();
    let package = ExteriorCellPackage {
        revision: "exterior-cell-package-v9-shared-weather-catalog-object-store-terrain".into(),
        content_fingerprint: "content".into(),
        cell_form_id: 0x0000_0cb8,
        worldspace_form_id: 0x3c,
        grid: bevyout_core::manifest::exterior::GridCoordinate::new(17, -9),
        origin: [0.0; 3],
        terrain: None,
        static_objects: Vec::new(),
        dynamic_objects: Vec::new(),
        distant_objects: Vec::new(),
        actors: Vec::new(),
        local_lights: Vec::new(),
        navigation: None,
        water: None,
        environment: PreparedExteriorEnvironment::default(),
        diagnostics: Vec::new(),
    };
    fs::write(&package_path, ron::ser::to_string(&package).unwrap()).unwrap();

    let mut manifest = compact_manifest(&root);
    let error = hydrate_exterior_package(&mut manifest).unwrap_err();
    assert!(
        error.to_string().contains(EXTERIOR_CELL_PACKAGE_REVISION),
        "expected stale-revision error to name the current revision, got: {error}"
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn compact_exterior_scene_serialization_omits_duplicate_payloads() {
    let manifest = compact_manifest(Path::new("cache"));
    let encoded = ron::ser::to_string_pretty(&manifest, ron::ser::PrettyConfig::default()).unwrap();
    assert!(
        !encoded
            .lines()
            .any(|line| line.trim() == "diagnostics: [],")
    );
    assert!(!encoded.lines().any(|line| line.trim() == "exterior: None,"));
}
