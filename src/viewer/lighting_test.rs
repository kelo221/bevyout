//! Hermetic visual proof for prepared static plus realtime moving shadows.

use std::{path::PathBuf, sync::Arc};

use anyhow::{Result, bail};
use bevy::camera::{Exposure, Hdr};
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::light::{NotShadowCaster, NotShadowReceiver, PointLightShadowMap, ShadowFilteringMethod};
use bevy::pbr::{
    BakedPointLightShadow, BakedPointShadowMap, BakedPointShadowReceiver,
    DefaultOpaqueRendererMethod, PointLightShadowSamples,
};
use bevy::prelude::*;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::WindowPlugin;

use super::RenderReportBuffer;
use super::agent_bridge;
use super::lighting_demo_policy::DemoOrbit;
use crate::cli::LightingTestArgs;
use crate::vsa::{
    DynamicLight, DynamicLightEffect, DynamicLightIlluminationMode, DynamicLightShadowProxy,
    DynamicLightType, DynamicLightVolumetricParameters, DynamicLightVolumetricType,
    DynamicLightingDiagnostics, DynamicLightingPlugin, DynamicLightingSettings,
    DynamicLightingView, STATIC_POINT_SHADOW_NEAR_Z, StaticShadowBakeLight,
    bake_static_point_shadow_bytes,
};

const STATIC_PILLAR_CENTER: [f32; 3] = [-2.0, 1.25, 0.0];
const STATIC_PILLAR_SIZE: [f32; 3] = [1.5, 2.5, 1.5];
const LIGHT_TRANSLATION: [f32; 3] = [0.0, 5.0, 0.0];
const LIGHT_RANGE: f32 = 14.0;
const DEMO_HISTORY: usize = 120;

pub fn lighting_test(args: LightingTestArgs) -> Result<()> {
    if let Some(seconds) = args.trace_seconds
        && (!seconds.is_finite() || seconds <= 0.0)
    {
        bail!("--trace-seconds must be finite and greater than zero");
    }

    let static_triangles = cuboid_triangles(
        Vec3::from_array(STATIC_PILLAR_CENTER),
        Vec3::from_array(STATIC_PILLAR_SIZE),
    );
    let depth = bake_static_point_shadow_bytes(
        &static_triangles,
        &[StaticShadowBakeLight {
            translation: Vec3::from_array(LIGHT_TRANSLATION),
            range: LIGHT_RANGE,
        }],
        args.shadow_resolution,
        STATIC_POINT_SHADOW_NEAR_Z,
    )?;
    println!(
        "lighting test: baked {} static triangles into one {}x{} point-shadow cubemap",
        static_triangles.len(),
        args.shadow_resolution,
        args.shadow_resolution,
    );

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevyout hybrid lighting test".into(),
                resolution: (1280, 720).into(),
                focused: true,
                ..default()
            }),
            ..default()
        }),
        FrameTimeDiagnosticsPlugin::new(DEMO_HISTORY),
        RenderDiagnosticsPlugin,
        DynamicLightingPlugin,
    ));
    app.insert_resource(ClearColor(Color::srgb(0.015, 0.018, 0.025)))
        .insert_resource(DefaultOpaqueRendererMethod::forward())
        .insert_resource(PointLightShadowMap { size: 256 })
        .insert_resource(PointLightShadowSamples(1))
        .insert_resource(BakedPointShadowMap {
            data: Some(Arc::from(depth.into_boxed_slice())),
            fingerprint: Some(format!("lighting-test-v1-{}", args.shadow_resolution)),
            resolution: args.shadow_resolution,
            layers: 1,
        })
        .insert_resource(RenderReportBuffer::default())
        .insert_resource(DemoMotion {
            paused: args.gpu_acceptance_capture.is_some(),
            ..Default::default()
        })
        .add_systems(Startup, setup_lighting_test)
        .add_systems(
            Update,
            (
                animate_moving_shadow_caster,
                toggle_dynamic_lighting,
                toggle_shadow_sources,
                update_dynamic_lighting_status,
                super::record_render_sample,
                super::update_fps_text,
            ),
        );
    app.add_plugins(crate::console::ConsolePlugin);

    if let Some(seconds) = args.trace_seconds {
        app.insert_resource(DemoExitTimer(seconds))
            .add_systems(Update, stop_after_trace_window);
    }
    if let Some(path) = args.gpu_acceptance_capture {
        app.insert_resource(GpuAcceptanceCapture {
            path,
            control_path: args.gpu_acceptance_control_capture,
            custom_only: args.gpu_acceptance_custom_only,
            disable_custom: args.gpu_acceptance_disable_custom,
            orthographic: args.gpu_acceptance_orthographic,
        })
        .add_systems(
            Update,
            (configure_gpu_acceptance, capture_gpu_acceptance).chain(),
        );
    }
    if args.agent_bridge {
        agent_bridge::install(&mut app, args.agent_port);
    }

    app.run();
    Ok(())
}

