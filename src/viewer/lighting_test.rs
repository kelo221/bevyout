//! Hermetic visual proof for prepared static plus realtime moving shadows.

use std::sync::Arc;

use anyhow::{Result, bail};
use bevy::camera::Exposure;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::light::{NotShadowCaster, NotShadowReceiver, PointLightShadowMap, ShadowFilteringMethod};
use bevy::pbr::{
    BakedPointLightShadow, BakedPointShadowMap, BakedPointShadowReceiver, PointLightShadowSamples,
};
use bevy::prelude::*;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use bevy::window::WindowPlugin;

use super::RenderReportBuffer;
use super::agent_bridge;
use super::lighting_demo_policy::DemoOrbit;
use crate::cli::LightingTestArgs;
use crate::vsa::{
    DynamicLight, LightEffect, STATIC_POINT_SHADOW_NEAR_Z, StaticShadowBakeLight,
    bake_static_point_shadow_bytes, update_dynamic_lights,
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
    ));
    app.insert_resource(ClearColor(Color::srgb(0.015, 0.018, 0.025)))
        .insert_resource(PointLightShadowMap { size: 256 })
        .insert_resource(PointLightShadowSamples(1))
        .insert_resource(BakedPointShadowMap {
            data: Some(Arc::from(depth.into_boxed_slice())),
            fingerprint: Some(format!("lighting-test-v1-{}", args.shadow_resolution)),
            resolution: args.shadow_resolution,
            layers: 1,
        })
        .insert_resource(RenderReportBuffer::default())
        .insert_resource(DemoMotion::default())
        .add_systems(Startup, setup_lighting_test)
        .add_systems(
            Update,
            (
                animate_moving_shadow_caster,
                toggle_shadow_sources,
                update_dynamic_lights,
                super::record_render_sample,
                super::update_fps_text,
            ),
        );
    app.add_plugins(crate::console::ConsolePlugin);

    if let Some(seconds) = args.trace_seconds {
        app.insert_resource(DemoExitTimer(seconds))
            .add_systems(Update, stop_after_trace_window);
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
) {
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.08, 0.10, 0.16),
        brightness: 90.0,
        affects_lightmapped_meshes: true,
    });

    let floor_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.58, 0.61, 0.68),
        perceptual_roughness: 0.92,
        ..default()
    });
    commands.spawn((
        Name::new("Hybrid shadow receiver floor"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(16.0, 16.0))),
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

    let strobe_intensity = 18_000.0;
    commands.spawn((
        Name::new("DynamicLighting purple strobe"),
        PointLight {
            color: Color::srgb(0.42, 0.18, 1.0),
            intensity: strobe_intensity,
            range: 9.0,
            shadow_maps_enabled: false,
            ..default()
        },
        DynamicLight::strobe(strobe_intensity, 4.0),
        Transform::from_xyz(3.5, 2.5, -2.5),
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
    for (index, effect) in LightEffect::ALL.into_iter().enumerate() {
        let column = index % 5;
        let row = index / 5;
        let position = Vec3::new(-5.5 + column as f32 * 2.75, 0.65, -5.0 + row as f32 * 2.2);
        commands.spawn((
            Name::new(format!("DynamicLighting effect {effect:?}")),
            PointLight {
                color: effect_colors[index],
                intensity: 3_200.0,
                range: 2.8,
                shadow_maps_enabled: false,
                ..default()
            },
            DynamicLight::with_effect(3_200.0, effect),
            Transform::from_translation(position),
        ));
    }

    commands.spawn((
        Name::new("Lighting test camera"),
        Camera3d::default(),
        ShadowFilteringMethod::Hardware2x2,
        Tonemapping::AcesFitted,
        Exposure { ev100: 2.0 },
        Transform::from_xyz(10.0, 7.5, 12.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        Text::new(concat!(
            "HYBRID LIGHTING TEST\n",
            "Orange pillar: static shadow baked once on CPU\n",
            "Blue block: moves and casts a realtime shadow\n",
            "Floor: combines prepared + realtime visibility\n",
            "Purple point: isolated DynamicLighting Strobe effect\n",
            "Far edge: all 15 imported intensity effects (color-coded)\n",
            "1: toggle baked static shadow | 2: toggle realtime shadow\n",
            "Space: pause / resume motion"
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

fn toggle_shadow_sources(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut prepared_samples: ResMut<PointLightShadowSamples>,
    mut lights: Query<&mut PointLight, With<HybridPointLight>>,
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
        for mut light in &mut lights {
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
