use super::*;

fn ui_app() -> App {
    let mut app = App::new();
    app.add_systems(Startup, spawn_console_ui);
    app.update();
    app
}

#[test]
fn toggle_characters_are_scrubbed_without_losing_the_draft() {
    assert_eq!(sanitize_console_draft("setpos z 2`~"), "setpos z 2");
    assert_eq!(
        sanitize_console_draft("legitimate draft"),
        "legitimate draft"
    );
}

#[test]
fn input_filter_accepts_commands_and_rejects_console_toggles_and_newlines() {
    assert!("setpos z 2".chars().all(console_input_character_allowed));
    assert!(!console_input_character_allowed('`'));
    assert!(!console_input_character_allowed('~'));
    assert!(!console_input_character_allowed('\n'));
    assert!(!console_input_character_allowed('\r'));
}

#[test]
fn selected_reference_uses_label_and_fallout_hex_format() {
    assert_eq!(
        selected_reference_text(Some(0x7b240), Some("VaultDoorRef"), Some("Vault Door")),
        "VaultDoorRef (0007b240)"
    );
    assert_eq!(
        selected_reference_text(Some(0x7b240), None, Some("Vault Door")),
        "Vault Door (0007b240)"
    );
    assert_eq!(
        selected_reference_text(Some(0x7b240), None, None),
        "(0007b240)"
    );
    assert_eq!(selected_reference_text(None, Some("ignored"), None), "");
}

#[test]
fn mouse_wheel_cycles_front_to_back_and_wraps() {
    assert_eq!(next_pick_index(Some(0), 3, 1.0), Some(1));
    assert_eq!(next_pick_index(Some(2), 3, 1.0), Some(0));
    assert_eq!(next_pick_index(Some(0), 3, -1.0), Some(2));
    assert_eq!(next_pick_index(None, 3, 1.0), Some(0));
    assert_eq!(next_pick_index(None, 3, -1.0), Some(2));
    assert_eq!(next_pick_index(Some(0), 0, 1.0), None);
    assert_eq!(next_pick_index(Some(0), 3, 0.0), None);
}

#[test]
fn console_pick_ray_collects_hits_behind_the_nearest_mesh() {
    let settings = console_pick_settings();
    assert!(!(settings.early_exit_test)(Entity::PLACEHOLDER));
}

#[test]
fn machine_value_fallback_stays_on_one_line() {
    let value = serde_json::json!({ "camera_mode": "free" });
    assert_eq!(compact_console_value(&value), r#"{"camera_mode":"free"}"#);
    assert!(!compact_console_value(&value).contains('\n'));
}

#[test]
fn console_layout_is_transparent_empty_and_bottom_aligned() {
    let mut app = ui_app();
    let world = app.world_mut();
    let root = {
        let mut query = world.query_filtered::<(Entity, &Node), With<ConsoleRoot>>();
        let (entity, node) = query.single(world).unwrap();
        assert_eq!(node.left, px(0));
        assert_eq!(node.right, px(0));
        assert_eq!(node.top, px(0));
        assert_eq!(node.bottom, px(0));
        entity
    };
    assert_eq!(
        world.get::<BackgroundColor>(root),
        Some(&BackgroundColor::default())
    );

    let mut input =
        world.query_filtered::<(&Node, &EditableText, &EditableTextFilter), With<ConsoleInput>>();
    let (node, editable, _) = input.single(world).unwrap();
    assert_eq!(node.left, px(40));
    assert_eq!(node.bottom, px(10));
    assert_eq!(node.width, percent(80));
    assert_eq!(node.overflow.x, OverflowAxis::Clip);
    assert_eq!(editable.max_characters, Some(2048));
    assert_eq!(editable.visible_width, Some(100.0));
    assert!(editable_value(editable).is_empty());

    let mut viewport = world.query_filtered::<
        (&Node, &ScrollPosition, &ChildOf),
        (With<ConsoleScrollbackViewport>, With<ScrollArea>),
    >();
    let (node, position, parent) = viewport.single(world).unwrap();
    assert_eq!(parent.parent(), root);
    assert_eq!(node.left, px(40));
    assert_eq!(node.bottom, px(INPUT_STRIP_HEIGHT));
    assert_eq!(node.width, percent(80));
    assert_eq!(node.height, percent(50));
    assert_eq!(node.overflow.y, OverflowAxis::Scroll);
    assert_eq!(position.0, Vec2::ZERO);

    let mut scrollback = world.query_filtered::<&Text, With<ConsoleScrollback>>();
    assert!(scrollback.single(world).unwrap().0.is_empty());
    let mut title = world.query_filtered::<&Text, With<ConsoleTitle>>();
    assert!(title.single(world).unwrap().0.is_empty());
}

#[test]
fn close_scrubs_toggle_characters_and_recaptures_cursor() {
    let mut app = ui_app();
    app.init_resource::<InputFocus>()
        .add_systems(Update, close_console_ui);
    app.world_mut().spawn((Window::default(), PrimaryWindow));
    {
        let world = app.world_mut();
        let mut query = world.query_filtered::<&mut EditableText, With<ConsoleInput>>();
        replace_editable(&mut query.single_mut(world).unwrap(), "draft`~ stays");
    }
    app.update();

    let world = app.world_mut();
    let mut input = world.query_filtered::<&EditableText, With<ConsoleInput>>();
    assert_eq!(editable_value(input.single(world).unwrap()), "draft stays");
    let mut cursor = world.query_filtered::<&CursorOptions, With<PrimaryWindow>>();
    let cursor = cursor.single(world).unwrap();
    assert!(!cursor.visible);
    assert_eq!(cursor.grab_mode, CursorGrabMode::Locked);
}
