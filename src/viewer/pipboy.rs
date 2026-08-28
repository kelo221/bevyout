//! Fallout 3-style Pip-Boy 3000 screen: the Stats status view (issue #71's
//! Items surface and issue #100's Data view beside it), framed by the
//! reference hardware's look -- a dim phosphor CRT with a radial glow, corner
//! brackets, the LVL/HP/AP/XP stat bar up top, section tabs along the bottom
//! of the screen, and the STATS/ITEMS/DATA button bank on the bezel.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::ui::{BackgroundGradient, ColorStop, RadialGradient, RadialGradientShape, UiPosition};
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::app_state::GameplayModal;

use super::audio::PlaySound;
use super::bindings::HotkeyBindings;
use super::effects::{
    ActiveEffectsList, Addictions, EffectCatalog, PlayerEffectComponents, PlayerRadiation,
    PlayerVitals, RngResource, apply_ingestible,
};
use super::fallout_ui::{
    BEZEL, BEZEL_EDGE, BEZEL_RECESS, LAMP, LAMP_DIM, PHOSPHOR as GREEN, PHOSPHOR_DIM as GREEN_DIM,
    PHOSPHOR_FAINT as GREEN_FAINT, SCREEN, SCREEN_GLOW, glow, spawn_corner_brackets,
    spawn_selection_marker,
};
use super::interaction::{
    CanonicalItemLedger, EquipToggleRequested, InteractionNotice, PlayerEquipment, PlayerInventory,
    item_rules, item_use,
};
use super::inventory::{DropAction, StackKey, drop_action};
use super::pipboy_reader::OpenReaderRequested;
use super::player::FpsPlayer;
use super::stats::{
    ActorStats, PlayerLimbTarget, PlayerProgression, StatsSettings, restore_targeted_stimpak,
};
use super::{
    CellInfo, PreparedItemCatalog, PreparedItemCategory, PreparedItemDefinition, PreparedItemStats,
    cell_label,
};

mod presentation;
mod stats;
#[cfg(test)]
use stats::{MoveDirection, StatusBodyPart, StatusFigurePart, StatusPartLayout, quick_aid_line};
use stats::{
    StatusFigureEditor, StatusFigureLayout, StatusMoveRepeat, handle_status_figure_editor,
    spawn_stats_body,
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
/// view's caption. Live values come from [`crate::viewer::inspection`]; this
/// resource only formats them.
#[derive(Resource, Debug, Clone)]
struct PlayerStatus {
    name: String,
    level: u32,
    hp_current: u32,
    hp_max: u32,
    ap_current: Option<u32>,
    ap_max: u32,
    xp_current: u32,
    xp_next: u32,
    radiation_line: String,
    effect_lines: Vec<String>,
    world_clock_line: String,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self {
            name: "Player".into(),
            level: 1,
            hp_current: 100,
            hp_max: 100,
            ap_current: None,
            ap_max: 75,
            xp_current: 0,
            xp_next: 200,
            radiation_line: String::new(),
            effect_lines: Vec::new(),
            world_clock_line: String::new(),
        }
    }
}

fn project_status_from_snapshot(
    status: &mut PlayerStatus,
    snapshot: &bevyout_core::inspection::RpgInspectionSnapshot,
) {
    status.name = snapshot.player.name.clone();
    status.level = u32::from(snapshot.player.level);
    status.hp_current = snapshot.player.hp_current;
    status.hp_max = snapshot.player.hp_max;
    status.ap_current = snapshot.player.ap_current;
    status.ap_max = snapshot.player.ap_max;
    status.xp_current = snapshot.player.xp_current;
    status.xp_next = snapshot.player.xp_next;
    status.radiation_line = bevyout_core::inspection::radiation_stage_line(snapshot);
    status.effect_lines = snapshot
        .effects
        .entries
        .iter()
        .map(|entry| {
            format!(
                "{}  {}  {:+}  {} ms",
                entry.source, entry.actor_value, entry.magnitude, entry.remaining_ms
            )
        })
        .collect();
    status.world_clock_line = bevyout_core::inspection::world_clock_line(snapshot);
}

