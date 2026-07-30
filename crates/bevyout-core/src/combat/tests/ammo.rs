use super::*;

#[test]
fn operational_reload_only_consumes_missing_rounds() {
    let decision = plan_reload(
        MagazineState {
            ammo_form_id: Some(10),
            loaded: 7,
        },
        10,
        12,
        20,
    )
    .unwrap();
    assert_eq!(decision.kind, ReloadKind::Operational);
    assert_eq!(decision.consume_reserve, 5);
    assert_eq!(decision.return_loaded, 0);
}

#[test]
fn ammo_switch_returns_old_rounds_before_consuming_new_ones() {
    let decision = plan_reload(
        MagazineState {
            ammo_form_id: Some(10),
            loaded: 7,
        },
        20,
        12,
        20,
    )
    .unwrap();
    assert_eq!(decision.kind, ReloadKind::AmmoSwitch);
    assert_eq!(decision.return_loaded, 7);
    assert_eq!(decision.consume_reserve, 12);
}

#[test]
fn dry_fire_does_not_mutate_the_magazine() {
    let mut magazine = MagazineState {
        ammo_form_id: Some(10),
        loaded: 0,
    };
    assert_eq!(consume_round(&mut magazine), Err(FireBlockReason::Empty));
    assert_eq!(magazine.loaded, 0);
}