#[derive(Component)]
struct MovingShadowCaster(DemoOrbit);

#[derive(Component)]
struct HybridPointLight;

#[derive(Component)]
struct DynamicLightingStatusText;

#[derive(Resource)]
struct GpuAcceptanceCapture {
    path: PathBuf,
    control_path: Option<PathBuf>,
    custom_only: bool,
    disable_custom: bool,
    orthographic: bool,
}

fn configure_gpu_acceptance(
    options: Res<GpuAcceptanceCapture>,
    mut configured: Local<bool>,
    mut settings: ResMut<DynamicLightingSettings>,
    mut motion: ResMut<DemoMotion>,
    mut bevy_lights: Query<&mut Visibility, With<HybridPointLight>>,
    mut proxies: Query<&mut Visibility, (With<DynamicLightShadowProxy>, Without<HybridPointLight>)>,
    mut views: Query<(&Transform, &mut Projection), With<DynamicLightingView>>,
) {
    if *configured {
        return;
    }
    settings.enabled = !options.disable_custom;
    motion.paused = true;
    let Ok((_, mut projection)) = views.single_mut() else {
        return;
    };
    if options.orthographic {
        *projection = Projection::Orthographic(OrthographicProjection {
            scale: 0.012,
            ..OrthographicProjection::default_3d()
        });
    }
    if options.custom_only {
        settings.shadow_proxies_enabled = false;
        for mut visibility in &mut bevy_lights {
            *visibility = Visibility::Hidden;
        }
        for mut visibility in &mut proxies {
            *visibility = Visibility::Hidden;
        }
    }
    *configured = true;
}

