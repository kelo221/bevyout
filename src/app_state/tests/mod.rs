use std::path::PathBuf;

use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

use super::plugin::RequestStateTransition;
use super::*;

/// Headless app: `MinimalPlugins` + `StatesPlugin` + the slice under test.
/// No rendering, no window — matches the plan's T35.1-T35.7 requirement.
fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, AppStatePlugin));
    app
}

/// Same as [`test_app`], but also wires the CLI-style Boot/Loading
/// auto-advance systems that `viewer::run_view` adds in the real app —
/// needed for the synthetic Boot->Loading->InGame scenarios (T35.6, T35.7).
/// Kept separate from `test_app` so the plain transition-table tests can
/// drive `AppState` by hand without racing an auto-advance system.
fn test_app_with_auto_advance() -> App {
    let mut app = test_app();
    app.add_systems(Update, (auto_advance_from_boot, auto_advance_from_loading));
    app
}

fn send(app: &mut App, request: RequestStateTransition) {
    app.world_mut()
        .resource_mut::<Messages<RequestStateTransition>>()
        .write(request);
}

fn current_app_state(app: &App) -> AppState {
    *app.world().resource::<State<AppState>>().get()
}

fn current_modal(app: &App) -> GameplayModal {
    *app.world().resource::<State<GameplayModal>>().get()
}

/// Drives one legal request through to completion: writes the message, then
/// updates enough times for `StateTransition` to apply it.
fn apply(app: &mut App, request: RequestStateTransition) {
    send(app, request);
    app.update();
    app.update();
}

// T35.1: every legal transition in the F35.3 table lands in the expected state.
#[test]
fn every_legal_app_transition_lands_in_expected_state() {
    let mut app = test_app();
    assert_eq!(current_app_state(&app), AppState::Boot);

    apply(&mut app, RequestStateTransition::App(AppState::Loading));
    assert_eq!(current_app_state(&app), AppState::Loading);

    apply(&mut app, RequestStateTransition::App(AppState::MainMenu));
    assert_eq!(current_app_state(&app), AppState::MainMenu);

    apply(&mut app, RequestStateTransition::App(AppState::Loading));
    assert_eq!(current_app_state(&app), AppState::Loading);

    apply(&mut app, RequestStateTransition::App(AppState::InGame));
    assert_eq!(current_app_state(&app), AppState::InGame);

    apply(&mut app, RequestStateTransition::App(AppState::MainMenu));
    assert_eq!(current_app_state(&app), AppState::MainMenu);
}

#[test]
fn every_legal_modal_transition_lands_in_expected_state() {
    let mut app = test_app();
    apply(&mut app, RequestStateTransition::App(AppState::Loading));
    apply(&mut app, RequestStateTransition::App(AppState::InGame));
    assert_eq!(current_app_state(&app), AppState::InGame);
    assert_eq!(current_modal(&app), GameplayModal::None);

    for modal in [
        GameplayModal::Paused,
        GameplayModal::Dialogue,
        GameplayModal::PipBoy,
        GameplayModal::Console,
    ] {
        apply(&mut app, RequestStateTransition::Modal(modal));
        assert_eq!(current_modal(&app), modal);
        apply(&mut app, RequestStateTransition::Modal(GameplayModal::None));
        assert_eq!(current_modal(&app), GameplayModal::None);
    }
}

// T35.2: representative illegal transitions are rejected; state unchanged.
#[test]
fn illegal_transitions_are_rejected_and_state_is_unchanged() {
    let mut app = test_app();
    assert_eq!(current_app_state(&app), AppState::Boot);

    // Boot -> InGame is not in the legal table.
    apply(&mut app, RequestStateTransition::App(AppState::InGame));
    assert_eq!(current_app_state(&app), AppState::Boot);

    // Loading -> Paused: modal transitions require AppState::InGame.
    apply(&mut app, RequestStateTransition::App(AppState::Loading));
    assert_eq!(current_app_state(&app), AppState::Loading);
    apply(
        &mut app,
        RequestStateTransition::Modal(GameplayModal::Paused),
    );
    assert_eq!(current_modal(&app), GameplayModal::None);

    apply(&mut app, RequestStateTransition::App(AppState::InGame));
    assert_eq!(current_app_state(&app), AppState::InGame);

    // Paused -> Dialogue: only None <-> X round trips are legal.
    apply(
        &mut app,
        RequestStateTransition::Modal(GameplayModal::Paused),
    );
    assert_eq!(current_modal(&app), GameplayModal::Paused);
    apply(
        &mut app,
        RequestStateTransition::Modal(GameplayModal::Dialogue),
    );
    assert_eq!(current_modal(&app), GameplayModal::Paused);
    apply(&mut app, RequestStateTransition::Modal(GameplayModal::None));
    assert_eq!(current_modal(&app), GameplayModal::None);

    // MainMenu -> PipBoy: modal transitions require AppState::InGame.
    apply(&mut app, RequestStateTransition::App(AppState::MainMenu));
    assert_eq!(current_app_state(&app), AppState::MainMenu);
    apply(
        &mut app,
        RequestStateTransition::Modal(GameplayModal::PipBoy),
    );
    assert_eq!(current_modal(&app), GameplayModal::None);
}

