use super::*;
use crate::console::{ConsoleExecutor, ConsolePlugin, ConsoleRequest, ConsoleSessionId};
use crate::vsa::{PreparedItemCategory, PreparedItemDefinition};
use bevy::state::app::StatesPlugin;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, ConsolePlugin))
        .insert_resource(player::PlayerNoClip::default())
        .insert_resource(player::PhysicsDisabled(false))
        .insert_resource(LightingScale(1.0))
        .insert_resource(IrradianceIntensity(1.0))
        .insert_resource(AmbientScale(1.0))
        .insert_resource(FogStrength(1.0))
        .insert_resource(AoStrength(1.0))
        .insert_resource(UnlitMode(false))
        .insert_resource(LightsDisabled(false))
        .insert_resource(PreparedPointShadowRuntime::default())
        .insert_resource(PointLightShadowSamples::default())
        .insert_resource(BoxdddDebugDrawSettings::default())
        .insert_resource(player::StepDebugSettings::default())
        .insert_resource(interaction::PlayerInventory::default())
        .insert_resource(interaction::PlayerEquipment::default());
    app.init_resource::<interaction::CanonicalItemLedger>();
    app.init_state::<GameplayModal>();
    let camera = player::CameraModeState {
        collision_build_complete: true,
        collisions_ready: true,
        ..default()
    };
    app.insert_resource(camera);
    app.world_mut().spawn((
        Camera3d::default(),
        Projection::Perspective(super::super::default_perspective_projection()),
        HorizontalFov::default(),
        Bloom::default(),
        Tonemapping::AgX,
        Transform::from_xyz(0.0, 2.0, 0.0),
        super::super::FlyCamera {
            yaw: 0.0,
            pitch: 0.0,
            speed: 8.0,
        },
    ));
    install(&mut app);
    player::set_camera_mode(app.world_mut(), player::CameraMode::Fps).unwrap();
    app.update();
    app
}

fn current_tonemapper(app: &mut App) -> Tonemapping {
    let world = app.world_mut();
    let mut query = world.query_filtered::<&Tonemapping, With<Camera3d>>();
    *query.single(world).unwrap()
}

fn exec(app: &mut App, line: &str) -> crate::console::ConsoleOutput {
    ConsoleExecutor::execute(
        app.world_mut(),
        ConsoleRequest {
            session: ConsoleSessionId::new("test"),
            line: line.into(),
        },
    )
}

#[test]
fn toggles_and_time_multiplier_change_focused_state() {
    let mut app = test_app();
    let game_ui = app.world_mut().spawn((GameUi, Visibility::Inherited)).id();
    let diagnostics = app
        .world_mut()
        .spawn((DiagnosticUi, Visibility::Inherited))
        .id();
    assert!(exec(&mut app, "tcl").value["no_clip"].as_bool().unwrap());
    assert!(exec(&mut app, "tm").ok);
    assert_eq!(
        app.world().get::<Visibility>(game_ui),
        Some(&Visibility::Hidden)
    );
    assert!(exec(&mut app, "tdt").ok);
    assert_eq!(
        app.world().get::<Visibility>(diagnostics),
        Some(&Visibility::Hidden)
    );
    assert!(exec(&mut app, "sgtm 2").ok);
    assert_eq!(
        app.world().resource::<Time<Virtual>>().relative_speed(),
        2.0
    );
}

#[test]
fn fov_reports_sets_and_validates_horizontal_degrees() {
    let mut app = test_app();

    let current = exec(&mut app, "fov");
    assert_eq!(current.value["degrees"], 90.0);
    assert_eq!(current.value["axis"], "horizontal");

    let changed = exec(&mut app, "fov 110");
    assert!(changed.ok);
    assert_eq!(changed.value["degrees"], 110.0);
    let mut query = app
        .world_mut()
        .query_filtered::<&Projection, With<Camera3d>>();
    let Projection::Perspective(perspective) = query.single(app.world()).unwrap() else {
        panic!("expected a perspective camera");
    };
    let expected = horizontal_to_vertical_fov(110.0, perspective.aspect_ratio);
    assert!((perspective.fov - expected).abs() < 1e-6);

    assert_eq!(exec(&mut app, "fov 9").error.unwrap().code, "out_of_range");
    assert_eq!(
        exec(&mut app, "fov 171").error.unwrap().code,
        "out_of_range"
    );
    assert_eq!(exec(&mut app, "fov nope").error.unwrap().code, "bad_type");
    assert_eq!(
        exec(&mut app, "fov 90 extra").error.unwrap().code,
        "bad_arity"
    );
}

#[test]
fn screenshot_rejects_headless_and_unsafe_names() {
    let mut app = test_app();
    assert_eq!(
        exec(&mut app, "screenshot").error.unwrap().code,
        "unsupported"
    );
    assert_eq!(
        exec(&mut app, "screenshot ../escape").error.unwrap().code,
        "invalid_path"
    );
}

