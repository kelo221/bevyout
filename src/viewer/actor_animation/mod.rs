//! Gameplay actor-animation integration.
//!
//! Preparation decides which KF files belong to each actor and converts one
//! shared clip pack per skeleton. This plugin consumes that catalog for actors
//! spawned by the ordinary cell flow. The animation zoo remains a diagnostic
//! client and is deliberately not part of this runtime path.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use bevy::animation::{AnimatedBy, AnimationTargetId, RepeatAnimation, animated_field};
use bevy::app::AnimationSystems;
use bevy::asset::LoadState;
use bevy::gltf::{Gltf, GltfNode};
use bevy::prelude::*;
use bevy::transform::TransformSystems;
use bevyout_core::actor_animation::{
    PreparedActorAnimationCatalog, PreparedActorAnimationClip, PreparedActorAnimationKind,
    PreparedActorAnimationLoopMode, PreparedActorAnimationRootMotionPolicy,
    PreparedActorAnimationSet, canonical_mesh_path,
};

use super::actor::{ActorRuntime, ActorRuntimeState};
use super::world::ActiveCell;
use super::{
    ACTOR_ANIMATION_CATALOG_REVISION, PreparedItemCatalog, PreparedItemStats,
    PreparedSceneManifest, fingerprint,
};

pub(crate) mod idle_policy;
pub(crate) mod policy;

use idle_policy::{
    IdleAuthority, IdleConditionEvaluator, IdleConditionOutcome, IdleDecision,
    IdleEvaluationTrigger, IdleLifecycleFacts, IdlePackageCollection, IdlePackageContext,
    IdleRejectionReason, IdleSource,
};
use policy::{
    ActorAnimationContext, ActorAnimationState, resolve_clip, should_advance_playback,
    should_repeat, state_after_completion, weapon_animation_prefix,
};

const MOVEMENT_EPSILON_METRES: f32 = 0.002;
const RUN_SPEED_METRES_PER_SECOND: f32 = 3.0;
const TURN_EPSILON_RADIANS: f32 = 0.004;
const TRANSITION_SECONDS: f32 = 0.12;
const MOVEMENT_HOLD_SECONDS: f32 = 0.15;

pub(crate) struct ActorAnimationPlugin;

impl Plugin for ActorAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorAnimationCatalogs>()
            .init_resource::<AnimationSetCache>()
            .add_systems(
                Update,
                (
                    register_gameplay_actors,
                    resolve_animation_sets,
                    bind_gameplay_actors,
                    select_gameplay_animation,
                    drive_gameplay_animation,
                    prune_unused_animation_sets,
                )
                    .chain()
                    .in_set(super::plugins::ViewerSet::WorldSync),
            )
            .add_systems(
                PostUpdate,
                retarget_gameplay_actors
                    .after(AnimationSystems)
                    .before(TransformSystems::Propagate),
            );
    }
}

/// Prepared catalogs keyed by the cell whose manifest links them. Neighbor
/// preload inserts its catalog before spawning that cell's actors.
#[derive(Resource, Default)]
pub(crate) struct ActorAnimationCatalogs(HashMap<u32, Arc<PreparedActorAnimationCatalog>>);

impl ActorAnimationCatalogs {
    pub(crate) fn insert(&mut self, cell_form_id: u32, catalog: PreparedActorAnimationCatalog) {
        self.0.insert(cell_form_id, Arc::new(catalog));
    }

    pub(crate) fn remove(&mut self, cell_form_id: u32) {
        self.0.remove(&cell_form_id);
    }

    fn actor_set(
        &self,
        reference_form_id: u32,
        base_form_id: u32,
    ) -> Option<(u32, &PreparedActorAnimationSet, PreparedActorAnimationKind)> {
        self.0.iter().find_map(|(cell, catalog)| {
            let mapping = catalog.actor_mappings.iter().find(|mapping| {
                mapping.reference_form_id == reference_form_id
                    || (mapping.reference_form_id == 0 && mapping.base_form_id == base_form_id)
            })?;
            let set = catalog
                .animation_sets
                .iter()
                .find(|set| set.id == mapping.animation_set_id)?;
            Some((*cell, set, mapping.kind))
        })
    }

    fn contains_set(&self, set_id: &str) -> bool {
        self.0
            .values()
            .any(|catalog| catalog.animation_sets.iter().any(|set| set.id == set_id))
    }

    fn actor_catalog(
        &self,
        reference_form_id: u32,
        base_form_id: u32,
    ) -> Option<(u32, Arc<PreparedActorAnimationCatalog>)> {
        self.0.iter().find_map(|(cell, catalog)| {
            catalog
                .actor_mappings
                .iter()
                .any(|mapping| {
                    mapping.reference_form_id == reference_form_id
                        || (mapping.reference_form_id == 0 && mapping.base_form_id == base_form_id)
                })
                .then(|| (*cell, Arc::clone(catalog)))
        })
    }

    fn idle_definition(
        &self,
        reference_form_id: u32,
        base_form_id: u32,
        idle_form_id: u32,
    ) -> Option<bevyout_core::actor_animation::PreparedActorIdleDefinition> {
        self.actor_catalog(reference_form_id, base_form_id)
            .and_then(|(_, catalog)| {
                catalog
                    .idle_definitions
                    .iter()
                    .find(|definition| definition.form_id == idle_form_id)
                    .cloned()
            })
    }
}

