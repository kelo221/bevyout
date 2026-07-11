use anyhow::{Context, Result};
use bevy::camera::Exposure;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::gltf::GltfMeshName;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy::render::occlusion_culling::OcclusionCulling;
use bevy::window::{CursorGrabMode, CursorOptions};
use ron::de::from_str;
use std::fs;
use std::path::PathBuf;

use crate::cli::ViewArgs;
use crate::vsa::PreparedSceneManifest;

pub(crate) fn view(args: ViewArgs) -> Result<()> {
    let manifest_path = fs::canonicalize(&args.manifest).context("manifest does not exist")?;
    let text = fs::read_to_string(&manifest_path)?;
    let manifest: PreparedSceneManifest = from_str(&text).context("invalid scene manifest")?;
    let asset_root = PathBuf::from(&manifest.asset_root);
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(AssetPlugin {
            file_path: asset_root.to_string_lossy().to_string(),
            ..default()
        }),
        FrameTimeDiagnosticsPlugin::default(),
    ))
    .insert_resource(manifest)
    .insert_resource(UnlitMode(false))
    .insert_resource(LightingScale(8192.0))
    .insert_resource(AmbientScale(0.05))
    .insert_resource(LightsDisabled(false))
    .add_systems(
        Startup,
        (capture_cursor, spawn_prepared_scene, spawn_reticle),
    )
    .add_systems(
        Update,
        (
            adjust_lighting,
            adjust_ambient,
            adjust_bloom,
            toggle_lights_disabled,
            apply_lighting_scale,
            update_fps_text,
            toggle_unlit_mode,
            apply_unlit_mode,
            inspect_center_hit,
            free_fly_camera,
        ),
    )
    .run();
    Ok(())
}

fn spawn_reticle(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Px(5.0),
            height: Val::Px(5.0),
            margin: UiRect::all(Val::Px(-2.5)),
            ..default()
        },
        BackgroundColor(Color::WHITE),
        ZIndex(100),
    ));
    commands.spawn((
        Text::new("FPS --"),
        FpsText,
        Node {
            position_type: PositionType::Absolute,
            top: px(8),
            right: px(10),
            ..default()
        },
    ));
}

#[derive(Component)]
struct FpsText;

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut text: Single<&mut Text, With<FpsText>>) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
        .unwrap_or(0.0);
    text.0 = format!("{fps:.0} FPS");
}

fn inspect_center_hit(
    keys: Res<ButtonInput<KeyCode>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut raycast: MeshRayCast,
    names: Query<&GltfMeshName>,
) {
    if !keys.just_pressed(KeyCode::Enter) {
        return;
    }
    let Ok((camera, transform)) = cameras.single() else {
        return;
    };
    let Some(viewport) = camera.logical_viewport_size() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(transform, viewport * 0.5) else {
        return;
    };
    let settings = MeshRayCastSettings {
        visibility: RayCastVisibility::VisibleInView,
        ..default()
    };
    if let Some((entity, hit)) = raycast.cast_ray(ray, &settings).first() {
        let name = names
            .get(*entity)
            .map(|name| name.0.as_str())
            .unwrap_or("unnamed");
        info!(
            "center hit: entity {entity:?}, mesh {name:?}, distance {:.2}",
            hit.distance
        );
    } else {
        info!("center hit: none");
    }
}

