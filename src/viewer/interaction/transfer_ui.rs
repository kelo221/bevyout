//! Container transfer modal (issue #75, F75.2/F75.4).
//!
//! Follows the same `GameplayModal`-gated overlay pattern as
//! `viewer::console_ui`: a hidden UI root toggled by `OnEnter`/`OnExit` on
//! the modal, cursor released while it's up, gameplay input already blocked
//! everywhere else in the viewer by the pre-existing `run_if(in_state(
//! GameplayModal::None))` gates (see `interaction::install`).
//!
//! Deviation from the plan (documented for the orchestrator): `GameplayModal`
//! has no dedicated `Container` variant, and `app_state/plugin.rs` is
//! outside this issue's file-ownership boundary. This reuses the existing,
//! otherwise-unused `GameplayModal::Dialogue` slot -- already a legal
//! `None <-> Dialogue` transition in `LEGAL_MODAL_TRANSITIONS` -- rather
//! than adding a new variant. If a later issue wants `Dialogue` for actual
//! NPC conversations, `app_state/plugin.rs` gains a dedicated
//! `GameplayModal::Container` variant and this module's `GameplayModal::
//! Dialogue` references become `GameplayModal::Container`; nothing else
//! about this module's structure changes.

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::app_state::{GameplayModal, RequestStateTransition};

use super::animation::{self, ClipTransition};
use super::{
    ActiveContainerTarget, ContainerStates, InteractionState, PlaySound, PlayerInventory,
    container_policy,
};

/// F75.1's "take/store a stack" ops move up to this many units per
/// keypress, clamped to what's actually available -- a fixed amount rather
/// than a numeric-entry widget, matching this issue's keyboard-only,
/// name+count UI scope (no item art/detailed stats, no barter).
const STACK_TRANSFER_SIZE: i32 = 10;

#[derive(Component)]
struct TransferRoot;

#[derive(Component)]
struct TransferTitleText;

#[derive(Component)]
struct TransferContainerText;

#[derive(Component)]
struct TransferPlayerText;

#[derive(Component)]
struct TransferHintText;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransferPane {
    Container,
    Player,
}

#[derive(Resource)]
struct TransferUiState {
    focus: TransferPane,
    container_index: usize,
    player_index: usize,
}

impl Default for TransferUiState {
    fn default() -> Self {
        Self {
            focus: TransferPane::Container,
            container_index: 0,
            player_index: 0,
        }
    }
}

pub(super) fn install(app: &mut App) {
    app.init_resource::<TransferUiState>()
        .add_systems(Startup, spawn_transfer_ui)
        .add_systems(OnEnter(GameplayModal::Dialogue), open_transfer_modal)
        .add_systems(OnExit(GameplayModal::Dialogue), close_transfer_modal)
        .add_systems(
            Update,
            (handle_transfer_input, close_on_escape).run_if(in_state(GameplayModal::Dialogue)),
        );
}

