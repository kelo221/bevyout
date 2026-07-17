use anyhow::{Context, Result};
use bevy::asset::AssetId;
use bevy::camera::Exposure;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::gltf::GltfMeshName;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::light::{IrradianceVolume, LightProbe, PointLightShadowMap, ShadowFilteringMethod};
use bevy::math::{cubic_splines::LinearSpline, vec2};
use bevy::mesh::{Mesh, VertexAttributeValues};
use bevy::pbr::{
    BakedPointLightShadow, BakedPointShadowMap, DistanceFog, FogFalloff, PointLightShadowSamples,
};
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
    CellInfo, FO3_SCALE, ITEM_CATALOG_REVISION, ImageSpaceInfo, NIF_CONVERTER_REVISION,
    PHYSICS_ASSET_SCHEMA_VERSION, PreparedCellLighting, PreparedItemCatalog, PreparedItemCategory,
    PreparedItemDefinition, PreparedItemStats, PreparedSceneManifest, PreparedSemantic, bake,
    cell_label, ensure_baked_scene_compatible, ensure_prepared_manifest_compatible,
    find_cached_manifest, fingerprint, is_bake_static, prepare, resolve_cached_manifest,
};

/// Bevy-owned wrapper around the engine-independent prepared-scene contract.
#[derive(Resource, Clone, Debug, Deref, DerefMut)]
pub(crate) struct LoadedSceneManifest(pub(crate) PreparedSceneManifest);

mod animation;
mod audio;
mod interaction;
mod inventory;
mod openmw_player;
mod pipboy;
mod pipboy_reader;
mod player;
mod world;
mod world_items;

mod agent_bridge;
mod app;
mod bindings;
mod console;
mod console_ui;
mod controls;
mod diagnostics;
#[cfg(test)]
mod hybrid_shadow_policy;
mod lighting;
mod material_shading;
mod material_shading_policy;
mod nav;
mod nav_overlay;
mod performance_policy;
mod scene;

pub(crate) use app::run_view;
pub(crate) use controls::*;
pub(crate) use diagnostics::*;
pub(crate) use lighting::*;
pub(crate) use scene::*;

const DEFAULT_LIGHTING_SCALE: f32 = 128.0;
const CELL_DIRECTIONAL_ILLUMINANCE: f32 = 10_000.0;
const DEFAULT_FOG_STRENGTH: f32 = 0.01;
const RENDER_REPORT_HISTORY: usize = 600;
/// Runtime point-shadow cubemap face size. Prepared/baked shadow artifacts
/// keep their independent 512px cache resolution.
pub(crate) const REALTIME_POINT_SHADOW_MAP_SIZE: usize = 256;
const FPS_HUD_UPDATE_INTERVAL_SECS: f32 = 0.1;
pub(crate) const DEFAULT_HORIZONTAL_FOV_DEGREES: f32 = 90.0;
pub(crate) const DEFAULT_WINDOW_WIDTH: u32 = 1920;
pub(crate) const DEFAULT_WINDOW_HEIGHT: u32 = 1080;

pub fn view(args: ViewArgs) -> Result<()> {
    run_view(
        args.manifest,
        args.disable_physics,
        args.realtime_shadows,
        args.trace_seconds,
        args.agent_bridge.then_some(args.agent_port),
        args.save_slot,
    )
}

pub fn render(args: RenderArgs) -> Result<()> {
    let cache_dir = args
        .cache_dir
        .clone()
        .unwrap_or_else(|| PathBuf::from(".bevyout/cache"));
    let mut manifest_path = match find_cached_manifest(&cache_dir, &args.selector)? {
        Some(path) => path,
        None => {
            if args.agent_bridge {
                return Err(anyhow::anyhow!(
                    "agent bridge launch requires a cached prepared scene for '{}'; run `prepare {}` first",
                    args.selector,
                    args.selector,
                ));
            }
            let prompt = format!(
                "Prepared scene '{}' was not found. Import it now?",
                args.selector
            );
            if !confirm(&prompt)? {
                return resolve_cached_manifest(&cache_dir, &args.selector).map(|_| ());
            }
            prepare_for_render(&args, &cache_dir, false)?
        }
    };
    let mut manifest = read_manifest(&manifest_path)?;
    if next_render_cache_action(&manifest) == RenderCacheAction::Reprepare {
        let compatibility_error = ensure_prepared_manifest_compatible(
            &manifest,
            NIF_CONVERTER_REVISION,
            PHYSICS_ASSET_SCHEMA_VERSION,
        )
        .expect_err("reprepare action requires an incompatible prepared manifest");
        if args.agent_bridge {
            return Err(anyhow::anyhow!(
                "{compatibility_error}\nagent bridge launch will not reprepare interactively; run `prepare {}` first",
                args.selector,
            ));
        }
        let prompt = format!(
            "{compatibility_error}\nRefresh '{}' now using cached converted assets?",
            cell_label(&manifest.cell)
        );
        if !confirm(&prompt)? {
            return Err(compatibility_error);
        }
        manifest_path = prepare_for_render(&args, &cache_dir, true)?;
        manifest = read_manifest(&manifest_path)?;
        ensure_prepared_manifest_compatible(
            &manifest,
            NIF_CONVERTER_REVISION,
            PHYSICS_ASSET_SCHEMA_VERSION,
        )?;
    }

    if next_render_cache_action(&manifest) == RenderCacheAction::Rebake {
        let bake_error = ensure_baked_scene_compatible(&manifest).err();
        let prompt = if let Some(error) = bake_error.as_ref() {
            format!("{error}\nRe-bake '{}' now?", cell_label(&manifest.cell))
        } else {
            format!(
                "Prepared scene '{}' has no irradiance bake. Bake it now?",
                cell_label(&manifest.cell)
            )
        };
        if args.agent_bridge {
            let reason = bake_error.map_or_else(
                || {
                    format!(
                        "prepared scene '{}' has no irradiance bake",
                        cell_label(&manifest.cell)
                    )
                },
                |error| error.to_string(),
            );
            return Err(anyhow::anyhow!(
                "{reason}\nagent bridge launch requires a compatible irradiance bake; run `bake {}` first",
                args.selector,
            ));
        }
        if confirm(&prompt)? {
            bake_for_render(&args, &cache_dir)?;
        } else if let Some(error) = bake_error {
            return Err(error);
        }
    }
    run_view(
        manifest_path,
        args.disable_physics,
        args.realtime_shadows,
        args.trace_seconds,
        args.agent_bridge.then_some(args.agent_port),
        None,
    )
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
        console::GameUi,
    ));
    commands.spawn((
        Text::new("FPS --"),
        FpsText,
        console::DiagnosticUi,
        Node {
            position_type: PositionType::Absolute,
            top: px(8),
            right: px(10),
            ..default()
        },
    ));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RenderCacheAction {
    Reprepare,
    Rebake,
    Ready,
}

