use bevy::prelude::SystemSet;

/// Stable ownership/order names for the fixed-step navigation runtime.
///
/// The sets name the ownership and ordering seam between the capability
/// modules under `agent/`, `world/`, `doors/`, `traversal/`, and `handoff/`.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum NavRuntimeSet {
    WorldLifecycle,
    AgentRestore,
    DoorAvailability,
    LinkLifecycle,
    Movement,
    FallGuard,
    Presentation,
    Traversal,
    Diagnostics,
}

use bevy::prelude::*;
use bevy_landmass::{Landmass3dPlugin, LandmassSystems};

use crate::viewer::nav::agent::{
    NavAgentLedger, NavCellFallBounds, NavSolveRate, NavSolveStepCounter, PendingPlayerSwapDoor,
};
use crate::viewer::nav::agent::{
    advance_nav_solve_step_counter, apply_agent_physics_movement, drive_bound_actor_locomotion,
    face_bound_actors, nav_fall_guard_system, nav_solve_gate, update_agent_desired_velocity_blend,
};
use crate::viewer::nav::debug::DebugAgentRoster;
use crate::viewer::nav::diagnostics::logging::{log_agent_state_changes, log_path_latency};
use crate::viewer::nav::doors::availability::door_availability_system;
use crate::viewer::nav::doors::traversal::{door_link_system, door_traversal_system};
use crate::viewer::nav::handoff::{
    despawn_stale_navmesh_archipelago, restore_ledgered_agents_system,
};
use crate::viewer::nav::traversal::landmass_sync::{
    refresh_landmass_animation_link_input, restore_landmass_type_index_costs,
};
use crate::viewer::nav::traversal::{merge_traversal_system, resume_pending_merge_repath_system};
use crate::viewer::nav::world::player_obstacle::sync_player_nav_character;
use crate::viewer::nav::world::state::NavArchipelagoState;

pub(crate) struct NavBackendPlugin;

impl Plugin for NavBackendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Landmass3dPlugin::default())
            .init_resource::<NavArchipelagoState>()
            .init_resource::<DebugAgentRoster>()
            .init_resource::<NavAgentLedger>()
            .init_resource::<PendingPlayerSwapDoor>()
            .init_resource::<NavSolveRate>()
            .init_resource::<NavSolveStepCounter>()
            .init_resource::<NavCellFallBounds>()
            .configure_sets(
                FixedPreUpdate,
                LandmassSystems::Update.run_if(nav_solve_gate),
            )
            .configure_sets(
                FixedUpdate,
                (
                    NavRuntimeSet::WorldLifecycle,
                    NavRuntimeSet::AgentRestore,
                    NavRuntimeSet::DoorAvailability,
                    NavRuntimeSet::LinkLifecycle,
                    NavRuntimeSet::Movement,
                    NavRuntimeSet::FallGuard,
                    NavRuntimeSet::Presentation,
                    NavRuntimeSet::Traversal,
                    NavRuntimeSet::Diagnostics,
                )
                    .chain(),
            )
            .add_systems(
                FixedPreUpdate,
                advance_nav_solve_step_counter.before(LandmassSystems::SyncExistence),
            )
            .add_systems(
                FixedPreUpdate,
                sync_player_nav_character.before(LandmassSystems::SyncValues),
            )
            .add_systems(
                FixedPreUpdate,
                refresh_landmass_animation_link_input
                    .after(LandmassSystems::SyncExistence)
                    .before(LandmassSystems::SyncValues),
            )
            .add_systems(
                FixedPreUpdate,
                restore_landmass_type_index_costs
                    .after(LandmassSystems::SyncValues)
                    .before(LandmassSystems::Update),
            )
            .add_systems(
                FixedPreUpdate,
                update_agent_desired_velocity_blend.after(LandmassSystems::Output),
            )
            .add_systems(
                FixedPreUpdate,
                // Issue #162: gated on the same `nav_solve_gate` condition
                // as `LandmassSystems::Update` itself (not every fixed
                // tick) -- see `resume_pending_merge_repath_system`'s doc
                // comment for why its one-tick-later restore must line up
                // with an actual solve tick, not an arbitrary movement
                // tick, when `NavSolveRate` throttles the solve below the
                // fixed-tick cadence.
                resume_pending_merge_repath_system
                    .after(LandmassSystems::Output)
                    .run_if(nav_solve_gate),
            )
            .add_systems(
                FixedUpdate,
                despawn_stale_navmesh_archipelago.in_set(NavRuntimeSet::WorldLifecycle),
            )
            .add_systems(
                FixedUpdate,
                restore_ledgered_agents_system.in_set(NavRuntimeSet::AgentRestore),
            )
            .add_systems(
                FixedUpdate,
                door_availability_system.in_set(NavRuntimeSet::DoorAvailability),
            )
            .add_systems(
                FixedUpdate,
                door_link_system.in_set(NavRuntimeSet::LinkLifecycle),
            )
            .add_systems(
                FixedUpdate,
                apply_agent_physics_movement.in_set(NavRuntimeSet::Movement),
            )
            .add_systems(
                FixedUpdate,
                nav_fall_guard_system.in_set(NavRuntimeSet::FallGuard),
            )
            .add_systems(
                FixedUpdate,
                (
                    // Issue #188, strictly after the KCC has moved and the
                    // fall guard has had its say: facing reads the desired/
                    // achieved pair this tick's movement just stashed, and
                    // the locomotion request must not be issued for an agent
                    // the guard is about to release.
                    face_bound_actors
                        .in_set(NavRuntimeSet::Presentation)
                        .before(drive_bound_actor_locomotion),
                    drive_bound_actor_locomotion.in_set(NavRuntimeSet::Presentation),
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    merge_traversal_system
                        .in_set(NavRuntimeSet::Traversal)
                        .before(door_traversal_system),
                    door_traversal_system.in_set(NavRuntimeSet::Traversal),
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    log_agent_state_changes
                        .in_set(NavRuntimeSet::Diagnostics)
                        .before(log_path_latency),
                    log_path_latency.in_set(NavRuntimeSet::Diagnostics),
                ),
            );
    }
}
