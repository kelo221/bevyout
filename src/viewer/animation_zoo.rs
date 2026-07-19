//! Isolated laboratory for cycling prepared external-KF actor animations.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use bevy::animation::{AnimatedBy, AnimationTargetId, RepeatAnimation};
use bevy::gltf::{Gltf, GltfAssetLabel, GltfNode};
use bevy::prelude::*;
use bevyout_core::actor_animation::{
    PreparedActorAnimationCatalog, PreparedActorAnimationClip, PreparedActorAnimationClipStatus,
    PreparedActorAnimationLoopMode, PreparedActorAnimationRootMotionPolicy,
    PreparedActorAnimationTextKey,
};
use ron::de::from_str;
use serde::Serialize;

use crate::cli::AnimationZooArgs;
use crate::vsa::{
    PreparedPlacement, PreparedSceneManifest, PreparedSemantic, find_cached_manifest, fingerprint,
};

use super::agent_bridge::AnimationZooAgentBridgePlugin;
use super::{LoadedSceneManifest, WorldAssetRoot};

mod policy;

use policy::{ZooControlAction, ZooPlaybackPolicy};

const FLOOR_CENTER: Vec3 = Vec3::new(0.0, -0.25, 0.0);

pub fn animation_zoo(args: AnimationZooArgs) -> Result<()> {
    let cache_dir = args
        .cache_dir
        .clone()
        .unwrap_or_else(|| PathBuf::from(".bevyout/cache"));
    let manifest_path = find_cached_manifest(&cache_dir, &args.selector)?.ok_or_else(|| {
        anyhow::anyhow!(
            "prepared scene '{}' was not found under {}; run `prepare {}` first",
            args.selector,
            cache_dir.display(),
            args.selector
        )
    })?;
    let manifest = read_manifest(&manifest_path)?;
    let actor_form_id = parse_form_id(&args.actor)?;
    let placement = select_actor(&manifest, actor_form_id)?.clone();
    let asset_root = PathBuf::from(&manifest.asset_root);
    let catalog = read_animation_catalog(&manifest, &asset_root)?;
    let mapping = catalog
        .actor_mappings
        .iter()
        .find(|mapping| mapping.reference_form_id == actor_form_id)
        .with_context(|| {
            format!("actor reference {actor_form_id:08x} has no prepared animation mapping")
        })?;
    let set = catalog
        .animation_sets
        .iter()
        .find(|set| set.id == mapping.animation_set_id)
        .context("prepared actor animation mapping references a missing set")?;
    let appearance_path = placement
        .asset_path
        .clone()
        .context("prepared actor has no appearance GLB")?;
    let clip_pack_path = set.clip_pack_asset_path.clone().with_context(|| {
        let reason = set
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .next()
            .unwrap_or("no compatible clip pack was prepared");
        format!(
            "actor animation set has no clip pack: {reason}; rerun prepare with --actor-animation-converter blender"
        )
    })?;
    require_file(&asset_root.join(&appearance_path), "actor appearance GLB")?;
    require_file(
        &asset_root.join(&clip_pack_path),
        "actor animation clip pack",
    )?;

    let compatible_clips = set
        .clips
        .iter()
        .filter(|clip| clip.status == PreparedActorAnimationClipStatus::Ready)
        .cloned()
        .collect::<Vec<_>>();
    if compatible_clips.is_empty() {
        bail!("actor animation set contains no compatible clips");
    }
    let start_index = if let Some(start) = args.start_clip.as_deref() {
        compatible_clips
            .iter()
            .position(|clip| clip.name.eq_ignore_ascii_case(start))
            .with_context(|| format!("prepared animation clip '{start}' was not found"))?
    } else {
        0
    };
    let skipped_clips = set.clips.len().saturating_sub(compatible_clips.len());
    let actor_name = placement
        .editor_id
        .clone()
        .or_else(|| placement.display_name.clone())
        .unwrap_or_else(|| format!("actor-{actor_form_id:08x}"));
    let definition = AnimationZooDefinition {
        actor_form_id,
        actor_name,
        appearance_path,
        clip_pack_path,
        root_scale: super::actor::placement_root_scale(&placement)
            .abs()
            .max(0.0001),
        clips: compatible_clips,
        start_index,
        skipped_clips,
        trace_seconds: args.trace_seconds,
    };

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                file_path: asset_root.to_string_lossy().into_owned(),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: format!(
                        "bevyout animation zoo - {} ({actor_form_id:08x})",
                        definition.actor_name
                    ),
                    ..default()
                }),
                ..default()
            }),
    );
    if args.agent_bridge {
        app.add_plugins(AnimationZooAgentBridgePlugin {
            port: args.agent_port,
        });
    }
    app.insert_resource(LoadedSceneManifest(manifest))
        .insert_resource(definition)
        .init_resource::<AnimationZooRuntime>()
        .init_resource::<AnimationZooProbe>()
        .add_systems(Startup, spawn_zoo)
        .add_systems(
            Update,
            (
                resolve_animation_zoo,
                keyboard_controls,
                drive_playback,
                update_probe,
                update_hud,
                draw_reference,
                exit_after_trace,
            )
                .chain(),
        )
        .run();
    Ok(())
}

