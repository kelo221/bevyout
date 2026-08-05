use anyhow::{Context, Result};
use bevy::asset::AssetId;
use bevy::camera::Exposure;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::gltf::GltfMeshName;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::light::{IrradianceVolume, PointLightShadowMap, ShadowFilteringMethod};
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
use std::path::{Component, Path, PathBuf};

use crate::app_state::{
    AppState, GameplayModal, LoadingTarget, auto_advance_from_boot, auto_advance_from_loading,
};
use crate::cli::{ActorAnimationConverter, BakeArgs, PrepareArgs, RenderArgs, ViewArgs};
#[cfg(test)]
use crate::vsa::PREPARED_CONVERTER_REVISION;
use bevyout_core::actor_animation::{PreparedActorAnimationKind, PreparedActorAnimationSet};
pub(crate) use bevyout_core::lighting::{
    CELL_DIRECTIONAL_ILLUMINANCE, DEFAULT_LIGHTING_SCALE, point_light_intensity,
};

use crate::vsa::{
    ACTOR_ANIMATION_CATALOG_REVISION, CellInfo, FO3_SCALE, ITEM_CATALOG_REVISION, ImageSpaceInfo,
    PHYSICS_ASSET_SCHEMA_VERSION, PreparedCellLighting, PreparedItemCatalog, PreparedItemCategory,
    PreparedItemDefinition, PreparedItemStats, PreparedSceneManifest, PreparedSemantic,
    SUPPORTED_PREPARED_CONVERTER_REVISIONS, bake, cell_label, ensure_baked_scene_compatible,
    ensure_prepared_manifest_compatible_any, find_cached_manifest, fingerprint, is_bake_static,
    prepare, resolve_cached_manifest,
};

/// Bevy-owned wrapper around the engine-independent prepared-scene contract.
#[derive(Resource, Clone, Debug, Deref, DerefMut)]
pub(crate) struct LoadedSceneManifest(pub(crate) PreparedSceneManifest);

mod actor;
mod actor_animation;
pub(crate) mod actor_residency;
mod actor_state;
mod ai;
mod animation;
mod animation_zoo;
mod audio;
mod cinema;
mod interaction;
mod inventory;
mod openmw_player;
mod pipboy;
mod pipboy_reader;
mod player;
mod plugins;
mod weapon;
mod world;
mod world_items;

mod agent_bridge;
mod ao_policy;
mod app;
mod bindings;
mod console;
mod console_ui;
mod controls;
mod day_night;
mod diagnostics;
pub(crate) mod dialogue;
mod fallout_ui;
mod glow_card_policy;
mod hud;
#[cfg(test)]
mod hybrid_shadow_policy;
mod lighting;
mod material_clamp_policy;
mod nav;
mod nav_overlay;
mod pause_menu;
mod perception;
mod performance_policy;
mod ragdoll_lab;
mod realtime_shadow_policy;
mod scene;
mod screen_fx;

