//! Fallout 3-style Pip-Boy 3000 screen: the Stats status view (issue #71's
//! Items surface and issue #100's Data view beside it), framed by the
//! reference hardware's look -- a dim phosphor CRT with a radial glow, corner
//! brackets, the LVL/HP/AP/XP stat bar up top, section tabs along the bottom
//! of the screen, and the STATS/ITEMS/DATA button bank on the bezel.

use bevy::prelude::*;
use bevy::ui::{
    BackgroundGradient, ColorStop, RadialGradient, RadialGradientShape, UiPosition,
    widget::TextShadow,
};
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::app_state::GameplayModal;

use super::audio::PlaySound;
use super::bindings::HotkeyBindings;
use super::interaction::{
    EquipToggleRequested, InteractionNotice, PlayerEquipment, PlayerInventory, item_rules, item_use,
};
use super::inventory::{DropAction, StackKey, TransferResult, drop_action};
use super::pipboy_reader::OpenReaderRequested;
use super::{
    CellInfo, PreparedItemCatalog, PreparedItemCategory, PreparedItemDefinition, PreparedItemStats,
    cell_label,
};

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
/// Secondary labels and inactive controls: readable, but clearly not the
/// bright primary phosphor.
const GREEN_FAINT: Color = Color::srgba(0.1, 0.55, 0.26, 0.9);
const SCREEN: Color = Color::srgba(0.005, 0.025, 0.012, 0.97);
/// The CRT's radial phosphor bloom behind everything else.
const SCREEN_GLOW: Color = Color::srgba(0.1, 0.55, 0.26, 0.14);
/// Soft offset duplicate behind text, faking phosphor bleed.
const TEXT_GLOW: Color = Color::srgba(0.18, 1.0, 0.48, 0.5);

fn glow() -> TextShadow {
    TextShadow {
        offset: Vec2::new(1.0, 1.0),
        color: TEXT_GLOW,
    }
}

#[derive(Message, Clone, Copy, Debug)]
pub(crate) struct DropInventoryStackRequested {
    pub(crate) key: StackKey,
    pub(crate) count: i32,
}

/// Which top-level Pip-Boy surface is showing: the Stats status view, the
/// wave-1 Items surface, or issue #100's Data view. Matches the three
/// physical STATS/ITEMS/DATA buttons of the reference hardware.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipBoyView {
    Stats,
    Items,
    Data,
}

fn view_label(view: PipBoyView) -> &'static str {
    match view {
        PipBoyView::Stats => "STATS",
        PipBoyView::Items => "ITEMS",
        PipBoyView::Data => "DATA",
    }
}

/// Display-only player status behind the header stat bar and the Stats
/// view's caption. There is no health/XP gameplay system in scope yet, so
/// these are the factory defaults; a future stats slice owns those rules and
/// overwrites this resource -- the Pip-Boy only ever renders it.
#[derive(Resource, Debug, Clone)]
struct PlayerStatus {
    name: String,
    level: u32,
    hp_current: u32,
    hp_max: u32,
    ap_current: u32,
    ap_max: u32,
    xp_current: u32,
    xp_next: u32,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self {
            name: "Player".into(),
            level: 1,
            hp_current: 100,
            hp_max: 100,
            ap_current: 85,
            ap_max: 85,
            xp_current: 0,
            xp_next: 200,
        }
    }
}

/// The header bar's four stat segments, in display order, as (label, value)
/// pairs -- kept pure so the exact formatting is unit-testable.
fn stat_segments(status: &PlayerStatus) -> [(&'static str, String); 4] {
    [
        ("LVL", status.level.to_string()),
        ("HP", format!("{}/{}", status.hp_current, status.hp_max)),
        ("AP", format!("{}/{}", status.ap_current, status.ap_max)),
        ("XP", format!("{}/{}", status.xp_current, status.xp_next)),
    ]
}

/// F100.2/F100.3: the Data view's sections. Deliberately only these two --
/// map, quests, and radio are out of M3 scope entirely, with no placeholder
/// entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DataSection {
    Notes,
    World,
}

#[derive(Resource, Debug)]
struct PipBoyState {
    view: PipBoyView,
    category: PreparedItemCategory,
    selected: Option<StackKey>,
    data_section: DataSection,
}