/// Reads and validates a manifest-linked actor catalog. An absent link is a
/// valid catalog-free cell; a stale or mismatched link is not.
pub(crate) fn load_catalog_for_manifest(
    manifest: &PreparedSceneManifest,
    asset_root: &Path,
) -> Result<Option<PreparedActorAnimationCatalog>> {
    let Some(relative) = manifest.actor_animation_catalog_path.as_deref() else {
        return Ok(None);
    };
    let path = asset_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    let bytes = fs::read(&path)
        .with_context(|| format!("reading actor animation catalog {}", path.display()))?;
    if manifest.actor_animation_catalog_hash.as_deref() != Some(fingerprint(&bytes).as_str()) {
        bail!("actor animation catalog hash does not match scene manifest");
    }
    let catalog: PreparedActorAnimationCatalog = ron::de::from_bytes(&bytes)
        .with_context(|| format!("invalid actor animation catalog {}", path.display()))?;
    if manifest.actor_animation_catalog_revision.as_deref() != Some(catalog.revision.as_str()) {
        bail!("actor animation catalog revision does not match scene manifest");
    }
    if catalog.revision != ACTOR_ANIMATION_CATALOG_REVISION {
        bail!(
            "actor animation catalog revision {} is stale, expected {ACTOR_ANIMATION_CATALOG_REVISION}; run `prepare` again",
            catalog.revision
        );
    }
    if catalog.source_fingerprint != manifest.source_fingerprint {
        bail!(
            "actor animation catalog fingerprint {} does not match scene {}",
            catalog.source_fingerprint,
            manifest.source_fingerprint
        );
    }
    Ok(Some(catalog))
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

#[derive(Clone)]
struct ResolvedClip {
    catalog: PreparedActorAnimationClip,
    handle: Handle<AnimationClip>,
    node: AnimationNodeIndex,
}

struct CachedAnimationSet {
    definition: PreparedActorAnimationSet,
    pack: Handle<Gltf>,
    resolved: Option<Arc<ResolvedAnimationSet>>,
    failed: Option<String>,
}

struct ResolvedAnimationSet {
    graph: Handle<AnimationGraph>,
    pack_targets: Vec<PackTarget>,
    clips: HashMap<String, ResolvedClip>,
}

#[derive(Debug, Clone, Copy)]
struct SpecialIdlePlayback {
    form_id: u32,
    loops_remaining: u8,
    source: IdleSource,
}

#[derive(Resource, Default)]
struct AnimationSetCache(HashMap<String, CachedAnimationSet>);

#[derive(Component, Debug, Clone, Default)]
pub(crate) struct ActorAnimationIntent {
    pub(crate) requested: Option<ActorAnimationState>,
    pub(crate) requested_idle: Option<u32>,
    pub(crate) forced_idle: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuntimePhase {
    Loading,
    Playing,
    Failed,
}

#[derive(Clone, Copy)]
struct RetargetTarget {
    entity: Entity,
    source_index: usize,
    target_rest_root: Mat4,
}

#[derive(Component)]
pub(crate) struct ActorAnimationRuntime {
    cell_form_id: u32,
    set_id: String,
    kind: PreparedActorAnimationKind,
    female: bool,
    phase: RuntimePhase,
    retarget_targets: Vec<RetargetTarget>,
    current_state: ActorAnimationState,
    selected_clip: Option<String>,
    started_clip: Option<String>,
    previous_clip: Option<String>,
    transition_elapsed: f32,
    selected_loop_mode: PreparedActorAnimationLoopMode,
    selected_root_motion: PreparedActorAnimationRootMotionPolicy,
    last_position: Vec3,
    last_yaw: f32,
    automatic_state: ActorAnimationState,
    movement_hold_remaining: f32,
    diagnostic: Option<String>,
    idle_authority: IdleAuthority,
    special_idle: Option<SpecialIdlePlayback>,
    was_stationary: bool,
}

impl ActorAnimationRuntime {
    pub(crate) fn state_label(&self) -> &'static str {
        self.current_state.label()
    }

    pub(crate) fn clip_name(&self) -> Option<&str> {
        self.selected_clip.as_deref()
    }

    pub(crate) fn diagnostic(&self) -> Option<&str> {
        self.diagnostic.as_deref()
    }

    pub(crate) fn set_id(&self) -> &str {
        &self.set_id
    }

    pub(crate) fn bound_target_count(&self) -> usize {
        self.retarget_targets.len()
    }

    pub(crate) fn loop_mode(&self) -> PreparedActorAnimationLoopMode {
        self.selected_loop_mode
    }

    pub(crate) fn root_motion_policy(&self) -> PreparedActorAnimationRootMotionPolicy {
        self.selected_root_motion
    }

    pub(crate) fn idle_authority(&self) -> &IdleAuthority {
        &self.idle_authority
    }

    pub(crate) fn special_idle_form_id(&self) -> Option<u32> {
        self.special_idle.map(|idle| idle.form_id)
    }

    pub(crate) fn special_idle_source(&self) -> Option<IdleSource> {
        self.special_idle.map(|idle| idle.source)
    }

    pub(crate) fn forced_idle_lifecycle_ready(&self) -> bool {
        self.phase == RuntimePhase::Playing
            && self.automatic_state == ActorAnimationState::Idle
            && self.movement_hold_remaining <= f32::EPSILON
    }
}

/// Narrow gameplay API for scripted/AI systems and the console adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IdleRequestError {
    pub(crate) code: &'static str,
    pub(crate) message: String,
}

pub(crate) fn request_forced_idle(
    world: &mut World,
    entity: Entity,
    idle_form_id: u32,
) -> std::result::Result<u32, IdleRequestError> {
    let actor = world
        .get::<ActorRuntime>(entity)
        .ok_or_else(|| IdleRequestError {
            code: "unknown_actor",
            message: "reference is not a known projected actor".into(),
        })?;
    let reference_form_id = actor.reference_form_id;
    let base_form_id = actor.base_form_id;
    let definition = world
        .get_resource::<ActorAnimationCatalogs>()
        .and_then(|catalogs| {
            catalogs.idle_definition(reference_form_id, base_form_id, idle_form_id)
        })
        .ok_or_else(|| IdleRequestError {
            code: "unknown_idle",
            message: format!(
                "prepared IDLE {idle_form_id:08x} is unknown for actor {reference_form_id:08x}"
            ),
        })?;
    if !matches!(
        definition.group_section,
        idle_policy::SPECIAL_IDLE_GROUP | idle_policy::WHOLE_BODY_GROUP
    ) {
        return Err(IdleRequestError {
            code: "unsupported_group",
            message: format!(
                "IDLE {idle_form_id:08x} group section {} is unsupported; only 7 and 20 are available",
                definition.group_section
            ),
        });
    }
    let runtime = world
        .get::<ActorAnimationRuntime>(entity)
        .ok_or_else(|| IdleRequestError {
            code: "unavailable_clip",
            message: "actor animation controller is not ready".into(),
        })?;
    let active = world
        .get_resource::<ActiveCell>()
        .is_none_or(|active| active.0 == runtime.cell_form_id);
    let visible = world
        .get::<InheritedVisibility>(entity)
        .is_none_or(|visibility| visibility.get());
    let alive = world
        .get::<super::actor_state::ActorStateRuntime>(entity)
        .is_none_or(|state| state.life_state == bevyout_core::actor_state::ActorLifeState::Alive);
    let ragdolled = world
        .get::<super::player::RagdollToggle>(entity)
        .is_some_and(|toggle| toggle.0);
    if !active || !visible || !alive || ragdolled || !runtime.forced_idle_lifecycle_ready() {
        return Err(IdleRequestError {
            code: "invalid_lifecycle",
            message: format!(
                "actor {reference_form_id:08x} must be stationary, alive, loaded, visible, and not ragdolled"
            ),
        });
    }
    let bound_indices = runtime
        .retarget_targets
        .iter()
        .map(|target| target.source_index)
        .collect::<Vec<_>>();
    let cache = world
        .get_resource::<AnimationSetCache>()
        .and_then(|cache| cache.0.get(runtime.set_id()))
        .and_then(|cached| cached.resolved.as_ref())
        .ok_or_else(|| IdleRequestError {
            code: "unavailable_clip",
            message: format!(
                "IDLE {idle_form_id:08x} clip is not Ready in the loaded animation set"
            ),
        })?;
    let clip_name = definition
        .clip_name
        .as_deref()
        .ok_or_else(|| IdleRequestError {
            code: "unavailable_clip",
            message: format!("IDLE {idle_form_id:08x} has no prepared clip"),
        })?;
    let Some(clip) = cache.clips.get(&clip_name.to_ascii_lowercase()) else {
        return Err(IdleRequestError {
            code: "unavailable_clip",
            message: format!("IDLE {idle_form_id:08x} clip {clip_name} is unavailable"),
        });
    };
    if clip.catalog.status != bevyout_core::actor_animation::PreparedActorAnimationClipStatus::Ready
    {
        return Err(IdleRequestError {
            code: "unavailable_clip",
            message: format!("IDLE {idle_form_id:08x} clip {clip_name} is not Ready"),
        });
    }
    let missing = missing_required_targets(&clip.catalog.required_targets, cache, bound_indices);
    if !missing.is_empty() {
        return Err(IdleRequestError {
            code: "unavailable_clip",
            message: format!(
                "IDLE {idle_form_id:08x} clip {clip_name} is incompatible with the bound actor targets"
            ),
        });
    }
    let mut intent = world
        .get_mut::<ActorAnimationIntent>(entity)
        .ok_or_else(|| IdleRequestError {
            code: "unavailable_clip",
            message: "actor animation intent is unavailable".into(),
        })?;
    intent.requested_idle = Some(idle_form_id);
    intent.forced_idle = true;
    intent.requested = None;
    Ok(reference_form_id)
}

