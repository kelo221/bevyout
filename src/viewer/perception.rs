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
use bevyout_core::actor_state::{ActorLifeState, ActorValue, SpecialAttribute};
use bevyout_core::detection::{
    DetectionConfig, DetectionEvidence, ObserverHudInput, gameplay_light_bps,
    project_detection_hud, quantize_geometry, update_from_evidence,
};
use bevyout_core::disposition::{
    Aggression, DispositionActor, DispositionTarget, DispositionThresholds, FactionMembership,
    Hostility, resolve_disposition,
};
use bevyout_core::faction::FactionRelationTable;
use bevyout_core::perception::{
    AwarenessEvent, AwarenessState, PerceptionConfig, PerceptionInputs, TargetClass, TargetId,
};

use super::actor::ActorRuntime;
use super::actor_state::{ActorDefinitionCatalogs, ActorStateRuntime};
use super::hud::HudDetection;
use super::interaction::{PlacementRoot, PlayerEquipment};
use super::player::{CAPSULE_HEIGHT, FpsPlayer, KccState};
use super::world::ActiveSaveState;

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
                    persist_awareness,
                    summarize_awareness,
                    project_hud_detection,
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

fn observer_perception(state_runtime: &ActorStateRuntime) -> u16 {
    state_runtime
        .definition
        .resolve_value(
            &bevyout_core::actor_state::ActorInstanceState::default(),
            ActorValue::Special(SpecialAttribute::Perception),
        )
        .effective
        .clamp(0.0, 10.0)
        .round() as u16
}

fn movement_noise_bps(kcc: Option<&KccState>) -> u16 {
    let speed = kcc.map(|state| state.velocity.xz().length()).unwrap_or(0.0);
    if !speed.is_finite() {
        return 0;
    }
    (speed / 4.5 * 10_000.0).clamp(0.0, 10_000.0).round() as u16
}

fn armor_noise_bps(equipment: Option<&PlayerEquipment>) -> u16 {
    if equipment.is_some_and(|equipment| equipment.equipped_apparel().next().is_some()) {
        2_500
    } else {
        0
    }
}

#[allow(clippy::type_complexity)]
fn update_actor_awareness(
    time: Res<Time>,
    settings: Res<PerceptionSettings>,
    context: NonSend<BoxdddPhysicsContext>,
    ambient: Option<Res<GlobalAmbientLight>>,
    players: Query<(&GlobalTransform, Option<&KccState>), With<FpsPlayer>>,
    equipment: Option<Res<PlayerEquipment>>,
    mut actors: Query<(
        &ActorRuntime,
        &ActorStateRuntime,
        &GlobalTransform,
        &mut ActorAwareness,
    )>,
) {
    let dt = time.delta_secs();
    let dt_ms = (dt.max(0.0) * 1_000.0).round() as u32;
    let detection = DetectionConfig::from(settings.0);
    let light_bps = ambient
        .map(|light| {
            let [r, g, b, a] = light.color.to_srgba().to_f32_array();
            gameplay_light_bps([r, g, b, a]).unwrap_or(0)
        })
        .unwrap_or(0);
    let player = players.iter().next();
    let player_eye =
        player.map(|(transform, _)| transform.translation() + Vec3::Y * (CAPSULE_HEIGHT * 0.5));
    let movement = movement_noise_bps(player.and_then(|(_, kcc)| kcc));
    let armor = armor_noise_bps(equipment.as_deref());

    for (runtime, state_runtime, transform, mut awareness) in &mut actors {
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
            let event = awareness.state.update(&[], &settings.0, dt);
            log_awareness_event(runtime.reference_form_id, event);
            awareness.last_player = None;
            continue;
        };

        let actor_eye = transform.translation() + Vec3::Y * ACTOR_EYE_HEIGHT;
        let has_los = line_of_sight(&context, actor_eye, player_eye);
        let inputs =
            player_perception_inputs(actor_eye, *transform.forward(), player_eye, has_los, true);
        let evidence = match quantize_geometry(inputs.distance, inputs.angle_to_target) {
            Ok((distance_mm, angle_millidegrees)) => DetectionEvidence {
                observer: TargetId {
                    class: TargetClass::Actor,
                    form_id: runtime.reference_form_id,
                },
                subject: TargetId::player(),
                distance_mm,
                angle_millidegrees,
                light_bps,
                movement_noise_bps: movement,
                armor_noise_bps: armor,
                observer_perception: observer_perception(state_runtime),
                has_line_of_sight: has_los,
                delta_ms: dt_ms,
            },
            Err(_) => {
                awareness.last_player = Some(inputs);
                continue;
            }
        };
        let event = update_from_evidence(&mut awareness.state, &[evidence], &detection);
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
    save: Option<Res<ActiveSaveState>>,
    actors: Query<
        (Entity, &ActorRuntime, &ActorStateRuntime),
        (With<PlacementRoot>, Without<ActorAwareness>),
    >,
) {
    for (entity, runtime, state_runtime) in &actors {
        let restored = save.as_ref().and_then(|save| {
            save.0
                .cells
                .get(&state_runtime.cell_form_id)
                .and_then(|cell| cell.actors.get(&runtime.reference_form_id))
                .map(|state| ActorAwareness {
                    state: state.awareness,
                    last_player: None,
                })
        });
        commands.entity(entity).insert(restored.unwrap_or_default());
    }
}

