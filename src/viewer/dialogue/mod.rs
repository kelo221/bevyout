//! Replaceable Yarn dialogue adapter for the Bevy 0.19 viewer.
//!
//! The authoritative state lives in [`DialogueRuntime`] and the core
//! contracts. NPC entities only carry [`DialogueBinding`]; the one persistent
//! runner marker is spawned once for the local player.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::PathBuf;

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevyout_core::dialogue::{
    ActiveDialogueCheckpoint, DIALOGUE_VOICE_INDEX_REVISION, DialogueChoiceId, DialogueError,
    DialogueErrorCode, DialogueKey, DialogueLineKey, DialogueLinePresentation, DialoguePhase,
    DialoguePresentation, DialogueSessionId, DialogueSpeaker, DialogueStartRequest,
    NarrativeVariableState, PreparedDialogueVoiceIndex,
};

use super::interaction::PlacementRoot;
use super::plugins::ViewerSet;
use crate::app_state::{GameplayModal, RequestStateTransition};
use crate::vsa::PreparedSemantic;
use crate::vsa::dialogue::{PreparedDialogueCatalog, PreparedDialogueNode};

#[allow(dead_code)]
mod host;
mod presentation;
#[cfg(feature = "dialogue-yarn")]
mod yarn;

pub(crate) use host::{DialogueHostState, HostCommand, YarnHostBridge};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) enum DialogueSet {
    Input,
    Lifecycle,
    Advance,
    Host,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DialogueReadiness {
    Loading,
    Ready,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DialogueBusyPolicy {
    Reject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DialogueUiPhase {
    Hidden,
    Revealing,
    Continue,
    Choice,
    Command,
    Closing,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum DialogueVoiceAnchorKind {
    Mouth,
    Head,
    ActorRoot,
    #[default]
    Unanchored,
}

impl DialogueVoiceAnchorKind {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Mouth => "Mouth",
            Self::Head => "Head",
            Self::ActorRoot => "ActorRoot",
            Self::Unanchored => "Unanchored",
        }
    }

    pub(crate) const fn is_spatial(self) -> bool {
        !matches!(self, Self::Unanchored)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum DialogueVoiceTimingState {
    #[default]
    None,
    Loading,
    Playing,
    Fallback,
    Completed,
}

impl DialogueVoiceTimingState {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Loading => "Loading",
            Self::Playing => "Playing",
            Self::Fallback => "Fallback",
            Self::Completed => "Completed",
        }
    }

    pub(crate) const fn timing_source(self) -> &'static str {
        match self {
            Self::Playing | Self::Completed => "Audio",
            Self::Fallback => "Text",
            Self::None | Self::Loading => "Pending",
        }
    }
}

#[derive(Debug, Clone, Component)]
pub(crate) struct PrimaryDialogueRunner;

#[derive(Debug, Clone, Component, PartialEq, Eq)]
pub(crate) struct DialogueBinding {
    pub dialogue: DialogueKey,
    pub speaker: u32,
    pub listener: Option<u32>,
}

#[derive(Debug, Clone)]
struct ActiveDialogue {
    request: DialogueStartRequest,
    session: DialogueSessionId,
    node: String,
    line_index: usize,
    completed_actions: BTreeSet<String>,
}

#[derive(Debug, Resource)]
pub(crate) struct DialogueRuntime {
    pub(crate) readiness: DialogueReadiness,
    pub(crate) busy_policy: DialogueBusyPolicy,
    pub(crate) catalog: Option<PreparedDialogueCatalog>,
    pub(crate) bundle_hash: String,
    pub(crate) phase: DialoguePhase,
    pub(crate) ui_phase: DialogueUiPhase,
    pub(crate) presentation: DialoguePresentation,
    pub(crate) speaker: DialogueSpeaker,
    pub(crate) variables: NarrativeVariableState,
    pub(crate) runner_entity: Option<Entity>,
    pub(crate) camera_focused: bool,
    pub(crate) input_gated: bool,
    pub(crate) continue_edge: bool,
    pub(crate) selected_choice: Option<DialogueChoiceId>,
    pub(crate) pending_starts: VecDeque<DialogueStartRequest>,
    pub(crate) pending_commands: VecDeque<HostCommand>,
    pub(crate) completed_action_keys: BTreeSet<String>,
    pub(crate) line_elapsed_seconds: f32,
    pub(crate) line_duration_seconds: f32,
    pub(crate) active_line_key: Option<DialogueLineKey>,
    pub(crate) voice_anchor: DialogueVoiceAnchorKind,
    pub(crate) voice_timing: DialogueVoiceTimingState,
    pub(crate) voice_load_elapsed_seconds: f32,
    pub(crate) diagnostics: Vec<DialogueError>,
    pub(crate) trace: Vec<String>,
    active: Option<ActiveDialogue>,
    next_session: u64,
    pending_checkpoint: Option<ActiveDialogueCheckpoint>,
    pending_checkpoint_bundle_hash: Option<String>,
}

impl Default for DialogueRuntime {
    fn default() -> Self {
        Self {
            readiness: DialogueReadiness::Loading,
            busy_policy: DialogueBusyPolicy::Reject,
            catalog: None,
            bundle_hash: String::new(),
            phase: DialoguePhase::Loading,
            ui_phase: DialogueUiPhase::Hidden,
            presentation: DialoguePresentation::default(),
            speaker: DialogueSpeaker::default(),
            variables: NarrativeVariableState::default(),
            runner_entity: None,
            camera_focused: false,
            input_gated: false,
            continue_edge: false,
            selected_choice: None,
            pending_starts: VecDeque::new(),
            pending_commands: VecDeque::new(),
            completed_action_keys: BTreeSet::new(),
            line_elapsed_seconds: 0.0,
            line_duration_seconds: 0.0,
            active_line_key: None,
            voice_anchor: DialogueVoiceAnchorKind::Unanchored,
            voice_timing: DialogueVoiceTimingState::None,
            voice_load_elapsed_seconds: 0.0,
            diagnostics: Vec::new(),
            trace: Vec::new(),
            active: None,
            next_session: 1,
            pending_checkpoint: None,
            pending_checkpoint_bundle_hash: None,
        }
    }
}

impl DialogueRuntime {
    pub(crate) fn set_catalog(&mut self, catalog: PreparedDialogueCatalog) {
        self.bundle_hash = catalog.bundle_hash();
        self.readiness = if catalog.is_ready() {
            DialogueReadiness::Ready
        } else {
            DialogueReadiness::Failed
        };
        self.phase = if self.readiness == DialogueReadiness::Ready {
            DialoguePhase::Ready
        } else {
            DialoguePhase::Failed
        };
        self.catalog = Some(catalog);
        if let Some(expected_hash) = self.pending_checkpoint_bundle_hash.take()
            && expected_hash != self.bundle_hash
        {
            self.pending_checkpoint = None;
            self.diagnostics.push(DialogueError::new(
                DialogueErrorCode::BundleMismatch,
                "active dialogue checkpoint was quarantined because the bundle changed",
            ));
        }
    }

    pub(crate) fn is_active(&self) -> bool {
        self.active.is_some()
    }

    pub(crate) fn snapshot(&self) -> bevyout_core::dialogue::DialogueSnapshot {
        let active = self.active.as_ref().map(|active| ActiveDialogueCheckpoint {
            dialogue: active.request.dialogue.clone(),
            node: active.node.clone(),
            speaker: active.request.speaker,
            listener: active.request.listener,
            completed_actions: active
                .completed_actions
                .iter()
                .cloned()
                .map(Into::into)
                .collect(),
        });
        bevyout_core::dialogue::DialogueSnapshot {
            schema_version: bevyout_core::dialogue::DIALOGUE_SNAPSHOT_SCHEMA_VERSION,
            bundle_hash: self.bundle_hash.clone(),
            variables: self.variables.clone(),
            active,
        }
    }

    pub(crate) fn boundary_snapshot(&mut self) -> bevyout_core::dialogue::DialogueSnapshot {
        self.variables.clear_session_boundary();
        let mut snapshot = self.snapshot();
        snapshot.active = None;
        snapshot
    }

    pub(crate) fn restore_snapshot(&mut self, snapshot: bevyout_core::dialogue::DialogueSnapshot) {
        self.variables = snapshot.variables;
        if let Some(checkpoint) = snapshot.active {
            if !self.bundle_hash.is_empty() && checkpoint.dialogue.as_str().is_empty() {
                self.diagnostics.push(DialogueError::new(
                    DialogueErrorCode::MalformedContent,
                    "active dialogue checkpoint has no dialogue key",
                ));
            } else if self.bundle_hash.is_empty() {
                self.pending_checkpoint_bundle_hash = Some(snapshot.bundle_hash);
                self.pending_checkpoint = Some(checkpoint);
            } else if snapshot.bundle_hash == self.bundle_hash {
                self.pending_checkpoint = Some(checkpoint);
            } else {
                self.diagnostics.push(DialogueError::new(
                    DialogueErrorCode::BundleMismatch,
                    "active dialogue checkpoint was quarantined because the bundle changed",
                ));
            }
        }
    }

    fn report(&mut self, error: DialogueError) {
        self.phase = DialoguePhase::Failed;
        self.diagnostics.push(error);
    }
}

#[derive(Debug, Clone, Message)]
pub(crate) struct DialogueStartRequested(pub DialogueStartRequest);

#[derive(Debug, Clone, Message)]
pub(crate) struct DialogueContinueRequested;

#[derive(Debug, Clone, Message)]
pub(crate) struct DialogueChoiceSelected(pub DialogueChoiceId);

#[derive(Debug, Clone, Message)]
pub(crate) struct DialogueInterrupted {
    pub reason: String,
}

#[derive(Debug, Clone, Message)]
pub(crate) struct DialogueDiagnosticMessage(pub DialogueError);

#[derive(Debug, Clone, Resource, Default)]
pub(crate) struct DialogueActivationRequest {
    pub entity: Option<Entity>,
}

#[derive(Debug, Resource, Default)]
pub(crate) struct DialoguePrefetchQueue {
    pub line_keys: BTreeSet<bevyout_core::dialogue::DialogueLineKey>,
}

#[derive(Debug, Resource, Default)]
pub(crate) struct DialogueTelemetry {
    pub lines_presented: u64,
    pub choices_selected: u64,
    pub commands_enqueued: u64,
    pub diagnostics: u64,
    pub coverage_checks: u64,
    trace_cursor: usize,
}

#[derive(Debug, Clone, Message)]
pub(crate) struct DialogueHotReloadRequested {
    pub source_paths: Vec<String>,
}

pub(crate) struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DialogueRuntime>()
            .init_resource::<DialogueActivationRequest>()
            .init_resource::<DialoguePrefetchQueue>()
            .init_resource::<DialogueTelemetry>()
            .init_resource::<presentation::DialoguePresentationProviders>()
            .init_resource::<presentation::DialogueUiState>()
            .init_resource::<YarnHostBridge>()
            .init_resource::<DialogueHostState>()
            .add_message::<DialogueStartRequested>()
            .add_message::<DialogueContinueRequested>()
            .add_message::<DialogueChoiceSelected>()
            .add_message::<DialogueInterrupted>()
            .add_message::<DialogueDiagnosticMessage>()
            .add_message::<DialogueHotReloadRequested>()
            .configure_sets(
                Update,
                (
                    DialogueSet::Input,
                    DialogueSet::Lifecycle,
                    DialogueSet::Advance,
                    DialogueSet::Host,
                )
                    .chain(),
            )
            .add_systems(Startup, (spawn_primary_runner, install_host_api).chain())
            .add_systems(
                Update,
                (
                    load_prepared_catalog
                        .in_set(DialogueSet::Lifecycle)
                        .before(attach_prepared_dialogue_bindings),
                    attach_prepared_dialogue_bindings.in_set(DialogueSet::Lifecycle),
                    presentation::handle_dialogue_ui_input.in_set(DialogueSet::Input),
                    read_dialogue_input.in_set(DialogueSet::Input),
                    activate_bound_npc.in_set(DialogueSet::Input),
                    populate_dialogue_prefetch.in_set(DialogueSet::Input),
                    route_activation.in_set(DialogueSet::Lifecycle),
                    hot_reload_dialogue.in_set(DialogueSet::Lifecycle),
                    process_dialogue_lifecycle.in_set(DialogueSet::Lifecycle),
                    advance_dialogue.in_set(DialogueSet::Advance),
                    collect_dialogue_diagnostics.in_set(DialogueSet::Host),
                    record_dialogue_telemetry.in_set(DialogueSet::Host),
                    host::apply_host_commands.in_set(DialogueSet::Host),
                    presentation::sync_dialogue_timing
                        .in_set(DialogueSet::Advance)
                        .after(advance_dialogue),
                )
                    .in_set(ViewerSet::Dialogue),
            )
            .add_systems(
                Update,
                presentation::reanchor_dialogue_voice.in_set(ViewerSet::WorldSync),
            )
            .add_systems(
                Update,
                (
                    presentation::update_dialogue_ui,
                    presentation::update_dialogue_ui_styles,
                )
                    .in_set(ViewerSet::Ui),
            )
            .add_systems(OnEnter(GameplayModal::Dialogue), open_dialogue_cursor)
            .add_systems(
                OnExit(GameplayModal::Dialogue),
                (clear_input_gate, close_dialogue_cursor).chain(),
            );
    }
}

