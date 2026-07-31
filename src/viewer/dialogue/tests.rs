use bevy::asset::AssetLoader;
use bevy::audio::PlaybackMode;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

use bevyout_core::dialogue::{
    DIALOGUE_VOICE_INDEX_REVISION, DialogueKey, DialogueLineKey, DialoguePhase,
    DialogueStartRequest, DialogueStartSource, DialogueVoiceAsset, PreparedDialogueVoiceIndex,
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

#[test]
fn bevy_runtime_audio_loader_registers_original_ogg_vorbis_assets() {
    let loader = bevy::audio::AudioLoader;
    assert!(loader.extensions().contains(&"ogg"));
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
    app.update();
    assert_eq!(
        app.world().resource::<DialogueRuntime>().phase,
        DialoguePhase::PresentingOptions
    );
    assert!(
        app.world()
            .resource::<DialogueRuntime>()
            .presentation
            .line
            .is_none()
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
fn focused_authored_npc_starts_from_the_existing_use_key() {
    let mut app = app_with_dialogue();
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .set_catalog(catalog());
    let entity = app
        .world_mut()
        .spawn(DialogueBinding {
            dialogue: DialogueKey::new("Guard"),
            speaker: 0x10,
            listener: None,
        })
        .id();
    app.insert_resource(crate::viewer::interaction::InteractionState {
        focused: Some(entity),
        open: Default::default(),
    });
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyE);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
    app.update();

    let runtime = app.world().resource::<DialogueRuntime>();
    assert_eq!(runtime.phase, DialoguePhase::PresentingLine);
    assert_eq!(
        runtime
            .presentation
            .line
            .as_ref()
            .map(|line| line.text.as_str()),
        Some("Halt.")
    );
}

#[test]
fn moira_uses_the_exact_generated_fallout_conversation_binding() {
    let source_path = "dialogue/generated/actors/0002d2bc.yarn";
    let dialogue = DialogueKey::new("fallout_actor_0002d2bc");
    let mut prepared = prepare_catalog(vec![DialogueSource {
        relative_path: source_path.into(),
        kind: DialogueSourceKind::ImportedGenerated,
        content: "title: fallout_actor_0002d2bc\nmode: imported\n---\n// bo_line_key: fallout:fallout3.esm:0001d76a:1\nSpeaker0002d2bc: Exact line\n-> Real option -> fallout_actor_0002d2bc_topic\n===\ntitle: fallout_actor_0002d2bc_topic\nmode: imported\n---\n// bo_line_key: fallout:fallout3.esm:0002d2b5:1\nSpeaker0002d2bc: Exact answer\n===\n".into(),
    }]);
    prepared
        .actor_bindings
        .push(crate::vsa::dialogue::PreparedDialogueActorBinding {
            actor_reference_form_id: 0x0002_d2bc,
            actor_base_form_id: 0x0002_d3c0,
            actor_editor_id: Some("MoiraBrown".into()),
            actor_display_name: Some("Moira Brown".into()),
            dialogue: dialogue.clone(),
            source_path: source_path.into(),
        });

    assert_eq!(
        select_prepared_dialogue(
            &prepared,
            true,
            0x0002_d2bc,
            0x0002_d3c0,
            Some("MoiraBrown"),
        ),
        Some(dialogue)
    );

    let mut app = app_with_dialogue();
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .set_catalog(prepared);
    let entity = app
        .world_mut()
        .spawn(DialogueBinding {
            dialogue: DialogueKey::new("fallout_actor_0002d2bc"),
            speaker: 0x0002_d2bc,
            listener: None,
        })
        .id();
    app.insert_resource(crate::viewer::interaction::InteractionState {
        focused: Some(entity),
        open: Default::default(),
    });
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyE);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
    app.update();

    let line = app
        .world()
        .resource::<DialogueRuntime>()
        .presentation
        .line
        .as_ref()
        .expect("Moira's first generated line is presented");
    assert_eq!(line.speaker.display_name, "Moira Brown");
}

#[test]
fn dialogue_ui_root_hides_after_the_session_closes() {
    let mut app = app_with_dialogue();
    let root = app
        .world_mut()
        .spawn((presentation::DialogueUiRoot, Visibility::Inherited))
        .id();
    app.world_mut().resource_mut::<DialogueRuntime>().ui_phase = DialogueUiPhase::Hidden;
    app.update();
    assert_eq!(
        app.world().get::<Visibility>(root),
        Some(&Visibility::Hidden)
    );

    app.world_mut().resource_mut::<DialogueRuntime>().ui_phase = DialogueUiPhase::Continue;
    app.update();
    assert_eq!(
        app.world().get::<Visibility>(root),
        Some(&Visibility::Inherited)
    );
}

#[test]
fn dialogue_visual_states_match_the_fallout_three_layout() {
    let mut app = app_with_dialogue();
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .set_catalog(catalog());
    app.world_mut()
        .resource_mut::<Messages<DialogueStartRequested>>()
        .write(DialogueStartRequested(DialogueStartRequest {
            dialogue: DialogueKey::new("Guard"),
            speaker: Some(0x10.into()),
            listener: None,
            source: DialogueStartSource::AuthoredNpc,
        }));
    app.update();
    app.update();

    let world = app.world_mut();
    let mut panels = world.query_filtered::<&BorderColor, With<presentation::DialogueUiPanel>>();
    assert_eq!(
        *panels.single(world).expect("dialogue panel"),
        BorderColor::all(Color::NONE)
    );
    let mut lines =
        world.query_filtered::<Option<&BorderColor>, With<presentation::DialogueUiLineButton>>();
    assert_eq!(
        *lines
            .single(world)
            .expect("dialogue line hit target")
            .expect("explicit borderless line style"),
        BorderColor::all(Color::NONE)
    );

    app.world_mut()
        .resource_mut::<Messages<DialogueContinueRequested>>()
        .write(DialogueContinueRequested);
    app.update();
    app.update();

    assert!(
        app.world()
            .resource::<DialogueRuntime>()
            .presentation
            .line
            .is_none()
    );
    let world = app.world_mut();
    let mut panels = world.query_filtered::<&BorderColor, With<presentation::DialogueUiPanel>>();
    assert_eq!(
        *panels.single(world).expect("dialogue panel"),
        BorderColor::all(crate::viewer::fallout_ui::PHOSPHOR_DIM)
    );
    let mut options = world.query_filtered::<
        (&BackgroundColor, Option<&BorderColor>),
        With<presentation::DialogueUiOptionButton>,
    >();
    let options = options.iter(world).collect::<Vec<_>>();
    assert_eq!(options.len(), 1);
    assert_eq!(options[0].0.0, Color::NONE);
    assert_eq!(
        *options[0].1.expect("explicit borderless option style"),
        BorderColor::all(Color::NONE)
    );
}

#[test]
fn clicked_option_uses_the_same_choice_message_as_keyboard_selection() {
    let mut app = app_with_dialogue();
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .set_catalog(catalog());
    app.world_mut()
        .resource_mut::<Messages<DialogueStartRequested>>()
        .write(DialogueStartRequested(DialogueStartRequest {
            dialogue: DialogueKey::new("Guard"),
            speaker: Some(0x10.into()),
            listener: None,
            source: DialogueStartSource::AuthoredNpc,
        }));
    app.update();
    app.update();
    app.world_mut()
        .resource_mut::<Messages<DialogueContinueRequested>>()
        .write(DialogueContinueRequested);
    app.update();
    app.update();

    let option = {
        let world = app.world_mut();
        let mut query =
            world.query_filtered::<Entity, With<presentation::DialogueUiOptionButton>>();
        query.iter(world).next().expect("dialogue option button")
    };
    app.world_mut()
        .entity_mut(option)
        .insert(Interaction::Pressed);
    app.update();

    assert_eq!(
        app.world().resource::<DialogueRuntime>().phase,
        DialoguePhase::PresentingLine
    );
    assert_eq!(
        app.world()
            .resource::<DialogueRuntime>()
            .presentation
            .line
            .as_ref()
            .map(|line| line.text.as_str()),
        Some("Leave")
    );
}

#[test]
fn number_key_selects_the_same_dialogue_option_as_click() {
    let mut app = app_with_dialogue();
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .set_catalog(catalog());
    app.world_mut()
        .resource_mut::<Messages<DialogueStartRequested>>()
        .write(DialogueStartRequested(DialogueStartRequest {
            dialogue: DialogueKey::new("Guard"),
            speaker: Some(0x10.into()),
            listener: None,
            source: DialogueStartSource::AuthoredNpc,
        }));
    app.update();
    app.update();
    app.world_mut()
        .resource_mut::<Messages<DialogueContinueRequested>>()
        .write(DialogueContinueRequested);
    app.update();
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::Digit1);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
    app.update();

    assert_eq!(
        app.world()
            .resource::<DialogueRuntime>()
            .presentation
            .line
            .as_ref()
            .map(|line| line.text.as_str()),
        Some("Leave")
    );
}

