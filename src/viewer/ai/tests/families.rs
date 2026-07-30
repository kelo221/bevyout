use super::*;

fn obs(position: [f32; 3], nav_reached: bool) -> FamilyObservation {
    FamilyObservation::new(position, nav_reached, false)
}

fn failed_obs(position: [f32; 3]) -> FamilyObservation {
    FamilyObservation::new(position, false, true)
}

fn follow_obs(actor: [f32; 3], leader: [f32; 3]) -> FamilyObservation {
    FamilyObservation {
        target_position: Some(leader),
        ..FamilyObservation::new(actor, false, false)
    }
}

#[test]
fn dispatch_maps_the_seven_supported_types() {
    assert_eq!(
        PackageFamily::from_package_type(1),
        Some(PackageFamily::Follow)
    );
    assert_eq!(
        PackageFamily::from_package_type(3),
        Some(PackageFamily::Eat)
    );
    assert_eq!(
        PackageFamily::from_package_type(4),
        Some(PackageFamily::Sleep)
    );
    assert_eq!(
        PackageFamily::from_package_type(5),
        Some(PackageFamily::Idle)
    );
    assert_eq!(
        PackageFamily::from_package_type(6),
        Some(PackageFamily::Travel)
    );
    assert_eq!(
        PackageFamily::from_package_type(12),
        Some(PackageFamily::Sandbox)
    );
    assert_eq!(
        PackageFamily::from_package_type(13),
        Some(PackageFamily::Patrol)
    );
    // An unmapped type is still not driven.
    assert_eq!(PackageFamily::from_package_type(9), None);
}

#[test]
fn only_sandbox_refuses_to_open_doors() {
    assert!(!PackageFamily::Sandbox.opens_doors());
    for family in [
        PackageFamily::Travel,
        PackageFamily::Patrol,
        PackageFamily::Idle,
        PackageFamily::Eat,
        PackageFamily::Sleep,
        PackageFamily::Follow,
    ] {
        assert!(family.opens_doors(), "{} should open doors", family.label());
    }
}

#[test]
fn travel_routes_then_completes_on_arrival() {
    let mut driver = FamilyDriver::new(
        PackageFamily::Travel,
        vec![Waypoint::at([10.0, 0.0, 0.0])],
        0.5,
    );
    // First tick: routes to the destination.
    let step = driver.tick(&obs([0.0, 0.0, 0.0], false), 0.1);
    assert_eq!(step.request, Some(FamilyRequest::Route([10.0, 0.0, 0.0])));
    assert_eq!(step.signal, LifecycleSignal::Continue);
    // Not re-issued while unchanged and still en route.
    let step = driver.tick(&obs([5.0, 0.0, 0.0], false), 0.1);
    assert_eq!(step.request, None);
    assert_eq!(step.signal, LifecycleSignal::Continue);
    // Arrival: stop steering and complete.
    let step = driver.tick(&obs([10.0, 0.0, 0.0], true), 0.1);
    assert_eq!(step.request, Some(FamilyRequest::Stop));
    assert_eq!(step.signal, LifecycleSignal::Complete);
}

#[test]
fn arrival_tolerance_is_a_radius_not_the_nav_flag() {
    let mut inside = FamilyDriver::new(
        PackageFamily::Travel,
        vec![Waypoint::at([0.0, 0.0, 0.0])],
        1.0,
    );
    // Within the 1.0 tolerance but nav has not latched reached: still counts.
    let step = inside.tick(&obs([0.6, 0.0, 0.6], false), 0.1);
    assert_eq!(step.signal, LifecycleSignal::Complete);

    let mut outside = FamilyDriver::new(
        PackageFamily::Travel,
        vec![Waypoint::at([0.0, 0.0, 0.0])],
        1.0,
    );
    // Just outside the tolerance: keeps routing.
    let step = outside.tick(&obs([1.5, 0.0, 0.0], false), 0.1);
    assert_eq!(step.signal, LifecycleSignal::Continue);
    assert!(matches!(step.request, Some(FamilyRequest::Route(_))));
}

#[test]
fn travel_re_issues_route_on_failure_then_fails_the_attempt() {
    let mut driver = FamilyDriver::new(
        PackageFamily::Travel,
        vec![Waypoint::at([10.0, 0.0, 0.0])],
        0.5,
    );
    driver.tick(&obs([0.0, 0.0, 0.0], false), 0.1); // initial route
    // Each failure below the ceiling re-issues the route rather than failing.
    for _ in 0..MAX_ROUTE_REISSUES {
        let step = driver.tick(&failed_obs([0.0, 0.0, 0.0]), 0.1);
        assert_eq!(
            step.request,
            Some(FamilyRequest::Route([10.0, 0.0, 0.0])),
            "route re-issued on failure"
        );
        assert_eq!(step.signal, LifecycleSignal::Continue);
    }
    // One failure past the ceiling gives up this attempt.
    let step = driver.tick(&failed_obs([0.0, 0.0, 0.0]), 0.1);
    assert_eq!(step.signal, LifecycleSignal::Fail);
}

