use std::collections::HashMap;

use bevy::prelude::*;
use bevy_landmass::coords::ThreeD;
use bevy_landmass::prelude::*;
use bevy_landmass::{
    AnimationLinkReachedDistance, PauseAgent, PermittedAnimationLinks, UsingAnimationLink,
};
use serde_json::json;

use crate::console::{ConsoleCommandResult, ConsoleError, ConsoleInvocation};
use crate::viewer::nav::agent::*;
use crate::viewer::nav::api;
use crate::viewer::nav::debug::player::*;
use crate::viewer::nav::door_link;

fn console_error_from_nav(error: api::NavError) -> ConsoleError {
    ConsoleError::new(error.code, error.message)
}

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
        "bind" => bind_agent(world, rest),
        "goto" => goto_agent(world, rest),
        "path" => path_probe(world, rest),
        "probe" => animation_link_probe(world, rest),
        "travel" => travel_agent(world, rest),
        "status" => agent_status(world, rest),
        "despawn" => despawn_agent(world, rest),
        "solverate" => solve_rate_command(world, rest),
        other => Err(ConsoleError::new(
            "unknown_subcommand",
            format!(
                "unknown tna subcommand '{other}'; expected spawn, bind, goto, path, probe, travel, status, despawn, or solverate"
            ),
        )),
    }
}

pub(crate) fn usage_reply() -> ConsoleCommandResult {
    let usage = "usage: tna spawn [<index>]|bind [<index>] <actor-reference-formid>|goto [<index>] <x> <y> <z>|goto [<index>] player|path [<index>] <x> <y> <z>|probe [<index>] <sx> <sy> <sz> <ex> <ey> <ez>|travel [<index>] <door-formid>|status [<index>]|despawn [<index>]|solverate [<n>]";
    ConsoleCommandResult::new(json!({ "usage": usage }), vec![usage.to_string()])
}

/// `tna solverate [<n>]` (issue #114 added scope, wave 5): reports the
/// current `NavSolveRate` divisor with no argument, following the
/// `getrender`/`setrender` get-or-set convention; sets it with one. `n` must
/// be a positive integer (`0` would mean "never solve", not "always solve";
/// `movement_policy::should_solve`/`solve_blend_fraction` both clamp
/// defensively too, but the console rejects it outright rather than
/// silently reinterpreting it).
pub(crate) fn solve_rate_command(
    world: &mut World,
    rest: &[String],
) -> Result<ConsoleCommandResult, ConsoleError> {
    match rest {
        [] => {
            let interval = world.resource::<NavSolveRate>().0;
            Ok(ConsoleCommandResult::new(
                json!({ "interval": interval }),
                vec![format!("nav solve rate interval={interval}")],
            ))
        }
        [value] => {
            let interval = value
                .parse::<u32>()
                .ok()
                .filter(|&n| n >= 1)
                .ok_or_else(|| {
                    ConsoleError::new(
                        "bad_type",
                        "tna solverate interval must be a positive integer",
                    )
                })?;
            world.resource_mut::<NavSolveRate>().0 = interval;
            info!("nav solve rate interval={interval}");
            Ok(ConsoleCommandResult::new(
                json!({ "interval": interval }),
                vec![format!("nav solve rate interval set to {interval}")],
            ))
        }
        _ => Err(ConsoleError::new(
            "bad_arity",
            "tna solverate accepts at most one interval",
        )),
    }
}

/// Parses an agent index argument. Every `tna` subcommand that used to
/// address the single spike agent now takes this as an optional leading
/// token; omitting it defaults to agent 0 (issue #114 feature 4's
/// back-compat requirement). Issue #215 removed the four-slot cap; the only
/// remaining ceiling is the defensive dense-allocation bound.
pub(crate) fn parse_agent_index(value: &str) -> Result<usize, ConsoleError> {
    value
        .parse::<usize>()
        .ok()
        .filter(|index| *index <= MAX_AGENT_INDEX)
        .ok_or_else(|| {
            ConsoleError::new(
                "bad_agent_index",
                format!("agent index must be an integer 0..={MAX_AGENT_INDEX}"),
            )
        })
}

