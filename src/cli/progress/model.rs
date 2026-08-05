use clap::ValueEnum;

/// Selects the presentation policy for command progress written to stderr.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum ProgressMode {
    /// Use a single updating line on a terminal and plain lines when redirected.
    #[default]
    Auto,
    /// Always use the single-line terminal renderer.
    Tty,
    /// Always use newline-delimited progress lines.
    Plain,
    /// Do not emit progress events.
    Off,
}

/// A known or open-ended amount of work.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorkEstimate {
    pub completed: u64,
    pub total: Option<u64>,
}

impl WorkEstimate {
    pub(crate) fn complete_unit(&mut self, total: Option<u64>) {
        self.completed = self.completed.saturating_add(1);
        if total.is_some() {
            self.total = total;
        }
    }
}

/// Backend-independent progress notifications emitted by prepare and bake.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProgressEvent {
    Started {
        operation: String,
        total: Option<u64>,
    },
    PhaseStarted {
        phase: String,
        total: Option<u64>,
    },
    UnitCompleted {
        completed: u64,
        total: Option<u64>,
        cache_hit: Option<bool>,
    },
    Message {
        text: String,
    },
    Finished {
        success: bool,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhaseSnapshot {
    pub name: String,
    pub work: WorkEstimate,
    pub timing: ProgressTiming,
}

/// Coarse phase timing derived from completed work units. ETA stays unknown
/// until at least two units establish a usable rate.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProgressTiming {
    pub elapsed_ms: u64,
    pub eta_ms: Option<u64>,
}

/// Immutable state passed to renderers and future JSON/GUI backends.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProgressSnapshot {
    pub operation: Option<String>,
    pub operation_work: WorkEstimate,
    pub phases: Vec<PhaseSnapshot>,
    pub current_phase: Option<String>,
    pub current_work: WorkEstimate,
    pub current_timing: ProgressTiming,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub jobs_completed: u64,
    pub jobs_failed: u64,
    pub backend: Option<String>,
    pub samples: Option<u32>,
    pub bounces: Option<u32>,
    pub message: Option<String>,
    pub finished: bool,
    pub success: Option<bool>,
}
