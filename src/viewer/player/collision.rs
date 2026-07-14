//! Prepared and dynamic collision construction.

use std::path::PathBuf;
use std::sync::Arc;

use bevy::math::Affine3A;

use super::*;

/// Issue #52: a destination cell's colliders queued for staggered
/// construction across subsequent frames (`advance_pending_collider_builds`)
/// rather than all at once, so activating a large cell never spikes a
/// single frame. `None` when no swap-triggered collider build is in
/// progress.
#[derive(Resource, Default)]
pub(crate) struct PendingColliderBuild(Option<PendingColliderBuildState>);

struct PendingColliderBuildState {
    manifest: Arc<PreparedSceneManifest>,
    queue: super::super::world::ColliderBuildQueue<usize>,
    root_by_reference: HashMap<u32, Entity>,
    unknown_layers: HashSet<u8>,
    static_marker_spawned: bool,
}

/// Issue #52: queues collider construction for every enabled,
/// physics-bearing placement in `manifest`, to be drained at
/// `world::COLLIDER_BUILD_BUDGET_PER_FRAME` per frame by
/// `advance_pending_collider_builds`. `root_by_reference` must map each
/// queued placement's `reference_form_id` to its already-spawned entity
/// (see `scene::spawn_cell_content`) so dynamic bodies attach to the right
/// entity. A no-op while physics is disabled.
pub(crate) fn queue_collider_build(
    world: &mut World,
    manifest: Arc<PreparedSceneManifest>,
    root_by_reference: HashMap<u32, Entity>,
) {
    if world.resource::<PhysicsDisabled>().0 {
        return;
    }
    let indices: Vec<usize> = manifest
        .placements
        .iter()
        .enumerate()
        .filter(|(_, placement)| {
            placement.initially_enabled && placement.physics_asset_path.is_some()
        })
        .map(|(index, _)| index)
        .collect();
    if indices.is_empty() {
        return;
    }
    let form_id = manifest.cell.form_id;
    let queue = super::super::world::ColliderBuildQueue::new(indices);
    let queued = queue.len();
    world.resource_mut::<PendingColliderBuild>().0 = Some(PendingColliderBuildState {
        manifest,
        queue,
        root_by_reference,
        unknown_layers: HashSet::new(),
        static_marker_spawned: false,
    });
    info!("swap colliders queued for {form_id:08x} ({queued} placements)");
}

