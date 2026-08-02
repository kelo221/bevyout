//! Bevy adapter that drives the pure package families (`ai::families`,
//! issues #196/#197) on nav-bound actors.
//!
//! [`ActorPackageController`] carries one actor's [`PackageLifecycle`] (#194)
//! and its [`FamilyDriver`] (#196/#197). [`drive_actor_packages`] is the
//! system that finally makes the lifecycle's time transitions run: each tick it
//! samples the bound nav agent (position, reached-target, no-path) into a
//! [`FamilyObservation`], ticks the lifecycle, and -- while the package is
//! `Running` -- ticks the family, then applies the single [`FamilyRequest`] it
//! returns and the [`LifecycleSignal`] transition.
//!
//! # One movement authority (verdict §2.3)
//!
//! This adapter translates a family's request into exactly two runtime effects:
//! a nav route (`agent::route_agent_to_point`/`clear_agent_target`) and an
//! animation state (`request_actor_animation`). It never writes
//! `Transform.translation`; occupying an interaction point is a claim in
//! [`PackageInteractionOccupancy`] plus the nav route that already put the actor
//! there. `family_driver_writes_no_transform_translation` below is the
//! minimal-`App` test that fails if that ever changes.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use super::families::{
    FamilyAnimation, FamilyDriver, FamilyObservation, FamilyRequest, LifecycleSignal,
    PackageFamily, Waypoint,
};
use super::lifecycle::{LifecyclePhase, PackageLifecycle};
use super::resolution::{ResolutionContext, ResolvedReference};
use crate::viewer::actor_animation::policy::ActorAnimationState;
use crate::viewer::actor_animation::request_actor_animation;
use crate::viewer::interaction;
use crate::viewer::nav::agent;

/// Compatibility re-export for the Bevy adapter and console callers. The
/// policy lives in the pure family module so feature tests exercise the same
/// value as runtime package drivers.
pub(crate) use super::families::DEFAULT_ARRIVAL_TOLERANCE;

/// The set of interaction points currently occupied by an eat/sleep package,
/// so a second actor's family does not claim furniture already in use
/// (`families::select_interaction_point` reads this). Runtime-only; not
/// persisted.
#[derive(Resource, Default)]
pub(crate) struct PackageInteractionOccupancy(pub(crate) HashSet<u32>);

/// One actor's running package: the lifecycle, the family driving it, and the
/// identities the `runpackage` console view reports.
#[derive(Component)]
pub(crate) struct ActorPackageController {
    pub(crate) reference_form_id: u32,
    pub(crate) selected_form_id: u32,
    pub(crate) lifecycle: PackageLifecycle,
    pub(crate) driver: FamilyDriver,
    /// The leader entity a follow (#198) samples each tick for its moving
    /// target; `None` for every non-follow family.
    pub(crate) follow_target: Option<Entity>,
}

impl ActorPackageController {
    fn new(
        reference_form_id: u32,
        selected_form_id: u32,
        driver: FamilyDriver,
        follow_target: Option<Entity>,
    ) -> Self {
        let mut lifecycle = PackageLifecycle::new();
        lifecycle.on_select(Some(selected_form_id));
        Self {
            reference_form_id,
            selected_form_id,
            lifecycle,
            driver,
            follow_target,
        }
    }

    /// Starts a scheduled `family` (Travel/Patrol/Idle/Eat/Sleep) running the
    /// `selected_form_id` package over `waypoints`.
    pub(crate) fn start(
        reference_form_id: u32,
        selected_form_id: u32,
        family: PackageFamily,
        waypoints: Vec<Waypoint>,
        arrival_tolerance: f32,
    ) -> Self {
        Self::new(
            reference_form_id,
            selected_form_id,
            FamilyDriver::new(family, waypoints, arrival_tolerance),
            None,
        )
    }

    /// Starts a Follow (#198) package: `follow_target` is the leader entity
    /// whose live position the driver maintains its distance band from.
    pub(crate) fn start_follow(
        reference_form_id: u32,
        selected_form_id: u32,
        follow_target: Entity,
        band_min: f32,
        band_max: f32,
        arrival_tolerance: f32,
    ) -> Self {
        Self::new(
            reference_form_id,
            selected_form_id,
            FamilyDriver::follow(band_min, band_max, arrival_tolerance),
            Some(follow_target),
        )
    }

