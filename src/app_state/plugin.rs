use std::path::PathBuf;

use bevy::prelude::*;

/// Top-level application flow (F35.1).
#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub(crate) enum AppState {
    #[default]
    Boot,
    Loading,
    MainMenu,
    InGame,
}

/// Mutually-exclusive gameplay modal, meaningful only while
/// `AppState::InGame` (F35.2).
#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub(crate) enum GameplayModal {
    #[default]
    None,
    Paused,
    Dialogue,
    PipBoy,
    Console,
    /// Issue #52's door-transition fallback: a destination cell that is not
    /// resident-`Ready` shows a minimal loading overlay while its manifest
    /// loads, rather than swapping the cell instantly.
    Loading,
}

/// Single entry point for requesting a state change (F35.3). Requests are
/// validated against a legal-transition table; illegal requests are
/// rejected with a `warn!` diagnostic and leave the current state
/// unchanged (F35.4).
#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub(crate) enum RequestStateTransition {
    App(AppState),
    Modal(GameplayModal),
}

/// The clear `Loading` entry point later save flows hook into (F35.9).
/// `Continue`/`LoadSave` are reserved for a later milestone (New/Continue/
/// LoadSave), not yet wired to any CLI or UI path.
#[derive(Resource, Debug, Clone)]
pub(crate) enum LoadingTarget {
    NewGame {
        manifest: PathBuf,
    },
    #[allow(dead_code)]
    Continue,
    #[allow(dead_code)]
    LoadSave {
        slot: String,
    },
}

/// Legal `AppState` transitions (F35.3). Everything not listed here is
/// rejected.
const LEGAL_APP_TRANSITIONS: &[(AppState, AppState)] = &[
    (AppState::Boot, AppState::Loading),
    (AppState::Loading, AppState::MainMenu),
    (AppState::Loading, AppState::InGame),
    (AppState::MainMenu, AppState::Loading),
    (AppState::InGame, AppState::MainMenu),
];

/// Legal `GameplayModal` transitions (F35.3). Only entered/exited from
/// `None`; modals never transition directly into one another. Also gated
/// on `AppState::InGame` by the validation system below.
const LEGAL_MODAL_TRANSITIONS: &[(GameplayModal, GameplayModal)] = &[
    (GameplayModal::None, GameplayModal::Paused),
    (GameplayModal::Paused, GameplayModal::None),
    (GameplayModal::None, GameplayModal::Dialogue),
    (GameplayModal::Dialogue, GameplayModal::None),
    (GameplayModal::None, GameplayModal::PipBoy),
    (GameplayModal::PipBoy, GameplayModal::None),
    (GameplayModal::None, GameplayModal::Console),
    (GameplayModal::Console, GameplayModal::None),
    (GameplayModal::None, GameplayModal::Loading),
    (GameplayModal::Loading, GameplayModal::None),
];

fn is_legal_app_transition(from: AppState, to: AppState) -> bool {
    LEGAL_APP_TRANSITIONS.contains(&(from, to))
}

fn is_legal_modal_transition(from: GameplayModal, to: GameplayModal) -> bool {
    LEGAL_MODAL_TRANSITIONS.contains(&(from, to))
}

/// Validates every queued `RequestStateTransition` against the legal
/// tables (F35.3) and applies it via `NextState`, or rejects it with a
/// `warn!` diagnostic naming the from/to states (F35.4).
fn apply_state_transition_requests(
    mut requests: MessageReader<RequestStateTransition>,
    app_state: Res<State<AppState>>,
    modal_state: Res<State<GameplayModal>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_modal: ResMut<NextState<GameplayModal>>,
) {
    for request in requests.read() {
        match request {
            RequestStateTransition::App(target) => {
                let current = *app_state.get();
                if is_legal_app_transition(current, *target) {
                    next_app_state.set(*target);
                } else {
                    warn!(
                        "rejected app state transition: {current:?} -> {target:?} is not a legal transition"
                    );
                }
            }
            RequestStateTransition::Modal(target) => {
                let current_app = *app_state.get();
                let current_modal = *modal_state.get();
                if current_app != AppState::InGame {
                    warn!(
                        "rejected modal transition: {current_modal:?} -> {target:?} requires AppState::InGame, but app state is {current_app:?}"
                    );
                } else if is_legal_modal_transition(current_modal, *target) {
                    next_modal.set(*target);
                } else {
                    warn!(
                        "rejected modal transition: {current_modal:?} -> {target:?} is not a legal transition"
                    );
                }
            }
        }
    }
}

/// Boot is transient: request the move to Loading as soon as we observe it
/// (F35.6 auto-advance, no menu stop for the CLI view/render commands).
///
/// Not registered by [`AppStatePlugin`] itself — callers that want CLI-style
/// auto-advance (currently only `viewer::run_view`, and this slice's own
/// synthetic tests) add it explicitly alongside the plugin. Generic
/// transition-table tests that drive `AppState` by hand leave it out so
/// auto-advance cannot race a manually requested transition.
pub(crate) fn auto_advance_from_boot(
    state: Res<State<AppState>>,
    mut requests: MessageWriter<RequestStateTransition>,
) {
    if *state.get() == AppState::Boot {
        requests.write(RequestStateTransition::App(AppState::Loading));
    }
}

