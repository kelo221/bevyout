//! Flat Pip-Boy Items surface for M3 wave 1 (issue #71).

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::app_state::GameplayModal;

use super::audio::PlaySound;
use super::bindings::HotkeyBindings;
use super::interaction::{
    EquipToggleRequested, InteractionNotice, PlayerEquipment, PlayerInventory, item_rules, item_use,
};
use super::inventory::{DropAction, StackKey, TransferResult, drop_action};
use super::pipboy_reader::OpenReaderRequested;
use super::{PreparedItemCatalog, PreparedItemCategory, PreparedItemDefinition, PreparedItemStats};

/// F98.3: hotkey digits 1-8, in display order, paired with their `HotkeyBindings` slot number.
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

fn is_equip_eligible(category: PreparedItemCategory) -> bool {
    matches!(
        category,
        PreparedItemCategory::Weapons | PreparedItemCategory::Apparel | PreparedItemCategory::Ammo
    )
}

const GREEN: Color = Color::srgb(0.18, 1.0, 0.48);
const GREEN_DIM: Color = Color::srgba(0.08, 0.45, 0.22, 0.85);
const SCREEN: Color = Color::srgba(0.005, 0.025, 0.012, 0.97);

#[derive(Message, Clone, Copy, Debug)]
pub(crate) struct DropInventoryStackRequested {
    pub(crate) key: StackKey,
    pub(crate) count: i32,
}

#[derive(Resource, Debug)]
struct PipBoyState {
    category: PreparedItemCategory,
    selected: Option<StackKey>,
}

impl Default for PipBoyState {
    fn default() -> Self {
        Self {
            category: PreparedItemCategory::Weapons,
            selected: None,
        }
    }
}

#[derive(Resource, Clone, Copy, Debug)]
struct QuantityPicker {
    key: StackKey,
    quantity: i32,
    max: i32,
}

#[derive(Component)]
struct PipBoyRoot;

#[derive(Component, Clone, Copy)]
struct ItemRow(StackKey);

#[derive(Component, Clone, Copy)]
struct CategoryTab(PreparedItemCategory);

#[derive(Component)]
struct QuantityOverlay;

#[derive(Component)]
struct QuantityText;

#[derive(Component, Clone, Copy)]
enum QuantityButton {
    Minus,
    Plus,
    Confirm,
    Cancel,
}

/// F99.2/F99.3: the details pane's contextual action button, shown only for
/// a selected stack that `item_use::classify` calls `Use` (Aid) or `Read`
/// (Book/Note with text) -- `Inert` stacks (Key/Misc, textless Book/Note, a
/// quest-flagged Aid item) show no button.
#[derive(Component, Clone, Copy)]
enum ItemActionButton {
    Use(StackKey),
    Read(u32),
}

pub(crate) fn install(app: &mut App) {
    app.init_resource::<PipBoyState>()
        // `handle_item_action_button`'s dependencies, normally registered by
        // `interaction`/`audio`/`pipboy_reader`'s installs -- `init_resource`
        // and `add_message` are both no-ops when already registered, so this
        // only matters for the self-contained test harness below.
        .init_resource::<InteractionNotice>()
        .add_message::<PlaySound>()
        .add_message::<OpenReaderRequested>()
        .add_message::<DropInventoryStackRequested>()
        .add_systems(OnEnter(GameplayModal::PipBoy), enter_pipboy)
        .add_systems(OnExit(GameplayModal::PipBoy), exit_pipboy)
        .add_systems(
            Update,
            (
                handle_item_rows,
                handle_category_tabs,
                handle_quantity_buttons,
                handle_item_action_button,
                handle_equip_and_hotkeys,
                refresh_after_inventory_change,
            )
                .run_if(in_state(GameplayModal::PipBoy)),
        );
}

