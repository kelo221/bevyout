use super::*;
use bevy::ecs::world::CommandQueue;
use bevy::math::{Rect, Vec2};
use bevyout_core::manifest::{PreparedBake, PreparedLightmapAtlas, PreparedLightmapFormat};

#[test]
fn lightmap_runtime_policy_keeps_dynamic_diffuse_and_excludes_baked_static_diffuse() {
    assert!(runtime_lightmapped_diffuse_enabled(None));

    let empty_bake = PreparedBake {
        bake_revision: None,
        source_fingerprint: "fixture".into(),
        scene_path: "scene.glb".into(),
        lightmaps: Vec::new(),
        lightmap_variance_pages: Vec::new(),
        lightmap_bindings: Vec::new(),
        bake_settings: Default::default(),
        irradiance_volume: None,
    };
    assert!(runtime_lightmapped_diffuse_enabled(Some(&empty_bake)));

    let surface_bake = PreparedBake {
        lightmaps: vec![PreparedLightmapAtlas {
            asset_path: "baked/lightmap.ktx2".into(),
            width: 3996,
            height: 3980,
            format: PreparedLightmapFormat::Rgba16Float,
            content_hash: "fixture".into(),
        }],
        ..empty_bake
    };
    assert!(!runtime_lightmapped_diffuse_enabled(Some(&surface_bake)));
}

#[test]
fn prepared_lightmaps_attach_only_under_baked_static_root() {
    let mut app = App::new();
    app.insert_resource(PreparedLightmap {
        bindings: std::collections::HashMap::from([(
            7,
            PreparedLightmapBinding {
                image: Handle::default(),
                uv_rect: Rect::from_corners(Vec2::ZERO, Vec2::ONE),
            },
        )]),
    });
    app.add_systems(Update, attach_prepared_lightmaps);

    let static_root = app.world_mut().spawn(BakedStaticSceneRoot).id();
    let dynamic_root = app.world_mut().spawn_empty().id();
    let static_mesh = app
        .world_mut()
        .spawn((
            Mesh3d::default(),
            MeshMaterial3d::<StandardMaterial>::default(),
            GltfExtras {
                value: r#"{"bevyout":{"lightmap_binding":7}}"#.into(),
            },
            ChildOf(static_root),
        ))
        .id();
    let dynamic_mesh = app
        .world_mut()
        .spawn((
            Mesh3d::default(),
            MeshMaterial3d::<StandardMaterial>::default(),
            GltfExtras {
                value: r#"{"bevyout":{"lightmap_binding":7}}"#.into(),
            },
            ChildOf(dynamic_root),
        ))
        .id();

    app.update();

    assert!(app.world().entity(static_mesh).contains::<Lightmap>());
    assert!(
        app.world()
            .entity(static_mesh)
            .contains::<PreparedLightmapAttached>()
    );
    assert!(!app.world().entity(dynamic_mesh).contains::<Lightmap>());
    assert!(
        !app.world()
            .entity(dynamic_mesh)
            .contains::<PreparedLightmapAttached>()
    );
}

#[test]
fn prepared_lightmap_binding_reads_primitive_extras() {
    let extras = GltfExtras {
        value: r#"{"bevyout":{"primitive_key":"fixture/quad","lightmap_binding":42}}"#.into(),
    };
    assert_eq!(diagnostic_lightmap_binding_id(&extras), Some(42));
}

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
        image_space_modifier_catalog_path: None,
        image_space_modifier_catalog_revision: None,
        image_space_modifier_catalog_hash: None,
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
        exterior: None,
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
fn fallout_surface_classification_uses_authored_types_and_flags() {
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

    for (shader_type, expected) in [
        (FALLOUT_SHADER_TYPE_HAIR_TINT, FALLOUT_SURFACE_HAIR),
        (FALLOUT_SHADER_TYPE_SKIN_TINT, FALLOUT_SURFACE_SKIN),
    ] {
        let material = parse_fallout_material_extra(
            &serde_json::json!({
                "bevyout_fallout_material": {
                    "shader_type": shader_type
                }
            })
            .to_string(),
        )
        .expect("typed surface extras");
        assert_eq!(fallout_surface_kind(&material, None), expected);
    }

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
        FALLOUT_SURFACE_LEGACY_WORLD
    );
    assert_eq!(
        fallout_surface_kind(&eye, None),
        FALLOUT_SURFACE_EYE,
        "source flag remains authoritative when a mesh name is unavailable"
    );
}

