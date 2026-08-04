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
fn animation_link_source_portal_keeps_horizontal_extent_for_vertical_travel() {
    let (start, end) =
        animation_link_start_edge(Vec3::new(4.0, 10.0, 8.0), Vec3::new(4.0, 12.0, 8.0));

    assert_ne!(start, end);
    assert!(start.is_finite());
    assert!(end.is_finite());
    assert!((end.x - start.x).abs() > 0.0);
    assert_eq!(end.z, start.z);
}

#[test]
fn agent_family_refuses_doors_reads_the_nav_owned_flag() {
    let mut world = World::new();
    let agent = world.spawn_empty().id();

    // Unflagged: opens doors (the common case, and every door-opening family).
    assert!(!agent_family_refuses_doors(&world, agent));

    // The AI slice flags a Wander/Sandbox actor (`!opens_doors()`).
    set_agent_refuses_doors(&mut world, agent, true);
    assert!(
        world.get::<AgentRefusesDoors>(agent).is_some(),
        "the setter installs the nav-owned marker"
    );
    assert!(
        agent_family_refuses_doors(&world, agent),
        "a flagged agent refuses to open doors"
    );

    // Package stop / release clears it: the actor opens doors again.
    set_agent_refuses_doors(&mut world, agent, false);
    assert!(world.get::<AgentRefusesDoors>(agent).is_none());
    assert!(!agent_family_refuses_doors(&world, agent));
}

#[test]
fn a_blocked_agent_reports_its_real_near_zero_velocity() {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(0.0, -0.1, 0.0),
        boxddd::Vec3::new(5.0, 0.1, 5.0),
    );
    // A wall immediately in front (+X) of the agent's start position.
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(1.0, 1.0, 0.0),
        boxddd::Vec3::new(0.1, 2.0, 5.0),
    );
    let mover = fixture_capsule();
    let filter = fixture_filter();
    let desired_horizontal = Vec2::new(AGENT_DESIRED_SPEED, 0.0);

    // Walk toward the wall for two seconds -- plenty of time to close
    // the ~0.55 m gap and press into it -- then look at the final
    // tick's achieved displacement, once the agent is pinned.
    let mut position = Vec3::new(0.0, AGENT_HEIGHT / 2.0, 0.0);
    let mut velocity = Vec3::ZERO;
    let mut grounded = true;
    let mut achieved = Vec3::ZERO;
    for _ in 0..120 {
        let (new_position, new_velocity, new_grounded) = step_agent_kcc(
            &mut world,
            &mover,
            filter,
            filter,
            position,
            velocity,
            grounded,
            desired_horizontal,
            1.0 / 60.0,
        );
        achieved = (new_position - position) / (1.0 / 60.0);
        position = new_position;
        velocity = new_velocity;
        grounded = new_grounded;
    }
    assert!(grounded, "agent stays grounded while blocked by a wall");
    assert!(
        Vec2::new(achieved.x, achieved.z).length() < 0.2,
        "achieved horizontal speed should be near zero pinned against a wall, got {achieved:?}"
    );
    let outcome = movement_policy::decide_collision_outcome(movement_policy::VelocityObservation {
        desired_horizontal_speed: desired_horizontal.length(),
        achieved_horizontal_speed: Vec2::new(achieved.x, achieved.z).length(),
    });
    assert_eq!(outcome, movement_policy::CollisionOutcome::Blocked);
    assert_eq!(
        velocity.x, desired_horizontal.x,
        "the KCC's own remembered velocity still reflects the input -- it is the *achieved transform delta*, not this, that gets fed back to landmass"
    );
}

#[test]
fn resolve_status_prefers_door_link_pause_over_landmass_state() {
    let paused = door_link::DoorLinkState::Paused {
        door_form_id: 0x99,
        waited_ticks: 1,
        destination: door_link::LinkDestination::IntraCell,
    };
    assert_eq!(
        resolve_status(AgentState::Moving, paused),
        landmass_graph::NavAgentStatus::Paused
    );
    assert_eq!(
        resolve_status(AgentState::Idle, door_link::DoorLinkState::Idle),
        landmass_graph::NavAgentStatus::Idle
    );
}

#[test]
fn resolve_status_maps_a_failed_door_lifecycle_to_unreachable_not_paused() {
    // Issue #155 feature 4: `Failed` (the `MAX_WAIT_TICKS`-exhausted
    // terminal, reached identically whether the underlying cause was a
    // locked mid-route crossing or a two-sided door link) used to
    // resolve to `Paused` here even though the log line at the same
    // call site already says `nav agent unreachable` -- `tna status`/
    // the HUD must agree with that wording, not contradict it.
    let failed = door_link::DoorLinkState::Failed { door_form_id: 0x99 };
    assert_eq!(
        resolve_status(AgentState::Moving, failed),
        landmass_graph::NavAgentStatus::Unreachable
    );
    assert_ne!(
        resolve_status(AgentState::Moving, failed),
        landmass_graph::NavAgentStatus::Paused
    );
}

#[test]
fn resolve_status_reports_travel_reached_as_its_own_status() {
    let reached = door_link::DoorLinkState::TravelReached {
        door_form_id: 0x99,
        destination_cell_form_id: 0xC0DE,
    };
    assert_eq!(
        resolve_status(AgentState::Idle, reached),
        landmass_graph::NavAgentStatus::TravelReached
    );
    assert_eq!(
        landmass_graph::NavAgentStatus::TravelReached.as_str(),
        "travel-reached"
    );
}

