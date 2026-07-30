//! Container transfer modal (issue #75, F75.2/F75.4).
//!
//! Follows `viewer::pipboy`'s established Items-screen pattern rather than a
//! bespoke keyboard-only layout: a dedicated `GameplayModal::Container`
//! state (`OnEnter`/`OnExit` spawn/despawn the screen), the same
//! green-on-dark styling constants, mouse `Interaction`-driven row
//! selection/rebuild-on-change idiom, and item names resolved through
//! `PreparedItemCatalog` (falling back to the container's prepared-entry
//! names, then the hex form id, when the catalog has no entry -- matching
//! this issue's "name + count is enough" scope).
//!
//! History: this reused the otherwise-unused `GameplayModal::Dialogue` slot
//! at wave-2 integration time because `app_state/plugin.rs` was outside this
//! issue's file-ownership boundary. Wave-1's Pip-Boy landed its own
//! dedicated `GameplayModal::PipBoy` variant; this module now gets the same
//! treatment (`GameplayModal::Container`) rather than continuing to borrow
//! `Dialogue`.

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::app_state::{GameplayModal, RequestStateTransition};
use crate::vsa::{PreparedItemCatalog, PreparedItemDefinition};

use super::super::fallout_ui::{
    PHOSPHOR as GREEN, PHOSPHOR_DIM as GREEN_DIM, PHOSPHOR_FAINT as GREEN_FAINT,
    SCREEN_TRANSLUCENT, glow, spawn_corner_brackets, spawn_selection_marker,
};
use super::super::inventory::{DropAction, InventoryStack, StackKey, drop_action};
use super::animation::{self, ClipTransition};
use super::{
    ActiveContainerTarget, CanonicalItemLedger, ContainerStates, InteractionState, PlaySound,
    PlayerEquipment, PlayerInventory, container_policy, item_rules,
};

/// Which side of the transfer a [`TransferQuantityPicker`] is acting on --
/// mirrors `pipboy::QuantityPicker`'s role, but this modal has two possible
/// directions instead of one ("drop").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransferDirection {
    ContainerToPlayer,
    PlayerToContainer,
}

