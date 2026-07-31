use super::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn normalizes_sound_paths_and_adds_mp3_fallback() {
    assert_eq!(
        sound_path_candidates(r"\FX\AMB\Room.WAV"),
        vec![
            "sound/fx/amb/room.wav".to_string(),
            "sound/fx/amb/room.mp3".to_string(),
        ]
    );
    assert_eq!(
        sound_path_candidates(r"sound\music\theme.mp3"),
        vec!["sound/music/theme.mp3".to_string()]
    );
    assert!(sound_path_candidates("../outside.wav").is_empty());
}

#[test]
fn sound_path_candidates_keep_directory_paths_as_the_primary_candidate() {
    assert_eq!(
        sound_path_candidates(r"fx\drs\containers\metalbox\open\"),
        vec![
            "sound/fx/drs/containers/metalbox/open".to_string(),
            "sound/fx/drs/containers/metalbox/open.mp3".to_string(),
        ]
    );
}

#[test]
fn loose_directory_audio_uses_a_deterministic_direct_child() {
    let root = std::env::temp_dir().join(format!("bevyout-audio-directory-{}", std::process::id()));
    let directory = root.join("sound/fx/drs/containers/metalbox/open");
    fs::create_dir_all(&directory).unwrap();
    fs::write(directory.join("z_last.wav"), b"last").unwrap();
    fs::write(directory.join("a_first.wav"), b"first").unwrap();

    let resolved = resolve_audio_asset(&root, &[], r"fx\drs\containers\metalbox\open\")
        .unwrap()
        .expect("directory sound should resolve");
    assert_eq!(
        resolved.source_path,
        "sound/fx/drs/containers/metalbox/open/a_first.wav"
    );
    assert_eq!(resolved.bytes, b"first");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn emits_base_dlc_and_plugin_stem_archive_names() {
    assert_eq!(
        audio_archive_candidate_names("Fallout3.esm"),
        vec![
            "Fallout - Sound.bsa",
            "Fallout - Sounds.bsa",
            "Fallout3 - Sound.bsa",
            "Fallout3 - Sounds.bsa",
            "Fallout3.bsa",
        ]
    );
    assert_eq!(
        dialogue_voice_archive_candidate_names("Fallout3.esm"),
        vec![
            "Fallout - Voices.bsa",
            "Fallout - MenuVoices.bsa",
            "Fallout3 - Voices.bsa",
            "Fallout3 - MenuVoices.bsa",
            "Fallout3.bsa",
        ]
    );
    assert_eq!(
        audio_archive_candidate_names("BrokenSteel.esm"),
        vec![
            "BrokenSteel - Sound.bsa",
            "BrokenSteel - Sounds.bsa",
            "BrokenSteel.bsa",
        ]
    );
    assert_eq!(
        dialogue_voice_archive_candidate_names("BrokenSteel.esm"),
        vec![
            "BrokenSteel - Voices.bsa",
            "BrokenSteel - MenuVoices.bsa",
            "BrokenSteel.bsa",
        ]
    );
}

#[test]
fn case_insensitive_dedup_keeps_the_highest_precedence_name() {
    assert_eq!(
        deduplicate_case_insensitive(vec![
            "Selected - Sounds.bsa".into(),
            "selected - sounds.BSA".into(),
            "Master - Sounds.bsa".into(),
        ]),
        vec!["Selected - Sounds.bsa", "Master - Sounds.bsa"]
    );
}

#[test]
fn stages_identical_content_once_and_preserves_extension() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "bevyout-audio-assets-{}-{nonce}",
        std::process::id()
    ));
    let first = ResolvedAudioAsset {
        source_path: "sound/fx/first.wav".into(),
        origin: AudioAssetOrigin::Archive(PathBuf::from("first.bsa")),
        bytes: b"audio bytes".to_vec(),
    };
    let second = ResolvedAudioAsset {
        source_path: "sound/fx/second.wav".into(),
        origin: AudioAssetOrigin::Loose(PathBuf::from("second.wav")),
        bytes: first.bytes.clone(),
    };

    let staged_first = stage_audio_asset(&first, &root).unwrap();
    let staged_second = stage_audio_asset(&second, &root).unwrap();
    assert_eq!(staged_first, staged_second);
    assert_eq!(
        staged_first
            .path
            .extension()
            .and_then(|value| value.to_str()),
        Some("wav")
    );
    assert_eq!(fs::read(&staged_first.path).unwrap(), first.bytes);

    fs::remove_dir_all(root).unwrap();
}
