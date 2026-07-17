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
use super::{door_link, landmass_graph, ledger_policy, repath};

const AGENT_RADIUS: f32 = 0.35;
const AGENT_HEIGHT: f32 = 1.8;
const AGENT_DESIRED_SPEED: f32 = 2.5;
const AGENT_MAX_SPEED: f32 = 3.5;
const AGENT_TARGET_REACHED_DISTANCE: f32 = 0.5;
/// Fixed kinematic crossing duration for a door-link traversal (spike
/// simplification -- #113 can derive this from the link's real length and
/// the agent's desired speed instead).
const DOOR_TRAVERSAL_SECONDS: f32 = 0.6;
/// How close (metres) the agent must get to a travel door's triangle
/// midpoint before the door lifecycle starts (issue #113 feature 3).
/// Slightly wider than `AGENT_TARGET_REACHED_DISTANCE` so landmass's own
/// target-reached stop always lands inside it.
const TRAVEL_ARRIVAL_DISTANCE: f32 = 0.75;

/// Synthetic ledger identity for the one test nav agent this console
/// command family drives (issue #134). The ledger/eligibility policy
/// (`ledger_policy`) is written generically against an `agent_id` (a real
/// multi-agent future would mint one per actor), but this spike only ever
/// has the one -- wiring always passes this constant. Formatted like a
/// FormID (`{:08x}`) in tracing lines, consistent with every other
/// identifier this module logs.
const TEST_AGENT_ID: u32 = 1;

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

/// What an off-mesh animation-link entity represents (issue #113): a
/// same-cell cross-mesh merge seam (always open, crossed without any door
/// interaction) or an intra-cell two-sided door link (wave 3's pause ->
/// open -> traverse lifecycle).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LinkKind {
    Merge,
    Door { form_id: u32 },
}

/// A two-sided intra-cell door link currently excluded from route planning
/// because its door is locked (issue #113 feature 3: "blocked until
/// usable"). The geometry is retained so `door_availability_system` can
/// spawn the real animation link the moment the door becomes usable.
#[derive(Debug, Clone, Copy)]
struct BlockedDoorLink {
    door_form_id: u32,
    start: Vec3,
    end: Vec3,
}

/// A travel door reachable from this cell's nav mesh (issue #113 feature
/// 3): its single-sided triangle midpoint (the routing target), the door
/// placement's own position (the traversal end point -- the agent walks
/// *to* the door, never through into the unloaded destination cell), and
/// the destination cell the existing world-transition metadata (#51/#52)
/// resolves it to.
#[derive(Debug, Clone, Copy)]
struct TravelDoorLink {
    triangle_midpoint: Vec3,
    door_position: Vec3,
    destination_cell_form_id: u32,
    /// The door reference FormID in the destination cell this travel door
    /// pairs with (issue #134): the ledger's `DoorMarker` spawn kind
    /// resolves the agent's restore position from this door's own placed
    /// position once the destination cell is active.
    destination_door_form_id: u32,
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
    /// Animation-link entity -> what it represents, so `door_link_system`
    /// can map a `ReachedAnimationLink3d.link_entity` back to either a door
    /// reference to activate or a merge seam to cross directly.
    link_kinds: HashMap<Entity, LinkKind>,
    /// Two-sided door links currently excluded as blocked (locked door).
    blocked_door_links: Vec<BlockedDoorLink>,
    /// Door reference FormID -> terminal travel-link data.
    travel_doors: HashMap<u32, TravelDoorLink>,
    /// Last observed per-door usability (open, or not locked), for
    /// `door_availability_system`'s change detection -- exactly one repath
    /// per actual flip.
    door_usable: HashMap<u32, bool>,
    /// Doors' prepared lock/key data + placement entity resolution inputs,
    /// captured from the manifest at build time so the availability poll
    /// does not re-borrow the manifest every frame.
    door_lock_info: HashMap<u32, DoorLockInfo>,
}

#[derive(Debug, Clone, Copy)]
struct DoorLockInfo {
    lock_level: Option<i8>,
    key_form_id: Option<u32>,
}

#[derive(Resource, Default)]
struct TestNavAgentState {
    entity: Option<Entity>,
    door_link: door_link::DoorLinkState,
    /// Set by `door_link_system` when a link is first reached, consumed by
    /// the same system once the door opens to start the `DoorTraversal`.
    pending_traversal: Option<(Vec3, Vec3)>,
    /// The link the agent is currently interacting with (for `tna status`'s
    /// `link=` report and for `door_traversal_system` to know whether a
    /// finished crossing should drive the door state machine).
    active_link: Option<LinkKind>,
    /// A pending travel-door route (issue #113 feature 3): the agent is
    /// heading to this door's triangle; arrival starts the door lifecycle
    /// with a `Travel` destination. Consumed by #134's `tna travel`.
    travel_intent: Option<u32>,
    /// `Time::elapsed_secs()` when the last `tna goto` ran, for the
    /// path-latency log line.
    goto_started_at: Option<f32>,
    latency_logged: bool,
    /// Last `AgentState` `log_agent_state_changes` reported, so the stable
    /// evidence lines fire once per actual change instead of every frame.
    last_logged_state: Option<AgentState>,
}

/// Intercell nav-agent ledger (issue #134): survives cell-swap teardown
/// (an ordinary Bevy `Resource`, untouched by `teardown_archipelago`) so an
/// agent handed off through a travel door, or frozen in place by a
/// player-initiated swap, can be restored once its cell is active again.
#[derive(Resource, Default)]
struct NavAgentLedger(ledger_policy::Ledger);

/// The origin door reference the player just used to trigger a cell swap
/// (issue #134), noted by `note_player_swap_door` and consumed exactly
/// once by `despawn_stale_navmesh_archipelago` the next time it detects
/// the resulting stale archipelago. `None` when the swap-triggering cause
/// carried no door (there is currently no such path at runtime, but the
/// consumer treats an absent note as "no eligibility information" rather
/// than assuming a door, so any future non-door cell change still freezes
/// a live agent instead of losing it).
#[derive(Resource, Default)]
struct PendingPlayerSwapDoor(Option<u32>);

pub(crate) struct NavBackendPlugin;