pub(crate) fn request_actor_animation(
    world: &mut World,
    entity: Entity,
    state: ActorAnimationState,
) -> Result<()> {
    let Some(mut intent) = world.get_mut::<ActorAnimationIntent>(entity) else {
        bail!("entity has no gameplay actor animation controller");
    };
    intent.requested = Some(state);
    intent.requested_idle = None;
    intent.forced_idle = false;
    Ok(())
}

struct RuntimeIdleConditions;

struct RuntimeIdleConditionFunctions<'a> {
    facts: &'a idle_policy::IdleRuntimeFacts,
    random_percent: u8,
}

impl super::ai::selection::ConditionFunctions for RuntimeIdleConditionFunctions<'_> {
    fn evaluate(&self, function_index: u16, param1: u32, _param2: u32) -> Option<f32> {
        match function_index {
            101 => Some(f32::from(u8::from(self.facts.weapon_out))),
            71 => Some(f32::from(u8::from(self.facts.factions.contains(&param1)))),
            77 => Some(f32::from(self.random_percent)),
            182 => Some(f32::from(u8::from(
                self.facts.equipped_item_form_ids.contains(&param1),
            ))),
            451 => Some(f32::from(u8::from(
                self.facts.last_idle_played == Some(param1),
            ))),
            _ => None,
        }
    }
}

impl IdleConditionEvaluator for RuntimeIdleConditions {
    fn evaluate(
        &self,
        conditions: &[Vec<u8>],
        random_percent: u8,
        facts: &idle_policy::IdleRuntimeFacts,
    ) -> IdleConditionOutcome {
        let functions = RuntimeIdleConditionFunctions {
            facts,
            random_percent,
        };
        match super::ai::selection::evaluate_conditions(conditions, &functions) {
            super::ai::selection::ConditionOutcome::True => IdleConditionOutcome::True,
            super::ai::selection::ConditionOutcome::False => IdleConditionOutcome::False,
            super::ai::selection::ConditionOutcome::Unevaluable => {
                IdleConditionOutcome::Unevaluable
            }
        }
    }
}

fn package_idle_context(
    controller: Option<&super::ai::family_runtime::ActorPackageController>,
    cache: &super::ai::catalog_cache::PackageCatalogCache,
) -> Option<IdlePackageContext> {
    let package_form_id = controller?.selected_form_id;
    let entry = cache.packages.as_ref().and_then(|(_, catalog)| {
        catalog
            .packages
            .iter()
            .find(|entry| entry.form_id == package_form_id)
    })?;
    Some(IdlePackageContext {
        form_id: package_form_id,
        general_flags: entry.general_flags,
        collection: entry
            .idle_collection
            .as_ref()
            .map(|collection| IdlePackageCollection {
                flags: collection.flags,
                timer_seconds: collection.timer_seconds,
                animation_form_ids: collection.animation_form_ids.clone(),
            }),
    })
}

fn register_gameplay_actors(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    catalogs: Res<ActorAnimationCatalogs>,
    mut cache: ResMut<AnimationSetCache>,
    actors: Query<(Entity, &ActorRuntime, &GlobalTransform), Without<ActorAnimationRuntime>>,
) {
    for (entity, actor, global) in &actors {
        let Some((cell_form_id, set, prepared_kind)) =
            catalogs.actor_set(actor.reference_form_id, actor.base_form_id)
        else {
            continue;
        };
        let female = actor
            .assembly
            .as_ref()
            .is_some_and(|assembly| assembly.female);
        let skeleton_matches = actor
            .assembly
            .as_ref()
            .and_then(|assembly| assembly.skeleton_path.as_deref())
            .is_some_and(|skeleton| {
                canonical_mesh_path(skeleton) == canonical_mesh_path(&set.skeleton_path)
            });
        if !skeleton_matches {
            commands.entity(entity).insert((
                ActorAnimationIntent::default(),
                ActorAnimationRuntime {
                    cell_form_id,
                    set_id: set.id.clone(),
                    kind: prepared_kind,
                    female,
                    phase: RuntimePhase::Failed,
                    retarget_targets: Vec::new(),
                    current_state: ActorAnimationState::Idle,
                    selected_clip: None,
                    started_clip: None,
                    previous_clip: None,
                    transition_elapsed: TRANSITION_SECONDS,
                    selected_loop_mode: PreparedActorAnimationLoopMode::Unknown,
                    selected_root_motion: PreparedActorAnimationRootMotionPolicy::Unknown,
                    last_position: global.translation(),
                    last_yaw: global
                        .compute_transform()
                        .rotation
                        .to_euler(EulerRot::YXZ)
                        .0,
                    automatic_state: ActorAnimationState::Idle,
                    movement_hold_remaining: 0.0,
                    diagnostic: Some(format!(
                        "actor skeleton does not match animation set {}",
                        set.skeleton_path
                    )),
                    idle_authority: IdleAuthority::default(),
                    special_idle: None,
                    was_stationary: false,
                },
            ));
            continue;
        }
        let Some(pack_path) = set.clip_pack_asset_path.as_ref() else {
            commands.entity(entity).insert((
                ActorAnimationIntent::default(),
                ActorAnimationRuntime {
                    cell_form_id,
                    set_id: set.id.clone(),
                    kind: prepared_kind,
                    female,
                    phase: RuntimePhase::Failed,
                    retarget_targets: Vec::new(),
                    current_state: ActorAnimationState::Idle,
                    selected_clip: None,
                    started_clip: None,
                    previous_clip: None,
                    transition_elapsed: TRANSITION_SECONDS,
                    selected_loop_mode: PreparedActorAnimationLoopMode::Unknown,
                    selected_root_motion: PreparedActorAnimationRootMotionPolicy::Unknown,
                    last_position: global.translation(),
                    last_yaw: global
                        .compute_transform()
                        .rotation
                        .to_euler(EulerRot::YXZ)
                        .0,
                    automatic_state: ActorAnimationState::Idle,
                    movement_hold_remaining: 0.0,
                    diagnostic: Some("prepared animation set has no clip pack".into()),
                    idle_authority: IdleAuthority::default(),
                    special_idle: None,
                    was_stationary: false,
                },
            ));
            continue;
        };
        cache
            .0
            .entry(set.id.clone())
            .or_insert_with(|| CachedAnimationSet {
                definition: set.clone(),
                pack: asset_server.load::<Gltf>(pack_path.clone()),
                resolved: None,
                failed: None,
            });
        commands.entity(entity).insert((
            AnimationPlayer::default(),
            ActorAnimationIntent::default(),
            ActorAnimationRuntime {
                cell_form_id,
                set_id: set.id.clone(),
                kind: prepared_kind,
                female,
                phase: RuntimePhase::Loading,
                retarget_targets: Vec::new(),
                current_state: ActorAnimationState::Idle,
                selected_clip: None,
                started_clip: None,
                previous_clip: None,
                transition_elapsed: TRANSITION_SECONDS,
                selected_loop_mode: PreparedActorAnimationLoopMode::Unknown,
                selected_root_motion: PreparedActorAnimationRootMotionPolicy::Unknown,
                last_position: global.translation(),
                last_yaw: global
                    .compute_transform()
                    .rotation
                    .to_euler(EulerRot::YXZ)
                    .0,
                automatic_state: ActorAnimationState::Idle,
                movement_hold_remaining: 0.0,
                diagnostic: None,
                idle_authority: IdleAuthority::default(),
                special_idle: None,
                was_stationary: false,
            },
        ));
    }
}