fn read_manifest(path: &Path) -> Result<PreparedSceneManifest> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("could not read prepared scene {}", path.display()))?;
    from_str(&text).with_context(|| format!("invalid prepared scene {}", path.display()))
}

fn read_animation_catalog(
    manifest: &PreparedSceneManifest,
    asset_root: &Path,
) -> Result<PreparedActorAnimationCatalog> {
    let relative = manifest
        .actor_animation_catalog_path
        .as_deref()
        .context("prepared scene has no actor animation catalog link; rerun prepare")?;
    let path = asset_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    let bytes = fs::read(&path)
        .with_context(|| format!("could not read actor animation catalog {}", path.display()))?;
    if let Some(expected) = manifest.actor_animation_catalog_hash.as_deref()
        && fingerprint(&bytes) != expected
    {
        bail!("actor animation catalog hash does not match the scene manifest");
    }
    let text = std::str::from_utf8(&bytes).context("actor animation catalog is not UTF-8")?;
    let catalog: PreparedActorAnimationCatalog =
        from_str(text).context("actor animation catalog is invalid RON")?;
    if manifest
        .actor_animation_catalog_revision
        .as_deref()
        .is_some_and(|revision| revision != catalog.revision)
    {
        bail!("actor animation catalog revision does not match the scene manifest");
    }
    Ok(catalog)
}

fn parse_form_id(value: &str) -> Result<u32> {
    let value = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    u32::from_str_radix(value, 16).with_context(|| format!("invalid actor FormID '{value}'"))
}

fn select_actor(manifest: &PreparedSceneManifest, form_id: u32) -> Result<&PreparedPlacement> {
    let placement = manifest
        .placements
        .iter()
        .find(|placement| placement.reference_form_id == form_id)
        .with_context(|| format!("actor reference {form_id:08x} is not in the prepared scene"))?;
    if !matches!(
        placement.semantic,
        PreparedSemantic::Npc(_) | PreparedSemantic::Creature(_)
    ) {
        bail!("reference {form_id:08x} is not a prepared NPC or creature");
    }
    Ok(placement)
}

fn require_file(path: &Path, label: &str) -> Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        bail!("prepared {label} is missing: {}", path.display())
    }
}

