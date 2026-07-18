//! Isolated A/B laboratory for prepared articulated actors.
//!
//! This module deliberately does not install `ViewerPlugins`: the market,
//! player, navigation, streaming, and production BoxDDD world are absent.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use avian3d::prelude as avian;
use avian3d::schedule::PhysicsTime;
use bevy::gltf::GltfAssetLabel;
use bevy::prelude::*;
use bevy::transform::TransformSystems;
use bevy_boxddd::boxddd::{
    self, BodyDef, BodyId, BodyType, BoxHull, JointId, PrismaticJointDef, RevoluteJointDef,
    ShapeDef, SphericalJointDef,
};
use bevy_boxddd::prelude::{BoxdddPhysicsContext, BoxdddPhysicsPlugin, BoxdddPhysicsSettings};
use bevy_boxddd::resources::BoxdddErrorPolicy;
use glam::Affine3A;
use ron::de::from_str;
use serde::Serialize;

use crate::cli::{RagdollLabArgs, RagdollLabBackend};
use crate::vsa::{
    PreparedPhysicsAsset, PreparedPhysicsBody, PreparedPhysicsShape, PreparedPlacement,
    PreparedSceneManifest, PreparedSemantic, find_cached_manifest, read_physics_asset,
};

use super::LoadedSceneManifest;
use super::agent_bridge::RagdollLabAgentBridgePlugin;
use super::player::{
    PreparedShapeOptions, actor_node_names_match, create_dynamic_body_at_local_anchor,
    create_prepared_shape, normalize_dynamic_mass, ragdoll_joint_local_anchor,
    ragdoll_local_transform, ragdoll_resolved_world, recenter_ragdoll_body, tune_ragdoll_body,
};

const LAB_DROP_HEIGHT: f32 = 1.75;
const LAB_FLOOR_HALF_EXTENTS: Vec3 = Vec3::new(6.0, 0.25, 6.0);
const LAB_FLOOR_CENTER: Vec3 = Vec3::new(0.0, -0.25, 0.0);
const LAB_WORLD_LAYER: u32 = 1 << 0;
const LAB_RAGDOLL_LAYER: u32 = 1 << 1;
const DIAGNOSTIC_TIMES: [f32; 3] = [0.25, 1.25, 3.5];

fn lab_floor_collision_layers() -> avian::CollisionLayers {
    avian::CollisionLayers::from_bits(LAB_WORLD_LAYER, LAB_RAGDOLL_LAYER)
}

fn lab_ragdoll_collision_layers() -> avian::CollisionLayers {
    avian::CollisionLayers::from_bits(LAB_RAGDOLL_LAYER, LAB_WORLD_LAYER)
}

fn lab_contact_friction(coefficient: f32) -> avian::Friction {
    avian::Friction::new(coefficient.max(0.6)).with_combine_rule(avian::CoefficientCombine::Max)
}

fn lab_sleep_threshold() -> avian::SleepThreshold {
    avian::SleepThreshold {
        linear: 0.2,
        angular: 0.8,
    }
}

pub fn ragdoll_lab(args: RagdollLabArgs) -> Result<()> {
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
    let asset_path = placement
        .asset_path
        .as_ref()
        .context("prepared actor has no GLB asset path")?;
    let physics_path = placement
        .physics_asset_path
        .as_ref()
        .context("prepared actor has no physics sidecar")?;
    let asset_root = PathBuf::from(&manifest.asset_root);
    require_file(&asset_root.join(asset_path), "actor GLB")?;
    let physics_file = asset_root.join(physics_path);
    require_file(&physics_file, "actor physics sidecar")?;
    let physics = read_physics_asset(&physics_file)
        .with_context(|| format!("could not read {}", physics_file.display()))?;
    validate_actor_physics(&physics)?;

    let definition = RagdollLabDefinition {
        backend: args.backend,
        actor_form_id,
        asset_path: asset_path.clone(),
        placement: lab_placement(&placement),
        physics,
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
                        "bevyout ragdoll lab - {:08x} ({})",
                        actor_form_id, args.backend
                    ),
                    ..default()
                }),
                ..default()
            }),
    );
    match args.backend {
        RagdollLabBackend::Avian => {
            app.add_plugins(avian::PhysicsPlugins::default())
                .insert_resource(avian::SubstepCount(8))
                .insert_resource(avian::Gravity(Vec3::new(0.0, -9.81, 0.0)));
        }
        RagdollLabBackend::Boxddd => {
            app.add_plugins(BoxdddPhysicsPlugin::new(BoxdddPhysicsSettings {
                gravity: Vec3::new(0.0, -9.81, 0.0),
                sub_step_count: 8,
                error_policy: BoxdddErrorPolicy::MessageAndLog,
                ..default()
            }));
        }
    }
    if args.agent_bridge {
        app.add_plugins(RagdollLabAgentBridgePlugin {
            port: args.agent_port,
        });
    }
    app.insert_resource(LoadedSceneManifest(manifest))
        .insert_resource(definition)
        .init_resource::<RagdollLabRuntime>()
        .init_resource::<LabPoseCache>()
        .init_resource::<RagdollLabProbe>()
        .add_systems(Startup, spawn_laboratory)
        .add_systems(
            Update,
            (
                resolve_actor_nodes,
                activate_avian_ragdoll,
                activate_boxddd_ragdoll,
                update_avian_poses,
                update_boxddd_poses,
                lab_controls,
                reset_avian_ragdoll,
                reset_boxddd_ragdoll,
                draw_lab_reference,
                update_lab_hud,
                exit_after_trace,
            )
                .chain(),
        )
        .add_systems(
            PostUpdate,
            sync_ragdoll_nodes.before(TransformSystems::Propagate),
        )
        .add_systems(
            PostUpdate,
            update_lab_diagnostics.after(TransformSystems::Propagate),
        )
        .run();
    Ok(())
}

fn read_manifest(path: &Path) -> Result<PreparedSceneManifest> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("could not read prepared scene {}", path.display()))?;
    from_str(&text).with_context(|| format!("invalid prepared scene {}", path.display()))
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
        .ok_or_else(|| {
            anyhow::anyhow!("actor reference {form_id:08x} is not in the prepared scene")
        })?;
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

