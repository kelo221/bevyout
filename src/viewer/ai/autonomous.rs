//! Autonomous package driver (issue #218): the gameplay system that makes
//! the M4 wave's patrol/animation/collider stack run without any console
//! command. "Load a cell -> every alive actor binds a nav agent, selects and
//! runs its package, and walks its route" is the whole point of this wave;
//! before this module the only entry point was `tna bind` + `runpackage`,
//! typed by hand once per actor.
//!
//! # Life-state gate (mirrors `actor_state::seed_actor_states`)
//!
//! `project_prepared_actors` (`viewer::actor`) inserts [`ActorRuntime`] one
//! system before `seed_actor_states` (`viewer::actor_state`) inserts
//! [`ActorStateRuntime`] via `Added<ActorRuntime>` -- so `life_state` is
//! guaranteed present the instant `ActorStateRuntime` itself is `Added`.
//! [`queue_autonomous_start_candidates`] mirrors that exact filter.
//!
//! # Two systems, not one (real `Added<>` tracking)
//!
//! `Added<T>` is only tracked correctly by a `Query` system parameter Bevy
//! caches across calls -- an ad-hoc `World::query_filtered` built fresh
//! inside an exclusive system (the shape every other exclusive system in
//! `nav/agent.rs` uses for its *unfiltered* queries) would see every
//! matching entity as "added" on every single call, not just genuinely new
//! ones. Binding + starting a package needs `&mut World` (through
//! [`nav::api::bind_actor`] and
//! [`crate::viewer::console::ai_package_commands::start_package`]), which an
//! ordinary system cannot also do alongside a `Query` parameter. So this
//! splits in two: an ordinary system with a real `Query<..., Added<..>>`
//! queues candidates into [`PendingAutonomousStarts`], and an exclusive
//! system drains that queue and does the mutation.
//!
//! # Idempotent, and it never fights the console
//!
//! [`super::autonomous_gate::eligible_for_autonomous_start`] (pure, no Bevy)
//! is the single gate both here and its own cucumber scenario exercise: an
//! actor is only auto-bound if alive, not already nav-bound (a `tna bind` ran
//! first), and not already carrying an [`ActorPackageController`] (a
//! `runpackage` ran first). `tna`/`runpackage` keep working exactly as
//! before, side by side with this system.

use bevy::prelude::*;
use bevyout_core::actor_state::ActorLifeState;

use super::autonomous_gate::eligible_for_autonomous_start;
use super::family_runtime::ActorPackageController;
use super::lifecycle::PackageLifecycle;
use super::selection::GameInstant;
use crate::viewer::actor::ActorRuntime;
use crate::viewer::actor_state::ActorStateRuntime;
use crate::viewer::console::ai_package_commands::start_package;
use crate::viewer::day_night::GameClock;
use crate::viewer::nav::api;

/// Toggle for the autonomous driver, default on -- this wave's whole
/// deliverable is "on by default, no console command needed". Exists so
/// tests (and any future headless flow that wants only console-driven
/// actors) can disable it without removing the systems from the schedule.
#[derive(Resource)]
pub(crate) struct AutonomousPackageDriverEnabled(pub(crate) bool);

impl Default for AutonomousPackageDriverEnabled {
    fn default() -> Self {
        Self(true)
    }
}

/// Actors newly seeded with [`ActorStateRuntime`] this frame, queued by
/// [`queue_autonomous_start_candidates`] for [`drive_pending_autonomous_starts`]
/// to bind + start: `(entity, reference_form_id, is_alive)`.
#[derive(Resource, Default)]
pub(crate) struct PendingAutonomousStarts(Vec<(Entity, u32, bool)>);

/// Queues every actor whose life-state was just seeded this frame -- a
/// plain `Query` so Bevy's real change-detection tracks `Added<>` correctly
/// across frames (see the module doc's "two systems, not one"). Does no
/// gating itself beyond reading `life_state`: [`drive_pending_autonomous_starts`]
/// makes the actual start/skip decision, since only it has `&mut World`
/// enough to check whether the console already touched the actor.
fn queue_autonomous_start_candidates(
    actors: Query<(Entity, &ActorRuntime, &ActorStateRuntime), Added<ActorStateRuntime>>,
    enabled: Res<AutonomousPackageDriverEnabled>,
    mut pending: ResMut<PendingAutonomousStarts>,
) {
    if !enabled.0 {
        return;
    }
    for (entity, runtime, state) in &actors {
        pending.0.push((
            entity,
            runtime.reference_form_id,
            state.life_state == ActorLifeState::Alive,
        ));
    }
}