#[derive(Resource)]
struct AnimationZooDefinition {
    actor_form_id: u32,
    actor_name: String,
    appearance_path: String,
    clip_pack_path: String,
    root_scale: f32,
    clips: Vec<PreparedActorAnimationClip>,
    start_index: usize,
    skipped_clips: usize,
    trace_seconds: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ZooPhase {
    #[default]
    Loading,
    Playing,
    Failed,
}

#[derive(Debug, Clone)]
struct ResolvedZooClip {
    catalog: PreparedActorAnimationClip,
    node: AnimationNodeIndex,
    duration: f32,
}

#[derive(Resource, Default)]
struct AnimationZooRuntime {
    phase: ZooPhase,
    actor_root: Option<Entity>,
    player: Option<Entity>,
    pack: Option<Handle<Gltf>>,
    clips: Vec<ResolvedZooClip>,
    policy: Option<ZooPlaybackPolicy>,
    bind_pose: HashMap<Entity, Transform>,
    pending_controls: Vec<ZooControlAction>,
    last_started_generation: Option<u64>,
    elapsed: f32,
    error: Option<String>,
}

#[derive(Resource, Debug, Clone, Default, Serialize)]
pub(crate) struct AnimationZooProbe {
    pub(crate) actor: String,
    pub(crate) actor_form_id: String,
    pub(crate) current_clip: Option<String>,
    pub(crate) source_kf_path: Option<String>,
    pub(crate) source_sequence_name: Option<String>,
    pub(crate) source_start_seconds: Option<f32>,
    pub(crate) source_end_seconds: Option<f32>,
    pub(crate) source_frequency: Option<f32>,
    pub(crate) source_phase: Option<f32>,
    pub(crate) source_loop_mode: Option<PreparedActorAnimationLoopMode>,
    pub(crate) root_motion_policy: Option<PreparedActorAnimationRootMotionPolicy>,
    pub(crate) accumulation_root: Option<String>,
    pub(crate) index: usize,
    pub(crate) count: usize,
    pub(crate) playback_state: String,
    pub(crate) elapsed: f32,
    pub(crate) duration: f32,
    pub(crate) speed: f32,
    pub(crate) loop_current: bool,
    pub(crate) completed_cycles: u64,
    pub(crate) missing_targets: Vec<String>,
    pub(crate) required_targets: Vec<String>,
    pub(crate) animated_targets: Vec<String>,
    pub(crate) controller_types: Vec<String>,
    pub(crate) interpolator_types: Vec<String>,
    pub(crate) text_keys: Vec<PreparedActorAnimationTextKey>,
    pub(crate) skipped_clips: usize,
    pub(crate) error: Option<String>,
}

#[derive(Component)]
struct ZooActorRoot;

#[derive(Component)]
struct ZooHud;

fn spawn_zoo(
    mut commands: Commands,
    definition: Res<AnimationZooDefinition>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut runtime: ResMut<AnimationZooRuntime>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(12.0, 0.5, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.13, 0.15),
            perceptual_roughness: 0.92,
            ..default()
        })),
        Transform::from_translation(FLOOR_CENTER),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 6_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, -0.55, 0.0)),
    ));
    commands.spawn((
        PointLight {
            intensity: 18_000.0,
            range: 12.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, 3.0),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.8, 3.0, 6.2).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
    let root = commands
        .spawn((
            WorldAssetRoot(
                asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset(definition.appearance_path.clone())),
            ),
            Transform::from_scale(Vec3::splat(definition.root_scale)),
            AnimationPlayer::default(),
            ZooActorRoot,
        ))
        .id();
    runtime.actor_root = Some(root);
    let pack = asset_server.load::<Gltf>(definition.clip_pack_path.clone());
    runtime.pack = Some(pack);
    commands.spawn((
        Text::new("Animation Zoo: loading prepared actor and clip pack..."),
        ZooHud,
        Node {
            position_type: PositionType::Absolute,
            left: px(12),
            top: px(12),
            ..default()
        },
    ));
}

fn descendants(root: Entity, children: &Query<&Children>) -> Vec<Entity> {
    let mut output = Vec::new();
    let mut pending = vec![root];
    while let Some(entity) = pending.pop() {
        output.push(entity);
        if let Ok(children) = children.get(entity) {
            pending.extend(children.iter());
        }
    }
    output
}

