use std::collections::BTreeMap;

use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;
use bevyout_core::dialogue::{
    DialogueChoiceId, DialogueCoverageReport, DialogueLineKey, DialoguePresentationPolicy,
    DialogueVoiceAsset,
};

use super::{
    DialogueChoiceSelected, DialogueContinueRequested, DialogueRuntime, DialogueTelemetry,
    DialogueUiPhase,
};
use crate::viewer::fallout_ui::{PHOSPHOR, PHOSPHOR_DIM, PHOSPHOR_FAINT, SCREEN_TRANSLUCENT, glow};

const PANEL_LEFT_RIGHT_PERCENT: f32 = 19.0;
const PANEL_BOTTOM_PERCENT: f32 = 7.0;
const PANEL_MAX_WIDTH: f32 = 1_120.0;
const PANEL_PADDING: f32 = 18.0;
const SPEAKER_FONT_SIZE: f32 = 22.0;
const LINE_FONT_SIZE: f32 = 24.0;
const OPTION_FONT_SIZE: f32 = 21.0;
const OPTION_ROW_MIN_HEIGHT: f32 = 46.0;
const BORDER_WIDTH: f32 = 1.0;

#[derive(Component)]
pub(crate) struct DialogueUiRoot;

#[derive(Component)]
pub(crate) struct DialogueUiPanel;

#[derive(Component)]
pub(crate) struct DialogueUiContent;

#[derive(Component)]
struct DialogueUiSpeakerText;

#[derive(Component)]
pub(crate) struct DialogueUiLineButton;

#[derive(Component)]
struct DialogueUiLineText;

#[derive(Component, Clone)]
pub(crate) struct DialogueUiOptionButton(pub DialogueChoiceId);

#[derive(Component)]
struct DialogueUiOptionText;

#[derive(Component)]
pub(crate) struct DialogueVoicePlayer;

#[derive(Debug, Default, Resource)]
pub(crate) struct DialogueUiState {
    signature: String,
}

#[derive(Debug, Clone)]
struct DialogueUiModel {
    speaker: String,
    line: Option<String>,
    options: Vec<(DialogueChoiceId, String, bool)>,
}

#[derive(Debug, Resource)]
pub(crate) struct DialoguePresentationProviders {
    pub policy: DialoguePresentationPolicy,
    pub localization: BTreeMap<(String, String), String>,
    pub voice: BTreeMap<DialogueLineKey, DialogueVoiceAsset>,
    pub diagnostics: Vec<String>,
}

impl Default for DialoguePresentationProviders {
    fn default() -> Self {
        Self {
            policy: DialoguePresentationPolicy {
                language: "en-US".into(),
                subtitles_enabled: true,
                typewriter_enabled: true,
                skip_requires_second_press: false,
                accessible_choice_numbers: true,
            },
            localization: BTreeMap::new(),
            voice: BTreeMap::new(),
            diagnostics: Vec::new(),
        }
    }
}

impl DialoguePresentationProviders {
    pub(crate) fn localized_text(&self, key: &DialogueLineKey, fallback: &str) -> String {
        self.localization
            .get(&(key.to_string(), self.policy.language.clone()))
            .cloned()
            .unwrap_or_else(|| fallback.to_owned())
    }

    pub(crate) fn coverage(
        &self,
        keys: impl IntoIterator<Item = DialogueLineKey>,
    ) -> DialogueCoverageReport {
        let mut report = DialogueCoverageReport::default();
        for key in keys {
            report.total_lines += 1;
            if self
                .localization
                .contains_key(&(key.to_string(), self.policy.language.clone()))
            {
                report.localized_lines += 1;
            } else {
                report.missing_localization.push(key.clone());
            }
            if self.voice.contains_key(&key) {
                report.voiced_lines += 1;
            } else {
                report.missing_voice.push(key);
            }
        }
        report
    }
}

