use anyhow::{Context, Result};
use bevy::asset::AssetId;
use bevy::camera::Exposure;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::system::SystemParam;
use bevy::gltf::GltfMeshName;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::light::{IrradianceVolume, LightProbe};
use bevy::math::{cubic_splines::LinearSpline, vec2};
use bevy::mesh::{Mesh, VertexAttributeValues};
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy::post_process::auto_exposure::{
    AutoExposure, AutoExposureCompensationCurve, AutoExposurePlugin,
};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use bevy::render::occlusion_culling::OcclusionCulling;
use bevy::render::view::ColorGrading;
use bevy::window::{CursorGrabMode, CursorOptions, PresentMode};
use ron::de::from_str;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

use crate::app_state::{
    AppState, AppStatePlugin, GameplayModal, LoadingTarget, auto_advance_from_boot,
    auto_advance_from_loading,
};
use crate::cli::{BakeArgs, BakeQuality, PrepareArgs, RenderArgs, ViewArgs};
use crate::vsa::{
    CellInfo, FO3_SCALE, ImageSpaceInfo, NIF_CONVERTER_REVISION, PHYSICS_ASSET_SCHEMA_VERSION,
    PreparedCellLighting, PreparedSceneManifest, PreparedSemantic, bake, cell_label,
    ensure_baked_scene_compatible, ensure_prepared_manifest_compatible, find_cached_manifest,
    is_bake_static, prepare, resolve_cached_manifest,
};

mod audio;
mod interaction;
mod openmw_player;
mod player;

mod app;
mod controls;
mod diagnostics;
mod scene;

pub(crate) use app::run_view;
pub(crate) use controls::*;
pub(crate) use diagnostics::*;
pub(crate) use scene::*;

const DEFAULT_LIGHTING_SCALE: f32 = 128.0;
const CELL_DIRECTIONAL_ILLUMINANCE: f32 = 10_000.0;
const DEFAULT_FOG_STRENGTH: f32 = 0.01;
const RENDER_REPORT_HISTORY: usize = 600;

pub fn view(args: ViewArgs) -> Result<()> {
    run_view(args.manifest, args.disable_physics, args.trace_seconds)
}

pub fn render(args: RenderArgs) -> Result<()> {
    let cache_dir = args
        .cache_dir
        .unwrap_or_else(|| PathBuf::from(".bevyout/cache"));
    let manifest_path = match find_cached_manifest(&cache_dir, &args.selector)? {
        Some(path) => path,
        None => {
            let prompt = format!(
                "Prepared scene '{}' was not found. Import it now?",
                args.selector
            );
            if !confirm(&prompt)? {
                return resolve_cached_manifest(&cache_dir, &args.selector).map(|_| ());
            }
            prepare(PrepareArgs {
                selector: Some(args.selector.clone()),
                game_root: args.game_root.clone(),
                plugin: args.plugin.clone(),
                cell: None,
                blender: args.blender.clone(),
                cache_dir: Some(cache_dir.clone()),
                force: false,
                rebuild_assets: false,
                strict: false,
            })?;
            resolve_cached_manifest(&cache_dir, &args.selector)?
        }
    };
    let manifest = read_manifest(&manifest_path)?;
    if needs_irradiance_bake(&manifest) {
        let prompt = format!(
            "Prepared scene '{}' has no irradiance bake. Bake it now?",
            cell_label(&manifest.cell)
        );
        if confirm(&prompt)? {
            bake(BakeArgs {
                manifest: None,
                selector: Some(args.selector.clone()),
                cache_dir: Some(cache_dir.clone()),
                quality: BakeQuality::Irradiance,
                irradiance_spacing_meters: 8.0,
                irradiance_samples: 64,
                static_batch_chunk_meters: 64.0,
                blender: args.blender.clone(),
                irradiance_blender: args.irradiance_blender.clone(),
                toktx: args.toktx.clone(),
                force: false,
                keep_intermediate: false,
            })?;
        }
    }
    run_view(manifest_path, args.disable_physics, args.trace_seconds)
}

fn read_manifest(manifest_path: &Path) -> Result<PreparedSceneManifest> {
    let manifest_path = fs::canonicalize(manifest_path).context("manifest does not exist")?;
    let text = fs::read_to_string(manifest_path)?;
    from_str(&text).context("invalid scene manifest")
}

fn needs_irradiance_bake(manifest: &PreparedSceneManifest) -> bool {
    manifest
        .bake
        .as_ref()
        .and_then(|bake| bake.irradiance_volume.as_ref())
        .is_none()
}

fn confirm(prompt: &str) -> Result<bool> {
    use std::io::{self, IsTerminal, Write};

    let stdin = io::stdin();
    if !stdin.is_terminal() {
        return Ok(false);
    }
    let mut answer = String::new();
    loop {
        eprint!("{prompt} [Y/n] ");
        io::stderr().flush()?;
        answer.clear();
        if stdin.read_line(&mut answer)? == 0 {
            return Ok(false);
        }
        match answer.trim().to_ascii_lowercase().as_str() {
            "" | "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => eprintln!("Please answer yes or no."),
        }
    }
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
    commands.spawn((
        Text::new("Adjusting: Lighting scale\nPage Up/Down: select   F1/F2: change"),
        AdjustmentHud,
        Node {
            position_type: PositionType::Absolute,
            top: px(8),
            left: px(10),
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

#[derive(Component)]
pub(crate) struct FlyCamera {
    pub(crate) yaw: f32,
    pub(crate) pitch: f32,
    pub(crate) speed: f32,
}

fn free_fly_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse: MessageReader<MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    cursor_options: Single<&CursorOptions>,
    mode: Res<player::CameraModeState>,
    mut query: Query<(&mut Transform, &mut FlyCamera), With<Camera3d>>,
    time: Res<Time>,
) {
    let wheel_delta = wheel.read().map(|event| event.y).sum::<f32>();
    let captured = matches!(cursor_options.grab_mode, CursorGrabMode::Locked);
    let delta = mouse
        .read()
        .fold(Vec2::ZERO, |sum, event| sum + event.delta);
    if mode.mode != player::CameraMode::Free || !captured {
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
#[path = "tests/mod.rs"]
mod tests;
