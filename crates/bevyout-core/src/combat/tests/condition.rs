use super::*;
use crate::combat::CombatRngDomain;

fn draw(value: u32) -> CombatRngDraw {
    CombatRngDraw {
        domain: CombatRngDomain::FireJam,
        index: 0,
        value,
    }
}

#[test]
fn condition_scales_damage_and_degrades_once() {
    let policy = WeaponConditionPolicy::with_degradation(Some(100), 5);
    let decision = policy
        .evaluate_fire(10.0, Some(50), draw(u32::MAX))
        .unwrap();
    assert_eq!(decision.condition_before, Some(50));
    assert_eq!(decision.condition_after, Some(45));
    assert_eq!(decision.damage_multiplier, 0.625);
    assert_eq!(decision.damage, 6.25);
    assert!(!decision.jammed);
}

#[test]
fn zero_condition_keeps_minimum_effectiveness_and_can_jam() {
    let policy = WeaponConditionPolicy::new(Some(100));
    let decision = policy.evaluate_fire(10.0, Some(0), draw(0)).unwrap();
    assert_eq!(decision.damage_multiplier, MIN_DAMAGE_EFFECTIVENESS);
    assert_eq!(decision.condition_after, Some(0));
    assert!(decision.jammed);
}

#[test]
fn missing_condition_uses_maximum_and_never_jams() {
    let policy = WeaponConditionPolicy::new(Some(100));
    let decision = policy.evaluate_fire(10.0, None, draw(0)).unwrap();
    assert_eq!(decision.condition_before, Some(100));
    assert_eq!(decision.condition_after, Some(99));
    assert_eq!(decision.damage_multiplier, 1.0);
    assert!(!decision.jammed);
}

#[test]
fn conditionless_weapon_has_full_damage_and_no_jam() {
    let policy = WeaponConditionPolicy::new(None);
    let decision = policy.evaluate_fire(10.0, None, draw(0)).unwrap();
    assert_eq!(decision.condition_before, None);
    assert_eq!(decision.condition_after, None);
    assert_eq!(decision.damage, 10.0);
    assert!(!decision.jammed);
}
