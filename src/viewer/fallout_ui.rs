//! Shared phosphor-screen presentation used by the Pip-Boy and transfer UI.

use bevy::prelude::*;
use bevy::ui::widget::TextShadow;

pub(crate) const PHOSPHOR: Color = Color::srgb(0.18, 1.0, 0.48);
pub(crate) const PHOSPHOR_DIM: Color = Color::srgba(0.08, 0.45, 0.22, 0.85);
pub(crate) const PHOSPHOR_FAINT: Color = Color::srgba(0.1, 0.55, 0.26, 0.9);
pub(crate) const SCREEN: Color = Color::srgba(0.004, 0.022, 0.011, 0.96);
pub(crate) const SCREEN_TRANSLUCENT: Color = Color::srgba(0.004, 0.022, 0.011, 0.88);
pub(crate) const SCREEN_GLOW: Color = Color::srgba(0.1, 0.55, 0.26, 0.14);
pub(crate) const TEXT_GLOW: Color = Color::srgba(0.18, 1.0, 0.48, 0.5);
pub(crate) const BEZEL: Color = Color::srgb(0.035, 0.045, 0.033);
pub(crate) const BEZEL_EDGE: Color = Color::srgb(0.11, 0.13, 0.095);
pub(crate) const BEZEL_RECESS: Color = Color::srgb(0.015, 0.019, 0.014);
pub(crate) const LAMP: Color = Color::srgb(1.0, 0.47, 0.08);
pub(crate) const LAMP_DIM: Color = Color::srgb(0.16, 0.085, 0.025);

pub(crate) fn glow() -> TextShadow {
    TextShadow {
        offset: Vec2::new(1.0, 1.0),
        color: TEXT_GLOW,
    }
}

pub(crate) fn spawn_corner_brackets(
    parent: &mut ChildSpawnerCommands,
    offset: f32,
    size: f32,
    width: f32,
) {
    for (left, right, top, bottom, border) in [
        (
            Some(offset),
            None,
            Some(offset),
            None,
            UiRect {
                left: Val::Px(width),
                top: Val::Px(width),
                ..default()
            },
        ),
        (
            None,
            Some(offset),
            Some(offset),
            None,
            UiRect {
                right: Val::Px(width),
                top: Val::Px(width),
                ..default()
            },
        ),
        (
            Some(offset),
            None,
            None,
            Some(offset),
            UiRect {
                left: Val::Px(width),
                bottom: Val::Px(width),
                ..default()
            },
        ),
        (
            None,
            Some(offset),
            None,
            Some(offset),
            UiRect {
                right: Val::Px(width),
                bottom: Val::Px(width),
                ..default()
            },
        ),
    ] {
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: left.map(Val::Px).unwrap_or_default(),
                right: right.map(Val::Px).unwrap_or_default(),
                top: top.map(Val::Px).unwrap_or_default(),
                bottom: bottom.map(Val::Px).unwrap_or_default(),
                width: Val::Px(size),
                height: Val::Px(size),
                border,
                ..default()
            },
            BorderColor::all(PHOSPHOR),
        ));
    }
}

pub(crate) fn spawn_selection_marker(parent: &mut ChildSpawnerCommands, selected: bool) {
    parent.spawn((
        Node {
            width: Val::Px(9.0),
            height: Val::Px(9.0),
            margin: UiRect::right(Val::Px(10.0)),
            flex_shrink: 0.0,
            ..default()
        },
        BackgroundColor(if selected { PHOSPHOR } else { Color::NONE }),
    ));
}
