use super::*;
use bevy::asset::AssetPlugin;
use bevy::state::app::StatesPlugin;
use std::time::Duration;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, AssetPlugin::default()))
        .init_asset::<Image>()
        .init_state::<GameplayModal>()
        .insert_resource(PlayerInventory::default())
        .insert_resource(PlayerEquipment::default())
        .insert_resource(HotkeyBindings::default())
        .insert_resource(ButtonInput::<MouseButton>::default())
        .insert_resource(ButtonInput::<KeyCode>::default())
        .insert_resource(PreparedItemCatalog::default())
        .add_message::<EquipToggleRequested>()
        .init_resource::<InteractionNotice>();
    install(&mut app);
    app
}

#[test]
fn pipboy_round_trip_releases_and_recaptures_pointer() {
    let mut app = test_app();
    let window = app
        .world_mut()
        .spawn((CursorOptions::default(), PrimaryWindow))
        .id();
    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::PipBoy);
    app.update();
    let cursor = app.world().entity(window).get::<CursorOptions>().unwrap();
    assert!(cursor.visible);
    assert_eq!(cursor.grab_mode, CursorGrabMode::None);
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<PipBoyRoot>>()
            .iter(app.world())
            .count(),
        1
    );

    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::None);
    app.update();
    let cursor = app.world().entity(window).get::<CursorOptions>().unwrap();
    assert!(!cursor.visible);
    assert_eq!(cursor.grab_mode, CursorGrabMode::Locked);
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<PipBoyRoot>>()
            .iter(app.world())
            .count(),
        0
    );
}

#[test]
fn pipboy_uses_a_centered_crt_device_hierarchy() {
    let mut app = test_app();
    app.world_mut()
        .spawn((CursorOptions::default(), PrimaryWindow));
    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::PipBoy);
    app.update();

    for (label, count) in [
        (
            "device shell",
            app.world_mut()
                .query_filtered::<Entity, With<PipBoyDevice>>()
                .iter(app.world())
                .count(),
        ),
        (
            "CRT screen",
            app.world_mut()
                .query_filtered::<Entity, With<PipBoyScreen>>()
                .iter(app.world())
                .count(),
        ),
        (
            "screen header",
            app.world_mut()
                .query_filtered::<Entity, With<PipBoyHeader>>()
                .iter(app.world())
                .count(),
        ),
        (
            "screen footer",
            app.world_mut()
                .query_filtered::<Entity, With<PipBoyFooter>>()
                .iter(app.world())
                .count(),
        ),
        (
            "physical button bank",
            app.world_mut()
                .query_filtered::<Entity, With<PipBoyButtonBank>>()
                .iter(app.world())
                .count(),
        ),
    ] {
        assert_eq!(count, 1, "expected one {label}");
    }

    let screen = app
        .world_mut()
        .query_filtered::<&Node, With<PipBoyScreen>>()
        .single(app.world())
        .expect("the CRT screen should have layout");
    assert_eq!(screen.aspect_ratio, Some(4.0 / 3.0));
    assert_eq!(screen.max_width, Val::Px(1040.0));
}

#[test]
fn status_figure_part_selection_wraps_in_both_directions() {
    let mut editor = StatusFigureEditor::default();
    assert_eq!(editor.selected, StatusBodyPart::Head);

    editor.select_previous();
    assert_eq!(editor.selected, StatusBodyPart::RightLeg);

    editor.select_next();
    assert_eq!(editor.selected, StatusBodyPart::Head);
    editor.select_next();
    assert_eq!(editor.selected, StatusBodyPart::Face);
}

#[test]
fn status_figure_movement_changes_only_the_selected_part() {
    let mut layout = StatusFigureLayout::default();
    let original_head = layout.part(StatusBodyPart::Head);
    let original_arm = layout.part(StatusBodyPart::LeftArm);

    layout.move_part(StatusBodyPart::LeftArm, -10, 1);

    assert_eq!(layout.part(StatusBodyPart::Head), original_head);
    assert_eq!(
        layout.part(StatusBodyPart::LeftArm),
        StatusPartLayout {
            left: original_arm.left - 10,
            top: original_arm.top + 1,
            ..original_arm
        }
    );
}

#[test]
fn status_figure_repeat_waits_then_ticks_at_a_fixed_interval() {
    let mut repeat = StatusMoveRepeat::default();
    assert_eq!(
        repeat.steps(MoveDirection::Right, true, true, Duration::ZERO),
        1
    );
    assert_eq!(
        repeat.steps(
            MoveDirection::Right,
            true,
            false,
            Duration::from_millis(299)
        ),
        0
    );
    assert_eq!(
        repeat.steps(MoveDirection::Right, true, false, Duration::from_millis(51)),
        2,
        "the 300 ms initial repeat and 50 ms interval should both fire"
    );
    assert_eq!(
        repeat.steps(
            MoveDirection::Right,
            false,
            false,
            Duration::from_millis(500)
        ),
        0
    );
    assert_eq!(repeat.direction, None);
}