#[test]
fn patrol_visits_markers_in_order_and_cycles() {
    let markers = vec![
        Waypoint {
            position: [0.0, 0.0, 0.0],
            wait_seconds: 0.0,
            orientation_yaw: None,
            interaction_point: None,
        },
        Waypoint {
            position: [10.0, 0.0, 0.0],
            wait_seconds: 0.0,
            orientation_yaw: None,
            interaction_point: None,
        },
        Waypoint {
            position: [10.0, 0.0, 10.0],
            wait_seconds: 0.0,
            orientation_yaw: None,
            interaction_point: None,
        },
    ];
    let mut driver = FamilyDriver::new(PackageFamily::Patrol, markers, 0.5);
    // Route to marker 0.
    let step = driver.tick(&obs([-5.0, 0.0, 0.0], false), 0.1);
    assert_eq!(step.request, Some(FamilyRequest::Route([0.0, 0.0, 0.0])));
    assert_eq!(driver.marker_index(), 0);
    // Reach 0 (no wait) -> advance to 1.
    let step = driver.tick(&obs([0.0, 0.0, 0.0], true), 0.1);
    assert_eq!(step.signal, LifecycleSignal::AdvanceStep);
    assert_eq!(step.request, Some(FamilyRequest::Route([10.0, 0.0, 0.0])));
    assert_eq!(driver.marker_index(), 1);
    // Reach 1 -> advance to 2.
    let step = driver.tick(&obs([10.0, 0.0, 0.0], true), 0.1);
    assert_eq!(step.signal, LifecycleSignal::AdvanceStep);
    assert_eq!(driver.marker_index(), 2);
    // Reach 2 -> cycles back to 0.
    let step = driver.tick(&obs([10.0, 0.0, 10.0], true), 0.1);
    assert_eq!(step.signal, LifecycleSignal::AdvanceStep);
    assert_eq!(driver.marker_index(), 0);
    assert_eq!(step.request, Some(FamilyRequest::Route([0.0, 0.0, 0.0])));
}

#[test]
fn patrol_waits_and_idles_at_a_marker_before_advancing() {
    let markers = vec![
        Waypoint {
            position: [0.0, 0.0, 0.0],
            wait_seconds: 2.0,
            orientation_yaw: None,
            interaction_point: None,
        },
        Waypoint::at([10.0, 0.0, 0.0]),
    ];
    let mut driver = FamilyDriver::new(PackageFamily::Patrol, markers, 0.5);
    driver.tick(&obs([-1.0, 0.0, 0.0], false), 0.1); // route to 0
    // Arrive: begins the wait, idling in place, NOT advancing yet.
    let step = driver.tick(&obs([0.0, 0.0, 0.0], true), 0.1);
    assert_eq!(
        step.request,
        Some(FamilyRequest::Play(FamilyAnimation::Idle))
    );
    assert_eq!(step.signal, LifecycleSignal::Continue);
    assert_eq!(driver.step_label(), "waiting");
    assert_eq!(driver.marker_index(), 0);
    // Still waiting mid-window.
    let step = driver.tick(&obs([0.0, 0.0, 0.0], true), 1.0);
    assert_eq!(step.signal, LifecycleSignal::Continue);
    assert_eq!(driver.marker_index(), 0);
    // Wait elapses -> advance to marker 1.
    let step = driver.tick(&obs([0.0, 0.0, 0.0], true), 1.5);
    assert_eq!(step.signal, LifecycleSignal::AdvanceStep);
    assert_eq!(driver.marker_index(), 1);
}

#[test]
fn idle_routes_to_the_location_then_plays_idle_forever() {
    let mut driver = FamilyDriver::new(
        PackageFamily::Idle,
        vec![Waypoint::at([4.0, 0.0, 0.0])],
        0.5,
    );
    let step = driver.tick(&obs([0.0, 0.0, 0.0], false), 0.1);
    assert_eq!(step.request, Some(FamilyRequest::Route([4.0, 0.0, 0.0])));
    // Arrive -> idle, never completes.
    let step = driver.tick(&obs([4.0, 0.0, 0.0], true), 0.1);
    assert_eq!(
        step.request,
        Some(FamilyRequest::Play(FamilyAnimation::Idle))
    );
    assert_eq!(step.signal, LifecycleSignal::Continue);
    assert_eq!(driver.step_label(), "idling");
    // Idempotent play: no repeat request while already idling.
    let step = driver.tick(&obs([4.0, 0.0, 0.0], true), 0.1);
    assert_eq!(step.request, None);
    assert_eq!(step.signal, LifecycleSignal::Continue);
}