fn validate_actor_physics(asset: &PreparedPhysicsAsset) -> Result<()> {
    let usable = asset
        .bodies
        .iter()
        .filter(|body| !body.phantom && !body.shapes.is_empty())
        .collect::<Vec<_>>();
    let body_ids = usable
        .iter()
        .map(|body| body.group_id)
        .collect::<HashSet<_>>();
    if body_ids.is_empty() {
        bail!("actor physics sidecar contains no usable bodies");
    }
    if body_ids.len() != usable.len() {
        bail!("actor physics sidecar contains duplicate body group IDs");
    }
    let missing = asset
        .joints
        .iter()
        .filter(|joint| !body_ids.contains(&joint.body_a) || !body_ids.contains(&joint.body_b))
        .map(|joint| format!("{}-{}", joint.body_a, joint.body_b))
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        bail!(
            "actor physics sidecar has unresolved joint endpoints: {}",
            missing.join(", ")
        );
    }
    let Some(&root) = body_ids.iter().min() else {
        unreachable!("empty body sets return above");
    };
    let mut connected = HashSet::from([root]);
    loop {
        let before = connected.len();
        for joint in &asset.joints {
            if connected.contains(&joint.body_a) {
                connected.insert(joint.body_b);
            }
            if connected.contains(&joint.body_b) {
                connected.insert(joint.body_a);
            }
        }
        if connected.len() == before {
            break;
        }
    }
    let mut disconnected = body_ids.difference(&connected).copied().collect::<Vec<_>>();
    disconnected.sort_unstable();
    if !disconnected.is_empty() {
        bail!(
            "actor physics sidecar has a disconnected joint graph; unreachable bodies: {}",
            disconnected
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    Ok(())
}

fn lab_placement(source: &PreparedPlacement) -> PreparedPlacement {
    let mut placement = source.clone();
    placement.translation = [0.0, LAB_DROP_HEIGHT, 0.0];
    placement.rotation_xyzw = [0.0, 0.0, 0.0, 1.0];
    placement
}

#[derive(Resource)]
struct RagdollLabDefinition {
    backend: RagdollLabBackend,
    actor_form_id: u32,
    asset_path: String,
    placement: PreparedPlacement,
    physics: PreparedPhysicsAsset,
    trace_seconds: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
enum LabPhase {
    #[default]
    Loading,
    Ready,
    Active,
    Paused,
    Failed,
}


#[derive(Resource, Default)]
struct RagdollLabRuntime {
    phase: LabPhase,
    actor_root: Option<Entity>,
    resolved_nodes: HashMap<u32, Vec<Entity>>,
    node_rest_locals: HashMap<Entity, Transform>,
    node_rest_globals: HashMap<Entity, Affine3A>,
    bindings: Vec<LabNodeBinding>,
    body_bounds: HashMap<u32, Vec<LabBoundSample>>,
    bodies: HashMap<u32, LabBodyHandle>,
    joints: Vec<LabJointHandle>,
    joint_diagnostics: Vec<LabJointDiagnostic>,
    physics_entities: Vec<Entity>,
    reset_requested: bool,
    reset_count: u32,
    drop_elapsed: f32,
    emitted_snapshots: usize,
    failure: Option<String>,
    boxddd_floor_ready: bool,
}

#[derive(Clone, Copy)]
enum LabBodyHandle {
    Avian(Entity),
    Boxddd(BodyId),
}

#[derive(Clone, Copy)]
enum LabJointHandle {
    Avian(Entity),
    Boxddd(JointId),
}

struct LabNodeBinding {
    group_id: u32,
    nodes: Vec<Entity>,
    rest_body: Affine3A,
    rest_node_globals: Vec<Affine3A>,
}

struct LabJointDiagnostic {
    body_a: u32,
    body_b: u32,
    local_anchor_a: Vec3,
    local_anchor_b: Vec3,
}

#[derive(Clone, Copy)]
struct LabBoundSample {
    point: Vec3,
    radius: f32,
}

#[derive(Clone, Copy)]
struct BodySample {
    affine: Affine3A,
    linear_speed: f32,
    angular_speed: f32,
    sleeping: bool,
}

#[derive(Resource, Default)]
struct LabPoseCache(HashMap<u32, BodySample>);

#[derive(Resource, Debug, Clone, Default, Serialize)]
pub(crate) struct RagdollLabProbe {
    pub(crate) backend: String,
    pub(crate) actor_form_id: String,
    pub(crate) phase: String,
    pub(crate) body_count: usize,
    pub(crate) joint_count: usize,
    pub(crate) expected_joint_count: usize,
    pub(crate) missing_joints: usize,
    pub(crate) awake_bodies: usize,
    pub(crate) maximum_speed: f32,
    pub(crate) maximum_linear_speed: f32,
    pub(crate) maximum_angular_speed: f32,
    pub(crate) maximum_anchor_separation: f32,
    pub(crate) maximum_node_position_error: f32,
    pub(crate) floor_penetration: f32,
    pub(crate) sleeping: bool,
    pub(crate) elapsed_drop_time: f32,
    pub(crate) orphaned_entities_after_reset: usize,
    pub(crate) reset_count: u32,
    pub(crate) error: Option<String>,
}

#[derive(Component)]
struct LabActorRoot;

#[derive(Component)]
struct LabPhysicsEntity;

#[derive(Component)]
struct LabRagdollEntity;

#[derive(Component)]
struct LabHud;

fn spawn_laboratory(
    mut commands: Commands,
    definition: Res<RagdollLabDefinition>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut runtime: ResMut<RagdollLabRuntime>,
) {
    let floor_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.13, 0.15),
        perceptual_roughness: 0.92,
        metallic: 0.0,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(12.0, 0.5, 12.0))),
        MeshMaterial3d(floor_material),
        Transform::from_translation(LAB_FLOOR_CENTER),
    ));
    if definition.backend == RagdollLabBackend::Avian {
        commands.spawn((
            avian::RigidBody::Static,
            avian::Collider::cuboid(12.0, 0.5, 12.0),
            lab_floor_collision_layers(),
            lab_contact_friction(1.0),
            avian::Restitution::ZERO,
            Transform::from_translation(LAB_FLOOR_CENTER),
            LabPhysicsEntity,
        ));
    }
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
                    .load(GltfAssetLabel::Scene(0).from_asset(definition.asset_path.clone())),
            ),
            Transform::from_translation(Vec3::new(0.0, LAB_DROP_HEIGHT, 0.0))
                .with_scale(Vec3::splat(definition.placement.scale.abs().max(0.0001))),
            LabActorRoot,
        ))
        .id();
    runtime.actor_root = Some(root);
    commands.spawn((
        Text::new("Ragdoll lab: loading prepared actor..."),
        LabHud,
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

fn resolve_actor_nodes(
    definition: Res<RagdollLabDefinition>,
    mut runtime: ResMut<RagdollLabRuntime>,
    children: Query<&Children>,
    names: Query<&Name>,
    transforms: Query<(&Transform, &GlobalTransform)>,
) {
    if runtime.phase != LabPhase::Loading {
        return;
    }
    let Some(root) = runtime.actor_root else {
        return;
    };
    let descendants = descendants(root, &children);
    if descendants.len() <= 1 {
        return;
    }
    let named = descendants
        .iter()
        .filter_map(|entity| names.get(*entity).ok().map(|name| (*entity, name.as_str())))
        .collect::<Vec<_>>();
    if named.is_empty() {
        return;
    }
    let mut resolved = HashMap::new();
    let mut missing = Vec::new();
    for body in usable_bodies(&definition.physics) {
        let Some(node) = body.node.as_deref() else {
            missing.push(format!("body {} has no skeleton node", body.group_id));
            continue;
        };
        let matches = named
            .iter()
            .filter_map(|(entity, scene_name)| {
                actor_node_names_match(node, scene_name).then_some(*entity)
            })
            .collect::<Vec<_>>();
        if matches.is_empty() {
            missing.push(format!("body {} node '{node}'", body.group_id));
        } else {
            resolved.insert(body.group_id, matches);
        }
    }
    if !missing.is_empty() {
        runtime.failure = Some(format!(
            "incomplete actor skeleton resolution: {}",
            missing.join(", ")
        ));
        runtime.phase = LabPhase::Failed;
        return;
    }
    for nodes in resolved.values() {
        for node in nodes {
            if let Ok((local, global)) = transforms.get(*node) {
                runtime.node_rest_locals.insert(*node, *local);
                runtime.node_rest_globals.insert(*node, global.affine());
            } else {
                runtime.failure = Some(format!("resolved skeleton node {node:?} has no transform"));
                runtime.phase = LabPhase::Failed;
                return;
            }
        }
    }
    runtime.resolved_nodes = resolved;
    runtime.phase = LabPhase::Ready;
}

fn usable_bodies(asset: &PreparedPhysicsAsset) -> impl Iterator<Item = &PreparedPhysicsBody> {
    asset
        .bodies
        .iter()
        .filter(|body| !body.phantom && !body.shapes.is_empty())
}

fn prepared_dynamic_body(source: &PreparedPhysicsBody) -> Option<(PreparedPhysicsBody, Vec3)> {
    let mut body = source.clone();
    body.motion_type = "MO_SYS_DYNAMIC".into();
    if !body.gravity_factor.is_finite() {
        body.gravity_factor = 1.0;
    }
    body.mass = if body.mass.is_finite() && body.mass > 0.0 {
        body.mass
    } else {
        5.0
    };
    tune_ragdoll_body(&mut body);
    body.linear_damping = body.linear_damping.max(0.6);
    body.angular_damping = body.angular_damping.max(1.0);
    body.constrained = false;
    body.shapes.retain(PreparedPhysicsShape::supports_dynamic);
    if body.shapes.is_empty() {
        return None;
    }
    let anchor = recenter_ragdoll_body(&mut body);
    Some((body, anchor))
}

fn activate_avian_ragdoll(
    mut commands: Commands,
    definition: Res<RagdollLabDefinition>,
    mut runtime: ResMut<RagdollLabRuntime>,
) {
    if definition.backend != RagdollLabBackend::Avian || runtime.phase != LabPhase::Ready {
        return;
    }
    let scale = definition.placement.scale.abs().max(0.0001);
    let origin = Vec3::from_array(definition.placement.translation);
    let prepared = usable_bodies(&definition.physics)
        .map(|source| {
            prepared_dynamic_body(source)
                .filter(|(body, _)| {
                    body.shapes
                        .iter()
                        .all(|shape| avian_shape(shape, scale).is_some())
                })
                .map(|body| (source.group_id, body))
        })
        .collect::<Option<Vec<_>>>();
    let Some(prepared) = prepared else {
        fail_runtime(
            &mut runtime,
            "at least one body has no Avian-compatible collider",
        );
        return;
    };
    let mut body_anchors = HashMap::new();
    for (group_id, (body, anchor)) in prepared {
        runtime
            .body_bounds
            .insert(group_id, body_bound_samples(&body, scale));
        let entity = spawn_avian_body(&mut commands, &body, origin + anchor * scale, scale);
        runtime.physics_entities.push(entity);
        runtime
            .bodies
            .insert(group_id, LabBodyHandle::Avian(entity));
        body_anchors.insert(group_id, anchor);
        add_node_binding(&mut runtime, group_id, origin + anchor * scale);
    }
    if !all_joint_endpoints_resolved(&definition.physics, &runtime.bodies) {
        fail_runtime(
            &mut runtime,
            "Avian joint endpoint resolution changed during activation",
        );
        return;
    }
    for joint in &definition.physics.joints {
        let (LabBodyHandle::Avian(body_a), LabBodyHandle::Avian(body_b)) =
            (runtime.bodies[&joint.body_a], runtime.bodies[&joint.body_b])
        else {
            unreachable!("backend-specific handles are checked above");
        };
        let anchor_a =
            ragdoll_joint_local_anchor(joint.anchor_a, body_anchors[&joint.body_a], scale);
        let anchor_b =
            ragdoll_joint_local_anchor(joint.anchor_b, body_anchors[&joint.body_b], scale);
        let frame_a = local_joint_frame(joint.frame_a_rotation_xyzw);
        let frame_b = local_joint_frame(joint.frame_b_rotation_xyzw);
        let joint_entity = match joint.kind.as_str() {
            "spherical" => {
                let mut value = avian::SphericalJoint::new(body_a, body_b)
                    .with_local_anchor1(anchor_a)
                    .with_local_anchor2(anchor_b)
                    .with_local_basis1(frame_a)
                    .with_local_basis2(frame_b)
                    .with_twist_axis(avian_twist_axis());
                if let Some(cone) = joint.cone_limit {
                    let cone = cone.max(0.0);
                    value = value.with_swing_limits(-cone, cone);
                }
                if let (Some(lower), Some(upper)) =
                    (joint.twist_lower_limit, joint.twist_upper_limit)
                {
                    value = value.with_twist_limits(lower, upper.max(lower));
                }
                commands
                    .spawn((
                        value,
                        avian::JointCollisionDisabled,
                        LabPhysicsEntity,
                        LabRagdollEntity,
                    ))
                    .id()
            }
            "prismatic" => {
                let mut value = avian::PrismaticJoint::new(body_a, body_b)
                    .with_local_anchor1(anchor_a)
                    .with_local_anchor2(anchor_b)
                    .with_local_basis1(frame_a)
                    .with_local_basis2(frame_b)
                    .with_slider_axis(Vec3::X);
                let (enabled, lower, upper) =
                    ordered_limits(joint.lower_limit, joint.upper_limit, scale);
                if enabled {
                    value = value.with_limits(lower, upper);
                }
                commands
                    .spawn((
                        value,
                        avian::JointCollisionDisabled,
                        LabPhysicsEntity,
                        LabRagdollEntity,
                    ))
                    .id()
            }
            _ => {
                let mut value = avian::RevoluteJoint::new(body_a, body_b)
                    .with_local_anchor1(anchor_a)
                    .with_local_anchor2(anchor_b)
                    .with_local_basis1(frame_a)
                    .with_local_basis2(frame_b)
                    .with_hinge_axis(Vec3::Z);
                if joint.lower_limit.is_some() || joint.upper_limit.is_some() {
                    let lower = joint.lower_limit.unwrap_or(0.0);
                    let upper = joint.upper_limit.unwrap_or(lower).max(lower);
                    value = value.with_angle_limits(lower, upper);
                }
                commands
                    .spawn((
                        value,
                        avian::JointCollisionDisabled,
                        LabPhysicsEntity,
                        LabRagdollEntity,
                    ))
                    .id()
            }
        };
        runtime.physics_entities.push(joint_entity);
        runtime.joints.push(LabJointHandle::Avian(joint_entity));
        runtime.joint_diagnostics.push(LabJointDiagnostic {
            body_a: joint.body_a,
            body_b: joint.body_b,
            local_anchor_a: anchor_a,
            local_anchor_b: anchor_b,
        });
    }
    finish_activation(&mut runtime);
}

fn spawn_avian_body(
    commands: &mut Commands,
    body: &PreparedPhysicsBody,
    position: Vec3,
    scale: f32,
) -> Entity {
    let mut body_commands = commands.spawn((
        avian::RigidBody::Dynamic,
        Transform::from_translation(position),
        avian::LinearVelocity(Vec3::ZERO),
        avian::AngularVelocity(Vec3::ZERO),
        avian::GravityScale(body.gravity_factor),
        avian::LinearDamping(body.linear_damping.max(0.0)),
        avian::AngularDamping(body.angular_damping.max(0.0)),
        avian::Mass(body.mass),
        avian::CenterOfMass(Vec3::from_array(body.center_of_mass) * scale),
        avian::NoAutoMass,
        avian::NoAutoCenterOfMass,
        lab_sleep_threshold(),
        LabPhysicsEntity,
        LabRagdollEntity,
    ));
    if body.max_linear_velocity > 0.0 {
        body_commands.insert(avian::MaxLinearSpeed(body.max_linear_velocity * scale));
    }
    if body.max_angular_velocity > 0.0 {
        body_commands.insert(avian::MaxAngularSpeed(body.max_angular_velocity));
    }
    if body.ccd_enabled {
        body_commands.insert(avian::SweptCcd::default());
    }
    if !body.sleep_enabled {
        body_commands.insert(avian::SleepingDisabled);
    }
    if let Some(inertia) = avian_inertia(body, scale) {
        body_commands.insert((inertia, avian::NoAutoAngularInertia));
    }
    let entity = body_commands.id();
    commands.entity(entity).with_children(|children| {
        for shape in &body.shapes {
            if let Some((collider, transform)) = avian_shape(shape, scale) {
                children.spawn((
                    collider,
                    transform,
                    lab_ragdoll_collision_layers(),
                    lab_contact_friction(body.friction),
                    avian::Restitution::new(body.restitution.max(0.0))
                        .with_combine_rule(avian::CoefficientCombine::Max),
                ));
            }
        }
    });
    entity
}

fn avian_inertia(body: &PreparedPhysicsBody, scale: f32) -> Option<avian::AngularInertia> {
    if body
        .inertia
        .iter()
        .flatten()
        .any(|value| !value.is_finite())
        || body.inertia[0][0] <= 0.0
        || body.inertia[1][1] <= 0.0
        || body.inertia[2][2] <= 0.0
    {
        return None;
    }
    let factor = scale * scale;
    let matrix = Mat3::from_cols(
        Vec3::from_array(body.inertia[0]) * factor,
        Vec3::from_array(body.inertia[1]) * factor,
        Vec3::from_array(body.inertia[2]) * factor,
    );
    avian::AngularInertia::try_from_mat3(matrix).ok()
}

fn avian_shape(shape: &PreparedPhysicsShape, scale: f32) -> Option<(avian::Collider, Transform)> {
    match shape {
        PreparedPhysicsShape::Box {
            center,
            half_extents,
            rotation_xyzw,
        } => {
            let half = Vec3::from_array(*half_extents) * scale;
            Some((
                avian::Collider::cuboid(half.x * 2.0, half.y * 2.0, half.z * 2.0),
                Transform {
                    translation: Vec3::from_array(*center) * scale,
                    rotation: Quat::from_array(*rotation_xyzw).normalize(),
                    ..default()
                },
            ))
        }
        PreparedPhysicsShape::Sphere { center, radius } => Some((
            avian::Collider::sphere(radius * scale),
            Transform::from_translation(Vec3::from_array(*center) * scale),
        )),
        PreparedPhysicsShape::Capsule {
            point1,
            point2,
            radius,
        } => Some((
            avian::Collider::capsule_endpoints(
                radius * scale,
                Vec3::from_array(*point1) * scale,
                Vec3::from_array(*point2) * scale,
            ),
            Transform::IDENTITY,
        )),
        PreparedPhysicsShape::ConvexHull { points } => avian::Collider::convex_hull(
            points
                .iter()
                .map(|point| Vec3::from_array(*point) * scale)
                .collect(),
        )
        .map(|collider| (collider, Transform::IDENTITY)),
        PreparedPhysicsShape::TriangleMesh { .. } => None,
    }
}

fn avian_twist_axis() -> Vec3 {
    Vec3::Z
}

fn local_joint_frame(rotation_xyzw: [f32; 4]) -> Quat {
    Quat::from_array(rotation_xyzw).normalize()
}

fn body_bound_samples(body: &PreparedPhysicsBody, scale: f32) -> Vec<LabBoundSample> {
    let mut samples = Vec::new();
    for shape in &body.shapes {
        match shape {
            PreparedPhysicsShape::Box {
                center,
                half_extents,
                rotation_xyzw,
            } => {
                let center = Vec3::from_array(*center) * scale;
                let half = Vec3::from_array(*half_extents) * scale;
                let rotation = Quat::from_array(*rotation_xyzw).normalize();
                for x in [-half.x, half.x] {
                    for y in [-half.y, half.y] {
                        for z in [-half.z, half.z] {
                            samples.push(LabBoundSample {
                                point: center + rotation * Vec3::new(x, y, z),
                                radius: 0.0,
                            });
                        }
                    }
                }
            }
            PreparedPhysicsShape::Sphere { center, radius } => samples.push(LabBoundSample {
                point: Vec3::from_array(*center) * scale,
                radius: radius * scale,
            }),
            PreparedPhysicsShape::Capsule {
                point1,
                point2,
                radius,
            } => {
                for point in [point1, point2] {
                    samples.push(LabBoundSample {
                        point: Vec3::from_array(*point) * scale,
                        radius: radius * scale,
                    });
                }
            }
            PreparedPhysicsShape::ConvexHull { points } => {
                samples.extend(points.iter().map(|point| LabBoundSample {
                    point: Vec3::from_array(*point) * scale,
                    radius: 0.0,
                }));
            }
            PreparedPhysicsShape::TriangleMesh { .. } => {}
        }
    }
    samples
}

fn ordered_limits(lower: Option<f32>, upper: Option<f32>, scale: f32) -> (bool, f32, f32) {
    let enabled = lower.is_some() || upper.is_some();
    let lower = lower.unwrap_or(0.0);
    let upper = upper.unwrap_or(lower).max(lower);
    (enabled, lower * scale, upper * scale)
}

fn activate_boxddd_ragdoll(
    definition: Res<RagdollLabDefinition>,
    mut runtime: ResMut<RagdollLabRuntime>,
    mut context: Option<NonSendMut<BoxdddPhysicsContext>>,
) {
    if definition.backend != RagdollLabBackend::Boxddd || runtime.phase != LabPhase::Ready {
        return;
    }
    let Some(context) = context.as_deref_mut() else {
        return;
    };
    let Some(world) = context.world_mut() else {
        return;
    };
    if !runtime.boxddd_floor_ready {
        let floor = world.create_body(
            BodyDef::builder()
                .body_type(BodyType::Static)
                .position(to_box_vec3(LAB_FLOOR_CENTER))
                .build(),
        );
        world.create_hull_shape(
            floor,
            &ShapeDef::default(),
            &BoxHull::new(
                LAB_FLOOR_HALF_EXTENTS.x,
                LAB_FLOOR_HALF_EXTENTS.y,
                LAB_FLOOR_HALF_EXTENTS.z,
            ),
        );
        runtime.boxddd_floor_ready = true;
    }
    let scale = definition.placement.scale.abs().max(0.0001);
    let prepared = usable_bodies(&definition.physics)
        .map(|source| prepared_dynamic_body(source).map(|body| (source.group_id, body)))
        .collect::<Option<Vec<_>>>();
    let Some(prepared) = prepared else {
        fail_runtime(
            &mut runtime,
            "at least one body has no BoxDDD-compatible collider",
        );
        return;
    };
    let mut body_anchors = HashMap::new();
    for (group_id, (body, anchor)) in prepared {
        runtime
            .body_bounds
            .insert(group_id, body_bound_samples(&body, scale));
        let body_id =
            create_dynamic_body_at_local_anchor(world, &definition.placement, &body, anchor);
        let shape_count = body
            .shapes
            .iter()
            .filter_map(|shape| {
                create_prepared_shape(
                    world,
                    body_id,
                    &body,
                    shape,
                    &definition.placement,
                    PreparedShapeOptions {
                        dynamic: true,
                        local_space: true,
                        collision_group: 0,
                    },
                )
            })
            .count();
        if shape_count != body.shapes.len() {
            let _ = world.try_destroy_body(body_id);
            destroy_boxddd_ragdoll(world, &mut runtime);
            fail_runtime(
                &mut runtime,
                format!(
                    "body {group_id} produced {shape_count}/{} BoxDDD shapes",
                    body.shapes.len()
                ),
            );
            return;
        }
        normalize_dynamic_mass(world, body_id, &body, scale);
        runtime
            .bodies
            .insert(group_id, LabBodyHandle::Boxddd(body_id));
        body_anchors.insert(group_id, anchor);
        let position = Vec3::from_array(definition.placement.translation) + anchor * scale;
        add_node_binding(&mut runtime, group_id, position);
    }
    if !all_joint_endpoints_resolved(&definition.physics, &runtime.bodies) {
        fail_runtime(
            &mut runtime,
            "BoxDDD joint endpoint resolution changed during activation",
        );
        return;
    }
    for joint in &definition.physics.joints {
        let (LabBodyHandle::Boxddd(body_a), LabBodyHandle::Boxddd(body_b)) =
            (runtime.bodies[&joint.body_a], runtime.bodies[&joint.body_b])
        else {
            unreachable!("backend-specific handles are checked above");
        };
        let anchor_a =
            ragdoll_joint_local_anchor(joint.anchor_a, body_anchors[&joint.body_a], scale);
        let anchor_b =
            ragdoll_joint_local_anchor(joint.anchor_b, body_anchors[&joint.body_b], scale);
        let frame_a = boxddd::Transform::new(
            to_box_vec3(anchor_a),
            to_box_quat(local_joint_frame(joint.frame_a_rotation_xyzw)),
        );
        let frame_b = boxddd::Transform::new(
            to_box_vec3(anchor_b),
            to_box_quat(local_joint_frame(joint.frame_b_rotation_xyzw)),
        );
        let created = match joint.kind.as_str() {
            "spherical" => {
                let mut value = SphericalJointDef::new(body_a, body_b)
                    .local_frame_a(frame_a)
                    .local_frame_b(frame_b)
                    .collide_connected(false);
                if let Some(cone) = joint.cone_limit {
                    value = value.cone_limit(true, cone.max(0.0));
                }
                if let (Some(lower), Some(upper)) =
                    (joint.twist_lower_limit, joint.twist_upper_limit)
                {
                    value = value.twist_limit(true, lower, upper.max(lower));
                }
                world.try_create_spherical_joint(value)
            }
            "prismatic" => {
                let (enabled, lower, upper) =
                    ordered_limits(joint.lower_limit, joint.upper_limit, scale);
                world.try_create_prismatic_joint(
                    PrismaticJointDef::new(body_a, body_b)
                        .local_frame_a(frame_a)
                        .local_frame_b(frame_b)
                        .collide_connected(false)
                        .limit(enabled, lower, upper),
                )
            }
            _ => {
                let lower = joint.lower_limit.unwrap_or(0.0);
                let upper = joint.upper_limit.unwrap_or(lower).max(lower);
                world.try_create_revolute_joint(
                    RevoluteJointDef::new(body_a, body_b)
                        .local_frame_a(frame_a)
                        .local_frame_b(frame_b)
                        .collide_connected(false)
                        .limit(
                            joint.lower_limit.is_some() || joint.upper_limit.is_some(),
                            lower,
                            upper,
                        ),
                )
            }
        };
        match created {
            Ok(id) => runtime.joints.push(LabJointHandle::Boxddd(id)),
            Err(error) => {
                destroy_boxddd_ragdoll(world, &mut runtime);
                fail_runtime(
                    &mut runtime,
                    format!(
                        "BoxDDD rejected {} joint {}-{}: {error}",
                        joint.kind, joint.body_a, joint.body_b
                    ),
                );
                return;
            }
        }
        runtime.joint_diagnostics.push(LabJointDiagnostic {
            body_a: joint.body_a,
            body_b: joint.body_b,
            local_anchor_a: anchor_a,
            local_anchor_b: anchor_b,
        });
    }
    finish_activation(&mut runtime);
}

fn destroy_boxddd_ragdoll(world: &mut boxddd::World, runtime: &mut RagdollLabRuntime) {
    for joint in runtime.joints.drain(..) {
        if let LabJointHandle::Boxddd(joint) = joint {
            let _ = world.try_destroy_joint(joint, true);
        }
    }
    for body in runtime.bodies.values() {
        if let LabBodyHandle::Boxddd(body) = body {
            let _ = world.try_destroy_body(*body);
        }
    }
    runtime.bodies.clear();
    runtime.bindings.clear();
    runtime.body_bounds.clear();
    runtime.joint_diagnostics.clear();
}

fn all_joint_endpoints_resolved(
    physics: &PreparedPhysicsAsset,
    bodies: &HashMap<u32, LabBodyHandle>,
) -> bool {
    physics
        .joints
        .iter()
        .all(|joint| bodies.contains_key(&joint.body_a) && bodies.contains_key(&joint.body_b))
}

fn add_node_binding(runtime: &mut RagdollLabRuntime, group_id: u32, body_position: Vec3) {
    let nodes = runtime.resolved_nodes[&group_id].clone();
    let rest_node_globals = nodes
        .iter()
        .filter_map(|node| runtime.node_rest_globals.get(node).copied())
        .collect::<Vec<_>>();
    runtime.bindings.push(LabNodeBinding {
        group_id,
        nodes,
        rest_body: Affine3A::from_translation(body_position),
        rest_node_globals,
    });
}

fn finish_activation(runtime: &mut RagdollLabRuntime) {
    runtime.phase = LabPhase::Active;
    runtime.drop_elapsed = 0.0;
    runtime.emitted_snapshots = 0;
    runtime.failure = None;
}

fn fail_runtime(runtime: &mut RagdollLabRuntime, error: impl Into<String>) {
    runtime.failure = Some(error.into());
    runtime.phase = LabPhase::Failed;
}

fn to_box_vec3(value: Vec3) -> boxddd::Vec3 {
    boxddd::Vec3::new(value.x, value.y, value.z)
}

fn to_box_quat(value: Quat) -> boxddd::Quat {
    boxddd::Quat::new(boxddd::Vec3::new(value.x, value.y, value.z), value.w)
}

fn from_box_quat(value: boxddd::Quat) -> Quat {
    Quat::from_xyzw(value.v.x, value.v.y, value.v.z, value.s).normalize()
}

fn update_avian_poses(
    definition: Res<RagdollLabDefinition>,
    runtime: Res<RagdollLabRuntime>,
    mut cache: ResMut<LabPoseCache>,
    bodies: Query<
        (
            &Transform,
            &avian::LinearVelocity,
            &avian::AngularVelocity,
            Has<avian::Sleeping>,
        ),
        With<LabRagdollEntity>,
    >,
) {
    if definition.backend != RagdollLabBackend::Avian {
        return;
    }
    cache.0.clear();
    for (group, handle) in &runtime.bodies {
        let LabBodyHandle::Avian(entity) = handle else {
            continue;
        };
        let Ok((transform, linear, angular, sleeping)) = bodies.get(*entity) else {
            continue;
        };
        cache.0.insert(
            *group,
            BodySample {
                affine: transform.compute_affine(),
                linear_speed: linear.0.length(),
                angular_speed: angular.0.length(),
                sleeping,
            },
        );
    }
}

fn update_boxddd_poses(
    definition: Res<RagdollLabDefinition>,
    runtime: Res<RagdollLabRuntime>,
    mut cache: ResMut<LabPoseCache>,
    context: Option<NonSend<BoxdddPhysicsContext>>,
) {
    if definition.backend != RagdollLabBackend::Boxddd {
        return;
    }
    cache.0.clear();
    let Some(context) = context.as_deref() else {
        return;
    };
    let Some(world) = context.world() else {
        return;
    };
    for (group, handle) in &runtime.bodies {
        let LabBodyHandle::Boxddd(body) = handle else {
            continue;
        };
        let Ok(transform) = world.try_body_transform(*body) else {
            continue;
        };
        let linear = world.try_body_linear_velocity(*body).unwrap_or_default();
        let angular = world.try_body_angular_velocity(*body).unwrap_or_default();
        cache.0.insert(
            *group,
            BodySample {
                affine: Affine3A::from_rotation_translation(
                    from_box_quat(transform.q),
                    Vec3::new(transform.p.x, transform.p.y, transform.p.z),
                ),
                linear_speed: Vec3::new(linear.x, linear.y, linear.z).length(),
                angular_speed: Vec3::new(angular.x, angular.y, angular.z).length(),
                sleeping: !world.try_body_awake(*body).unwrap_or(true),
            },
        );
    }
}

fn sync_ragdoll_nodes(
    runtime: Res<RagdollLabRuntime>,
    cache: Res<LabPoseCache>,
    globals: Query<(Entity, &GlobalTransform)>,
    parents: Query<(Entity, &ChildOf)>,
    mut nodes: Query<(Entity, &mut Transform), Without<LabPhysicsEntity>>,
) {
    if !matches!(runtime.phase, LabPhase::Active | LabPhase::Paused) {
        return;
    }
    let global_affines = globals
        .iter()
        .map(|(entity, transform)| (entity, transform.affine()))
        .collect::<HashMap<_, _>>();
    let parent_entities = parents
        .iter()
        .map(|(entity, parent)| (entity, parent.parent()))
        .collect::<HashMap<_, _>>();
    let local_snapshots = nodes
        .iter_mut()
        .map(|(entity, transform)| (entity, *transform))
        .collect::<HashMap<_, _>>();
    let mut desired_worlds = HashMap::new();
    for binding in &runtime.bindings {
        let Some(sample) = cache.0.get(&binding.group_id) else {
            continue;
        };
        let delta = sample.affine * binding.rest_body.inverse();
        for (node, rest_node) in binding.nodes.iter().zip(&binding.rest_node_globals) {
            desired_worlds.insert(*node, delta * *rest_node);
        }
    }
    for (node_entity, desired_world) in &desired_worlds {
        let parent_world = parent_entities
            .get(node_entity)
            .and_then(|parent| {
                ragdoll_resolved_world(
                    *parent,
                    &desired_worlds,
                    &parent_entities,
                    &global_affines,
                    &local_snapshots,
                )
            })
            .unwrap_or(Affine3A::IDENTITY);
        let Some(local) = ragdoll_local_transform(parent_world, *desired_world) else {
            continue;
        };
        if let Ok((_, mut transform)) = nodes.get_mut(*node_entity) {
            *transform = local;
        }
    }
}

fn update_lab_diagnostics(
    definition: Res<RagdollLabDefinition>,
    time: Res<Time>,
    cache: Res<LabPoseCache>,
    mut runtime: ResMut<RagdollLabRuntime>,
    mut probe: ResMut<RagdollLabProbe>,
    globals: Query<&GlobalTransform>,
    ragdoll_entities: Query<Entity, With<LabRagdollEntity>>,
) {
    if runtime.phase == LabPhase::Active {
        runtime.drop_elapsed += time.delta_secs();
    }
    let maximum_linear_speed = cache
        .0
        .values()
        .map(|sample| sample.linear_speed)
        .fold(0.0, f32::max);
    let maximum_angular_speed = cache
        .0
        .values()
        .map(|sample| sample.angular_speed)
        .fold(0.0, f32::max);
    let maximum_speed = maximum_linear_speed.max(maximum_angular_speed);
    let awake_bodies = cache.0.values().filter(|sample| !sample.sleeping).count();
    let maximum_anchor_separation = runtime
        .joint_diagnostics
        .iter()
        .filter_map(|joint| {
            let body_a = cache.0.get(&joint.body_a)?;
            let body_b = cache.0.get(&joint.body_b)?;
            let anchor_a = body_a.affine.transform_point3(joint.local_anchor_a);
            let anchor_b = body_b.affine.transform_point3(joint.local_anchor_b);
            Some(anchor_a.distance(anchor_b))
        })
        .fold(0.0, f32::max);
    let mut maximum_node_position_error = 0.0_f32;
    for binding in &runtime.bindings {
        let Some(sample) = cache.0.get(&binding.group_id) else {
            continue;
        };
        let delta = sample.affine * binding.rest_body.inverse();
        for (node, rest) in binding.nodes.iter().zip(&binding.rest_node_globals) {
            let Ok(actual) = globals.get(*node) else {
                continue;
            };
            let (_, _, rest_translation) = rest.to_scale_rotation_translation();
            let expected = delta.transform_point3(rest_translation);
            maximum_node_position_error =
                maximum_node_position_error.max(expected.distance(actual.translation()));
        }
    }
    let floor_penetration = runtime
        .body_bounds
        .iter()
        .filter_map(|(group, bounds)| {
            let body = cache.0.get(group)?;
            bounds
                .iter()
                .map(|bound| {
                    let world_y = body.affine.transform_point3(bound.point).y - bound.radius;
                    (-world_y).max(0.0)
                })
                .reduce(f32::max)
        })
        .fold(0.0, f32::max);
    let tracked = runtime
        .physics_entities
        .iter()
        .copied()
        .collect::<HashSet<_>>();
    let orphaned = ragdoll_entities
        .iter()
        .filter(|entity| !tracked.contains(entity))
        .count();
    *probe = RagdollLabProbe {
        backend: definition.backend.to_string(),
        actor_form_id: format!("{:08x}", definition.actor_form_id),
        phase: phase_name(runtime.phase).into(),
        body_count: runtime.bodies.len(),
        joint_count: runtime.joints.len(),
        expected_joint_count: definition.physics.joints.len(),
        missing_joints: definition
            .physics
            .joints
            .len()
            .saturating_sub(runtime.joints.len()),
        awake_bodies,
        maximum_speed,
        maximum_linear_speed,
        maximum_angular_speed,
        maximum_anchor_separation,
        maximum_node_position_error,
        floor_penetration,
        sleeping: !cache.0.is_empty() && awake_bodies == 0,
        elapsed_drop_time: runtime.drop_elapsed,
        orphaned_entities_after_reset: orphaned,
        reset_count: runtime.reset_count,
        error: runtime.failure.clone(),
    };
    while runtime.emitted_snapshots < DIAGNOSTIC_TIMES.len()
        && runtime.drop_elapsed >= DIAGNOSTIC_TIMES[runtime.emitted_snapshots]
    {
        let at = DIAGNOSTIC_TIMES[runtime.emitted_snapshots];
        info!(
            "ragdoll-lab snapshot t={at:.2} {}",
            serde_json::to_string(&*probe).unwrap_or_else(|_| "{}".into())
        );
        runtime.emitted_snapshots += 1;
    }
}

fn phase_name(phase: LabPhase) -> &'static str {
    match phase {
        LabPhase::Loading => "loading",
        LabPhase::Ready => "ready",
        LabPhase::Active => "active",
        LabPhase::Paused => "paused",
        LabPhase::Failed => "failed",
    }
}

fn lab_controls(
    keys: Res<ButtonInput<KeyCode>>,
    definition: Res<RagdollLabDefinition>,
    mut runtime: ResMut<RagdollLabRuntime>,
    mut physics_time: Option<ResMut<Time<avian::Physics>>>,
    mut context: Option<NonSendMut<BoxdddPhysicsContext>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    if keys.just_pressed(KeyCode::KeyR) {
        runtime.reset_requested = true;
    }
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    match runtime.phase {
        LabPhase::Active => {
            runtime.phase = LabPhase::Paused;
            if definition.backend == RagdollLabBackend::Avian
                && let Some(time) = physics_time.as_deref_mut()
            {
                time.pause();
            }
            set_boxddd_awake(&runtime, context.as_deref_mut(), false);
        }
        LabPhase::Paused => {
            runtime.phase = LabPhase::Active;
            if definition.backend == RagdollLabBackend::Avian
                && let Some(time) = physics_time.as_deref_mut()
            {
                time.unpause();
            }
            set_boxddd_awake(&runtime, context.as_deref_mut(), true);
        }
        _ => {}
    }
}

fn set_boxddd_awake(
    runtime: &RagdollLabRuntime,
    context: Option<&mut BoxdddPhysicsContext>,
    awake: bool,
) {
    let Some(context) = context else {
        return;
    };
    let Some(world) = context.world_mut() else {
        return;
    };
    for handle in runtime.bodies.values() {
        if let LabBodyHandle::Boxddd(body) = handle {
            let _ = world.try_set_body_awake(*body, awake);
        }
    }
}

fn reset_avian_ragdoll(
    mut commands: Commands,
    definition: Res<RagdollLabDefinition>,
    mut runtime: ResMut<RagdollLabRuntime>,
    mut nodes: Query<&mut Transform, Without<LabPhysicsEntity>>,
    mut physics_time: Option<ResMut<Time<avian::Physics>>>,
) {
    if definition.backend != RagdollLabBackend::Avian || !runtime.reset_requested {
        return;
    }
    for joint in runtime.joints.drain(..) {
        if let LabJointHandle::Avian(entity) = joint {
            commands.entity(entity).despawn();
        }
    }
    for body in runtime.bodies.values() {
        if let LabBodyHandle::Avian(entity) = body {
            commands.entity(*entity).despawn();
        }
    }
    if let Some(time) = physics_time.as_deref_mut() {
        time.unpause();
    }
    restore_and_clear(&mut runtime, &mut nodes);
}

fn reset_boxddd_ragdoll(
    definition: Res<RagdollLabDefinition>,
    mut runtime: ResMut<RagdollLabRuntime>,
    mut nodes: Query<&mut Transform, Without<LabPhysicsEntity>>,
    mut context: Option<NonSendMut<BoxdddPhysicsContext>>,
) {
    if definition.backend != RagdollLabBackend::Boxddd || !runtime.reset_requested {
        return;
    }
    if let Some(context) = context.as_deref_mut()
        && let Some(world) = context.world_mut()
    {
        for joint in runtime.joints.drain(..) {
            if let LabJointHandle::Boxddd(joint) = joint {
                let _ = world.try_destroy_joint(joint, true);
            }
        }
        for body in runtime.bodies.values() {
            if let LabBodyHandle::Boxddd(body) = body {
                let _ = world.try_destroy_body(*body);
            }
        }
    }
    restore_and_clear(&mut runtime, &mut nodes);
}

fn restore_and_clear(
    runtime: &mut RagdollLabRuntime,
    nodes: &mut Query<&mut Transform, Without<LabPhysicsEntity>>,
) {
    for (node, rest) in &runtime.node_rest_locals {
        if let Ok(mut transform) = nodes.get_mut(*node) {
            *transform = *rest;
        }
    }
    runtime.bindings.clear();
    runtime.body_bounds.clear();
    runtime.bodies.clear();
    runtime.joints.clear();
    runtime.joint_diagnostics.clear();
    runtime.physics_entities.clear();
    runtime.reset_requested = false;
    runtime.reset_count += 1;
    runtime.drop_elapsed = 0.0;
    runtime.emitted_snapshots = 0;
    runtime.failure = None;
    runtime.phase = LabPhase::Ready;
}

fn draw_lab_reference(mut gizmos: Gizmos) {
    for index in -6..=6 {
        let position = index as f32;
        let color = if index == 0 {
            Color::srgb(0.3, 0.34, 0.4)
        } else {
            Color::srgba(0.25, 0.27, 0.31, 0.55)
        };
        gizmos.line(
            Vec3::new(position, 0.002, -6.0),
            Vec3::new(position, 0.002, 6.0),
            color,
        );
        gizmos.line(
            Vec3::new(-6.0, 0.002, position),
            Vec3::new(6.0, 0.002, position),
            color,
        );
    }
    gizmos.arrow(Vec3::ZERO, Vec3::X, Color::srgb(1.0, 0.15, 0.15));
    gizmos.arrow(Vec3::ZERO, Vec3::Y, Color::srgb(0.15, 1.0, 0.2));
    gizmos.arrow(Vec3::ZERO, Vec3::Z, Color::srgb(0.2, 0.45, 1.0));
}

fn update_lab_hud(probe: Res<RagdollLabProbe>, mut text: Single<&mut Text, With<LabHud>>) {
    text.0 = format!(
        "Ragdoll Lab | {} | actor {} | {}\nBodies {} | joints {}/{} | awake {} | sleeping {}\nmax linear {:.3} m/s | angular {:.3} rad/s | anchor gap {:.4} m\nnode error {:.4} m | floor penetration {:.4} m\ndrop {:.2}s | resets {} | orphaned {}\nSpace pause/resume | R reset/drop | Esc exit{}",
        probe.backend,
        probe.actor_form_id,
        probe.phase,
        probe.body_count,
        probe.joint_count,
        probe.expected_joint_count,
        probe.awake_bodies,
        probe.sleeping,
        probe.maximum_linear_speed,
        probe.maximum_angular_speed,
        probe.maximum_anchor_separation,
        probe.maximum_node_position_error,
        probe.floor_penetration,
        probe.elapsed_drop_time,
        probe.reset_count,
        probe.orphaned_entities_after_reset,
        probe
            .error
            .as_ref()
            .map(|error| format!("\nERROR: {error}"))
            .unwrap_or_default(),
    );
}

fn exit_after_trace(
    definition: Res<RagdollLabDefinition>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    use crate::vsa::PreparedPhysicsSource;
    use bevy::time::TimeUpdateStrategy;

    fn body(group_id: u32) -> PreparedPhysicsBody {
        PreparedPhysicsBody {
            group_id,
            node: Some(format!("Bip01 Test {group_id}")),
            shapes: vec![PreparedPhysicsShape::Sphere {
                center: [0.0; 3],
                radius: 0.25,
            }],
            ..default()
        }
    }

    #[test]
    fn form_ids_accept_plain_and_prefixed_hex() {
        assert_eq!(parse_form_id("00041606").unwrap(), 0x0004_1606);
        assert_eq!(parse_form_id("0x00041606").unwrap(), 0x0004_1606);
        assert!(parse_form_id("raider").is_err());
    }

    #[test]
    fn actor_sidecar_rejects_incomplete_joint_endpoints() {
        let mut asset = PreparedPhysicsAsset {
            schema_version: 3,
            source: PreparedPhysicsSource::AuthoredHavok,
            bodies: vec![body(1)],
            joints: vec![Default::default()],
        };
        asset.joints[0].body_a = 1;
        asset.joints[0].body_b = 2;
        assert!(
            validate_actor_physics(&asset)
                .unwrap_err()
                .to_string()
                .contains("1-2")
        );
    }

    #[test]
    fn actor_sidecar_rejects_unconstrained_or_disconnected_bodies() {
        let asset = PreparedPhysicsAsset {
            schema_version: 3,
            source: PreparedPhysicsSource::AuthoredHavok,
            bodies: vec![body(1), body(2), body(3)],
            joints: Vec::new(),
        };
        let error = validate_actor_physics(&asset).unwrap_err().to_string();
        assert!(error.contains("disconnected joint graph"));
        assert!(error.contains("2, 3"));
    }

    #[test]
    fn avian_uses_converter_authored_local_z_twist_axis() {
        assert_eq!(avian_twist_axis(), Vec3::Z);
    }

    #[test]
    fn complete_local_joint_frame_keeps_xyzw_orientation() {
        let expected = Quat::from_euler(EulerRot::XYZ, 0.3, -0.4, 0.7).normalize();
        let actual = local_joint_frame(expected.to_array());
        assert!(actual.angle_between(expected) < 1.0e-6);
    }

    #[test]
    fn prepared_lab_body_uses_limp_zero_bounce_tuning() {
        let mut source = body(1);
        source.linear_damping = 0.1;
        source.angular_damping = 0.05;
        source.friction = 0.3;
        source.restitution = 0.9;
        source.linear_velocity = [4.0, 5.0, 6.0];
        source.angular_velocity = [1.0, 2.0, 3.0];
        source.sleep_enabled = false;

        let (prepared, _) = prepared_dynamic_body(&source).unwrap();

        assert_eq!(prepared.linear_velocity, [0.0; 3]);
        assert_eq!(prepared.angular_velocity, [0.0; 3]);
        assert!(prepared.linear_damping >= 0.6);
        assert!(prepared.angular_damping >= 1.0);
        assert!(prepared.friction >= 0.6);
        assert_eq!(prepared.restitution, 0.0);
        assert!(prepared.sleep_enabled);
    }

    #[test]
    fn lab_ragdoll_collides_with_floor_but_not_itself() {
        let floor = lab_floor_collision_layers();
        let ragdoll = lab_ragdoll_collision_layers();

        assert!(floor.interacts_with(ragdoll));
        assert!(ragdoll.interacts_with(floor));
        assert!(!ragdoll.interacts_with(ragdoll));
    }

    #[test]
    fn lab_sleep_threshold_allows_only_settled_contact_drift() {
        let threshold = lab_sleep_threshold();

        assert_eq!(threshold.linear, 0.2);
        assert_eq!(threshold.angular, 0.8);
    }

    #[test]
    fn prismatic_limits_preserve_order_and_scale() {
        assert_eq!(
            ordered_limits(Some(-0.2), Some(0.4), 2.0),
            (true, -0.4, 0.8)
        );
        assert_eq!(ordered_limits(Some(0.5), Some(-0.5), 1.0), (true, 0.5, 0.5));
        assert_eq!(ordered_limits(None, None, 1.0), (false, 0.0, 0.0));
    }

    fn avian_test_app(gravity: Vec3) -> App {
        let step = Duration::from_secs_f32(1.0 / 60.0);
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            avian::PhysicsPlugins::default(),
            TransformPlugin,
        ));
        app.insert_resource(avian::SubstepCount(8));
        app.insert_resource(avian::Gravity(gravity));
        app.insert_resource(Time::<Fixed>::from_duration(step));
        app.insert_resource(TimeUpdateStrategy::ManualDuration(step));
        app.finish();
        app
    }

    fn run_steps(app: &mut App, count: usize) {
        for _ in 0..count {
            app.update();
        }
    }

    #[test]
    fn avian_revolute_limb_retains_its_hinge_anchor() {
        let mut app = avian_test_app(Vec3::ZERO);
        let anchor = app
            .world_mut()
            .spawn((
                avian::RigidBody::Static,
                avian::Position(Vec3::new(0.0, 2.0, 0.0)),
            ))
            .id();
        let limb = app
            .world_mut()
            .spawn((
                avian::RigidBody::Dynamic,
                avian::Position(Vec3::new(0.0, 1.0, 0.0)),
                avian::Collider::capsule(0.15, 0.7),
                avian::AngularVelocity(Vec3::new(0.0, 0.0, 2.0)),
            ))
            .id();
        app.world_mut().spawn((
            avian::RevoluteJoint::new(anchor, limb)
                .with_local_anchor1(Vec3::new(0.0, -1.0, 0.0))
                .with_local_anchor2(Vec3::ZERO)
                .with_angle_limits(-0.75, 0.75),
            avian::JointCollisionDisabled,
        ));
        run_steps(&mut app, 120);
        let anchor_position = app.world().get::<avian::Position>(anchor).unwrap().0
            + app.world().get::<avian::Rotation>(anchor).unwrap().0 * Vec3::new(0.0, -1.0, 0.0);
        let limb_position = app.world().get::<avian::Position>(limb).unwrap().0;
        assert!(anchor_position.distance(limb_position) < 0.05);
    }

    #[test]
    fn avian_spherical_shoulder_keeps_anchor_and_local_z_twist() {
        let mut app = avian_test_app(Vec3::ZERO);
        let torso = app
            .world_mut()
            .spawn((
                avian::RigidBody::Static,
                avian::Position(Vec3::new(0.0, 2.0, 0.0)),
            ))
            .id();
        let arm = app
            .world_mut()
            .spawn((
                avian::RigidBody::Dynamic,
                avian::Position(Vec3::new(1.0, 2.0, 0.0)),
                avian::Collider::capsule(0.12, 0.7),
                avian::AngularVelocity(Vec3::new(0.5, 0.0, 1.0)),
            ))
            .id();
        app.world_mut().spawn((
            avian::SphericalJoint::new(torso, arm)
                .with_local_anchor1(Vec3::X)
                .with_local_anchor2(Vec3::ZERO)
                .with_twist_axis(avian_twist_axis())
                .with_swing_limits(-0.8, 0.8)
                .with_twist_limits(-0.4, 0.4),
            avian::JointCollisionDisabled,
        ));
        run_steps(&mut app, 120);
        let torso_anchor = app.world().get::<avian::Position>(torso).unwrap().0 + Vec3::X;
        let arm_anchor = app.world().get::<avian::Position>(arm).unwrap().0;
        assert!(torso_anchor.distance(arm_anchor) < 0.05);
    }

    #[test]
    fn avian_chain_lands_without_separating_or_tunnelling() {
        let mut app = avian_test_app(Vec3::new(0.0, -9.81, 0.0));
        app.world_mut().spawn((
            avian::RigidBody::Static,
            avian::Collider::cuboid(8.0, 0.5, 8.0),
            avian::Position(Vec3::new(0.0, -0.25, 0.0)),
        ));
        let mut bodies = Vec::new();
        for index in 0..3 {
            bodies.push(
                app.world_mut()
                    .spawn((
                        avian::RigidBody::Dynamic,
                        avian::Collider::sphere(0.2),
                        avian::Position(Vec3::new(0.0, 2.5 + index as f32 * 0.4, 0.0)),
                        avian::LinearDamping(0.4),
                        avian::AngularDamping(0.4),
                    ))
                    .id(),
            );
        }
        for pair in bodies.windows(2) {
            app.world_mut().spawn((
                avian::SphericalJoint::new(pair[0], pair[1])
                    .with_local_anchor1(Vec3::new(0.0, 0.2, 0.0))
                    .with_local_anchor2(Vec3::new(0.0, -0.2, 0.0)),
                avian::JointCollisionDisabled,
            ));
        }
        run_steps(&mut app, 360);
        let positions = bodies
            .iter()
            .map(|body| app.world().get::<avian::Position>(*body).unwrap().0)
            .collect::<Vec<_>>();
        assert!(positions.iter().all(|position| position.y >= 0.15));
        assert!(
            positions
                .windows(2)
                .all(|pair| pair[0].distance(pair[1]) < 0.5)
        );
    }

    #[test]
    fn avian_damped_body_eventually_sleeps() {
        let mut app = avian_test_app(Vec3::new(0.0, -9.81, 0.0));
        app.world_mut().spawn((
            avian::RigidBody::Static,
            avian::Collider::cuboid(8.0, 0.5, 8.0),
            avian::Position(Vec3::new(0.0, -0.25, 0.0)),
        ));
        let body = app
            .world_mut()
            .spawn((
                avian::RigidBody::Dynamic,
                avian::Collider::sphere(0.2),
                avian::Position(Vec3::new(0.0, 1.0, 0.0)),
                avian::LinearDamping(1.0),
                avian::AngularDamping(1.0),
            ))
            .id();
        run_steps(&mut app, 600);
        assert!(app.world().entity(body).contains::<avian::Sleeping>());
    }
}