fn spawn_prepared_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    manifest: Res<PreparedSceneManifest>,
    lighting: Res<LightingScale>,
    ambient_scale: Res<AmbientScale>,
) {
    let focus = scene_focus(&manifest);
    commands.spawn((
        Camera3d::default(),
        DepthPrepass,
        OcclusionCulling,
        Bloom::NATURAL,
        Tonemapping::TonyMcMapface,
        Exposure { ev100: 12.0 },
        Transform::from_translation(focus + Vec3::new(0.0, 4.0, 12.0)).looking_at(focus, Vec3::Y),
        FlyCamera {
            yaw: 0.0,
            pitch: -0.15,
            speed: 8.0,
        },
    ));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(
            manifest.cell.ambient_rgba[0],
            manifest.cell.ambient_rgba[1],
            manifest.cell.ambient_rgba[2],
        ),
        brightness: 25.0 * lighting.0 * ambient_scale.0,
        affects_lightmapped_meshes: true,
    });
    for light in &manifest.lights {
        commands.spawn((
            PointLight {
                intensity: light.radius * light.radius * 2.0 * lighting.0,
                range: light.radius,
                color: Color::srgb(
                    light.color_rgba[0],
                    light.color_rgba[1],
                    light.color_rgba[2],
                ),
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_translation(Vec3::from_array(light.translation)),
        ));
    }
    for placement in &manifest.placements {
        let Some(path) = placement.asset_path.as_ref() else {
            continue;
        };
        commands.spawn((
            WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.clone()))),
            Transform {
                translation: Vec3::from_array(placement.translation),
                rotation: Quat::from_xyzw(
                    placement.rotation_xyzw[0],
                    placement.rotation_xyzw[1],
                    placement.rotation_xyzw[2],
                    placement.rotation_xyzw[3],
                ),
                scale: Vec3::splat(placement.scale),
            },
        ));
    }
    info!(
        "loaded cell {} with {} placements, {} diagnostics; camera focus {:?}",
        manifest.cell.form_id,
        manifest.placements.len(),
        manifest.diagnostics.len(),
        focus,
    );
}

fn scene_focus(manifest: &PreparedSceneManifest) -> Vec3 {
    let mut minimum = Vec3::splat(f32::INFINITY);
    let mut maximum = Vec3::splat(f32::NEG_INFINITY);
    let mut found = false;
    for placement in &manifest.placements {
        if placement.asset_path.is_none() {
            continue;
        }
        let position = Vec3::from_array(placement.translation);
        minimum = minimum.min(position);
        maximum = maximum.max(position);
        found = true;
    }
    if found {
        (minimum + maximum) * 0.5
    } else {
        Vec3::ZERO
    }
}

fn capture_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

#[derive(Resource)]
struct UnlitMode(bool);

#[derive(Resource)]
struct LightingScale(f32);

#[derive(Resource)]
struct AmbientScale(f32);

#[derive(Resource)]
struct LightsDisabled(bool);

fn adjust_lighting(keys: Res<ButtonInput<KeyCode>>, mut lighting: ResMut<LightingScale>) {
    let previous = lighting.0;
    if keys.just_pressed(KeyCode::F1) {
        lighting.0 = (lighting.0 * 0.5).max(0.0001);
    } else if keys.just_pressed(KeyCode::F2) {
        lighting.0 = (lighting.0 * 2.0).min(262_144.0);
    }
    if lighting.0 != previous {
        info!("lighting scale: {:.4}", lighting.0);
    }
}

fn adjust_ambient(keys: Res<ButtonInput<KeyCode>>, mut ambient: ResMut<AmbientScale>) {
    let previous = ambient.0;
    if keys.just_pressed(KeyCode::F4) {
        ambient.0 = (ambient.0 * 0.5).max(0.0001);
    } else if keys.just_pressed(KeyCode::F5) {
        ambient.0 = (ambient.0 * 2.0).min(4096.0);
    }
    if ambient.0 != previous {
        info!("ambient scale: {:.4}", ambient.0);
    }
}