#[derive(Resource, Default)]
struct TransferUiState {
    selected_container: Option<u32>,
    selected_player: Option<StackKey>,
    active_pane: TransferPaneSide,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum TransferPaneSide {
    Player,
    #[default]
    Container,
}

/// Quantity-picker overlay state, opened when a row click's `drop_action`
/// policy (F71's threshold: <=3 units moves one immediately, more opens this
/// picker) calls for a choice. Mirrors `pipboy::QuantityPicker` field-for-
/// field; `condition` carries the selected player stack's identity through
/// to `store` on confirm (container stacks have no condition to carry).
#[derive(Resource, Clone, Copy, Debug)]
struct TransferQuantityPicker {
    direction: TransferDirection,
    form_id: u32,
    condition: Option<u32>,
    quantity: i32,
    max: i32,
}

#[derive(Component)]
struct TransferRoot;

#[derive(Component, Clone, Copy)]
struct ContainerRow(u32);

#[derive(Component, Clone, Copy)]
struct PlayerRow(StackKey);

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

#[derive(Component, Clone, Copy)]
enum TransferActionButton {
    TakeAll,
    Exit,
}

pub(super) fn install(app: &mut App) {
    app.init_resource::<TransferUiState>()
        .add_systems(OnEnter(GameplayModal::Container), open_transfer_modal)
        .add_systems(OnExit(GameplayModal::Container), close_transfer_modal)
        .add_systems(
            Update,
            (
                handle_container_rows,
                handle_player_rows,
                handle_quantity_buttons,
                handle_action_buttons,
                refresh_after_change,
                close_on_escape,
            )
                .run_if(in_state(GameplayModal::Container)),
        );
}

#[allow(clippy::too_many_arguments)]
fn open_transfer_modal(
    mut commands: Commands,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut ui_state: ResMut<TransferUiState>,
    active: Res<ActiveContainerTarget>,
    container_states: Res<ContainerStates>,
    inventory: Res<PlayerInventory>,
    catalog: Res<PreparedItemCatalog>,
    assets: Res<AssetServer>,
) {
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
    *ui_state = TransferUiState::default();
    let Some(active_container) = active.0.as_ref() else {
        return;
    };
    normalize_selection(
        &mut ui_state,
        active_container,
        &container_states,
        &inventory,
    );
    spawn_screen(
        &mut commands,
        active_container,
        &container_states,
        &inventory,
        &catalog,
        &assets,
        &ui_state,
    );
}

/// F75.4: closing the modal (Esc, see `close_on_escape`) also closes the
/// container it was showing -- the close sound/clip, `InteractionState::
/// open` bookkeeping, and clearing `ActiveContainerTarget` all mirror what
/// the pre-modal container branch of `activate_focused_placement` used to
/// do inline.
#[allow(clippy::too_many_arguments)]
fn close_transfer_modal(
    mut commands: Commands,
    roots: Query<Entity, With<TransferRoot>>,
    overlays: Query<Entity, With<QuantityOverlay>>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut active: ResMut<ActiveContainerTarget>,
    mut state: ResMut<InteractionState>,
    transforms: Query<&GlobalTransform>,
    placements: Query<&super::PlacementRoot>,
    mut sounds: MessageWriter<PlaySound>,
    mut animation_playback: MessageWriter<animation::PlayPlacementAnimation>,
) {
    for entity in &roots {
        commands.entity(entity).despawn();
    }
    for entity in &overlays {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<TransferQuantityPicker>();
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }

    let Some(active_container) = active.0.take() else {
        return;
    };
    state.open.remove(&active_container.entity);
    let position = transforms
        .get(active_container.entity)
        .map(|transform| transform.translation())
        .unwrap_or_default();
    if let Ok(root) = placements.get(active_container.entity)
        && let Some(form_id) = root.placement().audio.close_sound_form_id
    {
        sounds.write(PlaySound::container_at(form_id, position));
    }
    animation_playback.write(animation::PlayPlacementAnimation {
        root: active_container.entity,
        transition: ClipTransition::Closing,
        lead_ms: 0.0,
    });
    info!(
        "{} {} ({:08x}) closed",
        active_container.kind.label(),
        active_container.name,
        active_container.reference_form_id
    );
}

fn close_on_escape(
    keys: Res<ButtonInput<KeyCode>>,
    mut requests: MessageWriter<RequestStateTransition>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        requests.write(RequestStateTransition::Modal(GameplayModal::None));
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_action_buttons(
    buttons: Query<(&Interaction, &TransferActionButton), Changed<Interaction>>,
    active: Res<ActiveContainerTarget>,
    mut container_states: ResMut<ContainerStates>,
    mut inventory: ResMut<PlayerInventory>,
    mut canonical: ResMut<CanonicalItemLedger>,
    catalog: Res<PreparedItemCatalog>,
    mut sounds: MessageWriter<PlaySound>,
    mut requests: MessageWriter<RequestStateTransition>,
) {
    for (interaction, action) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            TransferActionButton::Exit => {
                requests.write(RequestStateTransition::Modal(GameplayModal::None));
            }
            TransferActionButton::TakeAll => {
                let Some(active_container) = active.0.as_ref() else {
                    continue;
                };
                let Some(container) = container_states.get_mut(active_container.reference_form_id)
                else {
                    continue;
                };
                let mut played_sound = false;
                for (form_id, result) in take_all(
                    container,
                    &mut inventory,
                    active_container.owner_form_id,
                    active_container.reference_form_id,
                    &mut canonical,
                ) {
                    if log_transfer(
                        result,
                        form_id,
                        active_container.reference_form_id,
                        "-> player",
                    ) && !played_sound
                    {
                        played_sound = write_transfer_sound(
                            &mut sounds,
                            &catalog,
                            form_id,
                            TransferDirection::ContainerToPlayer,
                        );
                    }
                }
            }
        }
    }
}

fn take_all(
    container: &mut container_policy::ContainerState,
    inventory: &mut PlayerInventory,
    owner_form_id: Option<u32>,
    reference_form_id: u32,
    canonical: &mut CanonicalItemLedger,
) -> Vec<(u32, Result<i32, container_policy::TransferError>)> {
    container
        .stacks
        .clone()
        .into_iter()
        .map(|(form_id, count)| {
            (
                form_id,
                take(
                    container,
                    inventory,
                    form_id,
                    count,
                    owner_form_id,
                    reference_form_id,
                    canonical,
                ),
            )
        })
        .collect()
}

/// Sorted `(form_id, count)` view of a container's current stacks -- the
/// pure policy doesn't keep `stacks` sorted after mutation (`apply_delta`
/// updates in place / appends), so the UI sorts on render instead of
/// re-sorting on every mutation.
fn sorted_container_stacks(states: &ContainerStates, reference_form_id: u32) -> Vec<(u32, i32)> {
    let mut stacks = states
        .get(reference_form_id)
        .map(|state| state.stacks.clone())
        .unwrap_or_default();
    stacks.sort_unstable_by_key(|&(form_id, _)| form_id);
    stacks
}

