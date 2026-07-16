//! `bevy_landmass` runtime plugin + `tna` (test nav agent) console command
//! family (issue #112, M4 wave 3). Owns one `Archipelago3d` per active cell
//! (one `Island3dBundle` per prepared nav mesh within it, one
//! `AnimationLink3dBundle` per intra-cell door-link descriptor), built
//! lazily from `PreparedSceneManifest::nav_graph` on the first `tna spawn`,
//! torn down on cell swap. Movement is kinematic for the spike: the
//! landmass-computed desired velocity is applied to the agent's `Transform`
//! each frame and fed back as the agent's own velocity (mirrors
//! `bevy_landmass`'s own `basic.rs` example) -- no `bevy_boxddd` physics, no
//! stepping/slopes; #114 owns grounded movement.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_landmass::coords::ThreeD;
use bevy_landmass::prelude::*;
use bevy_landmass::{
    NavMeshHandle, PauseAgent, PointSampleDistance3d, TargetReachedCondition, UsingAnimationLink,
};
use serde_json::json;

use crate::console::{ConsoleCommandResult, ConsoleError, ConsoleInvocation};
use crate::vsa::PreparedSceneManifest;

use super::super::{interaction, player};
use super::{door_link, landmass_graph};

const AGENT_RADIUS: f32 = 0.35;
const AGENT_HEIGHT: f32 = 1.8;
const AGENT_DESIRED_SPEED: f32 = 2.5;
const AGENT_MAX_SPEED: f32 = 3.5;
const AGENT_TARGET_REACHED_DISTANCE: f32 = 0.5;
/// Fixed kinematic crossing duration for a door-link traversal (spike
/// simplification -- #113 can derive this from the link's real length and
/// the agent's desired speed instead).
const DOOR_TRAVERSAL_SECONDS: f32 = 0.6;

/// Marks the one test agent this console command family drives.
#[derive(Component)]
struct TestNavAgentMarker;

/// Present on the agent entity while it is kinematically crossing a
/// door-link edge (`start` -> `end`), holding `apply_kinematic_velocity`
/// off the transform until the crossing completes.
#[derive(Component)]
struct DoorTraversal {
    start: Vec3,
    end: Vec3,
    elapsed: f32,
}

/// One archipelago + its islands/links for the currently loaded cell,
/// built lazily by `ensure_archipelago` and torn down by
/// `despawn_stale_navmesh_archipelago` on cell swap (mirrors
/// `nav_overlay::despawn_stale_nav_overlay`'s pattern).
#[derive(Resource, Default)]
struct NavArchipelagoState {
    cell_form_id: Option<u32>,
    archipelago: Option<Entity>,
    islands: Vec<Entity>,
    links: Vec<Entity>,
    /// Animation-link entity -> the door FormID it represents, so
    /// `door_link_system` can map a `ReachedAnimationLink3d.link_entity`
    /// back to a door reference to activate.
    link_doors: HashMap<Entity, u32>,
}

#[derive(Resource, Default)]
struct TestNavAgentState {
    entity: Option<Entity>,
    door_link: door_link::DoorLinkState,
    /// Set by `door_link_system` when a link is first reached, consumed by
    /// the same system once the door opens to start the `DoorTraversal`.
    pending_traversal: Option<(Vec3, Vec3)>,
    /// `Time::elapsed_secs()` when the last `tna goto` ran, for the
    /// path-latency log line.
    goto_started_at: Option<f32>,
    latency_logged: bool,
    /// Last `AgentState` `log_agent_state_changes` reported, so the stable
    /// evidence lines fire once per actual change instead of every frame.
    last_logged_state: Option<AgentState>,
}

pub(crate) struct NavBackendPlugin;

impl Plugin for NavBackendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Landmass3dPlugin::default())
            .init_resource::<NavArchipelagoState>()
            .init_resource::<TestNavAgentState>()
            .add_systems(
                Update,
                (
                    despawn_stale_navmesh_archipelago,
                    door_link_system,
                    sync_velocity_from_desired,
                    apply_kinematic_velocity,
                    door_traversal_system,
                    log_agent_state_changes,
                    log_path_latency,
                )
                    .chain(),
            );
    }
}

pub(crate) fn install(app: &mut App) {
    app.add_plugins(NavBackendPlugin);
}

fn no_nav_graph_error() -> ConsoleError {
    ConsoleError::new("no_nav_graph", "no nav graph prepared for this cell")
}

