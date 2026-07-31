use super::*;
use crate::vsa::openmw_esm4::{ActorData, BaseRecord, ParsedPlugin, ReferenceRecord};

fn wav_bytes(sample_rate: u32, samples: usize) -> Vec<u8> {
    let mut output = Cursor::new(Vec::new());
    let mut writer = hound::WavWriter::new(
        &mut output,
        hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        },
    )
    .unwrap();
    for _ in 0..samples {
        writer.write_sample(0_i16).unwrap();
    }
    writer.finalize().unwrap();
    output.into_inner()
}

#[test]
fn wav_voice_assets_are_preserved_without_transcoding() {
    let bytes = wav_bytes(16_000, 8_000);
    let asset = ResolvedAudioAsset {
        source_path: "sound/voice/test.wav".into(),
        origin: crate::vsa::audio_assets::AudioAssetOrigin::Loose("test.wav".into()),
        bytes: bytes.clone(),
    };
    assert_eq!(validate_voice_asset(&asset).unwrap(), bytes);
    assert_eq!(
        crate::vsa::dialogue::dialogue_voice_duration_millis(&asset.source_path, &bytes).unwrap(),
        500
    );
}

#[test]
fn actor_demands_only_include_enabled_voice_capable_present_actors() {
    let mut parsed = ParsedPlugin::default();
    let mut base = BaseRecord::default();
    base.actor = Some(ActorData {
        voice_form_id: Some(10),
        ..Default::default()
    });
    parsed.bases.insert(1, base);
    parsed.references.push(ReferenceRecord {
        kind: ReferenceKind::Npc,
        form_id: 20,
        base_form_id: 1,
        initially_enabled: true,
        ..Default::default()
    });
    parsed.references.push(ReferenceRecord {
        kind: ReferenceKind::Npc,
        form_id: 21,
        base_form_id: 1,
        initially_enabled: false,
        ..Default::default()
    });
    let mut diagnostics = Vec::new();
    let demands = collect_actor_demands(
        &parsed,
        &HashMap::from([(10_u32, "FemaleAdult01".into())]),
        &mut diagnostics,
    );
    assert_eq!(demands.len(), 1);
    assert_eq!(demands[0].actor_reference_form_id, 20);
    assert!(diagnostics.is_empty());
}

#[test]
fn discovery_fingerprint_changes_with_cell_or_plugin_bytes() {
    let sources = [PluginSource {
        name: "Fallout3.esm",
        bytes: b"plugin",
    }];
    assert_eq!(
        discovery_fingerprint(1, &sources),
        discovery_fingerprint(1, &sources)
    );
    assert_ne!(
        discovery_fingerprint(1, &sources),
        discovery_fingerprint(2, &sources)
    );
}

#[test]
fn voice_paths_match_only_the_exact_voice_type_component() {
    assert!(voice_path_matches_type(
        "Sound/Voice/Fallout3.esm/FemaleUniqueMoira/line.ogg",
        "femaleuniquemoira"
    ));
    assert!(!voice_path_matches_type(
        "sound/voice/fallout3.esm/femaleunique/line.ogg",
        "FemaleUniqueMoira"
    ));
}

#[test]
fn unresolved_speakerless_info_is_aggregated_as_one_information_diagnostic() {
    assert!(speakerless_skip_diagnostic(0, 2).is_none());
    let diagnostic = speakerless_skip_diagnostic(25_119, 2).unwrap();
    assert_eq!(diagnostic.severity, "info");
    assert_eq!(diagnostic.code, "speakerless_info_skipped");
    assert!(diagnostic.line_key.is_none());
    assert!(diagnostic.message.contains("25119"));
    assert!(diagnostic.message.contains("2 present actor voice types"));
}

#[test]
fn malformed_ogg_is_rejected_without_transcoding() {
    let asset = ResolvedAudioAsset {
        source_path: "sound/voice/fallout3.esm/femaleuniquemoira/line.ogg".into(),
        origin: AudioAssetOrigin::Loose("line.ogg".into()),
        bytes: b"not an ogg stream".to_vec(),
    };
    let error = validate_voice_asset(&asset).unwrap_err().to_string();
    assert!(error.contains("reading dialogue OGG"));
}

fn subrecord(signature: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut bytes = signature.to_vec();
    bytes.extend_from_slice(&(data.len() as u16).to_le_bytes());
    bytes.extend_from_slice(data);
    bytes
}

fn record(signature: &[u8; 4], form_id: u32, payload: &[u8]) -> Vec<u8> {
    record_with_flags(signature, 0, form_id, payload)
}

fn record_with_flags(signature: &[u8; 4], flags: u32, form_id: u32, payload: &[u8]) -> Vec<u8> {
    let mut bytes = signature.to_vec();
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&flags.to_le_bytes());
    bytes.extend_from_slice(&form_id.to_le_bytes());
    bytes.extend_from_slice(&[0; 8]);
    bytes.extend_from_slice(payload);
    bytes
}

fn grup(label: u32, group_type: i32, children: &[u8]) -> Vec<u8> {
    let mut bytes = b"GRUP".to_vec();
    bytes.extend_from_slice(&((children.len() + 24) as u32).to_le_bytes());
    bytes.extend_from_slice(&label.to_le_bytes());
    bytes.extend_from_slice(&group_type.to_le_bytes());
    bytes.extend_from_slice(&[0; 8]);
    bytes.extend_from_slice(children);
    bytes
}

