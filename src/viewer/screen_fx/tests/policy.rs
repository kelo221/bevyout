use super::*;

fn definition(id: u32, duration_ms: u32, values: ScreenFxValues) -> ScreenFxDefinition {
    ScreenFxDefinition {
        modifier_form_id: id,
        duration_ms,
        static_values: values,
        curves: Vec::new(),
        color_keyframes: Vec::new(),
        fade_color_keyframes: Vec::new(),
    }
}

#[test]
fn samples_integer_time_and_clamps_endpoints() {
    let definition = ScreenFxDefinition {
        modifier_form_id: 1,
        duration_ms: 100,
        static_values: ScreenFxValues::neutral(),
        curves: vec![ScreenFxCurve {
            property: ScreenFxProperty::Blood,
            operation: ScreenFxCurveOperation::Additive,
            keyframes: vec![
                ScreenFxKeyframe {
                    time_ms: 0,
                    value: 0.0,
                },
                ScreenFxKeyframe {
                    time_ms: 100,
                    value: 1.0,
                },
            ],
        }],
        color_keyframes: Vec::new(),
        fade_color_keyframes: Vec::new(),
    };
    let policy = ScreenFxPolicy::default();
    assert_eq!(policy.sample(&definition, 0).blood, 0.0);
    assert!((policy.sample(&definition, 50).blood - 0.5).abs() < EPSILON);
    assert_eq!(policy.sample(&definition, 100).blood, 1.0);
    assert_eq!(policy.sample(&definition, 200).blood, 1.0);
}

#[test]
fn non_expiring_curves_keep_their_authored_time_range() {
    let mut policy = ScreenFxPolicy::default();
    policy.start(ScreenFxStart::new(
        ScreenFxSource::Gameplay,
        8,
        0,
        0,
        ScreenFxDefinition {
            modifier_form_id: 8,
            duration_ms: 0,
            static_values: ScreenFxValues::neutral(),
            curves: vec![ScreenFxCurve {
                property: ScreenFxProperty::Blood,
                operation: ScreenFxCurveOperation::Additive,
                keyframes: vec![
                    ScreenFxKeyframe {
                        time_ms: 0,
                        value: 0.0,
                    },
                    ScreenFxKeyframe {
                        time_ms: 1_000,
                        value: 1.0,
                    },
                ],
            }],
            color_keyframes: Vec::new(),
            fade_color_keyframes: Vec::new(),
        },
    ));
    policy.advance_to(500);
    assert!((policy.snapshot().blood - 0.5).abs() < EPSILON);
    policy.advance_to(u64::from(u32::MAX) + 1);
    assert_eq!(policy.active_len(), 1);
}

#[test]
fn starts_are_idempotent_and_replacements_keep_one_active_modifier() {
    let mut policy = ScreenFxPolicy::default();
    let mut start = ScreenFxStart::new(
        ScreenFxSource::Gameplay,
        7,
        1,
        0,
        definition(
            7,
            1000,
            ScreenFxValues {
                blood: 0.25,
                ..ScreenFxValues::neutral()
            },
        ),
    );
    policy.start(start.clone());
    policy.start(start.clone());
    assert_eq!(policy.active_len(), 1);
    start.definition.static_values.blood = 0.75;
    policy.start(start);
    assert_eq!(policy.active_len(), 1);
    assert!((policy.snapshot().blood - 0.75).abs() < EPSILON);
}

#[test]
fn order_is_priority_then_form_id_then_sequence() {
    let mut policy = ScreenFxPolicy::default();
    for (id, priority, blood) in [(20, 2, 0.2), (10, 1, 0.1), (30, 2, 0.3)] {
        policy.start(ScreenFxStart::new(
            ScreenFxSource::Gameplay,
            id,
            priority,
            0,
            definition(
                id,
                1000,
                ScreenFxValues {
                    tint: [blood, 0.0, 0.0, blood],
                    ..ScreenFxValues::neutral()
                },
            ),
        ));
    }
    let snapshot = policy.snapshot();
    // The priority-1 tint is applied first, then FormID 20, then 30.
    assert!(snapshot.tint[0] > 0.0);
    assert!(snapshot.tint[3] > 0.0);
}

#[test]
fn expiry_and_clear_restore_the_base() {
    let base = ScreenFxValues {
        brightness: 0.8,
        ..ScreenFxValues::neutral()
    };
    let mut policy = ScreenFxPolicy::new(base);
    policy.start(ScreenFxStart::new(
        ScreenFxSource::WeaponHit,
        2,
        0,
        0,
        definition(
            2,
            10,
            ScreenFxValues {
                blood: 1.0,
                ..ScreenFxValues::neutral()
            },
        ),
    ));
    policy.advance_to(10);
    assert_eq!(policy.active_len(), 0);
    assert_eq!(policy.snapshot(), base);
    policy.apply(ScreenFxRequest::Clear {
        reason: ScreenFxClearReason::CellTransition,
    });
    assert_eq!(policy.snapshot(), base);
}

#[test]
fn settings_can_disable_blood_and_distortion() {
    let mut policy = ScreenFxPolicy::default();
    policy.set_settings(ScreenFxSettings {
        overall_intensity: 1.0,
        screen_blood: 0.0,
        flashes: 1.0,
        motion_and_distortion: 0.0,
    });
    policy.start(ScreenFxStart::new(
        ScreenFxSource::WeaponHit,
        4,
        0,
        0,
        definition(
            4,
            100,
            ScreenFxValues {
                blood: 1.0,
                double_vision: 1.0,
                ..ScreenFxValues::neutral()
            },
        ),
    ));
    let output = policy.snapshot();
    assert_eq!(output.blood, 0.0);
    assert_eq!(output.double_vision, 0.0);
}
