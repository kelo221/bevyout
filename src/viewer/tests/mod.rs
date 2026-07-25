use super::*;
use bevy::light::{FogVolume, NotShadowCaster, VolumetricFog};
use bevy::pbr::BakedPointShadowReceiver;
use bevy::post_process::bloom::{Bloom, BloomCompositeMode};

fn compatible_render_manifest() -> PreparedSceneManifest {
    let mut manifest: PreparedSceneManifest =
        ron::de::from_str(include_str!("../../../features/fixtures/scene.ron"))
            .expect("schema fixture should parse");
    manifest.schema_version = crate::vsa::CURRENT_MANIFEST_SCHEMA_VERSION;
    manifest.prepare_revision = Some(crate::vsa::CURRENT_PREPARE_REVISION.into());
    manifest.converter_revision = Some(PREPARED_CONVERTER_REVISION.into());
    manifest.physics_schema_version = Some(PHYSICS_ASSET_SCHEMA_VERSION);
    manifest.bake = None;
    manifest
}

#[test]
fn render_recovery_refreshes_preparation_before_offering_a_bake() {
    let mut manifest = compatible_render_manifest();
    manifest.schema_version -= 1;
    assert_eq!(
        next_render_cache_action(&manifest),
        RenderCacheAction::Reprepare
    );

    manifest.schema_version = crate::vsa::CURRENT_MANIFEST_SCHEMA_VERSION;
    assert_eq!(
        next_render_cache_action(&manifest),
        RenderCacheAction::Rebake
    );
}

#[test]
fn render_recovery_rebakes_stale_bakes_and_accepts_current_ones() {
    let mut manifest = compatible_render_manifest();
    manifest.bake = Some(crate::vsa::PreparedBake {
        bake_revision: Some(crate::vsa::CURRENT_BAKE_REVISION.into()),
        source_fingerprint: "fixture".into(),
        scene_path: "baked/scene.glb".into(),
        irradiance_volume: Some(crate::vsa::PreparedIrradianceVolume {
            asset_path: "baked/irradiance.ktx2".into(),
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0; 3],
            resolution: [1; 3],
            intensity: 1.0,
        }),
    });
    assert_eq!(
        next_render_cache_action(&manifest),
        RenderCacheAction::Ready
    );

    manifest.bake.as_mut().unwrap().bake_revision = Some("stale-bake".into());
    assert_eq!(
        next_render_cache_action(&manifest),
        RenderCacheAction::Rebake
    );
}

#[test]
fn glow_card_names_are_detected_without_matching_regular_meshes() {
    assert!(is_glow_card_mesh_name("LightGlow01:0.001"));
    assert!(is_glow_card_mesh_name("lightglow01"));
    assert!(!is_glow_card_mesh_name("ShackHangingLight02:51"));
}

#[test]
fn render_report_path_resolves_to_project_root() {
    let manifest = Path::new("project/.bevyout/cache/scenes/000151e3/scene.ron");
    assert_eq!(
        render_report_path(manifest),
        Path::new("project").join("render_timings.csv")
    );
}

#[test]
fn ao_strength_scales_baked_darkness_without_changing_alpha() {
    let baseline = VertexAttributeValues::Float32x4(vec![[0.72, 0.8, 0.9, 0.5]]);
    let mut values = baseline.clone();
    scale_ao_colors(&mut values, &baseline, 0.5);
    let VertexAttributeValues::Float32x4(values) = values else {
        panic!("expected float colors");
    };
    assert!((values[0][0] - 0.86).abs() < 0.001);
    assert!((values[0][1] - 0.9).abs() < 0.001);
    assert!((values[0][3] - 0.5).abs() < 0.001);

    let mut disabled = baseline.clone();
    scale_ao_colors(&mut disabled, &baseline, 0.0);
    let VertexAttributeValues::Float32x4(disabled) = disabled else {
        panic!("expected float colors");
    };
    assert_eq!(disabled[0], [1.0, 1.0, 1.0, 0.5]);

    let baseline = VertexAttributeValues::Unorm16x4(vec![[47_185, 52_428, 58_982, 65_535]]);
    let mut values = baseline.clone();
    scale_ao_colors(&mut values, &baseline, 0.0);
    let VertexAttributeValues::Unorm16x4(values) = values else {
        panic!("expected normalized 16-bit colors");
    };
    assert_eq!(values[0], [65_535, 65_535, 65_535, 65_535]);
}

