//! Viewer bindings whose gameplay implementations are not available yet.

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::app_state::{AppState, GameplayModal};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UnsupportedAction {
    RunWalkToggle,
    AlwaysRunToggle,
    AutomaticForwardRun,
    ThirdPersonView,
    AttackShoot,
    AimOrBlock,
    Vats,
    Reload,
    Holster,
    Hotkey(u8),
}

impl UnsupportedAction {
    fn label(self) -> String {
        match self {
            Self::RunWalkToggle => "run/walk toggle".into(),
            Self::AlwaysRunToggle => "always-run toggle".into(),
            Self::AutomaticForwardRun => "automatic forward run".into(),
            Self::ThirdPersonView => "third-person view".into(),
            Self::AttackShoot => "attack/shoot".into(),
            Self::AimOrBlock => "aim down sights/block".into(),
            Self::Vats => "V.A.T.S. targeting".into(),
            Self::Reload => "reload".into(),
            Self::Holster => "holster weapon".into(),
            Self::Hotkey(number) => format!("item/weapon hotkey {number}"),
        }
    }
}

fn keyboard_action(key: KeyCode) -> Option<UnsupportedAction> {
    match key {
        KeyCode::ShiftLeft | KeyCode::ShiftRight => Some(UnsupportedAction::RunWalkToggle),
        KeyCode::CapsLock => Some(UnsupportedAction::AlwaysRunToggle),
        KeyCode::KeyX => Some(UnsupportedAction::AutomaticForwardRun),
        KeyCode::KeyF => Some(UnsupportedAction::ThirdPersonView),
        KeyCode::KeyV => Some(UnsupportedAction::Vats),
        KeyCode::KeyR => Some(UnsupportedAction::Reload),
        KeyCode::Digit1 => Some(UnsupportedAction::Hotkey(1)),
        KeyCode::Digit2 => Some(UnsupportedAction::Hotkey(2)),
        KeyCode::Digit3 => Some(UnsupportedAction::Hotkey(3)),
        KeyCode::Digit4 => Some(UnsupportedAction::Hotkey(4)),
        KeyCode::Digit5 => Some(UnsupportedAction::Hotkey(5)),
        KeyCode::Digit6 => Some(UnsupportedAction::Hotkey(6)),
        KeyCode::Digit7 => Some(UnsupportedAction::Hotkey(7)),
        KeyCode::Digit8 => Some(UnsupportedAction::Hotkey(8)),
        _ => None,
    }
}

pub(crate) fn install(app: &mut App) {
    app.add_systems(
        Update,
        report_unsupported_bindings
            .run_if(in_state(AppState::InGame))
            .run_if(in_state(GameplayModal::None)),
    )
    .add_systems(
        Update,
        report_pipboy_flashlight_hold
            .run_if(in_state(AppState::InGame))
            .run_if(in_state(GameplayModal::PipBoy)),
    );
}

fn report_unsupported_bindings(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    modal: Res<State<GameplayModal>>,
    windows: Query<&CursorOptions, With<PrimaryWindow>>,
) {
    let captured = windows
        .single()
        .is_ok_and(|options| options.grab_mode == CursorGrabMode::Locked);
    for action in collect_unsupported_actions(&keys, &buttons, captured, *modal.get()) {
        report(action);
    }
}

fn collect_unsupported_actions(
    keys: &ButtonInput<KeyCode>,
    buttons: &ButtonInput<MouseButton>,
    captured: bool,
    modal: GameplayModal,
) -> Vec<UnsupportedAction> {
    if modal != GameplayModal::None {
        return Vec::new();
    }

    let mut actions = Vec::new();
    for key in [
        KeyCode::ShiftLeft,
        KeyCode::ShiftRight,
        KeyCode::CapsLock,
        KeyCode::KeyX,
        KeyCode::KeyF,
        KeyCode::KeyV,
        KeyCode::KeyR,
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
    ] {
        if keys.just_pressed(key)
            && let Some(action) = keyboard_action(key)
        {
            actions.push(action);
        }
    }
    if keys.just_released(KeyCode::KeyR) {
        actions.push(UnsupportedAction::Holster);
    }

    if captured && buttons.just_pressed(MouseButton::Left) {
        actions.push(UnsupportedAction::AttackShoot);
    }
    if captured && buttons.just_pressed(MouseButton::Right) {
        actions.push(UnsupportedAction::AimOrBlock);
    }
    actions
}

fn report_pipboy_flashlight_hold(keys: Res<ButtonInput<KeyCode>>, mut reported: Local<bool>) {
    if !keys.pressed(KeyCode::Tab) {
        *reported = false;
        return;
    }
    if !*reported {
        info!("NOT_IMPLEMENTED: Pip-Boy flashlight");
        *reported = true;
    }
}

fn report(action: UnsupportedAction) {
    info!("NOT_IMPLEMENTED: {}", action.label());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_bindings_cover_requested_placeholder_actions() {
        assert_eq!(
            keyboard_action(KeyCode::ShiftLeft),
            Some(UnsupportedAction::RunWalkToggle)
        );
        assert_eq!(
            keyboard_action(KeyCode::CapsLock),
            Some(UnsupportedAction::AlwaysRunToggle)
        );
        assert_eq!(
            keyboard_action(KeyCode::KeyX),
            Some(UnsupportedAction::AutomaticForwardRun)
        );
        assert_eq!(
            keyboard_action(KeyCode::KeyF),
            Some(UnsupportedAction::ThirdPersonView)
        );
        assert_eq!(
            keyboard_action(KeyCode::KeyV),
            Some(UnsupportedAction::Vats)
        );
        assert_eq!(
            keyboard_action(KeyCode::Digit8),
            Some(UnsupportedAction::Hotkey(8))
        );
    }

    #[test]
    fn unsupported_action_labels_are_stable() {
        assert_eq!(UnsupportedAction::Hotkey(3).label(), "item/weapon hotkey 3");
        assert_eq!(
            UnsupportedAction::ThirdPersonView.label(),
            "third-person view"
        );
    }

    #[test]
    fn unsupported_actions_are_edge_triggered_and_modal_gated() {
        let mut keys = ButtonInput::<KeyCode>::default();
        let buttons = ButtonInput::<MouseButton>::default();
        keys.press(KeyCode::KeyF);
        assert_eq!(
            collect_unsupported_actions(&keys, &buttons, false, GameplayModal::None),
            vec![UnsupportedAction::ThirdPersonView]
        );
        keys.clear();
        assert!(
            collect_unsupported_actions(&keys, &buttons, false, GameplayModal::None).is_empty()
        );

        keys.press(KeyCode::KeyF);
        assert!(
            collect_unsupported_actions(&keys, &buttons, false, GameplayModal::Console).is_empty()
        );
    }
}