fn teardown_archipelago(world: &mut World) {
    let state = std::mem::take(&mut *world.resource_mut::<NavArchipelagoState>());
    for entity in state
        .links
        .into_iter()
        .chain(state.islands)
        .chain(state.archipelago)
    {
        if let Ok(entity) = world.get_entity_mut(entity) {
            entity.despawn();
        }
    }
}

/// Builds (or reuses, if already current for this cell) the archipelago,
/// islands, and door-link entities for the active cell's prepared nav
/// graph. Lazy: only called from `tna spawn`, never eagerly per cell swap.
fn ensure_archipelago(world: &mut World) -> Result<(), ConsoleError> {
    let (current_cell, path) = {
        let manifest = world
            .get_resource::<PreparedSceneManifest>()
            .ok_or_else(no_nav_graph_error)?;
        let path = super::nav_graph_path(manifest).ok_or_else(no_nav_graph_error)?;
        (manifest.cell.form_id, path)
    };

    let already_current = {
        let state = world.resource::<NavArchipelagoState>();
        state.cell_form_id == Some(current_cell) && state.archipelago.is_some()
    };
    if already_current {
        return Ok(());
    }
    teardown_archipelago(world);

    let graph = super::read_nav_graph(&path).map_err(|error| {
        warn!("nav graph read failed at {}: {error:#}", path.display());
        no_nav_graph_error()
    })?;
    let mesh_inputs = super::mesh_inputs(&graph);

    // `from_agent_radius(0.35)` alone gives a 0.07 m horizontal / 0.35 m
    // below sampling envelope -- far too tight for FO3 data, where the
    // NAVM surface sits below the placed feet position and stairs/slopes
    // put the agent well above the polygon plane (the landmass FAQ's
    // vertical-sampling guidance; confirmed empirically: the default
    // envelope reports `AgentNotOnNavMesh` for an agent standing on the
    // MegatonPlayerHouse mesh). Widen the sample distances to humanoid
    // scale; keep the rest of `from_agent_radius`'s avoidance defaults.
    let mut options = ArchipelagoOptions::from_agent_radius(AGENT_RADIUS);
    options.point_sample_distance = PointSampleDistance3d {
        horizontal_distance: 1.0,
        distance_above: 1.0,
        distance_below: 2.0,
        vertical_preference_ratio: 2.0,
        animation_link_max_vertical_distance: 1.0,
    };
    let archipelago_entity = world.spawn(Archipelago3d::new(options)).id();

    let mut islands = Vec::new();
    for mesh in &mesh_inputs {
        let result = landmass_graph::build_navigation_mesh(mesh);
        for diagnostic in &result.diagnostics {
            warn!(
                "nav landmass conversion mesh {:08x}: {}",
                mesh.form_id, diagnostic.message
            );
        }
        let Some(valid) = result.nav_mesh else {
            continue;
        };
        let handle = world.resource_mut::<Assets<NavMesh3d>>().add(NavMesh3d {
            nav_mesh: Arc::new(valid),
        });
        let island_entity = world
            .spawn(Island3dBundle {
                island: Island,
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
                nav_mesh: NavMeshHandle::<ThreeD>(handle),
            })
            .id();
        islands.push(island_entity);
    }

    if islands.is_empty() {
        if let Ok(entity) = world.get_entity_mut(archipelago_entity) {
            entity.despawn();
        }
        return Err(ConsoleError::new(
            "nav_mesh_invalid",
            "prepared nav graph produced no valid landmass islands",
        ));
    }

    let mut links = Vec::new();
    let mut link_doors = HashMap::new();
    for descriptor in landmass_graph::door_link_descriptors(&mesh_inputs) {
        let start = Vec3::from_array(descriptor.side_a.midpoint);
        let end = Vec3::from_array(descriptor.side_b.midpoint);
        let link_entity = world
            .spawn(AnimationLink3dBundle {
                link: AnimationLink3d {
                    start_edge: (start, start),
                    end_edge: (end, end),
                    kind: 0,
                    cost: 1.0,
                    bidirectional: true,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            })
            .id();
        link_doors.insert(link_entity, descriptor.door_form_id);
        links.push(link_entity);
    }

    *world.resource_mut::<NavArchipelagoState>() = NavArchipelagoState {
        cell_form_id: Some(current_cell),
        archipelago: Some(archipelago_entity),
        islands,
        links,
        link_doors,
    };
    Ok(())
}

fn player_transform_query(world: &mut World) -> Option<Vec3> {
    let mut query = world.query_filtered::<&GlobalTransform, With<player::FpsPlayer>>();
    query.single(world).ok().map(|t| t.translation())
}

fn player_entity_query(world: &mut World) -> Option<Entity> {
    let mut query = world.query_filtered::<Entity, With<player::FpsPlayer>>();
    query.single(world).ok()
}

/// `tna` command dispatcher: `invocation.args[0]` is the subcommand;
/// `tna` with no arguments prints usage rather than erroring.
pub(crate) fn tna_command(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let Some(subcommand) = invocation.args.first() else {
        return Ok(usage_reply());
    };
    let rest = &invocation.args[1..];
    match subcommand.as_str() {
        "spawn" => spawn_agent(world, rest),
        "goto" => goto_agent(world, rest),
        "status" => agent_status(world, rest),
        "despawn" => despawn_agent(world, rest),
        other => Err(ConsoleError::new(
            "unknown_subcommand",
            format!("unknown tna subcommand '{other}'; expected spawn, goto, status, or despawn"),
        )),
    }
}

fn usage_reply() -> ConsoleCommandResult {
    let usage = "usage: tna spawn|goto <x> <y> <z>|goto player|status|despawn";
    ConsoleCommandResult::new(json!({ "usage": usage }), vec![usage.to_string()])
}

fn spawn_agent(world: &mut World, rest: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    if !rest.is_empty() {
        return Err(ConsoleError::new(
            "bad_arity",
            "tna spawn does not accept arguments",
        ));
    }
    ensure_archipelago(world)?;
    if world.resource::<TestNavAgentState>().entity.is_some() {
        return Err(ConsoleError::new(
            "already_spawned",
            "a test nav agent is already spawned; use tna despawn first",
        ));
    }
    let position = player_transform_query(world)
        .ok_or_else(|| ConsoleError::new("player_unavailable", "the FPS player does not exist"))?;
    let archipelago_entity = world
        .resource::<NavArchipelagoState>()
        .archipelago
        .expect("ensure_archipelago populated the archipelago");

    let cylinder_height = (AGENT_HEIGHT - 2.0 * AGENT_RADIUS).max(0.0);
    let mesh = world
        .resource_mut::<Assets<Mesh>>()
        .add(Capsule3d::new(AGENT_RADIUS, cylinder_height));
    let material = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.85, 0.9),
            ..default()
        });

    let agent_entity = world
        .spawn((
            TestNavAgentMarker,
            Transform::from_translation(position),
            Visibility::Inherited,
            Agent3dBundle {
                agent: default(),
                settings: AgentSettings {
                    radius: AGENT_RADIUS,
                    desired_speed: AGENT_DESIRED_SPEED,
                    max_speed: AGENT_MAX_SPEED,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            },
            TargetReachedCondition::Distance(Some(AGENT_TARGET_REACHED_DISTANCE)),
        ))
        .id();
    let visual = world
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(0.0, AGENT_HEIGHT / 2.0, 0.0),
        ))
        .id();
    world.entity_mut(agent_entity).add_child(visual);

    *world.resource_mut::<TestNavAgentState>() = TestNavAgentState {
        entity: Some(agent_entity),
        ..default()
    };
    info!(
        "nav agent spawn position=({:.2},{:.2},{:.2})",
        position.x, position.y, position.z
    );
    Ok(ConsoleCommandResult::new(
        json!({ "position": [position.x, position.y, position.z] }),
        vec![format!(
            "nav agent spawned at ({:.2}, {:.2}, {:.2})",
            position.x, position.y, position.z
        )],
    ))
}

