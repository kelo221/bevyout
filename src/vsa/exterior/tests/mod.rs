use super::super::openmw_esm4::{
    BaseRecord, LandRecord, LandTextureAssignment, LandTextureWeight, ParsedPlugin, ReferenceKind,
    ReferenceRecord,
};
use super::super::openmw_esm4::{LandscapeTextureRecord, TextureSetRecord};
use super::terrain_from_land;
use bevyout_core::actor::{ActorAssemblyBlueprint, ActorFallbackDecision, ActorKind};
use bevyout_core::manifest::exterior::PreparedTerrain;
use bevyout_core::manifest::exterior::{
    ExteriorCellPackage, ExteriorCoordinatePolicy, GridCoordinate, PreparedExteriorEnvironment,
    PreparedExteriorObject,
};
use bevyout_core::manifest::{CellInfo, PreparedActor, PreparedPlacement, PreparedSemantic};
use std::collections::HashMap;
use std::fs;

fn synthetic_exterior_cell() -> CellInfo {
    CellInfo {
        form_id: 0x0c67,
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
        grid: Some((3, -1)),
        worldspace_form_id: Some(0x3c),
        day_night_profile: None,
        day_night_preview_profile: None,
    }
}

fn synthetic_actor_reference(
    kind: ReferenceKind,
    form_id: u32,
    base_form_id: u32,
) -> ReferenceRecord {
    ReferenceRecord {
        kind,
        form_id,
        parent_cell_form_id: 0x0c67,
        base_form_id,
        position: [10.0, 20.0, 30.0],
        rotation: [0.0, 0.0, 0.0],
        scale: 1.0,
        count: 1,
        initially_enabled: true,
        ..Default::default()
    }
}

/// `BaseRecord::ignored_subrecords` is private to `openmw_esm4` (not even
/// `pub(crate)`), so cross-module fixtures cannot use `..Default::default()`
/// struct-update syntax on it. Go through `BaseRecord::default()` and field
/// assignment instead, which only touches the `pub(crate)` fields tests need.
fn synthetic_base(kind: &str, model: &str) -> BaseRecord {
    let mut base = BaseRecord::default();
    base.kind = kind.into();
    base.model = Some(model.into());
    base
}

fn minimal_actor_blueprint(kind: ActorKind, reference_form_id: u32) -> ActorAssemblyBlueprint {
    ActorAssemblyBlueprint {
        source_base_form_id: 0x1000,
        resolved_base_form_id: 0x1000,
        reference_form_id,
        kind,
        female: false,
        race_form_id: None,
        root_scale: 1.0,
        skeleton_path: Some("actors/eyebot/skeleton.glb".into()),
        mesh_parts: Vec::new(),
        apparel: Vec::new(),
        eye_form_id: None,
        eye_texture_path: None,
        facegen: None,
        facegen_reconstruction_fingerprint: None,
        facegen_diagnostics: Vec::new(),
        equipped_weapon: None,
        fallback: ActorFallbackDecision::default(),
    }
}

#[test]
fn achr_reference_is_prepared_as_an_actor_entry_not_a_dynamic_mesh() {
    let mut parsed = ParsedPlugin::default();
    let reference = synthetic_actor_reference(ReferenceKind::Npc, 0x0006_38e8, 0x0001_cf73);
    parsed.references.push(reference);
    parsed.bases = HashMap::from([(
        0x0001_cf73,
        synthetic_base("NPC_", "Creatures\\Eyebot\\Skeleton.nif"),
    )])
    .into();
    let cell = synthetic_exterior_cell();

    let package = super::build_cell_package(&parsed, &cell, "content", None, false).unwrap();

    assert!(
        package
            .dynamic_objects
            .iter()
            .all(|object| object.reference_form_id != 0x0006_38e8),
        "actor reference must not be flattened into dynamic_objects"
    );
    assert_eq!(package.actors.len(), 1);
    let actor = &package.actors[0];
    assert_eq!(actor.reference_form_id, 0x0006_38e8);
    assert_eq!(actor.base_form_id, 0x0001_cf73);
    assert_eq!(actor.kind, ActorKind::Humanoid);
    assert!(actor.initially_enabled);
}