fn spawn_gpu_acceptance_targets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    camera: Transform,
) {
    let forward = camera.forward().as_vec3();
    let right = camera.right().as_vec3();
    let up = camera.up().as_vec3();
    let panel_mesh = meshes.add(Cuboid::new(1.5, 1.2, 0.1));
    let panel_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.32, 0.36),
        emissive: LinearRgba::rgb(0.01, 0.012, 0.016),
        metallic: 0.15,
        perceptual_roughness: 0.22,
        ..default()
    });
    let colors = [
        Color::srgb(1.0, 0.15, 0.08),
        Color::srgb(1.0, 0.62, 0.08),
        Color::srgb(0.75, 1.0, 0.12),
        Color::srgb(0.12, 1.0, 0.38),
        Color::srgb(0.08, 0.82, 1.0),
        Color::srgb(0.18, 0.36, 1.0),
        Color::srgb(0.62, 0.18, 1.0),
        Color::srgb(1.0, 0.14, 0.68),
    ];
    let columns = [-4.5, -1.5, 1.5, 4.5];
    let rows = [-0.8, -2.3];
    for (index, light_type) in DynamicLightType::ALL.into_iter().enumerate() {
        let center = camera.translation
            + forward * 8.0
            + right * columns[index % columns.len()]
            + up * rows[index / columns.len()];
        commands.spawn((
            Name::new(format!("GPU acceptance receiver {light_type:?}")),
            Mesh3d(panel_mesh.clone()),
            MeshMaterial3d(panel_material.clone()),
            Transform::from_translation(center).with_rotation(camera.rotation),
            NotShadowCaster,
        ));
        let light_position = center - forward * 0.65;
        let rotation = match light_type {
            DynamicLightType::Discoball
            | DynamicLightType::Interference
            | DynamicLightType::Rotor
            | DynamicLightType::Disco => Quat::from_rotation_arc(Vec3::Z, forward),
            DynamicLightType::Point
            | DynamicLightType::Spot
            | DynamicLightType::Wave
            | DynamicLightType::Shock => camera.rotation,
        };
        commands.spawn((
            Name::new(format!("GPU acceptance light {light_type:?}")),
            DynamicLight::with_effect(320.0, DynamicLightEffect::Steady)
                .with_type(light_type)
                .with_color(colors[index])
                .with_radius(1.6)
                .with_shadows(false),
            Transform::from_translation(light_position).with_rotation(rotation),
            GlobalTransform::default(),
        ));
    }

    let volume_only = |volumetric_type, radius, thickness, intensity| {
        DynamicLight::with_effect(0.0, DynamicLightEffect::Steady)
            .with_color(Color::WHITE)
            .with_volumetric(DynamicLightVolumetricParameters {
                volumetric_type,
                radius,
                thickness,
                intensity,
                visibility: 2.0,
            })
    };
    commands.spawn((
        Name::new("GPU acceptance clear-background sphere fog"),
        volume_only(DynamicLightVolumetricType::Sphere, 1.7, 2.4, 0.75)
            .with_color(Color::srgb(0.05, 0.65, 1.0)),
        Transform::from_translation(camera.translation + forward * 10.0 - right * 2.4 + up * 2.1),
        GlobalTransform::default(),
    ));
    let mut cone = volume_only(DynamicLightVolumetricType::ConeZ, 8.0, 2.0, 0.65)
        .with_color(Color::srgb(1.0, 0.28, 0.06));
    cone.config.spatial.inner_cutoff_degrees = 4.0;
    cone.config.spatial.outer_cutoff_degrees = 8.0;
    commands.spawn((
        Name::new("GPU acceptance clear-background ConeZ fog"),
        cone,
        Transform::from_translation(camera.translation + forward * 4.0 + right * 2.5 + up * 1.8)
            .with_rotation(Quat::from_rotation_arc(Vec3::Z, forward)),
        GlobalTransform::default(),
    ));
}

#[derive(Default)]
struct GpuCaptureState {
    phase: u8,
    waited_frames: u8,
    reported_wait: bool,
}

fn capture_gpu_acceptance(
    mut commands: Commands,
    time: Res<Time>,
    options: Res<GpuAcceptanceCapture>,
    diagnostics: Res<DynamicLightingDiagnostics>,
    mut settings: ResMut<DynamicLightingSettings>,
    mut state: Local<GpuCaptureState>,
) {
    if time.elapsed_secs() >= 5.0 && state.phase == 0 && !state.reported_wait {
        warn!(
            "gpu acceptance waiting: surface_ready={} volumetric_ready={} lights={} volumes={}",
            diagnostics.surface_pass_ready(),
            diagnostics.volumetric_pass_ready(),
            diagnostics.extracted_light_count(),
            diagnostics.extracted_volumetric_light_count(),
        );
        state.reported_wait = true;
    }
    if time.elapsed_secs() < 1.5
        || !diagnostics.surface_pass_ready()
        || !diagnostics.volumetric_pass_ready()
        || (options.custom_only
            && (diagnostics.extracted_light_count() != 10
                || diagnostics.extracted_volumetric_light_count() != 2))
    {
        return;
    }
    match state.phase {
        0 => {
            settings.freeze_effect_time = true;
            state.phase = 1;
        }
        1 => {
            state.waited_frames += 1;
            // The forward PBR pipeline may still be compiling after the
            // artifact buffers become ready, especially on a cold DX12 run.
            // Give it enough presented frames before the enabled capture so
            // the comparison cannot accidentally record the fallback frame.
            if state.waited_frames < 24 {
                return;
            }
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(options.path.clone()));
            state.waited_frames = 0;
            state.phase = 2;
        }
        2 if options.control_path.is_some() => {
            state.waited_frames += 1;
            if state.waited_frames >= 8 {
                settings.enabled = false;
                state.waited_frames = 0;
                state.phase = 3;
            }
        }
        3 => {
            state.waited_frames += 1;
            if state.waited_frames >= 4 {
                commands
                    .spawn(Screenshot::primary_window())
                    .observe(save_to_disk(options.control_path.clone().unwrap()));
                state.phase = 4;
            }
        }
        _ => {}
    }
}