fn collect_pack_target_paths(
    handle: &Handle<GltfNode>,
    nodes: &Assets<GltfNode>,
    path: &mut Vec<Name>,
    output: &mut Vec<(String, AnimationTargetId, Vec<String>)>,
) -> Option<()> {
    let node = nodes.get(handle)?;
    path.push(Name::new(node.name.clone()));
    output.push((
        node.name.to_ascii_lowercase(),
        AnimationTargetId::from_names(path.iter()),
        path.iter().map(|name| name.as_str().to_owned()).collect(),
    ));
    for child in &node.children {
        collect_pack_target_paths(child, nodes, path, output)?;
    }
    path.pop();
    Some(())
}

fn appearance_name_path(
    mut entity: Entity,
    root: Entity,
    parents: &Query<&ChildOf>,
    names: &Query<&Name>,
) -> Vec<String> {
    let mut path = Vec::new();
    loop {
        if let Ok(name) = names.get(entity) {
            path.push(name.as_str().to_owned());
        }
        if entity == root {
            break;
        }
        let Ok(parent) = parents.get(entity) else {
            break;
        };
        entity = parent.parent();
    }
    path.reverse();
    path
}

fn path_has_case_insensitive_suffix(actual: &[String], expected: &[String]) -> bool {
    actual.len() >= expected.len()
        && actual[actual.len() - expected.len()..]
            .iter()
            .zip(expected)
            .all(|(left, right)| left.eq_ignore_ascii_case(right))
}

#[allow(clippy::too_many_arguments)]
fn resolve_animation_zoo(
    mut commands: Commands,
    definition: Res<AnimationZooDefinition>,
    mut runtime: ResMut<AnimationZooRuntime>,
    players: Query<Entity, With<AnimationPlayer>>,
    children: Query<&Children>,
    parents: Query<&ChildOf>,
    names: Query<&Name>,
    transforms: Query<&Transform>,
    gltfs: Res<Assets<Gltf>>,
    nodes: Res<Assets<GltfNode>>,
    clip_assets: Res<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    if runtime.phase != ZooPhase::Loading {
        return;
    }
    let Some(root) = runtime.actor_root else {
        return;
    };
    let hierarchy = descendants(root, &children);
    if hierarchy.len() <= 1 {
        return;
    }
    let player_entity = root;
    if !players.contains(player_entity) {
        return;
    }
    let Some(pack_handle) = runtime.pack.as_ref() else {
        return;
    };
    let Some(pack) = gltfs.get(pack_handle) else {
        return;
    };
    if !pack
        .named_animations
        .values()
        .all(|handle| clip_assets.get(handle).is_some())
    {
        return;
    }
    let mut target_paths = Vec::new();
    for handle in &pack.nodes {
        let Some(node) = nodes.get(handle) else {
            return;
        };
        if node.is_animation_root
            && collect_pack_target_paths(handle, &nodes, &mut Vec::new(), &mut target_paths)
                .is_none()
        {
            return;
        }
    }
    if target_paths.is_empty() {
        runtime.phase = ZooPhase::Failed;
        runtime.error = Some("clip pack contains no named animation hierarchy".into());
        return;
    }
    let mut targets_by_name = HashMap::<String, Vec<(AnimationTargetId, Vec<String>)>>::new();
    for (name, id, path) in target_paths {
        targets_by_name.entry(name).or_default().push((id, path));
    }
    let mut bound_targets = 0_usize;
    for entity in &hierarchy {
        let Ok(name) = names.get(*entity) else {
            continue;
        };
        let Some(candidates) = targets_by_name.get(&name.as_str().to_ascii_lowercase()) else {
            continue;
        };
        let appearance_path = appearance_name_path(*entity, root, &parents, &names);
        let selected = if candidates.len() == 1 {
            candidates.first()
        } else {
            candidates
                .iter()
                .find(|(_, expected)| path_has_case_insensitive_suffix(&appearance_path, expected))
        };
        if let Some((id, _)) = selected {
            commands
                .entity(*entity)
                .insert((*id, AnimatedBy(player_entity)));
            bound_targets += 1;
        }
    }
    if bound_targets == 0 {
        runtime.phase = ZooPhase::Failed;
        runtime.error =
            Some("clip pack target hierarchy does not match any prepared actor nodes".into());
        return;
    }
    let mut graph = AnimationGraph::new();
    let graph_root = graph.root;
    let mut resolved = Vec::new();
    let mut missing = Vec::new();
    for catalog_clip in &definition.clips {
        let Some(handle) = pack.named_animations.get(catalog_clip.name.as_str()) else {
            missing.push(catalog_clip.name.clone());
            continue;
        };
        let Some(clip) = clip_assets.get(handle) else {
            return;
        };
        let node = graph.add_clip(handle.clone(), 1.0, graph_root);
        resolved.push(ResolvedZooClip {
            catalog: catalog_clip.clone(),
            node,
            duration: clip.duration(),
        });
    }
    if resolved.is_empty() {
        runtime.phase = ZooPhase::Failed;
        runtime.error = Some("clip pack contains no catalog-compatible named animations".into());
        return;
    }
    let graph_handle = graphs.add(graph);
    commands.entity(player_entity).insert((
        AnimationGraphHandle(graph_handle),
        AnimationTransitions::new(),
    ));
    runtime.bind_pose = hierarchy
        .into_iter()
        .filter_map(|entity| {
            transforms
                .get(entity)
                .ok()
                .copied()
                .map(|transform| (entity, transform))
        })
        .collect();
    let start_name = &definition.clips[definition.start_index].name;
    let start_index = resolved
        .iter()
        .position(|clip| clip.catalog.name == *start_name)
        .unwrap_or(0);
    runtime.policy = Some(ZooPlaybackPolicy::new(resolved.len(), start_index));
    runtime.clips = resolved;
    runtime.player = Some(player_entity);
    runtime.phase = ZooPhase::Playing;
    if !missing.is_empty() {
        runtime.error = Some(format!(
            "clip pack omitted {} prepared name(s): {}",
            missing.len(),
            missing.join(", ")
        ));
    }
    info!(
        "animation-zoo ready actor={:08x} clips={} skipped={}",
        definition.actor_form_id,
        runtime.clips.len(),
        definition.skipped_clips + missing.len()
    );
}