impl Plugin for NavBackendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Landmass3dPlugin::default())
            .init_resource::<NavArchipelagoState>()
            .init_resource::<TestNavAgentState>()
            .init_resource::<NavAgentLedger>()
            .init_resource::<PendingPlayerSwapDoor>()
            .add_systems(
                Update,
                (
                    despawn_stale_navmesh_archipelago,
                    restore_ledgered_agents_system,
                    door_availability_system,
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

/// Called from `world::swap::activate_resident_cell` (issue #134) once a
/// player-initiated cell swap is definitely proceeding: records which
/// origin door reference the player used so the next
/// `despawn_stale_navmesh_archipelago` pass (whichever frame it detects
/// the resulting stale archipelago) can decide follow-through vs. freeze
/// for any live nav agent still in the departing cell.
pub(crate) fn note_player_swap_door(world: &mut World, door_form_id: u32) {
    world.resource_mut::<PendingPlayerSwapDoor>().0 = Some(door_form_id);
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
    let (current_cell, path, travel_destinations, door_lock_info, door_positions) = {
        let manifest = world
            .get_resource::<PreparedSceneManifest>()
            .ok_or_else(no_nav_graph_error)?;
        let path = super::nav_graph_path(manifest).ok_or_else(no_nav_graph_error)?;
        let travel_destinations = super::travel_door_destinations(manifest);
        let mut door_lock_info = HashMap::new();
        let mut door_positions = HashMap::new();
        for placement in &manifest.placements {
            if let crate::vsa::PreparedSemantic::Door(door) = &placement.semantic {
                door_lock_info.insert(
                    placement.reference_form_id,
                    DoorLockInfo {
                        lock_level: door.lock_level,
                        key_form_id: door.key_form_id,
                    },
                );
                door_positions.insert(
                    placement.reference_form_id,
                    Vec3::from_array(placement.translation),
                );
            }
        }
        (
            manifest.cell.form_id,
            path,
            travel_destinations,
            door_lock_info,
            door_positions,
        )
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
    let merge_inputs = super::merge_inputs(&graph);

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
    let mut link_kinds = HashMap::new();
    let mut blocked_door_links = Vec::new();
    let mut door_usable = HashMap::new();

    // Same-cell cross-mesh merge links (issue #113 feature 2). Real FO3
    // meshes never share seam vertex positions, so landmass's native island
    // boundary linking cannot connect them (it needs coincident boundary
    // vertices); generated walk-through animation links across the matched
    // boundary edges are the path real data takes.
    for descriptor in landmass_graph::merge_link_descriptors(&mesh_inputs, &merge_inputs) {
        let start = Vec3::from_array(descriptor.side_a.midpoint);
        let end = Vec3::from_array(descriptor.side_b.midpoint);
        for link_entity in spawn_link_pair(world, archipelago_entity, start, end) {
            link_kinds.insert(link_entity, LinkKind::Merge);
            links.push(link_entity);
        }
    }

    // Two-sided intra-cell door links (wave 3). A locked door is excluded
    // from route planning as blocked until usable (issue #113 feature 3);
    // `door_availability_system` spawns the link when the door becomes
    // usable and triggers a repath.
    for descriptor in landmass_graph::door_link_descriptors(&mesh_inputs) {
        let start = Vec3::from_array(descriptor.side_a.midpoint);
        let end = Vec3::from_array(descriptor.side_b.midpoint);
        let usable = door_usable_now(world, descriptor.door_form_id, &door_lock_info);
        door_usable.insert(descriptor.door_form_id, usable);
        if usable {
            for link_entity in spawn_link_pair(world, archipelago_entity, start, end) {
                link_kinds.insert(
                    link_entity,
                    LinkKind::Door {
                        form_id: descriptor.door_form_id,
                    },
                );
                links.push(link_entity);
            }
        } else {
            info!(
                "nav agent door link {:08x} blocked: locked",
                descriptor.door_form_id
            );
            blocked_door_links.push(BlockedDoorLink {
                door_form_id: descriptor.door_form_id,
                start,
                end,
            });
        }
    }

    // Single-sided door triangles whose reference resolves to a travel door
    // (issue #113 feature 3): terminal travel links. No landmass animation
    // link is spawned -- the far side lives in another cell's NAVM, so there
    // is nothing on-mesh to link to; the agent routes *to* the triangle and
    // the door lifecycle runs there.
    let mut travel_doors = HashMap::new();
    for door in landmass_graph::single_sided_doors(&mesh_inputs) {
        let Some(&destination) = travel_destinations.get(&door.door_form_id) else {
            continue;
        };
        let triangle_midpoint = Vec3::from_array(door.side.midpoint);
        let door_position = door_positions
            .get(&door.door_form_id)
            .copied()
            .unwrap_or(triangle_midpoint);
        door_usable.insert(
            door.door_form_id,
            door_usable_now(world, door.door_form_id, &door_lock_info),
        );
        info!(
            "nav agent travel door {:08x} -> cell {:08x}",
            door.door_form_id, destination.cell_form_id
        );
        travel_doors.insert(
            door.door_form_id,
            TravelDoorLink {
                triangle_midpoint,
                door_position,
                destination_cell_form_id: destination.cell_form_id,
                destination_door_form_id: destination.door_reference_form_id,
            },
        );
    }

    *world.resource_mut::<NavArchipelagoState>() = NavArchipelagoState {
        cell_form_id: Some(current_cell),
        archipelago: Some(archipelago_entity),
        islands,
        links,
        link_kinds,
        blocked_door_links,
        travel_doors,
        door_usable,
        door_lock_info,
    };
    Ok(())
}

/// Spawns one logical off-mesh link as *two* unidirectional
/// `AnimationLink3d`s (start -> end and end -> start) rather than one
/// `bidirectional: true` link: landmass 0.9.1's bidirectional path
/// (`nav_data.rs`'s reverse `OffMeshLink` insert) indexes the *start*
/// island's polygon array with the *end* portal's polygon index when
/// computing `destination_type_index`, which panics ("index out of bounds")
/// the moment the two ends sit on different islands -- exactly the
/// cross-mesh case every link this module spawns is for. Confirmed on real
/// FranklinMetro02 data (end polygon 260 vs start island's 72 polygons);
/// two unidirectional links take the correctly-indexed non-bidirectional
/// path and are semantically identical. Reported upstream as
/// <https://github.com/andriyDev/landmass/issues/192>; collapse back to one
/// `bidirectional: true` link once a fixed release is adopted.
fn spawn_link_pair(
    world: &mut World,
    archipelago_entity: Entity,
    start: Vec3,
    end: Vec3,
) -> [Entity; 2] {
    let mut spawn_one = |from: Vec3, to: Vec3| {
        world
            .spawn(AnimationLink3dBundle {
                link: AnimationLink3d {
                    start_edge: (from, from),
                    end_edge: (to, to),
                    kind: 0,
                    cost: 1.0,
                    bidirectional: false,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            })
            .id()
    };
    [spawn_one(start, end), spawn_one(end, start)]
}

/// Whether `door_form_id` is currently usable for route planning: already
/// open (runtime `InteractionState.open`), or not locked per its prepared
/// lock/key data and the player's inventory (the same
/// `interaction::door_is_locked` check the activation prompt uses -- never a
/// second lock model). A door with no prepared lock info is usable.
fn door_usable_now(
    world: &World,
    door_form_id: u32,
    door_lock_info: &HashMap<u32, DoorLockInfo>,
) -> bool {
    // `resolve_reference` panics without a `RefRegistry` resource, which
    // minimal test worlds may not have -- guard the open-set lookup on it.
    let open = world
        .get_resource::<crate::console::RefRegistry>()
        .is_some()
        && crate::console::resolve_reference(world, &format!("{door_form_id:08x}"))
            .ok()
            .is_some_and(|entity| {
                world
                    .get_resource::<interaction::InteractionState>()
                    .is_some_and(|state| state.open.contains(&entity))
            });
    let locked = door_lock_info.get(&door_form_id).is_some_and(|info| {
        let door = crate::vsa::PreparedDoor {
            lock_level: info.lock_level,
            key_form_id: info.key_form_id,
            destination: None,
        };
        match world.get_resource::<interaction::PlayerInventory>() {
            Some(inventory) => interaction::door_is_locked(&door, inventory),
            // No inventory resource (minimal test worlds): key
            // possession can't help, so locked is decided by the lock
            // level alone.
            None => door.lock_level.is_some_and(|level| level > 0),
        }
    });
    repath::door_usable(repath::DoorObservation { locked, open })
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
        "travel" => travel_agent(world, rest),
        "status" => agent_status(world, rest),
        "despawn" => despawn_agent(world, rest),
        other => Err(ConsoleError::new(
            "unknown_subcommand",
            format!(
                "unknown tna subcommand '{other}'; expected spawn, goto, travel, status, or despawn"
            ),
        )),
    }
}

fn usage_reply() -> ConsoleCommandResult {
    let usage = "usage: tna spawn|goto <x> <y> <z>|goto player|travel <door-formid>|status|despawn";
    ConsoleCommandResult::new(json!({ "usage": usage }), vec![usage.to_string()])
}

/// Spawns the capsule mesh + `bevy_landmass` agent entity at `position` in
/// the already-current archipelago (`ensure_archipelago` must have run).
/// Shared by `spawn_agent` (the `tna spawn` console command, positioned at
/// the player) and `restore_ledgered_agent` (issue #134, positioned at a
/// resolved ledger spawn point) -- neither sets `TestNavAgentState` itself,
/// so callers own that and its accompanying log line.
fn spawn_test_agent(world: &mut World, position: Vec3) -> Entity {
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
    agent_entity
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
    let agent_entity = spawn_test_agent(world, position);

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

/// Parses a bare or `0x`-prefixed hex FormID argument (`tna travel`'s door
/// selector), mirroring `console::parse_item_form_id`'s grammar -- that
/// helper is private to `console.rs`, outside this wave's file-ownership
/// boundary, so this is a small intentional duplicate rather than a new
/// cross-module dependency.
fn parse_form_id(value: &str) -> Option<u32> {
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    ((1..=8).contains(&digits.len()) && digits.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| u32::from_str_radix(digits, 16).ok())
        .flatten()
}

/// `tna travel <door-formid>` (issue #134): routes the test agent through
/// the given travel door end-to-end, wiring up `request_travel`.
fn travel_agent(world: &mut World, rest: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    let [door] = rest else {
        return Err(ConsoleError::new(
            "bad_arity",
            "tna travel requires exactly one door FormID",
        ));
    };
    let door_form_id = parse_form_id(door)
        .ok_or_else(|| ConsoleError::new("bad_type", "tna travel door FormID must be hex"))?;
    request_travel(world, door_form_id)?;
    Ok(ConsoleCommandResult::new(
        json!({ "door_form_id": door_form_id }),
        vec![format!(
            "nav agent travel requested to door {door_form_id:08x}"
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
        // Issue #134: a handed-off or frozen agent has no live entity but
        // still exists in the ledger -- report that instead of the "no
        // agent" error `tna spawn` would otherwise imply is needed.
        if let Some(entry) = world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(TEST_AGENT_ID)
        {
            let line = format!("nav agent handed off to cell {:08x}", entry.cell_form_id);
            return Ok(ConsoleCommandResult::new(
                json!({ "status": "handed-off", "cell": entry.cell_form_id }),
                vec![line],
            ));
        }
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
    let (door_link_state, link_desc) = {
        let state = world.resource::<TestNavAgentState>();
        (state.door_link, active_link_description(state))
    };
    let status = resolve_status(landmass_state, door_link_state);
    let target_desc = world
        .get::<AgentTarget3d>(agent_entity)
        .map(describe_target)
        .unwrap_or_else(|| "none".to_string());
    let mut line = format!(
        "nav agent status={} position=({:.2},{:.2},{:.2}) target={}",
        status.as_str(),
        position.x,
        position.y,
        position.z,
        target_desc
    );
    if let Some(link) = &link_desc {
        line.push_str(&format!(" link={link}"));
    }
    Ok(ConsoleCommandResult::new(
        json!({
            "status": status.as_str(),
            "position": [position.x, position.y, position.z],
            "target": target_desc,
            "link": link_desc,
        }),
        vec![line],
    ))
}

fn resolve_status(
    landmass_state: AgentState,
    door_link_state: door_link::DoorLinkState,
) -> landmass_graph::NavAgentStatus {
    if door_link::is_travel_reached(door_link_state) {
        return landmass_graph::NavAgentStatus::TravelReached;
    }
    if door_link::is_paused(door_link_state) || door_link::is_failed(door_link_state) {
        return landmass_graph::NavAgentStatus::Paused;
    }
    landmass_graph::map_agent_state(landmass_state)
}

/// The `link=` suffix for `tna status` (issue #113 feature 5): the active
/// link kind while interacting with one (`merge` while crossing a merge
/// seam, `door <formid>` through a door lifecycle), else `None`.
fn active_link_description(state: &TestNavAgentState) -> Option<String> {
    match state.active_link {
        Some(LinkKind::Merge) => Some("merge".to_string()),
        Some(LinkKind::Door { form_id }) => Some(format!("door {form_id:08x}")),
        None => match state.door_link {
            door_link::DoorLinkState::TravelReached {
                door_form_id,
                destination_cell_form_id,
            } => Some(format!(
                "door {door_form_id:08x} cell {destination_cell_form_id:08x}"
            )),
            _ => None,
        },
    }
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
    archipelago_state: Res<NavArchipelagoState>,
    mut ledger: ResMut<NavAgentLedger>,
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
            match state.active_link {
                // A merge-seam crossing involves no door: the state machine
                // never left Idle, so completion only clears the link.
                Some(LinkKind::Merge) | None => {
                    state.active_link = None;
                }
                Some(LinkKind::Door { .. }) => {
                    let new_state = door_link::transition(
                        state.door_link,
                        door_link::DoorLinkEvent::TraversalComplete,
                    );
                    let Some((door_form_id, destination_cell_form_id)) = (match new_state {
                        door_link::DoorLinkState::TravelReached {
                            door_form_id,
                            destination_cell_form_id,
                        } => Some((door_form_id, destination_cell_form_id)),
                        _ => None,
                    }) else {
                        state.door_link = new_state;
                        state.active_link = None;
                        continue;
                    };
                    // Issue #113's terminal travel seam: the agent stopped at
                    // the traversed door. Issue #134 owns what happens next:
                    // the agent leaves the active cell entirely, ledgered for
                    // the destination cell at that door's own paired marker.
                    info!(
                        "nav agent travel reached {door_form_id:08x} -> cell {destination_cell_form_id:08x}"
                    );
                    let destination_door_form_id = archipelago_state
                        .travel_doors
                        .get(&door_form_id)
                        .map(|link| link.destination_door_form_id);
                    match destination_door_form_id {
                        Some(destination_door_form_id) => {
                            ledger.0.record(ledger_policy::LedgerEntry {
                                agent_id: TEST_AGENT_ID,
                                cell_form_id: destination_cell_form_id,
                                spawn_kind: ledger_policy::SpawnKind::DoorMarker {
                                    destination_door_form_id,
                                },
                                remaining_target: None,
                            });
                            info!(
                                "nav agent handoff {TEST_AGENT_ID:08x} -> cell {destination_cell_form_id:08x}"
                            );
                            commands.entity(entity).despawn();
                            *state = TestNavAgentState::default();
                        }
                        None => {
                            // Defensive fallback (should not happen:
                            // `travel_doors` always carries this once
                            // `TravelReached` fired through it) -- keep
                            // #113's original behaviour rather than losing
                            // the agent silently.
                            warn!(
                                "nav agent handoff {door_form_id:08x}: no destination door metadata; agent left at the travel door"
                            );
                            commands.entity(entity).remove::<AgentTarget3d>();
                            state.travel_intent = None;
                            state.door_link = new_state;
                            state.active_link = None;
                        }
                    }
                }
            }
        }
    }
}

/// Requests the door `door_form_id` open through the same boundary the
/// `activate` console command uses. A door that is currently locked is
/// deliberately *not* scripted open (issue #113 feature 3: no teleporting
/// through closed doors, and a locked door resolves to the deterministic
/// `Failed` outcome via the wait bound) -- `scripted_door_open` bypasses
/// locks by design (dev tooling), so the lock gate lives here.
fn request_door_open(world: &mut World, door_form_id: u32) {
    let lock_info = world
        .resource::<NavArchipelagoState>()
        .door_lock_info
        .clone();
    if !door_usable_now(world, door_form_id, &lock_info) {
        info!("nav agent door {door_form_id:08x} locked; waiting");
        return;
    }
    if world
        .get_resource::<crate::console::RefRegistry>()
        .is_none()
    {
        warn!("nav agent door {door_form_id:08x}: reference not resolvable");
        return;
    }
    match crate::console::resolve_reference(world, &format!("{door_form_id:08x}")) {
        Ok(door_entity) => {
            interaction::scripted_door_open(world, door_entity);
        }
        Err(_) => {
            warn!("nav agent door {door_form_id:08x}: reference not resolvable");
        }
    }
}

/// Drives the door-link lifecycle: detects the agent reaching an off-mesh
/// link (a merge seam is crossed directly; a door link runs the pause ->
/// scripted-open -> wait -> traverse lifecycle) or arriving at a travel
/// door's triangle (issue #113 feature 3), requests the door open through
/// the same boundary the `activate` console command uses
/// (`interaction::scripted_door_open`), polls `InteractionState.open`, and
/// starts the kinematic crossing once the door is open. An exclusive
/// (`&mut World`) system since it needs to both query components and call
/// into `interaction`'s `&mut World`-based scripted door boundary in the
/// same step.
fn door_link_system(world: &mut World) {
    let Some(agent_entity) = world.resource::<TestNavAgentState>().entity else {
        return;
    };
    if world.get_entity(agent_entity).is_err() {
        return;
    }
    let current_state = world.resource::<TestNavAgentState>().door_link;

    match current_state {
        door_link::DoorLinkState::Idle
        | door_link::DoorLinkState::Failed { .. }
        | door_link::DoorLinkState::TravelReached { .. } => {
            // Travel arrival (issue #113 feature 3): a pending travel
            // intent whose door triangle the agent has reached starts the
            // door lifecycle with a Travel destination.
            let travel_arrival = world
                .resource::<TestNavAgentState>()
                .travel_intent
                .and_then(|door_form_id| {
                    let link = world
                        .resource::<NavArchipelagoState>()
                        .travel_doors
                        .get(&door_form_id)
                        .copied()?;
                    let position = world.get::<Transform>(agent_entity)?.translation;
                    (position.distance(link.triangle_midpoint) <= TRAVEL_ARRIVAL_DISTANCE)
                        .then_some((door_form_id, link))
                });
            if let Some((door_form_id, link)) = travel_arrival {
                let new_state = door_link::transition(
                    current_state,
                    door_link::DoorLinkEvent::LinkReached {
                        door_form_id,
                        destination: door_link::LinkDestination::Travel {
                            destination_cell_form_id: link.destination_cell_form_id,
                        },
                    },
                );
                world.entity_mut(agent_entity).insert(PauseAgent);
                request_door_open(world, door_form_id);
                info!("nav agent door wait {door_form_id:08x}");
                let mut state = world.resource_mut::<TestNavAgentState>();
                state.door_link = new_state;
                state.active_link = Some(LinkKind::Door {
                    form_id: door_form_id,
                });
                state.pending_traversal = Some((link.triangle_midpoint, link.door_position));
                return;
            }

            let Some(reached) = world.get::<ReachedAnimationLink3d>(agent_entity) else {
                return;
            };
            let link_entity = reached.link_entity;
            let start_point = reached.start_point;
            let end_point = reached.end_point;
            let Some(&link_kind) = world
                .resource::<NavArchipelagoState>()
                .link_kinds
                .get(&link_entity)
            else {
                return;
            };
            match link_kind {
                LinkKind::Merge => {
                    // A merge seam has no door: cross it immediately.
                    world.entity_mut(agent_entity).insert((
                        UsingAnimationLink,
                        DoorTraversal {
                            start: start_point,
                            end: end_point,
                            elapsed: 0.0,
                        },
                    ));
                    world.resource_mut::<TestNavAgentState>().active_link = Some(LinkKind::Merge);
                }
                LinkKind::Door {
                    form_id: door_form_id,
                } => {
                    let new_state = door_link::transition(
                        current_state,
                        door_link::DoorLinkEvent::LinkReached {
                            door_form_id,
                            destination: door_link::LinkDestination::IntraCell,
                        },
                    );
                    world.entity_mut(agent_entity).insert(PauseAgent);
                    request_door_open(world, door_form_id);
                    info!("nav agent door wait {door_form_id:08x}");
                    let mut state = world.resource_mut::<TestNavAgentState>();
                    state.door_link = new_state;
                    state.active_link = Some(link_kind);
                    state.pending_traversal = Some((start_point, end_point));
                }
            }
        }
        door_link::DoorLinkState::Paused { door_form_id, .. } => {
            let door_open = world
                .get_resource::<crate::console::RefRegistry>()
                .is_some()
                && crate::console::resolve_reference(world, &format!("{door_form_id:08x}"))
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
                let mut state = world.resource_mut::<TestNavAgentState>();
                state.active_link = None;
                state.travel_intent = None;
            }
            world.resource_mut::<TestNavAgentState>().door_link = new_state;
        }
        door_link::DoorLinkState::Traversing { .. } => {
            // `door_traversal_system` owns the crossing and emits
            // `TraversalComplete` once it finishes.
        }
    }
}

/// Polls every tracked door's usability once per frame and reacts to
/// *changes* only (issue #113 feature 4): the pure `repath::decide` table
/// turns a flip into a repath, applied here as (a) spawning/despawning the
/// affected two-sided door link so route planning includes/excludes it, (b)
/// re-inserting the agent's current target so landmass replans, and (c)
/// while paused at that very door, requesting the (now unlocked) door open.
/// Exactly one repath per actual state change -- the cached `door_usable`
/// map is the change detector.
fn door_availability_system(world: &mut World) {
    let tracked: Vec<(u32, bool)> = world
        .resource::<NavArchipelagoState>()
        .door_usable
        .iter()
        .map(|(&form_id, &usable)| (form_id, usable))
        .collect();
    if tracked.is_empty() {
        return;
    }
    let lock_info = world
        .resource::<NavArchipelagoState>()
        .door_lock_info
        .clone();
    for (door_form_id, was_usable) in tracked {
        let now_usable = door_usable_now(world, door_form_id, &lock_info);
        if now_usable == was_usable {
            continue;
        }
        world
            .resource_mut::<NavArchipelagoState>()
            .door_usable
            .insert(door_form_id, now_usable);

        let observation = repath::RepathObservation {
            door_became_blocked: !now_usable,
            door_became_unblocked: now_usable,
            ..Default::default()
        };
        if repath::decide(observation) != repath::RepathDecision::Repath {
            continue;
        }

        // Structural link update for two-sided door links.
        if now_usable {
            let blocked = {
                let mut state = world.resource_mut::<NavArchipelagoState>();
                let index = state
                    .blocked_door_links
                    .iter()
                    .position(|link| link.door_form_id == door_form_id);
                index.map(|index| state.blocked_door_links.remove(index))
            };
            if let Some(link) = blocked {
                let archipelago_entity = world
                    .resource::<NavArchipelagoState>()
                    .archipelago
                    .expect("availability tracking implies a built archipelago");
                for link_entity in spawn_link_pair(world, archipelago_entity, link.start, link.end)
                {
                    let mut state = world.resource_mut::<NavArchipelagoState>();
                    state.link_kinds.insert(
                        link_entity,
                        LinkKind::Door {
                            form_id: door_form_id,
                        },
                    );
                    state.links.push(link_entity);
                }
            }
        } else {
            let removed: Vec<Entity> = {
                let state = world.resource::<NavArchipelagoState>();
                state
                    .link_kinds
                    .iter()
                    .filter(|(_, kind)| {
                        matches!(kind, LinkKind::Door { form_id } if *form_id == door_form_id)
                    })
                    .map(|(&entity, _)| entity)
                    .collect()
            };
            // The door's link is spawned as a unidirectional pair (see
            // `spawn_link_pair`); despawn every entity but record only one
            // blocked entry, from the first entity's own orientation, so a
            // later unblock respawns exactly one pair.
            let mut recorded = false;
            for link_entity in removed {
                let (start, end) = world
                    .get::<AnimationLink3d>(link_entity)
                    .map(|link| (link.start_edge.0, link.end_edge.0))
                    .unwrap_or_default();
                if let Ok(entity) = world.get_entity_mut(link_entity) {
                    entity.despawn();
                }
                let mut state = world.resource_mut::<NavArchipelagoState>();
                state.link_kinds.remove(&link_entity);
                state.links.retain(|entity| *entity != link_entity);
                if !recorded {
                    state.blocked_door_links.push(BlockedDoorLink {
                        door_form_id,
                        start,
                        end,
                    });
                    recorded = true;
                }
            }
        }

        // Route refresh: re-insert the agent's current target so landmass
        // replans with the updated link set. `AgentTarget3d` is not `Clone`;
        // rebuild the equivalent value by matching its variants.
        let agent = world.resource::<TestNavAgentState>().entity;
        if let Some(agent_entity) = agent {
            let target = world
                .get::<AgentTarget3d>(agent_entity)
                .and_then(|target| match target {
                    AgentTarget3d::None => None,
                    AgentTarget3d::Point(point) => Some(AgentTarget3d::Point(*point)),
                    AgentTarget3d::Entity(entity) => Some(AgentTarget3d::Entity(*entity)),
                });
            if let Some(target) = target {
                world.entity_mut(agent_entity).insert(target);
            }
        }

        // A paused wait on this exact door can now proceed.
        if now_usable
            && matches!(
                world.resource::<TestNavAgentState>().door_link,
                door_link::DoorLinkState::Paused { door_form_id: paused, .. } if paused == door_form_id
            )
        {
            request_door_open(world, door_form_id);
        }

        info!(
            "nav agent repath door {door_form_id:08x} {}",
            if now_usable { "unblocked" } else { "blocked" }
        );
    }
}

/// Routes the test agent to `door_form_id`'s travel-door triangle and arms
/// the travel lifecycle (issue #113 feature 3). The traversal terminates at
/// the door with the `TravelReached` status; `door_traversal_system` (issue
/// #134) consumes that seam and hands the agent off to the destination
/// cell. Wired to the console as `tna travel <door-formid>`.
pub(crate) fn request_travel(world: &mut World, door_form_id: u32) -> Result<(), ConsoleError> {
    let Some(agent_entity) = world.resource::<TestNavAgentState>().entity else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned; use tna spawn first",
        ));
    };
    if world
        .resource::<TestNavAgentState>()
        .travel_intent
        .is_some()
    {
        return Err(ConsoleError::new(
            "travel_in_progress",
            "a travel request is already in progress",
        ));
    }
    let Some(link) = world
        .resource::<NavArchipelagoState>()
        .travel_doors
        .get(&door_form_id)
        .copied()
    else {
        return Err(ConsoleError::new(
            "unknown_travel_door",
            format!("no travel door {door_form_id:08x} is reachable from this cell's nav mesh"),
        ));
    };
    world
        .entity_mut(agent_entity)
        .insert(AgentTarget3d::Point(link.triangle_midpoint));
    world.resource_mut::<TestNavAgentState>().travel_intent = Some(door_form_id);
    info!("nav agent travel start {door_form_id:08x}");
    Ok(())
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
/// the active cell no longer matches the archipelago's cell, tear it down.
/// `PreparedSceneManifest` is optional so this system never panics in a
/// console-harness test world that never inserted one.
///
/// Issue #134 shipped amendment: wave 3's original behaviour despawned any
/// live test agent along with the stale archipelago, silently losing it.
/// That is replaced with `ledger_departing_agent`, which runs *before*
/// `teardown_archipelago` (it needs `NavArchipelagoState.travel_doors`,
/// still populated for the departing cell) and ledgers the agent instead --
/// follow-through to the destination cell if `note_player_swap_door` noted
/// the exact door the agent's active route was targeting, otherwise frozen
/// in place in the departing cell.
fn despawn_stale_navmesh_archipelago(world: &mut World) {
    let Some(current_cell) = world
        .get_resource::<PreparedSceneManifest>()
        .map(|manifest| manifest.cell.form_id)
    else {
        return;
    };
    let Some(source_cell) = world
        .resource::<NavArchipelagoState>()
        .cell_form_id
        .filter(|&cell| cell != current_cell)
    else {
        return;
    };
    let used_door = world.resource_mut::<PendingPlayerSwapDoor>().0.take();
    ledger_departing_agent(world, source_cell, used_door);
    teardown_archipelago(world);
}

/// The point-target component of `AgentTarget3d`, if any -- an `Entity`
/// target (e.g. `tna goto player`) cannot be meaningfully frozen, since the
/// target entity will not exist once the agent is restored, so it is
/// dropped rather than ledgered.
fn point_target(target: &AgentTarget3d) -> Option<[f32; 3]> {
    match target {
        AgentTarget3d::Point(point) => Some(point.to_array()),
        _ => None,
    }
}

/// Issue #134's player-initiated-swap ledgering: despawns the live test
/// agent (if any) into `NavAgentLedger`, deciding follow-through vs. freeze
/// via `ledger_policy::decide_swap_eligibility`. Must run before
/// `teardown_archipelago` clears `NavArchipelagoState.travel_doors`, the
/// source of a follow-through's destination-door metadata.
fn ledger_departing_agent(world: &mut World, source_cell: u32, used_door: Option<u32>) {
    let Some(agent_entity) = world.resource::<TestNavAgentState>().entity else {
        return;
    };
    let route_door = world.resource::<TestNavAgentState>().travel_intent;
    let position = world
        .get::<Transform>(agent_entity)
        .map(|transform| transform.translation.to_array())
        .unwrap_or_default();
    let remaining_target = world
        .get::<AgentTarget3d>(agent_entity)
        .and_then(point_target);

    let follow_through_link = used_door.and_then(|door| {
        let eligible = ledger_policy::decide_swap_eligibility(route_door, door)
            == ledger_policy::SwapEligibility::FollowThrough;
        eligible
            .then(|| {
                world
                    .resource::<NavArchipelagoState>()
                    .travel_doors
                    .get(&door)
                    .copied()
            })
            .flatten()
    });

    if let Some(link) = follow_through_link {
        world
            .resource_mut::<NavAgentLedger>()
            .0
            .record(ledger_policy::LedgerEntry {
                agent_id: TEST_AGENT_ID,
                cell_form_id: link.destination_cell_form_id,
                spawn_kind: ledger_policy::SpawnKind::DoorMarker {
                    destination_door_form_id: link.destination_door_form_id,
                },
                remaining_target: None,
            });
        info!(
            "nav agent handoff {TEST_AGENT_ID:08x} -> cell {:08x}",
            link.destination_cell_form_id
        );
    } else {
        world
            .resource_mut::<NavAgentLedger>()
            .0
            .record(ledger_policy::LedgerEntry {
                agent_id: TEST_AGENT_ID,
                cell_form_id: source_cell,
                spawn_kind: ledger_policy::SpawnKind::FrozenPosition { position },
                remaining_target,
            });
        info!("nav agent freeze {TEST_AGENT_ID:08x} cell {source_cell:08x}");
    }

    if let Ok(entity) = world.get_entity_mut(agent_entity) {
        entity.despawn();
    }
    *world.resource_mut::<TestNavAgentState>() = TestNavAgentState::default();
}

/// The placed position of the door reference `door_form_id` in the
/// *active* cell's manifest, if it exists there (issue #134's `DoorMarker`
/// spawn resolution).
fn door_position_in_active_cell(world: &World, door_form_id: u32) -> Option<Vec3> {
    world
        .get_resource::<PreparedSceneManifest>()?
        .placements
        .iter()
        .find(|placement| placement.reference_form_id == door_form_id)
        .map(|placement| Vec3::from_array(placement.translation))
}

/// Issue #134's restore side: once a cell containing ledgered entries
/// becomes active, claims them (`ledger_policy::Ledger::claim_for_
/// activation`) and spawns each restored entry, or diagnoses a stale
/// `DoorMarker` entry whose destination door is missing from this cell's
/// manifest. Cheap no-op the overwhelming majority of frames: bails before
/// touching the archipelago unless the ledger actually holds an entry for
/// the active cell.
fn restore_ledgered_agents_system(world: &mut World) {
    let Some(current_cell) = world
        .get_resource::<PreparedSceneManifest>()
        .map(|manifest| manifest.cell.form_id)
    else {
        return;
    };
    let has_pending = world
        .resource::<NavAgentLedger>()
        .0
        .entry_for(TEST_AGENT_ID)
        .is_some_and(|entry| entry.cell_form_id == current_cell);
    if !has_pending {
        return;
    }
    // Only ever the one test agent: an entry is only ever ledgered while no
    // entity exists, so a live entity here means restoration already
    // happened this activation (or `tna spawn` ran first) -- do not
    // double-spawn.
    if world.resource::<TestNavAgentState>().entity.is_some() {
        return;
    }

    let known_door_form_ids: std::collections::HashSet<u32> = world
        .resource::<PreparedSceneManifest>()
        .placements
        .iter()
        .filter(|placement| matches!(placement.semantic, crate::vsa::PreparedSemantic::Door(_)))
        .map(|placement| placement.reference_form_id)
        .collect();

    let claim = world
        .resource_mut::<NavAgentLedger>()
        .0
        .claim_for_activation(current_cell, &known_door_form_ids);

    for stale in claim.stale {
        warn!(
            "nav agent ledger stale {:08x} cell {:08x}: destination door {:08x} absent from the active cell",
            stale.agent_id, stale.cell_form_id, stale.missing_door_form_id
        );
    }

    for entry in claim.restored {
        restore_ledgered_agent(world, entry);
    }
}

fn restore_ledgered_agent(world: &mut World, entry: ledger_policy::LedgerEntry) {
    if ensure_archipelago(world).is_err() {
        warn!(
            "nav agent restore {:08x} cell {:08x}: no nav graph for this cell; ledger entry dropped",
            entry.agent_id, entry.cell_form_id
        );
        return;
    }
    let position = match entry.spawn_kind {
        ledger_policy::SpawnKind::FrozenPosition { position } => Vec3::from_array(position),
        ledger_policy::SpawnKind::DoorMarker {
            destination_door_form_id,
        } => match door_position_in_active_cell(world, destination_door_form_id) {
            Some(position) => position,
            None => {
                // `restore_ledgered_agents_system` already filtered stale
                // entries against `known_door_form_ids` from the same
                // manifest this reads, so this should not happen; guarded
                // defensively rather than trusted.
                warn!(
                    "nav agent restore {:08x} cell {:08x}: destination door {destination_door_form_id:08x} placement missing; ledger entry dropped",
                    entry.agent_id, entry.cell_form_id
                );
                return;
            }
        },
    };
    let agent_entity = spawn_test_agent(world, position);
    if let Some(target) = entry.remaining_target {
        world
            .entity_mut(agent_entity)
            .insert(AgentTarget3d::Point(Vec3::from_array(target)));
    }
    *world.resource_mut::<TestNavAgentState>() = TestNavAgentState {
        entity: Some(agent_entity),
        ..default()
    };
    info!(
        "nav agent restore {:08x} cell {:08x}",
        entry.agent_id, entry.cell_form_id
    );
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
        world.init_resource::<NavAgentLedger>();
        world.init_resource::<PendingPlayerSwapDoor>();
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

    /// Issue #134 shipped amendment: wave 3's teardown used to despawn a
    /// live test agent along with the stale archipelago, losing it. It is
    /// now ledgered instead -- here with no door noted
    /// (`PendingPlayerSwapDoor` defaults to `None`), so the agent freezes
    /// in the *departing* cell at its current position rather than being
    /// silently dropped.
    #[test]
    fn archipelago_teardown_on_cell_swap_ledgers_the_agent_instead_of_losing_it() {
        let mut world = harness_world();
        let archipelago = world.spawn_empty().id();
        let island = world.spawn_empty().id();
        world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xC0DE);
        world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago);
        world.resource_mut::<NavArchipelagoState>().islands = vec![island];
        let agent = world
            .spawn((TestNavAgentMarker, Transform::from_xyz(1.0, 2.0, 3.0)))
            .id();
        world.resource_mut::<TestNavAgentState>().entity = Some(agent);

        world.insert_resource(minimal_manifest(0xBEEF));
        despawn_stale_navmesh_archipelago(&mut world);

        assert!(world.get_entity(archipelago).is_err());
        assert!(world.get_entity(island).is_err());
        assert!(world.get_entity(agent).is_err(), "the live entity is gone");
        assert!(
            world
                .resource::<NavArchipelagoState>()
                .cell_form_id
                .is_none()
        );
        assert!(world.resource::<TestNavAgentState>().entity.is_none());

        let entry = world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(TEST_AGENT_ID)
            .expect("the agent must be ledgered, not lost");
        assert_eq!(entry.cell_form_id, 0xC0DE, "frozen in the departing cell");
        assert_eq!(
            entry.spawn_kind,
            ledger_policy::SpawnKind::FrozenPosition {
                position: [1.0, 2.0, 3.0]
            }
        );
    }

    /// Issue #134: a player-initiated swap through the exact door a live
    /// agent's active route was targeting hands it off to the destination
    /// cell (follow-through) instead of freezing it in the departing cell.
    #[test]
    fn a_player_swap_through_the_agents_own_route_door_follows_through() {
        let mut world = harness_world();
        world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xC0DE);
        world.resource_mut::<NavArchipelagoState>().archipelago = Some(world.spawn_empty().id());
        world
            .resource_mut::<NavArchipelagoState>()
            .travel_doors
            .insert(
                0x99,
                TravelDoorLink {
                    triangle_midpoint: Vec3::ZERO,
                    door_position: Vec3::ZERO,
                    destination_cell_form_id: 0xBEEF,
                    destination_door_form_id: 0x1234,
                },
            );
        let agent = world
            .spawn((TestNavAgentMarker, Transform::from_xyz(5.0, 0.0, 0.0)))
            .id();
        world.resource_mut::<TestNavAgentState>().entity = Some(agent);
        world.resource_mut::<TestNavAgentState>().travel_intent = Some(0x99);
        world.resource_mut::<PendingPlayerSwapDoor>().0 = Some(0x99);

        world.insert_resource(minimal_manifest(0xBEEF));
        despawn_stale_navmesh_archipelago(&mut world);

        assert!(world.get_entity(agent).is_err());
        let entry = world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(TEST_AGENT_ID)
            .expect("the agent must be ledgered");
        assert_eq!(
            entry.cell_form_id, 0xBEEF,
            "ledgered to the destination cell"
        );
        assert_eq!(
            entry.spawn_kind,
            ledger_policy::SpawnKind::DoorMarker {
                destination_door_form_id: 0x1234
            }
        );
    }

    /// Issue #134: a player swap through a door the agent's route was *not*
    /// targeting freezes it in the departing cell, same as an untargeted
    /// idle agent -- strict eligibility, no offscreen pathfinding.
    #[test]
    fn a_player_swap_through_a_different_door_still_freezes_the_agent() {
        let mut world = harness_world();
        world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xC0DE);
        world.resource_mut::<NavArchipelagoState>().archipelago = Some(world.spawn_empty().id());
        let agent = world
            .spawn((TestNavAgentMarker, Transform::from_xyz(7.0, 0.0, 0.0)))
            .id();
        world.resource_mut::<TestNavAgentState>().entity = Some(agent);
        // The agent is routed to a different travel door than the one the
        // player used.
        world.resource_mut::<TestNavAgentState>().travel_intent = Some(0x50);
        world.resource_mut::<PendingPlayerSwapDoor>().0 = Some(0x99);

        world.insert_resource(minimal_manifest(0xBEEF));
        despawn_stale_navmesh_archipelago(&mut world);

        assert!(world.get_entity(agent).is_err());
        let entry = world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(TEST_AGENT_ID)
            .expect("the agent must be ledgered, not lost");
        assert_eq!(entry.cell_form_id, 0xC0DE, "frozen in the departing cell");
        assert_eq!(
            entry.spawn_kind,
            ledger_policy::SpawnKind::FrozenPosition {
                position: [7.0, 0.0, 0.0]
            }
        );
    }

    /// Issue #134: a cell claimed by a ledgered entry spawns exactly one
    /// agent on activation, at the destination door's own placed position.
    #[test]
    fn matching_cell_activation_restores_exactly_one_ledgered_agent() {
        let mut world = harness_world();
        world.init_resource::<Assets<Mesh>>();
        world.init_resource::<Assets<StandardMaterial>>();
        world
            .resource_mut::<NavAgentLedger>()
            .0
            .record(ledger_policy::LedgerEntry {
                agent_id: TEST_AGENT_ID,
                cell_form_id: 0xBEEF,
                spawn_kind: ledger_policy::SpawnKind::DoorMarker {
                    destination_door_form_id: 0x1234,
                },
                remaining_target: None,
            });

        let mut manifest = minimal_manifest(0xBEEF);
        manifest
            .placements
            .push(door_placement_at(0x1234, [9.0, 1.0, 2.0]));
        // `PreparedSceneManifest.nav_graph` only needs to be `Some` here --
        // `ensure_archipelago` short-circuits on its `already_current`
        // check (below) before it would ever read this path from disk.
        manifest.nav_graph = Some(crate::vsa::PreparedNavGraphSource::default());
        world.insert_resource(manifest);
        // Pre-seed the archipelago as already current for 0xBEEF so
        // `ensure_archipelago` returns immediately without any real
        // `bevy_landmass`/file-I/O plumbing -- this test is about the
        // ledger claim + spawn-count contract, not archipelago building
        // (already covered by other tests/real-data acceptance).
        let archipelago_entity = world.spawn_empty().id();
        world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xBEEF);
        world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago_entity);

        restore_ledgered_agents_system(&mut world);

        let mut query = world.query_filtered::<Entity, With<TestNavAgentMarker>>();
        let agents: Vec<Entity> = query.iter(&world).collect();
        assert_eq!(agents.len(), 1, "exactly one agent must be spawned");
        assert!(
            world
                .resource::<NavAgentLedger>()
                .0
                .entry_for(TEST_AGENT_ID)
                .is_none(),
            "the claimed entry must be consumed"
        );
        let agent_entity = agents[0];
        let position = world.get::<Transform>(agent_entity).unwrap().translation;
        assert_eq!(
            position,
            Vec3::new(9.0, 1.0, 2.0),
            "spawned at the door marker"
        );
        assert_eq!(
            world.resource::<TestNavAgentState>().entity,
            Some(agent_entity)
        );
    }

    #[test]
    fn resolve_status_prefers_door_link_pause_over_landmass_state() {
        let paused = door_link::DoorLinkState::Paused {
            door_form_id: 0x99,
            waited_ticks: 1,
            destination: door_link::LinkDestination::IntraCell,
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

    #[test]
    fn resolve_status_reports_travel_reached_as_its_own_status() {
        let reached = door_link::DoorLinkState::TravelReached {
            door_form_id: 0x99,
            destination_cell_form_id: 0xC0DE,
        };
        assert_eq!(
            resolve_status(AgentState::Idle, reached),
            landmass_graph::NavAgentStatus::TravelReached
        );
        assert_eq!(
            landmass_graph::NavAgentStatus::TravelReached.as_str(),
            "travel-reached"
        );
    }

    /// Plan #113 minimal-App test: a travel-door request routes the agent
    /// to the door triangle and, on arrival, drives the existing
    /// `DoorLinkState` lifecycle (pause -> scripted-open boundary -> wait
    /// -> traverse) to the `TravelReached` terminal seam.
    #[test]
    fn travel_request_routes_to_the_door_and_completes_the_lifecycle() {
        let mut world = harness_world();
        world.init_resource::<Time>();
        world.init_resource::<interaction::InteractionState>();
        let mut registry = crate::console::RefRegistry::default();
        // `scripted_door_open` requires the resolved entity to carry a
        // `PlacementRoot` (the same invariant the `activate` command has).
        let door_entity = world
            .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
            .id();
        registry.register(door_entity, 0x99, None);
        world.insert_resource(registry);

        let agent = world
            .spawn((TestNavAgentMarker, Transform::from_xyz(0.0, 0.0, 0.0)))
            .id();
        world.resource_mut::<TestNavAgentState>().entity = Some(agent);
        world
            .resource_mut::<NavArchipelagoState>()
            .travel_doors
            .insert(
                0x99,
                TravelDoorLink {
                    triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                    door_position: Vec3::new(6.0, 0.0, 0.0),
                    destination_cell_form_id: 0xC0DE,
                    destination_door_form_id: 0x1234,
                },
            );

        request_travel(&mut world, 0x99).expect("travel request succeeds");
        assert!(matches!(
            world.get::<AgentTarget3d>(agent),
            Some(AgentTarget3d::Point(point)) if *point == Vec3::new(5.0, 0.0, 0.0)
        ));

        // Not yet at the door: the lifecycle must not start.
        door_link_system(&mut world);
        assert_eq!(
            world.resource::<TestNavAgentState>().door_link,
            door_link::DoorLinkState::Idle
        );

        // Arrive at the triangle midpoint: pause + door-open request.
        world.get_mut::<Transform>(agent).unwrap().translation = Vec3::new(5.0, 0.0, 0.0);
        door_link_system(&mut world);
        assert!(is_paused(&world));
        assert!(world.get::<PauseAgent>(agent).is_some());

        // The unlocked door was scripted open through the interaction
        // boundary by the arrival itself (same code path as `activate`).
        assert!(
            world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&door_entity),
            "arrival must scripted-open the unlocked door"
        );

        // The open door resumes into the kinematic crossing.
        door_link_system(&mut world);
        assert!(door_link::is_traversing(
            world.resource::<TestNavAgentState>().door_link
        ));
        assert!(world.get::<DoorTraversal>(agent).is_some());

        // Complete the crossing (elapsed already past the fixed duration).
        use bevy::ecs::system::RunSystemOnce;
        world.get_mut::<DoorTraversal>(agent).unwrap().elapsed = 10.0;
        world
            .run_system_once(door_traversal_system)
            .expect("traversal system runs");

        // Issue #134: the agent is handed off, not left standing at the
        // door -- despawned from the active cell and ledgered for the
        // destination cell at the paired door's marker.
        assert!(
            world.get_entity(agent).is_err(),
            "the agent must leave the active cell entirely on handoff"
        );
        assert!(world.resource::<TestNavAgentState>().entity.is_none());
        assert_eq!(
            world.resource::<TestNavAgentState>().door_link,
            door_link::DoorLinkState::Idle,
            "state is fully reset, not left in TravelReached"
        );
        let entry = world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(TEST_AGENT_ID)
            .expect("the agent must be ledgered on handoff");
        assert_eq!(entry.cell_form_id, 0xC0DE);
        assert_eq!(
            entry.spawn_kind,
            ledger_policy::SpawnKind::DoorMarker {
                destination_door_form_id: 0x1234
            }
        );
    }

    fn is_paused(world: &World) -> bool {
        door_link::is_paused(world.resource::<TestNavAgentState>().door_link)
    }

    /// Minimal travel-door placement for the lifecycle tests.
    fn door_placement(reference_form_id: u32) -> crate::vsa::PreparedPlacement {
        crate::vsa::PreparedPlacement {
            reference_form_id,
            base_form_id: 1,
            asset_path: None,
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            error: None,
            physics_asset_path: None,
            physics_source: None,
            physics_classification: Default::default(),
            step_support: false,
            mutability: Default::default(),
            mutability_root_form_id: None,
            reference_kind: "REFR".into(),
            base_kind: "DOOR".into(),
            editor_id: None,
            display_name: None,
            count: 1,
            semantic: crate::vsa::PreparedSemantic::Door(crate::vsa::PreparedDoor {
                lock_level: None,
                key_form_id: None,
                destination: None,
            }),
            initially_enabled: true,
            enable_parent: None,
            owner_form_id: None,
            owner_faction_rank: None,
            inventory: Vec::new(),
            audio: Default::default(),
            ao_mode: "ao-none".into(),
        }
    }

    /// A door placement at a specific position (issue #134's restore
    /// tests, which spawn at a resolved door-marker position).
    fn door_placement_at(
        reference_form_id: u32,
        translation: [f32; 3],
    ) -> crate::vsa::PreparedPlacement {
        crate::vsa::PreparedPlacement {
            translation,
            ..door_placement(reference_form_id)
        }
    }

    /// Plan #113 minimal-App test: a locked travel door never scripted-opens
    /// (no teleporting through closed doors) and resolves to the existing
    /// deterministic `Failed` status via the wait bound.
    #[test]
    fn locked_travel_door_fails_deterministically_without_opening() {
        let mut world = harness_world();
        world.init_resource::<interaction::InteractionState>();
        let mut registry = crate::console::RefRegistry::default();
        let door_entity = world.spawn_empty().id();
        registry.register(door_entity, 0x99, None);
        world.insert_resource(registry);

        let agent = world
            .spawn((TestNavAgentMarker, Transform::from_xyz(5.0, 0.0, 0.0)))
            .id();
        world.resource_mut::<TestNavAgentState>().entity = Some(agent);
        {
            let mut state = world.resource_mut::<NavArchipelagoState>();
            state.travel_doors.insert(
                0x99,
                TravelDoorLink {
                    triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                    door_position: Vec3::new(6.0, 0.0, 0.0),
                    destination_cell_form_id: 0xC0DE,
                    destination_door_form_id: 0x1234,
                },
            );
            state.door_lock_info.insert(
                0x99,
                DoorLockInfo {
                    lock_level: Some(50),
                    key_form_id: None,
                },
            );
            state.door_usable.insert(0x99, false);
        }

        request_travel(&mut world, 0x99).expect("routing to a locked door is allowed");
        door_link_system(&mut world);
        assert!(is_paused(&world));
        assert!(
            !world
                .resource::<interaction::InteractionState>()
                .open
                .contains(&door_entity),
            "a locked door must never be scripted open by the nav agent"
        );

        for _ in 0..door_link::MAX_WAIT_TICKS {
            door_link_system(&mut world);
        }
        assert_eq!(
            world.resource::<TestNavAgentState>().door_link,
            door_link::DoorLinkState::Failed { door_form_id: 0x99 }
        );
    }

    /// Plan #113 minimal-App test: a door state change triggers exactly one
    /// repath -- the blocked two-sided link spawns once when the door
    /// becomes usable, and repeated polls with no further change do
    /// nothing.
    #[test]
    fn a_door_state_change_triggers_exactly_one_repath() {
        let mut world = harness_world();
        world.init_resource::<interaction::InteractionState>();
        let mut registry = crate::console::RefRegistry::default();
        let door_entity = world.spawn_empty().id();
        registry.register(door_entity, 0x99, None);
        world.insert_resource(registry);

        let archipelago = world.spawn_empty().id();
        {
            let mut state = world.resource_mut::<NavArchipelagoState>();
            state.archipelago = Some(archipelago);
            state.door_lock_info.insert(
                0x99,
                DoorLockInfo {
                    lock_level: Some(50),
                    key_form_id: None,
                },
            );
            state.door_usable.insert(0x99, false);
            state.blocked_door_links.push(BlockedDoorLink {
                door_form_id: 0x99,
                start: Vec3::ZERO,
                end: Vec3::new(1.0, 0.0, 0.0),
            });
        }

        // No change: locked stays locked, nothing spawns.
        door_availability_system(&mut world);
        assert!(world.resource::<NavArchipelagoState>().links.is_empty());
        assert_eq!(
            world
                .resource::<NavArchipelagoState>()
                .blocked_door_links
                .len(),
            1
        );

        // The door opens (e.g. the player activates it): one flip, one
        // repath -- the link spawns (one unidirectional pair, see
        // `spawn_link_pair`) and the blocked entry is consumed.
        world
            .resource_mut::<interaction::InteractionState>()
            .open
            .insert(door_entity);
        door_availability_system(&mut world);
        assert_eq!(world.resource::<NavArchipelagoState>().links.len(), 2);
        assert!(
            world
                .resource::<NavArchipelagoState>()
                .blocked_door_links
                .is_empty()
        );

        // Steady state: repeated polls never spawn another link pair.
        door_availability_system(&mut world);
        door_availability_system(&mut world);
        assert_eq!(world.resource::<NavArchipelagoState>().links.len(), 2);
    }

    /// Plan #113 minimal-App test: never two concurrent travel requests.
    #[test]
    fn concurrent_travel_requests_are_rejected() {
        let mut world = harness_world();
        let agent = world.spawn(TestNavAgentMarker).id();
        world.resource_mut::<TestNavAgentState>().entity = Some(agent);
        world
            .resource_mut::<NavArchipelagoState>()
            .travel_doors
            .insert(
                0x99,
                TravelDoorLink {
                    triangle_midpoint: Vec3::ZERO,
                    door_position: Vec3::ZERO,
                    destination_cell_form_id: 0xC0DE,
                    destination_door_form_id: 0x1234,
                },
            );
        request_travel(&mut world, 0x99).expect("first request succeeds");
        let error = request_travel(&mut world, 0x99).unwrap_err();
        assert_eq!(error.code, "travel_in_progress");
    }

    #[test]
    fn travel_request_errors_without_an_agent_or_a_known_door() {
        let mut world = harness_world();
        assert_eq!(
            request_travel(&mut world, 0x99).unwrap_err().code,
            "no_agent"
        );
        let agent = world.spawn(TestNavAgentMarker).id();
        world.resource_mut::<TestNavAgentState>().entity = Some(agent);
        assert_eq!(
            request_travel(&mut world, 0x99).unwrap_err().code,
            "unknown_travel_door"
        );
    }

    #[test]
    fn active_link_description_reports_merge_door_and_travel_reached() {
        let mut state = TestNavAgentState::default();
        assert_eq!(active_link_description(&state), None);

        state.active_link = Some(LinkKind::Merge);
        assert_eq!(active_link_description(&state), Some("merge".to_string()));

        state.active_link = Some(LinkKind::Door { form_id: 0x99 });
        assert_eq!(
            active_link_description(&state),
            Some("door 00000099".to_string())
        );

        state.active_link = None;
        state.door_link = door_link::DoorLinkState::TravelReached {
            door_form_id: 0x99,
            destination_cell_form_id: 0xC0DE,
        };
        assert_eq!(
            active_link_description(&state),
            Some("door 00000099 cell 0000c0de".to_string())
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
