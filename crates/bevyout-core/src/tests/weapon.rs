use super::*;

#[test]
fn reload_blocks_fire_until_advance_completes() {
    let mut state = WeaponState::new(WeaponDefinition::new(9.0, 100.0));
    assert_eq!(state.request_reload(), ReloadDecision::Started);
    assert_eq!(state.request_fire(), FireDecision::BlockedReloading);
    state.advance(DEFAULT_RELOAD_SECONDS);
    assert_eq!(state.request_fire(), FireDecision::Fired { shot_index: 1 });
}

#[test]
fn damage_mutates_health_and_marks_lethal_state() {
    let mut definition = ActorDefinition {
        base_form_id: 1,
        reference_form_id: 2,
        ..Default::default()
    };
    definition.base_values.insert(ActorValue::Health, 8.0);
    let mut state = ActorInstanceState::new(2, ActorLifeState::Alive);

    let outcome = apply_actor_damage(&definition, &mut state, 9.0).unwrap();

    assert_eq!(outcome.remaining_health, 0.0);
    assert!(outcome.killed);
    assert_eq!(state.life_state, ActorLifeState::Dead);
    assert_eq!(
        definition
            .resolve_value(&state, ActorValue::Health)
            .effective,
        0.0
    );
}