#[test]
fn developer_commands_and_aliases_are_registered_and_structured() {
    let mut app = test_app();
    assert!(exec(&mut app, "help toggleflycam").ok);
    assert!(exec(&mut app, "help togglecollisiongeometry").ok);
    assert!(exec(&mut app, "help fov").ok);
    let ragdoll_probe_help = exec(&mut app, "help ragdollprobe");
    assert!(ragdoll_probe_help.ok);
    assert_eq!(ragdoll_probe_help.value["mutating"], false);
    let actor_inspect_help = exec(&mut app, "help actorinspect");
    assert!(actor_inspect_help.ok);
    assert_eq!(actor_inspect_help.value["mutating"], false);
    let free_camera = exec(&mut app, "toggleflycam");
    assert_eq!(free_camera.value["camera_mode"], "free");
    assert_eq!(free_camera.log, ["Free camera enabled."]);
    let fps_camera = exec(&mut app, "tfc");
    assert_eq!(fps_camera.value["camera_mode"], "fps");
    assert_eq!(fps_camera.log, ["Free camera disabled."]);

    let collision_geometry = exec(&mut app, "togglecollisiongeometry");
    assert_eq!(collision_geometry.value["enabled"], true);
    assert_eq!(collision_geometry.log, ["Collision geometry enabled."]);
    let stair_debug = exec(&mut app, "stairdebug");
    assert_eq!(stair_debug.value["enabled"], true);
    assert_eq!(stair_debug.log, ["Stair debugging enabled."]);
    let unlit = exec(&mut app, "tunlit");
    assert_eq!(unlit.value["enabled"], true);
    assert_eq!(unlit.log, ["Unlit mode enabled."]);
    let lights = exec(&mut app, "tlights");
    assert_eq!(lights.value["lights_enabled"], false);
    assert_eq!(lights.log, ["Lights disabled."]);
}

#[test]
fn render_settings_validate_boundaries_before_mutation() {
    let mut app = test_app();
    for (setting, low, high) in [
        ("lighting", 0.0001, 262_144.0),
        ("irradiance", 0.0, 4096.0),
        ("ambient", 0.0001, 4096.0),
        ("bloom_intensity", 0.0, 1.0),
        ("bloom_softness", 0.0, 1.0),
        ("fog", 0.0, 1.0),
        ("ao", 0.0, 1.0),
    ] {
        assert!(exec(&mut app, &format!("setrender {setting} {low}")).ok);
        assert!(exec(&mut app, &format!("setrender {setting} {high}")).ok);
    }
    assert!(exec(&mut app, "setrender shadow_samples 0").ok);
    assert_eq!(app.world().resource::<PointLightShadowSamples>().0, 0);
    assert!(exec(&mut app, "setrender shadow_samples 1").ok);
    assert_eq!(app.world().resource::<PointLightShadowSamples>().0, 1);
    assert!(!app.world().resource::<RealtimeShadowSettings>().enabled);
    assert!(exec(&mut app, "setrender realtime_shadows 1").ok);
    assert!(app.world().resource::<RealtimeShadowSettings>().enabled);
    assert!(exec(&mut app, "setrender realtime_shadows 0").ok);
    assert!(!app.world().resource::<RealtimeShadowSettings>().enabled);
    assert_eq!(
        exec(&mut app, "getrender realtime_shadows").value["value"],
        0
    );
    assert!(exec(&mut app, "setrender bloom_threshold 5000").ok);
    let before = app.world().resource::<LightingScale>().0;
    assert_eq!(
        exec(&mut app, "setrender lighting 0").error.unwrap().code,
        "out_of_range"
    );
    assert_eq!(app.world().resource::<LightingScale>().0, before);
    assert_eq!(
        exec(&mut app, "setrender lighting NaN").error.unwrap().code,
        "non_finite"
    );
    assert_eq!(app.world().resource::<LightingScale>().0, before);
    assert_eq!(
        exec(&mut app, "setrender unknown 1").error.unwrap().code,
        "unknown_setting"
    );
    assert_eq!(
        exec(&mut app, "setrender shadow_samples 2")
            .error
            .unwrap()
            .code,
        "out_of_range"
    );
    assert!(exec(&mut app, "shadowcache status").ok);
    assert_eq!(
        exec(&mut app, "shadowcache rebuild").error.unwrap().code,
        "prepare_required"
    );
    assert_eq!(
        exec(&mut app, "getrender").value.as_object().unwrap().len(),
        10
    );
}

#[test]
fn tonemap_reports_changes_and_rejects_invalid_requests_without_mutation() {
    let mut app = test_app();
    let initial = exec(&mut app, "tonemap");
    assert_eq!(initial.value["tonemapper"], "agx");
    assert_eq!(initial.log, ["Tonemapper is AgX."]);

    for (input, expected, display) in [
        ("none", "none", "None"),
        ("reinhard", "reinhard", "Reinhard"),
        (
            "reinhard_luminance",
            "reinhard_luminance",
            "Reinhard Luminance",
        ),
        ("aces_fitted", "aces_fitted", "ACES Fitted"),
        ("agx", "agx", "AgX"),
        (
            "somewhat_boring_display_transform",
            "somewhat_boring_display_transform",
            "Somewhat Boring Display Transform",
        ),
        ("tony_mc_mapface", "tony_mc_mapface", "Tony McMapface"),
        ("blender_filmic", "blender_filmic", "Blender Filmic"),
        (
            "khronos_pbr_neutral",
            "khronos_pbr_neutral",
            "Khronos PBR Neutral",
        ),
    ] {
        let changed = exec(&mut app, &format!("tonemap {input}"));
        assert_eq!(changed.value["tonemapper"], expected);
        assert_eq!(changed.log, [format!("Tonemapper set to {display}.")]);
        assert_eq!(
            current_tonemapper(&mut app),
            parse_tonemapper(expected).unwrap()
        );
    }

    let case_insensitive = exec(&mut app, "tonemap AGX");
    assert_eq!(case_insensitive.value["tonemapper"], "agx");
    assert_eq!(current_tonemapper(&mut app), Tonemapping::AgX);

    let before_invalid = current_tonemapper(&mut app);
    assert_eq!(
        exec(&mut app, "tonemap unsupported").error.unwrap().code,
        "unknown_tonemapper"
    );
    assert_eq!(current_tonemapper(&mut app), before_invalid);
    assert_eq!(
        exec(&mut app, "tonemap agx extra").error.unwrap().code,
        "bad_arity"
    );
    assert_eq!(current_tonemapper(&mut app), before_invalid);
}