    /// Starts a Sandbox/Wander (#198) package roaming within `radius` of
    /// `center`.
    pub(crate) fn start_wander(
        reference_form_id: u32,
        selected_form_id: u32,
        center: [f32; 3],
        radius: f32,
        idle_seconds: f32,
        seed: u64,
        arrival_tolerance: f32,
    ) -> Self {
        Self::new(
            reference_form_id,
            selected_form_id,
            FamilyDriver::wander(center, radius, idle_seconds, seed, arrival_tolerance),
            None,
        )
    }
}

/// Maps a family's abstract animation onto the runtime clip state. Eat/Sleep
/// have no dedicated furniture clips yet, so they fall back to the idle clip --
/// the actor stands at the occupied point rather than mid-stride. The distinct
/// intent is preserved in `families::FamilyAnimation` for future clip work.
const fn animation_state(animation: FamilyAnimation) -> ActorAnimationState {
    match animation {
        FamilyAnimation::Idle | FamilyAnimation::Eat | FamilyAnimation::Sleep => {
            ActorAnimationState::Idle
        }
    }
}

/// Per-actor package tick. Exclusive because a family request routes the nav
/// agent and requests an animation, both of which need `&mut World`.
pub(crate) fn drive_actor_packages(world: &mut World) {
    let dt = world.resource::<Time>().delta_secs();
    let entities: Vec<Entity> = world
        .query_filtered::<Entity, With<ActorPackageController>>()
        .iter(world)
        .collect();

    for entity in entities {
        // Observation: sample the bound nav agent before touching the controller.
        let actor_position = world
            .get::<Transform>(entity)
            .map_or([0.0; 3], |transform| transform.translation.to_array());
        // A follow family reads its leader's live position (None == the leader
        // is gone: target loss). Every other family leaves this None.
        let follow_target = world
            .get::<ActorPackageController>(entity)
            .and_then(|controller| controller.follow_target);
        let target_position = follow_target
            .and_then(|leader| world.get::<Transform>(leader))
            .map(|transform| transform.translation.to_array());
        let observation = FamilyObservation {
            target_position,
            // #185's door-link `Failed` terminal names the door the agent's
            // route gave up on; a follow abandons at a door it cannot open.
            blocking_door: agent::agent_blocking_door(world, entity),
            ..FamilyObservation::new(
                actor_position,
                agent::agent_reached_target(world, entity),
                agent::agent_route_failed(world, entity),
            )
        };

        // Phase A: advance the lifecycle clock and (while running) the family,
        // all under a single mutable borrow of the controller.
        let Some((request, signal, released, claim, orientation, family_label, ref_form_id)) = ({
            let Some(mut controller) = world.get_mut::<ActorPackageController>(entity) else {
                continue;
            };
            controller.lifecycle.tick(dt);
            if controller.lifecycle.phase() != LifecyclePhase::Running {
                None
            } else {
                let step = controller.driver.tick(&observation, dt);
                match step.signal {
                    LifecycleSignal::AdvanceStep => controller.lifecycle.advance_step(),
                    LifecycleSignal::Complete => controller.lifecycle.complete(),
                    LifecycleSignal::Fail => {
                        controller.lifecycle.fail();
                    }
                    LifecycleSignal::Continue => {}
                }
                let released = if matches!(
                    step.signal,
                    LifecycleSignal::Complete | LifecycleSignal::Fail
                ) {
                    controller.driver.release()
                } else {
                    None
                };
                let claim = controller.driver.occupied_point();
                // The authored idle/patrol-marker facing to apply when the actor
                // is holding a pose (a `Play` request), if any.
                let orientation = matches!(step.request, Some(FamilyRequest::Play(_)))
                    .then(|| controller.driver.current_orientation_yaw())
                    .flatten();
                Some((
                    step.request,
                    step.signal,
                    released,
                    claim,
                    orientation,
                    controller.driver.family().label(),
                    controller.reference_form_id,
                ))
            }
        }) else {
            continue;
        };

        // Phase B: apply the effects (occupancy registry, nav route, animation).
        if let Some(point) = released {
            world
                .resource_mut::<PackageInteractionOccupancy>()
                .0
                .remove(&point);
        }
        if let Some(point) = claim {
            world
                .resource_mut::<PackageInteractionOccupancy>()
                .0
                .insert(point);
        }
        match request {
            Some(FamilyRequest::Route(point)) => {
                agent::route_agent_to_point(world, entity, Vec3::from_array(point));
                // Moving: facing belongs to navigation again.
                agent::set_facing_authority(world, entity, false);
            }
            Some(FamilyRequest::Stop) => {
                agent::clear_agent_target(world, entity);
                agent::set_facing_authority(world, entity, false);
            }
            Some(FamilyRequest::Play(animation)) => {
                let _ = request_actor_animation(world, entity, animation_state(animation));
                if let Some(yaw) = orientation {
                    // Holding an authored pose: claim facing so the nav-derived
                    // writer (`face_bound_actors`) yields, then write the pose
                    // yaw. Exactly one system writes rotation this frame -- this
                    // is a rotation write only; translation (the KCC's sole
                    // authority) is never touched here.
                    agent::set_facing_authority(world, entity, true);
                    if let Some(mut transform) = world.get_mut::<Transform>(entity) {
                        transform.rotation = Quat::from_rotation_y(yaw);
                    }
                } else {
                    // No authored pose to hold: let navigation steer facing.
                    agent::set_facing_authority(world, entity, false);
                }
            }
            None => {}
        }
        // Completion/failure also stops any lingering nav route so the actor
        // does not keep steering toward a finished package's target.
        if matches!(signal, LifecycleSignal::Complete | LifecycleSignal::Fail) {
            agent::clear_agent_target(world, entity);
        }
        if signal != LifecycleSignal::Continue {
            // A follow that failed because of a door it could not open names
            // that specific door (the `Unreachable`-naming terminal, verdict
            // §4.5) rather than an anonymous fail.
            if family_label == "follow"
                && signal == LifecycleSignal::Fail
                && let Some(door) = observation.blocking_door
            {
                info!("package family {ref_form_id:08x} follow unreachable door {door:08x}");
            } else {
                info!(
                    "package family {ref_form_id:08x} {family_label} {}",
                    lifecycle_signal_label(signal)
                );
            }
        }
    }
}