/// F98.3: while the Items view has a row selected, `E` toggles equip/unequip
/// for eligible rows (Weapons/Apparel/Ammo) and digits 1-8 bind that row to a
/// hotkey slot -- the "simple, discoverable" Pip-Boy assignment interaction;
/// pressing the same digit outside the Pip-Boy (`bindings::apply_hotkeys`)
/// equips whatever is bound to it.
fn handle_equip_and_hotkeys(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<PipBoyState>,
    catalog: Res<PreparedItemCatalog>,
    mut equip_requests: MessageWriter<EquipToggleRequested>,
    mut hotkeys: ResMut<HotkeyBindings>,
    mut notice: ResMut<InteractionNotice>,
) {
    let Some(selected) = state.selected else {
        return;
    };
    let Some(item) = catalog
        .items
        .iter()
        .find(|item| item.base_form_id == selected.base_form_id)
    else {
        return;
    };
    if !is_equip_eligible(item.category) {
        return;
    }
    if keys.just_pressed(KeyCode::KeyE) {
        equip_requests.write(EquipToggleRequested(selected));
    }
    for (key_code, number) in HOTKEY_DIGITS {
        if keys.just_pressed(key_code) {
            hotkeys.assign(number, selected);
            notice.show(format!("Bound hotkey {number}"));
        }
    }
}

fn enter_pipboy(
    mut commands: Commands,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
    inventory: Res<PlayerInventory>,
    equipment: Res<PlayerEquipment>,
    catalog: Res<PreparedItemCatalog>,
    assets: Res<AssetServer>,
    mut state: ResMut<PipBoyState>,
) {
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
    normalize_selection(&mut state, &inventory, &catalog);
    spawn_screen(
        &mut commands,
        &inventory,
        &equipment,
        &catalog,
        &assets,
        &state,
    );
}

fn exit_pipboy(
    mut commands: Commands,
    roots: Query<Entity, With<PipBoyRoot>>,
    overlays: Query<Entity, With<QuantityOverlay>>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn();
    }
    for entity in &overlays {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<QuantityPicker>();
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }
}

#[allow(clippy::too_many_arguments)]
fn refresh_after_inventory_change(
    mut commands: Commands,
    inventory: Res<PlayerInventory>,
    equipment: Res<PlayerEquipment>,
    catalog: Res<PreparedItemCatalog>,
    assets: Res<AssetServer>,
    mut state: ResMut<PipBoyState>,
    roots: Query<Entity, With<PipBoyRoot>>,
    picker: Option<Res<QuantityPicker>>,
) {
    if (!inventory.is_changed() && !equipment.is_changed()) || picker.is_some() {
        return;
    }
    normalize_selection(&mut state, &inventory, &catalog);
    rebuild(
        &mut commands,
        &roots,
        &inventory,
        &equipment,
        &catalog,
        &assets,
        &state,
    );
}

