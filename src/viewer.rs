use anyhow::{Context, Result};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
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
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        file_path: asset_root.to_string_lossy().to_string(),
        ..default()
    }))
    .insert_resource(manifest)
    .add_systems(Startup, (capture_cursor, spawn_prepared_scene))
    .add_systems(Update, free_fly_camera)
    .run();
    Ok(())
}

fn spawn_prepared_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    manifest: Res<PreparedSceneManifest>,
) {
    let focus = scene_focus(&manifest);
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(focus + Vec3::new(0.0, 4.0, 12.0)).looking_at(focus, Vec3::Y),
        FlyCamera {
            yaw: 0.0,
            pitch: -0.15,
        },
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 7000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((AmbientLight {
        color: Color::srgb(
            manifest.cell.ambient_rgba[0],
            manifest.cell.ambient_rgba[1],
            manifest.cell.ambient_rgba[2],
        ),
        brightness: 250.0,
        affects_lightmapped_meshes: true,
    },));
    for light in &manifest.lights {
        commands.spawn((
            PointLight {
                intensity: 450.0,
                range: light.radius,
                color: Color::srgb(
                    light.color_rgba[0],
                    light.color_rgba[1],
                    light.color_rgba[2],
                ),
                shadow_maps_enabled: true,
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

#[derive(Component)]
struct FlyCamera {
    yaw: f32,
    pitch: f32,
}

fn free_fly_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse: MessageReader<MouseMotion>,
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
    if keys.pressed(KeyCode::KeyQ) {
        direction -= Vec3::Y;
    }
    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * 8.0 * time.delta_secs();
    }
}