#[test]
fn acre_reference_is_prepared_as_a_creature_actor_entry() {
    let mut parsed = ParsedPlugin::default();
    let reference = synthetic_actor_reference(ReferenceKind::Creature, 0x0005_1efa, 0x0002_0000);
    parsed.references.push(reference);
    parsed.bases = HashMap::from([(
        0x0002_0000,
        synthetic_base("CREA", "Creatures\\Radroach\\Radroach.nif"),
    )])
    .into();
    let cell = synthetic_exterior_cell();

    let package = super::build_cell_package(&parsed, &cell, "content", None, false).unwrap();

    assert_eq!(package.actors.len(), 1);
    assert_eq!(package.actors[0].kind, ActorKind::Creature);
    assert!(
        package
            .dynamic_objects
            .iter()
            .all(|object| object.reference_form_id != 0x0005_1efa)
    );
}

#[test]
fn apply_staged_assets_preserves_the_staged_actor_assembly() {
    let mut parsed = ParsedPlugin::default();
    let reference = synthetic_actor_reference(ReferenceKind::Npc, 0x0006_38e8, 0x0001_cf73);
    parsed.references.push(reference);
    parsed.bases = HashMap::from([(
        0x0001_cf73,
        synthetic_base("NPC_", "Creatures\\Eyebot\\Skeleton.nif"),
    )])
    .into();
    let cell = synthetic_exterior_cell();
    let mut package = super::build_cell_package(&parsed, &cell, "content", None, false).unwrap();
    assert_eq!(package.actors.len(), 1);

    let blueprint = minimal_actor_blueprint(ActorKind::Humanoid, 0x0006_38e8);
    let placement = PreparedPlacement {
        reference_form_id: 0x0006_38e8,
        base_form_id: 0x0001_cf73,
        asset_path: Some("actors/eyebot/skeleton.glb".into()),
        translation: [10.0, 20.0, 30.0],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        error: None,
        physics_asset_path: None,
        physics_source: None,
        physics_classification: Default::default(),
        step_support: false,
        mutability: Default::default(),
        mutability_root_form_id: None,
        reference_kind: "ACHR".into(),
        base_kind: "NPC_".into(),
        editor_id: None,
        display_name: None,
        count: 1,
        semantic: PreparedSemantic::Npc(PreparedActor {
            base_template_form_id: None,
            assembly: Some(blueprint.clone()),
        }),
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        linked_reference_form_id: None,
        inventory: Vec::new(),
        audio: Default::default(),
        ao_mode: "ao-none".into(),
    };

    super::apply_staged_assets(&mut package, &[placement], &HashMap::new());

    assert_eq!(package.actors[0].assembly, Some(blueprint));
    assert_eq!(
        package.actors[0].asset_path.as_deref(),
        Some("actors/eyebot/skeleton.glb")
    );
}

/// Non-actor exterior objects (doors, movable statics) must round-trip
/// through `apply_staged_assets` exactly as before: only `asset_path`/
/// `physics_asset_path` are copied off the matching placement.
#[test]
fn non_actor_dynamic_object_shape_is_unchanged() {
    let mut package = ExteriorCellPackage {
        revision: "test".into(),
        content_fingerprint: "test".into(),
        cell_form_id: 1,
        worldspace_form_id: 2,
        grid: GridCoordinate::new(0, 0),
        origin: [0.0; 3],
        terrain: None,
        water: None,
        static_objects: Vec::new(),
        dynamic_objects: vec![PreparedExteriorObject {
            reference_form_id: 5,
            base_form_id: 6,
            asset_path: None,
            physics_asset_path: None,
            door_destination: None,
            position: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            initially_enabled: true,
            persistent: false,
            dynamic: true,
            distant: false,
        }],
        distant_objects: Vec::new(),
        actors: Vec::new(),
        local_lights: Vec::new(),
        navigation: None,
        environment: PreparedExteriorEnvironment::default(),
        diagnostics: Vec::new(),
    };
    let placement = PreparedPlacement {
        reference_form_id: 5,
        base_form_id: 6,
        asset_path: Some("assets/door.glb".into()),
        translation: [0.0; 3],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        error: None,
        physics_asset_path: Some("assets/door.physics.ron".into()),
        physics_source: None,
        physics_classification: Default::default(),
        step_support: false,
        mutability: Default::default(),
        mutability_root_form_id: None,
        reference_kind: "REFR".into(),
        base_kind: "DOOR".into(),
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
    };

    super::apply_staged_assets(&mut package, &[placement], &HashMap::new());

    assert_eq!(
        package.dynamic_objects[0].asset_path.as_deref(),
        Some("assets/door.glb")
    );
    assert_eq!(
        package.dynamic_objects[0].physics_asset_path.as_deref(),
        Some("assets/door.physics.ron")
    );
}

