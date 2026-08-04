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

#[test]
fn physics_movement_zeroes_velocity_while_cell_physics_is_building() {
    use bevy::ecs::system::RunSystemOnce;

    let mut world = World::new();
    world.init_resource::<DebugAgentRoster>();
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
            NavAgent,
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
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);

    world
        .run_system_once(apply_agent_physics_movement)
        .expect("system runs");

    let kcc = world.get::<AgentKcc>(agent).unwrap();
    assert_eq!(kcc.velocity, Vec3::ZERO);
    assert!(!kcc.grounded);
    assert_eq!(world.get::<Velocity3d>(agent).unwrap().velocity, Vec3::ZERO);
}

#[test]
fn stuck_detection_does_not_false_trigger_against_a_vertically_offset_target() {
    use bevy::ecs::system::RunSystemOnce;

    let mut world = World::new();
    world.init_resource::<DebugAgentRoster>();
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
            NavAgent,
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
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);

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

#[test]
fn desired_velocity_blends_between_solves_and_is_exact_at_interval_one() {
    use bevy::ecs::system::RunSystemOnce;

    fn blend_test_world(blend: AgentDesiredVelocityBlend) -> (World, Entity) {
        let mut world = World::new();
        world.init_resource::<DebugAgentRoster>();
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
                NavAgent,
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