#[test]
fn idle_carries_the_authored_orientation() {
    let driver = FamilyDriver::new(
        PackageFamily::Idle,
        vec![Waypoint {
            position: [0.0, 0.0, 0.0],
            wait_seconds: 0.0,
            orientation_yaw: Some(1.5),
            interaction_point: None,
        }],
        0.5,
    );
    assert_eq!(driver.current_orientation_yaw(), Some(1.5));
}

#[test]
fn eat_routes_then_occupies_and_plays_eat() {
    let mut driver = FamilyDriver::new(
        PackageFamily::Eat,
        vec![Waypoint {
            position: [3.0, 0.0, 0.0],
            wait_seconds: 0.0,
            orientation_yaw: None,
            interaction_point: Some(0xF00D),
        }],
        0.5,
    );
    let step = driver.tick(&obs([0.0, 0.0, 0.0], false), 0.1);
    assert_eq!(step.request, Some(FamilyRequest::Route([3.0, 0.0, 0.0])));
    assert_eq!(driver.occupied_point(), None);
    // Arrive: claims the furniture and plays the eat state.
    let step = driver.tick(&obs([3.0, 0.0, 0.0], true), 0.1);
    assert_eq!(
        step.request,
        Some(FamilyRequest::Play(FamilyAnimation::Eat))
    );
    assert_eq!(step.signal, LifecycleSignal::Continue);
    assert_eq!(driver.occupied_point(), Some(0xF00D));
    assert_eq!(driver.step_label(), "occupying");
}

#[test]
fn sleep_releases_its_occupancy_on_preempt() {
    let mut driver = FamilyDriver::new(
        PackageFamily::Sleep,
        vec![Waypoint {
            position: [0.0, 0.0, 0.0],
            wait_seconds: 0.0,
            orientation_yaw: None,
            interaction_point: Some(0xBED),
        }],
        0.5,
    );
    driver.tick(&obs([0.0, 0.0, 0.0], true), 0.1); // occupy
    assert_eq!(driver.occupied_point(), Some(0xBED));
    // Preempt/complete releases the claim and returns it for the registry.
    assert_eq!(driver.release(), Some(0xBED));
    assert_eq!(driver.occupied_point(), None);
    // Idempotent release.
    assert_eq!(driver.release(), None);
}

#[test]
fn interaction_point_selection_picks_nearest_free() {
    let waypoints = vec![
        Waypoint {
            position: [10.0, 0.0, 0.0],
            wait_seconds: 0.0,
            orientation_yaw: None,
            interaction_point: Some(1),
        },
        Waypoint {
            position: [2.0, 0.0, 0.0],
            wait_seconds: 0.0,
            orientation_yaw: None,
            interaction_point: Some(2),
        },
    ];
    let free = HashSet::new();
    assert_eq!(
        select_interaction_point(&waypoints, [0.0, 0.0, 0.0], &free),
        Some(1),
        "nearest is index 1 (point 2)"
    );
    // With point 2 occupied, the farther free point 1 is chosen instead.
    let mut occupied = HashSet::new();
    occupied.insert(2u32);
    assert_eq!(
        select_interaction_point(&waypoints, [0.0, 0.0, 0.0], &occupied),
        Some(0)
    );
    // Everything occupied -> no free furniture.
    occupied.insert(1u32);
    assert_eq!(
        select_interaction_point(&waypoints, [0.0, 0.0, 0.0], &occupied),
        None
    );
}

// -- Follow (#198) -------------------------------------------------------

#[test]
fn follow_closes_when_past_the_outer_band_and_re_paths_the_moving_leader() {
    // Band [2, 5]: hold within 2m, chase past 5m.
    let mut driver = FamilyDriver::follow(2.0, 5.0, 0.5);
    // Leader 8m away (> outer): start closing, route to it.
    let step = driver.tick(&follow_obs([0.0, 0.0, 0.0], [8.0, 0.0, 0.0]), 0.1);
    assert_eq!(step.request, Some(FamilyRequest::Route([8.0, 0.0, 0.0])));
    assert_eq!(step.signal, LifecycleSignal::Continue);
    assert_eq!(driver.step_label(), "routing");
    // Leader took a small step (< repath epsilon-ish): no fresh route spam.
    let step = driver.tick(&follow_obs([1.0, 0.0, 0.0], [8.1, 0.0, 0.0]), 0.1);
    assert_eq!(step.request, None);
    // Leader moved well away: re-path to the new leader point.
    let step = driver.tick(&follow_obs([2.0, 0.0, 0.0], [12.0, 0.0, 0.0]), 0.1);
    assert_eq!(step.request, Some(FamilyRequest::Route([12.0, 0.0, 0.0])));
}