#[test]
fn unstaged_source_model_is_not_published_as_a_runtime_exterior_asset() {
    let mut package = ExteriorCellPackage {
        revision: "test".into(),
        content_fingerprint: "test".into(),
        cell_form_id: 1,
        worldspace_form_id: 2,
        grid: GridCoordinate::new(0, 0),
        origin: [0.0; 3],
        terrain: None,
        water: None,
        static_objects: vec![PreparedExteriorObject {
            reference_form_id: 3,
            base_form_id: 4,
            asset_path: Some("MarkerCOCHeading.nif".into()),
            physics_asset_path: None,
            door_destination: None,
            position: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            initially_enabled: true,
            persistent: false,
            dynamic: false,
            distant: false,
        }],
        dynamic_objects: Vec::new(),
        distant_objects: Vec::new(),
        actors: Vec::new(),
        local_lights: Vec::new(),
        navigation: None,
        environment: PreparedExteriorEnvironment::default(),
        diagnostics: Vec::new(),
    };

    super::apply_staged_assets(&mut package, &[], &HashMap::new());

    assert_eq!(package.static_objects[0].asset_path, None);
}

#[test]
fn terrain_fixture_has_shared_shape_and_cell_origin() {
    let mut land = LandRecord {
        form_id: 1,
        cell_form_id: 2,
        heights: vec![0.0; 33 * 33],
        ..Default::default()
    };
    land.heights[32] = 70.0;
    let terrain = terrain_from_land(&land, GridCoordinate::new(-2, 3));
    assert!(terrain.is_well_formed());
    let origin = ExteriorCoordinatePolicy::default().grid_origin(GridCoordinate::new(-2, 3));
    assert_eq!(terrain.positions[0][0], origin[0] as f32);
    assert_eq!(terrain.positions[32][1], 1.0);
    assert_eq!(ParsedPlugin::default().road_count, 0);
}

#[test]
fn terrain_fixture_keeps_source_colors_and_has_deterministic_blend_weights() {
    let mut land = LandRecord {
        heights: vec![0.0; 33 * 33],
        colors: vec![[10, 20, 30]; 33 * 33],
        ..Default::default()
    };
    land.texture_layers = vec![1, 2];
    let terrain = terrain_from_land(&land, GridCoordinate::new(0, 0));
    assert_eq!(terrain.colors[0], [10, 20, 30, 255]);
    assert_eq!(
        terrain.blend_weights[0]
            .iter()
            .map(|v| u16::from(*v))
            .sum::<u16>(),
        255
    );
}

#[test]
fn terrain_fixture_projects_vtxt_overlay_weights_into_normalized_channels() {
    let mut land = LandRecord {
        heights: vec![0.0; 33 * 33],
        texture_layers: vec![1, 2],
        texture_assignments: vec![
            LandTextureAssignment {
                form_id: 1,
                quadrant: 0,
                layer: 0,
                base: true,
                weights: Vec::new(),
            },
            LandTextureAssignment {
                form_id: 2,
                quadrant: 0,
                layer: 1,
                base: false,
                weights: vec![LandTextureWeight {
                    position: 0,
                    opacity: 0.75,
                }],
            },
        ],
        ..Default::default()
    };

    let terrain = terrain_from_land(&land, GridCoordinate::new(0, 0));
    assert!(terrain.blend_weights[0][1] >= 190);
    assert!(terrain.blend_weights[0][0] <= 65);
    assert_eq!(
        terrain.blend_weights[0]
            .iter()
            .map(|value| u16::from(*value))
            .sum::<u16>(),
        255
    );
    land.texture_assignments[1].weights[0].opacity = 0.0;
    let fallback = terrain_from_land(&land, GridCoordinate::new(0, 0));
    assert_eq!(fallback.blend_weights[0], [255, 0, 0, 0]);
}