/// Spawns the capsule mesh + `bevy_landmass` agent entity at `position` in
/// the already-current archipelago (`ensure_archipelago` must have run),
/// with its `AgentRuntime`/`AgentKcc` components at their defaults. Shared
/// by `spawn_agent` (the `tna spawn` console command, positioned at the
/// player) and `restore_ledgered_agent` (issue #134, positioned at a
/// resolved ledger spawn point) -- neither sets `DebugAgentRoster` itself,
/// so callers own that and its accompanying log line.
pub(crate) fn spawn_test_agent(world: &mut World, position: Vec3) -> Entity {
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
            NavAgent,
            Transform::from_translation(position),
            Visibility::Inherited,
            // Offset zero: a capsule entity's own transform already is the
            // capsule centre (issue #188 introduced the offset for bound
            // actors only, leaving this path bit-for-bit as it was).
            agent_components(archipelago_entity, 0.0),
            // The debug capsule's transform is centre-level but its nav
            // points are feet-level; make landmass's full-3D link reach
            // distance explicit for this centre-based agent only. Bound
            // actors retain the normal radius-based threshold because their
            // placement-root transform is feet-level.
            AnimationLinkReachedDistance(TEST_AGENT_ANIMATION_LINK_REACHED_DISTANCE),
        ))
        .id();
    // Zero offset (issue #114 real-data regression fix, M4 wave 5): the
    // parent `agent_entity`'s `Transform` is already the capsule *centre*
    // (physics-authoritative movement positions it there, mirroring the
    // player's own capsule-centre convention -- see `spawn_bare_agent`'s
    // doc comment and the horizontal-distance regression fix a few commits
    // back), not feet level like the wave-3/4 navmesh-Y-snapped kinematic
    // agent this `AGENT_HEIGHT / 2.0` offset used to compensate for. Lifting
    // the visual child by another half-height on top of an already-centred
    // parent double-counts that offset, floating the rendered capsule a
    // full half-height above the floor even though the physics capsule
    // (steps/slopes) sits correctly. `Capsule3d`'s mesh is centred at its
    // own local origin, so a zero-offset child renders centred exactly on
    // the parent -- the capsule bottom lands on the feet/floor.
    let visual = world
        .spawn((Mesh3d(mesh), MeshMaterial3d(material), Transform::IDENTITY))
        .id();
    world.entity_mut(agent_entity).add_child(visual);
    // Issue #155 feature 2: this agent's very first path query must already
    // respect whatever is locked in the active cell -- see
    // `apply_door_lock_overrides`'s doc comment.
    apply_door_lock_overrides(world, agent_entity);
    agent_entity
}

pub(crate) fn spawn_agent(
    world: &mut World,
    rest: &[String],
) -> Result<ConsoleCommandResult, ConsoleError> {
    let index = match rest {
        [] => 0,
        [index] => parse_agent_index(index)?,
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna spawn accepts at most one agent index",
            ));
        }
    };
    ensure_archipelago(world).map_err(console_error_from_nav)?;
    if world.resource::<DebugAgentRoster>().is_occupied(index) {
        return Err(ConsoleError::new(
            "already_spawned",
            "a test nav agent is already spawned at this index; use tna despawn first",
        ));
    }
    let position = player_transform_query(world)
        .ok_or_else(|| ConsoleError::new("player_unavailable", "the FPS player does not exist"))?;
    let agent_entity = spawn_test_agent(world, position);
    world
        .resource_mut::<DebugAgentRoster>()
        .set(index, Some(agent_entity));
    world
        .entity_mut(agent_entity)
        .insert(DebugNavAgent { index });
    info!(
        "nav agent {index} spawn position=({:.2},{:.2},{:.2})",
        position.x, position.y, position.z
    );
    Ok(ConsoleCommandResult::new(
        json!({ "index": index, "position": [position.x, position.y, position.z] }),
        vec![format!(
            "nav agent {index} spawned at ({:.2}, {:.2}, {:.2})",
            position.x, position.y, position.z
        )],
    ))
}