fn keyboard_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut runtime: ResMut<AnimationZooRuntime>,
    mut exit: MessageWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    let controls = [
        (KeyCode::Space, ZooControlAction::TogglePause),
        (KeyCode::ArrowLeft, ZooControlAction::Previous),
        (KeyCode::ArrowRight, ZooControlAction::Next),
        (KeyCode::KeyR, ZooControlAction::Restart),
        (KeyCode::KeyL, ZooControlAction::ToggleLoop),
        (KeyCode::ArrowUp, ZooControlAction::SpeedUp),
        (KeyCode::ArrowDown, ZooControlAction::SpeedDown),
    ];
    for (key, action) in controls {
        if keys.just_pressed(key) {
            runtime.pending_controls.push(action);
        }
    }
}

fn restore_bind_pose(cache: &HashMap<Entity, Transform>, transforms: &mut Query<&mut Transform>) {
    for (entity, bind_transform) in cache {
        if let Ok(mut transform) = transforms.get_mut(*entity) {
            *transform = *bind_transform;
        }
    }
}

fn drive_playback(
    mut runtime: ResMut<AnimationZooRuntime>,
    mut players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    mut transforms: Query<&mut Transform>,
) {
    if runtime.phase != ZooPhase::Playing {
        return;
    }
    let Some(player_entity) = runtime.player else {
        return;
    };
    let Ok((mut player, mut transitions)) = players.get_mut(player_entity) else {
        return;
    };
    let controls = std::mem::take(&mut runtime.pending_controls);
    let Some(mut policy) = runtime.policy.take() else {
        return;
    };
    for action in controls {
        policy.apply(action);
    }
    if runtime.last_started_generation == Some(policy.restart_generation)
        && let Some(active) = runtime
            .clips
            .get(policy.index)
            .and_then(|clip| player.animation(clip.node))
        && active.is_finished()
    {
        policy.finished();
    }
    if runtime.last_started_generation != Some(policy.restart_generation) {
        restore_bind_pose(&runtime.bind_pose, &mut transforms);
        let clip_node = runtime.clips[policy.index].node;
        let clip_name = runtime.clips[policy.index].catalog.name.clone();
        let clip_source = runtime.clips[policy.index].catalog.source_kf_path.clone();
        let active = transitions.play(&mut player, clip_node, Duration::ZERO);
        active.set_speed(policy.speed);
        active.set_repeat(if policy.loop_current {
            RepeatAnimation::Forever
        } else {
            RepeatAnimation::Never
        });
        if policy.paused {
            active.pause();
        }
        runtime.last_started_generation = Some(policy.restart_generation);
        runtime.elapsed = 0.0;
        info!(
            "animation-zoo clip {}/{} {} source={} speed={:.2} loop={}",
            policy.index + 1,
            policy.clip_count,
            clip_name,
            clip_source,
            policy.speed,
            policy.loop_current
        );
    }
    if let Some(clip) = runtime.clips.get(policy.index)
        && let Some(active) = player.animation_mut(clip.node)
    {
        active.set_speed(policy.speed);
        active.set_repeat(if policy.loop_current {
            RepeatAnimation::Forever
        } else {
            RepeatAnimation::Never
        });
        if policy.paused && !active.is_paused() {
            active.pause();
        } else if !policy.paused && active.is_paused() {
            active.resume();
        }
        runtime.elapsed = active.seek_time();
    }
    runtime.policy = Some(policy);
}

