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
    static_queue: super::super::world::ColliderBuildQueue<usize>,
    dynamic_queue: super::super::world::ColliderBuildQueue<usize>,
    static_ready: bool,
    started: Instant,
    startup: bool,
    defer_dynamic_until_next_update: bool,
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
        *world.resource_mut::<CellPhysicsReadiness>() = CellPhysicsReadiness::Ready;
        return;
    }
    let work = manifest
        .placements
        .iter()
        .enumerate()
        .filter(|(_, placement)| {
            placement.initially_enabled && placement.physics_asset_path.is_some()
        })
        .map(|(index, placement)| {
            (
                index,
                placement.physics_classification == PreparedPhysicsClassification::Dynamic,
            )
        });
    let (static_indices, dynamic_indices) = super::super::world::partition_collider_indices(work);
    let static_queue = super::super::world::ColliderBuildQueue::new(static_indices);
    let dynamic_queue = super::super::world::ColliderBuildQueue::new(dynamic_indices);
    let queued = static_queue.len() + dynamic_queue.len();
    if queued == 0 {
        *world.resource_mut::<CellPhysicsReadiness>() = CellPhysicsReadiness::Ready;
        return;
    }
    let form_id = manifest.cell.form_id;
    world.resource_mut::<PendingColliderBuild>().0 = Some(PendingColliderBuildState {
        manifest,
        static_queue,
        dynamic_queue,
        static_ready: false,
        started: Instant::now(),
        startup: false,
        defer_dynamic_until_next_update: false,
        root_by_reference,
        unknown_layers: HashSet::new(),
        static_marker_spawned: false,
    });
    *world.resource_mut::<CellPhysicsReadiness>() = CellPhysicsReadiness::BuildingStatic;
    info!("swap colliders queued for {form_id:08x} ({queued} placements)");
}

/// Issue #52: drains up to `world::COLLIDER_BUILD_BUDGET_PER_FRAME`
/// queued placements per frame from `PendingColliderBuild`. Static and
/// keyframed work is always drained before the dynamic queue starts, so every
/// gravity-enabled prop sees the destination floor and walls on its first
/// simulation step. Sidecars the startup load never saw (a swapped-in cell's)
/// are read lazily here, so their file I/O rides the same per-frame budget. A
/// no-op when nothing is queued.
#[allow(clippy::too_many_arguments)]
pub(crate) fn advance_pending_collider_builds(
    mut commands: Commands,
    mut pending: ResMut<PendingColliderBuild>,
    mut physics_assets: ResMut<PreparedPhysicsAssets>,
    mut stats: ResMut<CollisionRuntimeStats>,
    mut collision_world: ResMut<PreparedCollisionWorld>,
    mut cell_physics: ResMut<CellPhysicsReadiness>,
    mut camera_state: ResMut<CameraModeState>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    mut restores: ResMut<super::super::world::PersistRestores>,
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
    if state.defer_dynamic_until_next_update {
        // Startup builds the static phase from OnEnter. Leave one schedule
        // boundary for BoxDDD's first broad-phase update before creating
        // gravity-enabled bodies.
        state.defer_dynamic_until_next_update = false;
        return;
    }
    let asset_root = PathBuf::from(&state.manifest.asset_root);
    let form_id = state.manifest.cell.form_id;
    if !state.static_ready && state.static_queue.is_empty() {
        state.static_ready = true;
        world
            .try_rebuild_static_tree()
            .expect("BoxDDD failed to rebuild swapped static tree");
        *cell_physics = if state.dynamic_queue.is_empty() {
            CellPhysicsReadiness::Ready
        } else {
            CellPhysicsReadiness::BuildingDynamic
        };
        info!("swap colliders static ready for {form_id:08x}");
    }
    let phase = super::super::world::next_collider_build_phase(
        !state.static_queue.is_empty(),
        !state.dynamic_queue.is_empty(),
        state.static_ready,
    );
    let dynamic_phase = matches!(phase, super::super::world::ColliderBuildPhase::Dynamic);
    let batch = match phase {
        super::super::world::ColliderBuildPhase::Static => state
            .static_queue
            .drain_budget(super::super::world::COLLIDER_BUILD_BUDGET_PER_FRAME),
        super::super::world::ColliderBuildPhase::Dynamic => state
            .dynamic_queue
            .drain_budget(super::super::world::COLLIDER_BUILD_BUDGET_PER_FRAME),
        super::super::world::ColliderBuildPhase::Ready => Vec::new(),
    };
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
            state.manifest.cell.form_id,
            &placement,
            &physics_assets,
            static_body,
            &state.root_by_reference,
            &mut collision_world,
            &mut stats,
            &mut state.unknown_layers,
            &mut state.static_marker_spawned,
            dynamic_phase,
            &mut restores,
        );
    }
    if !state.static_ready && state.static_queue.is_empty() {
        state.static_ready = true;
        world
            .try_rebuild_static_tree()
            .expect("BoxDDD failed to rebuild swapped static tree");
        *cell_physics = if state.dynamic_queue.is_empty() {
            CellPhysicsReadiness::Ready
        } else {
            CellPhysicsReadiness::BuildingDynamic
        };
        info!("swap colliders static ready for {form_id:08x}");
    }
    let complete = state.static_ready && state.dynamic_queue.is_empty();
    if complete {
        let startup = state.startup;
        let started = state.started;
        *cell_physics = CellPhysicsReadiness::Ready;
        if startup {
            stats.awake_dynamic_bodies = stats.dynamic_bodies;
            stats.sleeping_dynamic_bodies = 0;
            stats.dynamic_transform_updates = 0;
            stats.cooking_millis = started.elapsed().as_secs_f64() * 1000.0;
            camera_state.collision_build_complete = true;
            camera_state.collisions_ready = stats.shapes > 0;
            info!("prepared collision dynamic colliders ready for {form_id:08x}");
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
            if !camera_state.collisions_ready {
                warn!(
                    "prepared physics produced no active player-blocking shapes; FPS will be forced no-clip"
                );
            }
        } else {
            info!("swap colliders dynamic ready for {form_id:08x}");
            info!("swap colliders complete for {form_id:08x}");
        }
    }
    if complete {
        pending.0 = None;
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn build_prepared_colliders(
    mut commands: Commands,
    physics_disabled: Res<PhysicsDisabled>,
    manifest: Res<crate::viewer::LoadedSceneManifest>,
    physics_assets: Res<PreparedPhysicsAssets>,
    mut state: ResMut<CameraModeState>,
    mut stats: ResMut<CollisionRuntimeStats>,
    mut collision_world: ResMut<PreparedCollisionWorld>,
    mut pending: ResMut<PendingColliderBuild>,
    mut cell_physics: ResMut<CellPhysicsReadiness>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    roots: Query<(Entity, &super::super::interaction::PlacementRoot)>,
    mut restores: ResMut<super::super::world::PersistRestores>,
) {
    if physics_disabled.0 {
        state.collision_build_complete = true;
        *cell_physics = CellPhysicsReadiness::Ready;
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
    // The startup path is intentionally all-at-once, but it still needs the
    // same collision ordering as a cell swap: BoxDDD must see every static
    // and keyframed shape before a dynamic body is allowed to simulate.
    let (static_indices, dynamic_indices) = super::super::world::partition_collider_indices(
        manifest
            .placements
            .iter()
            .enumerate()
            .filter(|(_, placement)| {
                placement.initially_enabled && placement.physics_asset_path.is_some()
            })
            .map(|(index, placement)| {
                (
                    index,
                    placement.physics_classification == PreparedPhysicsClassification::Dynamic,
                )
            }),
    );
    for index in &static_indices {
        let placement = &manifest.placements[*index];
        build_colliders_for_placement(
            world,
            &mut commands,
            manifest.cell.form_id,
            placement,
            &physics_assets,
            static_body,
            &root_by_reference,
            &mut collision_world,
            &mut stats,
            &mut unknown_layers,
            &mut static_marker_spawned,
            false,
            &mut restores,
        );
    }
    // BoxDDD rebuilds its static broad phase during a world step. Startup has
    // not had that step yet, so publish the completed static phase explicitly
    // before creating gravity-enabled bodies. Swap construction gets this
    // boundary naturally from its static-only frame.
    if let Err(panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = world.try_rebuild_static_tree();
    })) {
        warn!(
            "BoxDDD static tree rebuild encountered panic ({panic:?}); dropping invalid static tree nodes and continuing"
        );
    }
    let form_id = manifest.cell.form_id;
    info!("prepared collision static ready for {form_id:08x}");
    if dynamic_indices.is_empty() {
        stats.awake_dynamic_bodies = stats.dynamic_bodies;
        stats.sleeping_dynamic_bodies = 0;
        stats.dynamic_transform_updates = 0;
        stats.cooking_millis = started.elapsed().as_secs_f64() * 1000.0;
        state.collision_build_complete = true;
        state.collisions_ready = stats.shapes > 0;
        *cell_physics = CellPhysicsReadiness::Ready;
        info!("prepared collision dynamic colliders ready for {form_id:08x}");
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
                "prepared physics produced no active player-blocking shapes; FPS will be forced no-clip"
            );
        }
        return;
    }
    stats.awake_dynamic_bodies = 0;
    stats.sleeping_dynamic_bodies = 0;
    stats.dynamic_transform_updates = 0;
    pending.0 = Some(PendingColliderBuildState {
        manifest: Arc::new((**manifest).clone()),
        static_queue: super::super::world::ColliderBuildQueue::new(Vec::new()),
        dynamic_queue: super::super::world::ColliderBuildQueue::new(dynamic_indices),
        static_ready: true,
        started,
        startup: true,
        defer_dynamic_until_next_update: true,
        root_by_reference,
        unknown_layers,
        static_marker_spawned,
    });
    state.collision_build_complete = false;
    state.collisions_ready = stats.shapes > 0;
    *cell_physics = CellPhysicsReadiness::BuildingDynamic;
}