/// Drains [`PendingAutonomousStarts`], binding + starting a package for
/// every eligible actor (`autonomous_gate::eligible_for_autonomous_start`).
/// A skipped or failed actor never fails the frame -- every early-return
/// `runpackage`'s console command would take instead becomes a `warn!` and
/// the loop moves on, per issue #218 ("a system cannot fail a user").
fn drive_pending_autonomous_starts(world: &mut World) {
    let pending = std::mem::take(&mut world.resource_mut::<PendingAutonomousStarts>().0);
    if pending.is_empty() {
        return;
    }
    let enabled = world
        .get_resource::<AutonomousPackageDriverEnabled>()
        .is_some_and(|toggle| toggle.0);
    if !enabled {
        return;
    }
    let instant = game_instant(world);
    for (entity, reference_form_id, is_alive) in pending {
        let already_nav_bound = api::is_bound(world, entity);
        let already_has_controller = world.get::<ActorPackageController>(entity).is_some();
        if !eligible_for_autonomous_start(is_alive, already_nav_bound, already_has_controller) {
            continue;
        }
        if let Err(error) = api::bind_actor(world, entity) {
            warn!(
                "autonomous package driver: bind {reference_form_id:08x} skipped: {} ({})",
                error.message(),
                error.code()
            );
            continue;
        }
        match start_package(world, entity, reference_form_id, instant) {
            Ok(_) => {
                resume_saved_package_checkpoint(world, entity, reference_form_id);
                info!("autonomous package driver: bound + started actor {reference_form_id:08x}");
            }
            Err(error) => {
                // Binding and package start form one autonomous transaction.
                // A catalog/schedule/resolution failure must not leave an
                // agent with no controller consuming nav runtime work forever.
                api::release_actor(world, entity);
                warn!(
                    "autonomous package driver: start {reference_form_id:08x} skipped: {} ({})",
                    error.message, error.code
                );
            }
        }
    }
}

/// Resumes a package the actor was already running when its cell was
/// unloaded (M6 W3-C). The selection/resolution work is still done by
/// [`start_package`]; only the lifecycle's step and elapsed time are replaced,
/// and only when the freshly selected package is the checkpointed one -- a
/// different schedule choice must win over a stale checkpoint. No save shape
/// changes: `ActorInstanceState::package` already carried this field.
fn resume_saved_package_checkpoint(world: &mut World, entity: Entity, reference_form_id: u32) {
    let Some(checkpoint) =
        crate::viewer::world::exterior::saved_package_checkpoint(world, reference_form_id)
    else {
        return;
    };
    let Some(mut controller) = world.get_mut::<ActorPackageController>(entity) else {
        return;
    };
    if controller.selected_form_id != checkpoint.package_form_id {
        return;
    }
    controller.lifecycle = PackageLifecycle::from_checkpoint(checkpoint);
    info!(
        "autonomous package driver: resumed {reference_form_id:08x} package {:08x} step {}",
        checkpoint.package_form_id, checkpoint.procedure_index
    );
    // Issue #305 review: the checkpoint is a one-shot resume. Leaving it in
    // `ActiveSaveState` would rewind the actor a second time if the same
    // package starts again later in this session without an intervening
    // exterior unload (the only writer of this field).
    crate::viewer::world::exterior::clear_saved_package_checkpoint(world, reference_form_id);
}

fn game_instant(world: &World) -> GameInstant {
    GameInstant {
        hour: world
            .get_resource::<GameClock>()
            .map_or(GameInstant::default().hour, |clock| clock.hour),
        ..GameInstant::default()
    }
}

/// Registers the two systems above, chained after
/// [`super::family_runtime::drive_actor_packages`]'s own per-tick driver so a
/// freshly bound-and-started actor gets its first tick the same frame it
/// spawns. Called from [`super::family_runtime::AiPackagePlugin::build`].
pub(crate) fn register(app: &mut App) {
    app.init_resource::<AutonomousPackageDriverEnabled>()
        .init_resource::<PendingAutonomousStarts>()
        .add_systems(
            Update,
            (
                queue_autonomous_start_candidates,
                drive_pending_autonomous_starts,
            )
                .chain()
                .after(crate::viewer::actor_state::seed_actor_states)
                .before(super::family_runtime::drive_actor_packages),
        );
}

#[cfg(test)]
#[path = "tests/autonomous.rs"]
mod tests;
