use super::*;

#[test]
fn classifies_laser_and_ten_mm_reload_parts() {
    assert_eq!(
        classify_reload_part("##LPSideLatch:direct"),
        Some(ReloadPart::LaserSideLatch)
    );
    assert_eq!(
        classify_reload_part("##LPSmallEnergyCell"),
        Some(ReloadPart::LaserEnergyCell)
    );
    assert_eq!(
        classify_reload_part("##Slide:direct"),
        Some(ReloadPart::TenMmSlide)
    );
    assert_eq!(classify_reload_part("##Clip"), Some(ReloadPart::TenMmClip));
}

#[test]
fn firing_only_clips_are_excluded() {
    assert_eq!(classify_reload_part("##LPTrigger:direct"), None);
    assert_eq!(classify_reload_part("##Trigger:direct"), None);
    assert_eq!(classify_reload_part("##Hammer:direct"), None);
    assert!(!has_authored_reload_parts(&[
        "##Trigger:direct",
        "##Hammer:direct"
    ]));
}

#[test]
fn reload_phases_are_deterministic() {
    assert_eq!(reload_phase(WeaponAction::Idle, 0.0), ReloadPhase::Idle);
    assert_eq!(
        reload_phase(WeaponAction::Reloading, 0.0),
        ReloadPhase::Opening
    );
    assert_eq!(
        reload_phase(WeaponAction::Reloading, 0.20),
        ReloadPhase::Holding
    );
    assert_eq!(
        reload_phase(WeaponAction::Reloading, 0.55),
        ReloadPhase::Inserting
    );
    assert_eq!(
        reload_phase(WeaponAction::Reloading, 0.80),
        ReloadPhase::Closing
    );
    assert_eq!(
        reload_phase(WeaponAction::Reloading, 1.0),
        ReloadPhase::Closing
    );
}

#[test]
fn missing_authored_clips_leave_procedural_fallback_available() {
    assert!(!has_authored_reload_parts(&[]));
    assert!(!has_authored_reload_parts(&["##LPTrigger:direct"]));
}

#[test]
fn reverse_and_forward_playback_set_direction_and_seek() {
    let mut player = AnimationPlayer::default();
    let clip = WeaponClip {
        node: AnimationNodeIndex::new(3),
        duration: 0.033,
    };
    play_reverse(&mut player, clip);
    let active = player.animation(clip.node).expect("reverse clip active");
    assert!(active.is_playback_reversed());
    assert!((active.seek_time() - clip.duration).abs() < f32::EPSILON);

    play_forward(&mut player, clip);
    let active = player.animation(clip.node).expect("forward clip active");
    assert!(!active.is_playback_reversed());
    assert!(active.seek_time().abs() < f32::EPSILON);
}