#[derive(Resource, Default)]
struct DemoMotion {
    elapsed_seconds: f32,
    paused: bool,
}

#[derive(Resource)]
struct DemoExitTimer(f32);

fn setup_lighting_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    acceptance: Option<Res<GpuAcceptanceCapture>>,
) {
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.08, 0.10, 0.16),
        brightness: 90.0,
        affects_lightmapped_meshes: true,
    });

    if let Some(options) = acceptance.filter(|options| options.custom_only) {
        let camera_transform =
            Transform::from_xyz(14.0, 13.0, 19.0).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y);
        spawn_gpu_acceptance_targets(&mut commands, &mut meshes, &mut materials, camera_transform);
        let projection = if options.orthographic {
            Projection::Orthographic(OrthographicProjection {
                scale: 0.012,
                ..OrthographicProjection::default_3d()
            })
        } else {
            Projection::default()
        };
        commands.spawn((
            Name::new("GPU acceptance camera"),
            Camera3d::default(),
            projection,
            Hdr,
            Msaa::Off,
            DepthPrepass,
            DynamicLightingView,
            ShadowFilteringMethod::Hardware2x2,
            Tonemapping::AcesFitted,
            Exposure { ev100: 2.0 },
            camera_transform,
        ));
        return;
    }

    let floor_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.58, 0.61, 0.68),
        perceptual_roughness: 0.92,
        ..default()
    });
    commands.spawn((
        Name::new("Hybrid shadow receiver floor"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 24.0))),
        MeshMaterial3d(floor_material),
        BakedPointShadowReceiver,
        NotShadowCaster,
    ));

    let pillar_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.92, 0.43, 0.16),
        perceptual_roughness: 0.75,
        ..default()
    });
    commands.spawn((
        Name::new("Static orange pillar (CPU baked)"),
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::from_array(STATIC_PILLAR_SIZE)))),
        MeshMaterial3d(pillar_material),
        Transform::from_translation(Vec3::from_array(STATIC_PILLAR_CENTER)),
        BakedPointShadowReceiver,
        NotShadowCaster,
    ));

    let moving_orbit = DemoOrbit {
        center: [0.0, 1.0, 0.0],
        radius: 3.0,
        radians_per_second: 0.8,
    };
    let moving_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.10, 0.42, 1.0),
        metallic: 0.15,
        perceptual_roughness: 0.35,
        ..default()
    });
    commands.spawn((
        Name::new("Moving blue realtime shadow caster"),
        Mesh3d(meshes.add(Cuboid::new(1.25, 2.0, 1.25))),
        MeshMaterial3d(moving_material),
        Transform::from_translation(Vec3::from_array(moving_orbit.position(0.0))),
        BakedPointShadowReceiver,
        MovingShadowCaster(moving_orbit),
    ));

    commands.spawn((
        Name::new("Hybrid point light"),
        PointLight {
            color: Color::srgb(1.0, 0.82, 0.62),
            intensity: 90_000.0,
            range: LIGHT_RANGE,
            radius: 0.0,
            shadow_maps_enabled: true,
            shadow_map_near_z: STATIC_POINT_SHADOW_NEAR_Z,
            ..default()
        },
        BakedPointLightShadow {
            layer: 0,
            baked_translation: Vec3::from_array(LIGHT_TRANSLATION),
            baked_range: LIGHT_RANGE,
            near_z: STATIC_POINT_SHADOW_NEAR_Z,
        },
        HybridPointLight,
        Transform::from_translation(Vec3::from_array(LIGHT_TRANSLATION)),
    ));

    let light_marker = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.78, 0.35),
        emissive: LinearRgba::rgb(12.0, 8.0, 2.0),
        unlit: true,
        ..default()
    });
    commands.spawn((
        Name::new("Point-light marker"),
        Mesh3d(meshes.add(Sphere::new(0.16).mesh().uv(24, 12))),
        MeshMaterial3d(light_marker),
        Transform::from_translation(Vec3::from_array(LIGHT_TRANSLATION)),
        NotShadowCaster,
        NotShadowReceiver,
    ));

    let receiver_mesh = meshes.add(Cuboid::new(1.7, 0.18, 1.7));
    let receiver_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.24, 0.28),
        perceptual_roughness: 0.82,
        ..default()
    });

    let strobe_intensity = 120.0;
    let strobe_color = Color::srgb(0.42, 0.18, 1.0);
    let strobe_receiver = Vec3::new(7.0, 0.09, 0.0);
    commands.spawn((
        Name::new("DynamicLighting purple strobe receiver"),
        Mesh3d(receiver_mesh.clone()),
        MeshMaterial3d(receiver_material.clone()),
        Transform::from_translation(strobe_receiver),
        NotShadowCaster,
    ));
    let strobe_transform = Transform::from_translation(strobe_receiver + Vec3::Y * 1.2);
    let strobe_entity = commands
        .spawn((
            Name::new("DynamicLighting purple strobe"),
            DynamicLight::strobe(strobe_intensity)
                .with_color(strobe_color)
                .with_radius(2.4)
                .with_bounce_approximation(true)
                .with_shadows(true)
                .with_volumetric(DynamicLightVolumetricParameters {
                    volumetric_type: DynamicLightVolumetricType::Sphere,
                    radius: 2.2,
                    thickness: 2.0,
                    intensity: 0.55,
                    visibility: 2.0,
                }),
            strobe_transform,
            GlobalTransform::default(),
        ))
        .id();
    commands.spawn((
        Name::new("DynamicLighting strobe shadow-only proxy"),
        DynamicLightShadowProxy::shadow_only_point_light(2.4),
        DynamicLightShadowProxy::realtime(strobe_entity),
        strobe_transform,
        Visibility::Visible,
    ));
    let strobe_blocker_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.13, 0.16),
        perceptual_roughness: 0.7,
        ..default()
    });
    commands.spawn((
        Name::new("DynamicLighting strobe shadow blocker"),
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.9, 0.5))),
        MeshMaterial3d(strobe_blocker_material),
        Transform::from_translation(strobe_receiver + Vec3::Y * 0.54),
    ));

    let effect_colors = [
        Color::srgb(1.0, 0.20, 0.12),
        Color::srgb(1.0, 0.55, 0.10),
        Color::srgb(1.0, 0.90, 0.18),
        Color::srgb(0.52, 1.0, 0.20),
        Color::srgb(0.10, 1.0, 0.58),
        Color::srgb(0.10, 0.82, 1.0),
        Color::srgb(0.18, 0.42, 1.0),
        Color::srgb(0.42, 0.18, 1.0),
        Color::srgb(0.78, 0.18, 1.0),
        Color::srgb(1.0, 0.18, 0.68),
        Color::srgb(1.0, 0.30, 0.36),
        Color::srgb(1.0, 0.72, 0.35),
        Color::srgb(0.72, 1.0, 0.42),
        Color::srgb(0.30, 1.0, 0.90),
        Color::srgb(0.38, 0.62, 1.0),
    ];
    for (index, effect) in DynamicLightEffect::ALL.into_iter().enumerate() {
        let column = index % 5;
        let row = index / 5;
        let receiver_position =
            Vec3::new(-5.5 + column as f32 * 2.75, 0.09, -6.0 + row as f32 * 2.2);
        commands.spawn((
            Name::new(format!("DynamicLighting effect receiver {effect:?}")),
            Mesh3d(receiver_mesh.clone()),
            MeshMaterial3d(receiver_material.clone()),
            Transform::from_translation(receiver_position),
            NotShadowCaster,
        ));
        commands.spawn((
            Name::new(format!("DynamicLighting effect {effect:?}")),
            DynamicLight::with_effect(80.0, effect)
                .with_color(effect_colors[index])
                .with_bounce_approximation(true)
                .with_radius(1.65),
            Transform::from_translation(receiver_position + Vec3::Y * 1.0),
            GlobalTransform::default(),
        ));
    }

    let spatial_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.72, 0.74, 0.78),
        perceptual_roughness: 0.9,
        ..default()
    });
    let spatial_mesh = meshes.add(Cuboid::new(2.5, 2.5, 0.14));
    let spatial_colors = [
        Color::srgb(1.0, 0.32, 0.24),
        Color::srgb(1.0, 0.64, 0.18),
        Color::srgb(0.82, 1.0, 0.22),
        Color::srgb(0.20, 1.0, 0.48),
        Color::srgb(0.16, 0.88, 1.0),
        Color::srgb(0.24, 0.46, 1.0),
        Color::srgb(0.62, 0.26, 1.0),
        Color::srgb(1.0, 0.24, 0.72),
    ];
    for (index, light_type) in DynamicLightType::ALL.into_iter().enumerate() {
        let column = index % 4;
        let row = index / 4;
        let receiver_x = [-7.5, -4.2, 4.2, 7.5][column];
        let receiver_position = Vec3::new(receiver_x, 1.35, 4.2 + row as f32 * 3.6);
        commands.spawn((
            Name::new(format!("DynamicLighting spatial receiver {light_type:?}")),
            Mesh3d(spatial_mesh.clone()),
            MeshMaterial3d(spatial_material.clone()),
            Transform::from_translation(receiver_position),
            NotShadowCaster,
        ));
        commands.spawn((
            Name::new(format!("DynamicLighting spatial {light_type:?}")),
            DynamicLight::with_effect(160.0, DynamicLightEffect::Steady)
                .with_type(light_type)
                .with_color(spatial_colors[index])
                .with_bounce_approximation(true)
                .with_radius(3.1),
            Transform::from_translation(receiver_position + Vec3::Z * 1.8).with_rotation(
                Quat::from_rotation_y(core::f32::consts::PI)
                    * Quat::from_rotation_z((index as f32 - 3.5) * 0.11),
            ),
            GlobalTransform::default(),
        ));
    }

    let volume_only = |volumetric_type, radius, thickness, intensity, visibility| {
        DynamicLight::with_effect(0.0, DynamicLightEffect::Steady).with_volumetric(
            DynamicLightVolumetricParameters {
                volumetric_type,
                radius,
                thickness,
                intensity,
                visibility,
            },
        )
    };
    commands.spawn((
        Name::new("DynamicLighting cyan scaled box fog"),
        volume_only(DynamicLightVolumetricType::Box, 2.1, 2.8, 0.45, 2.5)
            .with_color(Color::srgb(0.08, 0.75, 0.92)),
        Transform::from_xyz(0.0, 1.0, 0.0).with_scale(Vec3::new(1.7, 0.55, 0.9)),
        GlobalTransform::default(),
    ));

    let cone_z_position = Vec3::new(-6.0, 0.45, 5.5);
    commands.spawn((
        Name::new("DynamicLighting amber ConeZ fog"),
        volume_only(DynamicLightVolumetricType::ConeZ, 6.5, 2.2, 0.52, 3.0)
            .with_color(Color::srgb(1.0, 0.42, 0.08)),
        Transform::from_translation(cone_z_position).looking_at(Vec3::new(-2.0, 2.0, 1.0), Vec3::Y),
        GlobalTransform::default(),
    ));

    let cone_y_position = Vec3::new(5.5, 0.35, 5.0);
    let cone_y_direction = (Vec3::new(2.0, 3.0, 0.0) - cone_y_position).normalize();
    commands.spawn((
        Name::new("DynamicLighting green ConeY fog"),
        volume_only(DynamicLightVolumetricType::ConeY, 6.0, 2.2, 0.50, 3.0)
            .with_color(Color::srgb(0.12, 1.0, 0.32)),
        Transform::from_translation(cone_y_position)
            .with_rotation(Quat::from_rotation_arc(Vec3::Y, cone_y_direction)),
        GlobalTransform::default(),
    ));

    commands.spawn((
        Name::new("Lighting test camera"),
        Camera3d::default(),
        Hdr,
        Msaa::Off,
        DepthPrepass,
        DynamicLightingView,
        ShadowFilteringMethod::Hardware2x2,
        Tonemapping::AcesFitted,
        Exposure { ev100: 2.0 },
        Transform::from_xyz(14.0, 13.0, 19.0).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
    ));

    commands.spawn((
        Text::new(concat!(
            "HYBRID LIGHTING TEST\n",
            "Orange pillar: static shadow baked once on CPU\n",
            "Blue block: moves and casts a realtime shadow\n",
            "Floor: combines prepared + realtime visibility\n",
            "Purple point: isolated DynamicLighting Strobe effect\n",
            "Near grid: all 15 temporal effects on isolated receivers\n",
            "Far grid: Point, Spot, Discoball, Wave / Interference, Rotor, Shock, Disco\n",
            "Fog: strobing Sphere, scaled Box, rotated ConeZ and ConeY\n",
            "1: toggle baked static shadow | 2: toggle realtime shadow | B: direct/bounce\n",
            "3: custom pass | 4: Bevy lights | 5: shadow proxy | 6: fog | F: freeze | Space: motion"
        )),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            left: px(16),
            ..default()
        },
    ));

    commands.spawn((
        Text::new("DynamicLighting extracted -- | Bevy lights -- | proxy --"),
        DynamicLightingStatusText,
        TextFont {
            font_size: FontSize::Px(14.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            right: px(16),
            ..default()
        },
    ));

    commands.spawn((
        Text::new("FPS --"),
        super::FpsText,
        super::console::DiagnosticUi,
        Node {
            position_type: PositionType::Absolute,
            right: px(10),
            bottom: px(10),
            ..default()
        },
    ));
}