#[test]
fn image_space_eye_adaptation_speed_uses_documented_endpoints() {
    assert!((image_space_eye_adaptation_speed(0.0) * 2.0 - 16.0).abs() < f32::EPSILON);
    assert!((image_space_eye_adaptation_speed(1.0) * 2.0 - 1.0).abs() < f32::EPSILON);
    assert!(image_space_eye_adaptation_speed(0.5) > image_space_eye_adaptation_speed(0.9));
}

#[test]
fn image_space_tint_maps_neutral_white_to_neutral_balance() {
    let (temperature, tint) = image_space_tint_to_white_balance([1.0, 1.0, 1.0], 1.0)
        .expect("white has a valid chromaticity");
    assert!(temperature.abs() < 0.001);
    assert!(tint.abs() < 0.001);
}

#[test]
fn image_space_warm_tint_produces_a_warm_artistic_cast() {
    let (temperature, tint) = image_space_tint_to_white_balance([0.69, 0.56, 0.30], 0.5)
        .expect("warm tint has a valid chromaticity");
    assert!(temperature > 0.1);
    assert!(tint < -0.1);
}

#[test]
fn image_space_settings_map_to_grading_and_target_exposure() {
    let mut image_space = ImageSpaceInfo {
        flags: 0x0f,
        hdr_target_lum: 2.0,
        cinematic_brightness: 4.0,
        cinematic_saturation: 0.5,
        cinematic_contrast_avg_lum: 0.25,
        cinematic_contrast: 1.5,
        cinematic_brightness_tint_rgb: [0.8, 0.9, 1.0],
        cinematic_brightness_tint_value: 1.0,
        ..default()
    };
    image_space.eye_adapt_speed = 0.25;
    let mut curves = Assets::<AutoExposureCompensationCurve>::default();
    let (grading, auto_exposure) = camera_post_processing(Some(&image_space), &mut curves);

    assert!((grading.global.exposure - 2.0).abs() < f32::EPSILON);
    assert!((grading.global.post_saturation - 0.5).abs() < f32::EPSILON);
    let expected_gamma = 1.0 / 1.5;
    let expected_gain = 0.25_f32.powf((1.0 - 1.5) / 1.5);
    assert_eq!(grading.shadows.contrast, 1.0);
    assert_eq!(grading.midtones.contrast, 1.0);
    assert_eq!(grading.highlights.contrast, 1.0);
    assert!((grading.shadows.gamma - expected_gamma).abs() < f32::EPSILON);
    assert!((grading.midtones.gamma - expected_gamma).abs() < f32::EPSILON);
    assert!((grading.highlights.gamma - expected_gamma).abs() < f32::EPSILON);
    assert!((grading.shadows.gain - expected_gain).abs() < f32::EPSILON);
    assert!((grading.midtones.gain - expected_gain).abs() < f32::EPSILON);
    assert!((grading.highlights.gain - expected_gain).abs() < f32::EPSILON);
    assert!(grading.global.tint.abs() > 0.0 || grading.global.temperature.abs() > 0.0);
    let auto_exposure = auto_exposure.expect("image space enables auto exposure");
    assert!((auto_exposure.speed_brighten - 12.25).abs() < f32::EPSILON);
    assert_eq!(curves.len(), 1);
}

