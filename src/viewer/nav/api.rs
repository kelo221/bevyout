//! Narrow navigation contract shared by AI, world interaction, and console
//! adapters.
//!
//! The implementation is composed from the capability modules under
//! `agent/`, `doors/`, and `handoff/`. Keeping this boundary in one place
//! prevents new callers from depending on Landmass components or the
//! debug-agent implementation.

use bevy::prelude::{Entity, Vec3, World};
use bevyout_core::form_id::FormId;

use super::agent;
use super::doors::access::{
    set_door_key_form_id as door_set_door_key_form_id,
    set_door_lock_level as door_set_door_lock_level,
};
use super::handoff::note_player_swap_door;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NavError {
    ActorUnavailable(Entity),
    ActorNotBound(Entity),
    TargetUnavailable(Entity),
    WorldUnavailable,
    GoalBusy,
    DoorUnavailable(FormId),
    Custom { code: String, message: String },
}

impl NavError {
    pub(crate) fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Custom {
            code: code.into(),
            message: message.into(),
        }
    }

    pub(crate) fn code(&self) -> String {
        match self {
            Self::ActorUnavailable(_) => "actor_unavailable".to_string(),
            Self::ActorNotBound(_) => "actor_not_bound".to_string(),
            Self::TargetUnavailable(_) => "target_unavailable".to_string(),
            Self::WorldUnavailable => "world_unavailable".to_string(),
            Self::GoalBusy => "travel_in_progress".to_string(),
            Self::DoorUnavailable(_) => "unknown_travel_door".to_string(),
            Self::Custom { code, .. } => code.clone(),
        }
    }

    pub(crate) fn message(&self) -> String {
        match self {
            Self::ActorUnavailable(actor) => format!("navigation actor {actor:?} no longer exists"),
            Self::ActorNotBound(actor) => format!("navigation actor {actor:?} is not fully bound"),
            Self::TargetUnavailable(target) => {
                format!("navigation target {target:?} no longer exists")
            }
            Self::WorldUnavailable => "navigation world is unavailable".to_string(),
            Self::GoalBusy => "navigation goal is already in progress".to_string(),
            Self::DoorUnavailable(door) => format!("travel door {door:?} is unavailable"),
            Self::Custom { message, .. } => message.clone(),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum NavGoal {
    Point(Vec3),
    Entity(Entity),
    TravelDoor(FormId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DoorUsePolicy {
    MayOpen,
    RefusesDoors,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NavFailureReason {
    NoPath,
    AgentOffNavmesh,
    TargetOffNavmesh,
    BlockedDoor(FormId),
    WorldUnavailable,
    HandoffFailed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NavStatus {
    Idle,
    Routing,
    WaitingForDoor(FormId),
    TraversingDoor(FormId),
    TravelReady(FormId),
    Reached,
    Failed(NavFailureReason),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct NavObservation {
    pub(crate) status: NavStatus,
    pub(crate) generation: agent::RouteGeneration,
}

impl NavObservation {
    pub(crate) fn is_reached(self) -> bool {
        matches!(self.status, NavStatus::Reached)
    }

    pub(crate) fn is_failed(self) -> bool {
        matches!(self.status, NavStatus::Failed(_))
    }

    pub(crate) fn blocking_door(self) -> Option<u32> {
        match self.status {
            NavStatus::Failed(NavFailureReason::BlockedDoor(form_id)) => Some(form_id.into()),
            _ => None,
        }
    }
}

pub(crate) fn bind_actor(world: &mut World, actor: Entity) -> Result<(), NavError> {
    agent::bind_agent_entity(world, actor)
}

pub(crate) fn release_actor(world: &mut World, actor: Entity) {
    agent::release_bound_actor(world, actor);
}

pub(crate) fn set_goal(world: &mut World, actor: Entity, goal: NavGoal) -> Result<(), NavError> {
    agent::replace_goal(world, actor, Some(goal))?;
    Ok(())
}

pub(crate) fn cancel_goal(world: &mut World, actor: Entity) -> Result<(), NavError> {
    agent::replace_goal(world, actor, None)?;
    Ok(())
}

pub(crate) fn observation(world: &World, actor: Entity) -> NavObservation {
    agent::nav_observation(world, actor)
}

pub(crate) fn set_door_policy(world: &mut World, actor: Entity, policy: DoorUsePolicy) {
    agent::set_agent_refuses_doors(world, actor, matches!(policy, DoorUsePolicy::RefusesDoors));
}

pub(crate) fn set_pose_authority(world: &mut World, actor: Entity, pose_authored: bool) {
    agent::set_facing_authority(world, actor, pose_authored);
}

#[allow(dead_code)]
pub(crate) fn note_player_door_transition(world: &mut World, door: FormId) {
    note_player_swap_door(world, door.into());
}

pub(crate) fn set_door_lock_level(world: &mut World, door: FormId, lock_level: Option<i8>) {
    door_set_door_lock_level(world, door.into(), lock_level);
}

pub(crate) fn set_door_key_form_id(world: &mut World, door: FormId, key_form_id: Option<FormId>) {
    door_set_door_key_form_id(world, door.into(), key_form_id.map(Into::into));
}

pub(crate) fn is_bound(world: &World, actor: Entity) -> bool {
    agent::is_nav_bound(world, actor)
}

pub(crate) fn register_release_hook(
    world: &mut World,
    hook: impl Fn(&mut World, Entity) + Send + Sync + 'static,
) {
    agent::register_bound_actor_release_hook(world, hook);
}

#[cfg(test)]
pub(crate) fn insert_test_archipelago_state(world: &mut World) {
    agent::insert_test_archipelago_state(world);
}

#[cfg(test)]
pub(crate) fn mark_test_archipelago_current(
    world: &mut World,
    cell_form_id: u32,
    archipelago: Entity,
) {
    agent::mark_test_archipelago_current(world, cell_form_id, archipelago);
}

#[cfg(test)]
pub(crate) fn insert_test_nav_agent(world: &mut World, actor: Entity) {
    world.entity_mut(actor).insert((
        agent::NavAgent,
        agent::AgentRuntime::default(),
        agent::AgentKcc::default(),
    ));
}

#[cfg(test)]
#[path = "tests/api.rs"]
mod tests;