#[test]
fn travel_request_routes_to_the_door_and_completes_the_lifecycle() {
    let mut world = harness_world();
    world.init_resource::<Time>();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    // `scripted_door_open` requires the resolved entity to carry a
    // `PlacementRoot` (the same invariant the `activate` command has).
    let door_entity = world
        .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
        .id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    world
        .resource_mut::<NavArchipelagoState>()
        .travel_doors
        .insert(
            0x99,
            TravelDoorLink {
                triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                door_position: Vec3::new(6.0, 0.0, 0.0),
                destination_cell_form_id: 0xC0DE,
                destination_door_form_id: 0x1234,
            },
        );

    request_travel(&mut world, 0, 0x99).expect("travel request succeeds");
    assert!(matches!(
        world.get::<AgentTarget3d>(agent),
        Some(AgentTarget3d::Point(point)) if *point == Vec3::new(5.0, 0.0, 0.0)
    ));

    // Not yet at the door: the lifecycle must not start.
    door_link_system(&mut world);
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Idle
    );

    // Arrive at the triangle midpoint: pause + door-open request.
    world.get_mut::<Transform>(agent).unwrap().translation = Vec3::new(5.0, 0.0, 0.0);
    door_link_system(&mut world);
    assert!(is_paused(&world, agent));
    assert!(world.get::<PauseAgent>(agent).is_some());

    // The unlocked door was scripted open through the interaction
    // boundary by the arrival itself (same code path as `activate`).
    assert!(
        world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity),
        "arrival must scripted-open the unlocked door"
    );

    // The open door resumes into the kinematic crossing.
    door_link_system(&mut world);
    assert!(door_link::is_traversing(
        world.get::<AgentRuntime>(agent).unwrap().door_link
    ));
    assert!(world.get::<DoorTraversal>(agent).is_some());

    // Complete the crossing (elapsed already past the fixed duration).
    use bevy::ecs::system::RunSystemOnce;
    world.get_mut::<DoorTraversal>(agent).unwrap().elapsed = 10.0;
    world
        .run_system_once(door_traversal_system)
        .expect("traversal system runs");

    // Issue #134: the agent is handed off, not left standing at the
    // door -- despawned from the active cell and ledgered for the
    // destination cell at the paired door's marker.
    assert!(
        world.get_entity(agent).is_err(),
        "the agent must leave the active cell entirely on handoff"
    );
    assert!(world.resource::<DebugAgentRoster>().entities[0].is_none());
    let entry = world
        .resource::<NavAgentLedger>()
        .0
        .entry_for(agent_ledger_id(0))
        .expect("the agent must be ledgered on handoff");
    assert_eq!(entry.cell_form_id, 0xC0DE);
    assert_eq!(
        entry.spawn_kind,
        ledger_policy::SpawnKind::DoorMarker {
            destination_door_form_id: 0x1234
        }
    );
}

#[test]
fn travel_arrival_tolerates_the_agent_capsule_centre_sitting_above_the_feet_level_door_midpoint() {
    let mut world = harness_world();
    world.init_resource::<Time>();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world
        .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
        .id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    world
        .resource_mut::<NavArchipelagoState>()
        .travel_doors
        .insert(
            0x99,
            TravelDoorLink {
                // Feet-level midpoint, exactly like real prepared nav
                // graph data.
                triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                door_position: Vec3::new(6.0, 0.0, 0.0),
                destination_cell_form_id: 0xC0DE,
                destination_door_form_id: 0x1234,
            },
        );

    request_travel(&mut world, 0, 0x99).expect("travel request succeeds");

    // Arrive horizontally at the triangle midpoint, but at capsule-
    // centre height (0.9 m above the feet-level midpoint) -- the exact
    // shape of the regression: a 3D distance check would read ~0.9 m,
    // just outside `TRAVEL_ARRIVAL_DISTANCE` (0.75 m), and never pause.
    world.get_mut::<Transform>(agent).unwrap().translation = Vec3::new(5.0, 0.9, 0.0);
    door_link_system(&mut world);
    assert!(
        is_paused(&world, agent),
        "the horizontal-plane arrival check must still fire despite the capsule-centre-vs-feet vertical offset"
    );
    assert!(world.get::<PauseAgent>(agent).is_some());
    assert!(
        world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity),
        "arrival must scripted-open the unlocked door"
    );
}

#[test]
fn locked_travel_door_fails_deterministically_without_opening() {
    let mut world = harness_world();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world.spawn_empty().id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    {
        let mut state = world.resource_mut::<NavArchipelagoState>();
        state.travel_doors.insert(
            0x99,
            TravelDoorLink {
                triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                door_position: Vec3::new(6.0, 0.0, 0.0),
                destination_cell_form_id: 0xC0DE,
                destination_door_form_id: 0x1234,
            },
        );
        state.door_lock_info.insert(
            0x99,
            DoorLockInfo {
                lock_level: Some(50),
                key_form_id: None,
                ..Default::default()
            },
        );
        state.door_usable.insert(0x99, false);
    }

    request_travel(&mut world, 0, 0x99).expect("routing to a locked door is allowed");
    door_link_system(&mut world);
    assert!(is_paused(&world, agent));
    assert!(
        !world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity),
        "a locked door must never be scripted open by the nav agent"
    );

    for _ in 0..door_link::MAX_WAIT_TICKS {
        door_link_system(&mut world);
    }
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Failed { door_form_id: 0x99 }
    );
}