#[test]
fn status_figure_clipboard_block_is_complete_and_stable() {
    let mut layout = StatusFigureLayout::default();
    layout.move_part(StatusBodyPart::Face, 3, -2);

    let copied = layout.clipboard_text();
    let layouts = [
        (173, 45, 123, 133),
        (188, 61, 70, 93),
        (148, 113, 148, 186),
        (250, 133, 145, 75),
        (51, 125, 139, 78),
        (214, 204, 104, 162),
        (116, 210, 122, 162),
    ];
    for (left, top, width, height) in layouts {
        assert_eq!(
            copied
                .matches(&format!(
                    "StatusPartLayout::new({left}, {top}, {width}, {height})"
                ))
                .count(),
            1,
            "the layout should appear exactly once in the copied block"
        );
    }
    assert!(
        copied.starts_with("const STATUS_PART_LAYOUTS: [StatusPartLayout; 7] = ["),
        "the result should be ready to paste back into the defaults: {copied}"
    );
}

#[test]
fn status_figure_debug_toggle_is_stats_only_and_preserves_layout() {
    let mut app = test_app();
    open_pipboy(&mut app);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::F1);
    app.update();
    assert!(app.world().resource::<StatusFigureEditor>().enabled);

    app.world_mut()
        .resource_mut::<StatusFigureLayout>()
        .move_part(StatusBodyPart::Head, 4, 5);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
    app.world_mut().resource_mut::<PipBoyState>().view = PipBoyView::Items;
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::F1);
    app.update();

    assert!(
        app.world().resource::<StatusFigureEditor>().enabled,
        "F1 must not toggle the editor from Items"
    );
    assert_eq!(
        app.world()
            .resource::<StatusFigureLayout>()
            .part(StatusBodyPart::Head)
            .left,
        StatusFigureLayout::default()
            .part(StatusBodyPart::Head)
            .left
            + 4
    );
}

#[test]
fn status_figure_arrow_input_moves_one_or_ten_pixels() {
    let mut app = test_app();
    open_pipboy(&mut app);
    let original = app
        .world()
        .resource::<StatusFigureLayout>()
        .part(StatusBodyPart::Head);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::F1);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::ArrowRight);
    app.update();
    let after_single = app
        .world()
        .resource::<StatusFigureLayout>()
        .part(StatusBodyPart::Head);
    assert_eq!(after_single.left, original.left + 1);
    assert_eq!(after_single.top, original.top);

    {
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.release(KeyCode::ArrowRight);
        keys.clear();
        keys.press(KeyCode::ShiftLeft);
        keys.press(KeyCode::ArrowDown);
    }
    app.update();
    let after_large = app
        .world()
        .resource::<StatusFigureLayout>()
        .part(StatusBodyPart::Head);
    assert_eq!(after_large.left, original.left + 1);
    assert_eq!(after_large.top, original.top + 10);
}

#[test]
fn rebuilt_stats_figure_uses_the_runtime_layout() {
    let mut app = test_app();
    app.world_mut()
        .resource_mut::<StatusFigureLayout>()
        .move_part(StatusBodyPart::RightLeg, 17, -9);
    open_pipboy(&mut app);

    let expected = app
        .world()
        .resource::<StatusFigureLayout>()
        .part(StatusBodyPart::RightLeg);
    let node = app
        .world_mut()
        .query::<(&StatusFigurePart, &Node)>()
        .iter(app.world())
        .find(|(part, _)| part.0 == StatusBodyPart::RightLeg)
        .map(|(_, node)| node)
        .expect("the adjusted right leg should be spawned");
    assert_eq!(node.left, Val::Px(expected.left as f32));
    assert_eq!(node.top, Val::Px(expected.top as f32));
    assert_eq!(node.width, Val::Px(expected.width as f32));
    assert_eq!(node.height, Val::Px(expected.height as f32));
}

// -- issue #99 (F99.1/F99.2): consumable use from the Items view --

fn aid_item(base_form_id: u32, quest_item: bool) -> PreparedItemDefinition {
    PreparedItemDefinition {
        base_form_id,
        record_kind: "ALCH".into(),
        category: PreparedItemCategory::Aid,
        editor_id: Some("Stimpak".into()),
        display_name: Some("Stimpak".into()),
        source_model_path: None,
        icon_asset_path: None,
        world_asset_path: None,
        physics_asset_path: None,
        drop_collider: Default::default(),
        value: None,
        weight: None,
        quest_item,
        stats: PreparedItemStats::Aid {
            effects: vec![crate::vsa::PreparedItemEffect {
                form_id: 0x99,
                label: "Restore Health".into(),
            }],
        },
        audio: Default::default(),
    }
}

