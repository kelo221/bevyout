use super::*;
use proptest::prelude::*;

#[test]
fn base_overrides_template_before_additive_layers() {
    let mut definition = ActorDefinition::default();
    definition.template_values.insert(ActorValue::Health, 80.0);
    definition.base_values.insert(ActorValue::Health, 100.0);
    definition.race_modifiers.insert(ActorValue::Health, 5.0);
    definition.class_modifiers.insert(ActorValue::Health, 10.0);
    definition
        .faction_modifiers
        .insert(ActorValue::Health, -2.0);
    let mut state = ActorInstanceState::new(1, ActorLifeState::Alive);
    state.set_value_mutation(ActorValue::Health, -25.0).unwrap();

    assert_eq!(
        definition.resolve_value(&state, ActorValue::Health),
        ResolvedActorValue {
            inherited_or_base: 100.0,
            authored_modifier: 13.0,
            runtime_mutation: -25.0,
            effective: 88.0,
        }
    );
}

#[test]
fn repeated_seed_does_not_reset_mutations() {
    let mut store = ActorStateStore::default();
    assert_eq!(
        store.seed(1, 2, ActorLifeState::Alive).unwrap(),
        ActorSeedOutcome::Inserted
    );
    store
        .get_mut(1, 2)
        .unwrap()
        .set_value_mutation(ActorValue::Health, -12.0)
        .unwrap();
    assert_eq!(
        store.seed(1, 2, ActorLifeState::Alive).unwrap(),
        ActorSeedOutcome::Existing
    );
    assert_eq!(
        store.get(1, 2).unwrap().value_mutations[&ActorValue::Health],
        -12.0
    );
}

#[test]
fn non_finite_mutation_is_rejected() {
    let mut state = ActorInstanceState::new(1, ActorLifeState::Alive);
    assert!(matches!(
        state.set_value_mutation(ActorValue::Health, f32::NAN),
        Err(ActorStateError::NonFiniteValue { .. })
    ));
}

#[test]
fn empty_modifier_layers_resolve_to_positive_zero() {
    let definition = ActorDefinition::default();
    let state = ActorInstanceState::new(1, ActorLifeState::Alive);

    let resolved = definition.resolve_value(&state, ActorValue::Health);

    assert_eq!(resolved.authored_modifier.to_bits(), 0.0_f32.to_bits());
}

proptest! {
    #[test]
    fn finite_layers_resolve_deterministically(
        template in -10_000f32..10_000f32,
        base in -10_000f32..10_000f32,
        race in -1_000f32..1_000f32,
        class in -1_000f32..1_000f32,
        faction in -1_000f32..1_000f32,
        runtime in -10_000f32..10_000f32,
    ) {
        let mut definition = ActorDefinition::default();
        definition.template_values.insert(ActorValue::Health, template);
        definition.base_values.insert(ActorValue::Health, base);
        definition.race_modifiers.insert(ActorValue::Health, race);
        definition.class_modifiers.insert(ActorValue::Health, class);
        definition.faction_modifiers.insert(ActorValue::Health, faction);
        let mut state = ActorInstanceState::new(1, ActorLifeState::Alive);
        state.set_value_mutation(ActorValue::Health, runtime).unwrap();

        let first = definition.resolve_value(&state, ActorValue::Health);
        let second = definition.resolve_value(&state, ActorValue::Health);
        prop_assert_eq!(first, second);
        let expected = base + race + class + faction + runtime;
        prop_assert!((first.effective - expected).abs() <= 0.002);
    }
}
