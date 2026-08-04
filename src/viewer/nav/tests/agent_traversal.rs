use super::*;
use crate::viewer::nav::world::portals::{MergeLinkRejection, validate_merge_link_collision};
use std::collections::HashSet;

use super::tests_support::*;

#[test]
fn merge_traversal_system_sweeps_the_agent_to_the_far_portal_point() {
    use bevy::ecs::system::RunSystemOnce;

    let mut world = World::new();
    world.init_resource::<DebugAgentRoster>();
    world.init_resource::<NavArchipelagoState>();
    world.insert_resource(PhysicsDisabled(false));
    world.insert_resource(CellPhysicsReadiness::Ready);
    world.init_resource::<Time>();
    world
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
    // `merge_traversal_system` collides against the real
    // `player::player_collision_filter()`/`stair_support_filter()`
    // queries (same as `apply_agent_physics_movement`), so the fixture
    // geometry must use `add_player_compatible_floor`'s filter, not
    // `add_fixture_box`'s self-consistent-but-unrelated one (that
    // mismatch was the root cause of an earlier version of this test
    // free-falling straight through the floor).
    let mut physics_world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(0.0, -0.1, 0.0),
        boxddd::Vec3::new(5.0, 0.1, 5.0),
    );
    world.insert_non_send(BoxdddPhysicsContext::from_world(physics_world));

    let link = world.spawn_empty().id();
    world
        .resource_mut::<NavArchipelagoState>()
        .link_kinds
        .insert(link, LinkKind::Merge { kind: 1 });
    let target = Vec3::new(2.0, AGENT_HEIGHT / 2.0, 0.0);
    let agent = world
        .spawn((
            NavAgent,
            AgentKcc {
                grounded: true,
                ..default()
            },
            Transform::from_xyz(0.0, AGENT_HEIGHT / 2.0, 0.0),
            AgentRuntime {
                active_link: Some(LinkKind::Merge { kind: 1 }),
                ..default()
            },
            MergeTraversal {
                source: Vec3::new(0.0, AGENT_HEIGHT / 2.0, 0.0),
                target,
                crossing_started: true,
                reached_distance: merge_traversal_reached_distance(2.0),
                elapsed: 0.0,
                timeout: merge_traversal_timeout(2.0),
                link_kind: 1,
            },
            UsingAnimationLink,
            ReachedAnimationLink3d {
                link_entity: link,
                start_point: Vec3::new(0.0, AGENT_HEIGHT / 2.0, 0.0),
                end_point: target,
            },
            AgentTarget3d::Point(target),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);

    // Plenty of ticks for the KCC (desired speed 2.5 m/s) to cover the
    // 2 m crossing on flat open ground.
    for _ in 0..180 {
        world
            .run_system_once(merge_traversal_system)
            .expect("system runs");
        if world.get::<MergeTraversal>(agent).is_none() {
            break;
        }
    }

    assert!(
        world.get::<MergeTraversal>(agent).is_none(),
        "a clear crossing must complete and remove MergeTraversal"
    );
    assert!(world.get::<UsingAnimationLink>(agent).is_none());
    assert!(world.get::<ReachedAnimationLink3d>(agent).is_none());
    assert_eq!(world.get::<AgentRuntime>(agent).unwrap().active_link, None);
    assert!(matches!(
        world.get::<AgentTarget3d>(agent),
        Some(AgentTarget3d::None)
    ));
    assert!(world.get::<PendingMergeRepath>(agent).is_some());
    let position = world.get::<Transform>(agent).unwrap().translation;
    assert!(
        (position.x - target.x).abs() < MERGE_TRAVERSAL_REACHED_DISTANCE + 0.1,
        "agent should have swept to the far portal point, got {position:?}"
    );
    let kcc = world.get::<AgentKcc>(agent).unwrap();
    assert!(!kcc.stuck, "a clear crossing must never latch stuck");
    assert!(!kcc.collision_blocked);

    // `ReachedAnimationLink3d` is synchronized from Landmass one fixed phase
    // later. Completion must consume the stale marker now, otherwise the
    // door-link driver restarts the same merge handoff at its far endpoint.
    drive_door_link_for_agent(&mut world, agent);
    assert!(
        world.get::<MergeTraversal>(agent).is_none(),
        "a completed merge link must not restart from the stale reached marker"
    );

    world
        .run_system_once(resume_pending_merge_repath_system)
        .expect("repath restore runs");
    assert_eq!(
        world.get::<AgentTarget3d>(agent),
        Some(&AgentTarget3d::Point(target)),
        "the real route target must resume after the one-tick corridor clear"
    );
}

