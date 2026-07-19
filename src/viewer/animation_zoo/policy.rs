//! Engine-independent animation-zoo playback state machine.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ZooControlAction {
    Previous,
    Next,
    Restart,
    TogglePause,
    ToggleLoop,
    SpeedUp,
    SpeedDown,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ZooPlaybackPolicy {
    pub(crate) index: usize,
    pub(crate) clip_count: usize,
    pub(crate) paused: bool,
    pub(crate) loop_current: bool,
    pub(crate) speed: f32,
    pub(crate) completed_cycles: u64,
    pub(crate) restart_generation: u64,
}

impl ZooPlaybackPolicy {
    pub(crate) fn new(clip_count: usize, index: usize) -> Self {
        Self {
            index: if clip_count == 0 {
                0
            } else {
                index % clip_count
            },
            clip_count,
            paused: false,
            loop_current: false,
            speed: 1.0,
            completed_cycles: 0,
            restart_generation: 0,
        }
    }

    /// Applies an explicit control. Returns true when the caller must restore
    /// bind pose and restart playback.
    pub(crate) fn apply(&mut self, action: ZooControlAction) -> bool {
        match action {
            ZooControlAction::Previous if self.clip_count > 0 => {
                self.index = (self.index + self.clip_count - 1) % self.clip_count;
                self.restart_generation += 1;
                true
            }
            ZooControlAction::Next if self.clip_count > 0 => {
                self.advance_index();
                self.restart_generation += 1;
                true
            }
            ZooControlAction::Restart if self.clip_count > 0 => {
                self.restart_generation += 1;
                true
            }
            ZooControlAction::TogglePause => {
                self.paused = !self.paused;
                false
            }
            ZooControlAction::ToggleLoop => {
                self.loop_current = !self.loop_current;
                false
            }
            ZooControlAction::SpeedUp => {
                self.speed = (self.speed * 2.0).min(4.0);
                false
            }
            ZooControlAction::SpeedDown => {
                self.speed = (self.speed * 0.5).max(0.25);
                false
            }
            ZooControlAction::Previous | ZooControlAction::Next | ZooControlAction::Restart => {
                false
            }
        }
    }

    pub(crate) fn finished(&mut self) -> bool {
        if self.clip_count == 0 {
            return false;
        }
        if !self.loop_current {
            self.advance_index();
        }
        self.restart_generation += 1;
        true
    }

    fn advance_index(&mut self) {
        self.index += 1;
        if self.index >= self.clip_count {
            self.index = 0;
            self.completed_cycles += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automatic_advance_wraps_and_counts_cycles() {
        let mut state = ZooPlaybackPolicy::new(2, 1);
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
    fn empty_clip_sets_are_safe_no_ops() {
        let mut state = ZooPlaybackPolicy::new(0, 99);
        assert!(!state.finished());
        assert!(!state.apply(ZooControlAction::Next));
        assert_eq!(state.index, 0);
    }
}