fn toggle_dynamic_lighting(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<DynamicLightingSettings>,
    mut dynamic_lights: Query<&mut DynamicLight>,
) {
    if keyboard.just_pressed(KeyCode::Digit3) {
        settings.enabled = !settings.enabled;
        info!(
            "lighting test: custom DynamicLighting {}",
            if settings.enabled {
                "enabled"
            } else {
                "disabled"
            }
        );
    }
    if keyboard.just_pressed(KeyCode::KeyF) {
        settings.freeze_effect_time = !settings.freeze_effect_time;
        info!(
            "lighting test: DynamicLighting effect time {}",
            if settings.freeze_effect_time {
                "frozen"
            } else {
                "running"
            }
        );
    }
    if keyboard.just_pressed(KeyCode::KeyB) {
        let enable_bounce = dynamic_lights.iter().next().is_none_or(|light| {
            light.config.illumination_mode != DynamicLightIlluminationMode::SingleBounce
        });
        for mut light in &mut dynamic_lights {
            light.config.illumination_mode = if enable_bounce {
                DynamicLightIlluminationMode::SingleBounce
            } else {
                DynamicLightIlluminationMode::DirectIllumination
            };
        }
        info!(
            "lighting test: DynamicLighting {}",
            if enable_bounce {
                "single-bounce illumination"
            } else {
                "direct-only illumination"
            }
        );
    }
    if keyboard.just_pressed(KeyCode::Digit5) {
        settings.shadow_proxies_enabled = !settings.shadow_proxies_enabled;
        info!(
            "lighting test: DynamicLighting shadow proxies {}",
            if settings.shadow_proxies_enabled {
                "enabled"
            } else {
                "disabled"
            }
        );
    }
    if keyboard.just_pressed(KeyCode::Digit6) {
        settings.volumetric_enabled = !settings.volumetric_enabled;
        info!(
            "lighting test: DynamicLighting volumetric fog {}",
            if settings.volumetric_enabled {
                "enabled"
            } else {
                "disabled"
            }
        );
    }
}