fn aid_test_app(quest_item: bool) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, AssetPlugin::default()))
        .init_asset::<Image>()
        .init_state::<GameplayModal>()
        .insert_resource(PlayerInventory::from_stack_states([
            super::super::inventory::InventoryStack {
                base_form_id: 0x77,
                count: 3,
                condition: None,
            },
        ]))
        .insert_resource(PlayerEquipment::default())
        .insert_resource(HotkeyBindings::default())
        .insert_resource(ButtonInput::<MouseButton>::default())
        .insert_resource(ButtonInput::<KeyCode>::default())
        .insert_resource(PreparedItemCatalog {
            revision: "test".into(),
            source_fingerprint: "test".into(),
            items: vec![aid_item(0x77, quest_item)],
        })
        .add_message::<EquipToggleRequested>();
    app.world_mut()
        .spawn((CursorOptions::default(), PrimaryWindow));
    install(&mut app);
    let snapshot = app.world().resource::<PlayerInventory>().legacy_snapshot();
    app.world_mut()
        .resource_mut::<CanonicalItemLedger>()
        .sync_player(&snapshot)
        .unwrap();
    use bevyout_core::actor_state::ActorValue;
    use bevyout_core::effects::{
        CONDITION_FUNCTION_HAS_PERK, CONDITION_OPER_EQUAL, IngestibleCondition,
        IngestibleDefinition, IngestibleEffect,
    };
    app.world_mut()
        .resource_mut::<EffectCatalog>()
        .ingestibles
        .insert(
            0x77,
            IngestibleDefinition {
                form_id: 0x77,
                editor_id: "Stimpak".into(),
                effects: vec![
                    IngestibleEffect {
                        magnitude: 30.0,
                        actor_value: Some(ActorValue::Health),
                        condition: Some(IngestibleCondition {
                            oper: CONDITION_OPER_EQUAL,
                            comparison_value: 0.0,
                            function: CONDITION_FUNCTION_HAS_PERK,
                            param1: 0x0009_4ebf,
                        }),
                        ..IngestibleEffect::default()
                    },
                    IngestibleEffect {
                        magnitude: 36.0,
                        actor_value: Some(ActorValue::Health),
                        condition: Some(IngestibleCondition {
                            oper: CONDITION_OPER_EQUAL,
                            comparison_value: 1.0,
                            function: CONDITION_FUNCTION_HAS_PERK,
                            param1: 0x0009_4ebf,
                        }),
                        ..IngestibleEffect::default()
                    },
                ],
                ..IngestibleDefinition::default()
            },
        );
    app.world_mut().spawn((
        FpsPlayer::default(),
        ActorStats::default(),
        PlayerVitals {
            current_health: 140.0,
        },
        PlayerRadiation::default(),
        ActiveEffectsList::default(),
        Addictions::default(),
    ));
    app.world_mut().resource_mut::<PipBoyState>().view = PipBoyView::Items;
    app.world_mut().resource_mut::<PipBoyState>().category = PreparedItemCategory::Aid;
    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::PipBoy);
    app.update();
    app
}

#[test]
fn using_an_aid_stack_decrements_the_authoritative_inventory() {
    let mut app = aid_test_app(false);
    let button = app
        .world_mut()
        .query::<(Entity, &ItemActionButton)>()
        .iter(app.world())
        .find_map(|(entity, action)| matches!(action, ItemActionButton::Use(_)).then_some(entity))
        .expect("an Aid stack should render a USE button");
    *app.world_mut().get_mut::<Interaction>(button).unwrap() = Interaction::Pressed;
    app.update();

    assert_eq!(app.world().resource::<PlayerInventory>().count(0x77), 2);
    let canonical_count = app
        .world()
        .resource::<CanonicalItemLedger>()
        .ledger
        .holders()
        .get(&bevyout_core::item_transaction::HolderId::Player)
        .unwrap()
        .items
        .iter()
        .map(|item| item.count)
        .sum::<u32>();
    assert_eq!(canonical_count, 2);
    let health = app
        .world_mut()
        .query::<&PlayerVitals>()
        .single(app.world())
        .unwrap()
        .current_health;
    assert_eq!(health, 170.0);
    assert_eq!(
        app.world().resource::<InteractionNotice>().text(),
        "Used Stimpak: Restore Health"
    );
}

#[test]
fn a_quest_flagged_aid_stack_renders_no_use_button() {
    let mut app = aid_test_app(true);
    assert_eq!(
        app.world_mut()
            .query::<&ItemActionButton>()
            .iter(app.world())
            .count(),
        0
    );
    assert_eq!(app.world().resource::<PlayerInventory>().count(0x77), 3);
}

fn seed_item(app: &mut App, base_form_id: u32, category: PreparedItemCategory, name: &str) {
    app.world_mut().resource_mut::<PlayerInventory>().add_stack(
        super::super::inventory::InventoryStack {
            base_form_id,
            count: 1,
            condition: None,
        },
    );
    app.world_mut()
        .resource_mut::<PreparedItemCatalog>()
        .items
        .push(PreparedItemDefinition {
            base_form_id,
            record_kind: "WEAP".into(),
            category,
            editor_id: None,
            display_name: Some(name.into()),
            source_model_path: None,
            icon_asset_path: None,
            world_asset_path: None,
            physics_asset_path: None,
            drop_collider: Default::default(),
            value: None,
            weight: None,
            quest_item: false,
            stats: PreparedItemStats::Apparel {
                armor_rating: None,
                max_condition: None,
                biped_slot_mask: Some(1),
            },
            audio: Default::default(),
        });
}