/// The nav-agent component set, minus the marker and the debug capsule
/// mesh: everything an entity needs to be routed and physically moved.
/// Shared by `spawn_test_agent` (a fresh capsule) and `bind_agent` (an
/// already-live projected actor), so the two paths cannot drift into
/// disagreeing about what an agent is.
pub(crate) fn bind_agent(
    world: &mut World,
    rest: &[String],
) -> Result<ConsoleCommandResult, ConsoleError> {
    let (index, form_id) = match rest {
        [form_id] => (0, form_id),
        [index, form_id] => (parse_agent_index(index)?, form_id),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna bind requires [<index>] <actor-reference-formid>",
            ));
        }
    };
    let reference_form_id = parse_form_id(form_id)
        .ok_or_else(|| ConsoleError::new("bad_type", "tna bind actor FormID must be hex"))?;
    if world.resource::<DebugAgentRoster>().is_occupied(index) {
        return Err(ConsoleError::new(
            "already_spawned",
            "a nav agent already occupies this index; use tna despawn first",
        ));
    }
    let entity = actor_entity_by_reference(world, reference_form_id).ok_or_else(|| {
        ConsoleError::new(
            "no_actor",
            format!("no projected actor with reference FormID {reference_form_id:08x}"),
        )
    })?;
    bind_agent_entity(world, entity).map_err(console_error_from_nav)?;
    world
        .resource_mut::<DebugAgentRoster>()
        .set(index, Some(entity));
    world.entity_mut(entity).insert(DebugNavAgent { index });
    let position = world
        .get::<Transform>(entity)
        .map_or(Vec3::ZERO, |transform| transform.translation);
    info!(
        "nav agent {index} bound actor {reference_form_id:08x} position=({:.2},{:.2},{:.2})",
        position.x, position.y, position.z
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "index": index,
            "reference_form_id": reference_form_id,
            "position": [position.x, position.y, position.z],
        }),
        vec![format!(
            "nav agent {index} bound to actor {reference_form_id:08x} at ({:.2}, {:.2}, {:.2})",
            position.x, position.y, position.z
        )],
    ))
}

/// Parses a bare or `0x`-prefixed hex FormID argument (`tna travel`'s door
/// selector), mirroring `console::parse_item_form_id`'s grammar -- that
/// helper is private to `console.rs`, outside this wave's file-ownership
/// boundary, so this is a small intentional duplicate rather than a new
/// cross-module dependency.
pub(crate) fn parse_form_id(value: &str) -> Option<u32> {
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    ((1..=8).contains(&digits.len()) && digits.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| u32::from_str_radix(digits, 16).ok())
        .flatten()
}

/// `tna travel [<index>] <door-formid>` (issue #134; indexed #114): routes
/// the given agent through the given travel door end-to-end, wiring up
/// `request_travel`.
pub(crate) fn travel_agent(
    world: &mut World,
    rest: &[String],
) -> Result<ConsoleCommandResult, ConsoleError> {
    let (index, door) = match rest {
        [door] => (0, door),
        [index, door] => (parse_agent_index(index)?, door),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna travel requires [<index>] <door-formid>",
            ));
        }
    };
    let door_form_id = parse_form_id(door)
        .ok_or_else(|| ConsoleError::new("bad_type", "tna travel door FormID must be hex"))?;
    request_travel(world, index, door_form_id).map_err(console_error_from_nav)?;
    Ok(ConsoleCommandResult::new(
        json!({ "index": index, "door_form_id": door_form_id }),
        vec![format!(
            "nav agent {index} travel requested to door {door_form_id:08x}"
        )],
    ))
}

pub(crate) fn parse_goto_point(x: &str, y: &str, z: &str) -> Result<Vec3, ConsoleError> {
    let parse = |value: &str| {
        value.parse::<f32>().map_err(|_| {
            ConsoleError::new("bad_type", "tna goto coordinates must be finite numbers")
        })
    };
    Ok(Vec3::new(parse(x)?, parse(y)?, parse(z)?))
}

pub(crate) fn goto_player_target(world: &mut World) -> Result<AgentTarget3d, ConsoleError> {
    let player_entity = player_entity_query(world)
        .ok_or_else(|| ConsoleError::new("player_unavailable", "the FPS player does not exist"))?;
    Ok(AgentTarget3d::Entity(player_entity))
}

