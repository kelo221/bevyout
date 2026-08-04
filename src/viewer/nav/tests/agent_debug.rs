use std::collections::HashSet;

use super::*;
use crate::console::{ConsoleError, ConsoleInvocation, ConsoleSessionId};
use crate::viewer::nav::world::links::*;
use crate::viewer::nav::world::portals::*;
use crate::vsa::{PreparedNavGraph, PreparedNavMesh, PreparedNavPolygon};
use bevy::ecs::system::SystemState;
use bevy_boxddd::boxddd::{BodyDef, BodyType, BoxHull, Filter, ShapeDef};
use bevy_landmass::prelude::*;
use bevyout_core::manifest::exterior::ExteriorBorderPortal;

use super::tests_support::*;

#[test]
fn fall_guard_releases_a_bound_actor_without_a_debug_roster_slot() {
    let mut world = World::new();
    world.init_resource::<DebugAgentRoster>();
    world.insert_resource(NavCellFallBounds { min_y: Some(0.0) });
    let actor = world
        .spawn((
            NavAgent,
            actor_binding::NavBoundActor::default(),
            AgentKcc::default(),
            Transform::from_xyz(0.0, -100.0, 0.0),
        ))
        .id();

    assert!(
        world
            .resource::<DebugAgentRoster>()
            .index_of(actor)
            .is_none()
    );
    nav_fall_guard_system(&mut world);

    assert!(
        world.get_entity(actor).is_ok(),
        "the actor itself remains owned by the world slice"
    );
    assert!(
        !is_nav_bound(&world, actor),
        "the runaway nav agent was released"
    );
}

#[test]
fn the_visual_capsule_is_centred_on_the_agent_parent_not_raised_above_it() {
    let mut world = World::new();
    world.init_resource::<Assets<Mesh>>();
    world.init_resource::<Assets<StandardMaterial>>();
    world.init_resource::<NavArchipelagoState>();
    let archipelago_entity = world.spawn_empty().id();
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago_entity);

    // A parent position with a nonzero, non-round Y so an accidental
    // absolute (rather than relative-to-parent) offset would also be
    // caught.
    let parent_position = Vec3::new(1.0, 2.0, 3.0);
    let agent = spawn_test_agent(&mut world, parent_position);

    let children = world
        .get::<Children>(agent)
        .expect("spawn_test_agent adds exactly one visual child");
    assert_eq!(children.len(), 1, "exactly one visual child");
    let visual = children[0];

    // A zero local offset is exactly the "world Y equals the parent's
    // world Y" statement for a child with no rotation/scale on the
    // parent (`spawn_test_agent`'s agent entity carries neither) --
    // asserted directly on the local `Transform` rather than via
    // `GlobalTransform`, which this bare `World` never propagates.
    let visual_local = world.get::<Transform>(visual).unwrap();
    assert_eq!(
        visual_local.translation,
        Vec3::ZERO,
        "the visual child must be centred on the agent parent (zero local offset) -- \
             the parent transform is already the capsule centre post-#114, not feet level"
    );
}

#[test]
fn debug_agent_animation_links_allow_its_capsule_center_offset() {
    let mut world = World::new();
    world.init_resource::<Assets<Mesh>>();
    world.init_resource::<Assets<StandardMaterial>>();
    world.init_resource::<NavArchipelagoState>();
    let archipelago_entity = world.spawn_empty().id();
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago_entity);

    let agent = spawn_test_agent(&mut world, Vec3::new(1.0, 2.0, 3.0));
    let reached_distance = world
        .get::<AnimationLinkReachedDistance>(agent)
        .expect("debug agent carries an explicit link reach distance")
        .0;
    assert_eq!(
        reached_distance,
        AGENT_HEIGHT * 0.5 + AGENT_RADIUS,
        "landmass measures animation-link arrival in full 3D while the debug capsule transform is centre-level"
    );
}

#[test]
fn no_args_prints_usage_without_erroring() {
    let mut world = harness_world();
    let result = tna_command(&mut world, &invocation(&[])).expect("usage is not an error");
    assert_eq!(result.log.len(), 1);
    assert!(result.log[0].starts_with("usage:"));
}

#[test]
fn unknown_subcommand_is_an_error() {
    let mut world = harness_world();
    let error = tna_command(&mut world, &invocation(&["dance"])).unwrap_err();
    assert_eq!(error.code, "unknown_subcommand");
}