fn open_dialogue_cursor(mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
}

fn close_dialogue_cursor(mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }
}

fn activate_bound_npc(
    keys: Res<ButtonInput<KeyCode>>,
    interaction: Option<Res<crate::viewer::interaction::InteractionState>>,
    bindings: Query<&DialogueBinding>,
    mut starts: MessageWriter<DialogueStartRequested>,
) {
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }
    let Some(entity) = interaction.and_then(|interaction| interaction.focused) else {
        return;
    };
    let Ok(binding) = bindings.get(entity) else {
        return;
    };
    starts.write(DialogueStartRequested(DialogueStartRequest {
        dialogue: binding.dialogue.clone(),
        speaker: Some(binding.speaker.into()),
        listener: binding.listener.map(Into::into),
        source: bevyout_core::dialogue::DialogueStartSource::AuthoredNpc,
    }));
}

fn attach_prepared_dialogue_bindings(
    runtime: Res<DialogueRuntime>,
    roots: Query<(Entity, &PlacementRoot, Option<&DialogueBinding>)>,
    mut commands: Commands,
) {
    let Some(catalog) = runtime.catalog.as_ref() else {
        return;
    };
    for (entity, root, existing) in &roots {
        let placement = root.placement();
        let desired = select_prepared_dialogue(
            catalog,
            matches!(&placement.semantic, PreparedSemantic::Npc(_)),
            placement.reference_form_id,
            placement.base_form_id,
            placement.editor_id.as_deref(),
        )
        .map(|dialogue| DialogueBinding {
            dialogue,
            speaker: placement.reference_form_id,
            listener: None,
        });
        match (existing, desired) {
            (Some(current), Some(desired)) if current == &desired => {}
            (_, Some(desired)) => {
                commands.entity(entity).insert(desired);
            }
            (Some(_), None) => {
                commands.entity(entity).remove::<DialogueBinding>();
            }
            (None, None) => {}
        }
    }
}