/// Builds every BoxDDD body/shape for one placement (shared by the startup
/// `build_prepared_colliders` pass and issue #52's staggered
/// `advance_pending_collider_builds`), attaching static shapes to the
/// single shared `static_body` and dynamic ones to a per-entity body looked
/// up in `root_by_reference`. `allow_dynamic_bodies` is false during the
/// destination static phase and true only after that phase is complete.
#[allow(clippy::too_many_arguments)]
fn build_colliders_for_placement(
    world: &mut boxddd::World,
    commands: &mut Commands,
    cell_form_id: u32,
    placement: &crate::vsa::PreparedPlacement,
    physics_assets: &PreparedPhysicsAssets,
    static_body: BodyId,
    root_by_reference: &HashMap<u32, Entity>,
    collision_world: &mut PreparedCollisionWorld,
    stats: &mut CollisionRuntimeStats,
    unknown_layers: &mut HashSet<u8>,
    static_marker_spawned: &mut bool,
    allow_dynamic_bodies: bool,
    restores: &mut super::super::world::PersistRestores,
) {
    // Actor collision is opt-in through the developer `ragdoll` command. A
    // locked T-pose must not silently acquire a static skeleton collider.
    if matches!(
        placement.semantic,
        crate::vsa::PreparedSemantic::Npc(_) | crate::vsa::PreparedSemantic::Creature(_)
    ) {
        return;
    }
    // Issues #60/#61: a reference the save layer deleted (taken pickup)
    // must never get colliders rebuilt for it.
    if restores.suppressed.contains(&placement.reference_form_id) {
        return;
    }
    let Some(path) = placement.physics_asset_path.as_ref() else {
        return;
    };
    let Some(asset) = physics_assets.assets.get(path) else {
        warn!("missing preloaded physics sidecar {path}");
        return;
    };
    let dynamic_entity = (allow_dynamic_bodies
        && placement.physics_classification == PreparedPhysicsClassification::Dynamic)
        .then(|| root_by_reference.get(&placement.reference_form_id).copied())
        .flatten();
    // Issues #60/#61: re-activating a cell that stayed resident re-queues
    // its collider build; this placement's dynamic body is still alive with
    // its live pose/velocity, and building a second one would snap the
    // entity back to its manifest rest pose. Keep the live body.
    if let Some(entity) = dynamic_entity
        && collision_world.dynamic_bodies.contains_key(&entity)
    {
        return;
    }
    let rotation = Quat::from_array(placement.rotation_xyzw);
    let translation = Vec3::from_array(placement.translation);
    if !translation.is_finite()
        || !rotation.is_finite()
        || !placement.scale.is_finite()
        || placement.scale.abs() <= 0.0001
    {
        warn!(
            "BoxDDD dropped collider for reference {:08x} with non-finite transform: pos={:?}, rot={:?}, scale={}",
            placement.reference_form_id,
            placement.translation,
            placement.rotation_xyzw,
            placement.scale
        );
        return;
    }
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
            // Issue #60 (F60.2): a saved pose/velocity for this reference is
            // re-applied the moment its body exists, so the staggered build
            // itself performs the restoration instead of a later system
            // fighting it.
            if let Some(restore) = restores.bodies.remove(&placement.reference_form_id) {
                apply_dynamic_body_restore(world, body_id, &restore);
                info!(
                    "save restore body {:08x} sleeping={}",
                    placement.reference_form_id, restore.sleeping
                );
            }
            collision_world.dynamic_bodies.insert(entity, body_id);
            collision_world.dynamic_entities.insert(body_id, entity);
            collision_world
                .ledger
                .record_dynamic_body(cell_form_id, body_id);
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
                .ledger
                .record_keyframed_body(cell_form_id, body_id);
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
            info!(
                "building shape {} for reference {:08x} asset {}",
                shape.kind(),
                placement.reference_form_id,
                placement
                    .physics_asset_path
                    .as_deref()
                    .unwrap_or_else(|| placement.asset_path.as_deref().unwrap_or(""))
            );
            let result = create_prepared_shape(
                world,
                body_id,
                body,
                shape,
                placement,
                PreparedShapeOptions {
                    dynamic,
                    local_space,
                    collision_group: 0,
                },
            );
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
            if body_id == static_body {
                collision_world
                    .ledger
                    .record_static_shape(cell_form_id, shape_id);
            } else {
                collision_world
                    .ledger
                    .record_body_shape(cell_form_id, shape_id);
            }
            stats.shapes += 1;
            stats.packed_triangles += triangles;
            *stats.shape_kinds.entry(shape.kind()).or_default() += 1;
        }
        if dynamic {
            normalize_dynamic_mass(world, body_id, body, placement.scale.abs());
        }
    }
}