#[test]
fn terrain_material_resolves_txst_and_writes_cell_local_albedo() {
    let root =
        std::env::temp_dir().join(format!("bevyout-terrain-material-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("textures")).unwrap();
    image::RgbaImage::from_pixel(2, 2, image::Rgba([32, 64, 128, 255]))
        .save(root.join("textures/test.png"))
        .unwrap();
    image::RgbaImage::from_pixel(2, 2, image::Rgba([128, 0, 128, 64]))
        .save(root.join("textures/test_n.png"))
        .unwrap();

    let mut terrain = PreparedTerrain {
        width: 2,
        height: 2,
        positions: vec![[0.0; 3]; 4],
        normals: vec![[0.0, 1.0, 0.0]; 4],
        colors: vec![[255; 4]; 4],
        blend_weights: vec![[255, 0, 0, 0]; 4],
        texture_layers: vec![11],
        albedo_asset_path: None,
        normal_asset_path: None,
        collision_heights: vec![0.0; 4],
    };
    let landscape = HashMap::from([(
        11,
        LandscapeTextureRecord {
            form_id: 11,
            editor_id: Some("SyntheticLandscape".into()),
            texture_set_form_id: Some(22),
            diffuse_path: None,
        },
    )]);
    let texture_sets = HashMap::from([(
        22,
        TextureSetRecord {
            form_id: 22,
            diffuse_path: Some("textures/test.png".into()),
            normal_path: Some("textures/test_n.png".into()),
        },
    )]);
    let mut diagnostics = Vec::new();
    super::super::prepare_terrain_albedo(
        &mut terrain,
        &[],
        &landscape,
        &texture_sets,
        &root,
        &[],
        &root,
        "synthetic",
        0x1234,
        &mut diagnostics,
    )
    .unwrap();
    let relative = terrain.albedo_asset_path.as_deref().unwrap();
    let output = root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    assert!(output.is_file());
    let output = image::open(output).unwrap().to_rgba8();
    assert_eq!(output.dimensions(), (1024, 1024));
    assert_eq!(output.get_pixel(100, 100).0, [32, 64, 128, 255]);
    let normal_relative = terrain.normal_asset_path.as_deref().unwrap();
    let normal_output = root.join(normal_relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    let normal_output = image::open(normal_output).unwrap().to_rgba8();
    assert_eq!(normal_output.dimensions(), (1024, 1024));
    assert_eq!(normal_output.get_pixel(100, 100).0, [128, 255, 128, 64]);
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("textures/test.png"))
    );

    let mut same_payload_from_another_cell = terrain.clone();
    super::super::prepare_terrain_albedo(
        &mut same_payload_from_another_cell,
        &[],
        &landscape,
        &texture_sets,
        &root,
        &[],
        &root,
        "synthetic",
        0x5678,
        &mut diagnostics,
    )
    .unwrap();
    assert_eq!(
        same_payload_from_another_cell.albedo_asset_path, terrain.albedo_asset_path,
        "different terrain recipes with identical final bytes must share one object"
    );
    assert_eq!(
        same_payload_from_another_cell.normal_asset_path, terrain.normal_asset_path,
        "identical final normal maps must also share one object"
    );
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn terrain_material_composites_all_quadrant_layers_instead_of_four_channels() {
    let root = std::env::temp_dir().join(format!(
        "bevyout-terrain-material-quadrants-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("textures")).unwrap();

    let colors = [
        (11, [255, 0, 0, 255]),
        (12, [0, 255, 0, 255]),
        (13, [0, 0, 255, 255]),
        (14, [255, 255, 0, 255]),
        (15, [255, 0, 255, 255]),
        (16, [0, 255, 255, 255]),
        (17, [255, 255, 255, 255]),
    ];
    for (form_id, color) in colors {
        image::RgbaImage::from_pixel(2, 2, image::Rgba(color))
            .save(root.join(format!("textures/{form_id}.png")))
            .unwrap();
    }
    fs::create_dir_all(root.join("textures/landscape")).unwrap();
    image::RgbaImage::from_pixel(2, 2, image::Rgba([10, 20, 30, 255]))
        .save_with_format(
            root.join("textures/landscape/dirtwasteland01.dds"),
            image::ImageFormat::Png,
        )
        .unwrap();
    let full_weight_grid = || {
        (0..(17 * 17))
            .map(|position| LandTextureWeight {
                position,
                opacity: 1.0,
            })
            .collect::<Vec<_>>()
    };
    let assignments = vec![
        LandTextureAssignment {
            form_id: 11,
            quadrant: 0,
            layer: 0,
            base: true,
            weights: Vec::new(),
        },
        LandTextureAssignment {
            form_id: 12,
            quadrant: 0,
            layer: 7,
            base: false,
            weights: full_weight_grid(),
        },
        LandTextureAssignment {
            form_id: 13,
            quadrant: 1,
            layer: 0,
            base: true,
            weights: Vec::new(),
        },
        LandTextureAssignment {
            form_id: 16,
            quadrant: 1,
            layer: 1,
            base: false,
            weights: full_weight_grid(),
        },
        LandTextureAssignment {
            form_id: 0,
            quadrant: 2,
            layer: 0,
            base: true,
            weights: Vec::new(),
        },
        LandTextureAssignment {
            form_id: 15,
            quadrant: 3,
            layer: 0,
            base: true,
            weights: Vec::new(),
        },
        LandTextureAssignment {
            form_id: 17,
            quadrant: 3,
            layer: 1,
            base: false,
            weights: full_weight_grid(),
        },
    ];
    let mut terrain = PreparedTerrain {
        width: 33,
        height: 33,
        positions: vec![[0.0; 3]; 33 * 33],
        normals: vec![[0.0, 1.0, 0.0]; 33 * 33],
        colors: vec![[255; 4]; 33 * 33],
        blend_weights: vec![[255, 0, 0, 0]; 33 * 33],
        texture_layers: colors.iter().map(|(form_id, _)| *form_id).collect(),
        albedo_asset_path: None,
        normal_asset_path: None,
        collision_heights: vec![0.0; 33 * 33],
    };
    let landscape = colors
        .iter()
        .map(|(form_id, _)| {
            (
                *form_id,
                LandscapeTextureRecord {
                    form_id: *form_id,
                    editor_id: Some(format!("Synthetic{form_id}")),
                    texture_set_form_id: Some(100 + *form_id),
                    diffuse_path: None,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let texture_sets = colors
        .iter()
        .map(|(form_id, _)| {
            (
                100 + *form_id,
                TextureSetRecord {
                    form_id: 100 + *form_id,
                    diffuse_path: Some(format!("textures/{form_id}.png")),
                    normal_path: None,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let mut diagnostics = Vec::new();
    super::super::prepare_terrain_albedo(
        &mut terrain,
        &assignments,
        &landscape,
        &texture_sets,
        &root,
        &[],
        &root,
        "synthetic-quadrants",
        0x5678,
        &mut diagnostics,
    )
    .unwrap();

    let relative = terrain.albedo_asset_path.as_deref().unwrap();
    let output = image::open(root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR)))
        .unwrap()
        .to_rgba8();
    assert_eq!(output.get_pixel(100, 100).0, colors[1].1);
    assert_eq!(output.get_pixel(900, 100).0, colors[5].1);
    assert_eq!(output.get_pixel(100, 900).0, [10, 20, 30, 255]);
    assert_eq!(output.get_pixel(900, 900).0, colors[6].1);
    let _ = fs::remove_dir_all(&root);
}