fn resolve_animation_sets(
    asset_server: Res<AssetServer>,
    gltfs: Res<Assets<Gltf>>,
    nodes: Res<Assets<GltfNode>>,
    clips: Res<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut cache: ResMut<AnimationSetCache>,
) {
    for cached in cache.0.values_mut() {
        if cached.resolved.is_some() || cached.failed.is_some() {
            continue;
        }
        if let Some(LoadState::Failed(error)) = asset_server.get_load_state(&cached.pack) {
            cached.failed = Some(format!("clip pack failed to load: {error}"));
            continue;
        }
        let Some(pack) = gltfs.get(&cached.pack) else {
            continue;
        };
        if !pack
            .named_animations
            .values()
            .all(|handle| clips.get(handle).is_some())
        {
            continue;
        }
        let mut pack_targets = Vec::new();
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
                    &mut pack_targets,
                )
                .is_none()
            {
                return;
            }
        }
        if pack_targets.is_empty() {
            cached.failed = Some("clip pack contains no animation target hierarchy".into());
            continue;
        }
        let mut graph = AnimationGraph::new();
        let root = graph.root;
        let mut resolved_clips = HashMap::new();
        for catalog_clip in &cached.definition.clips {
            let Some(handle) = pack.named_animations.get(catalog_clip.name.as_str()) else {
                continue;
            };
            let node = graph.add_clip(handle.clone(), 1.0, root);
            resolved_clips.insert(
                catalog_clip.name.to_ascii_lowercase(),
                ResolvedClip {
                    catalog: catalog_clip.clone(),
                    handle: handle.clone(),
                    node,
                },
            );
        }
        if resolved_clips.is_empty() {
            cached.failed = Some("clip pack contains no catalog-compatible animations".into());
            continue;
        }
        cached.resolved = Some(Arc::new(ResolvedAnimationSet {
            graph: graphs.add(graph),
            pack_targets,
            clips: resolved_clips,
        }));
    }
}

/// Drops the shared strong handles once no resident cell catalog references
/// a set. Actor entities being evicted keep their graph/clip handles alive
/// until despawn, while revisiting the cell reloads a fresh cache entry.
fn prune_unused_animation_sets(
    catalogs: Res<ActorAnimationCatalogs>,
    mut cache: ResMut<AnimationSetCache>,
) {
    cache.0.retain(|set_id, _| catalogs.contains_set(set_id));
}

#[allow(clippy::too_many_arguments)]
fn bind_gameplay_actors(
    mut commands: Commands,
    cache: Res<AnimationSetCache>,
    mut actors: Query<(Entity, &mut ActorAnimationRuntime)>,
    children: Query<&Children>,
    parents: Query<&ChildOf>,
    names: Query<&Name>,
    globals: Query<&GlobalTransform>,
) {
    for (root, mut runtime) in &mut actors {
        // A WorldAsset scene can finish (or refresh) its hierarchy after the
        // animation controller has already bound it. In that case the old
        // retarget entities are despawned and the controller would keep
        // sampling clips into dead targets forever, leaving the newly spawned
        // appearance in its bind/T-pose. Treat a missing bound target as a
        // recoverable load and bind the current hierarchy again.
        if runtime.phase == RuntimePhase::Playing
            && runtime
                .retarget_targets
                .iter()
                .any(|target| globals.get(target.entity).is_err())
        {
            runtime.phase = RuntimePhase::Loading;
            runtime.retarget_targets.clear();
            runtime.selected_clip = None;
            runtime.started_clip = None;
            runtime.previous_clip = None;
            runtime.current_state = ActorAnimationState::Idle;
            runtime.transition_elapsed = TRANSITION_SECONDS;
            runtime.diagnostic = None;
            info!(
                "actor-animation rebinding root={} reason=appearance-hierarchy-changed",
                root
            );
        }
        if runtime.phase != RuntimePhase::Loading {
            continue;
        }
        let Some(cached) = cache.0.get(&runtime.set_id) else {
            continue;
        };
        if let Some(error) = cached.failed.as_ref() {
            runtime.phase = RuntimePhase::Failed;
            runtime.diagnostic = Some(error.clone());
            warn!(
                "actor-animation failed set={} reason={error}",
                runtime.set_id
            );
            continue;
        }
        let Some(set) = cached.resolved.as_ref() else {
            continue;
        };
        // `descendants` now excludes `root` itself (issue #188), so the
        // "scene has not spawned its hierarchy yet" guard is emptiness
        // rather than a length of one.
        let hierarchy = descendants(root, &children);
        if hierarchy.is_empty() {
            continue;
        }
        let Ok(root_global) = globals.get(root) else {
            continue;
        };
        let root_inverse = root_global.to_matrix().inverse();
        let mut targets_by_name = HashMap::<String, Vec<usize>>::new();
        for (index, target) in set.pack_targets.iter().enumerate() {
            targets_by_name
                .entry(target.name_key.clone())
                .or_default()
                .push(index);
        }
        let mut targets = Vec::new();
        let mut bound_source_indices = HashSet::new();
        for entity in hierarchy {
            let Ok(name) = names.get(entity) else {
                continue;
            };
            let Some(candidates) = targets_by_name.get(&animation_node_name_key(name.as_str()))
            else {
                continue;
            };
            let actual_path = appearance_name_path(entity, root, &parents, &names);
            let selected = if candidates.len() == 1 {
                candidates.first().copied()
            } else {
                candidates.iter().find_map(|index| {
                    path_has_case_insensitive_suffix(&actual_path, &set.pack_targets[*index].path)
                        .then_some(*index)
                })
            };
            let Some(source_index) = selected else {
                continue;
            };
            let Ok(global) = globals.get(entity) else {
                continue;
            };
            // Gameplay actors use the same manually retargeted clip pack as
            // the animation zoo, but their appearance hierarchy is spawned
            // separately from the clip-pack GLB. Register the resolved pack
            // target and player on each matched appearance node so Bevy's
            // animation systems know which player owns the target. The
            // explicit retarget pass below still writes the final pose in
            // the actor's local space after AnimationSystems.
            let target_id = set.pack_targets[source_index].id;
            commands
                .entity(entity)
                .insert((target_id, AnimatedBy(root)));
            targets.push(RetargetTarget {
                entity,
                source_index,
                target_rest_root: root_inverse * global.to_matrix(),
            });
            bound_source_indices.insert(source_index);
        }
        if targets.is_empty() {
            runtime.phase = RuntimePhase::Failed;
            runtime.diagnostic =
                Some("clip pack hierarchy did not match prepared actor nodes".into());
            warn!(
                "actor-animation failed set={} reason=no-bound-targets",
                runtime.set_id
            );
            continue;
        }
        let idle_context = ActorAnimationContext {
            kind: runtime.kind,
            female: runtime.female,
            weapon_prefix: None,
        };
        let Some(idle_selection) = resolve_clip(
            &cached.definition.clips,
            idle_context,
            ActorAnimationState::Idle,
        ) else {
            runtime.phase = RuntimePhase::Failed;
            runtime.diagnostic = Some("animation set has no compatible idle clip".into());
            warn!(
                "actor-animation failed set={} reason=no-idle",
                runtime.set_id
            );
            continue;
        };
        let idle = cached
            .definition
            .clips
            .iter()
            .find(|clip| clip.name == idle_selection.clip_name)
            .expect("selection comes from this prepared set");
        let missing_required = missing_required_targets(
            &idle.required_targets,
            set,
            bound_source_indices.iter().copied(),
        );
        if !missing_required.is_empty() {
            let mut missing = missing_required.into_iter().collect::<Vec<_>>();
            missing.sort();
            runtime.phase = RuntimePhase::Failed;
            runtime.diagnostic = Some(format!(
                "prepared actor is missing required animation targets: {}",
                missing.join(", ")
            ));
            warn!(
                "actor-animation failed set={} reason=missing-required-targets count={}",
                runtime.set_id,
                missing.len()
            );
            continue;
        }
        targets.sort_by_key(|target| target.source_index);
        runtime.retarget_targets = targets;
        runtime.phase = RuntimePhase::Playing;
        commands.entity(root).insert((
            AnimationGraphHandle(set.graph.clone()),
            AnimationTransitions::new(),
        ));
        info!(
            "actor-animation ready root={} set={} targets={}",
            root,
            runtime.set_id,
            runtime.retarget_targets.len()
        );
    }
}