fn goto_agent(world: &mut World, rest: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    let Some(agent_entity) = world.resource::<TestNavAgentState>().entity else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned; use tna spawn first",
        ));
    };
    let (target, description) = match rest {
        [value] if value == "player" => {
            let player_entity = player_entity_query(world).ok_or_else(|| {
                ConsoleError::new("player_unavailable", "the FPS player does not exist")
            })?;
            (AgentTarget3d::Entity(player_entity), "player".to_string())
        }
        [x, y, z] => {
            let parse = |value: &String| {
                value.parse::<f32>().map_err(|_| {
                    ConsoleError::new("bad_type", "tna goto coordinates must be finite numbers")
                })
            };
            let point = Vec3::new(parse(x)?, parse(y)?, parse(z)?);
            (
                AgentTarget3d::Point(point),
                format!("({:.2}, {:.2}, {:.2})", point.x, point.y, point.z),
            )
        }
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna goto requires <x> <y> <z> or the literal 'player'",
            ));
        }
    };
    world.entity_mut(agent_entity).insert(target);
    let elapsed = world.resource::<Time>().elapsed_secs();
    {
        let mut state = world.resource_mut::<TestNavAgentState>();
        state.goto_started_at = Some(elapsed);
        state.latency_logged = false;
    }
    Ok(ConsoleCommandResult::new(
        json!({ "target": description }),
        vec![format!("nav agent target set to {description}")],
    ))
}

