use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

use bevyout_core::dialogue::{
    DialogueKey, DialoguePhase, DialogueStartRequest, DialogueStartSource,
};

use super::*;
use crate::app_state::{AppState, AppStatePlugin, GameplayModal, RequestStateTransition};
use crate::vsa::dialogue::{DialogueSource, DialogueSourceKind, prepare_catalog};

fn app_with_dialogue() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, AppStatePlugin, DialoguePlugin));
    app.world_mut()
        .resource_mut::<Messages<RequestStateTransition>>()
        .write(RequestStateTransition::App(AppState::Loading));
    app.update();
    app.update();
    app.world_mut()
        .resource_mut::<Messages<RequestStateTransition>>()
        .write(RequestStateTransition::App(AppState::InGame));
    app.update();
    app.update();
    app
}

fn catalog() -> crate::vsa::dialogue::PreparedDialogueCatalog {
    prepare_catalog(vec![DialogueSource {
        relative_path: "authored/guard.yarn".into(),
        kind: DialogueSourceKind::Authored,
        content: "title: Guard\n---\nGuard: Halt.\n-> Leave -> End\n===\ntitle: End\n---\nGuard: Leave\n===\n".into(),
    }])
}

#[test]
fn authored_start_presents_a_line_and_restores_controls_after_choice() {
    let mut app = app_with_dialogue();
    let prepared = catalog();
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .set_catalog(prepared);
    assert_eq!(
        *app.world().resource::<State<AppState>>().get(),
        AppState::InGame
    );
    app.world_mut()
        .resource_mut::<Messages<DialogueStartRequested>>()
        .write(DialogueStartRequested(DialogueStartRequest {
            dialogue: DialogueKey::new("Guard"),
            speaker: Some(0x10.into()),
            listener: Some(0x20.into()),
            source: DialogueStartSource::AuthoredNpc,
        }));
    app.update();
    app.update();
    assert_eq!(
        app.world().resource::<DialogueRuntime>().phase,
        DialoguePhase::PresentingLine
    );
    assert!(app.world().resource::<DialogueRuntime>().input_gated);
    assert_eq!(
        *app.world().resource::<State<GameplayModal>>().get(),
        GameplayModal::Dialogue
    );

    app.world_mut()
        .resource_mut::<Messages<DialogueContinueRequested>>()
        .write(DialogueContinueRequested);
    app.update();
    assert_eq!(
        app.world().resource::<DialogueRuntime>().phase,
        DialoguePhase::PresentingOptions
    );

    let choice = app
        .world()
        .resource::<DialogueRuntime>()
        .presentation
        .options[0]
        .choice
        .clone();
    app.world_mut()
        .resource_mut::<Messages<DialogueChoiceSelected>>()
        .write(DialogueChoiceSelected(choice));
    app.update();
    assert!(app.world().resource::<DialogueRuntime>().is_active());
    app.world_mut()
        .resource_mut::<Messages<DialogueContinueRequested>>()
        .write(DialogueContinueRequested);
    app.update();
    app.update();
    app.update();
    assert!(!app.world().resource::<DialogueRuntime>().input_gated);
    assert!(!app.world().resource::<DialogueRuntime>().is_active());
    assert_eq!(
        *app.world().resource::<State<GameplayModal>>().get(),
        GameplayModal::None
    );
}

#[test]
fn host_registration_and_exactly_once_action_mutation_are_deterministic() {
    let mut bridge = YarnHostBridge::default();
    assert!(bridge.install_bevyout_yarn_api().is_empty());
    assert_eq!(bridge.functions["bo_item_count"].support, "pure");
    assert_eq!(bridge.commands["bo_run_action"].support, "deferred");
    bridge.enqueue(HostCommand::RunAction {
        key: "action:1".into(),
        action: "set_quest".into(),
    });
    bridge.enqueue(HostCommand::RunAction {
        key: "action:1".into(),
        action: "set_quest".into(),
    });
    let mut app = App::new();
    app.insert_resource(bridge);
    app.init_resource::<DialogueHostState>();
    app.add_systems(Update, host::apply_host_commands);
    app.update();
    assert_eq!(app.world().resource::<DialogueHostState>().trace.len(), 1);
    let bridge = app.world().resource::<YarnHostBridge>();
    assert_eq!(
        bridge
            .evaluate_function(
                app.world().resource::<DialogueHostState>(),
                "bo_item_count",
                &["10"]
            )
            .unwrap(),
        bevyout_core::dialogue::NarrativeValue::Number(0)
    );
}

#[test]
fn host_async_completion_handles_are_explicit_and_one_shot() {
    let mut bridge = YarnHostBridge::default();
    let handle = bridge.enqueue_async(HostCommand::EndDialogue, "dialogue-command-1");
    assert_eq!(handle, "dialogue-command-1");
    assert!(bridge.complete_async(&handle));
    assert!(!bridge.complete_async(&handle));
}

#[test]
fn compatible_checkpoint_resumes_at_the_authored_node() {
    let mut app = app_with_dialogue();
    let prepared = catalog();
    let bundle_hash = prepared.bundle_hash();
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .set_catalog(prepared);
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .restore_snapshot(bevyout_core::dialogue::DialogueSnapshot {
            schema_version: bevyout_core::dialogue::DIALOGUE_SNAPSHOT_SCHEMA_VERSION,
            bundle_hash,
            variables: Default::default(),
            active: Some(bevyout_core::dialogue::ActiveDialogueCheckpoint {
                dialogue: DialogueKey::new("Guard"),
                node: "End".into(),
                speaker: Some(0x10.into()),
                listener: Some(0x20.into()),
                completed_actions: Vec::new(),
            }),
        });
    app.world_mut()
        .resource_mut::<Messages<DialogueStartRequested>>()
        .write(DialogueStartRequested(DialogueStartRequest {
            dialogue: DialogueKey::new("Guard"),
            speaker: Some(0x10.into()),
            listener: Some(0x20.into()),
            source: DialogueStartSource::CheckpointResume,
        }));
    app.update();
    app.update();
    assert_eq!(
        app.world()
            .resource::<DialogueRuntime>()
            .presentation
            .line
            .as_ref()
            .unwrap()
            .text,
        "Leave"
    );
}

#[test]
fn checkpoint_bundle_mismatch_is_quarantined_without_starting_stale_content() {
    let mut app = app_with_dialogue();
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .set_catalog(catalog());
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .restore_snapshot(bevyout_core::dialogue::DialogueSnapshot {
            schema_version: bevyout_core::dialogue::DIALOGUE_SNAPSHOT_SCHEMA_VERSION,
            bundle_hash: "stale-bundle".into(),
            variables: Default::default(),
            active: Some(bevyout_core::dialogue::ActiveDialogueCheckpoint {
                dialogue: DialogueKey::new("Guard"),
                node: "End".into(),
                speaker: None,
                listener: None,
                completed_actions: Vec::new(),
            }),
        });
    let runtime = app.world().resource::<DialogueRuntime>();
    assert!(runtime.pending_checkpoint.is_none());
    assert!(
        runtime
            .diagnostics
            .iter()
            .any(|error| error.code == DialogueErrorCode::BundleMismatch)
    );
}