#[allow(clippy::too_many_arguments)]
fn handle_item_rows(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    rows: Query<(&Interaction, &ItemRow), Changed<Interaction>>,
    all_rows: Query<(&Interaction, &ItemRow)>,
    inventory: Res<PlayerInventory>,
    equipment: Res<PlayerEquipment>,
    catalog: Res<PreparedItemCatalog>,
    assets: Res<AssetServer>,
    mut state: ResMut<PipBoyState>,
    roots: Query<Entity, With<PipBoyRoot>>,
    picker: Option<Res<QuantityPicker>>,
    mut drops: MessageWriter<DropInventoryStackRequested>,
) {
    for (interaction, row) in &rows {
        if *interaction == Interaction::Pressed && state.selected != Some(row.0) {
            state.selected = Some(row.0);
            rebuild(
                &mut commands,
                &roots,
                &inventory,
                &equipment,
                &catalog,
                &assets,
                &state,
            );
            return;
        }
    }
    if !mouse.just_pressed(MouseButton::Right) || picker.is_some() {
        return;
    }
    let Some(row) = all_rows
        .iter()
        .find_map(|(interaction, row)| (*interaction == Interaction::Hovered).then_some(row.0))
    else {
        return;
    };
    state.selected = Some(row);
    let count = inventory
        .stack_states()
        .into_iter()
        .find(|stack| stack.key() == row)
        .map_or(0, |stack| stack.count);
    match drop_action(count) {
        Some(DropAction::DropOne) => {
            drops.write(DropInventoryStackRequested { key: row, count: 1 });
        }
        Some(DropAction::ChooseQuantity { max, default, .. }) => {
            commands.insert_resource(QuantityPicker {
                key: row,
                quantity: default,
                max,
            });
            spawn_quantity_picker(&mut commands, default, max);
        }
        None => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_category_tabs(
    mut commands: Commands,
    tabs: Query<(&Interaction, &CategoryTab), Changed<Interaction>>,
    inventory: Res<PlayerInventory>,
    equipment: Res<PlayerEquipment>,
    catalog: Res<PreparedItemCatalog>,
    assets: Res<AssetServer>,
    mut state: ResMut<PipBoyState>,
    roots: Query<Entity, With<PipBoyRoot>>,
) {
    for (interaction, tab) in &tabs {
        if *interaction == Interaction::Pressed && state.category != tab.0 {
            state.category = tab.0;
            state.selected = None;
            normalize_selection(&mut state, &inventory, &catalog);
            rebuild(
                &mut commands,
                &roots,
                &inventory,
                &equipment,
                &catalog,
                &assets,
                &state,
            );
            return;
        }
    }
}

fn handle_quantity_buttons(
    mut commands: Commands,
    buttons: Query<(&Interaction, &QuantityButton), Changed<Interaction>>,
    mut picker: Option<ResMut<QuantityPicker>>,
    mut texts: Query<&mut Text, With<QuantityText>>,
    overlays: Query<Entity, With<QuantityOverlay>>,
    mut drops: MessageWriter<DropInventoryStackRequested>,
) {
    let Some(ref mut picker) = picker else {
        return;
    };
    for (interaction, button) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match button {
            QuantityButton::Minus => picker.quantity = (picker.quantity - 1).max(1),
            QuantityButton::Plus => picker.quantity = (picker.quantity + 1).min(picker.max),
            QuantityButton::Confirm => {
                drops.write(DropInventoryStackRequested {
                    key: picker.key,
                    count: picker.quantity,
                });
                close_quantity_picker(&mut commands, &overlays);
                return;
            }
            QuantityButton::Cancel => {
                close_quantity_picker(&mut commands, &overlays);
                return;
            }
        }
        for mut text in &mut texts {
            text.0 = format!("{} / {}", picker.quantity, picker.max);
        }
    }
}

fn close_quantity_picker(commands: &mut Commands, overlays: &Query<Entity, With<QuantityOverlay>>) {
    for entity in overlays {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<QuantityPicker>();
}

/// F99.2 (use) / F99.3 (read): the details pane's single contextual action
/// button. Using an Aid stack drives the existing `Inventory::remove` op
/// through the `item_use` pure classification, plays its prepared pickup
/// sound as the stand-in "use" audio (the manifest has no dedicated use-sound
/// field yet), and posts a notice naming the item and its effect labels
/// (magnitudes are out of scope, see issue #99). Reading just forwards to
/// `pipboy_reader`'s public seam -- the stack is never touched.
fn handle_item_action_button(
    buttons: Query<(&Interaction, &ItemActionButton), Changed<Interaction>>,
    mut inventory: ResMut<PlayerInventory>,
    catalog: Res<PreparedItemCatalog>,
    mut notice: ResMut<InteractionNotice>,
    mut sounds: MessageWriter<PlaySound>,
    mut reader_requests: MessageWriter<OpenReaderRequested>,
) {
    for (interaction, action) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match *action {
            ItemActionButton::Use(key) => {
                use_item(key, &mut inventory, &catalog, &mut notice, &mut sounds)
            }
            ItemActionButton::Read(base_form_id) => {
                reader_requests.write(OpenReaderRequested { base_form_id });
            }
        }
    }
}

fn use_item(
    key: StackKey,
    inventory: &mut PlayerInventory,
    catalog: &PreparedItemCatalog,
    notice: &mut InteractionNotice,
    sounds: &mut MessageWriter<PlaySound>,
) {
    let Some(item) = catalog
        .items
        .iter()
        .find(|item| item.base_form_id == key.base_form_id)
    else {
        return;
    };
    if item_use::classify(item_use_stats(&item.stats), item.quest_item)
        != item_use::ItemUseAction::Use
    {
        return;
    }
    if !matches!(
        inventory.remove(key, item_use::USE_CONSUMES_COUNT),
        TransferResult::Applied { .. }
    ) {
        return;
    }
    if let Some(form_id) = item.audio.pickup_sound_form_id {
        sounds.write(PlaySound {
            form_id,
            position: None,
        });
    }
    let name = item_name(item);
    let effects = match &item.stats {
        PreparedItemStats::Aid { effects } => effects
            .iter()
            .map(|effect| effect.label.clone())
            .collect::<Vec<_>>()
            .join(", "),
        _ => String::new(),
    };
    notice.show(if effects.is_empty() {
        format!("Used {name}")
    } else {
        format!("Used {name}: {effects}")
    });
    info!("used {:08x} ({name})", key.base_form_id);
}

/// F99.2/F99.3: the details pane's contextual button label/action for a
/// selected stack, or `None` for an `Inert` one (no button rendered).
fn item_action_button(
    key: StackKey,
    item: &PreparedItemDefinition,
) -> Option<(&'static str, ItemActionButton)> {
    match item_use::classify(item_use_stats(&item.stats), item.quest_item) {
        item_use::ItemUseAction::Use => Some(("USE", ItemActionButton::Use(key))),
        item_use::ItemUseAction::Read => Some(("READ", ItemActionButton::Read(item.base_form_id))),
        item_use::ItemUseAction::Inert => None,
    }
}

/// Mirrors the relevant part of `PreparedItemStats` into `item_use`'s
/// dependency-free `ItemStats` (see that module's doc comment for why it
/// keeps its own local type rather than depending on this Bevy-touching
/// one).
fn item_use_stats(stats: &PreparedItemStats) -> item_use::ItemStats {
    match stats {
        PreparedItemStats::Aid { .. } => item_use::ItemStats::Aid,
        PreparedItemStats::Book { text, .. } => item_use::ItemStats::Book {
            has_text: text.as_deref().is_some_and(|text| !text.is_empty()),
        },
        PreparedItemStats::Note { text } => item_use::ItemStats::Note {
            has_text: text.as_deref().is_some_and(|text| !text.is_empty()),
        },
        PreparedItemStats::Key => item_use::ItemStats::Key,
        PreparedItemStats::Misc => item_use::ItemStats::Misc,
        PreparedItemStats::Weapon { .. }
        | PreparedItemStats::Apparel { .. }
        | PreparedItemStats::Ammo { .. } => item_use::ItemStats::Other,
    }
}

#[allow(clippy::too_many_arguments)]
fn rebuild(
    commands: &mut Commands,
    roots: &Query<Entity, With<PipBoyRoot>>,
    inventory: &PlayerInventory,
    equipment: &PlayerEquipment,
    catalog: &PreparedItemCatalog,
    assets: &AssetServer,
    state: &PipBoyState,
) {
    for root in roots {
        commands.entity(root).despawn();
    }
    spawn_screen(commands, inventory, equipment, catalog, assets, state);
}

fn spawn_screen(
    commands: &mut Commands,
    inventory: &PlayerInventory,
    equipment: &PlayerEquipment,
    catalog: &PreparedItemCatalog,
    assets: &AssetServer,
    state: &PipBoyState,
) {
    let mut rows = visible_rows(inventory, catalog, state.category);
    rows.sort_by(|(a_stack, a), (b_stack, b)| {
        item_name(a)
            .to_ascii_lowercase()
            .cmp(&item_name(b).to_ascii_lowercase())
            .then(a_stack.base_form_id.cmp(&b_stack.base_form_id))
    });
    let selected_item = state.selected.and_then(|key| {
        rows.iter()
            .find(|(stack, _)| stack.key() == key)
            .map(|(stack, item)| (*stack, *item))
    });
    let weight = inventory
        .total_weight(|form_id| {
            catalog
                .items
                .iter()
                .find(|item| item.base_form_id == form_id)
                .and_then(|item| item_rules::carried_weight(item.quest_item, item.weight))
        })
        .max(0.0);
    commands
        .spawn((
            PipBoyRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(48.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(SCREEN),
            GlobalZIndex(500),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new(format!(
                    "ITEMS                                      WG {weight:.1}"
                )),
                TextColor(GREEN),
                TextFont {
                    font_size: FontSize::Px(32.0),
                    ..default()
                },
                Node {
                    height: Val::Px(56.0),
                    border: UiRect::bottom(Val::Px(2.0)),
                    ..default()
                },
                BorderColor::all(GREEN_DIM),
            ));
            root.spawn(Node {
                flex_grow: 1.0,
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            })
            .with_children(|body| {
                body.spawn(Node {
                    width: Val::Percent(52.0),
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::scroll_y(),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .with_children(|list| {
                    if rows.is_empty() {
                        list.spawn((Text::new("No items"), TextColor(GREEN_DIM)));
                    }
                    for (stack, item) in &rows {
                        let key = stack.key();
                        let selected = state.selected == Some(key);
                        list.spawn((
                            Button,
                            ItemRow(key),
                            Node {
                                min_height: Val::Px(42.0),
                                width: Val::Percent(100.0),
                                padding: UiRect::horizontal(Val::Px(10.0)),
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(if selected { 2.0 } else { 0.0 })),
                                ..default()
                            },
                            BackgroundColor(if selected { GREEN_DIM } else { Color::NONE }),
                            BorderColor::all(GREEN),
                        ))
                        .with_child((
                            Text::new(format!(
                                "{}{}{}",
                                equipped_marker(equipment.is_equipped(key)),
                                item_name(item),
                                count_suffix(stack.count)
                            )),
                            TextColor(GREEN),
                            TextFont {
                                font_size: FontSize::Px(24.0),
                                ..default()
                            },
                        ));
                    }
                });
                body.spawn(Node {
                    width: Val::Percent(48.0),
                    padding: UiRect::all(Val::Px(24.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|details| {
                    if let Some((stack, item)) = selected_item {
                        if let Some(path) = item.icon_asset_path.as_deref() {
                            details.spawn((
                                ImageNode {
                                    image: assets.load(path.to_owned()),
                                    color: GREEN,
                                    ..default()
                                },
                                Node {
                                    width: Val::Px(256.0),
                                    height: Val::Px(256.0),
                                    margin: UiRect::bottom(Val::Px(24.0)),
                                    ..default()
                                },
                            ));
                        } else {
                            warn!("pipboy missing icon base={:08x}", item.base_form_id);
                            details.spawn((
                                Text::new("[ NO ITEM ART ]"),
                                TextColor(GREEN_DIM),
                                TextFont {
                                    font_size: FontSize::Px(26.0),
                                    ..default()
                                },
                                Node {
                                    height: Val::Px(256.0),
                                    align_content: AlignContent::Center,
                                    ..default()
                                },
                            ));
                        }
                        details.spawn((
                            Text::new(detail_text(stack, item)),
                            TextColor(GREEN),
                            TextFont {
                                font_size: FontSize::Px(22.0),
                                ..default()
                            },
                            Node {
                                width: Val::Percent(100.0),
                                border: UiRect::top(Val::Px(2.0)),
                                padding: UiRect::top(Val::Px(18.0)),
                                ..default()
                            },
                            BorderColor::all(GREEN_DIM),
                        ));
                        if let Some((label, action)) = item_action_button(stack.key(), item) {
                            details
                                .spawn((
                                    Button,
                                    action,
                                    Node {
                                        margin: UiRect::top(Val::Px(16.0)),
                                        padding: UiRect::axes(Val::Px(18.0), Val::Px(8.0)),
                                        border: UiRect::all(Val::Px(1.0)),
                                        ..default()
                                    },
                                    BorderColor::all(GREEN),
                                    BackgroundColor(GREEN_DIM),
                                ))
                                .with_child((Text::new(label), TextColor(GREEN)));
                        }
                    }
                });
            });
            root.spawn(Node {
                height: Val::Px(64.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceAround,
                border: UiRect::top(Val::Px(2.0)),
                ..default()
            })
            .with_children(|tabs| {
                for (category, label) in categories() {
                    let active = state.category == category;
                    tabs.spawn((
                        Button,
                        CategoryTab(category),
                        Node {
                            padding: UiRect::axes(Val::Px(18.0), Val::Px(8.0)),
                            border: UiRect::all(Val::Px(if active { 2.0 } else { 0.0 })),
                            ..default()
                        },
                        BackgroundColor(if active { GREEN_DIM } else { Color::NONE }),
                        BorderColor::all(GREEN),
                    ))
                    .with_child((
                        Text::new(label),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(24.0),
                            ..default()
                        },
                    ));
                }
            });
        });
}

fn spawn_quantity_picker(commands: &mut Commands, quantity: i32, max: i32) {
    commands
        .spawn((
            QuantityOverlay,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(35.0),
                top: Val::Percent(35.0),
                width: Val::Percent(30.0),
                height: Val::Px(220.0),
                padding: UiRect::all(Val::Px(24.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(SCREEN),
            BorderColor::all(GREEN),
            GlobalZIndex(520),
        ))
        .with_children(|overlay| {
            overlay.spawn((
                Text::new("DROP HOW MANY?"),
                TextColor(GREEN),
                TextFont {
                    font_size: FontSize::Px(26.0),
                    ..default()
                },
            ));
            overlay
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    quantity_button(row, "−", QuantityButton::Minus);
                    row.spawn((
                        QuantityText,
                        Text::new(format!("{quantity} / {max}")),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(28.0),
                            ..default()
                        },
                    ));
                    quantity_button(row, "+", QuantityButton::Plus);
                });
            overlay
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                })
                .with_children(|row| {
                    quantity_button(row, "DROP", QuantityButton::Confirm);
                    quantity_button(row, "CANCEL", QuantityButton::Cancel);
                });
        });
}

fn quantity_button(parent: &mut ChildSpawnerCommands, label: &str, action: QuantityButton) {
    parent
        .spawn((
            Button,
            action,
            Node {
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BorderColor::all(GREEN),
            BackgroundColor(GREEN_DIM),
        ))
        .with_child((Text::new(label), TextColor(GREEN)));
}

fn visible_rows<'a>(
    inventory: &PlayerInventory,
    catalog: &'a PreparedItemCatalog,
    category: PreparedItemCategory,
) -> Vec<(super::inventory::InventoryStack, &'a PreparedItemDefinition)> {
    inventory
        .stack_states()
        .into_iter()
        .filter_map(|stack| {
            catalog
                .items
                .iter()
                .find(|item| item.base_form_id == stack.base_form_id && item.category == category)
                .map(|item| (stack, item))
        })
        .collect()
}

fn normalize_selection(
    state: &mut PipBoyState,
    inventory: &PlayerInventory,
    catalog: &PreparedItemCatalog,
) {
    let rows = visible_rows(inventory, catalog, state.category);
    if !rows
        .iter()
        .any(|(stack, _)| state.selected == Some(stack.key()))
    {
        state.selected = rows.first().map(|(stack, _)| stack.key());
    }
}

fn item_name(item: &PreparedItemDefinition) -> String {
    item.display_name
        .clone()
        .or_else(|| item.editor_id.clone())
        .unwrap_or_else(|| format!("{:08X}", item.base_form_id))
}

fn count_suffix(count: i32) -> String {
    if count > 1 {
        format!(" ({count})")
    } else {
        String::new()
    }
}

/// F98.3: the equipped marker shown ahead of an equipped row's name.
fn equipped_marker(equipped: bool) -> &'static str {
    if equipped { "[E] " } else { "" }
}