// T35.3: modal round trip; a counter gated on GameplayModal::None does not
// tick while any modal is active, and ticks again after exit.
#[derive(Resource, Default)]
struct NoneOnlyTicks(u32);

fn tick_when_modal_none(mut ticks: ResMut<NoneOnlyTicks>) {
    ticks.0 += 1;
}

#[test]
fn modal_round_trip_gates_a_none_only_counter() {
    let mut app = test_app();
    app.init_resource::<NoneOnlyTicks>().add_systems(
        Update,
        tick_when_modal_none.run_if(in_state(GameplayModal::None)),
    );

    apply(&mut app, RequestStateTransition::App(AppState::Loading));
    apply(&mut app, RequestStateTransition::App(AppState::InGame));
    app.update();
    let baseline = app.world().resource::<NoneOnlyTicks>().0;
    assert!(baseline > 0, "counter should tick while modal is None");

    for modal in [
        GameplayModal::Paused,
        GameplayModal::PipBoy,
        GameplayModal::Dialogue,
        GameplayModal::Console,
    ] {
        apply(&mut app, RequestStateTransition::Modal(modal));
        let before = app.world().resource::<NoneOnlyTicks>().0;
        app.update();
        app.update();
        let after = app.world().resource::<NoneOnlyTicks>().0;
        assert_eq!(
            after, before,
            "counter must not tick while {modal:?} is active"
        );

        apply(&mut app, RequestStateTransition::Modal(GameplayModal::None));
        let before = app.world().resource::<NoneOnlyTicks>().0;
        app.update();
        let after = app.world().resource::<NoneOnlyTicks>().0;
        assert!(
            after > before,
            "counter should resume ticking after exiting {modal:?}"
        );
    }
}

// T35.4: no duplicate execution — a system gated only on AppState::InGame
// (representing an always-on input probe) increments exactly once per
// frame across a full modal round trip, regardless of which modal is active.
#[derive(Resource, Default)]
struct ProbeTicks(u32);

fn probe_system(mut ticks: ResMut<ProbeTicks>) {
    ticks.0 += 1;
}

#[test]
fn probe_system_runs_exactly_once_per_frame_across_modal_round_trip() {
    let mut app = test_app();
    app.init_resource::<ProbeTicks>()
        .add_systems(Update, probe_system.run_if(in_state(AppState::InGame)));

    apply(&mut app, RequestStateTransition::App(AppState::Loading));
    apply(&mut app, RequestStateTransition::App(AppState::InGame));

    let mut expected = app.world().resource::<ProbeTicks>().0;
    let sequence = [
        RequestStateTransition::Modal(GameplayModal::Paused),
        RequestStateTransition::Modal(GameplayModal::None),
        RequestStateTransition::Modal(GameplayModal::PipBoy),
        RequestStateTransition::Modal(GameplayModal::None),
        RequestStateTransition::Modal(GameplayModal::Dialogue),
        RequestStateTransition::Modal(GameplayModal::None),
        RequestStateTransition::Modal(GameplayModal::Console),
        RequestStateTransition::Modal(GameplayModal::None),
    ];
    for request in sequence {
        send(&mut app, request);
        app.update();
        expected += 1;
        assert_eq!(app.world().resource::<ProbeTicks>().0, expected);
        app.update();
        expected += 1;
        assert_eq!(app.world().resource::<ProbeTicks>().0, expected);
    }
}