/// Attaches or removes the prepared articulated actor bodies for the developer
/// ragdoll toggle. The first body is mirrored in the legacy one-body map so
/// existing persistence and transform-sync paths continue to work.
pub(crate) fn process_ragdoll_toggles(
    mut commands: Commands,
    physics_assets: Res<PreparedPhysicsAssets>,
    manifest: Res<crate::viewer::LoadedSceneManifest>,
    mut collision: ResMut<PreparedCollisionWorld>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    query: Query<(
        Entity,
        &super::super::interaction::PlacementRoot,
        &Transform,
        Option<&super::RagdollToggle>,
    )>,
    mut node_transforms: Query<&mut Transform, Without<super::super::interaction::PlacementRoot>>,
) {
    let Some(world) = context.world_mut() else {
        return;
    };
    for (entity, root, runtime_transform, toggle) in &query {
        let enabled = toggle.is_some_and(|toggle| toggle.0);
        let existing = collision.ragdoll_bodies.get(&entity).cloned();
        if !enabled {
            if let Some(bodies) = existing {
                let restores = toggle.is_none().then(|| {
                    collect_ragdoll_node_restores(
                        bodies
                            .iter()
                            .filter_map(|body| collision.ragdoll_nodes.get(body)),
                    )
                });
                if let Some(joints) = collision.ragdoll_joints.remove(&entity) {
                    for joint in joints {
                        let _ = world.try_destroy_joint(joint.id, true);
                    }
                }
                collision.ragdoll_bodies.remove(&entity);
                collision.dynamic_bodies.remove(&entity);
                collision.sleeping_dynamic_bodies.remove(&entity);
                for body in bodies {
                    collision.ragdoll_nodes.remove(&body);
                    collision.dynamic_entities.remove(&body);
                    let _ = world.try_destroy_body(body);
                }
                if let Some(restores) = restores {
                    for (node, rest) in restores {
                        if let Ok(mut transform) = node_transforms.get_mut(node) {
                            *transform = rest;
                        }
                    }
                }
            }
            continue;
        }
        if existing.is_some() {
            continue;
        }
        let placement = ragdoll_runtime_placement(root.placement(), runtime_transform);
        if !matches!(
            placement.semantic,
            crate::vsa::PreparedSemantic::Npc(_) | crate::vsa::PreparedSemantic::Creature(_)
        ) {
            continue;
        }
        let Some(asset) = placement
            .physics_asset_path
            .as_ref()
            .and_then(|path| physics_assets.assets.get(path))
        else {
            continue;
        };
        let mut body_ids = HashMap::new();
        let collision_group = ragdoll_collision_group(placement.reference_form_id);
        for source in &asset.bodies {
            // Actor skeleton layers are intentionally filtered from ordinary
            // player collision (for example layer 8), but a developer
            // ragdoll still needs those bodies. Only phantom/empty bodies are
            // omitted here.
            if source.phantom || source.shapes.is_empty() {
                continue;
            }
            let mut body = source.clone();
            body.motion_type = "MO_SYS_DYNAMIC".into();
            // Havok actor skeleton bodies are commonly authored keyframed
            // with zero gravity. The developer drop explicitly turns gravity
            // back on while preserving the source mass/limits.
            body.gravity_factor = 1.0;
            body.mass = if body.mass.is_finite() && body.mass > 0.0 {
                body.mass
            } else {
                5.0
            };
            tune_ragdoll_body(&mut body);
            body.constrained = false;
            body.shapes.retain(PreparedPhysicsShape::supports_dynamic);
            if body.shapes.is_empty() {
                continue;
            }
            let body_anchor = recenter_ragdoll_body(&mut body);
            let body_id =
                create_dynamic_body_at_local_anchor(world, &placement, &body, body_anchor);
            let shapes = body
                .shapes
                .iter()
                .filter_map(|shape| {
                    create_prepared_shape(
                        world,
                        body_id,
                        &body,
                        shape,
                        &placement,
                        PreparedShapeOptions {
                            dynamic: true,
                            local_space: true,
                            collision_group,
                        },
                    )
                    .map(|(shape_id, _)| shape_id)
                })
                .collect::<Vec<_>>();
            if shapes.is_empty() {
                let _ = world.try_destroy_body(body_id);
                continue;
            }
            normalize_dynamic_mass(world, body_id, &body, placement.scale.abs());
            body_ids.insert(source.group_id, (body_id, body_anchor));
            if let Some(node) = source.node.clone() {
                collision.ragdoll_nodes.insert(
                    body_id,
                    RagdollNodeBinding {
                        root: entity,
                        node,
                        node_entities: Vec::new(),
                        rest_body: None,
                        rest_node_globals: Vec::new(),
                        rest_node_locals: Vec::new(),
                    },
                );
            }
            collision.dynamic_entities.insert(body_id, entity);
            collision
                .ledger
                .record_dynamic_body(manifest.cell.form_id, body_id);
            for shape_id in shapes {
                collision.surfaces.insert(
                    shape_id,
                    CollisionSurface {
                        material: body.material,
                    },
                );
                collision
                    .ledger
                    .record_body_shape(manifest.cell.form_id, shape_id);
            }
        }
        let Some(primary) = body_ids.values().next().map(|(body, _)| *body) else {
            continue;
        };
        let mut joint_ids = Vec::new();
        for joint in &asset.joints {
            let (Some((body_a, anchor_a)), Some((body_b, anchor_b))) =
                (body_ids.get(&joint.body_a), body_ids.get(&joint.body_b))
            else {
                warn!(
                    "ragdoll sidecar runtime endpoint missing actor={:08x} bodies={}-{}",
                    placement.reference_form_id, joint.body_a, joint.body_b
                );
                continue;
            };
            let scale = placement.scale.abs();
            let frame_a = boxddd::Transform::new(
                to_box_vec3(ragdoll_joint_local_anchor(joint.anchor_a, *anchor_a, scale)),
                to_box_quat(Quat::from_array(joint.frame_a_rotation_xyzw)),
            );
            let frame_b = boxddd::Transform::new(
                to_box_vec3(ragdoll_joint_local_anchor(joint.anchor_b, *anchor_b, scale)),
                to_box_quat(Quat::from_array(joint.frame_b_rotation_xyzw)),
            );
            let result = match joint.kind.as_str() {
                "spherical" => {
                    let mut definition = SphericalJointDef::new(*body_a, *body_b)
                        .local_frame_a(frame_a)
                        .local_frame_b(frame_b);
                    if let Some(angle) = joint.cone_limit {
                        definition = definition.cone_limit(true, angle.max(0.0));
                    }
                    if let (Some(lower), Some(upper)) =
                        (joint.twist_lower_limit, joint.twist_upper_limit)
                    {
                        definition = definition.twist_limit(true, lower, upper.max(lower));
                    }
                    world.try_create_spherical_joint(definition)
                }
                "prismatic" => world.try_create_prismatic_joint(
                    PrismaticJointDef::new(*body_a, *body_b)
                        .local_frame_a(frame_a)
                        .local_frame_b(frame_b)
                        .limit(
                            joint.lower_limit.is_some() || joint.upper_limit.is_some(),
                            joint.lower_limit.unwrap_or(0.0),
                            joint
                                .upper_limit
                                .unwrap_or(0.0)
                                .max(joint.lower_limit.unwrap_or(0.0)),
                        ),
                ),
                _ => world.try_create_revolute_joint(
                    RevoluteJointDef::new(*body_a, *body_b)
                        .local_frame_a(frame_a)
                        .local_frame_b(frame_b)
                        .limit(
                            joint.lower_limit.is_some() || joint.upper_limit.is_some(),
                            joint.lower_limit.unwrap_or(0.0),
                            joint
                                .upper_limit
                                .unwrap_or(0.0)
                                .max(joint.lower_limit.unwrap_or(0.0)),
                        ),
                ),
            };
            if let Ok(joint_id) = result {
                joint_ids.push(RagdollJointBinding {
                    id: joint_id,
                    kind: joint.kind.clone(),
                    body_a: joint.body_a,
                    body_b: joint.body_b,
                    node_a: asset
                        .bodies
                        .iter()
                        .find(|body| body.group_id == joint.body_a)
                        .and_then(|body| body.node.clone()),
                    node_b: asset
                        .bodies
                        .iter()
                        .find(|body| body.group_id == joint.body_b)
                        .and_then(|body| body.node.clone()),
                });
            }
        }
        let all_bodies = body_ids
            .into_values()
            .map(|(body, _)| body)
            .collect::<Vec<_>>();
        collision.dynamic_bodies.insert(entity, primary);
        collision.ragdoll_bodies.insert(entity, all_bodies);
        collision.ragdoll_joints.insert(entity, joint_ids);
        commands.entity(entity).insert(PhysicsCollider);
        info!(
            "ragdoll enabled reference {:08x} bodies={} joints={}",
            placement.reference_form_id,
            collision.ragdoll_bodies[&entity].len(),
            collision.ragdoll_joints[&entity].len()
        );
    }
}

