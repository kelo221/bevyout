use super::*;

fn wav_bytes(sample_rate: u32, samples: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    {
        let cursor = std::io::Cursor::new(&mut bytes);
        let mut writer = hound::WavWriter::new(
            cursor,
            hound::WavSpec {
                channels: 1,
                sample_rate,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            },
        )
        .unwrap();
        for _ in 0..samples {
            writer.write_sample(0i16).unwrap();
        }
        writer.finalize().unwrap();
    }
    bytes
}

fn authored(path: &str, content: &str) -> DialogueSource {
    DialogueSource {
        relative_path: path.into(),
        kind: DialogueSourceKind::Authored,
        content: content.into(),
    }
}

#[test]
fn preparation_sorts_sources_and_fingerprints_content() {
    let a = authored("authored/z.yarn", "title: Z\n---\nZed: hello\n===\n");
    let b = authored("authored/a.yarn", "title: A\n---\nAda: hi\n===\n");
    let left = prepare_catalog(vec![a.clone(), b.clone()]);
    let right = prepare_catalog(vec![b, a]);
    assert_eq!(left, right);
    assert_eq!(
        left.conversations
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["A", "Z"]
    );
    assert!(left.is_ready());
}

#[test]
fn malformed_and_duplicate_nodes_are_diagnostics() {
    let duplicate = authored(
        "dialogue.yarn",
        "title: Start\n---\nhello\n===\ntitle: Start\n---\nagain\n===\n",
    );
    let malformed = authored("broken.yarn", "title: Broken\nno separator\n");
    let catalog = prepare_catalog(vec![duplicate, malformed]);
    assert!(
        catalog
            .diagnostics
            .iter()
            .any(|d| d.code == "duplicate_node")
    );
    assert!(
        catalog
            .diagnostics
            .iter()
            .any(|d| d.code == "malformed_node")
    );
    assert!(!catalog.is_ready());
}

#[test]
fn nodes_in_one_source_share_a_conversation_and_preserve_destinations() {
    let catalog = prepare_catalog(vec![authored(
        "authored/guard.yarn",
        "title: Start\n---\nGuard: hello\n-> Continue -> Checkpoint\n===\ntitle: Checkpoint\n---\nGuard: done\n===\n",
    )]);
    let conversation = catalog.conversation(&DialogueKey::new("Start")).unwrap();
    assert!(conversation.nodes.contains_key("Checkpoint"));
    assert_eq!(
        conversation.nodes["Start"].options[0]
            .destination
            .as_deref(),
        Some("Checkpoint")
    );
}