#[test]
fn a_door_state_change_triggers_exactly_one_repath() {
    let mut world = harness_world();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world.spawn_empty().id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let archipelago = world.spawn_empty().id();
    {
        let mut state = world.resource_mut::<NavArchipelagoState>();
        state.archipelago = Some(archipelago);
        state.door_lock_info.insert(
            0x99,
            DoorLockInfo {
                lock_level: Some(50),
                key_form_id: None,
                ..Default::default()
            },
        );
        state.door_usable.insert(0x99, false);
        state.blocked_door_links.push(BlockedDoorLink {
            door_form_id: 0x99,
            start: Vec3::ZERO,
            end: Vec3::new(1.0, 0.0, 0.0),
        });
    }

    // No change: locked stays locked, nothing spawns.
    door_availability_system(&mut world);
    assert!(world.resource::<NavArchipelagoState>().links.is_empty());
    assert_eq!(
        world
            .resource::<NavArchipelagoState>()
            .blocked_door_links
            .len(),
        1
    );

    // The door opens (e.g. the player activates it): one flip, one
    // repath -- the link spawns (one unidirectional pair, see
    // `spawn_link_pair`) and the blocked entry is consumed.
    world
        .resource_mut::<interaction::InteractionState>()
        .open
        .insert(door_entity);
    door_availability_system(&mut world);
    assert_eq!(world.resource::<NavArchipelagoState>().links.len(), 2);
    assert!(
        world
            .resource::<NavArchipelagoState>()
            .blocked_door_links
            .is_empty()
    );

    // Steady state: repeated polls never spawn another link pair.
    door_availability_system(&mut world);
    door_availability_system(&mut world);
    assert_eq!(world.resource::<NavArchipelagoState>().links.len(), 2);
}

#[test]
fn set_door_lock_level_propagates_through_door_availability_system() {
    let mut world = harness_world();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world.spawn_empty().id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let archipelago = world.spawn_empty().id();
    // The door starts usable with a real link pair already spawned
    // (mirroring what `ensure_archipelago`/an earlier unblock would have
    // produced), so locking it has a link to remove.
    let link_entities = spawn_link_pair(&mut world, archipelago, Vec3::ZERO, Vec3::X, 1.0, 0);
    for link_entity in link_entities {
        let link = world.get::<AnimationLink3d>(link_entity).unwrap();
        assert_ne!(link.start_edge.0, link.start_edge.1);
        assert!(link.start_edge.0.is_finite());
        assert!(link.start_edge.1.is_finite());
        assert_eq!(link.end_edge.0, link.end_edge.1);
    }
    {
        let mut state = world.resource_mut::<NavArchipelagoState>();
        state.archipelago = Some(archipelago);
        state.door_lock_info.insert(
            0x99,
            DoorLockInfo {
                lock_level: None,
                key_form_id: Some(0x1234),
                ..Default::default()
            },
        );
        state.door_usable.insert(0x99, true);
        for link_entity in link_entities {
            state
                .link_kinds
                .insert(link_entity, LinkKind::Door { form_id: 0x99 });
            state.links.push(link_entity);
        }
    }

    // Locking the door records the level and preserves the existing key
    // requirement, without needing to pass it again.
    set_door_lock_level(&mut world, 0x99, Some(50));
    assert_eq!(door_lock_level_for_test(&world, 0x99), Some(50));
    assert_eq!(
        world
            .resource::<NavArchipelagoState>()
            .door_lock_info
            .get(&0x99)
            .unwrap()
            .key_form_id,
        Some(0x1234)
    );

    // The next poll sees the flip: the door becomes unusable and its
    // link is removed (recorded as blocked) -- one repath.
    door_availability_system(&mut world);
    assert_eq!(
        world
            .resource::<NavArchipelagoState>()
            .door_usable
            .get(&0x99),
        Some(&false)
    );
    assert_eq!(
        world
            .resource::<NavArchipelagoState>()
            .blocked_door_links
            .len(),
        1
    );

    // Clearing the lock (level 0 in the console command maps to `None`
    // here) makes the door usable again on the following poll.
    set_door_lock_level(&mut world, 0x99, None);
    assert_eq!(door_lock_level_for_test(&world, 0x99), None);
    door_availability_system(&mut world);
    assert_eq!(
        world
            .resource::<NavArchipelagoState>()
            .door_usable
            .get(&0x99),
        Some(&true)
    );
    assert!(
        world
            .resource::<NavArchipelagoState>()
            .blocked_door_links
            .is_empty()
    );
}

#[test]
fn set_door_lock_level_records_state_for_a_door_with_no_nav_triangles() {
    let mut world = harness_world();
    assert_eq!(door_lock_level_for_test(&world, 0x77), None);
    set_door_lock_level(&mut world, 0x77, Some(25));
    assert_eq!(door_lock_level_for_test(&world, 0x77), Some(25));
    set_door_lock_level(&mut world, 0x77, None);
    assert_eq!(door_lock_level_for_test(&world, 0x77), None);
}

#[test]
fn a_stalled_agent_short_of_a_closed_door_still_opens_it() {
    let mut world = harness_world();
    world.init_resource::<Time>();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world
        .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
        .id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            // Stopped 2.2 m short of the crossing, exactly the measured
            // shortfall, with the target beyond the door.
            Transform::from_xyz(2.8, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
            AgentKcc::default(),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    world
        .resource_mut::<NavArchipelagoState>()
        .mid_route_doors
        .push(MidRouteDoor {
            door_form_id: 0x99,
            vertices: door_triangle_around(Vec3::new(5.0, 0.0, 0.0)),
        });

    // Still making progress: short of the crossing is simply short of
    // the crossing, and must not gate.
    door_link_system(&mut world);
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Idle,
        "an agent still moving must not gate from a distance"
    );

    // Progress stops: now the door it is stalled against gets opened.
    world.get_mut::<AgentKcc>(agent).unwrap().collision_blocked = true;
    door_link_system(&mut world);
    assert!(is_paused(&world, agent));
    assert!(
        world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity),
        "a stalled agent short of a closed door must request it open"
    );

    // And it resumes on the next tick, exactly like the containment gate.
    door_link_system(&mut world);
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Idle
    );
    assert!(world.get::<PauseAgent>(agent).is_none());
}

