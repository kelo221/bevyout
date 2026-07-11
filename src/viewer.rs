use anyhow::{Context, Result};
use bevy::asset::AssetId;
use bevy::camera::Exposure;
use bevy::color::LinearRgba;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::gltf::GltfMeshName;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::math::{Rect, cubic_splines::LinearSpline, vec2};
use bevy::pbr::Lightmap;
use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy::post_process::auto_exposure::{
    AutoExposure, AutoExposureCompensationCurve, AutoExposurePlugin,
};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy::render::occlusion_culling::OcclusionCulling;
use bevy::render::view::ColorGrading;
use bevy::window::{CursorGrabMode, CursorOptions};
use ron::de::from_str;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::cli::ViewArgs;
use crate::vsa::{ImageSpaceInfo, PreparedSceneManifest};

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
        AutoExposurePlugin,
    ))
    .insert_resource(manifest)
    .insert_resource(UnlitMode(false))
    .insert_resource(LightingScale(128.0))
    .insert_resource(AmbientScale(0.05))
    .insert_resource(LightsDisabled(false))
    .insert_resource(LightmapOnlyMode::default())
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
            toggle_lightmap_only,
            apply_lightmap_only_mode,
            apply_baked_lightmaps,
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
    mut compensation_curves: ResMut<Assets<AutoExposureCompensationCurve>>,
    manifest: Res<PreparedSceneManifest>,
    lighting: Res<LightingScale>,
    ambient_scale: Res<AmbientScale>,
) {
    let focus = scene_focus(&manifest);
    let runtime_lighting = manifest
        .bake
        .as_ref()
        .is_none_or(|bake| bake.runtime_lighting);
    let (color_grading, auto_exposure) =
        camera_post_processing(manifest.cell.image_space.as_ref(), &mut compensation_curves);
    let mut camera = commands.spawn((
        Camera3d::default(),
        DepthPrepass,
        OcclusionCulling,
        Bloom::NATURAL,
        Tonemapping::TonyMcMapface,
        Exposure { ev100: 12.0 },
        color_grading,
        Transform::from_translation(focus + Vec3::new(0.0, 4.0, 12.0)).looking_at(focus, Vec3::Y),
        FlyCamera {
            yaw: 0.0,
            pitch: -0.15,
            speed: 8.0,
        },
    ));
    if let Some(auto_exposure) = auto_exposure {
        camera.insert(auto_exposure);
        if let Some(image_space) = manifest.cell.image_space.as_ref() {
            info!(
                "applying ImageSpace {:08x} ({}) eye_adapt_speed={:.3} target_lum={:.3}",
                image_space.form_id,
                image_space.editor_id.as_deref().unwrap_or("<unnamed>"),
                image_space.eye_adapt_speed,
                image_space.hdr_target_lum,
            );
        }
    } else {
        warn!(
            "cell {:08x} has no resolved ImageSpace; retaining fixed viewer post-processing",
            manifest.cell.form_id
        );
    }
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(
            manifest.cell.ambient_rgba[0],
            manifest.cell.ambient_rgba[1],
            manifest.cell.ambient_rgba[2],
        ),
        brightness: 25.0 * lighting.0 * ambient_scale.0,
        affects_lightmapped_meshes: runtime_lighting,
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
                affects_lightmapped_mesh_diffuse: runtime_lighting,
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_translation(Vec3::from_array(light.translation)),
        ));
    }
    if let Some(bake) = &manifest.bake {
        commands.spawn(WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(bake.scene_path.clone())),
        ));
        info!(
            "loading baked scene {} with {} lightmap pages",
            bake.scene_path,
            bake.lightmaps.len()
        );
    } else {
        for placement in &manifest.placements {
            let Some(path) = placement.asset_path.as_ref() else {
                continue;
            };
            commands.spawn((
                WorldAssetRoot(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.clone())),
                ),
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
    }
    info!(
        "loaded cell {} with {} placements, {} diagnostics; camera focus {:?}",
        manifest.cell.form_id,
        manifest.placements.len(),
        manifest.diagnostics.len(),
        focus,
    );
}

