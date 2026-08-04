//! Travel-door request and handoff intent setup.

use bevy::prelude::*;

use crate::viewer::nav::agent::AgentRuntime;
use crate::viewer::nav::api;
use crate::viewer::nav::debug::DebugAgentRoster;
use crate::viewer::nav::world::state::{NavArchipelagoState, TravelDoorLink};

/// Routes agent `index` to `door_form_id`'s travel-door triangle and arms
/// the travel lifecycle (issue #113 feature 3; indexed #114). The traversal
/// terminates at the door with the `TravelReached` status;
/// `door_traversal_system` (issue #134) consumes that seam and hands the
/// agent off to the destination cell. Wired to the console as `tna travel
/// [<index>] <door-formid>`.
pub(crate) fn request_travel(
    world: &mut World,
    index: usize,
    door_form_id: u32,
) -> Result<(), api::NavError> {
    let Some(agent_entity) = world.resource::<DebugAgentRoster>().get(index) else {
        return Err(api::NavError::new(
            "no_agent",
            "no test nav agent is spawned at this index; use tna spawn first",
        ));
    };
    if world
        .get::<AgentRuntime>(agent_entity)
        .is_some_and(|runtime| runtime.travel_intent.is_some())
    {
        return Err(api::NavError::GoalBusy);
    }
    let result = api::set_goal(
        world,
        agent_entity,
        api::NavGoal::TravelDoor(door_form_id.into()),
    );
    if result.is_ok() {
        info!("nav agent {index} travel start {door_form_id:08x}");
    }
    result
}

/// Resolves a travel door without changing actor state. Goal replacement owns
/// cancellation and common route initialization; keeping resolution pure makes
/// validation safe before the old route is invalidated.
pub(crate) fn resolve_travel_door(
    world: &World,
    door_form_id: u32,
) -> Result<TravelDoorLink, api::NavError> {
    world
        .get_resource::<NavArchipelagoState>()
        .ok_or(api::NavError::WorldUnavailable)?
        .travel_doors
        .get(&door_form_id)
        .copied()
        .ok_or_else(|| api::NavError::DoorUnavailable(door_form_id.into()))
}