#[test]
fn clicked_spoken_line_advances_without_waiting_for_the_timer() {
    let mut app = app_with_dialogue();
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .set_catalog(catalog());
    app.world_mut()
        .resource_mut::<Messages<DialogueStartRequested>>()
        .write(DialogueStartRequested(DialogueStartRequest {
            dialogue: DialogueKey::new("Guard"),
            speaker: Some(0x10.into()),
            listener: None,
            source: DialogueStartSource::AuthoredNpc,
        }));
    app.update();
    app.update();

    let line = {
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<presentation::DialogueUiLineButton>>();
        query.iter(world).next().expect("dialogue line button")
    };
    app.world_mut()
        .entity_mut(line)
        .insert(Interaction::Pressed);
    app.update();

    assert_eq!(
        app.world().resource::<DialogueRuntime>().phase,
        DialoguePhase::PresentingOptions
    );
}

#[test]
fn unvoiced_line_advances_after_the_deterministic_reading_duration() {
    let mut app = app_with_dialogue();
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .set_catalog(catalog());
    app.world_mut()
        .resource_mut::<Messages<DialogueStartRequested>>()
        .write(DialogueStartRequested(DialogueStartRequest {
            dialogue: DialogueKey::new("Guard"),
            speaker: Some(0x10.into()),
            listener: None,
            source: DialogueStartSource::AuthoredNpc,
        }));
    app.update();
    app.update();
    assert_eq!(
        app.world().resource::<DialogueRuntime>().phase,
        DialoguePhase::PresentingLine
    );

    let duration = app
        .world()
        .resource::<DialogueRuntime>()
        .line_duration_seconds;
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .line_elapsed_seconds = duration;
    app.update();
    app.update();

    assert_eq!(
        app.world().resource::<DialogueRuntime>().phase,
        DialoguePhase::PresentingOptions
    );
}