#[test]
fn a_stalled_agent_never_opens_a_door_behind_it() {
    let mut world = harness_world();
    world.init_resource::<Time>();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world
        .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
        .id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            // The door is behind the agent, away from the target.
            Transform::from_xyz(2.8, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
            AgentKcc {
                collision_blocked: true,
                ..Default::default()
            },
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    world
        .resource_mut::<NavArchipelagoState>()
        .mid_route_doors
        .push(MidRouteDoor {
            door_form_id: 0x99,
            vertices: door_triangle_around(Vec3::new(0.6, 0.0, 0.0)),
        });

    door_link_system(&mut world);
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Idle
    );
    assert!(
        !world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity),
        "a door behind the agent is not what it is stalled on"
    );
}

#[test]
fn mid_route_crossing_gate_tolerates_the_agent_capsule_centre_vertical_offset() {
    let mut world = harness_world();
    world.init_resource::<Time>();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world
        .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
        .id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            Transform::from_xyz(0.0, 0.9, 0.0),
            AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    world
        .resource_mut::<NavArchipelagoState>()
        .mid_route_doors
        .push(MidRouteDoor {
            door_form_id: 0x99,
            // Feet-level midpoint, exactly like real prepared nav graph
            // data -- the agent's own Y stays at capsule-centre height
            // (0.9 m) the whole time, never snapped down to match it.
            vertices: door_triangle_around(Vec3::new(5.0, 0.0, 0.0)),
        });

    world.get_mut::<Transform>(agent).unwrap().translation = Vec3::new(5.0, 0.9, 0.0);
    door_link_system(&mut world);
    assert!(
        is_paused(&world, agent),
        "the horizontal-plane crossing gate must still fire despite the capsule-centre-vs-feet vertical offset"
    );
    assert!(
        world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity),
        "arrival must scripted-open the unlocked door"
    );
}

#[test]
fn a_travel_request_to_a_door_still_hands_off_even_though_it_is_also_a_crossing_gate_candidate() {
    let mut world = harness_world();
    world.init_resource::<Time>();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world
        .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
        .id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    {
        let mut state = world.resource_mut::<NavArchipelagoState>();
        state.travel_doors.insert(
            0x99,
            TravelDoorLink {
                triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                door_position: Vec3::new(6.0, 0.0, 0.0),
                destination_cell_form_id: 0xC0DE,
                destination_door_form_id: 0x1234,
            },
        );
        // The crossing-gate candidate set also carries this door (the
        // real-data shape) -- it must not hijack the travel-arrival
        // check once `travel_intent` targets it.
        state.mid_route_doors.push(MidRouteDoor {
            door_form_id: 0x99,
            vertices: door_triangle_around(Vec3::new(5.0, 0.0, 0.0)),
        });
    }

    request_travel(&mut world, 0, 0x99).expect("travel request succeeds");
    assert!(matches!(
        world.get::<AgentTarget3d>(agent),
        Some(AgentTarget3d::Point(point)) if *point == Vec3::new(5.0, 0.0, 0.0)
    ));

    world.get_mut::<Transform>(agent).unwrap().translation = Vec3::new(5.0, 0.0, 0.0);
    door_link_system(&mut world);
    assert!(is_paused(&world, agent));
    assert!(
        world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity),
        "arrival must scripted-open the unlocked door"
    );

    door_link_system(&mut world);
    assert!(
        door_link::is_traversing(world.get::<AgentRuntime>(agent).unwrap().door_link),
        "the travel-arrival check, not the crossing gate, must own this door once travel_intent targets it"
    );
    assert!(
        world.get::<DoorTraversal>(agent).is_some(),
        "a real travel handoff crosses through a DoorTraversal lerp, unlike the gap-less crossing-gate case"
    );

    use bevy::ecs::system::RunSystemOnce;
    world.get_mut::<DoorTraversal>(agent).unwrap().elapsed = 10.0;
    world
        .run_system_once(door_traversal_system)
        .expect("traversal system runs");

    assert!(
        world.get_entity(agent).is_err(),
        "a real travel_intent arrival must still hand the agent off to the destination cell"
    );
    let entry = world
        .resource::<NavAgentLedger>()
        .0
        .entry_for(agent_ledger_id(0))
        .expect("the agent must be ledgered on handoff");
    assert_eq!(entry.cell_form_id, 0xC0DE);
}

#[test]
fn a_goto_crossing_a_locked_travel_door_mid_route_fails_deterministically_without_opening() {
    let mut world = harness_world();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world.spawn_empty().id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    {
        let mut state = world.resource_mut::<NavArchipelagoState>();
        state.mid_route_doors.push(MidRouteDoor {
            door_form_id: 0x99,
            vertices: door_triangle_around(Vec3::new(5.0, 0.0, 0.0)),
        });
        state.travel_doors.insert(
            0x99,
            TravelDoorLink {
                triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                door_position: Vec3::new(5.0, 0.0, 0.0),
                destination_cell_form_id: 0xC0DE,
                destination_door_form_id: 0x1234,
            },
        );
        state.door_lock_info.insert(
            0x99,
            DoorLockInfo {
                lock_level: Some(50),
                key_form_id: None,
                ..Default::default()
            },
        );
        state.door_usable.insert(0x99, false);
    }

    door_link_system(&mut world);
    assert!(is_paused(&world, agent));
    assert!(
        !world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity),
        "a locked door must never be scripted open by the nav agent"
    );

    for _ in 0..door_link::MAX_WAIT_TICKS {
        door_link_system(&mut world);
    }
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Failed { door_form_id: 0x99 }
    );
    // Real-data correction (M4 wave 10): the agent stays stopped at the
    // link because its `AgentTarget3d` is cleared to `None` on this
    // exact `Failed` terminal (the #165 fix a few lines up this file --
    // nothing left to route toward), not because `PauseAgent` is kept
    // attached. `PauseAgent` must NOT survive this terminal: `landmass`
    // treats it as a standing "never solve this agent again" flag, so
    // leaving it here would silently freeze the agent even after a
    // later `tna goto`/`tna travel` sets a fresh target once the lock
    // clears (confirmed live on FranklinMetro02 0001a273, see
    // `a_failed_mid_route_door_wait_does_not_leave_the_agent_
    // permanently_paused`'s doc comment for the real-data trace).
    assert!(
        matches!(world.get::<AgentTarget3d>(agent), Some(AgentTarget3d::None)),
        "the agent stays stopped at the link because its target is cleared"
    );
    assert!(
        world.get::<PauseAgent>(agent).is_none(),
        "PauseAgent must not survive the Failed terminal, or a later retarget would silently never move the agent"
    );
}