#[test]
fn tonemap_rejects_missing_or_ambiguous_cameras() {
    let mut missing = test_app();
    let camera = {
        let world = missing.world_mut();
        let mut query = world.query_filtered::<Entity, With<Camera3d>>();
        query.single(world).unwrap()
    };
    missing.world_mut().despawn(camera);
    assert_eq!(
        exec(&mut missing, "tonemap agx").error.unwrap().code,
        "camera_unavailable"
    );

    let mut ambiguous = test_app();
    ambiguous
        .world_mut()
        .spawn((Camera3d::default(), Tonemapping::TonyMcMapface));
    let before = {
        let world = ambiguous.world_mut();
        let mut query = world.query_filtered::<&Tonemapping, With<Camera3d>>();
        *query.iter(world).next().unwrap()
    };
    assert_eq!(
        exec(&mut ambiguous, "tonemap none").error.unwrap().code,
        "camera_unavailable"
    );
    let after = {
        let world = ambiguous.world_mut();
        let mut query = world.query_filtered::<&Tonemapping, With<Camera3d>>();
        *query.iter(world).next().unwrap()
    };
    assert_eq!(after, before);
}

#[test]
fn forced_no_clip_cannot_enable_unavailable_collision() {
    let mut app = test_app();
    app.world_mut().resource_mut::<player::PlayerNoClip>().0 = true;
    app.world_mut().resource_mut::<player::PhysicsDisabled>().0 = true;
    assert_eq!(
        exec(&mut app, "tcl").error.unwrap().code,
        "physics_disabled"
    );
    assert!(app.world().resource::<player::PlayerNoClip>().0);

    app.world_mut().resource_mut::<player::PhysicsDisabled>().0 = false;
    app.world_mut()
        .resource_mut::<player::CameraModeState>()
        .collisions_ready = false;
    assert_eq!(
        exec(&mut app, "tcl").error.unwrap().code,
        "collision_unavailable"
    );
    assert!(app.world().resource::<player::PlayerNoClip>().0);
}

#[test]
fn console_suppression_preserves_tm_and_tdt_state() {
    let mut app = test_app();
    let game_ui = app.world_mut().spawn((GameUi, Visibility::Inherited)).id();
    let diagnostic_ui = app
        .world_mut()
        .spawn((DiagnosticUi, Visibility::Inherited))
        .id();

    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::Console);
    app.update();
    assert_eq!(
        app.world().get::<Visibility>(game_ui),
        Some(&Visibility::Hidden)
    );
    assert_eq!(
        app.world().get::<Visibility>(diagnostic_ui),
        Some(&Visibility::Hidden)
    );

    assert!(exec(&mut app, "tm").ok);
    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::None);
    app.update();
    assert_eq!(
        app.world().get::<Visibility>(game_ui),
        Some(&Visibility::Hidden)
    );
    assert_eq!(
        app.world().get::<Visibility>(diagnostic_ui),
        Some(&Visibility::Inherited)
    );

    assert!(exec(&mut app, "tm").ok);
    assert!(exec(&mut app, "tdt").ok);
    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::Console);
    app.update();
    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::None);
    app.update();
    assert_eq!(
        app.world().get::<Visibility>(game_ui),
        Some(&Visibility::Inherited)
    );
    assert_eq!(
        app.world().get::<Visibility>(diagnostic_ui),
        Some(&Visibility::Hidden)
    );
}

// -- activate (scripted door travel, M2 wave 2) -----------------------

/// Registers a placement built from minimal RON (avoids widening the
/// `vsa` re-export surface just for test constructors) under FormID
/// 0x10 / EditorID "TestRef".
fn register_placement(app: &mut App, semantic_ron: &str) {
    let ron = format!(
        "(
            reference_form_id: 16,
            base_form_id: 1,
            asset_path: None,
            translation: (0.0, 0.0, 0.0),
            rotation_xyzw: (0.0, 0.0, 0.0, 1.0),
            scale: 1.0,
            error: None,
            semantic: {semantic_ron},
        )"
    );
    let placement: crate::vsa::PreparedPlacement = ron::de::from_str(&ron).unwrap();
    let entity = app
        .world_mut()
        .spawn((
            Transform::default(),
            interaction::PlacementRoot::new(placement),
        ))
        .id();
    app.world_mut()
        .resource_mut::<crate::console::RefRegistry>()
        .register(entity, 0x10, Some("TestRef"));
}

const DOOR_WITH_DESTINATION: &str = "Door((
    lock_level: None,
    key_form_id: None,
    destination: Some((
        door_reference_form_id: 32,
        cell_form_id: 148753,
        translation: (1.0, 2.0, 3.0),
        rotation_xyzw: (0.0, 0.0, 0.0, 1.0),
    )),
))";

fn error_code(output: &crate::console::ConsoleOutput) -> String {
    output
        .error
        .as_ref()
        .expect("expected an error")
        .code
        .clone()
}

