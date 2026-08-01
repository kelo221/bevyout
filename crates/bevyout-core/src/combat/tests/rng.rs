use super::*;

#[test]
fn same_seed_and_domain_sequence_is_stable() {
    let mut first = CombatRngState::from_seed(42);
    let mut second = CombatRngState::from_seed(42);
    assert_eq!(
        first.draw(CombatRngDomain::FireJam),
        second.draw(CombatRngDomain::FireJam)
    );
    assert_eq!(
        first.draw(CombatRngDomain::ReloadJam),
        second.draw(CombatRngDomain::ReloadJam)
    );
    assert_eq!(first.draw_index, 2);
}

#[test]
fn domains_are_separated() {
    let mut fire = CombatRngState::from_seed(7);
    let mut reload = CombatRngState::from_seed(7);
    assert_ne!(
        fire.draw(CombatRngDomain::FireJam).unwrap().value,
        reload.draw(CombatRngDomain::ReloadJam).unwrap().value
    );
}

#[test]
fn invalid_revision_does_not_advance_index() {
    let mut state = CombatRngState {
        revision: "old".into(),
        seed: 1,
        draw_index: 4,
    };
    assert!(state.draw(CombatRngDomain::FireJam).is_err());
    assert_eq!(state.draw_index, 4);
}
