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
}
