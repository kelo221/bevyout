use super::*;
use crate::viewer::nav::agent::{
    RefreshLandmassAnimationLinkInput, SuspendedLandmassTypeIndexCosts,
};
use bevy::ecs::system::RunSystemOnce;

/// Minimal world holding one bound actor: the components the two
/// systems here read and write, and nothing else.
fn bound_actor_world() -> (World, Entity) {
    let mut world = World::new();
    world.insert_resource(Time::<()>::default());
    let entity = world
        .spawn((
            Transform::from_translation(Vec3::new(1.0, 2.0, 3.0)),
            AgentKcc::default(),
            NavBoundActor::default(),
            crate::viewer::actor_animation::ActorAnimationIntent::default(),
        ))
        .id();
    (world, entity)
}

fn advance(world: &mut World, seconds: f32) {
    let mut time = world.resource_mut::<Time>();
    time.advance_by(std::time::Duration::from_secs_f32(seconds));
}

fn requested(world: &World, entity: Entity) -> Option<ActorAnimationState> {
    world
        .get::<crate::viewer::actor_animation::ActorAnimationIntent>(entity)
        .and_then(|intent| intent.requested)
}

fn settle_locomotion(world: &mut World) {
    for _ in 0..40 {
        advance(world, 1.0 / 64.0);
        world.run_system_once(drive_bound_actor_locomotion).unwrap();
    }
}

#[test]
fn a_moving_bound_actor_requests_a_locomotion_state() {
    let (mut world, entity) = bound_actor_world();
    world
        .get_mut::<AgentKcc>(entity)
        .unwrap()
        .last_achieved_horizontal = Vec2::new(AGENT_DESIRED_SPEED, 0.0);
    settle_locomotion(&mut world);
    assert_eq!(requested(&world, entity), Some(ActorAnimationState::Run));

    world
        .get_mut::<AgentKcc>(entity)
        .unwrap()
        .last_achieved_horizontal = Vec2::new(0.8, 0.0);
    settle_locomotion(&mut world);
    assert_eq!(requested(&world, entity), Some(ActorAnimationState::Walk));
}

#[test]
fn a_stationary_bound_actor_requests_idle() {
    let (mut world, entity) = bound_actor_world();
    world.run_system_once(drive_bound_actor_locomotion).unwrap();
    assert_eq!(requested(&world, entity), Some(ActorAnimationState::Idle));
}

/// A wedged actor -- full desired velocity, zero achieved -- must not
/// stride on the spot. This is the Bevy-side twin of the pure policy's
/// own wedge test, exercised through the component the system reads.
#[test]
fn a_wedged_bound_actor_requests_idle() {
    let (mut world, entity) = bound_actor_world();
    {
        let mut kcc = world.get_mut::<AgentKcc>(entity).unwrap();
        kcc.last_desired_horizontal = Vec2::new(AGENT_DESIRED_SPEED, 0.0);
        kcc.last_achieved_horizontal = Vec2::ZERO;
    }
    world.run_system_once(drive_bound_actor_locomotion).unwrap();
    assert_eq!(requested(&world, entity), Some(ActorAnimationState::Idle));
}

#[test]
fn facing_turns_toward_the_desired_direction_at_a_bounded_rate() {
    let (mut world, entity) = bound_actor_world();
    // Desired velocity along world +Z: `bound_actor_target_yaw` (see
    // `ACTOR_MODEL_FORWARD_YAW_OFFSET`) reaches it at a yaw of +PI/2 (a
    // left turn from the identity facing) -- +X or -X would land the
    // target exactly on the +/-PI wrap boundary, an ambiguous edge case
    // this test deliberately avoids.
    world
        .get_mut::<AgentKcc>(entity)
        .unwrap()
        .last_desired_horizontal = Vec2::new(0.0, AGENT_DESIRED_SPEED);
    advance(&mut world, 0.1);
    world.run_system_once(face_bound_actors).unwrap();
    let yaw = world
        .get::<Transform>(entity)
        .unwrap()
        .rotation
        .to_euler(EulerRot::YXZ)
        .0;
    let step = FACING_TURN_RATE_RADIANS_PER_SECOND * 0.1;
    assert!(
        (yaw - step).abs() < 1e-5,
        "one bounded step toward +PI/2, got {yaw}"
    );
    let rate = world.get::<NavBoundActor>(entity).unwrap().yaw_rate;
    assert!(rate > locomotion::TURN_ENTER_RATE, "{rate}");
}