fn detail_text(stack: super::inventory::InventoryStack, item: &PreparedItemDefinition) -> String {
    let mut lines = vec![item_name(item)];
    if let Some(value) = item.value {
        lines.push(format!("VAL  {value}"));
    }
    if let Some(weight) = item.weight {
        lines.push(format!("WG   {weight:.1}"));
    }
    if let Some(condition) = stack.condition {
        lines.push(format!("CND  {condition}"));
    }
    match &item.stats {
        PreparedItemStats::Weapon {
            damage,
            max_condition,
            clip_size,
            speed,
            reach,
            ..
        } => {
            if let Some(damage) = damage {
                lines.push(format!("DAM  {damage}"));
            }
            if let Some(max) = max_condition {
                lines.push(format!("MAX CND  {max}"));
            }
            if let Some(clip) = clip_size {
                lines.push(format!("CLIP  {clip}"));
            }
            if let Some(speed) = speed {
                lines.push(format!("SPEED  {speed:.2}"));
            }
            if let Some(reach) = reach {
                lines.push(format!("REACH  {reach:.2}"));
            }
        }
        PreparedItemStats::Apparel {
            armor_rating,
            max_condition,
            ..
        } => {
            if let Some(rating) = armor_rating {
                lines.push(format!("DR  {rating:.1}"));
            }
            if let Some(max) = max_condition {
                lines.push(format!("MAX CND  {max}"));
            }
        }
        PreparedItemStats::Ammo { damage, speed } => {
            if let Some(damage) = damage {
                lines.push(format!("DAM  {damage:.1}"));
            }
            if let Some(speed) = speed {
                lines.push(format!("SPEED  {speed:.1}"));
            }
        }
        PreparedItemStats::Aid { effects } => {
            lines.push("EFFECTS".into());
            lines.extend(effects.iter().map(|effect| effect.label.clone()));
        }
        PreparedItemStats::Book { text, .. } | PreparedItemStats::Note { text } => {
            if let Some(text) = text {
                lines.push(text.clone());
            }
        }
        PreparedItemStats::Key => lines.push("KEY".into()),
        PreparedItemStats::Misc => {}
    }
    lines.join("\n")
}

fn categories() -> [(PreparedItemCategory, &'static str); 5] {
    [
        (PreparedItemCategory::Weapons, "Weapons"),
        (PreparedItemCategory::Apparel, "Apparel"),
        (PreparedItemCategory::Aid, "Aid"),
        (PreparedItemCategory::Misc, "Misc"),
        (PreparedItemCategory::Ammo, "Ammo"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::AssetPlugin;
    use bevy::state::app::StatesPlugin;

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin, AssetPlugin::default()))
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
            .find_map(|(entity, action)| {
                matches!(action, ItemActionButton::Use(_)).then_some(entity)
            })
            .expect("an Aid stack should render a USE button");
        *app.world_mut().get_mut::<Interaction>(button).unwrap() = Interaction::Pressed;
        app.update();

        assert_eq!(app.world().resource::<PlayerInventory>().count(0x77), 2);
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
        app.world_mut()
            .resource_mut::<NextState<GameplayModal>>()
            .set(GameplayModal::PipBoy);
        app.world_mut().resource_mut::<PipBoyState>().category = PreparedItemCategory::Misc;
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
}
