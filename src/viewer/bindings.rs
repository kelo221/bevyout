//! Viewer bindings whose gameplay implementations are not available yet,
//! plus (issue #98) the item/weapon hotkey bindings that are now real.

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::app_state::{AppState, GameplayModal};

use super::interaction::EquipToggleRequested;
use super::inventory::StackKey;

/// Issue #98 (F98.3): hotkey digits 1-8, each optionally bound to a
/// `StackKey` assigned from the Pip-Boy (`pipboy::handle_equip_and_hotkeys`).
/// Pressing the same digit outside the Pip-Boy (`apply_hotkeys` below)
/// equips whatever is bound to it.
#[derive(Resource, Debug, Default, Clone)]
pub(crate) struct HotkeyBindings([Option<StackKey>; 8]);

impl HotkeyBindings {
    pub(crate) fn get(&self, number: u8) -> Option<StackKey> {
        self.0
            .get(usize::from(number.wrapping_sub(1)))
            .copied()
            .flatten()
    }

    pub(crate) fn assign(&mut self, number: u8, key: StackKey) {
        if let Some(slot) = self.0.get_mut(usize::from(number.wrapping_sub(1))) {
            *slot = Some(key);
        }
    }
}

const HOTKEY_DIGITS: [(KeyCode, u8); 8] = [
    (KeyCode::Digit1, 1),
    (KeyCode::Digit2, 2),
    (KeyCode::Digit3, 3),
    (KeyCode::Digit4, 4),
    (KeyCode::Digit5, 5),
    (KeyCode::Digit6, 6),
    (KeyCode::Digit7, 7),
    (KeyCode::Digit8, 8),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UnsupportedAction {
    RunWalkToggle,
    AlwaysRunToggle,
    AutomaticForwardRun,
    ThirdPersonView,
    AimOrBlock,
    Vats,
}

impl UnsupportedAction {
    fn label(self) -> String {
        match self {
            Self::RunWalkToggle => "run/walk toggle".into(),
            Self::AlwaysRunToggle => "always-run toggle".into(),
            Self::AutomaticForwardRun => "automatic forward run".into(),
            Self::ThirdPersonView => "third-person view".into(),
            Self::AimOrBlock => "aim down sights/block".into(),
            Self::Vats => "V.A.T.S. targeting".into(),
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
        _ => None,
    }
}

pub(crate) struct BindingsPlugin;

impl Plugin for BindingsPlugin {
    fn build(&self, app: &mut App) {
        install(app);
    }
}

fn install(app: &mut App) {
    app.init_resource::<HotkeyBindings>()
        .add_systems(
            Update,
            report_unsupported_bindings
                .in_set(super::plugins::ViewerSet::Input)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameplayModal::None)),
        )
        .add_systems(
            Update,
            apply_hotkeys
                .in_set(super::plugins::ViewerSet::Input)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameplayModal::None)),
        )
        .add_systems(
            Update,
            report_pipboy_flashlight_hold
                .in_set(super::plugins::ViewerSet::Input)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameplayModal::PipBoy)),
        );
}

/// Issue #98 (F98.3): outside the Pip-Boy, pressing a bound hotkey digit
/// equips (or unequips, since equip is a toggle -- see
/// `player::equipment::EquipmentState::toggle`) whatever `StackKey` is
/// bound to it.
fn apply_hotkeys(
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<HotkeyBindings>,
    mut requests: MessageWriter<EquipToggleRequested>,
) {
    for (key_code, number) in HOTKEY_DIGITS {
        if keys.just_pressed(key_code)
            && let Some(bound) = bindings.get(number)
        {
            requests.write(EquipToggleRequested(bound));
        }
    }
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
    ] {
        if keys.just_pressed(key)
            && let Some(action) = keyboard_action(key)
        {
            actions.push(action);
        }
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
#[path = "tests/bindings.rs"]
mod tests;
