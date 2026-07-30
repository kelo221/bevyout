use super::*;

#[test]
fn geometry_reports_distance_and_forward_cone_angle() {
    // Actor at origin facing -Z (Bevy forward); player 5m ahead.
    let inputs = player_perception_inputs(
        Vec3::ZERO,
        Vec3::NEG_Z,
        Vec3::new(0.0, 0.0, -5.0),
        true,
        true,
    );
    assert!((inputs.distance - 5.0).abs() < 1e-4);
    assert!(inputs.angle_to_target < 1e-3);
}

#[test]
fn geometry_reports_target_directly_behind_as_wide_angle() {
    let inputs = player_perception_inputs(
        Vec3::ZERO,
        Vec3::NEG_Z,
        Vec3::new(0.0, 0.0, 5.0),
        true,
        true,
    );
    assert!((inputs.angle_to_target - std::f32::consts::PI).abs() < 1e-3);
}

#[test]
fn summary_consumer_reflects_the_single_awareness_authority() {
    // A minimal app with no physics: prove the awareness authority
    // (ActorAwareness) feeds the consumer (PerceptionSummary).
    let mut app = App::new();
    app.init_resource::<PerceptionSummary>()
        .add_systems(Update, summarize_awareness);

    let aware = ActorAwareness {
        state: AwarenessState {
            confidence: 1.0,
            acquired: Some(TargetId::player()),
            ..Default::default()
        },
        last_player: None,
    };
    app.world_mut().spawn((
        ActorRuntime {
            base_form_id: 0x20,
            reference_form_id: 0x30,
            kind: bevyout_core::actor::ActorKind::Humanoid,
            assembly: None,
        },
        aware,
    ));
    let unaware = ActorAwareness::default();
    app.world_mut().spawn((
        ActorRuntime {
            base_form_id: 0x21,
            reference_form_id: 0x31,
            kind: bevyout_core::actor::ActorKind::Humanoid,
            assembly: None,
        },
        unaware,
    ));

    app.update();

    let summary = app.world().resource::<PerceptionSummary>();
    assert!(summary.aware_of_player.contains(&0x30));
    assert!(!summary.aware_of_player.contains(&0x31));
}