#[test]
fn follow_hysteresis_does_not_stutter_at_the_band_edge() {
    // Band [2, 5]. The follower must not flip start/stop while the leader
    // hovers *between* the thresholds (the dead band).
    let mut driver = FamilyDriver::follow(2.0, 5.0, 0.5);
    // Start closing (leader at 6m > outer).
    let start = driver.tick(&follow_obs([0.0, 0.0, 0.0], [6.0, 0.0, 0.0]), 0.1);
    assert!(matches!(start.request, Some(FamilyRequest::Route(_))));
    // Now actor is 3.5m from the leader: inside the dead band (2..5). A
    // single-threshold follower would already stop; hysteresis keeps it
    // closing (no Stop), so no stutter.
    let mid = driver.tick(&follow_obs([2.5, 0.0, 0.0], [6.0, 0.0, 0.0]), 0.1);
    assert_ne!(mid.request, Some(FamilyRequest::Stop));
    assert_eq!(driver.step_label(), "routing");
    // Reaches the inner band (1.5m <= 2): now it holds.
    let close = driver.tick(&follow_obs([4.5, 0.0, 0.0], [6.0, 0.0, 0.0]), 0.1);
    assert_eq!(close.request, Some(FamilyRequest::Stop));
    assert_eq!(driver.step_label(), "idling");
    // Leader drifts to 3.5m again (still inside the dead band): the holder
    // stays put -- does NOT resume closing. No stutter on the way back in.
    let hold = driver.tick(&follow_obs([4.5, 0.0, 0.0], [8.0, 0.0, 0.0]), 0.1);
    // 3.5m < outer(5): still holding, only an (idempotent) idle at most.
    assert_ne!(hold.request, Some(FamilyRequest::Route([8.0, 0.0, 0.0])));
}

#[test]
fn follow_holds_and_idles_within_the_band() {
    let mut driver = FamilyDriver::follow(2.0, 5.0, 0.5);
    // Leader 3m away (inside band): hold, play idle, never route.
    let step = driver.tick(&follow_obs([0.0, 0.0, 0.0], [3.0, 0.0, 0.0]), 0.1);
    assert_eq!(
        step.request,
        Some(FamilyRequest::Play(FamilyAnimation::Idle))
    );
    assert_eq!(step.signal, LifecycleSignal::Continue);
    // Idempotent: no repeat idle while still holding.
    let step = driver.tick(&follow_obs([0.0, 0.0, 0.0], [3.0, 0.0, 0.0]), 0.1);
    assert_eq!(step.request, None);
}

#[test]
fn follow_target_loss_stops_and_idles_without_completing() {
    let mut driver = FamilyDriver::follow(2.0, 5.0, 0.5);
    // Close toward a leader first so there is an active route to clear.
    driver.tick(&follow_obs([0.0, 0.0, 0.0], [9.0, 0.0, 0.0]), 0.1);
    // Leader vanishes: clear the route, then idle -- and never complete.
    let lost = FamilyObservation::new([1.0, 0.0, 0.0], false, false);
    let step = driver.tick(&lost, 0.1);
    assert_eq!(step.request, Some(FamilyRequest::Stop));
    assert_eq!(step.signal, LifecycleSignal::Continue);
    assert_eq!(driver.step_label(), "idling");
    let step = driver.tick(&lost, 0.1);
    assert_eq!(
        step.request,
        Some(FamilyRequest::Play(FamilyAnimation::Idle))
    );
    assert_eq!(step.signal, LifecycleSignal::Continue);
}

#[test]
fn follow_blocked_by_a_locked_door_names_it_and_abandons() {
    let mut driver = FamilyDriver::follow(2.0, 5.0, 0.5);
    driver.tick(&follow_obs([0.0, 0.0, 0.0], [20.0, 0.0, 0.0]), 0.1);
    // The route gave up on a locked door: name it, stop, fail the attempt.
    let blocked = FamilyObservation {
        route_failed: true,
        blocking_door: Some(0x0001_ABCD),
        ..follow_obs([5.0, 0.0, 0.0], [20.0, 0.0, 0.0])
    };
    let step = driver.tick(&blocked, 0.1);
    assert_eq!(step.request, Some(FamilyRequest::Stop));
    assert_eq!(step.signal, LifecycleSignal::Fail);
    assert_eq!(driver.blocked_door(), Some(0x0001_ABCD));
    assert_eq!(driver.step_label(), "blocked");
}