#[test]
fn a_stationary_bound_actor_holds_its_facing() {
    let (mut world, entity) = bound_actor_world();
    world
        .entity_mut(entity)
        .insert(Transform::from_rotation(Quat::from_rotation_y(0.7)));
    advance(&mut world, 0.1);
    world.run_system_once(face_bound_actors).unwrap();
    let yaw = world
        .get::<Transform>(entity)
        .unwrap()
        .rotation
        .to_euler(EulerRot::YXZ)
        .0;
    assert!((yaw - 0.7).abs() < 1e-5, "{yaw}");
    assert_eq!(world.get::<NavBoundActor>(entity).unwrap().yaw_rate, 0.0);
}

/// **The rotation double-writer contract.** Across a Stop/arrival
/// transition the AI adapter writes an authored pose yaw and claims facing
/// (`FacingAuthority::PoseAuthored`) while `AgentKcc.last_desired_horizontal`
/// is still decaying. The nav-derived writer must yield -- so exactly one
/// system sets rotation this frame and the surviving rotation is the intended
/// pose yaw, not a half-applied turn toward the stale desired velocity.
///
/// Verified to genuinely fail: dropping the `PoseAuthored` yield in
/// `face_bound_actors` makes the first assertion red (the nav writer turns
/// the actor off the authored pose).
#[test]
fn pose_authored_facing_wins_over_a_still_decaying_desired_velocity() {
    const POSE_YAW: f32 = 0.9;
    let (mut world, entity) = bound_actor_world();
    // What the AI adapter writes this transition frame: the authored pose
    // yaw plus the claim that it -- not nav -- owns facing now.
    world
        .entity_mut(entity)
        .insert(Transform::from_rotation(Quat::from_rotation_y(POSE_YAW)))
        .insert(FacingAuthority::PoseAuthored);
    // A still-decaying desired velocity the nav writer, left unchecked, would
    // turn toward (well above the facing deadband).
    world
        .get_mut::<AgentKcc>(entity)
        .unwrap()
        .last_desired_horizontal = Vec2::new(0.0, AGENT_DESIRED_SPEED);
    advance(&mut world, 0.1);
    world.run_system_once(face_bound_actors).unwrap();
    let yaw = world
        .get::<Transform>(entity)
        .unwrap()
        .rotation
        .to_euler(EulerRot::YXZ)
        .0;
    assert!(
        (yaw - POSE_YAW).abs() < 1e-6,
        "nav writer overrode the authored pose yaw: got {yaw}, expected {POSE_YAW}"
    );
    assert_eq!(
        world.get::<NavBoundActor>(entity).unwrap().yaw_rate,
        0.0,
        "the yielding writer must report no achieved turn"
    );

    // Control -- guard the guard: with facing handed back to navigation, the
    // *same* decaying velocity DOES turn the actor, proving the yield above is
    // the flag's doing and not a dead input.
    world.entity_mut(entity).insert(FacingAuthority::NavDerived);
    world.run_system_once(face_bound_actors).unwrap();
    let turned = world
        .get::<Transform>(entity)
        .unwrap()
        .rotation
        .to_euler(EulerRot::YXZ)
        .0;
    assert!(
        (turned - POSE_YAW).abs() > 1e-4,
        "nav-derived facing should turn toward the desired velocity, got {turned}"
    );
}

/// A bound actor pivoting on the spot at the route start selects a turn
/// clip, which is what makes `turn_left`/`turn_right` reachable at all.
///
/// Desired velocity along world +Z: at the entity's starting identity
/// rotation, the actor's true forward (`ACTOR_MODEL_FORWARD_YAW_OFFSET`)
/// already points world -X, so this needs a genuine turn (unlike world
/// -X, which the true forward already faces with zero rotation, and
/// unlike world +X, which lands exactly on the +/-PI wrap boundary).
#[test]
fn a_bound_actor_pivoting_in_place_requests_a_turn_clip() {
    let (mut world, entity) = bound_actor_world();
    world
        .get_mut::<AgentKcc>(entity)
        .unwrap()
        .last_desired_horizontal = Vec2::new(0.0, AGENT_DESIRED_SPEED);
    // A single facing sample is intentionally filtered by the yaw EMA. A
    // sustained pivot crosses the turn-enter band after a few fixed ticks.
    for _ in 0..8 {
        advance(&mut world, 1.0 / 64.0);
        world.run_system_once(face_bound_actors).unwrap();
        world.run_system_once(drive_bound_actor_locomotion).unwrap();
    }
    assert_eq!(
        requested(&world, entity),
        Some(ActorAnimationState::TurnLeft)
    );
}

