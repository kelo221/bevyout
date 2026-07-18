//! Interaction prompt and notice UI construction.

use super::*;

pub(super) fn spawn_interaction_ui(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        InteractionPromptText,
        super::super::console::GameUi,
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(58.0),
            margin: UiRect::left(Val::Px(-140.0)),
            width: Val::Px(280.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        ZIndex(110),
    ));
    commands.spawn((
        Text::new(""),
        InteractionNoticeText,
        super::super::console::GameUi,
        TextColor(Color::srgb(1.0, 0.9, 0.5)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(63.0),
            margin: UiRect::left(Val::Px(-240.0)),
            width: Val::Px(480.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        ZIndex(110),
    ));
}
