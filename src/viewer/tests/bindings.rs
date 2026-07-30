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
    // M5/#237: R is now owned by the weapon reload adapter.
    assert_eq!(keyboard_action(KeyCode::KeyR), None);
}

#[test]
fn unsupported_action_labels_are_stable() {
    assert_eq!(
        UnsupportedAction::ThirdPersonView.label(),
        "third-person view"
    );
    assert_eq!(
        UnsupportedAction::AimOrBlock.label(),
        "aim down sights/block"
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
    assert!(collect_unsupported_actions(&keys, &buttons, false, GameplayModal::None).is_empty());

    keys.press(KeyCode::KeyF);
    assert!(collect_unsupported_actions(&keys, &buttons, false, GameplayModal::Console).is_empty());
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