// -- equip toggle and hotkeys (issue #98, F98.3) -----------------------

#[test]
fn pressing_e_writes_an_equip_toggle_request_for_the_selected_row() {
    let mut app = test_app();
    seed_item(&mut app, 1, PreparedItemCategory::Apparel, "Test Armor");
    app.world_mut().resource_mut::<PipBoyState>().view = PipBoyView::Items;
    app.world_mut().resource_mut::<PipBoyState>().category = PreparedItemCategory::Apparel;
    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::PipBoy);
    app.update();
    let selected = app
        .world()
        .resource::<PipBoyState>()
        .selected
        .expect("a row should be auto-selected");
    assert_eq!(selected.base_form_id, 1);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyE);
    app.update();
    let messages = app.world().resource::<Messages<EquipToggleRequested>>();
    let request = messages
        .iter_current_update_messages()
        .next()
        .expect("expected an EquipToggleRequested message");
    assert_eq!(request.0, selected);
}

#[test]
fn pressing_a_digit_binds_the_selected_row_to_that_hotkey_slot() {
    let mut app = test_app();
    seed_item(&mut app, 2, PreparedItemCategory::Weapons, "Test Rifle");
    app.world_mut().resource_mut::<PipBoyState>().view = PipBoyView::Items;
    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::PipBoy);
    app.update();
    let selected = app
        .world()
        .resource::<PipBoyState>()
        .selected
        .expect("a row should be auto-selected");
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::Digit3);
    app.update();
    assert_eq!(
        app.world().resource::<HotkeyBindings>().get(3),
        Some(selected)
    );
}

#[test]
fn ineligible_categories_do_not_bind_hotkeys_or_equip() {
    let mut app = test_app();
    seed_item(&mut app, 3, PreparedItemCategory::Misc, "Junk");
    app.world_mut().resource_mut::<PipBoyState>().view = PipBoyView::Items;
    app.world_mut().resource_mut::<PipBoyState>().category = PreparedItemCategory::Misc;
    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::PipBoy);
    app.update();
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::Digit1);
    app.update();
    assert_eq!(app.world().resource::<HotkeyBindings>().get(1), None);
    assert_eq!(
        app.world()
            .resource::<Messages<EquipToggleRequested>>()
            .iter_current_update_messages()
            .count(),
        0
    );
}

#[test]
fn equipped_rows_show_the_equipped_marker() {
    use super::super::player::equipment::EquipKind;
    let mut app = test_app();
    seed_item(&mut app, 4, PreparedItemCategory::Apparel, "Test Armor");
    let key = StackKey {
        base_form_id: 4,
        condition: None,
    };
    app.world_mut()
        .resource_mut::<PlayerEquipment>()
        .toggle(key, EquipKind::Apparel { biped_slot_mask: 1 })
        .unwrap();
    app.world_mut().resource_mut::<PipBoyState>().view = PipBoyView::Items;
    app.world_mut().resource_mut::<PipBoyState>().category = PreparedItemCategory::Apparel;
    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::PipBoy);
    app.update();
    let texts: Vec<String> = app
        .world_mut()
        .query::<&Text>()
        .iter(app.world())
        .map(|text| text.0.clone())
        .collect();
    assert!(
        texts.iter().any(|text| text == "[E] Test Armor"),
        "expected an equipped marker, got {texts:?}"
    );
}

// -- issue #100: Data tab, Notes view-model, Notes/World views ---------

fn stats_item(
    base_form_id: u32,
    name: &str,
    stats: PreparedItemStats,
    quest_item: bool,
) -> PreparedItemDefinition {
    PreparedItemDefinition {
        base_form_id,
        record_kind: "NOTE".into(),
        category: PreparedItemCategory::Misc,
        editor_id: None,
        display_name: Some(name.into()),
        source_model_path: None,
        icon_asset_path: None,
        world_asset_path: None,
        physics_asset_path: None,
        drop_collider: Default::default(),
        value: None,
        weight: None,
        quest_item,
        stats,
        audio: Default::default(),
    }
}

fn stack(base_form_id: u32, count: i32) -> super::super::inventory::InventoryStack {
    super::super::inventory::InventoryStack {
        base_form_id,
        count,
        condition: None,
    }
}

