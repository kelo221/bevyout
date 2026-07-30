//! Runtime actor perception and target awareness (issue #116).
//!
//! Thin Bevy adapter over the pure `bevyout_core::perception` policy: every
//! decision (visibility, confidence, acquire/loss hysteresis, target ordering)
//! lives in the core state machine; this module only computes per-tick
//! geometry from transforms, resolves line of sight against the physics world,
//! and stores the resulting [`AwarenessState`] on the actor entity.
//!
//! [`ActorAwareness`] is the *single authoritative* target-awareness state.
//! AI package selection (#193) and the future M5 combat boundary read it;
//! neither keeps a second, competing target authority. This module owns
//! awareness/relationship state only -- no attack, damage, or death.

use bevy::prelude::*;
use bevy_boxddd::boxddd::{self, QueryFilter};
use bevy_boxddd::prelude::BoxdddPhysicsContext;
use bevyout_core::actor_state::ActorLifeState;
use bevyout_core::perception::{
    AwarenessEvent, AwarenessState, PerceptionConfig, PerceptionInputs, TargetId,
};

use super::actor::ActorRuntime;
use super::actor_state::ActorStateRuntime;
use super::interaction::PlacementRoot;
use super::player::{CAPSULE_HEIGHT, FpsPlayer};

/// Eye height above an actor's feet-pivot root, metres. Perception rays and the
/// view cone originate here.
const ACTOR_EYE_HEIGHT: f32 = 1.6;
/// Margin subtracted from the actor->target ray so the target's own capsule is
/// never counted as an occluder.
const LOS_TARGET_MARGIN: f32 = 0.6;

pub(crate) struct PerceptionPlugin;

impl Plugin for PerceptionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PerceptionSettings>()
            .init_resource::<PerceptionSummary>()
            .add_systems(
                Update,
                (
                    attach_awareness,
                    update_actor_awareness,
                    summarize_awareness,
                )
                    .chain()
                    .in_set(super::plugins::ViewerSet::WorldSync),
            );
    }
}

/// The tunable perception thresholds, wrapping the pure config as a resource.
#[derive(Resource, Debug, Clone, Default)]
pub(crate) struct PerceptionSettings(pub(crate) PerceptionConfig);

/// The single authoritative awareness state for one actor, plus the last
/// player-directed perception geometry so console/AI consumers can report it
/// without recomputing.
#[derive(Component, Debug, Clone, Default)]
pub(crate) struct ActorAwareness {
    pub(crate) state: AwarenessState,
    pub(crate) last_player: Option<PerceptionInputs>,
}

/// A read-only consumer projection proving the awareness authority feeds
/// downstream systems: the set of actor references currently aware of the
/// player. AI package selection and combat consume the same authority.
#[derive(Resource, Debug, Clone, Default)]
pub(crate) struct PerceptionSummary {
    pub(crate) aware_of_player: std::collections::BTreeSet<u32>,
}

/// Pure geometry for the player candidate as seen by one actor. Kept separate
/// from the physics-coupled system so it is unit-testable without a world.
#[must_use]
pub(crate) fn player_perception_inputs(
    actor_eye: Vec3,
    actor_forward: Vec3,
    player_eye: Vec3,
    has_line_of_sight: bool,
    player_detectable: bool,
) -> PerceptionInputs {
    let to_player = player_eye - actor_eye;
    let distance = to_player.length();
    let angle_to_target = if distance > f32::EPSILON {
        actor_forward
            .normalize_or_zero()
            .angle_between(to_player / distance)
    } else {
        0.0
    };
    PerceptionInputs {
        target: TargetId::player(),
        position: player_eye.to_array(),
        distance,
        angle_to_target,
        has_line_of_sight,
        detectable: player_detectable,
    }
}

