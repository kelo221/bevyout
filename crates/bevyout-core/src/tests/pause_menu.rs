use super::*;

#[test]
fn default_selection_is_continue() {
    let state = PauseMenuState::default();
    assert_eq!(state.selected(), PauseMenuOption::Continue);
    assert_eq!(state.activate(), Some(PauseMenuAction::Continue));
}

#[test]
fn navigation_wraps_through_all_entries() {
    let mut state = PauseMenuState::new();
    state.move_up();
    assert_eq!(state.selected(), PauseMenuOption::Quit);
    state.move_down();
    assert_eq!(state.selected(), PauseMenuOption::Continue);
    for _ in 0..5 {
        state.move_down();
    }
    assert_eq!(state.selected(), PauseMenuOption::Quit);
    assert_eq!(state.activate(), Some(PauseMenuAction::Quit));
}

#[test]
fn disabled_entries_do_not_activate() {
    let mut state = PauseMenuState::new();
    state.select(PauseMenuOption::Save);
    assert!(!state.selected().is_enabled());
    assert_eq!(state.activate(), None);
    state.select(PauseMenuOption::Settings);
    assert_eq!(state.activate(), None);
}

#[test]
fn labels_match_fallout_title_case() {
    assert_eq!(PauseMenuOption::Continue.label(), "Continue");
    assert_eq!(PauseMenuOption::Settings.label(), "Settings");
    assert_eq!(PauseMenuOption::Quit.label(), "Quit");
}