fn persist_awareness(
    mut save: Option<ResMut<ActiveSaveState>>,
    actors: Query<(&ActorRuntime, &ActorStateRuntime, &ActorAwareness), Changed<ActorAwareness>>,
) {
    let Some(save) = save.as_mut() else {
        return;
    };
    for (runtime, state_runtime, awareness) in &actors {
        if let Some(state) = save
            .0
            .cells
            .get_mut(&state_runtime.cell_form_id)
            .and_then(|cell| cell.actors.get_mut(&runtime.reference_form_id))
        {
            state.awareness = awareness.state;
        }
    }
}

fn observer_is_hostile(catalogs: Option<&ActorDefinitionCatalogs>, runtime: &ActorRuntime) -> bool {
    let Some(catalogs) = catalogs else {
        return false;
    };
    let Some((cell_form_id, definition)) =
        catalogs.definition(runtime.reference_form_id, runtime.base_form_id)
    else {
        return false;
    };
    let table = catalogs
        .catalogs()
        .into_iter()
        .find(|(cell, _)| *cell == cell_form_id)
        .map(|(_, catalog)| catalog.faction_table.clone())
        .unwrap_or_else(FactionRelationTable::default);
    let aggression = catalogs
        .catalogs()
        .into_iter()
        .find(|(cell, _)| *cell == cell_form_id)
        .and_then(|(_, catalog)| {
            catalog.entries.iter().find_map(|entry| match entry {
                crate::vsa::ActorCatalogEntry::Prepared(blueprint)
                    if blueprint.reference_form_id == runtime.reference_form_id
                        || blueprint.base_form_id == runtime.base_form_id =>
                {
                    blueprint
                        .ai_data
                        .map(|data| Aggression::from_raw(data.aggression))
                }
                _ => None,
            })
        })
        .unwrap_or_default();
    let observer = DispositionActor {
        factions: definition
            .factions
            .iter()
            .map(|membership| FactionMembership {
                faction_form_id: membership.faction_form_id,
                rank: membership.rank,
            })
            .collect(),
        base_disposition: definition
            .base_values
            .get(&ActorValue::Disposition)
            .copied()
            .unwrap_or(0.0)
            .round() as i32,
        aggression,
        race_disposition_adjust: 0,
    };
    resolve_disposition(
        &observer,
        &DispositionTarget::default(),
        &table,
        &DispositionThresholds::default(),
    )
    .hostility
        == Hostility::Hostile
}

fn project_hud_detection(
    catalogs: Option<Res<ActorDefinitionCatalogs>>,
    mut hud: ResMut<HudDetection>,
    actors: Query<(&ActorRuntime, &ActorAwareness)>,
) {
    let mut observers = Vec::new();
    for (runtime, awareness) in &actors {
        observers.push(ObserverHudInput {
            hostile: observer_is_hostile(catalogs.as_deref(), runtime),
            acquired_player: awareness
                .state
                .target()
                .is_some_and(|target| target == TargetId::player()),
            confidence_milli: awareness.state.confidence_milli,
        });
    }
    hud.0 = project_detection_hud(&observers);
}

#[cfg(test)]
#[path = "tests/perception.rs"]
mod tests;
