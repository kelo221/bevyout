//! Gameplay and diagnostic UI visibility console commands.

use super::*;

pub(super) struct UiCommandProvider;

impl ConsoleCommandProvider for UiCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new(
                "tm",
                "tm",
                "Toggle gameplay UI; the open console remains visible.",
                toggle_game_ui,
            )
            .mutating(),
            ConsoleCommand::new(
                "tdt",
                "tdt",
                "Toggle diagnostic UI entities.",
                toggle_diagnostic_ui,
            )
            .mutating(),
            ConsoleCommand::new(
                "tdi",
                "tdi",
                "Toggle the debug info HUD (player position, active cell, test nav agents).",
                toggle_debug_info,
            )
            .mutating(),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

pub(super) fn toggle_game_ui(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let visible = {
        let mut state = world.resource_mut::<GameUiState>();
        state.visible = !state.visible;
        state.visible
    };
    apply_game_ui_visibility(world);
    Ok(toggle_result(
        json!({ "visible": visible }),
        "Game UI",
        visible,
    ))
}

pub(super) fn toggle_diagnostic_ui(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let visible = {
        let mut state = world.resource_mut::<DiagnosticUiState>();
        state.visible = !state.visible;
        state.visible
    };
    apply_diagnostic_ui_visibility(world);
    Ok(toggle_result(
        json!({ "visible": visible }),
        "Diagnostic UI",
        visible,
    ))
}

/// Issue #151: toggles `diagnostics::DebugInfoState`, mirroring `tdt`'s
/// `toggle_diagnostic_ui` shape exactly (no world side effect beyond the
/// flag; `diagnostics::update_debug_info_hud` change-detects it, issue
/// #268).
pub(super) fn toggle_debug_info(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let enabled = {
        let mut state = world.resource_mut::<diagnostics::DebugInfoState>();
        state.enabled = !state.enabled;
        state.enabled
    };
    Ok(toggle_result(
        json!({ "enabled": enabled }),
        "Debug info",
        enabled,
    ))
}

pub(super) fn console_open(world: &World) -> bool {
    world
        .get_resource::<State<GameplayModal>>()
        .is_some_and(|modal| *modal.get() == GameplayModal::Console)
}

pub(super) fn apply_game_ui_visibility(world: &mut World) {
    let visible = world.resource::<GameUiState>().visible && !console_open(world);
    let mut query = world.query_filtered::<&mut Visibility, With<GameUi>>();
    for mut visibility in query.iter_mut(world) {
        *visibility = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

pub(super) fn apply_diagnostic_ui_visibility(world: &mut World) {
    let visible = world.resource::<DiagnosticUiState>().visible && !console_open(world);
    let mut query = world.query_filtered::<&mut Visibility, With<DiagnosticUi>>();
    for mut visibility in query.iter_mut(world) {
        *visibility = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

pub(super) type UiVisibilityQuery<'w, 's> = Query<
    'w,
    's,
    (
        Option<&'static GameUi>,
        Option<&'static DiagnosticUi>,
        &'static mut Visibility,
    ),
    Or<(With<GameUi>, With<DiagnosticUi>)>,
>;

pub(super) fn sync_ui_visibility(
    modal: Option<Res<State<GameplayModal>>>,
    game: Res<GameUiState>,
    diagnostic: Res<DiagnosticUiState>,
    mut entities: UiVisibilityQuery<'_, '_>,
) {
    let modal_open = modal.is_some_and(|modal| *modal.get() != GameplayModal::None);
    for (game_marker, diagnostic_marker, mut visibility) in &mut entities {
        let visible = !modal_open
            && game_marker.is_none_or(|_| game.visible)
            && diagnostic_marker.is_none_or(|_| diagnostic.visible);
        *visibility = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}
