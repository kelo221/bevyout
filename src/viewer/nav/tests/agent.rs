use std::collections::HashSet;

use super::*;
use crate::console::ConsoleSessionId;
use bevy_boxddd::boxddd::{BodyDef, BodyType, BoxHull, Filter, ShapeDef};

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
fn exterior_portal_link_endpoints_are_inset_into_the_owning_cell() {
    let interval = [[10.0, 20.0, 30.0], [10.0, 21.0, 32.0]];

    let min_x = inset_exterior_portal_interval(interval, 1);
    assert_eq!(min_x[0][0], 10.0 + EXTERIOR_PORTAL_LINK_INSET_METRES);
    assert_eq!(min_x[0][1], interval[0][1]);
    assert_eq!(min_x[0][2], interval[0][2]);

    let max_x = inset_exterior_portal_interval(interval, 0);
    assert_eq!(max_x[0][0], 10.0 - EXTERIOR_PORTAL_LINK_INSET_METRES);

    let min_z = inset_exterior_portal_interval(interval, 2);
    assert_eq!(min_z[0][2], 30.0 + EXTERIOR_PORTAL_LINK_INSET_METRES);

    let max_z = inset_exterior_portal_interval(interval, 3);
    assert_eq!(max_z[0][2], 30.0 - EXTERIOR_PORTAL_LINK_INSET_METRES);
}

fn invocation(args: &[&str]) -> ConsoleInvocation {
    ConsoleInvocation {
        request_id: 1,
        frame: 1,
        session: ConsoleSessionId::new("test"),
        command: "tna".into(),
        args: args.iter().map(|arg| arg.to_string()).collect(),
        target: None,
    }
}

fn harness_world() -> World {
    let mut world = World::new();
    world.init_resource::<NavArchipelagoState>();
    world.init_resource::<TestNavAgentState>();
    world.init_resource::<NavAgentLedger>();
    world.init_resource::<PendingPlayerSwapDoor>();
    world
}

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