fn adjust_bloom(keys: Res<ButtonInput<KeyCode>>, mut cameras: Query<&mut Bloom, With<Camera3d>>) {
    let Ok(mut bloom) = cameras.single_mut() else {
        return;
    };
    let mut changed = false;
    if keys.just_pressed(KeyCode::F6) {
        bloom.intensity = (bloom.intensity * 0.5).max(0.0);
        changed = true;
    } else if keys.just_pressed(KeyCode::F7) {
        bloom.intensity = (bloom.intensity * 2.0).min(1.0);
        changed = true;
    } else if keys.just_pressed(KeyCode::F8) {
        bloom.prefilter.threshold = (bloom.prefilter.threshold - 0.1).max(0.0);
        changed = true;
    } else if keys.just_pressed(KeyCode::F9) {
        bloom.prefilter.threshold += 0.1;
        changed = true;
    } else if keys.just_pressed(KeyCode::F10) {
        bloom.prefilter.threshold_softness =
            (bloom.prefilter.threshold_softness - 0.1).clamp(0.0, 1.0);
        changed = true;
    } else if keys.just_pressed(KeyCode::F11) {
        bloom.prefilter.threshold_softness =
            (bloom.prefilter.threshold_softness + 0.1).clamp(0.0, 1.0);
        changed = true;
    }
    if changed {
        info!(
            "bloom: intensity {:.2}, threshold {:.2}, softness {:.2}",
            bloom.intensity, bloom.prefilter.threshold, bloom.prefilter.threshold_softness
        );
    }
}

fn toggle_lights_disabled(keys: Res<ButtonInput<KeyCode>>, mut disabled: ResMut<LightsDisabled>) {
    if keys.just_pressed(KeyCode::F3) {
        disabled.0 = !disabled.0;
        info!(
            "all runtime lights: {}",
            if disabled.0 { "off" } else { "on" }
        );
    }
}

fn apply_lighting_scale(
    lighting: Res<LightingScale>,
    ambient_scale: Res<AmbientScale>,
    disabled: Res<LightsDisabled>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut points: Query<&mut PointLight>,
) {
    if !lighting.is_changed() && !ambient_scale.is_changed() && !disabled.is_changed() {
        return;
    }
    ambient.brightness = if disabled.0 {
        0.0
    } else {
        25.0 * lighting.0 * ambient_scale.0
    };
    for mut light in &mut points {
        light.intensity = if disabled.0 {
            0.0
        } else {
            light.range * light.range * 2.0 * lighting.0
        };
    }
}

fn toggle_unlit_mode(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<UnlitMode>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        mode.0 = !mode.0;
        info!(
            "unlit diagnostic mode: {}",
            if mode.0 { "on" } else { "off" }
        );
    }
}

fn apply_unlit_mode(mode: Res<UnlitMode>, mut materials: ResMut<Assets<StandardMaterial>>) {
    for material in materials.iter_mut().map(|(_, material)| material) {
        material.unlit = mode.0;
    }
}

#[derive(Component)]
struct FlyCamera {
    yaw: f32,
    pitch: f32,
    speed: f32,
}

fn free_fly_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse: MessageReader<MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    mut cursor_options: Single<&mut CursorOptions>,
    mut query: Query<(&mut Transform, &mut FlyCamera), With<Camera3d>>,
    time: Res<Time>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    } else if mouse_buttons.just_pressed(MouseButton::Left) {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }
    let wheel_delta = wheel.read().map(|event| event.y).sum::<f32>();
    let captured = matches!(cursor_options.grab_mode, CursorGrabMode::Locked);
    let delta = mouse
        .read()
        .fold(Vec2::ZERO, |sum, event| sum + event.delta);
    if !captured {
        return;
    }
    let Ok((mut transform, mut camera)) = query.single_mut() else {
        return;
    };
    if wheel_delta != 0.0 {
        camera.speed = (camera.speed * 1.2_f32.powf(wheel_delta)).clamp(0.25, 256.0);
        info!("camera speed: {:.2}", camera.speed);
    }
    camera.yaw -= delta.x * 0.002;
    camera.pitch = (camera.pitch - delta.y * 0.002).clamp(-1.5, 1.5);
    transform.rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    let mut direction = Vec3::ZERO;
    let forward = transform.forward();
    let right = transform.right();
    if keys.pressed(KeyCode::KeyW) {
        direction += *forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction -= *forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += *right;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction -= *right;
    }
    if keys.pressed(KeyCode::KeyE) {
        direction += Vec3::Y;
    }
    if keys.pressed(KeyCode::KeyZ) {
        direction -= Vec3::Y;
    }
    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * camera.speed * time.delta_secs();
    }
}