#[allow(clippy::type_complexity)]
fn select_gameplay_animation(
    time: Res<Time>,
    item_catalog: Res<PreparedItemCatalog>,
    cache: Res<AnimationSetCache>,
    catalogs: Res<ActorAnimationCatalogs>,
    package_cache: Option<Res<super::ai::catalog_cache::PackageCatalogCache>>,
    active_cell: Option<Res<ActiveCell>>,
    mut actors: Query<(
        &GlobalTransform,
        &InheritedVisibility,
        &ActorRuntime,
        &ActorRuntimeState,
        Option<&super::actor_state::ActorStateRuntime>,
        Option<&super::player::RagdollToggle>,
        Option<&super::ai::family_runtime::ActorPackageController>,
        &mut ActorAnimationRuntime,
        &mut ActorAnimationIntent,
    )>,
) {
    let dt = time.delta_secs().max(f32::EPSILON);
    let now_seconds = time.elapsed_secs();
    for (
        global,
        visibility,
        actor,
        actor_state,
        actor_lifecycle,
        ragdoll,
        package_controller,
        mut runtime,
        mut intent,
    ) in &mut actors
    {
        if runtime.phase != RuntimePhase::Playing {
            continue;
        }
        let position = global.translation();
        let movement = (position - runtime.last_position).xz().length();
        let speed = movement / dt;
        let yaw = global
            .compute_transform()
            .rotation
            .to_euler(EulerRot::YXZ)
            .0;
        let yaw_delta = shortest_angle(yaw - runtime.last_yaw);
        runtime.last_position = position;
        runtime.last_yaw = yaw;
        let measured_locomotion = if speed > RUN_SPEED_METRES_PER_SECOND
            || (runtime.automatic_state == ActorAnimationState::Run
                && speed > RUN_SPEED_METRES_PER_SECOND * 0.8)
        {
            Some(ActorAnimationState::Run)
        } else if movement > MOVEMENT_EPSILON_METRES {
            Some(ActorAnimationState::Walk)
        } else {
            None
        };
        if let Some(state) = measured_locomotion {
            runtime.automatic_state = state;
            runtime.movement_hold_remaining = MOVEMENT_HOLD_SECONDS;
        } else {
            runtime.movement_hold_remaining = (runtime.movement_hold_remaining - dt).max(0.0);
            if runtime.movement_hold_remaining == 0.0 {
                runtime.automatic_state = if yaw_delta > TURN_EPSILON_RADIANS {
                    ActorAnimationState::TurnLeft
                } else if yaw_delta < -TURN_EPSILON_RADIANS {
                    ActorAnimationState::TurnRight
                } else {
                    ActorAnimationState::Idle
                };
            }
        }
        let stationary = runtime.automatic_state == ActorAnimationState::Idle
            && runtime.movement_hold_remaining <= f32::EPSILON;
        let active = active_cell
            .as_ref()
            .is_some_and(|active| active.0 == runtime.cell_form_id);
        let loaded = should_advance_playback(active, visibility.get());
        let package = package_cache
            .as_ref()
            .and_then(|cache| package_idle_context(package_controller, cache));
        let previous_package = runtime.idle_authority.active_package_form_id;
        runtime.idle_authority.on_package_entry(
            package.as_ref().map(|package| package.form_id),
            now_seconds,
            package
                .as_ref()
                .and_then(|package| package.collection.as_ref())
                .map_or(0.0, |collection| collection.timer_seconds),
            stationary,
        );
        if stationary && !runtime.was_stationary {
            runtime
                .idle_authority
                .on_stationary_entry(true, now_seconds);
        } else if !stationary {
            runtime
                .idle_authority
                .on_stationary_entry(false, now_seconds);
        }
        if previous_package != runtime.idle_authority.active_package_form_id
            && runtime.special_idle.is_some()
        {
            runtime
                .idle_authority
                .stop(Some(IdleRejectionReason::PackageTransition));
            runtime.special_idle = None;
            runtime.selected_clip = None;
            runtime.started_clip = None;
            runtime.previous_clip = None;
            info!(
                "actor-idle stop {:08x} reason=package_transition",
                actor.reference_form_id
            );
        }
        runtime.was_stationary = stationary;
        let lifecycle = IdleLifecycleFacts {
            moving: !stationary,
            alive: actor_lifecycle.is_none_or(|state| {
                state.life_state == bevyout_core::actor_state::ActorLifeState::Alive
            }),
            ragdolled: ragdoll.is_some_and(|toggle| toggle.0),
            loaded,
            equipment_transition: intent.requested.is_some_and(|state| {
                matches!(
                    state,
                    ActorAnimationState::Equip | ActorAnimationState::Unequip
                )
            }) || matches!(
                runtime.current_state,
                ActorAnimationState::Equip | ActorAnimationState::Unequip
            ),
        };
        if !lifecycle.special_idle_eligible() {
            if runtime.special_idle.is_some() {
                info!(
                    "actor-idle stop {:08x} reason={}",
                    actor.reference_form_id,
                    lifecycle.rejection().label()
                );
                runtime.special_idle = None;
                runtime.idle_authority.stop(Some(lifecycle.rejection()));
                runtime.selected_clip = None;
                runtime.started_clip = None;
                runtime.previous_clip = None;
                runtime.current_state = runtime.automatic_state;
            }
            intent.requested_idle = None;
        }
        let Some(cached) = cache.0.get(&runtime.set_id) else {
            continue;
        };
        let Some(set) = cached.resolved.as_ref() else {
            continue;
        };
        let definitions = catalogs
            .actor_catalog(actor.reference_form_id, actor.base_form_id)
            .map(|(_, catalog)| catalog.idle_definitions.clone())
            .unwrap_or_default();
        let mut idle_facts = idle_policy::IdleRuntimeFacts {
            weapon_out: actor_state.bound_item_form_id.is_some(),
            last_idle_played: runtime.idle_authority.last_idle_form_id,
            ..idle_policy::IdleRuntimeFacts::default()
        };
        if let Some(state) = actor_lifecycle {
            idle_facts.factions = state
                .definition
                .factions
                .iter()
                .map(|membership| membership.faction_form_id)
                .collect();
        }
        if let Some(item) = actor_state.bound_item_form_id {
            idle_facts.equipped_item_form_ids.insert(item);
        }
        let evaluator = RuntimeIdleConditions;
        if let Some(idle_form_id) = intent.requested_idle.take() {
            let decision = runtime.idle_authority.force_select(
                actor.reference_form_id,
                now_seconds,
                lifecycle,
                &idle_facts,
                &definitions,
                idle_form_id,
                &evaluator,
            );
            if install_special_idle(&mut runtime, set, &definitions, decision) {
                info!(
                    "actor-idle select/play {:08x} source=forced idle={idle_form_id:08x}",
                    actor.reference_form_id
                );
                continue;
            }
            runtime.diagnostic = Some(format!(
                "actor-idle reject {}",
                runtime
                    .idle_authority
                    .last_rejection
                    .map_or("unknown", IdleRejectionReason::label)
            ));
        }
        if stationary
            && runtime.special_idle.is_none()
            && intent.requested.is_none()
            && runtime.current_state == ActorAnimationState::Idle
            && runtime.selected_clip.is_some()
        {
            let due = now_seconds + f32::EPSILON
                >= runtime.idle_authority.next_eligible_evaluation_seconds;
            if due {
                let decision = runtime.idle_authority.select(
                    actor.reference_form_id,
                    now_seconds,
                    lifecycle,
                    &idle_facts,
                    package.as_ref(),
                    &definitions,
                    if package
                        .as_ref()
                        .and_then(|package| package.collection.as_ref())
                        .is_some()
                    {
                        IdleEvaluationTrigger::PackageTimer
                    } else {
                        IdleEvaluationTrigger::BaseIdleLoop
                    },
                    &evaluator,
                );
                if let idle_policy::IdleDecision::Selected(selection) = &decision {
                    info!(
                        "actor-idle select {:08x} source={} idle={:08x}",
                        actor.reference_form_id,
                        selection.source.label(),
                        selection.form_id
                    );
                }
                if install_special_idle(&mut runtime, set, &definitions, decision) {
                    info!(
                        "actor-idle play {:08x} idle={:08x}",
                        actor.reference_form_id,
                        runtime
                            .idle_authority
                            .current_idle_form_id
                            .unwrap_or_default()
                    );
                    continue;
                }
            }
        }
        if !should_advance_playback(active, visibility.get()) {
            continue;
        }
        if intent.requested.is_none()
            && runtime.current_state.is_one_shot()
            && runtime.selected_clip.is_some()
        {
            continue;
        }
        let requested = intent.requested.unwrap_or(runtime.automatic_state);
        if requested == runtime.current_state && runtime.selected_clip.is_some() {
            continue;
        }
        let weapon_prefix = actor_state
            .bound_item_form_id
            .and_then(|form_id| {
                item_catalog
                    .items
                    .iter()
                    .find(|item| item.base_form_id == form_id)
            })
            .and_then(|item| match &item.stats {
                PreparedItemStats::Weapon { animation_type, .. } => *animation_type,
                _ => None,
            })
            .and_then(weapon_animation_prefix);
        let context = ActorAnimationContext {
            kind: runtime.kind,
            female: runtime.female,
            weapon_prefix,
        };
        let Some(mut selection) = resolve_clip(&cached.definition.clips, context, requested) else {
            runtime.diagnostic = Some(format!("no ready clip for {}", requested.label()));
            continue;
        };
        let selected_definition = cached
            .definition
            .clips
            .iter()
            .find(|clip| clip.name == selection.clip_name)
            .expect("selection comes from this prepared set");
        let missing = missing_required_targets(
            &selected_definition.required_targets,
            set,
            runtime
                .retarget_targets
                .iter()
                .map(|target| target.source_index),
        );
        let target_diagnostic = if missing.is_empty() {
            None
        } else if requested != ActorAnimationState::Idle {
            selection = resolve_clip(
                &cached.definition.clips,
                ActorAnimationContext {
                    kind: runtime.kind,
                    female: runtime.female,
                    weapon_prefix: None,
                },
                ActorAnimationState::Idle,
            )
            .expect("binding already validated the canonical idle clip");
            Some(format!(
                "{} missing {} required target(s); fell back to {}",
                requested.label(),
                missing.len(),
                selection.clip_name
            ))
        } else {
            runtime.diagnostic = Some(format!(
                "idle clip is missing {} required animation target(s)",
                missing.len()
            ));
            continue;
        };
        runtime.current_state = requested;
        runtime.selected_clip = Some(selection.clip_name.clone());
        runtime.selected_loop_mode = selection.loop_mode;
        if requested == ActorAnimationState::Idle
            && package
                .as_ref()
                .and_then(|package| package.collection.as_ref())
                .is_none()
        {
            let base_loop_seconds = cached
                .definition
                .clips
                .iter()
                .find(|clip| clip.name == selection.clip_name)
                .and_then(|clip| clip.duration_seconds)
                .unwrap_or(1.0);
            runtime
                .idle_authority
                .schedule_evaluation_after(now_seconds, base_loop_seconds);
        }
        runtime.selected_root_motion = selection.root_motion_policy;
        runtime.diagnostic = target_diagnostic.or_else(|| {
            selection
                .fallback_from
                .map(|state| format!("{} fell back to {}", state.label(), selection.clip_name))
        });
    }
}