// T35.5: Paused pauses Time<Virtual>; exiting resumes it.
#[test]
fn paused_modal_pauses_and_resumes_virtual_time() {
    let mut app = test_app();
    apply(&mut app, RequestStateTransition::App(AppState::Loading));
    apply(&mut app, RequestStateTransition::App(AppState::InGame));
    assert!(!app.world().resource::<Time<Virtual>>().is_paused());

    apply(
        &mut app,
        RequestStateTransition::Modal(GameplayModal::Paused),
    );
    assert!(app.world().resource::<Time<Virtual>>().is_paused());

    apply(&mut app, RequestStateTransition::Modal(GameplayModal::None));
    assert!(!app.world().resource::<Time<Virtual>>().is_paused());
}

#[test]
fn console_modal_pauses_and_resumes_virtual_time() {
    let mut app = test_app();
    apply(&mut app, RequestStateTransition::App(AppState::Loading));
    apply(&mut app, RequestStateTransition::App(AppState::InGame));
    apply(
        &mut app,
        RequestStateTransition::Modal(GameplayModal::Console),
    );
    assert!(app.world().resource::<Time<Virtual>>().is_paused());
    apply(&mut app, RequestStateTransition::Modal(GameplayModal::None));
    assert!(!app.world().resource::<Time<Virtual>>().is_paused());
}

#[test]
fn pipboy_modal_pauses_and_resumes_virtual_time() {
    let mut app = test_app();
    apply(&mut app, RequestStateTransition::App(AppState::Loading));
    apply(&mut app, RequestStateTransition::App(AppState::InGame));
    apply(
        &mut app,
        RequestStateTransition::Modal(GameplayModal::PipBoy),
    );
    assert!(app.world().resource::<Time<Virtual>>().is_paused());
    apply(&mut app, RequestStateTransition::Modal(GameplayModal::None));
    assert!(!app.world().resource::<Time<Virtual>>().is_paused());
}

fn tap_key(app: &mut App, key: KeyCode) {
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(key);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
    app.update();
}

#[test]
fn backquote_opens_console_and_escape_closes_it() {
    let mut app = test_app();
    apply(&mut app, RequestStateTransition::App(AppState::Loading));
    apply(&mut app, RequestStateTransition::App(AppState::InGame));
    tap_key(&mut app, KeyCode::Backquote);
    assert_eq!(current_modal(&app), GameplayModal::Console);
    assert!(app.world().resource::<Time<Virtual>>().is_paused());
    tap_key(&mut app, KeyCode::Escape);
    assert_eq!(current_modal(&app), GameplayModal::None);
    assert!(!app.world().resource::<Time<Virtual>>().is_paused());
}

// T35.6: synthetic scenario — Boot -> Loading -> InGame with a fake
// LoadingTarget; OnEnter(InGame) spawn hook fires exactly once.
#[derive(Resource, Default)]
struct SpawnHookCount(u32);

fn spawn_hook(mut count: ResMut<SpawnHookCount>) {
    count.0 += 1;
}

fn drive_to_in_game(app: &mut App) {
    for _ in 0..8 {
        app.update();
        if current_app_state(app) == AppState::InGame {
            break;
        }
    }
}

#[test]
fn boot_to_loading_to_in_game_fires_spawn_hook_exactly_once() {
    let mut app = test_app_with_auto_advance();
    app.init_resource::<SpawnHookCount>()
        .add_systems(OnEnter(AppState::InGame), spawn_hook)
        .insert_resource(LoadingTarget::NewGame {
            manifest: PathBuf::from("fixtures/synthetic.scene.ron"),
        });

    drive_to_in_game(&mut app);
    assert_eq!(current_app_state(&app), AppState::InGame);
    assert_eq!(app.world().resource::<SpawnHookCount>().0, 1);

    // Further updates must not re-fire the hook.
    app.update();
    app.update();
    assert_eq!(app.world().resource::<SpawnHookCount>().0, 1);
}

// T35.7: determinism — two fresh apps driven through Boot -> Loading ->
// InGame report identical initial state (state value + spawn-hook count).
#[test]
fn two_fresh_apps_reach_identical_initial_state() {
    fn build_and_drive() -> (AppState, u32) {
        let mut app = test_app_with_auto_advance();
        app.init_resource::<SpawnHookCount>()
            .add_systems(OnEnter(AppState::InGame), spawn_hook)
            .insert_resource(LoadingTarget::NewGame {
                manifest: PathBuf::from("fixtures/synthetic.scene.ron"),
            });
        drive_to_in_game(&mut app);
        (
            current_app_state(&app),
            app.world().resource::<SpawnHookCount>().0,
        )
    }

    let first = build_and_drive();
    let second = build_and_drive();
    assert_eq!(first, second);
    assert_eq!(first.0, AppState::InGame);
    assert_eq!(first.1, 1);
}
