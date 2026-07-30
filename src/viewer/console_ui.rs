//! In-game grave-key console frontend.

use std::path::PathBuf;

use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseWheel;
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy::prelude::*;
use bevy::text::{EditableText, EditableTextFilter, TextCursorStyle, TextEdit};
use bevy::transform::TransformSystems;
use bevy::ui::UiSystems;
use bevy::ui_widgets::ScrollArea;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::app_state::GameplayModal;
use crate::console::openmw_ui::{
    CommandHistory, CompletionState, ConsoleTranscript, is_clear_submission, load_history,
    save_history,
};
use crate::console::{
    ConsoleQueue, ConsoleRegistry, ConsoleResponses, ConsoleSessionId, ConsoleSessionStore,
    ConsoleSystemSet, RefRegistry,
};

use super::interaction::{PlacementRoot, find_placement_root};

const UI_SESSION: &str = "ui";
const INPUT_STRIP_HEIGHT: f32 = 42.0;

#[derive(Component)]
struct ConsoleRoot;

#[derive(Component)]
struct ConsoleTitle;

#[derive(Component)]
struct ConsoleScrollback;

#[derive(Component)]
struct ConsoleScrollbackViewport;

#[derive(Component)]
struct ConsoleInput;

#[derive(Resource)]
struct ConsoleUiState {
    history: CommandHistory,
    completion: CompletionState,
    transcript: ConsoleTranscript,
    transcript_dirty: bool,
    history_path: PathBuf,
    pick_cycle: ConsolePickCycle,
}

#[derive(Default)]
struct ConsolePickCycle {
    /// Unique placement roots ordered from the camera toward the back of the
    /// scene along the last pixel clicked, matching the Gamebryo console.
    candidates: Vec<Entity>,
}

pub(crate) struct ConsoleUiPlugin;

impl Plugin for ConsoleUiPlugin {
    fn build(&self, app: &mut App) {
        install(app);
    }
}

fn install(app: &mut App) {
    let history_path = PathBuf::from(".bevyout/console_history.txt");
    let history = load_history(&history_path).unwrap_or_else(|error| {
        warn!("could not load console history: {error}");
        CommandHistory::default()
    });
    app.insert_resource(ConsoleUiState {
        history,
        completion: CompletionState::default(),
        transcript: ConsoleTranscript::default(),
        transcript_dirty: false,
        history_path,
        pick_cycle: ConsolePickCycle::default(),
    })
    .add_systems(Startup, spawn_console_ui)
    .add_systems(OnEnter(GameplayModal::Console), open_console_ui)
    .add_systems(OnExit(GameplayModal::Console), close_console_ui)
    .add_systems(
        Update,
        (
            select_reference_with_mouse,
            handle_console_input,
            update_console_title,
        )
            .chain()
            .in_set(super::plugins::ViewerSet::Ui)
            .run_if(in_state(GameplayModal::Console)),
    )
    .add_systems(
        Update,
        log_console_key_events.run_if(in_state(GameplayModal::Console)),
    )
    .add_systems(
        PostUpdate,
        (consume_console_responses, render_console_scrollback)
            .chain()
            .after(ConsoleSystemSet::Execute)
            .before(UiSystems::Content),
    )
    .add_systems(
        PostUpdate,
        settle_console_scrollback_position
            .after(UiSystems::Layout)
            .before(TransformSystems::Propagate),
    );
}