#[test]
fn bundle_writes_explicit_authored_and_generated_sources() {
    let root = std::env::temp_dir().join(format!("bevyout-dialogue-unit-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let output = prepare_dialogue_bundle(
        &root,
        vec![
            authored("authored/guard.yarn", "title: Start\n---\nhello\n===\n"),
            DialogueSource {
                relative_path: "generated/guard.yarn".into(),
                kind: DialogueSourceKind::ImportedGenerated,
                content: "title: Start\n---\nimported\n===\n".into(),
            },
        ],
    )
    .unwrap();
    let bundle = output.bundle.unwrap();
    assert_eq!(bundle.catalog_path, "dialogue/catalog.ron");
    assert!(root.join("dialogue/authored/guard.yarn").is_file());
    assert!(root.join("dialogue/generated/guard.yarn").is_file());
    assert!(root.join("dialogue/node_index.ron").is_file());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn voice_manifest_is_versioned_and_wav_duration_is_prepared() {
    let manifest = parse_voice_manifest(
        "(entries: [(line_key: \"Start:0\", source_path: \"dialogue/voice/hello.wav\")])",
    )
    .unwrap();
    assert_eq!(manifest.revision, "dialogue-voice-source-v1");

    let root = std::env::temp_dir().join(format!("bevyout-dialogue-voice-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let output = prepare_dialogue_bundle_with_voice(
        &root,
        vec![authored(
            "authored/guard.yarn",
            "title: Start\n---\nGuard: hello\n===\n",
        )],
        Some(DialogueVoiceInput {
            manifest_path: "dialogue/voice.ron".into(),
            entries: vec![DialogueVoiceInputEntry {
                line_key: DialogueLineKey::new("Start:0"),
                source_path: "dialogue/voice/hello.wav".into(),
                bytes: wav_bytes(16_000, 8_000),
            }],
        }),
    )
    .unwrap();
    let bundle = output.bundle.unwrap();
    assert_eq!(bundle.revision, DIALOGUE_BUNDLE_REVISION);
    let voice_path = root.join("dialogue/voice_index.ron");
    let index: PreparedDialogueVoiceIndex =
        ron::de::from_bytes(&std::fs::read(voice_path).unwrap()).unwrap();
    assert_eq!(
        index.revision,
        bevyout_core::dialogue::DIALOGUE_VOICE_INDEX_REVISION
    );
    assert_eq!(index.entries.len(), 1);
    assert_eq!(index.entries[0].duration_millis, 500);
    assert!(root.join(&index.entries[0].asset_path).is_file());
    assert_eq!(
        bundle.content_fingerprint,
        dialogue_bundle_fingerprint(&output.catalog, Some(&index))
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn voice_preparation_rejects_duplicate_unknown_and_unsupported_entries() {
    let source = authored(
        "authored/guard.yarn",
        "title: Start\n---\nGuard: hello\n===\n",
    );
    let duplicate = DialogueVoiceInput {
        manifest_path: "voice.ron".into(),
        entries: vec![
            DialogueVoiceInputEntry {
                line_key: DialogueLineKey::new("Start:0"),
                source_path: "one.wav".into(),
                bytes: wav_bytes(8_000, 8),
            },
            DialogueVoiceInputEntry {
                line_key: DialogueLineKey::new("Start:0"),
                source_path: "two.wav".into(),
                bytes: wav_bytes(8_000, 8),
            },
        ],
    };
    let root = std::env::temp_dir().join(format!(
        "bevyout-dialogue-voice-invalid-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    let error = prepare_dialogue_bundle_with_voice(&root, vec![source.clone()], Some(duplicate))
        .unwrap_err()
        .to_string();
    assert!(error.contains("listed more than once"));

    let error = prepare_dialogue_bundle_with_voice(
        &root,
        vec![source.clone()],
        Some(DialogueVoiceInput {
            manifest_path: "voice.ron".into(),
            entries: vec![DialogueVoiceInputEntry {
                line_key: DialogueLineKey::new("Missing:0"),
                source_path: "missing.wav".into(),
                bytes: wav_bytes(8_000, 8),
            }],
        }),
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("not present in the catalog"));

    let error = prepare_dialogue_bundle_with_voice(
        &root,
        vec![source],
        Some(DialogueVoiceInput {
            manifest_path: "voice.ron".into(),
            entries: vec![DialogueVoiceInputEntry {
                line_key: DialogueLineKey::new("Start:0"),
                source_path: "hello.mp3".into(),
                bytes: Vec::new(),
            }],
        }),
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("supported WAV"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn fallout_report_keeps_unsupported_records_visible() {
    let records = vec![
        FalloutDialogueRecord {
            plugin: "Fallout3.esm".into(),
            form_id: 1,
            signature: "DIAL".into(),
            topic_key: "topic".into(),
            ..Default::default()
        },
        FalloutDialogueRecord {
            plugin: "mod.esp".into(),
            form_id: 2,
            signature: "SCEN".into(),
            topic_key: "topic".into(),
            ..Default::default()
        },
    ];
    let report = inventory_fallout_dialogue(&records);
    assert_eq!(report.dial_total, 1);
    assert_eq!(report.unsupported.len(), 1);
}

#[test]
fn fallout_report_detects_topic_cycles_deterministically() {
    let records = vec![
        FalloutDialogueRecord {
            signature: "DIAL".into(),
            topic_key: "a".into(),
            links: vec!["b".into()],
            ..Default::default()
        },
        FalloutDialogueRecord {
            signature: "DIAL".into(),
            topic_key: "b".into(),
            links: vec!["a".into()],
            ..Default::default()
        },
    ];
    let report = inventory_fallout_dialogue(&records);
    assert_eq!(report.topic_total, 2);
    assert_eq!(
        report.cycles,
        vec![vec![String::from("a"), String::from("b")]]
    );
}

#[test]
fn generated_fallout_output_is_byte_stable() {
    let records = vec![FalloutDialogueRecord {
        plugin: "Fallout3.esm".into(),
        form_id: 2,
        signature: "INFO".into(),
        topic_key: "topic".into(),
        text: Some("response".into()),
        ..Default::default()
    }];
    let (first, _) = generate_fallout_conversation("topic", &records).unwrap();
    let (second, _) = generate_fallout_conversation("topic", &records).unwrap();
    assert_eq!(first, second);
    assert_eq!(
        generated_source_fingerprint(&first),
        generated_source_fingerprint(&second)
    );
}
