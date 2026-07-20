//! Binds a projected actor ([`ActorRuntime`]) to a nav agent (issue #188),
//! so one entity carries routing, KCC movement, skeleton and animation --
//! the seam the navigation feature (#111-#184) and the actor-animation
//! feature (#104/#106) previously did not meet across.
//!
//! Included into `nav/agent.rs` as a `#[path]` submodule (the same shape
//! `fall_guard` uses) so it can reach that module's private agent
//! components and constants through `super::` without growing its already
//! oversized module root (post-mortem verdict §2.6).
//!
//! # One movement authority
//!
//! The KCC in `super::apply_agent_physics_movement` is the *only* thing in
//! this codebase that may write a nav agent's `Transform.translation`.
//! Nothing here writes translation at all: [`face_bound_actors`] writes
//! `Transform.rotation` only, derived from the desired velocity navigation
//! already produced, and [`drive_bound_actor_locomotion`] writes nothing but
//! an animation request. Animation is display-only in both directions --
//! clip playback never feeds back into the agent transform, which is the
//! shape verdict §2.3 records `landmass`'s border ORCA taking for the
//! feature's entire life. `agent_transform_is_bit_identical_with_and_without_clip_playback`
//! below is the test that fails if that ever changes.
//!
//! The `tna` debug-capsule path is untouched: every system here is filtered
//! on [`NavBoundActor`], which only a `tna bind` ever inserts.

use bevy::prelude::*;

use super::locomotion::{self, LocomotionObservation, LocomotionState};
use super::{AGENT_DESIRED_SPEED, AGENT_HEIGHT, AgentKcc};
use crate::viewer::actor::ActorRuntime;
use crate::viewer::actor_animation::policy::ActorAnimationState;
use crate::viewer::actor_animation::request_actor_animation;

const _: () = {
    // `locomotion.rs` is std-only and cannot import `AGENT_DESIRED_SPEED`,
    // so it restates it. Fail the build rather than let the duplicate drift:
    // every locomotion threshold is a fraction of this number.
    assert!(locomotion::ROUTE_SPEED_METRES_PER_SECOND == AGENT_DESIRED_SPEED);
};

/// How fast (rad/s) a bound actor swings its facing toward the direction
/// navigation is steering it: 180 deg/s, i.e. a full about-face in one
/// second. Fast enough that an actor handed a route behind it starts walking
/// forwards almost immediately rather than moonwalking down the corridor,
/// slow enough that the pivot is a visible turn -- which is exactly what
/// makes `locomotion::TURN_ENTER_RATE` (a quarter of this) reachable, and
/// therefore the turn clips selectable.
const FACING_TURN_RATE_RADIANS_PER_SECOND: f32 = std::f32::consts::PI;

/// Below this desired horizontal speed (m/s) a bound actor holds its current
/// facing instead of chasing a direction. The same `WALK_EXIT_SPEED` floor
/// the locomotion policy calls "not moving": a near-zero desired velocity
/// has an essentially random direction, and turning toward it would make a
/// standing actor jitter.
const FACING_DEADBAND_SPEED: f32 = locomotion::WALK_EXIT_SPEED;

/// Marks an [`ActorRuntime`] entity that owns a nav agent. Insertion (by
/// `super::bind_actor_agent`) is what turns an ordinary projected actor into
/// a routed one; removal releases it back to `actor_animation`'s own
/// unbound-actor behaviour. Carries only the locomotion state machine's
/// memory, because hysteresis needs the previous verdict and nothing else.
#[derive(Component, Debug, Clone, Copy, Default)]
pub(super) struct NavBoundActor {
    /// The locomotion state last requested, i.e. the `previous` argument
    /// `locomotion::next_locomotion_state` needs for its hysteresis bands.
    pub(super) locomotion: LocomotionState,
    /// Signed yaw rate (rad/s) [`face_bound_actors`] actually applied this
    /// tick -- the *achieved* turn, matching the achieved-not-desired rule
    /// the speed input follows.
    pub(super) yaw_rate: f32,
}

/// The Y offset from a bound actor's placement root (feet level, where
/// preparation puts an actor reference) to the KCC capsule *centre*, which
/// is what `apply_agent_physics_movement` positions. `tna` capsules keep an
/// offset of zero because their entity transform already *is* the capsule
/// centre; without this, a bound actor would be sunk half a capsule into the
/// floor.
pub(super) const BOUND_ACTOR_CAPSULE_OFFSET_Y: f32 = AGENT_HEIGHT / 2.0;