fn spawn_console_ui(mut commands: Commands) {
    commands
        .spawn((
            ConsoleRoot,
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                right: px(0),
                top: px(0),
                bottom: px(0),
                ..default()
            },
            ZIndex(1000),
            Visibility::Hidden,
        ))
        .with_children(|root| {
            root.spawn((
                ConsoleTitle,
                Text::new(""),
                TextLayout::justify(Justify::Center),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    top: px(8),
                    left: px(0),
                    width: percent(100),
                    ..default()
                },
            ));
            root.spawn((
                ConsoleScrollbackViewport,
                ScrollArea,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(40),
                    bottom: px(INPUT_STRIP_HEIGHT),
                    width: percent(80),
                    height: percent(50),
                    overflow: Overflow::scroll_y(),
                    ..default()
                },
            ))
            .with_child((
                ConsoleScrollback,
                Text::new(""),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                Node {
                    width: percent(100),
                    ..default()
                },
            ));
            root.spawn((
                ConsoleInput,
                EditableText {
                    max_characters: Some(2048),
                    visible_width: Some(100.0),
                    allow_newlines: false,
                    ..default()
                },
                EditableTextFilter::new(console_input_character_allowed),
                TextCursorStyle::default(),
                TextLayout::no_wrap(),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    left: px(40),
                    bottom: px(10),
                    width: percent(80),
                    overflow: Overflow::clip_x(),
                    ..default()
                },
            ));
        });
}

fn open_console_ui(
    mut roots: Query<&mut Visibility, With<ConsoleRoot>>,
    inputs: Query<Entity, With<ConsoleInput>>,
    mut focus: ResMut<InputFocus>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    for mut visibility in &mut roots {
        *visibility = Visibility::Inherited;
    }
    if let Ok(input) = inputs.single() {
        focus.set(input, FocusCause::Navigated);
    }
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
}

fn close_console_ui(
    mut roots: Query<&mut Visibility, With<ConsoleRoot>>,
    mut inputs: Query<&mut EditableText, With<ConsoleInput>>,
    mut focus: ResMut<InputFocus>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    for mut visibility in &mut roots {
        *visibility = Visibility::Hidden;
    }
    focus.clear();
    if let Ok(mut input) = inputs.single_mut() {
        let draft = editable_value(&input);
        let sanitized = sanitize_console_draft(&draft);
        if draft != sanitized {
            replace_editable(&mut input, &sanitized);
        }
    }
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }
}

fn sanitize_console_draft(draft: &str) -> String {
    draft
        .chars()
        .filter(|&character| console_input_character_allowed(character))
        .collect()
}

fn console_input_character_allowed(character: char) -> bool {
    !matches!(character, '`' | '~' | '\n' | '\r')
}

fn selected_reference_text(
    form_id: Option<u32>,
    editor_id: Option<&str>,
    display_name: Option<&str>,
) -> String {
    let Some(form_id) = form_id else {
        return String::new();
    };
    editor_id
        .filter(|value| !value.is_empty())
        .or_else(|| display_name.filter(|value| !value.is_empty()))
        .map_or_else(
            || format!("({form_id:08x})"),
            |label| format!("{label} ({form_id:08x})"),
        )
}

fn next_pick_index(
    current: Option<usize>,
    candidate_count: usize,
    wheel_delta: f32,
) -> Option<usize> {
    if candidate_count == 0 || wheel_delta == 0.0 {
        return None;
    }
    Some(match (current, wheel_delta.is_sign_positive()) {
        (Some(index), true) => (index + 1) % candidate_count,
        (Some(index), false) => (index + candidate_count - 1) % candidate_count,
        (None, true) => 0,
        (None, false) => candidate_count - 1,
    })
}

fn console_pick_settings() -> MeshRayCastSettings<'static> {
    MeshRayCastSettings::default()
        .with_visibility(RayCastVisibility::VisibleInView)
        .never_early_exit()
}