#[test]
fn notes_rows_select_only_readable_stacks_sorted_by_name() {
    let inventory = PlayerInventory::from_stack_states([
        stack(1, 1), // note with text, name sorts last
        stack(2, 2), // book with text, name sorts first (case-insensitively)
        stack(3, 1), // textless note: inert, filtered out
        stack(4, 1), // aid: usable not readable, filtered out
        stack(5, 1), // quest-flagged note with text: still readable
        stack(6, 1), // uncataloged: filtered out
    ]);
    let catalog = PreparedItemCatalog {
        revision: "test".into(),
        source_fingerprint: "test".into(),
        items: vec![
            stats_item(
                1,
                "zebra note",
                PreparedItemStats::Note {
                    text: Some("z".into()),
                },
                false,
            ),
            stats_item(
                2,
                "Alpha book",
                PreparedItemStats::Book {
                    flags: None,
                    text: Some("a".into()),
                },
                false,
            ),
            stats_item(
                3,
                "empty note",
                PreparedItemStats::Note { text: None },
                false,
            ),
            aid_item(4, false),
            stats_item(
                5,
                "quest note",
                PreparedItemStats::Note {
                    text: Some("q".into()),
                },
                true,
            ),
        ],
    };
    let rows: Vec<u32> = notes_rows(&inventory, &catalog)
        .iter()
        .map(|(stack, _)| stack.base_form_id)
        .collect();
    assert_eq!(rows, [2, 5, 1]);
}

#[test]
fn world_lines_report_cell_identity_and_play_time() {
    let cell = CellInfo {
        form_id: 0x0001_51e3,
        editor_id: Some("MegatonPlayerHouse".into()),
        name: Some("My Megaton House".into()),
        interior: true,
        behave_like_exterior: false,
        ambient_rgba: [0.0; 4],
        directional_rgba: [0.0; 4],
        image_space_form_id: None,
        image_space: None,
        lighting_template_form_id: None,
        lighting_template_flags: 0,
        lighting_template: None,
        raw_lighting: None,
        effective_lighting: None,
        water_form_id: None,
        water_height: None,
        grid: None,
        worldspace_form_id: None,
        day_night_profile: None,
        day_night_preview_profile: None,
    };
    assert_eq!(
        world_lines(Some((&cell, 7)), 3661.0),
        [
            "CELL  My Megaton House",
            "LOC   MegatonPlayerHouse (000151e3)",
            "INTERIOR",
            "PLACEMENTS  7",
            "PLAY TIME  1:01:01",
        ]
    );
}

#[test]
fn world_lines_without_a_session_fall_back() {
    assert_eq!(
        world_lines(None, 0.0),
        ["NO ACTIVE CELL", "PLAY TIME  0:00:00"]
    );
}

fn seed_note(app: &mut App, base_form_id: u32, name: &str, text: &str) {
    app.world_mut()
        .resource_mut::<PlayerInventory>()
        .add_stack(stack(base_form_id, 1));
    app.world_mut()
        .resource_mut::<PreparedItemCatalog>()
        .items
        .push(stats_item(
            base_form_id,
            name,
            PreparedItemStats::Note {
                text: Some(text.into()),
            },
            false,
        ));
}

fn open_pipboy(app: &mut App) {
    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::PipBoy);
    app.update();
}

fn press_view_tab(app: &mut App, view: PipBoyView) {
    let entity = app
        .world_mut()
        .query::<(Entity, &ViewTab)>()
        .iter(app.world())
        .find_map(|(entity, tab)| (tab.0 == view).then_some(entity))
        .expect("the view tab should be spawned");
    *app.world_mut().get_mut::<Interaction>(entity).unwrap() = Interaction::Pressed;
    app.update();
}

fn press_data_section_tab(app: &mut App, section: DataSection) {
    let entity = app
        .world_mut()
        .query::<(Entity, &DataSectionTab)>()
        .iter(app.world())
        .find_map(|(entity, tab)| (tab.0 == section).then_some(entity))
        .expect("the data section tab should be spawned");
    *app.world_mut().get_mut::<Interaction>(entity).unwrap() = Interaction::Pressed;
    app.update();
}

#[test]
fn data_tab_shows_the_notes_list() {
    let mut app = test_app();
    seed_note(&mut app, 0x21, "Keller Family Transcript", "tape text");
    seed_item(&mut app, 1, PreparedItemCategory::Apparel, "Test Armor");
    open_pipboy(&mut app);
    press_view_tab(&mut app, PipBoyView::Data);
    let notes: Vec<u32> = app
        .world_mut()
        .query::<&NoteRow>()
        .iter(app.world())
        .map(|row| row.0)
        .collect();
    assert_eq!(notes, [0x21]);
    assert_eq!(
        app.world_mut()
            .query::<&ItemRow>()
            .iter(app.world())
            .count(),
        0,
        "the Items surface should be replaced while Data is showing"
    );
}

#[test]
fn activating_a_note_row_requests_the_reader() {
    let mut app = test_app();
    seed_note(&mut app, 0x21, "Keller Family Transcript", "tape text");
    open_pipboy(&mut app);
    press_view_tab(&mut app, PipBoyView::Data);
    let row = app
        .world_mut()
        .query::<(Entity, &NoteRow)>()
        .iter(app.world())
        .find_map(|(entity, row)| (row.0 == 0x21).then_some(entity))
        .expect("the note row should be spawned");
    *app.world_mut().get_mut::<Interaction>(row).unwrap() = Interaction::Pressed;
    app.update();
    let messages = app.world().resource::<Messages<OpenReaderRequested>>();
    let request = messages
        .iter_current_update_messages()
        .next()
        .expect("expected an OpenReaderRequested message");
    assert_eq!(request.base_form_id, 0x21);
}

