//! In-game grave-key console frontend.

use std::collections::VecDeque;
use std::path::PathBuf;

use bevy::input::mouse::MouseWheel;
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy::prelude::*;
use bevy::text::{EditableText, EditableTextFilter, TextCursorStyle, TextEdit};
use bevy::transform::TransformSystems;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::app_state::GameplayModal;
use crate::console::openmw_ui::{CommandHistory, CompletionState, load_history, save_history};
use crate::console::{
    ConsoleQueue, ConsoleRegistry, ConsoleResponses, ConsoleSessionId, ConsoleSessionStore,
    ConsoleSystemSet, RefRegistry,
};

use super::interaction::{PlacementRoot, find_placement_root};

const UI_SESSION: &str = "ui";
const SCROLLBACK_LIMIT: usize = 200;
const INPUT_STRIP_HEIGHT: f32 = 42.0;

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
    pick_cycle: ConsolePickCycle,
}

#[derive(Default)]
struct ConsolePickCycle {
    /// Unique placement roots ordered from the camera toward the back of the
    /// scene along the last pixel clicked, matching the Gamebryo console.
    candidates: Vec<Entity>,
}

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
            .run_if(in_state(GameplayModal::Console)),
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
                ConsoleScrollback,
                Text::new(""),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    left: px(40),
                    bottom: px(INPUT_STRIP_HEIGHT),
                    width: percent(80),
                    max_height: percent(50),
                    overflow: Overflow::clip(),
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
                EditableTextFilter::new(|character| !matches!(character, '`' | '~')),
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
        .filter(|character| !matches!(character, '`' | '~'))
        .collect()
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
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut raycast: MeshRayCast,
    parents: Query<&ChildOf>,
    roots: Query<&PlacementRoot>,
    references: Res<RefRegistry>,
    mut sessions: ResMut<ConsoleSessionStore>,
    mut ui: ResMut<ConsoleUiState>,
) {
    let wheel_delta = wheel.read().map(|event| event.y).sum::<f32>();
    if !buttons.just_pressed(MouseButton::Left) {
        if wheel_delta == 0.0 {
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
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
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
    mut scrollback: Query<&mut Text, With<ConsoleScrollback>>,
) {
    let mut changed = false;
    while let Some(response) = responses.0.pop_front() {
        changed = true;
        push_scrollback(&mut ui.scrollback, response.request.line);
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
            push_scrollback(
                &mut ui.scrollback,
                compact_console_value(&response.output.value),
            );
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

fn compact_console_value(value: &serde_json::Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| value.to_string())
}

fn push_scrollback(scrollback: &mut VecDeque<String>, line: String) {
    scrollback.push_back(line);
    while scrollback.len() > SCROLLBACK_LIMIT {
        scrollback.pop_front();
    }
}

#[cfg(test)]
mod tests {
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

        let mut input = world
            .query_filtered::<(&Node, &EditableText, &EditableTextFilter), With<ConsoleInput>>();
        let (node, editable, _) = input.single(world).unwrap();
        assert_eq!(node.left, px(40));
        assert_eq!(node.bottom, px(10));
        assert!(editable_value(editable).is_empty());

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
}
