//! Flat Pip-Boy Items surface for M3 wave 1 (issue #71).

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::app_state::GameplayModal;

use super::interaction::{PlayerInventory, item_rules};
use super::inventory::{DropAction, StackKey, drop_action};
use super::{PreparedItemCatalog, PreparedItemCategory, PreparedItemDefinition, PreparedItemStats};

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

pub(crate) fn install(app: &mut App) {
    app.init_resource::<PipBoyState>()
        .add_message::<DropInventoryStackRequested>()
        .add_systems(OnEnter(GameplayModal::PipBoy), enter_pipboy)
        .add_systems(OnExit(GameplayModal::PipBoy), exit_pipboy)
        .add_systems(
            Update,
            (
                handle_item_rows,
                handle_category_tabs,
                handle_quantity_buttons,
                refresh_after_inventory_change,
            )
                .run_if(in_state(GameplayModal::PipBoy)),
        );
}

fn enter_pipboy(
    mut commands: Commands,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
    inventory: Res<PlayerInventory>,
    catalog: Res<PreparedItemCatalog>,
    assets: Res<AssetServer>,
    mut state: ResMut<PipBoyState>,
) {
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
    normalize_selection(&mut state, &inventory, &catalog);
    spawn_screen(&mut commands, &inventory, &catalog, &assets, &state);
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

fn refresh_after_inventory_change(
    mut commands: Commands,
    inventory: Res<PlayerInventory>,
    catalog: Res<PreparedItemCatalog>,
    assets: Res<AssetServer>,
    mut state: ResMut<PipBoyState>,
    roots: Query<Entity, With<PipBoyRoot>>,
    picker: Option<Res<QuantityPicker>>,
) {
    if !inventory.is_changed() || picker.is_some() {
        return;
    }
    normalize_selection(&mut state, &inventory, &catalog);
    rebuild(&mut commands, &roots, &inventory, &catalog, &assets, &state);
}

#[allow(clippy::too_many_arguments)]
fn handle_item_rows(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    rows: Query<(&Interaction, &ItemRow), Changed<Interaction>>,
    all_rows: Query<(&Interaction, &ItemRow)>,
    inventory: Res<PlayerInventory>,
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
            rebuild(&mut commands, &roots, &inventory, &catalog, &assets, &state);
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

fn handle_category_tabs(
    mut commands: Commands,
    tabs: Query<(&Interaction, &CategoryTab), Changed<Interaction>>,
    inventory: Res<PlayerInventory>,
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
            rebuild(&mut commands, &roots, &inventory, &catalog, &assets, &state);
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

fn rebuild(
    commands: &mut Commands,
    roots: &Query<Entity, With<PipBoyRoot>>,
    inventory: &PlayerInventory,
    catalog: &PreparedItemCatalog,
    assets: &AssetServer,
    state: &PipBoyState,
) {
    for root in roots {
        commands.entity(root).despawn();
    }
    spawn_screen(commands, inventory, catalog, assets, state);
}

fn spawn_screen(
    commands: &mut Commands,
    inventory: &PlayerInventory,
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
                            Text::new(format!("{}{}", item_name(item), count_suffix(stack.count))),
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

    #[test]
    fn pipboy_round_trip_releases_and_recaptures_pointer() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin, AssetPlugin::default()))
            .init_state::<GameplayModal>()
            .insert_resource(PlayerInventory::default())
            .insert_resource(ButtonInput::<MouseButton>::default())
            .insert_resource(PreparedItemCatalog::default());
        let window = app
            .world_mut()
            .spawn((CursorOptions::default(), PrimaryWindow))
            .id();
        install(&mut app);
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
}