fn install_special_idle(
    runtime: &mut ActorAnimationRuntime,
    set: &ResolvedAnimationSet,
    definitions: &[bevyout_core::actor_animation::PreparedActorIdleDefinition],
    decision: IdleDecision,
) -> bool {
    let IdleDecision::Selected(selection) = decision else {
        return false;
    };
    let Some(definition) = definitions
        .iter()
        .find(|definition| definition.form_id == selection.form_id)
    else {
        runtime
            .idle_authority
            .stop(Some(IdleRejectionReason::MissingIdle));
        return false;
    };
    let Some(clip_name) = definition.clip_name.as_deref() else {
        runtime
            .idle_authority
            .stop(Some(IdleRejectionReason::MissingClip));
        return false;
    };
    let Some(clip) = set.clips.get(&clip_name.to_ascii_lowercase()) else {
        runtime
            .idle_authority
            .stop(Some(IdleRejectionReason::MissingClip));
        return false;
    };
    runtime.current_state = ActorAnimationState::Idle;
    runtime.selected_clip = Some(clip_name.to_owned());
    runtime.selected_loop_mode = PreparedActorAnimationLoopMode::Clamp;
    runtime.selected_root_motion = clip.catalog.root_motion_policy;
    runtime.special_idle = Some(SpecialIdlePlayback {
        form_id: selection.form_id,
        loops_remaining: selection.loop_count.max(1),
        source: selection.source,
    });
    runtime.diagnostic = selection
        .diagnostic
        .map(|reason| format!("idle diagnostic {}", reason.label()));
    true
}