fn agent_status(world: &mut World, rest: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    if !rest.is_empty() {
        return Err(ConsoleError::new(
            "bad_arity",
            "tna status does not accept arguments",
        ));
    }
    let Some(agent_entity) = world.resource::<TestNavAgentState>().entity else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned; use tna spawn first",
        ));
    };
    let position = world
        .get::<GlobalTransform>(agent_entity)
        .map(|t| t.translation())
        .unwrap_or_default();
    let landmass_state = world
        .get::<AgentState>(agent_entity)
        .copied()
        .unwrap_or_default();
    let door_link_state = world.resource::<TestNavAgentState>().door_link;
    let status = resolve_status(landmass_state, door_link_state);
    let target_desc = world
        .get::<AgentTarget3d>(agent_entity)
        .map(describe_target)
        .unwrap_or_else(|| "none".to_string());
    let line = format!(
        "nav agent status={} position=({:.2},{:.2},{:.2}) target={}",
        status.as_str(),
        position.x,
        position.y,
        position.z,
        target_desc
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "status": status.as_str(),
            "position": [position.x, position.y, position.z],
            "target": target_desc,
        }),
        vec![line],
    ))
}

fn resolve_status(
    landmass_state: AgentState,
    door_link_state: door_link::DoorLinkState,
) -> landmass_graph::NavAgentStatus {
    if door_link::is_paused(door_link_state) || door_link::is_failed(door_link_state) {
        return landmass_graph::NavAgentStatus::Paused;
    }
    landmass_graph::map_agent_state(landmass_state)
}

fn describe_target(target: &AgentTarget3d) -> String {
    match target {
        AgentTarget3d::None => "none".to_string(),
        AgentTarget3d::Point(point) => format!("({:.2}, {:.2}, {:.2})", point.x, point.y, point.z),
        AgentTarget3d::Entity(entity) => format!("entity:{entity:?}"),
    }
}

fn despawn_agent(world: &mut World, rest: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    if !rest.is_empty() {
        return Err(ConsoleError::new(
            "bad_arity",
            "tna despawn does not accept arguments",
        ));
    }
    let Some(agent_entity) = world.resource::<TestNavAgentState>().entity else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned; use tna spawn first",
        ));
    };
    if let Ok(entity) = world.get_entity_mut(agent_entity) {
        entity.despawn();
    }
    *world.resource_mut::<TestNavAgentState>() = TestNavAgentState::default();
    Ok(ConsoleCommandResult::new(
        json!({ "despawned": true }),
        vec!["nav agent despawned".to_string()],
    ))
}

// ---------------------------------------------------------------------
// Runtime systems
// ---------------------------------------------------------------------

fn sync_velocity_from_desired(
    mut agents: Query<(&mut Velocity3d, &AgentDesiredVelocity3d), With<TestNavAgentMarker>>,
) {
    for (mut velocity, desired) in &mut agents {
        velocity.velocity = desired.velocity();
    }
}

type KinematicAgentQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Transform, &'static Velocity3d),
    (With<TestNavAgentMarker>, Without<DoorTraversal>),
>;

/// Sampling envelope for the per-frame ground snap in
/// [`apply_kinematic_velocity`]. Matches the archipelago's own
/// `point_sample_distance` (see `ensure_archipelago`): the snap must never
/// find ground the pathing layer itself would not, or the agent could stand
/// somewhere landmass considers off-mesh.
const GROUND_SNAP_SAMPLE_DISTANCE: PointSampleDistance3d = PointSampleDistance3d {
    horizontal_distance: 1.0,
    distance_above: 1.0,
    distance_below: 2.0,
    vertical_preference_ratio: 2.0,
    animation_link_max_vertical_distance: 1.0,
};