fn spawn_transfer_ui(mut commands: Commands) {
    commands
        .spawn((
            TransferRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(15.0),
                right: Val::Percent(15.0),
                top: Val::Percent(12.0),
                bottom: Val::Percent(12.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
            ZIndex(500),
            Visibility::Hidden,
        ))
        .with_children(|root| {
            root.spawn((
                TransferTitleText,
                Text::new(""),
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(8.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ));
            root.spawn((
                TransferContainerText,
                Text::new(""),
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(40.0),
                    left: Val::Px(24.0),
                    width: Val::Percent(45.0),
                    ..default()
                },
            ));
            root.spawn((
                TransferPlayerText,
                Text::new(""),
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(40.0),
                    right: Val::Px(24.0),
                    width: Val::Percent(45.0),
                    ..default()
                },
            ));
            root.spawn((
                TransferHintText,
                Text::new(
                    "Up/Down select, Left/Right switch side, Enter take/store one, \
                     A all, S stack, Esc close",
                ),
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(8.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ));
        });
}

fn open_transfer_modal(
    mut roots: Query<&mut Visibility, With<TransferRoot>>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut ui_state: ResMut<TransferUiState>,
    mut time: ResMut<Time<Virtual>>,
) {
    for mut visibility in &mut roots {
        *visibility = Visibility::Inherited;
    }
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
    *ui_state = TransferUiState::default();
    time.pause();
}

/// F75.4: closing the modal (Esc, see `close_on_escape`) also closes the
/// container it was showing -- the close sound/clip, `InteractionState::
/// open` bookkeeping, and clearing `ActiveContainerTarget` all mirror what
/// the pre-modal container branch of `activate_focused_placement` used to
/// do inline.
#[allow(clippy::too_many_arguments)]
fn close_transfer_modal(
    mut roots: Query<&mut Visibility, With<TransferRoot>>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut time: ResMut<Time<Virtual>>,
    mut active: ResMut<ActiveContainerTarget>,
    mut state: ResMut<InteractionState>,
    transforms: Query<&GlobalTransform>,
    placements: Query<&super::PlacementRoot>,
    mut sounds: MessageWriter<PlaySound>,
    mut animation_playback: MessageWriter<animation::PlayPlacementAnimation>,
) {
    for mut visibility in &mut roots {
        *visibility = Visibility::Hidden;
    }
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }
    time.unpause();

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
        sounds.write(PlaySound::at(form_id, position));
    }
    animation_playback.write(animation::PlayPlacementAnimation {
        root: active_container.entity,
        transition: ClipTransition::Closing,
        lead_ms: 0.0,
    });
    info!(
        "container {} ({:08x}) closed",
        active_container.name, active_container.reference_form_id
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

fn stack_label(form_id: u32, count: i32, names: &std::collections::HashMap<u32, String>) -> String {
    let name = names
        .get(&form_id)
        .cloned()
        .unwrap_or_else(|| format!("{form_id:08x}"));
    format!("{name} x{count}")
}

fn render_pane(
    title: &str,
    stacks: &[(u32, i32)],
    names: &std::collections::HashMap<u32, String>,
    selected: usize,
    focused: bool,
) -> String {
    let mut lines = vec![format!("{title}{}", if focused { " <" } else { "" })];
    if stacks.is_empty() {
        lines.push("  (empty)".into());
    }
    for (index, &(form_id, count)) in stacks.iter().enumerate() {
        let marker = if focused && index == selected {
            "> "
        } else {
            "  "
        };
        lines.push(format!("{marker}{}", stack_label(form_id, count, names)));
    }
    lines.join("\n")
}

/// Clamps `index` into `[0, len)` (or `0` when `len == 0`).
fn clamp_index(index: usize, len: usize) -> usize {
    if len == 0 { 0 } else { index.min(len - 1) }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn handle_transfer_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<TransferUiState>,
    active: Res<ActiveContainerTarget>,
    mut container_states: ResMut<ContainerStates>,
    mut inventory: ResMut<PlayerInventory>,
    // One `Query<&mut Text, With<X>>` per pane, in a `ParamSet` -- all three
    // target the same `Text` component on disjoint marker-filtered
    // entities, which Bevy's per-system query-conflict check does not infer
    // from distinct `With<...>` types alone (runtime panic otherwise).
    mut texts: ParamSet<(
        Query<&mut Text, With<TransferTitleText>>,
        Query<&mut Text, With<TransferContainerText>>,
        Query<&mut Text, With<TransferPlayerText>>,
    )>,
) {
    let Some(active_container) = active.0.as_ref() else {
        return;
    };

    // Read the whole (`Copy`) selection state into locals up front and only
    // ever write back through direct field assignment (never a stored
    // `&mut ui_state.<field>`) -- `ui_state` is a `ResMut`, so holding a
    // field reference across later `ui_state.focus`/`ui_state.<index>`
    // reads would be a live mutable borrow across those reads.
    let mut focus = ui_state.focus;
    if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::ArrowRight) {
        focus = match focus {
            TransferPane::Container => TransferPane::Player,
            TransferPane::Player => TransferPane::Container,
        };
    }
    let mut container_index = ui_state.container_index;
    let mut player_index = ui_state.player_index;

    let mut container_stacks =
        sorted_container_stacks(&container_states, active_container.reference_form_id);
    let mut player_stacks = inventory.stacks();

    {
        let (selected, pane_len) = match focus {
            TransferPane::Container => (&mut container_index, container_stacks.len()),
            TransferPane::Player => (&mut player_index, player_stacks.len()),
        };
        if keys.just_pressed(KeyCode::ArrowDown) && *selected + 1 < pane_len {
            *selected += 1;
        }
        if keys.just_pressed(KeyCode::ArrowUp) && *selected > 0 {
            *selected -= 1;
        }
        *selected = clamp_index(*selected, pane_len);
    }

    let amount = if keys.just_pressed(KeyCode::Enter) {
        Some(TransferAmount::One)
    } else if keys.just_pressed(KeyCode::KeyS) {
        Some(TransferAmount::Stack)
    } else if keys.just_pressed(KeyCode::KeyA) {
        Some(TransferAmount::All)
    } else {
        None
    };

    let selected_form_id = match focus {
        TransferPane::Container => container_stacks.get(container_index).map(|&(id, _)| id),
        TransferPane::Player => player_stacks.get(player_index).map(|&(id, _)| id),
    };
    if let Some(amount) = amount
        && let Some(form_id) = selected_form_id
        && let Some(container) = container_states.get_mut(active_container.reference_form_id)
    {
        let result = match focus {
            TransferPane::Container => take(container, &mut inventory, form_id, amount),
            TransferPane::Player => store(container, &mut inventory, form_id, amount),
        };
        match result {
            Ok(moved) => info!(
                "container transfer: {:08x} x{} {} {:08x}",
                form_id,
                moved,
                match focus {
                    TransferPane::Container => "-> player",
                    TransferPane::Player => "-> container",
                },
                active_container.reference_form_id
            ),
            Err(error) => warn!("container transfer rejected: {error:?}"),
        }
        container_stacks =
            sorted_container_stacks(&container_states, active_container.reference_form_id);
        player_stacks = inventory.stacks();
        container_index = clamp_index(container_index, container_stacks.len());
        player_index = clamp_index(player_index, player_stacks.len());
    }

    ui_state.focus = focus;
    ui_state.container_index = container_index;
    ui_state.player_index = player_index;

    if let Ok(mut text) = texts.p0().single_mut() {
        text.0 = active_container.name.clone();
    }
    if let Ok(mut text) = texts.p1().single_mut() {
        text.0 = render_pane(
            "Container",
            &container_stacks,
            &active_container.item_names,
            container_index,
            focus == TransferPane::Container,
        );
    }
    if let Ok(mut text) = texts.p2().single_mut() {
        text.0 = render_pane(
            "You",
            &player_stacks,
            &active_container.item_names,
            player_index,
            focus == TransferPane::Player,
        );
    }
}