fn select_prepared_dialogue(
    catalog: &PreparedDialogueCatalog,
    is_npc: bool,
    reference_form_id: u32,
    base_form_id: u32,
    editor_id: Option<&str>,
) -> Option<DialogueKey> {
    if !is_npc {
        return None;
    }
    let binding = catalog
        .actor_bindings
        .iter()
        .find(|binding| binding.actor_reference_form_id == reference_form_id)
        .or_else(|| {
            let mut matching_base = catalog
                .actor_bindings
                .iter()
                .filter(|binding| binding.actor_base_form_id == base_form_id);
            let first = matching_base.next()?;
            matching_base.next().is_none().then_some(first)
        })
        .and_then(|binding| {
            catalog
                .conversation(&binding.dialogue)
                .map(|_| binding.dialogue.clone())
        });
    binding.or_else(|| {
        editor_id.and_then(|editor_id| {
            let dialogue = DialogueKey::new(editor_id);
            catalog.conversation(&dialogue).map(|_| dialogue)
        })
    })
}

fn spawn_primary_runner(mut commands: Commands, mut runtime: ResMut<DialogueRuntime>) {
    if runtime.runner_entity.is_none() {
        runtime.runner_entity = Some(commands.spawn(PrimaryDialogueRunner).id());
    }
}

