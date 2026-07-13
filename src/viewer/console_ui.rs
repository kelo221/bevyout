//! In-game grave-key console frontend.

use std::collections::VecDeque;
use std::path::PathBuf;

use bevy::input_focus::{FocusCause, InputFocus};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle, TextEdit};
use bevy::transform::TransformSystems;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::app_state::GameplayModal;
use crate::console::openmw_ui::{CommandHistory, CompletionState, load_history, save_history};
use crate::console::{
    ConsoleQueue, ConsoleRegistry, ConsoleResponses, ConsoleSessionId, ConsoleSystemSet,
    RefRegistry,
};

const UI_SESSION: &str = "ui";
const SCROLLBACK_LIMIT: usize = 200;

#[derive(Component)]
struct ConsoleRoot;

#[derive(Component)]
struct ConsoleTitle;

#[derive(Component)]
struct ConsoleScrollback;

#[derive(Component)]
struct ConsoleInput;

#[derive(Resource)]
struct ConsoleUiState {
    history: CommandHistory,
    completion: CompletionState,
    scrollback: VecDeque<String>,
    history_path: PathBuf,
}

#[derive(Clone, Copy)]
struct CursorSnapshot {
    visible: bool,
    grab_mode: CursorGrabMode,
}

#[derive(Resource, Default)]
struct ConsoleCursorRestore(Option<CursorSnapshot>);

pub(crate) fn install(app: &mut App) {
    let history_path = PathBuf::from(".bevyout/console_history.txt");
    let history = load_history(&history_path).unwrap_or_else(|error| {
        warn!("could not load console history: {error}");
        CommandHistory::default()
    });
    app.insert_resource(ConsoleUiState {
        history,
        completion: CompletionState::default(),
        scrollback: VecDeque::new(),
        history_path,
    })
    .init_resource::<ConsoleCursorRestore>()
    .add_systems(Startup, spawn_console_ui)
    .add_systems(OnEnter(GameplayModal::Console), open_console_ui)
    .add_systems(OnExit(GameplayModal::Console), close_console_ui)
    .add_systems(
        Update,
        (handle_console_input, update_console_title).run_if(in_state(GameplayModal::Console)),
    )
    .add_systems(
        PostUpdate,
        consume_console_responses
            .after(ConsoleSystemSet::Execute)
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
                height: percent(58),
                padding: UiRect::all(px(10)),
                flex_direction: FlexDirection::Column,
                row_gap: px(6),
                ..default()
            },
            BackgroundColor(Color::srgba(0.015, 0.02, 0.025, 0.94)),
            ZIndex(1000),
            Visibility::Hidden,
        ))
        .with_children(|root| {
            root.spawn((
                ConsoleTitle,
                Text::new("Console — no reference selected"),
                TextColor(Color::srgb(0.95, 0.82, 0.45)),
                TextFont {
                    font_size: FontSize::Px(17.0),
                    ..default()
                },
            ));
            root.spawn((
                ConsoleScrollback,
                Text::new("Bevyout console ready. Type 'help' for commands."),
                TextColor(Color::srgb(0.86, 0.9, 0.92)),
                TextFont {
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                Node {
                    width: percent(100),
                    min_height: px(0),
                    flex_basis: px(0),
                    flex_grow: 1.0,
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
            root.spawn((
                Node {
                    width: percent(100),
                    min_height: px(32),
                    padding: UiRect::axes(px(8), px(4)),
                    border: UiRect::all(px(1)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    flex_shrink: 0.0,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.08, 0.09, 0.1, 0.98)),
                BorderColor::all(Color::srgb(0.35, 0.42, 0.46)),
            ))
            .with_children(|row| {
                row.spawn((
                    Text::new("> "),
                    TextColor(Color::srgb(0.95, 0.82, 0.45)),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                ));
                row.spawn((
                    ConsoleInput,
                    EditableText {
                        max_characters: Some(2048),
                        visible_width: Some(100.0),
                        allow_newlines: false,
                        ..default()
                    },
                    TextCursorStyle::default(),
                    TextLayout::no_wrap(),
                    TextColor(Color::WHITE),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    Node {
                        flex_grow: 1.0,
                        overflow: Overflow::clip_x(),
                        ..default()
                    },
                ));
            });
        });
}

fn open_console_ui(
    mut roots: Query<&mut Visibility, With<ConsoleRoot>>,
    inputs: Query<Entity, With<ConsoleInput>>,
    mut focus: ResMut<InputFocus>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut restore: ResMut<ConsoleCursorRestore>,
) {
    for mut visibility in &mut roots {
        *visibility = Visibility::Inherited;
    }
    if let Ok(input) = inputs.single() {
        focus.set(input, FocusCause::Navigated);
    }
    if let Ok(mut cursor) = cursor.single_mut() {
        restore.0 = Some(CursorSnapshot {
            visible: cursor.visible,
            grab_mode: cursor.grab_mode,
        });
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
}

fn close_console_ui(
    mut roots: Query<&mut Visibility, With<ConsoleRoot>>,
    mut focus: ResMut<InputFocus>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut restore: ResMut<ConsoleCursorRestore>,
) {
    for mut visibility in &mut roots {
        *visibility = Visibility::Hidden;
    }
    focus.clear();
    if let (Some(snapshot), Ok(mut cursor)) = (restore.0.take(), cursor.single_mut()) {
        cursor.visible = snapshot.visible;
        cursor.grab_mode = snapshot.grab_mode;
    }
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
                push_scrollback(&mut ui.scrollback, candidate);
            }
        }
    }
}

fn update_console_title(
    sessions: Res<crate::console::ConsoleSessionStore>,
    references: Res<RefRegistry>,
    mut title: Query<&mut Text, With<ConsoleTitle>>,
) {
    let Ok(mut title) = title.single_mut() else {
        return;
    };
    let session = ConsoleSessionId::new(UI_SESSION);
    title.0 = sessions.selected(&session).map_or_else(
        || "Console — no reference selected".to_string(),
        |entity| format!("Console — {}", references.label(entity)),
    );
}

fn consume_console_responses(
    mut responses: ResMut<ConsoleResponses>,
    mut ui: ResMut<ConsoleUiState>,
    mut scrollback: Query<&mut Text, With<ConsoleScrollback>>,
) {
    let mut changed = false;
    while let Some(response) = responses.0.pop_front() {
        changed = true;
        push_scrollback(&mut ui.scrollback, format!("> {}", response.request.line));
        let has_log = !response.output.log.is_empty();
        for line in response.output.log {
            push_scrollback(&mut ui.scrollback, line);
        }
        if let Some(error) = response.output.error {
            push_scrollback(
                &mut ui.scrollback,
                format!("[{}] {}", error.code, error.message),
            );
        } else if !has_log && !response.output.value.is_null() {
            let value = serde_json::to_string_pretty(&response.output.value)
                .unwrap_or_else(|_| response.output.value.to_string());
            for line in value.lines() {
                push_scrollback(&mut ui.scrollback, line.to_string());
            }
        }
    }
    if changed && let Ok(mut text) = scrollback.single_mut() {
        text.0 = ui
            .scrollback
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join("\n");
    }
}

fn push_scrollback(scrollback: &mut VecDeque<String>, line: String) {
    scrollback.push_back(line);
    while scrollback.len() > SCROLLBACK_LIMIT {
        scrollback.pop_front();
    }
}