#[test]
fn world_section_shows_the_session_summary() {
    let mut app = test_app();
    open_pipboy(&mut app);
    press_view_tab(&mut app, PipBoyView::Data);
    press_data_section_tab(&mut app, DataSection::World);
    let texts: Vec<String> = app
        .world_mut()
        .query::<&Text>()
        .iter(app.world())
        .map(|text| text.0.clone())
        .collect();
    // The bare harness has no prepared scene manifest, so the World
    // section falls back; cell rendering itself is covered by the pure
    // `world_lines` test above.
    assert!(
        texts.iter().any(|text| text == "NO ACTIVE CELL"),
        "expected the no-session line, got {texts:?}"
    );
    assert!(
        texts.iter().any(|text| text.starts_with("PLAY TIME  ")),
        "expected a play-time line, got {texts:?}"
    );
}

#[test]
fn items_view_still_works_after_a_data_round_trip() {
    let mut app = test_app();
    seed_item(&mut app, 1, PreparedItemCategory::Apparel, "Test Armor");
    app.world_mut().resource_mut::<PipBoyState>().category = PreparedItemCategory::Apparel;
    open_pipboy(&mut app);
    press_view_tab(&mut app, PipBoyView::Data);
    // Equip/hotkey input must be inert while Data is showing.
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyE);
    app.update();
    assert_eq!(
        app.world()
            .resource::<Messages<EquipToggleRequested>>()
            .iter_current_update_messages()
            .count(),
        0,
        "E must not equip from the Data view"
    );
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset_all();
    press_view_tab(&mut app, PipBoyView::Items);
    let rows: Vec<StackKey> = app
        .world_mut()
        .query::<&ItemRow>()
        .iter(app.world())
        .map(|row| row.0)
        .collect();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].base_form_id, 1);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyE);
    app.update();
    assert_eq!(
        app.world()
            .resource::<Messages<EquipToggleRequested>>()
            .iter_current_update_messages()
            .count(),
        1,
        "E should equip again once back on Items"
    );
}

// -- issue #121 (F121.1/F121.2): a row click triggers its primary action --

/// A minimal cataloged item that carries no primary action of its own
/// (`Misc` category, `Misc` stats) unless the caller overrides
/// `category` -- `row_primary_action` only reads `category` for the
/// equip check, so this is enough to exercise Weapons/Apparel/Ammo too.
fn category_item(base_form_id: u32, category: PreparedItemCategory) -> PreparedItemDefinition {
    PreparedItemDefinition {
        base_form_id,
        record_kind: "MISC".into(),
        category,
        editor_id: None,
        display_name: Some(format!("Item {base_form_id:08X}")),
        source_model_path: None,
        icon_asset_path: None,
        world_asset_path: None,
        physics_asset_path: None,
        drop_collider: Default::default(),
        value: None,
        weight: None,
        quest_item: false,
        stats: PreparedItemStats::Misc,
        audio: Default::default(),
    }
}

#[test]
fn row_primary_action_equips_weapons_apparel_and_ammo() {
    let key = StackKey {
        base_form_id: 1,
        condition: None,
    };
    for category in [
        PreparedItemCategory::Weapons,
        PreparedItemCategory::Apparel,
        PreparedItemCategory::Ammo,
    ] {
        let item = category_item(1, category);
        assert_eq!(
            row_primary_action(key, &item),
            Some(RowPrimaryAction::Equip(key)),
            "category {category:?} should be equip-eligible"
        );
    }
}

#[test]
fn row_primary_action_uses_a_non_quest_aid_stack() {
    let key = StackKey {
        base_form_id: 0x77,
        condition: None,
    };
    assert_eq!(
        row_primary_action(key, &aid_item(0x77, false)),
        Some(RowPrimaryAction::Use(key))
    );
}

#[test]
fn row_primary_action_is_none_for_a_quest_flagged_aid_stack() {
    let key = StackKey {
        base_form_id: 0x77,
        condition: None,
    };
    assert_eq!(row_primary_action(key, &aid_item(0x77, true)), None);
}

#[test]
fn row_primary_action_reads_a_book_with_text() {
    let key = StackKey {
        base_form_id: 5,
        condition: None,
    };
    let item = stats_item(
        5,
        "Alpha book",
        PreparedItemStats::Book {
            flags: None,
            text: Some("a".into()),
        },
        false,
    );
    assert_eq!(
        row_primary_action(key, &item),
        Some(RowPrimaryAction::Read(5))
    );
}

#[test]
fn row_primary_action_is_none_for_a_textless_book() {
    let key = StackKey {
        base_form_id: 5,
        condition: None,
    };
    let item = stats_item(
        5,
        "empty",
        PreparedItemStats::Book {
            flags: None,
            text: None,
        },
        false,
    );
    assert_eq!(row_primary_action(key, &item), None);
}

#[test]
fn row_primary_action_is_none_for_key_and_misc() {
    let key = StackKey {
        base_form_id: 9,
        condition: None,
    };
    assert_eq!(
        row_primary_action(key, &stats_item(9, "key", PreparedItemStats::Key, false)),
        None
    );
    assert_eq!(
        row_primary_action(key, &category_item(9, PreparedItemCategory::Misc)),
        None
    );
}