#[test]
fn image_space_flags_zero_leave_cinematic_grading_neutral() {
    let image_space = ImageSpaceInfo {
        flags: 0,
        cinematic_brightness: 4.0,
        cinematic_saturation: 0.1,
        cinematic_contrast: 1.5,
        cinematic_brightness_tint_rgb: [0.1, 0.8, 0.2],
        cinematic_brightness_tint_value: 1.0,
        ..default()
    };
    let mut curves = Assets::<AutoExposureCompensationCurve>::default();
    let (grading, _) = camera_post_processing(Some(&image_space), &mut curves);

    assert_eq!(grading.global.exposure, 0.0);
    assert_eq!(grading.global.post_saturation, 1.0);
    assert_eq!(grading.shadows.contrast, 1.0);
    assert_eq!(grading.midtones.contrast, 1.0);
    assert_eq!(grading.highlights.contrast, 1.0);
    assert_eq!(grading.global.temperature, 0.0);
    assert_eq!(grading.global.tint, 0.0);
}

#[test]
fn image_space_bloom_keeps_old_viewer_values_as_the_neutral_profile() {
    assert_eq!(
        image_space_bloom_values(Some(&ImageSpaceInfo::default()), true),
        (0.2, 0.05, 0.2)
    );

    let image_space = ImageSpaceInfo {
        hdr_bright_scale: 1.5,
        hdr_bright_clamp: 0.35,
        bloom_blur_radius: 0.8,
        bloom_alpha_mult_interior: 0.2,
        bloom_alpha_mult_exterior: 0.5,
        ..default()
    };

    let interior = image_space_bloom_values(Some(&image_space), true);
    assert!((interior.0 - 0.06).abs() < 0.00001);
    assert!((interior.1 - (0.05 * 0.35 / 0.225)).abs() < 0.00001);
    assert!((interior.2 - 0.22).abs() < 0.00001);

    let exterior = image_space_bloom_values(Some(&image_space), false);
    assert!((exterior.0 - 0.15).abs() < 0.00001);
    assert!((exterior.1 - (0.05 * 0.35 / 0.225)).abs() < 0.00001);
    assert!((exterior.2 - 0.22).abs() < 0.00001);
}

#[test]
fn image_space_refresh_updates_cells_without_overwriting_bloom_overrides() {
    let mut world = World::new();
    world.insert_resource(Assets::<AutoExposureCompensationCurve>::default());
    world.insert_resource(ImageSpaceBloomOverrides {
        intensity: Some(0.9),
        threshold: Some(0.7),
        softness: None,
    });
    world.spawn((Camera3d::default(), Bloom::default()));

    let mut cell = compatible_render_manifest().cell;
    cell.interior = true;
    cell.image_space = Some(ImageSpaceInfo {
        hdr_bright_scale: 1.5,
        hdr_bright_clamp: 0.35,
        bloom_blur_radius: 0.8,
        bloom_alpha_mult_interior: 0.2,
        bloom_alpha_mult_exterior: 0.5,
        ..default()
    });
    refresh_camera_post_processing(&mut world, &cell);

    let camera = world
        .query_filtered::<Entity, With<Camera3d>>()
        .single(&world)
        .expect("test camera");
    let bloom = world.get::<Bloom>(camera).expect("camera bloom");
    assert_eq!(bloom.intensity, 0.9);
    assert_eq!(bloom.prefilter.threshold, 0.7);
    assert!((bloom.prefilter.threshold_softness - 0.22).abs() < 0.00001);

    cell.interior = false;
    cell.image_space.as_mut().unwrap().bloom_blur_radius = 8.0;
    refresh_camera_post_processing(&mut world, &cell);

    let bloom = world.get::<Bloom>(camera).expect("camera bloom");
    assert_eq!(bloom.intensity, 0.9);
    assert_eq!(bloom.prefilter.threshold, 0.7);
    assert!((bloom.prefilter.threshold_softness - 0.4).abs() < 0.00001);
}

#[test]
fn missing_image_space_keeps_fixed_camera_post_processing() {
    let mut curves = Assets::<AutoExposureCompensationCurve>::default();
    let (grading, auto_exposure) = camera_post_processing(None, &mut curves);
    assert_eq!(grading.global.exposure, 0.0);
    assert!(auto_exposure.is_none());
    assert_eq!(curves.len(), 0);
}

