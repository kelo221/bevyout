use super::*;

fn clip(name: &str, source: &str, sequence: &str) -> PreparedActorAnimationClip {
    PreparedActorAnimationClip {
        name: name.into(),
        source_kf_path: source.into(),
        source_sequence_name: Some(sequence.into()),
        status: PreparedActorAnimationClipStatus::Ready,
        loop_mode: PreparedActorAnimationLoopMode::Loop,
        root_motion_policy: PreparedActorAnimationRootMotionPolicy::PreserveAuthored,
        ..Default::default()
    }
}

#[test]
fn canonical_adult_path_beats_the_first_colliding_glb_name() {
    let clips = [
        clip(
            "mtforward",
            "meshes/characters/_male/locomotion/child/mtforward.kf",
            "Forward",
        ),
        clip(
            "mtforward__3",
            "meshes/characters/_male/locomotion/male/mtforward.kf",
            "Forward",
        ),
    ];
    let selected = resolve_clip(
        &clips,
        ActorAnimationContext {
            kind: PreparedActorAnimationKind::Npc,
            female: false,
            weapon_prefix: None,
        },
        ActorAnimationState::Walk,
    )
    .unwrap();
    assert_eq!(selected.clip_name, "mtforward__3");
}

#[test]
fn weapon_state_requires_the_authored_prefix() {
    let clips = [
        clip("1hmequip", "1hmequip.kf", "Equip"),
        clip("1hpequip", "1hpequip.kf", "Equip"),
        clip("mtidle", "locomotion/mtidle.kf", "Idle"),
    ];
    let selected = resolve_clip(
        &clips,
        ActorAnimationContext {
            kind: PreparedActorAnimationKind::Npc,
            female: false,
            weapon_prefix: Some("1hp"),
        },
        ActorAnimationState::Equip,
    )
    .unwrap();
    assert_eq!(selected.clip_name, "1hpequip");
}

#[test]
fn missing_requested_state_falls_back_to_idle() {
    let clips = [clip(
        "mtidle",
        "meshes/characters/_male/locomotion/mtidle.kf",
        "Idle",
    )];
    let selected = resolve_clip(
        &clips,
        ActorAnimationContext {
            kind: PreparedActorAnimationKind::Npc,
            female: false,
            weapon_prefix: None,
        },
        ActorAnimationState::Run,
    )
    .unwrap();
    assert_eq!(selected.state, ActorAnimationState::Idle);
    assert_eq!(selected.fallback_from, Some(ActorAnimationState::Run));
}

#[test]
fn completed_one_shots_and_turns_return_to_idle() {
    assert_eq!(
        state_after_completion(ActorAnimationState::Equip),
        ActorAnimationState::Idle
    );
    assert_eq!(
        state_after_completion(ActorAnimationState::TurnLeft),
        ActorAnimationState::Idle
    );
    assert_eq!(
        state_after_completion(ActorAnimationState::Run),
        ActorAnimationState::Run
    );
}