/// The header bar's four stat segments, in display order, as (label, value)
/// pairs -- kept pure so the exact formatting is unit-testable.
fn stat_segments(status: &PlayerStatus) -> [(&'static str, String); 4] {
    let ap = match status.ap_current {
        Some(current) => format!("{}/{}", current, status.ap_max),
        None => format!("—/{}", status.ap_max),
    };
    [
        ("LVL", status.level.to_string()),
        ("HP", format!("{}/{}", status.hp_current, status.hp_max)),
        ("AP", ap),
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

#[derive(Component)]
struct PipBoyDevice;

#[derive(Component)]
struct PipBoyScreen;

#[derive(Component)]
struct PipBoyHeader;

#[derive(Component)]
struct PipBoyFooter;

#[derive(Component)]
struct PipBoyButtonBank;

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
    figure_layout: Res<'w, StatusFigureLayout>,
    figure_editor: Res<'w, StatusFigureEditor>,
    presentation: Res<'w, presentation::PipBoyPresentation>,
    progression: Option<Res<'w, PlayerProgression>>,
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
        .init_resource::<StatusFigureLayout>()
        .init_resource::<StatusFigureEditor>()
        .init_resource::<StatusMoveRepeat>()
        .init_resource::<Clipboard>()
        .init_resource::<CanonicalItemLedger>()
        .init_resource::<EffectCatalog>()
        .init_resource::<StatsSettings>()
        .init_resource::<PlayerProgression>()
        .init_resource::<PlayerLimbTarget>()
        .init_resource::<RngResource>()
        // `handle_item_action_button`'s dependencies, normally registered by
        // `interaction`/`audio`/`pipboy_reader`'s installs -- `init_resource`
        // and `add_message` are both no-ops when already registered, so this
        // only matters for the self-contained test harness below.
        .init_resource::<InteractionNotice>()
        .add_message::<PlaySound>()
        .add_message::<OpenReaderRequested>()
        .add_message::<DropInventoryStackRequested>()
        .add_message::<ItemRowActivated>()
        .add_plugins(presentation::PipBoyPresentationPlugin)
        .add_systems(
            OnEnter(GameplayModal::PipBoy),
            (
                presentation::begin_open,
                project_player_status,
                enter_pipboy,
            )
                .chain(),
        )
        .add_systems(
            OnExit(GameplayModal::PipBoy),
            (exit_pipboy, presentation::finish_close).chain(),
        )
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
                handle_status_figure_editor,
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

fn project_player_status(world: &mut World) {
    let snapshot = super::inspection::rpg_snapshot_from_world(world);
    if let Some(mut status) = world.get_resource_mut::<PlayerStatus>() {
        project_status_from_snapshot(&mut status, &snapshot);
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
    mut aid_use: AidUseContext,
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
                use_item(key, &mut aid_use, &catalog, &mut notice, &mut sounds)
            }
            ItemActionButton::Read(base_form_id) => {
                reader_requests.write(OpenReaderRequested { base_form_id });
            }
        }
    }
}

type AidPlayerQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static ActorStats,
        &'static mut PlayerVitals,
        &'static mut PlayerRadiation,
        &'static mut ActiveEffectsList,
        &'static mut Addictions,
    ),
    With<FpsPlayer>,
>;

#[derive(SystemParam)]
struct AidUseContext<'w, 's> {
    canonical: ResMut<'w, CanonicalItemLedger>,
    inventory: ResMut<'w, PlayerInventory>,
    effect_catalog: Res<'w, EffectCatalog>,
    settings: Res<'w, StatsSettings>,
    rng: ResMut<'w, RngResource>,
    progression: ResMut<'w, PlayerProgression>,
    limb_target: Res<'w, PlayerLimbTarget>,
    player: AidPlayerQuery<'w, 's>,
}