/// Rebuilds selection so it always lands on a still-present row (mirrors
/// `pipboy::normalize_selection`): the first row of each pane when nothing
/// (or a since-emptied stack) is selected.
fn normalize_selection(
    ui_state: &mut TransferUiState,
    active_container: &super::ActiveContainer,
    container_states: &ContainerStates,
    inventory: &PlayerInventory,
) {
    let container_stacks =
        sorted_container_stacks(container_states, active_container.reference_form_id);
    if !container_stacks
        .iter()
        .any(|&(form_id, _)| ui_state.selected_container == Some(form_id))
    {
        ui_state.selected_container = container_stacks.first().map(|&(form_id, _)| form_id);
    }
    let player_stacks = inventory.stack_states();
    if !player_stacks
        .iter()
        .any(|stack| ui_state.selected_player == Some(stack.key()))
    {
        ui_state.selected_player = player_stacks.first().map(|stack| stack.key());
    }
    if container_stacks.is_empty() && !player_stacks.is_empty() {
        ui_state.active_pane = TransferPaneSide::Player;
    } else if player_stacks.is_empty() && !container_stacks.is_empty() {
        ui_state.active_pane = TransferPaneSide::Container;
    }
}

/// Catalog-first name lookup (matches `pipboy::item_name`'s
/// display_name/editor_id/hex fallback chain), falling back to the
/// container's prepared-entry names (leveled-resolved items and anything
/// the catalog doesn't cover) and finally the hex form id.
fn resolve_name(
    form_id: u32,
    catalog: &PreparedItemCatalog,
    prepared_names: &std::collections::HashMap<u32, String>,
) -> String {
    catalog
        .items
        .iter()
        .find(|item| item.base_form_id == form_id)
        .map(catalog_item_name)
        .or_else(|| prepared_names.get(&form_id).cloned())
        .unwrap_or_else(|| format!("{form_id:08x}"))
}

