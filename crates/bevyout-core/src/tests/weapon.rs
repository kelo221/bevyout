use super::*;
use crate::combat::body::BodyPartId;
use crate::combat::limbs::ShotId;

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

#[test]
fn limb_milli_uses_applied_health_damage() {
    let mut definition = ActorDefinition {
        base_form_id: 1,
        reference_form_id: 2,
        ..Default::default()
    };
    definition.base_values.insert(ActorValue::Health, 5.0);
    let mut state = ActorInstanceState::new(2, ActorLifeState::Alive);
    let evidence = ImpactEvidence {
        distance_meters: 1.0,
        body_part: Some(BodyPartId::Torso),
        shot_id: Some(ShotId::from_weapon_shot(9, 1)),
        target: None,
    };
    let outcome = resolve_actor_impact(
        WeaponDefinition::new(10.0, 100.0),
        evidence,
        &definition,
        &mut state,
    )
    .unwrap();
    let ImpactOutcome::Actor(actor) = outcome else {
        panic!("expected actor impact");
    };
    assert_eq!(actor.health.applied_damage, 5.0);
    assert_eq!(actor.limb.unwrap().remaining_milli, 95_000);
}

#[test]
fn reequip_shot_identities_are_independent() {
    let mut definition = ActorDefinition {
        base_form_id: 1,
        reference_form_id: 2,
        ..Default::default()
    };
    definition.base_values.insert(ActorValue::Health, 50.0);
    let mut state = ActorInstanceState::new(2, ActorLifeState::Alive);
    let first = resolve_actor_impact(
        WeaponDefinition::new(10.0, 100.0),
        ImpactEvidence {
            distance_meters: 1.0,
            body_part: Some(BodyPartId::Torso),
            shot_id: Some(ShotId::from_weapon_shot(1, 1)),
            target: None,
        },
        &definition,
        &mut state,
    )
    .unwrap();
    let second = resolve_actor_impact(
        WeaponDefinition::new(10.0, 100.0),
        ImpactEvidence {
            distance_meters: 1.0,
            body_part: Some(BodyPartId::Torso),
            shot_id: Some(ShotId::from_weapon_shot(2, 1)),
            target: None,
        },
        &definition,
        &mut state,
    )
    .unwrap();
    assert!(!matches!(first, ImpactOutcome::Duplicate));
    assert!(!matches!(second, ImpactOutcome::Duplicate));
    assert_eq!(state.limbs.part(BodyPartId::Torso).current_milli, 80_000);
}