fn register_actor_placement(app: &mut App) -> bevyout_core::item_transaction::ItemInstanceId {
    use bevyout_core::actor::{
        ActorAssemblyBlueprint, ActorAttachmentPoint, ActorFallbackDecision, ActorFallbackLevel,
        ActorFallbackReason, ActorKind, ActorMeshRole, ActorProxyKind, AssembledApparel,
        AssembledMeshPart, AssembledWeapon, FaceGenPolicy,
    };

    register_placement(app, "Npc((base_template_form_id: None, assembly: None))");
    let root = app
        .world_mut()
        .query_filtered::<Entity, With<interaction::PlacementRoot>>()
        .single(app.world())
        .unwrap();
    let assembly = ActorAssemblyBlueprint {
        source_base_form_id: 1,
        resolved_base_form_id: 2,
        reference_form_id: 16,
        kind: ActorKind::Humanoid,
        female: true,
        race_form_id: Some(3),
        root_scale: 1.125,
        skeleton_path: Some("meshes/characters/_male/skeleton.nif".into()),
        mesh_parts: vec![AssembledMeshPart {
            name: "head".into(),
            source_form_id: Some(4),
            model_path: "meshes/characters/head.nif".into(),
            attachment_point: ActorAttachmentPoint::Head,
            role: ActorMeshRole::Head(0),
            is_visible: true,
        }],
        apparel: vec![AssembledApparel {
            item_form_id: 5,
            model_path: Some("assets/armor.glb".into()),
            biped_slot_mask: 4,
            model_available: true,
        }],
        eye_form_id: Some(7),
        eye_texture_path: Some("textures/characters/eyes/brown.dds".into()),
        equipped_weapon: Some(AssembledWeapon {
            item_form_id: 6,
            model_path: Some("assets/rifle.glb".into()),
            attachment_point: ActorAttachmentPoint::RightHand,
            model_available: true,
        }),
        fallback: ActorFallbackDecision {
            base_form_id: 2,
            reference_form_id: 16,
            level: ActorFallbackLevel::RaceSexSpecific,
            facegen_policy: FaceGenPolicy::RestPoseFallback,
            proxy_kind: ActorProxyKind::None,
            reasons: vec![ActorFallbackReason::MissingFaceGen],
        },
    };
    let holder = HolderId::Actor {
        reference_form_id: 16,
    };
    let item_id = {
        let mut canonical = app
            .world_mut()
            .resource_mut::<interaction::CanonicalItemLedger>();
        canonical
            .ledger
            .insert_holder(
                holder,
                bevyout_core::item_transaction::ItemHolderState::default(),
            )
            .unwrap();
        let item_id = canonical
            .ledger
            .insert_new_item(
                holder,
                6,
                1,
                bevyout_core::item_transaction::ItemState::default(),
            )
            .unwrap();
        canonical.ledger.equip(holder, item_id).unwrap();
        item_id
    };
    app.world_mut().entity_mut(root).insert((
        actor::ActorRuntime {
            base_form_id: 2,
            reference_form_id: 16,
            kind: ActorKind::Humanoid,
            assembly: Some(assembly),
        },
        actor::ActorRuntimeState {
            holder,
            holder_seeded: true,
            proxy_entity: None,
            bound_item_form_id: Some(6),
            weapon_model: Some(actor::ActorWeaponModel {
                item_form_id: 6,
                model_path: "assets/rifle.glb".into(),
            }),
            weapon: actor::ActorWeaponRuntimeState::MissingAttachmentNode {
                expected: "RightHand/Weapon".into(),
            },
            diagnostics: vec![actor::ActorRuntimeDiagnostic {
                code: "missing_weapon_attachment_node",
                message: "weapon socket is missing".into(),
            }],
        },
    ));
    item_id
}

#[test]
fn actorinspect_reports_prepared_and_runtime_assembly_state() {
    let mut app = test_app();
    let item_id = register_actor_placement(&mut app);

    let output = exec(&mut app, "actorinspect 00000010");
    assert!(output.ok, "actorinspect failed: {:?}", output.error);
    assert_eq!(output.value["reference_form_id"], 16);
    assert_eq!(output.value["source_base_form_id"], 1);
    assert_eq!(output.value["base_form_id"], 2);
    assert_eq!(output.value["kind"], "humanoid");
    assert_eq!(output.value["scale"], 1.125);
    assert_eq!(output.value["fallback"]["tier"], "RaceSexSpecific");
    assert_eq!(
        output.value["fallback"]["reasons"][0]["code"],
        "missing_facegen"
    );
    assert_eq!(output.value["parts"][0]["role"], "Head(0)");
    assert_eq!(output.value["eyes"]["form_id"], 7);
    assert_eq!(
        output.value["eyes"]["texture_path"],
        "textures/characters/eyes/brown.dds"
    );
    assert_eq!(output.value["apparel"][0]["item_form_id"], 5);
    assert_eq!(output.value["weapon"]["prepared"]["item_form_id"], 6);
    assert_eq!(
        output.value["weapon"]["runtime"]["state"]["status"],
        "missing_attachment_node"
    );
    assert_eq!(output.value["runtime"]["holder_seeded"], true);
    assert_eq!(output.value["runtime"]["canonical"]["present"], true);
    assert_eq!(
        output.value["runtime"]["canonical"]["equipped_instance_id"],
        item_id.0
    );
    assert_eq!(
        output.value["runtime"]["canonical"]["items"][0]["base_form_id"],
        6
    );
    assert_eq!(
        output.value["runtime"]["diagnostics"][0]["code"],
        "missing_weapon_attachment_node"
    );
    assert!(output.log[0].contains("tier=RaceSexSpecific"));
}