#[test]
fn merge_traversal_aligns_with_the_source_portal_before_crossing() {
    use bevy::ecs::system::RunSystemOnce;

    let mut world = harness_world();
    world.insert_resource(PhysicsDisabled(false));
    world.insert_resource(CellPhysicsReadiness::Ready);
    world.init_resource::<Time>();
    world
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));

    let mut physics_world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(1.0, -0.1, 1.0),
        boxddd::Vec3::new(5.0, 0.1, 5.0),
    );
    // Blocks the direct diagonal from the early-reached position to the far
    // endpoint, while leaving the source-alignment and seam-crossing legs
    // clear by more than the capsule radius.
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(1.0, 1.0, 1.0),
        boxddd::Vec3::new(0.25, 2.0, 0.25),
    );
    world.insert_non_send(BoxdddPhysicsContext::from_world(physics_world));

    let archipelago = world.spawn_empty().id();
    let link = world.spawn_empty().id();
    {
        let mut nav = world.resource_mut::<NavArchipelagoState>();
        nav.archipelago = Some(archipelago);
        nav.merge_link_kind_count = 1;
        nav.link_kinds.insert(link, LinkKind::Merge { kind: 1 });
    }
    let source = Vec3::new(0.0, 0.0, 2.0);
    let target = Vec3::new(2.0, 0.0, 2.0);
    let agent = world
        .spawn((
            NavAgent,
            AgentKcc {
                grounded: true,
                ..default()
            },
            Transform::from_xyz(0.0, AGENT_HEIGHT / 2.0, 0.0),
            AgentRuntime::default(),
            ReachedAnimationLink3d {
                link_entity: link,
                start_point: source,
                end_point: target,
            },
            AgentTarget3d::Point(target),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);

    drive_door_link_for_agent(&mut world, agent);
    assert!(world.get::<MergeTraversal>(agent).is_some());

    for _ in 0..420 {
        world
            .run_system_once(merge_traversal_system)
            .expect("system runs");
        if world.get::<MergeTraversal>(agent).is_none() {
            break;
        }
    }

    assert!(
        world.get::<MergeTraversal>(agent).is_none(),
        "a clear source-aligned crossing must complete"
    );
    let kcc = world.get::<AgentKcc>(agent).unwrap();
    assert!(
        !kcc.stuck,
        "the clear staged crossing must not be quarantined"
    );
    assert!(!kcc.collision_blocked);
    let position = world.get::<Transform>(agent).unwrap().translation;
    assert!(
        movement_policy::horizontal_distance(position.to_array(), target.to_array())
            < MERGE_TRAVERSAL_REACHED_DISTANCE + 0.1,
        "agent should finish on the far side, got {position:?}"
    );
}

#[test]
fn drive_door_link_does_not_restart_an_active_merge_traversal() {
    let mut world = harness_world();
    let archipelago = world.spawn_empty().id();
    let link = world.spawn_empty().id();
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago);
    world
        .resource_mut::<NavArchipelagoState>()
        .link_kinds
        .insert(link, LinkKind::Merge { kind: 7 });

    let start = Vec3::new(0.0, AGENT_HEIGHT / 2.0, 0.0);
    let end = Vec3::new(0.4, AGENT_HEIGHT / 2.0, 0.0);
    let agent = world
        .spawn((
            AgentRuntime {
                active_link: Some(LinkKind::Merge { kind: 7 }),
                ..default()
            },
            Transform::from_translation(start),
            ReachedAnimationLink3d {
                link_entity: link,
                start_point: start,
                end_point: end,
            },
            MergeTraversal {
                source: start,
                target: end,
                crossing_started: true,
                reached_distance: merge_traversal_reached_distance(0.4),
                elapsed: 0.25,
                timeout: merge_traversal_timeout(0.4),
                link_kind: 7,
            },
            UsingAnimationLink,
        ))
        .id();

    drive_door_link_for_agent(&mut world, agent);

    let traversal = world
        .get::<MergeTraversal>(agent)
        .expect("an active merge traversal must remain in flight");
    assert_eq!(traversal.elapsed, 0.25);
    assert_eq!(traversal.target, end);
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().active_link,
        Some(LinkKind::Merge { kind: 7 })
    );
}