/// Rotates each bound actor toward the direction navigation is steering it,
/// at a bounded rate, and records the yaw rate it actually achieved.
///
/// Facing is nav-derived, never animation-derived: the input is the desired
/// horizontal velocity `apply_agent_physics_movement` already blended and
/// stashed on [`AgentKcc`] this tick. Writing `Transform.rotation` here is
/// not a second movement authority -- translation, the thing the KCC owns,
/// is never touched (see the module doc comment).
pub(super) fn face_bound_actors(
    time: Res<Time>,
    mut actors: Query<(&mut Transform, &AgentKcc, &mut NavBoundActor)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    for (mut transform, kcc, mut bound) in &mut actors {
        let desired = kcc.last_desired_horizontal;
        if desired.length() < FACING_DEADBAND_SPEED {
            bound.yaw_rate = 0.0;
            continue;
        }
        // Bevy's forward is -Z, and a yaw rotation about +Y maps it to
        // (-sin(yaw), 0, -cos(yaw)); inverting that for the desired (x, z)
        // direction gives the target yaw.
        let target_yaw = (-desired.x).atan2(-desired.y);
        let current_yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
        let error = shortest_yaw_delta(target_yaw - current_yaw);
        let step = error.clamp(
            -FACING_TURN_RATE_RADIANS_PER_SECOND * dt,
            FACING_TURN_RATE_RADIANS_PER_SECOND * dt,
        );
        // Replaces the whole rotation rather than composing onto it: a
        // walking actor is upright by definition, and the prepared
        // placement's authored pitch/roll (if any) is a standing pose that
        // has no meaning once the actor is routed.
        transform.rotation = Quat::from_rotation_y(current_yaw + step);
        bound.yaw_rate = step / dt;
    }
}

/// Wraps a yaw difference into `(-PI, PI]` so a facing correction always
/// takes the short way round.
fn shortest_yaw_delta(delta: f32) -> f32 {
    let wrapped = delta.rem_euclid(std::f32::consts::TAU);
    if wrapped > std::f32::consts::PI {
        wrapped - std::f32::consts::TAU
    } else {
        wrapped
    }
}

/// Feeds each bound actor's already-computed achieved motion into the pure
/// [`locomotion`] policy and hands the verdict to the existing gameplay
/// animation API.
///
/// This is deliberately a thin consumer: it recomputes nothing. The
/// horizontal speed is the `achieved` half of the very pair
/// `apply_agent_physics_movement` samples for
/// `movement_policy::decide_collision_outcome`, and the yaw rate is what
/// [`face_bound_actors`] actually applied.
pub(super) fn drive_bound_actor_locomotion(world: &mut World) {
    let mut query = world.query_filtered::<(Entity, &AgentKcc, &mut NavBoundActor), ()>();
    let requests: Vec<(Entity, LocomotionState, LocomotionState)> = query
        .iter_mut(world)
        .map(|(entity, kcc, mut bound)| {
            let previous = bound.locomotion;
            let next = locomotion::next_locomotion_state(
                previous,
                LocomotionObservation {
                    achieved_horizontal_speed: kcc.last_achieved_horizontal_speed,
                    yaw_rate: bound.yaw_rate,
                },
            );
            bound.locomotion = next;
            (entity, previous, next)
        })
        .collect();
    for (entity, previous, next) in requests {
        if previous != next {
            info!(
                "nav actor locomotion {} {} -> {}",
                entity,
                previous.label(),
                next.label()
            );
        }
        // Requested every tick, not only on change: `actor_animation`
        // clears `ActorAnimationIntent.requested` when a one-shot (a turn)
        // finishes, and a continuing turn must be re-requested to replay.
        // `select_gameplay_animation` already short-circuits an unchanged
        // request, so this is idempotent.
        let _ = request_actor_animation(world, entity, animation_state(next));
    }
}

/// Total mapping from the locomotion policy's verdict onto the animation
/// module's state enum. The two enums are deliberately separate (see
/// [`LocomotionState`]); this is the only place they meet.
const fn animation_state(state: LocomotionState) -> ActorAnimationState {
    match state {
        LocomotionState::Idle => ActorAnimationState::Idle,
        LocomotionState::Walk => ActorAnimationState::Walk,
        LocomotionState::Run => ActorAnimationState::Run,
        LocomotionState::TurnLeft => ActorAnimationState::TurnLeft,
        LocomotionState::TurnRight => ActorAnimationState::TurnRight,
    }
}