/// Maps one prepared Havok body to the GLTF node that owns its collision
/// objects. The first physics pose and node pose are captured lazily because
/// scene children are spawned after the placement root.
#[derive(Clone)]
pub(crate) struct RagdollNodeBinding {
    pub(crate) root: Entity,
    pub(crate) node: String,
    pub(crate) node_entities: Vec<Entity>,
    pub(crate) rest_body: Option<Affine3A>,
    pub(crate) rest_node_globals: Vec<Affine3A>,
    pub(crate) rest_node_locals: Vec<Transform>,
}

#[derive(Clone)]
pub(crate) struct RagdollJointBinding {
    pub(crate) id: JointId,
    pub(crate) kind: String,
    pub(crate) body_a: u32,
    pub(crate) body_b: u32,
    pub(crate) node_a: Option<String>,
    pub(crate) node_b: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct RagdollProbeSnapshot {
    pub(crate) instantiated: bool,
    pub(crate) body_count: usize,
    pub(crate) joint_count: usize,
    pub(crate) awake_count: usize,
    pub(crate) sleep_disabled_count: usize,
    pub(crate) max_linear_speed: f32,
    pub(crate) max_angular_speed: f32,
    pub(crate) max_joint_linear_separation: f32,
    pub(crate) max_joint_angular_separation: f32,
    pub(crate) max_node_position_error: f32,
    pub(crate) max_node_angle_error: f32,
    pub(crate) bodies: Vec<RagdollBodyProbe>,
    pub(crate) joints: Vec<RagdollJointProbe>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct RagdollBodyProbe {
    pub(crate) index: usize,
    pub(crate) node: Option<String>,
    pub(crate) awake: bool,
    pub(crate) sleep_enabled: bool,
    pub(crate) linear_speed: f32,
    pub(crate) angular_speed: f32,
    pub(crate) position: [f32; 3],
    pub(crate) node_position_error: Option<f32>,
    pub(crate) node_angle_error: Option<f32>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct RagdollJointProbe {
    pub(crate) index: usize,
    pub(crate) kind: String,
    pub(crate) body_a: u32,
    pub(crate) body_b: u32,
    pub(crate) node_a: Option<String>,
    pub(crate) node_b: Option<String>,
    pub(crate) linear_separation: f32,
    pub(crate) angular_separation: f32,
    pub(crate) constraint_force: [f32; 3],
    pub(crate) constraint_torque: [f32; 3],
}

/// Captures a bounded, read-only view of one live articulated actor. This is
/// deliberately queried on demand rather than sampled every frame.
pub(crate) fn probe_ragdoll(world: &mut World, entity: Entity) -> RagdollProbeSnapshot {
    let Some(collision) = world.get_resource::<PreparedCollisionWorld>() else {
        return empty_ragdoll_probe();
    };
    let Some(body_ids) = collision.ragdoll_bodies.get(&entity).cloned() else {
        return empty_ragdoll_probe();
    };
    let joint_ids = collision
        .ragdoll_joints
        .get(&entity)
        .cloned()
        .unwrap_or_default();
    let bindings = body_ids
        .iter()
        .filter_map(|body| {
            collision
                .ragdoll_nodes
                .get(body)
                .cloned()
                .map(|binding| (*body, binding))
        })
        .collect::<HashMap<_, _>>();
    let node_globals = bindings
        .values()
        .flat_map(|binding| binding.node_entities.iter().copied())
        .filter_map(|node| {
            world
                .get::<GlobalTransform>(node)
                .map(|transform| (node, transform.affine()))
        })
        .collect::<HashMap<_, _>>();

    let Some(context) = world.get_non_send::<BoxdddPhysicsContext>() else {
        return empty_ragdoll_probe();
    };
    let Some(physics) = context.world() else {
        return empty_ragdoll_probe();
    };
    let mut bodies = Vec::with_capacity(body_ids.len());
    for (index, body) in body_ids.iter().copied().enumerate() {
        let Ok(transform) = physics.try_body_transform(body) else {
            continue;
        };
        let linear = physics.try_body_linear_velocity(body).unwrap_or_default();
        let angular = physics.try_body_angular_velocity(body).unwrap_or_default();
        let awake = physics.try_body_awake(body).unwrap_or(false);
        let sleep_enabled = physics.try_body_sleep_enabled(body).unwrap_or(false);
        let body_affine = Affine3A::from_rotation_translation(
            from_box_quat(transform.q),
            Vec3::new(transform.p.x, transform.p.y, transform.p.z),
        );
        let binding = bindings.get(&body);
        let (node_position_error, node_angle_error) = binding
            .and_then(|binding| {
                let rest_body = binding.rest_body?;
                let delta = body_affine * rest_body.inverse();
                let errors = binding
                    .node_entities
                    .iter()
                    .zip(&binding.rest_node_globals)
                    .filter_map(|(node, rest_node)| {
                        let actual = node_globals.get(node)?;
                        let expected = delta * *rest_node;
                        let (_, expected_rotation, expected_translation) =
                            expected.to_scale_rotation_translation();
                        let (_, actual_rotation, actual_translation) =
                            actual.to_scale_rotation_translation();
                        Some((
                            expected_translation.distance(actual_translation),
                            expected_rotation.angle_between(actual_rotation),
                        ))
                    })
                    .collect::<Vec<_>>();
                (!errors.is_empty()).then(|| {
                    errors
                        .into_iter()
                        .fold((0.0_f32, 0.0_f32), |maxima, error| {
                            (maxima.0.max(error.0), maxima.1.max(error.1))
                        })
                })
            })
            .map_or((None, None), |(position, angle)| {
                (Some(position), Some(angle))
            });
        bodies.push(RagdollBodyProbe {
            index,
            node: binding.map(|binding| binding.node.clone()),
            awake,
            sleep_enabled,
            linear_speed: Vec3::new(linear.x, linear.y, linear.z).length(),
            angular_speed: Vec3::new(angular.x, angular.y, angular.z).length(),
            position: [transform.p.x, transform.p.y, transform.p.z],
            node_position_error,
            node_angle_error,
        });
    }
    let joints = joint_ids
        .iter()
        .enumerate()
        .filter_map(|(index, joint)| {
            let linear_separation = physics.try_joint_linear_separation(joint.id).ok()?;
            let angular_separation = physics.try_joint_angular_separation(joint.id).ok()?;
            let force = physics
                .try_joint_constraint_force(joint.id)
                .unwrap_or_default();
            let torque = physics
                .try_joint_constraint_torque(joint.id)
                .unwrap_or_default();
            Some(RagdollJointProbe {
                index,
                kind: joint.kind.clone(),
                body_a: joint.body_a,
                body_b: joint.body_b,
                node_a: joint.node_a.clone(),
                node_b: joint.node_b.clone(),
                linear_separation,
                angular_separation,
                constraint_force: [force.x, force.y, force.z],
                constraint_torque: [torque.x, torque.y, torque.z],
            })
        })
        .collect::<Vec<_>>();
    RagdollProbeSnapshot {
        instantiated: true,
        body_count: bodies.len(),
        joint_count: joints.len(),
        awake_count: bodies.iter().filter(|body| body.awake).count(),
        sleep_disabled_count: bodies.iter().filter(|body| !body.sleep_enabled).count(),
        max_linear_speed: bodies
            .iter()
            .map(|body| body.linear_speed)
            .fold(0.0, f32::max),
        max_angular_speed: bodies
            .iter()
            .map(|body| body.angular_speed)
            .fold(0.0, f32::max),
        max_joint_linear_separation: joints
            .iter()
            .map(|joint| joint.linear_separation.abs())
            .fold(0.0, f32::max),
        max_joint_angular_separation: joints
            .iter()
            .map(|joint| joint.angular_separation.abs())
            .fold(0.0, f32::max),
        max_node_position_error: bodies
            .iter()
            .filter_map(|body| body.node_position_error)
            .fold(0.0, f32::max),
        max_node_angle_error: bodies
            .iter()
            .filter_map(|body| body.node_angle_error)
            .fold(0.0, f32::max),
        bodies,
        joints,
    }
}

fn empty_ragdoll_probe() -> RagdollProbeSnapshot {
    RagdollProbeSnapshot {
        instantiated: false,
        body_count: 0,
        joint_count: 0,
        awake_count: 0,
        sleep_disabled_count: 0,
        max_linear_speed: 0.0,
        max_angular_speed: 0.0,
        max_joint_linear_separation: 0.0,
        max_joint_angular_separation: 0.0,
        max_node_position_error: 0.0,
        max_node_angle_error: 0.0,
        bodies: Vec::new(),
        joints: Vec::new(),
    }
}

pub(crate) fn collect_ragdoll_node_restores<'a>(
    bindings: impl IntoIterator<Item = &'a RagdollNodeBinding>,
) -> Vec<(Entity, Transform)> {
    bindings
        .into_iter()
        .flat_map(|binding| {
            binding
                .node_entities
                .iter()
                .copied()
                .zip(binding.rest_node_locals.iter().cloned())
        })
        .collect()
}

fn shape_anchor(shape: &PreparedPhysicsShape) -> Vec3 {
    match shape {
        PreparedPhysicsShape::Box { center, .. } | PreparedPhysicsShape::Sphere { center, .. } => {
            Vec3::from_array(*center)
        }
        PreparedPhysicsShape::Capsule { point1, point2, .. } => {
            (Vec3::from_array(*point1) + Vec3::from_array(*point2)) * 0.5
        }
        PreparedPhysicsShape::ConvexHull { points }
        | PreparedPhysicsShape::TriangleMesh {
            vertices: points, ..
        } => {
            if points.is_empty() {
                Vec3::ZERO
            } else {
                points
                    .iter()
                    .map(|point| Vec3::from_array(*point))
                    .sum::<Vec3>()
                    / points.len() as f32
            }
        }
    }
}

pub(crate) fn tune_ragdoll_body(body: &mut PreparedPhysicsBody) {
    body.linear_damping = body.linear_damping.max(0.2);
    body.angular_damping = body.angular_damping.max(0.25);
    body.friction = body.friction.max(0.6);
    body.restitution = 0.0;
    body.sleep_enabled = true;
    body.linear_velocity = [0.0; 3];
    body.angular_velocity = [0.0; 3];
}

fn translate_shape(shape: &mut PreparedPhysicsShape, translation: Vec3) {
    let translate = |point: &mut [f32; 3]| {
        *point = (Vec3::from_array(*point) + translation).to_array();
    };
    match shape {
        PreparedPhysicsShape::Box { center, .. } | PreparedPhysicsShape::Sphere { center, .. } => {
            translate(center)
        }
        PreparedPhysicsShape::Capsule { point1, point2, .. } => {
            translate(point1);
            translate(point2);
        }
        PreparedPhysicsShape::ConvexHull { points }
        | PreparedPhysicsShape::TriangleMesh {
            vertices: points, ..
        } => {
            for point in points {
                translate(point);
            }
        }
    }
}

pub(crate) fn recenter_ragdoll_body(body: &mut PreparedPhysicsBody) -> Vec3 {
    if body.shapes.is_empty() {
        return Vec3::ZERO;
    }
    let anchor = body.shapes.iter().map(shape_anchor).sum::<Vec3>() / body.shapes.len() as f32;
    for shape in &mut body.shapes {
        translate_shape(shape, -anchor);
    }
    body.center_of_mass = (Vec3::from_array(body.center_of_mass) - anchor).to_array();
    anchor
}

pub(crate) fn ragdoll_joint_local_anchor(
    authored_anchor: [f32; 3],
    body_anchor: Vec3,
    scale: f32,
) -> Vec3 {
    (Vec3::from_array(authored_anchor) - body_anchor) * scale
}

pub(crate) fn ragdoll_collision_group(_reference_form_id: u32) -> i32 {
    // Group zero uses the ordinary category/mask filter. Joint-connected
    // bodies still use BoxDDD's default collideConnected=false, while
    // non-adjacent limbs can now collide with the torso and each other rather
    // than collapsing through the complete articulated actor.
    0
}

pub(crate) fn ragdoll_runtime_placement(
    prepared: &crate::vsa::PreparedPlacement,
    runtime: &Transform,
) -> crate::vsa::PreparedPlacement {
    let mut placement = prepared.clone();
    placement.translation = runtime.translation.to_array();
    placement.rotation_xyzw = runtime.rotation.normalize().to_array();
    let uniform_scale = runtime.scale.x.abs();
    if uniform_scale.is_finite() && uniform_scale > 0.0 {
        placement.scale = uniform_scale;
    }
    placement
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

fn actor_node_key(name: &str) -> String {
    let mut parts = name
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>();
    if parts.len() >= 3 && parts.first().is_some_and(|part| part.starts_with("bip")) {
        let side = parts.last().is_some_and(|part| part == "l" || part == "r");
        if side {
            let side = parts.pop().expect("checked non-empty actor node parts");
            parts.insert(1, side);
        }
    }
    parts.concat()
}

pub(crate) fn actor_node_names_match(physics_name: &str, scene_name: &str) -> bool {
    actor_node_key(physics_name) == actor_node_key(scene_name)
}

fn find_actor_node_descendants(
    root: Entity,
    name: &str,
    children: &Query<&Children>,
    names: &Query<&Name>,
) -> Vec<Entity> {
    let mut matches = Vec::new();
    let mut queue = vec![root];
    while let Some(entity) = queue.pop() {
        if names
            .get(entity)
            .is_ok_and(|candidate| actor_node_names_match(name, candidate.as_str()))
        {
            matches.push(entity);
        }
        if let Ok(child_list) = children.get(entity) {
            queue.extend(child_list.iter());
        }
    }
    matches
}

pub(crate) const fn dynamic_body_drives_placement_root(
    articulated_ragdoll: bool,
    primary_body: bool,
) -> bool {
    !articulated_ragdoll && primary_body
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
    create_dynamic_body_at_local_anchor(world, placement, body, Vec3::ZERO)
}

pub(crate) fn create_dynamic_body_at_local_anchor(
    world: &mut boxddd::World,
    placement: &crate::vsa::PreparedPlacement,
    body: &PreparedPhysicsBody,
    local_anchor: Vec3,
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
            .position(to_box_vec3(
                Vec3::from_array(placement.translation) + rotation * (local_anchor * scale),
            ))
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

/// Issue #60 (F60.2): re-applies a saved dynamic-body pose/velocity/sleep
/// state. Shared by the collider-build hook above (fresh bodies) and
/// `world::persist`'s immediate path (bodies that survived as live
/// residents).
pub(crate) fn apply_dynamic_body_restore(
    world: &mut boxddd::World,
    body_id: BodyId,
    restore: &super::super::world::DynamicBodyRestore,
) {
    let _ = world.try_set_body_transform(
        body_id,
        to_box_vec3(restore.translation),
        to_box_quat(restore.rotation),
    );
    let _ = world.try_set_body_linear_velocity(body_id, to_box_vec3(restore.linear_velocity));
    let _ = world.try_set_body_angular_velocity(body_id, to_box_vec3(restore.angular_velocity));
    let _ = world.try_set_body_awake(body_id, !restore.sleeping);
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PreparedShapeOptions {
    pub(crate) dynamic: bool,
    pub(crate) local_space: bool,
    pub(crate) collision_group: i32,
}

pub(crate) fn create_prepared_shape(
    world: &mut boxddd::World,
    body_id: BodyId,
    body: &PreparedPhysicsBody,
    shape: &PreparedPhysicsShape,
    placement: &crate::vsa::PreparedPlacement,
    options: PreparedShapeOptions,
) -> Option<(ShapeId, usize)> {
    let PreparedShapeOptions {
        dynamic,
        local_space,
        collision_group,
    } = options;
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
            group_index: collision_group,
        })
        .user_material_id(u64::from(body.material.unwrap_or(0)))
        .build();
    let shape_id = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<ShapeId> {
        match shape {
            PreparedPhysicsShape::Box {
                center,
                half_extents,
                rotation_xyzw,
            } => {
                let center = point(*center);
                let rotation = rotation(*rotation_xyzw);
                let half = Vec3::from_array(*half_extents) * scale;
                if half.x <= 0.0001 || half.y <= 0.0001 || half.z <= 0.0001 {
                    return None;
                }
                let hull = BoxHull::transformed(
                    half.x,
                    half.y,
                    half.z,
                    boxddd::Transform::new(to_box_vec3(center), to_box_quat(rotation)),
                );
                world.try_create_hull_shape(body_id, &shape_def, &hull).ok()
            }
            PreparedPhysicsShape::Sphere { center, radius } => {
                let r = radius * scale;
                if r <= 0.0001 {
                    return None;
                }
                world
                    .try_create_sphere_shape(
                        body_id,
                        &shape_def,
                        &boxddd::Sphere::new(to_box_vec3(point(*center)), r),
                    )
                    .ok()
            }
            PreparedPhysicsShape::Capsule {
                point1,
                point2,
                radius,
            } => {
                let r = radius * scale;
                if r <= 0.0001 {
                    return None;
                }
                world
                    .try_create_capsule_shape(
                        body_id,
                        &shape_def,
                        &boxddd::Capsule::new(
                            to_box_vec3(point(*point1)),
                            to_box_vec3(point(*point2)),
                            r,
                        ),
                    )
                    .ok()
            }
            PreparedPhysicsShape::ConvexHull { points } => {
                if points.len() < 4 {
                    return None;
                }
                let (mut min_x, mut max_x) = (f32::INFINITY, f32::NEG_INFINITY);
                let (mut min_y, mut max_y) = (f32::INFINITY, f32::NEG_INFINITY);
                let (mut min_z, mut max_z) = (f32::INFINITY, f32::NEG_INFINITY);
                for p in points {
                    if !p[0].is_finite() || !p[1].is_finite() || !p[2].is_finite() {
                        return None;
                    }
                    min_x = min_x.min(p[0]);
                    max_x = max_x.max(p[0]);
                    min_y = min_y.min(p[1]);
                    max_y = max_y.max(p[1]);
                    min_z = min_z.min(p[2]);
                    max_z = max_z.max(p[2]);
                }
                if (max_x - min_x) < 0.001 || (max_y - min_y) < 0.001 || (max_z - min_z) < 0.001 {
                    warn!(
                        "BoxDDD dropped coplanar 2D convex_hull shape for reference {:08x}",
                        placement.reference_form_id
                    );
                    return None;
                }
                let points = points
                    .iter()
                    .map(|value| to_box_vec3(point(*value)))
                    .collect::<Vec<_>>();
                let hull = Hull::from_points(&points, 64).ok()?;
                world
                    .try_create_created_hull_shape(body_id, &shape_def, &hull)
                    .ok()
            }
            PreparedPhysicsShape::TriangleMesh { vertices, indices } => {
                if vertices.len() < 3 || indices.len() < 3 {
                    return None;
                }
                let vertices = vertices
                    .iter()
                    .map(|value| to_box_vec3(point(*value)))
                    .collect::<Vec<_>>();
                if vertices
                    .iter()
                    .any(|v| !v.x.is_finite() || !v.y.is_finite() || !v.z.is_finite())
                {
                    return None;
                }
                let indices = indices
                    .iter()
                    .map(|index| i32::try_from(*index).ok())
                    .collect::<Option<Vec<_>>>()?;
                if !indices.len().is_multiple_of(3) {
                    return None;
                }
                if indices
                    .iter()
                    .any(|&i| i < 0 || (i as usize) >= vertices.len())
                {
                    return None;
                }
                let mut two_sided_indices = Vec::with_capacity(indices.len() * 2);
                for triangle in indices.chunks_exact(3) {
                    two_sided_indices.extend_from_slice(triangle);
                    if !dynamic {
                        two_sided_indices.extend_from_slice(&[
                            triangle[0],
                            triangle[2],
                            triangle[1],
                        ]);
                    }
                }
                let mesh = boxddd::MeshData::builder(vertices, two_sided_indices)
                    .build()
                    .ok()?;
                world
                    .try_create_mesh_shape(
                        body_id,
                        &shape_def,
                        mesh,
                        boxddd::Vec3::new(1.0, 1.0, 1.0),
                    )
                    .ok()
            }
        }
    }))
    .ok()??;
    let triangle_count = match shape {
        PreparedPhysicsShape::TriangleMesh { indices, .. } if !dynamic => indices.len() * 2 / 3,
        _ => shape.triangle_count(),
    };
    Some((shape_id, triangle_count))
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

/// Issue #63: destroys everything `cell`'s collider build registered in the
/// per-cell ledger — its shapes on the shared static body, its keyframed
/// door/activator bodies, and its dynamic bodies — and cancels a
/// still-draining build for the cell. Called on swap-away (after
/// `world::persist`'s capture, which snapshots the dynamic poses this
/// destroys) and on eviction. Before this existed, every swap re-queued the
/// destination's full placement list with no teardown, so revisits
/// duplicated static shapes and keyframed bindings, and a departed cell's
/// colliders persisted for the rest of the session.
pub(crate) fn teardown_cell_colliders(world: &mut World, cell: u32) {
    {
        let mut pending = world.resource_mut::<PendingColliderBuild>();
        if pending
            .0
            .as_ref()
            .is_some_and(|state| state.manifest.cell.form_id == cell)
        {
            pending.0 = None;
            info!("swap colliders cancelled for {cell:08x} (cell torn down mid-build)");
        }
    }
    world.resource_scope(|world, mut collision_world: Mut<PreparedCollisionWorld>| {
        let mut context = world.non_send_mut::<BoxdddPhysicsContext>();
        let Some(boxddd_world) = context.world_mut() else {
            return;
        };
        let Some(owned) = collision_world.ledger.release(cell) else {
            return;
        };
        let static_shapes = owned.shape_count();
        let bodies = owned.body_count();
        for body in owned.dynamic_bodies {
            if let Some(entity) = collision_world.dynamic_entities.remove(&body) {
                collision_world.dynamic_bodies.remove(&entity);
                collision_world.sleeping_dynamic_bodies.remove(&entity);
            }
            let _ = boxddd_world.try_destroy_body(body);
        }
        let keyframed: HashSet<BodyId> = owned.keyframed_bodies.iter().copied().collect();
        for body in owned.keyframed_bodies {
            let _ = boxddd_world.try_destroy_body(body);
        }
        collision_world.keyframed_bodies.retain(|_, bindings| {
            bindings.retain(|binding| !keyframed.contains(&binding.body));
            !bindings.is_empty()
        });
        for shape in owned.static_shapes {
            let _ = boxddd_world.try_destroy_shape(shape, false);
            collision_world.surfaces.remove(&shape);
        }
        // Shapes on the destroyed bodies died with them; only their
        // surface-material bookkeeping needs cleaning.
        for shape in owned.body_shapes {
            collision_world.surfaces.remove(&shape);
        }
        info!("colliders teardown {cell:08x} static_shapes={static_shapes} bodies={bodies}");
    });
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
        if let Some(joints) = collision_world.ragdoll_joints.remove(&entity) {
            for joint in joints {
                let _ = world.try_destroy_joint(joint.id, true);
            }
        }
        if let Some(bodies) = collision_world.ragdoll_bodies.remove(&entity) {
            collision_world.dynamic_bodies.remove(&entity);
            collision_world.sleeping_dynamic_bodies.remove(&entity);
            for body in bodies {
                collision_world.ragdoll_nodes.remove(&body);
                collision_world.dynamic_entities.remove(&body);
                let _ = world.try_destroy_body(body);
            }
        } else if let Some(body) = collision_world.dynamic_bodies.remove(&entity) {
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

/// M3 wave 1: attaches a dynamic body to a player-created dropped item using
/// the prepared sidecar when it is dynamic-safe, otherwise a conservative
/// box proxy. Kept here so runtime drops share BoxDDD ownership/filtering with
/// prepared placements rather than creating a second physics integration.
pub(crate) fn attach_runtime_item_colliders(
    mut commands: Commands,
    disabled: Res<PhysicsDisabled>,
    physics_assets: Res<PreparedPhysicsAssets>,
    mut collision: ResMut<PreparedCollisionWorld>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    pending: Query<(
        Entity,
        &super::super::world_items::PendingRuntimeCollider,
        &super::super::world_items::RuntimeWorldItem,
    )>,
) {
    let Some(world) = context.world_mut() else {
        return;
    };
    for (entity, pending, runtime_item) in &pending {
        if disabled.0 {
            commands
                .entity(entity)
                .remove::<super::super::world_items::PendingRuntimeCollider>();
            continue;
        }
        let placement = &pending.placement;
        let prepared = placement
            .physics_asset_path
            .as_ref()
            .and_then(|path| physics_assets.assets.get(path));
        let mut body = match &pending.drop_collider {
            crate::vsa::PreparedDropCollider::Authored => prepared.and_then(|asset| {
                asset
                    .bodies
                    .iter()
                    .find(|body| {
                        body_blocks_player(body)
                            && !body.shapes.is_empty()
                            && body
                                .shapes
                                .iter()
                                .all(PreparedPhysicsShape::supports_dynamic)
                    })
                    .cloned()
            }),
            crate::vsa::PreparedDropCollider::BoundsProxy {
                center,
                half_extents,
            } => Some(PreparedPhysicsBody {
                motion_type: "MO_SYS_DYNAMIC".into(),
                quality_type: "MO_QUAL_MOVING".into(),
                mass: 1.0,
                linear_damping: 0.05,
                angular_damping: 0.05,
                shapes: vec![PreparedPhysicsShape::Box {
                    center: *center,
                    half_extents: *half_extents,
                    rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                }],
                ..default()
            }),
            crate::vsa::PreparedDropCollider::Missing => None,
        };
        let Some(mut body) = body.take() else {
            warn!(
                "drop physics unavailable runtime={}",
                runtime_item.runtime_id
            );
            commands
                .entity(entity)
                .remove::<super::super::world_items::PendingRuntimeCollider>();
            continue;
        };
        body.shapes.retain(PreparedPhysicsShape::supports_dynamic);
        let mut placement = placement.clone();
        placement.physics_classification = PreparedPhysicsClassification::Dynamic;
        let body_id = create_dynamic_body(world, &placement, &body);
        let shapes = body
            .shapes
            .iter()
            .filter_map(|shape| {
                create_prepared_shape(
                    world,
                    body_id,
                    &body,
                    shape,
                    &placement,
                    PreparedShapeOptions {
                        dynamic: true,
                        local_space: true,
                        collision_group: 0,
                    },
                )
                .map(|(shape_id, _)| shape_id)
            })
            .collect::<Vec<_>>();
        if shapes.is_empty() {
            let _ = world.try_destroy_body(body_id);
            warn!(
                "drop physics unavailable runtime={}",
                runtime_item.runtime_id
            );
        } else {
            normalize_dynamic_mass(world, body_id, &body, placement.scale.abs());
            if let Some(restore) = pending.restore {
                apply_dynamic_body_restore(world, body_id, &restore);
            }
            collision.dynamic_bodies.insert(entity, body_id);
            collision.dynamic_entities.insert(body_id, entity);
            collision
                .ledger
                .record_dynamic_body(runtime_item.cell_form_id, body_id);
            for shape_id in shapes {
                collision.surfaces.insert(
                    shape_id,
                    CollisionSurface {
                        material: body.material,
                    },
                );
                collision
                    .ledger
                    .record_body_shape(runtime_item.cell_form_id, shape_id);
            }
            commands.entity(entity).insert(PhysicsCollider);
            info!("drop physics ready runtime={}", runtime_item.runtime_id);
        }
        commands
            .entity(entity)
            .remove::<super::super::world_items::PendingRuntimeCollider>();
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn sync_dynamic_transforms(
    mut collision_world: ResMut<PreparedCollisionWorld>,
    context: NonSend<BoxdddPhysicsContext>,
    mut stats: ResMut<CollisionRuntimeStats>,
    mut roots: Query<&mut Transform, With<PhysicsCollider>>,
    transforms: Query<(Entity, &GlobalTransform)>,
    names: Query<&Name>,
    children: Query<&Children>,
    parents: Query<(Entity, &ChildOf)>,
    mut nodes: Query<(Entity, &mut Transform), Without<PhysicsCollider>>,
) {
    stats.dynamic_transform_updates = 0;
    let Some(world) = context.world() else {
        return;
    };
    let global_affines = transforms
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
    let mut updates = 0;
    if world
        .try_with_body_events_view(|events| {
            for event in events {
                let body = event.body_id();
                let Some(entity) = collision_world.dynamic_entities.get(&body).copied() else {
                    continue;
                };
                let was_sleeping = collision_world.sleeping_dynamic_bodies.contains(&entity);
                if event.fell_asleep() {
                    collision_world.sleeping_dynamic_bodies.insert(entity);
                } else {
                    collision_world.sleeping_dynamic_bodies.remove(&entity);
                }
                // BoxDDD normally emits no repeated events for a settled
                // body, but keep the invariant explicit: a body that was
                // already sleeping must not cause another Bevy transform
                // write until a wake event arrives. The first sleep event is
                // still applied so the final pose is not left one frame stale.
                if was_sleeping && event.fell_asleep() {
                    continue;
                }
                // Articulated ragdolls drive their skeleton nodes below. Moving
                // the placement root with one arbitrary body double-applies the
                // fall and can pull an otherwise-colliding actor through floors.
                let articulated_ragdoll = collision_world.ragdoll_bodies.contains_key(&entity);
                let primary_body =
                    collision_world.dynamic_bodies.get(&entity).copied() == Some(body);
                if !dynamic_body_drives_placement_root(articulated_ragdoll, primary_body) {
                    continue;
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

    // Body events are intentionally sparse for sleeping actors. Read every
    // live ragdoll pose so articulated meshes continue to follow joints even
    // when BoxDDD did not emit a transform event this fixed step.
    let mut desired_worlds = HashMap::new();
    for (body, binding) in collision_world.ragdoll_nodes.iter_mut() {
        let Ok(physics_transform) = world.try_body_transform(*body) else {
            binding.node_entities.clear();
            binding.rest_body = None;
            binding.rest_node_globals.clear();
            binding.rest_node_locals.clear();
            continue;
        };
        if binding.node_entities.is_empty()
            || binding
                .node_entities
                .iter()
                .any(|entity| !global_affines.contains_key(entity))
        {
            binding.node_entities =
                find_actor_node_descendants(binding.root, &binding.node, &children, &names);
            binding.rest_body = None;
            binding.rest_node_globals.clear();
            binding.rest_node_locals.clear();
        }
        if binding.node_entities.is_empty() {
            continue;
        }
        let body_affine = Affine3A::from_rotation_translation(
            from_box_quat(physics_transform.q),
            Vec3::new(
                physics_transform.p.x,
                physics_transform.p.y,
                physics_transform.p.z,
            ),
        );
        let rest_body = *binding.rest_body.get_or_insert(body_affine);
        if binding.rest_node_globals.len() != binding.node_entities.len() {
            binding.rest_node_globals = binding
                .node_entities
                .iter()
                .filter_map(|entity| global_affines.get(entity).copied())
                .collect();
            binding.rest_node_locals = binding
                .node_entities
                .iter()
                .filter_map(|entity| local_snapshots.get(entity).cloned())
                .collect();
        }
        let delta = body_affine * rest_body.inverse();
        for (index, node_entity) in binding.node_entities.iter().enumerate() {
            let Some(rest_node) = binding.rest_node_globals.get(index).copied() else {
                continue;
            };
            desired_worlds.insert(*node_entity, delta * rest_node);
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
        let Ok((_, mut transform)) = nodes.get_mut(*node_entity) else {
            continue;
        };
        *transform = local;
    }
    stats.dynamic_transform_updates = updates;
    stats.sleeping_dynamic_bodies = collision_world.sleeping_dynamic_bodies.len();
    stats.awake_dynamic_bodies = collision_world
        .dynamic_bodies
        .len()
        .saturating_sub(stats.sleeping_dynamic_bodies);
}

pub(crate) fn ragdoll_resolved_world(
    entity: Entity,
    desired_worlds: &HashMap<Entity, Affine3A>,
    parents: &HashMap<Entity, Entity>,
    current_worlds: &HashMap<Entity, Affine3A>,
    current_locals: &HashMap<Entity, Transform>,
) -> Option<Affine3A> {
    let mut cursor = entity;
    let mut local_chain = Vec::new();
    for _ in 0..128 {
        if let Some(world) = desired_worlds.get(&cursor).copied() {
            return Some(
                local_chain
                    .iter()
                    .rev()
                    .fold(world, |parent, local: &Affine3A| parent * *local),
            );
        }
        if let Some(local) = current_locals.get(&cursor) {
            local_chain.push(local.compute_affine());
            if let Some(parent) = parents.get(&cursor).copied() {
                cursor = parent;
                continue;
            }
            return Some(
                local_chain
                    .iter()
                    .rev()
                    .fold(Affine3A::IDENTITY, |parent, local| parent * *local),
            );
        }
        if let Some(world) = current_worlds.get(&cursor).copied() {
            return Some(
                local_chain
                    .iter()
                    .rev()
                    .fold(world, |parent, local| parent * *local),
            );
        }
        return None;
    }
    None
}

pub(crate) fn ragdoll_local_transform(
    parent_world: Affine3A,
    desired_world: Affine3A,
) -> Option<Transform> {
    let local = parent_world.inverse() * desired_world;
    let (scale, rotation, translation) = local.to_scale_rotation_translation();
    (scale.is_finite() && rotation.is_finite() && translation.is_finite()).then_some(Transform {
        translation,
        rotation,
        scale,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn sync_player_proxy(
    physics_disabled: Res<PhysicsDisabled>,
    no_clip: Res<PlayerNoClip>,
    cell_physics: Res<CellPhysicsReadiness>,
    state: Res<CameraModeState>,
    time: Res<Time<Fixed>>,
    mut collision_world: ResMut<PreparedCollisionWorld>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    players: Query<&Transform, With<FpsPlayer>>,
) {
    let Some(world) = context.world_mut() else {
        return;
    };
    let player_transform = (cell_physics.static_collision_ready()
        && !physics_disabled.0
        && !no_clip.0
        && state.mode == CameraMode::Fps)
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