#[test]
fn unlocking_a_mid_route_door_triggers_one_repath_that_frees_a_paused_agent() {
    let mut world = harness_world();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world
        .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
        .id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    {
        let mut state = world.resource_mut::<NavArchipelagoState>();
        state.mid_route_doors.push(MidRouteDoor {
            door_form_id: 0x99,
            vertices: door_triangle_around(Vec3::new(5.0, 0.0, 0.0)),
        });
        state.door_lock_info.insert(
            0x99,
            DoorLockInfo {
                lock_level: Some(50),
                key_form_id: None,
                ..Default::default()
            },
        );
        state.door_usable.insert(0x99, false);
    }

    // The agent walks up to the locked door and waits.
    door_link_system(&mut world);
    assert!(is_paused(&world, agent));
    assert!(
        !world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity)
    );

    // No change: nothing happens.
    door_availability_system(&mut world);
    assert!(is_paused(&world, agent));
    assert!(
        !world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity)
    );

    // The lock is cleared (e.g. the player picks/keys it elsewhere):
    // one usability flip, one repath -- `door_availability_system`
    // requests the door open again for the agent already paused on it.
    world
        .resource_mut::<NavArchipelagoState>()
        .door_lock_info
        .insert(
            0x99,
            DoorLockInfo {
                lock_level: None,
                key_form_id: None,
                ..Default::default()
            },
        );
    door_availability_system(&mut world);
    assert!(
        world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity),
        "the repath must retry the scripted-open request for the door the agent is paused on"
    );

    // Steady state: repeated polls do not re-trigger the repath.
    door_availability_system(&mut world);
    door_availability_system(&mut world);

    // The next tick resumes and completes the (gap-less) crossing.
    door_link_system(&mut world);
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Idle
    );
}

#[test]
fn a_failed_mid_route_door_wait_does_not_leave_the_agent_permanently_paused() {
    let mut world = harness_world();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world.spawn_empty().id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    {
        let mut state = world.resource_mut::<NavArchipelagoState>();
        state.mid_route_doors.push(MidRouteDoor {
            door_form_id: 0x99,
            vertices: door_triangle_around(Vec3::new(5.0, 0.0, 0.0)),
        });
        state.door_lock_info.insert(
            0x99,
            DoorLockInfo {
                lock_level: Some(25),
                key_form_id: None,
                ..Default::default()
            },
        );
        state.door_usable.insert(0x99, false);
    }

    // Walk the agent up to the locked door and exhaust the wait bound.
    for _ in 0..=door_link::MAX_WAIT_TICKS {
        door_link_system(&mut world);
    }
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Failed { door_form_id: 0x99 },
        "test setup: the door-link cycle must reach the documented Failed terminal"
    );
    assert!(
        world.get::<PauseAgent>(agent).is_none(),
        "PauseAgent must not survive the Failed terminal -- a stale PauseAgent silently \
             freezes every subsequent tna goto/tna travel at the landmass level regardless of \
             any fresh AgentTarget3d, exactly the real-data symptom this test pins"
    );
}

#[test]
fn a_failed_travel_arrival_does_not_leave_the_agent_permanently_paused() {
    let mut world = harness_world();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world.spawn_empty().id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(5.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    {
        let mut state = world.resource_mut::<NavArchipelagoState>();
        state.travel_doors.insert(
            0x99,
            TravelDoorLink {
                triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                door_position: Vec3::new(6.0, 0.0, 0.0),
                destination_cell_form_id: 0xC0DE,
                destination_door_form_id: 0x1234,
            },
        );
        state.mid_route_doors.push(MidRouteDoor {
            door_form_id: 0x99,
            vertices: door_triangle_around(Vec3::new(5.0, 0.0, 0.0)),
        });
        state.door_lock_info.insert(
            0x99,
            DoorLockInfo {
                lock_level: Some(25),
                key_form_id: None,
                ..Default::default()
            },
        );
        state.door_usable.insert(0x99, false);
    }

    request_travel(&mut world, 0, 0x99).expect("routing to a locked door is allowed");
    for _ in 0..(door_link::MAX_WAIT_TICKS * 2) {
        door_link_system(&mut world);
    }
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Failed { door_form_id: 0x99 }
    );
    assert!(
        world.get::<PauseAgent>(agent).is_none(),
        "PauseAgent must not survive a failed travel arrival either -- the same is_failed \
             branch handles both destination types"
    );
}

#[test]
fn concurrent_travel_requests_are_rejected() {
    let mut world = harness_world();
    let agent = world.spawn((NavAgent, AgentRuntime::default())).id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    world
        .resource_mut::<NavArchipelagoState>()
        .travel_doors
        .insert(
            0x99,
            TravelDoorLink {
                triangle_midpoint: Vec3::ZERO,
                door_position: Vec3::ZERO,
                destination_cell_form_id: 0xC0DE,
                destination_door_form_id: 0x1234,
            },
        );
    request_travel(&mut world, 0, 0x99).expect("first request succeeds");
    let error = request_travel(&mut world, 0, 0x99).unwrap_err();
    assert_eq!(error.code, "travel_in_progress");
}