#[test]
fn actorinspect_rejects_bad_arity_and_non_actor_references() {
    let mut app = test_app();
    assert_eq!(error_code(&exec(&mut app, "actorinspect")), "bad_arity");
    register_placement(&mut app, "Static");
    assert_eq!(
        error_code(&exec(&mut app, "actorinspect 00000010")),
        "not_actor"
    );
}

#[test]
fn activate_requires_exactly_one_reference() {
    let mut app = test_app();
    assert_eq!(error_code(&exec(&mut app, "activate")), "bad_arity");
    assert_eq!(error_code(&exec(&mut app, "activate a b")), "bad_arity");
}

#[test]
fn activate_rejects_non_door_and_destination_less_references() {
    let mut app = test_app();
    register_placement(&mut app, "Static");
    assert_eq!(
        error_code(&exec(&mut app, "activate 00000010")),
        "not_a_door"
    );

    let mut app = test_app();
    register_placement(
        &mut app,
        "Door((lock_level: None, key_form_id: None, destination: None))",
    );
    assert_eq!(
        error_code(&exec(&mut app, "activate TestRef")),
        "no_destination"
    );
}

// Wave-4 amendment: containers toggle their open state through the
// console, so the persistence gate can be driven over the agent bridge.
#[test]
fn activate_toggles_a_container_open_and_closed() {
    let mut app = test_app();
    app.add_message::<super::super::audio::PlaySound>();
    app.add_message::<super::super::animation::PlayPlacementAnimation>();
    register_placement(&mut app, "Container");
    let output = exec(&mut app, "activate 00000010");
    assert!(output.ok, "activate failed: {:?}", output.error);
    assert_eq!(output.value["opened"], true);
    let output = exec(&mut app, "activate 00000010");
    assert!(output.ok, "activate failed: {:?}", output.error);
    assert_eq!(output.value["opened"], false);
}

fn register_corpse_placement(app: &mut App) {
    let ron = r#"(
        reference_form_id: 16,
        base_form_id: 1,
        asset_path: None,
        translation: (0.0, 0.0, 0.0),
        rotation_xyzw: (0.0, 0.0, 0.0, 1.0),
        scale: 1.0,
        error: None,
        semantic: Corpse,
        inventory: [
            (base_form_id: 2, count: 3, record_kind: "MISC", editor_id: Some("CorpseItem"), display_name: Some("Corpse Item"), leveled: false),
        ],
    )"#;
    let placement: crate::vsa::PreparedPlacement = ron::de::from_str(ron).unwrap();
    let entity = app
        .world_mut()
        .spawn((
            Transform::default(),
            interaction::PlacementRoot::new(placement),
        ))
        .id();
    app.world_mut()
        .resource_mut::<crate::console::RefRegistry>()
        .register(entity, 0x10, Some("TestCorpse"));
}

// F118.2: scripted corpse activation uses the loot-holder transfer seam,
// seeds the stable FormID-keyed state, and requests the existing modal.
#[test]
fn activate_opens_a_corpse_holder_with_stable_console_output() {
    let mut app = test_app();
    app.add_message::<super::super::audio::PlaySound>();
    app.add_message::<super::super::animation::PlayPlacementAnimation>();
    app.add_message::<crate::app_state::RequestStateTransition>();
    register_corpse_placement(&mut app);

    let output = exec(&mut app, "activate TestCorpse");
    assert!(output.ok, "activate failed: {:?}", output.error);
    assert_eq!(output.value["reference_form_id"], 16);
    assert_eq!(output.value["kind"], "corpse");
    assert_eq!(output.value["opened"], true);
    assert_eq!(output.log, ["corpse 00000010 opened"]);
    assert_eq!(
        app.world().resource::<interaction::ContainerStates>().0[&0x10].stacks,
        vec![(0x2, 3)]
    );
    let requests = app
        .world()
        .resource::<Messages<crate::app_state::RequestStateTransition>>();
    assert!(requests.iter_current_update_messages().any(|request| {
        *request == crate::app_state::RequestStateTransition::Modal(GameplayModal::Container)
    }));
}

// F118.2: non-corpse actor references remain unsupported until the actor
// simulation/death slice exists; the error is deterministic.
#[test]
fn activate_rejects_a_live_actor_as_a_corpse_holder() {
    let mut app = test_app();
    register_placement(&mut app, "Npc((base_template_form_id: None))");
    assert_eq!(
        error_code(&exec(&mut app, "activate TestRef")),
        "not_a_door"
    );
}

// -- activate pickup (issue #84, F84.2) --------------------------------

const PICKUP_SEMANTIC: &str = "Pickup((category: \"Misc\", value: None, weight: None))";

#[test]
fn activate_picks_up_a_prepared_item_and_despawns_the_reference() {
    let mut app = test_app();
    register_placement(&mut app, PICKUP_SEMANTIC);
    let entity = app
        .world_mut()
        .query_filtered::<Entity, With<interaction::PlacementRoot>>()
        .single(app.world())
        .unwrap();
    let output = exec(&mut app, "activate 00000010");
    assert!(output.ok, "activate failed: {:?}", output.error);
    assert_eq!(output.value["reference_form_id"], 16);
    assert_eq!(output.value["base_form_id"], 1);
    assert_eq!(output.value["count"], 1);
    assert_eq!(output.log, ["picked up 00000001 x1"]);
    assert_eq!(
        app.world()
            .resource::<interaction::PlayerInventory>()
            .count(1),
        1
    );
    assert!(!app.world().entities().contains(entity));
}