/// Regression for #241: autonomous binding deliberately takes no console
/// roster slot, but the fall guard is gameplay behavior and must cover the
/// complete agent component set.
#[test]
fn fall_guard_releases_a_bound_actor_without_a_debug_roster_slot() {
    let mut world = World::new();
    world.init_resource::<TestNavAgentState>();
    world.insert_resource(NavCellFallBounds { min_y: Some(0.0) });
    let actor = world
        .spawn((
            TestNavAgentMarker,
            actor_binding::NavBoundActor::default(),
            AgentKcc::default(),
            Transform::from_xyz(0.0, -100.0, 0.0),
        ))
        .id();

    assert!(
        world
            .resource::<TestNavAgentState>()
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

/// **Wander-no-open-doors (#198), reproduced through the nav-owned flag.**
/// The door-open seam (`request_door_open`) refuses to open doors for an
/// actor whose active package must not (Sandbox/Wander) purely by reading the
/// nav-owned [`AgentRefusesDoors`] marker -- it no longer reaches up into an
/// AI type. The AI slice SETS the marker (`set_agent_refuses_doors` with
/// `!family.opens_doors()`); nav reads only its own component here. An
/// unflagged agent (every `tna`-driven agent and every door-opening family)
/// opens doors exactly as before.
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

/// A permissive `boxddd` collision filter/shape pairing scoped to these
/// tests: `step_agent_kcc` takes its filters as parameters, so a fixture
/// world only needs *a* consistent category/mask pair, not the real
/// player categories (those are private to `player/mod.rs`).
fn fixture_filter() -> boxddd::QueryFilter {
    boxddd::QueryFilter::new().category_bits(1).mask_bits(1)
}

fn fixture_shape_def() -> ShapeDef {
    ShapeDef::builder()
        .filter(Filter {
            category_bits: 1,
            mask_bits: 1,
            group_index: 0,
        })
        .build()
}

fn fixture_capsule() -> boxddd::Capsule {
    boxddd::Capsule::new(
        [0.0, -(AGENT_HEIGHT * 0.5 - AGENT_RADIUS), 0.0],
        [0.0, AGENT_HEIGHT * 0.5 - AGENT_RADIUS, 0.0],
        AGENT_RADIUS,
    )
}

fn add_fixture_box(world: &mut boxddd::World, center: boxddd::Vec3, half_extents: boxddd::Vec3) {
    let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    world.create_hull_shape(
        body,
        &fixture_shape_def(),
        &BoxHull::transformed(
            half_extents.x,
            half_extents.y,
            half_extents.z,
            boxddd::Transform::new(center, boxddd::Quat::IDENTITY),
        ),
    );
}

/// Issue #114: the navmesh `sample_point` Y-snap from wave 3's kinematic
/// spike is gone -- physics is ground authority now. Drops the agent
/// capsule from above a flat floor collider through `step_agent_kcc`
/// (the same free helper `apply_agent_physics_movement` calls) with no
/// landmass/App involved at all, and asserts it settles to rest on the
/// floor via real `boxddd` collision.
#[test]
fn agent_kcc_settles_onto_a_flat_floor_via_physics_collision() {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    // Floor top face at y = 0.0.
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(0.0, -0.1, 0.0),
        boxddd::Vec3::new(5.0, 0.1, 5.0),
    );
    let mover = fixture_capsule();
    let filter = fixture_filter();

    let mut position = Vec3::new(0.0, 3.0, 0.0);
    let mut velocity = Vec3::ZERO;
    let mut grounded = false;
    for _ in 0..300 {
        let (new_position, new_velocity, new_grounded) = step_agent_kcc(
            &mut world,
            &mover,
            filter,
            filter,
            position,
            velocity,
            grounded,
            Vec2::ZERO,
            1.0 / 60.0,
        );
        position = new_position;
        velocity = new_velocity;
        grounded = new_grounded;
        if grounded {
            break;
        }
    }
    assert!(grounded, "agent must come to rest on the floor via physics");
    let expected_y = AGENT_HEIGHT / 2.0;
    assert!(
        (position.y - expected_y).abs() < 0.05,
        "agent y should settle near the floor (expected {expected_y}), got {}",
        position.y
    );
    assert_eq!(
        velocity.y, 0.0,
        "vertical velocity is cleared once grounded"
    );
}

/// Issue #114 minimal-World test: desired vs. actual velocity feedback.
/// A wall square in front of a grounded agent means the KCC sweep
/// achieves (near-)zero horizontal displacement no matter what landmass
/// desired -- `movement_policy::decide_collision_outcome` must classify
/// that as `Blocked`, and the achieved velocity handed back to landmass
/// is the real, near-zero one, not the desired one.
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

/// Issue #114 minimal-World test: grounded gating on
/// `CellPhysicsReadiness`, mirroring the player controller's own guard
/// (`player/movement.rs::apply_player_controls`). While the destination
/// cell's static collision has not finished building, the agent must
/// not move through geometry that is not there yet -- velocity and
/// grounded state are forced to zero/false every tick.
#[test]
fn physics_movement_zeroes_velocity_while_cell_physics_is_building() {
    use bevy::ecs::system::RunSystemOnce;

    let mut world = World::new();
    world.init_resource::<TestNavAgentState>();
    world.insert_resource(PhysicsDisabled(false));
    world.insert_resource(CellPhysicsReadiness::BuildingStatic);
    world.init_resource::<Time>();
    world
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
    world.insert_non_send(BoxdddPhysicsContext::disabled());
    world.init_resource::<NavSolveStepCounter>();
    world.init_resource::<NavSolveRate>();

    let agent = world
        .spawn((
            TestNavAgentMarker,
            AgentKcc {
                velocity: Vec3::splat(3.0),
                grounded: true,
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Velocity3d::default(),
            AgentDesiredVelocityBlend::default(),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);

    world
        .run_system_once(apply_agent_physics_movement)
        .expect("system runs");

    let kcc = world.get::<AgentKcc>(agent).unwrap();
    assert_eq!(kcc.velocity, Vec3::ZERO);
    assert!(!kcc.grounded);
    assert_eq!(world.get::<Velocity3d>(agent).unwrap().velocity, Vec3::ZERO);
}

/// Regression test (issue #114 added scope, M4 wave 5 real-data
/// acceptance finding): the stuck-vs-target distance must also compare
/// on the horizontal plane, exactly like the two door-proximity gates
/// above. A target sitting directly below/above the agent (same X/Z,
/// wildly different Y -- capsule-centre vs. feet-level, or simply a
/// route target at a different storey) must never latch `stuck` purely
/// from that vertical gap as long as the agent is not moving away from
/// it horizontally: a 3D distance check would never close that gap and
/// would falsely report `stuck` at a target the agent has, on the
/// ground plane that actually matters for navigation, already reached.
#[test]
fn stuck_detection_does_not_false_trigger_against_a_vertically_offset_target() {
    use bevy::ecs::system::RunSystemOnce;

    let mut world = World::new();
    world.init_resource::<TestNavAgentState>();
    world.insert_resource(PhysicsDisabled(false));
    world.insert_resource(CellPhysicsReadiness::Ready);
    world.init_resource::<Time>();
    world
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
    world.insert_non_send(BoxdddPhysicsContext::from_world(
        boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world"),
    ));
    world.init_resource::<NavSolveStepCounter>();
    world.init_resource::<NavSolveRate>();

    let agent = world
        .spawn((
            TestNavAgentMarker,
            AgentKcc::default(),
            Transform::from_xyz(0.0, 5.0, 0.0),
            Velocity3d::default(),
            // No desired motion at all: the agent stays put on X/Z
            // (only falling under gravity in an empty physics world),
            // exactly on top of its target's X/Z but ~5 m above its Y.
            AgentDesiredVelocityBlend::default(),
            AgentTarget3d::Point(Vec3::new(0.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);

    let ticks = movement_policy::STUCK_RECOVERY_TICKS + movement_policy::STUCK_FAILURE_TICKS + 10;
    for _ in 0..ticks {
        world
            .run_system_once(apply_agent_physics_movement)
            .expect("system runs");
    }

    let kcc = world.get::<AgentKcc>(agent).unwrap();
    assert!(
        !kcc.stuck,
        "a target directly above/below the agent must never latch `stuck` via the horizontal-only distance check"
    );
}

/// Issue #154 feature 4: a clear merge-portal crossing sweeps the
/// agent to the far portal point (not an instant teleport/lerp -- the
/// KCC needs several ticks to physically cover the distance) and clears
/// `MergeTraversal`/`UsingAnimationLink`/`active_link` once it arrives.
#[test]
fn merge_traversal_system_sweeps_the_agent_to_the_far_portal_point() {
    use bevy::ecs::system::RunSystemOnce;

    let mut world = World::new();
    world.init_resource::<TestNavAgentState>();
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

    let target = Vec3::new(2.0, AGENT_HEIGHT / 2.0, 0.0);
    let agent = world
        .spawn((
            TestNavAgentMarker,
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
                target,
                elapsed: 0.0,
                timeout: merge_traversal_timeout(2.0),
                link_kind: 1,
            },
            UsingAnimationLink,
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);

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
    assert_eq!(world.get::<AgentRuntime>(agent).unwrap().active_link, None);
    let position = world.get::<Transform>(agent).unwrap().translation;
    assert!(
        (position.x - target.x).abs() < MERGE_TRAVERSAL_REACHED_DISTANCE + 0.1,
        "agent should have swept to the far portal point, got {position:?}"
    );
    let kcc = world.get::<AgentKcc>(agent).unwrap();
    assert!(!kcc.stuck, "a clear crossing must never latch stuck");
    assert!(!kcc.collision_blocked);
}

/// Issue #154 feature 4 / issue #162: a merge-portal crossing whose far
/// side is walled off must fail visibly through the existing
/// stuck/blocked reporting (`kcc.stuck`/`kcc.collision_blocked`, the
/// same fields `tna status` and the stable `nav agent stuck <id>`/`nav
/// agent collision-blocked <id>` log lines already use) rather than
/// teleporting the agent through the wall via a scripted lerp. Issue
/// #162 replaced the wave-8 wholesale route clear with per-agent
/// quarantine: this test now also pins that the specific link's kind
/// gets quarantined, `PermittedAnimationLinks` excludes exactly that
/// kind (never kind 0, the reserved door kind), the real target is
/// captured (not discarded) behind a one-tick `PendingMergeRepath`
/// blank, and -- unlike the old behaviour this replaces -- an in-
/// progress `travel_intent` survives untouched.
#[test]
fn merge_traversal_system_quarantines_the_link_and_preserves_the_real_destination() {
    use bevy::ecs::system::RunSystemOnce;

    let mut world = World::new();
    world.init_resource::<TestNavAgentState>();
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
            TestNavAgentMarker,
            AgentKcc {
                grounded: true,
                ..default()
            },
            Transform::from_xyz(0.0, AGENT_HEIGHT / 2.0, 0.0),
            AgentRuntime {
                active_link: Some(LinkKind::Merge { kind: 2 }),
                // A live travel intent, to prove #162 preserves it
                // (the pre-#162 behaviour cleared it unconditionally).
                travel_intent: Some(0x77),
                ..default()
            },
            MergeTraversal {
                target,
                elapsed: 0.0,
                timeout: merge_traversal_timeout(5.0),
                link_kind: 2,
            },
            UsingAnimationLink,
            AgentTarget3d::Point(target),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);

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
        Some(0x77),
        "issue #162: an in-progress travel intent is the real destination and must survive a quarantine, unlike the pre-#162 wholesale clear"
    );
}

// -------------------------------------------------------------
// Issue #162: resume_pending_merge_repath_system /
// clear_merge_link_quarantine.
// -------------------------------------------------------------

/// The normal case: nothing retargeted the agent during the one-tick
/// gap, so the real target `merge_traversal_system`'s timeout branch
/// captured is restored verbatim and the marker is consumed.
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

/// An `Entity` target (e.g. `tna goto player`) round-trips through the
/// snapshot the same way a `Point` does.
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

/// A `tna goto`/`tna travel` issued during the one-tick gap already
/// retargeted the agent (`AgentTarget3d` is no longer `None`) -- the
/// stale captured target must not clobber it.
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

/// Issue #162 feature 2: `clear_merge_link_quarantine` resets both the
/// tracked kind set and the live `PermittedAnimationLinks` component
/// back to `All`. `goto_agent`/`request_travel` call this on every new
/// target so a previous route's quarantine never leaks into a
/// completely different one.
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

/// `tna goto` clears a live quarantine (issue #162 feature 2's
/// lifecycle rule): a new goto is a new routing intent, so whatever
/// links a previous route quarantined no longer apply.
#[test]
fn goto_agent_clears_a_live_merge_link_quarantine() {
    let mut world = harness_world();
    world.init_resource::<Time>();
    let mut runtime = AgentRuntime::default();
    runtime.quarantined_merge_link_kinds.insert(1);
    let agent = world
        .spawn((
            TestNavAgentMarker,
            runtime,
            AgentKcc::default(),
            PermittedAnimationLinks::Kinds(Arc::new(HashSet::from([0]))),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);

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

/// `tna travel` clears a live quarantine the same way `tna goto` does.
#[test]
fn request_travel_clears_a_live_merge_link_quarantine() {
    let mut world = harness_world();
    let mut runtime = AgentRuntime::default();
    runtime.quarantined_merge_link_kinds.insert(1);
    let agent = world.spawn((TestNavAgentMarker, runtime)).id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// Issue #154 real-data acceptance correction: a candidate whose two
/// portal points sit on the same connected, unobstructed floor must
/// pass runtime collision-visibility validation.
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

/// The FranklinMetro02 real-data finding this correction fixes: a
/// candidate whose far portal point overhangs empty space (no floor
/// underneath, only a ledge at the near side) must be rejected for
/// missing ground support, not accepted and left to sweep an agent off
/// the edge into the void at traversal time.
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

/// A candidate whose straight-line crossing is physically blocked by
/// intervening geometry (not merely a portal accepted on abstract
/// topology alone) must be rejected as swept-blocked.
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

/// Regression test (issue #114 added scope, M4 wave 5 real-data
/// acceptance finding): `spawn_test_agent`'s visual child must sit
/// exactly centred on its parent (zero local offset), never raised.
/// Physics-authoritative movement's parent `Transform` is already the
/// capsule *centre* -- the wave-3/4 kinematic agent's `AGENT_HEIGHT /
/// 2.0` visual-lift compensated for that agent's `Transform` instead
/// sitting at feet level (navmesh-Y-snapped every tick); reintroducing
/// that lift on a now-already-centred parent double-counts it and
/// floats the rendered capsule a full half-height above the floor even
/// though the physics capsule (steps/slopes) sits correctly. Tied
/// explicitly to the centre-based parent so this can't silently
/// regress if someone reintroduces a feet-level assumption for either
/// side of the parent/child pair.
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
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(Entity::PLACEHOLDER);
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
    let entity = world.spawn(TestNavAgentMarker).id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(entity);
    let result = tna_command(&mut world, &invocation(&["despawn"])).expect("despawn succeeds");
    assert_eq!(result.log, ["nav agent 0 despawned"]);
    assert!(world.resource::<TestNavAgentState>().entities[0].is_none());
    assert!(world.get_entity(entity).is_err());
}

/// Issue #134 shipped amendment: wave 3's teardown used to despawn a
/// live test agent along with the stale archipelago, losing it. It is
/// now ledgered instead -- here with no door noted
/// (`PendingPlayerSwapDoor` defaults to `None`), so the agent freezes
/// in the *departing* cell at its current position rather than being
/// silently dropped.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(1.0, 2.0, 3.0),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);

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
    assert!(world.resource::<TestNavAgentState>().entities[0].is_none());

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

/// Issue #134: a player-initiated swap through the exact door a live
/// agent's active route was targeting hands it off to the destination
/// cell (follow-through) instead of freezing it in the departing cell.
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
            TestNavAgentMarker,
            AgentRuntime {
                travel_intent: Some(0x99),
                ..default()
            },
            Transform::from_xyz(5.0, 0.0, 0.0),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// Issue #134: a player swap through a door the agent's route was *not*
/// targeting freezes it in the departing cell, same as an untargeted
/// idle agent -- strict eligibility, no offscreen pathfinding.
#[test]
fn a_player_swap_through_a_different_door_still_freezes_the_agent() {
    let mut world = harness_world();
    world.resource_mut::<NavArchipelagoState>().cell_form_id = Some(0xC0DE);
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(world.spawn_empty().id());
    let agent = world
        .spawn((
            TestNavAgentMarker,
            // The agent is routed to a different travel door than the
            // one the player used.
            AgentRuntime {
                travel_intent: Some(0x50),
                ..default()
            },
            Transform::from_xyz(7.0, 0.0, 0.0),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// Issue #134: a cell claimed by a ledgered entry spawns exactly one
/// agent on activation, at the destination door's own placed position.
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

    let mut query = world.query_filtered::<Entity, With<TestNavAgentMarker>>();
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
        world.resource::<TestNavAgentState>().entities[0],
        Some(agent_entity)
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

/// Plan #113 minimal-App test: a travel-door request routes the agent
/// to the door triangle and, on arrival, drives the existing
/// `DoorLinkState` lifecycle (pause -> scripted-open boundary -> wait
/// -> traverse) to the `TravelReached` terminal seam.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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
    assert!(world.resource::<TestNavAgentState>().entities[0].is_none());
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

/// Regression test (issue #114 added scope, M4 wave 5 real-data
/// acceptance finding): physics-authoritative movement's `Transform` is
/// the capsule *centre*, not feet-level like `triangle_midpoint` (a
/// nav-graph point). The wave-3/4 kinematic agent Y-snapped its
/// `Transform` onto the navmesh every tick, incidentally erasing this
/// gap; every other travel-arrival test in this file sets the agent's Y
/// to match the door's exactly, which is why the regression this test
/// targets shipped unnoticed. A ~0.9 m vertical offset (roughly
/// `AGENT_HEIGHT / 2`, matching the real Vault101a 00028579 numbers from
/// acceptance) must not stop the arrival gate from firing.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

fn is_paused(world: &World, agent: Entity) -> bool {
    door_link::is_paused(
        world
            .get::<AgentRuntime>(agent)
            .map(|runtime| runtime.door_link)
            .unwrap_or_default(),
    )
}

/// Minimal travel-door placement for the lifecycle tests.
fn door_placement(reference_form_id: u32) -> crate::vsa::PreparedPlacement {
    crate::vsa::PreparedPlacement {
        reference_form_id,
        base_form_id: 1,
        asset_path: None,
        translation: [0.0; 3],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        error: None,
        physics_asset_path: None,
        physics_source: None,
        physics_classification: Default::default(),
        step_support: false,
        mutability: Default::default(),
        mutability_root_form_id: None,
        reference_kind: "REFR".into(),
        base_kind: "DOOR".into(),
        editor_id: None,
        display_name: None,
        count: 1,
        semantic: crate::vsa::PreparedSemantic::Door(crate::vsa::PreparedDoor {
            lock_level: None,
            key_form_id: None,
            trapped: false,
            destination: None,
        }),
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        linked_reference_form_id: None,
        inventory: Vec::new(),
        audio: Default::default(),
        ao_mode: "ao-none".into(),
    }
}

/// A door triangle (issue #155 feature 3) whose horizontal footprint
/// contains `center` -- the shape every `MidRouteDoor` fixture below
/// needs now that the crossing gate is point-in-triangle, not
/// centroid-proximity. Spans 2 m either side of `center` along X and Z,
/// well inside old `MID_ROUTE_DOOR_GATE_DISTANCE` scale but large
/// enough that the test-fixture agent positions below (which move
/// straight along X, holding Z fixed) reliably land inside it.
fn door_triangle_around(center: Vec3) -> [Vec3; 3] {
    [
        center + Vec3::new(-2.0, 0.0, -2.0),
        center + Vec3::new(2.0, 0.0, -2.0),
        center + Vec3::new(0.0, 0.0, 2.0),
    ]
}

/// A door placement at a specific position (issue #134's restore
/// tests, which spawn at a resolved door-marker position).
fn door_placement_at(
    reference_form_id: u32,
    translation: [f32; 3],
) -> crate::vsa::PreparedPlacement {
    crate::vsa::PreparedPlacement {
        translation,
        ..door_placement(reference_form_id)
    }
}

/// Plan #113 minimal-App test: a locked travel door never scripted-opens
/// (no teleporting through closed doors) and resolves to the existing
/// deterministic `Failed` status via the wait bound.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// Plan #113 minimal-App test: a door state change triggers exactly one
/// repath -- the blocked two-sided link spawns once when the door
/// becomes usable, and repeated polls with no further change do
/// nothing.
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

/// Issue #163 (`setlock`): the narrow `set_door_lock_level` mutation
/// point behaves exactly like a manifest-authored lock for
/// `door_availability_system`'s change detection -- inserting a level
/// records it (preserving any existing `key_form_id`), and clearing it
/// (`None`) flips a previously-locked door usable and drives the exact
/// same one-repath link-spawn `a_door_state_change_triggers_exactly_one_
/// repath` exercises via a direct field poke, this time through the
/// console-facing setter instead.
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

/// Issue #163: a door with no `door_usable` entry (no in-cell nav
/// triangles) still records the lock level -- the pure state mutation
/// the issue calls out as the fallback when the flip itself isn't
/// observable through `door_availability_system` (nothing tracks that
/// door for availability polling in the first place).
#[test]
fn set_door_lock_level_records_state_for_a_door_with_no_nav_triangles() {
    let mut world = harness_world();
    assert_eq!(door_lock_level_for_test(&world, 0x77), None);
    set_door_lock_level(&mut world, 0x77, Some(25));
    assert_eq!(door_lock_level_for_test(&world, 0x77), Some(25));
    set_door_lock_level(&mut world, 0x77, None);
    assert_eq!(door_lock_level_for_test(&world, 0x77), None);
}

/// Plan #137 minimal-App test (real-data-corrected): a `goto` past a
/// closed unlocked door mid-route drives the existing `DoorLinkState`
/// lifecycle exactly once via the crossing-check trigger, then returns
/// to `Idle` in the same cell. The door is *also* registered as a
/// travel door (`travel_doors`) -- real FO3 data shows nearly every
/// single-sided door resolves to a travel destination, and this is the
/// exact case the orchestrator's real-data review found ungated: an
/// agent with no `travel_intent` for this door must not be handed off
/// (no ledger entry, no despawn, no `DoorTraversal` -- there is no
/// off-mesh gap to lerp across since it merely crosses the triangle on
/// the way to a farther point).
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
            // A plain `goto` well beyond the door -- no `travel_intent`
            // for this (or any) door.
            AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// Issue #177 acceptance: the containment gate can be *starved*. Real
/// data (Vault 101, `VDoor01`) had an agent routed at a closed in-cell
/// door halt ~2 m short of its crossing with a completely free collision
/// sweep -- never entering the polygon, so never gating and never
/// opening the door. A stalled agent must gate on the crossing its route
/// continues through even without standing on it.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            // Stopped 2.2 m short of the crossing, exactly the measured
            // shortfall, with the target beyond the door.
            Transform::from_xyz(2.8, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
            AgentKcc::default(),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// The approach gate must not fire for a door the agent is stalled
/// *beside* or *past*: only one its own route continues through.
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
            TestNavAgentMarker,
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
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// Regression test (issue #114 added scope, M4 wave 5 real-data
/// acceptance finding): same shape as
/// `travel_arrival_tolerates_the_agent_capsule_centre_sitting_above_the_feet_level_door_midpoint`,
/// for the #137/#155 mid-route crossing gate -- a capsule-centre agent
/// above a feet-level `MidRouteDoor::vertices` triangle must still
/// trigger the crossing gate instead of silently clipping through the
/// closed door.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(0.0, 0.9, 0.0),
            AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// Plan #137 minimal-App test (real-data-corrected): a `tna travel`
/// request to a door still produces the full travel lifecycle and
/// handoff, even though the very same door is also a crossing-gate
/// candidate (`mid_route_doors`) -- the agent's own `travel_intent`
/// must exclude that door from the crossing check, or the two paths
/// would fight over the same arrival.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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
/// Plan #137 minimal-App test (real-data-corrected): a locked door
/// crossed mid-route -- again also registered as a travel door, the
/// real-data shape -- by an agent with no `travel_intent` for it never
/// scripted-opens and resolves to the existing deterministic `Failed`
/// outcome via the wait bound, instead of letting the agent clip
/// through.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// Plan #137 minimal-App test: a mid-route door's usability flip reuses
/// `door_availability_system` unchanged -- the same generic per-door
/// tracking two-sided/travel doors already populate -- so clearing a
/// lock while an agent waits on it triggers exactly one repath (a
/// `request_door_open` retry) that frees the paused agent.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// Real-data verification (M4 wave 10, post-#153 merge): a `PauseAgent`
/// leak on the door-link `Failed` terminal, found live on
/// FranklinMetro02 (0001a273) while chasing a reported "unreachable,
/// blocked" symptom near door 0007f7e3. Auditing every polygon
/// correlation in `nav/mod.rs::mesh_inputs`/`landmass_graph.rs`
/// (`door_type_indices`, `resolve_polygon_type_index`, `door_sides`,
/// `merge_link_descriptors`) confirmed all of them key strictly by the
/// *authored* `PolygonInput::index`/`DoorInput::triangle_index` value
/// via `HashMap`/`.find()`, never by list position -- so `#153`'s new
/// `.filter(|polygon| polygon.walkable)` (which does introduce list
/// *position* gaps relative to the authored index, since filtering
/// happens before `landmass_graph` ever sees the polygons) cannot
/// misattribute a door's type index or lock-cost override to the wrong
/// polygon. Real-data door 0007f7e3's own triangle (mesh 0005429f,
/// index 438) was confirmed `walkable: true`/`contains_door: true` with
/// vertex positions exactly matching the reported corridor -- the
/// original "unreachable" was door 0007f7e3's genuine authored lock
/// (level 25) correctly blocking the only route through it, the
/// existing, tested mid-route crossing-gate behaviour (issue #137/
/// #155), not an index-misalignment bug.
///
/// The *real* defect: once `setlock 0007f7e3 0` unblocked the door and
/// a *fresh* `tna goto` was reissued, the agent never actually moved
/// again -- frozen at the door's own triangle, `tna status` reporting
/// `paused` forever even though the door-link FSM itself correctly
/// reached `Idle`. `PauseAgent` (inserted the moment this door-link
/// cycle first paused the agent) was only ever removed on the
/// `is_traversing` transition; the `Failed` terminal above left it
/// attached, and `landmass` treats a `PauseAgent`-carrying entity as
/// permanently `AgentState::Paused` -- it skips that agent's own path/
/// movement solving every tick regardless of any later `AgentTarget3d`
/// a fresh `tna goto`/`tna travel` sets. This test pins the fix:
/// `PauseAgent` must be gone once the door-link cycle reaches `Failed`,
/// the same as it already is on `Traversing`.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(10.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// The travel-arrival counterpart: `PauseAgent` must not survive a
/// *travel* door's `Failed` terminal either -- the same code branch
/// (`is_failed(new_state)`) handles both `LinkDestination::IntraCell`
/// and `LinkDestination::Travel`, so the leak (and its fix) apply
/// identically to both. `locked_travel_arrival_settles_at_a_stable_
/// unreachable_terminal_not_an_oscillation` above already exercises this
/// exact setup for the FSM-only assertions; this test adds the
/// `PauseAgent` check that revealed the real-data bug.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(5.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// Plan #113 minimal-App test: never two concurrent travel requests.
#[test]
fn concurrent_travel_requests_are_rejected() {
    let mut world = harness_world();
    let agent = world
        .spawn((TestNavAgentMarker, AgentRuntime::default()))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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
    let agent = world
        .spawn((TestNavAgentMarker, AgentRuntime::default()))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
    assert_eq!(
        request_travel(&mut world, 0, 0x99).unwrap_err().code,
        "unknown_travel_door"
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

/// Issue #215: debug indices are independent and grow beyond the original
/// four slots, while the defensive dense-allocation ceiling is enforced.
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
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(Entity::PLACEHOLDER);

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

/// Issue #114 feature 4: an indexed `tna goto` addresses exactly the
/// named agent slot, leaving every other slot's target untouched --
/// the back-compat bare form (no index) still defaults to agent 0.
#[test]
fn indexed_goto_addresses_only_the_named_agent_slot() {
    let mut world = harness_world();
    world.init_resource::<Time>();
    let agent0 = world
        .spawn((
            TestNavAgentMarker,
            AgentRuntime::default(),
            AgentKcc::default(),
        ))
        .id();
    let agent1 = world
        .spawn((
            TestNavAgentMarker,
            AgentRuntime::default(),
            AgentKcc::default(),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent0);
    world.resource_mut::<TestNavAgentState>().entities[1] = Some(agent1);

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

fn minimal_manifest(cell_form_id: u32) -> PreparedSceneManifest {
    PreparedSceneManifest {
        schema_version: 17,
        prepare_revision: None,
        converter_revision: None,
        physics_schema_version: None,
        asset_root: ".".into(),
        source_plugin: "Fallout3.esm".into(),
        source_fingerprint: "content-hash".into(),
        item_catalog_path: None,
        item_catalog_revision: None,
        item_catalog_hash: None,
        recipe_catalog_path: None,
        recipe_catalog_revision: None,
        recipe_catalog_hash: None,
        actor_catalog_path: None,
        actor_catalog_revision: None,
        actor_catalog_hash: None,
        actor_animation_catalog_path: None,
        actor_animation_catalog_revision: None,
        actor_animation_catalog_hash: None,
        source_plugins: Vec::new(),
        visual_issues: Vec::new(),
        cell: crate::vsa::CellInfo {
            form_id: cell_form_id,
            editor_id: None,
            name: None,
            interior: true,
            behave_like_exterior: false,
            ambient_rgba: [0.0; 4],
            directional_rgba: [0.0; 4],
            image_space_form_id: None,
            image_space: None,
            lighting_template_form_id: None,
            lighting_template_flags: 0,
            lighting_template: None,
            raw_lighting: None,
            effective_lighting: None,
            water_form_id: None,
            water_height: None,
            grid: None,
            worldspace_form_id: None,
            day_night_profile: None,
            day_night_preview_profile: None,
        },
        placements: Vec::new(),
        lights: Vec::new(),
        diagnostics: Vec::new(),
        navmeshes: Vec::new(),
        nav_graph: None,
        cell_audio: Default::default(),
        audio_clips: Vec::new(),
        footstep_sets: Vec::new(),
        hard_landing_clips: Vec::new(),
        bake: None,
        static_point_shadows: None,
        reflection_probes: None,
        mutability_summary: Default::default(),
        leveled_lists: Default::default(),
        dialogue: None,
        exterior: None,
    }
}

// -----------------------------------------------------------------
// Issue #169: setlock issued before the archipelago exists.
// -----------------------------------------------------------------

/// Writes a synthetic `navgraph.ron` (never Bethesda-derived -- see
/// AGENTS.md's git caution) under a scratch cache dir and returns a
/// manifest whose `nav_graph.asset_path`/`asset_root` resolve to it,
/// plus one door placement -- mirrors `nav_overlay.rs`'s own test
/// helper of the same shape (private to that module, so duplicated
/// rather than shared, the same rationale `nav/mod.rs::read_nav_graph`'s
/// doc comment gives for its own duplication). Exercises
/// `ensure_archipelago`'s real file-reading path directly, rather than
/// the `already_current` short-circuit every other test in this module
/// relies on -- issue #169's bug is specifically in what that path does
/// with `door_lock_info` before the short-circuit is even possible (the
/// very first build of a session). The mesh carries no door triangles
/// of its own: these tests are about `NavArchipelagoState::
/// door_lock_info` surviving the rebuild, not about door-typed
/// pathing (that is issue #155's own coverage).
fn manifest_with_nav_graph_and_door(
    cell_form_id: u32,
    door_form_id: u32,
    authored_lock_level: Option<i8>,
) -> PreparedSceneManifest {
    let graph = crate::vsa::PreparedNavGraph {
        cell_form_id,
        meshes: vec![crate::vsa::PreparedNavMesh {
            form_id: 0x10,
            vertices: vec![
                [0.0, 0.0, 0.0],
                [4.0, 0.0, 0.0],
                [0.0, 0.0, 4.0],
                [4.0, 0.0, 4.0],
            ],
            polygons: vec![
                crate::vsa::PreparedNavPolygon {
                    index: 0,
                    vertex_indices: [0, 1, 2],
                    ..Default::default()
                },
                crate::vsa::PreparedNavPolygon {
                    index: 1,
                    vertex_indices: [1, 3, 2],
                    ..Default::default()
                },
            ],
            ..Default::default()
        }],
        ..Default::default()
    };
    let dir = std::env::temp_dir().join(format!(
        "bevyout-nav-agent-test-{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let relative = "navgraph.ron";
    std::fs::write(
        dir.join(relative),
        ron::ser::to_string_pretty(&graph, ron::ser::PrettyConfig::default()).unwrap(),
    )
    .unwrap();
    let mut manifest = minimal_manifest(cell_form_id);
    manifest.asset_root = dir.to_string_lossy().into_owned();
    manifest.nav_graph = Some(crate::vsa::PreparedNavGraphSource {
        asset_path: relative.into(),
        ..Default::default()
    });
    manifest.placements = vec![crate::vsa::PreparedPlacement {
        semantic: crate::vsa::PreparedSemantic::Door(crate::vsa::PreparedDoor {
            lock_level: authored_lock_level,
            key_form_id: None,
            trapped: false,
            destination: None,
        }),
        ..door_placement(door_form_id)
    }];
    manifest
}

/// Minimal resources `ensure_archipelago` unconditionally touches,
/// beyond what `harness_world()` already provides -- `PhysicsDisabled`
/// is set `true` so the merge-link collision-validation pass (this
/// fixture's mesh has no merges anyway) never needs a real
/// `BoxdddPhysicsContext`.
fn archipelago_build_world() -> World {
    let mut world = harness_world();
    world.init_resource::<Assets<NavMesh3d>>();
    world.init_resource::<NavCellFallBounds>();
    world.insert_resource(PhysicsDisabled(true));
    world
}

/// Build-after-unlock (issue #169's exact repro): the door's authored
/// data is locked (`Some(25)`), but a `setlock` unlock landed *before*
/// `ensure_archipelago` ever ran -- `NavArchipelagoState` is
/// `init_resource`d empty at plugin install, well before the first
/// `tna spawn`, so `set_door_lock_level` (`setlock`'s own narrow
/// mutation point) already has somewhere to write. The runtime unlock
/// must win over the authored lock at build time.
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

/// The build-after-lock counterpart: authored data is unlocked
/// (`None`), but a runtime `setlock` recorded a lock before the
/// archipelago ever existed.
#[test]
fn an_early_setlock_lock_survives_the_first_archipelago_build() {
    let mut world = archipelago_build_world();
    let manifest = manifest_with_nav_graph_and_door(0xBEEF, 0x99, None);
    world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    set_door_lock_level(&mut world, 0x99, Some(50));

    ensure_archipelago(&mut world).expect("archipelago builds");

    assert_eq!(
        door_lock_level_for_test(&world, 0x99),
        Some(50),
        "the runtime lock recorded before the archipelago existed must survive the build, winning over the authored (unlocked) baseline"
    );
}

/// Regression pin: a door this session's `setlock` never touched keeps
/// its authored value -- the merge in `ensure_archipelago` must not
/// blanket-override every door with whatever the (empty)
/// `NavArchipelagoState.door_lock_info` happens to hold, only apply an
/// actual recorded runtime change.
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

/// A lock change issued *after* the archipelago already exists still
/// applies (the pre-#169 path -- `set_door_lock_level` writing directly
/// into the live `NavArchipelagoState.door_lock_info`, no rebuild
/// needed). Pinned here alongside the early-setlock tests so the two
/// timing cases -- before and after the first build -- are both
/// covered in one place.
#[test]
fn a_late_setlock_change_still_applies_without_a_rebuild() {
    let mut world = archipelago_build_world();
    let manifest = manifest_with_nav_graph_and_door(0xBEEF, 0x99, Some(25));
    world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    ensure_archipelago(&mut world).expect("archipelago builds");
    assert_eq!(door_lock_level_for_test(&world, 0x99), Some(25));

    set_door_lock_level(&mut world, 0x99, None);
    assert_eq!(
        door_lock_level_for_test(&world, 0x99),
        None,
        "a lock change after the archipelago exists must apply immediately, no rebuild needed"
    );
}

// -----------------------------------------------------------------
// Wave 5 added scope (#114 movement fidelity): fixed-timestep movement,
// player-as-landmass-character avoidance, configurable solve interval.
// -----------------------------------------------------------------

/// A `boxddd` collision filter compatible with the *real* hardcoded
/// `player::player_collision_filter()`/`stair_support_filter()` queries
/// `apply_agent_physics_movement` uses (those category constants are
/// private to `player/mod.rs`, so this mirrors their known bit values --
/// `WORLD_STATIC = 1`, `STEP_SUPPORT = 16` -- directly): a floor shape
/// built with it is both an ordinary collision surface and a
/// step-support surface. `mask_bits` is maximally permissive since a
/// static, passive shape like a floor is only ever the *target* of a
/// query, never the querying side.
fn fixture_floor_filter() -> Filter {
    Filter {
        category_bits: 1 | 16,
        mask_bits: u64::MAX,
        group_index: 0,
    }
}

/// A flat floor box (top face at `center.y + half_extents.y`) using
/// [`fixture_floor_filter`] rather than [`fixture_shape_def`]'s
/// self-consistent-but-arbitrary filter, so the real
/// `apply_agent_physics_movement` system (not just the pure
/// `step_agent_kcc`/`move_mover` helpers, which take their filter as a
/// parameter) actually collides with and stands on it.
fn add_player_compatible_floor(
    world: &mut boxddd::World,
    center: boxddd::Vec3,
    half_extents: boxddd::Vec3,
) {
    let shape_def = ShapeDef::builder().filter(fixture_floor_filter()).build();
    let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    world.create_hull_shape(
        body,
        &shape_def,
        &BoxHull::transformed(
            half_extents.x,
            half_extents.y,
            half_extents.z,
            boxddd::Transform::new(center, boxddd::Quat::IDENTITY),
        ),
    );
}

/// Builds a minimal `App` with the full `NavBackendPlugin` wiring:
/// `Landmass3dPlugin` (in `FixedPreUpdate`) plus this file's own
/// `FixedUpdate` agent chain and the solve-rate gate on
/// `LandmassSystems::Update`, exactly as `install` wires it in the real
/// viewer -- plus `TransformPlugin` so `GlobalTransform` reflects
/// `Transform` without needing a full render/window stack. Physics
/// readiness resources (`PhysicsDisabled`, `CellPhysicsReadiness`) and a
/// `BoxdddPhysicsContext` holding a flat floor spanning
/// [`spawn_fixture_island`]'s 4x4 footprint (top face at `y = 0.0`,
/// matching the island mesh plane exactly) are inserted directly rather
/// than through `player::install`, which pulls in the full window/input/
/// asset surface these tests do not need. A real floor -- not just an
/// empty physics world -- matters here: without one the capsule free-
/// falls under gravity every tick and drifts outside the navmesh's
/// vertical sampling envelope within a couple dozen ticks, flipping the
/// agent to `AgentState::AgentNotOnNavMesh` and losing its desired
/// velocity entirely (confirmed the hard way while writing the
/// avoidance-deflection test below).
fn fixed_tick_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        bevy::MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        bevy::transform::TransformPlugin,
        NavBackendPlugin,
    ));
    app.insert_resource(PhysicsDisabled(false));
    app.insert_resource(CellPhysicsReadiness::Ready);
    let mut physics_world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(2.0, -0.1, 2.0),
        boxddd::Vec3::new(4.0, 0.1, 4.0),
    );
    app.world_mut()
        .insert_non_send(BoxdddPhysicsContext::from_world(physics_world));
    app
}

/// Advances exactly one fixed tick by hand: advances `Time<Fixed>` by
/// its configured timestep, publishes that as the generic `Res<Time>`
/// clock the way the real fixed-main loop does
/// (`bevy_time::fixed::run_fixed_main_schedule`'s own per-expend body),
/// then runs `FixedPreUpdate` (landmass, the player-character sync, and
/// the solve-rate bookkeeping) followed by `FixedUpdate` (this file's
/// agent chain) directly by schedule label -- the same technique
/// `nav_overlay.rs`'s own landmass harness test uses for
/// `FixedPreUpdate` alone, extended across both schedules so a whole
/// tick is deterministic with no dependency on real wall-clock elapsed
/// time.
fn run_one_fixed_tick(world: &mut World) {
    let timestep = world.resource::<Time<Fixed>>().timestep();
    world.resource_mut::<Time<Fixed>>().advance_by(timestep);
    let generic = world.resource::<Time<Fixed>>().as_generic();
    *world.resource_mut::<Time>() = generic;
    world.run_schedule(FixedPreUpdate);
    world.run_schedule(FixedUpdate);
}

/// Issue #184 regression fixture: a straight walkable corridor the agent
/// walks the length of, plus a *connected* side bay chopped into many thin
/// sliver triangles whose border edges sit a few metres off the corridor.
/// This is the synthetic shape of the 00024512 stall -- a landing joined to
/// finely re-triangulated geometry (issue #171 emits exactly these slivers)
/// whose borders `landmass::avoidance` flattens into one 2D `dodgy_2d`
/// obstacle set. Nothing here is keyed on a cell or a coordinate: it is the
/// geometry class, not the instance.
///
/// Corridor: `x in [0, 3]`, `z in [0, CORRIDOR_LENGTH]`. Side bay:
/// `x in [3, 5.5]`, `z in [4, 8]`, split into `slivers` strips.
fn stall_fixture_mesh(slivers: usize) -> landmass_graph::MeshInput {
    const CORRIDOR_LENGTH: f32 = 14.0;
    const BAY_START: f32 = 4.0;
    const BAY_END: f32 = 8.0;
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut index_of: HashMap<(i64, i64, i64), u32> = HashMap::new();
    // Shared vertex grid: landmass connects polygons by identical vertex
    // *indices*, so every quad corner must resolve to one entry or the
    // whole mesh degenerates into disconnected triangles with no interior
    // edges at all. Keyed in full 3D because the switchback below folds
    // back under the corridor, putting two surfaces at one `(x, z)`.
    let mut vertex = |vertices: &mut Vec<[f32; 3]>, x: f32, y: f32, z: f32| -> u32 {
        let key = (
            (x * 1e4).round() as i64,
            (y * 1e4).round() as i64,
            (z * 1e4).round() as i64,
        );
        *index_of.entry(key).or_insert_with(|| {
            vertices.push([x, y, z]);
            (vertices.len() - 1) as u32
        })
    };

    // The z cuts every quad row shares: 1 m steps along the corridor, plus
    // one cut per sliver through the bay's span so the bay's strips share
    // real edges with the corridor rather than T-junctioning onto it.
    let strip = (BAY_END - BAY_START) / slivers as f32;
    let mut cuts: Vec<f32> = (0..=CORRIDOR_LENGTH as usize).map(|z| z as f32).collect();
    cuts.extend((0..=slivers).map(|index| BAY_START + index as f32 * strip));
    cuts.sort_by(f32::total_cmp);
    cuts.dedup_by(|a, b| (*a - *b).abs() < 1e-4);

    let mut polygons: Vec<landmass_graph::PolygonInput> = Vec::new();
    let mut quad = |vertices: &mut Vec<[f32; 3]>,
                    polygons: &mut Vec<landmass_graph::PolygonInput>,
                    (x0, y0): (f32, f32),
                    (x1, y1): (f32, f32),
                    z0: f32,
                    z1: f32| {
        let (a, b, c, d) = (
            vertex(vertices, x0, y0, z0),
            vertex(vertices, x1, y1, z0),
            vertex(vertices, x0, y0, z1),
            vertex(vertices, x1, y1, z1),
        );
        for mut indices in [[a, b, c], [b, d, c]] {
            // One consistent XZ winding across the whole mesh: the lower
            // flight runs back along -x, which flips a naively-ordered
            // quad's winding and makes landmass reject the mesh outright.
            let corner = |index: u32| {
                let v = vertices[index as usize];
                (v[0], v[2])
            };
            let (p, q, r) = (corner(indices[0]), corner(indices[1]), corner(indices[2]));
            if (q.0 - p.0) * (r.1 - p.1) - (q.1 - p.1) * (r.0 - p.0) < 0.0 {
                indices.swap(1, 2);
            }
            polygons.push(landmass_graph::PolygonInput {
                index: polygons.len() as u32,
                vertex_indices: indices,
                is_water: false,
                is_preferred_pathing: false,
            });
        }
    };

    for pair in cuts.windows(2) {
        let (z0, z1) = (pair[0], pair[1]);
        quad(&mut vertices, &mut polygons, (0.0, 0.0), (3.0, 0.0), z0, z1);
        // A switchback stair descending off the corridor's x = 3 edge and
        // folding back *underneath* it: genuinely connected walkable ground
        // (an agent could walk down it), finely re-triangulated the way
        // issue #171's sub-triangle clip emits real FO3 stairs. This is the
        // ingredient that matters -- `landmass::avoidance` explores into it
        // through that shared edge and `dodgy_2d` is strictly 2D, so the
        // lower flight's borders project straight onto the corridor
        // footprint the agent is standing on.
        if z0 >= BAY_START - 1e-4 && z1 <= BAY_END + 1e-4 {
            quad(
                &mut vertices,
                &mut polygons,
                (3.0, 0.0),
                (4.0, -0.5),
                z0,
                z1,
            );
            quad(
                &mut vertices,
                &mut polygons,
                (4.0, -0.5),
                (0.0, -1.3),
                z0,
                z1,
            );
        }
    }
    landmass_graph::MeshInput {
        form_id: 0x184,
        vertices,
        polygons,
        doors: Vec::new(),
        derived_doors: Vec::new(),
    }
}

/// Builds an app around [`stall_fixture_mesh`] with an explicit border
/// avoidance horizon, runs an agent the length of the corridor, and reports
/// the furthest `z` it reached plus the lowest desired speed it was ever
/// steered at. Everything except `obstacle_avoidance_time_horizon` matches
/// the shipped `archipelago_options`.
fn run_stall_fixture(horizon: f32) -> (f32, f32) {
    let mut app = App::new();
    app.add_plugins((
        bevy::MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        bevy::transform::TransformPlugin,
        NavBackendPlugin,
    ));
    app.insert_resource(PhysicsDisabled(false));
    app.insert_resource(CellPhysicsReadiness::Ready);
    let mut physics_world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(2.75, -0.1, 7.0),
        boxddd::Vec3::new(6.0, 0.1, 10.0),
    );
    app.world_mut()
        .insert_non_send(BoxdddPhysicsContext::from_world(physics_world));

    let mesh_input = stall_fixture_mesh(64);
    let valid =
        landmass_graph::build_navigation_mesh(&mesh_input, &[], &BTreeMap::new(), &BTreeMap::new())
            .nav_mesh
            .expect("stall fixture validates");
    let handle = app
        .world_mut()
        .resource_mut::<Assets<NavMesh3d>>()
        .add(NavMesh3d {
            nav_mesh: Arc::new(valid),
        });
    let mut options = archipelago_options();
    options.obstacle_avoidance_time_horizon = horizon;
    let archipelago = app.world_mut().spawn(Archipelago3d::new(options)).id();
    app.world_mut().spawn(Island3dBundle {
        island: Island,
        archipelago_ref: ArchipelagoRef3d::new(archipelago),
        nav_mesh: NavMeshHandle::<ThreeD>(handle),
    });
    app.world_mut()
        .resource_mut::<NavArchipelagoState>()
        .archipelago = Some(archipelago);

    let centre = Vec3::new(0.0, AGENT_HEIGHT * 0.5, 0.0);
    let agent = spawn_bare_agent(
        app.world_mut(),
        archipelago,
        Vec3::new(1.5, 0.0, 1.0) + centre,
        Vec3::new(1.5, 0.0, 13.0) + centre,
    );
    let mut furthest = f32::MIN;
    let mut slowest = f32::MAX;
    let trace = std::env::var("BEVYOUT_STALL_TRACE").is_ok();
    for tick in 0..600 {
        run_one_fixed_tick(app.world_mut());
        let world = app.world();
        if trace && tick % 20 == 0 {
            let position = world.get::<Transform>(agent).unwrap().translation;
            let desired = world
                .get::<AgentDesiredVelocity3d>(agent)
                .map(|v| v.velocity())
                .unwrap_or(Vec3::ZERO);
            println!(
                "h={horizon} t{tick}: pos=({:.2},{:.2},{:.2}) |d|={:.3} state={:?}",
                position.x,
                position.y,
                position.z,
                desired.length(),
                world.get::<AgentState>(agent).copied()
            );
        }
        furthest = furthest.max(world.get::<Transform>(agent).unwrap().translation.z);
        // Only sample steering while the agent is still short of the
        // target: decelerating on arrival is correct, not a stall.
        if furthest < 12.0
            && let Some(desired) = world.get::<AgentDesiredVelocity3d>(agent)
        {
            slowest = slowest.min(desired.velocity().length());
        }
    }
    (furthest, slowest)
}

/// Issue #184: an agent must cross a stretch of corridor that has finely
/// re-triangulated walkable geometry a few metres to one side, without its
/// steering collapsing. Before the fix, `landmass`'s navmesh-border ORCA
/// avoidance flattened that side bay's border edges into a `dodgy_2d`
/// obstacle set dense enough to drive the *desired* velocity
/// asymptotically to zero -- a contactless halt, with the capsule sweep
/// completely free, that `apply_agent_physics_movement` could only report
/// as `reason=no_contact_no_progress`.
///
/// Asserted as a pair so the fixture itself is proven to reproduce: with
/// landmass's stock `0.25` horizon the agent creeps to a halt beside the
/// stair and is steered at a near-zero speed, and with the shipped
/// `NAV_BORDER_AVOIDANCE_TIME_HORIZON` it walks the whole corridor at its
/// full desired speed.
#[test]
fn an_agent_crosses_finely_triangulated_ground_without_its_steering_collapsing() {
    let (stock_furthest, stock_slowest) = run_stall_fixture(0.25);
    assert!(
        stock_slowest < AGENT_DESIRED_SPEED * 0.5,
        "fixture must reproduce the pre-fix steering collapse, \
             slowest desired speed was {stock_slowest} (furthest z {stock_furthest})"
    );

    let (furthest, slowest) = run_stall_fixture(NAV_BORDER_AVOIDANCE_TIME_HORIZON);
    assert!(
        furthest > 12.0,
        "the agent must walk the corridor, reached z {furthest} only"
    );
    assert!(
        slowest > AGENT_DESIRED_SPEED * 0.9,
        "steering must never collapse on clear ground, \
             slowest desired speed was {slowest}"
    );
}

/// Issue #184: the shipped options must keep navmesh-border ORCA avoidance
/// clamped to at most one fixed tick -- the property that makes the
/// asymptotic `1 - dt / horizon` stall impossible -- while leaving
/// agent/character avoidance (issue #114 feature 4) at landmass's own
/// default. A regression here is silent: it costs no test but reopens the
/// contactless-stall class.
#[test]
fn archipelago_options_clamp_border_avoidance_but_keep_agent_avoidance() {
    let options = archipelago_options();
    let stock = ArchipelagoOptions::<ThreeD>::from_agent_radius(AGENT_RADIUS);
    assert!(
        options.obstacle_avoidance_time_horizon > 0.0,
        "a zero horizon divides by zero inside dodgy_2d"
    );
    assert!(
        options.obstacle_avoidance_time_horizon <= 1e-3,
        "navmesh-border avoidance must stay disabled; anything this side of \
             ~1e-3 reopens the contactless-stall class, got {}",
        options.obstacle_avoidance_time_horizon
    );
    assert_eq!(
        options.avoidance_time_horizon, stock.avoidance_time_horizon,
        "agent/character avoidance must keep landmass's default"
    );
    assert_eq!(
        options.neighbourhood, stock.neighbourhood,
        "the avoidance neighbourhood must keep landmass's default"
    );
}

/// Spawns the same synthetic two-triangle 4x4 island fixture
/// `nav_overlay.rs`'s own landmass harness test uses, wired directly
/// into `NavArchipelagoState` (bypassing the manifest/
/// `ensure_archipelago` plumbing these unit tests do not need). Returns
/// the archipelago entity.
fn spawn_fixture_island(world: &mut World) -> Entity {
    let mesh_input = landmass_graph::MeshInput {
        form_id: 0x10,
        vertices: vec![
            [0.0, 0.0, 0.0],
            [4.0, 0.0, 0.0],
            [0.0, 0.0, 4.0],
            [4.0, 0.0, 4.0],
        ],
        polygons: vec![
            landmass_graph::PolygonInput {
                index: 0,
                vertex_indices: [0, 1, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
            landmass_graph::PolygonInput {
                index: 1,
                vertex_indices: [1, 3, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
        ],
        doors: Vec::new(),
        derived_doors: Vec::new(),
    };
    let valid =
        landmass_graph::build_navigation_mesh(&mesh_input, &[], &BTreeMap::new(), &BTreeMap::new())
            .nav_mesh
            .expect("synthetic square validates");
    let nav_mesh_handle = world.resource_mut::<Assets<NavMesh3d>>().add(NavMesh3d {
        nav_mesh: Arc::new(valid),
    });
    // The exact options `ensure_archipelago` applies for real cells --
    // widened sampling envelope plus the clamped border-avoidance horizon
    // (see `archipelago_options`).
    let archipelago_entity = world.spawn(Archipelago3d::new(archipelago_options())).id();
    world.spawn(Island3dBundle {
        island: Island,
        archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
        nav_mesh: NavMeshHandle::<ThreeD>(nav_mesh_handle),
    });
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago_entity);
    archipelago_entity
}

/// Spawns a bare nav agent (no console-tracked `TestNavAgentState` slot,
/// no visual mesh) directly into `archipelago_entity`, targeting `target`
/// from `start`. Mirrors the component set `spawn_test_agent` builds,
/// minus the roster bookkeeping and visuals these App-level movement
/// tests do not need.
fn spawn_bare_agent(
    world: &mut World,
    archipelago_entity: Entity,
    start: Vec3,
    target: Vec3,
) -> Entity {
    let agent = world
        .spawn((
            TestNavAgentMarker,
            AgentKcc::default(),
            AgentDesiredVelocityBlend::default(),
            Transform::from_translation(start),
            Agent3dBundle {
                agent: default(),
                settings: AgentSettings {
                    radius: AGENT_RADIUS,
                    desired_speed: AGENT_DESIRED_SPEED,
                    max_speed: AGENT_MAX_SPEED,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            },
            TargetReachedCondition::Distance(Some(AGENT_TARGET_REACHED_DISTANCE)),
        ))
        .id();
    world.entity_mut(agent).insert(AgentTarget3d::Point(target));
    agent
}

/// Task 1 (fixed-timestep movement) + the solve-rate gate: the agent
/// keeps advancing horizontally toward its target on every fixed tick,
/// including a tick the solve is gated off on (`NavSolveRate(2)`).
/// Warms up over a few ticks first so both halves of the blend
/// (`AgentDesiredVelocityBlend`) hold real, nonzero solved values rather
/// than the zero-initialized default.
#[test]
fn movement_runs_every_fixed_tick_including_when_the_solve_is_gated_off() {
    let mut app = fixed_tick_test_app();
    let archipelago_entity = spawn_fixture_island(app.world_mut());
    // `Transform.translation` is the capsule *centre* (mirrors
    // production: `spawn_test_agent` places new agents at the player's
    // own capsule-centre position), so standing on a floor whose top
    // face is at `y = 0.0` means starting at `y = AGENT_HEIGHT / 2`, not
    // `y = 0.0` -- the target's own Y does not matter to physics, only
    // to which navmesh point it samples onto.
    let agent = spawn_bare_agent(
        app.world_mut(),
        archipelago_entity,
        Vec3::new(0.5, AGENT_HEIGHT / 2.0, 0.5),
        Vec3::new(3.5, 0.0, 3.5),
    );
    app.world_mut().insert_resource(NavSolveRate(2));

    for _ in 0..4 {
        run_one_fixed_tick(app.world_mut());
    }
    let step = app.world().resource::<NavSolveStepCounter>().0;
    assert_eq!(step, 4, "four ticks were driven by hand");
    assert!(
        movement_policy::should_solve(step, 2),
        "tick 4 is a solve tick at interval 2 -- the warm-up assumption this test relies on"
    );
    let position_after_solve_tick = app.world().get::<Transform>(agent).unwrap().translation;

    // Tick 5: a skip tick (5 % 2 = 1). Movement must still run.
    run_one_fixed_tick(app.world_mut());
    assert!(
        !movement_policy::should_solve(5, 2),
        "tick 5 is a skip tick at interval 2 -- the assertion below relies on this"
    );
    let position_after_skip_tick = app.world().get::<Transform>(agent).unwrap().translation;

    assert_ne!(
        Vec2::new(position_after_solve_tick.x, position_after_solve_tick.z),
        Vec2::new(position_after_skip_tick.x, position_after_skip_tick.z),
        "the agent must keep moving horizontally on a fixed tick the solve is gated off"
    );
}

/// Task 2: a landmass character mirrors the FPS player's position and
/// actual KCC velocity every fixed tick, and is present in the same
/// archipelago the agent/island use (`ArchipelagoRef3d` points at it).
/// The player entity is spawned through the real production path
/// (`player::set_camera_mode`) rather than constructed by hand: both
/// `FpsPlayer` and the rest of `KccState`'s fields are private outside
/// `player`, and this wave's file-ownership boundary allows exactly one
/// accessor edit to `player/mod.rs` (`KccState::velocity`, made
/// `pub(crate)`), not a test-only constructor.
#[test]
fn a_landmass_character_mirrors_the_player_and_exists_in_the_archipelago() {
    let mut app = fixed_tick_test_app();
    let archipelago_entity = spawn_fixture_island(app.world_mut());
    let character_entity = spawn_player_nav_character(app.world_mut(), archipelago_entity);
    app.world_mut()
        .resource_mut::<NavArchipelagoState>()
        .player_character = Some(character_entity);

    app.world_mut().init_resource::<player::CameraModeState>();
    app.world_mut().init_resource::<player::PlayerNoClip>();
    app.world_mut()
        .insert_resource(player::PhysicsDisabled(false));
    app.world_mut()
        .init_resource::<crate::console::RefRegistry>();
    let camera_local_height = player::EYE_HEIGHT - player::CAPSULE_HEIGHT * 0.5;
    let player_center = Vec3::new(1.0, 0.0, 1.0);
    let camera_transform =
        Transform::from_translation(player_center + Vec3::Y * camera_local_height);
    app.world_mut().spawn((
        Camera3d::default(),
        camera_transform,
        GlobalTransform::from(camera_transform),
        crate::viewer::FlyCamera {
            yaw: 0.0,
            pitch: 0.0,
            speed: 0.0,
        },
    ));
    player::set_camera_mode(app.world_mut(), player::CameraMode::Fps)
        .expect("an FPS player spawns from a fresh Free-mode camera");
    let player_entity = app
        .world()
        .resource::<player::CameraModeState>()
        .player
        .expect("set_camera_mode recorded the new player entity");

    let player_velocity = Vec3::new(1.5, 0.0, -0.5);
    app.world_mut()
        .get_mut::<player::KccState>(player_entity)
        .expect("set_camera_mode spawned a KccState")
        .velocity = player_velocity;

    // Force transform propagation once so the player's `GlobalTransform`
    // reflects the `Transform` `set_camera_mode` just set -- this
    // minimal App has no render/window stack driving `app.update()`, so
    // propagation is run directly by schedule label.
    app.world_mut().run_schedule(PostUpdate);

    run_one_fixed_tick(app.world_mut());

    let character_transform = app.world().get::<Transform>(character_entity).unwrap();
    assert!(
        character_transform.translation.distance(player_center) < 1e-3,
        "the character must mirror the player's position, got {:?}",
        character_transform.translation
    );
    let character_velocity = app
        .world()
        .get::<Velocity3d>(character_entity)
        .unwrap()
        .velocity;
    assert_eq!(
        character_velocity, player_velocity,
        "the character must mirror the player's actual KCC velocity"
    );

    let archipelago_ref = app
        .world()
        .get::<ArchipelagoRef3d>(character_entity)
        .expect("the character carries an ArchipelagoRef3d");
    assert_eq!(
        archipelago_ref.entity, archipelago_entity,
        "the character is present in the same archipelago the agent/island use"
    );
}

/// Task 2 (continued): a landmass character standing directly on an
/// agent's straight-line path deflects the agent's desired velocity away
/// from that straight line -- RVO avoidance treating the character as a
/// non-agent obstacle, driven against a real archipelago (the same
/// pattern `nav_overlay.rs`'s own landmass harness test uses).
#[test]
fn a_landmass_character_in_the_agents_path_deflects_its_desired_velocity() {
    let mut app = fixed_tick_test_app();
    let archipelago_entity = spawn_fixture_island(app.world_mut());

    // `start`/`target` are the logical navmesh-plane (`y = 0.0`) points
    // the straight-line/character-placement math below works in;
    // `spawn_bare_agent` gets a *capsule-centre* start position instead
    // (`Transform.translation` is the capsule centre, mirroring
    // production's `spawn_test_agent`) so it actually stands on
    // `fixed_tick_test_app`'s floor rather than starting embedded in it.
    let start = Vec3::new(0.5, 0.0, 0.5);
    let target = Vec3::new(3.5, 0.0, 3.5);
    let agent = spawn_bare_agent(
        app.world_mut(),
        archipelago_entity,
        Vec3::new(start.x, AGENT_HEIGHT / 2.0, start.z),
        target,
    );

    // A character close enough to the agent's straight-line path that a
    // collision is predicted from the very first tick (RVO's avoidance
    // only predicts a collision within its 0.5s time horizon --
    // `ArchipelagoOptions::from_agent_radius`'s default -- so a
    // character sitting far down the path would not yet register as a
    // threat at the agent's initial, still-ramping-up speed), nudged a
    // hair off the path's exact centreline. A perfectly centred,
    // perfectly head-on approach is a degenerate case for RVO/ORCA --
    // slowing straight down is exactly as valid a non-colliding
    // solution as swerving either way when the geometry is perfectly
    // symmetric, and dodgy_2d picks that (confirmed empirically: dead-
    // centre placement here converges on a shrinking, undeflected
    // desired velocity, not a sideways one). A small perpendicular
    // offset breaks the symmetry the same way a real player almost
    // never walks exactly down an agent's route centreline.
    let direction = (target - start).normalize();
    let perpendicular = Vec3::new(-direction.z, 0.0, direction.x);
    let close_point = start + direction * 1.0 + perpendicular * 0.15;
    app.world_mut().spawn((
        Character3dBundle {
            character: default(),
            settings: CharacterSettings {
                radius: player::CAPSULE_RADIUS,
            },
            archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
        },
        Transform::from_translation(close_point),
        Velocity3d::default(),
    ));

    // Default solve rate (every tick): let the solve settle over enough
    // ticks for the agent to close in on the character and for RVO's
    // avoidance response to actually deflect it.
    for _ in 0..60 {
        run_one_fixed_tick(app.world_mut());
    }

    let blend = app.world().get::<AgentDesiredVelocityBlend>(agent).unwrap();
    let desired = blend.latest;
    assert!(
        desired.length() > 0.01,
        "the agent must still have a nonzero desired velocity with the character present, got {desired:?}"
    );

    let straight_line = (target - start).normalize();
    let desired_direction = desired.normalize();
    let cos_angle = straight_line.dot(desired_direction);
    assert!(
        cos_angle < 0.99,
        "a character blocking the straight-line path must deflect the agent's desired velocity away from it (cos={cos_angle}, desired={desired:?})"
    );
}

/// Task 3 (solve-output interpolation, user-directed addendum): at
/// interval 2, on the in-between (skip) tick, the desired velocity
/// `apply_agent_physics_movement` actually applies is strictly between
/// the two most recently completed solve outputs -- not equal to
/// either. At interval 1, it is always exactly the latest solved value,
/// regardless of whatever `previous` holds -- confirming the
/// interpolation is an exact no-op at the default rate. Uses an empty
/// `boxddd::World` (no static geometry) so the achieved horizontal
/// velocity written back to `Velocity3d` is the *unobstructed* applied
/// input exactly -- a direct, physics-real assertion on the actual
/// consuming system, not just the pure `solve_blend_fraction` table.
#[test]
fn desired_velocity_blends_between_solves_and_is_exact_at_interval_one() {
    use bevy::ecs::system::RunSystemOnce;

    fn blend_test_world(blend: AgentDesiredVelocityBlend) -> (World, Entity) {
        let mut world = World::new();
        world.init_resource::<TestNavAgentState>();
        world.insert_resource(PhysicsDisabled(false));
        world.insert_resource(CellPhysicsReadiness::Ready);
        world.init_resource::<Time>();
        world
            .resource_mut::<Time>()
            .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
        world.insert_non_send(BoxdddPhysicsContext::from_world(
            boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world"),
        ));
        let agent = world
            .spawn((
                TestNavAgentMarker,
                AgentKcc::default(),
                blend,
                Transform::from_xyz(0.0, 5.0, 0.0),
                Velocity3d::default(),
            ))
            .id();
        (world, agent)
    }

    let previous = Vec3::new(2.5, 0.0, 0.0);
    let latest = Vec3::new(0.0, 0.0, 2.5);
    let blend = AgentDesiredVelocityBlend { previous, latest };

    // Interval 2, on a skip tick (3 % 2 = 1, fraction 0.5): strictly
    // between the two, not equal to either.
    let (mut world, agent) = blend_test_world(blend);
    world.insert_resource(NavSolveRate(2));
    world.insert_resource(NavSolveStepCounter(3));
    world
        .run_system_once(apply_agent_physics_movement)
        .expect("system runs");
    let achieved = world.get::<Velocity3d>(agent).unwrap().velocity;
    assert!(
        achieved.x > 0.0 && achieved.x < previous.x,
        "achieved.x={} must be strictly between 0.0 (latest.x) and {} (previous.x)",
        achieved.x,
        previous.x
    );
    assert!(
        achieved.z > 0.0 && achieved.z < latest.z,
        "achieved.z={} must be strictly between 0.0 (previous.z) and {} (latest.z)",
        achieved.z,
        latest.z
    );

    // Interval 1: always exactly the latest value, regardless of the
    // step counter or of `previous`.
    let (mut world, agent) = blend_test_world(blend);
    world.insert_resource(NavSolveRate(1));
    world.insert_resource(NavSolveStepCounter(7));
    world
        .run_system_once(apply_agent_physics_movement)
        .expect("system runs");
    let achieved = world.get::<Velocity3d>(agent).unwrap().velocity;
    assert!(
        (achieved.x - latest.x).abs() < 1e-3,
        "at interval 1 the applied value must equal `latest` exactly, got achieved.x={}",
        achieved.x
    );
    assert!(
        (achieved.z - latest.z).abs() < 1e-3,
        "at interval 1 the applied value must equal `latest` exactly, got achieved.z={}",
        achieved.z
    );
}

// -------------------------------------------------------------
// Issue #155 features 1/2: door polygon typing + query-time lock
// exclusion, exercised against a real `Archipelago3d` solve (this file
// owns the live-Bevy tests -- `landmass_graph.rs` stays Bevy-engine-free,
// see its module doc comment). No physics/floor and no `FixedUpdate`
// movement chain is needed here (unlike `fixed_tick_test_app`'s other
// consumers): these tests only assert `AgentState`, which
// `Landmass3dPlugin`'s `FixedPreUpdate` systems alone produce, exactly
// mirroring `nav_overlay.rs`'s own minimal landmass-only harness test.
// -------------------------------------------------------------

/// Two rooms (`Room A` west, `Room B` east), connected by two
/// *independent* two-triangle corridors that share no vertex with each
/// other: a "door" corridor (triangles 4/5, typed under door FormID
/// `0x99` when `with_bypass` doors are wanted) along the south edges,
/// and -- only when `with_bypass` is true -- a plain "bypass" corridor
/// (triangles 6/7, never typed) along the north edges. `with_bypass:
/// false` yields a mesh where the door corridor is the *only* route
/// between the rooms (invariants 1/3); `true` adds the independent
/// alternate route (invariant 2). Room A's interior point `(0.7, 0.0,
/// 1.0)` and Room B's interior point `(8.7, 0.0, 1.7)` are this
/// fixture's start/target throughout (Room B is offset +1 in Z from
/// Room A -- see the vertex list below for why).
fn door_topology_mesh(with_bypass: bool) -> landmass_graph::MeshInput {
    let vertices = vec![
        [0.0, 0.0, 0.0], // 0: Room A SW
        [2.0, 0.0, 0.0], // 1: Room A SE
        [0.0, 0.0, 4.0], // 2: Room A NW
        [2.0, 0.0, 4.0], // 3: Room A NE
        // Room B is offset +1 in Z relative to Room A (z:1..5, not
        // z:0..4): using the *same* Z range as Room A would put both
        // rooms' south edges (and both north edges) on the exact same
        // Z line, making the door/bypass quads degenerate (three
        // collinear corners, zero area) instead of real triangles.
        [8.0, 0.0, 1.0],  // 4: Room B SW
        [10.0, 0.0, 1.0], // 5: Room B SE
        [8.0, 0.0, 5.0],  // 6: Room B NW
        [10.0, 0.0, 5.0], // 7: Room B NE
    ];
    let mut polygons = vec![
        // Room A (SW/NE halves).
        landmass_graph::PolygonInput {
            index: 0,
            vertex_indices: [0, 1, 2],
            is_water: false,
            is_preferred_pathing: false,
        },
        landmass_graph::PolygonInput {
            index: 1,
            vertex_indices: [1, 3, 2],
            is_water: false,
            is_preferred_pathing: false,
        },
        // Room B (SW/NE halves).
        landmass_graph::PolygonInput {
            index: 2,
            vertex_indices: [4, 5, 6],
            is_water: false,
            is_preferred_pathing: false,
        },
        landmass_graph::PolygonInput {
            index: 3,
            vertex_indices: [5, 7, 6],
            is_water: false,
            is_preferred_pathing: false,
        },
        // Door corridor: Room A's south edge (0,1) <-> Room B's south
        // edge (4,5).
        landmass_graph::PolygonInput {
            index: 4,
            vertex_indices: [0, 1, 4],
            is_water: false,
            is_preferred_pathing: false,
        },
        landmass_graph::PolygonInput {
            index: 5,
            vertex_indices: [1, 5, 4],
            is_water: false,
            is_preferred_pathing: false,
        },
    ];
    if with_bypass {
        // Bypass corridor: Room A's north edge (3,2) <-> Room B's north
        // edge (7,6), reusing those rooms' own existing corner vertices
        // (no new vertices needed) -- and, critically, sharing no
        // vertex at all with the door corridor's own (0,1,4,5), so the
        // two corridors are topologically independent routes.
        polygons.push(landmass_graph::PolygonInput {
            index: 6,
            vertex_indices: [2, 3, 6],
            is_water: false,
            is_preferred_pathing: false,
        });
        polygons.push(landmass_graph::PolygonInput {
            index: 7,
            vertex_indices: [3, 7, 6],
            is_water: false,
            is_preferred_pathing: false,
        });
    }
    landmass_graph::MeshInput {
        form_id: 0x10,
        vertices,
        polygons,
        doors: vec![
            landmass_graph::DoorInput {
                triangle_index: 4,
                door_reference_form_id: Some(0x99),
            },
            landmass_graph::DoorInput {
                triangle_index: 5,
                door_reference_form_id: Some(0x99),
            },
        ],
        derived_doors: Vec::new(),
    }
}

const DOOR_TOPOLOGY_ROOM_A_POINT: Vec3 = Vec3::new(0.7, 0.0, 1.0);
const DOOR_TOPOLOGY_ROOM_B_POINT: Vec3 = Vec3::new(8.7, 0.0, 1.7);

/// Builds a minimal landmass-only App (mirrors `nav_overlay.rs`'s own
/// harness test, not `fixed_tick_test_app`'s physics-laden one -- these
/// tests only need `AgentState`, which `Landmass3dPlugin`'s own
/// `FixedPreUpdate` systems alone produce), spawns `door_topology_mesh`
/// as a single island, and spawns one agent at `DOOR_TOPOLOGY_ROOM_A_
/// POINT` targeting `DOOR_TOPOLOGY_ROOM_B_POINT`. `lock_override`, if
/// `Some`, is inserted on the agent *before* the first solve -- the
/// "door already locked when the query is issued" shape the wave's
/// acceptance script exercises, and issue #155 feature 2's actual
/// contract (`apply_door_lock_overrides` is exercised separately, at
/// the `NavArchipelagoState`-driven integration level, by the mid-route
/// gating tests above; this harness drives the raw `bevy_landmass`
/// component directly since it has no `NavArchipelagoState`/manifest to
/// build one from).
fn door_topology_test_app(with_bypass: bool, lock_override: Option<f32>) -> (App, Entity) {
    let mesh = door_topology_mesh(with_bypass);
    let door_type_indices = landmass_graph::door_type_indices(std::slice::from_ref(&mesh));
    assert_eq!(
        door_type_indices.get(&0x99),
        Some(&1),
        "test setup: the door must resolve to type index 1"
    );
    let build_result =
        landmass_graph::build_navigation_mesh(&mesh, &[], &door_type_indices, &BTreeMap::new());
    let valid = build_result.nav_mesh.unwrap_or_else(|| {
        panic!(
            "door_topology_mesh always validates: {:?}",
            build_result.diagnostics
        )
    });

    let mut app = App::new();
    app.add_plugins((
        bevy::MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        Landmass3dPlugin::default(),
    ));
    let nav_mesh_handle = app
        .world_mut()
        .resource_mut::<Assets<NavMesh3d>>()
        .add(NavMesh3d {
            nav_mesh: Arc::new(valid),
        });
    let archipelago = app
        .world_mut()
        .spawn(Archipelago3d::new(archipelago_options()))
        .id();
    app.world_mut().spawn(Island3dBundle {
        island: Island,
        archipelago_ref: ArchipelagoRef3d::new(archipelago),
        nav_mesh: NavMeshHandle::<ThreeD>(nav_mesh_handle),
    });

    let mut agent_entity = app.world_mut().spawn((
        Agent3dBundle {
            agent: default(),
            settings: AgentSettings {
                radius: AGENT_RADIUS,
                desired_speed: AGENT_DESIRED_SPEED,
                max_speed: AGENT_MAX_SPEED,
            },
            archipelago_ref: ArchipelagoRef3d::new(archipelago),
        },
        Transform::from_translation(DOOR_TOPOLOGY_ROOM_A_POINT),
        AgentTarget3d::Point(DOOR_TOPOLOGY_ROOM_B_POINT),
    ));
    if let Some(cost) = lock_override {
        let mut overrides = AgentTypeIndexCostOverrides::default();
        assert!(
            overrides.set_type_index_cost(1, cost),
            "test setup: the override cost must be > 0.0"
        );
        agent_entity.insert(overrides);
    }
    let agent = agent_entity.id();
    (app, agent)
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

// -----------------------------------------------------------------
// Issue #177: closed-blocker cost overrides compose with lock state
// -----------------------------------------------------------------

/// Bare-`World` fixture for `apply_door_lock_overrides`: one blocker
/// FormID with both a gate type index (priced on *usability*, i.e. lock)
/// and an interior/blocking type index (priced on *open*, and on whether
/// the blocker can be opened at all), so every combination can be
/// asserted on one component.
fn closed_blocker_override_world(usable: bool, open: bool, openable: bool) -> (World, Entity) {
    let mut world = harness_world();
    let agent = world.spawn_empty().id();
    let mut state = world.resource_mut::<NavArchipelagoState>();
    state.door_usable.insert(0x99, usable);
    state.door_open.insert(0x99, open);
    state.door_type_indices.insert(0x99, 1);
    state.closed_door_type_indices.insert(0x99, 2);
    if openable {
        state.openable_blockers.insert(0x99);
    }
    (world, agent)
}

fn override_costs(world: &World, agent: Entity) -> Vec<(usize, f32)> {
    let mut costs: Vec<(usize, f32)> = world
        .get::<AgentTypeIndexCostOverrides>(agent)
        .expect("the agent must carry overrides")
        .iter()
        .map(|(&index, &cost)| (index, cost))
        .collect();
    costs.sort_by_key(|(index, _)| *index);
    costs
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

/// An activator placement whose reference is `reference_form_id`, the
/// solid gear-door class issue #186 is about. `Default` audio carries no
/// sound FormIDs, so activation is silent in this harness.
fn activator_placement(reference_form_id: u32) -> crate::vsa::PreparedPlacement {
    let mut placement = door_placement(reference_form_id);
    placement.base_kind = "ACTI".into();
    placement.semantic = crate::vsa::PreparedSemantic::Activator;
    placement
}

/// Issue #186, the *signal* test (verdict §2.1): drive an activator
/// blocker through the **real interaction boundary** and assert nav's
/// override lifts -- deliberately not the #177 shape that pokes
/// `door_open` directly (`closed_blocker_override_world`), which is why
/// this class of desync shipped. A closed, not-openable gear door
/// (`VaultGearDoor`'s prepared shape: `openable = false`) is impassable;
/// activating it open through `scripted_activator_toggle` -> the shared
/// `InteractionState.open` signal -> `door_availability_system` clears the
/// override so the route is free; activating it shut restores it.
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

    let agent = world
        .spawn((TestNavAgentMarker, AgentRuntime::default()))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// The population itself, in isolation (verdict §1: the #177 cost tests
/// bypassed this signal, which is why the desync shipped): activating an
/// activator inserts it into `InteractionState.open`; this fails if the
/// open-state population is ever removed from the activator path.
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
fn derived_gate_and_blocking_associations_take_distinct_type_indices() {
    // Issue #177 feature 2: the two classes must never share an index,
    // or opening a door would clear the wrong override.
    let mesh = landmass_graph::MeshInput {
        form_id: 0x10,
        vertices: Vec::new(),
        polygons: Vec::new(),
        doors: Vec::new(),
        derived_doors: vec![
            landmass_graph::DerivedDoorInput {
                triangle_index: 1,
                door_reference_form_id: 0x99,
                blocks_when_closed: false,
                openable: true,
            },
            landmass_graph::DerivedDoorInput {
                triangle_index: 2,
                door_reference_form_id: 0x99,
                blocks_when_closed: true,
                openable: true,
            },
        ],
    };
    let meshes = [mesh];
    let door_indices = landmass_graph::door_type_indices(&meshes);
    let closed_indices = landmass_graph::closed_door_type_indices(&meshes, &door_indices);
    assert_eq!(door_indices.get(&0x99), Some(&1));
    assert_eq!(closed_indices.get(&0x99), Some(&2));
    assert_eq!(
        landmass_graph::preferred_pathing_type_index(&door_indices, &closed_indices),
        3
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

// -------------------------------------------------------------
// Issue #168: preferred-path base cost, exercised against a real
// `Archipelago3d` solve (this file owns the live-Bevy tests -- see
// `landmass_graph.rs`'s own module doc comment for why it stays
// Bevy-engine-free).
// -------------------------------------------------------------

/// A two-room mesh with two independent, geometrically congruent
/// corridors (issue #168): south (ordinary) and north (issue #156's
/// `NVTR` `PREFERRED_PATHING` flag, `is_preferred_pathing: true`) --
/// each corridor is the other translated by exactly `+8` in Z, so a
/// route through either is the identical length. `PREFERRED_PATH_
/// START`/`PREFERRED_PATH_TARGET` sit at each room's own Z-midpoint,
/// equidistant from both corridors by construction: only
/// `PREFERRED_PATHING_TYPE_INDEX_COST` (never distance) can make one
/// strictly cheaper than the other. Room B is offset `+0.5` in Z from
/// Room A at the corridor-connection edges -- the same non-degenerate-
/// triangle requirement `door_topology_mesh`'s own doc comment
/// explains (three vertices at the identical Z would make a
/// zero-area triangle) -- and both corridors carry the identical
/// offset, preserving their congruence.
fn preferred_path_mesh() -> landmass_graph::MeshInput {
    landmass_graph::MeshInput {
        form_id: 0x10,
        vertices: vec![
            [0.0, 0.0, 0.0],  // 0: Room A SW
            [2.0, 0.0, 0.0],  // 1: Room A SE
            [0.0, 0.0, 8.0],  // 2: Room A NW
            [2.0, 0.0, 8.0],  // 3: Room A NE
            [8.0, 0.0, 0.5],  // 4: Room B SW
            [10.0, 0.0, 0.5], // 5: Room B SE
            [8.0, 0.0, 8.5],  // 6: Room B NW
            [10.0, 0.0, 8.5], // 7: Room B NE
        ],
        polygons: vec![
            landmass_graph::PolygonInput {
                index: 0,
                vertex_indices: [0, 1, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
            landmass_graph::PolygonInput {
                index: 1,
                vertex_indices: [1, 3, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
            landmass_graph::PolygonInput {
                index: 2,
                vertex_indices: [4, 5, 6],
                is_water: false,
                is_preferred_pathing: false,
            },
            landmass_graph::PolygonInput {
                index: 3,
                vertex_indices: [5, 7, 6],
                is_water: false,
                is_preferred_pathing: false,
            },
            // South corridor (Room A/B south edges): ordinary.
            landmass_graph::PolygonInput {
                index: 4,
                vertex_indices: [0, 1, 4],
                is_water: false,
                is_preferred_pathing: false,
            },
            landmass_graph::PolygonInput {
                index: 5,
                vertex_indices: [1, 5, 4],
                is_water: false,
                is_preferred_pathing: false,
            },
            // North corridor (Room A/B north edges): preferred pathing.
            landmass_graph::PolygonInput {
                index: 6,
                vertex_indices: [2, 3, 6],
                is_water: false,
                is_preferred_pathing: true,
            },
            landmass_graph::PolygonInput {
                index: 7,
                vertex_indices: [3, 7, 6],
                is_water: false,
                is_preferred_pathing: true,
            },
        ],
        doors: Vec::new(),
        derived_doors: Vec::new(),
    }
}

const PREFERRED_PATH_START: Vec3 = Vec3::new(1.0, 0.0, 4.0);
const PREFERRED_PATH_TARGET: Vec3 = Vec3::new(9.0, 0.0, 4.5);

#[test]
fn a_preferred_corridor_is_chosen_over_an_equal_length_ordinary_one() {
    let mesh = preferred_path_mesh();
    let door_type_indices = BTreeMap::new();
    let preferred_index =
        landmass_graph::preferred_pathing_type_index(&door_type_indices, &BTreeMap::new());
    let build_result =
        landmass_graph::build_navigation_mesh(&mesh, &[], &door_type_indices, &BTreeMap::new());
    let valid = build_result.nav_mesh.unwrap_or_else(|| {
        panic!(
            "preferred_path_mesh always validates: {:?}",
            build_result.diagnostics
        )
    });

    let mut app = App::new();
    app.add_plugins((
        bevy::MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        Landmass3dPlugin::default(),
    ));
    let nav_mesh_handle = app
        .world_mut()
        .resource_mut::<Assets<NavMesh3d>>()
        .add(NavMesh3d {
            nav_mesh: Arc::new(valid),
        });
    let mut archipelago_component = Archipelago3d::new(archipelago_options());
    // The exact production call under test (issue #168,
    // `apply_preferred_pathing_base_cost`).
    archipelago_component
        .set_type_index_cost(preferred_index, PREFERRED_PATHING_TYPE_INDEX_COST)
        .expect("PREFERRED_PATHING_TYPE_INDEX_COST is a positive finite documented constant");
    let archipelago = app.world_mut().spawn(archipelago_component).id();
    app.world_mut().spawn(Island3dBundle {
        island: Island,
        archipelago_ref: ArchipelagoRef3d::new(archipelago),
        nav_mesh: NavMeshHandle::<ThreeD>(nav_mesh_handle),
    });
    let agent = app
        .world_mut()
        .spawn((
            Agent3dBundle {
                agent: default(),
                settings: AgentSettings {
                    radius: AGENT_RADIUS,
                    desired_speed: AGENT_DESIRED_SPEED,
                    max_speed: AGENT_MAX_SPEED,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago),
            },
            Transform::from_translation(PREFERRED_PATH_START),
            AgentTarget3d::Point(PREFERRED_PATH_TARGET),
        ))
        .id();

    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);

    assert_ne!(
        app.world().get::<AgentState>(agent).copied(),
        Some(AgentState::NoPath),
        "both corridors are open; a path must be found"
    );
    let desired = app.world().get::<AgentDesiredVelocity3d>(agent).unwrap();
    assert!(
        desired.velocity().z > 0.0,
        "the cheaper preferred (north, +Z) corridor must be chosen over the \
             equal-length ordinary (south, -Z) one, got desired velocity {:?}",
        desired.velocity()
    );
}

// -------------------------------------------------------------
// Issue #165: locked travel-target door respects runtime lock state.
// -------------------------------------------------------------

/// Real-data root cause (found by driving `locked_travel_door_fails_
/// deterministically_without_opening`'s shape through the actual
/// `NavBackendPlugin` schedule instead of hand-calling `door_link_
/// system`): `request_door_open`'s internal lock check already refused
/// to open the door, so the door genuinely never opened -- but the
/// `Failed` transition only cleared `travel_intent`, leaving
/// `AgentTarget3d` still pointed at the door's own triangle. Every real
/// travel door is also a `mid_route_doors` candidate (`nav/agent.rs`'s
/// module doc: `single_sided_doors` populates both sets), and the
/// mid-route gate's travel-intent exclusion is keyed on `travel_intent`
/// alone -- once that clears, the very next tick the gate "rediscovers"
/// the agent standing in the door's own triangle with a target still
/// set, and restarts the whole pause -> wait -> `Failed` cycle via
/// `IntraCell`, forever: `tna status` observed alternating between
/// `Paused` and `Unreachable` on a real locked travel door instead of
/// settling at the documented terminal. This test pins that exact
/// shape (the door registered in both `travel_doors` and
/// `mid_route_doors`, as every real one is) and proves the fix holds
/// across many more ticks than `MAX_WAIT_TICKS`, not just the first
/// `Failed` transition.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(5.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// F165.2: unlocking the door and reissuing the travel (the existing
/// one-repath retry contract -- `request_travel` only refuses a
/// concurrent request, and `door_link::transition`'s own table already
/// restarts the lifecycle cleanly from `Failed` on a fresh
/// `LinkReached`) completes the hand-off normally.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(5.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

/// Real-data acceptance follow-up (orchestrator, contaminated-leg-B
/// measurement): a *prior* successful travel through this exact door
/// leaves it physically open in `InteractionState.open` forever (a
/// hand-off never closes it). A later `setlock` + reissued `tna
/// travel` then reaches the travel-arrival branch with the door
/// already open on the very first tick -- no fresh scripted-open
/// request is ever needed, so the lock check living on the open-
/// *request* path (the arrival branch's `crossing_gate` consult,
/// `request_door_open`'s internal check) never runs, and without the
/// `Paused`-arm fix below the agent would walk straight through into
/// `Traversing` -> `TravelReached` -> a scripted hand-off through a
/// locked door. A hand-off is a scripted cell transition, not a
/// physical walk-through: lock state must be authoritative for it
/// regardless of the door's current physical open state.
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
            TestNavAgentMarker,
            AgentRuntime::default(),
            Transform::from_xyz(5.0, 0.0, 0.0),
            AgentTarget3d::Point(Vec3::new(5.0, 0.0, 0.0)),
        ))
        .id();
    world.resource_mut::<TestNavAgentState>().entities[0] = Some(agent);
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

// ---------------------------------------------------------------
// Issue #172: authored-stair step capability of the agent KCC.
//
// These pin the swept-capsule step behaviour `step_agent_kcc` gets
// from the shared `player::move_mover`/`try_step_up`/`try_step_down`
// helpers against FO3-scale stair geometry built as *triangle meshes*
// (the shape authored `AuthoredHavok` statics cook to), including the
// seam between two adjacent colliders.
//
// They exist because #172 was filed as a stair-climbing defect after
// agents wedged in Vault 101 Entrance (00024512) at z ~= -80.4.
// Replaying that cell's real collision through this same
// `step_agent_kcc` entry point showed the wedge is *not* a step
// failure: the capsule is pressed against the closed `VaultGearDoor`
// activator collider, whose face sits at z = -80.0 (agent radius 0.35
// -> capsule centre stops at -80.35, the measured value). Removing
// that one collider from the replay lets the agent walk straight
// through. See the issue for the full evidence. The coverage below
// stays as the regression guard that stair traversal itself is, and
// remains, sound.
// ---------------------------------------------------------------

/// Appends an axis-aligned box as triangles, wound both ways: prepared
/// static collision is cooked two-sided (see `player::collision`'s
/// `TriangleMesh` path), so fixtures must be too.
fn push_box_triangles(
    vertices: &mut Vec<boxddd::Vec3>,
    indices: &mut Vec<i32>,
    min: [f32; 3],
    max: [f32; 3],
) {
    let base = i32::try_from(vertices.len()).expect("fixture vertex count fits in i32");
    for &(x, y, z) in &[
        (min[0], min[1], min[2]),
        (max[0], min[1], min[2]),
        (max[0], min[1], max[2]),
        (min[0], min[1], max[2]),
        (min[0], max[1], min[2]),
        (max[0], max[1], min[2]),
        (max[0], max[1], max[2]),
        (min[0], max[1], max[2]),
    ] {
        vertices.push(boxddd::Vec3::new(x, y, z));
    }
    const FACES: [[i32; 3]; 12] = [
        [0, 1, 2],
        [0, 2, 3],
        [4, 6, 5],
        [4, 7, 6],
        [0, 4, 5],
        [0, 5, 1],
        [1, 5, 6],
        [1, 6, 2],
        [2, 6, 7],
        [2, 7, 3],
        [3, 7, 4],
        [3, 4, 0],
    ];
    for face in FACES {
        indices.extend_from_slice(&[base + face[0], base + face[1], base + face[2]]);
        indices.extend_from_slice(&[base + face[0], base + face[2], base + face[1]]);
    }
}

fn add_fixture_mesh(world: &mut boxddd::World, vertices: Vec<boxddd::Vec3>, indices: Vec<i32>) {
    let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    let mesh = boxddd::MeshData::builder(vertices, indices)
        .build()
        .expect("fixture triangle mesh");
    world
        .try_create_mesh_shape(
            body,
            &fixture_shape_def(),
            mesh,
            boxddd::Vec3::new(1.0, 1.0, 1.0),
        )
        .expect("fixture triangle mesh shape");
}

const STAIR_STEPS: usize = 10;
const STAIR_RISE: f32 = 0.24;
const STAIR_RUN: f32 = 0.28;

/// A flight of `STAIR_STEPS` FO3-scale treads ascending in +Z between
/// two landings, split into **two separate TriangleMesh statics** after
/// `seam_after` treads -- the two-collider seam #172 called out.
fn add_stair_fixture(world: &mut boxddd::World, seam_after: usize) {
    let half_width = 2.0;
    let mut lower = (Vec::new(), Vec::new());
    let mut upper = (Vec::new(), Vec::new());
    push_box_triangles(
        &mut lower.0,
        &mut lower.1,
        [-half_width, -1.0, -4.0],
        [half_width, 0.0, 0.0],
    );
    for index in 0..STAIR_STEPS {
        let z0 = index as f32 * STAIR_RUN;
        let top = (index + 1) as f32 * STAIR_RISE;
        let target = if index < seam_after {
            &mut lower
        } else {
            &mut upper
        };
        push_box_triangles(
            &mut target.0,
            &mut target.1,
            [-half_width, top - 1.0, z0],
            [half_width, top, z0 + STAIR_RUN],
        );
    }
    let top = STAIR_STEPS as f32 * STAIR_RISE;
    let z0 = STAIR_STEPS as f32 * STAIR_RUN;
    push_box_triangles(
        &mut upper.0,
        &mut upper.1,
        [-half_width, top - 1.0, z0],
        [half_width, top, z0 + 4.0],
    );
    add_fixture_mesh(world, lower.0, lower.1);
    add_fixture_mesh(world, upper.0, upper.1);
}

/// Walks the agent capsule through `step_agent_kcc` for `ticks` fixed
/// steps at `AGENT_DESIRED_SPEED`, returning the position trace.
fn walk_agent(world: &mut boxddd::World, start: Vec3, desired: Vec2, ticks: usize) -> Vec<Vec3> {
    let mover = fixture_capsule();
    let filter = fixture_filter();
    let mut position = start;
    let mut velocity = Vec3::ZERO;
    let mut grounded = false;
    let mut trace = Vec::with_capacity(ticks);
    for _ in 0..ticks {
        let (new_position, new_velocity, new_grounded) = step_agent_kcc(
            world,
            &mover,
            filter,
            filter,
            position,
            velocity,
            grounded,
            desired,
            1.0 / 60.0,
        );
        position = new_position;
        velocity = new_velocity;
        grounded = new_grounded;
        trace.push(position);
    }
    trace
}

/// F172.1 (ascending): the swept KCC climbs authored-scale risers and
/// carries the climb across the seam between two TriangleMesh statics.
#[test]
fn agent_kcc_climbs_authored_scale_stairs_across_a_collider_seam() {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_stair_fixture(&mut world, STAIR_STEPS / 2);
    let top_z = STAIR_STEPS as f32 * STAIR_RUN;
    let top_y = STAIR_STEPS as f32 * STAIR_RISE;

    let trace = walk_agent(
        &mut world,
        Vec3::new(0.0, AGENT_HEIGHT / 2.0 + 0.1, -2.0),
        Vec2::new(0.0, AGENT_DESIRED_SPEED),
        360,
    );

    let arrived = trace
        .iter()
        .find(|position| position.z > top_z + 0.5)
        .unwrap_or_else(|| {
            panic!(
                "agent must reach the top landing; wedged at {:?}",
                trace.last()
            )
        });
    assert!(
        (arrived.y - (top_y + AGENT_HEIGHT / 2.0)).abs() < 0.15,
        "agent must stand on the top landing, got {arrived:?} (landing y {top_y})"
    );
}

/// F172.1 (descending): the same flight, walked downward. Guards the
/// step-down probe, and with it the #164 fall guard's premise that
/// walking a stair down is never a fall.
#[test]
fn agent_kcc_descends_authored_scale_stairs_across_a_collider_seam() {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_stair_fixture(&mut world, STAIR_STEPS / 2);
    let top_z = STAIR_STEPS as f32 * STAIR_RUN;
    let top_y = STAIR_STEPS as f32 * STAIR_RISE;

    let trace = walk_agent(
        &mut world,
        Vec3::new(0.0, top_y + AGENT_HEIGHT / 2.0 + 0.1, top_z + 2.0),
        Vec2::new(0.0, -AGENT_DESIRED_SPEED),
        360,
    );

    let arrived = trace
        .iter()
        .find(|position| position.z < -0.5)
        .unwrap_or_else(|| {
            panic!(
                "agent must reach the bottom landing; wedged at {:?}",
                trace.last()
            )
        });
    assert!(
        (arrived.y - AGENT_HEIGHT / 2.0).abs() < 0.15,
        "agent must walk the flight down rather than fall it, got {arrived:?}"
    );
    assert!(
        trace
            .iter()
            .take_while(|position| position.z > -0.5)
            .all(|position| position.y > AGENT_HEIGHT / 2.0 - 0.2),
        "the descent must stay on the treads -- never drop below the bottom landing"
    );
}

/// F172.1 (negative): step handling stays bounded. A ledge taller than
/// the shared step height is not climbable, so the agent stops in
/// front of it rather than being lifted onto it.
#[test]
fn agent_kcc_refuses_a_ledge_taller_than_step_height() {
    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(0.0, -0.5, -2.0),
        boxddd::Vec3::new(4.0, 0.5, 4.0),
    );
    // Ledge top at y = 0.8, well above the ~0.486 m step height.
    add_fixture_box(
        &mut world,
        boxddd::Vec3::new(0.0, -0.1, 4.0),
        boxddd::Vec3::new(4.0, 0.9, 2.0),
    );

    let trace = walk_agent(
        &mut world,
        Vec3::new(0.0, AGENT_HEIGHT / 2.0 + 0.1, -4.0),
        Vec2::new(0.0, AGENT_DESIRED_SPEED),
        360,
    );

    let last = trace.last().copied().expect("trace is non-empty");
    assert!(
        last.y < 0.8 + AGENT_HEIGHT / 2.0 - 0.1,
        "a ledge above step height must not be climbed, got {last:?}"
    );
    assert!(
        last.z < 2.0,
        "the agent must come to rest in front of the ledge, got {last:?}"
    );
}

// ---------------------------------------------------------------
// Issue #148 wedge investigation harness (env-gated, no committed
// game data). Rebuilds a prepared cell's collision through the *real*
// `player::create_prepared_shape` cook, keeps a shape -> placement
// map the runtime does not keep, and replays `step_agent_kcc` so a
// wedge can be attributed to a named collider.
//
//   BEVYOUT_WEDGE_SCENE=/abs/path/scene.ron \
//   BEVYOUT_WEDGE_START=9.6,106,-73.1 \
//   BEVYOUT_WEDGE_TARGET=5,106,-73 \
//   cargo test-dev --lib wedge_replay -- --nocapture --ignored
// ---------------------------------------------------------------

fn wedge_vec(name: &str, fallback: Vec3) -> Vec3 {
    let Ok(raw) = std::env::var(name) else {
        return fallback;
    };
    let parts = raw
        .split(',')
        .map(|part| part.trim().parse::<f32>().expect("numeric wedge vector"))
        .collect::<Vec<_>>();
    assert_eq!(parts.len(), 3, "{name} must be x,y,z");
    Vec3::new(parts[0], parts[1], parts[2])
}

struct WedgeWorld {
    world: boxddd::World,
    owners: HashMap<u32, String>,
}

impl WedgeWorld {
    fn owner(&self, shape: boxddd::ShapeId) -> String {
        self.owners
            .get(&shape_key(shape))
            .cloned()
            .unwrap_or_else(|| format!("<unmapped shape {:?}>", shape))
    }
}

fn shape_key(shape: boxddd::ShapeId) -> u32 {
    // `ShapeId` is opaque; its Debug form is stable enough to key on
    // within one world, and cheaper than threading a parallel index.
    let text = format!("{shape:?}");
    let digits = text
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();
    digits.parse().unwrap_or(0)
}

/// Cooks every enabled placement's prepared collision exactly the way
/// `player::build_prepared_colliders` does (same shapes, same
/// categories/masks), recording which placement each shape came from.
fn build_wedge_world(scene: &std::path::Path, skip: &[u32]) -> WedgeWorld {
    let text = std::fs::read_to_string(scene).expect("scene manifest");
    let manifest: crate::vsa::PreparedSceneManifest =
        ron::de::from_str(&text).expect("valid scene manifest");
    let asset_root = scene.parent().unwrap().parent().unwrap().parent().unwrap();

    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    let static_body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    let mut owners = HashMap::new();

    for placement in &manifest.placements {
        if !placement.initially_enabled || skip.contains(&placement.reference_form_id) {
            continue;
        }
        if matches!(
            placement.semantic,
            crate::vsa::PreparedSemantic::Npc(_) | crate::vsa::PreparedSemantic::Creature(_)
        ) {
            continue;
        }
        let Some(relative) = placement.physics_asset_path.as_ref() else {
            continue;
        };
        let path = asset_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
        let Ok(asset) = crate::vsa::read_physics_asset(&path) else {
            continue;
        };
        let dynamic =
            placement.physics_classification == crate::vsa::PreparedPhysicsClassification::Dynamic;
        for body in &asset.bodies {
            let body_id = if dynamic {
                world.create_body(BodyDef::builder().body_type(BodyType::Dynamic).build())
            } else {
                static_body
            };
            for shape in &body.shapes {
                let created = player::create_prepared_shape(
                    &mut world,
                    body_id,
                    body,
                    shape,
                    placement,
                    player::PreparedShapeOptions {
                        dynamic,
                        local_space: false,
                        collision_group: 0,
                    },
                );
                if let Some((shape_id, _)) = created {
                    owners.insert(
                        shape_key(shape_id),
                        format!(
                            "{} ({:08x}) {:?}/{}",
                            placement.editor_id.as_deref().unwrap_or("<no editor id>"),
                            placement.reference_form_id,
                            placement.physics_classification,
                            shape.kind(),
                        ),
                    );
                }
            }
        }
    }
    WedgeWorld { world, owners }
}

#[test]
#[ignore = "requires a prepared cell: set BEVYOUT_WEDGE_SCENE"]
fn wedge_replay() {
    let Ok(scene) = std::env::var("BEVYOUT_WEDGE_SCENE") else {
        return;
    };
    let scene = std::path::PathBuf::from(scene);
    let start = wedge_vec("BEVYOUT_WEDGE_START", Vec3::new(9.6, 106.0, -73.1));
    let target = wedge_vec("BEVYOUT_WEDGE_TARGET", Vec3::new(5.0, 106.0, -73.0));

    let skip = std::env::var("BEVYOUT_WEDGE_SKIP")
        .ok()
        .map(|raw| {
            raw.split(',')
                .filter(|part| !part.trim().is_empty())
                .map(|part| {
                    u32::from_str_radix(part.trim().trim_start_matches("0x"), 16)
                        .expect("hex reference form id")
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let mut wedge = build_wedge_world(&scene, &skip);
    {
        let mover = fixture_capsule();
        let cf = player::player_collision_filter();
        for (label, probe) in [
            ("spawn", start + Vec3::new(0.0, AGENT_HEIGHT / 2.0, 0.0)),
            ("target", target + Vec3::new(0.0, AGENT_HEIGHT / 2.0, 0.0)),
        ] {
            let bp = player::to_box_vec3(probe);
            let planes = wedge
                .world
                .collide_mover(bp, &mover, cf)
                .unwrap_or_default();
            let ground = wedge
                .world
                .cast_mover(bp, &mover, boxddd::Vec3::new(0.0, -1.2, 0.0), cf)
                .unwrap_or(1.0);
            println!(
                "{label} ({:.2},{:.2},{:.2}): contacts={} ground_cast={ground:.3}",
                probe.x,
                probe.y,
                probe.z,
                planes.len()
            );
            for plane in planes.iter().take(4) {
                println!(
                    "    n=({:.2},{:.2},{:.2}) <- {}",
                    plane.plane.normal.x,
                    plane.plane.normal.y,
                    plane.plane.normal.z,
                    wedge.owner(plane.shape_id)
                );
            }
        }
    }
    println!("cooked {} shapes (skipped {skip:08x?})", wedge.owners.len());

    let mover = fixture_capsule();
    let collision_filter = player::player_collision_filter();
    let support_filter = player::stair_support_filter();

    let mut position = start + Vec3::new(0.0, AGENT_HEIGHT / 2.0, 0.0);
    let mut velocity = Vec3::ZERO;
    let mut grounded = false;
    for tick in 0..600 {
        let to_target = Vec2::new(target.x - position.x, target.z - position.z);
        let desired = to_target.normalize_or_zero() * AGENT_DESIRED_SPEED;
        let (p, v, g) = step_agent_kcc(
            &mut wedge.world,
            &mover,
            collision_filter,
            support_filter,
            position,
            velocity,
            grounded,
            desired,
            1.0 / 60.0,
        );
        let moved = (p - position).length();
        if tick < 5 || tick % 60 == 0 {
            println!(
                "t{tick}: ({:.3},{:.3},{:.3}) grounded={g} moved={moved:.4}",
                p.x, p.y, p.z
            );
        }
        position = p;
        velocity = v;
        grounded = g;
    }
    println!(
        "REST ({:.3},{:.3},{:.3}) grounded={grounded}",
        position.x, position.y, position.z
    );

    // Who is touching the capsule at rest?
    let box_pos = player::to_box_vec3(position);
    let planes = wedge
        .world
        .collide_mover(box_pos, &mover, collision_filter)
        .unwrap_or_default();
    println!("contacts at rest: {}", planes.len());
    for plane in &planes {
        println!(
            "  normal=({:.3},{:.3},{:.3}) point=({:.2},{:.2},{:.2}) <- {}",
            plane.plane.normal.x,
            plane.plane.normal.y,
            plane.plane.normal.z,
            plane.point.x,
            plane.point.y,
            plane.point.z,
            wedge.owner(plane.shape_id)
        );
    }

    // What stops the forward sweep?
    let to_target = Vec2::new(target.x - position.x, target.z - position.z);
    let step = to_target.normalize_or_zero() * AGENT_DESIRED_SPEED / 60.0;
    let delta = boxddd::Vec3::new(step.x, 0.0, step.y);
    let fraction = wedge
        .world
        .cast_mover(box_pos, &mover, delta, collision_filter)
        .unwrap_or(1.0);
    println!("forward sweep fraction={fraction:.4} (1.0 = unobstructed)");
}

/// Issue #184 investigation harness: the same env-gated replay as
/// `wedge_replay`, but with the *real* archipelago (this cell's prepared
/// nav graph) driving steering instead of a straight line, so a stall
/// can be attributed to landmass rather than the KCC.
#[test]
#[ignore = "requires a prepared cell: set BEVYOUT_WEDGE_SCENE"]
fn stall_replay() {
    let Ok(scene) = std::env::var("BEVYOUT_WEDGE_SCENE") else {
        return;
    };
    let scene = std::path::PathBuf::from(scene);
    let start = wedge_vec("BEVYOUT_WEDGE_START", Vec3::new(9.6, 106.0, -73.1));
    let target = wedge_vec("BEVYOUT_WEDGE_TARGET", Vec3::new(5.0, 106.0, -73.0));
    let graph_path = scene.parent().unwrap().join("navmesh/navgraph.ron");

    let wedge = build_wedge_world(&scene, &[]);
    let mut app = App::new();
    app.add_plugins((
        bevy::MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        bevy::transform::TransformPlugin,
        NavBackendPlugin,
    ));
    app.insert_resource(PhysicsDisabled(false));
    app.insert_resource(CellPhysicsReadiness::Ready);
    app.world_mut()
        .insert_non_send(BoxdddPhysicsContext::from_world(wedge.world));

    let graph = super::super::read_nav_graph(&graph_path).expect("nav graph");
    let mesh_inputs = super::super::mesh_inputs(&graph);
    let door_type_indices = landmass_graph::door_type_indices(&mesh_inputs);
    let closed_door_type_indices =
        landmass_graph::closed_door_type_indices(&mesh_inputs, &door_type_indices);
    let mut options = archipelago_options();
    // Issue #184 kept these two overrides: sweeping them is how the stall
    // was attributed to border avoidance in the first place (the horizon
    // sets the decay rate, the neighbourhood the border set).
    if let Ok(raw) = std::env::var("BEVYOUT_OBSTACLE_HORIZON") {
        options.obstacle_avoidance_time_horizon = raw.parse().expect("numeric horizon");
    }
    if let Ok(raw) = std::env::var("BEVYOUT_NEIGHBOURHOOD") {
        options.neighbourhood = raw.parse().expect("numeric neighbourhood");
    }
    println!(
        "obstacle_avoidance_time_horizon={} neighbourhood={}",
        options.obstacle_avoidance_time_horizon, options.neighbourhood
    );
    let archipelago_entity = app.world_mut().spawn(Archipelago3d::new(options)).id();
    apply_preferred_pathing_base_cost(
        app.world_mut(),
        archipelago_entity,
        &door_type_indices,
        &closed_door_type_indices,
    );
    for mesh in &mesh_inputs {
        let Some(valid) =
            landmass_graph::build_navigation_mesh(mesh, &[], &door_type_indices, &BTreeMap::new())
                .nav_mesh
        else {
            continue;
        };
        let handle = app
            .world_mut()
            .resource_mut::<Assets<NavMesh3d>>()
            .add(NavMesh3d {
                nav_mesh: Arc::new(valid),
            });
        app.world_mut().spawn(Island3dBundle {
            island: Island,
            archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            nav_mesh: NavMeshHandle::<ThreeD>(handle),
        });
    }
    app.world_mut()
        .resource_mut::<NavArchipelagoState>()
        .archipelago = Some(archipelago_entity);

    let centre = Vec3::new(0.0, AGENT_HEIGHT * 0.5, 0.0);
    let agent = spawn_bare_agent(
        app.world_mut(),
        archipelago_entity,
        start + centre,
        target + centre,
    );
    for tick in 0..600 {
        run_one_fixed_tick(app.world_mut());
        let world = app.world();
        let position = world.get::<Transform>(agent).unwrap().translation;
        let desired = world
            .get::<AgentDesiredVelocity3d>(agent)
            .map(|value| value.velocity())
            .unwrap_or(Vec3::ZERO);
        let state = world.get::<AgentState>(agent).copied();
        let kcc = world.get::<AgentKcc>(agent).unwrap();
        let (stuck, blocked, recovery, without) = (
            kcc.stuck,
            kcc.collision_blocked,
            kcc.recovery_active,
            kcc.ticks_without_progress,
        );
        let sampled = world
            .get::<Archipelago3d>(archipelago_entity)
            .and_then(|arch| {
                arch.sample_point(position, &AGENT_POINT_SAMPLE_DISTANCE)
                    .ok()
                    .map(|p| (p.point(), p.type_index()))
            });
        if let Some((point, type_index)) = sampled
            && tick % 20 == 0
        {
            println!(
                "    sample -> ({:.3},{:.3},{:.3}) type={type_index} dy={:.3} dxz={:.3}",
                point.x,
                point.y,
                point.z,
                position.y - point.y,
                Vec2::new(point.x - position.x, point.z - position.z).length()
            );
        }
        if tick % 20 == 0 {
            println!(
                "t{tick}: pos=({:.3},{:.3},{:.3}) desired=({:.3},{:.3},{:.3}) |d|={:.3} state={state:?} stuck={stuck} blocked={blocked} rec={recovery} nprog={without}",
                position.x,
                position.y,
                position.z,
                desired.x,
                desired.y,
                desired.z,
                desired.length()
            );
        }
    }
}