#[test]
fn fallout_bloom_uses_explicit_old_school_baseline() {
    let bloom = super::scene::fallout_bloom();

    assert_eq!(bloom.intensity, 0.2);
    assert_eq!(bloom.prefilter.threshold, 0.05);
    assert_eq!(bloom.prefilter.threshold_softness, 0.2);
    assert_eq!(bloom.composite_mode, BloomCompositeMode::Additive);
    assert!(bloom.prefilter.threshold > 0.0);
    assert_ne!(
        bloom.prefilter.threshold,
        Bloom::NATURAL.prefilter.threshold
    );
}

#[test]
fn prepared_static_meshes_leave_runtime_casters_but_physics_meshes_do_not() {
    let mut app = App::new();
    app.add_systems(Update, mark_prepared_shadow_meshes);

    let static_root = app
        .world_mut()
        .spawn((
            PreparedPointShadowReceiverRoot,
            BakedStaticSceneRoot,
            Transform::default(),
        ))
        .id();
    let moving_root = app
        .world_mut()
        .spawn((PreparedPointShadowReceiverRoot, Transform::default()))
        .id();
    let static_mesh = app
        .world_mut()
        .spawn((
            Mesh3d::default(),
            Transform::default(),
            ChildOf(static_root),
        ))
        .id();
    let moving_mesh = app
        .world_mut()
        .spawn((
            Mesh3d::default(),
            Transform::default(),
            ChildOf(moving_root),
        ))
        .id();

    app.update();

    assert!(
        app.world()
            .entity(static_mesh)
            .contains::<BakedPointShadowReceiver>()
    );
    assert!(
        app.world()
            .entity(static_mesh)
            .contains::<NotShadowCaster>()
    );
    assert!(
        app.world()
            .entity(moving_mesh)
            .contains::<BakedPointShadowReceiver>()
    );
    assert!(
        !app.world()
            .entity(moving_mesh)
            .contains::<NotShadowCaster>()
    );
}

#[test]
fn forward_shader_combines_prepared_and_realtime_visibility() {
    let shader = include_str!("../../../third_party/bevy_pbr-0.19.0/src/render/pbr_functions.wgsl");
    assert_eq!(shader.matches("shadows::fetch_point_shadow(").count(), 2);
    assert!(shader.contains("dominant_point_light_uses_baked_shadow"));
    assert!(shader.contains("dominant_point_light_uses_realtime_shadow"));
    assert!(shader.contains("min(dominant_shadow, realtime_shadow)"));
}

#[test]
fn fog_uses_fo3_distances_and_rejects_invalid_ranges() {
    let lighting = PreparedCellLighting {
        fog_rgba: [0.1, 0.2, 0.3, 1.0],
        directional_rgba: [0.4, 0.5, 0.6, 1.0],
        fog_near: 10.0,
        fog_far: 100.0,
        fog_clip_distance: 80.0,
        fog_power: 0.5,
        ..default()
    };
    let fog = distance_fog(&lighting, DEFAULT_FOG_STRENGTH).expect("valid FO3 fog should map");
    match fog.falloff {
        FogFalloff::Linear { start, end } => {
            assert!((start - 10.0 * FO3_SCALE).abs() < f32::EPSILON);
            assert!((end - 80.0 * FO3_SCALE).abs() < f32::EPSILON);
        }
        _ => panic!("expected linear fog"),
    }
    assert_eq!(fog.directional_light_exponent, 1.0);
    assert!(
        distance_fog(
            &PreparedCellLighting {
                fog_near: 20.0,
                fog_far: 10.0,
                ..lighting
            },
            DEFAULT_FOG_STRENGTH,
        )
        .is_none()
    );
}

#[test]
fn volumetric_fog_default_preserves_authored_values_at_five_percent() {
    assert_eq!(DEFAULT_VOLUMETRIC_FOG_MULTIPLIER, 0.05);
}

