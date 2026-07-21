//! Monofonto right-stack pause UI with CRT overlays over a freeze-frame.

use bevy::prelude::*;
use bevy::ui::{
    BackgroundGradient, ColorStop, RadialGradient, RadialGradientShape, UiPosition,
    widget::NodeImageMode,
};
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevyout_core::pause_menu::{PauseMenuAction, PauseMenuOption, PauseMenuState};

use crate::app_state::{GameplayModal, RequestStateTransition};

use super::snapshot::{self, PauseSnapshot};

const ACTIVE: Color = Color::srgb(0.55, 1.0, 0.45);
const ACTIVE_DIM: Color = Color::srgba(0.35, 0.75, 0.32, 0.95);
const DISABLED: Color = Color::srgba(0.28, 0.52, 0.24, 0.55);
const SELECTED_GLOW: Color = Color::srgba(0.55, 1.0, 0.45, 0.55);
const GRID: Color = Color::srgba(0.35, 0.7, 0.3, 0.18);
const TICK: Color = Color::srgba(0.45, 0.85, 0.4, 0.55);
const TINT: Color = Color::srgba(0.20, 0.20, 0.05, 0.40);
const VIGNETTE: Color = Color::srgba(0.0, 0.0, 0.0, 0.72);
/// Opaque fill under the freeze-frame so the live 3D view can never bleed
/// through letterboxing or a late/missing snapshot.
const BACKDROP_FILL: Color = Color::srgb(0.06, 0.05, 0.02);
const FONT_SIZE: f32 = 28.0;
const ROW_HEIGHT: f32 = 40.0;

#[derive(Resource, Clone)]
pub(super) struct PauseMenuFont(pub(super) Handle<Font>);

#[derive(Resource)]
pub(super) struct PauseMenuScanlines(pub(super) Handle<Image>);

#[derive(Resource, Default)]
pub(super) struct PauseMenuUiState {
    pub(super) selection: PauseMenuState,
}

#[derive(Component)]
pub(super) struct PauseMenuRoot;

#[derive(Component)]
pub(super) struct PauseMenuBackdrop;

#[derive(Component)]
pub(super) struct PauseMenuLabel(PauseMenuOption);

#[derive(Component)]
pub(super) struct PauseMenuRow(PauseMenuOption);

pub(super) fn load_pause_menu_assets(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    mut images: ResMut<Assets<Image>>,
) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join("pipboy-monofonto.ttf");
    match std::fs::read(&path) {
        Ok(bytes) => {
            let handle = fonts.add(Font::from_bytes(bytes));
            commands.insert_resource(PauseMenuFont(handle));
        }
        Err(error) => {
            warn!(
                "pause menu: failed to load monofonto from {}: {error}",
                path.display()
            );
            commands.insert_resource(PauseMenuFont(Handle::default()));
        }
    }
    let scanlines = images.add(snapshot::scanline_texture());
    commands.insert_resource(PauseMenuScanlines(scanlines));
}

pub(super) fn open_pause_menu(
    mut commands: Commands,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut ui_state: ResMut<PauseMenuUiState>,
    font: Res<PauseMenuFont>,
    scanlines: Res<PauseMenuScanlines>,
    snapshot: Res<PauseSnapshot>,
) {
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
    ui_state.selection = PauseMenuState::new();
    // Spawn hidden so the same-frame screenshot does not bake the menu into
    // the freeze-frame; `reveal_pause_menu` unhides once the capture lands
    // (or after the capture is abandoned).
    spawn_screen(
        &mut commands,
        &font.0,
        &scanlines.0,
        &ui_state.selection,
        snapshot.handle.clone(),
        Visibility::Hidden,
    );
}

pub(super) fn close_pause_menu(
    mut commands: Commands,
    roots: Query<Entity, With<PauseMenuRoot>>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn();
    }
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }
}

fn spawn_screen(
    commands: &mut Commands,
    font: &Handle<Font>,
    scanlines: &Handle<Image>,
    selection: &PauseMenuState,
    snapshot: Option<Handle<Image>>,
    visibility: Visibility,
) {
    commands
        .spawn((
            PauseMenuRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            // Opaque base so nothing from the world camera can show through.
            BackgroundColor(BACKDROP_FILL),
            GlobalZIndex(1500),
            visibility,
        ))
        .with_children(|root| {
            // Layer 0: freeze-frame stretched to the full viewport. Default
            // ImageNode mode is Auto (intrinsic texels) which left a small
            // "backsquare" of blur over the still-live 3D scene.
            let has_snapshot = snapshot.is_some();
            root.spawn((
                PauseMenuBackdrop,
                ImageNode {
                    image: snapshot.unwrap_or_default(),
                    color: if has_snapshot {
                        Color::WHITE
                    } else {
                        // Transparent 1x1 default handle: stay invisible until
                        // the capture lands; the opaque root fills the gap.
                        Color::NONE
                    },
                    image_mode: NodeImageMode::Stretch,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
            ));

            // Layer 1: amber/green CRT tint.
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(TINT),
            ));

            // Layer 2: tiled scanlines.
            root.spawn((
                ImageNode {
                    image: scanlines.clone(),
                    image_mode: NodeImageMode::Tiled {
                        tile_x: true,
                        tile_y: true,
                        stretch_value: 1.0,
                    },
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
            ));

            // Layer 3: edge vignette.
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
                    UiPosition::CENTER,
                    RadialGradientShape::FarthestCorner,
                    vec![
                        ColorStop::new(Color::NONE, Val::Percent(45.0)),
                        ColorStop::new(VIGNETTE, Val::Percent(100.0)),
                    ],
                )),
            ));

            spawn_grid(root);
            spawn_menu_column(root, font, selection);
        });
}