fn next_render_cache_action(manifest: &PreparedSceneManifest) -> RenderCacheAction {
    if ensure_prepared_manifest_compatible(
        manifest,
        NIF_CONVERTER_REVISION,
        PHYSICS_ASSET_SCHEMA_VERSION,
    )
    .is_err()
    {
        RenderCacheAction::Reprepare
    } else if needs_irradiance_bake(manifest) || ensure_baked_scene_compatible(manifest).is_err() {
        RenderCacheAction::Rebake
    } else {
        RenderCacheAction::Ready
    }
}

fn prepare_for_render(args: &RenderArgs, cache_dir: &Path, force: bool) -> Result<PathBuf> {
    prepare(PrepareArgs {
        selectors: vec![args.selector.clone()],
        all: false,
        all_interiors: false,
        worldspace: None,
        list_only: false,
        check_fingerprints: false,
        game_root: args.game_root.clone(),
        plugin: args.plugin.clone(),
        cell: None,
        blender: args.blender.clone(),
        toktx: args.toktx.clone(),
        shadow_resolution: args.shadow_resolution,
        rebuild_shadows: args.rebuild_shadows,
        cache_dir: Some(cache_dir.to_path_buf()),
        force,
        rebuild_assets: false,
        strict: false,
        jobs: None,
        retry_failed: false,
    })?;
    resolve_cached_manifest(cache_dir, &args.selector)
}

fn bake_for_render(args: &RenderArgs, cache_dir: &Path) -> Result<()> {
    bake(BakeArgs {
        manifest: None,
        selector: Some(args.selector.clone()),
        all_interiors: false,
        retry_failed: false,
        cache_dir: Some(cache_dir.to_path_buf()),
        quality: BakeQuality::Irradiance,
        irradiance_spacing_meters: 8.0,
        irradiance_samples: 64,
        static_batch_chunk_meters: 64.0,
        blender: args.blender.clone(),
        irradiance_blender: args.irradiance_blender.clone(),
        toktx: args.toktx.clone(),
        force: false,
        keep_intermediate: false,
    })
}

#[derive(Component)]
struct FpsText;

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    report: Res<RenderReportBuffer>,
    time: Res<Time>,
    mut elapsed: Local<f32>,
    mut text: Single<&mut Text, With<FpsText>>,
) {
    *elapsed += time.delta_secs();
    if *elapsed < FPS_HUD_UPDATE_INTERVAL_SECS {
        return;
    }
    // Preserve any fractional remainder so the cadence stays close to 10 Hz
    // even when the frame rate is not an exact multiple of the interval.
    *elapsed %= FPS_HUD_UPDATE_INTERVAL_SECS;
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
        .unwrap_or(0.0);
    let one_percent_low = summarize_render_samples(&report, None, RENDER_REPORT_HISTORY, 0.0)
        .p99_ms
        .filter(|frame_time_ms| *frame_time_ms > 0.0)
        .map(|frame_time_ms| 1000.0 / frame_time_ms)
        .unwrap_or(0.0);
    text.0 = format!("{fps:.0} FPS | 1% low {one_percent_low:.0} FPS");
}

#[derive(Component)]
pub(crate) struct FlyCamera {
    pub(crate) yaw: f32,
    pub(crate) pitch: f32,
    pub(crate) speed: f32,
}

#[allow(clippy::too_many_arguments)]
fn free_fly_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse: MessageReader<MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    cursor_options: Single<&CursorOptions>,
    modal: Res<State<GameplayModal>>,
    mode: Res<player::CameraModeState>,
    mut query: Query<(&mut Transform, &mut FlyCamera), With<Camera3d>>,
    time: Res<Time>,
    mut was_captured: Local<bool>,
) {
    let wheel_delta = wheel.read().map(|event| event.y).sum::<f32>();
    let captured = matches!(cursor_options.grab_mode, CursorGrabMode::Locked);
    let delta = mouse
        .read()
        .fold(Vec2::ZERO, |sum, event| sum + event.delta);
    let gameplay_active = modal.get() == &GameplayModal::None;
    if !controls::mouse_look_is_safe(&mut was_captured, captured, gameplay_active)
        || mode.mode != player::CameraMode::Free
    {
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