#[test]
fn spawn_without_a_nav_graph_reuses_the_no_nav_graph_wording() {
    let mut world = harness_world();
    let error = tna_command(&mut world, &invocation(&["spawn"])).unwrap_err();
    assert_eq!(error.code, "no_nav_graph");
    assert_eq!(error.message, "no nav graph prepared for this cell");
}

#[test]
fn goto_without_a_spawned_agent_is_an_error() {
    let mut world = harness_world();
    let error = tna_command(&mut world, &invocation(&["goto", "1", "2", "3"])).unwrap_err();
    assert_eq!(error.code, "no_agent");
}

#[test]
fn goto_bad_arity_is_rejected() {
    let mut world = harness_world();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(Entity::PLACEHOLDER);
    let error = tna_command(&mut world, &invocation(&["goto", "1", "2"])).unwrap_err();
    assert_eq!(error.code, "bad_arity");
}

#[test]
fn status_without_a_spawned_agent_is_an_error() {
    let mut world = harness_world();
    let error = tna_command(&mut world, &invocation(&["status"])).unwrap_err();
    assert_eq!(error.code, "no_agent");
}

#[test]
fn despawn_without_a_spawned_agent_is_an_error() {
    let mut world = harness_world();
    let error = tna_command(&mut world, &invocation(&["despawn"])).unwrap_err();
    assert_eq!(error.code, "no_agent");
}

#[test]
fn despawn_round_trip_clears_state() {
    let mut world = harness_world();
    let entity = world.spawn(NavAgent).id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(entity);
    let result = tna_command(&mut world, &invocation(&["despawn"])).expect("despawn succeeds");
    assert_eq!(result.log, ["nav agent 0 despawned"]);
    assert!(world.resource::<DebugAgentRoster>().entities[0].is_none());
    assert!(world.get_entity(entity).is_err());
}

#[test]
fn spawn_indices_are_independent_and_grow_past_the_old_cap() {
    let mut world = harness_world();
    // Pre-seed the archipelago as already current so `ensure_archipelago`
    // (which `spawn_agent` always calls first, same as wave 3/4) returns
    // immediately without needing a real manifest/nav-graph file --
    // this test is about the index/occupancy contract, not archipelago
    // building.
    let mut manifest = minimal_manifest(0xBEEF);
    manifest.nav_graph = Some(crate::vsa::PreparedNavGraphSource::default());
    world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));
    world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xBEEF);
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(world.spawn_empty().id());
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(Entity::PLACEHOLDER);

    let error = tna_command(&mut world, &invocation(&["spawn", "0"])).unwrap_err();
    assert_eq!(error.code, "already_spawned");

    // A different index is an independent slot -- it gets past the
    // occupancy check to the next requirement (a live FPS player),
    // proving index 0's occupancy did not block it.
    let error = tna_command(&mut world, &invocation(&["spawn", "1"])).unwrap_err();
    assert_eq!(error.code, "player_unavailable");

    let error = tna_command(&mut world, &invocation(&["spawn", "41"])).unwrap_err();
    assert_eq!(error.code, "player_unavailable");

    let out_of_range = (MAX_AGENT_INDEX + 1).to_string();
    let error = tna_command(&mut world, &invocation(&["spawn", &out_of_range])).unwrap_err();
    assert_eq!(error.code, "bad_agent_index");
}

#[test]
fn indexed_goto_addresses_only_the_named_agent_slot() {
    let mut world = harness_world();
    world.init_resource::<Time>();
    let agent0 = world
        .spawn((NavAgent, AgentRuntime::default(), AgentKcc::default()))
        .id();
    let agent1 = world
        .spawn((NavAgent, AgentRuntime::default(), AgentKcc::default()))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent0);
    world.resource_mut::<DebugAgentRoster>().entities[1] = Some(agent1);

    tna_command(&mut world, &invocation(&["goto", "1", "5", "6", "7"]))
        .expect("indexed goto succeeds");

    assert!(
        matches!(
            world.get::<AgentTarget3d>(agent1),
            Some(AgentTarget3d::Point(point)) if *point == Vec3::new(5.0, 6.0, 7.0)
        ),
        "agent 1 got the target"
    );
    assert!(
        world.get::<AgentTarget3d>(agent0).is_none(),
        "agent 0 is untouched by an indexed goto for a different agent"
    );
}
