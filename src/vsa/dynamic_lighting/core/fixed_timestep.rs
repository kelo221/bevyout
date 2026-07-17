use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct FixedTimestep {
    pub(crate) time_per_step: f32,
    pub(crate) accumulator: f32,
    pub(crate) pending_steps: u32,
}

impl FixedTimestep {
    pub(crate) const fn new(time_per_step: f32) -> Self {
        Self {
            time_per_step,
            accumulator: 0.0,
            pending_steps: 0,
        }
    }

    /// Exact source behavior: pending steps are counted, while effect dispatch
    /// intentionally evaluates the fixed branch once when the count is nonzero.
    pub(crate) fn update(&mut self, delta_seconds: f32) {
        self.pending_steps = 0;
        self.accumulator += delta_seconds;
        if self.accumulator >= self.time_per_step {
            self.pending_steps = (self.accumulator / self.time_per_step) as u32;
            self.accumulator -= self.pending_steps as f32 * self.time_per_step;
        }
    }
}

impl Default for FixedTimestep {
    fn default() -> Self {
        Self::new(1.0 / 30.0)
    }
}