fn install_host_api(mut bridge: ResMut<YarnHostBridge>) {
    let diagnostics = bridge.install_bevyout_yarn_api();
    bridge.diagnostics.extend(diagnostics);
}

fn collect_dialogue_diagnostics(
    mut messages: MessageReader<DialogueDiagnosticMessage>,
    mut runtime: ResMut<DialogueRuntime>,
) {
    for message in messages.read() {
        runtime.trace.push(format!(
            "diagnostic {:?}: {}",
            message.0.code, message.0.message
        ));
    }
}

fn populate_dialogue_prefetch(
    runtime: Res<DialogueRuntime>,
    mut prefetch: ResMut<DialoguePrefetchQueue>,
) {
    prefetch.line_keys.clear();
    let Some(active) = runtime.active.as_ref() else {
        return;
    };
    let Some(catalog) = runtime.catalog.as_ref() else {
        return;
    };
    let Some(conversation) = catalog.conversation(&active.request.dialogue) else {
        return;
    };
    let Some(node) = conversation.nodes.get(&active.node) else {
        return;
    };
    prefetch.line_keys.extend(
        node.lines
            .iter()
            .skip(active.line_index.saturating_add(1))
            .map(|line| line.key.clone()),
    );
}

fn record_dialogue_telemetry(
    runtime: Res<DialogueRuntime>,
    mut telemetry: ResMut<DialogueTelemetry>,
) {
    for trace in runtime.trace.iter().skip(telemetry.trace_cursor) {
        if trace.starts_with("line ") {
            telemetry.lines_presented += 1;
        } else if trace.starts_with("choice ") {
            telemetry.choices_selected += 1;
        } else if trace.starts_with("diagnostic ") {
            telemetry.diagnostics += 1;
        }
    }
    telemetry.trace_cursor = runtime.trace.len();
}

fn hot_reload_dialogue(
    mut requests: MessageReader<DialogueHotReloadRequested>,
    manifest: Option<Res<super::LoadedSceneManifest>>,
    mut runtime: ResMut<DialogueRuntime>,
) {
    for request in requests.read() {
        let Some(manifest) = manifest.as_deref() else {
            runtime.report(DialogueError::new(
                DialogueErrorCode::NotReady,
                "dialogue hot reload requires a loaded scene manifest",
            ));
            continue;
        };
        let Some(bundle) = manifest.dialogue.as_ref() else {
            runtime.report(DialogueError::new(
                DialogueErrorCode::MissingDialogue,
                "dialogue hot reload has no prepared bundle",
            ));
            continue;
        };
        if runtime.is_active() {
            runtime.report(DialogueError::new(
                DialogueErrorCode::Busy,
                "dialogue hot reload is deferred while a session is active",
            ));
            continue;
        }
        let requested = request
            .source_paths
            .iter()
            .map(|path| path.strip_prefix("dialogue/").unwrap_or(path).to_owned())
            .collect::<BTreeSet<_>>();
        let prepared_sources = bundle
            .source_paths
            .iter()
            .map(|path| path.strip_prefix("dialogue/").unwrap_or(path))
            .collect::<BTreeSet<_>>();
        if requested.is_empty()
            || requested.iter().any(|path| {
                !path.starts_with("authored/") || !prepared_sources.contains(path.as_str())
            })
        {
            runtime.report(DialogueError::new(
                DialogueErrorCode::MalformedContent,
                "hot reload accepts only explicit prepared authored source paths",
            ));
            continue;
        }
        let asset_root = PathBuf::from(&manifest.asset_root);
        let mut sources = Vec::new();
        let mut failed = false;
        for path in &bundle.source_paths {
            let relative = path.strip_prefix("dialogue/").unwrap_or(path);
            let source_path = prepared_dialogue_source_path(&asset_root, bundle, path);
            match fs::read_to_string(&source_path) {
                Ok(content) => sources.push(crate::vsa::dialogue::DialogueSource {
                    relative_path: relative.into(),
                    kind: prepared_dialogue_source_kind(path),
                    content,
                }),
                Err(error) => {
                    failed = true;
                    runtime.report(DialogueError::new(
                        DialogueErrorCode::MalformedContent,
                        format!(
                            "hot reload could not read {}: {error}",
                            source_path.display()
                        ),
                    ));
                }
            }
        }
        if !failed {
            let (actor_bindings, authored_voice_manifest_paths) = runtime
                .catalog
                .as_ref()
                .map(|catalog| {
                    (
                        catalog.actor_bindings.clone(),
                        catalog.authored_voice_manifest_paths.clone(),
                    )
                })
                .unwrap_or_default();
            let mut catalog =
                crate::vsa::dialogue::prepare_catalog_with_actor_bindings(sources, actor_bindings);
            catalog.authored_voice_manifest_paths = authored_voice_manifest_paths;
            runtime.set_catalog(catalog);
        }
    }
}

fn prepared_dialogue_source_path(
    asset_root: &std::path::Path,
    bundle: &bevyout_core::dialogue::PreparedDialogueBundleRef,
    source_path: &str,
) -> PathBuf {
    let catalog_path = asset_root.join(
        bundle
            .catalog_path
            .replace('/', std::path::MAIN_SEPARATOR_STR),
    );
    let dialogue_root = catalog_path.parent().unwrap_or(asset_root);
    let relative = source_path
        .strip_prefix("dialogue/")
        .unwrap_or(source_path)
        .replace('/', std::path::MAIN_SEPARATOR_STR);
    dialogue_root.join(relative)
}

