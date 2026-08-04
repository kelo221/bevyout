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
fn agent_index_must_fit_the_unique_u32_ledger_identity() {
    let largest_valid = MAX_AGENT_INDEX;
    assert_eq!(
        parse_agent_index(&largest_valid.to_string()),
        Ok(largest_valid)
    );

    let first_unrepresentable = MAX_AGENT_INDEX + 1;
    let error = parse_agent_index(&first_unrepresentable.to_string()).unwrap_err();
    assert_eq!(error.code, "bad_agent_index");
}

#[test]
fn archipelago_teardown_on_cell_swap_ledgers_the_agent_instead_of_losing_it() {
    let mut world = harness_world();
    let archipelago = world.spawn_empty().id();
    let island = world.spawn_empty().id();
    world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xC0DE);
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago);
    world.resource_mut::<NavArchipelagoState>().islands = vec![island];
    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            Transform::from_xyz(1.0, 2.0, 3.0),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);

    world.insert_resource(crate::viewer::LoadedSceneManifest(minimal_manifest(0xBEEF)));
    despawn_stale_navmesh_archipelago(&mut world);

    assert!(world.get_entity(archipelago).is_err());
    assert!(world.get_entity(island).is_err());
    assert!(world.get_entity(agent).is_err(), "the live entity is gone");
    assert!(
        world
            .resource::<NavArchipelagoState>()
            .cell_form_id
            .is_none()
    );
    assert!(world.resource::<DebugAgentRoster>().entities[0].is_none());

    let entry = world
        .resource::<NavAgentLedger>()
        .0
        .entry_for(agent_ledger_id(0))
        .expect("the agent must be ledgered, not lost");
    assert_eq!(entry.cell_form_id, 0xC0DE, "frozen in the departing cell");
    assert_eq!(
        entry.spawn_kind,
        ledger_policy::SpawnKind::FrozenPosition {
            position: [1.0, 2.0, 3.0]
        }
    );
}

#[test]
fn a_player_swap_through_the_agents_own_route_door_follows_through() {
    let mut world = harness_world();
    world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xC0DE);
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(world.spawn_empty().id());
    world
        .resource_mut::<NavArchipelagoState>()
        .travel_doors
        .insert(
            0x99,
            TravelDoorLink {
                triangle_midpoint: Vec3::ZERO,
                door_position: Vec3::ZERO,
                destination_cell_form_id: 0xBEEF,
                destination_door_form_id: 0x1234,
            },
        );
    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime {
                travel_intent: Some(0x99),
                ..default()
            },
            Transform::from_xyz(5.0, 0.0, 0.0),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    world.resource_mut::<PendingPlayerSwapDoor>().0 = Some(0x99);

    world.insert_resource(crate::viewer::LoadedSceneManifest(minimal_manifest(0xBEEF)));
    despawn_stale_navmesh_archipelago(&mut world);

    assert!(world.get_entity(agent).is_err());
    let entry = world
        .resource::<NavAgentLedger>()
        .0
        .entry_for(agent_ledger_id(0))
        .expect("the agent must be ledgered");
    assert_eq!(
        entry.cell_form_id, 0xBEEF,
        "ledgered to the destination cell"
    );
    assert_eq!(
        entry.spawn_kind,
        ledger_policy::SpawnKind::DoorMarker {
            destination_door_form_id: 0x1234
        }
    );
}

#[test]
fn a_player_swap_through_a_different_door_still_freezes_the_agent() {
    let mut world = harness_world();
    world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xC0DE);
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(world.spawn_empty().id());
    let agent = world
        .spawn((
            NavAgent,
            // The agent is routed to a different travel door than the
            // one the player used.
            AgentRuntime {
                travel_intent: Some(0x50),
                ..default()
            },
            Transform::from_xyz(7.0, 0.0, 0.0),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);
    world.resource_mut::<PendingPlayerSwapDoor>().0 = Some(0x99);

    world.insert_resource(crate::viewer::LoadedSceneManifest(minimal_manifest(0xBEEF)));
    despawn_stale_navmesh_archipelago(&mut world);

    assert!(world.get_entity(agent).is_err());
    let entry = world
        .resource::<NavAgentLedger>()
        .0
        .entry_for(agent_ledger_id(0))
        .expect("the agent must be ledgered, not lost");
    assert_eq!(entry.cell_form_id, 0xC0DE, "frozen in the departing cell");
    assert_eq!(
        entry.spawn_kind,
        ledger_policy::SpawnKind::FrozenPosition {
            position: [7.0, 0.0, 0.0]
        }
    );
}

