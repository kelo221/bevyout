use bevy::prelude::*;

use crate::viewer::interaction;
use crate::viewer::nav::agent::AgentRefusesDoors;
use crate::viewer::nav::doors::access::door_usable_now;
use crate::viewer::nav::world::state::NavArchipelagoState;

pub(crate) fn request_door_open(world: &mut World, agent_entity: Entity, door_form_id: u32) {
    // OpenMW parity (`AiWander::execute` never calls `openDoors()`,
    // `if getTypeId() == TypeIdWander return;`): a wander/sandbox package does
    // not open doors. #185 deliberately left this gate for when the package
    // family became available (#198). A sandboxing actor treats a closed door
    // as a wall and roams elsewhere. This is the minimal hook -- gating the one
    // door-open request site rather than threading the package type through the
    // whole door-link state machine; the pause/wait cycle still runs and
    // terminates at the documented `Failed` bound, it just never opens the door.
    if agent_family_refuses_doors(world, agent_entity) {
        info!("nav agent door {door_form_id:08x}: wander package does not open doors");
        return;
    }
    let lock_info = world
        .resource::<NavArchipelagoState>()
        .door_lock_info
        .clone();
    if !door_usable_now(world, Some(agent_entity), door_form_id, &lock_info) {
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

/// Whether the active AI package on `agent_entity` refuses to open doors --
/// true only when the AI slice has flagged this agent with [`AgentRefusesDoors`]
/// (its Sandbox/Wander family; see `PackageFamily::opens_doors`). Reads a
/// nav-owned marker the AI slice sets, not an AI type -- so nav does not depend
/// on `ai`. Actors with no running package (the common `tna`-driven agent) are
/// unflagged and open doors as before.
pub(crate) fn agent_family_refuses_doors(world: &World, agent_entity: Entity) -> bool {
    world.get::<AgentRefusesDoors>(agent_entity).is_some()
}
