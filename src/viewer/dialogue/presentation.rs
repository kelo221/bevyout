use std::collections::BTreeMap;

use bevy::prelude::*;
use bevyout_core::dialogue::{
    DialogueCoverageReport, DialogueLineKey, DialoguePresentationPolicy, DialogueVoiceAsset,
};

use super::{DialogueRuntime, DialogueTelemetry, DialogueUiPhase};

#[derive(Component)]
pub(crate) struct DialogueUiRoot;

#[derive(Component)]
pub(crate) struct DialogueUiText;

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
                skip_requires_second_press: true,
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

pub(crate) fn update_dialogue_ui(
    mut commands: Commands,
    mut runtime: ResMut<DialogueRuntime>,
    roots: Query<Entity, With<DialogueUiRoot>>,
    mut texts: Query<&mut Text, With<DialogueUiText>>,
    providers: Res<DialoguePresentationProviders>,
    mut telemetry: ResMut<DialogueTelemetry>,
) {
    let text = if let Some(line) = runtime.presentation.line.clone() {
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
        let line_text = providers.localized_text(&line.line_key, &line.text);
        if line.speaker.display_name.is_empty() {
            line_text
        } else {
            format!("{}: {}", line.speaker.display_name, line_text)
        }
    } else if !runtime.presentation.options.is_empty() {
        runtime
            .presentation
            .options
            .iter()
            .enumerate()
            .map(|(index, option)| format!("{}. {}", index + 1, option.text))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        String::new()
    };
    if let Some(catalog) = runtime.catalog.as_ref() {
        let coverage = providers.coverage(catalog.line_keys.iter().cloned());
        telemetry.coverage_checks += 1;
        if !coverage.missing_localization.is_empty() && providers.diagnostics.is_empty() {
            // Keep the provider boundary observable without turning a missing
            // presentation asset into a world-state failure.
            runtime.trace.push(format!(
                "presentation missing_localization={}",
                coverage.missing_localization.len()
            ));
        }
    }
    for mut ui_text in &mut texts {
        ui_text.0 = text.clone();
    }
    let visible = !matches!(
        runtime.ui_phase,
        DialogueUiPhase::Hidden | DialogueUiPhase::Closing
    );
    if roots.is_empty() && visible {
        commands
            .spawn((
                DialogueUiRoot,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(10.0),
                    right: Val::Percent(10.0),
                    bottom: Val::Percent(8.0),
                    padding: UiRect::all(Val::Px(18.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.88)),
            ))
            .with_children(|parent| {
                parent.spawn((DialogueUiText, Text::new(text)));
            });
    }
}
