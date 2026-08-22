use super::*;
use crate::console::{ConsoleExecutor, ConsolePlugin, ConsoleRequest, ConsoleSessionId};
use crate::viewer::world::exterior::ExteriorWorldspaceLodSettings;
use crate::viewer::{ImageSpaceBloomOverrides, LegacyChanSettings, OverlayLightingSettings};
use crate::vsa::{PreparedItemCategory, PreparedItemDefinition, PreparedSceneManifest};
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
        .insert_resource(VolumetricFogMultiplier(1.0))
        .insert_resource(AoStrength(1.0))
        .insert_resource(EmissionScale(0.0))
        .insert_resource(MaterialClampSettings::default())
        .insert_resource(LegacyChanSettings::default())
        .insert_resource(OverlayLightingSettings::default())
        .insert_resource(super::super::controls::ReflectionProbeSettings::default())
        .insert_resource(ImageSpaceBloomOverrides::default())
        .init_resource::<super::super::screen_fx::ScreenFxRuntime>()
        .init_resource::<super::super::screen_fx::ScreenFxCatalog>()
        .init_resource::<Messages<super::super::screen_fx::ScreenFxRequested>>()
        .insert_resource(UnlitMode(false))
        .insert_resource(LightsDisabled(false))
        .insert_resource(PreparedPointShadowRuntime::default())
        .insert_resource(ExteriorWorldspaceLodSettings { enabled: false })
        .insert_resource(PointLightShadowSamples::default())
        .insert_resource(BoxdddDebugDrawSettings::default())
        .insert_resource(player::StepDebugSettings::default())
        .insert_resource(interaction::PlayerInventory::default())
        .insert_resource(interaction::PlayerEquipment::default());
    app.insert_resource(super::super::day_night::GameClock::default())
        .insert_resource(super::super::day_night::DayNightPreview::default());
    app.init_resource::<interaction::CanonicalItemLedger>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<super::super::world::ActiveSaveState>();
    app.init_resource::<super::super::actor_state::ActorDefinitionCatalogs>();
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
    app.add_plugins((
        super::super::stats::StatsPlugin,
        super::super::effects::EffectsPlugin,
    ));
    player::set_camera_mode(app.world_mut(), player::CameraMode::Fps).unwrap();
    app.update();
    app
}