impl Default for PipBoyState {
    fn default() -> Self {
        Self {
            view: PipBoyView::Stats,
            category: PreparedItemCategory::Weapons,
            selected: None,
            data_section: DataSection::Notes,
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

/// F100.1: the top-level Items/Data switcher tabs in the header row,
/// following `CategoryTab`'s interaction pattern one level up.
#[derive(Component, Clone, Copy)]
struct ViewTab(PipBoyView);

/// F100.2/F100.3: the Notes/World section tabs shown in the footer while
/// the Data view is active.
#[derive(Component, Clone, Copy)]
struct DataSectionTab(DataSection);

/// F100.2: a Data -> Notes list row; pressing it opens issue #99's reader
/// on this base form id through `OpenReaderRequested`.
#[derive(Component, Clone, Copy)]
struct NoteRow(u32);

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

/// Everything `spawn_screen` reads, bundled (`interaction`'s
/// `LeveledResolveContext` precedent) so the rebuild-triggering systems
/// stay small as issue #100 widens the screen beyond the Items surface.
/// `manifest` is `Option` because the bare test harness runs without a
/// prepared scene; the World section then shows a no-session line.
#[derive(bevy::ecs::system::SystemParam)]
struct ScreenSources<'w> {
    inventory: Res<'w, PlayerInventory>,
    equipment: Res<'w, PlayerEquipment>,
    catalog: Res<'w, PreparedItemCatalog>,
    assets: Res<'w, AssetServer>,
    manifest: Option<Res<'w, crate::viewer::LoadedSceneManifest>>,
    time: Res<'w, Time>,
    status: Res<'w, PlayerStatus>,
}

pub(crate) struct PipBoyPlugin;

impl Plugin for PipBoyPlugin {
    fn build(&self, app: &mut App) {
        install(app);
    }
}

fn install(app: &mut App) {
    app.init_resource::<PipBoyState>()
        .init_resource::<PlayerStatus>()
        // `handle_item_action_button`'s dependencies, normally registered by
        // `interaction`/`audio`/`pipboy_reader`'s installs -- `init_resource`
        // and `add_message` are both no-ops when already registered, so this
        // only matters for the self-contained test harness below.
        .init_resource::<InteractionNotice>()
        .add_message::<PlaySound>()
        .add_message::<OpenReaderRequested>()
        .add_message::<DropInventoryStackRequested>()
        .add_message::<ItemRowActivated>()
        .add_systems(OnEnter(GameplayModal::PipBoy), enter_pipboy)
        .add_systems(OnExit(GameplayModal::PipBoy), exit_pipboy)
        .add_systems(
            Update,
            (
                // F121.1: chained so a row's primary action, if any, is
                // resolved the same frame the click is detected.
                (handle_item_rows, handle_item_row_activation).chain(),
                handle_category_tabs,
                handle_view_tabs,
                handle_data_section_tabs,
                handle_note_rows,
                handle_quantity_buttons,
                handle_item_action_button,
                handle_equip_and_hotkeys,
                refresh_after_inventory_change,
            )
                .in_set(super::plugins::ViewerSet::Ui)
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
    // F100.1: equip/hotkey input belongs to the Items surface only; the
    // Data view keeps the last selection but must not act on it.
    if state.view != PipBoyView::Items {
        return;
    }
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
    sources: ScreenSources,
    mut state: ResMut<PipBoyState>,
) {
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
    normalize_selection(&mut state, &sources.inventory, &sources.catalog);
    spawn_screen(&mut commands, &sources, &state);
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
    sources: ScreenSources,
    mut state: ResMut<PipBoyState>,
    roots: Query<Entity, With<PipBoyRoot>>,
    picker: Option<Res<QuantityPicker>>,
) {
    if (!sources.inventory.is_changed() && !sources.equipment.is_changed()) || picker.is_some() {
        return;
    }
    normalize_selection(&mut state, &sources.inventory, &sources.catalog);
    rebuild(&mut commands, &roots, &sources, &state);
}

#[allow(clippy::too_many_arguments)]
fn handle_item_rows(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    rows: Query<(&Interaction, &ItemRow), Changed<Interaction>>,
    all_rows: Query<(&Interaction, &ItemRow)>,
    sources: ScreenSources,
    mut state: ResMut<PipBoyState>,
    roots: Query<Entity, With<PipBoyRoot>>,
    picker: Option<Res<QuantityPicker>>,
    mut drops: MessageWriter<DropInventoryStackRequested>,
    mut activations: MessageWriter<ItemRowActivated>,
) {
    // F121.1: a click both selects (as before) and, if the row has a
    // primary action, triggers it -- whether or not the row was already
    // selected. Selection state is updated (and the screen rebuilt) first
    // so the click still behaves like today's plain-select click even for
    // rows with no primary action (e.g. Misc).
    for (interaction, row) in &rows {
        if *interaction == Interaction::Pressed {
            if state.selected != Some(row.0) {
                state.selected = Some(row.0);
                rebuild(&mut commands, &roots, &sources, &state);
            }
            if let Some(item) = sources
                .catalog
                .items
                .iter()
                .find(|item| item.base_form_id == row.0.base_form_id)
                && let Some(action) = row_primary_action(row.0, item)
            {
                activations.write(ItemRowActivated(action));
            }
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
    let count = sources
        .inventory
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
    sources: ScreenSources,
    mut state: ResMut<PipBoyState>,
    roots: Query<Entity, With<PipBoyRoot>>,
) {
    for (interaction, tab) in &tabs {
        if *interaction == Interaction::Pressed && state.category != tab.0 {
            state.category = tab.0;
            state.selected = None;
            normalize_selection(&mut state, &sources.inventory, &sources.catalog);
            rebuild(&mut commands, &roots, &sources, &state);
            return;
        }
    }
}

/// The top-level Stats/Items/Data switcher (the bezel button bank) --
/// `handle_category_tabs`' interaction pattern one level up. Switching back
/// to Items re-normalizes the selection so the view is never left pointing
/// at a stack that was consumed or dropped while Data was showing.
fn handle_view_tabs(
    mut commands: Commands,
    tabs: Query<(&Interaction, &ViewTab), Changed<Interaction>>,
    sources: ScreenSources,
    mut state: ResMut<PipBoyState>,
    roots: Query<Entity, With<PipBoyRoot>>,
) {
    for (interaction, tab) in &tabs {
        if *interaction == Interaction::Pressed && state.view != tab.0 {
            state.view = tab.0;
            if state.view == PipBoyView::Items {
                normalize_selection(&mut state, &sources.inventory, &sources.catalog);
            }
            rebuild(&mut commands, &roots, &sources, &state);
            return;
        }
    }
}

/// F100.2/F100.3: the Notes/World section tabs inside the Data view.
fn handle_data_section_tabs(
    mut commands: Commands,
    tabs: Query<(&Interaction, &DataSectionTab), Changed<Interaction>>,
    sources: ScreenSources,
    mut state: ResMut<PipBoyState>,
    roots: Query<Entity, With<PipBoyRoot>>,
) {
    for (interaction, tab) in &tabs {
        if *interaction == Interaction::Pressed && state.data_section != tab.0 {
            state.data_section = tab.0;
            rebuild(&mut commands, &roots, &sources, &state);
            return;
        }
    }
}

/// F100.2: pressing a Notes row opens issue #99's reader through its public
/// `OpenReaderRequested` seam -- the stack itself is never touched.
fn handle_note_rows(
    rows: Query<(&Interaction, &NoteRow), Changed<Interaction>>,
    mut reader_requests: MessageWriter<OpenReaderRequested>,
) {
    for (interaction, row) in &rows {
        if *interaction == Interaction::Pressed {
            reader_requests.write(OpenReaderRequested {
                base_form_id: row.0,
            });
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
            gain_db: 0.0,
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

/// F121.1: which action, if any, a Pip-Boy Items row's primary interaction
/// (a click, or the `E` key) performs. Equip-eligible categories
/// (Weapons/Apparel/Ammo, `is_equip_eligible`) equip/unequip; everything else
/// routes through the exact same `item_use::classify` call the details
/// pane's USE/READ button (`item_action_button`) uses. `Inert` stacks
/// (Key/Misc, a textless Book/Note, a quest-flagged Aid item) have no
/// primary action, so a click on one only selects it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RowPrimaryAction {
    Equip(StackKey),
    Use(StackKey),
    Read(u32),
}

fn row_primary_action(key: StackKey, item: &PreparedItemDefinition) -> Option<RowPrimaryAction> {
    if is_equip_eligible(item.category) {
        return Some(RowPrimaryAction::Equip(key));
    }
    match item_use::classify(item_use_stats(&item.stats), item.quest_item) {
        item_use::ItemUseAction::Use => Some(RowPrimaryAction::Use(key)),
        item_use::ItemUseAction::Read => Some(RowPrimaryAction::Read(item.base_form_id)),
        item_use::ItemUseAction::Inert => None,
    }
}

/// F121.1: written by `handle_item_rows` when a clicked row has a primary
/// action; consumed by `handle_item_row_activation` in the same frame.
#[derive(Message, Clone, Copy, Debug)]
struct ItemRowActivated(RowPrimaryAction);

/// F121.1: performs a row's primary action through the exact same paths the
/// `E` key (`EquipToggleRequested`, `handle_equip_and_hotkeys`) and the
/// details-pane button (`use_item`/`OpenReaderRequested`,
/// `handle_item_action_button`) already use -- this system only dispatches.
fn handle_item_row_activation(
    mut activations: MessageReader<ItemRowActivated>,
    mut inventory: ResMut<PlayerInventory>,
    catalog: Res<PreparedItemCatalog>,
    mut notice: ResMut<InteractionNotice>,
    mut sounds: MessageWriter<PlaySound>,
    mut reader_requests: MessageWriter<OpenReaderRequested>,
    mut equip_requests: MessageWriter<EquipToggleRequested>,
) {
    for activation in activations.read() {
        match activation.0 {
            RowPrimaryAction::Equip(key) => {
                equip_requests.write(EquipToggleRequested(key));
            }
            RowPrimaryAction::Use(key) => {
                use_item(key, &mut inventory, &catalog, &mut notice, &mut sounds);
            }
            RowPrimaryAction::Read(base_form_id) => {
                reader_requests.write(OpenReaderRequested { base_form_id });
            }
        }
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

fn rebuild(
    commands: &mut Commands,
    roots: &Query<Entity, With<PipBoyRoot>>,
    sources: &ScreenSources,
    state: &PipBoyState,
) {
    for root in roots {
        commands.entity(root).despawn();
    }
    spawn_screen(commands, sources, state);
}

fn spawn_screen(commands: &mut Commands, sources: &ScreenSources, state: &PipBoyState) {
    let weight = sources
        .inventory
        .total_weight(|form_id| {
            sources
                .catalog
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
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(SCREEN),
            GlobalZIndex(500),
        ))
        .with_children(|root| {
            // CRT phosphor bloom behind everything, brightest toward the
            // upper middle like the reference screen.
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundGradient::from(RadialGradient::new(
                    UiPosition::anchor(Vec2::new(0.0, -0.35)),
                    RadialGradientShape::FarthestCorner,
                    vec![
                        ColorStop::new(SCREEN_GLOW, Val::Percent(0.0)),
                        ColorStop::new(Color::NONE, Val::Percent(100.0)),
                    ],
                )),
            ));
            root.spawn(Node {
                flex_grow: 1.0,
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::axes(Val::Px(40.0), Val::Px(16.0)),
                ..default()
            })
            .with_children(|screen| {
                spawn_header(screen, state, &sources.status);
                match state.view {
                    PipBoyView::Stats => spawn_stats_body(screen, sources, &sources.status),
                    PipBoyView::Items => spawn_items_body(screen, sources, state),
                    PipBoyView::Data => spawn_data_body(screen, sources, state),
                }
                spawn_footer(screen, state, weight);
            });
            spawn_view_buttons(root, state);
            spawn_corner_brackets(root);
        });
}

/// The stat bar along the top of the screen: the active view's name on the
/// left, then the framed LVL/HP/AP/XP segments from the reference layout.
fn spawn_header(screen: &mut ChildSpawnerCommands, state: &PipBoyState, status: &PlayerStatus) {
    screen
        .spawn((
            Node {
                height: Val::Px(64.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                border: UiRect::bottom(Val::Px(2.0)),
                padding: UiRect::bottom(Val::Px(6.0)),
                ..default()
            },
            BorderColor::all(GREEN_DIM),
        ))
        .with_children(|header| {
            header.spawn((
                Text::new(view_label(state.view)),
                TextColor(GREEN),
                TextFont {
                    font_size: FontSize::Px(40.0),
                    ..default()
                },
                glow(),
            ));
            header
                .spawn(Node {
                    column_gap: Val::Px(24.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|segments| {
                    for (label, value) in stat_segments(status) {
                        segments
                            .spawn((
                                Node {
                                    padding: UiRect::axes(Val::Px(14.0), Val::Px(3.0)),
                                    border: UiRect {
                                        top: Val::Px(2.0),
                                        bottom: Val::Px(2.0),
                                        ..default()
                                    },
                                    column_gap: Val::Px(12.0),
                                    align_items: AlignItems::Baseline,
                                    ..default()
                                },
                                BorderColor::all(GREEN_DIM),
                            ))
                            .with_children(|segment| {
                                segment.spawn((
                                    Text::new(label),
                                    TextColor(GREEN_FAINT),
                                    TextFont {
                                        font_size: FontSize::Px(18.0),
                                        ..default()
                                    },
                                ));
                                segment.spawn((
                                    Text::new(value),
                                    TextColor(GREEN),
                                    TextFont {
                                        font_size: FontSize::Px(30.0),
                                        ..default()
                                    },
                                    glow(),
                                ));
                            });
                    }
                });
        });
}

/// The section-tab strip above the button bank. Items shows the reference
/// screen's bottom-left carry weight; Stats has exactly one section in scope
/// (Status), rendered as the boxed active label from the reference.
fn spawn_footer(screen: &mut ChildSpawnerCommands, state: &PipBoyState, weight: f32) {
    screen
        .spawn((
            Node {
                height: Val::Px(56.0),
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                border: UiRect::top(Val::Px(2.0)),
                padding: UiRect::top(Val::Px(6.0)),
                ..default()
            },
            BorderColor::all(GREEN_DIM),
        ))
        .with_children(|footer| {
            footer
                .spawn(Node {
                    width: Val::Percent(20.0),
                    ..default()
                })
                .with_children(|left| {
                    if state.view == PipBoyView::Items {
                        left.spawn((
                            Text::new(format!("WG {weight:.1}")),
                            TextColor(GREEN_FAINT),
                            TextFont {
                                font_size: FontSize::Px(22.0),
                                ..default()
                            },
                        ));
                    }
                });
            footer
                .spawn(Node {
                    width: Val::Percent(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(14.0),
                    ..default()
                })
                .with_children(|tabs| match state.view {
                    PipBoyView::Stats => {
                        tabs.spawn((
                            Node {
                                padding: UiRect::axes(Val::Px(18.0), Val::Px(4.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor::all(GREEN),
                            BackgroundColor(GREEN_DIM),
                        ))
                        .with_child((
                            Text::new("Status"),
                            TextColor(GREEN),
                            TextFont {
                                font_size: FontSize::Px(24.0),
                                ..default()
                            },
                            glow(),
                        ));
                    }
                    PipBoyView::Items => {
                        let entries: Vec<_> = categories()
                            .into_iter()
                            .map(|(category, label)| {
                                (CategoryTab(category), label, state.category == category)
                            })
                            .collect();
                        footer_tabs(tabs, entries);
                    }
                    PipBoyView::Data => {
                        let entries: Vec<_> = data_sections()
                            .into_iter()
                            .map(|(section, label)| {
                                (
                                    DataSectionTab(section),
                                    label,
                                    state.data_section == section,
                                )
                            })
                            .collect();
                        footer_tabs(tabs, entries);
                    }
                });
            footer.spawn(Node {
                width: Val::Percent(20.0),
                ..default()
            });
        });
}

/// Shared body for the Items/Data footer strips: tabs joined by the short
/// dim rules the reference draws between section names.
fn footer_tabs(
    tabs: &mut ChildSpawnerCommands,
    entries: Vec<(impl Component, &'static str, bool)>,
) {
    let last = entries.len().saturating_sub(1);
    for (index, (marker, label, active)) in entries.into_iter().enumerate() {
        footer_tab(tabs, label, active, marker);
        if index != last {
            tabs.spawn((
                Node {
                    width: Val::Px(26.0),
                    height: Val::Px(2.0),
                    ..default()
                },
                BackgroundColor(GREEN_DIM),
            ));
        }
    }
}

/// The physical button bank on the bezel below the screen: three large
/// STATS/ITEMS/DATA buttons, the active one lit like the reference lamp.
fn spawn_view_buttons(root: &mut ChildSpawnerCommands, state: &PipBoyState) {
    root.spawn(Node {
        height: Val::Px(88.0),
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        column_gap: Val::Px(56.0),
        ..default()
    })
    .with_children(|bank| {
        for view in [PipBoyView::Stats, PipBoyView::Items, PipBoyView::Data] {
            let active = state.view == view;
            bank.spawn((
                Button,
                ViewTab(view),
                Node {
                    width: Val::Px(190.0),
                    height: Val::Px(54.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(3.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BorderColor::all(if active { GREEN } else { GREEN_FAINT }),
                BackgroundColor(if active { GREEN_DIM } else { Color::NONE }),
            ))
            .with_child((
                Text::new(view_label(view)),
                TextColor(if active { GREEN } else { GREEN_FAINT }),
                TextFont {
                    font_size: FontSize::Px(26.0),
                    ..default()
                },
                glow(),
            ));
        }
    });
}

/// The L-shaped bracket marks at the screen's four corners.
fn spawn_corner_brackets(root: &mut ChildSpawnerCommands) {
    const OFFSET: f32 = 10.0;
    const SIZE: f32 = 42.0;
    const WIDTH: f32 = 3.0;
    for (left, right, top, bottom, border) in [
        (
            Some(OFFSET),
            None,
            Some(OFFSET),
            None,
            UiRect {
                left: Val::Px(WIDTH),
                top: Val::Px(WIDTH),
                ..default()
            },
        ),
        (
            None,
            Some(OFFSET),
            Some(OFFSET),
            None,
            UiRect {
                right: Val::Px(WIDTH),
                top: Val::Px(WIDTH),
                ..default()
            },
        ),
        (
            Some(OFFSET),
            None,
            None,
            Some(OFFSET),
            UiRect {
                left: Val::Px(WIDTH),
                bottom: Val::Px(WIDTH),
                ..default()
            },
        ),
        (
            None,
            Some(OFFSET),
            None,
            Some(OFFSET),
            UiRect {
                right: Val::Px(WIDTH),
                bottom: Val::Px(WIDTH),
                ..default()
            },
        ),
    ] {
        root.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: left.map(Val::Px).unwrap_or_default(),
                right: right.map(Val::Px).unwrap_or_default(),
                top: top.map(Val::Px).unwrap_or_default(),
                bottom: bottom.map(Val::Px).unwrap_or_default(),
                width: Val::Px(SIZE),
                height: Val::Px(SIZE),
                border,
                ..default()
            },
            BorderColor::all(GREEN),
        ));
    }
}

/// One capsule condition bar from the Status figure: a framed outline with
/// the filled portion inside. No limb-damage system exists yet, so every bar
/// is full; the fraction parameter keeps the shape honest for when one does.
fn limb_bar(parent: &mut ChildSpawnerCommands, fraction: f32) {
    parent
        .spawn((
            Node {
                width: Val::Px(130.0),
                height: Val::Px(18.0),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::MAX,
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(GREEN),
        ))
        .with_child((
            Node {
                width: Val::Percent(fraction.clamp(0.0, 1.0) * 100.0),
                height: Val::Percent(100.0),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BackgroundColor(GREEN),
        ));
}

/// A rounded outline segment of the status figure (head, torso, arms, legs).
fn figure_segment(parent: &mut ChildSpawnerCommands, width: f32, height: f32, radius: f32) {
    parent.spawn((
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            border: UiRect::all(Val::Px(4.0)),
            border_radius: BorderRadius::all(Val::Px(radius)),
            ..default()
        },
        BorderColor::all(GREEN),
    ));
}

/// The Stats view's Status section: CND/RAD/EFF labels on the left, the
/// line-art dweller with its six limb condition bars in the middle, the
/// quick-use Aid line on the right, and the name/level caption below --
/// the reference STATS > Status screen.
fn spawn_stats_body(
    screen: &mut ChildSpawnerCommands,
    sources: &ScreenSources,
    status: &PlayerStatus,
) {
    screen
        .spawn(Node {
            flex_grow: 1.0,
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            ..default()
        })
        .with_children(|body| {
            body.spawn(Node {
                width: Val::Percent(15.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                row_gap: Val::Px(20.0),
                padding: UiRect {
                    left: Val::Px(12.0),
                    top: Val::Px(40.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|labels| {
                labels
                    .spawn((
                        Node {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(4.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BorderColor::all(GREEN),
                    ))
                    .with_child((
                        Text::new("CND"),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(26.0),
                            ..default()
                        },
                        glow(),
                    ));
                for label in ["RAD", "EFF"] {
                    labels.spawn((
                        Text::new(label),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(26.0),
                            ..default()
                        },
                        glow(),
                        Node {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(4.0)),
                            ..default()
                        },
                    ));
                }
            });
            body.spawn(Node {
                width: Val::Percent(60.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(16.0),
                ..default()
            })
            .with_children(|figure| {
                limb_bar(figure, 1.0);
                figure_segment(figure, 110.0, 110.0, 55.0);
                figure
                    .spawn(Node {
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(16.0),
                        ..default()
                    })
                    .with_children(|row| {
                        limb_bar(row, 1.0);
                        figure_segment(row, 118.0, 18.0, 9.0);
                        row.spawn((
                            Node {
                                width: Val::Px(122.0),
                                height: Val::Px(132.0),
                                border: UiRect::all(Val::Px(4.0)),
                                border_radius: BorderRadius::all(Val::Px(18.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            BorderColor::all(GREEN),
                        ))
                        .with_children(|torso| {
                            torso
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(16.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        border_radius: BorderRadius::MAX,
                                        padding: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BorderColor::all(GREEN),
                                ))
                                .with_child((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        border_radius: BorderRadius::MAX,
                                        ..default()
                                    },
                                    BackgroundColor(GREEN),
                                ));
                        });
                        figure_segment(row, 118.0, 18.0, 9.0);
                        limb_bar(row, 1.0);
                    });
                figure
                    .spawn(Node {
                        column_gap: Val::Px(48.0),
                        ..default()
                    })
                    .with_children(|legs| {
                        for _ in 0..2 {
                            legs.spawn(Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(14.0),
                                ..default()
                            })
                            .with_children(|leg| {
                                figure_segment(leg, 36.0, 132.0, 12.0);
                                limb_bar(leg, 1.0);
                            });
                        }
                    });
                figure.spawn((
                    Text::new(format!("{} - Level {}", status.name, status.level)),
                    TextColor(GREEN),
                    TextFont {
                        font_size: FontSize::Px(28.0),
                        ..default()
                    },
                    glow(),
                    Node {
                        margin: UiRect::top(Val::Px(14.0)),
                        ..default()
                    },
                ));
            });
            body.spawn(Node {
                width: Val::Percent(25.0),
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect {
                    right: Val::Px(12.0),
                    top: Val::Px(48.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|quick| {
                if let Some(line) = quick_aid_line(&sources.inventory, &sources.catalog) {
                    quick.spawn((
                        Text::new(line),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(22.0),
                            ..default()
                        },
                        glow(),
                    ));
                }
            });
        });
}

/// The Status screen's quick-use line. The reference shows the assigned
/// healing item; there is no heal-slot concept here, so prefer a
/// Stimpak-named Aid stack, otherwise the largest Aid stack in the pack.
fn quick_aid_line(inventory: &PlayerInventory, catalog: &PreparedItemCatalog) -> Option<String> {
    let mut aid: Vec<(i32, &PreparedItemDefinition)> = inventory
        .stack_states()
        .into_iter()
        .filter_map(|stack| {
            catalog
                .items
                .iter()
                .find(|item| {
                    item.base_form_id == stack.base_form_id
                        && item.category == PreparedItemCategory::Aid
                })
                .map(|item| (stack.count, item))
        })
        .collect();
    aid.sort_by(|(a_count, a), (b_count, b)| {
        let a_stimpak = item_name(a).to_ascii_lowercase().contains("stimpak");
        let b_stimpak = item_name(b).to_ascii_lowercase().contains("stimpak");
        b_stimpak
            .cmp(&a_stimpak)
            .then(b_count.cmp(a_count))
            .then(a.base_form_id.cmp(&b.base_form_id))
    });
    aid.first()
        .map(|(count, item)| format!("({count}) {}", item_name(item)))
}

fn spawn_items_body(root: &mut ChildSpawnerCommands, sources: &ScreenSources, state: &PipBoyState) {
    let mut rows = visible_rows(&sources.inventory, &sources.catalog, state.category);
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
                        equipped_marker(sources.equipment.is_equipped(key)),
                        item_name(item),
                        count_suffix(stack.count)
                    )),
                    TextColor(GREEN),
                    TextFont {
                        font_size: FontSize::Px(24.0),
                        ..default()
                    },
                    glow(),
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
                    details
                        .spawn((
                            Node {
                                padding: UiRect::all(Val::Px(14.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                margin: UiRect::bottom(Val::Px(24.0)),
                                ..default()
                            },
                            BorderColor::all(GREEN_DIM),
                        ))
                        .with_child((
                            ImageNode {
                                image: sources.assets.load(path.to_owned()),
                                color: GREEN,
                                ..default()
                            },
                            Node {
                                width: Val::Px(228.0),
                                height: Val::Px(228.0),
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
                    glow(),
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
                                padding: UiRect::axes(Val::Px(18.0), Val::Px(6.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            BorderColor::all(GREEN),
                            BackgroundColor(GREEN_DIM),
                        ))
                        .with_child((
                            Text::new(label),
                            TextColor(GREEN),
                            TextFont {
                                font_size: FontSize::Px(20.0),
                                ..default()
                            },
                            glow(),
                        ));
                }
            }
        });
    });
}

/// F100.2/F100.3: the Data view body -- either the Notes list of readable
/// stacks or the read-only World session summary.
fn spawn_data_body(root: &mut ChildSpawnerCommands, sources: &ScreenSources, state: &PipBoyState) {
    match state.data_section {
        DataSection::Notes => {
            let rows = notes_rows(&sources.inventory, &sources.catalog);
            root.spawn(Node {
                flex_grow: 1.0,
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                overflow: Overflow::scroll_y(),
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            })
            .with_children(|list| {
                if rows.is_empty() {
                    list.spawn((Text::new("No notes"), TextColor(GREEN_DIM)));
                }
                for (stack, item) in &rows {
                    list.spawn((
                        Button,
                        NoteRow(item.base_form_id),
                        Node {
                            min_height: Val::Px(42.0),
                            width: Val::Percent(100.0),
                            padding: UiRect::horizontal(Val::Px(10.0)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_child((
                        Text::new(format!("{}{}", item_name(item), count_suffix(stack.count))),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(24.0),
                            ..default()
                        },
                        glow(),
                    ));
                }
            });
        }
        DataSection::World => {
            root.spawn(Node {
                flex_grow: 1.0,
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(24.0)),
                row_gap: Val::Px(12.0),
                ..default()
            })
            .with_children(|body| {
                let session = sources
                    .manifest
                    .as_deref()
                    .map(|manifest| (&manifest.cell, manifest.placements.len()));
                for line in world_lines(session, sources.time.elapsed_secs_f64()) {
                    body.spawn((
                        Text::new(line),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(24.0),
                            ..default()
                        },
                        glow(),
                    ));
                }
            });
        }
    }
}

/// One footer tab (Items category tabs and Data section tabs share the
/// exact same look; only the marker component differs).
fn footer_tab(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    active: bool,
    marker: impl Component,
) {
    parent
        .spawn((
            Button,
            marker,
            Node {
                padding: UiRect::axes(Val::Px(18.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(if active { 2.0 } else { 0.0 })),
                ..default()
            },
            BackgroundColor(if active { GREEN_DIM } else { Color::NONE }),
            BorderColor::all(GREEN),
        ))
        .with_child((
            Text::new(label.to_owned()),
            TextColor(if active { GREEN } else { GREEN_FAINT }),
            TextFont {
                font_size: FontSize::Px(24.0),
                ..default()
            },
            glow(),
        ));
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
                border_radius: BorderRadius::all(Val::Px(10.0)),
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

/// F100.2: which inventory stacks appear in the Data -> Notes list --
/// exactly the stacks issue #99's `classify` calls `Read` (Book/Note with
/// authored text; quest-flagged ones stay readable, everything else is
/// filtered out) -- sorted deterministically by lowercase display name,
/// then stack key.
fn notes_rows<'a>(
    inventory: &PlayerInventory,
    catalog: &'a PreparedItemCatalog,
) -> Vec<(super::inventory::InventoryStack, &'a PreparedItemDefinition)> {
    let mut rows: Vec<_> = inventory
        .stack_states()
        .into_iter()
        .filter_map(|stack| {
            catalog
                .items
                .iter()
                .find(|item| item.base_form_id == stack.base_form_id)
                .filter(|item| {
                    item_use::classify(item_use_stats(&item.stats), item.quest_item)
                        == item_use::ItemUseAction::Read
                })
                .map(|item| (stack, item))
        })
        .collect();
    rows.sort_by(|(a_stack, a), (b_stack, b)| {
        item_name(a)
            .to_ascii_lowercase()
            .cmp(&item_name(b).to_ascii_lowercase())
            .then(a_stack.key().cmp(&b_stack.key()))
    });
    rows
}

/// F100.3: the World section's read-only lines, mirroring what
/// `bevyout.session` reports over the agent bridge (cell identity and
/// placement count from the prepared manifest) plus the same play-time
/// clock the save header records (`Time::elapsed_secs_f64`). Display only
/// -- no new stat tracking.
fn world_lines(session: Option<(&CellInfo, usize)>, play_seconds: f64) -> Vec<String> {
    let mut lines = Vec::new();
    match session {
        Some((cell, placement_count)) => {
            if let Some(name) = cell.name.as_deref() {
                lines.push(format!("CELL  {name}"));
            }
            lines.push(format!("LOC   {}", cell_label(cell)));
            lines.push(
                if cell.interior {
                    "INTERIOR"
                } else {
                    "EXTERIOR"
                }
                .into(),
            );
            lines.push(format!("PLACEMENTS  {placement_count}"));
        }
        None => lines.push("NO ACTIVE CELL".into()),
    }
    lines.push(format!("PLAY TIME  {}", format_play_time(play_seconds)));
    lines
}

/// F100.3: whole seconds as `H:MM:SS`, matching the save header's
/// `play_time_seconds` clock.
fn format_play_time(seconds: f64) -> String {
    let total = seconds.max(0.0) as u64;
    format!(
        "{}:{:02}:{:02}",
        total / 3600,
        (total % 3600) / 60,
        total % 60
    )
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

fn data_sections() -> [(DataSection, &'static str); 2] {
    [(DataSection::Notes, "Notes"), (DataSection::World, "World")]
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
}
