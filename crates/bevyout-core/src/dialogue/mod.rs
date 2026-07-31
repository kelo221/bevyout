//! Bevy-free dialogue contracts shared by preparation, runtime adapters, and
//! save/load.  Yarn types intentionally stop at the viewer boundary.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::form_id::FormId;

pub const DIALOGUE_SNAPSHOT_SCHEMA_VERSION: u32 = 1;
pub const DIALOGUE_BUNDLE_REVISION: &str = "dialogue-bundle-v3";
pub const DIALOGUE_VOICE_INDEX_REVISION: &str = "dialogue-voice-v2";

macro_rules! string_id {
    ($name:ident) => {
        #[derive(
            Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
        )]
        pub struct $name(pub String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

string_id!(DialogueKey);
string_id!(DialogueChoiceId);
string_id!(DialogueSessionId);
string_id!(DialogueLineKey);
string_id!(DialogueActionKey);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DialogueStartSource {
    AuthoredNpc,
    ImportedFallout,
    Agent,
    Script,
    CheckpointResume,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueStartRequest {
    pub dialogue: DialogueKey,
    pub speaker: Option<FormId>,
    pub listener: Option<FormId>,
    pub source: DialogueStartSource,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueSpeaker {
    pub stable_id: Option<FormId>,
    pub display_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DialogueLinePresentation {
    pub line_key: DialogueLineKey,
    pub text: String,
    pub speaker: DialogueSpeaker,
    pub voice_key: Option<String>,
    pub localization_key: Option<String>,
    pub reveal_seconds: f32,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueOptionPresentation {
    pub choice: DialogueChoiceId,
    pub text: String,
    pub line_key: Option<DialogueLineKey>,
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DialoguePresentation {
    pub line: Option<DialogueLinePresentation>,
    pub options: Vec<DialogueOptionPresentation>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum DialoguePhase {
    #[default]
    Loading,
    Ready,
    Running,
    PresentingLine,
    PresentingOptions,
    WaitingCommand,
    Closing,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DialogueErrorCode {
    NotReady,
    Busy,
    MissingDialogue,
    MissingNode,
    MalformedContent,
    UnsupportedHostApi,
    HostCommandFailed,
    SaveBlocked,
    BundleMismatch,
    Interrupted,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueError {
    pub code: DialogueErrorCode,
    pub message: String,
    pub dialogue: Option<DialogueKey>,
    pub source: Option<String>,
}

impl DialogueError {
    pub fn new(code: DialogueErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            dialogue: None,
            source: None,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum NarrativeValue {
    #[default]
    Null,
    Bool(bool),
    Number(i64),
    Text(String),
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct NarrativeVariableState {
    pub persistent: BTreeMap<String, NarrativeValue>,
    #[serde(skip)]
    pub session: BTreeMap<String, NarrativeValue>,
    #[serde(skip)]
    pub temporary: BTreeMap<String, NarrativeValue>,
}

impl NarrativeVariableState {
    pub fn get(&self, name: &str) -> Option<&NarrativeValue> {
        self.temporary
            .get(name)
            .or_else(|| self.session.get(name))
            .or_else(|| self.persistent.get(name))
    }

    pub fn set(&mut self, name: impl Into<String>, value: NarrativeValue) {
        let name = name.into();
        let destination = if name.starts_with("$global_") {
            &mut self.persistent
        } else if name.starts_with("$session_") {
            &mut self.session
        } else {
            &mut self.temporary
        };
        destination.insert(name, value);
    }

    pub fn clear_session_boundary(&mut self) {
        self.session.clear();
        self.temporary.clear();
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActiveDialogueCheckpoint {
    pub dialogue: DialogueKey,
    pub node: String,
    pub speaker: Option<FormId>,
    pub listener: Option<FormId>,
    pub completed_actions: Vec<DialogueActionKey>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueSnapshot {
    pub schema_version: u32,
    pub bundle_hash: String,
    pub variables: NarrativeVariableState,
    pub active: Option<ActiveDialogueCheckpoint>,
}

impl Default for DialogueSnapshot {
    fn default() -> Self {
        Self::boundary(String::new(), NarrativeVariableState::default())
    }
}

impl DialogueSnapshot {
    pub fn boundary(bundle_hash: impl Into<String>, variables: NarrativeVariableState) -> Self {
        Self {
            schema_version: DIALOGUE_SNAPSHOT_SCHEMA_VERSION,
            bundle_hash: bundle_hash.into(),
            variables,
            active: None,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueNodeIndexEntry {
    pub node: String,
    pub source_path: String,
    pub source_line: u32,
    pub source_key: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PreparedDialogueBundleRef {
    pub revision: String,
    #[serde(default = "default_dialogue_catalog_path")]
    pub catalog_path: String,
    pub source_paths: Vec<String>,
    pub node_index_path: String,
    pub voice_index_path: Option<String>,
    pub localization_index_path: Option<String>,
    pub content_fingerprint: String,
}

fn default_dialogue_catalog_path() -> String {
    "dialogue/catalog.ron".into()
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueVoiceAsset {
    pub line_key: DialogueLineKey,
    pub asset_path: String,
    pub duration_millis: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueVoiceDiagnostic {
    pub severity: String,
    pub code: String,
    pub line_key: Option<DialogueLineKey>,
    pub source_path: Option<String>,
    pub message: String,
}

/// Prepared, content-addressed voice assets keyed by stable dialogue line.
///
/// The source manifest and its fingerprint are retained so preparation can
/// invalidate stale voice data without making the viewer scan source folders.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PreparedDialogueVoiceIndex {
    pub revision: String,
    pub source_manifest_path: String,
    pub source_fingerprint: String,
    pub entries: Vec<DialogueVoiceAsset>,
    pub diagnostics: Vec<DialogueVoiceDiagnostic>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueLocalizationEntry {
    pub line_key: DialogueLineKey,
    pub language: String,
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialoguePresentationPolicy {
    pub language: String,
    pub subtitles_enabled: bool,
    pub typewriter_enabled: bool,
    pub skip_requires_second_press: bool,
    pub accessible_choice_numbers: bool,
}

impl DialoguePresentationPolicy {
    pub fn reveal_duration_seconds(&self, text: &str, voice_millis: Option<u32>) -> f32 {
        if let Some(duration) = voice_millis {
            return duration as f32 / 1000.0;
        }
        if !self.typewriter_enabled {
            return 0.0;
        }
        (text.chars().count() as f32 * 0.025).max(0.05)
    }

    pub fn auto_advance_duration_seconds(&self, text: &str, voice_millis: Option<u32>) -> f32 {
        if let Some(duration) = voice_millis {
            return duration as f32 / 1000.0;
        }
        (text.chars().count() as f32 * 0.025).max(0.5)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueCoverageReport {
    pub total_lines: usize,
    pub localized_lines: usize,
    pub voiced_lines: usize,
    pub missing_localization: Vec<DialogueLineKey>,
    pub missing_voice: Vec<DialogueLineKey>,
    pub unsupported_records: usize,
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