#[test]
fn drive_merge_link_treats_a_capsule_already_on_the_portal_segment_as_crossing() {
    let mut world = harness_world();
    let archipelago = world.spawn_empty().id();
    let link = world.spawn_empty().id();
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago);
    world
        .resource_mut::<NavArchipelagoState>()
        .link_kinds
        .insert(link, LinkKind::Merge { kind: 9 });

    let source = Vec3::ZERO;
    let target = Vec3::new(0.36, 0.0, 0.06);
    // Mirrors the c49 short seam: Landmass reports the link with the capsule
    // already most of the way between the two validated endpoints.
    let agent = world
        .spawn((
            AgentRuntime::default(),
            Transform::from_xyz(0.27, AGENT_HEIGHT / 2.0, 0.04),
            ReachedAnimationLink3d {
                link_entity: link,
                start_point: source,
                end_point: target,
            },
        ))
        .id();

    drive_door_link_for_agent(&mut world, agent);

    assert!(
        world
            .get::<MergeTraversal>(agent)
            .expect("merge traversal")
            .crossing_started,
        "a capsule already on the portal segment must continue toward the far side"
    );
}

#[test]
fn merge_link_input_refresh_preserves_costs_and_consumes_its_request() {
    let mut world = World::new();
    let mut overrides = AgentTypeIndexCostOverrides::default();
    assert!(overrides.set_type_index_cost(3, 4.5));
    let agent = world
        .spawn((overrides, RefreshLandmassAnimationLinkInput))
        .id();
    refresh_landmass_animation_link_input(&mut world);

    assert!(
        world.get::<AgentTypeIndexCostOverrides>(agent).is_none(),
        "Landmass must take its non-early-return input branch for the transition"
    );
    assert!(
        world
            .get::<SuspendedLandmassTypeIndexCosts>(agent)
            .is_some()
    );
    assert!(
        world
            .get::<RefreshLandmassAnimationLinkInput>(agent)
            .is_none(),
        "the schedule-boundary refresh must run exactly once per transition"
    );

    restore_landmass_type_index_costs(&mut world);

    let refreshed = world
        .get::<AgentTypeIndexCostOverrides>(agent)
        .expect("restored overrides");
    assert_eq!(refreshed.iter().collect::<Vec<_>>(), vec![(&3, &4.5)]);
    assert!(
        world
            .get::<SuspendedLandmassTypeIndexCosts>(agent)
            .is_none(),
        "the cost suspension must last only for Landmass's input system"
    );
}