/// `tna goto [<index>] <x> <y> <z>|player` (indexed #114): the leading
/// index token is optional and distinguished purely by argument count, so
/// every previously single-agent form (`goto <x> <y> <z>`, `goto player`)
/// is unchanged and still addresses agent 0.
pub(crate) fn goto_agent(
    world: &mut World,
    rest: &[String],
) -> Result<ConsoleCommandResult, ConsoleError> {
    let (index, target, description) = match rest {
        [value] if value == "player" => (0, goto_player_target(world)?, "player".to_string()),
        [index, value] if value == "player" => (
            parse_agent_index(index)?,
            goto_player_target(world)?,
            "player".to_string(),
        ),
        [x, y, z] => {
            let point = parse_goto_point(x, y, z)?;
            (
                0,
                AgentTarget3d::Point(point),
                format!("({:.2}, {:.2}, {:.2})", point.x, point.y, point.z),
            )
        }
        [index, x, y, z] => {
            let point = parse_goto_point(x, y, z)?;
            (
                parse_agent_index(index)?,
                AgentTarget3d::Point(point),
                format!("({:.2}, {:.2}, {:.2})", point.x, point.y, point.z),
            )
        }
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna goto requires [<index>] <x> <y> <z> or [<index>] player",
            ));
        }
    };
    let Some(agent_entity) = world.resource::<DebugAgentRoster>().get(index) else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned at this index; use tna spawn first",
        ));
    };
    route_agent_to_target(world, agent_entity, target);
    Ok(ConsoleCommandResult::new(
        json!({ "index": index, "target": description }),
        vec![format!("nav agent {index} target set to {description}")],
    ))
}

/// `tna path [<index>] <x> <y> <z>` queries the current landmass
/// archipelago directly, without changing the agent's route. This is a
/// diagnostic seam for distinguishing a missing animation-link attachment
/// from movement/physics failure: the returned `PathStep::AnimationLink`
/// entries are the pathfinder's authoritative off-mesh-link decisions.
pub(crate) fn path_probe(
    world: &mut World,
    rest: &[String],
) -> Result<ConsoleCommandResult, ConsoleError> {
    let (index, target) = match rest {
        [x, y, z] => (0, parse_goto_point(x, y, z)?),
        [index, x, y, z] => (parse_agent_index(index)?, parse_goto_point(x, y, z)?),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna path requires [<index>] <x> <y> <z>",
            ));
        }
    };
    let Some(agent_entity) = world.resource::<DebugAgentRoster>().get(index) else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned at this index; use tna spawn first",
        ));
    };
    let position = world
        .get::<GlobalTransform>(agent_entity)
        .map(|transform| transform.translation())
        .or_else(|| {
            world
                .get::<Transform>(agent_entity)
                .map(|transform| transform.translation)
        })
        .ok_or_else(|| ConsoleError::new("agent_unavailable", "nav agent has no transform"))?;
    let archipelago_entity = world
        .resource::<NavArchipelagoState>()
        .archipelago
        .ok_or_else(|| ConsoleError::new("no_archipelago", "no nav archipelago is active"))?;
    let archipelago = world
        .get::<Archipelago3d>(archipelago_entity)
        .ok_or_else(|| ConsoleError::new("no_archipelago", "no nav archipelago is active"))?;
    let sample_distance = archipelago
        .get_agent_options()
        .point_sample_distance
        .clone();
    let start = archipelago
        .sample_point(position, &sample_distance)
        .map_err(|error| ConsoleError::new("start_off_navmesh", error.to_string()))?;
    let end = archipelago
        .sample_point(target, &sample_distance)
        .map_err(|error| ConsoleError::new("target_off_navmesh", error.to_string()))?;
    let permitted = world
        .get::<PermittedAnimationLinks>(agent_entity)
        .cloned()
        .unwrap_or_default();
    let path = archipelago
        .find_path(&start, &end, &HashMap::new(), permitted)
        .map_err(|error| ConsoleError::new("no_path", error.to_string()))?;
    let steps = path
        .iter()
        .map(|step| format!("{step:?}"))
        .collect::<Vec<_>>();
    let animation_links = path
        .iter()
        .filter(|step| matches!(step, bevy_landmass::PathStep::AnimationLink { .. }))
        .count();
    let start_point = start.point();
    let end_point = end.point();
    let start_island = format!("{:?}", start.island());
    let end_island = format!("{:?}", end.island());
    let line = format!(
        "nav agent {index} path steps={} animation_links={} samples={} -> {}",
        steps.len(),
        animation_links,
        start_island,
        end_island
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "index": index,
            "position": [position.x, position.y, position.z],
            "target": [target.x, target.y, target.z],
            "start_sample": [start_point.x, start_point.y, start_point.z],
            "target_sample": [end_point.x, end_point.y, end_point.z],
            "start_island": start_island,
            "end_island": end_island,
            "steps": steps,
            "animation_links": animation_links,
        }),
        vec![line],
    ))
}

