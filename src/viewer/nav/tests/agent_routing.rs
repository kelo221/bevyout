use super::*;
use crate::viewer::nav::api::{NavGoal, NavStatus};
use bevy_landmass::prelude::AgentTarget3d;

use super::tests_support::*;

fn route_test_world() -> (World, Entity) {
    let mut world = harness_world();
    world.init_resource::<Time>();
    let archipelago = world.spawn_empty().id();
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago);
    let actor = world
        .spawn((NavAgent, AgentRuntime::default(), AgentKcc::default()))
        .id();
    (world, actor)
}

#[test]
fn replacing_a_travel_goal_clears_the_old_door_lifecycle() {
    let (mut world, actor) = route_test_world();
    world
        .resource_mut::<NavArchipelagoState>()
        .travel_doors
        .insert(
            0x99,
            TravelDoorLink {
                triangle_midpoint: Vec3::new(1.0, 0.0, 0.0),
                door_position: Vec3::ZERO,
                destination_cell_form_id: 0xC0DE,
                destination_door_form_id: 0x1234,
            },
        );

    let first = replace_goal(&mut world, actor, Some(NavGoal::TravelDoor(0x99u32.into())))
        .expect("travel route accepted");
    world.entity_mut(actor).insert(PauseAgent);
    {
        let mut runtime = world.get_mut::<AgentRuntime>(actor).unwrap();
        runtime.door_link = door_link::DoorLinkState::Paused {
            door_form_id: 0x99,
            waited_ticks: 4,
            destination: door_link::LinkDestination::Travel {
                destination_cell_form_id: 0xC0DE,
            },
        };
        runtime.pending_traversal = Some((Vec3::ZERO, Vec3::ONE));
        runtime.active_link = Some(LinkKind::Door { form_id: 0x99 });
    }

    let second = replace_goal(
        &mut world,
        actor,
        Some(NavGoal::Point(Vec3::new(4.0, 0.0, 0.0))),
    )
    .expect("replacement route accepted");
    assert_ne!(first, second);
    let runtime = world.get::<AgentRuntime>(actor).unwrap();
    assert_eq!(runtime.route_generation, second);
    assert_eq!(runtime.door_link, door_link::DoorLinkState::Idle);
    assert_eq!(runtime.pending_traversal, None);
    assert_eq!(runtime.active_link, None);
    assert_eq!(runtime.travel_intent, None);
    assert!(world.get::<PauseAgent>(actor).is_none());
    assert!(matches!(
        world.get::<AgentTarget3d>(actor),
        Some(AgentTarget3d::Point(point)) if *point == Vec3::new(4.0, 0.0, 0.0)
    ));
    assert_eq!(nav_observation(&world, actor).status, NavStatus::Routing);
}

#[test]
fn cancellation_invalidates_route_generation_and_clears_travel_readiness() {
    let (mut world, actor) = route_test_world();
    let old_generation = {
        let mut runtime = world.get_mut::<AgentRuntime>(actor).unwrap();
        runtime.route_generation = RouteGeneration(7);
        runtime.door_link = door_link::DoorLinkState::TravelReached {
            door_form_id: 0x99,
            destination_cell_form_id: 0xC0DE,
        };
        runtime.travel_intent = Some(TravelIntent {
            generation: RouteGeneration(7),
            door_form_id: 0x99,
        });
        runtime.route_generation
    };
    let new_generation = replace_goal(&mut world, actor, None).expect("cancel accepted");
    assert_ne!(old_generation, new_generation);
    assert_eq!(
        world.get::<AgentRuntime>(actor).unwrap().door_link,
        door_link::DoorLinkState::Idle
    );
    assert_eq!(nav_observation(&world, actor).status, NavStatus::Routing);
}

#[test]
fn unbound_or_stale_goal_submission_returns_typed_errors_without_mutation() {
    let mut world = World::new();
    let actor = world.spawn(NavAgent).id();
    assert_eq!(
        replace_goal(&mut world, actor, Some(NavGoal::Point(Vec3::ZERO))),
        Err(api::NavError::ActorNotBound(actor))
    );

    let (mut world, actor) = route_test_world();
    let stale_target = world.spawn_empty().id();
    world.despawn(stale_target);
    assert_eq!(
        replace_goal(&mut world, actor, Some(NavGoal::Entity(stale_target)),),
        Err(api::NavError::TargetUnavailable(stale_target))
    );
}