fn update_dynamic_lighting_status(
    settings: Res<DynamicLightingSettings>,
    diagnostics: Res<DynamicLightingDiagnostics>,
    bevy_lights: Query<&Visibility, (With<PointLight>, Without<DynamicLightShadowProxy>)>,
    proxies: Query<&Visibility, With<DynamicLightShadowProxy>>,
    mut text: Single<&mut Text, With<DynamicLightingStatusText>>,
) {
    let visible_bevy_lights = bevy_lights
        .iter()
        .filter(|visibility| **visibility != Visibility::Hidden)
        .count();
    let visible_proxies = proxies
        .iter()
        .filter(|visibility| **visibility != Visibility::Hidden)
        .count();
    text.0 = format!(
        "Custom extracted {} | clipped {} | pass {} | effects {} | fog {} | volumes {}\nBevy lights {} | shadow proxies {}",
        diagnostics.extracted_light_count(),
        diagnostics.truncated_light_count(),
        if settings.enabled { "ON" } else { "OFF" },
        if settings.freeze_effect_time {
            "FROZEN"
        } else {
            "RUNNING"
        },
        if settings.volumetric_enabled {
            "ON"
        } else {
            "OFF"
        },
        diagnostics.extracted_volumetric_light_count(),
        visible_bevy_lights,
        visible_proxies,
    );
}