fn camera_post_processing(
    image_space: Option<&ImageSpaceInfo>,
    compensation_curves: &mut Assets<AutoExposureCompensationCurve>,
) -> (ColorGrading, Option<AutoExposure>) {
    let Some(image_space) = image_space else {
        return (ColorGrading::default(), None);
    };

    let flags = image_space.flags;
    let mut color_grading = ColorGrading::default();
    if flags & 0x08 != 0 {
        color_grading.global.exposure = image_space.brightness.max(0.0001).log2();
    }
    if flags & 0x01 != 0 {
        color_grading.global.post_saturation = image_space.cinematic_saturation.max(0.0);
    }
    if flags & 0x02 != 0 {
        let contrast = image_space.cinematic_contrast.max(0.0);
        color_grading.shadows.contrast = contrast;
        color_grading.midtones.contrast = contrast;
        color_grading.highlights.contrast = contrast;
    }
    if flags & 0x04 != 0
        && let Some((temperature, tint)) = image_space_tint_to_white_balance(
            image_space.cinematic_brightness_tint_rgb,
            image_space.cinematic_brightness_tint_value,
        )
    {
        color_grading.global.temperature = temperature;
        color_grading.global.tint = tint;
    }

    let target_lum = image_space.hdr_target_lum.max(0.001);
    let compensation = target_lum.log2();
    let compensation_curve = compensation_curves.add(
        AutoExposureCompensationCurve::from_curve(LinearSpline::new([
            vec2(-8.0, compensation),
            vec2(8.0, compensation),
        ]))
        .expect("flat auto-exposure compensation curve is valid"),
    );
    let speed = image_space_eye_adaptation_speed(image_space.eye_adapt_speed);
    let auto_exposure = AutoExposure {
        speed_brighten: speed,
        speed_darken: speed,
        compensation_curve,
        ..default()
    };
    (color_grading, Some(auto_exposure))
}

fn image_space_eye_adaptation_speed(value: f32) -> f32 {
    0.5 + (1.0 - value.clamp(0.0, 1.0)) * 7.5
}

fn image_space_tint_to_white_balance(rgb: [f32; 3], strength: f32) -> Option<(f32, f32)> {
    let [r, g, b] = rgb;
    let x = 0.412_456_4 * r + 0.357_576_1 * g + 0.180_437_5 * b;
    let y = 0.212_672_9 * r + 0.715_152_2 * g + 0.072_175 * b;
    let z = 0.019_333_9 * r + 0.119_192 * g + 0.950_304_1 * b;
    let sum = x + y + z;
    if sum <= f32::EPSILON || !sum.is_finite() {
        return None;
    }
    let target_x = x / sum;
    let target_y = y / sum;
    let strength = strength.max(0.0);
    Some((
        (0.3127 - target_x) * strength,
        (target_y - 0.3290) * strength,
    ))
}

type BakedMeshQuery<'w> = (
    Entity,
    &'w GltfMeshName,
    &'w MeshMaterial3d<StandardMaterial>,
    Option<&'w Lightmap>,
);