/// Applies the landmass desired velocity kinematically, then snaps the
/// agent's y to the nav-mesh surface under its new x/z
/// (`Archipelago3d::sample_point`). Without the snap a sloped/stair path
/// (FranklinMetro02's descending floor, mesh y spanning ~94-105 m) leaves y
/// frozen at spawn height until the agent exits the vertical sampling
/// envelope and flips to `AgentNotOnNavMesh` mid-route. A miss (agent
/// momentarily outside the envelope, e.g. pushed off-mesh by avoidance)
/// leaves y unchanged rather than teleporting; landmass then reports the
/// off-mesh state through the normal status path. Door-link crossings
/// (`DoorTraversal`) are excluded -- the lerp owns the transform there.
fn apply_kinematic_velocity(
    time: Res<Time>,
    state: Res<NavArchipelagoState>,
    archipelagos: Query<&Archipelago3d>,
    mut agents: KinematicAgentQuery<'_, '_>,
) {
    let archipelago = state
        .archipelago
        .and_then(|entity| archipelagos.get(entity).ok());
    for (mut transform, velocity) in &mut agents {
        transform.translation += velocity.velocity * time.delta_secs();
        if let Some(archipelago) = archipelago
            && let Ok(sampled) =
                archipelago.sample_point(transform.translation, &GROUND_SNAP_SAMPLE_DISTANCE)
        {
            transform.translation.y = sampled.point().y;
        }
    }
}

fn door_traversal_system(
    time: Res<Time>,
    mut agents: Query<(Entity, &mut Transform, &mut DoorTraversal), With<TestNavAgentMarker>>,
    mut state: ResMut<TestNavAgentState>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut traversal) in &mut agents {
        traversal.elapsed += time.delta_secs();
        let t = (traversal.elapsed / DOOR_TRAVERSAL_SECONDS).clamp(0.0, 1.0);
        transform.translation = traversal.start.lerp(traversal.end, t);
        if t >= 1.0 {
            commands
                .entity(entity)
                .remove::<DoorTraversal>()
                .remove::<UsingAnimationLink>();
            state.door_link =
                door_link::transition(state.door_link, door_link::DoorLinkEvent::TraversalComplete);
        }
    }
}

/// Drives the door-link lifecycle: detects the agent reaching an off-mesh
/// link, requests the door open through the same boundary the `activate`
/// console command uses (`interaction::scripted_door_open`), polls
/// `InteractionState.open`, and starts the kinematic crossing once the door
/// is open. An exclusive (`&mut World`) system since it needs to both query
/// components and call into `interaction`'s `&mut World`-based scripted
/// door boundary in the same step.
fn door_link_system(world: &mut World) {
    let Some(agent_entity) = world.resource::<TestNavAgentState>().entity else {
        return;
    };
    if world.get_entity(agent_entity).is_err() {
        return;
    }
    let current_state = world.resource::<TestNavAgentState>().door_link;

    match current_state {
        door_link::DoorLinkState::Idle | door_link::DoorLinkState::Failed { .. } => {
            let Some(reached) = world.get::<ReachedAnimationLink3d>(agent_entity) else {
                return;
            };
            let link_entity = reached.link_entity;
            let start_point = reached.start_point;
            let end_point = reached.end_point;
            let Some(&door_form_id) = world
                .resource::<NavArchipelagoState>()
                .link_doors
                .get(&link_entity)
            else {
                return;
            };
            let new_state = door_link::transition(
                current_state,
                door_link::DoorLinkEvent::LinkReached { door_form_id },
            );
            world.entity_mut(agent_entity).insert(PauseAgent);
            match crate::console::resolve_reference(world, &format!("{door_form_id:08x}")) {
                Ok(door_entity) => {
                    interaction::scripted_door_open(world, door_entity);
                }
                Err(_) => {
                    warn!("nav agent door {door_form_id:08x}: reference not resolvable");
                }
            }
            info!("nav agent door wait {door_form_id:08x}");
            let mut state = world.resource_mut::<TestNavAgentState>();
            state.door_link = new_state;
            state.pending_traversal = Some((start_point, end_point));
        }
        door_link::DoorLinkState::Paused { door_form_id, .. } => {
            let door_open =
                crate::console::resolve_reference(world, &format!("{door_form_id:08x}"))
                    .ok()
                    .is_some_and(|door_entity| {
                        world
                            .resource::<interaction::InteractionState>()
                            .open
                            .contains(&door_entity)
                    });
            let new_state =
                door_link::transition(current_state, door_link::DoorLinkEvent::Tick { door_open });
            if door_link::is_traversing(new_state) {
                world.entity_mut(agent_entity).remove::<PauseAgent>();
                let pending = world
                    .resource_mut::<TestNavAgentState>()
                    .pending_traversal
                    .take();
                if let Some((start, end)) = pending {
                    world.entity_mut(agent_entity).insert((
                        UsingAnimationLink,
                        DoorTraversal {
                            start,
                            end,
                            elapsed: 0.0,
                        },
                    ));
                }
                info!("nav agent door resume {door_form_id:08x}");
            } else if door_link::is_failed(new_state) {
                warn!(
                    "nav agent door {door_form_id:08x}: gave up waiting for it to open; agent stopped at the link"
                );
                info!("nav agent unreachable");
            }
            world.resource_mut::<TestNavAgentState>().door_link = new_state;
        }
        door_link::DoorLinkState::Traversing { .. } => {
            // `door_traversal_system` owns the crossing and emits
            // `TraversalComplete` once it finishes.
        }
    }
}