fn update_probe(
    definition: Res<AnimationZooDefinition>,
    runtime: Res<AnimationZooRuntime>,
    mut probe: ResMut<AnimationZooProbe>,
) {
    let policy = runtime.policy.as_ref();
    let clip = policy.and_then(|policy| runtime.clips.get(policy.index));
    *probe = AnimationZooProbe {
        actor: definition.actor_name.clone(),
        actor_form_id: format!("{:08x}", definition.actor_form_id),
        current_clip: clip.map(|clip| clip.catalog.name.clone()),
        source_kf_path: clip.map(|clip| clip.catalog.source_kf_path.clone()),
        source_sequence_name: clip.and_then(|clip| clip.catalog.source_sequence_name.clone()),
        source_start_seconds: clip.and_then(|clip| clip.catalog.source_start_seconds),
        source_end_seconds: clip.and_then(|clip| clip.catalog.source_end_seconds),
        source_frequency: clip.and_then(|clip| clip.catalog.source_frequency),
        source_phase: clip.and_then(|clip| clip.catalog.source_phase),
        source_loop_mode: clip.map(|clip| clip.catalog.loop_mode),
        root_motion_policy: clip.map(|clip| clip.catalog.root_motion_policy),
        accumulation_root: clip.and_then(|clip| clip.catalog.accumulation_root.clone()),
        index: policy.map_or(0, |policy| policy.index),
        count: policy.map_or(0, |policy| policy.clip_count),
        playback_state: match runtime.phase {
            ZooPhase::Loading => "loading",
            ZooPhase::Failed => "failed",
            ZooPhase::Playing if policy.is_some_and(|policy| policy.paused) => "paused",
            ZooPhase::Playing => "playing",
        }
        .into(),
        elapsed: runtime.elapsed,
        duration: clip.map_or(0.0, |clip| clip.duration),
        speed: policy.map_or(1.0, |policy| policy.speed),
        loop_current: policy.is_some_and(|policy| policy.loop_current),
        completed_cycles: policy.map_or(0, |policy| policy.completed_cycles),
        missing_targets: clip
            .map(|clip| clip.catalog.missing_targets.clone())
            .unwrap_or_default(),
        required_targets: clip
            .map(|clip| clip.catalog.required_targets.clone())
            .unwrap_or_default(),
        animated_targets: clip
            .map(|clip| clip.catalog.animated_targets.clone())
            .unwrap_or_default(),
        controller_types: clip
            .map(|clip| clip.catalog.controller_types.clone())
            .unwrap_or_default(),
        interpolator_types: clip
            .map(|clip| clip.catalog.interpolator_types.clone())
            .unwrap_or_default(),
        text_keys: clip
            .map(|clip| clip.catalog.text_keys.clone())
            .unwrap_or_default(),
        skipped_clips: definition.skipped_clips,
        error: runtime.error.clone(),
    };
}