/// Which of F75.1's five named transfer ops a keypress requests --
/// `Enter`/`KeyS`/`KeyA` map to one/stack/all respectively, in either
/// direction depending on which pane is focused (see `handle_transfer_input`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransferAmount {
    One,
    Stack,
    All,
}

/// Container -> player. Delegates to `container_policy`'s named ops
/// (`take_one`/`take_stack`/`take_all`) against the container's real
/// `stacks` and a throwaway destination scratch vec -- the player side has
/// no backing `Vec` to hand the pure policy (only the #75/#71 `grant`/
/// `remove` API), so the actual player-side effect is applied afterward via
/// `grant`, keyed off the amount the policy validated and moved.
fn take(
    container: &mut container_policy::ContainerState,
    inventory: &mut PlayerInventory,
    form_id: u32,
    amount: TransferAmount,
) -> Result<i32, container_policy::TransferError> {
    let mut discard = Vec::new();
    let moved = match amount {
        TransferAmount::One => {
            container_policy::take_one(&mut container.stacks, &mut discard, form_id)
        }
        TransferAmount::Stack => {
            let count =
                STACK_TRANSFER_SIZE.min(container_policy::stack_count(&container.stacks, form_id));
            container_policy::take_stack(&mut container.stacks, &mut discard, form_id, count)
        }
        TransferAmount::All => {
            container_policy::take_all(&mut container.stacks, &mut discard, form_id)
        }
    }?;
    inventory.grant(form_id, moved);
    Ok(moved)
}

/// Player -> container, the mirror of `take`: a scratch vec seeded with the
/// player's real current count (from `PlayerInventory::count`) stands in
/// for the missing backing `Vec` on the source side of `container_policy`'s
/// named ops, and the real player-side effect is applied afterward via
/// `remove`. F75.1 has no "store all" op distinct from "store stack" --
/// `TransferAmount::All` just uses the player's full held count as the
/// requested stack size.
fn store(
    container: &mut container_policy::ContainerState,
    inventory: &mut PlayerInventory,
    form_id: u32,
    amount: TransferAmount,
) -> Result<i32, container_policy::TransferError> {
    let available = inventory.count(form_id);
    let mut scratch = vec![(form_id, available)];
    let moved = match amount {
        TransferAmount::One => {
            container_policy::store_one(&mut scratch, &mut container.stacks, form_id)
        }
        TransferAmount::Stack => {
            let count = STACK_TRANSFER_SIZE.min(available);
            container_policy::store_stack(&mut scratch, &mut container.stacks, form_id, count)
        }
        TransferAmount::All => {
            container_policy::store_stack(&mut scratch, &mut container.stacks, form_id, available)
        }
    }?;
    inventory.remove(form_id, moved);
    Ok(moved)
}
