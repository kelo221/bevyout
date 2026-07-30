use super::*;

fn fast_config() -> PerceptionConfig {
    // Deterministic, single-step acquisition for unit tests.
    PerceptionConfig {
        sight_range: 40.0,
        view_cone_half_angle: std::f32::consts::FRAC_PI_2,
        acquire_confidence: 0.5,
        lose_confidence: 0.1,
        gain_per_second: 1.0,
        decay_per_second: 1.0,
        forget_seconds: 2.0,
    }
}

fn visible_player(distance: f32) -> PerceptionInputs {
    PerceptionInputs {
        target: TargetId::player(),
        position: [distance, 0.0, 0.0],
        distance,
        angle_to_target: 0.0,
        has_line_of_sight: true,
        detectable: true,
    }
}

#[test]
fn sustained_visibility_acquires_after_hysteresis_threshold() {
    let config = fast_config();
    let mut state = AwarenessState::default();
    // gain 1.0/s, acquire at 0.5: one 0.4s tick is not enough, two are.
    assert_eq!(
        state.update(&[visible_player(5.0)], &config, 0.4),
        AwarenessEvent::Idle
    );
    assert!(!state.is_aware());
    assert_eq!(
        state.update(&[visible_player(5.0)], &config, 0.4),
        AwarenessEvent::Acquired(TargetId::player())
    );
    assert!(state.is_aware());
    assert_eq!(state.target(), Some(TargetId::player()));
}

#[test]
fn out_of_cone_or_out_of_range_target_is_not_visible() {
    let config = fast_config();
    let behind = PerceptionInputs {
        angle_to_target: std::f32::consts::PI,
        ..visible_player(5.0)
    };
    let far = visible_player(1000.0);
    assert!(!behind.is_visible(&config));
    assert!(!far.is_visible(&config));
}

#[test]
fn brief_occlusion_does_not_immediately_lose_the_target() {
    let config = fast_config();
    let mut state = AwarenessState {
        confidence: 1.0,
        acquired: Some(TargetId::player()),
        ..Default::default()
    };
    let occluded = PerceptionInputs {
        has_line_of_sight: false,
        ..visible_player(5.0)
    };
    // One short occluded tick: still retained (confidence 1.0 -> 0.5, timer 0.5s).
    assert_eq!(
        state.update(&[occluded], &config, 0.5),
        AwarenessEvent::Retained(TargetId::player())
    );
    assert!(state.is_aware());
}

#[test]
fn prolonged_occlusion_forgets_the_target_via_the_timer() {
    let mut config = fast_config();
    config.forget_seconds = 1.0;
    config.decay_per_second = 0.0; // isolate the forget timer
    let mut state = AwarenessState {
        confidence: 1.0,
        acquired: Some(TargetId::player()),
        ..Default::default()
    };
    let occluded = PerceptionInputs {
        has_line_of_sight: false,
        ..visible_player(5.0)
    };
    assert_eq!(
        state.update(&[occluded], &config, 1.0),
        AwarenessEvent::Lost(TargetId::player())
    );
    assert!(!state.is_aware());
}

#[test]
fn dead_or_vanished_target_is_lost_immediately() {
    let config = fast_config();
    let mut state = AwarenessState {
        confidence: 1.0,
        acquired: Some(TargetId::player()),
        ..Default::default()
    };
    // Vanished entirely from the candidate set (e.g. cell unload).
    assert_eq!(
        state.update(&[], &config, 0.1),
        AwarenessEvent::Lost(TargetId::player())
    );
    assert!(!state.is_aware());
}

#[test]
fn undetectable_acquired_target_is_lost_immediately() {
    let config = fast_config();
    let mut state = AwarenessState {
        confidence: 1.0,
        acquired: Some(TargetId::player()),
        ..Default::default()
    };
    let dead = PerceptionInputs {
        detectable: false,
        ..visible_player(5.0)
    };
    assert_eq!(
        state.update(&[dead], &config, 0.1),
        AwarenessEvent::Lost(TargetId::player())
    );
}

#[test]
fn best_visible_prefers_nearest_then_player_then_form_id() {
    let config = fast_config();
    let near_actor = PerceptionInputs {
        target: TargetId {
            class: TargetClass::Actor,
            form_id: 0x50,
        },
        ..visible_player(3.0)
    };
    let far_player = visible_player(10.0);
    let nearest = [far_player, near_actor];
    let chosen = select_best_visible(&nearest, &config).unwrap();
    assert_eq!(chosen.target, near_actor.target);

    // Equidistant: the player wins on class priority.
    let equal_player = visible_player(3.0);
    let equal_actor = PerceptionInputs {
        target: TargetId {
            class: TargetClass::Actor,
            form_id: 0x1,
        },
        ..visible_player(3.0)
    };
    let equal = [equal_actor, equal_player];
    let chosen = select_best_visible(&equal, &config).unwrap();
    assert_eq!(chosen.target.class, TargetClass::Player);
}

#[test]
fn acquired_target_is_kept_over_a_closer_newcomer_until_lost() {
    let config = fast_config();
    let mut state = AwarenessState {
        confidence: 1.0,
        acquired: Some(TargetId::player()),
        ..Default::default()
    };
    let closer_actor = PerceptionInputs {
        target: TargetId {
            class: TargetClass::Actor,
            form_id: 0x50,
        },
        ..visible_player(1.0)
    };
    let player = visible_player(5.0);
    assert_eq!(
        state.update(&[closer_actor, player], &config, 0.1),
        AwarenessEvent::Retained(TargetId::player())
    );
}

#[test]
fn clear_resets_all_awareness() {
    let mut state = AwarenessState {
        confidence: 1.0,
        acquired: Some(TargetId::player()),
        time_since_seen: 3.0,
        last_known_position: Some([1.0, 2.0, 3.0]),
    };
    state.clear();
    assert_eq!(state, AwarenessState::default());
    assert!(!state.is_aware());
}

#[test]
fn awareness_state_round_trips_through_serde_for_save_load() {
    let state = AwarenessState {
        confidence: 0.6,
        acquired: Some(TargetId {
            class: TargetClass::Actor,
            form_id: 0x1234,
        }),
        time_since_seen: 1.5,
        last_known_position: Some([4.0, 5.0, 6.0]),
    };
    let text = ron::ser::to_string(&state).unwrap();
    let round: AwarenessState = ron::de::from_str(&text).unwrap();
    assert_eq!(state, round);
}