fn press_item_row(app: &mut App, key: StackKey) {
    let entity = app
        .world_mut()
        .query::<(Entity, &ItemRow)>()
        .iter(app.world())
        .find_map(|(entity, row)| (row.0 == key).then_some(entity))
        .expect("the item row should be spawned");
    *app.world_mut().get_mut::<Interaction>(entity).unwrap() = Interaction::Pressed;
    app.update();
}

#[test]
fn clicking_an_equip_eligible_row_writes_an_equip_toggle_request_and_selects_it() {
    let mut app = test_app();
    seed_item(&mut app, 1, PreparedItemCategory::Apparel, "Alpha Armor");
    seed_item(&mut app, 2, PreparedItemCategory::Apparel, "Beta Armor");
    app.world_mut().resource_mut::<PipBoyState>().view = PipBoyView::Items;
    app.world_mut().resource_mut::<PipBoyState>().category = PreparedItemCategory::Apparel;
    open_pipboy(&mut app);
    let key_2 = StackKey {
        base_form_id: 2,
        condition: None,
    };
    // "Alpha Armor" (base_form_id 1) sorts first, so normalize_selection
    // auto-selects it; clicking the *other* row exercises the
    // select-and-act path in one click.
    assert_ne!(app.world().resource::<PipBoyState>().selected, Some(key_2));
    press_item_row(&mut app, key_2);
    assert_eq!(
        app.world().resource::<PipBoyState>().selected,
        Some(key_2),
        "a click selects the row"
    );
    let request = app
        .world()
        .resource::<Messages<EquipToggleRequested>>()
        .iter_current_update_messages()
        .next()
        .expect("expected an EquipToggleRequested message");
    assert_eq!(request.0, key_2);
}

#[test]
fn clicking_an_already_selected_row_still_triggers_its_action() {
    let mut app = test_app();
    seed_item(&mut app, 1, PreparedItemCategory::Apparel, "Test Armor");
    app.world_mut().resource_mut::<PipBoyState>().view = PipBoyView::Items;
    app.world_mut().resource_mut::<PipBoyState>().category = PreparedItemCategory::Apparel;
    open_pipboy(&mut app);
    let key = StackKey {
        base_form_id: 1,
        condition: None,
    };
    assert_eq!(app.world().resource::<PipBoyState>().selected, Some(key));
    // Two real frames apart, so this counts cumulative `Messages::len()`
    // rather than `iter_current_update_messages()`: with `MinimalPlugins`
    // the message-buffer swap is gated by a `FixedUpdate` signal that
    // doesn't necessarily fire on every `app.update()`, so two writes in
    // two separate frames aren't guaranteed to land in fresh per-frame
    // windows -- `len()` (messages_a + messages_b) doesn't depend on that
    // swap and is still exactly "how many were written so far".
    press_item_row(&mut app, key);
    assert_eq!(
        app.world()
            .resource::<Messages<EquipToggleRequested>>()
            .len(),
        1,
        "the first click on an already-selected row still equips"
    );
    press_item_row(&mut app, key);
    assert_eq!(
        app.world()
            .resource::<Messages<EquipToggleRequested>>()
            .len(),
        2,
        "a second click toggles again"
    );
}

#[test]
fn clicking_an_aid_row_consumes_it_through_use_item() {
    let mut app = aid_test_app(false);
    let key = StackKey {
        base_form_id: 0x77,
        condition: None,
    };
    press_item_row(&mut app, key);
    assert_eq!(app.world().resource::<PlayerInventory>().count(0x77), 2);
    assert_eq!(
        app.world().resource::<InteractionNotice>().text(),
        "Used Stimpak: Restore Health"
    );
}

#[test]
fn clicking_a_row_with_no_primary_action_only_selects() {
    let mut app = test_app();
    seed_item(&mut app, 3, PreparedItemCategory::Misc, "Junk");
    app.world_mut().resource_mut::<PipBoyState>().view = PipBoyView::Items;
    app.world_mut().resource_mut::<PipBoyState>().category = PreparedItemCategory::Misc;
    open_pipboy(&mut app);
    let key = StackKey {
        base_form_id: 3,
        condition: None,
    };
    press_item_row(&mut app, key);
    assert_eq!(app.world().resource::<PipBoyState>().selected, Some(key));
    assert_eq!(
        app.world()
            .resource::<Messages<EquipToggleRequested>>()
            .iter_current_update_messages()
            .count(),
        0
    );
    assert_eq!(
        app.world()
            .resource::<Messages<OpenReaderRequested>>()
            .iter_current_update_messages()
            .count(),
        0
    );
}

// -- Stats view, header stat bar, and the bezel button bank -------------

fn screen_texts(app: &mut App) -> Vec<String> {
    app.world_mut()
        .query::<&Text>()
        .iter(app.world())
        .map(|text| text.0.clone())
        .collect()
}