#[test]
fn follow_plain_no_path_keeps_trying_the_leader_not_the_door_terminal() {
    // A no-path *without* a blocking door is not the named-door terminal:
    // the follow keeps re-pathing the leader (it may just have stepped
    // off-mesh for a frame), so it does NOT report a blocked door.
    let mut driver = FamilyDriver::follow(2.0, 5.0, 0.5);
    let failing = FamilyObservation {
        route_failed: true,
        blocking_door: None,
        ..follow_obs([0.0, 0.0, 0.0], [9.0, 0.0, 0.0])
    };
    let step = driver.tick(&failing, 0.1);
    assert!(matches!(step.request, Some(FamilyRequest::Route(_))));
    assert_eq!(step.signal, LifecycleSignal::Continue);
    assert_eq!(driver.blocked_door(), None);
}

// -- Sandbox / Wander (#198) --------------------------------------------

#[test]
fn wander_routes_within_the_radius_and_idles_between_legs() {
    let center = [10.0, 0.0, 10.0];
    let radius = 6.0;
    let mut driver = FamilyDriver::wander(center, radius, 2.0, 0x00C0_FFEE, 0.5);
    // First tick draws a roam point and routes to it -- within the radius.
    let step = driver.tick(&FamilyObservation::new(center, false, false), 0.1);
    let Some(FamilyRequest::Route(point)) = step.request else {
        panic!("expected a roam route, got {:?}", step.request);
    };
    let dx = point[0] - center[0];
    let dz = point[2] - center[2];
    assert!(
        (dx * dx + dz * dz).sqrt() <= radius + 1e-3,
        "roam point {point:?} escaped the radius"
    );
    assert_eq!(point[1], center[1], "roam stays on the ground plane");
    // Arrive: idle in place for the dwell, no immediate re-route.
    let step = driver.tick(&FamilyObservation::new(point, true, false), 0.1);
    assert_eq!(
        step.request,
        Some(FamilyRequest::Play(FamilyAnimation::Idle))
    );
    assert_eq!(driver.step_label(), "idling");
    // Dwell elapses: roam to the next point (a step boundary).
    let step = driver.tick(&FamilyObservation::new(point, true, false), 5.0);
    assert!(matches!(step.request, Some(FamilyRequest::Route(_))));
    assert_eq!(step.signal, LifecycleSignal::AdvanceStep);
}

#[test]
fn wander_point_selection_is_deterministic_for_a_seed() {
    let sequence = |seed: u64| -> Vec<[f32; 3]> {
        let mut state = WanderState::new([0.0, 0.0, 0.0], 5.0, 0.0, seed);
        (0..8).map(|_| state.next_point()).collect()
    };
    // Same seed -> identical sequence (reproducible across runs/resumes).
    assert_eq!(sequence(0xABCD), sequence(0xABCD));
    // Different seeds diverge (not a constant generator).
    assert_ne!(sequence(0xABCD), sequence(0x1234));
}

#[test]
fn wander_points_are_always_bounded_by_the_radius() {
    let radius = 4.0;
    let mut state = WanderState::new([1.0, 2.0, 3.0], radius, 0.0, 42);
    for _ in 0..2000 {
        let point = state.next_point();
        let dx = point[0] - 1.0;
        let dz = point[2] - 3.0;
        assert!((dx * dx + dz * dz).sqrt() <= radius + 1e-3);
        assert_eq!(point[1], 2.0);
    }
}

#[test]
fn wander_re_rolls_an_unreachable_point_then_fails_after_the_ceiling() {
    let mut driver = FamilyDriver::wander([0.0, 0.0, 0.0], 5.0, 1.0, 7, 0.5);
    driver.tick(&FamilyObservation::new([0.0, 0.0, 0.0], false, false), 0.1);
    // Each failure below the ceiling re-rolls to a fresh routed point.
    for _ in 0..MAX_ROUTE_REISSUES {
        let step = driver.tick(&failed_obs([0.0, 0.0, 0.0]), 0.1);
        assert!(matches!(step.request, Some(FamilyRequest::Route(_))));
        assert_eq!(step.signal, LifecycleSignal::Continue);
    }
    // One past the ceiling gives up the attempt.
    let step = driver.tick(&failed_obs([0.0, 0.0, 0.0]), 0.1);
    assert_eq!(step.signal, LifecycleSignal::Fail);
}