#[test]
fn travel_request_errors_without_an_agent_or_a_known_door() {
    let mut world = harness_world();
    assert_eq!(
        request_travel(&mut world, 0, 0x99).unwrap_err().code,
        "no_agent"
    );
    let agent = world.spawn((NavAgent, AgentRuntime::default())).id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    assert_eq!(
        request_travel(&mut world, 0, 0x99).unwrap_err().code,
        "unknown_travel_door"
    );
}

#[test]
fn an_early_setlock_unlock_survives_the_first_archipelago_build() {
    let mut world = archipelago_build_world();
    let manifest = manifest_with_nav_graph_and_door(0xBEEF, 0x99, Some(25));
    world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    set_door_lock_level(&mut world, 0x99, None);

    ensure_archipelago(&mut world).expect("archipelago builds");

    assert_eq!(
        door_lock_level_for_test(&world, 0x99),
        None,
        "the runtime unlock recorded before the archipelago existed must survive the build, winning over the authored lock level"
    );
}

#[test]
fn a_door_untouched_by_setlock_keeps_its_authored_lock_level() {
    let mut world = archipelago_build_world();
    let manifest = manifest_with_nav_graph_and_door(0xBEEF, 0x99, Some(25));
    world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    ensure_archipelago(&mut world).expect("archipelago builds");

    assert_eq!(
        door_lock_level_for_test(&world, 0x99),
        Some(25),
        "an untouched door's authored lock level must be unchanged by the merge"
    );
}

#[test]
fn locking_the_only_route_door_fails_at_query_time_not_after_walking_and_waiting() {
    // Invariant 1: the door corridor is the mesh's *only* route (no
    // bypass); locking it (the real `LOCKED_DOOR_TYPE_INDEX_COST`
    // sentinel, applied before the very first solve) must produce
    // `NoPath` immediately -- landmass never even attempts to walk the
    // agent there and wait, unlike this file's pre-#155 proximity gate.
    let (mut app, agent) = door_topology_test_app(false, Some(LOCKED_DOOR_TYPE_INDEX_COST));
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    assert_eq!(
        app.world().get::<AgentState>(agent).copied(),
        Some(AgentState::NoPath),
        "a locked door with no alternate route must fail at query time"
    );
    // No walk-and-wait: the agent's position never left its spawn.
    assert_eq!(
        app.world().get::<Transform>(agent).unwrap().translation,
        DOOR_TOPOLOGY_ROOM_A_POINT
    );
}

#[test]
fn locking_a_door_with_an_alternate_route_selects_the_alternate() {
    // Invariant 2: the same lock, but the bypass corridor exists too --
    // the solver must find *a* route (not `NoPath`), necessarily via
    // the bypass, since the door corridor's own type index is
    // cost-excluded for this agent.
    let (mut app, agent) = door_topology_test_app(true, Some(LOCKED_DOOR_TYPE_INDEX_COST));
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    let state = app.world().get::<AgentState>(agent).copied();
    assert_ne!(
        state,
        Some(AgentState::NoPath),
        "an alternate route must be found when the door is locked, got {state:?}"
    );
}

#[test]
fn a_closed_unlocked_door_stays_passable_but_expensive() {
    // The acceptance correction: an unbounded cost here would stop the
    // agent ever reaching the door it is supposed to open, so a closed
    // *openable* door's interior is merely expensive. Its doorway
    // crossing stays ordinary walkable ground.
    let (mut world, agent) = closed_blocker_override_world(true, false, true);
    apply_door_lock_overrides(&mut world, agent);
    assert_eq!(
        override_costs(&world, agent),
        vec![(2, CLOSED_DOOR_TYPE_INDEX_COST)]
    );
    assert!(
        CLOSED_DOOR_TYPE_INDEX_COST.is_finite(),
        "a closed openable door must stay plannable"
    );
}

#[test]
fn a_closed_blocker_that_cannot_be_opened_is_impassable() {
    // The ungated kinematic-activator class (a vault gear door with no
    // open/close FSM): there is no sanctioned crossing, so the route must
    // fail fast rather than walk the agent into a solid.
    let (mut world, agent) = closed_blocker_override_world(true, false, false);
    apply_door_lock_overrides(&mut world, agent);
    assert_eq!(
        override_costs(&world, agent),
        vec![(2, LOCKED_DOOR_TYPE_INDEX_COST)]
    );
}

#[test]
fn an_open_unlocked_door_blocks_nothing() {
    let (mut world, agent) = closed_blocker_override_world(true, true, true);
    apply_door_lock_overrides(&mut world, agent);
    assert_eq!(override_costs(&world, agent), Vec::new());
}

#[test]
fn a_closed_locked_door_stays_an_impassable_barrier() {
    // Wave-9 behaviour, unchanged: locked is a barrier on both the
    // crossing and the interior, so the route fails at query time rather
    // than walking the agent to a door it cannot open.
    let (mut world, agent) = closed_blocker_override_world(false, false, true);
    apply_door_lock_overrides(&mut world, agent);
    assert_eq!(
        override_costs(&world, agent),
        vec![
            (1, LOCKED_DOOR_TYPE_INDEX_COST),
            (2, LOCKED_DOOR_TYPE_INDEX_COST),
        ]
    );
}

#[test]
fn an_open_locked_door_blocks_nothing_either() {
    // `repath::door_usable`'s existing rule: an open door is passable
    // regardless of its lock record. The interior override must agree --
    // keying it on `open` rather than `usable` is what makes these two
    // compose instead of contradicting each other.
    let (mut world, agent) = closed_blocker_override_world(true, true, true);
    apply_door_lock_overrides(&mut world, agent);
    assert_eq!(override_costs(&world, agent), Vec::new());
}