/// Logs the stable evidence lines exactly once per actual state change.
/// `bevy_landmass`'s `sync_agent_state` rewrites `AgentState` every frame
/// (Bevy change detection triggers on the write, not the value), so a
/// `Changed<AgentState>` filter would re-log every frame; the previous value
/// is tracked in `TestNavAgentState` instead.
fn log_agent_state_changes(
    agents: Query<&AgentState, With<TestNavAgentMarker>>,
    mut state: ResMut<TestNavAgentState>,
) {
    let Ok(agent_state) = agents.single() else {
        return;
    };
    if state.last_logged_state == Some(*agent_state) {
        return;
    }
    state.last_logged_state = Some(*agent_state);
    match agent_state {
        AgentState::ReachedTarget => info!("nav agent reached"),
        AgentState::AgentNotOnNavMesh | AgentState::TargetNotOnNavMesh | AgentState::NoPath => {
            info!("nav agent unreachable state={agent_state:?}");
        }
        _ => {}
    }
}

fn log_path_latency(
    time: Res<Time>,
    agents: Query<&AgentState, With<TestNavAgentMarker>>,
    mut state: ResMut<TestNavAgentState>,
) {
    if state.latency_logged {
        return;
    }
    let Some(started_at) = state.goto_started_at else {
        return;
    };
    let Ok(agent_state) = agents.single() else {
        return;
    };
    if matches!(agent_state, AgentState::Moving | AgentState::ReachedTarget) {
        let latency_ms = (time.elapsed_secs() - started_at) * 1000.0;
        info!("nav agent path latency_ms={latency_ms:.1}");
        state.latency_logged = true;
    }
}