// The pickup route only accepts `PreparedSemantic::Pickup`; every other
// unsupported semantic (Furniture here, matching the door/container
// routes rejecting `Static` above) still errors deterministically.
#[test]
fn activate_rejects_unsupported_semantics_for_pickup() {
    let mut app = test_app();
    register_placement(&mut app, "Furniture");
    assert_eq!(
        error_code(&exec(&mut app, "activate 00000010")),
        "not_a_door"
    );
}

#[test]
fn activate_door_with_destination_writes_a_travel_request() {
    let mut app = test_app();
    app.add_message::<interaction::DoorTravelRequested>();
    register_placement(&mut app, DOOR_WITH_DESTINATION);
    let output = exec(&mut app, "activate 00000010");
    assert!(output.ok, "activate failed: {:?}", output.error);
    assert_eq!(output.value["destination_cell_form_id"], 148753);
    let requests = app
        .world()
        .resource::<Messages<interaction::DoorTravelRequested>>();
    let request = requests
        .iter_current_update_messages()
        .next()
        .expect("expected a DoorTravelRequested message");
    assert_eq!(request.destination_cell_form_id, 148753);
    assert_eq!(request.translation, Vec3::new(1.0, 2.0, 3.0));
}

// -- save (issue #60, F60.3) ------------------------------------------

#[test]
fn save_validates_arity_and_fails_deterministically_without_a_world() {
    let mut app = test_app();
    assert_eq!(error_code(&exec(&mut app, "save")), "bad_arity");
    assert_eq!(error_code(&exec(&mut app, "save a b")), "bad_arity");
    // The console harness has no active cell, so the failure is a
    // structured error rather than a panic.
    assert_eq!(error_code(&exec(&mut app, "save slot1")), "save_failed");
}

/// Wave-3 amendment: a door with a discovered Open clip stages its travel
/// behind the open lead instead of firing it the same frame, matching the
/// player's Enter activation.
#[test]
fn activate_animated_door_defers_travel_by_the_open_lead() {
    let mut app = test_app();
    app.add_message::<interaction::DoorTravelRequested>();
    register_placement(&mut app, DOOR_WITH_DESTINATION);
    let root = app
        .world_mut()
        .query_filtered::<Entity, With<interaction::PlacementRoot>>()
        .single(app.world())
        .unwrap();
    let player = app.world_mut().spawn_empty().id();
    app.world_mut()
        .entity_mut(root)
        .insert(super::super::animation::AnimatedPlacement::for_test(
            player,
            &[("Open", 1.33)],
        ));
    let output = exec(&mut app, "activate 00000010");
    assert!(output.ok, "activate failed: {:?}", output.error);
    let lead = output.value["open_lead_ms"].as_f64().unwrap();
    assert!(lead > 0.0, "expected a nonzero open lead, got {lead}");
    let requests = app
        .world()
        .resource::<Messages<interaction::DoorTravelRequested>>();
    assert_eq!(
        requests.iter_current_update_messages().count(),
        0,
        "travel must be staged behind the open lead, not fired same-frame"
    );
}

// -- additem (issue #84, F84.1) -----------------------------------------

#[test]
fn additem_rejects_bad_arity_bad_count_and_bad_form_id() {
    let mut app = test_app();
    assert_eq!(error_code(&exec(&mut app, "additem")), "bad_arity");
    assert_eq!(error_code(&exec(&mut app, "additem 1 2 3")), "bad_arity");
    assert_eq!(
        error_code(&exec(&mut app, "additem 00000001 0")),
        "bad_count"
    );
    assert_eq!(
        error_code(&exec(&mut app, "additem 00000001 -1")),
        "bad_count"
    );
    // Bare short hex is a valid FormID, so an invalid count is reported
    // as bad_count, not masked by the FormID parse.
    assert_eq!(error_code(&exec(&mut app, "additem f -5")), "bad_count");
    assert_eq!(error_code(&exec(&mut app, "additem zz")), "bad_type");
    assert_eq!(
        error_code(&exec(&mut app, "additem 123456789 1")),
        "bad_type"
    );
    assert_eq!(
        app.world()
            .resource::<interaction::PlayerInventory>()
            .count(1),
        0
    );
    assert_eq!(
        app.world()
            .resource::<interaction::PlayerInventory>()
            .count(0xf),
        0
    );
}

// Issue #84's gate evidence line is literally `player.additem f 100`:
// the Bethesda console accepts bare short hex without a 0x prefix, and
// the player. prefix is accepted without selecting a container.
#[test]
fn additem_accepts_bare_short_hex_and_the_player_prefix() {
    let mut app = test_app();
    let player = app.world_mut().spawn(Transform::default()).id();
    app.world_mut()
        .resource_mut::<crate::console::RefRegistry>()
        .set_player(player);
    let output = exec(&mut app, "player.additem f 100");
    assert!(output.ok, "additem failed: {:?}", output.error);
    assert_eq!(output.value["form_id"], 15);
    assert_eq!(output.value["count"], 100);
    assert_eq!(output.value["total"], 100);
    assert_eq!(output.log, ["additem 0000000f x100; inventory now has 100"]);
    assert_eq!(
        app.world()
            .resource::<interaction::PlayerInventory>()
            .count(0xf),
        100
    );
}