fn spawn_grid(root: &mut ChildSpawnerCommands) {
    root.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(3.0),
            right: Val::Percent(3.0),
            top: Val::Percent(4.0),
            bottom: Val::Percent(4.0),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BorderColor::all(GRID),
    ));

    for pct in [33.3_f32, 66.6_f32] {
        root.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(pct),
                top: Val::Percent(4.0),
                bottom: Val::Percent(4.0),
                width: Val::Px(1.0),
                ..default()
            },
            BackgroundColor(GRID),
        ));
        root.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(pct),
                left: Val::Percent(3.0),
                right: Val::Percent(3.0),
                height: Val::Px(1.0),
                ..default()
            },
            BackgroundColor(GRID),
        ));
    }

    for (top, bottom, glyph) in [
        (Val::Percent(3.0), Val::Auto, "▲"),
        (Val::Auto, Val::Percent(3.0), "▼"),
    ] {
        for left_side in [true, false] {
            let (left, right) = if left_side {
                (Val::Percent(3.5), Val::Auto)
            } else {
                (Val::Auto, Val::Percent(3.5))
            };
            root.spawn((
                Text::new(glyph),
                TextColor(TICK),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    left,
                    right,
                    top,
                    bottom,
                    ..default()
                },
            ));
        }
    }
}

fn spawn_menu_column(
    root: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    selection: &PauseMenuState,
) {
    root.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Percent(6.0),
        top: Val::Percent(0.0),
        bottom: Val::Percent(0.0),
        width: Val::Px(220.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::FlexEnd,
        row_gap: Val::Px(6.0),
        ..default()
    })
    .with_children(|column| {
        for option in PauseMenuOption::ALL {
            let selected = selection.selected() == option;
            spawn_row(column, font, option, selected);
        }
    });
}

fn spawn_row(
    column: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    option: PauseMenuOption,
    selected: bool,
) {
    let (color, shadow) = row_style(option, selected);
    column
        .spawn((
            Button,
            PauseMenuRow(option),
            Node {
                height: Val::Px(ROW_HEIGHT),
                min_width: Val::Px(160.0),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|row| {
            row.spawn((
                PauseMenuLabel(option),
                Text::new(option.label()),
                TextColor(color),
                TextFont {
                    font: FontSource::Handle(font.clone()),
                    font_size: FontSize::Px(FONT_SIZE),
                    ..default()
                },
                TextLayout::justify(Justify::Right),
                shadow,
            ));
        });
}

fn row_style(option: PauseMenuOption, selected: bool) -> (Color, TextShadow) {
    if !option.is_enabled() {
        return (
            DISABLED,
            TextShadow {
                offset: Vec2::ZERO,
                color: Color::NONE,
            },
        );
    }
    if selected {
        (
            ACTIVE,
            TextShadow {
                offset: Vec2::new(1.0, 1.0),
                color: SELECTED_GLOW,
            },
        )
    } else {
        (
            ACTIVE_DIM,
            TextShadow {
                offset: Vec2::new(1.0, 1.0),
                color: Color::srgba(0.35, 0.75, 0.32, 0.25),
            },
        )
    }
}

pub(super) fn handle_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<PauseMenuUiState>,
    mut labels: Query<(&PauseMenuLabel, &mut TextColor, &mut TextShadow)>,
    mut requests: MessageWriter<RequestStateTransition>,
    mut exit: MessageWriter<AppExit>,
) {
    let mut dirty = false;
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        ui_state.selection.move_up();
        dirty = true;
    }
    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        ui_state.selection.move_down();
        dirty = true;
    }
    if dirty {
        refresh_labels(&ui_state.selection, &mut labels);
    }

    let confirm = keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space);
    if confirm {
        apply_action(ui_state.selection.activate(), &mut requests, &mut exit);
    }
}

pub(super) fn handle_pointer(
    mut ui_state: ResMut<PauseMenuUiState>,
    rows: Query<(&Interaction, &PauseMenuRow), Changed<Interaction>>,
    mut labels: Query<(&PauseMenuLabel, &mut TextColor, &mut TextShadow)>,
    mut requests: MessageWriter<RequestStateTransition>,
    mut exit: MessageWriter<AppExit>,
) {
    let mut dirty = false;
    let mut clicked = None;
    for (interaction, row) in &rows {
        match *interaction {
            Interaction::Hovered if row.0.is_enabled() => {
                if ui_state.selection.selected() != row.0 {
                    ui_state.selection.select(row.0);
                    dirty = true;
                }
            }
            Interaction::Pressed if row.0.is_enabled() => {
                ui_state.selection.select(row.0);
                dirty = true;
                clicked = Some(row.0);
            }
            _ => {}
        }
    }
    if dirty {
        refresh_labels(&ui_state.selection, &mut labels);
    }
    if let Some(option) = clicked {
        apply_action(
            PauseMenuAction::from_option(option),
            &mut requests,
            &mut exit,
        );
    }
}

fn refresh_labels(
    selection: &PauseMenuState,
    labels: &mut Query<(&PauseMenuLabel, &mut TextColor, &mut TextShadow)>,
) {
    for (label, mut color, mut shadow) in labels.iter_mut() {
        let selected = selection.selected() == label.0;
        let (next_color, next_shadow) = row_style(label.0, selected);
        *color = TextColor(next_color);
        *shadow = next_shadow;
    }
}

fn apply_action(
    action: Option<PauseMenuAction>,
    requests: &mut MessageWriter<RequestStateTransition>,
    exit: &mut MessageWriter<AppExit>,
) {
    match action {
        Some(PauseMenuAction::Continue) => {
            requests.write(RequestStateTransition::Modal(GameplayModal::None));
        }
        Some(PauseMenuAction::Quit) => {
            info!("pause menu: quit");
            exit.write(AppExit::Success);
        }
        None => {}
    }
}