fn prepared_dialogue_source_kind(source_path: &str) -> crate::vsa::dialogue::DialogueSourceKind {
    if source_path
        .strip_prefix("dialogue/")
        .unwrap_or(source_path)
        .starts_with("authored/")
    {
        crate::vsa::dialogue::DialogueSourceKind::Authored
    } else {
        crate::vsa::dialogue::DialogueSourceKind::ImportedGenerated
    }
}

fn load_prepared_catalog(
    mut runtime: ResMut<DialogueRuntime>,
    mut providers: ResMut<presentation::DialoguePresentationProviders>,
    manifest: Option<Res<super::LoadedSceneManifest>>,
) {
    if runtime.catalog.is_some() {
        return;
    }
    let Some(manifest) = manifest else {
        return;
    };
    let Some(bundle) = manifest.dialogue.as_ref() else {
        return;
    };
    let catalog_path = PathBuf::from(&manifest.asset_root).join(
        bundle
            .catalog_path
            .replace('/', std::path::MAIN_SEPARATOR_STR),
    );
    let Some(catalog) = fs::read(&catalog_path)
        .ok()
        .and_then(|bytes| ron::de::from_bytes::<PreparedDialogueCatalog>(&bytes).ok())
    else {
        runtime.report(DialogueError::new(
            DialogueErrorCode::MissingDialogue,
            format!(
                "unable to load prepared dialogue catalog {}",
                catalog_path.display()
            ),
        ));
        return;
    };

    load_dialogue_catalog_with_voice(&mut runtime, &mut providers, &manifest, bundle, catalog);
}

fn load_dialogue_catalog_with_voice(
    runtime: &mut DialogueRuntime,
    providers: &mut presentation::DialoguePresentationProviders,
    manifest: &super::LoadedSceneManifest,
    bundle: &bevyout_core::dialogue::PreparedDialogueBundleRef,
    catalog: PreparedDialogueCatalog,
) {
    providers.voice.clear();
    let voice_index = bundle.voice_index_path.as_deref().and_then(|path| {
        load_prepared_voice_index(
            &PathBuf::from(&manifest.asset_root),
            path,
            &catalog,
            &mut runtime.diagnostics,
            &mut providers.voice,
        )
    });
    let voice_demand = bundle.voice_demand_path.as_deref().and_then(|path| {
        load_prepared_voice_demand_report(
            &PathBuf::from(&manifest.asset_root),
            path,
            &mut runtime.diagnostics,
        )
    });
    let metadata_error = crate::vsa::dialogue::validate_dialogue_bundle_metadata(
        bundle,
        &catalog,
        voice_index.as_ref(),
        voice_demand.as_ref(),
    )
    .err();
    #[cfg(feature = "dialogue-yarn")]
    if let Err(diagnostics) =
        yarn::compile_sources(&PathBuf::from(&manifest.asset_root), &catalog.source_paths)
    {
        for diagnostic in diagnostics {
            runtime.diagnostics.push(DialogueError::new(
                DialogueErrorCode::MalformedContent,
                diagnostic.message,
            ));
        }
        runtime.readiness = DialogueReadiness::Failed;
        runtime.phase = DialoguePhase::Failed;
    } else {
        runtime.set_catalog(catalog);
        if let Some(error) = metadata_error {
            runtime.readiness = DialogueReadiness::Failed;
            runtime.report(DialogueError::new(
                DialogueErrorCode::BundleMismatch,
                error.to_string(),
            ));
        }
    }
    #[cfg(not(feature = "dialogue-yarn"))]
    {
        runtime.set_catalog(catalog);
        if let Some(error) = metadata_error {
            runtime.readiness = DialogueReadiness::Failed;
            runtime.report(DialogueError::new(
                DialogueErrorCode::BundleMismatch,
                error.to_string(),
            ));
        }
    }
}

fn load_prepared_voice_demand_report(
    asset_root: &std::path::Path,
    relative_path: &str,
    diagnostics: &mut Vec<DialogueError>,
) -> Option<bevyout_core::dialogue::PreparedDialogueVoiceDemandReport> {
    let path = asset_root.join(relative_path.replace('/', std::path::MAIN_SEPARATOR_STR));
    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) => {
            diagnostics.push(DialogueError::new(
                DialogueErrorCode::MissingDialogue,
                format!(
                    "unable to load prepared dialogue voice demand {}: {error}",
                    path.display()
                ),
            ));
            return None;
        }
    };
    let report = match ron::de::from_bytes::<
        bevyout_core::dialogue::PreparedDialogueVoiceDemandReport,
    >(&bytes)
    {
        Ok(report) => report,
        Err(error) => {
            diagnostics.push(DialogueError::new(
                DialogueErrorCode::MalformedContent,
                format!(
                    "unable to parse prepared dialogue voice demand {}: {error}",
                    path.display()
                ),
            ));
            return None;
        }
    };
    if report.revision != bevyout_core::dialogue::DIALOGUE_VOICE_DEMAND_REVISION {
        diagnostics.push(DialogueError::new(
            DialogueErrorCode::MalformedContent,
            format!(
                "unsupported prepared dialogue voice demand revision {}",
                report.revision
            ),
        ));
        return None;
    }
    Some(report)
}