#[test]
fn catalog_cache_keys_are_unambiguous_across_component_boundaries() {
    let left = super::ai_package_commands::catalog_cache_key(&["a|b", "c"]);
    let right = super::ai_package_commands::catalog_cache_key(&["a", "b|c"]);
    let same = super::ai_package_commands::catalog_cache_key(&["a|b", "c"]);

    assert_ne!(left, right);
    assert_eq!(left, same);
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
fn dialogue_commands_expose_state_and_queue_visible_choices() {
    let mut app = test_app();
    app.insert_resource(ButtonInput::<KeyCode>::default())
        .add_message::<crate::app_state::RequestStateTransition>();
    app.add_plugins(crate::viewer::dialogue::DialoguePlugin);
    app.update();

    let start = exec(&mut app, "dialoguestart MoiraBrown");
    assert!(start.ok);
    assert_eq!(start.value["dialogue"], "MoiraBrown");

    let reload = exec(&mut app, "dialoguereload authored/moira_brown.yarn");
    assert!(reload.ok);
    assert_eq!(reload.value["queued"], true);
    assert_eq!(reload.value["source_paths"][0], "authored/moira_brown.yarn");
    assert_eq!(
        app.world()
            .resource::<Messages<crate::viewer::dialogue::DialogueStartRequested>>()
            .iter_current_update_messages()
            .count(),
        1
    );

    {
        let mut runtime = app
            .world_mut()
            .resource_mut::<crate::viewer::dialogue::DialogueRuntime>();
        runtime.readiness = crate::viewer::dialogue::DialogueReadiness::Ready;
        runtime.phase = bevyout_core::dialogue::DialoguePhase::PresentingOptions;
        runtime.presentation.options = vec![
            bevyout_core::dialogue::DialogueOptionPresentation {
                choice: bevyout_core::dialogue::DialogueChoiceId::new("MoiraBrown:0"),
                text: "Ask about the crater".into(),
                line_key: None,
                enabled: true,
            },
            bevyout_core::dialogue::DialogueOptionPresentation {
                choice: bevyout_core::dialogue::DialogueChoiceId::new("MoiraBrown:1"),
                text: "Ask about supplies".into(),
                line_key: None,
                enabled: true,
            },
        ];
    }

    let state = exec(&mut app, "dialoguestate");
    assert!(state.ok);
    assert_eq!(state.value["phase"], "PresentingOptions");
    assert_eq!(state.value["voice_anchor"], "Unanchored");
    assert_eq!(state.value["voice_spatial"], false);
    assert_eq!(state.value["options"][1]["text"], "Ask about supplies");

    app.world_mut()
        .resource_mut::<crate::viewer::dialogue::DialogueRuntime>()
        .voice_anchor = crate::viewer::dialogue::DialogueVoiceAnchorKind::Mouth;
    let state = exec(&mut app, "dialoguestate");
    assert_eq!(state.value["voice_anchor"], "Mouth");
    assert_eq!(state.value["voice_spatial"], true);

    let choice = exec(&mut app, "dialoguechoice 2");
    assert!(choice.ok);
    assert_eq!(choice.value["choice"], "MoiraBrown:1");
    assert_eq!(
        app.world()
            .resource::<Messages<crate::viewer::dialogue::DialogueChoiceSelected>>()
            .iter_current_update_messages()
            .count(),
        1
    );
    assert_eq!(
        exec(&mut app, "dialoguechoice 3").error.unwrap().code,
        "out_of_range"
    );
}

#[test]
fn weapon_commands_expose_state_and_queue_normal_action_requests() {
    let mut app = test_app();
    app.init_resource::<super::super::weapon::PlayerWeaponRuntime>()
        .add_message::<super::super::weapon::FireWeaponRequested>()
        .add_message::<super::super::weapon::ReloadWeaponRequested>()
        .add_message::<super::super::weapon::ClearWeaponJamRequested>();

    let state = exec(&mut app, "weaponstate");
    assert!(state.ok);
    assert_eq!(state.value["action"], "idle");
    assert_eq!(state.value["ammo_accounting"], true);
    let ammo = exec(&mut app, "ammostate player");
    assert!(ammo.ok);
    assert_eq!(ammo.value["schema"], "bevyout.m5.inspect");
    assert_eq!(ammo.value["available"], true);
    let combat = exec(&mut app, "combatstate");
    assert_eq!(combat.value["capabilities"]["ammo"], true);
    assert_eq!(combat.value["capabilities"]["condition"], true);
    assert_eq!(combat.value["capabilities"]["vats"], false);
    let vats = exec(&mut app, "vatsstate");
    assert_eq!(vats.value["available"], false);
    assert_eq!(vats.value["reason"], "planned_wave_7");
    let hitboxes = exec(&mut app, "hitboxdebug state");
    assert_eq!(hitboxes.value["available"], false);
    assert_eq!(hitboxes.value["reason"], "planned_wave_6");

    assert!(exec(&mut app, "weaponfire").ok);
    assert!(exec(&mut app, "weaponreload").ok);
    assert!(exec(&mut app, "weaponclearjam").ok);
    assert_eq!(
        app.world()
            .resource::<Messages<super::super::weapon::FireWeaponRequested>>()
            .iter_current_update_messages()
            .count(),
        1
    );
    assert_eq!(
        app.world()
            .resource::<Messages<super::super::weapon::ReloadWeaponRequested>>()
            .iter_current_update_messages()
            .count(),
        1
    );
    assert_eq!(
        app.world()
            .resource::<Messages<super::super::weapon::ClearWeaponJamRequested>>()
            .iter_current_update_messages()
            .count(),
        1
    );
}

#[test]
fn screen_fx_commands_report_and_queue_catalog_modifier_lifecycle() {
    let mut app = test_app();
    let help = exec(&mut app, "help screenfx");
    assert!(help.ok);
    assert!(
        help.value["signature"]
            .as_str()
            .expect("screenfx help signature")
            .contains("settings")
    );

    app.world_mut()
        .resource_mut::<super::super::screen_fx::ScreenFxCatalog>()
        .modifiers
        .insert(
            0x0000_1234,
            bevyout_core::image_space::ImageSpaceModifier {
                form_id: 0x0000_1234,
                editor_id: Some("SyntheticFlash".into()),
                duration_ms: 250,
                ..default()
            },
        );

    let status = exec(&mut app, "screenfx status");
    assert!(status.ok);
    assert_eq!(status.value["schema"], "bevyout.m5.screen_fx");
    assert_eq!(status.value["catalog_records"], 1);

    let start = exec(&mut app, "screenfx start 1234 7");
    assert!(start.ok);
    assert_eq!(start.value["form_id"], "00001234");
    assert_eq!(start.value["priority"], 7);

    let settings = exec(&mut app, "screenfx settings 0.5 0.25 0 0.75");
    assert!(settings.ok);
    assert_eq!(settings.value["screen_blood"], 0.25);
    assert_eq!(
        app.world()
            .resource::<super::super::screen_fx::ScreenFxRuntime>()
            .policy
            .settings()
            .motion_and_distortion,
        0.75
    );

    let stop = exec(&mut app, "screenfx stop 1234");
    assert!(stop.ok);
    let clear = exec(&mut app, "screenfx clear death");
    assert!(clear.ok);
    assert_eq!(clear.value["reason"], "Death");
    assert_eq!(
        app.world()
            .resource::<Messages<super::super::screen_fx::ScreenFxRequested>>()
            .iter_current_update_messages()
            .count(),
        3
    );
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
    assert!(exec(&mut app, "settime 24").ok);
    assert_eq!(
        app.world()
            .resource::<super::super::day_night::GameClock>()
            .hour,
        0.0
    );
    assert!(exec(&mut app, "settimescale 1440").ok);
    let time = exec(&mut app, "gettime");
    assert_eq!(time.value["timescale"], 1440.0);
    assert_eq!(time.value["cycle_seconds"], 60.0);
    assert_eq!(
        app.world().resource::<Time<Virtual>>().relative_speed(),
        2.0,
        "Fallout timescale must not alter sgtm"
    );
    assert_eq!(
        exec(&mut app, "settimescale 86401").error.unwrap().code,
        "out_of_range"
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
    let actor_animation_help = exec(&mut app, "help actoranim");
    assert!(actor_animation_help.ok);
    assert_eq!(actor_animation_help.value["mutating"], true);
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
    assert_eq!(
        exec(&mut app, "getrender reflection_probe_strength").value["value"],
        10.0
    );
    for (setting, low, high) in [
        ("lighting", 0.0001, 262_144.0),
        ("irradiance", 0.0, 4096.0),
        ("ambient", 0.0001, 4096.0),
        ("bloom_intensity", 0.0, 1.0),
        ("bloom_softness", 0.0, 1.0),
        ("fog", 0.0, 1.0),
        ("volumetric_fog", 0.0, 100.0),
        ("ao", 0.0, 1.0),
        ("emission", 0.0, 1.0),
        ("roughness_scale", 0.5, 2.0),
        ("chan_strength", 0.0, 1.0),
        ("reflection_probe_strength", 0.0, 4096.0),
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
    assert!(
        !app.world()
            .resource::<ExteriorWorldspaceLodSettings>()
            .enabled
    );
    assert!(exec(&mut app, "setrender worldspace_lod 1").ok);
    assert!(
        app.world()
            .resource::<ExteriorWorldspaceLodSettings>()
            .enabled
    );
    assert!(exec(&mut app, "setrender worldspace_lod 0").ok);
    assert!(exec(&mut app, "setrender metallic 0").ok);
    assert!(
        !app.world()
            .resource::<MaterialClampSettings>()
            .metallic_enabled()
    );
    assert_eq!(exec(&mut app, "getrender metallic").value["value"], 0);
    assert!(exec(&mut app, "setrender metallic 1").ok);
    assert!(
        app.world()
            .resource::<MaterialClampSettings>()
            .metallic_enabled()
    );
    assert!(exec(&mut app, "setrender dielectric_specular 0").ok);
    assert!(
        !app.world()
            .resource::<super::super::controls::MaterialClampSettings>()
            .dielectric_enabled()
    );
    assert_eq!(
        exec(&mut app, "getrender dielectric_specular").value["value"],
        0
    );
    assert!(exec(&mut app, "setrender dielectric_specular 1").ok);
    assert!(
        app.world()
            .resource::<super::super::controls::MaterialClampSettings>()
            .dielectric_enabled()
    );
    assert!(exec(&mut app, "setrender roughness_scale 1.75").ok);
    assert_eq!(
        app.world()
            .resource::<super::super::controls::MaterialClampSettings>()
            .roughness_scale(),
        1.75
    );
    assert_eq!(
        exec(&mut app, "getrender roughness_scale").value["value"],
        1.75
    );
    assert!(exec(&mut app, "setrender chan_strength 0.5").ok);
    assert_eq!(app.world().resource::<LegacyChanSettings>().strength(), 0.5);
    assert_eq!(
        exec(&mut app, "getrender chan_strength").value["value"],
        0.5
    );
    assert!(exec(&mut app, "setrender reflection_probe_strength 2.5").ok);
    assert_eq!(
        app.world()
            .resource::<super::super::controls::ReflectionProbeSettings>()
            .strength(),
        2.5
    );
    assert_eq!(
        exec(&mut app, "getrender reflection_probe_strength").value["value"],
        2.5
    );
    assert!(exec(&mut app, "setrender overlay_shadows 1").ok);
    assert!(exec(&mut app, "setrender overlay_reflections 1").ok);
    assert_eq!(
        exec(&mut app, "getrender overlay_shadows").value["value"],
        1
    );
    assert_eq!(
        exec(&mut app, "getrender overlay_reflections").value["value"],
        1
    );
    assert_eq!(
        exec(&mut app, "getrender overlay_lightmaps").value["value"],
        0
    );
    assert_eq!(
        exec(&mut app, "setrender overlay_lightmaps 1")
            .error
            .unwrap()
            .code,
        "requires_rebuild"
    );
    assert!(exec(&mut app, "setrender reflection_probes 0").ok);
    assert!(
        !app.world()
            .resource::<super::super::controls::ReflectionProbeSettings>()
            .enabled()
    );
    assert_eq!(
        app.world()
            .resource::<super::super::controls::ReflectionProbeSettings>()
            .strength(),
        2.5
    );
    assert!(exec(&mut app, "setrender reflection_probes 1").ok);
    assert!(
        app.world()
            .resource::<super::super::controls::ReflectionProbeSettings>()
            .enabled()
    );
    assert!(exec(&mut app, "setrender bloom_threshold 5000").ok);
    assert_eq!(
        app.world().resource::<ImageSpaceBloomOverrides>().threshold,
        Some(5000.0)
    );
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
    assert_eq!(
        exec(&mut app, "setrender dielectric_specular 0.5")
            .error
            .unwrap()
            .code,
        "out_of_range"
    );
    assert_eq!(
        exec(&mut app, "setrender roughness_scale 2.01")
            .error
            .unwrap()
            .code,
        "out_of_range"
    );
    assert_eq!(
        exec(&mut app, "setrender chan_strength 1.01")
            .error
            .unwrap()
            .code,
        "out_of_range"
    );
    assert_eq!(
        exec(&mut app, "setrender reflection_probe_strength 4096.01")
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
        23
    );
    assert!(exec(&mut app, "setrender day_night_preview 1").ok);
    assert_eq!(
        exec(&mut app, "getrender day_night_preview").value["value"],
        1
    );
}

#[test]
fn unchanged_chan_strength_does_not_trigger_material_propagation() {
    let mut app = test_app();
    app.world_mut().clear_trackers();
    assert!(exec(&mut app, "setrender chan_strength 1").ok);
    assert!(
        !app.world()
            .resource_ref::<LegacyChanSettings>()
            .is_changed()
    );

    assert!(exec(&mut app, "setrender chan_strength 0.5").ok);
    assert!(
        app.world()
            .resource_ref::<LegacyChanSettings>()
            .is_changed()
    );
}

#[test]
fn getrender_imagespace_reports_the_active_record_and_bloom_override() {
    let mut app = test_app();
    let mut manifest = fixture_manifest();
    manifest.cell.interior = true;
    manifest.cell.image_space = Some(Default::default());
    let image_space = manifest.cell.image_space.as_mut().unwrap();
    image_space.form_id = 0x1234;
    image_space.editor_id = Some("TestImageSpace".into());
    image_space.flags = 0;
    image_space.hdr_target_lum = 1.25;
    app.world_mut()
        .insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    assert!(exec(&mut app, "setrender bloom_threshold 0.7").ok);
    let output = exec(&mut app, "getrender imagespace");
    assert!(output.ok, "getrender imagespace failed: {:?}", output.error);
    assert_eq!(output.value["form_id"], 0x1234);
    assert_eq!(output.value["editor_id"], "TestImageSpace");
    assert_eq!(output.value["flags"], 0);
    assert_eq!(output.value["resolved"], true);
    assert_eq!(output.value["hdr_target_lum"], 1.25);
    assert!(
        (output.value["bloom_overrides"]["threshold"]
            .as_f64()
            .expect("numeric bloom override")
            - 0.7)
            .abs()
            < 0.00001
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
        facegen: None,
        facegen_reconstruction_fingerprint: None,
        facegen_diagnostics: Vec::new(),
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
    assert_eq!(output.value["facegen"]["status"], "RestPoseFallback");
    assert_eq!(
        output.value["facegen"]["geometry_status"],
        "RestPoseFallback"
    );
    assert_eq!(
        output.value["facegen"]["texture_status"],
        "RestPoseFallback"
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
    assert_eq!(output.value["animation"]["present"], false);
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
fn actorstate_mutations_join_definition_save_and_canonical_holder() {
    use bevyout_core::actor_state::{
        ActorDefinition, ActorFactionMembership, ActorInstanceState, ActorLifeState, ActorValue,
    };

    let mut app = test_app();
    let item_id = register_actor_placement(&mut app);
    let entity = app
        .world_mut()
        .query_filtered::<Entity, With<actor::ActorRuntime>>()
        .single(app.world())
        .unwrap();
    let mut definition = ActorDefinition {
        base_form_id: 2,
        reference_form_id: 16,
        factions: vec![ActorFactionMembership {
            faction_form_id: 0x1f17b,
            rank: 2,
            title: Some("Raider".into()),
        }],
        ..Default::default()
    };
    definition.base_values.insert(ActorValue::Health, 100.0);
    app.world_mut()
        .entity_mut(entity)
        .insert(super::super::actor_state::ActorStateRuntime {
            cell_form_id: 0x17f37,
            definition: std::sync::Arc::new(definition),
            life_state: ActorLifeState::Alive,
        });
    app.world_mut()
        .resource_mut::<super::super::world::ActiveSaveState>()
        .0
        .cells
        .entry(0x17f37)
        .or_default()
        .actors
        .insert(16, ActorInstanceState::new(16, ActorLifeState::Alive));

    assert!(exec(&mut app, "setactorvalue 00000010 health -12").ok);
    assert!(exec(&mut app, "setactorlife 00000010 dead").ok);
    assert!(exec(&mut app, "setactorpackage 00000010 0002c6f1 3 4.5").ok);
    let output = exec(&mut app, "actorstate 00000010");

    assert!(output.ok, "actorstate failed: {:?}", output.error);
    assert_eq!(output.value["life_state"], "dead");
    assert_eq!(output.value["definition"]["base_form_id"], 2);
    assert_eq!(output.value["definition"]["factions"][0]["rank"], 2);
    assert_eq!(output.value["values"][0]["name"], "health");
    assert_eq!(output.value["values"][0]["runtime_mutation"], -12.0);
    assert_eq!(output.value["values"][0]["effective"], 88.0);
    assert_eq!(output.value["package"]["form_id"], 0x2c6f1);
    assert_eq!(output.value["canonical"]["equipped_instance_id"], item_id.0);
    assert!(output.log[0].contains("life=dead"));
}

#[test]
fn actorstate_rejects_non_finite_mutations_and_unknown_lifecycle() {
    let mut app = test_app();
    register_actor_placement(&mut app);
    app.world_mut()
        .resource_mut::<super::super::world::ActiveSaveState>()
        .0
        .cells
        .entry(0x17f37)
        .or_default()
        .actors
        .insert(
            16,
            bevyout_core::actor_state::ActorInstanceState::new(
                16,
                bevyout_core::actor_state::ActorLifeState::Alive,
            ),
        );

    assert_eq!(
        error_code(&exec(&mut app, "setactorvalue 00000010 health NaN")),
        "bad_value"
    );
    assert_eq!(
        error_code(&exec(&mut app, "setactorlife 00000010 sleeping")),
        "bad_value"
    );
}

#[test]
fn actoranim_uses_the_gameplay_request_component() {
    let mut app = test_app();
    register_actor_placement(&mut app);
    let entity = app
        .world_mut()
        .query_filtered::<Entity, With<actor::ActorRuntime>>()
        .single(app.world())
        .unwrap();
    app.world_mut()
        .entity_mut(entity)
        .insert(actor_animation::ActorAnimationIntent::default());

    let output = exec(&mut app, "actoranim 00000010 run");
    assert!(output.ok, "actoranim failed: {:?}", output.error);
    assert_eq!(output.value["reference_form_id"], 16);
    assert_eq!(output.value["requested_state"], "run");
    assert_eq!(output.log, ["actoranim 00000010 run"]);
    assert_eq!(
        app.world()
            .get::<actor_animation::ActorAnimationIntent>(entity)
            .and_then(|intent| intent.requested),
        Some(actor_animation::policy::ActorAnimationState::Run)
    );
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
fn activate_rejects_a_non_door_reference() {
    let mut app = test_app();
    register_placement(&mut app, "Static");
    assert_eq!(
        error_code(&exec(&mut app, "activate 00000010")),
        "not_a_door"
    );
}

/// Issue #177: an ordinary in-cell door (no travel destination) toggles open
/// and closed instead of failing with `no_destination`. Before this it had no
/// open mechanism anywhere in the runtime.
#[test]
fn activate_toggles_a_destination_less_door_open_and_closed() {
    let mut app = test_app();
    app.add_message::<super::super::audio::PlaySound>();
    app.add_message::<super::super::animation::PlayPlacementAnimation>();
    register_placement(
        &mut app,
        "Door((lock_level: None, key_form_id: None, destination: None))",
    );
    let output = exec(&mut app, "activate TestRef");
    assert!(output.ok, "activate failed: {:?}", output.error);
    assert_eq!(output.value["opened"], true);
    let output = exec(&mut app, "activate TestRef");
    assert!(output.ok, "activate failed: {:?}", output.error);
    assert_eq!(output.value["opened"], false);
}

/// Issue #186: `activate` drives a solid activator blocker (vault gear door)
/// open and closed, the same human-testable parity #177 gave in-cell doors.
/// Before this, `activate` hard-rejected any non door/container/corpse/pickup
/// reference with `not_a_door`, so a gear door could only be opened by the
/// player and nav could never be told it had opened.
#[test]
fn activate_toggles_an_activator_blocker_open_and_closed() {
    let mut app = test_app();
    app.add_message::<super::super::audio::PlaySound>();
    app.add_message::<super::super::animation::PlayPlacementAnimation>();
    register_placement(&mut app, "Activator");
    let output = exec(&mut app, "activate TestRef");
    assert!(output.ok, "activate failed: {:?}", output.error);
    assert_eq!(output.value["kind"], "activator");
    assert_eq!(output.value["opened"], true);
    let output = exec(&mut app, "activate TestRef");
    assert!(output.ok, "activate failed: {:?}", output.error);
    assert_eq!(output.value["opened"], false);
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

// -- setlock (issue #163: GECK lock/unlock console parity) -------------

#[test]
fn setlock_requires_reference_and_level() {
    let mut app = test_app();
    assert_eq!(error_code(&exec(&mut app, "setlock")), "bad_arity");
    assert_eq!(error_code(&exec(&mut app, "setlock 00000010")), "bad_arity");
    // Issue #185: a third argument is now a valid (optional) key FormID, so
    // `bad_arity` only fires past four total arguments.
    assert_eq!(
        error_code(&exec(&mut app, "setlock 00000010 5 6 7")),
        "bad_arity"
    );
}

#[test]
fn setlock_third_argument_sets_or_clears_the_key_form_id() {
    let mut app = test_app();
    register_placement(&mut app, DOOR_WITH_DESTINATION);
    let output = exec(&mut app, "setlock 00000010 25 6");
    assert!(output.ok, "{output:?}");
    assert_eq!(output.value["key_form_id"], serde_json::json!(6));
    let output = exec(&mut app, "setlock 00000010 25 none");
    assert!(output.ok, "{output:?}");
    assert_eq!(output.value["key_form_id"], serde_json::json!(null));
}

#[test]
fn setlock_rejects_a_bad_key_form_id() {
    let mut app = test_app();
    register_placement(&mut app, DOOR_WITH_DESTINATION);
    assert_eq!(
        error_code(&exec(&mut app, "setlock 00000010 25 not-hex")),
        "bad_type"
    );
}

#[test]
fn setlock_rejects_unknown_and_non_door_references() {
    let mut app = test_app();
    assert_eq!(
        error_code(&exec(&mut app, "setlock 00000010 5")),
        "reference_not_found"
    );

    register_placement(&mut app, "Static");
    assert_eq!(
        error_code(&exec(&mut app, "setlock 00000010 5")),
        "not_a_door"
    );
}

#[test]
fn setlock_rejects_non_integer_and_out_of_range_levels() {
    let mut app = test_app();
    register_placement(&mut app, DOOR_WITH_DESTINATION);
    assert_eq!(
        error_code(&exec(&mut app, "setlock 00000010 abc")),
        "bad_type"
    );
    assert_eq!(
        error_code(&exec(&mut app, "setlock 00000010 -1")),
        "bad_type"
    );
    assert_eq!(
        error_code(&exec(&mut app, "setlock 00000010 128")),
        "bad_type"
    );
}

/// Issue #163: setting a level updates both consumers' state from the one
/// command -- the interaction-side `PlacementRoot` component the player's
/// own E-activation reads, and the nav-side `door_lock_info` snapshot
/// `door_availability_system` polls -- and level 0 clears both, preserving
/// the door's key requirement rather than discarding it.
#[test]
fn setlock_sets_and_clears_the_interaction_and_nav_lock_state_together() {
    let mut app = test_app();
    nav::agent::init_test_archipelago_state(app.world_mut());
    register_placement(
        &mut app,
        "Door((lock_level: None, key_form_id: Some(200), destination: None))",
    );

    let output = exec(&mut app, "setlock 00000010 50");
    assert!(output.ok, "setlock failed: {:?}", output.error);
    assert_eq!(output.value["reference_form_id"], 16);
    assert_eq!(output.value["lock_level"], 50);
    assert_eq!(output.log, vec!["setlock 00000010 level 50"]);
    assert_eq!(
        nav::agent::door_lock_level_for_test(app.world(), 0x10),
        Some(50)
    );

    let entity = app
        .world_mut()
        .query_filtered::<Entity, With<interaction::PlacementRoot>>()
        .single(app.world())
        .unwrap();
    let door_state = |app: &mut App| {
        let placement = app
            .world()
            .get::<interaction::PlacementRoot>(entity)
            .unwrap()
            .placement()
            .clone();
        match placement.semantic {
            crate::vsa::PreparedSemantic::Door(door) => door,
            _ => panic!("expected a door placement"),
        }
    };
    let door = door_state(&mut app);
    assert_eq!(door.lock_level, Some(50));
    assert_eq!(door.key_form_id, Some(200), "the key requirement is kept");

    let output = exec(&mut app, "setlock 00000010 0");
    assert!(output.ok, "setlock failed: {:?}", output.error);
    assert_eq!(output.value["lock_level"], Value::Null);
    assert_eq!(output.log, vec!["setlock 00000010 unlocked"]);
    assert_eq!(
        nav::agent::door_lock_level_for_test(app.world(), 0x10),
        None
    );
    let door = door_state(&mut app);
    assert_eq!(door.lock_level, None);
    assert_eq!(door.key_form_id, Some(200));
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
                animation_type: None,
                first_person_model_object_form_id: None,
                first_person_asset_path: None,
                fire_sound_3d_form_id: None,
                fire_sound_2d_form_id: None,
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

// -- tdi (issue #151) --------------------------------------------------

#[test]
fn tdi_toggles_debug_info_state_and_defaults_off() {
    let mut app = test_app();
    assert!(
        !app.world()
            .resource::<diagnostics::DebugInfoState>()
            .enabled
    );
    let output = exec(&mut app, "tdi");
    assert!(output.ok, "tdi failed: {:?}", output.error);
    assert_eq!(output.log, ["Debug info enabled."]);
    assert!(
        app.world()
            .resource::<diagnostics::DebugInfoState>()
            .enabled
    );
    let output = exec(&mut app, "tdi");
    assert!(output.ok, "tdi failed: {:?}", output.error);
    assert_eq!(output.log, ["Debug info disabled."]);
    assert!(
        !app.world()
            .resource::<diagnostics::DebugInfoState>()
            .enabled
    );
}

#[test]
fn tdi_rejects_arguments() {
    let mut app = test_app();
    assert_eq!(error_code(&exec(&mut app, "tdi now")), "bad_arity");
}

// -- tp (issue #152) -----------------------------------------------------

fn fixture_manifest() -> PreparedSceneManifest {
    ron::de::from_str(include_str!("../../../features/fixtures/scene.ron"))
        .expect("synthetic scene fixture should parse")
}

#[test]
fn tp_rejects_bad_arity() {
    let mut app = test_app();
    assert_eq!(error_code(&exec(&mut app, "tp 1 2")), "bad_arity");
    assert_eq!(error_code(&exec(&mut app, "tp 1 2 3 4 5")), "bad_arity");
}

#[test]
fn tp_rejects_non_finite_coordinates() {
    let mut app = test_app();
    assert_eq!(error_code(&exec(&mut app, "tp nope 2 3")), "bad_type");
    assert_eq!(error_code(&exec(&mut app, "tp 1 NaN 3")), "bad_type");
}

#[test]
fn tp_rejects_a_bad_cell_form_id() {
    let mut app = test_app();
    assert_eq!(error_code(&exec(&mut app, "tp 1 2 3 zzzz")), "bad_type");
}

// T152.1: the 3-arg form sets all three axes in a single write (never a
// partial, one-axis-at-a-time state) and drives the same
// `console_transform_mutated` reset `[player.]setpos` triggers.
#[test]
fn tp_with_three_args_atomically_repositions_the_player() {
    let mut app = test_app();
    let player = app
        .world_mut()
        .query_filtered::<Entity, With<player::FpsPlayer>>()
        .single(app.world())
        .unwrap();
    let output = exec(&mut app, "tp 4 5 6");
    assert!(output.ok, "tp failed: {:?}", output.error);
    assert_eq!(output.value["x"], 4.0);
    assert_eq!(output.value["y"], 5.0);
    assert_eq!(output.value["z"], 6.0);
    assert_eq!(
        output.log,
        ["tp: teleported player to (4.000, 5.000, 6.000)."]
    );
    let transform = app.world().get::<Transform>(player).unwrap();
    assert_eq!(transform.translation, Vec3::new(4.0, 5.0, 6.0));
}

#[test]
fn tp_with_a_destination_equal_to_the_active_cell_skips_the_swap() {
    let mut app = test_app();
    app.add_message::<interaction::DoorTravelRequested>();
    let manifest = fixture_manifest();
    let active_cell = manifest.cell.form_id;
    app.world_mut()
        .insert_resource(super::super::world::ActiveCell(active_cell));
    app.world_mut()
        .insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    let output = exec(&mut app, &format!("tp 7 8 9 {active_cell:08x}"));
    assert!(output.ok, "tp failed: {:?}", output.error);
    let requests = app
        .world()
        .resource::<Messages<interaction::DoorTravelRequested>>();
    assert_eq!(
        requests.iter_current_update_messages().count(),
        0,
        "a same-cell tp must not request a swap"
    );
}

#[test]
fn tp_to_an_unprepared_cell_fails_deterministically() {
    let mut app = test_app();
    app.add_message::<interaction::DoorTravelRequested>();
    let manifest = fixture_manifest();
    app.world_mut()
        .insert_resource(super::super::world::ActiveCell(manifest.cell.form_id));
    app.world_mut()
        .insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    // The fixture's asset_root ("cache/00017f37") exists nowhere, so
    // any destination other than the active cell is unprepared.
    assert_eq!(
        error_code(&exec(&mut app, "tp 1 2 3 000badd0")),
        "cell_not_found"
    );
    let requests = app
        .world()
        .resource::<Messages<interaction::DoorTravelRequested>>();
    assert_eq!(requests.iter_current_update_messages().count(), 0);
}

#[test]
fn tp_to_a_prepared_different_cell_writes_a_travel_request_at_the_given_position() {
    let mut app = test_app();
    app.add_message::<interaction::DoorTravelRequested>();
    let mut manifest = fixture_manifest();
    let source_cell = manifest.cell.form_id;
    let destination_cell = 0x0002_0002u32;
    let temp_root = std::env::temp_dir().join(format!(
        "bevyout-console-tp-test-{}-{destination_cell:08x}",
        std::process::id()
    ));
    let scene_dir = temp_root
        .join("scenes")
        .join(format!("{destination_cell:08x}"));
    std::fs::create_dir_all(&scene_dir).expect("create synthetic prepared-cell fixture dir");
    std::fs::write(scene_dir.join("scene.ron"), "()")
        .expect("write synthetic prepared-cell fixture file");
    manifest.asset_root = temp_root.to_string_lossy().into_owned();
    app.world_mut()
        .insert_resource(super::super::world::ActiveCell(source_cell));
    app.world_mut()
        .insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    let output = exec(&mut app, "tp 4 5 6 00020002");
    let cleanup = std::fs::remove_dir_all(&temp_root);
    assert!(output.ok, "tp failed: {:?}", output.error);
    assert_eq!(output.value["cell_form_id"], destination_cell);
    let requests = app
        .world()
        .resource::<Messages<interaction::DoorTravelRequested>>();
    let request = requests
        .iter_current_update_messages()
        .next()
        .expect("expected a DoorTravelRequested message");
    assert_eq!(request.destination_cell_form_id, destination_cell);
    assert_eq!(request.translation, Vec3::new(4.0, 5.0, 6.0));
    assert_eq!(request.door_form_id, 0);
    cleanup.expect("remove synthetic prepared-cell fixture dir");
}

// ---------------------------------------------------------------------
// showpackages (issue #176): console-harness tests over a synthetic
// per-cell actor catalog + content-set-wide package catalog written to a
// temp asset root, the same fixture pattern as the `tp`-to-a-prepared-cell
// tests above.
// ---------------------------------------------------------------------

fn write_showpackages_fixture(
    test_name: &str,
    reference_form_id: u32,
    base_form_id: u32,
    package_form_ids: Vec<u32>,
    packages: Vec<crate::vsa::PreparedPackageEntry>,
) -> (PreparedSceneManifest, std::path::PathBuf) {
    let mut manifest = fixture_manifest();
    let cell = manifest.cell.form_id;
    let fingerprint = format!("showpackages-fp-{test_name}");
    let temp_root = std::env::temp_dir().join(format!(
        "bevyout-console-{test_name}-{}-{reference_form_id:08x}",
        std::process::id()
    ));
    let scene_dir = temp_root.join("scenes").join(format!("{cell:08x}"));
    std::fs::create_dir_all(&scene_dir).expect("create synthetic prepared-cell fixture dir");

    let actor_catalog = crate::vsa::PreparedActorCatalog {
        revision: crate::vsa::ACTOR_CATALOG_REVISION.into(),
        source_fingerprint: fingerprint.clone(),
        entries: vec![crate::vsa::ActorCatalogEntry::Prepared(Box::new(
            crate::vsa::ActorBlueprint {
                base_form_id,
                reference_form_id,
                record_kind: "NPC_".into(),
                package_form_ids,
                ..crate::vsa::ActorBlueprint::default()
            },
        ))],
        counters: Default::default(),
        faction_table: Default::default(),
    };
    std::fs::write(
        scene_dir.join("actors.ron"),
        ron::ser::to_string_pretty(&actor_catalog, ron::ser::PrettyConfig::default())
            .expect("serialize synthetic actor catalog"),
    )
    .expect("write synthetic actor catalog fixture");
    manifest.actor_catalog_path = Some(format!("scenes/{cell:08x}/actors.ron"));

    let catalog_dir = temp_root.join("catalogs").join(&fingerprint);
    std::fs::create_dir_all(&catalog_dir).expect("create synthetic package catalog dir");
    let package_catalog = crate::vsa::PreparedPackageCatalog {
        revision: crate::vsa::PACKAGE_CATALOG_REVISION.into(),
        source_fingerprint: fingerprint.clone(),
        packages,
        counters: Default::default(),
    };
    std::fs::write(
        catalog_dir.join("packages.ron"),
        ron::ser::to_string_pretty(&package_catalog, ron::ser::PrettyConfig::default())
            .expect("serialize synthetic package catalog"),
    )
    .expect("write synthetic package catalog fixture");

    manifest.source_fingerprint = fingerprint;
    manifest.asset_root = temp_root.to_string_lossy().into_owned();
    (manifest, temp_root)
}

#[test]
fn showpackages_reports_a_known_actors_packages_in_priority_order() {
    let (manifest, temp_root) = write_showpackages_fixture(
        "known-actor",
        16,
        1,
        vec![0x50, 0x60],
        vec![
            crate::vsa::PreparedPackageEntry {
                form_id: 0x50,
                editor_id: Some("PKWander".into()),
                package_type: 5,
                conditions: vec![vec![0; 4], vec![1; 4]],
                ..Default::default()
            },
            crate::vsa::PreparedPackageEntry {
                form_id: 0x60,
                editor_id: Some("PKSleep".into()),
                package_type: 4,
                ..Default::default()
            },
        ],
    );
    let mut app = test_app();
    register_placement(
        &mut app,
        "Npc((base_template_form_id: None, assembly: None))",
    );
    app.world_mut()
        .insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    let output = exec(&mut app, "showpackages 00000010");
    let cleanup = std::fs::remove_dir_all(&temp_root);
    assert!(output.ok, "showpackages failed: {:?}", output.error);
    assert_eq!(output.value["actor_reference_form_id"], 16);
    assert_eq!(output.value["actor_base_form_id"], 1);
    let packages = output.value["packages"].as_array().unwrap();
    assert_eq!(packages.len(), 2);
    assert_eq!(packages[0]["form_id"], 0x50);
    assert_eq!(packages[0]["editor_id"], "PKWander");
    assert_eq!(packages[0]["package_type_label"], "Wander");
    assert_eq!(packages[0]["condition_count"], 2);
    assert_eq!(packages[1]["form_id"], 0x60);
    assert_eq!(packages[1]["editor_id"], "PKSleep");
    assert!(output.log[0].contains("2 package(s) in priority order"));
    assert!(output.log[1].contains("#1/2 00000050"));
    assert!(output.log[1].contains("\"PKWander\""));
    assert!(output.log[2].contains("#2/2 00000060"));
    cleanup.expect("remove synthetic showpackages fixture dir");
}

#[test]
fn showpackages_reports_a_clear_line_for_an_actor_without_packages() {
    let (manifest, temp_root) =
        write_showpackages_fixture("no-packages", 16, 1, Vec::new(), Vec::new());
    let mut app = test_app();
    register_placement(
        &mut app,
        "Npc((base_template_form_id: None, assembly: None))",
    );
    app.world_mut()
        .insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    let output = exec(&mut app, "showpackages 00000010");
    let cleanup = std::fs::remove_dir_all(&temp_root);
    assert!(output.ok, "showpackages failed: {:?}", output.error);
    assert_eq!(output.value["packages"].as_array().unwrap().len(), 0);
    assert!(output.log[0].contains("has no packages"));
    cleanup.expect("remove synthetic showpackages fixture dir");
}

#[test]
fn showpackages_resolves_a_raw_base_formid_not_currently_placed() {
    let (manifest, temp_root) = write_showpackages_fixture(
        "base-formid",
        16,
        0x2A,
        vec![0x50],
        vec![crate::vsa::PreparedPackageEntry {
            form_id: 0x50,
            package_type: 0,
            ..Default::default()
        }],
    );
    let mut app = test_app();
    // No live placement is registered: `2A` is only known through the
    // actor catalog, proving the base-FormID half of
    // "actor-reference-or-base-formid" works without a spawned entity.
    app.world_mut()
        .insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    let output = exec(&mut app, "showpackages 0000002A");
    let cleanup = std::fs::remove_dir_all(&temp_root);
    assert!(output.ok, "showpackages failed: {:?}", output.error);
    assert_eq!(output.value["actor_base_form_id"], 0x2A);
    assert_eq!(output.value["packages"].as_array().unwrap().len(), 1);
    cleanup.expect("remove synthetic showpackages fixture dir");
}

/// A minimal `PreparedPlacement`, matching `register_placement`'s RON
/// shape but with a caller-chosen FormID/base/translation/asset/linked
/// reference -- used to build both a spawned actor and an unspawned
/// asset-less marker for the #213 context-builder tests below.
fn synthetic_placement(
    reference_form_id: u32,
    base_form_id: u32,
    translation: [f32; 3],
    asset_path: Option<&str>,
    linked_reference_form_id: Option<u32>,
) -> crate::vsa::PreparedPlacement {
    let ron = format!(
        "(
            reference_form_id: {reference_form_id},
            base_form_id: {base_form_id},
            asset_path: {asset_path_ron},
            translation: ({}, {}, {}),
            rotation_xyzw: (0.0, 0.0, 0.0, 1.0),
            scale: 1.0,
            error: None,
            semantic: Static,
            linked_reference_form_id: {linked_ron},
        )",
        translation[0],
        translation[1],
        translation[2],
        asset_path_ron = asset_path.map_or_else(|| "None".to_string(), |p| format!("Some({p:?})")),
        linked_ron =
            linked_reference_form_id.map_or_else(|| "None".to_string(), |id| format!("Some({id})")),
    );
    ron::de::from_str(&ron).expect("synthetic placement RON should parse")
}

/// A minimal `PreparedPackageEntry` carrying just a `PLDT` location --
/// `PackageLocationInput` is `pub(crate)` to `vsa::prepare`, not reachable
/// by name from this module, so this goes through RON like
/// `synthetic_placement` above.
fn package_with_location(
    form_id: u32,
    package_type: u8,
    location_type: u32,
) -> crate::vsa::PreparedPackageEntry {
    let ron = format!(
        "(
            form_id: {form_id},
            package_type: {package_type},
            location: Some((
                location_type: {location_type},
                form_id: None,
                raw_value: 0,
                radius: 512,
            )),
        )"
    );
    ron::de::from_str(&ron).expect("synthetic package entry RON should parse")
}

// Issue #213: `build_resolution_context` folds the manifest's full
// placement list in (not just spawned entities), so an asset-less patrol
// marker -- skipped at spawn, never a `PlacementRoot` entity -- is still a
// resolvable reference, and the querying actor's *authored* editor location
// and linked reference come from its manifest placement rather than its
// live (possibly since-moved) `Transform`.
#[test]
fn showpackages_resolves_near_editor_location_from_the_actors_authored_placement() {
    let (mut manifest, temp_root) = write_showpackages_fixture(
        "editor-location",
        0x10,
        1,
        vec![0x50],
        vec![package_with_location(0x50, 12, 3)], // Sandbox, NearEditorLocation
    );
    // The actor's *authored* point (2, 3, 4) differs from where it will
    // actually spawn (0, 0, 0) below -- proving editor location comes from
    // the manifest placement, not the live Transform.
    manifest.placements = vec![synthetic_placement(
        0x10,
        1,
        [2.0, 3.0, 4.0],
        Some("meshes/npc.glb"),
        None,
    )];
    let mut app = test_app();
    register_placement(
        &mut app,
        "Npc((base_template_form_id: None, assembly: None))",
    );
    app.world_mut()
        .insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    let output = exec(&mut app, "showpackages 00000010");
    let cleanup = std::fs::remove_dir_all(&temp_root);
    assert!(output.ok, "showpackages failed: {:?}", output.error);
    let location = &output.value["resolution"]["location"];
    assert_eq!(location["resolved"], true);
    assert_eq!(location["source"], "editor-location");
    assert_eq!(location["position"], serde_json::json!([2.0, 3.0, 4.0]));
    cleanup.expect("remove synthetic showpackages fixture dir");
}

#[test]
fn showpackages_resolves_near_linked_reference_from_an_unspawned_manifest_marker() {
    let (mut manifest, temp_root) = write_showpackages_fixture(
        "linked-reference",
        0x10,
        1,
        vec![0x50],
        vec![package_with_location(0x50, 13, 6)], // Patrol, NearLinkedReference
    );
    manifest.placements = vec![
        synthetic_placement(0x10, 1, [0.0, 0.0, 0.0], Some("meshes/npc.glb"), Some(0x99)),
        // The marker is asset-less (no GLB) and deliberately never spawned
        // below -- `scene::spawn_cell_content` would skip it too -- so it
        // is only reachable through the manifest fold-in.
        synthetic_placement(0x99, 0x34, [9.0, 8.0, 7.0], None, None),
    ];
    let mut app = test_app();
    register_placement(
        &mut app,
        "Npc((base_template_form_id: None, assembly: None))",
    );
    app.world_mut()
        .insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    let output = exec(&mut app, "showpackages 00000010");
    let cleanup = std::fs::remove_dir_all(&temp_root);
    assert!(output.ok, "showpackages failed: {:?}", output.error);
    let location = &output.value["resolution"]["location"];
    assert_eq!(location["resolved"], true);
    assert_eq!(location["source"], "linked-reference");
    assert_eq!(location["position"], serde_json::json!([9.0, 8.0, 7.0]));
    cleanup.expect("remove synthetic showpackages fixture dir");
}

#[test]
fn showpackages_rejects_an_unknown_formid_deterministically() {
    let (manifest, temp_root) =
        write_showpackages_fixture("unknown-formid", 16, 1, Vec::new(), Vec::new());
    let mut app = test_app();
    app.world_mut()
        .insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    let output = exec(&mut app, "showpackages 0000dead");
    let cleanup = std::fs::remove_dir_all(&temp_root);
    assert_eq!(error_code(&output), "unknown_actor");
    cleanup.expect("remove synthetic showpackages fixture dir");
}

#[test]
fn showpackages_rejects_bad_arity_and_non_actor_references() {
    let mut app = test_app();
    assert_eq!(error_code(&exec(&mut app, "showpackages")), "bad_arity");
    assert_eq!(
        error_code(&exec(&mut app, "showpackages a b c")),
        "bad_arity"
    );
    // The optional second argument is a game-hour; a non-numeric one is a
    // type error, not arity.
    assert_eq!(
        error_code(&exec(&mut app, "showpackages a notanhour")),
        "bad_type"
    );
    register_placement(&mut app, "Static");
    assert_eq!(
        error_code(&exec(&mut app, "showpackages 00000010")),
        "not_actor"
    );
}

#[test]
fn showpackages_requires_a_loaded_cell_manifest() {
    let mut app = test_app();
    register_placement(
        &mut app,
        "Npc((base_template_form_id: None, assembly: None))",
    );
    assert_eq!(
        error_code(&exec(&mut app, "showpackages 00000010")),
        "cell_unavailable"
    );
}

#[test]
fn playidle_is_registered_and_rejects_unknown_actor_deterministically() {
    let mut app = test_app();
    let help = exec(&mut app, "help playidle");
    assert!(help.ok);
    assert_eq!(help.value["mutating"], true);
    assert_eq!(
        error_code(&exec(&mut app, "playidle 00000010 00000001")),
        "unknown_actor"
    );
}

// ---------------------------------------------------------------------
// M9 wave 1 (#310): player RPG stats console surface.
// ---------------------------------------------------------------------

#[test]
fn getav_reads_derived_health_and_sheet_values_with_and_without_prefix() {
    let mut app = test_app();
    let output = exec(&mut app, "player.getav health");
    assert!(output.ok, "player.getav health failed: {:?}", output.error);
    assert_eq!(output.value["result"].as_f64(), Some(200.0));
    let bare = exec(&mut app, "getav strength");
    assert!(bare.ok, "bare getav failed: {:?}", bare.error);
    assert_eq!(bare.value["result"].as_f64(), Some(5.0));
    let skill = exec(&mut app, "getav small_guns");
    assert!(skill.ok, "getav small_guns failed: {:?}", skill.error);
    assert_eq!(skill.value["result"].as_f64(), Some(15.0));
}

#[test]
fn getav_rejects_unknown_values_and_health_mutation_clamps() {
    let mut app = test_app();
    let output = exec(&mut app, "player.getav nosuchvalue");
    assert!(!output.ok);
    assert_eq!(error_code(&output), "unknown_actor_value");
    let health = exec(&mut app, "player.setav health 500");
    assert!(health.ok);
    assert_eq!(health.value["result"].as_f64(), Some(200.0));
}

#[test]
fn modav_and_setav_clamp_special_into_one_to_ten() {
    let mut app = test_app();
    let raised = exec(&mut app, "player.modav strength 10");
    assert!(raised.ok, "modav failed: {:?}", raised.error);
    assert_eq!(raised.value["result"], 10);
    let reduced = exec(&mut app, "player.modav strength -30");
    assert!(reduced.ok);
    assert_eq!(reduced.value["result"], 1);
    let set = exec(&mut app, "player.setav strength 8");
    assert!(set.ok);
    assert_eq!(set.value["result"], 8);
}

#[test]
fn rewardxp_crosses_the_level_threshold_and_updates_derived_health() {
    let mut app = test_app();
    let output = exec(&mut app, "player.rewardxp 200");
    assert!(output.ok, "rewardxp failed: {:?}", output.error);
    assert_eq!(output.value["level"], 2);
    assert_eq!(output.value["levels_gained"], 1);
    assert_eq!(output.value["skill_points_gained"], 15);
    // Raising the maximum does not grant free healing.
    let health = exec(&mut app, "player.getav health");
    assert_eq!(health.value["result"].as_f64(), Some(200.0));
    let set = exec(&mut app, "player.setav health 999");
    assert_eq!(set.value["result"].as_f64(), Some(210.0));
}

#[test]
fn advlevel_advances_one_level_and_respects_the_cap() {
    let mut app = test_app();
    let output = exec(&mut app, "player.advlevel");
    assert!(output.ok, "advlevel failed: {:?}", output.error);
    assert_eq!(output.value["level"], 2);
    assert_eq!(output.value["skill_points_gained"], 15);
    // Dump enough XP to hit the default cap, then expect rejection.
    let capped = exec(&mut app, "player.rewardxp 999999");
    assert!(capped.ok);
    assert_eq!(capped.value["level"], 30);
    let beyond = exec(&mut app, "player.advlevel");
    assert!(!beyond.ok);
    assert_eq!(error_code(&beyond), "at_level_cap");
}

// ---------------------------------------------------------------------
// M9 wave 2 (#314): player perk console surface.
// ---------------------------------------------------------------------

/// Synthetic catalog with Swift Learner (00031DD3) and Educated
/// (00031DD8) exactly as decoded from the real Fallout3.esm, plus one
/// non-playable perk that `showperks --eligible` must exclude.
fn perk_test_catalog() -> super::super::stats::PerkCatalog {
    use bevyout_core::actor_state::{ActorValue, SpecialAttribute};
    use bevyout_core::perks::{
        ENTRY_CODE_BONUS_SKILL_POINTS, ENTRY_CODE_XP_AWARD_MULTIPLIER, EntryPointPayload,
        PerkDefinition, PerkEntry,
    };
    use std::collections::BTreeMap;
    let swift = PerkDefinition {
        form_id: 0x0003_1dd3,
        editor_id: "SwiftLearner".into(),
        min_level: 2,
        ranks: 3,
        playable: true,
        conditions: vec![bevyout_core::perks::PerkCondition {
            actor_value: ActorValue::Special(SpecialAttribute::Intelligence),
            threshold: 4,
        }],
        entries: [1.1_f32, 1.2, 1.3]
            .into_iter()
            .enumerate()
            .map(|(rank, value)| PerkEntry::EntryPoint {
                rank: rank as u8,
                code: ENTRY_CODE_XP_AWARD_MULTIPLIER,
                param_count: 3,
                priority: 0,
                payload: EntryPointPayload::Value(value),
            })
            .collect(),
        ..PerkDefinition::default()
    };
    let educated = PerkDefinition {
        form_id: 0x0003_1dd8,
        editor_id: "Educated".into(),
        min_level: 4,
        ranks: 1,
        playable: true,
        conditions: vec![bevyout_core::perks::PerkCondition {
            actor_value: ActorValue::Special(SpecialAttribute::Intelligence),
            threshold: 4,
        }],
        entries: vec![PerkEntry::EntryPoint {
            rank: 0,
            code: ENTRY_CODE_BONUS_SKILL_POINTS,
            param_count: 2,
            priority: 0,
            payload: EntryPointPayload::Value(3.0),
        }],
        ..PerkDefinition::default()
    };
    let quest_only = PerkDefinition {
        form_id: 0x0006_1857,
        editor_id: "WellRestedPerk".into(),
        min_level: 1,
        ranks: 1,
        playable: false,
        ..PerkDefinition::default()
    };
    super::super::stats::PerkCatalog(
        [swift, educated, quest_only]
            .into_iter()
            .map(|perk| (perk.form_id, perk))
            .collect::<BTreeMap<_, _>>(),
    )
}

#[test]
fn addperk_enforces_eligibility_and_grants_ranks() {
    let mut app = test_app();
    app.insert_resource(perk_test_catalog());
    // Level 1 player vs the level-2 gate: blocked with the reason.
    let blocked = exec(&mut app, "player.addperk 00031dd3");
    assert!(!blocked.ok);
    assert_eq!(error_code(&blocked), "perk_ineligible");
    let message = blocked
        .error
        .as_ref()
        .map(|error| error.message.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("level"),
        "error text should name the gate: {message}"
    );
    // Level up to 2 (INT 5 passes the INT 4 condition) and grant rank 1.
    assert!(exec(&mut app, "player.advlevel").ok);
    let granted = exec(&mut app, "player.addperk 00031dd3");
    assert!(granted.ok, "addperk failed: {:?}", granted.error);
    assert_eq!(granted.value["rank"].as_i64(), Some(1));
    assert_eq!(granted.value["ranks"].as_i64(), Some(3));
    assert_eq!(
        granted.value["modifiers"]["xp_award_multiplier_bps"].as_i64(),
        Some(11_000)
    );
    // The owned rank's XP multiplier reaches the rewardxp path: 1000 XP
    // awards 1100.
    let award = exec(&mut app, "player.rewardxp 1000");
    assert!(award.ok);
    assert_eq!(award.value["xp_multiplier_bps"].as_i64(), Some(11_000));
    assert_eq!(award.value["xp"].as_i64(), Some(1_300));
    // The second rank upgrades 1 -> 2 and the multiplier follows.
    let second = exec(&mut app, "player.addperk 31dd3");
    assert!(second.ok, "short-hex addperk failed: {:?}", second.error);
    assert_eq!(second.value["rank"].as_i64(), Some(2));
    assert_eq!(
        second.value["modifiers"]["xp_award_multiplier_bps"].as_i64(),
        Some(12_000)
    );
    // Unknown perks and malformed form ids are rejected.
    assert_eq!(
        error_code(&exec(&mut app, "player.addperk deadbeef")),
        "unknown_perk"
    );
    assert_eq!(
        error_code(&exec(&mut app, "player.addperk nothex")),
        "bad_form_id"
    );
    assert_eq!(error_code(&exec(&mut app, "player.addperk")), "bad_arity");
}

#[test]
fn removeperk_and_hasperk_track_owned_ranks() {
    let mut app = test_app();
    app.insert_resource(perk_test_catalog());
    assert_eq!(
        error_code(&exec(&mut app, "player.removeperk 00031dd3")),
        "perk_not_owned"
    );
    assert!(exec(&mut app, "player.advlevel").ok);
    assert!(exec(&mut app, "player.addperk 00031dd3").ok);
    assert!(exec(&mut app, "player.addperk 00031dd3").ok);
    let owned = exec(&mut app, "player.hasperk 00031dd3");
    assert!(owned.ok);
    assert_eq!(owned.value["rank"].as_i64(), Some(2));
    assert_eq!(owned.value["ranks"].as_i64(), Some(3));
    let removed = exec(&mut app, "player.removeperk 00031dd3");
    assert!(removed.ok, "removeperk failed: {:?}", removed.error);
    assert_eq!(removed.value["rank"].as_i64(), Some(1));
    assert_eq!(
        removed.value["modifiers"]["xp_award_multiplier_bps"].as_i64(),
        Some(11_000)
    );
    let cleared = exec(&mut app, "player.removeperk 00031dd3");
    assert!(cleared.ok);
    assert_eq!(cleared.value["rank"].as_i64(), Some(0));
    assert_eq!(
        cleared.value["modifiers"]["xp_award_multiplier_bps"].as_i64(),
        Some(10_000),
        "the multiplier reverts to neutral"
    );
    assert_eq!(
        error_code(&exec(&mut app, "player.removeperk 00031dd3")),
        "perk_not_owned"
    );
    let none = exec(&mut app, "player.hasperk 00031dd3");
    assert!(none.ok);
    assert_eq!(none.value["rank"].as_i64(), Some(0));
}

#[test]
fn educated_grants_bonus_skill_points_on_level_up() {
    let mut app = test_app();
    app.insert_resource(perk_test_catalog());
    // Educated needs level 4; three level-ups reach it.
    for _ in 0..3 {
        assert!(exec(&mut app, "player.advlevel").ok);
    }
    let granted = exec(&mut app, "player.addperk 00031dd8");
    assert!(granted.ok, "addperk failed: {:?}", granted.error);
    assert_eq!(granted.value["rank"].as_i64(), Some(1));
    assert_eq!(
        granted.value["modifiers"]["bonus_skill_points"].as_i64(),
        Some(3)
    );
    // The next level-up grants 15 sheet points + 3 perk points.
    let level = exec(&mut app, "player.advlevel");
    assert!(level.ok);
    assert_eq!(level.value["skill_points_gained"].as_i64(), Some(18));
    assert_eq!(
        level.value["bonus_skill_points_per_level"].as_i64(),
        Some(3)
    );
}

#[test]
fn showperks_lists_owned_and_eligible_with_blocked_reasons() {
    let mut app = test_app();
    app.insert_resource(perk_test_catalog());
    assert!(exec(&mut app, "player.advlevel").ok);
    assert!(exec(&mut app, "player.addperk 00031dd3").ok);
    let owned = exec(&mut app, "showperks");
    assert!(owned.ok);
    assert_eq!(owned.value["count"].as_i64(), Some(1));
    assert_eq!(owned.value["perks"][0]["perk"], "SwiftLearner");
    assert_eq!(owned.value["perks"][0]["rank"].as_i64(), Some(1));
    assert_eq!(owned.value["perks"][0]["ranks"].as_i64(), Some(3));
    assert_eq!(
        owned.value["modifiers"]["xp_award_multiplier_bps"].as_i64(),
        Some(11_000)
    );
    // The eligible view covers every playable perk; the non-playable
    // quest perk stays out and blocked perks carry their reasons.
    let eligible = exec(&mut app, "showperks --eligible");
    assert!(eligible.ok);
    assert_eq!(eligible.value["count"].as_i64(), Some(2));
    let entries = eligible.value["eligible"].as_array().unwrap();
    let by_perk = |name: &str| {
        entries
            .iter()
            .find(|entry| entry["perk"] == name)
            .unwrap_or_else(|| panic!("expected {name} in the eligible list"))
    };
    let swift = by_perk("SwiftLearner");
    assert_eq!(swift["eligible"], true);
    assert_eq!(swift["reasons"].as_array().map(Vec::len), Some(0));
    let educated = by_perk("Educated");
    assert_eq!(educated["eligible"], false);
    assert_eq!(educated["reasons"][0]["kind"], "min_level");
    assert_eq!(educated["reasons"][0]["required"].as_i64(), Some(4));
}

// ---------------------------------------------------------------------
// M9 wave 3 (#318): chem/aid/radiation console surface.
// ---------------------------------------------------------------------

use crate::viewer::effects::EffectCatalog;

/// Synthetic effect catalog with the real-data ground-truth shapes:
/// Stimpak (instant heal 30, medicine), RadAway (instant -50 rads),
/// Buffout (STR +2 / END +3 for 240 s), and Jet (AP +30 for 240 s with a
/// 20% addiction chance against withdrawal 00033067). FormIDs match the
/// real GOTY Fallout3.esm records.
fn ingestible_test_catalog() -> EffectCatalog {
    use bevyout_core::actor_state::{ActorValue, SpecialAttribute};
    use bevyout_core::effects::{
        CONDITION_FUNCTION_HAS_PERK, CONDITION_OPER_EQUAL, IngestibleCondition,
        IngestibleDefinition, IngestibleEffect,
    };
    let instant = |mgef: u32, value: ActorValue, magnitude: f32| IngestibleEffect {
        mgef_form_id: mgef,
        magnitude,
        duration_s: 0,
        actor_value: Some(value),
        ..IngestibleEffect::default()
    };
    let timed = |mgef: u32, value: ActorValue, magnitude: f32| IngestibleEffect {
        mgef_form_id: mgef,
        magnitude,
        duration_s: 240,
        actor_value: Some(value),
        ..IngestibleEffect::default()
    };
    let stimpak_condition = |comparison_value| IngestibleCondition {
        oper: CONDITION_OPER_EQUAL,
        comparison_value,
        function: CONDITION_FUNCTION_HAS_PERK,
        param1: 0x0009_4ebf,
    };
    let stimpak = IngestibleDefinition {
        form_id: 0x0001_5169,
        editor_id: "Stimpak".into(),
        flags: 0x04,
        effects: vec![
            IngestibleEffect {
                condition: Some(stimpak_condition(0.0)),
                ..instant(0x48C7B, ActorValue::Health, 30.0)
            },
            IngestibleEffect {
                condition: Some(stimpak_condition(1.0)),
                ..instant(0x48C7B, ActorValue::Health, 36.0)
            },
        ],
        ..IngestibleDefinition::default()
    };
    let radaway = IngestibleDefinition {
        form_id: 0x0001_5167,
        editor_id: "RadAway".into(),
        // Real-data polarity: RestoreRadiationLevel carries +50 (removes
        // rads), verified against the GOTY ESM.
        effects: vec![instant(0x1517A, ActorValue::Rads, 50.0)],
        ..IngestibleDefinition::default()
    };
    let buffout = IngestibleDefinition {
        form_id: 0x0001_5163,
        editor_id: "Buffout".into(),
        weight: 0.1,
        effects: vec![
            timed(
                0x6697C,
                ActorValue::Special(SpecialAttribute::Strength),
                2.0,
            ),
            timed(
                0x6697D,
                ActorValue::Special(SpecialAttribute::Endurance),
                3.0,
            ),
        ],
        ..IngestibleDefinition::default()
    };
    let jet = IngestibleDefinition {
        form_id: 0x0001_5164,
        editor_id: "Jet".into(),
        weight: 0.2,
        withdrawal_form_id: 0x0003_3067,
        addiction_chance_percent: 20.0,
        effects: vec![timed(0x66EB8, ActorValue::ActionPoints, 30.0)],
        ..IngestibleDefinition::default()
    };
    let rad_x = IngestibleDefinition {
        form_id: 0x0001_5168,
        editor_id: "RadX".into(),
        effects: vec![timed(0x1517B, ActorValue::RadResist, 25.0)],
        ..IngestibleDefinition::default()
    };
    let irradiated_food = IngestibleDefinition {
        form_id: 0x000f_0001,
        editor_id: "IrradiatedFood".into(),
        effects: vec![instant(0x1517A, ActorValue::Rads, -100.0)],
        ..IngestibleDefinition::default()
    };
    EffectCatalog {
        ingestibles: [stimpak, radaway, buffout, jet, rad_x, irradiated_food]
            .into_iter()
            .map(|ingestible| (ingestible.form_id, ingestible))
            .collect(),
    }
}

#[test]
fn rads_reports_dose_and_threshold_penalties() {
    let mut app = test_app();
    let fresh = exec(&mut app, "player.rads");
    assert!(fresh.ok);
    assert_eq!(fresh.value["rads"].as_i64(), Some(0));
    assert_eq!(fresh.log, ["0 rads (no penalties)"]);
    let dosed = exec(&mut app, "addrads 600");
    assert!(dosed.ok);
    assert_eq!(dosed.value["absorbed_rads"].as_i64(), Some(600));
    let report = exec(&mut app, "rads");
    assert_eq!(report.value["rads"].as_i64(), Some(600));
    assert_eq!(report.value["threshold"].as_i64(), Some(600));
    let penalties = report.value["penalties"].as_str().unwrap();
    assert!(penalties.contains("endurance-3"), "{penalties}");
    assert!(penalties.contains("agility-2"), "{penalties}");
    assert!(penalties.contains("strength-1"), "{penalties}");
}

#[test]
fn addrads_clamps_at_the_fatal_cap_and_removerads_reverses() {
    let mut app = test_app();
    let fatal = exec(&mut app, "addrads 1500");
    assert_eq!(fatal.value["rads"].as_i64(), Some(1000));
    assert_eq!(fatal.value["fatal"], true);
    // RadAway semantics: removing more than held never goes below zero.
    let removed = exec(&mut app, "removerads 1200");
    assert_eq!(removed.value["removed_rads"].as_i64(), Some(1000));
    assert_eq!(removed.value["rads"].as_i64(), Some(0));
}

#[test]
fn addchem_applies_instant_heal_and_timed_modifiers_without_inventory() {
    let mut app = test_app();
    app.insert_resource(ingestible_test_catalog());
    let healed = exec(&mut app, "addchem 00015169");
    assert!(healed.ok, "addchem stimpak failed: {:?}", healed.error);
    assert_eq!(healed.value["application"]["editor_id"], "Stimpak");
    // Non-addictive items consume no PRNG draw and report no roll.
    assert_eq!(
        healed.value["application"]["rng_draw_index"].as_i64(),
        Some(0)
    );

    let buffed = exec(&mut app, "addchem 00015163");
    assert!(buffed.ok);
    assert_eq!(
        buffed.value["application"]["applied_modifiers"].as_i64(),
        Some(2)
    );
    // The ledger carries both timed modifiers; the base sheet is untouched.
    let list = exec(&mut app, "effects");
    assert_eq!(
        list.value["active_effects"].as_array().map(Vec::len),
        Some(2)
    );
    let strength = exec(&mut app, "getav strength");
    assert_eq!(strength.value["result"].as_f64(), Some(7.0));
    let action_points = exec(&mut app, "getav action_points");
    assert_eq!(action_points.value["result"].as_f64(), Some(75.0));
}

#[test]
fn projected_actor_values_and_rad_x_resistance_are_exposed() {
    let mut app = test_app();
    app.insert_resource(ingestible_test_catalog());
    assert_eq!(
        exec(&mut app, "getav action_points").value["result"].as_f64(),
        Some(75.0)
    );
    exec(&mut app, "addchem 00015164");
    assert_eq!(
        exec(&mut app, "getav action_points").value["result"].as_f64(),
        Some(105.0)
    );
    exec(&mut app, "addchem 00015168");
    assert_eq!(
        exec(&mut app, "getav rad_resist").value["result"].as_f64(),
        Some(25.0)
    );
    let dosed = exec(&mut app, "addrads 100");
    assert_eq!(dosed.value["absorbed_rads"].as_i64(), Some(75));
    let food = exec(&mut app, "addchem 000f0001");
    assert_eq!(food.value["application"]["rads_added"].as_i64(), Some(75));
    assert_eq!(exec(&mut app, "rads").value["rads"].as_i64(), Some(150));
}

#[test]
fn stimpak_selects_fast_metabolism_branch_and_health_console_clamps() {
    let mut app = test_app();
    app.insert_resource(ingestible_test_catalog());
    assert!(exec(&mut app, "setav health 140").ok);
    let base = exec(&mut app, "addchem 00015169");
    assert_eq!(base.value["application"]["healed_to"].as_f64(), Some(170.0));
    assert_eq!(
        base.value["application"]["condition_false"].as_i64(),
        Some(1)
    );
    assert_eq!(
        exec(&mut app, "getav health").value["result"].as_f64(),
        Some(170.0)
    );

    {
        let mut query = app
            .world_mut()
            .query::<&mut crate::viewer::stats::ActorPerks>();
        query
            .single_mut(app.world_mut())
            .unwrap()
            .0
            .set_rank(0x0009_4ebf, 1);
    }
    assert!(exec(&mut app, "setav health 140").ok);
    let perk = exec(&mut app, "addchem 00015169");
    assert_eq!(perk.value["application"]["healed_to"].as_f64(), Some(176.0));
    assert!(exec(&mut app, "setav health 999").ok);
    assert_eq!(
        exec(&mut app, "getav health").value["result"].as_f64(),
        Some(200.0)
    );
    assert!(exec(&mut app, "modav health -999").ok);
    assert_eq!(
        exec(&mut app, "getav health").value["result"].as_f64(),
        Some(0.0)
    );
}

#[test]
fn jet_rolls_addiction_against_the_seeded_rng_and_cure_clears_it() {
    let mut app = test_app();
    app.insert_resource(ingestible_test_catalog());
    // Seed 0's first draw is 7535 bps against Jet's effective 2000 bps:
    // no addiction, one draw consumed.
    let first = exec(&mut app, "addchem 00015164");
    assert!(first.ok);
    assert_eq!(first.value["application"]["addiction_roll"], false);
    assert_eq!(
        first.value["application"]["rng_draw_index"].as_i64(),
        Some(1)
    );
    assert_eq!(
        exec(&mut app, "effects").value["addictions"]
            .as_array()
            .map(Vec::len),
        Some(0)
    );

    // Force the machine into Addicted to prove cureaddiction clears it.
    {
        let mut query = app
            .world_mut()
            .query_filtered::<&mut crate::viewer::effects::Addictions, ()>();
        query
            .single_mut(app.world_mut())
            .unwrap()
            .0
            .addict(0x0003_3067);
    }
    let cured_all = exec(&mut app, "cureaddiction all");
    assert!(cured_all.ok);
    assert_eq!(cured_all.value["cured"].as_i64(), Some(1));
    assert_eq!(
        exec(&mut app, "effects").value["addictions"]
            .as_array()
            .map(Vec::len),
        Some(0)
    );

    // Curing a non-existent addiction errors.
    let missing = exec(&mut app, "cureaddiction 00033067");
    assert!(!missing.ok);
    assert_eq!(error_code(&missing), "not_addicted");
}

#[test]
fn useitem_applies_ingestible_effects_through_the_canonical_seam() {
    let mut app = test_app();
    app.insert_resource(ingestible_test_catalog());
    // Put a stimpak stack in the player inventory (canonical + projection).
    let stack = InventoryStack {
        base_form_id: 0x0001_5169,
        count: 2,
        condition: None,
    };
    let before = app
        .world()
        .resource::<crate::viewer::interaction::PlayerInventory>()
        .legacy_snapshot();
    app.world_mut()
        .resource_mut::<crate::viewer::interaction::CanonicalItemLedger>()
        .add_player_item(&before, stack)
        .unwrap();
    app.world_mut()
        .resource_mut::<crate::viewer::interaction::PlayerInventory>()
        .add_stack(stack);
    let item_id = app
        .world()
        .resource::<crate::viewer::interaction::CanonicalItemLedger>()
        .ledger
        .holders()
        .get(&HolderId::Player)
        .and_then(|state| state.items.first().map(|item| item.id))
        .unwrap();
    // Damage the player through the vitals component, then heal with one
    // stimpak: 140 + 30 = 170 under the level-1 max of 200.
    {
        let mut query = app
            .world_mut()
            .query_filtered::<&mut crate::viewer::effects::PlayerVitals, ()>();
        query.single_mut(app.world_mut()).unwrap().current_health = 140.0;
    }
    let output = exec(&mut app, &format!("useitem {:016x}", item_id.0));
    assert!(output.ok, "useitem failed: {:?}", output.error);
    assert_eq!(output.value["ingestible"]["editor_id"], "Stimpak");
    let healed_to = output.value["ingestible"]["healed_to"].as_f64().unwrap();
    assert!(
        (healed_to - 170.0).abs() < f64::EPSILON,
        "healed to {healed_to}"
    );
    assert_eq!(
        app.world()
            .resource::<crate::viewer::interaction::PlayerInventory>()
            .count(0x0001_5169),
        1
    );
    let canonical_count = app
        .world()
        .resource::<crate::viewer::interaction::CanonicalItemLedger>()
        .ledger
        .holders()
        .get(&HolderId::Player)
        .unwrap()
        .items
        .iter()
        .map(|item| item.count)
        .sum::<u32>();
    assert_eq!(canonical_count, 1);
}
