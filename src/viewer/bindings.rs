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
    AttackShoot,
    AimOrBlock,
    Vats,
    Reload,
    Holster,
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
        KeyCode::KeyR,
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
        // Issue #98: hotkeys 1-8 are real actions now, not placeholders.
        assert_eq!(keyboard_action(KeyCode::Digit8), None);
    }

    #[test]
    fn unsupported_action_labels_are_stable() {
        assert_eq!(
            UnsupportedAction::ThirdPersonView.label(),
            "third-person view"
        );
        assert_eq!(UnsupportedAction::Holster.label(), "holster weapon");
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

    // -- hotkey bindings (issue #98, F98.3) --------------------------------

    fn stack_key(base_form_id: u32) -> StackKey {
        StackKey {
            base_form_id,
            condition: None,
        }
    }

    #[test]
    fn hotkey_bindings_assign_and_get_round_trip_by_slot_number() {
        let mut bindings = HotkeyBindings::default();
        assert_eq!(bindings.get(1), None);
        bindings.assign(1, stack_key(5));
        bindings.assign(8, stack_key(9));
        assert_eq!(bindings.get(1), Some(stack_key(5)));
        assert_eq!(bindings.get(8), Some(stack_key(9)));
        assert_eq!(bindings.get(2), None);
        // Rebinding a slot overwrites it.
        bindings.assign(1, stack_key(6));
        assert_eq!(bindings.get(1), Some(stack_key(6)));
    }

    #[test]
    fn hotkey_bindings_ignore_out_of_range_slot_numbers() {
        let mut bindings = HotkeyBindings::default();
        bindings.assign(0, stack_key(5));
        bindings.assign(9, stack_key(5));
        assert_eq!(bindings.get(0), None);
        assert_eq!(bindings.get(9), None);
    }

    fn hotkey_test_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
            .init_state::<AppState>()
            .init_state::<GameplayModal>()
            .insert_resource(ButtonInput::<KeyCode>::default())
            .insert_resource(ButtonInput::<MouseButton>::default())
            .add_message::<EquipToggleRequested>();
        install(&mut app);
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update();
        app
    }

    #[test]
    fn pressing_a_bound_hotkey_outside_pipboy_writes_an_equip_toggle_request() {
        let mut app = hotkey_test_app();
        let key = stack_key(7);
        app.world_mut()
            .resource_mut::<HotkeyBindings>()
            .assign(4, key);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Digit4);
        app.update();
        let messages = app.world().resource::<Messages<EquipToggleRequested>>();
        let request = messages
            .iter_current_update_messages()
            .next()
            .expect("expected an EquipToggleRequested message");
        assert_eq!(request.0, key);
    }

    #[test]
    fn pressing_an_unbound_hotkey_writes_no_request() {
        let mut app = hotkey_test_app();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Digit4);
        app.update();
        assert_eq!(
            app.world()
                .resource::<Messages<EquipToggleRequested>>()
                .iter_current_update_messages()
                .count(),
            0
        );
    }

    #[test]
    fn hotkeys_do_not_fire_while_the_pipboy_is_open() {
        let mut app = hotkey_test_app();
        let key = stack_key(7);
        app.world_mut()
            .resource_mut::<HotkeyBindings>()
            .assign(4, key);
        app.world_mut()
            .resource_mut::<NextState<GameplayModal>>()
            .set(GameplayModal::PipBoy);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Digit4);
        app.update();
        assert_eq!(
            app.world()
                .resource::<Messages<EquipToggleRequested>>()
                .iter_current_update_messages()
                .count(),
            0
        );
    }
}