/// Best-effort line-of-sight test: casts from `origin` toward `target` and
/// reports blocked when anything is hit before the target margin. When the
/// physics world is unavailable (physics disabled or a minimal test app), it
/// conservatively reports clear sight.
fn line_of_sight(context: &BoxdddPhysicsContext, origin: Vec3, target: Vec3) -> bool {
    let Some(world) = context.world() else {
        return true;
    };
    let delta = target - origin;
    let distance = delta.length();
    if distance <= LOS_TARGET_MARGIN {
        return true;
    }
    let direction = delta / distance;
    let length = distance - LOS_TARGET_MARGIN;
    let ray_origin = boxddd::Vec3::new(origin.x, origin.y, origin.z);
    let translation = boxddd::Vec3::new(
        direction.x * length,
        direction.y * length,
        direction.z * length,
    );
    // Mirror the drop/surface probes: hit static (1) and dynamic (2) geometry,
    // which includes closed doors as real blockers.
    let filter = QueryFilter::new().category_bits(4).mask_bits(1 | 2);
    world
        .cast_ray(ray_origin, translation, filter)
        .map_or(true, |hits| hits.is_empty())
}

#[allow(clippy::type_complexity)]
fn update_actor_awareness(
    time: Res<Time>,
    settings: Res<PerceptionSettings>,
    context: NonSend<BoxdddPhysicsContext>,
    players: Query<&GlobalTransform, With<FpsPlayer>>,
    mut actors: Query<(
        &ActorRuntime,
        &ActorStateRuntime,
        &GlobalTransform,
        &mut ActorAwareness,
    )>,
) {
    let dt = time.delta_secs();
    let player_eye = players
        .iter()
        .next()
        .map(|transform| transform.translation() + Vec3::Y * (CAPSULE_HEIGHT * 0.5));

    for (runtime, state_runtime, transform, mut awareness) in &mut actors {
        // A dead actor perceives nothing; drop any acquired target.
        if state_runtime.life_state == ActorLifeState::Dead {
            if awareness.state.is_aware() {
                info!(
                    "perception loss {:08x} target=player reason=observer_dead",
                    runtime.reference_form_id
                );
            }
            awareness.state.clear();
            awareness.last_player = None;
            continue;
        }

        let Some(player_eye) = player_eye else {
            // No player in the world: the target has effectively disappeared.
            let event = awareness.state.update(&[], &settings.0, dt);
            log_awareness_event(runtime.reference_form_id, event);
            awareness.last_player = None;
            continue;
        };

        let actor_eye = transform.translation() + Vec3::Y * ACTOR_EYE_HEIGHT;
        let has_los = line_of_sight(&context, actor_eye, player_eye);
        let inputs =
            player_perception_inputs(actor_eye, *transform.forward(), player_eye, has_los, true);
        let event = awareness.state.update(&[inputs], &settings.0, dt);
        awareness.last_player = Some(inputs);
        log_awareness_event(runtime.reference_form_id, event);
    }
}

fn log_awareness_event(reference_form_id: u32, event: AwarenessEvent) {
    match event {
        AwarenessEvent::Acquired(target) => info!(
            "perception acquire {reference_form_id:08x} target={}:{:08x}",
            target.class.label(),
            target.form_id
        ),
        AwarenessEvent::Lost(target) => info!(
            "perception loss {reference_form_id:08x} target={}:{:08x}",
            target.class.label(),
            target.form_id
        ),
        AwarenessEvent::Idle | AwarenessEvent::Retained(_) => {}
    }
}

/// Consumer: projects the authoritative awareness of every live actor into the
/// shared summary resource. Only inserts references still aware this frame, so
/// a loss removes the actor.
fn summarize_awareness(
    mut summary: ResMut<PerceptionSummary>,
    actors: Query<(&ActorRuntime, &ActorAwareness)>,
) {
    summary.aware_of_player.clear();
    for (runtime, awareness) in &actors {
        if awareness
            .state
            .target()
            .is_some_and(|target| target == TargetId::player())
        {
            summary.aware_of_player.insert(runtime.reference_form_id);
        }
    }
}

/// Attaches an `ActorAwareness` to every actor placement as it appears, so the
/// awareness authority exists for the whole actor lifetime.
#[allow(clippy::type_complexity)]
pub(crate) fn attach_awareness(
    mut commands: Commands,
    actors: Query<
        Entity,
        (
            With<ActorRuntime>,
            With<PlacementRoot>,
            Without<ActorAwareness>,
        ),
    >,
) {
    for entity in &actors {
        commands.entity(entity).insert(ActorAwareness::default());
    }
}

#[cfg(test)]
#[path = "tests/perception.rs"]
mod tests;
