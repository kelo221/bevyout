//! Unit tests for the pure radiation kernel (M9 wave 3 #317). Threshold
//! penalties are the roadmap's vanilla table.

use super::*;

use crate::actor_state::SpecialAttribute;
use crate::radiation::{
    RADIATION_MAX, RadiationPool, apply_radiation, is_fatal, radiation_penalties, remove_rads,
    threshold_reached,
};

fn penalty(rads: u16, attribute: SpecialAttribute) -> i8 {
    radiation_penalties(rads)
        .get(&attribute)
        .copied()
        .unwrap_or(0)
}

#[test]
fn thresholds_step_through_the_vanilla_table() {
    assert_eq!(threshold_reached(0), 0);
    assert_eq!(threshold_reached(199), 0);
    assert_eq!(threshold_reached(200), 200);
    assert_eq!(threshold_reached(399), 200);
    assert_eq!(threshold_reached(400), 400);
    assert_eq!(threshold_reached(600), 600);
    assert_eq!(threshold_reached(800), 800);
    assert_eq!(threshold_reached(1000), 800);
}

#[test]
fn penalties_match_the_roadmap_values() {
    // Minor 200: -1 END.
    assert_eq!(penalty(200, SpecialAttribute::Endurance), -1);
    assert_eq!(penalty(200, SpecialAttribute::Agility), 0);
    // Advanced 400: -2 END, -1 AGI.
    assert_eq!(penalty(400, SpecialAttribute::Endurance), -2);
    assert_eq!(penalty(400, SpecialAttribute::Agility), -1);
    // Critical 600: -3 END, -2 AGI, -1 STR.
    assert_eq!(penalty(600, SpecialAttribute::Endurance), -3);
    assert_eq!(penalty(600, SpecialAttribute::Agility), -2);
    assert_eq!(penalty(600, SpecialAttribute::Strength), -1);
    // Deadly 800: -3 END, -2 AGI, -2 STR, -1 INT.
    assert_eq!(penalty(800, SpecialAttribute::Endurance), -3);
    assert_eq!(penalty(800, SpecialAttribute::Agility), -2);
    assert_eq!(penalty(800, SpecialAttribute::Strength), -2);
    assert_eq!(penalty(800, SpecialAttribute::Intelligence), -1);
    // Thresholds replace, not stack: 800 is not 200+400+600+800 combined.
    assert_eq!(penalty(800, SpecialAttribute::Endurance), -3);
    // Luck and charisma never take rad penalties.
    assert_eq!(penalty(1000, SpecialAttribute::Luck), 0);
    assert_eq!(penalty(1000, SpecialAttribute::Charisma), 0);
}

#[test]
fn fatal_only_at_the_cap() {
    assert!(!is_fatal(999));
    assert!(is_fatal(1000));
    assert!(is_fatal(RADIATION_MAX));
}

#[test]
fn apply_radiation_resists_and_saturates() {
    let mut pool = RadiationPool::default();
    // No resistance: the whole dose lands.
    let outcome = apply_radiation(&mut pool, 300, 0);
    assert_eq!(outcome.absorbed_rads, 300);
    assert_eq!(pool.rads, 300);
    assert!(!outcome.fatal);
    // 50% resistance (Rad-X at 2500 bps halves a 100 dose).
    let outcome = apply_radiation(&mut pool, 100, 5_000);
    assert_eq!(outcome.absorbed_rads, 50);
    assert_eq!(pool.rads, 350);
    // 85% hard-cap resistance leaves a seventh.
    let outcome = apply_radiation(&mut pool, 100, 8_500);
    assert_eq!(outcome.absorbed_rads, 15);
    // Saturation clamps at the cap and reports the fatal crossing.
    let outcome = apply_radiation(&mut pool, u16::MAX, 0);
    assert_eq!(pool.rads, RADIATION_MAX);
    assert!(outcome.fatal);
    assert_eq!(pool.threshold_reached(), 800);
}

#[test]
fn remove_rads_cures_but_never_below_zero() {
    let mut pool = RadiationPool::new(600);
    assert_eq!(remove_rads(&mut pool, 50), 50);
    assert_eq!(pool.rads, 550);
    // RadAway cannot go below zero.
    assert_eq!(remove_rads(&mut pool, 10_000), 550);
    assert_eq!(pool.rads, 0);
    assert_eq!(pool.threshold_reached(), 0);
}