fn start_base_idle_after_special(
    player: &mut AnimationPlayer,
    transitions: &mut AnimationTransitions,
    runtime: &mut ActorAnimationRuntime,
    clips: &[PreparedActorAnimationClip],
    set: &ResolvedAnimationSet,
    active: bool,
) {
    let Some(selection) = resolve_clip(
        clips,
        ActorAnimationContext {
            kind: runtime.kind,
            female: runtime.female,
            weapon_prefix: None,
        },
        ActorAnimationState::Idle,
    ) else {
        return;
    };
    let Some(clip) = set.clips.get(&selection.clip_name.to_ascii_lowercase()) else {
        return;
    };
    runtime.current_state = ActorAnimationState::Idle;
    runtime.selected_clip = Some(selection.clip_name.clone());
    runtime.selected_loop_mode = selection.loop_mode;
    runtime.selected_root_motion = selection.root_motion_policy;
    runtime.previous_clip = runtime.started_clip.take();
    runtime.transition_elapsed = if runtime.previous_clip.is_some() {
        0.0
    } else {
        TRANSITION_SECONDS
    };
    let playback = transitions.play(
        player,
        clip.node,
        Duration::from_secs_f32(TRANSITION_SECONDS),
    );
    playback.set_repeat(
        if should_repeat(ActorAnimationState::Idle, clip.catalog.loop_mode) {
            RepeatAnimation::Forever
        } else {
            RepeatAnimation::Never
        },
    );
    if !active {
        playback.pause();
    }
    runtime.started_clip = Some(selection.clip_name.clone());
    info!(
        "actor-animation play state=idle clip={} set={}",
        clip.catalog.name, runtime.set_id
    );
}

fn drive_gameplay_animation(
    time: Res<Time>,
    active_cell: Option<Res<ActiveCell>>,
    cache: Res<AnimationSetCache>,
    mut actors: Query<(
        &mut AnimationPlayer,
        &mut AnimationTransitions,
        &InheritedVisibility,
        &mut ActorAnimationRuntime,
        &mut ActorAnimationIntent,
    )>,
) {
    for (mut player, mut transitions, visibility, mut runtime, mut intent) in &mut actors {
        if runtime.phase != RuntimePhase::Playing {
            continue;
        }
        let active = should_advance_playback(
            active_cell
                .as_ref()
                .is_some_and(|active| active.0 == runtime.cell_form_id),
            visibility.get(),
        );
        let Some(cached) = cache.0.get(&runtime.set_id) else {
            continue;
        };
        let Some(set) = cached.resolved.as_ref() else {
            continue;
        };
        let Some(name) = runtime.selected_clip.clone() else {
            continue;
        };
        let Some(clip) = set.clips.get(&name.to_ascii_lowercase()) else {
            runtime.diagnostic = Some(format!("resolved clip {name} is absent from pack"));
            continue;
        };
        let mut resume_base_idle = false;
        if runtime.started_clip.as_deref() == Some(name.as_str())
            && let Some(animation) = player.animation_mut(clip.node)
        {
            if active {
                animation.resume();
                runtime.transition_elapsed =
                    (runtime.transition_elapsed + time.delta_secs()).min(TRANSITION_SECONDS);
                if runtime.transition_elapsed >= TRANSITION_SECONDS {
                    runtime.previous_clip = None;
                }
            } else {
                animation.pause();
            }
            if animation.is_finished() {
                if let Some(special) = runtime.special_idle.as_mut() {
                    if special.loops_remaining > 1 {
                        special.loops_remaining -= 1;
                        animation.seek_to(0.0);
                        continue;
                    }
                    let form_id = special.form_id;
                    runtime.special_idle = None;
                    runtime.idle_authority.stop(None);
                    runtime.selected_clip = None;
                    runtime.started_clip = None;
                    runtime.previous_clip = None;
                    runtime.transition_elapsed = TRANSITION_SECONDS;
                    runtime.current_state = ActorAnimationState::Idle;
                    intent.requested = None;
                    resume_base_idle = true;
                    info!(
                        "actor-idle stop {:08x} reason=complete idle={form_id:08x}",
                        runtime.idle_authority.last_idle_form_id.unwrap_or_default()
                    );
                } else {
                    let completed = runtime.current_state;
                    let next = state_after_completion(completed);
                    if next != completed {
                        runtime.current_state = next;
                        runtime.selected_clip = None;
                        runtime.started_clip = None;
                        runtime.previous_clip = None;
                        runtime.transition_elapsed = TRANSITION_SECONDS;
                        intent.requested = None;
                    }
                }
            }
            if resume_base_idle {
                start_base_idle_after_special(
                    &mut player,
                    &mut transitions,
                    &mut runtime,
                    &cached.definition.clips,
                    set,
                    active,
                );
            }
            continue;
        }
        runtime.previous_clip = runtime.started_clip.take();
        runtime.transition_elapsed = if runtime.previous_clip.is_some() {
            0.0
        } else {
            TRANSITION_SECONDS
        };
        let playback = transitions.play(
            &mut player,
            clip.node,
            Duration::from_secs_f32(TRANSITION_SECONDS),
        );
        playback.set_repeat(if runtime.special_idle.is_some() {
            RepeatAnimation::Never
        } else if should_repeat(runtime.current_state, clip.catalog.loop_mode) {
            RepeatAnimation::Forever
        } else {
            RepeatAnimation::Never
        });
        if !active {
            playback.pause();
        }
        runtime.started_clip = Some(name);
        info!(
            "actor-animation play state={} clip={} set={}",
            runtime.current_state.label(),
            clip.catalog.name,
            runtime.set_id
        );
    }
}