#[test]
fn additem_adds_the_default_count_of_one() {
    let mut app = test_app();
    let output = exec(&mut app, "additem 00000005");
    assert!(output.ok, "additem failed: {:?}", output.error);
    assert_eq!(output.value["form_id"], 5);
    assert_eq!(output.value["count"], 1);
    assert_eq!(output.value["total"], 1);
    assert_eq!(output.log, ["additem 00000005 x1; inventory now has 1"]);
    assert_eq!(
        app.world()
            .resource::<interaction::PlayerInventory>()
            .count(5),
        1
    );
}

#[test]
fn additem_adds_a_requested_count_seeded_with_catalog_condition() {
    let mut app = test_app();
    app.insert_resource(PreparedItemCatalog {
        revision: "test".into(),
        source_fingerprint: "test".into(),
        items: vec![PreparedItemDefinition {
            base_form_id: 0x42,
            record_kind: "WEAP".into(),
            category: PreparedItemCategory::Weapons,
            editor_id: None,
            display_name: None,
            source_model_path: None,
            icon_asset_path: None,
            world_asset_path: None,
            physics_asset_path: None,
            drop_collider: Default::default(),
            value: None,
            weight: None,
            quest_item: false,
            stats: PreparedItemStats::Weapon {
                damage: None,
                max_condition: Some(255),
                clip_size: None,
                speed: None,
                reach: None,
                ammo_form_id: None,
            },
            audio: Default::default(),
        }],
    });
    let output = exec(&mut app, "additem 00000042 3");
    assert!(output.ok, "additem failed: {:?}", output.error);
    assert_eq!(output.value["count"], 3);
    assert_eq!(output.value["total"], 3);
    assert_eq!(output.log, ["additem 00000042 x3; inventory now has 3"]);
    let stack = app
        .world()
        .resource::<interaction::PlayerInventory>()
        .stack_states()
        .into_iter()
        .find(|stack| stack.base_form_id == 0x42)
        .expect("expected the added stack");
    assert_eq!(stack.count, 3);
    assert_eq!(stack.condition, Some(255));
}

#[test]
fn additem_without_a_catalog_entry_adds_with_no_condition() {
    let mut app = test_app();
    let output = exec(&mut app, "additem 000000aa 2");
    assert!(output.ok, "additem failed: {:?}", output.error);
    assert_eq!(output.value["total"], 2);
    let stack = app
        .world()
        .resource::<interaction::PlayerInventory>()
        .stack_states()
        .into_iter()
        .find(|stack| stack.base_form_id == 0xaa)
        .expect("expected the added stack");
    assert_eq!(stack.count, 2);
    assert_eq!(stack.condition, None);
}

// -- equipitem (issue #98, F98.4) --------------------------------------

fn apparel_item(base_form_id: u32, biped_slot_mask: u32) -> PreparedItemDefinition {
    PreparedItemDefinition {
        base_form_id,
        record_kind: "ARMO".into(),
        category: PreparedItemCategory::Apparel,
        editor_id: None,
        display_name: None,
        source_model_path: None,
        icon_asset_path: None,
        world_asset_path: None,
        physics_asset_path: None,
        drop_collider: Default::default(),
        value: None,
        weight: None,
        quest_item: false,
        stats: PreparedItemStats::Apparel {
            armor_rating: None,
            max_condition: None,
            biped_slot_mask: Some(biped_slot_mask),
        },
        audio: Default::default(),
    }
}

#[test]
fn equipitem_rejects_bad_arity_and_bad_form_id() {
    let mut app = test_app();
    assert_eq!(error_code(&exec(&mut app, "equipitem")), "bad_arity");
    assert_eq!(error_code(&exec(&mut app, "equipitem 1 2")), "bad_arity");
    assert_eq!(error_code(&exec(&mut app, "equipitem zz")), "bad_type");
}

#[test]
fn equipitem_rejects_a_form_id_not_in_the_inventory() {
    let mut app = test_app();
    assert_eq!(
        error_code(&exec(&mut app, "equipitem 00000001")),
        "not_in_inventory"
    );
}

#[test]
fn equipitem_rejects_a_form_id_with_no_catalog_entry() {
    let mut app = test_app();
    exec(&mut app, "additem 00000001");
    assert_eq!(
        error_code(&exec(&mut app, "equipitem 00000001")),
        "no_catalog_entry"
    );
}

#[test]
fn equipitem_equips_and_toggling_again_unequips() {
    let mut app = test_app();
    app.insert_resource(PreparedItemCatalog {
        revision: "test".into(),
        source_fingerprint: "test".into(),
        items: vec![apparel_item(0x10, 0x1)],
    });
    exec(&mut app, "additem 00000010");
    let output = exec(&mut app, "player.equipitem 00000010");
    assert!(output.ok, "equipitem failed: {:?}", output.error);
    assert_eq!(output.value["equipped"], true);
    assert_eq!(output.log, ["equipitem 00000010 equipped"]);
    assert!(
        app.world()
            .resource::<interaction::PlayerEquipment>()
            .is_equipped(super::super::inventory::StackKey {
                base_form_id: 0x10,
                condition: None,
            })
    );

    let output = exec(&mut app, "equipitem 00000010");
    assert!(output.ok, "equipitem failed: {:?}", output.error);
    assert_eq!(output.value["equipped"], false);
    assert_eq!(output.log, ["equipitem 00000010 unequipped"]);
    assert!(
        !app.world()
            .resource::<interaction::PlayerEquipment>()
            .is_equipped(super::super::inventory::StackKey {
                base_form_id: 0x10,
                condition: None,
            })
    );
}