#[test]
fn volumetric_fog_uses_cell_range_and_live_fog_strength() {
    let lighting = PreparedCellLighting {
        fog_rgba: [0.1, 0.2, 0.3, 0.0],
        directional_rgba: [0.4, 0.5, 0.6, 0.0],
        fog_near: 10.0,
        fog_far: 100.0,
        fog_clip_distance: 80.0,
        ..default()
    };
    let weak = volumetric_fog_density(&lighting, 0.01, 1.0).expect("valid volumetric fog");
    let strong = volumetric_fog_density(&lighting, 0.2, 1.0).expect("valid volumetric fog");
    let multiplied =
        volumetric_fog_density(&lighting, 0.01, 10.0).expect("valid multiplied volumetric fog");
    assert!(weak > 0.0);
    assert!(strong > weak);
    assert!(multiplied > weak);

    let shorter_cell = PreparedCellLighting {
        fog_far: 50.0,
        ..lighting.clone()
    };
    let shorter = volumetric_fog_density(&shorter_cell, 0.01, 1.0).expect("valid volumetric fog");
    assert!(shorter > weak);

    let (camera_fog, volume_fog) =
        volumetric_fog_profile(&lighting, 0.01, 1.0).expect("valid volumetric profile");
    assert_eq!(camera_fog.step_count, 64);
    assert_eq!(volume_fog.absorption, 0.3);
    assert_eq!(volume_fog.scattering, 0.3);
    assert!((volume_fog.density_factor - weak).abs() < f32::EPSILON);
}

#[test]
fn volumetric_fog_system_attaches_a_camera_following_volume() {
    let mut app = App::new();
    app.insert_resource(FogStrength(DEFAULT_FOG_STRENGTH));
    app.insert_resource(VolumetricFogMultiplier(1.0));
    let mut manifest = compatible_render_manifest();
    manifest.cell.effective_lighting = Some(PreparedCellLighting {
        fog_rgba: [0.1, 0.2, 0.3, 0.0],
        directional_rgba: [0.4, 0.5, 0.6, 0.0],
        fog_near: 10.0,
        fog_far: 100.0,
        fog_clip_distance: 80.0,
        ..default()
    });
    app.insert_resource(crate::viewer::LoadedSceneManifest(manifest));
    app.world_mut().spawn(Camera3d::default());
    app.add_systems(Update, apply_volumetric_fog);

    app.update();

    let camera = app
        .world_mut()
        .query_filtered::<Entity, With<Camera3d>>()
        .single(app.world())
        .expect("test camera");
    assert!(app.world().entity(camera).contains::<VolumetricFog>());
    let volume_count = app
        .world_mut()
        .query_filtered::<Entity, With<CellVolumetricFog>>()
        .iter(app.world())
        .count();
    assert_eq!(volume_count, 1);
    let fog_volume_count = app
        .world_mut()
        .query_filtered::<Entity, With<FogVolume>>()
        .iter(app.world())
        .count();
    assert_eq!(fog_volume_count, 1);

    let volume = app
        .world_mut()
        .query_filtered::<Entity, With<CellVolumetricFog>>()
        .single(app.world())
        .expect("test fog volume");
    let initial_density = app
        .world()
        .get::<FogVolume>(volume)
        .expect("fog volume component")
        .density_factor;
    app.world_mut().resource_mut::<VolumetricFogMultiplier>().0 = 100.0;
    app.update();
    let volume_count_after_update = app
        .world_mut()
        .query_filtered::<Entity, With<CellVolumetricFog>>()
        .iter(app.world())
        .count();
    assert_eq!(volume_count_after_update, 1);
    let multiplied_density = app
        .world()
        .get::<FogVolume>(volume)
        .expect("updated fog volume component")
        .density_factor;
    assert!(multiplied_density > initial_density);
}

#[test]
fn directional_rotation_and_light_scale_are_deterministic() {
    let lighting = PreparedCellLighting {
        directional_rotation_z: 90,
        ..default()
    };
    let expected = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
    let actual = Quat::from_array(lighting.directional_rotation_xyzw());
    assert!(actual.dot(expected).abs() > 1.0 - 1e-5);
    assert_eq!(
        scaled_directional_illuminance(10_000.0, 256.0, false),
        20_000.0
    );
    assert_eq!(scaled_directional_illuminance(10_000.0, 256.0, true), 0.0);
}