#[test]
fn merge_traversal_system_quarantines_the_link_and_preserves_the_real_destination() {
    use bevy::ecs::system::RunSystemOnce;

    let mut world = World::new();
    world.init_resource::<DebugAgentRoster>();
    world.init_resource::<NavArchipelagoState>();
    // Three merge kinds exist in this build (1..=3); the blocked
    // crossing below uses kind 2, so the surviving allow-list must be
    // {0 (door), 1, 3} -- proof the quarantine is exactly this one
    // link, not every merge portal.
    world
        .resource_mut::<NavArchipelagoState>()
        .merge_link_kind_count = 3;
    world.insert_resource(PhysicsDisabled(false));
    world.insert_resource(CellPhysicsReadiness::Ready);
    world.init_resource::<Time>();
    world
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
    // Same real-player-filter requirement as the sweep test above: both
    // the floor and the wall must use `add_player_compatible_floor`'s
    // filter, not `add_fixture_box`'s, or `merge_traversal_system`'s
    // real collision query never sees either shape and the agent free-
    // falls through both -- which happened to still end in `stuck` (via
    // the vertical-gap guard on `nav_point_reached` never being
    // satisfied while falling) for the wrong reason entirely, not
    // because the wall actually blocked it.
    let mut physics_world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(0.0, -0.1, 0.0),
        boxddd::Vec3::new(5.0, 0.1, 5.0),
    );
    // A wall immediately in front (+X) of the agent's start position,
    // between it and the portal's far point.
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(1.0, 1.0, 0.0),
        boxddd::Vec3::new(0.1, 2.0, 5.0),
    );
    world.insert_non_send(BoxdddPhysicsContext::from_world(physics_world));

    let target = Vec3::new(5.0, AGENT_HEIGHT / 2.0, 0.0);
    let agent = world
        .spawn((
            NavAgent,
            AgentKcc {
                grounded: true,
                ..default()
            },
            Transform::from_xyz(0.0, AGENT_HEIGHT / 2.0, 0.0),
            AgentRuntime {
                active_link: Some(LinkKind::Merge { kind: 2 }),
                // A live travel intent, to prove #162 preserves it
                // (the pre-#162 behaviour cleared it unconditionally).
                travel_intent: Some(TravelIntent {
                    generation: RouteGeneration(1),
                    door_form_id: 0x77,
                }),
                ..default()
            },
            MergeTraversal {
                source: Vec3::new(0.0, AGENT_HEIGHT / 2.0, 0.0),
                target,
                crossing_started: true,
                reached_distance: merge_traversal_reached_distance(5.0),
                elapsed: 0.0,
                timeout: merge_traversal_timeout(5.0),
                link_kind: 2,
            },
            UsingAnimationLink,
            AgentTarget3d::Point(target),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);

    // The agent makes genuine initial progress closing the ~0.55 m gap
    // to the wall before it wedges (see
    // `a_blocked_agent_reports_its_real_near_zero_velocity`'s own
    // 120-tick budget for the same fixture shape) and then keeps
    // creeping forward by less than a measurable step forever --
    // that's exactly why this traversal uses an absolute deadline
    // rather than a resettable no-progress counter (see
    // `MERGE_TRAVERSAL_TIMEOUT_FACTOR`'s doc comment). Run comfortably
    // past the computed timeout in fixed-tick terms.
    let dt = 1.0 / 60.0;
    let ticks_to_timeout = (merge_traversal_timeout(5.0) / dt).ceil() as usize;
    for _ in 0..(ticks_to_timeout + 60) {
        world
            .run_system_once(merge_traversal_system)
            .expect("system runs");
        if world.get::<MergeTraversal>(agent).is_none() {
            break;
        }
    }

    let kcc = world.get::<AgentKcc>(agent).unwrap();
    assert!(
        kcc.stuck,
        "a wall-blocked crossing must report stuck, not silently keep pushing forever"
    );
    assert!(kcc.collision_blocked);
    assert!(
        world.get::<MergeTraversal>(agent).is_none(),
        "the traversal must stop, not keep the agent pinned mid-portal indefinitely"
    );
    assert!(world.get::<UsingAnimationLink>(agent).is_none());
    assert!(world.get::<ReachedAnimationLink3d>(agent).is_none());
    assert_eq!(world.get::<AgentRuntime>(agent).unwrap().active_link, None);
    let position = world.get::<Transform>(agent).unwrap().translation;
    assert!(
        position.x < target.x - 1.0,
        "the agent must never teleport through the wall to the far portal point, got {position:?}"
    );

    // Issue #162: exactly this one link's kind is quarantined.
    assert_eq!(
        world
            .get::<AgentRuntime>(agent)
            .unwrap()
            .quarantined_merge_link_kinds,
        BTreeSet::from([2]),
        "only the specific blocked link's kind must be quarantined"
    );
    // The allow-list keeps every other kind, including the reserved
    // door kind 0 -- a blocked merge portal must never lock a door.
    let permitted = world.get::<PermittedAnimationLinks>(agent).unwrap();
    match permitted {
        PermittedAnimationLinks::Kinds(kinds) => {
            assert_eq!(kinds.as_ref(), &HashSet::from([0, 1, 3]));
        }
        PermittedAnimationLinks::All => panic!("a quarantine must restrict, not stay `All`"),
    }

    // The real target/destination is kept, not cleared: it is
    // transiently blanked to force a genuine repath
    // (`resume_pending_merge_repath_system`'s doc comment) and
    // captured in `PendingMergeRepath` for that system to restore.
    assert!(
        matches!(world.get::<AgentTarget3d>(agent), Some(AgentTarget3d::None)),
        "the target is transiently blanked to force a repath, not removed outright"
    );
    let pending = world
        .get::<PendingMergeRepath>(agent)
        .expect("the real target must be captured for the next tick to restore");
    assert!(matches!(
        pending.target,
        AgentTargetSnapshot::Point(point) if point == target
    ));
    assert_eq!(
        world.get::<AgentRuntime>(agent).unwrap().travel_intent,
        Some(TravelIntent {
            generation: RouteGeneration(1),
            door_form_id: 0x77,
        }),
        "issue #162: an in-progress travel intent is the real destination and must survive a quarantine, unlike the pre-#162 wholesale clear"
    );
}