#[test]
fn fallout_surface_configuration_bounds_skin_and_hair_specular_response() {
    let mut app = test_app();
    app.add_systems(Update, configure_fallout_surface_materials);

    let mut handles = Vec::new();
    for shader_type in [FALLOUT_SHADER_TYPE_HAIR_TINT, FALLOUT_SHADER_TYPE_SKIN_TINT] {
        let handle = app
            .world_mut()
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                perceptual_roughness: 0.1,
                reflectance: 0.5,
                ..default()
            });
        app.world_mut().spawn((
            Mesh3d::default(),
            MeshMaterial3d(handle.clone()),
            GltfMaterialExtras {
                value: serde_json::json!({
                    "bevyout_fallout_material": { "shader_type": shader_type }
                })
                .to_string(),
            },
        ));
        handles.push(handle);
    }

    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    let hair = materials.get(&handles[0]).expect("hair material");
    assert_eq!(hair.fallout_surface_kind, FALLOUT_SURFACE_HAIR);
    assert_eq!(hair.perceptual_roughness, FALLOUT_HAIR_MIN_ROUGHNESS);
    assert_eq!(hair.reflectance, FALLOUT_HAIR_REFLECTANCE);
    assert_eq!(hair.anisotropy_strength, FALLOUT_HAIR_ANISOTROPY_STRENGTH);

    let skin = materials.get(&handles[1]).expect("skin material");
    assert_eq!(skin.fallout_surface_kind, FALLOUT_SURFACE_SKIN);
    assert_eq!(skin.perceptual_roughness, FALLOUT_SKIN_MIN_ROUGHNESS);
    assert_eq!(skin.reflectance, FALLOUT_SKIN_REFLECTANCE);
}

#[test]
fn flat_overlay_materials_get_depth_bias_and_stains_lose_reflections() {
    let mut app = test_app();
    app.add_systems(Update, configure_fallout_surface_materials);
    let stain = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            metallic: 0.6,
            reflectance: 0.7,
            perceptual_roughness: 0.2,
            ..default()
        });
    let entity = app
        .world_mut()
        .spawn((
            Mesh3d::default(),
            MeshMaterial3d(stain.clone()),
            GltfMeshName("Stain01:44".into()),
            GltfMaterialExtras {
                value: serde_json::json!({
                    "bevyout_fallout_material": { "shader_type": 1 }
                })
                .to_string(),
            },
        ))
        .id();
    app.world_mut().spawn((
        Mesh3d::default(),
        MeshMaterial3d(stain.clone()),
        GltfMaterialExtras {
            value: serde_json::json!({
                "bevyout_fallout_material": { "shader_type": 1 }
            })
            .to_string(),
        },
    ));

    app.update();
    let overlay_handle = app
        .world()
        .entity(entity)
        .get::<MeshMaterial3d<StandardMaterial>>()
        .unwrap()
        .0
        .clone();
    assert_ne!(overlay_handle, stain);
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    let material = materials.get(&overlay_handle).unwrap();
    assert_eq!(material.depth_bias, FALLOUT_OVERLAY_DEPTH_BIAS);
    assert_eq!(material.reflectance, FALLOUT_DECAL_REFLECTANCE);
    assert_eq!(material.metallic, 0.0);
    assert_eq!(material.perceptual_roughness, 1.0);
    let shared_material = materials.get(&stain).unwrap();
    assert_eq!(shared_material.depth_bias, 0.0);
    assert_eq!(shared_material.reflectance, 0.7);
    assert_eq!(shared_material.metallic, 0.6);
    assert_eq!(shared_material.perceptual_roughness, 0.2);
    assert!(app.world().entity(entity).contains::<NotShadowCaster>());
}

#[test]
fn legacy_world_materials_preserve_glossiness_and_receive_only_changed_chan_values() {
    let mut app = test_app();
    app.add_systems(
        Update,
        (
            configure_fallout_surface_materials,
            apply_legacy_chan_strength,
        )
            .chain(),
    );
    let legacy = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial::default());
    let generic = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial::default());
    app.world_mut().spawn((
        Mesh3d::default(),
        MeshMaterial3d(legacy.clone()),
        GltfMaterialExtras {
            value: serde_json::json!({
                "bevyout_fallout_material": {
                    "schema": 2,
                    "shader_type": 0,
                    "glossiness_exponent": 128.0
                }
            })
            .to_string(),
        },
    ));
    app.world_mut()
        .spawn((Mesh3d::default(), MeshMaterial3d(generic.clone())));

    app.update();
    {
        let materials = app.world().resource::<Assets<StandardMaterial>>();
        let legacy = materials.get(&legacy).unwrap();
        assert_eq!(legacy.fallout_surface_kind, FALLOUT_SURFACE_LEGACY_WORLD);
        assert_eq!(legacy.fallout_glossiness_exponent, 128.0);
        assert_eq!(legacy.fallout_chan_strength, 1.0);
        assert_eq!(
            materials.get(&generic).unwrap().fallout_surface_kind,
            FALLOUT_SURFACE_STANDARD
        );
    }
    assert_eq!(
        app.world().resource::<LegacyWorldMaterials>().handles.len(),
        1
    );

    app.world_mut()
        .resource_mut::<LegacyChanSettings>()
        .set_strength(0.25);
    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    assert_eq!(materials.get(&legacy).unwrap().fallout_chan_strength, 0.25);
    assert_eq!(materials.get(&generic).unwrap().fallout_chan_strength, 1.0);
}