#[allow(clippy::too_many_arguments)]
fn select_reference_with_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    mut wheel: MessageReader<MouseWheel>,
    windows: Query<&Window, With<PrimaryWindow>>,
    scrollback_viewport: Query<
        (&ComputedNode, &UiGlobalTransform),
        With<ConsoleScrollbackViewport>,
    >,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut raycast: MeshRayCast,
    parents: Query<&ChildOf>,
    roots: Query<&PlacementRoot>,
    references: Res<RefRegistry>,
    mut sessions: ResMut<ConsoleSessionStore>,
    mut ui: ResMut<ConsoleUiState>,
) {
    let wheel_delta = wheel.read().map(|event| event.y).sum::<f32>();
    let cursor = windows
        .single()
        .ok()
        .and_then(|window| window.cursor_position());
    let cursor_over_scrollback = cursor.is_some_and(|cursor| {
        scrollback_viewport
            .single()
            .is_ok_and(|(node, transform)| node.contains_point(*transform, cursor))
    });
    if !buttons.just_pressed(MouseButton::Left) {
        if wheel_delta == 0.0 {
            return;
        }
        if cursor_over_scrollback {
            return;
        }
        ui.pick_cycle.candidates.retain(|candidate| {
            roots.contains(*candidate) && references.identity(*candidate).is_some()
        });
        let session = ConsoleSessionId::new(UI_SESSION);
        let current = sessions.selected(&session).and_then(|selected| {
            ui.pick_cycle
                .candidates
                .iter()
                .position(|candidate| *candidate == selected)
        });
        let Some(next) = next_pick_index(current, ui.pick_cycle.candidates.len(), wheel_delta)
        else {
            return;
        };
        sessions.select(session, ui.pick_cycle.candidates[next]);
        return;
    }
    let Some(cursor) = cursor else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };
    if cursor.y >= window.height() - INPUT_STRIP_HEIGHT {
        return;
    }
    let ray = cameras.iter().find_map(|(camera, transform)| {
        camera
            .is_active
            .then(|| camera.viewport_to_world(transform, cursor).ok())
            .flatten()
    });
    let Some(ray) = ray else {
        return;
    };
    let settings = console_pick_settings();
    let mut candidates = Vec::new();
    let mut saw_unregistered_geometry = false;
    for (hit, _) in raycast.cast_ray(ray, &settings) {
        let Some(root) = find_placement_root(*hit, &parents, &roots) else {
            saw_unregistered_geometry = true;
            continue;
        };
        if references.identity(root).is_some() && !candidates.contains(&root) {
            candidates.push(root);
        }
    }
    let Some(first) = candidates.first().copied() else {
        ui.pick_cycle.candidates.clear();
        if saw_unregistered_geometry {
            info!("console pick: NOT_IMPLEMENTED (static-batched geometry)");
        }
        return;
    };
    ui.pick_cycle.candidates = candidates;
    sessions.select(ConsoleSessionId::new(UI_SESSION), first);
}

fn editable_value(editable: &EditableText) -> String {
    editable.value().into_iter().collect()
}

fn replace_editable(editable: &mut EditableText, value: &str) {
    editable.editor_mut().set_text(value);
    editable.queue_edit(TextEdit::TextEnd(false));
}

fn handle_console_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut inputs: Query<&mut EditableText, With<ConsoleInput>>,
    mut ui: ResMut<ConsoleUiState>,
    registry: Res<ConsoleRegistry>,
    references: Res<RefRegistry>,
    mut queue: ResMut<ConsoleQueue>,
) {
    let Ok(mut input) = inputs.single_mut() else {
        return;
    };
    let current = editable_value(&input);
    if keys.just_pressed(KeyCode::Enter) {
        if current.trim().is_empty() || input.is_composing() {
            return;
        }
        ui.history.record(current.clone());
        if let Err(error) = save_history(&ui.history_path, &ui.history) {
            warn!("could not persist console history: {error}");
        }
        ui.completion.reset();
        input.clear();
        if is_clear_submission(&current) {
            ui.transcript.clear();
            ui.transcript_dirty = true;
            return;
        }
        queue.0.push_back(crate::console::ConsoleRequest {
            session: ConsoleSessionId::new(UI_SESSION),
            line: current,
        });
        return;
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        let replacement = ui.history.up(&current);
        replace_editable(&mut input, &replacement);
        ui.completion.reset();
    } else if keys.just_pressed(KeyCode::ArrowDown) {
        let replacement = ui.history.down(&current);
        replace_editable(&mut input, &replacement);
        ui.completion.reset();
    } else if keys.just_pressed(KeyCode::Tab) {
        let candidates = registry
            .completion_names()
            .into_iter()
            .chain(references.completion_names())
            .collect::<Vec<_>>();
        let result = ui.completion.complete(&current, candidates);
        replace_editable(&mut input, &result.text);
        if result.list_candidates {
            for candidate in result.matches {
                push_scrollback(&mut ui, candidate);
            }
        }
    }
}