pub(crate) struct AnimationLinkDebugCapture {
    link_entity: Entity,
    agent_entity: Entity,
    installed: bool,
    in_corridor: bool,
    next_step: bool,
}

impl bevy_landmass::debug::DebugDrawer<ThreeD> for AnimationLinkDebugCapture {
    fn add_point(&mut self, _point_type: bevy_landmass::debug::PointType, _point: Vec3) {}

    fn add_line(&mut self, line_type: bevy_landmass::debug::LineType, _line: [Vec3; 2]) {
        match line_type {
            bevy_landmass::debug::LineType::AnimationLinkConnection(link)
                if link == self.link_entity =>
            {
                self.installed = true
            }
            bevy_landmass::debug::LineType::CorridorAnimationLink {
                agent,
                animation_link,
            } if agent == self.agent_entity && animation_link == self.link_entity => {
                self.in_corridor = true;
            }
            bevy_landmass::debug::LineType::PathAnimationLink {
                agent,
                animation_link,
            } if agent == self.agent_entity && animation_link == self.link_entity => {
                self.next_step = true;
            }
            _ => {}
        }
    }

    fn add_triangle(
        &mut self,
        _triangle_type: bevy_landmass::debug::TriangleType,
        _triangle: [Vec3; 3],
    ) {
    }
}