fn catalog_item_name(item: &PreparedItemDefinition) -> String {
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

#[allow(clippy::too_many_arguments)]
fn handle_container_rows(
    mut commands: Commands,
    rows: Query<(&Interaction, &ContainerRow), Changed<Interaction>>,
    active: Res<ActiveContainerTarget>,
    mut container_states: ResMut<ContainerStates>,
    mut inventory: ResMut<PlayerInventory>,
    mut canonical: ResMut<CanonicalItemLedger>,
    catalog: Res<PreparedItemCatalog>,
    mut sounds: MessageWriter<PlaySound>,
    mut ui_state: ResMut<TransferUiState>,
    picker: Option<Res<TransferQuantityPicker>>,
) {
    let Some(active_container) = active.0.as_ref() else {
        return;
    };
    for (interaction, row) in &rows {
        if *interaction != Interaction::Pressed || picker.is_some() {
            continue;
        }
        let form_id = row.0;
        ui_state.selected_container = Some(form_id);
        ui_state.active_pane = TransferPaneSide::Container;
        let Some(container) = container_states.get_mut(active_container.reference_form_id) else {
            return;
        };
        let count = container_policy::stack_count(&container.stacks, form_id);
        match drop_action(count) {
            Some(DropAction::DropOne) => {
                if log_transfer(
                    take(
                        container,
                        &mut inventory,
                        form_id,
                        1,
                        active_container.owner_form_id,
                        active_container.reference_form_id,
                        &mut canonical,
                    ),
                    form_id,
                    active_container.reference_form_id,
                    "-> player",
                ) {
                    write_transfer_sound(
                        &mut sounds,
                        &catalog,
                        form_id,
                        TransferDirection::ContainerToPlayer,
                    );
                }
            }
            Some(DropAction::ChooseQuantity { max, default, .. }) => {
                commands.insert_resource(TransferQuantityPicker {
                    direction: TransferDirection::ContainerToPlayer,
                    form_id,
                    condition: None,
                    quantity: default,
                    max,
                });
                spawn_quantity_picker(&mut commands, default, max);
            }
            None => {}
        }
        return;
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_player_rows(
    mut commands: Commands,
    rows: Query<(&Interaction, &PlayerRow), Changed<Interaction>>,
    active: Res<ActiveContainerTarget>,
    mut container_states: ResMut<ContainerStates>,
    mut inventory: ResMut<PlayerInventory>,
    mut canonical: ResMut<CanonicalItemLedger>,
    equipment: Res<PlayerEquipment>,
    catalog: Res<PreparedItemCatalog>,
    mut sounds: MessageWriter<PlaySound>,
    mut ui_state: ResMut<TransferUiState>,
    picker: Option<Res<TransferQuantityPicker>>,
) {
    let Some(active_container) = active.0.as_ref() else {
        return;
    };
    for (interaction, row) in &rows {
        if *interaction != Interaction::Pressed || picker.is_some() {
            continue;
        }
        let key = row.0;
        ui_state.selected_player = Some(key);
        ui_state.active_pane = TransferPaneSide::Player;
        // Issue #98 (F98.2): equipped items cannot be stored into a container.
        if equipment.is_equipped(key) {
            warn!(
                "container transfer rejected: item is equipped {:08x}",
                key.base_form_id
            );
            return;
        }
        // Issue #81: quest items never enter containers; the rejection happens
        // before store-one or the quantity picker, so the confirm path stays
        // unreachable for them.
        if let Err(rejection) = item_rules::can_store(quest_item(&catalog, key.base_form_id)) {
            warn!(
                "container transfer rejected: {rejection:?} {:08x}",
                key.base_form_id
            );
            return;
        }
        let count = inventory
            .stack_states()
            .into_iter()
            .find(|stack| stack.key() == key)
            .map_or(0, |stack| stack.count);
        let Some(container) = container_states.get_mut(active_container.reference_form_id) else {
            return;
        };
        match drop_action(count) {
            Some(DropAction::DropOne) => {
                if log_transfer(
                    store(
                        container,
                        &mut inventory,
                        key,
                        1,
                        active_container.reference_form_id,
                        &mut canonical,
                    ),
                    key.base_form_id,
                    active_container.reference_form_id,
                    "-> container",
                ) {
                    write_transfer_sound(
                        &mut sounds,
                        &catalog,
                        key.base_form_id,
                        TransferDirection::PlayerToContainer,
                    );
                }
            }
            Some(DropAction::ChooseQuantity { max, default, .. }) => {
                commands.insert_resource(TransferQuantityPicker {
                    direction: TransferDirection::PlayerToContainer,
                    form_id: key.base_form_id,
                    condition: key.condition,
                    quantity: default,
                    max,
                });
                spawn_quantity_picker(&mut commands, default, max);
            }
            None => {}
        }
        return;
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_quantity_buttons(
    mut commands: Commands,
    buttons: Query<(&Interaction, &QuantityButton), Changed<Interaction>>,
    mut picker: Option<ResMut<TransferQuantityPicker>>,
    mut texts: Query<&mut Text, With<QuantityText>>,
    overlays: Query<Entity, With<QuantityOverlay>>,
    active: Res<ActiveContainerTarget>,
    mut container_states: ResMut<ContainerStates>,
    mut inventory: ResMut<PlayerInventory>,
    mut canonical: ResMut<CanonicalItemLedger>,
    catalog: Res<PreparedItemCatalog>,
    mut sounds: MessageWriter<PlaySound>,
) {
    let Some(ref mut picker) = picker else {
        return;
    };
    let Some(active_container) = active.0.as_ref() else {
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
                if let Some(container) =
                    container_states.get_mut(active_container.reference_form_id)
                {
                    let transfer_direction = picker.direction;
                    let (result, form_id, direction_label) = match transfer_direction {
                        TransferDirection::ContainerToPlayer => (
                            take(
                                container,
                                &mut inventory,
                                picker.form_id,
                                picker.quantity,
                                active_container.owner_form_id,
                                active_container.reference_form_id,
                                &mut canonical,
                            ),
                            picker.form_id,
                            "-> player",
                        ),
                        TransferDirection::PlayerToContainer => {
                            let key = StackKey {
                                base_form_id: picker.form_id,
                                condition: picker.condition,
                            };
                            (
                                store(
                                    container,
                                    &mut inventory,
                                    key,
                                    picker.quantity,
                                    active_container.reference_form_id,
                                    &mut canonical,
                                ),
                                picker.form_id,
                                "-> container",
                            )
                        }
                    };
                    if log_transfer(
                        result,
                        form_id,
                        active_container.reference_form_id,
                        direction_label,
                    ) {
                        write_transfer_sound(&mut sounds, &catalog, form_id, transfer_direction);
                    }
                }
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

/// F75.2/F75.4/T75.3: rebuilds the screen whenever the container's stacks or
/// the player's inventory actually changed -- `take`/`store` mutate these
/// resources directly (rather than this system rebuilding inline), so a
/// take/store and a same-frame refresh never race into spawning two screens;
/// see `pipboy::refresh_after_inventory_change` for the same pattern.
#[allow(clippy::too_many_arguments)]
fn refresh_after_change(
    mut commands: Commands,
    active: Res<ActiveContainerTarget>,
    container_states: Res<ContainerStates>,
    inventory: Res<PlayerInventory>,
    catalog: Res<PreparedItemCatalog>,
    assets: Res<AssetServer>,
    mut ui_state: ResMut<TransferUiState>,
    roots: Query<Entity, With<TransferRoot>>,
    picker: Option<Res<TransferQuantityPicker>>,
) {
    if picker.is_some() || !(inventory.is_changed() || container_states.is_changed()) {
        return;
    }
    let Some(active_container) = active.0.as_ref() else {
        return;
    };
    normalize_selection(
        &mut ui_state,
        active_container,
        &container_states,
        &inventory,
    );
    rebuild(
        &mut commands,
        &roots,
        active_container,
        &container_states,
        &inventory,
        &catalog,
        &assets,
        &ui_state,
    );
}

fn quest_item(catalog: &PreparedItemCatalog, form_id: u32) -> bool {
    catalog
        .items
        .iter()
        .find(|item| item.base_form_id == form_id)
        .is_some_and(|item| item.quest_item)
}

fn log_transfer(
    result: Result<i32, container_policy::TransferError>,
    form_id: u32,
    reference_form_id: u32,
    direction: &str,
) -> bool {
    match result {
        Ok(moved) => {
            info!(
                "container transfer: {:08x} x{} {} {:08x}",
                form_id, moved, direction, reference_form_id
            );
            true
        }
        Err(error) => {
            warn!("container transfer rejected: {error:?}");
            false
        }
    }
}

fn write_transfer_sound(
    sounds: &mut MessageWriter<PlaySound>,
    catalog: &PreparedItemCatalog,
    form_id: u32,
    direction: TransferDirection,
) -> bool {
    let action = match direction {
        TransferDirection::ContainerToPlayer => "pickup",
        TransferDirection::PlayerToContainer => "drop",
    };
    let Some(item) = catalog
        .items
        .iter()
        .find(|item| item.base_form_id == form_id)
    else {
        warn!(
            "container transfer audio: item={form_id:08x} action={action} missing item catalog entry"
        );
        return false;
    };
    let Some(request) = transfer_sound_request(catalog, form_id, direction) else {
        warn!(
            "container transfer audio: item={form_id:08x} action={action} missing {} descriptor",
            match direction {
                TransferDirection::ContainerToPlayer => "pickup_sound_form_id",
                TransferDirection::PlayerToContainer => "drop_sound_form_id",
            }
        );
        return false;
    };
    info!(
        "container transfer audio: item={:08x} name={} action={} sound={:08x} queued",
        form_id,
        catalog_item_name(item),
        action,
        request.form_id
    );
    sounds.write(request);
    true
}

fn transfer_sound_request(
    catalog: &PreparedItemCatalog,
    form_id: u32,
    direction: TransferDirection,
) -> Option<PlaySound> {
    let item = catalog
        .items
        .iter()
        .find(|item| item.base_form_id == form_id)?;
    let sound_form_id = match direction {
        TransferDirection::ContainerToPlayer => item.audio.pickup_sound_form_id,
        TransferDirection::PlayerToContainer => item.audio.drop_sound_form_id,
    };
    Some(PlaySound {
        form_id: sound_form_id?,
        position: None,
        gain_db: 0.0,
    })
}

fn close_quantity_picker(commands: &mut Commands, overlays: &Query<Entity, With<QuantityOverlay>>) {
    for entity in overlays {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<TransferQuantityPicker>();
}

/// Container -> player, dispatching to whichever of `container_policy`'s
/// named ops matches the request: exactly the whole stack is `take_all`,
/// exactly one unit is `take_one` (the row-click gesture),
/// anything else (the quantity picker's chosen amount) is `take_stack`.
///
/// Ceiling (follow-up, not this issue): `ContainerState` stores condition-
/// less `(form_id, count)` pairs, so an item taken out always lands in the
/// player's condition-`None` bucket even if a future container source could
/// carry per-stack condition. Condition-aware container stacks are tracked
/// separately from this transfer plumbing.
fn take(
    container: &mut container_policy::ContainerState,
    inventory: &mut PlayerInventory,
    form_id: u32,
    count: i32,
    owner_form_id: Option<u32>,
    reference_form_id: u32,
    canonical: &mut CanonicalItemLedger,
) -> Result<i32, container_policy::TransferError> {
    let available = container_policy::stack_count(&container.stacks, form_id);
    if count <= 0 || count > available {
        return Err(if count <= 0 {
            container_policy::TransferError::NonPositiveCount
        } else {
            container_policy::TransferError::InsufficientSource
        });
    }
    canonical
        .sync_player(&inventory.legacy_snapshot())
        .map_err(|_| container_policy::TransferError::CanonicalTransaction)?;
    canonical
        .ensure_container(reference_form_id, &container.stacks)
        .map_err(|_| container_policy::TransferError::CanonicalTransaction)?;
    let holder = crate::item_transaction::HolderId::FixtureContainer { reference_form_id };
    let source_item = canonical
        .ledger
        .holders()
        .get(&holder)
        .and_then(|state| state.items.iter().find(|item| item.base_form_id == form_id))
        .cloned()
        .ok_or(container_policy::TransferError::CanonicalTransaction)?;
    let moved_condition = source_item.state.condition;
    canonical
        .ledger
        .execute(crate::item_transaction::TransactionRequest::Transfer {
            source: holder,
            destination: crate::item_transaction::HolderId::Player,
            item_id: source_item.id,
            count: u32::try_from(count)
                .map_err(|_| container_policy::TransferError::NonPositiveCount)?,
        })
        .map_err(|_| container_policy::TransferError::CanonicalTransaction)?;
    let mut discard = Vec::new();
    let moved = container_policy::transfer(&mut container.stacks, &mut discard, form_id, count)?;
    // Issue #81 (F81.4): taking from an owned container is theft; no
    // crime/karma consequences in M3, only the stable log line.
    if let item_rules::TakeClassification::Steal { owner_form_id } =
        item_rules::classify_take(owner_form_id)
    {
        info!("steal {:08x} owner {:08x}", form_id, owner_form_id);
    }
    let _ = inventory.add_stack(InventoryStack {
        base_form_id: form_id,
        count: moved,
        condition: moved_condition,
    });
    Ok(moved)
}

/// Player -> container, the mirror of `take` (`store_one`/`store_stack` --
/// F75.1 has no distinct "store all" op; moving the player's full held
/// amount is just `store_stack` with `count` equal to `available`). A
/// scratch vec seeded with the exact stack (`key`) being stored -- not the
/// player's summed count across every condition variant of
/// `key.base_form_id` -- stands in for the missing backing `Vec` on the
/// source side of `container_policy`'s named ops, and the real player-side
/// effect is applied afterward via `remove`. See `take`'s doc comment for
/// the same condition-drop ceiling in this direction: the stored copy loses
/// `key.condition`.
fn store(
    container: &mut container_policy::ContainerState,
    inventory: &mut PlayerInventory,
    key: StackKey,
    count: i32,
    reference_form_id: u32,
    canonical: &mut CanonicalItemLedger,
) -> Result<i32, container_policy::TransferError> {
    let available = inventory
        .stack_states()
        .into_iter()
        .find(|stack| stack.key() == key)
        .map_or(0, |stack| stack.count);
    if count <= 0 {
        return Err(container_policy::TransferError::NonPositiveCount);
    }
    if count > available {
        return Err(container_policy::TransferError::InsufficientSource);
    }
    canonical
        .sync_player(&inventory.legacy_snapshot())
        .map_err(|_| container_policy::TransferError::CanonicalTransaction)?;
    canonical
        .ensure_container(reference_form_id, &container.stacks)
        .map_err(|_| container_policy::TransferError::CanonicalTransaction)?;
    let player_item = canonical
        .ledger
        .holders()
        .get(&crate::item_transaction::HolderId::Player)
        .and_then(|state| {
            state.items.iter().find(|item| {
                item.base_form_id == key.base_form_id
                    && item.state.condition == key.condition
                    && item.count >= u32::try_from(count).unwrap_or_default()
            })
        })
        .cloned()
        .ok_or(container_policy::TransferError::CanonicalTransaction)?;
    canonical
        .ledger
        .execute(crate::item_transaction::TransactionRequest::Transfer {
            source: crate::item_transaction::HolderId::Player,
            destination: crate::item_transaction::HolderId::FixtureContainer { reference_form_id },
            item_id: player_item.id,
            count: u32::try_from(count)
                .map_err(|_| container_policy::TransferError::NonPositiveCount)?,
        })
        .map_err(|_| container_policy::TransferError::CanonicalTransaction)?;
    let mut scratch = vec![(key.base_form_id, available)];
    let moved =
        container_policy::transfer(&mut scratch, &mut container.stacks, key.base_form_id, count)?;
    let _ = inventory.remove(key, moved);
    Ok(moved)
}

#[allow(clippy::too_many_arguments)]
fn rebuild(
    commands: &mut Commands,
    roots: &Query<Entity, With<TransferRoot>>,
    active_container: &super::ActiveContainer,
    container_states: &ContainerStates,
    inventory: &PlayerInventory,
    catalog: &PreparedItemCatalog,
    assets: &AssetServer,
    ui_state: &TransferUiState,
) {
    for root in roots {
        commands.entity(root).despawn();
    }
    spawn_screen(
        commands,
        active_container,
        container_states,
        inventory,
        catalog,
        assets,
        ui_state,
    );
}

fn selected_detail<'a>(
    active_container: &super::ActiveContainer,
    container_states: &ContainerStates,
    inventory: &PlayerInventory,
    catalog: &'a PreparedItemCatalog,
    ui_state: &TransferUiState,
) -> Option<(InventoryStack, &'a PreparedItemDefinition)> {
    let stack = match ui_state.active_pane {
        TransferPaneSide::Player => inventory
            .stack_states()
            .into_iter()
            .find(|stack| ui_state.selected_player == Some(stack.key()))?,
        TransferPaneSide::Container => {
            let form_id = ui_state.selected_container?;
            let count = container_states
                .get(active_container.reference_form_id)
                .map(|container| container_policy::stack_count(&container.stacks, form_id))
                .unwrap_or_default();
            InventoryStack {
                base_form_id: form_id,
                count,
                condition: None,
            }
        }
    };
    catalog
        .items
        .iter()
        .find(|item| item.base_form_id == stack.base_form_id)
        .map(|item| (stack, item))
}

fn spawn_screen(
    commands: &mut Commands,
    active_container: &super::ActiveContainer,
    container_states: &ContainerStates,
    inventory: &PlayerInventory,
    catalog: &PreparedItemCatalog,
    assets: &AssetServer,
    ui_state: &TransferUiState,
) {
    let container_stacks =
        sorted_container_stacks(container_states, active_container.reference_form_id);
    let player_stacks = inventory.stack_states();
    let carried_weight = inventory.total_weight(|form_id| {
        catalog
            .items
            .iter()
            .find(|item| item.base_form_id == form_id)
            .and_then(|item| item_rules::carried_weight(item.quest_item, item.weight))
    });
    let detail = selected_detail(
        active_container,
        container_states,
        inventory,
        catalog,
        ui_state,
    );

    commands
        .spawn((
            TransferRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.008, 0.003, 0.28)),
            GlobalZIndex(500),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Percent(58.0),
                    height: Val::Percent(68.0),
                    max_width: Val::Px(1080.0),
                    max_height: Val::Px(720.0),
                    min_width: Val::Px(760.0),
                    min_height: Val::Px(480.0),
                    padding: UiRect::all(Val::Px(28.0)),
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(SCREEN_TRANSLUCENT),
                BorderColor::all(GREEN_DIM),
            ))
            .with_children(|panel| {
                panel
                    .spawn(Node {
                        flex_grow: 1.0,
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(42.0),
                        ..default()
                    })
                    .with_children(|body| {
                        spawn_pane(
                            body,
                            "<  ITEMS  >",
                            Some(format!("WG  {carried_weight:.1}")),
                            player_stacks.iter().map(|stack| {
                                (
                                    PlayerRow(stack.key()),
                                    resolve_name(
                                        stack.base_form_id,
                                        catalog,
                                        &active_container.item_names,
                                    ),
                                    stack.count,
                                    ui_state.active_pane == TransferPaneSide::Player
                                        && ui_state.selected_player == Some(stack.key()),
                                )
                            }),
                        );
                        spawn_pane(
                            body,
                            &format!("<  {}  >", active_container.name),
                            None,
                            container_stacks.iter().map(|&(form_id, count)| {
                                (
                                    ContainerRow(form_id),
                                    resolve_name(form_id, catalog, &active_container.item_names),
                                    count,
                                    ui_state.active_pane == TransferPaneSide::Container
                                        && ui_state.selected_container == Some(form_id),
                                )
                            }),
                        );
                    });
                panel
                    .spawn((
                        Node {
                            height: Val::Px(184.0),
                            width: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            border: UiRect::bottom(Val::Px(1.0)),
                            padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                            ..default()
                        },
                        BorderColor::all(GREEN_DIM),
                    ))
                    .with_children(|footer| {
                        footer
                            .spawn(Node {
                                width: Val::Percent(72.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(28.0),
                                ..default()
                            })
                            .with_children(|details| {
                                if let Some((stack, item)) = detail {
                                    if let Some(path) = item.icon_asset_path.as_deref() {
                                        details.spawn((
                                            ImageNode {
                                                image: assets.load(path.to_owned()),
                                                color: GREEN,
                                                ..default()
                                            },
                                            Node {
                                                width: Val::Px(168.0),
                                                height: Val::Px(168.0),
                                                flex_shrink: 0.0,
                                                ..default()
                                            },
                                        ));
                                    }
                                    let text = super::super::pipboy::detail_text(stack, item)
                                        .lines()
                                        .skip(1)
                                        .collect::<Vec<_>>()
                                        .join("\n");
                                    details.spawn((
                                        Text::new(text),
                                        TextColor(GREEN),
                                        TextFont {
                                            font_size: FontSize::Px(17.0),
                                            ..default()
                                        },
                                        glow(),
                                    ));
                                }
                            });
                        footer
                            .spawn(Node {
                                width: Val::Percent(28.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::FlexEnd,
                                row_gap: Val::Px(8.0),
                                ..default()
                            })
                            .with_children(|actions| {
                                transfer_action_button(
                                    actions,
                                    "TAKE ALL  A",
                                    TransferActionButton::TakeAll,
                                );
                                transfer_action_button(
                                    actions,
                                    "EXIT  ESC",
                                    TransferActionButton::Exit,
                                );
                            });
                    });
                spawn_corner_brackets(panel, 0.0, 28.0, 2.0);
            });
        });
}

/// Spawns one pane (container or player) of rows -- `row` carries the marker
/// component (`ContainerRow`/`PlayerRow`) each row entity gets so
/// `handle_container_rows`/`handle_player_rows` can identify which stack a
/// click/hover landed on.
fn spawn_pane<Row: Component + Copy>(
    body: &mut ChildSpawnerCommands,
    title: &str,
    auxiliary: Option<String>,
    rows: impl Iterator<Item = (Row, String, i32, bool)>,
) {
    body.spawn((
        Node {
            width: Val::Percent(50.0),
            flex_direction: FlexDirection::Column,
            overflow: Overflow::clip_y(),
            border: UiRect {
                top: Val::Px(1.0),
                left: Val::Px(1.0),
                right: Val::Px(1.0),
                bottom: Val::Px(1.0),
            },
            ..default()
        },
        BorderColor::all(GREEN_DIM),
    ))
    .with_children(|list| {
        list.spawn(Node {
            min_height: Val::Px(52.0),
            width: Val::Percent(100.0),
            padding: UiRect::horizontal(Val::Px(18.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|header| {
            header.spawn((
                Text::new(title.to_owned()),
                TextColor(GREEN),
                TextFont {
                    font_size: FontSize::Px(21.0),
                    ..default()
                },
                glow(),
            ));
            if let Some(auxiliary) = auxiliary {
                header.spawn((
                    Text::new(auxiliary),
                    TextColor(GREEN_FAINT),
                    TextFont {
                        font_size: FontSize::Px(17.0),
                        ..default()
                    },
                ));
            }
        });
        list.spawn((
            Text::new("^"),
            TextColor(GREEN),
            TextFont {
                font_size: FontSize::Px(18.0),
                ..default()
            },
            Node {
                height: Val::Px(22.0),
                padding: UiRect::left(Val::Px(6.0)),
                ..default()
            },
        ));
        let mut any = false;
        list.spawn(Node {
            flex_grow: 1.0,
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            overflow: Overflow::scroll_y(),
            padding: UiRect::horizontal(Val::Px(16.0)),
            ..default()
        })
        .with_children(|rows_parent| {
            for (row, name, count, selected) in rows {
                any = true;
                rows_parent
                    .spawn((
                        Button,
                        row,
                        Node {
                            min_height: Val::Px(34.0),
                            width: Val::Percent(100.0),
                            padding: UiRect::horizontal(Val::Px(4.0)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|row| {
                        spawn_selection_marker(row, selected);
                        row.spawn((
                            Text::new(format!("{name}{}", count_suffix(count))),
                            TextColor(if selected { GREEN } else { GREEN_FAINT }),
                            TextFont {
                                font_size: FontSize::Px(19.0),
                                ..default()
                            },
                            glow(),
                        ));
                    });
            }
            if !any {
                rows_parent.spawn((
                    Text::new("(empty)"),
                    TextColor(GREEN_DIM),
                    TextFont {
                        font_size: FontSize::Px(18.0),
                        ..default()
                    },
                ));
            }
        });
        list.spawn((
            Text::new("v"),
            TextColor(GREEN),
            TextFont {
                font_size: FontSize::Px(18.0),
                ..default()
            },
            Node {
                height: Val::Px(22.0),
                padding: UiRect::left(Val::Px(6.0)),
                ..default()
            },
        ));
    });
}

fn transfer_action_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: TransferActionButton,
) {
    parent
        .spawn((
            Button,
            action,
            Node {
                padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_child((
            Text::new(label.to_owned()),
            TextColor(GREEN),
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
                ..default()
            },
            BackgroundColor(SCREEN_TRANSLUCENT),
            BorderColor::all(GREEN),
            GlobalZIndex(520),
        ))
        .with_children(|overlay| {
            overlay.spawn((
                Text::new("MOVE HOW MANY?"),
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
                    quantity_button(row, "-", QuantityButton::Minus);
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
                    quantity_button(row, "MOVE", QuantityButton::Confirm);
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

#[cfg(test)]
#[path = "tests/transfer_ui.rs"]
mod tests;