fn update_hud(
    probe: Res<AnimationZooProbe>,
    definition: Res<AnimationZooDefinition>,
    runtime: Res<AnimationZooRuntime>,
    mut text: Single<&mut Text, With<ZooHud>>,
) {
    let clip = runtime
        .policy
        .as_ref()
        .and_then(|policy| runtime.clips.get(policy.index));
    let diagnostics = clip
        .map(|clip| {
            clip.catalog
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.message.as_str())
                .collect::<Vec<_>>()
                .join(" | ")
        })
        .unwrap_or_default();
    text.0 = format!(
        "Animation Zoo | {} | actor {} | {}\nClip {}/{} | {} ({})\nSource: {} | range {:?}..{:?} | source loop {:?}\nDuration {:.3}s | elapsed {:.3}s | speed {:.2}x | playback loop {} | cycles {}\nRoot motion {:?} ({}) | channels {} | targets {}/{} | missing targets {} | text keys {} | skipped clips {}\nSpace pause/resume | Left/Right previous/next | R restart | L loop | Up/Down speed | Esc exit{}{}",
        definition.actor_name,
        probe.actor_form_id,
        probe.playback_state,
        if probe.count == 0 { 0 } else { probe.index + 1 },
        probe.count,
        probe.current_clip.as_deref().unwrap_or("<loading>"),
        probe.source_sequence_name.as_deref().unwrap_or("<unnamed>"),
        probe.source_kf_path.as_deref().unwrap_or("<none>"),
        probe.source_start_seconds,
        probe.source_end_seconds,
        probe.source_loop_mode,
        probe.duration,
        probe.elapsed,
        probe.speed,
        probe.loop_current,
        probe.completed_cycles,
        probe.root_motion_policy,
        probe.accumulation_root.as_deref().unwrap_or("<none>"),
        clip.map_or(0, |clip| clip.catalog.animated_channel_count),
        clip.map_or(0, |clip| clip.catalog.animated_target_count),
        probe.required_targets.len(),
        probe.missing_targets.len(),
        probe.text_keys.len(),
        probe.skipped_clips,
        if diagnostics.is_empty() {
            String::new()
        } else {
            format!("\nDiagnostics: {diagnostics}")
        },
        probe
            .error
            .as_ref()
            .map(|error| format!("\nERROR: {error}"))
            .unwrap_or_default(),
    );
}

fn draw_reference(mut gizmos: Gizmos) {
    for index in -6..=6 {
        let value = index as f32;
        let color = if index == 0 {
            Color::srgb(0.3, 0.34, 0.4)
        } else {
            Color::srgba(0.25, 0.27, 0.31, 0.55)
        };
        gizmos.line(
            Vec3::new(value, 0.002, -6.0),
            Vec3::new(value, 0.002, 6.0),
            color,
        );
        gizmos.line(
            Vec3::new(-6.0, 0.002, value),
            Vec3::new(6.0, 0.002, value),
            color,
        );
    }
}

fn exit_after_trace(
    definition: Res<AnimationZooDefinition>,
    time: Res<Time<Real>>,
    mut exit: MessageWriter<AppExit>,
) {
    if definition
        .trace_seconds
        .is_some_and(|seconds| time.elapsed_secs() >= seconds.max(0.0))
    {
        exit.write(AppExit::Success);
    }
}