fn retarget_gameplay_actors(
    cache: Res<AnimationSetCache>,
    active_cell: Option<Res<ActiveCell>>,
    actors: Query<(
        &AnimationPlayer,
        &ActorAnimationRuntime,
        &GlobalTransform,
        &InheritedVisibility,
    )>,
    parents: Query<&ChildOf>,
    globals: Query<&GlobalTransform>,
    clips: Res<Assets<AnimationClip>>,
    mut transforms: Query<&mut Transform>,
) {
    for (player, runtime, root_global, visibility) in &actors {
        if runtime.phase != RuntimePhase::Playing
            || runtime.retarget_targets.is_empty()
            || !should_advance_playback(
                active_cell
                    .as_ref()
                    .is_some_and(|active| active.0 == runtime.cell_form_id),
                visibility.get(),
            )
        {
            continue;
        }
        let Some(cached) = cache.0.get(&runtime.set_id) else {
            continue;
        };
        let Some(set) = cached.resolved.as_ref() else {
            continue;
        };
        let Some(name) = runtime.selected_clip.as_ref() else {
            continue;
        };
        let Some(clip_info) = set.clips.get(&name.to_ascii_lowercase()) else {
            continue;
        };
        let Some(clip) = clips.get(&clip_info.handle) else {
            continue;
        };
        let Some(active) = player.animation(clip_info.node) else {
            continue;
        };
        let time = active.seek_time();
        let accumulation_root = clip_info
            .catalog
            .accumulation_root
            .as_deref()
            .map(animation_node_name_key);
        let previous = runtime
            .previous_clip
            .as_ref()
            .and_then(|name| set.clips.get(&name.to_ascii_lowercase()))
            .and_then(|info| {
                Some((
                    clips.get(&info.handle)?,
                    player.animation(info.node)?.seek_time(),
                    info.catalog
                        .accumulation_root
                        .as_deref()
                        .map(animation_node_name_key),
                ))
            });
        let transition_alpha = (runtime.transition_elapsed / TRANSITION_SECONDS).clamp(0.0, 1.0);
        let mut source_globals = vec![Mat4::IDENTITY; set.pack_targets.len()];
        for (index, target) in set.pack_targets.iter().enumerate() {
            let hold_root_motion =
                is_accumulation_target(accumulation_root.as_deref(), &target.name_key);
            let current =
                source_clip_transform(clip, target.id, time, target.rest_local, hold_root_motion);
            let local = previous.as_ref().map_or(current, |(previous, time, root)| {
                let hold_previous_root = is_accumulation_target(root.as_deref(), &target.name_key);
                let previous = source_clip_transform(
                    previous,
                    target.id,
                    *time,
                    target.rest_local,
                    hold_previous_root,
                );
                blend_transform(previous, current, transition_alpha)
            });
            source_globals[index] = target
                .parent
                .map_or(Mat4::IDENTITY, |parent| source_globals[parent])
                * local.to_matrix();
        }
        let root_inverse = root_global.to_matrix().inverse();
        let mut desired_root = HashMap::<Entity, Mat4>::new();
        for target in &runtime.retarget_targets {
            let source = &set.pack_targets[target.source_index];
            let desired = target.target_rest_root
                * source.rest_global.inverse()
                * source_globals[target.source_index];
            let parent_root = parents
                .get(target.entity)
                .ok()
                .and_then(|parent| desired_root.get(&parent.parent()).copied())
                .or_else(|| {
                    parents
                        .get(target.entity)
                        .ok()
                        .and_then(|parent| globals.get(parent.parent()).ok())
                        .map(|global| root_inverse * global.to_matrix())
                })
                .unwrap_or(Mat4::IDENTITY);
            if let Ok(mut transform) = transforms.get_mut(target.entity) {
                *transform = Transform::from_matrix(parent_root.inverse() * desired);
            }
            desired_root.insert(target.entity, desired);
        }
    }
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

/// Every entity strictly *below* `root`, never `root` itself.
///
/// The exclusion is load-bearing, not tidiness (issue #188 feature 4). The
/// only consumer is `bind_gameplay_actors`, whose results become
/// `retarget_gameplay_actors`' write targets. `root` is the actor's
/// placement root -- and, for a nav-bound actor, the very `Transform` the
/// KCC owns. If a clip-pack node name ever matched the root's own `Name`,
/// retargeting would start writing that transform each frame and animation
/// would silently become a second movement authority: precisely the shape
/// the navmesh post-mortem's §2.3 records `landmass`'s border ORCA taking
/// for the navigation feature's entire life. Skeletons live under the root
/// by construction, so nothing legitimate is lost by refusing to look at it.
fn descendants(root: Entity, children: &Query<&Children>) -> Vec<Entity> {
    let mut output = Vec::new();
    let Ok(root_children) = children.get(root) else {
        return output;
    };
    let mut pending: Vec<Entity> = root_children.iter().collect();
    while let Some(entity) = pending.pop() {
        output.push(entity);
        if let Ok(children) = children.get(entity) {
            pending.extend(children.iter());
        }
    }
    output
}

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

fn missing_required_targets(
    required_targets: &[String],
    set: &ResolvedAnimationSet,
    bound_indices: impl IntoIterator<Item = usize>,
) -> HashSet<String> {
    let bound_indices = bound_indices.into_iter().collect::<HashSet<_>>();
    required_targets
        .iter()
        // `##` nodes are Gamebryo controller pins (visibility, IK, grip),
        // not deform bones in the assembled actor appearance. They remain in
        // clip metadata but do not participate in pose retargeting.
        .filter(|required| requires_appearance_binding(required))
        .filter(|required| {
            let required = animation_node_name_key(required);
            !set.pack_targets.iter().enumerate().any(|(index, target)| {
                target.name_key == required && bound_indices.contains(&index)
            })
        })
        .cloned()
        .collect()
}

fn requires_appearance_binding(target: &str) -> bool {
    !target.starts_with("##")
}

fn is_accumulation_target(accumulation_root: Option<&str>, target: &str) -> bool {
    accumulation_root.is_some_and(|root| target == root || target == format!("{root} nonaccum"))
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

fn blend_transform(from: Transform, to: Transform, amount: f32) -> Transform {
    Transform {
        translation: from.translation.lerp(to.translation, amount),
        rotation: from.rotation.slerp(to.rotation, amount),
        scale: from.scale.lerp(to.scale, amount),
    }
}

fn shortest_angle(value: f32) -> f32 {
    (value + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

#[cfg(test)]
#[path = "tests/idle_policy.rs"]
mod idle_policy_tests;

#[cfg(test)]
#[path = "tests/runtime.rs"]
mod runtime_tests;
