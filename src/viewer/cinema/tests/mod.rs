use super::*;

#[test]
fn default_state_is_inactive() {
    let state = CinemaState::default();
    assert!(!state.is_active());
    assert_eq!(state.mode_label(), "inactive");
    assert_eq!(state.target_form_id(), None);
    assert!(state.restore_mode.is_none());
}

#[test]
fn active_modes_report_label_and_target() {
    let follow = CinemaState {
        mode: CinemaMode::Follow {
            target: 0x0005_cf10,
            dist: 4.0,
            height: 2.0,
        },
        restore_mode: None,
    };
    assert!(follow.is_active());
    assert_eq!(follow.mode_label(), "follow");
    assert_eq!(follow.target_form_id(), Some(0x0005_cf10));

    let point = CinemaState {
        mode: CinemaMode::LookAtPoint {
            point: Vec3::splat(1.0),
        },
        restore_mode: None,
    };
    assert_eq!(point.mode_label(), "lookat");
    assert_eq!(point.target_form_id(), None);
}