/// Regression for the live T-pose: a holding actor whose desired direction
/// flips every tick receives alternating full-rate turns from the facing
/// system. The animation request must settle to idle instead of restarting
/// `turn_left`/`turn_right` one-shots indefinitely.
#[test]
fn a_holding_actor_with_a_jittery_direction_settles_to_idle() {
    let (mut world, entity) = bound_actor_world();
    let mut turn_requests_after_warmup = 0;
    for tick in 0..128 {
        let direction = if tick % 2 == 0 {
            Vec2::new(0.0, AGENT_DESIRED_SPEED)
        } else {
            Vec2::new(0.0, -AGENT_DESIRED_SPEED)
        };
        world
            .get_mut::<AgentKcc>(entity)
            .unwrap()
            .last_desired_horizontal = direction;
        advance(&mut world, 1.0 / 64.0);
        world.run_system_once(face_bound_actors).unwrap();
        world.run_system_once(drive_bound_actor_locomotion).unwrap();
        if tick >= 40
            && matches!(
                requested(&world, entity),
                Some(ActorAnimationState::TurnLeft | ActorAnimationState::TurnRight)
            )
        {
            turn_requests_after_warmup += 1;
        }
    }
    assert_eq!(
        turn_requests_after_warmup, 0,
        "a holding actor kept requesting one-shot turn clips"
    );
    assert_eq!(requested(&world, entity), Some(ActorAnimationState::Idle));
}

/// **The one-authority invariant** (issue #188 feature 4, verdict
/// §2.3). Clip playback moves skeleton entities under the actor root;
/// it must never move the root itself, because the KCC owns that. Runs
/// the whole nav-side locomotion chain over a fixed tick count twice --
/// once with an emulated animation pass writing large translations into
/// the actor's bone hierarchy every tick, once with no clip at all --
/// and requires the root translation sequences to be *bit*-identical.
///
/// Verified to genuinely fail: wiring the bone's translation into the
/// root inside `face_bound_actors` makes this test red.
#[test]
fn agent_transform_is_bit_identical_with_and_without_clip_playback() {
    const TICKS: u32 = 120;

    fn run(with_clip_playback: bool) -> Vec<[u32; 3]> {
        let (mut world, entity) = bound_actor_world();
        {
            let mut kcc = world.get_mut::<AgentKcc>(entity).unwrap();
            kcc.last_desired_horizontal = Vec2::new(1.7, 1.1);
            kcc.last_achieved_horizontal = Vec2::new(2.02, 0.0);
        }
        // A skeleton bone under the actor root, standing in for an
        // animated accumulation root.
        let bone = world.spawn(Transform::IDENTITY).id();
        world.entity_mut(entity).add_child(bone);

        let mut samples = Vec::new();
        for tick in 0..TICKS {
            advance(&mut world, 1.0 / 64.0);
            if with_clip_playback {
                // What a root-motion clip does: displace the bone by a
                // large, ever-growing amount. If any of this module's
                // code let that reach the root, the sequences diverge.
                let drift = (tick % 251) as f32 * 0.37;
                world.get_mut::<Transform>(bone).unwrap().translation =
                    Vec3::new(drift, drift * 2.0, -drift);
            }
            world.run_system_once(face_bound_actors).unwrap();
            world.run_system_once(drive_bound_actor_locomotion).unwrap();
            let translation = world.get::<Transform>(entity).unwrap().translation;
            samples.push([
                translation.x.to_bits(),
                translation.y.to_bits(),
                translation.z.to_bits(),
            ]);
        }
        samples
    }

    let without = run(false);
    let with = run(true);
    assert_eq!(without.len(), TICKS as usize);
    assert_eq!(
        with, without,
        "clip playback changed the agent transform: animation has become a second movement authority"
    );
    // Guard the guard: a sequence of all-identical samples would make
    // the comparison above vacuous. The KCC is absent from this world,
    // so the root must be *stationary* -- assert that positively rather
    // than assuming it.
    assert!(
        without.iter().all(|sample| *sample == without[0]),
        "nothing in this module may move the root at all"
    );
}