pub(crate) fn handle_dialogue_ui_input(
    runtime: Res<DialogueRuntime>,
    options: Query<(&Interaction, &DialogueUiOptionButton), Changed<Interaction>>,
    lines: Query<&Interaction, (With<DialogueUiLineButton>, Changed<Interaction>)>,
    mut choices: MessageWriter<DialogueChoiceSelected>,
    mut continues: MessageWriter<DialogueContinueRequested>,
) {
    if !runtime.input_gated {
        return;
    }
    if runtime.phase == bevyout_core::dialogue::DialoguePhase::PresentingOptions {
        for (interaction, option) in &options {
            if *interaction == Interaction::Pressed
                && runtime
                    .presentation
                    .options
                    .iter()
                    .any(|candidate| candidate.choice == option.0 && candidate.enabled)
            {
                choices.write(DialogueChoiceSelected(option.0.clone()));
            }
        }
    }
    if runtime.phase == bevyout_core::dialogue::DialoguePhase::PresentingLine
        && lines
            .iter()
            .any(|interaction| *interaction == Interaction::Pressed)
    {
        continues.write(DialogueContinueRequested);
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn update_dialogue_ui_styles(
    options: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &DialogueUiOptionButton,
        ),
        (Changed<Interaction>, Without<DialogueUiLineButton>),
    >,
    lines: Query<
        (&mut BackgroundColor, &mut BorderColor),
        (
            With<DialogueUiLineButton>,
            Without<DialogueUiOptionButton>,
            Changed<Interaction>,
        ),
    >,
) {
    for (interaction, mut background, mut border, _) in options {
        background.0 = option_colors(*interaction);
        *border = BorderColor::all(Color::NONE);
    }
    for (mut background, mut border) in lines {
        background.0 = Color::NONE;
        *border = BorderColor::all(Color::NONE);
    }
}

fn option_colors(interaction: Interaction) -> Color {
    match interaction {
        Interaction::Pressed => PHOSPHOR_DIM.with_alpha(0.45),
        Interaction::Hovered => PHOSPHOR_DIM.with_alpha(0.25),
        Interaction::None => Color::NONE,
    }
}

pub(crate) fn sync_dialogue_timing(
    time: Res<Time>,
    mut commands: Commands,
    mut runtime: ResMut<DialogueRuntime>,
    providers: Res<DialoguePresentationProviders>,
    asset_server: Option<Res<AssetServer>>,
    players: Query<Entity, With<DialogueVoicePlayer>>,
) {
    let current_line = if runtime.phase == bevyout_core::dialogue::DialoguePhase::PresentingLine {
        runtime
            .presentation
            .line
            .as_ref()
            .map(|line| line.line_key.clone())
    } else {
        None
    };

    if current_line != runtime.active_line_key {
        for entity in &players {
            commands.entity(entity).despawn();
        }
        runtime.active_line_key = current_line.clone();
        runtime.line_elapsed_seconds = 0.0;
        runtime.line_duration_seconds = 0.0;

        if let Some(line) = runtime
            .presentation
            .line
            .as_ref()
            .filter(|_| current_line.is_some())
        {
            let voice = providers.voice.get(&line.line_key);
            runtime.line_duration_seconds = providers.policy.auto_advance_duration_seconds(
                &line.text,
                voice.map(|voice| voice.duration_millis),
            );
            if let (Some(asset_server), Some(voice)) = (asset_server.as_deref(), voice)
                && !voice.asset_path.is_empty()
            {
                commands.spawn((
                    DialogueVoicePlayer,
                    AudioPlayer::new(asset_server.load(voice.asset_path.clone())),
                    PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        spatial: false,
                        volume: Volume::Decibels(0.0),
                        ..default()
                    },
                ));
            }
        }
    }

    if runtime.phase == bevyout_core::dialogue::DialoguePhase::PresentingLine {
        runtime.line_elapsed_seconds += time.delta_secs();
        if runtime.line_duration_seconds > 0.0
            && runtime.line_elapsed_seconds >= runtime.line_duration_seconds
        {
            runtime.continue_edge = true;
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn update_dialogue_ui(
    mut commands: Commands,
    mut runtime: ResMut<DialogueRuntime>,
    mut ui_state: ResMut<DialogueUiState>,
    mut roots: Query<(&mut Visibility, Option<&mut BorderColor>), With<DialogueUiRoot>>,
    panels: Query<Entity, With<DialogueUiPanel>>,
    content: Query<Entity, With<DialogueUiContent>>,
    providers: Res<DialoguePresentationProviders>,
    mut telemetry: ResMut<DialogueTelemetry>,
) {
    let model = dialogue_ui_model(&mut runtime, &providers, &mut telemetry);
    let visible = !matches!(
        runtime.ui_phase,
        DialogueUiPhase::Hidden | DialogueUiPhase::Closing
    );
    let signature = dialogue_ui_signature(&runtime, &providers, &model);

    let options_visible = runtime.phase == bevyout_core::dialogue::DialoguePhase::PresentingOptions;
    for (mut visibility, border) in &mut roots {
        *visibility = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if let Some(mut border) = border {
            *border = BorderColor::all(if options_visible {
                PHOSPHOR_DIM
            } else {
                Color::NONE
            });
        }
    }
    if !visible {
        ui_state.signature = signature;
        return;
    }

    let root = panels.iter().next();
    let Some(root) = root else {
        let root = commands.spawn(dialogue_root()).id();
        commands
            .entity(root)
            .with_children(|parent| spawn_dialogue_content(parent, &model));
        ui_state.signature = signature;
        return;
    };

    if ui_state.signature == signature {
        return;
    }
    for entity in &content {
        commands.entity(entity).despawn();
    }
    commands
        .entity(root)
        .with_children(|parent| spawn_dialogue_content(parent, &model));
    ui_state.signature = signature;
}

fn dialogue_ui_model(
    runtime: &mut DialogueRuntime,
    providers: &DialoguePresentationProviders,
    telemetry: &mut DialogueTelemetry,
) -> DialogueUiModel {
    let line_snapshot = if runtime.phase == bevyout_core::dialogue::DialoguePhase::PresentingLine {
        runtime.presentation.line.clone()
    } else {
        None
    };
    let line = line_snapshot.map(|line| {
        let voice_millis = providers
            .voice
            .get(&line.line_key)
            .map(|voice| voice.duration_millis);
        let reveal_seconds = providers
            .policy
            .reveal_duration_seconds(&line.text, voice_millis);
        if let Some(current_line) = runtime.presentation.line.as_mut() {
            current_line.reveal_seconds = reveal_seconds;
        }
        providers.localized_text(&line.line_key, &line.text)
    });
    if let Some(catalog) = runtime.catalog.as_ref() {
        let coverage = providers.coverage(catalog.line_keys.iter().cloned());
        telemetry.coverage_checks += 1;
        if !coverage.missing_localization.is_empty() && providers.diagnostics.is_empty() {
            runtime.trace.push(format!(
                "presentation missing_localization={}",
                coverage.missing_localization.len()
            ));
        }
    }
    DialogueUiModel {
        speaker: runtime.speaker.display_name.clone(),
        line,
        options: runtime
            .presentation
            .options
            .iter()
            .map(|option| (option.choice.clone(), option.text.clone(), option.enabled))
            .collect(),
    }
}

fn dialogue_ui_signature(
    runtime: &DialogueRuntime,
    providers: &DialoguePresentationProviders,
    model: &DialogueUiModel,
) -> String {
    format!(
        "{:?}|{}|{}|{}|{:?}",
        runtime.ui_phase,
        providers.policy.language,
        model.speaker,
        model.line.as_deref().unwrap_or_default(),
        model.options
    )
}

fn dialogue_root() -> (
    DialogueUiRoot,
    DialogueUiPanel,
    Visibility,
    Node,
    BackgroundColor,
    BorderColor,
    GlobalZIndex,
) {
    (
        DialogueUiRoot,
        DialogueUiPanel,
        Visibility::Inherited,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(PANEL_LEFT_RIGHT_PERCENT),
            right: Val::Percent(PANEL_LEFT_RIGHT_PERCENT),
            bottom: Val::Percent(PANEL_BOTTOM_PERCENT),
            max_width: Val::Px(PANEL_MAX_WIDTH),
            padding: UiRect::all(Val::Px(PANEL_PADDING)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            border: UiRect::all(Val::Px(BORDER_WIDTH)),
            ..default()
        },
        BackgroundColor(SCREEN_TRANSLUCENT),
        BorderColor::all(Color::NONE),
        GlobalZIndex(500),
    )
}

fn spawn_dialogue_content(parent: &mut ChildSpawnerCommands, model: &DialogueUiModel) {
    if !model.speaker.is_empty() {
        parent
            .spawn((
                DialogueUiContent,
                Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
            ))
            .with_child((
                DialogueUiSpeakerText,
                Text::new(model.speaker.clone()),
                TextColor(PHOSPHOR),
                TextFont {
                    font_size: FontSize::Px(SPEAKER_FONT_SIZE),
                    ..default()
                },
                glow(),
            ));
    }

    if !model.options.is_empty() {
        parent
            .spawn((
                DialogueUiContent,
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
            ))
            .with_children(|options| {
                for (index, (choice, text, enabled)) in model.options.iter().enumerate() {
                    let number = if index < 9 && *enabled {
                        format!("{}. ", index + 1)
                    } else {
                        String::new()
                    };
                    options
                        .spawn((
                            Button,
                            DialogueUiOptionButton(choice.clone()),
                            Node {
                                width: Val::Percent(100.0),
                                min_height: Val::Px(OPTION_ROW_MIN_HEIGHT),
                                padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                                ..default()
                            },
                            BorderColor::all(Color::NONE),
                            BackgroundColor(Color::NONE),
                        ))
                        .with_child((
                            DialogueUiOptionText,
                            Text::new(format!("{number}{text}")),
                            TextColor(if *enabled { PHOSPHOR } else { PHOSPHOR_FAINT }),
                            TextFont {
                                font_size: FontSize::Px(OPTION_FONT_SIZE),
                                ..default()
                            },
                            glow(),
                        ));
                }
            });
    }

    if let Some(line) = model.line.as_deref() {
        parent
            .spawn((
                Button,
                DialogueUiContent,
                DialogueUiLineButton,
                Node {
                    width: Val::Percent(100.0),
                    min_height: Val::Px(52.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    ..default()
                },
                BorderColor::all(Color::NONE),
                BackgroundColor(Color::NONE),
            ))
            .with_child((
                DialogueUiLineText,
                Text::new(line),
                TextColor(PHOSPHOR),
                TextFont {
                    font_size: FontSize::Px(LINE_FONT_SIZE),
                    ..default()
                },
                glow(),
            ));
    }
}