/// Resolves the live [`ActorRuntime`] entity whose reference FormID is
/// `reference_form_id`.
pub(super) fn actor_entity_by_reference(
    world: &mut World,
    reference_form_id: u32,
) -> Option<Entity> {
    let mut query = world.query::<(Entity, &ActorRuntime)>();
    query
        .iter(world)
        .find(|(_, actor)| actor.reference_form_id == reference_form_id)
        .map(|(entity, _)| entity)
}

/// Releases a bound actor back to unbound behaviour: strips the nav agent
/// component set and asks for an idle clip, leaving the actor entity itself
/// (its skeleton, equipment and holder projection) completely intact.
///
/// This is what the #164 fall guard and `tna despawn` call for a bound
/// actor. Despawning is not an option there: a nav guard must not delete a
/// projected NPC, which the world/actor slice owns the lifetime of.
pub(super) fn release_bound_actor(world: &mut World, entity: Entity) {
    let Ok(mut entity_mut) = world.get_entity_mut(entity) else {
        return;
    };
    entity_mut.remove::<NavBoundActor>();
    super::remove_agent_components(&mut entity_mut);
    let _ = request_actor_animation(world, entity, ActorAnimationState::Idle);
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn a_moving_bound_actor_requests_a_locomotion_state() {
        let (mut world, entity) = bound_actor_world();
        world
            .get_mut::<AgentKcc>(entity)
            .unwrap()
            .last_achieved_horizontal_speed = AGENT_DESIRED_SPEED;
        world.run_system_once(drive_bound_actor_locomotion).unwrap();
        assert_eq!(requested(&world, entity), Some(ActorAnimationState::Run));

        world
            .get_mut::<AgentKcc>(entity)
            .unwrap()
            .last_achieved_horizontal_speed = 0.8;
        world.run_system_once(drive_bound_actor_locomotion).unwrap();
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
            kcc.last_achieved_horizontal_speed = 0.0;
        }
        world.run_system_once(drive_bound_actor_locomotion).unwrap();
        assert_eq!(requested(&world, entity), Some(ActorAnimationState::Idle));
    }

    #[test]
    fn facing_turns_toward_the_desired_direction_at_a_bounded_rate() {
        let (mut world, entity) = bound_actor_world();
        // Desired velocity along +X: Bevy's -Z forward reaches it at a yaw
        // of -PI/2 (a right turn from the identity facing).
        world
            .get_mut::<AgentKcc>(entity)
            .unwrap()
            .last_desired_horizontal = Vec2::new(AGENT_DESIRED_SPEED, 0.0);
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
            (yaw + step).abs() < 1e-5,
            "one bounded step toward -PI/2, got {yaw}"
        );
        let rate = world.get::<NavBoundActor>(entity).unwrap().yaw_rate;
        assert!(rate < -locomotion::TURN_ENTER_RATE, "{rate}");
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

    /// A bound actor pivoting on the spot at the route start selects a turn
    /// clip, which is what makes `turn_left`/`turn_right` reachable at all.
    #[test]
    fn a_bound_actor_pivoting_in_place_requests_a_turn_clip() {
        let (mut world, entity) = bound_actor_world();
        world
            .get_mut::<AgentKcc>(entity)
            .unwrap()
            .last_desired_horizontal = Vec2::new(-AGENT_DESIRED_SPEED, 0.0);
        advance(&mut world, 1.0 / 64.0);
        world.run_system_once(face_bound_actors).unwrap();
        world.run_system_once(drive_bound_actor_locomotion).unwrap();
        assert_eq!(
            requested(&world, entity),
            Some(ActorAnimationState::TurnLeft)
        );
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
                kcc.last_achieved_horizontal_speed = 2.02;
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
                    last_achieved_horizontal_speed: AGENT_DESIRED_SPEED,
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
        release_bound_actor(&mut world, entity);
        assert!(world.get_entity(entity).is_ok(), "the actor must survive");
        assert!(world.get::<NavBoundActor>(entity).is_none());
        assert!(world.get::<AgentKcc>(entity).is_none());
        assert_eq!(requested(&world, entity), Some(ActorAnimationState::Idle));
    }

    #[test]
    fn shortest_yaw_delta_takes_the_short_way_round() {
        let tau = std::f32::consts::TAU;
        assert!((shortest_yaw_delta(0.1) - 0.1).abs() < 1e-6);
        assert!((shortest_yaw_delta(-0.1) + 0.1).abs() < 1e-6);
        assert!((shortest_yaw_delta(tau - 0.1) + 0.1).abs() < 1e-5);
        assert!((shortest_yaw_delta(tau + 0.1) - 0.1).abs() < 1e-5);
    }
}