/// Issue #131 diagnostic: while the console is open, log every raw
/// `KeyboardInput` message at debug level. This is the console's own view of
/// the same event stream `release_stuck_keys_on_focus_change` reacts to, so
/// enabling `RUST_LOG=bevyout::viewer::console_ui=debug` during a macOS
/// Cmd+Shift+5 screen recording shows whether keystrokes are even reaching
/// the input system while typing appears stuck. Gated on
/// `GameplayModal::Console` (the same `run_if` every other console system
/// uses) so normal gameplay never pays for this.
fn log_console_key_events(mut keys: MessageReader<KeyboardInput>) {
    for event in keys.read() {
        debug!(
            "console key state={:?} logical={:?} text={:?}",
            event.state, event.logical_key, event.text
        );
    }
}

fn update_console_title(
    sessions: Res<crate::console::ConsoleSessionStore>,
    references: Res<RefRegistry>,
    placements: Query<&PlacementRoot>,
    mut title: Query<&mut Text, With<ConsoleTitle>>,
) {
    let Ok(mut title) = title.single_mut() else {
        return;
    };
    let session = ConsoleSessionId::new(UI_SESSION);
    let selected = sessions.selected(&session);
    let identity = selected.and_then(|entity| references.identity(entity));
    let display_name = selected
        .and_then(|entity| placements.get(entity).ok())
        .and_then(|root| root.placement().display_name.as_deref());
    title.0 = selected_reference_text(
        identity.and_then(|identity| identity.form_id),
        identity.and_then(|identity| identity.editor_id.as_deref()),
        display_name,
    );
}

fn consume_console_responses(
    mut responses: ResMut<ConsoleResponses>,
    mut ui: ResMut<ConsoleUiState>,
) {
    while let Some(response) = responses.0.pop_front() {
        push_scrollback(&mut ui, response.request.line);
        let has_log = !response.output.log.is_empty();
        for line in response.output.log {
            push_scrollback(&mut ui, line);
        }
        if let Some(error) = response.output.error {
            push_scrollback(&mut ui, format!("[{}] {}", error.code, error.message));
        } else if !has_log && !response.output.value.is_null() {
            push_scrollback(&mut ui, compact_console_value(&response.output.value));
        }
    }
}

fn render_console_scrollback(
    mut ui: ResMut<ConsoleUiState>,
    mut scrollback: Query<&mut Text, With<ConsoleScrollback>>,
    mut viewport: Query<&mut ScrollPosition, With<ConsoleScrollbackViewport>>,
) {
    if !ui.transcript_dirty {
        return;
    }
    if let Ok(mut text) = scrollback.single_mut() {
        text.0 = ui.transcript.rendered();
    }
    if let Ok(mut position) = viewport.single_mut() {
        position.y = if ui.transcript.is_empty() {
            0.0
        } else {
            f32::MAX
        };
    }
    ui.transcript_dirty = false;
}

fn settle_console_scrollback_position(
    mut viewport: Query<(&ComputedNode, &mut ScrollPosition), With<ConsoleScrollbackViewport>>,
) {
    let Ok((computed, mut position)) = viewport.single_mut() else {
        return;
    };
    if position.y == f32::MAX {
        position.y = computed.scroll_position.y * computed.inverse_scale_factor;
    }
}

fn compact_console_value(value: &serde_json::Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| value.to_string())
}

fn push_scrollback(ui: &mut ConsoleUiState, line: String) {
    ui.transcript.push(line);
    ui.transcript_dirty = true;
}

#[cfg(test)]
#[path = "tests/console_ui.rs"]
mod tests;