#[test]
fn invalid_legacy_world_exponent_falls_back_to_ten() {
    for value in [-1.0, f32::NAN, f32::INFINITY] {
        assert_eq!(sanitized_fallout_glossiness_exponent(value), 10.0);
    }
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
    app.init_resource::<LegacyChanSettings>();
    app.init_resource::<OverlayLightingSettings>();
    app.init_resource::<LegacyWorldMaterials>();
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

// ---------------------------------------------------------------------
// Issue #270 (PERF wave 1): glow-card classification is marker-driven
// (every inspected entity carries `GlowCardInspected`; markers despawn
// with their entities). The remove+add count-coincidence blind spot of
// the old `Local::<HashSet<Entity>>`/count-sentinel pair is covered.
// ---------------------------------------------------------------------

fn glow_card_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_systems(Update, configure_glow_cards);
    app
}

fn spawn_named_mesh(app: &mut App, name: &str) -> Entity {
    app.world_mut()
        .spawn((
            Transform::default(),
            Visibility::Visible,
            Mesh3d(Handle::<Mesh>::default()),
            GltfMeshName(name.to_string()),
        ))
        .id()
}

/// Change-detection probe placed after `configure_glow_cards`: a frame
/// with zero hits on these components is provably write-free (Bevy change
/// detection fires on deref-mut/commands only).
#[derive(Resource, Default)]
struct GlowCardWriteProbe {
    visibility_writes: usize,
    marker_writes: usize,
}

fn probe_glow_card_writes(
    visibilities: Query<Ref<Visibility>>,
    markers: Query<Ref<GlowCardInspected>>,
    mut probe: ResMut<GlowCardWriteProbe>,
) {
    for visibility in &visibilities {
        probe.visibility_writes += usize::from(visibility.is_changed());
    }
    for marker in &markers {
        probe.marker_writes += usize::from(marker.is_changed());
    }
}

#[test]
fn glow_card_spawned_during_a_remove_add_pair_is_still_hidden() {
    let mut app = glow_card_test_app();
    let lamp = spawn_named_mesh(&mut app, "ShackHangingLight02:51");
    app.update();
    assert!(
        app.world().entity(lamp).contains::<GlowCardInspected>(),
        "every inspected mesh carries the marker, glow or not"
    );

    // The #270 blind spot: one mesh despawns and another spawns inside one
    // tick. The old `last_mesh_count` sentinel saw `1 == 1` and skipped
    // the pass, leaving the brand-new glow card visible forever.
    app.world_mut().despawn(lamp);
    let glow = spawn_named_mesh(&mut app, "LightGlow01:0.001");
    app.update();

    assert!(app.world().entity(glow).contains::<GlowCardInspected>());
    assert!(app.world().entity(glow).contains::<GlowCard>());
    assert!(matches!(
        app.world().entity(glow).get::<Visibility>(),
        Some(Visibility::Hidden)
    ));
}

#[test]
fn every_inspected_mesh_gets_a_marker_and_only_glow_cards_are_hidden() {
    let mut app = glow_card_test_app();
    let lamp = spawn_named_mesh(&mut app, "ShackHangingLight02:51");
    let glow = spawn_named_mesh(&mut app, "lightglow01");
    app.update();

    assert!(app.world().entity(lamp).contains::<GlowCardInspected>());
    assert!(!app.world().entity(lamp).contains::<GlowCard>());
    assert!(matches!(
        app.world().entity(lamp).get::<Visibility>(),
        Some(Visibility::Visible)
    ));
    assert!(app.world().entity(glow).contains::<GlowCardInspected>());
    assert!(app.world().entity(glow).contains::<GlowCard>());
    assert!(matches!(
        app.world().entity(glow).get::<Visibility>(),
        Some(Visibility::Hidden)
    ));
}

#[test]
fn settled_glow_card_frames_perform_no_writes() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.init_resource::<GlowCardWriteProbe>();
    app.add_systems(
        Update,
        (configure_glow_cards, probe_glow_card_writes).chain(),
    );
    spawn_named_mesh(&mut app, "ShackHangingLight02:51");
    spawn_named_mesh(&mut app, "LightGlow01:0.001");
    // Absorb the classification frame; the guarantee covers steady frames.
    app.update();
    app.update();
    {
        let mut probe = app.world_mut().resource_mut::<GlowCardWriteProbe>();
        probe.visibility_writes = 0;
        probe.marker_writes = 0;
    }

    for frame in 0..3 {
        app.update();
        let probe = app.world().resource::<GlowCardWriteProbe>();
        assert_eq!(
            (probe.visibility_writes, probe.marker_writes),
            (0, 0),
            "settled frame {frame} wrote glow-card components"
        );
    }
}
