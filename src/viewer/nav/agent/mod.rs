//! Navigation agent composition root. Capability implementations live in
//! the sibling modules below; this file owns only shared imports, wiring, and
//! narrow re-exports.

#![allow(unused_imports)]

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;

use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_boxddd::boxddd;
use bevy_boxddd::prelude::BoxdddPhysicsContext;
use bevy_landmass::coords::ThreeD;
use bevy_landmass::prelude::*;
use bevy_landmass::{
    AgentTypeIndexCostOverrides, AnimationLinkReachedDistance, NavMeshHandle, PauseAgent,
    PermittedAnimationLinks, PointSampleDistance3d, TargetReachedCondition, UsingAnimationLink,
};
use bevyout_core::manifest::exterior::{
    ExteriorCellLifecycle, ExteriorCellPackage, GridCoordinate, matching_portals,
};
use serde_json::json;

use crate::viewer::actor::ActorRuntime;
use crate::vsa::PreparedNavGraph;
#[cfg(test)]
use crate::vsa::PreparedSceneManifest;

use super::super::openmw_player::GRAVITY;
use super::super::player::{CellPhysicsReadiness, PhysicsDisabled};
use super::super::world::exterior::ExteriorStreamState;
use super::super::{interaction, player};
use super::{api, door_link, landmass_graph, ledger_policy, movement_policy, openmw_doors, repath};

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
pub(crate) use self::components::*;
pub(crate) use self::fall_guard_runtime::nav_fall_guard_system;
pub(crate) use self::lifecycle::*;
pub(crate) use self::movement::*;
pub(crate) use self::routing::*;
pub(crate) use super::NavBackendPlugin;
pub(crate) use super::debug::*;
pub(crate) use super::diagnostics::*;
pub(crate) use super::doors::*;
pub(crate) use super::handoff::*;
pub(crate) use super::traversal::*;
pub(crate) use super::world::build::*;
pub(crate) use super::world::links::*;
pub(crate) use super::world::player_obstacle::*;
pub(crate) use super::world::portals::*;
pub(crate) use super::world::state::*;

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
