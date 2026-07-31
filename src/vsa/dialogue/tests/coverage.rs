use super::super::{
    DialogueVoiceRequirementOrigin, PreparedDialogueCatalog, PreparedDialogueVoiceRequirement,
};
use super::*;
use bevyout_core::dialogue::{
    DIALOGUE_VOICE_INDEX_REVISION, DialogueLineKey, DialogueVoiceAsset, PreparedDialogueVoiceIndex,
};

fn requirement(
    key: &str,
    speaker: Option<u32>,
    origin: DialogueVoiceRequirementOrigin,
) -> PreparedDialogueVoiceRequirement {
    PreparedDialogueVoiceRequirement {
        line_key: DialogueLineKey::new(key),
        speaker_form_id: speaker,
        source_path: match origin {
            DialogueVoiceRequirementOrigin::Authored => "authored/moira_brown.yarn".into(),
            DialogueVoiceRequirementOrigin::FalloutDiscovered => {
                "dialogue/generated/actors/0002d2bc.yarn".into()
            }
        },
        origin,
    }
}

#[test]
fn coverage_distinguishes_authored_mappings_from_fallout_failures() {
    let root = std::env::temp_dir().join(format!("bevyout-coverage-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("audio")).unwrap();
    let bytes = b"synthetic";
    std::fs::write(root.join("audio/voice.ogg"), bytes).unwrap();
    let fingerprint = format!("{:x}", Sha256::digest(bytes));
    let catalog = PreparedDialogueCatalog {
        voice_requirements: vec![
            requirement(
                "MoiraBrown:0",
                None,
                DialogueVoiceRequirementOrigin::Authored,
            ),
            requirement(
                "fallout:fallout3.esm:0001d76a:1",
                Some(0x0002_d2bc),
                DialogueVoiceRequirementOrigin::FalloutDiscovered,
            ),
            requirement(
                "fallout:fallout3.esm:0001d76a:2",
                Some(0x0002_d2bc),
                DialogueVoiceRequirementOrigin::FalloutDiscovered,
            ),
        ],
        source_paths: vec!["authored/moira_brown.yarn".into()],
        authored_voice_manifest_paths: vec!["dialogue/voice/moira_brown.ron".into()],
        ..Default::default()
    };
    let index = PreparedDialogueVoiceIndex {
        revision: DIALOGUE_VOICE_INDEX_REVISION.into(),
        entries: vec![DialogueVoiceAsset {
            line_key: DialogueLineKey::new("fallout:fallout3.esm:0001d76a:1"),
            asset_path: "audio/voice.ogg".into(),
            staged_fingerprint: Some(fingerprint),
            speaker_form_id: Some(0x0002_d2bc),
            ..Default::default()
        }],
        ..Default::default()
    };

    let coverage = assess_voice_coverage(&root, &catalog, Some(&index));
    assert_eq!(coverage.total_lines, 3);
    assert_eq!(coverage.mapped_lines, 1);
    assert_eq!(coverage.missing_authored[0].label(), "MoiraBrown:0");
    assert_eq!(
        coverage.missing_fallout[0].label(),
        "fallout:fallout3.esm:0001d76a:2@speaker=0002d2bc"
    );
    assert_eq!(
        voice_repair_guidance("MegatonCratersideSupply", &catalog, &coverage),
        "next command: cargo run-dev -- prepare MegatonCratersideSupply --dialogue-source dialogue/authored/moira_brown.yarn --dialogue-voice-manifest dialogue/voice/moira_brown.ron"
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn authored_voice_without_a_recorded_manifest_reports_the_mapping_blocker() {
    let catalog = PreparedDialogueCatalog {
        source_paths: vec!["authored/moira_brown.yarn".into()],
        voice_requirements: vec![requirement(
            "MoiraBrown:0",
            None,
            DialogueVoiceRequirementOrigin::Authored,
        )],
        ..Default::default()
    };
    let coverage = assess_voice_coverage(Path::new("."), &catalog, None);

    assert_eq!(
        voice_repair_guidance("MegatonCratersideSupply", &catalog, &coverage),
        "blocker: exact authored voice mapping manifest missing for sources=[dialogue/authored/moira_brown.yarn]; create the mapping contract, then rerun cargo run-dev -- prepare MegatonCratersideSupply"
    );
}

#[test]
fn exact_speaker_mapping_does_not_accept_another_actors_clip() {
    let catalog = PreparedDialogueCatalog {
        voice_requirements: vec![requirement(
            "fallout:fallout3.esm:0001d76a:1",
            Some(0x0002_d2bc),
            DialogueVoiceRequirementOrigin::FalloutDiscovered,
        )],
        ..Default::default()
    };
    let index = PreparedDialogueVoiceIndex {
        entries: vec![DialogueVoiceAsset {
            line_key: DialogueLineKey::new("fallout:fallout3.esm:0001d76a:1"),
            speaker_form_id: Some(0x0001_ff18),
            ..Default::default()
        }],
        ..Default::default()
    };
    let coverage = assess_voice_coverage(Path::new("."), &catalog, Some(&index));
    assert_eq!(coverage.mapped_lines, 0);
    assert_eq!(coverage.missing_fallout.len(), 1);
}