pub(super) fn load_prepared_voice_index(
    asset_root: &std::path::Path,
    relative_path: &str,
    catalog: &PreparedDialogueCatalog,
    diagnostics: &mut Vec<DialogueError>,
    voices: &mut BTreeMap<
        (DialogueLineKey, Option<u32>),
        bevyout_core::dialogue::DialogueVoiceAsset,
    >,
) -> Option<PreparedDialogueVoiceIndex> {
    let path = asset_root.join(relative_path.replace('/', std::path::MAIN_SEPARATOR_STR));
    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) => {
            diagnostics.push(DialogueError::new(
                DialogueErrorCode::MissingDialogue,
                format!(
                    "unable to load prepared dialogue voice index {}: {error}",
                    path.display()
                ),
            ));
            return None;
        }
    };
    let index = match ron::de::from_bytes::<PreparedDialogueVoiceIndex>(&bytes) {
        Ok(index) => index,
        Err(error) => {
            diagnostics.push(DialogueError::new(
                DialogueErrorCode::MalformedContent,
                format!(
                    "unable to parse prepared dialogue voice index {}: {error}",
                    path.display()
                ),
            ));
            return None;
        }
    };
    if index.revision != DIALOGUE_VOICE_INDEX_REVISION {
        diagnostics.push(DialogueError::new(
            DialogueErrorCode::MalformedContent,
            format!(
                "unsupported prepared dialogue voice index revision {}",
                index.revision
            ),
        ));
        return None;
    }
    for diagnostic in &index.diagnostics {
        diagnostics.push(DialogueError::new(
            DialogueErrorCode::MalformedContent,
            format!("voice index {}: {}", diagnostic.code, diagnostic.message),
        ));
    }

    let mut seen = BTreeSet::new();
    for voice in &index.entries {
        if !seen.insert((voice.line_key.clone(), voice.speaker_form_id)) {
            diagnostics.push(DialogueError::new(
                DialogueErrorCode::MalformedContent,
                format!(
                    "prepared dialogue voice index repeats {} for speaker {}",
                    voice.line_key,
                    voice
                        .speaker_form_id
                        .map(|speaker| format!("{speaker:08x}"))
                        .unwrap_or_else(|| "<unscoped>".into())
                ),
            ));
            continue;
        }
        if !catalog.line_keys.contains(&voice.line_key) {
            diagnostics.push(DialogueError::new(
                DialogueErrorCode::MalformedContent,
                format!(
                    "prepared dialogue voice index references unknown {}",
                    voice.line_key
                ),
            ));
            continue;
        }
        let relative_asset = std::path::Path::new(&voice.asset_path);
        if relative_asset.is_absolute()
            || relative_asset.components().any(|component| {
                component == std::path::Component::ParentDir
                    || component == std::path::Component::RootDir
            })
            || !matches!(
                std::path::Path::new(&voice.asset_path)
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .map(|extension| extension.to_ascii_lowercase())
                    .as_deref(),
                Some("wav" | "ogg")
            )
        {
            diagnostics.push(DialogueError::new(
                DialogueErrorCode::MalformedContent,
                format!(
                    "prepared dialogue voice asset path is invalid: {}",
                    voice.asset_path
                ),
            ));
            continue;
        }
        let asset_file =
            asset_root.join(voice.asset_path.replace('/', std::path::MAIN_SEPARATOR_STR));
        if !asset_file.is_file() {
            diagnostics.push(DialogueError::new(
                DialogueErrorCode::MissingDialogue,
                format!(
                    "prepared dialogue voice asset is missing: {}",
                    asset_file.display()
                ),
            ));
            continue;
        }
        voices.insert(
            (voice.line_key.clone(), voice.speaker_form_id),
            voice.clone(),
        );
    }
    Some(index)
}

fn read_dialogue_input(
    keys: Res<ButtonInput<KeyCode>>,
    runtime: Option<Res<DialogueRuntime>>,
    mut continue_messages: MessageWriter<DialogueContinueRequested>,
    mut choices: MessageWriter<DialogueChoiceSelected>,
) {
    let Some(runtime) = runtime else { return };
    if runtime.input_gated {
        if runtime.phase == DialoguePhase::PresentingOptions {
            let pressed = [
                (KeyCode::Digit1, 0usize),
                (KeyCode::Digit2, 1),
                (KeyCode::Digit3, 2),
                (KeyCode::Digit4, 3),
                (KeyCode::Digit5, 4),
                (KeyCode::Digit6, 5),
                (KeyCode::Digit7, 6),
                (KeyCode::Digit8, 7),
                (KeyCode::Digit9, 8),
            ];
            if let Some((_, index)) = pressed.iter().find(|(key, _)| keys.just_pressed(*key))
                && let Some(option) = runtime.presentation.options.get(*index)
            {
                choices.write(DialogueChoiceSelected(option.choice.clone()));
            }
        } else if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
            continue_messages.write(DialogueContinueRequested);
        }
    }
}

fn route_activation(
    activation: Option<Res<DialogueActivationRequest>>,
    bindings: Query<&DialogueBinding>,
    mut starts: MessageWriter<DialogueStartRequested>,
) {
    let Some(activation) = activation else { return };
    let Some(entity) = activation.entity else {
        return;
    };
    let Ok(binding) = bindings.get(entity) else {
        return;
    };
    starts.write(DialogueStartRequested(DialogueStartRequest {
        dialogue: binding.dialogue.clone(),
        speaker: Some(binding.speaker.into()),
        listener: binding.listener.map(Into::into),
        source: bevyout_core::dialogue::DialogueStartSource::AuthoredNpc,
    }));
}