/// The `tna` debug capsule (no [`NavBoundActor`]) is invisible to both
/// systems: it keeps the exact behaviour every nav wave has relied on.
#[test]
fn an_unbound_tna_capsule_is_untouched_by_the_binding_systems() {
    let mut world = World::new();
    world.insert_resource(Time::<()>::default());
    let capsule = world
        .spawn((
            Transform::from_translation(Vec3::new(4.0, 5.0, 6.0)),
            AgentKcc {
                last_desired_horizontal: Vec2::new(AGENT_DESIRED_SPEED, 0.0),
                last_achieved_horizontal: Vec2::new(AGENT_DESIRED_SPEED, 0.0),
                ..default()
            },
            crate::viewer::actor_animation::ActorAnimationIntent::default(),
        ))
        .id();
    advance(&mut world, 0.5);
    world.run_system_once(face_bound_actors).unwrap();
    world.run_system_once(drive_bound_actor_locomotion).unwrap();
    let transform = *world.get::<Transform>(capsule).unwrap();
    assert_eq!(transform.translation, Vec3::new(4.0, 5.0, 6.0));
    assert_eq!(transform.rotation, Quat::IDENTITY);
    assert_eq!(requested(&world, capsule), None);
}

#[test]
fn releasing_a_bound_actor_keeps_the_actor_and_asks_for_idle() {
    let (mut world, entity) = bound_actor_world();
    world.get_mut::<NavBoundActor>(entity).unwrap().locomotion = LocomotionState::Run;
    world.entity_mut(entity).insert((
        RefreshLandmassAnimationLinkInput,
        SuspendedLandmassTypeIndexCosts(None),
    ));
    release_bound_actor(&mut world, entity);
    assert!(world.get_entity(entity).is_ok(), "the actor must survive");
    assert!(world.get::<NavBoundActor>(entity).is_none());
    assert!(world.get::<AgentKcc>(entity).is_none());
    assert!(
        world
            .get::<RefreshLandmassAnimationLinkInput>(entity)
            .is_none()
    );
    assert!(
        world
            .get::<SuspendedLandmassTypeIndexCosts>(entity)
            .is_none()
    );
    assert_eq!(requested(&world, entity), Some(ActorAnimationState::Idle));
}

/// The unit test issue #208 asks for: assert the actor mesh's *true*
/// forward axis (Bevy's -X, per [`ACTOR_MODEL_FORWARD_YAW_OFFSET`]'s
/// derivation from the shipped skeleton NIF) lands on the desired travel
/// direction after [`bound_actor_target_yaw`] -- not the -Z a bare
/// `Capsule3d` has, which is exactly the 90-degree-left bug this fixes.
/// Pure glam math: no `World`, no Bevy systems.
#[test]
fn bound_actor_target_yaw_points_the_true_mesh_forward_along_desired_travel() {
    const TRUE_LOCAL_FORWARD: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
    let cases: [(&str, Vec2); 5] = [
        ("-Z", Vec2::new(0.0, -1.0)),
        ("+Z", Vec2::new(0.0, 1.0)),
        ("+X", Vec2::new(1.0, 0.0)),
        ("-X", Vec2::new(-1.0, 0.0)),
        ("diagonal +X+Z", Vec2::new(1.0, 1.0).normalize()),
    ];
    for (label, desired) in cases {
        let yaw = bound_actor_target_yaw(desired);
        let actual_forward = Quat::from_rotation_y(yaw) * TRUE_LOCAL_FORWARD;
        let expected_forward = Vec3::new(desired.x, 0.0, desired.y).normalize();
        assert!(
            actual_forward.abs_diff_eq(expected_forward, 1e-5),
            "{label}: expected true forward {expected_forward:?}, got {actual_forward:?} (yaw {yaw})"
        );
        // The bug this fixes, stated as a negative assertion: the *naive*
        // -Z-forward assumption must NOT land on the desired direction --
        // it is a fixed 90 degrees off, which is what made the mesh
        // visually face 90 degrees away from its travel direction.
        let naive_forward = Quat::from_rotation_y(yaw) * Vec3::NEG_Z;
        assert!(
            !naive_forward.abs_diff_eq(expected_forward, 1e-3),
            "{label}: naive -Z forward should NOT match desired (that was the bug)"
        );
    }
}

#[test]
fn shortest_yaw_delta_takes_the_short_way_round() {
    let tau = std::f32::consts::TAU;
    assert!((shortest_yaw_delta(0.1) - 0.1).abs() < 1e-6);
    assert!((shortest_yaw_delta(-0.1) + 0.1).abs() < 1e-6);
    assert!((shortest_yaw_delta(tau - 0.1) + 0.1).abs() < 1e-5);
    assert!((shortest_yaw_delta(tau + 0.1) - 0.1).abs() < 1e-5);
}