#[test]
fn a_blocker_with_no_open_state_recorded_is_treated_as_closed() {
    // Fail safe: an unknown open state must never leave the inside of a
    // solid freely traversable.
    let (mut world, agent) = closed_blocker_override_world(true, false, true);
    world
        .resource_mut::<NavArchipelagoState>()
        .door_open
        .remove(&0x99);
    apply_door_lock_overrides(&mut world, agent);
    assert_eq!(
        override_costs(&world, agent),
        vec![(2, CLOSED_DOOR_TYPE_INDEX_COST)]
    );
}

#[test]
fn activating_a_blocker_through_the_interaction_boundary_lifts_the_nav_override() {
    let mut world = harness_world();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let blocker = world
        .spawn(interaction::PlacementRoot::new(activator_placement(0x99)))
        .id();
    registry.register(blocker, 0x99, None);
    world.insert_resource(registry);

    let agent = world.spawn((NavAgent, AgentRuntime::default())).id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    {
        let mut state = world.resource_mut::<NavArchipelagoState>();
        state.door_usable.insert(0x99, true);
        state.door_open.insert(0x99, false);
        state.closed_door_type_indices.insert(0x99, 2);
        // Not openable, exactly like the authored VaultGearDoor: a closed
        // one is a hard barrier, and only a runtime open can lift it.
    }

    // Closed: the blocker's interior is impassable (route unreachable).
    apply_door_lock_overrides(&mut world, agent);
    assert_eq!(
        override_costs(&world, agent),
        vec![(2, LOCKED_DOOR_TYPE_INDEX_COST)],
        "a closed activator blocker must be impassable"
    );

    // Activate it open through the real console/BRP interaction boundary
    // -- nothing here touches `door_open` directly.
    let opened = interaction::scripted_activator_toggle(&mut world, blocker);
    assert!(opened, "activation must open the blocker");
    assert!(
        world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&blocker),
        "the interaction boundary must populate the shared open signal"
    );

    // Nav observes the open state on its next poll and rebuilds the
    // agent's overrides: the barrier is gone, the route completes.
    door_availability_system(&mut world);
    assert_eq!(
        world.resource::<NavArchipelagoState>().door_open.get(&0x99),
        Some(&true),
        "door_availability_system must observe the activated-open blocker"
    );
    assert_eq!(
        override_costs(&world, agent),
        Vec::new(),
        "an activated-open blocker must impose no cost -- the route completes"
    );

    // Activating it shut again restores the barrier: closed is unreachable.
    let opened = interaction::scripted_activator_toggle(&mut world, blocker);
    assert!(!opened, "second activation must close the blocker");
    door_availability_system(&mut world);
    assert_eq!(
        override_costs(&world, agent),
        vec![(2, LOCKED_DOOR_TYPE_INDEX_COST)],
        "a re-closed activator blocker must be impassable again"
    );
}

#[test]
fn activating_an_activator_populates_the_shared_open_signal() {
    let mut world = harness_world();
    world.init_resource::<interaction::InteractionState>();
    let blocker = world
        .spawn(interaction::PlacementRoot::new(activator_placement(0x99)))
        .id();

    assert!(interaction::scripted_activator_toggle(&mut world, blocker));
    assert!(
        world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&blocker)
    );
    assert!(!interaction::scripted_activator_toggle(&mut world, blocker));
    assert!(
        !world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&blocker)
    );
}

#[test]
fn a_closed_door_cost_still_lets_the_only_route_solve_against_a_live_archipelago() {
    // Issue #177 acceptance, against a real `Archipelago3d` solve on the
    // same one-route mesh the locked-door invariant above uses: the
    // closed-but-openable cost must still produce a path, or the agent
    // never reaches the door the crossing gate is supposed to open. This
    // is the exact difference between `CLOSED_DOOR_TYPE_INDEX_COST` and
    // `LOCKED_DOOR_TYPE_INDEX_COST`, pinned end to end rather than only
    // as an override-table assertion.
    let (mut app, agent) = door_topology_test_app(false, Some(CLOSED_DOOR_TYPE_INDEX_COST));
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    let state = app.world().get::<AgentState>(agent).copied();
    assert_ne!(
        state,
        Some(AgentState::NoPath),
        "a closed but openable door must stay plannable, got {state:?}"
    );
}

#[test]
fn unlocking_the_only_route_door_restores_the_direct_path() {
    // Invariant 3: starting locked (as above, `NoPath`), then clearing
    // the override wholesale (mirroring `apply_door_lock_overrides`'s
    // own "replace the whole component" rebuild on an unlock) must let
    // the agent path again on its very next solve -- `landmass`'s own
    // `does_agent_need_repath` retries every tick while `current_path`
    // is `None` (the `NoPath` state's own path value), so no explicit
    // retarget is needed for this case.
    let (mut app, agent) = door_topology_test_app(false, Some(LOCKED_DOOR_TYPE_INDEX_COST));
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    assert_eq!(
        app.world().get::<AgentState>(agent).copied(),
        Some(AgentState::NoPath),
        "test setup: the door must start locked-and-failed"
    );

    app.world_mut()
        .entity_mut(agent)
        .insert(AgentTypeIndexCostOverrides::default());
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);

    let state = app.world().get::<AgentState>(agent).copied();
    assert_ne!(
        state,
        Some(AgentState::NoPath),
        "unlocking must restore the direct route, got {state:?}"
    );
}