fn apply_baked_lightmaps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    manifest: Res<PreparedSceneManifest>,
    meshes: Query<BakedMeshQuery<'_>, With<Mesh3d>>,
    mut reported: Local<bool>,
) {
    let Some(bake) = manifest.bake.as_ref() else {
        return;
    };
    if !*reported {
        let count = meshes.iter().count();
        let matching = meshes
            .iter()
            .filter(|(_, name, _, _)| {
                bake.bindings
                    .iter()
                    .any(|binding| mesh_name_matches(&name.0, &binding.mesh_name))
            })
            .count();
        if count > 0 {
            let names = meshes
                .iter()
                .map(|(_, name, _, _)| name.0.as_str())
                .take(8)
                .collect::<Vec<_>>();
            if matching == 0 {
                warn!(
                    "baked scene spawned {count} mesh entities, but none match lightmap bindings; first names: {names:?}"
                );
            } else {
                info!(
                    "baked scene spawned {count} mesh entities; {matching} match lightmap bindings"
                );
            }
            *reported = true;
        }
    }
    for (entity, name, material_handle, existing_lightmap) in &meshes {
        if existing_lightmap.is_some() {
            continue;
        }
        let Some(binding) = bake
            .bindings
            .iter()
            .find(|binding| mesh_name_matches(&name.0, &binding.mesh_name))
        else {
            continue;
        };
        let Some(page) = bake.lightmaps.get(binding.page) else {
            warn!(
                "lightmap binding {} refers to missing page {}",
                name.0, binding.page
            );
            continue;
        };
        if let Some(mut material) = materials.get_mut(material_handle) {
            material.lightmap_exposure = bake.lightmap_exposure;
        }
        commands.entity(entity).insert(Lightmap {
            image: asset_server.load(page.asset_path.clone()),
            uv_rect: Rect {
                min: Vec2::new(binding.uv_rect[0], binding.uv_rect[1]),
                max: Vec2::new(binding.uv_rect[2], binding.uv_rect[3]),
            },
            bicubic_sampling: false,
        });
    }
}

fn mesh_name_matches(actual: &str, expected: &str) -> bool {
    actual == expected
        || actual
            .strip_prefix(expected)
            .is_some_and(|suffix| suffix.starts_with(':'))
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

#[derive(Resource, Default)]
struct LightmapOnlyMode {
    enabled: bool,
    originals: HashMap<AssetId<StandardMaterial>, StandardMaterial>,
}

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
    lightmap_only: Res<LightmapOnlyMode>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut points: Query<&mut PointLight>,
) {
    if !lighting.is_changed()
        && !ambient_scale.is_changed()
        && !disabled.is_changed()
        && !lightmap_only.is_changed()
    {
        return;
    }
    if lightmap_only.enabled {
        ambient.brightness = 0.0;
        for mut light in &mut points {
            light.intensity = 0.0;
        }
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

fn toggle_lightmap_only(
    keys: Res<ButtonInput<KeyCode>>,
    manifest: Res<PreparedSceneManifest>,
    mut mode: ResMut<LightmapOnlyMode>,
) {
    if keys.just_pressed(KeyCode::KeyL) {
        if manifest.bake.is_none() {
            warn!(
                "lightmap-only mode unavailable: this manifest has no completed baked lightmap; run bake --quality quick or final first"
            );
            return;
        }
        mode.enabled = !mode.enabled;
        info!(
            "lightmap-only diagnostic mode: {}",
            if mode.enabled { "on" } else { "off" }
        );
    }
}

fn apply_lightmap_only_mode(
    mut mode: ResMut<LightmapOnlyMode>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if mode.enabled {
        for (id, material) in materials.iter_mut() {
            let original = mode.originals.entry(id).or_insert_with(|| material.clone());
            material.base_color = Color::WHITE;
            material.base_color_texture = None;
            material.emissive = LinearRgba::BLACK;
            material.emissive_texture = None;
            material.metallic = 0.0;
            material.metallic_roughness_texture = None;
            material.reflectance = 0.0;
            material.normal_map_texture = None;
            material.occlusion_texture = None;
            material.unlit = false;
            material.lightmap_exposure = original.lightmap_exposure;
        }
    } else if mode.is_changed() {
        for (id, original) in mode.originals.drain() {
            if let Some(mut material) = materials.get_mut(id) {
                *material = original;
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_space_eye_adaptation_speed_uses_documented_endpoints() {
        assert!((image_space_eye_adaptation_speed(0.0) - 8.0).abs() < f32::EPSILON);
        assert!((image_space_eye_adaptation_speed(1.0) - 0.5).abs() < f32::EPSILON);
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
        assert!((auto_exposure.speed_brighten - 6.125).abs() < f32::EPSILON);
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
}