#[test]
fn equipitem_evicts_the_previous_occupant_of_a_shared_slot() {
    let mut app = test_app();
    app.insert_resource(PreparedItemCatalog {
        revision: "test".into(),
        source_fingerprint: "test".into(),
        items: vec![apparel_item(0x10, 0x1), apparel_item(0x11, 0x1)],
    });
    exec(&mut app, "additem 00000010");
    exec(&mut app, "additem 00000011");
    exec(&mut app, "equipitem 00000010");
    let output = exec(&mut app, "equipitem 00000011");
    assert!(output.ok, "equipitem failed: {:?}", output.error);
    assert_eq!(output.value["evicted"], json!(["00000010"]));
}

#[test]
fn equipitem_rejects_ammo_with_no_weapon_equipped() {
    let mut app = test_app();
    app.insert_resource(PreparedItemCatalog {
        revision: "test".into(),
        source_fingerprint: "test".into(),
        items: vec![PreparedItemDefinition {
            base_form_id: 0x20,
            record_kind: "AMMO".into(),
            category: PreparedItemCategory::Ammo,
            editor_id: None,
            display_name: None,
            source_model_path: None,
            icon_asset_path: None,
            world_asset_path: None,
            physics_asset_path: None,
            drop_collider: Default::default(),
            value: None,
            weight: None,
            quest_item: false,
            stats: PreparedItemStats::Ammo {
                damage: None,
                speed: None,
            },
            audio: Default::default(),
        }],
    });
    exec(&mut app, "additem 00000020");
    assert_eq!(
        error_code(&exec(&mut app, "equipitem 00000020")),
        "no_weapon_equipped"
    );
}

#[test]
fn canonical_instance_commands_update_ledger_and_runtime_projections() {
    let mut app = test_app();
    app.insert_resource(PreparedItemCatalog {
        revision: "test".into(),
        source_fingerprint: "test".into(),
        items: vec![apparel_item(0x10, 0x1)],
    });
    assert!(exec(&mut app, "additem 00000010").ok);
    let item_id = app
        .world()
        .resource::<interaction::CanonicalItemLedger>()
        .ledger
        .holders()[&HolderId::Player]
        .items[0]
        .id;
    let key = super::super::inventory::StackKey {
        base_form_id: 0x10,
        condition: None,
    };

    let equipped = exec(&mut app, &format!("equip {:016x}", item_id.0));
    assert!(equipped.ok, "equip failed: {:?}", equipped.error);
    assert_eq!(
        app.world()
            .resource::<interaction::CanonicalItemLedger>()
            .ledger
            .bindings()[&HolderId::Player]
            .equipped,
        Some(item_id)
    );
    assert!(
        app.world()
            .resource::<interaction::PlayerEquipment>()
            .is_equipped(key)
    );

    let hotkey = exec(&mut app, &format!("hotkey 0 {:016x}", item_id.0));
    assert!(hotkey.ok, "hotkey failed: {:?}", hotkey.error);
    assert_eq!(
        app.world()
            .resource::<interaction::CanonicalItemLedger>()
            .ledger
            .bindings()[&HolderId::Player]
            .hotkeys[0],
        Some(item_id)
    );
    assert_eq!(
        app.world()
            .resource::<super::super::bindings::HotkeyBindings>()
            .get(1),
        Some(key)
    );

    let unequipped = exec(&mut app, "unequip");
    assert!(unequipped.ok, "unequip failed: {:?}", unequipped.error);
    assert_eq!(
        app.world()
            .resource::<interaction::CanonicalItemLedger>()
            .ledger
            .bindings()[&HolderId::Player]
            .equipped,
        None
    );
    assert!(
        !app.world()
            .resource::<interaction::PlayerEquipment>()
            .is_equipped(key)
    );
}

#[test]
fn canonical_instance_commands_fail_without_partial_mutation() {
    let mut app = test_app();
    assert!(exec(&mut app, "additem 00000010").ok);
    let item_id = app
        .world()
        .resource::<interaction::CanonicalItemLedger>()
        .ledger
        .holders()[&HolderId::Player]
        .items[0]
        .id;
    let before_ledger = app
        .world()
        .resource::<interaction::CanonicalItemLedger>()
        .snapshot();
    let before_equipped = app
        .world()
        .resource::<interaction::PlayerEquipment>()
        .is_equipped(super::super::inventory::StackKey {
            base_form_id: 0x10,
            condition: None,
        });
    assert_eq!(
        error_code(&exec(&mut app, &format!("equip {:016x}", item_id.0))),
        "no_catalog_entry"
    );
    assert_eq!(
        app.world()
            .resource::<interaction::CanonicalItemLedger>()
            .snapshot(),
        before_ledger
    );
    assert_eq!(
        app.world()
            .resource::<interaction::PlayerEquipment>()
            .is_equipped(super::super::inventory::StackKey {
                base_form_id: 0x10,
                condition: None,
            }),
        before_equipped
    );

    assert_eq!(
        error_code(&exec(&mut app, "hotkey 0 deadbeef")),
        "item_not_found"
    );
    assert_eq!(
        app.world()
            .resource::<interaction::CanonicalItemLedger>()
            .snapshot(),
        before_ledger
    );
    assert!(
        app.world()
            .get_resource::<super::super::bindings::HotkeyBindings>()
            .is_none()
    );
}