#[test]
fn stat_segments_format_level_vitals_and_xp() {
    let status = PlayerStatus {
        name: "A".into(),
        level: 2,
        hp_current: 159,
        hp_max: 210,
        ap_current: 85,
        ap_max: 85,
        xp_current: 263,
        xp_next: 550,
    };
    assert_eq!(
        stat_segments(&status),
        [
            ("LVL", "2".to_string()),
            ("HP", "159/210".to_string()),
            ("AP", "85/85".to_string()),
            ("XP", "263/550".to_string()),
        ]
    );
}

/// An Aid-category catalog item with a custom display name (`aid_item`
/// is always "Stimpak").
fn named_aid_item(base_form_id: u32, name: &str) -> PreparedItemDefinition {
    let mut item = aid_item(base_form_id, false);
    item.display_name = Some(name.into());
    item
}

#[test]
fn quick_aid_line_prefers_stimpak_named_stacks() {
    let inventory = PlayerInventory::from_stack_states([stack(1, 9), stack(2, 13)]);
    let catalog = PreparedItemCatalog {
        revision: "test".into(),
        source_fingerprint: "test".into(),
        items: vec![named_aid_item(1, "Buffout"), named_aid_item(2, "Stimpak")],
    };
    // Even though Buffout has the larger stack for the count fallback,
    // the Stimpak name wins.
    assert_eq!(
        quick_aid_line(&inventory, &catalog),
        Some("(13) Stimpak".to_string())
    );
}

#[test]
fn quick_aid_line_falls_back_to_the_largest_aid_stack() {
    let inventory = PlayerInventory::from_stack_states([stack(1, 2), stack(2, 5), stack(3, 7)]);
    let catalog = PreparedItemCatalog {
        revision: "test".into(),
        source_fingerprint: "test".into(),
        items: vec![
            named_aid_item(1, "Bandages"),
            named_aid_item(2, "Med-X"),
            // Not an Aid stack: never eligible for the quick-use line.
            stats_item(3, "Wrench", PreparedItemStats::Misc, false),
        ],
    };
    assert_eq!(
        quick_aid_line(&inventory, &catalog),
        Some("(5) Med-X".to_string())
    );
}

#[test]
fn quick_aid_line_is_none_without_aid_stacks() {
    let inventory = PlayerInventory::from_stack_states([stack(1, 3)]);
    let catalog = PreparedItemCatalog {
        revision: "test".into(),
        source_fingerprint: "test".into(),
        items: vec![stats_item(1, "Wrench", PreparedItemStats::Misc, false)],
    };
    assert_eq!(quick_aid_line(&inventory, &catalog), None);
}

#[test]
fn opening_the_pipboy_shows_the_stats_status_screen() {
    let mut app = test_app();
    app.world_mut()
        .resource_mut::<PlayerInventory>()
        .add_stack(stack(0x77, 13));
    app.world_mut()
        .resource_mut::<PreparedItemCatalog>()
        .items
        .push(aid_item(0x77, false));
    open_pipboy(&mut app);
    assert_eq!(
        app.world().resource::<PipBoyState>().view,
        PipBoyView::Stats
    );
    let texts = screen_texts(&mut app);
    for expected in [
        "LVL",
        "HP",
        "AP",
        "XP",
        "CND",
        "RAD",
        "EFF",
        "Status",
        "Player - Level 1",
        "(13) Stimpak",
    ] {
        assert!(
            texts.iter().any(|text| text == expected),
            "expected '{expected}' on the status screen, got {texts:?}"
        );
    }
    // The bezel button bank renders one button per top-level view.
    assert_eq!(
        app.world_mut()
            .query::<&ViewTab>()
            .iter(app.world())
            .count(),
        3
    );
    assert!(
        texts.iter().filter(|text| text.as_str() == "STATS").count() >= 2,
        "the header label and the bezel button should both read STATS, got {texts:?}"
    );
}

#[test]
fn bezel_buttons_switch_between_all_three_views() {
    let mut app = test_app();
    open_pipboy(&mut app);
    press_view_tab(&mut app, PipBoyView::Items);
    assert_eq!(
        app.world().resource::<PipBoyState>().view,
        PipBoyView::Items
    );
    press_view_tab(&mut app, PipBoyView::Data);
    assert_eq!(app.world().resource::<PipBoyState>().view, PipBoyView::Data);
    press_view_tab(&mut app, PipBoyView::Stats);
    assert_eq!(
        app.world().resource::<PipBoyState>().view,
        PipBoyView::Stats
    );
}

#[test]
fn items_footer_shows_the_carry_weight() {
    let mut app = test_app();
    open_pipboy(&mut app);
    assert!(
        !screen_texts(&mut app)
            .iter()
            .any(|text| text.starts_with("WG ")),
        "the Stats view has no carry-weight readout"
    );
    press_view_tab(&mut app, PipBoyView::Items);
    assert!(
        screen_texts(&mut app)
            .iter()
            .any(|text| text.starts_with("WG ")),
        "the Items footer should carry the WG readout"
    );
}