/// Nav release hook (registered by [`AiPackagePlugin`] to satisfy nav's
/// bound-actor release contract): tears down a released actor's running package.
/// Nav's `release_bound_actor` (the #164 fall guard / `tna despawn`) calls this
/// after it strips the agent, so a package can no longer tick an agent-less
/// actor -- the controller is removed and any interaction point it claimed is
/// freed. A no-op for a released actor that was not running a package.
pub(crate) fn release_actor_package(world: &mut World, entity: Entity) {
    let Some(mut controller) = world.get_mut::<ActorPackageController>(entity) else {
        return;
    };
    let released = controller.driver.release();
    world.entity_mut(entity).remove::<ActorPackageController>();
    if let Some(point) = released {
        world
            .resource_mut::<PackageInteractionOccupancy>()
            .0
            .remove(&point);
    }
}

const fn lifecycle_signal_label(signal: LifecycleSignal) -> &'static str {
    match signal {
        LifecycleSignal::Continue => "continue",
        LifecycleSignal::AdvanceStep => "advance",
        LifecycleSignal::Complete => "complete",
        LifecycleSignal::Fail => "fail",
    }
}

/// Installs the AI package-family runtime: the shared occupancy registry and the
/// per-actor driver system. The driver is inert until a `runpackage` console
/// command attaches an [`ActorPackageController`], so there is no always-on
/// per-actor cost in an ordinary viewer session.
pub(crate) struct AiPackagePlugin;