fn process_dialogue_lifecycle(
    mut starts: MessageReader<DialogueStartRequested>,
    mut interruptions: MessageReader<DialogueInterrupted>,
    mut runtime: ResMut<DialogueRuntime>,
    mut modal_requests: MessageWriter<RequestStateTransition>,
    mut diagnostics: MessageWriter<DialogueDiagnosticMessage>,
) {
    for interruption in interruptions.read() {
        if runtime.active.is_some() {
            close_dialogue(&mut runtime, &mut modal_requests);
            runtime.report(DialogueError::new(
                DialogueErrorCode::Interrupted,
                interruption.reason.clone(),
            ));
        }
    }
    for message in starts.read() {
        if runtime.is_active() {
            let error = match runtime.busy_policy {
                DialogueBusyPolicy::Reject => {
                    DialogueError::new(DialogueErrorCode::Busy, "dialogue runner is already busy")
                }
            };
            diagnostics.write(DialogueDiagnosticMessage(error.clone()));
            runtime.diagnostics.push(error);
            continue;
        }
        if runtime.readiness != DialogueReadiness::Ready {
            let error =
                DialogueError::new(DialogueErrorCode::NotReady, "dialogue content is not ready");
            diagnostics.write(DialogueDiagnosticMessage(error.clone()));
            runtime.diagnostics.push(error);
            continue;
        }
        runtime.pending_starts.push_back(message.0.clone());
    }
    if runtime.active.is_none() {
        let Some(request) = runtime.pending_starts.pop_front() else {
            return;
        };
        let Some(catalog) = runtime.catalog.clone() else {
            return;
        };
        let Some(conversation) = catalog.conversation(&request.dialogue) else {
            let error = DialogueError {
                code: DialogueErrorCode::MissingDialogue,
                message: format!(
                    "dialogue {} is not in the prepared catalog",
                    request.dialogue
                ),
                dialogue: Some(request.dialogue.clone()),
                source: None,
            };
            diagnostics.write(DialogueDiagnosticMessage(error.clone()));
            runtime.report(error);
            return;
        };
        let checkpoint = runtime.pending_checkpoint.take();
        let node_name = checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.node.as_str())
            .unwrap_or(request.dialogue.as_str());
        let Some(node) = conversation
            .nodes
            .get(node_name)
            .or_else(|| conversation.nodes.get("Start"))
            .or_else(|| conversation.nodes.values().next())
        else {
            let error = DialogueError::new(
                DialogueErrorCode::MissingNode,
                "dialogue has no executable node",
            );
            diagnostics.write(DialogueDiagnosticMessage(error.clone()));
            runtime.report(error);
            return;
        };
        let completed_actions = checkpoint
            .map(|checkpoint| {
                checkpoint
                    .completed_actions
                    .into_iter()
                    .map(|key| key.0)
                    .collect()
            })
            .unwrap_or_default();
        runtime.active = Some(ActiveDialogue {
            request,
            session: DialogueSessionId::new(format!("dialogue-{}", runtime.next_session)),
            node: node.name.clone(),
            line_index: 0,
            completed_actions,
        });
        runtime.next_session += 1;
        runtime.phase = DialoguePhase::Running;
        runtime.ui_phase = DialogueUiPhase::Revealing;
        runtime.input_gated = true;
        runtime.camera_focused = true;
        runtime.line_elapsed_seconds = 0.0;
        runtime.line_duration_seconds = 0.0;
        runtime.active_line_key = None;
        runtime.voice_anchor = DialogueVoiceAnchorKind::Unanchored;
        runtime.voice_timing = DialogueVoiceTimingState::None;
        runtime.voice_load_elapsed_seconds = 0.0;
        modal_requests.write(RequestStateTransition::Modal(GameplayModal::Dialogue));
        let session = runtime
            .active
            .as_ref()
            .map(|active| active.session.to_string())
            .unwrap_or_default();
        runtime
            .trace
            .push(format!("start {} session={session}", node.name));
        present_node(&mut runtime, node);
    }
}

fn advance_dialogue(
    mut continue_messages: MessageReader<DialogueContinueRequested>,
    mut choices: MessageReader<DialogueChoiceSelected>,
    mut runtime: ResMut<DialogueRuntime>,
    mut bridge: ResMut<YarnHostBridge>,
    mut telemetry: ResMut<DialogueTelemetry>,
    mut modal_requests: MessageWriter<RequestStateTransition>,
) {
    for _ in continue_messages.read() {
        runtime.continue_edge = true;
    }
    for choice in choices.read() {
        runtime.selected_choice = Some(choice.0.clone());
    }
    let Some(active_snapshot) = runtime.active.clone() else {
        runtime.continue_edge = false;
        runtime.selected_choice = None;
        return;
    };
    let Some(catalog) = runtime.catalog.clone() else {
        return;
    };
    let Some(conversation) = catalog.conversation(&active_snapshot.request.dialogue) else {
        return;
    };
    let Some(node) = conversation.nodes.get(&active_snapshot.node) else {
        runtime.report(DialogueError::new(
            DialogueErrorCode::MissingNode,
            "active dialogue node disappeared",
        ));
        close_dialogue(&mut runtime, &mut modal_requests);
        return;
    };

    if runtime.phase == DialoguePhase::WaitingCommand {
        for action in &node.commands {
            if runtime.completed_action_keys.insert(action.to_string()) {
                if let Some(active) = runtime.active.as_mut() {
                    active.completed_actions.insert(action.to_string());
                }
                match bridge.command_from_text(action.as_str(), action.as_str()) {
                    Ok(command) => {
                        bridge.enqueue(command);
                        telemetry.commands_enqueued += 1;
                    }
                    Err(error) => runtime.diagnostics.push(error),
                }
            }
        }
        runtime.phase = DialoguePhase::Closing;
    }
    if runtime.phase == DialoguePhase::Closing {
        close_dialogue(&mut runtime, &mut modal_requests);
        return;
    }

    if runtime.phase == DialoguePhase::PresentingOptions {
        let Some(choice) = runtime.selected_choice.take() else {
            return;
        };
        let Some(option) = node.options.iter().find(|option| option.choice == choice) else {
            runtime.report(DialogueError::new(
                DialogueErrorCode::MalformedContent,
                "selected dialogue option is not present",
            ));
            close_dialogue(&mut runtime, &mut modal_requests);
            return;
        };
        runtime.trace.push(format!("choice {}", choice));
        if let Some(destination) = option
            .destination
            .as_deref()
            .filter(|destination| !destination.is_empty())
        {
            if conversation.nodes.contains_key(destination) {
                if let Some(active) = runtime.active.as_mut() {
                    active.node = destination.into();
                    active.line_index = 0;
                }
                runtime.presentation.line = None;
                runtime.presentation.options.clear();
                runtime.phase = DialoguePhase::Running;
                present_node(
                    &mut runtime,
                    conversation.nodes.get(destination).expect("checked"),
                );
            } else {
                runtime.report(DialogueError::new(
                    DialogueErrorCode::MissingNode,
                    format!("option target {destination} is missing"),
                ));
                close_dialogue(&mut runtime, &mut modal_requests);
            }
        } else {
            close_dialogue(&mut runtime, &mut modal_requests);
        }
        runtime.continue_edge = false;
        return;
    }
    if runtime.phase == DialoguePhase::PresentingLine && runtime.continue_edge {
        if let Some(active) = runtime.active.as_mut() {
            active.line_index += 1;
        }
        runtime.continue_edge = false;
        present_node(&mut runtime, node);
    }
}

