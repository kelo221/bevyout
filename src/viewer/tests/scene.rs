use super::*;
use bevy::ecs::world::CommandQueue;

/// A minimal placement matching `world::persist::tests::placement`'s
/// shape (same crate, different module -- kept local since that one is
/// private), overridden per test with `asset_path: None` and a chosen
/// semantic.
fn placement(reference_form_id: u32, semantic: PreparedSemantic) -> PreparedPlacement {
    PreparedPlacement {
        reference_form_id,
        base_form_id: 0x0001_2345,
        asset_path: None,
        translation: [1.0, 0.0, 2.0],
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
        editor_id: Some("CG04DeadOldLady".into()),
        display_name: Some("Old Lady".into()),
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

/// Mirrors `world::persist::tests::minimal_manifest` (private to that
/// module) with the one field this suite drives.
fn minimal_manifest(placements: Vec<PreparedPlacement>) -> PreparedSceneManifest {
    PreparedSceneManifest {
        schema_version: 13,
        prepare_revision: None,
        converter_revision: None,
        physics_schema_version: None,
        asset_root: ".".into(),
        source_plugin: "Fallout3.esm".into(),
        source_fingerprint: "content-hash".into(),
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
        source_plugins: Vec::new(),
        visual_issues: Vec::new(),
        cell: CellInfo {
            form_id: 0x0002_8138,
            editor_id: None,
            name: None,
            interior: true,
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
            grid: None,
            worldspace_form_id: None,
            day_night_profile: None,
            day_night_preview_profile: None,
        },
        placements,
        lights: Vec::new(),
        diagnostics: Vec::new(),
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
    }
}

#[test]
fn fallout_material_extra_accepts_native_object_and_blender_json_string() {
    let native = serde_json::json!({
        "bevyout_fallout_material": {
            "translucency_enabled": true,
            "translucency_strength": 0.35,
            "local_thickness": {"enabled": true, "strength": 0.4}
        }
    });
    let parsed = parse_fallout_material_extra(&native.to_string()).expect("native extras");
    assert!(parsed.translucency_enabled);
    assert_eq!(parsed.local_thickness.unwrap().strength, 0.4);

    let nested = serde_json::json!({
        "translucency_enabled": true,
        "translucency_strength": 0.2,
        "local_thickness": {"enabled": true, "strength": 0.2}
    });
    let blender = serde_json::json!({
        "bevyout_fallout_material": nested.to_string()
    });
    let parsed = parse_fallout_material_extra(&blender.to_string()).expect("Blender extras");
    assert_eq!(parsed.translucency_strength, 0.2);
}

#[test]
fn fallout_surface_classification_uses_flags_for_shader_type_one_assets() {
    let hair = parse_fallout_material_extra(
        &serde_json::json!({
            "bevyout_fallout_material": {
                "shader_type": 1,
                "shader_flags_1": 1u32 << 18
            }
        })
        .to_string(),
    )
    .expect("hair extras");
    assert_eq!(
        fallout_surface_kind(&hair, Some("NoHat")),
        FALLOUT_SURFACE_HAIR
    );

    let eye = parse_fallout_material_extra(
        &serde_json::json!({
            "bevyout_fallout_material": {
                "shader_type": 1,
                "shader_flags_1": 1u32 << 17
            }
        })
        .to_string(),
    )
    .expect("eye extras");
    assert_eq!(
        fallout_surface_kind(&eye, Some("EyeLeftHuman:0")),
        FALLOUT_SURFACE_EYE
    );
    assert_eq!(
        fallout_surface_kind(&eye, Some("GlassesReadingGO:0")),
        FALLOUT_SURFACE_STANDARD
    );
    assert_eq!(
        fallout_surface_kind(&eye, None),
        FALLOUT_SURFACE_EYE,
        "source flag remains authoritative when a mesh name is unavailable"
    );
}

#[test]
fn translucency_marks_non_fallout_extras_as_configured() {
    let mut app = test_app();
    let material = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial::default());
    let entity = app
        .world_mut()
        .spawn((
            Mesh3d::default(),
            MeshMaterial3d(material.clone()),
            GltfMaterialExtras {
                value: r#"{"extras_only":true}"#.into(),
            },
        ))
        .id();
    let no_thickness_entity = app
        .world_mut()
        .spawn((
            Mesh3d::default(),
            MeshMaterial3d(material),
            GltfMaterialExtras {
                value: r#"{"bevyout_fallout_material":{"translucency_enabled":true}}"#.into(),
            },
        ))
        .id();
    app.add_systems(Update, configure_fallout_translucency);

    app.update();
    assert!(
        app.world()
            .entity(entity)
            .contains::<FalloutMaterialConfigured>()
    );
    assert!(
        app.world()
            .entity(no_thickness_entity)
            .contains::<FalloutMaterialConfigured>()
    );

    app.update();
    assert!(
        app.world()
            .entity(entity)
            .contains::<FalloutMaterialConfigured>()
    );
    assert!(
        app.world()
            .entity(no_thickness_entity)
            .contains::<FalloutMaterialConfigured>()
    );
}

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins((bevy::MinimalPlugins, bevy::asset::AssetPlugin::default()));
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app
}