#[test]
fn resume_pending_merge_repath_system_restores_the_captured_target() {
    use bevy::ecs::system::RunSystemOnce;

    let mut world = World::new();
    let target = Vec3::new(3.0, 1.0, 4.0);
    let agent = world
        .spawn((
            PendingMergeRepath {
                target: AgentTargetSnapshot::Point(target),
            },
            AgentTarget3d::None,
        ))
        .id();

    world
        .run_system_once(resume_pending_merge_repath_system)
        .expect("system runs");

    assert!(
        matches!(
            world.get::<AgentTarget3d>(agent),
            Some(AgentTarget3d::Point(point)) if *point == target
        ),
        "the real target must be restored exactly"
    );
    assert!(
        world.get::<PendingMergeRepath>(agent).is_none(),
        "the marker must be consumed regardless of outcome"
    );
}

#[test]
fn resume_pending_merge_repath_system_restores_an_entity_target() {
    use bevy::ecs::system::RunSystemOnce;

    let mut world = World::new();
    let followed = world.spawn_empty().id();
    let agent = world
        .spawn((
            PendingMergeRepath {
                target: AgentTargetSnapshot::Entity(followed),
            },
            AgentTarget3d::None,
        ))
        .id();

    world
        .run_system_once(resume_pending_merge_repath_system)
        .expect("system runs");

    assert!(matches!(
        world.get::<AgentTarget3d>(agent),
        Some(AgentTarget3d::Entity(entity)) if *entity == followed
    ));
}

#[test]
fn resume_pending_merge_repath_system_does_not_clobber_a_fresh_retarget() {
    use bevy::ecs::system::RunSystemOnce;

    let mut world = World::new();
    let stale_target = Vec3::new(3.0, 1.0, 4.0);
    let fresh_target = Vec3::new(9.0, 1.0, 9.0);
    let agent = world
        .spawn((
            PendingMergeRepath {
                target: AgentTargetSnapshot::Point(stale_target),
            },
            AgentTarget3d::Point(fresh_target),
        ))
        .id();

    world
        .run_system_once(resume_pending_merge_repath_system)
        .expect("system runs");

    assert!(
        matches!(
            world.get::<AgentTarget3d>(agent),
            Some(AgentTarget3d::Point(point)) if *point == fresh_target
        ),
        "a retarget issued during the gap must win over the stale captured target"
    );
    assert!(
        world.get::<PendingMergeRepath>(agent).is_none(),
        "the marker must still be consumed even when the restore is skipped"
    );
}

#[test]
fn clear_merge_link_quarantine_resets_tracked_kinds_and_the_component() {
    let mut world = World::new();
    let mut runtime = AgentRuntime::default();
    runtime.quarantined_merge_link_kinds.insert(2);
    runtime.quarantined_merge_link_kinds.insert(3);
    let agent = world
        .spawn((
            runtime,
            PermittedAnimationLinks::Kinds(Arc::new(HashSet::from([0, 1]))),
        ))
        .id();

    clear_merge_link_quarantine(&mut world, agent);

    assert!(
        world
            .get::<AgentRuntime>(agent)
            .unwrap()
            .quarantined_merge_link_kinds
            .is_empty(),
        "the tracked kind set must be reset to empty"
    );
    assert!(matches!(
        world.get::<PermittedAnimationLinks>(agent),
        Some(PermittedAnimationLinks::All)
    ));
}

