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

use std::collections::HashSet;

use bevy::prelude::*;

use super::families::{
    FamilyAnimation, FamilyDriver, FamilyObservation, FamilyRequest, LifecycleSignal,
    PackageFamily, Waypoint,
};
use super::lifecycle::{LifecyclePhase, PackageLifecycle};
use crate::viewer::actor_animation::policy::ActorAnimationState;
use crate::viewer::actor_animation::request_actor_animation;
use crate::viewer::nav::agent;

/// Default arrival tolerance (metres) a family treats a waypoint as reached
/// within, even before the nav agent's own reached-target latch. Loose enough
/// that a KCC stopped a step short of the exact point still counts as arrived.
pub(crate) const DEFAULT_ARRIVAL_TOLERANCE: f32 = 1.5;

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
}

impl ActorPackageController {
    /// Starts `family` running the `selected_form_id` package over `waypoints`.
    pub(crate) fn start(
        reference_form_id: u32,
        selected_form_id: u32,
        family: PackageFamily,
        waypoints: Vec<Waypoint>,
        arrival_tolerance: f32,
    ) -> Self {
        let mut lifecycle = PackageLifecycle::new();
        lifecycle.on_select(Some(selected_form_id));
        Self {
            reference_form_id,
            selected_form_id,
            lifecycle,
            driver: FamilyDriver::new(family, waypoints, arrival_tolerance),
        }
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
        let observation = FamilyObservation {
            actor_position,
            nav_reached: agent::agent_reached_target(world, entity),
            route_failed: agent::agent_route_failed(world, entity),
        };

        // Phase A: advance the lifecycle clock and (while running) the family,
        // all under a single mutable borrow of the controller.
        let Some((request, signal, released, claim, family_label, ref_form_id)) = ({
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
                Some((
                    step.request,
                    step.signal,
                    released,
                    claim,
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
            }
            Some(FamilyRequest::Stop) => agent::clear_agent_target(world, entity),
            Some(FamilyRequest::Play(animation)) => {
                let _ = request_actor_animation(world, entity, animation_state(animation));
            }
            None => {}
        }
        // Completion/failure also stops any lingering nav route so the actor
        // does not keep steering toward a finished package's target.
        if matches!(signal, LifecycleSignal::Complete | LifecycleSignal::Fail) {
            agent::clear_agent_target(world, entity);
        }
        if signal != LifecycleSignal::Continue {
            info!(
                "package family {ref_form_id:08x} {family_label} {}",
                lifecycle_signal_label(signal)
            );
        }
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
            .add_systems(Update, drive_actor_packages);
    }
}

#[cfg(test)]
mod tests {
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
        let entity = world
            .spawn((
                Transform::from_translation(position),
                ActorAnimationIntent::default(),
                controller,
            ))
            .id();
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
}