pub(crate) fn animation_link_probe(
    world: &mut World,
    rest: &[String],
) -> Result<ConsoleCommandResult, ConsoleError> {
    let (index, start, end) = match rest {
        [sx, sy, sz, ex, ey, ez] => (
            0,
            parse_goto_point(sx, sy, sz)?,
            parse_goto_point(ex, ey, ez)?,
        ),
        [index, sx, sy, sz, ex, ey, ez] => (
            parse_agent_index(index)?,
            parse_goto_point(sx, sy, sz)?,
            parse_goto_point(ex, ey, ez)?,
        ),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna probe requires [<index>] <sx> <sy> <sz> <ex> <ey> <ez>",
            ));
        }
    };
    let Some(agent_entity) = world.resource::<DebugAgentRoster>().get(index) else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned at this index; use tna spawn first",
        ));
    };
    let archipelago_entity = world
        .resource::<NavArchipelagoState>()
        .archipelago
        .ok_or_else(|| ConsoleError::new("no_archipelago", "no nav archipelago is active"))?;
    let link_entity = world
        .resource::<NavArchipelagoState>()
        .links
        .iter()
        .copied()
        .find(|&entity| {
            world
                .get::<AnimationLink3d>(entity)
                .is_some_and(|link| {
                    let link_start = (link.start_edge.0 + link.start_edge.1) * 0.5;
                    let link_end = (link.end_edge.0 + link.end_edge.1) * 0.5;
                    link_start.distance(start) <= 0.05 && link_end.distance(end) <= 0.05
                })
        })
        .ok_or_else(|| {
            ConsoleError::new(
                "link_not_found",
                format!(
                    "no directional animation link matches ({:.3},{:.3},{:.3}) -> ({:.3},{:.3},{:.3})",
                    start.x, start.y, start.z, end.x, end.y, end.z
                ),
            )
        })?;
    let archipelago = world
        .get::<Archipelago3d>(archipelago_entity)
        .ok_or_else(|| ConsoleError::new("no_archipelago", "no nav archipelago is active"))?;
    let landmass_state = world
        .get::<AgentState>(agent_entity)
        .copied()
        .unwrap_or_default();
    let reach_distance = world
        .get::<AnimationLinkReachedDistance>(agent_entity)
        .map(|distance| distance.0);
    let using_animation_link = world.get::<UsingAnimationLink>(agent_entity).is_some();
    let reached_animation_link = world
        .get::<ReachedAnimationLink3d>(agent_entity)
        .map(|link| format!("{:?}", link.link_entity));
    let desired_velocity = world
        .get::<AgentDesiredVelocity3d>(agent_entity)
        .map(|velocity| velocity.velocity())
        .unwrap_or_default();
    let (door_link_state, active_link) = world
        .get::<AgentRuntime>(agent_entity)
        .map(|runtime| (runtime.door_link, runtime.active_link))
        .unwrap_or_default();
    let link_kind = world
        .resource::<NavArchipelagoState>()
        .link_kinds
        .get(&link_entity)
        .map(|kind| format!("{kind:?}"));
    let merge_traversal = world.get::<MergeTraversal>(agent_entity).is_some();
    let pause_agent = world.get::<PauseAgent>(agent_entity).is_some();
    let mut capture = AnimationLinkDebugCapture {
        link_entity,
        agent_entity,
        installed: false,
        in_corridor: false,
        next_step: false,
    };
    bevy_landmass::debug::draw_archipelago_debug(archipelago, &mut capture)
        .map_err(|error| ConsoleError::new("nav_data_dirty", error.to_string()))?;
    let line = format!(
        "merge-probe link={link_entity:?} kind={link_kind:?} installed={} in_corridor={} next_step={} state={landmass_state:?} reach_distance={:?} using={} reached={:?} door={door_link_state:?} active={active_link:?} merge_traversal={merge_traversal} paused={pause_agent} desired=({:.2},{:.2},{:.2})",
        capture.installed,
        capture.in_corridor,
        capture.next_step,
        reach_distance,
        using_animation_link,
        reached_animation_link,
        desired_velocity.x,
        desired_velocity.y,
        desired_velocity.z,
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "index": index,
            "link_entity": format!("{link_entity:?}"),
            "link_kind": link_kind,
            "start": [start.x, start.y, start.z],
            "end": [end.x, end.y, end.z],
            "installed": capture.installed,
            "in_corridor": capture.in_corridor,
            "next_step": capture.next_step,
            "landmass_state": format!("{landmass_state:?}"),
            "reach_distance": reach_distance,
            "using_animation_link": using_animation_link,
            "reached_animation_link": reached_animation_link,
            "door_link_state": format!("{door_link_state:?}"),
            "active_link": active_link.map(|link| format!("{link:?}")),
            "merge_traversal": merge_traversal,
            "pause_agent": pause_agent,
            "desired_velocity": [
                desired_velocity.x,
                desired_velocity.y,
                desired_velocity.z,
            ],
        }),
        vec![line],
    ))
}

