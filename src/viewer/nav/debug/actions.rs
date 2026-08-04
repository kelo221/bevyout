//! Runtime operations selected by the `tna` command dispatcher.

use bevy::prelude::*;
use bevy_landmass::prelude::*;
use serde_json::json;

use crate::console::{ConsoleCommandResult, ConsoleError, ConsoleInvocation};
use crate::viewer::nav::agent::actor_binding;
use crate::viewer::nav::agent::{
    DebugNavAgent, actor_entity_by_reference, bind_agent_entity, release_bound_actor,
    route_agent_to_target,
};
use crate::viewer::nav::debug::player::*;
use crate::viewer::nav::debug::{DebugAgentOrigin, DebugAgentRoster};
use crate::viewer::nav::doors::travel::request_travel;
use crate::viewer::nav::world::build::ensure_archipelago;

use super::capsule::despawn_debug_capsule;
use super::capsule::spawn_debug_capsule as spawn_test_agent;
use super::command::{console_error_from_nav, parse_agent_index};

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
    world.resource_mut::<DebugAgentRoster>().set_with_origin(
        index,
        Some(entity),
        DebugAgentOrigin::BoundActor { reference_form_id },
    );
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

pub(crate) fn parse_form_id(value: &str) -> Option<u32> {
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    ((1..=8).contains(&digits.len()) && digits.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| u32::from_str_radix(digits, 16).ok())
        .flatten()
}

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
    route_agent_to_target(world, agent_entity, target).map_err(console_error_from_nav)?;
    Ok(ConsoleCommandResult::new(
        json!({ "index": index, "target": description }),
        vec![format!("nav agent {index} target set to {description}")],
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
    let bound = matches!(
        world.resource::<DebugAgentRoster>().origin_of(agent_entity),
        Some(DebugAgentOrigin::BoundActor { .. })
    ) || world
        .get::<actor_binding::NavBoundActor>(agent_entity)
        .is_some();
    if bound {
        release_bound_actor(world, agent_entity);
    } else {
        despawn_debug_capsule(world, agent_entity);
    }
    world.resource_mut::<DebugAgentRoster>().set(index, None);
    let verb = if bound { "released" } else { "despawned" };
    Ok(ConsoleCommandResult::new(
        json!({ "index": index, "despawned": true, "bound_actor": bound }),
        vec![format!("nav agent {index} {verb}")],
    ))
}

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