/// Issue #52: drains up to `world::COLLIDER_BUILD_BUDGET_PER_FRAME`
/// queued placements per frame from `PendingColliderBuild`, building each
/// one's colliders exactly like the startup `build_prepared_colliders`
/// pass does (same shared `build_colliders_for_placement`), attaching them
/// to the single shared static body. Sidecars the startup load never saw
/// (a swapped-in cell's) are read lazily here, so their file I/O rides the
/// same per-frame budget. A no-op when nothing is queued.
pub(crate) fn advance_pending_collider_builds(
    mut commands: Commands,
    mut pending: ResMut<PendingColliderBuild>,
    mut physics_assets: ResMut<PreparedPhysicsAssets>,
    mut stats: ResMut<CollisionRuntimeStats>,
    mut collision_world: ResMut<PreparedCollisionWorld>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
) {
    if pending.0.is_none() {
        return;
    }
    let Some(world) = context.world_mut() else {
        return;
    };
    let Some(static_body) = collision_world.static_body else {
        return;
    };
    let state = pending.0.as_mut().expect("checked is_none above");
    let asset_root = PathBuf::from(&state.manifest.asset_root);
    let batch = state
        .queue
        .drain_budget(super::super::world::COLLIDER_BUILD_BUDGET_PER_FRAME);
    for index in &batch {
        let Some(placement) = state.manifest.placements.get(*index).cloned() else {
            continue;
        };
        if let Some(path) = placement.physics_asset_path.as_ref()
            && !physics_assets.ensure_sidecar_loaded(path, &asset_root)
        {
            continue;
        }
        build_colliders_for_placement(
            world,
            &mut commands,
            &placement,
            &physics_assets,
            static_body,
            &state.root_by_reference,
            &mut collision_world,
            &mut stats,
            &mut state.unknown_layers,
            &mut state.static_marker_spawned,
        );
    }
    let queue_empty = state.queue.is_empty();
    let form_id = state.manifest.cell.form_id;
    if queue_empty {
        pending.0 = None;
        info!("swap colliders complete for {form_id:08x}");
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn build_prepared_colliders(
    mut commands: Commands,
    physics_disabled: Res<PhysicsDisabled>,
    manifest: Res<PreparedSceneManifest>,
    physics_assets: Res<PreparedPhysicsAssets>,
    mut state: ResMut<CameraModeState>,
    mut stats: ResMut<CollisionRuntimeStats>,
    mut collision_world: ResMut<PreparedCollisionWorld>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    roots: Query<(Entity, &super::super::interaction::PlacementRoot)>,
) {
    if physics_disabled.0 {
        state.collision_build_complete = true;
        return;
    }
    let Some(world) = context.world_mut() else {
        return;
    };
    let started = Instant::now();
    let static_body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    collision_world.static_body = Some(static_body);
    stats.sidecar_bytes = physics_assets.payload_bytes;
    for asset in physics_assets.assets.values() {
        match asset.source {
            PreparedPhysicsSource::AuthoredHavok => stats.authored_assets += 1,
            PreparedPhysicsSource::GeneratedRender => stats.fallback_assets += 1,
        }
    }
    let root_by_reference = roots
        .iter()
        .map(|(entity, root)| (root.placement().reference_form_id, entity))
        .collect::<HashMap<_, _>>();
    let mut unknown_layers = HashSet::new();
    let mut static_marker_spawned = false;
    for placement in manifest
        .placements
        .iter()
        .filter(|placement| placement.initially_enabled)
    {
        build_colliders_for_placement(
            world,
            &mut commands,
            placement,
            &physics_assets,
            static_body,
            &root_by_reference,
            &mut collision_world,
            &mut stats,
            &mut unknown_layers,
            &mut static_marker_spawned,
        );
    }
    stats.awake_dynamic_bodies = stats.dynamic_bodies;
    stats.sleeping_dynamic_bodies = 0;
    stats.dynamic_transform_updates = 0;
    stats.cooking_millis = started.elapsed().as_secs_f64() * 1000.0;
    state.collision_build_complete = true;
    state.collisions_ready = stats.shapes > 0;
    info!(
        "BoxDDD prepared collision: {} authored / {} fallback assets, {} bodies ({} dynamic), {} shapes, {} packed triangles, {} filtered, {:.1} ms cook, {} sidecar bytes",
        stats.authored_assets,
        stats.fallback_assets,
        stats.bodies,
        stats.dynamic_bodies,
        stats.shapes,
        stats.packed_triangles,
        stats.filtered_shapes,
        stats.cooking_millis,
        stats.sidecar_bytes,
    );
    if !state.collisions_ready {
        warn!(
            "prepared physics produced no active player-blocking shapes; FPS will use forced no-clip"
        );
    }
}

/// Builds every BoxDDD body/shape for one placement (shared by the startup
/// `build_prepared_colliders` pass and issue #52's staggered
/// `advance_pending_collider_builds`), attaching static shapes to the
/// single shared `static_body` and dynamic ones to a per-entity body looked
/// up in `root_by_reference`.
#[allow(clippy::too_many_arguments)]
fn build_colliders_for_placement(
    world: &mut boxddd::World,
    commands: &mut Commands,
    placement: &crate::vsa::PreparedPlacement,
    physics_assets: &PreparedPhysicsAssets,
    static_body: BodyId,
    root_by_reference: &HashMap<u32, Entity>,
    collision_world: &mut PreparedCollisionWorld,
    stats: &mut CollisionRuntimeStats,
    unknown_layers: &mut HashSet<u8>,
    static_marker_spawned: &mut bool,
) {
    let Some(path) = placement.physics_asset_path.as_ref() else {
        return;
    };
    let Some(asset) = physics_assets.assets.get(path) else {
        warn!("missing preloaded physics sidecar {path}");
        return;
    };
    let dynamic_entity = (placement.physics_classification
        == PreparedPhysicsClassification::Dynamic)
        .then(|| root_by_reference.get(&placement.reference_form_id).copied())
        .flatten();
    let placement_root = root_by_reference.get(&placement.reference_form_id).copied();
    for body in &asset.bodies {
        if !body_blocks_player(body) {
            stats.filtered_shapes += body.shapes.len();
            continue;
        }
        if body.layer > 43 && unknown_layers.insert(body.layer) {
            warn!(
                "unknown Fallout Havok layer {} remains solid (reference {:08x})",
                body.layer, placement.reference_form_id
            );
        }
        let (body_id, dynamic, local_space) = if let Some(entity) = dynamic_entity {
            let body_id = create_dynamic_body(world, placement, body);
            collision_world.dynamic_bodies.insert(entity, body_id);
            collision_world.dynamic_entities.insert(body_id, entity);
            commands.entity(entity).insert(PhysicsCollider);
            stats.dynamic_bodies += 1;
            (body_id, true, true)
        } else if body.motion_type == "MO_SYS_KEYFRAMED"
            && body.shapes.iter().all(|shape| shape.supports_dynamic())
            && let Some(node) = body.node.as_ref()
            && let Some(root) = placement_root
        {
            // Issue #64: a keyframed body gets its own kinematic boxddd
            // body so `drive_keyframed_colliders` can move it with the
            // door/activator animation instead of leaving a rest-pose
            // collider across an open doorway.
            let body_id = create_kinematic_body(world, placement);
            collision_world
                .keyframed_bodies
                .entry(root)
                .or_default()
                .push(KeyframedColliderBinding {
                    node: node.clone(),
                    body: body_id,
                    node_entity: None,
                    rest_from_root: None,
                });
            if !*static_marker_spawned {
                commands.spawn(PhysicsCollider);
                *static_marker_spawned = true;
            }
            (body_id, false, true)
        } else {
            if !*static_marker_spawned {
                commands.spawn(PhysicsCollider);
                *static_marker_spawned = true;
            }
            (static_body, false, false)
        };
        stats.bodies += 1;
        for shape in &body.shapes {
            let result =
                create_prepared_shape(world, body_id, body, shape, placement, dynamic, local_space);
            let Some((shape_id, triangles)) = result else {
                stats.filtered_shapes += 1;
                warn!(
                    "BoxDDD rejected {} Havok shape for reference {:08x} body {}",
                    shape.kind(),
                    placement.reference_form_id,
                    body.group_id
                );
                continue;
            };
            collision_world.surfaces.insert(
                shape_id,
                CollisionSurface {
                    material: body.material,
                },
            );
            stats.shapes += 1;
            stats.packed_triangles += triangles;
            *stats.shape_kinds.entry(shape.kind()).or_default() += 1;
        }
        if dynamic {
            normalize_dynamic_mass(world, body_id, body, placement.scale.abs());
        }
    }
}

/// Issue #64: a keyframed body's kinematic collider, bound to the animated
/// scene node the converter recorded on the sidecar body (`node`).
pub(crate) struct KeyframedColliderBinding {
    node: String,
    body: BodyId,
    /// Resolved lazily: scene spawn is async, and a preloaded cell's scene
    /// can respawn under the surviving root (see `animation.rs`'s
    /// rediscovery), invalidating the entity.
    node_entity: Option<Entity>,
    /// Node pose relative to the root, captured when the node resolves
    /// (the scene spawns at rest pose).
    rest_from_root: Option<Affine3A>,
}

pub(crate) fn create_kinematic_body(
    world: &mut boxddd::World,
    placement: &crate::vsa::PreparedPlacement,
) -> BodyId {
    let rotation = Quat::from_array(placement.rotation_xyzw).normalize();
    world.create_body(
        BodyDef::builder()
            .body_type(BodyType::Kinematic)
            .position(to_box_vec3(Vec3::from_array(placement.translation)))
            .rotation(to_box_quat(rotation))
            .build(),
    )
}

/// Issue #64: world pose for a keyframed collider body — the node's rigid
/// delta from its rest pose, re-applied on top of the placement root.
pub(crate) fn keyframed_body_target(
    root: &GlobalTransform,
    rest_from_root: Affine3A,
    node: &GlobalTransform,
) -> Option<(Vec3, Quat)> {
    let root_affine = root.affine();
    let current_from_root = root_affine.inverse() * node.affine();
    let delta = current_from_root * rest_from_root.inverse();
    let (_, rotation, translation) = (root_affine * delta).to_scale_rotation_translation();
    (translation.is_finite() && rotation.is_finite()).then_some((translation, rotation))
}

fn find_named_descendant(
    root: Entity,
    name: &str,
    children: &Query<&Children>,
    names: &Query<&Name>,
) -> Option<Entity> {
    let mut queue = vec![root];
    while let Some(entity) = queue.pop() {
        if names.get(entity).is_ok_and(|n| n.as_str() == name) {
            return Some(entity);
        }
        if let Ok(child_list) = children.get(entity) {
            queue.extend(child_list.iter());
        }
    }
    None
}

/// Issue #64: every fixed step, steer each keyframed collider body toward
/// its animated node's current pose (kinematic target transforms integrate
/// velocities, so the player is pushed rather than tunneled). Idle doors
/// cost one affine compare per body.
pub(crate) fn drive_keyframed_colliders(
    time: Res<Time>,
    mut collision_world: ResMut<PreparedCollisionWorld>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    transforms: Query<&GlobalTransform>,
    names: Query<&Name>,
    children: Query<&Children>,
) {
    if collision_world.keyframed_bodies.is_empty() {
        return;
    }
    let Some(world) = context.world_mut() else {
        return;
    };
    let time_step = time.delta_secs().max(1e-6);
    let keyframed = &mut collision_world.keyframed_bodies;
    for (root, bindings) in keyframed.iter_mut() {
        let Ok(root_global) = transforms.get(*root) else {
            continue;
        };
        for binding in bindings.iter_mut() {
            if binding
                .node_entity
                .is_none_or(|entity| transforms.get(entity).is_err())
            {
                binding.node_entity =
                    find_named_descendant(*root, &binding.node, &children, &names);
                binding.rest_from_root = None;
            }
            let Some(node_entity) = binding.node_entity else {
                continue;
            };
            let Ok(node_global) = transforms.get(node_entity) else {
                continue;
            };
            let rest = *binding
                .rest_from_root
                .get_or_insert_with(|| root_global.affine().inverse() * node_global.affine());
            let Some((translation, rotation)) =
                keyframed_body_target(root_global, rest, node_global)
            else {
                continue;
            };
            let target =
                boxddd::WorldTransform::new(to_box_vec3(translation).into(), to_box_quat(rotation));
            let _ = world.try_set_body_target_transform(binding.body, target, time_step, true);
        }
    }
}

pub(crate) fn create_dynamic_body(
    world: &mut boxddd::World,
    placement: &crate::vsa::PreparedPlacement,
    body: &PreparedPhysicsBody,
) -> BodyId {
    let rotation = Quat::from_array(placement.rotation_xyzw).normalize();
    let scale = placement.scale.abs();
    let mut linear_velocity = rotation * (Vec3::from_array(body.linear_velocity) * scale);
    let mut angular_velocity = rotation * Vec3::from_array(body.angular_velocity);
    if body.max_linear_velocity > 0.0 {
        linear_velocity = linear_velocity.clamp_length_max(body.max_linear_velocity * scale);
    }
    if body.max_angular_velocity > 0.0 {
        angular_velocity = angular_velocity.clamp_length_max(body.max_angular_velocity);
    }
    let body_id = world.create_body(
        BodyDef::builder()
            .body_type(BodyType::Dynamic)
            .position(to_box_vec3(Vec3::from_array(placement.translation)))
            .rotation(to_box_quat(rotation))
            .linear_velocity(to_box_vec3(linear_velocity))
            .angular_velocity(to_box_vec3(angular_velocity))
            .gravity_scale(body.gravity_factor)
            .bullet(body.ccd_enabled)
            .build(),
    );
    let _ = world.try_set_body_linear_damping(body_id, body.linear_damping.max(0.0));
    let _ = world.try_set_body_angular_damping(body_id, body.angular_damping.max(0.0));
    let _ = world.try_enable_body_sleep(body_id, body.sleep_enabled);
    body_id
}

pub(crate) fn create_prepared_shape(
    world: &mut boxddd::World,
    body_id: BodyId,
    body: &PreparedPhysicsBody,
    shape: &PreparedPhysicsShape,
    placement: &crate::vsa::PreparedPlacement,
    dynamic: bool,
    local_space: bool,
) -> Option<(ShapeId, usize)> {
    if dynamic && !shape.supports_dynamic() {
        return None;
    }
    let scale = placement.scale.abs().max(0.0001);
    let placement_rotation = Quat::from_array(placement.rotation_xyzw).normalize();
    let placement_translation = Vec3::from_array(placement.translation);
    let point = |value: [f32; 3]| {
        let local = Vec3::from_array(value) * scale;
        if local_space {
            local
        } else {
            placement_rotation * local + placement_translation
        }
    };
    let rotation = |value: [f32; 4]| {
        let local = Quat::from_array(value).normalize();
        if local_space {
            local
        } else {
            placement_rotation * local
        }
    };
    let category = prepared_shape_category(
        dynamic,
        placement.step_support,
        placement.physics_classification,
    );
    let mask = if dynamic {
        WORLD_STATIC | WORLD_DYNAMIC | PLAYER_PROXY | PLAYER_QUERY
    } else {
        WORLD_DYNAMIC | PLAYER_QUERY
    };
    let shape_def = ShapeDef::builder()
        .density(if dynamic { 1.0 } else { 0.0 })
        .friction(body.friction.max(0.0))
        .restitution(body.restitution.max(0.0))
        .filter(Filter {
            category_bits: category,
            mask_bits: mask,
            group_index: 0,
        })
        .user_material_id(u64::from(body.material.unwrap_or(0)))
        .build();
    let shape_id = match shape {
        PreparedPhysicsShape::Box {
            center,
            half_extents,
            rotation_xyzw,
        } => {
            let center = point(*center);
            let rotation = rotation(*rotation_xyzw);
            let half = Vec3::from_array(*half_extents) * scale;
            let hull = BoxHull::transformed(
                half.x,
                half.y,
                half.z,
                boxddd::Transform::new(to_box_vec3(center), to_box_quat(rotation)),
            );
            world
                .try_create_hull_shape(body_id, &shape_def, &hull)
                .ok()?
        }
        PreparedPhysicsShape::Sphere { center, radius } => world
            .try_create_sphere_shape(
                body_id,
                &shape_def,
                &boxddd::Sphere::new(to_box_vec3(point(*center)), radius * scale),
            )
            .ok()?,
        PreparedPhysicsShape::Capsule {
            point1,
            point2,
            radius,
        } => world
            .try_create_capsule_shape(
                body_id,
                &shape_def,
                &boxddd::Capsule::new(
                    to_box_vec3(point(*point1)),
                    to_box_vec3(point(*point2)),
                    radius * scale,
                ),
            )
            .ok()?,
        PreparedPhysicsShape::ConvexHull { points } => {
            let points = points
                .iter()
                .map(|value| to_box_vec3(point(*value)))
                .collect::<Vec<_>>();
            let hull = Hull::from_points(&points, 64).ok()?;
            world
                .try_create_created_hull_shape(body_id, &shape_def, &hull)
                .ok()?
        }
        PreparedPhysicsShape::TriangleMesh { vertices, indices } => {
            let vertices = vertices
                .iter()
                .map(|value| to_box_vec3(point(*value)))
                .collect::<Vec<_>>();
            let indices = indices
                .iter()
                .map(|index| i32::try_from(*index).ok())
                .collect::<Option<Vec<_>>>()?;
            let mesh = boxddd::MeshData::builder(vertices, indices).build().ok()?;
            world
                .try_create_mesh_shape(body_id, &shape_def, mesh, boxddd::Vec3::new(1.0, 1.0, 1.0))
                .ok()?
        }
    };
    Some((shape_id, shape.triangle_count()))
}

pub(crate) fn prepared_shape_category(
    dynamic: bool,
    step_support: bool,
    classification: PreparedPhysicsClassification,
) -> u64 {
    if dynamic {
        WORLD_DYNAMIC
    } else if step_support && classification == PreparedPhysicsClassification::Static {
        WORLD_STATIC | STEP_SUPPORT
    } else {
        WORLD_STATIC
    }
}

pub(crate) fn normalize_dynamic_mass(
    world: &mut boxddd::World,
    body_id: BodyId,
    body: &PreparedPhysicsBody,
    scale: f32,
) {
    if !body.mass.is_finite() || body.mass <= 0.0 {
        return;
    }
    if world.try_apply_mass_from_shapes(body_id).is_err() {
        return;
    }
    let Ok(mut mass_data) = world.try_body_mass_data(body_id) else {
        return;
    };
    let ratio = if mass_data.mass > f32::EPSILON {
        body.mass / mass_data.mass
    } else {
        1.0
    };
    mass_data.mass = body.mass;
    mass_data.center = to_box_vec3(Vec3::from_array(body.center_of_mass) * scale);
    let authored_inertia_valid = body.inertia.iter().flatten().all(|value| value.is_finite())
        && body.inertia[0][0] > 0.0
        && body.inertia[1][1] > 0.0
        && body.inertia[2][2] > 0.0;
    if authored_inertia_valid {
        let inertia_scale = scale * scale;
        mass_data.inertia = boxddd::Matrix3 {
            cx: boxddd::Vec3::new(
                body.inertia[0][0] * inertia_scale,
                body.inertia[1][0] * inertia_scale,
                body.inertia[2][0] * inertia_scale,
            ),
            cy: boxddd::Vec3::new(
                body.inertia[0][1] * inertia_scale,
                body.inertia[1][1] * inertia_scale,
                body.inertia[2][1] * inertia_scale,
            ),
            cz: boxddd::Vec3::new(
                body.inertia[0][2] * inertia_scale,
                body.inertia[1][2] * inertia_scale,
                body.inertia[2][2] * inertia_scale,
            ),
        };
    } else {
        mass_data.inertia.cx = scale_box_vec3(mass_data.inertia.cx, ratio);
        mass_data.inertia.cy = scale_box_vec3(mass_data.inertia.cy, ratio);
        mass_data.inertia.cz = scale_box_vec3(mass_data.inertia.cz, ratio);
    }
    let _ = world.try_set_body_mass_data(body_id, mass_data);
}

pub(crate) fn cleanup_removed_dynamic_bodies(
    mut collision_world: ResMut<PreparedCollisionWorld>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    roots: Query<Entity, With<super::super::interaction::PlacementRoot>>,
) {
    let live = roots.iter().collect::<HashSet<_>>();
    let removed = collision_world
        .dynamic_bodies
        .keys()
        .filter(|entity| !live.contains(entity))
        .copied()
        .collect::<Vec<_>>();
    let Some(world) = context.world_mut() else {
        return;
    };
    for entity in removed {
        if let Some(body) = collision_world.dynamic_bodies.remove(&entity) {
            collision_world.dynamic_entities.remove(&body);
            collision_world.sleeping_dynamic_bodies.remove(&entity);
            let _ = world.try_destroy_body(body);
        }
    }
    let removed_keyframed = collision_world
        .keyframed_bodies
        .keys()
        .filter(|entity| !live.contains(entity))
        .copied()
        .collect::<Vec<_>>();
    for entity in removed_keyframed {
        if let Some(bindings) = collision_world.keyframed_bodies.remove(&entity) {
            for binding in bindings {
                let _ = world.try_destroy_body(binding.body);
            }
        }
    }
}

pub(crate) fn sync_dynamic_transforms(
    mut collision_world: ResMut<PreparedCollisionWorld>,
    context: NonSend<BoxdddPhysicsContext>,
    mut stats: ResMut<CollisionRuntimeStats>,
    mut roots: Query<&mut Transform, With<PhysicsCollider>>,
) {
    stats.dynamic_transform_updates = 0;
    let Some(world) = context.world() else {
        return;
    };
    let mut updates = 0;
    if world
        .try_with_body_events_view(|events| {
            for event in events {
                let body = event.body_id();
                let Some(entity) = collision_world.dynamic_entities.get(&body).copied() else {
                    continue;
                };
                if event.fell_asleep() {
                    collision_world.sleeping_dynamic_bodies.insert(entity);
                } else {
                    collision_world.sleeping_dynamic_bodies.remove(&entity);
                }
                let Ok(mut transform) = roots.get_mut(entity) else {
                    continue;
                };
                let physics_transform = event.transform();
                transform.translation = Vec3::new(
                    physics_transform.p.x,
                    physics_transform.p.y,
                    physics_transform.p.z,
                );
                transform.rotation = from_box_quat(physics_transform.q);
                updates += 1;
            }
        })
        .is_err()
    {
        return;
    }
    stats.dynamic_transform_updates = updates;
    stats.sleeping_dynamic_bodies = collision_world.sleeping_dynamic_bodies.len();
    stats.awake_dynamic_bodies = collision_world
        .dynamic_bodies
        .len()
        .saturating_sub(stats.sleeping_dynamic_bodies);
}

pub(crate) fn sync_player_proxy(
    physics_disabled: Res<PhysicsDisabled>,
    no_clip: Res<PlayerNoClip>,
    state: Res<CameraModeState>,
    time: Res<Time<Fixed>>,
    mut collision_world: ResMut<PreparedCollisionWorld>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    players: Query<&Transform, With<FpsPlayer>>,
) {
    let Some(world) = context.world_mut() else {
        return;
    };
    let player_transform = (!physics_disabled.0 && !no_clip.0 && state.mode == CameraMode::Fps)
        .then(|| players.single().ok())
        .flatten();
    let Some(transform) = player_transform else {
        if let Some(body) = collision_world.player_proxy.take() {
            let _ = world.try_destroy_body(body);
        }
        return;
    };
    let target = boxddd::WorldTransform::new(
        to_box_vec3(transform.translation).into(),
        to_box_quat(transform.rotation),
    );
    if let Some(body) = collision_world.player_proxy {
        let _ = world.try_set_body_target_transform(body, target, time.delta_secs(), true);
        return;
    }
    let body = world.create_body(
        BodyDef::builder()
            .body_type(BodyType::Kinematic)
            .position(to_box_vec3(transform.translation))
            .rotation(to_box_quat(transform.rotation))
            .build(),
    );
    let shape_def = ShapeDef::builder()
        .density(0.0)
        .friction(0.8)
        .filter(Filter {
            category_bits: PLAYER_PROXY,
            mask_bits: WORLD_DYNAMIC,
            group_index: 0,
        })
        .build();
    let capsule = boxddd::Capsule::new(
        [0.0, -(CAPSULE_HEIGHT * 0.5 - CAPSULE_RADIUS), 0.0],
        [0.0, CAPSULE_HEIGHT * 0.5 - CAPSULE_RADIUS, 0.0],
        CAPSULE_RADIUS,
    );
    if world
        .try_create_capsule_shape(body, &shape_def, &capsule)
        .is_ok()
    {
        collision_world.player_proxy = Some(body);
    } else {
        let _ = world.try_destroy_body(body);
    }
}