#[test]
fn goto_agent_clears_a_live_merge_link_quarantine() {
    let mut world = harness_world();
    world.init_resource::<Time>();
    let mut runtime = AgentRuntime::default();
    runtime.quarantined_merge_link_kinds.insert(1);
    let agent = world
        .spawn((
            NavAgent,
            runtime,
            AgentKcc::default(),
            PermittedAnimationLinks::Kinds(Arc::new(HashSet::from([0]))),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);

    tna_command(&mut world, &invocation(&["goto", "5", "6", "7"])).expect("goto succeeds");

    assert!(
        world
            .get::<AgentRuntime>(agent)
            .unwrap()
            .quarantined_merge_link_kinds
            .is_empty()
    );
    assert!(matches!(
        world.get::<PermittedAnimationLinks>(agent),
        Some(PermittedAnimationLinks::All)
    ));
}

#[test]
fn request_travel_clears_a_live_merge_link_quarantine() {
    let mut world = harness_world();
    let mut runtime = AgentRuntime::default();
    runtime.quarantined_merge_link_kinds.insert(1);
    let agent = world.spawn((NavAgent, runtime, AgentKcc::default())).id();
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

    request_travel(&mut world, 0, 0x99).expect("travel request succeeds");

    assert!(
        world
            .get::<AgentRuntime>(agent)
            .unwrap()
            .quarantined_merge_link_kinds
            .is_empty()
    );
}

#[test]
fn validate_merge_link_collision_accepts_a_clean_connected_floor() {
    let mut physics_world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(0.0, -0.1, 0.0),
        boxddd::Vec3::new(5.0, 0.1, 5.0),
    );
    let mover = fixture_capsule();
    let collision_filter = player::player_collision_filter();
    let support_filter = player::stair_support_filter();

    let result = validate_merge_link_collision(
        &mut physics_world,
        &mover,
        collision_filter,
        support_filter,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn validate_merge_link_collision_rejects_a_ledge_into_the_void() {
    let mut physics_world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    // Floor only under the near point (x in [-2, 2]); the far point at
    // x = 5 overhangs nothing.
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(0.0, -0.1, 0.0),
        boxddd::Vec3::new(2.0, 0.1, 5.0),
    );
    let mover = fixture_capsule();
    let collision_filter = player::player_collision_filter();
    let support_filter = player::stair_support_filter();

    let result = validate_merge_link_collision(
        &mut physics_world,
        &mover,
        collision_filter,
        support_filter,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(5.0, 0.0, 0.0),
    );
    assert!(
        matches!(result, Err(MergeLinkRejection::NoGroundSupport)),
        "{result:?}"
    );
}

#[test]
fn validate_merge_link_collision_rejects_a_swept_blocked_crossing() {
    let mut physics_world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(0.0, -0.1, 0.0),
        boxddd::Vec3::new(5.0, 0.1, 5.0),
    );
    // A wall spanning the full crossing width, directly between the
    // two portal points.
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(2.0, 1.0, 0.0),
        boxddd::Vec3::new(0.1, 2.0, 5.0),
    );
    let mover = fixture_capsule();
    let collision_filter = player::player_collision_filter();
    let support_filter = player::stair_support_filter();

    let result = validate_merge_link_collision(
        &mut physics_world,
        &mover,
        collision_filter,
        support_filter,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
    );
    assert!(
        matches!(result, Err(MergeLinkRejection::SweptBlocked)),
        "{result:?}"
    );
}

#[test]
fn active_link_description_reports_merge_door_and_travel_reached() {
    let mut runtime = AgentRuntime::default();
    assert_eq!(active_link_description(&runtime), None);

    runtime.active_link = Some(LinkKind::Merge { kind: 1 });
    assert_eq!(active_link_description(&runtime), Some("merge".to_string()));

    runtime.active_link = Some(LinkKind::Door { form_id: 0x99 });
    assert_eq!(
        active_link_description(&runtime),
        Some("door 00000099".to_string())
    );

    runtime.active_link = None;
    runtime.door_link = door_link::DoorLinkState::TravelReached {
        door_form_id: 0x99,
        destination_cell_form_id: 0xC0DE,
    };
    assert_eq!(
        active_link_description(&runtime),
        Some("door 00000099 cell 0000c0de".to_string())
    );
}