#[test]
fn info_without_topic_link_inherits_the_topic_children_group() {
    let dial_payload = subrecord(b"EDID", b"SyntheticTopic\0");
    let mut trdt = vec![0_u8; 20];
    trdt[12] = 1;
    let info_payload = [
        subrecord(b"DATA", &[0, 1]),
        subrecord(b"QSTI", &0x30_u32.to_le_bytes()),
        subrecord(b"TRDT", &trdt),
        subrecord(b"NAM1", b"Hello\0"),
    ]
    .concat();
    let children = [
        record(b"DIAL", 0x10, &dial_payload),
        record(b"INFO", 0x20, &info_payload),
    ]
    .concat();
    let plugin = [record(b"TES4", 0, &[]), grup(0x10, 7, &children)].concat();
    let sources = [PluginSource {
        name: "Fallout3.esm",
        bytes: &plugin,
    }];

    let records = collect_dialogue_records(&sources).unwrap();
    assert_eq!(
        records.infos.get(&0x20).and_then(|info| info.topic_form_id),
        Some(0x10)
    );
}

#[test]
fn load_order_replacements_and_deletions_are_reported() {
    let dial = record(b"DIAL", 0x10, &subrecord(b"EDID", b"Topic\0"));
    let dial_override = record(b"DIAL", 0x10, &subrecord(b"EDID", b"TopicOverride\0"));
    let info = record(b"INFO", 0x20, &[]);
    let info_delete = record_with_flags(b"INFO", 0x20, 0x20, &[]);
    let children = [dial, dial_override, info, info_delete].concat();
    let plugin = [record(b"TES4", 0, &[]), grup(0x10, 7, &children)].concat();
    let sources = [PluginSource {
        name: "Fallout3.esm",
        bytes: &plugin,
    }];

    let records = collect_dialogue_records(&sources).unwrap();
    assert!(
        records
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "overridden_record")
    );
    assert!(
        records
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "deleted_record")
    );
    assert!(!records.infos.contains_key(&0x20));
}

#[test]
fn generated_actor_conversation_keeps_fallout_keys_and_real_linked_options() {
    let demand = VoiceDemand {
        actor_reference_form_id: 0x0002_d2bc,
        actor_base_form_id: 0x0002_d3c0,
        actor_editor_id: Some("MoiraBrown".into()),
        actor_display_name: Some("Moira Brown".into()),
        voice_type_form_id: Some(0x0006_1e91),
        voice_type_editor_id: Some("FemaleUniqueMoira".into()),
    };
    let greeting = FalloutDialogueRecord {
        plugin: "Fallout3.esm".into(),
        form_id: 0x0001_d76a,
        signature: "INFO".into(),
        text: Some("Exact greeting".into()),
        topic_key: "GREETING".into(),
        voice_key: Some("fallout:fallout3.esm:0001d76a:1".into()),
        ..Default::default()
    };
    let answer = FalloutDialogueRecord {
        plugin: "Fallout3.esm".into(),
        form_id: 0x0002_d2b5,
        signature: "INFO".into(),
        text: Some("Exact answer".into()),
        topic_key: "MS03MoiraCurrentWork".into(),
        link_from: vec!["GREETING".into()],
        voice_key: Some("fallout:fallout3.esm:0002d2b5:1".into()),
        ..Default::default()
    };
    let internal = FalloutDialogueRecord {
        plugin: "Fallout3.esm".into(),
        form_id: 0x0002_d2b6,
        signature: "INFO".into(),
        text: Some("Internal exact answer".into()),
        topic_key: "InternalState".into(),
        voice_key: Some("fallout:fallout3.esm:0002d2b6:1".into()),
        ..Default::default()
    };
    let topics = BTreeMap::from([
        ("GREETING".into(), vec![greeting]),
        ("MS03MoiraCurrentWork".into(), vec![answer]),
        ("InternalState".into(), vec![internal]),
    ]);
    let dials = HashMap::from([
        (
            0x10,
            DialRecord {
                editor_id: Some("MS03MoiraCurrentWork".into()),
                display_text: Some("What are you working on?".into()),
                quest_form_id: None,
                top_level: true,
            },
        ),
        (
            0x11,
            DialRecord {
                editor_id: Some("InternalState".into()),
                display_text: Some("InternalState".into()),
                quest_form_id: None,
                top_level: true,
            },
        ),
    ]);

    let (source, binding) = generate_actor_conversation(&demand, topics, &dials).unwrap();
    assert_eq!(binding.dialogue.as_str(), "fallout_actor_0002d2bc");
    assert_eq!(binding.actor_display_name.as_deref(), Some("Moira Brown"));
    assert!(source.content.contains(
        "// bo_line_key: fallout:fallout3.esm:0001d76a:1\nSpeaker0002d2bc: Exact greeting"
    ));
    assert!(
        source
            .content
            .contains("-> What are you working on? -> fallout_actor_0002d2bc_MS03MoiraCurrentWork")
    );
    assert!(source.content.contains(
        "// bo_line_key: fallout:fallout3.esm:0002d2b5:1\nSpeaker0002d2bc: Exact answer"
    ));
    assert!(!source.content.contains("-> InternalState ->"));
}
