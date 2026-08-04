//! Travel-door request and handoff intent setup.

use bevy::prelude::*;
use bevy_landmass::prelude::AgentTarget3d;

use crate::viewer::nav::agent::*;
use crate::viewer::nav::api;
use crate::viewer::nav::doors::access::*;
use crate::viewer::nav::traversal::landmass_sync::clear_merge_link_quarantine;

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
    let result = request_travel_for_actor(world, agent_entity, door_form_id);
    if result.is_ok() {
        info!("nav agent {index} travel start {door_form_id:08x}");
    }
    result
}

/// Routes an already-resolved actor through a travel door. The console-facing
/// indexed wrapper above and the navigation façade both use this same state
/// transition so they cannot diverge.
pub(crate) fn request_travel_for_actor(
    world: &mut World,
    agent_entity: Entity,
    door_form_id: u32,
) -> Result<(), api::NavError> {
    if world
        .get::<AgentRuntime>(agent_entity)
        .is_some_and(|runtime| runtime.travel_intent.is_some())
    {
        return Err(api::NavError::new(
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
        return Err(api::NavError::new(
            "unknown_travel_door",
            format!("no travel door {door_form_id:08x} is reachable from this cell's nav mesh"),
        ));
    };
    world
        .entity_mut(agent_entity)
        .insert(AgentTarget3d::Point(link.triangle_midpoint));
    // Issue #162 feature 2: a fresh travel request is a new routing intent
    // -- any merge-portal quarantine from a previous route no longer
    // applies.
    clear_merge_link_quarantine(world, agent_entity);
    // Issue #185: same rationale as `goto_agent` -- re-evaluate this
    // agent's key-aware door-lock overrides for the fresh travel intent.
    apply_door_lock_overrides(world, agent_entity);
    if let Some(mut runtime) = world.get_mut::<AgentRuntime>(agent_entity) {
        runtime.travel_intent = Some(door_form_id);
    }
    Ok(())
}