impl Plugin for AiPackagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PackageInteractionOccupancy>()
            .init_resource::<super::catalog_cache::PackageCatalogCache>()
            .add_systems(Update, drive_actor_packages);
        // Satisfy nav's bound-actor release contract: when nav releases this
        // actor (the #164 fall guard / `tna despawn`), tear its package down too
        // -- otherwise the controller keeps ticking an agent-less actor forever
        // and its interaction point never frees.
        agent::register_bound_actor_release_hook(app.world_mut(), release_actor_package);
        // Issue #218: the always-on autonomous package driver. Registered
        // here (not a separate plugin) because it is the other half of this
        // plugin's job -- making a package actually run -- just triggered by
        // a fresh actor life-state instead of a `runpackage` console command.
        super::autonomous::register(app);
    }
}

/// Builds the pure [`ResolutionContext`] snapshot `ai::resolution` resolves
/// against, from the active cell's placements and every live
/// `interaction::PlacementRoot` entity (issue #218: moved out of the console
/// layer, mechanically -- it only reads `World` state -- so `runpackage` and
/// the autonomous package driver share one implementation).
///
/// Two passes because placements know the *authored* layout (every
/// reference, even ones with no live entity yet) while the live entity query
/// is what supplies an entity handle for a follow leader/interaction point;
/// the second pass's `position`/`entity` overwrite the first's for any
/// reference that is actually spawned, which is the live position that
/// matters once the actor has moved.
///
/// The authored placement yaw is retained for patrol-marker dwell poses. The
/// same `EulerRot::YXZ` convention is used by nav-facing code and by the
/// runtime pose writer, so pitch/roll are intentionally discarded here.
fn placement_yaw(rotation_xyzw: [f32; 4]) -> f32 {
    Quat::from_array(rotation_xyzw)
        .normalize()
        .to_euler(EulerRot::YXZ)
        .0
}

pub(crate) fn build_resolution_context(
    world: &mut World,
    actor_reference_form_id: u32,
) -> ResolutionContext {
    let current_cell_form_id = world
        .get_resource::<crate::viewer::LoadedSceneManifest>()
        .map_or(0, |manifest| manifest.0.cell.form_id);
    let mut references = HashMap::new();
    let mut bases: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut actor_position = [0.0f32; 3];
    let mut actor_editor_location = None;
    let mut linked_reference = None;

    if let Some(manifest) = world.get_resource::<crate::viewer::LoadedSceneManifest>() {
        for placement in &manifest.0.placements {
            let resolved = ResolvedReference {
                reference_form_id: placement.reference_form_id,
                base_form_id: placement.base_form_id,
                cell_form_id: current_cell_form_id,
                position: placement.translation,
                entity: None,
                linked_reference: placement.linked_reference_form_id,
                orientation_yaw: Some(placement_yaw(placement.rotation_xyzw)),
            };
            if placement.reference_form_id == actor_reference_form_id {
                actor_position = placement.translation;
                actor_editor_location = Some(placement.translation);
                linked_reference = placement.linked_reference_form_id;
            }
            bases
                .entry(placement.base_form_id)
                .or_default()
                .push(placement.reference_form_id);
            references.insert(placement.reference_form_id, resolved);
        }
    }

    let mut query = world.query::<(Entity, &interaction::PlacementRoot, &Transform)>();
    for (entity, root, transform) in query.iter(world) {
        let placement = root.placement();
        let position = transform.translation.to_array();
        if !references.contains_key(&placement.reference_form_id) {
            bases
                .entry(placement.base_form_id)
                .or_default()
                .push(placement.reference_form_id);
        }
        references.insert(
            placement.reference_form_id,
            ResolvedReference {
                reference_form_id: placement.reference_form_id,
                base_form_id: placement.base_form_id,
                cell_form_id: current_cell_form_id,
                position,
                entity: Some(entity.to_bits()),
                linked_reference: placement.linked_reference_form_id,
                orientation_yaw: Some(transform.rotation.to_euler(EulerRot::YXZ).0),
            },
        );
        if placement.reference_form_id == actor_reference_form_id {
            actor_position = position;
        }
    }

    ResolutionContext {
        current_cell_form_id,
        actor_position,
        actor_editor_location,
        linked_reference,
        references,
        bases,
        ..ResolutionContext::default()
    }
}

#[cfg(test)]
#[path = "tests/family_runtime.rs"]
mod tests;
