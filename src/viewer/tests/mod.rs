use super::*;
use bevy::light::NotShadowCaster;
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
fn image_space_settings_map_to_grading_and_target_exposure() {
    let mut image_space = ImageSpaceInfo {
        flags: 0x0f,
        hdr_target_lum: 2.0,
        brightness: 4.0,
        cinematic_saturation: 0.5,
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
    assert!((grading.shadows.contrast - 1.5).abs() < f32::EPSILON);
    assert!((grading.midtones.contrast - 1.5).abs() < f32::EPSILON);
    assert!((grading.highlights.contrast - 1.5).abs() < f32::EPSILON);
    assert!(grading.global.tint.abs() > 0.0 || grading.global.temperature.abs() > 0.0);
    let auto_exposure = auto_exposure.expect("image space enables auto exposure");
    assert!((auto_exposure.speed_brighten - 12.25).abs() < f32::EPSILON);
    assert_eq!(curves.len(), 1);
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