/// Mirrors `nav_overlay::despawn_stale_nav_overlay`'s pattern: the moment
/// the active cell no longer matches the archipelago's cell, tear it (and
/// the test agent) down. `PreparedSceneManifest` is optional so this system
/// never panics in a console-harness test world that never inserted one.
fn despawn_stale_navmesh_archipelago(world: &mut World) {
    let Some(current_cell) = world
        .get_resource::<PreparedSceneManifest>()
        .map(|manifest| manifest.cell.form_id)
    else {
        return;
    };
    let stale = world
        .resource::<NavArchipelagoState>()
        .cell_form_id
        .is_some_and(|cell| cell != current_cell);
    if !stale {
        return;
    }
    teardown_archipelago(world);
    if let Some(agent_entity) = world.resource::<TestNavAgentState>().entity
        && let Ok(entity) = world.get_entity_mut(agent_entity)
    {
        entity.despawn();
    }
    *world.resource_mut::<TestNavAgentState>() = TestNavAgentState::default();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::console::ConsoleSessionId;

    fn invocation(args: &[&str]) -> ConsoleInvocation {
        ConsoleInvocation {
            request_id: 1,
            frame: 1,
            session: ConsoleSessionId::new("test"),
            command: "tna".into(),
            args: args.iter().map(|arg| arg.to_string()).collect(),
            target: None,
        }
    }

    fn harness_world() -> World {
        let mut world = World::new();
        world.init_resource::<NavArchipelagoState>();
        world.init_resource::<TestNavAgentState>();
        world
    }

    /// Real-data acceptance defect on FranklinMetro02 (0001a273): a sloped
    /// route froze the agent's y at spawn height until it left the vertical
    /// sampling envelope and flipped to `AgentNotOnNavMesh`. This drives the
    /// real `Landmass3dPlugin` island sync (one flat square island at
    /// y = 2.0), places the agent 0.5 m above the surface, runs the kinematic
    /// system once, and asserts the ground snap pulled y onto the mesh.
    #[test]
    fn kinematic_velocity_snaps_agent_y_to_the_sampled_navmesh_surface() {
        use bevy::app::App;
        use bevy::ecs::system::RunSystemOnce;
        use std::sync::Arc;

        let mut app = App::new();
        app.add_plugins((
            bevy::MinimalPlugins,
            bevy::asset::AssetPlugin::default(),
            Landmass3dPlugin::default(),
        ));
        app.init_resource::<NavArchipelagoState>();
        app.init_resource::<TestNavAgentState>();

        // Flat square at y = 2.0 through the same pure conversion the
        // runtime uses.
        let mesh = landmass_graph::MeshInput {
            form_id: 0x10,
            vertices: vec![
                [0.0, 2.0, 0.0],
                [4.0, 2.0, 0.0],
                [0.0, 2.0, 4.0],
                [4.0, 2.0, 4.0],
            ],
            polygons: vec![
                landmass_graph::PolygonInput {
                    index: 0,
                    vertex_indices: [0, 1, 2],
                    is_water: false,
                },
                landmass_graph::PolygonInput {
                    index: 1,
                    vertex_indices: [1, 3, 2],
                    is_water: false,
                },
            ],
            doors: Vec::new(),
        };
        let valid = landmass_graph::build_navigation_mesh(&mesh)
            .nav_mesh
            .expect("synthetic square validates");
        let handle = app
            .world_mut()
            .resource_mut::<Assets<NavMesh3d>>()
            .add(NavMesh3d {
                nav_mesh: Arc::new(valid),
            });

        let mut options = ArchipelagoOptions::from_agent_radius(AGENT_RADIUS);
        options.point_sample_distance = GROUND_SNAP_SAMPLE_DISTANCE;
        let archipelago = app.world_mut().spawn(Archipelago3d::new(options)).id();
        app.world_mut().spawn(Island3dBundle {
            island: Island,
            archipelago_ref: ArchipelagoRef3d::new(archipelago),
            nav_mesh: NavMeshHandle::<ThreeD>(handle),
        });
        app.world_mut()
            .resource_mut::<NavArchipelagoState>()
            .archipelago = Some(archipelago);

        // Run the plugin's own sync systems (FixedPreUpdate) so the island
        // lands in the archipelago.
        app.world_mut().run_schedule(bevy::app::FixedPreUpdate);

        let agent = app
            .world_mut()
            .spawn((
                TestNavAgentMarker,
                Transform::from_xyz(2.0, 2.5, 2.0),
                Velocity3d::default(),
            ))
            .id();

        app.world_mut()
            .run_system_once(apply_kinematic_velocity)
            .expect("kinematic system runs");

        let y = app
            .world()
            .get::<Transform>(agent)
            .expect("agent has a transform")
            .translation
            .y;
        assert!(
            (y - 2.0).abs() < 1e-4,
            "agent y should snap to the mesh surface (2.0), got {y}"
        );
    }

    #[test]
    fn no_args_prints_usage_without_erroring() {
        let mut world = harness_world();
        let result = tna_command(&mut world, &invocation(&[])).expect("usage is not an error");
        assert_eq!(result.log.len(), 1);
        assert!(result.log[0].starts_with("usage:"));
    }

    #[test]
    fn unknown_subcommand_is_an_error() {
        let mut world = harness_world();
        let error = tna_command(&mut world, &invocation(&["dance"])).unwrap_err();
        assert_eq!(error.code, "unknown_subcommand");
    }

    #[test]
    fn spawn_without_a_nav_graph_reuses_the_no_nav_graph_wording() {
        let mut world = harness_world();
        let error = tna_command(&mut world, &invocation(&["spawn"])).unwrap_err();
        assert_eq!(error.code, "no_nav_graph");
        assert_eq!(error.message, "no nav graph prepared for this cell");
    }

    #[test]
    fn goto_without_a_spawned_agent_is_an_error() {
        let mut world = harness_world();
        let error = tna_command(&mut world, &invocation(&["goto", "1", "2", "3"])).unwrap_err();
        assert_eq!(error.code, "no_agent");
    }

    #[test]
    fn goto_bad_arity_is_rejected() {
        let mut world = harness_world();
        world.resource_mut::<TestNavAgentState>().entity = Some(Entity::PLACEHOLDER);
        let error = tna_command(&mut world, &invocation(&["goto", "1", "2"])).unwrap_err();
        assert_eq!(error.code, "bad_arity");
    }

    #[test]
    fn status_without_a_spawned_agent_is_an_error() {
        let mut world = harness_world();
        let error = tna_command(&mut world, &invocation(&["status"])).unwrap_err();
        assert_eq!(error.code, "no_agent");
    }

    #[test]
    fn despawn_without_a_spawned_agent_is_an_error() {
        let mut world = harness_world();
        let error = tna_command(&mut world, &invocation(&["despawn"])).unwrap_err();
        assert_eq!(error.code, "no_agent");
    }

    #[test]
    fn despawn_round_trip_clears_state() {
        let mut world = harness_world();
        let entity = world.spawn(TestNavAgentMarker).id();
        world.resource_mut::<TestNavAgentState>().entity = Some(entity);
        let result = tna_command(&mut world, &invocation(&["despawn"])).expect("despawn succeeds");
        assert_eq!(result.log, ["nav agent despawned"]);
        assert!(world.resource::<TestNavAgentState>().entity.is_none());
        assert!(world.get_entity(entity).is_err());
    }

    #[test]
    fn archipelago_teardown_on_cell_swap_clears_agent_too() {
        let mut world = harness_world();
        let archipelago = world.spawn_empty().id();
        let island = world.spawn_empty().id();
        world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xC0DE);
        world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago);
        world.resource_mut::<NavArchipelagoState>().islands = vec![island];
        let agent = world.spawn(TestNavAgentMarker).id();
        world.resource_mut::<TestNavAgentState>().entity = Some(agent);

        world.insert_resource(minimal_manifest(0xBEEF));
        despawn_stale_navmesh_archipelago(&mut world);

        assert!(world.get_entity(archipelago).is_err());
        assert!(world.get_entity(island).is_err());
        assert!(world.get_entity(agent).is_err());
        assert!(
            world
                .resource::<NavArchipelagoState>()
                .cell_form_id
                .is_none()
        );
        assert!(world.resource::<TestNavAgentState>().entity.is_none());
    }

    #[test]
    fn resolve_status_prefers_door_link_pause_over_landmass_state() {
        let paused = door_link::DoorLinkState::Paused {
            door_form_id: 0x99,
            waited_ticks: 1,
        };
        assert_eq!(
            resolve_status(AgentState::Moving, paused),
            landmass_graph::NavAgentStatus::Paused
        );
        assert_eq!(
            resolve_status(AgentState::Idle, door_link::DoorLinkState::Idle),
            landmass_graph::NavAgentStatus::Idle
        );
    }

    fn minimal_manifest(cell_form_id: u32) -> PreparedSceneManifest {
        PreparedSceneManifest {
            schema_version: 16,
            prepare_revision: None,
            converter_revision: None,
            physics_schema_version: None,
            asset_root: ".".into(),
            source_plugin: "Fallout3.esm".into(),
            source_fingerprint: "content-hash".into(),
            item_catalog_path: None,
            item_catalog_revision: None,
            item_catalog_hash: None,
            recipe_catalog_path: None,
            recipe_catalog_revision: None,
            recipe_catalog_hash: None,
            actor_catalog_path: None,
            actor_catalog_revision: None,
            actor_catalog_hash: None,
            source_plugins: Vec::new(),
            visual_issues: Vec::new(),
            cell: crate::vsa::CellInfo {
                form_id: cell_form_id,
                editor_id: None,
                name: None,
                interior: true,
                ambient_rgba: [0.0; 4],
                directional_rgba: [0.0; 4],
                image_space_form_id: None,
                image_space: None,
                lighting_template_form_id: None,
                lighting_template_flags: 0,
                lighting_template: None,
                raw_lighting: None,
                effective_lighting: None,
                water_form_id: None,
                water_height: None,
                grid: None,
                worldspace_form_id: None,
            },
            placements: Vec::new(),
            lights: Vec::new(),
            diagnostics: Vec::new(),
            navmeshes: Vec::new(),
            nav_graph: None,
            cell_audio: Default::default(),
            audio_clips: Vec::new(),
            footstep_sets: Vec::new(),
            hard_landing_clips: Vec::new(),
            bake: None,
            static_point_shadows: None,
            mutability_summary: Default::default(),
            leveled_lists: Default::default(),
        }
    }
}