pub(crate) fn queue_agent_control(world: &mut World, action: &str) -> Result<(), String> {
    let action = match action {
        "previous" => ZooControlAction::Previous,
        "next" => ZooControlAction::Next,
        "restart" => ZooControlAction::Restart,
        "toggle_pause" => ZooControlAction::TogglePause,
        "toggle_loop" => ZooControlAction::ToggleLoop,
        "speed_up" => ZooControlAction::SpeedUp,
        "speed_down" => ZooControlAction::SpeedDown,
        _ => {
            return Err(format!(
                "unknown action '{action}'; expected previous, next, restart, toggle_pause, toggle_loop, speed_up, or speed_down"
            ));
        }
    };
    let Some(mut runtime) = world.get_resource_mut::<AnimationZooRuntime>() else {
        return Err("the active viewer is not an animation zoo".into());
    };
    runtime.pending_controls.push(action);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::animation::{AnimatedBy, AnimationTargetId, animated_field};
    use bevy::ecs::system::RunSystemOnce;
    use bevy::math::curve::EaseFunction;
    use bevy::time::TimeUpdateStrategy;

    use super::*;

    #[test]
    fn form_ids_accept_plain_and_prefixed_hex() {
        assert_eq!(parse_form_id("00041606").unwrap(), 0x0004_1606);
        assert_eq!(parse_form_id("0x00041606").unwrap(), 0x0004_1606);
        assert!(parse_form_id("raider").is_err());
    }

    #[test]
    fn bridge_controls_queue_deterministic_actions() {
        let mut world = World::new();
        world.init_resource::<AnimationZooRuntime>();
        queue_agent_control(&mut world, "next").unwrap();
        assert_eq!(
            world.resource::<AnimationZooRuntime>().pending_controls,
            [ZooControlAction::Next]
        );
        assert!(queue_agent_control(&mut world, "dance").is_err());
    }

    #[test]
    fn pose_restoration_resets_actor_root_and_bones() {
        let mut world = World::new();
        let root = world.spawn(Transform::from_xyz(1.0, 2.0, 3.0)).id();
        let bone = world.spawn(Transform::from_rotation(Quat::IDENTITY)).id();
        let cache = HashMap::from([
            (root, *world.get::<Transform>(root).unwrap()),
            (bone, *world.get::<Transform>(bone).unwrap()),
        ]);
        world.get_mut::<Transform>(root).unwrap().translation = Vec3::splat(9.0);
        world.get_mut::<Transform>(bone).unwrap().rotation = Quat::from_rotation_x(1.0);
        world
            .run_system_once(move |mut transforms: Query<&mut Transform>| {
                restore_bind_pose(&cache, &mut transforms);
            })
            .unwrap();
        assert_eq!(
            world.get::<Transform>(root).unwrap().translation,
            Vec3::new(1.0, 2.0, 3.0)
        );
        assert_eq!(
            world.get::<Transform>(bone).unwrap().rotation,
            Quat::IDENTITY
        );
    }

    #[test]
    fn external_clip_targets_the_prepared_actor_hierarchy() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            TransformPlugin,
            AnimationPlugin,
        ));
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            100,
        )));
        let target_name = Name::new("Bip01 Spine");
        let target_id = AnimationTargetId::from_name(&target_name);
        let mut clip = AnimationClip::default();
        let curve = EasingCurve::new(Vec3::ZERO, Vec3::Y, EaseFunction::Linear)
            .reparametrize_linear(interval(0.0, 1.0).unwrap())
            .unwrap();
        clip.add_curve_to_target(
            target_id,
            AnimatableCurve::new(animated_field!(Transform::translation), curve),
        );
        let clip_handle = app
            .world_mut()
            .resource_mut::<Assets<AnimationClip>>()
            .add(clip);
        let (graph, node) = AnimationGraph::from_clip(clip_handle);
        let graph = app
            .world_mut()
            .resource_mut::<Assets<AnimationGraph>>()
            .add(graph);
        let mut player = AnimationPlayer::default();
        player.play(node);
        let actor = app
            .world_mut()
            .spawn((player, AnimationGraphHandle(graph), Transform::default()))
            .id();
        let bone = app
            .world_mut()
            .spawn((
                target_name,
                target_id,
                AnimatedBy(actor),
                Transform::default(),
            ))
            .id();
        app.world_mut().entity_mut(actor).add_child(bone);
        app.finish();
        for _ in 0..5 {
            app.update();
        }
        assert!(app.world().get::<Transform>(bone).unwrap().translation.y > 0.0);
    }
}