pub(crate) fn agent_status(
    world: &mut World,
    rest: &[String],
) -> Result<ConsoleCommandResult, ConsoleError> {
    let index = match rest {
        [] => 0,
        [index] => parse_agent_index(index)?,
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna status accepts at most one agent index",
            ));
        }
    };
    let Some(agent_entity) = world.resource::<DebugAgentRoster>().get(index) else {
        // Issue #134: a handed-off or frozen agent has no live entity but
        // still exists in the ledger -- report that instead of the "no
        // agent" error `tna spawn` would otherwise imply is needed.
        if let Some(entry) = world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(agent_ledger_id(index))
        {
            let line = format!(
                "nav agent {index} handed off to cell {:08x}",
                entry.cell_form_id
            );
            return Ok(ConsoleCommandResult::new(
                json!({ "index": index, "status": "handed-off", "cell": entry.cell_form_id }),
                vec![line],
            ));
        }
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned at this index; use tna spawn first",
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
    let (door_link_state, link_desc) = match world.get::<AgentRuntime>(agent_entity) {
        Some(runtime) => (runtime.door_link, active_link_description(runtime)),
        None => (door_link::DoorLinkState::default(), None),
    };
    let (grounded, stuck, collision_blocked) = world
        .get::<AgentKcc>(agent_entity)
        .map(|kcc| (kcc.grounded, kcc.stuck, kcc.collision_blocked))
        .unwrap_or_default();
    let status = resolve_status(landmass_state, door_link_state);
    let target_desc = world
        .get::<AgentTarget3d>(agent_entity)
        .map(describe_target)
        .unwrap_or_else(|| "none".to_string());
    let merge_traversal = world.get::<MergeTraversal>(agent_entity).map(|traversal| {
        json!({
            "source": [traversal.source.x, traversal.source.y, traversal.source.z],
            "target": [traversal.target.x, traversal.target.y, traversal.target.z],
            "crossing_started": traversal.crossing_started,
            "reached_distance": traversal.reached_distance,
            "elapsed": traversal.elapsed,
            "timeout": traversal.timeout,
            "link_kind": traversal.link_kind,
        })
    });
    let mut line = format!(
        "nav agent {index} status={} position=({:.2},{:.2},{:.2}) target={} grounded={grounded} stuck={stuck} blocked={collision_blocked}",
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
            "index": index,
            "status": status.as_str(),
            "position": [position.x, position.y, position.z],
            "target": target_desc,
            "link": link_desc,
            "grounded": grounded,
            "stuck": stuck,
            "blocked": collision_blocked,
            "merge_traversal": merge_traversal,
        }),
        vec![line],
    ))
}

pub(crate) fn despawn_agent(
    world: &mut World,
    rest: &[String],
) -> Result<ConsoleCommandResult, ConsoleError> {
    let index = match rest {
        [] => 0,
        [index] => parse_agent_index(index)?,
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tna despawn accepts at most one agent index",
            ));
        }
    };
    let Some(agent_entity) = world.resource::<DebugAgentRoster>().get(index) else {
        return Err(ConsoleError::new(
            "no_agent",
            "no test nav agent is spawned at this index; use tna spawn first",
        ));
    };
    // Issue #188: a bound actor is *released*, never despawned. The capsule
    // exists only to be an agent, so despawning it is right; a projected NPC
    // exists independently of navigation and the world/actor slice owns its
    // lifetime.
    let bound = world
        .get::<actor_binding::NavBoundActor>(agent_entity)
        .is_some();
    if bound {
        release_bound_actor(world, agent_entity);
    } else if let Ok(entity) = world.get_entity_mut(agent_entity) {
        entity.despawn();
    }
    world.resource_mut::<DebugAgentRoster>().set(index, None);
    let verb = if bound { "released" } else { "despawned" };
    Ok(ConsoleCommandResult::new(
        json!({ "index": index, "despawned": true, "bound_actor": bound }),
        vec![format!("nav agent {index} {verb}")],
    ))
}

/// Visible exterior navigation diagnostics. This deliberately reports the
/// prepared tile and border evidence without forcing a landmass build, so it
/// is useful before `tna spawn` and remains safe when a package has no NAVM.
pub(crate) fn exterior_command(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let manifest = world
        .get_resource::<crate::viewer::LoadedSceneManifest>()
        .ok_or_else(|| ConsoleError::new("unavailable", "no prepared scene is loaded"))?;
    let package = manifest
        .exterior
        .as_ref()
        .ok_or_else(|| ConsoleError::new("not_exterior", "active scene is not exterior"))?;
    let navigation = package.navigation.as_ref().ok_or_else(|| {
        ConsoleError::new("no_exterior_nav", "exterior package has no navigation tile")
    })?;
    match invocation.args.as_slice() {
        [command] if command == "exterior" => Ok(ConsoleCommandResult::value(json!({
            "cell_form_id": package.cell_form_id,
            "grid": [package.grid.x, package.grid.y],
            "vertices": navigation.vertices.len(),
            "triangles": navigation.triangles.len(),
            "border_portals": navigation.border_portals.len(),
            "revision": navigation.revision.as_str(),
        }))),
        [command] if command == "borders" => Ok(ConsoleCommandResult::value(json!({
            "cell_form_id": package.cell_form_id,
            "portals": &navigation.border_portals,
        }))),
        _ => Err(ConsoleError::new(
            "bad_args",
            "nav expects exterior or borders",
        )),
    }
}