#[test]
fn voice_timing_waits_for_sink_completion_and_falls_back_after_load_timeout() {
    assert_eq!(
        presentation::voice_timing_action(
            DialogueVoiceTimingState::Playing,
            Some(false),
            0.0,
            20.0,
            0.5,
        ),
        presentation::DialogueVoiceTimingAction::Wait
    );
    assert_eq!(
        presentation::voice_timing_action(
            DialogueVoiceTimingState::Playing,
            Some(true),
            0.0,
            0.1,
            0.5,
        ),
        presentation::DialogueVoiceTimingAction::CompleteAudio
    );
    assert_eq!(
        presentation::voice_timing_action(DialogueVoiceTimingState::Loading, None, 1.0, 0.0, 0.5,),
        presentation::DialogueVoiceTimingAction::EnterTextFallback
    );
    assert_eq!(
        presentation::voice_timing_action(DialogueVoiceTimingState::Fallback, None, 0.0, 0.5, 0.5,),
        presentation::DialogueVoiceTimingAction::CompleteText
    );
}

#[test]
fn prepared_voice_index_loads_only_catalog_keys_and_existing_wavs() {
    let root = std::env::temp_dir().join(format!(
        "bevyout-dialogue-runtime-voice-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("dialogue/audio")).unwrap();
    std::fs::write(root.join("dialogue/audio/guard.wav"), b"synthetic").unwrap();

    let catalog = catalog();
    let key = DialogueLineKey::new("Guard:0");
    let index = PreparedDialogueVoiceIndex {
        revision: DIALOGUE_VOICE_INDEX_REVISION.into(),
        source_manifest_path: "dialogue/voice/guard.ron".into(),
        source_fingerprint: "fingerprint".into(),
        cell_form_id: None,
        entries: vec![
            DialogueVoiceAsset {
                line_key: key.clone(),
                asset_path: "dialogue/audio/guard.wav".into(),
                duration_millis: 500,
                source_path: None,
                source_origin: None,
                source_fingerprint: None,
                staged_fingerprint: None,
                speaker_form_id: None,
                voice_type_form_id: None,
            },
            DialogueVoiceAsset {
                line_key: DialogueLineKey::new("Unknown:0"),
                asset_path: "dialogue/audio/guard.wav".into(),
                duration_millis: 500,
                source_path: None,
                source_origin: None,
                source_fingerprint: None,
                staged_fingerprint: None,
                speaker_form_id: None,
                voice_type_form_id: None,
            },
        ],
        diagnostics: Vec::new(),
    };
    std::fs::write(
        root.join("dialogue/voice_index.ron"),
        ron::ser::to_string(&index).unwrap(),
    )
    .unwrap();

    let mut diagnostics = Vec::new();
    let mut voices = std::collections::BTreeMap::new();
    let loaded = load_prepared_voice_index(
        &root,
        "dialogue/voice_index.ron",
        &catalog,
        &mut diagnostics,
        &mut voices,
    )
    .expect("valid voice index");
    assert_eq!(loaded.entries.len(), 2);
    assert_eq!(
        voices.get(&(key, None)).map(|voice| voice.duration_millis),
        Some(500)
    );
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].code, DialogueErrorCode::MalformedContent);
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn fallout_voice_lookup_is_exact_for_the_active_actor() {
    let key = DialogueLineKey::new("fallout:fallout3.esm:0001d76a:1");
    let mut providers = presentation::DialoguePresentationProviders::default();
    providers.voice.insert(
        (key.clone(), Some(0x0001_ff18)),
        DialogueVoiceAsset {
            line_key: key.clone(),
            asset_path: "audio/merc.ogg".into(),
            speaker_form_id: Some(0x0001_ff18),
            ..Default::default()
        },
    );
    providers.voice.insert(
        (key.clone(), Some(0x0002_d2bc)),
        DialogueVoiceAsset {
            line_key: key.clone(),
            asset_path: "audio/moira.ogg".into(),
            speaker_form_id: Some(0x0002_d2bc),
            ..Default::default()
        },
    );

    assert_eq!(
        providers
            .voice_for(&key, Some(0x0002_d2bc))
            .map(|voice| voice.asset_path.as_str()),
        Some("audio/moira.ogg")
    );
    assert!(providers.voice_for(&key, Some(0xdead_beef)).is_none());
}

fn dialogue_binding(speaker: u32) -> DialogueBinding {
    DialogueBinding {
        dialogue: DialogueKey::new("Guard"),
        speaker,
        listener: None,
    }
}

#[test]
fn voice_anchor_prefers_mouth_then_head_then_actor_root() {
    let mut world = World::new();
    let root = world
        .spawn((Name::new("ActorRoot"), dialogue_binding(0x10)))
        .id();
    let head = world.spawn((Name::new("bIp01 HeAd"), ChildOf(root))).id();
    let mouth = world.spawn((Name::new("mouthhuman"), ChildOf(head))).id();
    let mut queries = SystemState::<(
        Query<(Entity, &DialogueBinding)>,
        Query<&Children>,
        Query<&Name>,
    )>::new(&mut world);
    let (bindings, children, names) = queries.get(&world).unwrap();

    let anchor = presentation::resolve_voice_anchor(Some(0x10), &bindings, &children, &names);
    assert_eq!(anchor.kind, DialogueVoiceAnchorKind::Mouth);
    assert_eq!(anchor.entity, Some(mouth));

    world.despawn(mouth);
    let (bindings, children, names) = queries.get(&world).unwrap();
    let anchor = presentation::resolve_voice_anchor(Some(0x10), &bindings, &children, &names);
    assert_eq!(anchor.kind, DialogueVoiceAnchorKind::Head);
    assert_eq!(anchor.entity, Some(head));

    world.despawn(head);
    let (bindings, children, names) = queries.get(&world).unwrap();
    let anchor = presentation::resolve_voice_anchor(Some(0x10), &bindings, &children, &names);
    assert_eq!(anchor.kind, DialogueVoiceAnchorKind::ActorRoot);
    assert_eq!(anchor.entity, Some(root));
}

#[test]
fn unresolved_voice_speaker_remains_unanchored_and_non_spatial() {
    let mut world = World::new();
    let _ = world
        .spawn((Name::new("ActorRoot"), dialogue_binding(0x10)))
        .id();
    let mut queries = SystemState::<(
        Query<(Entity, &DialogueBinding)>,
        Query<&Children>,
        Query<&Name>,
    )>::new(&mut world);
    let (bindings, children, names) = queries.get(&world).unwrap();
    let anchor = presentation::resolve_voice_anchor(Some(0x20), &bindings, &children, &names);
    assert_eq!(anchor.kind, DialogueVoiceAnchorKind::Unanchored);
    assert_eq!(anchor.entity, None);
    assert!(!anchor.kind.is_spatial());
}

#[test]
fn voice_emitter_uses_identity_local_transform_and_spatial_settings() {
    let mut world = World::new();
    let root = world.spawn_empty().id();
    let voice = world
        .spawn((
            presentation::DialogueVoicePlayer,
            presentation::DialogueVoiceEmitter {
                anchor: DialogueVoiceAnchorKind::Mouth,
                anchor_entity: Some(root),
            },
            presentation::dialogue_voice_playback_settings(true),
            Transform::IDENTITY,
            ChildOf(root),
        ))
        .id();
    assert_eq!(world.get::<Transform>(voice), Some(&Transform::IDENTITY));
    assert_eq!(world.get::<ChildOf>(voice).map(ChildOf::parent), Some(root));
    let playback = world.get::<PlaybackSettings>(voice).unwrap();
    assert!(playback.spatial);
    assert!(matches!(playback.mode, PlaybackMode::Once));
}

#[test]
fn voice_reanchors_existing_player_when_imported_mouth_arrives() {
    let mut app = app_with_dialogue();
    app.world_mut()
        .resource_mut::<DialogueRuntime>()
        .set_catalog(catalog());
    let root = app
        .world_mut()
        .spawn((Name::new("ActorRoot"), dialogue_binding(0x10)))
        .id();
    app.world_mut()
        .resource_mut::<Messages<DialogueStartRequested>>()
        .write(DialogueStartRequested(DialogueStartRequest {
            dialogue: DialogueKey::new("Guard"),
            speaker: Some(0x10.into()),
            listener: None,
            source: DialogueStartSource::AuthoredNpc,
        }));
    app.update();
    app.update();

    let voice = app
        .world_mut()
        .spawn((
            presentation::DialogueVoicePlayer,
            presentation::DialogueVoiceEmitter {
                anchor: DialogueVoiceAnchorKind::ActorRoot,
                anchor_entity: Some(root),
            },
            presentation::dialogue_voice_playback_settings(true),
            Transform::IDENTITY,
            ChildOf(root),
        ))
        .id();
    app.update();
    assert_eq!(
        app.world().get::<ChildOf>(voice).map(ChildOf::parent),
        Some(root)
    );

    let mouth = app
        .world_mut()
        .spawn((Name::new("MouthHuman"), ChildOf(root)))
        .id();
    app.update();

    assert_eq!(
        app.world().get::<ChildOf>(voice).map(ChildOf::parent),
        Some(mouth)
    );
    assert_eq!(
        app.world()
            .get::<presentation::DialogueVoiceEmitter>(voice)
            .unwrap()
            .anchor,
        DialogueVoiceAnchorKind::Mouth
    );
    let mut players = app
        .world_mut()
        .query_filtered::<Entity, With<presentation::DialogueVoicePlayer>>();
    assert_eq!(players.iter(app.world()).count(), 1);
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
