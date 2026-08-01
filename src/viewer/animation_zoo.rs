//! Isolated laboratory for cycling prepared external-KF actor animations.

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use bevy::animation::{AnimatedBy, AnimationTargetId, RepeatAnimation, animated_field};
use bevy::app::AnimationSystems;
use bevy::camera::primitives::Aabb;
use bevy::gltf::{Gltf, GltfAssetLabel, GltfNode};
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::ui_widgets::ScrollArea;
use bevy::window::PrimaryWindow;
use bevyout_core::actor_animation::{
    PreparedActorAnimationCatalog, PreparedActorAnimationClip, PreparedActorAnimationClipStatus,
    PreparedActorAnimationLoopMode, PreparedActorAnimationRootMotionPolicy,
    PreparedActorAnimationTextKey,
};
use ron::de::from_str;
use serde::Serialize;

use crate::cli::AnimationZooArgs;
use crate::vsa::{
    ACTOR_ANIMATION_NATIVE_CONVERTER_REVISION, PreparedPlacement, PreparedSceneManifest,
    PreparedSemantic, find_cached_manifest, fingerprint,
};

use super::agent_bridge::AnimationZooAgentBridgePlugin;
use super::{LoadedSceneManifest, WorldAssetRoot};

mod policy;

use policy::{ZooControlAction, ZooPlaybackPolicy};

const FLOOR_CENTER: Vec3 = Vec3::new(0.0, -0.25, 0.0);
const ZOO_CAMERA_TARGET: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const ZOO_PANEL_WIDTH: f32 = 520.0;

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
            "actor animation set has no clip pack: {reason}; rerun prepare with --actor-animation-converter native"
        )
    })?;
    require_file(&asset_root.join(&appearance_path), "actor appearance GLB")?;
    require_file(
        &asset_root.join(&clip_pack_path),
        "actor animation clip pack",
    )?;
    validate_clip_pack_revision(&asset_root, &clip_pack_path)?;

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
        .init_resource::<AnimationZooViewState>()
        .init_resource::<AnimationZooProbe>()
        .add_systems(Startup, spawn_zoo)
        .add_systems(
            Update,
            (
                resolve_animation_zoo,
                zoo_ui_interactions,
                keyboard_controls,
                zoo_view_controls,
                drive_playback,
                apply_zoo_view,
                update_probe,
            )
                .chain(),
        )
        .add_systems(Update, update_zoo_ui)
        .add_systems(Update, draw_reference)
        .add_systems(Update, exit_after_trace)
        .add_systems(
            PostUpdate,
            retarget_animation_zoo
                .after(AnimationSystems)
                .before(TransformSystems::Propagate),
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

fn validate_clip_pack_revision(asset_root: &Path, clip_pack_path: &str) -> Result<()> {
    let report_path = asset_root.join(clip_pack_path).with_extension("json");
    let Ok(report) = fs::read_to_string(&report_path) else {
        return Ok(());
    };
    let Ok(report) = serde_json::from_str::<serde_json::Value>(&report) else {
        return Ok(());
    };
    let Some(revision) = report.get("revision").and_then(serde_json::Value::as_str) else {
        return Ok(());
    };
    if revision.starts_with("nifty-native-kf-clip-pack-")
        && revision != ACTOR_ANIMATION_NATIVE_CONVERTER_REVISION
    {
        bail!(
            "native actor animation pack revision '{revision}' is stale; rerun prepare with --actor-animation-converter native --rebuild-assets"
        );
    }
    Ok(())
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
    handle: Handle<AnimationClip>,
    node: AnimationNodeIndex,
    duration: f32,
    base_clip_index: Option<usize>,
}

#[derive(Clone)]
struct PackTarget {
    name_key: String,
    id: AnimationTargetId,
    path: Vec<String>,
    rest_local: Transform,
    rest_global: Mat4,
    parent: Option<usize>,
}

#[derive(Clone, Copy)]
struct ZooRetargetTarget {
    entity: Entity,
    source_index: usize,
    target_rest_global: Mat4,
}

#[derive(Resource, Default)]
struct AnimationZooRuntime {
    phase: ZooPhase,
    actor_root: Option<Entity>,
    asset_root: Option<Entity>,
    player: Option<Entity>,
    pack: Option<Handle<Gltf>>,
    pack_targets: Vec<PackTarget>,
    retarget_targets: Vec<ZooRetargetTarget>,
    bound_targets: usize,
    clips: Vec<ResolvedZooClip>,
    policy: Option<ZooPlaybackPolicy>,
    bind_pose: HashMap<Entity, Transform>,
    pending_controls: Vec<ZooControlAction>,
    last_started_generation: Option<u64>,
    elapsed: f32,
    ground_offset: f32,
    event_log: VecDeque<String>,
    error: Option<String>,
}

#[derive(Resource, Debug, Clone)]
struct AnimationZooViewState {
    camera_yaw: f32,
    camera_pitch: f32,
    camera_distance: f32,
    actor_yaw: f32,
    debug_visible: bool,
}

impl Default for AnimationZooViewState {
    fn default() -> Self {
        Self {
            camera_yaw: 0.66,
            camera_pitch: 0.24,
            camera_distance: 8.2,
            actor_yaw: 0.0,
            debug_visible: false,
        }
    }
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
    pub(crate) auto_advance: bool,
    pub(crate) completed_cycles: u64,
    pub(crate) missing_targets: Vec<String>,
    pub(crate) bound_targets: usize,
    pub(crate) required_targets: Vec<String>,
    pub(crate) animated_targets: Vec<String>,
    pub(crate) controller_types: Vec<String>,
    pub(crate) interpolator_types: Vec<String>,
    pub(crate) blend_base_clip: Option<String>,
    pub(crate) text_keys: Vec<PreparedActorAnimationTextKey>,
    pub(crate) skipped_clips: usize,
    pub(crate) ground_offset: f32,
    pub(crate) error: Option<String>,
}

#[derive(Component)]
struct ZooActorRoot;

#[derive(Component)]
struct ZooCamera;

#[derive(Component)]
struct ZooStatusText;

#[derive(Component)]
struct ZooDebugPanel;

#[derive(Component)]
struct ZooClipList;

#[derive(Component)]
struct ZooDebugText;

#[derive(Component)]
struct ZooSelectionText;

#[derive(Component, Clone, Copy)]
struct ZooClipButton(usize);

#[derive(Component, Clone, Copy)]
struct ZooActionButton(ZooUiAction);

#[derive(Component)]
struct ZooButton;

#[derive(Debug, Clone, Copy)]
enum ZooUiAction {
    Control(ZooControlAction),
    ToggleDebug,
    ResetCamera,
    ResetActor,
}

type ZooUiInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        Option<&'static ZooActionButton>,
        Option<&'static ZooClipButton>,
    ),
    (Changed<Interaction>, With<ZooButton>),
