//! `tna` command parsing and dispatch.

use bevy::prelude::*;
use serde_json::json;

use crate::console::{ConsoleCommandResult, ConsoleError, ConsoleInvocation};
use crate::viewer::nav::agent::MAX_AGENT_INDEX;
use crate::viewer::nav::api;

use super::actions::{bind_agent, despawn_agent, goto_agent, spawn_agent, travel_agent};
use super::probes::{agent_status, animation_link_probe, path_probe, solve_rate_command};

pub(crate) fn console_error_from_nav(error: api::NavError) -> ConsoleError {
    ConsoleError::new(error.code(), error.message())
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