fn use_item(
    key: StackKey,
    context: &mut AidUseContext,
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
    let Some(item_id) = context.canonical.player_item_id_for_stack(key) else {
        return;
    };
    let Ok(used) = context
        .canonical
        .use_player_item(&mut context.inventory, item_id)
    else {
        return;
    };
    if let Some(definition) = context.effect_catalog.get(used.base_form_id) {
        let Ok((stats, mut vitals, mut radiation, mut effects, mut addictions)) =
            context.player.single_mut()
        else {
            return;
        };
        apply_ingestible(
            definition,
            stats,
            &context.progression.perks,
            &context.settings,
            PlayerEffectComponents {
                vitals: &mut vitals,
                radiation: &mut radiation,
                effects: &mut effects,
                addictions: &mut addictions,
            },
            &mut context.rng.0,
        );
        if definition.restores_limbs() {
            restore_targeted_stimpak(&mut context.progression, context.limb_target.0);
        }
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
    mut aid_use: AidUseContext,
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
                use_item(key, &mut aid_use, &catalog, &mut notice, &mut sounds);
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
    let physical_screen = sources.presentation.ui_camera().is_some();
    let mut root = commands.spawn((
        PipBoyRoot,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(if physical_screen {
            SCREEN
        } else {
            Color::srgba(0.0, 0.008, 0.003, 0.44)
        }),
        GlobalZIndex(500),
    ));
    if let Some(camera) = sources.presentation.ui_camera() {
        root.insert(UiTargetCamera(camera));
    }
    root.with_children(|root| {
        if physical_screen {
            root.spawn((
                PipBoyScreen,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::axes(Val::Px(42.0), Val::Px(24.0)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(SCREEN),
            ))
            .with_children(|screen| {
                spawn_screen_contents(screen, sources, state, weight);
                spawn_view_buttons(screen, state);
            });
            return;
        }
        root.spawn((
            PipBoyDevice,
            Node {
                width: Val::Percent(76.0),
                height: Val::Percent(94.0),
                max_width: Val::Px(1120.0),
                max_height: Val::Px(1000.0),
                min_width: Val::Px(720.0),
                min_height: Val::Px(650.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(34.0), Val::Px(28.0)),
                border: UiRect::all(Val::Px(7.0)),
                border_radius: BorderRadius::all(Val::Px(52.0)),
                ..default()
            },
            BackgroundColor(BEZEL),
            BorderColor::all(BEZEL_EDGE),
        ))
        .with_children(|device| {
            device
                .spawn((
                    PipBoyScreen,
                    Node {
                        width: Val::Percent(100.0),
                        max_width: Val::Px(1040.0),
                        flex_grow: 1.0,
                        aspect_ratio: Some(4.0 / 3.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::axes(Val::Px(48.0), Val::Px(24.0)),
                        border: UiRect::all(Val::Px(9.0)),
                        border_radius: BorderRadius::all(Val::Px(42.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(SCREEN),
                    BorderColor::all(BEZEL_RECESS),
                ))
                .with_children(|screen| {
                    spawn_screen_contents(screen, sources, state, weight);
                });
            spawn_view_buttons(device, state);
        });
    });
}

fn spawn_screen_contents(
    screen: &mut ChildSpawnerCommands,
    sources: &ScreenSources,
    state: &PipBoyState,
    weight: f32,
) {
    screen.spawn((
        ImageNode {
            image: sources
                .assets
                .load("staging/interface/shared/background/pipboy.ktx2"),
            color: Color::srgba(0.18, 1.0, 0.48, 0.25),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
    ));
    screen.spawn((
        Node {
            position_type: PositionType::Absolute,
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
    spawn_header(screen, state, &sources.status);
    match state.view {
        PipBoyView::Stats => spawn_stats_body(screen, sources, &sources.status),
        PipBoyView::Items => spawn_items_body(screen, sources, state),
        PipBoyView::Data => spawn_data_body(screen, sources, state),
    }
    spawn_footer(screen, state, weight);
    spawn_corner_brackets(screen, 14.0, 34.0, 2.0);
}

/// The stat bar along the top of the screen: the active view's name on the
/// left, then the framed LVL/HP/AP/XP segments from the reference layout.
fn spawn_header(screen: &mut ChildSpawnerCommands, state: &PipBoyState, status: &PlayerStatus) {
    screen
        .spawn((
            PipBoyHeader,
            Node {
                height: Val::Px(52.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                border: UiRect::bottom(Val::Px(1.0)),
                padding: UiRect::bottom(Val::Px(5.0)),
                ..default()
            },
            BorderColor::all(GREEN_DIM),
        ))
        .with_children(|header| {
            header.spawn((
                Text::new(view_label(state.view)),
                TextColor(GREEN),
                TextFont {
                    font_size: FontSize::Px(30.0),
                    ..default()
                },
                glow(),
            ));
            header
                .spawn(Node {
                    column_gap: Val::Px(8.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|segments| {
                    for (label, value) in stat_segments(status) {
                        segments
                            .spawn((
                                Node {
                                    min_width: Val::Px(112.0),
                                    padding: UiRect::axes(Val::Px(10.0), Val::Px(2.0)),
                                    border: UiRect {
                                        top: Val::Px(1.0),
                                        bottom: Val::Px(1.0),
                                        ..default()
                                    },
                                    column_gap: Val::Px(8.0),
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
                                        font_size: FontSize::Px(14.0),
                                        ..default()
                                    },
                                ));
                                segment.spawn((
                                    Text::new(value),
                                    TextColor(GREEN),
                                    TextFont {
                                        font_size: FontSize::Px(22.0),
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
            PipBoyFooter,
            Node {
                height: Val::Px(48.0),
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                border: UiRect::top(Val::Px(1.0)),
                padding: UiRect::top(Val::Px(5.0)),
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
                                font_size: FontSize::Px(16.0),
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
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BorderColor::all(GREEN),
                            BackgroundColor(GREEN_DIM),
                        ))
                        .with_child((
                            Text::new("Status"),
                            TextColor(GREEN),
                            TextFont {
                                font_size: FontSize::Px(18.0),
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
    root.spawn((
        PipBoyButtonBank,
        Node {
            height: Val::Px(112.0),
            width: Val::Percent(66.0),
            min_width: Val::Px(430.0),
            margin: UiRect::top(Val::Px(10.0)),
            padding: UiRect::axes(Val::Px(34.0), Val::Px(8.0)),
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(5.0)),
            border_radius: BorderRadius::all(Val::Px(18.0)),
            ..default()
        },
        BackgroundColor(BEZEL_RECESS),
        BorderColor::all(BEZEL_EDGE),
    ))
    .with_children(|bank| {
        for view in [PipBoyView::Stats, PipBoyView::Items, PipBoyView::Data] {
            let active = state.view == view;
            bank.spawn(Node {
                width: Val::Px(100.0),
                height: Val::Px(88.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|control| {
                control.spawn((
                    Text::new(view_label(view)),
                    TextColor(GREEN_FAINT),
                    TextFont {
                        font_size: FontSize::Px(15.0),
                        ..default()
                    },
                ));
                control.spawn((
                    Button,
                    ViewTab(view),
                    Node {
                        width: Val::Px(58.0),
                        height: Val::Px(58.0),
                        border: UiRect::all(Val::Px(6.0)),
                        border_radius: BorderRadius::all(Val::Percent(50.0)),
                        ..default()
                    },
                    BorderColor::all(BEZEL_EDGE),
                    BackgroundColor(if active { LAMP } else { LAMP_DIM }),
                ));
            });
        }
    });
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
            width: Val::Percent(55.0),
            flex_direction: FlexDirection::Column,
            overflow: Overflow::scroll_y(),
            padding: UiRect::axes(Val::Px(8.0), Val::Px(14.0)),
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
                        min_height: Val::Px(34.0),
                        width: Val::Percent(100.0),
                        padding: UiRect::horizontal(Val::Px(5.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|row| {
                    spawn_selection_marker(row, selected);
                    row.spawn((
                        Text::new(format!(
                            "{}{}{}",
                            equipped_marker(sources.equipment.is_equipped(key)),
                            item_name(item),
                            count_suffix(stack.count)
                        )),
                        TextColor(if selected { GREEN } else { GREEN_FAINT }),
                        TextFont {
                            font_size: FontSize::Px(19.0),
                            ..default()
                        },
                        glow(),
                    ));
                });
            }
        });
        body.spawn(Node {
            width: Val::Percent(45.0),
            padding: UiRect::axes(Val::Px(18.0), Val::Px(14.0)),
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
                                padding: UiRect::all(Val::Px(8.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                margin: UiRect::bottom(Val::Px(14.0)),
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
                                width: Val::Px(176.0),
                                height: Val::Px(176.0),
                                ..default()
                            },
                        ));
                } else {
                    warn!("pipboy missing icon base={:08x}", item.base_form_id);
                    details.spawn((
                        Text::new("[ NO ITEM ART ]"),
                        TextColor(GREEN_DIM),
                        TextFont {
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                        Node {
                            height: Val::Px(192.0),
                            align_content: AlignContent::Center,
                            ..default()
                        },
                    ));
                }
                details.spawn((
                    Text::new(detail_text(stack, item)),
                    TextColor(GREEN),
                    TextFont {
                        font_size: FontSize::Px(17.0),
                        ..default()
                    },
                    glow(),
                    Node {
                        width: Val::Percent(100.0),
                        border: UiRect::top(Val::Px(1.0)),
                        padding: UiRect::top(Val::Px(12.0)),
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
                                padding: UiRect::axes(Val::Px(14.0), Val::Px(4.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BorderColor::all(GREEN),
                            BackgroundColor(GREEN_DIM),
                        ))
                        .with_child((
                            Text::new(label),
                            TextColor(GREEN),
                            TextFont {
                                font_size: FontSize::Px(16.0),
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
                            min_height: Val::Px(34.0),
                            width: Val::Percent(100.0),
                            padding: UiRect::horizontal(Val::Px(8.0)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_child((
                        Text::new(format!("{}{}", item_name(item), count_suffix(stack.count))),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(19.0),
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
                            font_size: FontSize::Px(19.0),
                            ..default()
                        },
                        glow(),
                    ));
                }
                if !sources.status.world_clock_line.is_empty() {
                    body.spawn((
                        Text::new(sources.status.world_clock_line.clone()),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(19.0),
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
                padding: UiRect::axes(Val::Px(12.0), Val::Px(3.0)),
                border: UiRect::all(Val::Px(if active { 1.0 } else { 0.0 })),
                ..default()
            },
            BackgroundColor(if active { GREEN_DIM } else { Color::NONE }),
            BorderColor::all(GREEN),
        ))
        .with_child((
            Text::new(label.to_owned()),
            TextColor(if active { GREEN } else { GREEN_FAINT }),
            TextFont {
                font_size: FontSize::Px(18.0),
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

pub(super) fn detail_text(
    stack: super::inventory::InventoryStack,
    item: &PreparedItemDefinition,
) -> String {
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
#[path = "tests/pipboy.rs"]
mod tests;
