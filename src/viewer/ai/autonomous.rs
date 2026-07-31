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
//! [`agent::bind_agent_entity`] and
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
use super::selection::GameInstant;
use crate::viewer::actor::ActorRuntime;
use crate::viewer::actor_state::ActorStateRuntime;
use crate::viewer::console::ai_package_commands::start_package;
use crate::viewer::day_night::GameClock;
use crate::viewer::nav::agent;

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
        let already_nav_bound = agent::is_nav_bound(world, entity);
        let already_has_controller = world.get::<ActorPackageController>(entity).is_some();
        if !eligible_for_autonomous_start(is_alive, already_nav_bound, already_has_controller) {
            continue;
        }
        if let Err(error) = agent::bind_agent_entity(world, entity) {
            warn!(
                "autonomous package driver: bind {reference_form_id:08x} skipped: {} ({})",
                error.message, error.code
            );
            continue;
        }
        match start_package(world, entity, reference_form_id, instant) {
            Ok(_) => {
                info!("autonomous package driver: bound + started actor {reference_form_id:08x}");
            }
            Err(error) => {
                // Binding and package start form one autonomous transaction.
                // A catalog/schedule/resolution failure must not leave an
                // agent with no controller consuming nav runtime work forever.
                agent::release_bound_actor(world, entity);
                warn!(
                    "autonomous package driver: start {reference_form_id:08x} skipped: {} ({})",
                    error.message, error.code
                );
            }
        }
    }
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
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;
    use bevyout_core::actor::ActorKind;

    /// A minimal world with a nav archipelago already "current" (so
    /// `bind_agent_entity`'s `ensure_archipelago` short-circuits without a
    /// real nav-graph file -- the same trick F3's own bind tests use) plus a
    /// synthetic actor+package catalog written to a temp asset root, so
    /// `start_package`'s real catalog reads succeed.
    struct Fixture {
        world: World,
        temp_root: std::path::PathBuf,
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.temp_root);
        }
    }

    /// The same golden manifest fixture `console::tests::fixture_manifest`
    /// parses (`ron::de::from_str`, not a hand-listed struct literal --
    /// `PreparedSceneManifest` has no `Default` and dozens of fields).
    fn fixture_manifest() -> crate::vsa::PreparedSceneManifest {
        ron::de::from_str(include_str!("../../../features/fixtures/scene.ron"))
            .expect("synthetic scene fixture should parse")
    }

    fn build_fixture(test_name: &str, reference_form_id: u32, package_type: u8) -> Fixture {
        let mut manifest = fixture_manifest();
        let cell_form_id = manifest.cell.form_id;
        let mut world = World::new();
        world.init_resource::<AutonomousPackageDriverEnabled>();
        world.init_resource::<PendingAutonomousStarts>();

        let archipelago = world.spawn_empty().id();
        agent::mark_test_archipelago_current(&mut world, cell_form_id, archipelago);

        let fingerprint = format!("autonomous-fp-{test_name}");
        let temp_root = std::env::temp_dir().join(format!(
            "bevyout-autonomous-{test_name}-{}-{reference_form_id:08x}",
            std::process::id()
        ));
        let scene_dir = temp_root.join("scenes").join(format!("{cell_form_id:08x}"));
        std::fs::create_dir_all(&scene_dir).expect("create synthetic prepared-cell fixture dir");

        let actor_catalog = crate::vsa::PreparedActorCatalog {
            revision: crate::vsa::ACTOR_CATALOG_REVISION.into(),
            source_fingerprint: fingerprint.clone(),
            entries: vec![crate::vsa::ActorCatalogEntry::Prepared(Box::new(
                crate::vsa::ActorBlueprint {
                    base_form_id: 1,
                    reference_form_id,
                    record_kind: "NPC_".into(),
                    package_form_ids: vec![0x50],
                    ..crate::vsa::ActorBlueprint::default()
                },
            ))],
            counters: Default::default(),
            faction_table: Default::default(),
        };
        let actor_catalog_ron =
            ron::ser::to_string_pretty(&actor_catalog, ron::ser::PrettyConfig::default())
                .expect("serialize synthetic actor catalog");
        std::fs::write(scene_dir.join("actors.ron"), &actor_catalog_ron)
            .expect("write synthetic actor catalog fixture");

        // Hand-authored RON, not a Rust struct literal: `PreparedPackageEntry
        // ::location`'s `PackageLocationInput` type is private to
        // `vsa::prepare::package_catalog` (not re-exported), so it cannot be
        // named outside `vsa` at all -- only deserialized. `location_type: 2`
        // is "Near Current Location", which always resolves to the actor's
        // own position with no FormID needed, so the started package's
        // resolution step succeeds deterministically.
        let catalog_dir = temp_root.join("catalogs").join(&fingerprint);
        std::fs::create_dir_all(&catalog_dir).expect("create synthetic package catalog dir");
        let packages_ron = format!(
            r#"(
    revision: "{revision}",
    source_fingerprint: "{fingerprint}",
    packages: [
        (
            form_id: 0x50,
            editor_id: Some("PKWander"),
            package_type: {package_type},
            location: Some((
                location_type: 2,
                form_id: None,
                raw_value: 0,
                radius: 5,
            )),
        ),
    ],
    counters: (
        total: 1,
        unsupported_type: 0,
        unsupported_subrecord: 0,
        deferred_subrecord: 0,
        unresolved_location: 0,
        unresolved_target: 0,
        out_of_scope_location: 0,
        out_of_scope_target: 0,
    ),
)
"#,
            revision = crate::vsa::PACKAGE_CATALOG_REVISION,
        );
        std::fs::write(catalog_dir.join("packages.ron"), packages_ron)
            .expect("write synthetic package catalog fixture");

        manifest.asset_root = temp_root.to_string_lossy().into_owned();
        manifest.source_fingerprint = fingerprint;
        manifest.actor_catalog_path = Some(format!("scenes/{cell_form_id:08x}/actors.ron"));
        manifest.actor_catalog_hash =
            Some(crate::viewer::fingerprint(actor_catalog_ron.as_bytes()));
        manifest.nav_graph = Some(crate::vsa::PreparedNavGraphSource::default());
        world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));

        Fixture { world, temp_root }
    }

    fn spawn_actor(
        world: &mut World,
        reference_form_id: u32,
        life_state: ActorLifeState,
    ) -> Entity {
        world
            .spawn((
                ActorRuntime {
                    base_form_id: 1,
                    reference_form_id,
                    kind: ActorKind::Humanoid,
                    assembly: None,
                },
                ActorStateRuntime {
                    cell_form_id: 0xBEEF,
                    definition: std::sync::Arc::new(Default::default()),
                    life_state,
                },
                Transform::default(),
            ))
            .id()
    }

    #[test]
    fn an_alive_actor_with_a_resolvable_package_auto_binds_and_starts() {
        let mut fixture = build_fixture("alive", 0x1001, 5 /* Wander */);
        let actor = spawn_actor(&mut fixture.world, 0x1001, ActorLifeState::Alive);

        fixture
            .world
            .run_system_once(queue_autonomous_start_candidates)
            .unwrap();
        drive_pending_autonomous_starts(&mut fixture.world);

        assert!(
            agent::is_nav_bound(&fixture.world, actor),
            "the alive actor must be nav-bound"
        );
        assert!(
            fixture.world.get::<ActorPackageController>(actor).is_some(),
            "the alive actor must have a running package controller"
        );
    }

    #[test]
    fn a_dead_actor_is_never_auto_bound() {
        let mut fixture = build_fixture("dead", 0x1002, 5);
        let actor = spawn_actor(&mut fixture.world, 0x1002, ActorLifeState::Dead);

        fixture
            .world
            .run_system_once(queue_autonomous_start_candidates)
            .unwrap();
        drive_pending_autonomous_starts(&mut fixture.world);

        assert!(
            !agent::is_nav_bound(&fixture.world, actor),
            "a corpse must never be nav-bound by the autonomous driver"
        );
        assert!(fixture.world.get::<ActorPackageController>(actor).is_none());
    }

    /// An actor the console already bound (`tna bind`) before its
    /// `ActorStateRuntime` arrived must not be bound a second time or have
    /// its console-driven state disturbed.
    #[test]
    fn an_already_nav_bound_actor_is_not_double_bound() {
        let mut fixture = build_fixture("console-bound", 0x1003, 5);
        let actor = spawn_actor(&mut fixture.world, 0x1003, ActorLifeState::Alive);
        // Simulate a `tna bind` that already ran, through the real bind
        // entry point (not a private-component insert -- `AgentKcc` is not
        // nameable outside `nav::agent`).
        agent::bind_agent_entity(&mut fixture.world, actor).expect("pre-bind succeeds");

        fixture
            .world
            .run_system_once(queue_autonomous_start_candidates)
            .unwrap();
        drive_pending_autonomous_starts(&mut fixture.world);

        // No package controller was attached -- the driver left the
        // console-bound actor alone rather than starting a second package
        // on top of whatever the console is already doing.
        assert!(fixture.world.get::<ActorPackageController>(actor).is_none());
    }

    /// The disable toggle (default on) is a real no-op switch: with it off,
    /// an otherwise-eligible alive actor is left completely untouched.
    #[test]
    fn the_disabled_toggle_is_a_complete_no_op() {
        let mut fixture = build_fixture("disabled", 0x1004, 5);
        fixture
            .world
            .resource_mut::<AutonomousPackageDriverEnabled>()
            .0 = false;
        let actor = spawn_actor(&mut fixture.world, 0x1004, ActorLifeState::Alive);

        fixture
            .world
            .run_system_once(queue_autonomous_start_candidates)
            .unwrap();
        drive_pending_autonomous_starts(&mut fixture.world);

        assert!(!agent::is_nav_bound(&fixture.world, actor));
        assert!(fixture.world.get::<ActorPackageController>(actor).is_none());
    }

    #[test]
    fn an_autonomous_batch_reads_each_catalog_once() {
        let mut fixture = build_fixture("catalog-cache", 0x1005, 5);
        let first = spawn_actor(&mut fixture.world, 0x1005, ActorLifeState::Alive);
        let second = spawn_actor(&mut fixture.world, 0x1005, ActorLifeState::Alive);

        fixture
            .world
            .run_system_once(queue_autonomous_start_candidates)
            .unwrap();
        drive_pending_autonomous_starts(&mut fixture.world);

        assert!(fixture.world.get::<ActorPackageController>(first).is_some());
        assert!(
            fixture
                .world
                .get::<ActorPackageController>(second)
                .is_some()
        );
        assert_eq!(
            fixture
                .world
                .resource::<super::super::catalog_cache::PackageCatalogCache>()
                .disk_loads(),
            (1, 1),
            "the whole actor batch must share its prepared catalog reads"
        );
    }

    #[test]
    fn autonomous_selection_uses_the_live_game_clock() {
        let mut world = World::new();
        world.insert_resource(GameClock {
            hour: 19.5,
            timescale: 30.0,
        });

        assert_eq!(game_instant(&world).hour, 19.5);
    }

    #[test]
    fn a_failed_package_start_rolls_back_the_autonomous_bind() {
        let mut fixture = build_fixture("rollback", 0x1006, 16 /* unsupported */);
        let actor = spawn_actor(&mut fixture.world, 0x1006, ActorLifeState::Alive);

        fixture
            .world
            .run_system_once(queue_autonomous_start_candidates)
            .unwrap();
        drive_pending_autonomous_starts(&mut fixture.world);

        assert!(!agent::is_nav_bound(&fixture.world, actor));
        assert!(fixture.world.get::<ActorPackageController>(actor).is_none());
    }
}