/// Loading resolves synchronously today: the `LoadingTarget` describes what
/// to load, and the caller (`viewer::run_view`) has already parsed/staged
/// the prepared manifest into resources before the app starts running,
/// so this system only decides where Loading hands off to (F35.6, F35.9).
/// With no `LoadingTarget` set, Loading lands on the `MainMenu` placeholder
/// (F35.11) — the CLI always sets one, so it never observes this path.
///
/// See [`auto_advance_from_boot`] for why this is opt-in rather than baked
/// into [`AppStatePlugin`].
pub(crate) fn auto_advance_from_loading(
    state: Res<State<AppState>>,
    loading_target: Option<Res<LoadingTarget>>,
    mut requests: MessageWriter<RequestStateTransition>,
) {
    if *state.get() != AppState::Loading {
        return;
    }
    let destination = match loading_target.as_deref() {
        Some(LoadingTarget::NewGame { manifest }) => {
            info!(
                "Loading: starting new game from prepared manifest {}",
                manifest.display()
            );
            AppState::InGame
        }
        Some(LoadingTarget::Continue | LoadingTarget::LoadSave { .. }) => AppState::InGame,
        None => AppState::MainMenu,
    };
    requests.write(RequestStateTransition::App(destination));
}

/// Esc toggles the Paused modal while in game (F35.8). Ignored while
/// another modal is active — only None<->X round trips are legal.
fn toggle_paused_on_escape(
    keys: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    modal_state: Res<State<GameplayModal>>,
    mut requests: MessageWriter<RequestStateTransition>,
) {
    if *app_state.get() != AppState::InGame || !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    let target = match *modal_state.get() {
        GameplayModal::None => GameplayModal::Paused,
        GameplayModal::Paused => GameplayModal::None,
        GameplayModal::Console => GameplayModal::None,
        GameplayModal::Dialogue | GameplayModal::PipBoy | GameplayModal::Loading => return,
    };
    requests.write(RequestStateTransition::Modal(target));
}

/// Tab toggles the PipBoy modal placeholder while in game (F35.8, F35.11:
/// no UI content required by this issue).
fn toggle_pipboy_on_tab(
    keys: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    modal_state: Res<State<GameplayModal>>,
    mut requests: MessageWriter<RequestStateTransition>,
) {
    if *app_state.get() != AppState::InGame || !keys.just_pressed(KeyCode::Tab) {
        return;
    }
    let target = match *modal_state.get() {
        GameplayModal::None => GameplayModal::PipBoy,
        GameplayModal::PipBoy => GameplayModal::None,
        GameplayModal::Paused
        | GameplayModal::Dialogue
        | GameplayModal::Console
        | GameplayModal::Loading => return,
    };
    requests.write(RequestStateTransition::Modal(target));
}

/// Grave/backquote opens and closes the developer console while in game.
fn toggle_console_on_backquote(
    keys: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    modal_state: Res<State<GameplayModal>>,
    mut requests: MessageWriter<RequestStateTransition>,
) {
    if *app_state.get() != AppState::InGame || !keys.just_pressed(KeyCode::Backquote) {
        return;
    }
    let target = match *modal_state.get() {
        GameplayModal::None => GameplayModal::Console,
        GameplayModal::Console => GameplayModal::None,
        GameplayModal::Paused
        | GameplayModal::Dialogue
        | GameplayModal::PipBoy
        | GameplayModal::Loading => return,
    };
    requests.write(RequestStateTransition::Modal(target));
}

/// Logs every applied state change (F35.8 diagnostics); rejected requests
/// are logged by `apply_state_transition_requests`.
fn log_state_transitions(
    mut app_transitions: MessageReader<StateTransitionEvent<AppState>>,
    mut modal_transitions: MessageReader<StateTransitionEvent<GameplayModal>>,
) {
    for transition in app_transitions.read() {
        if transition.exited != transition.entered {
            info!(
                "app state: {:?} -> {:?}",
                transition.exited, transition.entered
            );
        }
    }
    for transition in modal_transitions.read() {
        if transition.exited != transition.entered {
            info!(
                "gameplay modal: {:?} -> {:?}",
                transition.exited, transition.entered
            );
        }
    }
}

/// Paused freezes gameplay time; exiting resumes it (F35.7).
fn pause_virtual_time(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

fn resume_virtual_time(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}

/// Narrow VSA-slice plugin (F35.5): owns the state types, transition
/// validation, modal input, and Time<Virtual> pause/resume. No rendering
/// or gameplay logic lives here — callers gate their own systems with
/// `in_state::<AppState>`/`in_state::<GameplayModal>` (F35.7).
///
/// Requires `StatesPlugin` to already be present in the `App` (included in
/// `DefaultPlugins`; headless callers must add it explicitly alongside
/// `MinimalPlugins`).
///
/// Validation runs in `PostUpdate`, after every `Update`-schedule system —
/// including auto-advance systems callers add themselves (see
/// [`auto_advance_from_boot`]) — so request ordering never depends on
/// cross-module system ordering.
pub(crate) struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_state::<GameplayModal>()
            .add_message::<RequestStateTransition>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(
                Update,
                (
                    toggle_paused_on_escape,
                    toggle_pipboy_on_tab,
                    toggle_console_on_backquote,
                    log_state_transitions,
                ),
            )
            .add_systems(PostUpdate, apply_state_transition_requests)
            .add_systems(OnEnter(GameplayModal::Paused), pause_virtual_time)
            .add_systems(OnExit(GameplayModal::Paused), resume_virtual_time)
            .add_systems(OnEnter(GameplayModal::Console), pause_virtual_time)
            .add_systems(OnExit(GameplayModal::Console), resume_virtual_time)
            .add_systems(OnEnter(GameplayModal::PipBoy), pause_virtual_time)
            .add_systems(OnExit(GameplayModal::PipBoy), resume_virtual_time);
    }
}