>;

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
        ZooCamera,
    ));
    // Keep a stable wrapper above the imported WorldAsset. Some NIF-derived
    // GLBs contain an internal scene root whose serialized hierarchy must not
    // become the transform authority for the pedestal actor.
    let root = commands
        .spawn((
            Transform::from_scale(Vec3::splat(definition.root_scale)),
            Visibility::default(),
            AnimationPlayer::default(),
            ZooActorRoot,
        ))
        .id();
    let asset_root = commands
        .spawn((
            WorldAssetRoot(
                asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset(definition.appearance_path.clone())),
            ),
            Transform::IDENTITY,
            ChildOf(root),
        ))
        .id();
    runtime.actor_root = Some(root);
    runtime.asset_root = Some(asset_root);
    let pack = asset_server.load::<Gltf>(definition.clip_pack_path.clone());
    runtime.pack = Some(pack);
    spawn_zoo_ui(&mut commands, &definition);
}

fn spawn_zoo_ui(commands: &mut Commands, definition: &AnimationZooDefinition) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(14),
                top: px(14),
                max_width: px(520),
                padding: UiRect::all(px(10)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.015, 0.02, 0.03, 0.86)),
        ))
        .with_child((
            Text::new("Animation Zoo | loading actor and clip pack..."),
            ZooStatusText,
            TextColor(Color::srgb(0.88, 0.93, 1.0)),
            TextFont {
                font_size: FontSize::Px(16.0),
                ..default()
            },
        ));

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(14),
                bottom: px(14),
                max_width: px(520),
                padding: UiRect::all(px(8)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.015, 0.02, 0.03, 0.78)),
        ))
        .with_child((
            Text::new(
                "Mouse: left-drag orbit | right-drag rotate NPC | wheel zoom\n\
                 Q/E rotate NPC | Space pause | Left/Right previous/next | R restart\n\
                 L loop | Y cycle catalog | Up/Down speed | D debug details | C reset camera | X reset NPC | Esc exit",
            ),
            TextColor(Color::srgb(0.72, 0.78, 0.86)),
            TextFont {
                font_size: FontSize::Px(13.0),
                ..default()
            },
        ));

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: px(0),
                top: px(0),
                bottom: px(0),
                width: px(ZOO_PANEL_WIDTH),
                min_height: px(0),
                padding: UiRect::all(px(12)),
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
                ..default()
            },
            BackgroundColor(Color::srgba(0.025, 0.035, 0.05, 0.96)),
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new(format!(
                    "{} | {:08x}",
                    definition.actor_name, definition.actor_form_id
                )),
                TextColor(Color::srgb(0.94, 0.96, 1.0)),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
            ));
            panel
                .spawn((Node {
                    width: percent(100),
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: px(5),
                    row_gap: px(5),
                    ..default()
                },))
                .with_children(|controls| {
                    for (label, action) in [
                        ("Prev", ZooUiAction::Control(ZooControlAction::Previous)),
                        ("Pause", ZooUiAction::Control(ZooControlAction::TogglePause)),
                        ("Next", ZooUiAction::Control(ZooControlAction::Next)),
                        ("Restart", ZooUiAction::Control(ZooControlAction::Restart)),
                        ("Loop", ZooUiAction::Control(ZooControlAction::ToggleLoop)),
                        ("Cycle", ZooUiAction::Control(ZooControlAction::ToggleCycle)),
                        ("Speed −", ZooUiAction::Control(ZooControlAction::SpeedDown)),
                        ("Speed +", ZooUiAction::Control(ZooControlAction::SpeedUp)),
                        ("Reset view", ZooUiAction::ResetCamera),
                        ("Reset NPC", ZooUiAction::ResetActor),
                        ("Debug", ZooUiAction::ToggleDebug),
                    ] {
                        spawn_zoo_button(controls, label, ZooActionButton(action));
                    }
                });
            panel.spawn((
                Text::new(""),
                ZooSelectionText,
                TextColor(Color::srgb(0.68, 0.76, 0.88)),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
            ));
            panel
                .spawn((
                    Node {
                        min_height: px(0),
                        max_height: px(520),
                        width: percent(100),
                        padding: UiRect::all(px(6)),
                        overflow: Overflow::scroll_y(),
                        display: Display::None,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.01, 0.015, 0.025, 0.96)),
                    ZooDebugPanel,
                    ScrollArea,
                    Visibility::Hidden,
                ))
                .with_child((
                    Text::new(""),
                    ZooDebugText,
                    TextColor(Color::srgb(0.68, 0.76, 0.88)),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                ));
            panel.spawn((
                Text::new(format!("All animations ({})", definition.clips.len())),
                TextColor(Color::srgb(0.88, 0.91, 0.97)),
                TextFont {
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
            ));
            panel
                .spawn(Node {
                    width: percent(100),
                    min_height: px(0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::scroll_y(),
                    row_gap: px(2),
                    ..default()
                })
                .insert((ZooClipList, ScrollArea))
                .with_children(|list| {
                    for (index, clip) in definition.clips.iter().enumerate() {
                        list.spawn((
                            Button,
                            ZooButton,
                            ZooClipButton(index),
                            Node {
                                width: percent(100),
                                min_height: px(26),
                                padding: UiRect::horizontal(px(6)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                            BorderColor::all(Color::srgb(0.28, 0.42, 0.62)),
                        ))
                        .with_child((
                            Text::new(format!("{:04}  {}", index + 1, clip.name)),
                            TextColor(Color::srgb(0.78, 0.84, 0.94)),
                            TextFont {
                                font_size: FontSize::Px(13.0),
                                ..default()
                            },
                        ));
                    }
                });
        });
}

fn spawn_zoo_button(parent: &mut ChildSpawnerCommands, label: &str, action: ZooActionButton) {
    parent
        .spawn((
            Button,
            ZooButton,
            action,
            Node {
                min_width: px(58),
                min_height: px(28),
                padding: UiRect::horizontal(px(7)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.12, 0.18, 0.28, 1.0)),
            BorderColor::all(Color::srgb(0.28, 0.42, 0.62)),
        ))
        .with_child((
            Text::new(label),
            TextColor(Color::srgb(0.88, 0.93, 1.0)),
            TextFont {
                font_size: FontSize::Px(12.0),
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

/// Return the lowest world-space point of an entity's local-space AABB.
///
/// A skinned mesh's `Aabb` remains its bind-pose bound, but using all eight
/// corners still matters for articulated static nodes: rotating a node changes
/// its world-space vertical extent, which the old center/scale shortcut missed.
fn aabb_min_y(global: &GlobalTransform, aabb: &Aabb) -> f32 {
    let center = Vec3::from(aabb.center);
    let half_extents = Vec3::from(aabb.half_extents);
    let mut minimum = f32::INFINITY;
    for x in [-1.0, 1.0] {
        for y in [-1.0, 1.0] {
            for z in [-1.0, 1.0] {
                let local =
                    center + Vec3::new(x * half_extents.x, y * half_extents.y, z * half_extents.z);
                minimum = minimum.min(global.transform_point(local).y);
            }
        }
    }
    minimum
}

fn collect_pack_target_paths(
    handle: &Handle<GltfNode>,
    nodes: &Assets<GltfNode>,
    path: &mut Vec<Name>,
    parent: Option<usize>,
    parent_global: Mat4,
    output: &mut Vec<PackTarget>,
) -> Option<()> {
    let node = nodes.get(handle)?;
    path.push(Name::new(node.name.clone()));
    let index = output.len();
    let rest_global = parent_global * node.transform.to_matrix();
    output.push(PackTarget {
        name_key: animation_node_name_key(&node.name),
        id: AnimationTargetId::from_names(path.iter()),
        path: path.iter().map(|name| name.as_str().to_owned()).collect(),
        rest_local: node.transform,
        rest_global,
        parent,
    });
    for child in &node.children {
        collect_pack_target_paths(child, nodes, path, Some(index), rest_global, output)?;
    }
    path.pop();
    Some(())
}

/// NIFTools exposes Fallout's side suffix as `Bip01 Calf.L`, while the
/// native converter preserves the source node spelling `Bip01 L Calf`.
/// Canonicalizing only this known side-name convention keeps the original
/// `AnimationTargetId` paths intact while allowing the two prepared assets to
/// bind to one another.
fn animation_node_name_key(value: &str) -> String {
    let value = value.to_ascii_lowercase();
    let Some((prefix, side)) = value.rsplit_once('.') else {
        return value;
    };
    if !matches!(side, "l" | "r") {
        return value;
    }
    let Some((root, bone)) = prefix.split_once(' ') else {
        return value;
    };
    if root != "bip01" {
        return value;
    }
    format!("{root} {side} {bone}")
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
            .all(|(left, right)| animation_node_name_key(left) == animation_node_name_key(right))
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
    global_transforms: Query<&GlobalTransform>,
    aabbs: Query<&Aabb>,
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
            && collect_pack_target_paths(
                handle,
                &nodes,
                &mut Vec::new(),
                None,
                Mat4::IDENTITY,
                &mut target_paths,
            )
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
    let mut targets_by_name = HashMap::<String, Vec<usize>>::new();
    for (index, target) in target_paths.iter().enumerate() {
        targets_by_name
            .entry(target.name_key.clone())
            .or_default()
            .push(index);
    }
    let mut bound_targets = 0_usize;
    let mut retarget_targets = Vec::new();
    for entity in &hierarchy {
        let Ok(name) = names.get(*entity) else {
            continue;
        };
        let Some(candidates) = targets_by_name.get(&animation_node_name_key(name.as_str())) else {
            continue;
        };
        let appearance_path = appearance_name_path(*entity, root, &parents, &names);
        let selected = if candidates.len() == 1 {
            candidates.first().copied()
        } else {
            candidates
                .iter()
                .find(|index| {
                    path_has_case_insensitive_suffix(&appearance_path, &target_paths[**index].path)
                })
                .copied()
        };
        if let Some(source_index) = selected {
            let Some(target_rest_global) = global_transforms
                .get(*entity)
                .ok()
                .map(GlobalTransform::to_matrix)
            else {
                return;
            };
            let id = target_paths[source_index].id;
            commands
                .entity(*entity)
                .insert((id, AnimatedBy(player_entity)));
            retarget_targets.push(ZooRetargetTarget {
                entity: *entity,
                source_index,
                target_rest_global,
            });
            bound_targets += 1;
        }
    }
    if bound_targets == 0 {
        runtime.phase = ZooPhase::Failed;
        runtime.error =
            Some("clip pack target hierarchy does not match any prepared actor nodes".into());
        return;
    }
    let min_y = hierarchy
        .iter()
        .filter_map(|entity| {
            let aabb = aabbs.get(*entity).ok()?;
            let global = global_transforms.get(*entity).ok()?;
            Some(aabb_min_y(global, aabb))
        })
        .reduce(f32::min);
    runtime.ground_offset = min_y.map_or(0.0, |value| -value + 0.015);
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
            handle: handle.clone(),
            node,
            duration: clip.duration(),
            base_clip_index: None,
        });
    }
    if resolved.is_empty() {
        runtime.phase = ZooPhase::Failed;
        runtime.error = Some("clip pack contains no catalog-compatible named animations".into());
        return;
    }
    for index in 0..resolved.len() {
        let Some(base_name) = layer_base_clip_name(&resolved[index].catalog) else {
            continue;
        };
        resolved[index].base_clip_index = resolved
            .iter()
            .position(|candidate| candidate.catalog.name.eq_ignore_ascii_case(&base_name));
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
    retarget_targets.sort_by_key(|target| target.source_index);
    runtime.pack_targets = target_paths;
    runtime.retarget_targets = retarget_targets;
    runtime.bound_targets = bound_targets;
    runtime.clips = resolved;
    runtime.player = Some(player_entity);
    runtime.phase = ZooPhase::Playing;
    let ready_count = runtime.clips.len();
    let skipped_count = definition.skipped_clips + missing.len();
    let ground_offset = runtime.ground_offset;
    push_zoo_event(
        &mut runtime,
        format!(
            "ready: {ready_count} clips, {skipped_count} skipped, {bound_targets} bound targets, ground offset {ground_offset:+.3}"
        ),
    );
    if !missing.is_empty() {
        runtime.error = Some(format!(
            "clip pack omitted {} prepared name(s): {}",
            missing.len(),
            missing.join(", ")
        ));
    }
    info!(
        "animation-zoo ready actor={:08x} clips={} skipped={} bound_targets={}",
        definition.actor_form_id,
        runtime.clips.len(),
        definition.skipped_clips + missing.len(),
        bound_targets
    );
}

fn push_zoo_event(runtime: &mut AnimationZooRuntime, event: impl Into<String>) {
    runtime.event_log.push_back(event.into());
    while runtime.event_log.len() > 16 {
        runtime.event_log.pop_front();
    }
}

fn layer_base_clip_name(clip: &PreparedActorAnimationClip) -> Option<String> {
    // Bethesda's AimUp/AimDown and a few weapon-up/spin-down KF files are
    // authored as partial overlays. They must be evaluated over their paired
    // full-body clip; playing them as a replacement correctly animates only
    // the exported head/arm channels.
    if clip.animated_target_count > 12 {
        return None;
    }
    let name = clip.name.to_ascii_lowercase();
    let candidates = [
        name.strip_suffix("down").map(str::to_owned),
        name.strip_suffix("up").map(str::to_owned),
        Some(name.replace("down_", "_")),
        Some(name.replace("up_", "_")),
    ];
    candidates
        .into_iter()
        .flatten()
        .find(|candidate| candidate != &name)
}

#[allow(clippy::type_complexity)]
fn zoo_ui_interactions(
    interactions: ZooUiInteractionQuery<'_, '_>,
    mut runtime: ResMut<AnimationZooRuntime>,
    mut view: ResMut<AnimationZooViewState>,
) {
    for (interaction, action, clip) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Some(clip) = clip {
            runtime
                .pending_controls
                .push(ZooControlAction::Select(clip.0));
            continue;
        }
        let Some(action) = action else {
            continue;
        };
        match action.0 {
            ZooUiAction::Control(control) => runtime.pending_controls.push(control),
            ZooUiAction::ToggleDebug => view.debug_visible = !view.debug_visible,
            ZooUiAction::ResetCamera => {
                view.camera_yaw = 0.66;
                view.camera_pitch = 0.24;
                view.camera_distance = 8.2;
            }
            ZooUiAction::ResetActor => view.actor_yaw = 0.0,
        }
    }
}

fn source_clip_transform(
    clip: &AnimationClip,
    target: AnimationTargetId,
    time: f32,
    rest: Transform,
    hold_root_motion: bool,
) -> Transform {
    let mut transform = rest;
    if !hold_root_motion
        && let Some(value) =
            clip.sample_clamped(animated_field!(Transform::translation), target, time)
    {
        transform.translation = value;
    }
    if let Some(value) = clip.sample_clamped(animated_field!(Transform::rotation), target, time) {
        transform.rotation = value;
    }
    if let Some(value) = clip.sample_clamped(animated_field!(Transform::scale), target, time) {
        transform.scale = value;
    }
    transform
}

fn source_clip_transform_over_base(
    base_clip: Option<&AnimationClip>,
    selected_clip: &AnimationClip,
    target: AnimationTargetId,
    base_time: f32,
    selected_time: f32,
    rest: Transform,
    hold_root_motion: bool,
) -> Transform {
    let mut transform = base_clip
        .map(|clip| source_clip_transform(clip, target, base_time, rest, hold_root_motion))
        .unwrap_or(rest);
    if !hold_root_motion
        && let Some(value) = selected_clip.sample_clamped(
            animated_field!(Transform::translation),
            target,
            selected_time,
        )
    {
        transform.translation = value;
    }
    if let Some(value) =
        selected_clip.sample_clamped(animated_field!(Transform::rotation), target, selected_time)
    {
        transform.rotation = value;
    }
    if let Some(value) =
        selected_clip.sample_clamped(animated_field!(Transform::scale), target, selected_time)
    {
        transform.scale = value;
    }
    transform
}

fn retarget_global_transform(
    target_rest_global: Mat4,
    source_rest_global: Mat4,
    source_current_global: Mat4,
) -> Mat4 {
    target_rest_global * source_rest_global.inverse() * source_current_global
}

/// The Blender/NIFTools clip pack and the native actor GLB describe the same
/// skeleton in different local coordinate bases. Reconstruct the source pose
/// in the pack hierarchy, transfer each node's delta from source rest to target
/// rest in global space, and decompose it back into the native actor's locals.
/// This keeps the compatibility backend isolated to the zoo and avoids baking
/// a Blender-space pose into native Fallout-unit nodes.
fn retarget_animation_zoo(
    runtime: Res<AnimationZooRuntime>,
    players: Query<&AnimationPlayer>,
    clips: Res<Assets<AnimationClip>>,
    parents: Query<&ChildOf>,
    globals: Query<&GlobalTransform>,
    mut transforms: Query<&mut Transform>,
) {
    if runtime.phase != ZooPhase::Playing || runtime.retarget_targets.is_empty() {
        return;
    }
    let Some(policy) = runtime.policy.as_ref() else {
        return;
    };
    let Some(clip_info) = runtime.clips.get(policy.index) else {
        return;
    };
    let Some(clip) = clips.get(&clip_info.handle) else {
        return;
    };
    let base_clip = clip_info
        .base_clip_index
        .and_then(|index| runtime.clips.get(index))
        .and_then(|base| clips.get(&base.handle));
    let Some(player_entity) = runtime.player else {
        return;
    };
    let time = players
        .get(player_entity)
        .ok()
        .and_then(|player| player.animation(clip_info.node))
        .map_or(runtime.elapsed, |active| active.seek_time());
    let base_time = base_clip.map_or(0.0, |base| {
        if base.duration() > 0.0 {
            time.rem_euclid(base.duration())
        } else {
            0.0
        }
    });
    let accumulation_root = clip_info
        .catalog
        .accumulation_root
        .as_deref()
        .map(animation_node_name_key);

    let mut source_globals = vec![Mat4::IDENTITY; runtime.pack_targets.len()];
    for (index, target) in runtime.pack_targets.iter().enumerate() {
        let hold_root_motion = accumulation_root.as_deref().is_some_and(|root| {
            target.name_key == root || target.name_key == format!("{root} nonaccum")
        });
        let local = source_clip_transform_over_base(
            base_clip,
            clip,
            target.id,
            base_time,
            time,
            target.rest_local,
            hold_root_motion,
        );
        source_globals[index] = target
            .parent
            .map_or(Mat4::IDENTITY, |parent| source_globals[parent])
            * local.to_matrix();
    }

    let mut desired_globals = HashMap::<Entity, Mat4>::new();
    for target in &runtime.retarget_targets {
        let source = &runtime.pack_targets[target.source_index];
        let desired_global = retarget_global_transform(
            target.target_rest_global,
            source.rest_global,
            source_globals[target.source_index],
        );
        let parent_global = parents
            .get(target.entity)
            .ok()
            .and_then(|parent| desired_globals.get(&parent.parent()).copied())
            .or_else(|| {
                parents
                    .get(target.entity)
                    .ok()
                    .and_then(|parent| globals.get(parent.parent()).ok())
                    .map(GlobalTransform::to_matrix)
            })
            .unwrap_or(Mat4::IDENTITY);
        if let Ok(mut transform) = transforms.get_mut(target.entity) {
            *transform = Transform::from_matrix(parent_global.inverse() * desired_global);
        }
        desired_globals.insert(target.entity, desired_global);
    }
}

fn keyboard_controls(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut runtime: ResMut<AnimationZooRuntime>,
    mut view: ResMut<AnimationZooViewState>,
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
    if keys.just_pressed(KeyCode::KeyD) {
        view.debug_visible = !view.debug_visible;
    }
    if keys.pressed(KeyCode::KeyQ) {
        view.actor_yaw += time.delta_secs() * 1.8;
    }
    if keys.pressed(KeyCode::KeyE) {
        view.actor_yaw -= time.delta_secs() * 1.8;
    }
    if keys.just_pressed(KeyCode::KeyC) {
        view.camera_yaw = 0.66;
        view.camera_pitch = 0.24;
        view.camera_distance = 8.2;
    }
    if keys.just_pressed(KeyCode::KeyX) {
        view.actor_yaw = 0.0;
    }
}

fn zoo_view_controls(
    mut motions: MessageReader<MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut view: ResMut<AnimationZooViewState>,
) {
    let viewport_active = windows
        .single()
        .ok()
        .and_then(Window::cursor_position)
        .is_some_and(|position| {
            position.x
                < windows
                    .single()
                    .ok()
                    .map_or(0.0, |window| window.resolution.width() - ZOO_PANEL_WIDTH)
        });
    for event in motions.read() {
        if !viewport_active {
            continue;
        }
        if buttons.pressed(MouseButton::Left) {
            view.camera_yaw -= event.delta.x * 0.008;
            view.camera_pitch = (view.camera_pitch + event.delta.y * 0.006).clamp(-1.15, 1.25);
        } else if buttons.pressed(MouseButton::Right) {
            view.actor_yaw -= event.delta.x * 0.012;
        }
    }
    if viewport_active {
        for event in wheel.read() {
            let scale = match event.unit {
                MouseScrollUnit::Line => 0.8,
                MouseScrollUnit::Pixel => 0.008,
            };
            view.camera_distance = (view.camera_distance - event.y * scale).clamp(2.5, 18.0);
        }
    } else {
        wheel.clear();
    }
    view.camera_yaw = view.camera_yaw.rem_euclid(std::f32::consts::TAU);
    view.actor_yaw = view.actor_yaw.rem_euclid(std::f32::consts::TAU);
}

#[allow(clippy::type_complexity)]
fn apply_zoo_view(
    runtime: Res<AnimationZooRuntime>,
    view: Res<AnimationZooViewState>,
    mut cameras: Query<&mut Transform, With<ZooCamera>>,
    mut actors: Query<
        &mut Transform,
        (
            With<ZooActorRoot>,
            Without<ZooCamera>,
            Without<WorldAssetRoot>,
        ),
    >,
    mut assets: Query<
        &mut Transform,
        (
            With<WorldAssetRoot>,
            Without<ZooActorRoot>,
            Without<ZooCamera>,
        ),
    >,
) {
    // Keep the camera target in pedestal/world space. The imported asset's
    // bind-pose offset is applied to the asset root below; following that
    // correction with the camera would put the camera below the floor.
    let target = ZOO_CAMERA_TARGET;
    let horizontal = view.camera_pitch.cos() * view.camera_distance;
    let camera_position = target
        + Vec3::new(
            view.camera_yaw.sin() * horizontal,
            view.camera_pitch.sin() * view.camera_distance,
            view.camera_yaw.cos() * horizontal,
        );
    if let Ok(mut camera) = cameras.single_mut() {
        *camera = Transform::from_translation(camera_position).looking_at(target, Vec3::Y);
    }
    if let Ok(mut actor) = actors.single_mut() {
        actor.translation.y = 0.0;
        actor.rotation = Quat::IDENTITY;
    }
    if let Some(asset_root) = runtime.asset_root
        && let Ok(mut asset) = assets.get_mut(asset_root)
    {
        asset.translation.y = runtime.ground_offset;
        asset.rotation = Quat::from_rotation_y(view.actor_yaw);
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
        push_zoo_event(
            &mut runtime,
            format!(
                "clip {}/{} {} | speed {:.2}x | loop {}",
                policy.index + 1,
                policy.clip_count,
                clip_name,
                policy.speed,
                policy.loop_current
            ),
        );
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
        auto_advance: policy.is_some_and(|policy| policy.auto_advance),
        completed_cycles: policy.map_or(0, |policy| policy.completed_cycles),
        missing_targets: clip
            .map(|clip| clip.catalog.missing_targets.clone())
            .unwrap_or_default(),
        bound_targets: runtime.bound_targets,
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
        blend_base_clip: clip
            .and_then(|clip| clip.base_clip_index)
            .and_then(|index| runtime.clips.get(index))
            .map(|clip| clip.catalog.name.clone()),
        text_keys: clip
            .map(|clip| clip.catalog.text_keys.clone())
            .unwrap_or_default(),
        skipped_clips: definition.skipped_clips,
        ground_offset: runtime.ground_offset,
        error: runtime.error.clone(),
    };
}

#[allow(clippy::type_complexity)]
fn update_zoo_ui(
    probe: Res<AnimationZooProbe>,
    definition: Res<AnimationZooDefinition>,
    runtime: Res<AnimationZooRuntime>,
    view: Res<AnimationZooViewState>,
    mut texts: ParamSet<(
        Query<&mut Text, With<ZooStatusText>>,
        Query<&mut Text, With<ZooSelectionText>>,
        Query<&mut Text, With<ZooDebugText>>,
    )>,
    debug_panel: Single<(&mut Visibility, &mut Node), With<ZooDebugPanel>>,
    mut clip_rows: Query<(&ZooClipButton, &mut BackgroundColor)>,
) {
    let clip = runtime
        .policy
        .as_ref()
        .and_then(|policy| runtime.clips.get(policy.index));
    let status_text = format!(
        "{} | {}\nClip {}/{}  |  {}\n{} {:.2}x  |  loop {}  |  cycle {}  |  bound {}/{}  |  skipped {}",
        definition.actor_name,
        probe.actor_form_id,
        if probe.count == 0 { 0 } else { probe.index + 1 },
        probe.count,
        probe.current_clip.as_deref().unwrap_or("<loading>"),
        probe.playback_state,
        probe.speed,
        probe.loop_current,
        probe.auto_advance,
        probe.bound_targets,
        clip.map_or(0, |clip| clip.catalog.required_targets.len()),
        probe.skipped_clips,
    );
    let selection_text = format!(
        "Selected: {}{}\nMouse orbit/rotate on viewport; click any row to play it.",
        clip.map_or("<loading>", |clip| clip.catalog.name.as_str()),
        probe
            .blend_base_clip
            .as_deref()
            .map_or(String::new(), |base| format!("  |  layered over {base}"))
    );
    if let Ok(mut status) = texts.p0().single_mut() {
        status.0 = status_text;
    }
    if let Ok(mut selection) = texts.p1().single_mut() {
        selection.0 = selection_text;
    }
    let (mut debug_visibility, mut debug_node) = debug_panel.into_inner();
    *debug_visibility = if view.debug_visible {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    debug_node.display = if view.debug_visible {
        Display::Flex
    } else {
        Display::None
    };
    if view.debug_visible {
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
        let events = runtime
            .event_log
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join("\n");
        let debug_text = format!(
            "source: {}\nsequence: {}\nrange: {:?}..{:?} @ {:?} Hz\nduration: {:.3}s | elapsed: {:.3}s | cycles: {}\nloop current: {} | cycle catalog: {} | ground offset: {:+.3}\nroot motion: {:?} ({})\nblend base: {}\nchannels: {} | animated targets: {} | bound: {}\nrequired: {} | missing: {} | text keys: {}\ncontrollers: {}\ninterpolators: {}\n{}{}\n\nRecent events:\n{}",
            probe.source_kf_path.as_deref().unwrap_or("<none>"),
            probe.source_sequence_name.as_deref().unwrap_or("<unnamed>"),
            probe.source_start_seconds,
            probe.source_end_seconds,
            probe.source_frequency,
            probe.duration,
            probe.elapsed,
            probe.completed_cycles,
            probe.loop_current,
            probe.auto_advance,
            probe.ground_offset,
            probe.root_motion_policy,
            probe.accumulation_root.as_deref().unwrap_or("<none>"),
            probe.blend_base_clip.as_deref().unwrap_or("<none>"),
            clip.map_or(0, |clip| clip.catalog.animated_channel_count),
            probe.animated_targets.len(),
            probe.bound_targets,
            probe.required_targets.len(),
            probe.missing_targets.len(),
            probe.text_keys.len(),
            probe.controller_types.join(", "),
            probe.interpolator_types.join(", "),
            if diagnostics.is_empty() {
                "".to_owned()
            } else {
                format!("diagnostics: {diagnostics}\n")
            },
            probe
                .error
                .as_ref()
                .map(|error| format!("ERROR: {error}\n"))
                .unwrap_or_default(),
            events,
        );
        if let Ok(mut debug) = texts.p2().single_mut() {
            debug.0 = debug_text;
        }
    } else {
        if let Ok(mut debug) = texts.p2().single_mut() {
            debug.0.clear();
        }
    }
    for (button, mut background) in &mut clip_rows {
        *background = if Some(button.0) == runtime.policy.as_ref().map(|policy| policy.index) {
            BackgroundColor(Color::srgba(0.18, 0.32, 0.52, 1.0))
        } else {
            BackgroundColor(Color::NONE)
        };
    }
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
    let action = if let Some(index) = action.strip_prefix("select:") {
        ZooControlAction::Select(index.parse::<usize>().map_err(|_| {
            format!("select action requires a non-negative clip index, got '{index}'")
        })?)
    } else {
        match action {
            "previous" => ZooControlAction::Previous,
            "next" => ZooControlAction::Next,
            "restart" => ZooControlAction::Restart,
            "toggle_pause" => ZooControlAction::TogglePause,
            "toggle_loop" => ZooControlAction::ToggleLoop,
            "toggle_cycle" => ZooControlAction::ToggleCycle,
            "speed_up" => ZooControlAction::SpeedUp,
            "speed_down" => ZooControlAction::SpeedDown,
            _ => {
                return Err(format!(
                    "unknown action '{action}'; expected select:<index>, previous, next, restart, toggle_pause, toggle_loop, toggle_cycle, speed_up, or speed_down"
                ));
            }
        }
    };
    let Some(mut runtime) = world.get_resource_mut::<AnimationZooRuntime>() else {
        return Err("the active viewer is not an animation zoo".into());
    };
    runtime.pending_controls.push(action);
    Ok(())
}

#[cfg(test)]
#[path = "tests/animation_zoo.rs"]
mod tests;