/// Runs `spawn_cell_placements_chunk` against a bare app world:
/// `Assets<Mesh>`/`Assets<StandardMaterial>` are pulled out of the world
/// as owned locals for the call (the function takes `&mut Assets<..>`,
/// which conflicts with borrowing `&World` for `Commands` from the same
/// world at the same time) and the queued spawns are flushed before
/// returning.
fn spawn_chunk(
    app: &mut App,
    manifest: &PreparedSceneManifest,
    root: Entity,
) -> SpawnedCellContent {
    let mut meshes = app
        .world_mut()
        .remove_resource::<Assets<Mesh>>()
        .expect("test_app initializes Assets<Mesh>");
    let mut materials = app
        .world_mut()
        .remove_resource::<Assets<StandardMaterial>>()
        .expect("test_app initializes Assets<StandardMaterial>");
    let asset_server = app.world().resource::<AssetServer>().clone();

    let world = app.world_mut();
    let mut queue = CommandQueue::default();
    let (content, next) = {
        let mut commands = Commands::new(&mut queue, world);
        spawn_cell_placements_chunk(
            &mut commands,
            &asset_server,
            &mut meshes,
            &mut materials,
            manifest,
            root,
            None,
            0,
            usize::MAX,
        )
    };
    queue.apply(world);
    assert_eq!(next, manifest.placements.len());
    content
}

// Issue #120 (F119.2): a source-dead actor's `PreparedSemantic::Corpse`
// placement has no resolved GLB (FO3 actors have no standalone world
// model), yet must still be present and targetable in the running
// viewer -- `spawn_cell_placements_chunk` used to unconditionally skip
// every placement without an `asset_path`, which would have silently
// dropped the corpse from the scene entirely despite #118/#120's
// prepare-side classification. This pins the placeholder spawn that
// closes that gap: a real `Mesh3d` (so `update_focused_placement`'s
// `MeshRayCast` can find it) plus the exact `PlacementRoot` component
// #118's activation seam matches on.
#[test]
fn a_corpse_placement_without_an_asset_spawns_a_placeholder_with_activation_components() {
    let mut app = test_app();
    let manifest = minimal_manifest(vec![placement(0x0005_4398, PreparedSemantic::Corpse)]);
    let root = app
        .world_mut()
        .spawn((Transform::default(), Visibility::Visible))
        .id();

    let content = spawn_chunk(&mut app, &manifest, root);
    assert_eq!(content.placement_count, 1);
    assert!(
        content.scene_handles.is_empty(),
        "the placeholder has no async GLB to await"
    );

    let mut query = app.world_mut().query::<(
        &interaction::PlacementRoot,
        &Mesh3d,
        &MeshMaterial3d<StandardMaterial>,
        &ChildOf,
    )>();
    let (root_component, _mesh, _material, child_of) = query
        .single(app.world())
        .expect("exactly one placeholder entity");
    assert_eq!(root_component.placement().reference_form_id, 0x0005_4398);
    assert_eq!(
        root_component.placement().display_name.as_deref(),
        Some("Old Lady"),
        "display identity must reach the activation/transfer UI through PlacementRoot"
    );
    assert_eq!(child_of.parent(), root);
}

// The counterpart regression guard: a living `Npc` placement (no
// resolved GLB either, since FO3 actors never have a standalone world
// model) must stay unspawned exactly as before this issue -- real actor
// bodies are the #106-#108 track, not this placeholder.
#[test]
fn an_npc_placement_without_an_asset_spawns_an_actor_root_not_a_corpse_placeholder() {
    // Pre-merge (#120) an asset-less living Npc spawned nothing; M4
    // wave 7 (#107/#108, merged from master) now deliberately spawns a
    // bare `PlacementRoot` for living actors so ActorPlugin can project
    // their identity. The #120 invariant that survives the merge: a
    // living actor never receives the corpse placeholder mesh.
    let mut app = test_app();
    let manifest = minimal_manifest(vec![placement(
        0x0005_4399,
        PreparedSemantic::Npc(crate::vsa::PreparedActor {
            base_template_form_id: None,
            assembly: None,
        }),
    )]);
    let root = app
        .world_mut()
        .spawn((Transform::default(), Visibility::Visible))
        .id();

    let content = spawn_chunk(&mut app, &manifest, root);
    assert_eq!(content.placement_count, 1);

    let mut roots = app
        .world_mut()
        .query::<(&interaction::PlacementRoot, Option<&Mesh3d>)>();
    let (_, mesh) = roots
        .iter(app.world())
        .next()
        .expect("the living actor's placement root must spawn");
    assert!(
        mesh.is_none(),
        "a living actor must not receive the corpse placeholder mesh"
    );
}
