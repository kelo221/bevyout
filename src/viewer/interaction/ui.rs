//! Interaction prompt and notice UI construction.

use super::*;
use crate::viewer::fallout_ui::{PHOSPHOR, glow};

pub(super) fn clear_interaction_prompt(mut prompt: Query<&mut Text, With<InteractionPromptText>>) {
    if let Ok(mut prompt) = prompt.single_mut() {
        prompt.0.clear();
    }
}

pub(super) fn spawn_interaction_ui(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        InteractionPromptText,
        super::super::console::GameUi,
        TextColor(PHOSPHOR),
        TextFont {
            font_size: FontSize::Px(22.0),
            ..default()
        },
        glow(),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            bottom: Val::Percent(10.0),
            margin: UiRect::left(Val::Px(-180.0)),
            width: Val::Px(360.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        TextLayout::justify(Justify::Center),
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