#[test]
fn an_unlocked_typed_door_triangle_still_connects_its_neighbours() {
    // CONSTRAINT pin (issue #155 feature 1): typing a door triangle
    // must not remove or alter unrelated adjacency. No lock override at
    // all here -- the door corridor is typed (type index 1) but never
    // cost-excluded, so the direct route must still be found exactly as
    // if it had never been typed.
    let (mut app, agent) = door_topology_test_app(false, None);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    let state = app.world().get::<AgentState>(agent).copied();
    assert_ne!(
        state,
        Some(AgentState::NoPath),
        "a typed-but-unlocked door triangle must still connect its neighbours, got {state:?}"
    );
}

#[test]
fn locked_travel_arrival_settles_at_a_stable_unreachable_terminal_not_an_oscillation() {
    let mut world = harness_world();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world.spawn_empty().id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(5.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    {
        let mut state = world.resource_mut::<NavArchipelagoState>();
        state.travel_doors.insert(
            0x99,
            TravelDoorLink {
                triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                door_position: Vec3::new(6.0, 0.0, 0.0),
                destination_cell_form_id: 0xC0DE,
                destination_door_form_id: 0x1234,
            },
        );
        // Every real travel door is also a mid-route crossing-gate
        // candidate (see the module doc) -- reproducing the bug
        // requires this door to be registered on both sets, not just
        // `travel_doors`.
        state.mid_route_doors.push(MidRouteDoor {
            door_form_id: 0x99,
            vertices: door_triangle_around(Vec3::new(5.0, 0.0, 0.0)),
        });
        state.door_lock_info.insert(
            0x99,
            DoorLockInfo {
                lock_level: Some(25),
                key_form_id: None,
                ..Default::default()
            },
        );
        state.door_usable.insert(0x99, false);
    }

    request_travel(&mut world, 0, 0x99).expect("routing to a locked door is allowed");

    // Drive well past `MAX_WAIT_TICKS` -- long enough for the
    // pre-fix code to complete a full Paused -> Failed -> Paused
    // oscillation cycle at least once.
    for _ in 0..(door_link::MAX_WAIT_TICKS * 3) {
        door_link_system(&mut world);
    }

    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Failed { door_form_id: 0x99 },
        "a locked travel door must settle at the documented deterministic Failed terminal"
    );
    assert!(
        !world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity),
        "a locked door must never be scripted open by the nav agent"
    );
    assert!(
        world.get_entity(agent).is_ok(),
        "a locked travel target must never hand the agent off"
    );
    assert!(
        matches!(world.get::<AgentTarget3d>(agent), Some(AgentTarget3d::None)),
        "F165 fix: the failed target must be cleared so the mid-route \
             gate does not treat this agent as still routed through its own \
             just-failed door"
    );

    // Regression pin for the oscillation itself: further ticks must
    // not flip the status back to `Paused` -- the exact bug this test
    // is named for.
    for _ in 0..(door_link::MAX_WAIT_TICKS * 2) {
        door_link_system(&mut world);
        assert_eq!(
            world.get::<AgentRuntime>(agent).unwrap().door_link,
            door_link::DoorLinkState::Failed { door_form_id: 0x99 },
            "the Failed terminal must be stable, not an oscillation back into Paused"
        );
    }
}

#[test]
fn unlocking_after_a_failed_travel_arrival_and_reissuing_travel_hands_off_normally() {
    let mut world = harness_world();
    world.init_resource::<Time>();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world
        .spawn(interaction::PlacementRoot::new(door_placement(0x99)))
        .id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);

    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(5.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    {
        let mut state = world.resource_mut::<NavArchipelagoState>();
        state.travel_doors.insert(
            0x99,
            TravelDoorLink {
                triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                door_position: Vec3::new(6.0, 0.0, 0.0),
                destination_cell_form_id: 0xC0DE,
                destination_door_form_id: 0x1234,
            },
        );
        state.mid_route_doors.push(MidRouteDoor {
            door_form_id: 0x99,
            vertices: door_triangle_around(Vec3::new(5.0, 0.0, 0.0)),
        });
        state.door_lock_info.insert(
            0x99,
            DoorLockInfo {
                lock_level: Some(25),
                key_form_id: None,
                ..Default::default()
            },
        );
        state.door_usable.insert(0x99, false);
    }
    request_travel(&mut world, 0, 0x99).expect("routing to a locked door is allowed");
    // One call transitions the arrival into `Paused` (waited_ticks=0);
    // `MAX_WAIT_TICKS` further `Tick` calls exhaust the wait bound.
    door_link_system(&mut world);
    for _ in 0..door_link::MAX_WAIT_TICKS {
        door_link_system(&mut world);
    }
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Failed { door_form_id: 0x99 }
    );

    // Unlock (mirrors `setlock 0x99 0`) and reissue the travel: the
    // agent is still standing exactly at the door, so this is the
    // pure-FSM half of the retry; `request_travel` re-arms
    // `travel_intent` since it was cleared on failure.
    set_door_lock_level(&mut world, 0x99, None);
    request_travel(&mut world, 0, 0x99).expect("a fresh travel request after failure is allowed");

    door_link_system(&mut world);
    assert!(is_paused(&world, agent));
    assert!(
        world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity),
        "the now-unlocked door must be scripted open by the reissued travel"
    );

    door_link_system(&mut world);
    assert!(door_link::is_traversing(
        world.get::<AgentRuntime>(agent).unwrap().door_link
    ));

    use bevy::ecs::system::RunSystemOnce;
    world.get_mut::<DoorTraversal>(agent).unwrap().elapsed = 10.0;
    world
        .run_system_once(door_traversal_system)
        .expect("traversal system runs");

    assert!(
        world.get_entity(agent).is_err(),
        "the agent must be handed off once the retried travel completes"
    );
    let entry = world
        .resource::<NavAgentLedger>()
        .0
        .entry_for(agent_ledger_id(0))
        .expect("the agent must be ledgered on handoff");
    assert_eq!(entry.cell_form_id, 0xC0DE);
}
