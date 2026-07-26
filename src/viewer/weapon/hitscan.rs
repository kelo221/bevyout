use bevy::ecs::system::SystemParam;
use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy::prelude::*;

use bevyout_core::weapon::{
    ImpactEvidence, ImpactOutcome, impact_is_in_range, resolve_actor_impact,
};

use super::{AcceptedWeaponShot, FireReport, FireStatus, PlayerWeaponRuntime};
use crate::viewer::actor::ActorRuntime;
use crate::viewer::actor_state::ActorStateRuntime;
use crate::viewer::world::ActiveSaveState;

const MAX_PARENT_DEPTH: usize = 64;

#[derive(SystemParam)]
pub(super) struct WeaponHitQueries<'w, 's> {
    cameras: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<Camera3d>>,
    parents: Query<'w, 's, &'static ChildOf>,
    viewmodels: Query<'w, 's, Entity, With<super::presentation::WeaponViewmodelRoot>>,
    actors: Query<'w, 's, (&'static ActorRuntime, &'static ActorStateRuntime)>,
}

pub(super) fn resolve_accepted_shots(
    mut shots: MessageReader<AcceptedWeaponShot>,
    mut raycast: MeshRayCast,
    queries: WeaponHitQueries,
    mut save: ResMut<ActiveSaveState>,
    mut runtime: ResMut<PlayerWeaponRuntime>,
) {
    for shot in shots.read() {
        let Some(ray) = active_center_ray(&queries.cameras) else {
            runtime.last_fire = FireReport {
                status: FireStatus::Miss,
                shot_index: Some(shot.shot_index),
                ..Default::default()
            };
            continue;
        };
        let filter =
            |entity| !has_viewmodel_ancestor(entity, &queries.parents, &queries.viewmodels);
        let settings = MeshRayCastSettings {
            visibility: RayCastVisibility::VisibleInView,
            ..default()
        }
        .with_filter(&filter)
        .always_early_exit();
        let Some((hit_entity, hit)) = raycast.cast_ray(ray, &settings).first() else {
            runtime.last_fire = FireReport {
                status: FireStatus::Miss,
                shot_index: Some(shot.shot_index),
                ..Default::default()
            };
            continue;
        };
        let evidence = ImpactEvidence {
            distance_meters: hit.distance,
        };
        if !impact_is_in_range(shot.weapon.definition(), evidence) {
            runtime.last_fire = FireReport {
                status: FireStatus::Miss,
                shot_index: Some(shot.shot_index),
                hit_distance: Some(hit.distance),
                ..Default::default()
            };
            continue;
        }
        let Some(actor_entity) =
            find_actor_ancestor(*hit_entity, &queries.parents, &queries.actors)
        else {
            runtime.last_fire = FireReport {
                status: FireStatus::WorldHit,
                shot_index: Some(shot.shot_index),
                hit_distance: Some(hit.distance),
                ..Default::default()
            };
            continue;
        };
        let Ok((actor, projected)) = queries.actors.get(actor_entity) else {
            continue;
        };
        let Some(state) = save
            .0
            .cells
            .get_mut(&projected.cell_form_id)
            .and_then(|cell| cell.actors.get_mut(&actor.reference_form_id))
        else {
            runtime.last_fire = FireReport {
                status: FireStatus::ActorStateUnavailable,
                shot_index: Some(shot.shot_index),
                target_reference_form_id: Some(actor.reference_form_id),
                hit_distance: Some(hit.distance),
                ..Default::default()
            };
            continue;
        };
        match resolve_actor_impact(
            shot.weapon.definition(),
            evidence,
            &projected.definition,
            state,
        ) {
            Ok(ImpactOutcome::Actor(outcome)) => {
                runtime.last_fire = FireReport {
                    status: if outcome.killed {
                        FireStatus::ActorKilled
                    } else {
                        FireStatus::ActorHit
                    },
                    shot_index: Some(shot.shot_index),
                    target_reference_form_id: Some(actor.reference_form_id),
                    hit_distance: Some(hit.distance),
                    applied_damage: Some(outcome.applied_damage),
                    remaining_health: Some(outcome.remaining_health),
                };
                info!(
                    "weapon hit {:08x} target={:08x} damage={:.1} health={:.1} life={}",
                    shot.weapon.base_form_id,
                    actor.reference_form_id,
                    outcome.applied_damage,
                    outcome.remaining_health,
                    state.life_state.label()
                );
            }
            Ok(ImpactOutcome::OutOfRange) => unreachable!("range checked before actor resolution"),
            Err(error) => {
                runtime.last_fire = FireReport {
                    status: FireStatus::ActorStateUnavailable,
                    shot_index: Some(shot.shot_index),
                    target_reference_form_id: Some(actor.reference_form_id),
                    hit_distance: Some(hit.distance),
                    ..Default::default()
                };
                warn!(
                    "weapon hit {:08x} target={:08x} rejected: {error}",
                    shot.weapon.base_form_id, actor.reference_form_id
                );
            }
        }
    }
}

fn active_center_ray(
    cameras: &Query<(&Camera, &GlobalTransform), With<Camera3d>>,
) -> Option<Ray3d> {
    cameras.iter().find_map(|(camera, transform)| {
        if !camera.is_active {
            return None;
        }
        let viewport = camera.logical_viewport_size()?;
        camera.viewport_to_world(transform, viewport * 0.5).ok()
    })
}

fn has_viewmodel_ancestor(
    mut entity: Entity,
    parents: &Query<&ChildOf>,
    roots: &Query<Entity, With<super::presentation::WeaponViewmodelRoot>>,
) -> bool {
    for _ in 0..MAX_PARENT_DEPTH {
        if roots.contains(entity) {
            return true;
        }
        let Ok(parent) = parents.get(entity) else {
            return false;
        };
        entity = parent.parent();
    }
    false
}

fn find_actor_ancestor(
    mut entity: Entity,
    parents: &Query<&ChildOf>,
    actors: &Query<(&ActorRuntime, &ActorStateRuntime)>,
) -> Option<Entity> {
    for _ in 0..MAX_PARENT_DEPTH {
        if actors.contains(entity) {
            return Some(entity);
        }
        entity = parents.get(entity).ok()?.parent();
    }
    None
}
