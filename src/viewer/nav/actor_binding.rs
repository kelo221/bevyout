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

/// The fixed yaw correction between the travel-direction formula in
/// [`bound_actor_target_yaw`] (which assumes Bevy's default local forward,
/// -Z, the axis a bare `Capsule3d` -- the untouched `tna spawn` debug path
/// -- actually has) and a converted Fallout 3 actor mesh's true forward
/// axis, which is not -Z (issue #208).
///
/// Every bound-actor mesh comes from `convert_actor_scene`
/// (`src/vsa/nif_convert/mod.rs`) assembled onto the shared FO3 humanoid rig
/// (`Characters/_Male/Skeleton.NIF`, referenced by every humanoid actor
/// catalog entry's `model_path`). Parsing that shipped skeleton NIF
/// directly (`nif::fo3::extract_scene`) shows its root ("Scene Root", block
/// 0) at an identity transform, but that root's only child -- "Bip01", the
/// bone every other bone and every skinned body mesh hangs off -- carries a
/// baked local rotation of
/// `[[~0, -1, 0], [1, ~0, 0], [0, 0, 1]]`: a pure yaw (the invariant third
/// row/column is the NIF up axis, Z) of +90 degrees about that axis, with
/// no pitch or roll. `M * (0, 1, 0)` (NIF's own forward axis) comes out to
/// `(-1, 0, 0)`, i.e. NIF -X, not NIF +Y: this is the "authored yaw offset"
/// the alternative approach in the issue brief describes, and it is a
/// skeleton-rig constant rather than something that needs capturing per
/// actor at bind time, because every bound actor shares this one rig.
///
/// The global NIF-to-glTF coordinate conversion every converted asset gets
/// (`nifty`'s `fo3::glb::encode_glb`, "Z-up to glTF Y-up", the quarter-turn
/// about X wrapping the whole scene) contributes no yaw of its own: applying
/// its rotation to NIF's forward axis (0, 1, 0) yields Bevy's (0, 0, -1)
/// exactly. So composing the two: a bound actor's true local forward, in
/// Bevy space, is Bevy's -X -- a fixed quarter turn from the -Z a bare
/// primitive has.
///
/// Concretely: a mesh whose true local forward is -X, held at
/// `Transform::IDENTITY`, visually faces -X, not -Z. The travel-direction
/// formula in [`bound_actor_target_yaw`] computes the yaw that points *-Z*
/// at the desired direction; to point the *actual* -X forward there
/// instead, the applied yaw must be a quarter turn short of that -- hence
/// subtracting, not adding, a quarter turn here. This reproduces the
/// reported bug exactly when omitted: for a bound actor walking toward
/// world +X, the naive (no-offset) formula sets yaw so that -Z points at
/// +X, but the mesh's real -X-forward then visually points at -Z instead --
/// which, under this module's own left/right convention (`right =
/// cross(forward, up)`, so facing +X puts +Z on the right and -Z on the
/// left), is exactly "90 degrees to the left of the travel direction".
const ACTOR_MODEL_FORWARD_YAW_OFFSET: f32 = -std::f32::consts::FRAC_PI_2;

/// The yaw that turns a bound actor's true mesh forward (see
/// [`ACTOR_MODEL_FORWARD_YAW_OFFSET`]) toward `desired`, a horizontal
/// direction in the same `(x, z)` convention
/// `AgentKcc::last_desired_horizontal` uses. Pure glam math, extracted so it
/// can be unit-tested without a `World`.
fn bound_actor_target_yaw(desired: Vec2) -> f32 {
    // Bevy's forward is -Z, and a yaw rotation about +Y maps it to
    // (-sin(yaw), 0, -cos(yaw)); inverting that for the desired (x, z)
    // direction gives the yaw that would point -Z (not the actor's real
    // forward) at `desired`.
    let neg_z_forward_yaw = (-desired.x).atan2(-desired.y);
    neg_z_forward_yaw + ACTOR_MODEL_FORWARD_YAW_OFFSET
}

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

/// Which authority owns a bound actor's `Transform.rotation` this frame -- the
/// explicit signal that resolves the rotation double-writer race. The
/// nav-derived facing writer ([`face_bound_actors`]) yields whenever this is
/// [`Self::PoseAuthored`], so an AI family holding an authored idle-marker yaw
/// is the only system that writes rotation that frame; otherwise nav steers
/// facing. Absent (the common bound actor before any package, and every `tna`
/// capsule) reads as [`Self::NavDerived`]. The AI adapter sets it through
/// `super::set_facing_authority`; nav reads only this component -- across a
/// Stop/arrival transition (while `AgentKcc.last_desired_horizontal` is still
/// decaying) exactly one writer sets rotation, with pose taking precedence.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) enum FacingAuthority {
    #[default]
    NavDerived,
    PoseAuthored,
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
    mut actors: Query<(
        &mut Transform,
        &AgentKcc,
        &mut NavBoundActor,
        Option<&FacingAuthority>,
    )>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    for (mut transform, kcc, mut bound, authority) in &mut actors {
        // Rotation double-writer contract: when an AI family owns facing (it is
        // holding an authored pose yaw), the nav-derived writer yields so that
        // family's write is the only rotation this frame -- no race with a
        // still-decaying desired velocity across a Stop/arrival transition.
        if authority.copied().unwrap_or_default() == FacingAuthority::PoseAuthored {
            bound.yaw_rate = 0.0;
            continue;
        }
        let desired = kcc.last_desired_horizontal;
        if desired.length() < FACING_DEADBAND_SPEED {
            bound.yaw_rate = 0.0;
            continue;
        }
        let target_yaw = bound_actor_target_yaw(desired);
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
    // Run every registered teardown hook (e.g. the AI slice tears down a
    // running package controller and frees any interaction point it claimed),
    // so a released actor is no longer ticked by an AI system on an agent it no
    // longer has. Nav owns the contract; it never names the AI types.
    super::run_bound_actor_release_hooks(world, entity);
}

#[cfg(test)]
#[path = "tests/actor_binding.rs"]
mod tests;
