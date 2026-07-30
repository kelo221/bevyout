//! Engine-independent animation-zoo playback state machine.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ZooControlAction {
    Previous,
    Next,
    Select(usize),
    Restart,
    TogglePause,
    ToggleLoop,
    ToggleCycle,
    SpeedUp,
    SpeedDown,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ZooPlaybackPolicy {
    pub(crate) index: usize,
    pub(crate) clip_count: usize,
    pub(crate) paused: bool,
    pub(crate) loop_current: bool,
    pub(crate) auto_advance: bool,
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
            auto_advance: false,
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
            ZooControlAction::Select(index) if index < self.clip_count => {
                self.index = index;
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
            ZooControlAction::ToggleCycle => {
                self.auto_advance = !self.auto_advance;
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
            ZooControlAction::Previous
            | ZooControlAction::Next
            | ZooControlAction::Select(_)
            | ZooControlAction::Restart => false,
        }
    }

    pub(crate) fn finished(&mut self) -> bool {
        if self.clip_count == 0 {
            return false;
        }
        if !self.loop_current && self.auto_advance {
            self.advance_index();
        } else if !self.loop_current {
            return false;
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
#[path = "tests/policy.rs"]
mod tests;
