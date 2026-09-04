use super::*;
use crate::perception::{AwarenessState, TargetClass, TargetId};

fn observer() -> TargetId {
    TargetId {
        class: TargetClass::Actor,
        form_id: 0x10,
    }
}

fn evidence(
    light: u16,
    movement: u16,
    armor: u16,
    perception: u16,
    los: bool,
    distance_mm: u32,
    angle: u32,
    delta_ms: u32,
) -> DetectionEvidence {
    DetectionEvidence {
        observer: observer(),
        subject: TargetId::player(),
        distance_mm,
        angle_millidegrees: angle,
        light_bps: light,
        movement_noise_bps: movement,
        armor_noise_bps: armor,
        observer_perception: perception,
        has_line_of_sight: los,
        delta_ms,
    }
}

#[test]
fn each_maxed_factor_acquires_after_800_ms() {
    let config = DetectionConfig::golden();
    for term in [
        evidence(10_000, 0, 0, 0, true, 5_000, 0, 400),
        evidence(0, 10_000, 0, 0, true, 5_000, 0, 400),
        evidence(0, 0, 10_000, 0, true, 5_000, 0, 400),
        evidence(0, 0, 0, 10, true, 5_000, 0, 400),
    ] {
        let mut state = AwarenessState::default();
        assert_eq!(
            update_from_evidence(&mut state, &[term], &config),
            crate::perception::AwarenessEvent::Idle
        );
        let second = DetectionEvidence {
            delta_ms: 400,
            ..term
        };
        assert_eq!(
            update_from_evidence(&mut state, &[second], &config),
            crate::perception::AwarenessEvent::Acquired(TargetId::player())
        );
    }
}

#[test]
fn darkness_never_acquires() {
    let config = DetectionConfig::golden();
    let mut state = AwarenessState::default();
    let dark = evidence(0, 0, 0, 0, true, 5_000, 0, 400);
    update_from_evidence(&mut state, &[dark], &config);
    update_from_evidence(&mut state, &[dark], &config);
    assert!(!state.is_aware());
}

#[test]
fn occluded_or_out_of_cone_or_range_never_acquires() {
    let config = DetectionConfig::golden();
    for term in [
        evidence(10_000, 0, 0, 0, false, 5_000, 0, 400),
        evidence(10_000, 0, 0, 0, true, 5_000, 180_000, 400),
        evidence(10_000, 0, 0, 0, true, 1_000_000, 0, 400),
    ] {
        let mut state = AwarenessState::default();
        update_from_evidence(&mut state, &[term], &config);
        update_from_evidence(&mut state, &[term], &config);
        assert!(!state.is_aware());
    }
}

#[test]
fn hysteresis_does_not_flicker_around_acquire() {
    let config = DetectionConfig::golden();
    let mut state = AwarenessState {
        confidence_milli: 400,
        confidence: 0.4,
        ..Default::default()
    };
    let lit = evidence(10_000, 0, 0, 0, true, 5_000, 0, 100);
    assert_eq!(
        update_from_evidence(&mut state, &[lit], &config),
        crate::perception::AwarenessEvent::Idle
    );
    assert_eq!(state.confidence_milli, 500);
    let dark = evidence(0, 0, 0, 0, true, 5_000, 0, 100);
    assert_eq!(
        update_from_evidence(&mut state, &[dark], &config),
        crate::perception::AwarenessEvent::Idle
    );
    assert_eq!(state.confidence_milli, 400);
}

#[test]
fn equal_distance_picks_lowest_form_id_regardless_of_input_order() {
    let config = DetectionConfig::golden();
    let a = DetectionEvidence {
        subject: TargetId {
            class: TargetClass::Actor,
            form_id: 0xA,
        },
        ..evidence(10_000, 0, 0, 0, true, 5_000, 0, 400)
    };
    let b = DetectionEvidence {
        subject: TargetId {
            class: TargetClass::Actor,
            form_id: 0x5,
        },
        ..evidence(10_000, 0, 0, 0, true, 5_000, 0, 400)
    };
    for order in [[a, b], [b, a]] {
        let mut state = AwarenessState::default();
        update_from_evidence(&mut state, &order, &config);
        update_from_evidence(&mut state, &order, &config);
        assert_eq!(
            state.target(),
            Some(TargetId {
                class: TargetClass::Actor,
                form_id: 0x5
            })
        );
    }
}

#[test]
fn player_wins_equidistant_class_tie() {
    let config = DetectionConfig::golden();
    let actor = DetectionEvidence {
        subject: TargetId {
            class: TargetClass::Actor,
            form_id: 0x50,
        },
        ..evidence(10_000, 0, 0, 0, true, 5_000, 0, 400)
    };
    let player = evidence(10_000, 0, 0, 0, true, 5_000, 0, 400);
    let mut state = AwarenessState::default();
    update_from_evidence(&mut state, &[actor, player], &config);
    update_from_evidence(&mut state, &[actor, player], &config);
    assert_eq!(state.target(), Some(TargetId::player()));
}

#[test]
fn non_finite_geometry_is_rejected() {
    assert_eq!(
        quantize_geometry(f32::NAN, 0.0),
        Err(DetectionError::NonFiniteGeometry)
    );
    assert_eq!(
        quantize_geometry(1.0, f32::INFINITY),
        Err(DetectionError::NonFiniteGeometry)
    );
}

#[test]
fn legacy_confidence_migrates() {
    let mut state = AwarenessState {
        confidence: 0.6,
        ..Default::default()
    };
    migrate_legacy_awareness(&mut state);
    assert_eq!(state.confidence_milli, 600);
}

#[test]
fn hud_projection_hidden_caution_danger() {
    assert_eq!(project_detection_hud(&[]), DetectionHud::Hidden);
    assert_eq!(
        project_detection_hud(&[ObserverHudInput {
            hostile: true,
            acquired_player: false,
            confidence_milli: 400,
        }]),
        DetectionHud::Caution
    );
    assert_eq!(
        project_detection_hud(&[ObserverHudInput {
            hostile: true,
            acquired_player: true,
            confidence_milli: 800,
        }]),
        DetectionHud::Danger
    );
}
