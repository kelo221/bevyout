//! Navigation agent composition root. Capability implementations live in
//! the sibling modules below; this file owns only shared imports, wiring, and
//! narrow re-exports.

use std::collections::BTreeSet;

use crate::viewer::actor::ActorRuntime;
#[cfg(test)]
use crate::vsa::PreparedSceneManifest;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_boxddd::boxddd;
use bevy_boxddd::prelude::BoxdddPhysicsContext;
use bevy_landmass::coords::ThreeD;
use bevy_landmass::prelude::*;
use bevy_landmass::{
    AgentTypeIndexCostOverrides, PauseAgent, PointSampleDistance3d, TargetReachedCondition,
    UsingAnimationLink,
};
#[cfg(test)]
use bevy_landmass::{AnimationLinkReachedDistance, NavMeshHandle, PermittedAnimationLinks};

#[cfg(test)]
use super::super::interaction;
use super::super::openmw_player::GRAVITY;
use super::super::player;
use super::super::player::{CellPhysicsReadiness, PhysicsDisabled};
use super::{api, door_link, landmass_graph, ledger_policy, movement_policy};

#[cfg(test)]
use std::collections::{BTreeMap, HashMap};
#[cfg(test)]
use std::sync::Arc;
pub(crate) mod actor_binding;
mod components;
pub(crate) mod fall_guard;
mod fall_guard_runtime;
mod lifecycle;
mod locomotion;
mod movement;
mod routing;

pub(crate) use self::actor_binding::{
    actor_entity_by_reference, drive_bound_actor_locomotion, face_bound_actors,
};
pub(crate) use self::components::{
    AGENT_DESIRED_SPEED, AGENT_HEIGHT, AGENT_MAX_SPEED, AGENT_RADIUS,
    AGENT_TARGET_REACHED_DISTANCE, AgentDesiredVelocityBlend, AgentKcc, AgentRuntime,
    AgentTargetSnapshot, CLOSED_DOOR_TYPE_INDEX_COST, DOOR_TRAVERSAL_SECONDS, DebugNavAgent,
    DoorTraversal, INITIAL_AGENT_SLOTS, LOCKED_DOOR_TYPE_INDEX_COST, MAX_AGENT_INDEX,
    MERGE_LINK_SWEEP_TOLERANCE, MERGE_SOURCE_ALIGNMENT_DISTANCE, MERGE_TRAVERSAL_REACHED_DISTANCE,
    MERGE_TRAVERSAL_TIMEOUT_FACTOR, MERGE_TRAVERSAL_TIMEOUT_FLOOR_SECONDS, MergeTraversal,
    NavAgent, NavAgentLedger, NavCellFallBounds, NavSolveRate, NavSolveStepCounter,
    PREFERRED_PATHING_TYPE_INDEX_COST, PendingMergeRepath, PendingPlayerSwapDoor,
    RefreshLandmassAnimationLinkInput, RouteGeneration, SuspendedLandmassTypeIndexCosts,
    TRAVEL_ARRIVAL_DISTANCE, TravelIntent, WALKABLE_CONTACT_NORMAL_Y, agent_ledger_id,
    archipelago_options,
};
#[cfg(test)]
pub(crate) use self::components::{AGENT_POINT_SAMPLE_DISTANCE, NAV_BORDER_AVOIDANCE_TIME_HORIZON};
pub(crate) use self::fall_guard_runtime::nav_fall_guard_system;
pub(crate) use self::lifecycle::{
    AgentRefusesDoors, advance_nav_solve_step_counter, agent_components, bind_agent_entity,
    nav_solve_gate, register_bound_actor_release_hook, release_bound_actor,
    remove_agent_components, run_bound_actor_release_hooks, set_agent_refuses_doors,
    set_facing_authority, update_agent_desired_velocity_blend,
};
pub(crate) use self::movement::{
    apply_agent_physics_movement, step_agent_kcc, world_contact_report, write_agent_translation,
};
pub(crate) use self::routing::{
    active_link_description, describe_target, is_nav_bound, nav_observation, replace_goal,
    resolve_status, route_agent_to_target,
};
#[cfg(test)]
pub(crate) use self::routing::{insert_test_archipelago_state, mark_test_archipelago_current};
#[cfg(test)]
pub(crate) use super::debug::{DebugAgentRoster, parse_agent_index, spawn_test_agent, tna_command};
#[cfg(test)]
pub(crate) use super::diagnostics::{
    HUD_AGENT_LINE_LIMIT, HudAgentProjection, HudAgentRow, hud_agent_lines_from_rows,
};
#[cfg(test)]
pub(crate) use super::doors::access::{
    apply_door_lock_overrides, door_lock_level_for_test, init_test_archipelago_state,
    set_door_lock_level,
};
#[cfg(test)]
pub(crate) use super::doors::availability::door_availability_system;
#[cfg(test)]
pub(crate) use super::doors::runtime::agent_family_refuses_doors;
#[cfg(test)]
pub(crate) use super::doors::travel::request_travel;
#[cfg(test)]
pub(crate) use super::doors::traversal::{
    door_link_system, door_traversal_system, drive_door_link_for_agent,
};
#[cfg(test)]
pub(crate) use super::handoff::{
    despawn_stale_navmesh_archipelago, ledger_departing_agent, restore_ledgered_agents_system,
};
#[cfg(test)]
pub(crate) use super::traversal::{
    clear_merge_link_quarantine, merge_traversal_reached_distance, merge_traversal_system,
    merge_traversal_timeout, refresh_landmass_animation_link_input,
    restore_landmass_type_index_costs, resume_pending_merge_repath_system,
};
#[cfg(test)]
pub(crate) use super::world::build::{apply_preferred_pathing_base_cost, ensure_archipelago};
#[cfg(test)]
pub(crate) use super::world::state::{
    BlockedDoorLink, DoorLockInfo, LinkKind, MidRouteDoor, NavArchipelagoState, TravelDoorLink,
};
#[cfg(test)]
#[path = "../tests/agent_debug.rs"]
mod tests_debug;
#[cfg(test)]
#[path = "../tests/agent_diagnostics.rs"]
mod tests_diagnostics;
#[cfg(test)]
#[path = "../tests/agent_doors.rs"]
mod tests_doors;
#[cfg(test)]
#[path = "../tests/agent_handoff.rs"]
mod tests_handoff;
#[cfg(test)]
#[path = "../tests/agent_movement.rs"]
mod tests_movement;
#[cfg(test)]
#[path = "../tests/agent_routing.rs"]
mod tests_routing;
#[cfg(test)]
#[path = "../tests/support.rs"]
mod tests_support;
#[cfg(test)]
#[path = "../tests/agent_traversal.rs"]
mod tests_traversal;
#[cfg(test)]
#[path = "../tests/agent_wedge.rs"]
mod tests_wedge;
#[cfg(test)]
#[path = "../tests/agent_world.rs"]
mod tests_world;

pub(crate) fn install(app: &mut App) {
    app.add_plugins(super::NavBackendPlugin);
}
