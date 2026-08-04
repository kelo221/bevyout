use super::*;
use bevy::ecs::system::RunSystemOnce;
use bevy_landmass::prelude::AgentTarget3d;

use crate::viewer::actor_animation::ActorAnimationIntent;

/// A minimal world with one controllable actor: only the components the
/// driver reads and writes, plus the resources it needs.
fn controller_world(controller: ActorPackageController, position: Vec3) -> (World, Entity) {
    let mut world = World::new();
    world.insert_resource(Time::<()>::default());
    world.init_resource::<PackageInteractionOccupancy>();
    // `route_agent_to_target` (the `FamilyRequest::Route` seam) reads the
    // nav archipelago state; seed the empty one a minimal world needs.
    api::insert_test_archipelago_state(&mut world);
    let entity = world
        .spawn((
            Transform::from_translation(position),
            ActorAnimationIntent::default(),
            controller,
        ))
        .id();
    api::insert_test_nav_agent(&mut world, entity);
    (world, entity)
}

fn requested(world: &World, entity: Entity) -> Option<ActorAnimationState> {
    world
        .get::<ActorAnimationIntent>(entity)
        .and_then(|intent| intent.requested)
}

/// **The one-authority invariant** (verdict §2.3): a driven family issues
/// nav and animation *requests* and never writes the actor's translation.
#[test]
fn family_driver_writes_no_transform_translation() {
    // An idle family whose waypoint is right where the actor already stands:
    // the driver skips routing and asks for the idle animation immediately.
    let controller = ActorPackageController::start(
        0x0000_00AA,
        0x0000_1000,
        PackageFamily::Idle,
        vec![Waypoint::at([2.0, 3.0, 4.0])],
        DEFAULT_ARRIVAL_TOLERANCE,
    );
    let start = Vec3::new(2.0, 3.0, 4.0);
    let (mut world, entity) = controller_world(controller, start);

    world.run_system_once(drive_actor_packages).unwrap();

    // Animation was requested...
    assert_eq!(requested(&world, entity), Some(ActorAnimationState::Idle));
    // ...and the actor's translation is untouched by the family.
    assert_eq!(world.get::<Transform>(entity).unwrap().translation, start);
}

/// Travel routes through the nav seam (an `AgentTarget3d` insertion), again
/// with no transform write.
#[test]
fn travel_family_issues_a_nav_route_without_moving_the_actor() {
    let controller = ActorPackageController::start(
        0x0000_00AB,
        0x0000_2000,
        PackageFamily::Travel,
        vec![Waypoint::at([50.0, 0.0, 0.0])],
        DEFAULT_ARRIVAL_TOLERANCE,
    );
    let start = Vec3::new(0.0, 0.0, 0.0);
    let (mut world, entity) = controller_world(controller, start);

    world.run_system_once(drive_actor_packages).unwrap();

    // A route target was set (nav request), not a teleport.
    match world.get::<AgentTarget3d>(entity) {
        Some(AgentTarget3d::Point(point)) => {
            assert_eq!(*point, Vec3::new(50.0, 0.0, 0.0));
        }
        other => panic!("expected a routed point target, got {other:?}"),
    }
    assert_eq!(world.get::<Transform>(entity).unwrap().translation, start);
    assert_eq!(requested(&world, entity), None);
}

/// Eat claims its interaction point in the shared registry on arrival and
/// releases it when the package is stopped (preempt path via `release`).
#[test]
fn eat_family_claims_and_releases_its_interaction_point() {
    let controller = ActorPackageController::start(
        0x0000_00AC,
        0x0000_3000,
        PackageFamily::Eat,
        vec![Waypoint {
            position: [0.0, 0.0, 0.0],
            wait_seconds: 0.0,
            orientation_yaw: None,
            interaction_point: Some(0x0000_F00D),
        }],
        DEFAULT_ARRIVAL_TOLERANCE,
    );
    // Actor starts on the furniture, so it occupies on the first tick.
    let (mut world, entity) = controller_world(controller, Vec3::ZERO);

    world.run_system_once(drive_actor_packages).unwrap();
    assert!(
        world
            .resource::<PackageInteractionOccupancy>()
            .0
            .contains(&0x0000_F00D),
        "furniture claimed on arrival"
    );
    assert_eq!(requested(&world, entity), Some(ActorAnimationState::Idle));

    // Preempt: release the claim (what the `runpackage stop` path calls).
    let released = world
        .get_mut::<ActorPackageController>(entity)
        .unwrap()
        .driver
        .release();
    assert_eq!(released, Some(0x0000_F00D));
}