fn toggle_shadow_sources(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut prepared_samples: ResMut<PointLightShadowSamples>,
    mut lights: Query<(&mut PointLight, &mut Visibility), With<HybridPointLight>>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        let enable = prepared_samples.0 == 0;
        prepared_samples.0 = u32::from(enable);
        info!(
            "lighting test: prepared static shadow {}",
            if prepared_samples.0 == 0 {
                "disabled"
            } else {
                "enabled"
            }
        );
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        for (mut light, _) in &mut lights {
            light.shadow_maps_enabled = !light.shadow_maps_enabled;
            info!(
                "lighting test: realtime moving-object shadow {}",
                if light.shadow_maps_enabled {
                    "enabled"
                } else {
                    "disabled"
                }
            );
        }
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        for (_, mut visibility) in &mut lights {
            *visibility = if *visibility == Visibility::Hidden {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
            info!(
                "lighting test: Bevy PointLights {}",
                if *visibility == Visibility::Hidden {
                    "disabled"
                } else {
                    "enabled"
                }
            );
        }
    }
}

fn animate_moving_shadow_caster(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut motion: ResMut<DemoMotion>,
    mut casters: Query<(&MovingShadowCaster, &mut Transform)>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        motion.paused = !motion.paused;
    }
    if motion.paused {
        return;
    }

    motion.elapsed_seconds += time.delta_secs();
    for (caster, mut transform) in &mut casters {
        transform.translation = Vec3::from_array(caster.0.position(motion.elapsed_seconds));
        transform.rotate_y(time.delta_secs() * 1.2);
    }
}

