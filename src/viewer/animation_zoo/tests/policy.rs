use super::*;

#[test]
fn automatic_advance_wraps_and_counts_cycles() {
    let mut state = ZooPlaybackPolicy::new(2, 1);
    state.auto_advance = true;
    state.loop_current = false;
    assert!(state.finished());
    assert_eq!(state.index, 0);
    assert_eq!(state.completed_cycles, 1);
}

#[test]
fn previous_next_restart_pause_loop_and_speed_are_deterministic() {
    let mut state = ZooPlaybackPolicy::new(3, 0);
    assert!(state.apply(ZooControlAction::Previous));
    assert_eq!(state.index, 2);
    assert!(state.apply(ZooControlAction::Next));
    assert_eq!(state.index, 0);
    assert_eq!(state.completed_cycles, 1);
    assert!(state.apply(ZooControlAction::Restart));
    assert!(!state.apply(ZooControlAction::TogglePause));
    assert!(state.paused);
    state.apply(ZooControlAction::ToggleLoop);
    assert!(state.loop_current);
    let index = state.index;
    assert!(state.finished());
    assert_eq!(state.index, index);
    state.apply(ZooControlAction::SpeedUp);
    state.apply(ZooControlAction::SpeedUp);
    state.apply(ZooControlAction::SpeedUp);
    assert_eq!(state.speed, 4.0);
    for _ in 0..8 {
        state.apply(ZooControlAction::SpeedDown);
    }
    assert_eq!(state.speed, 0.25);
}

#[test]
fn selected_clip_holds_after_finishing_until_cycle_is_enabled() {
    let mut state = ZooPlaybackPolicy::new(2, 0);
    assert!(!state.finished());
    assert_eq!(state.index, 0);
    state.apply(ZooControlAction::ToggleCycle);
    assert!(state.auto_advance);
    assert!(state.finished());
    assert_eq!(state.index, 1);
}

#[test]
fn empty_clip_sets_are_safe_no_ops() {
    let mut state = ZooPlaybackPolicy::new(0, 99);
    assert!(!state.finished());
    assert!(!state.apply(ZooControlAction::Next));
    assert_eq!(state.index, 0);
}

#[test]
fn selecting_a_clip_restarts_that_clip_and_rejects_out_of_range_indices() {
    let mut state = ZooPlaybackPolicy::new(3, 0);
    assert!(state.apply(ZooControlAction::Select(2)));
    assert_eq!(state.index, 2);
    assert_eq!(state.restart_generation, 1);
    assert!(!state.apply(ZooControlAction::Select(3)));
    assert_eq!(state.index, 2);
    assert_eq!(state.restart_generation, 1);
}