pub use animation_zoo::animation_zoo;
pub(crate) use app::{RunViewOptions, run_view};
pub(crate) use controls::*;
pub(crate) use diagnostics::*;
pub(crate) use lighting::*;
pub use ragdoll_lab::ragdoll_lab;
pub(crate) use scene::*;

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
        RunViewOptions {
            disable_physics: args.disable_physics,
            realtime_shadows: args.realtime_shadows,
            worldspace_lod: args.worldspace_lod,
            trace_seconds: args.trace_seconds,
            day_night_cycle_seconds: args.day_night_cycle_seconds,
            agent_port: args.agent_bridge.then_some(args.agent_port),
            unfocused: args.unfocused,
            save_slot: args.save_slot,
        },
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
                    "agent bridge launch requires a cached prepared scene for '{}'; run `{}` first",
                    args.selector,
                    actor_animation_repair_command(&args.selector),
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
    let mut cache_action = next_render_cache_action(&manifest, args.actor_animation_converter);
    if cache_action == RenderCacheAction::Reprepare {
        let compatibility_error = ensure_prepared_manifest_compatible_any(
            &manifest,
            SUPPORTED_PREPARED_CONVERTER_REVISIONS,
            PHYSICS_ASSET_SCHEMA_VERSION,
        )
        .expect_err("reprepare action requires an incompatible prepared manifest");
        if args.agent_bridge {
            return Err(anyhow::anyhow!(
                "{compatibility_error}\nagent bridge launch will not reprepare interactively; run `{}`",
                actor_animation_repair_command(&args.selector),
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
        ensure_prepared_manifest_compatible_any(
            &manifest,
            SUPPORTED_PREPARED_CONVERTER_REVISIONS,
            PHYSICS_ASSET_SCHEMA_VERSION,
        )?;
        cache_action = next_render_cache_action(&manifest, args.actor_animation_converter);
    }

    if cache_action == RenderCacheAction::RepairActorAnimations {
        let readiness = actor_animation_cache_readiness(&manifest, args.actor_animation_converter);
        if let ActorAnimationCacheReadiness::RepairRequired(reason) = readiness {
            if args.agent_bridge {
                return Err(actor_animation_bridge_error(&reason, &args.selector));
            }
            let repair_command = actor_animation_repair_command(&args.selector);
            let prompt = format!(
                "{reason}\nRepair actor animations for '{}' with native conversion now? Run `{repair_command}`",
                cell_label(&manifest.cell),
            );
            if confirm(&prompt)? {
                manifest_path = prepare_for_render_with_converter(
                    &args,
                    &cache_dir,
                    true,
                    ActorAnimationConverter::Native,
                )?;
                manifest = read_manifest(&manifest_path)?;
                ensure_prepared_manifest_compatible_any(
                    &manifest,
                    SUPPORTED_PREPARED_CONVERTER_REVISIONS,
                    PHYSICS_ASSET_SCHEMA_VERSION,
                )?;
                match actor_animation_cache_readiness(&manifest, ActorAnimationConverter::Native) {
                    ActorAnimationCacheReadiness::Ready
                    | ActorAnimationCacheReadiness::NoActors => {}
                    ActorAnimationCacheReadiness::IntentionallyDisabled => {
                        return Err(anyhow::anyhow!(
                            "actor animation repair unexpectedly disabled conversion"
                        ));
                    }
                    ActorAnimationCacheReadiness::RepairRequired(reason) => {
                        return Err(anyhow::anyhow!(
                            "actor animation repair did not produce a ready cache: {reason}"
                        ));
                    }
                }
                cache_action = next_render_cache_action(&manifest, args.actor_animation_converter);
            } else {
                eprintln!("{}", actor_animation_static_warning());
                // Refusal is an explicit opt-out for this launch, not a reason
                // to ask the same question again after the bake decision.
                cache_action = next_render_bake_action(&manifest);
            }
        } else {
            // The catalog can be replaced by another process between the
            // action check and this proof. Do not prompt when it is already
            // ready, and let the normal bake policy make the next decision.
            cache_action = next_render_cache_action(&manifest, args.actor_animation_converter);
            if cache_action == RenderCacheAction::RepairActorAnimations {
                return Err(anyhow::anyhow!(
                    "actor animation cache changed while checking readiness"
                ));
            }
        }
    }

    if args.actor_animation_converter == ActorAnimationConverter::Disabled
        && manifest_has_runtime_actor_placements(&manifest)
    {
        eprintln!(
            "warning: actor animation conversion intentionally disabled; actors may render statically"
        );
    }

    if cache_action == RenderCacheAction::Rebake {
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
    if let Some(status) = dialogue_voice_status(&manifest, &args.selector) {
        match status {
            DialogueVoiceRenderStatus::Ready(message) => eprintln!("{message}"),
            DialogueVoiceRenderStatus::TextFallback(message) => {
                eprintln!("warning: {message}")
            }
        }
    }
    run_view(
        manifest_path,
        RunViewOptions {
            disable_physics: args.disable_physics,
            realtime_shadows: args.realtime_shadows,
            worldspace_lod: args.worldspace_lod,
            trace_seconds: args.trace_seconds,
            day_night_cycle_seconds: args.day_night_cycle_seconds,
            agent_port: args.agent_bridge.then_some(args.agent_port),
            // `render` has no standalone `--unfocused` flag (issue #180 only
            // added one to `view`); an agent-bridge launch still starts
            // unfocused automatically via `run_view`'s own agent_port check.
            unfocused: false,
            save_slot: None,
        },
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum DialogueVoiceRenderStatus {
    Ready(String),
    TextFallback(String),
}

fn dialogue_voice_status(
    manifest: &PreparedSceneManifest,
    selector: &str,
) -> Option<DialogueVoiceRenderStatus> {
    let bundle = manifest.dialogue.as_ref()?;
    let asset_root = PathBuf::from(&manifest.asset_root);
    let (catalog, coverage) = match crate::vsa::dialogue::coverage::read_prepared_voice_coverage(
        &asset_root,
        bundle,
    ) {
        Ok(coverage) => coverage,
        Err(error) => {
            return Some(DialogueVoiceRenderStatus::TextFallback(format!(
                "TEXT-FALLBACK dialogue voice readiness failed for {}: {error}. Visual rendering will continue intentionally with text timing. next command: cargo run-dev -- prepare {selector}",
                cell_label(&manifest.cell),
            )));
        }
    };
    if coverage.is_ready() {
        return Some(DialogueVoiceRenderStatus::Ready(format!(
            "dialogue voice ready for {}: {}",
            cell_label(&manifest.cell),
            coverage.summary()
        )));
    }
    let missing = coverage.missing_labels().join(", ");
    let repair_guidance =
        crate::vsa::dialogue::coverage::voice_repair_guidance(selector, &catalog, &coverage);
    Some(DialogueVoiceRenderStatus::TextFallback(format!(
        "TEXT-FALLBACK dialogue voice coverage incomplete for {}: {}; missing keys=[{}]; visual rendering will continue intentionally with bounded runtime text fallback; {repair_guidance}",
        cell_label(&manifest.cell),
        coverage.summary(),
        missing,
    )))
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
        TextLayout::justify(Justify::Right),
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
    RepairActorAnimations,
    Rebake,
    Ready,
}

/// Engine-independent result of validating the prepared actor-animation cache.
///
/// This deliberately describes cache facts rather than Bevy asset state. The
/// viewer can therefore make the repair decision before creating a window or
/// scheduling any animation systems.
#[derive(Clone, Debug, PartialEq, Eq)]
enum ActorAnimationCacheReadiness {
    Ready,
    NoActors,
    IntentionallyDisabled,
    RepairRequired(String),
}

fn manifest_has_runtime_actor_placements(manifest: &PreparedSceneManifest) -> bool {
    manifest.placements.iter().any(|placement| {
        matches!(
            placement.semantic,
            PreparedSemantic::Npc(_) | PreparedSemantic::Creature(_)
        )
    })
}

fn actor_animation_catalog_repair_reason(error: &anyhow::Error) -> String {
    let message = format!("{error:#}");
    if message.contains("hash does not match") {
        "actor animation catalog hash mismatch".into()
    } else if message.contains("revision does not match") {
        "actor animation catalog revision mismatch".into()
    } else if message.contains("is stale") {
        "actor animation catalog is stale".into()
    } else if message.contains("fingerprint") {
        "actor animation catalog source fingerprint mismatch".into()
    } else if message.contains("invalid actor animation catalog") {
        "actor animation catalog is invalid".into()
    } else if message.contains("reading actor animation catalog") {
        "actor animation catalog file is missing".into()
    } else {
        "actor animation catalog could not be validated".into()
    }
}

fn resolve_actor_animation_asset_path(asset_root: &Path, relative: &str) -> Option<PathBuf> {
    let relative = PathBuf::from(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    if relative.is_absolute()
        || relative
            .components()
            .any(|component| component == Component::ParentDir)
    {
        None
    } else {
        Some(asset_root.join(relative))
    }
}

fn actor_animation_set_repair_reason(
    set: &PreparedActorAnimationSet,
    kind: PreparedActorAnimationKind,
    female: bool,
    asset_root: &Path,
) -> Option<String> {
    let Some(pack_relative_path) = set.clip_pack_asset_path.as_deref() else {
        return Some("actor animation clip pack path is missing".into());
    };
    let Some(expected_hash) = set.clip_pack_hash.as_deref() else {
        return Some("actor animation clip pack hash is missing".into());
    };
    let Some(pack_path) = resolve_actor_animation_asset_path(asset_root, pack_relative_path) else {
        return Some("actor animation clip pack path is not relative to the asset root".into());
    };
    let pack_bytes = match fs::read(&pack_path) {
        Ok(bytes) => bytes,
        Err(_) => return Some("actor animation clip pack file is missing".into()),
    };
    if fingerprint(&pack_bytes) != expected_hash {
        return Some("actor animation clip pack hash mismatch".into());
    }

    let context = actor_animation::policy::ActorAnimationContext {
        kind,
        female,
        weapon_prefix: None,
    };
    let idle = actor_animation::policy::resolve_clip(
        &set.clips,
        context,
        actor_animation::policy::ActorAnimationState::Idle,
    );
    if idle.is_none_or(|selection| {
        selection.state != actor_animation::policy::ActorAnimationState::Idle
    }) {
        return Some("actor animation set has no Ready base idle clip".into());
    }
    let forward = actor_animation::policy::resolve_clip(
        &set.clips,
        context,
        actor_animation::policy::ActorAnimationState::Walk,
    );
    if forward.is_none_or(|selection| {
        selection.state != actor_animation::policy::ActorAnimationState::Walk
    }) {
        return Some("actor animation set has no Ready forward locomotion clip".into());
    }
    None
}

fn actor_animation_cache_readiness(
    manifest: &PreparedSceneManifest,
    converter: ActorAnimationConverter,
) -> ActorAnimationCacheReadiness {
    if converter == ActorAnimationConverter::Disabled {
        return ActorAnimationCacheReadiness::IntentionallyDisabled;
    }
    if !manifest_has_runtime_actor_placements(manifest) {
        return ActorAnimationCacheReadiness::NoActors;
    }
    let Some(catalog_relative_path) = manifest.actor_animation_catalog_path.as_deref() else {
        return ActorAnimationCacheReadiness::RepairRequired(
            "actor animation catalog path is missing".into(),
        );
    };
    let asset_root = PathBuf::from(&manifest.asset_root);
    if resolve_actor_animation_asset_path(&asset_root, catalog_relative_path).is_none() {
        return ActorAnimationCacheReadiness::RepairRequired(
            "actor animation catalog path is not relative to the asset root".into(),
        );
    }
    let catalog = match actor_animation::load_catalog_for_manifest(manifest, &asset_root) {
        Ok(Some(catalog)) => catalog,
        Ok(None) => {
            return ActorAnimationCacheReadiness::RepairRequired(
                "actor animation catalog path is missing".into(),
            );
        }
        Err(error) => {
            return ActorAnimationCacheReadiness::RepairRequired(
                actor_animation_catalog_repair_reason(&error),
            );
        }
    };

    for placement in &manifest.placements {
        let (female, expected_kind) = match &placement.semantic {
            PreparedSemantic::Npc(actor) => (
                actor
                    .assembly
                    .as_ref()
                    .is_some_and(|assembly| assembly.female),
                PreparedActorAnimationKind::Npc,
            ),
            PreparedSemantic::Creature(actor) => (
                actor
                    .assembly
                    .as_ref()
                    .is_some_and(|assembly| assembly.female),
                PreparedActorAnimationKind::Creature,
            ),
            _ => continue,
        };
        let Some(mapping) = catalog.actor_mappings.iter().find(|mapping| {
            mapping.reference_form_id == placement.reference_form_id
                || (mapping.reference_form_id == 0
                    && mapping.base_form_id == placement.base_form_id)
        }) else {
            return ActorAnimationCacheReadiness::RepairRequired(
                "actor animation mapping is missing for a placed actor".into(),
            );
        };
        if mapping.kind != expected_kind {
            return ActorAnimationCacheReadiness::RepairRequired(
                "actor animation mapping kind does not match a placed actor".into(),
            );
        }
        let Some(set) = catalog
            .animation_sets
            .iter()
            .find(|set| set.id == mapping.animation_set_id)
        else {
            return ActorAnimationCacheReadiness::RepairRequired(
                "actor animation set is missing for actor animation mapping".into(),
            );
        };
        if mapping.kind == PreparedActorAnimationKind::Creature
            && !set.clips.iter().any(|clip| {
                clip.status
                    == bevyout_core::actor_animation::PreparedActorAnimationClipStatus::Ready
            })
        {
            // Creature coverage is not yet universal (for example the
            // Protectron set can be mapped but have no compatible native KF
            // clips). Do not make an otherwise repairable humanoid cache
            // unlaunchable; the actor runtime will retain its normal static
            // fallback diagnostics for this unsupported set.
            continue;
        }
        if let Some(reason) =
            actor_animation_set_repair_reason(set, mapping.kind, female, &asset_root)
        {
            return ActorAnimationCacheReadiness::RepairRequired(reason);
        }
    }
    ActorAnimationCacheReadiness::Ready
}

fn next_render_bake_action(manifest: &PreparedSceneManifest) -> RenderCacheAction {
    if needs_irradiance_bake(manifest) || ensure_baked_scene_compatible(manifest).is_err() {
        RenderCacheAction::Rebake
    } else {
        RenderCacheAction::Ready
    }
}

fn next_render_cache_action(
    manifest: &PreparedSceneManifest,
    converter: ActorAnimationConverter,
) -> RenderCacheAction {
    if ensure_prepared_manifest_compatible_any(
        manifest,
        SUPPORTED_PREPARED_CONVERTER_REVISIONS,
        PHYSICS_ASSET_SCHEMA_VERSION,
    )
    .is_err()
    {
        RenderCacheAction::Reprepare
    } else {
        match actor_animation_cache_readiness(manifest, converter) {
            ActorAnimationCacheReadiness::RepairRequired(_) => {
                RenderCacheAction::RepairActorAnimations
            }
            ActorAnimationCacheReadiness::Ready
            | ActorAnimationCacheReadiness::NoActors
            | ActorAnimationCacheReadiness::IntentionallyDisabled => {
                next_render_bake_action(manifest)
            }
        }
    }
}

fn actor_animation_repair_command(selector: &str) -> String {
    format!("cargo run-dev -- prepare {selector} --actor-animation-converter native --force")
}

fn actor_animation_static_warning() -> &'static str {
    "warning: actor animation repair declined; actors may render statically"
}

fn actor_animation_bridge_error(reason: &str, selector: &str) -> anyhow::Error {
    anyhow::anyhow!(
        "{reason}\nagent bridge cannot prompt for actor animation repair; run `{}`",
        actor_animation_repair_command(selector),
    )
}

fn prepare_args_for_render(
    args: &RenderArgs,
    cache_dir: &Path,
    force: bool,
    actor_animation_converter: ActorAnimationConverter,
) -> PrepareArgs {
    PrepareArgs {
        progress: args.progress.clone(),
        selectors: vec![args.selector.clone()],
        all: false,
        all_interiors: false,
        worldspace: None,
        list_only: false,
        check_fingerprints: false,
        game_root: args.game_root.clone(),
        plugin: args.plugin.clone(),
        cell: None,
        actor_animation_converter,
        toktx: args.toktx.clone(),
        shadow_resolution: args.shadow_resolution,
        rebuild_shadows: args.rebuild_shadows,
        rebuild_reflection_probes: args.rebuild_reflection_probes,
        cache_dir: Some(cache_dir.to_path_buf()),
        dialogue_sources: Vec::new(),
        dialogue_voice_manifests: Vec::new(),
        dialogue_voice_discover: false,
        dialogue_voice_report: None,
        force,
        rebuild_assets: false,
        strict: false,
        jobs: None,
        retry_failed: false,
    }
}

fn prepare_for_render(args: &RenderArgs, cache_dir: &Path, force: bool) -> Result<PathBuf> {
    prepare_for_render_with_converter(args, cache_dir, force, args.actor_animation_converter)
}

fn prepare_for_render_with_converter(
    args: &RenderArgs,
    cache_dir: &Path,
    force: bool,
    actor_animation_converter: ActorAnimationConverter,
) -> Result<PathBuf> {
    prepare(prepare_args_for_render(
        args,
        cache_dir,
        force,
        actor_animation_converter,
    ))?;
    resolve_cached_manifest(cache_dir, &args.selector)
}

fn bake_for_render(args: &RenderArgs, cache_dir: &Path) -> Result<()> {
    bake(BakeArgs {
        progress: args.progress.clone(),
        manifest: None,
        selector: Some(args.selector.clone()),
        all_interiors: false,
        retry_failed: false,
        cache_dir: Some(cache_dir.to_path_buf()),
        lightmap_backend: crate::cli::LightmapBackendPreference::Auto,
        lightmap_environment_map: None,
        irradiance_spacing_meters: 8.0,
        irradiance_samples: 64,
        lightmap_min_samples: 8,
        lightmap_max_samples: 8,
        lightmap_variance_threshold: 0.0,
        lightmap_bounces: 1,
        // Let the selected bake backend choose its density. The GPU path uses
        // its fast default and the bake path can lower it automatically if a
        // large primitive still exceeds the atlas page limit.
        lightmap_texels_per_meter: None,
        lightmap_density_overrides: Vec::new(),
        lightmap_debug_uv: false,
        lightmap_debug_samples: false,
        lightmap_debug_variance: false,
        lightmap_denoise_iterations: 1,
        lightmap_tile_size: None,
        lightmap_force_retrace: false,
        static_batch_chunk_meters: None,
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
