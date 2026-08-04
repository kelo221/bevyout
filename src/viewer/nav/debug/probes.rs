//! Debug path, animation-link, and solver diagnostics.

use std::collections::HashMap;
use std::fmt::Debug;

use bevy::prelude::*;
use bevy_landmass::coords::ThreeD;
use bevy_landmass::prelude::*;
use bevy_landmass::{
    AnimationLinkReachedDistance, PauseAgent, PermittedAnimationLinks, UsingAnimationLink,
};
use serde_json::json;

use crate::console::{ConsoleCommandResult, ConsoleError};
use crate::viewer::nav::agent::{
    AgentKcc, AgentRuntime, MergeTraversal, NavAgentLedger, NavSolveRate, active_link_description,
    agent_ledger_id, describe_target, resolve_status,
};
use crate::viewer::nav::debug::DebugAgentRoster;
use crate::viewer::nav::door_link;
use crate::viewer::nav::world::state::NavArchipelagoState;

use super::actions::parse_goto_point;
use super::command::parse_agent_index;

pub(crate) fn format_path_steps<I>(steps: I) -> Vec<String>
where
    I: IntoIterator,
    I::Item: Debug,
{
    steps.into_iter().map(|step| format!("{step:?}")).collect()
}

/// Captures only the link-specific facts emitted by Landmass's debug drawer;
/// command dispatch remains in `command.rs`.
pub(crate) struct AnimationLinkDebugCapture {
    pub(crate) link_entity: Entity,
    pub(crate) agent_entity: Entity,
    pub(crate) installed: bool,
    pub(crate) in_corridor: bool,
    pub(crate) next_step: bool,
}

impl bevy_landmass::debug::DebugDrawer<ThreeD> for AnimationLinkDebugCapture {
    fn add_point(&mut self, _point_type: bevy_landmass::debug::PointType, _point: Vec3) {}

    fn add_line(&mut self, line_type: bevy_landmass::debug::LineType, _line: [Vec3; 2]) {
        match line_type {
            bevy_landmass::debug::LineType::AnimationLinkConnection(link)
                if link == self.link_entity =>
            {
                self.installed = true;
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

/// `tna solverate [<n>]`: report or update the fixed-step Landmass solve
/// interval. This is a runtime diagnostic, so its parser lives with the
/// other probes rather than with command dispatch.
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
    let steps = format_path_steps(path.iter());
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
            world.get::<AnimationLink3d>(entity).is_some_and(|link| {
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
            "desired_velocity": [desired_velocity.x, desired_velocity.y, desired_velocity.z],
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