fn present_node(runtime: &mut DialogueRuntime, node: &PreparedDialogueNode) {
    let Some(active) = runtime.active.as_ref() else {
        return;
    };
    if active.line_index < node.lines.len() {
        let line = &node.lines[active.line_index];
        let speaker = DialogueSpeaker {
            stable_id: active.request.speaker,
            display_name: prepared_speaker_display_name(
                runtime.catalog.as_ref(),
                &active.request.dialogue,
                active.request.speaker,
                line.speaker.as_deref(),
            ),
        };
        runtime.speaker = speaker.clone();
        runtime.voice_anchor = DialogueVoiceAnchorKind::Unanchored;
        runtime.presentation.line = Some(DialogueLinePresentation {
            line_key: line.key.clone(),
            text: line.text.clone(),
            speaker,
            voice_key: None,
            localization_key: Some(line.key.to_string()),
            reveal_seconds: 0.0,
        });
        runtime.presentation.options.clear();
        runtime.trace.push(format!("line {}", line.key));
        runtime.phase = DialoguePhase::PresentingLine;
        runtime.ui_phase = DialogueUiPhase::Continue;
        return;
    }
    if !node.options.is_empty() {
        runtime.presentation.line = None;
        runtime.active_line_key = None;
        runtime.line_elapsed_seconds = 0.0;
        runtime.line_duration_seconds = 0.0;
        runtime.voice_anchor = DialogueVoiceAnchorKind::Unanchored;
        runtime.voice_timing = DialogueVoiceTimingState::None;
        runtime.voice_load_elapsed_seconds = 0.0;
        runtime.presentation.options = node
            .options
            .iter()
            .map(
                |option| bevyout_core::dialogue::DialogueOptionPresentation {
                    choice: option.choice.clone(),
                    text: option.text.clone(),
                    line_key: None,
                    enabled: true,
                },
            )
            .collect();
        runtime.phase = DialoguePhase::PresentingOptions;
        runtime.ui_phase = DialogueUiPhase::Choice;
    } else {
        runtime.phase = DialoguePhase::WaitingCommand;
        runtime.ui_phase = DialogueUiPhase::Command;
    }
}

fn prepared_speaker_display_name(
    catalog: Option<&PreparedDialogueCatalog>,
    dialogue: &DialogueKey,
    speaker: Option<bevyout_core::form_id::FormId>,
    authored_fallback: Option<&str>,
) -> String {
    speaker
        .and_then(|speaker| {
            catalog.and_then(|catalog| {
                catalog.actor_bindings.iter().find(|binding| {
                    binding.dialogue == *dialogue && binding.actor_reference_form_id == speaker.0
                })
            })
        })
        .and_then(|binding| binding.actor_display_name.as_deref())
        .filter(|name| !name.trim().is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| authored_fallback.unwrap_or_default().to_owned())
}

fn close_dialogue(
    runtime: &mut DialogueRuntime,
    modal_requests: &mut MessageWriter<RequestStateTransition>,
) {
    runtime.active = None;
    runtime.pending_commands.clear();
    runtime.presentation = DialoguePresentation::default();
    runtime.speaker = DialogueSpeaker::default();
    runtime.line_elapsed_seconds = 0.0;
    runtime.line_duration_seconds = 0.0;
    runtime.active_line_key = None;
    runtime.voice_anchor = DialogueVoiceAnchorKind::Unanchored;
    runtime.voice_timing = DialogueVoiceTimingState::None;
    runtime.voice_load_elapsed_seconds = 0.0;
    runtime.phase = DialoguePhase::Ready;
    runtime.ui_phase = DialogueUiPhase::Closing;
    runtime.input_gated = false;
    runtime.camera_focused = false;
    runtime.variables.clear_session_boundary();
    modal_requests.write(RequestStateTransition::Modal(GameplayModal::None));
}

fn clear_input_gate(mut runtime: ResMut<DialogueRuntime>) {
    runtime.input_gated = false;
    runtime.camera_focused = false;
    runtime.continue_edge = false;
    runtime.selected_choice = None;
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
