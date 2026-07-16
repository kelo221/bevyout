use super::*;

fn clip(is_2d: bool, attenuation: u16) -> PreparedAudioClip {
    PreparedAudioClip {
        form_id: 1,
        editor_id: None,
        source_path: "sound/test.wav".into(),
        asset_path: Some("audio/test.wav".into()),
        flags: 0,
        min_attenuation: 0,
        max_attenuation: 0,
        frequency_adjustment: 0,
        static_attenuation_hundredths_db: attenuation,
        looping: false,
        is_2d,
    }
}

#[test]
fn static_attenuation_is_applied_as_negative_decibels() {
    let settings = playback_settings(&clip(false, 650), PlaybackMode::Loop, true, false);
    assert!((settings.volume.to_decibels() + 6.5).abs() < f32::EPSILON);
    assert!(settings.spatial);
}

#[test]
fn two_dimensional_and_unpositioned_sounds_are_not_spatial() {
    assert!(!playback_settings(&clip(true, 0), PlaybackMode::Loop, true, false).spatial);
    assert!(!playback_settings(&clip(false, 0), PlaybackMode::Despawn, false, false).spatial);
    assert!(!playback_settings(&clip(false, 0), PlaybackMode::Loop, true, true).spatial);
}

#[test]
fn sound_request_constructors_preserve_playback_space() {
    assert_eq!(PlaySound::at(8, Vec3::X).position, Some(Vec3::X));
    assert_eq!(PlaySound::at(8, Vec3::X).gain_db, 0.0);
}

#[test]
fn pickup_and_container_sounds_are_boosted_by_three_decibels() {
    assert_eq!(PlaySound::pickup_at(8, Vec3::X).gain_db, 3.0);
    assert_eq!(PlaySound::container_at(8, Vec3::X).gain_db, 3.0);
    let settings =
        playback_settings_with_gain(&clip(false, 650), PlaybackMode::Despawn, true, false, 3.0);
    assert!((settings.volume.to_decibels() + 3.5).abs() < f32::EPSILON);
}

#[test]
fn footstep_clip_selection_alternates_banks_and_wraps_variants() {
    let set = PreparedFootstepSet {
        surface: "concrete".into(),
        left: vec!["left-0.wav".into(), "left-1.wav".into()],
        right: vec!["right-0.wav".into(), "right-1.wav".into()],
        land: Vec::new(),
    };
    assert_eq!(
        select_clip_path(footstep_clips(&set, false), 0),
        Some("left-0.wav")
    );
    assert_eq!(
        select_clip_path(footstep_clips(&set, true), 1),
        Some("right-1.wav")
    );
    assert_eq!(
        select_clip_path(footstep_clips(&set, false), 2),
        Some("left-0.wav")
    );
    assert_eq!(select_clip_path(&[], 0), None);
    assert_eq!(select_clip_path(&[String::new()], 0), None);
}