#[test]
fn matching_cell_activation_restores_exactly_one_ledgered_agent() {
    let mut world = harness_world();
    world.init_resource::<Assets<Mesh>>();
    world.init_resource::<Assets<StandardMaterial>>();
    world
        .resource_mut::<NavAgentLedger>()
        .0
        .record(ledger_policy::LedgerEntry {
            agent_id: agent_ledger_id(0),
            cell_form_id: 0xBEEF,
            spawn_kind: ledger_policy::SpawnKind::DoorMarker {
                destination_door_form_id: 0x1234,
            },
            remaining_target: None,
        });

    let mut manifest = minimal_manifest(0xBEEF);
    manifest
        .placements
        .push(door_placement_at(0x1234, [9.0, 1.0, 2.0]));
    // `PreparedSceneManifest.nav_graph` only needs to be `Some` here --
    // `ensure_archipelago` short-circuits on its `already_current`
    // check (below) before it would ever read this path from disk.
    manifest.nav_graph = Some(crate::vsa::PreparedNavGraphSource::default());
    world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));
    // Pre-seed the archipelago as already current for 0xBEEF so
    // `ensure_archipelago` returns immediately without any real
    // `bevy_landmass`/file-I/O plumbing -- this test is about the
    // ledger claim + spawn-count contract, not archipelago building
    // (already covered by other tests/real-data acceptance).
    let archipelago_entity = world.spawn_empty().id();
    world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xBEEF);
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago_entity);

    restore_ledgered_agents_system(&mut world);

    let mut query = world.query_filtered::<Entity, With<NavAgent>>();
    let agents: Vec<Entity> = query.iter(&world).collect();
    assert_eq!(agents.len(), 1, "exactly one agent must be spawned");
    assert!(
        world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(agent_ledger_id(0))
            .is_none(),
        "the claimed entry must be consumed"
    );
    let agent_entity = agents[0];
    let position = world.get::<Transform>(agent_entity).unwrap().translation;
    assert_eq!(
        position,
        Vec3::new(9.0, 1.0, 2.0),
        "spawned at the door marker"
    );
    assert_eq!(
        world.resource::<DebugAgentRoster>().entities[0],
        Some(agent_entity)
    );
}

#[test]
fn a_goto_crossing_a_closed_unlocked_travel_door_mid_route_drives_the_lifecycle_once_with_no_handoff()
 {
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
            // A plain `goto` well beyond the door -- no `travel_intent`
            // for this (or any) door.
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
        // The same door is also a travel-door candidate -- this is the
        // real-data shape (see this file's module doc): the crossing
        // gate must still apply, and must still not hand off.
        state.travel_doors.insert(
            0x99,
            TravelDoorLink {
                triangle_midpoint: Vec3::new(5.0, 0.0, 0.0),
                door_position: Vec3::new(5.0, 0.0, 0.0),
                destination_cell_form_id: 0xC0DE,
                destination_door_form_id: 0x1234,
            },
        );
    }

    // Not yet at the door: the lifecycle must not start.
    door_link_system(&mut world);
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Idle
    );

    // Arrive at the triangle midpoint: pause + scripted-open request.
    world.get_mut::<Transform>(agent).unwrap().translation = Vec3::new(5.0, 0.0, 0.0);
    door_link_system(&mut world);
    assert!(is_paused(&world, agent));
    assert!(world.get::<PauseAgent>(agent).is_some());
    assert!(
        world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&door_entity),
        "arrival must scripted-open the unlocked door"
    );

    // The open door resumes -- and, unlike the off-mesh link cases,
    // the crossing completes in the same tick.
    door_link_system(&mut world);
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Idle,
        "an intra-cell mid-route crossing returns to Idle, not a handoff"
    );
    assert!(world.get::<DoorTraversal>(agent).is_none());
    assert!(world.get::<PauseAgent>(agent).is_none());
    assert!(
        world
            .get::<AgentRuntime>(agent)
            .unwrap()
            .active_link
            .is_none()
    );
    assert!(
        world.get_entity(agent).is_ok(),
        "the agent stays in the active cell"
    );
    assert!(
        world
            .resource::<NavAgentLedger>()
            .0
            .entry_for(agent_ledger_id(0))
            .is_none(),
        "crossing a travel door's triangle mid-route (not the agent's own travel_intent) must not ledger a handoff"
    );
}

#[test]
fn a_travel_target_left_open_by_a_prior_handoff_still_fails_when_locked() {
    let mut world = harness_world();
    world.init_resource::<interaction::InteractionState>();
    let mut registry = crate::console::RefRegistry::default();
    let door_entity = world.spawn_empty().id();
    registry.register(door_entity, 0x99, None);
    world.insert_resource(registry);
    // Simulates the state a prior successful hand-off leaves behind:
    // the door is physically open, with nothing left to close it.
    world
        .resource_mut::<interaction::InteractionState>()
        .open
        .insert(door_entity);

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

    // Arrival: the door is already open, so `crossing_gate` reports
    // `Pass` and `request_door_open` is skipped -- exactly the real
    // repro's shape. Must still only reach `Paused`, never
    // `Traversing`, on this very first tick.
    door_link_system(&mut world);
    assert!(
        is_paused(&world, agent),
        "an already-open but locked travel target must still pause, not hand off immediately"
    );

    // Exhaust the wait bound. At every tick the agent must still be
    // alive, on the ground, and never `Traversing` -- the physically
    // open door must never let a locked travel destination complete.
    for _ in 0..door_link::MAX_WAIT_TICKS {
        door_link_system(&mut world);
        assert!(
            world.get_entity(agent).is_ok(),
            "a locked travel target left physically open must never hand the agent off"
        );
        assert!(
            !door_link::is_traversing(world.get::<AgentRuntime>(agent).unwrap().door_link),
            "lock state must be authoritative for the hand-off regardless of physical open state"
        );
    }

    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().door_link,
        door_link::DoorLinkState::Failed { door_form_id: 0x99 },
        "an already-open but locked travel target must settle at the deterministic Failed terminal"
    );
    assert!(
        matches!(world.get::<AgentTarget3d>(agent), Some(AgentTarget3d::None)),
        "F165 fix: the failed target must be cleared here too"
    );

    // Stability: further ticks must not oscillate back into Paused or
    // Traversing (mirrors the closed-door oscillation regression pin
    // above).
    for _ in 0..(door_link::MAX_WAIT_TICKS * 2) {
        door_link_system(&mut world);
        assert_eq!(
            world.get::<AgentRuntime>(agent).unwrap().door_link,
            door_link::DoorLinkState::Failed { door_form_id: 0x99 },
            "the Failed terminal must be stable even though the door stays physically open"
        );
    }
}