/// **Release-while-package-running** (the #164 fall-guard / `tna despawn`
/// seam). When nav releases a bound actor mid-package, the package must be
/// torn down through the nav-owned release contract -- otherwise
/// `drive_actor_packages` keeps ticking an agent-less actor forever and its
/// interaction point never frees. Asserts the *observable* outcome, not
/// internal flags: the controller is gone, the point is freed, and a
/// subsequent driver tick neither re-claims the point nor re-inserts a route.
#[test]
fn releasing_a_bound_actor_tears_down_its_running_package() {
    const POINT: u32 = 0x0000_F00D;
    // An Eat package whose actor already stands on its furniture, so the
    // first tick claims the interaction point.
    let controller = ActorPackageController::start(
        0x0000_00AF,
        0x0000_6000,
        PackageFamily::Eat,
        vec![Waypoint {
            position: [0.0, 0.0, 0.0],
            wait_seconds: 0.0,
            orientation_yaw: None,
            interaction_point: Some(POINT),
        }],
        DEFAULT_ARRIVAL_TOLERANCE,
    );
    let (mut world, entity) = controller_world(controller, Vec3::ZERO);
    // Register the AI teardown hook exactly as `AiPackagePlugin::build` does.
    api::register_release_hook(&mut world, release_actor_package);

    world.run_system_once(drive_actor_packages).unwrap();
    assert!(
        world
            .resource::<PackageInteractionOccupancy>()
            .0
            .contains(&POINT),
        "furniture claimed on arrival"
    );

    // Nav releases the actor (the fall-guard / `tna despawn` path).
    api::release_actor(&mut world, entity);

    // Observable outcome: the package controller is gone and the point freed.
    assert!(
        world.get::<ActorPackageController>(entity).is_none(),
        "release must tear down the running package controller"
    );
    assert!(
        !world
            .resource::<PackageInteractionOccupancy>()
            .0
            .contains(&POINT),
        "release must free the claimed interaction point"
    );

    // ...and the driver no longer ticks it. A still-live Eat controller would
    // re-claim POINT and a routing family would re-insert an `AgentTarget3d`
    // on the now agent-less actor every tick; neither may happen.
    world.run_system_once(drive_actor_packages).unwrap();
    assert!(
        !world
            .resource::<PackageInteractionOccupancy>()
            .0
            .contains(&POINT),
        "a torn-down package must not re-claim its interaction point"
    );
    assert!(
        world.get::<AgentTarget3d>(entity).is_none(),
        "a torn-down package must not re-route the released actor"
    );
}

/// Follow (#198) routes toward its *leader* through the nav seam, sampling
/// the leader's live transform -- and never writes the actor's translation.
#[test]
fn follow_family_routes_to_the_leader_without_moving_the_actor() {
    let mut world = World::new();
    world.insert_resource(Time::<()>::default());
    world.init_resource::<PackageInteractionOccupancy>();
    api::insert_test_archipelago_state(&mut world);
    // The moving leader the follower samples each tick.
    let leader = world
        .spawn(Transform::from_translation(Vec3::new(30.0, 0.0, 0.0)))
        .id();
    let controller = ActorPackageController::start_follow(
        0x0000_00AD,
        0x0000_4000,
        leader,
        2.0,
        5.0,
        DEFAULT_ARRIVAL_TOLERANCE,
    );
    let start = Vec3::ZERO;
    let entity = world
        .spawn((
            Transform::from_translation(start),
            ActorAnimationIntent::default(),
            controller,
        ))
        .id();
    api::insert_test_nav_agent(&mut world, entity);

    world.run_system_once(drive_actor_packages).unwrap();

    // Routed to the leader's position (nav request), actor not teleported.
    match world.get::<AgentTarget3d>(entity) {
        Some(AgentTarget3d::Point(point)) => {
            assert_eq!(*point, Vec3::new(30.0, 0.0, 0.0));
        }
        other => panic!("expected a route to the leader, got {other:?}"),
    }
    assert_eq!(world.get::<Transform>(entity).unwrap().translation, start);
}

/// Sandbox (#198) roams to a bounded, deterministic point through the nav
/// seam -- again a route request, never a transform write.
#[test]
fn wander_family_routes_within_radius_without_moving_the_actor() {
    let controller = ActorPackageController::start_wander(
        0x0000_00AE,
        0x0000_5000,
        [0.0, 0.0, 0.0],
        6.0,
        2.0,
        99,
        DEFAULT_ARRIVAL_TOLERANCE,
    );
    let start = Vec3::ZERO;
    let (mut world, entity) = controller_world(controller, start);

    world.run_system_once(drive_actor_packages).unwrap();

    match world.get::<AgentTarget3d>(entity) {
        Some(AgentTarget3d::Point(point)) => {
            assert!(
                point.xz().length() <= 6.0 + 1e-3,
                "roam point {point:?} escaped the radius"
            );
        }
        other => panic!("expected a roam route, got {other:?}"),
    }
    assert_eq!(world.get::<Transform>(entity).unwrap().translation, start);
    // A wandering actor requested no animation on its first roam tick.
    assert_eq!(requested(&world, entity), None);
}

#[test]
fn build_resolution_context_preserves_the_actors_linked_patrol_marker() {
    const ACTOR_REF: u32 = 0x9010;
    const MARKER_REF: u32 = 0x9020;
    let mut manifest: crate::vsa::PreparedSceneManifest =
        ron::de::from_str(include_str!("../../../../features/fixtures/scene.ron"))
            .expect("synthetic scene fixture should parse");
    let mut actor = manifest.placements[0].clone();
    actor.reference_form_id = ACTOR_REF;
    actor.translation = [1.0, 2.0, 3.0];
    actor.linked_reference_form_id = Some(MARKER_REF);
    let mut marker = actor.clone();
    marker.reference_form_id = MARKER_REF;
    marker.translation = [5.0, 0.0, 5.0];
    marker.linked_reference_form_id = None;
    manifest.placements = vec![actor, marker];

    let mut world = World::new();
    world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));
    let context = build_resolution_context(&mut world, ACTOR_REF);

    assert_eq!(context.linked_reference, Some(MARKER_REF));
    let resolved = crate::viewer::ai::resolution::resolve_location(
        &crate::viewer::ai::resolution::PackageLocation {
            location_type: 6,
            form_id: None,
            raw_value: 0,
            radius: 0,
        },
        &context,
    )
    .expect("near-linked-reference location resolves");
    assert_eq!(resolved.position, [5.0, 0.0, 5.0]);
    assert_eq!(
        resolved.source,
        crate::viewer::ai::resolution::ResolutionSource::LinkedReference(MARKER_REF)
    );
}