fn stop_after_trace_window(
    time: Res<Time>,
    mut timer: ResMut<DemoExitTimer>,
    mut exit: MessageWriter<AppExit>,
) {
    timer.0 -= time.delta_secs();
    if timer.0 <= 0.0 {
        exit.write(AppExit::Success);
    }
}

fn cuboid_triangles(center: Vec3, size: Vec3) -> Vec<[Vec3; 3]> {
    let half = size * 0.5;
    let vertices = [
        center + Vec3::new(-half.x, -half.y, -half.z),
        center + Vec3::new(half.x, -half.y, -half.z),
        center + Vec3::new(half.x, half.y, -half.z),
        center + Vec3::new(-half.x, half.y, -half.z),
        center + Vec3::new(-half.x, -half.y, half.z),
        center + Vec3::new(half.x, -half.y, half.z),
        center + Vec3::new(half.x, half.y, half.z),
        center + Vec3::new(-half.x, half.y, half.z),
    ];
    const INDICES: [[usize; 3]; 12] = [
        [0, 2, 1],
        [0, 3, 2],
        [4, 5, 6],
        [4, 6, 7],
        [0, 1, 5],
        [0, 5, 4],
        [3, 7, 6],
        [3, 6, 2],
        [0, 4, 7],
        [0, 7, 3],
        [1, 2, 6],
        [1, 6, 5],
    ];
    INDICES
        .into_iter()
        .map(|[a, b, c]| [vertices[a], vertices[b], vertices[c]])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn procedural_pillar_geometry_matches_the_visible_cuboid() {
        let center = Vec3::from_array(STATIC_PILLAR_CENTER);
        let size = Vec3::from_array(STATIC_PILLAR_SIZE);
        let triangles = cuboid_triangles(center, size);
        assert_eq!(triangles.len(), 12);

        let minimum = triangles
            .iter()
            .flatten()
            .copied()
            .fold(Vec3::splat(f32::INFINITY), Vec3::min);
        let maximum = triangles
            .iter()
            .flatten()
            .copied()
            .fold(Vec3::splat(f32::NEG_INFINITY), Vec3::max);
        assert!(minimum.abs_diff_eq(center - size * 0.5, 1e-6));
        assert!(maximum.abs_diff_eq(center + size * 0.5, 1e-6));
    }
}
