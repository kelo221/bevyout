mod aggregate;
mod model;
mod render;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

pub use aggregate::ProgressAggregator;
pub use model::{PhaseSnapshot, ProgressEvent, ProgressMode, ProgressSnapshot, WorkEstimate};

use std::io::{self, IsTerminal, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use render::ProgressRenderer;

const DEFAULT_RENDER_INTERVAL: Duration = Duration::from_millis(100);

/// A cloneable, thread-safe event reporter. Rendering errors are deliberately
/// ignored: progress must never turn a successful prepare or bake into a
/// failed command because stderr was closed by a caller.
#[derive(Clone)]
pub struct ProgressReporter {
    aggregate: ProgressAggregator,
    renderer: Arc<Mutex<ProgressRenderer>>,
    current_phase: Arc<Mutex<Option<String>>>,
}

impl ProgressReporter {
    pub fn new(mode: ProgressMode) -> Self {
        let stderr = io::stderr();
        Self::with_writer_and_interval(
            mode,
            stderr,
            io::stderr().is_terminal(),
            DEFAULT_RENDER_INTERVAL,
        )
    }

    #[cfg(test)]
    pub(crate) fn with_writer<W: Write + Send + 'static>(
        mode: ProgressMode,
        writer: W,
        interactive: bool,
    ) -> Self {
        Self::with_writer_and_interval(mode, writer, interactive, DEFAULT_RENDER_INTERVAL)
    }

    pub(crate) fn with_writer_and_interval<W: Write + Send + 'static>(
        mode: ProgressMode,
        writer: W,
        interactive: bool,
        interval: Duration,
    ) -> Self {
        Self {
            aggregate: ProgressAggregator::new(),
            renderer: Arc::new(Mutex::new(ProgressRenderer::new(
                mode,
                writer,
                interactive,
                interval,
            ))),
            current_phase: Arc::new(Mutex::new(None)),
        }
    }

    /// Creates a worker-local phase cursor over the shared aggregate and
    /// renderer. Parallel workers can then advance their own nested phase
    /// without changing which phase another worker's unit completion targets.
    pub(crate) fn scoped(&self) -> Self {
        Self {
            aggregate: self.aggregate.clone(),
            renderer: self.renderer.clone(),
            current_phase: Arc::new(Mutex::new(None)),
        }
    }

    pub fn emit(&self, event: ProgressEvent) -> ProgressSnapshot {
        match &event {
            ProgressEvent::Started { .. } => self.set_current_phase(None),
            ProgressEvent::PhaseStarted { phase, .. } => {
                self.set_current_phase(Some(phase.clone()))
            }
            ProgressEvent::UnitCompleted { .. }
            | ProgressEvent::Message { .. }
            | ProgressEvent::Finished { .. } => {}
        }
        let force = matches!(
            event,
            ProgressEvent::Started { .. } | ProgressEvent::Finished { .. }
        );
        let snapshot = self.aggregate.record(event);
        self.render(&snapshot, force);
        snapshot
    }

    pub fn started(&self, operation: impl Into<String>, total: Option<u64>) -> ProgressSnapshot {
        self.emit(ProgressEvent::Started {
            operation: operation.into(),
            total,
        })
    }

    pub fn phase_started(&self, phase: impl Into<String>, total: Option<u64>) -> ProgressSnapshot {
        self.emit(ProgressEvent::PhaseStarted {
            phase: phase.into(),
            total,
        })
    }

    pub fn message(&self, text: impl Into<String>) -> ProgressSnapshot {
        self.emit(ProgressEvent::Message { text: text.into() })
    }

    pub fn unit_completed(&self, total: Option<u64>, cache_hit: Option<bool>) -> ProgressSnapshot {
        let snapshot = if let Some(phase) = self.current_phase() {
            self.aggregate
                .complete_unit_in_phase(phase, total, cache_hit)
        } else {
            self.aggregate.complete_unit(total, cache_hit)
        };
        self.render(&snapshot, false);
        snapshot
    }

    pub(crate) fn unit_completed_in_phase(
        &self,
        phase: impl Into<String>,
        total: Option<u64>,
        cache_hit: Option<bool>,
    ) -> ProgressSnapshot {
        self.set_current_phase(Some(phase.into()));
        let phase = self.current_phase().expect("phase was just set");
        let snapshot = self
            .aggregate
            .complete_unit_in_phase(phase, total, cache_hit);
        self.render(&snapshot, false);
        snapshot
    }

    pub fn cache_counts(&self, hits: u64, misses: u64) -> ProgressSnapshot {
        let snapshot = self.aggregate.cache_counts(hits, misses);
        self.render(&snapshot, false);
        snapshot
    }

    pub fn job_completed(&self, success: bool) -> ProgressSnapshot {
        let snapshot = self.aggregate.job_completed(success);
        self.render(&snapshot, false);
        snapshot
    }

    pub fn set_backend(&self, backend: impl Into<String>) -> ProgressSnapshot {
        let snapshot = self.aggregate.set_backend(backend);
        self.render(&snapshot, false);
        snapshot
    }

    pub fn set_sampling(&self, samples: Option<u32>, bounces: Option<u32>) -> ProgressSnapshot {
        let snapshot = self.aggregate.set_sampling(samples, bounces);
        self.render(&snapshot, false);
        snapshot
    }

    pub fn finished(&self, success: bool) -> ProgressSnapshot {
        self.emit(ProgressEvent::Finished { success })
    }

    pub fn snapshot(&self) -> ProgressSnapshot {
        self.aggregate.snapshot()
    }

    fn current_phase(&self) -> Option<String> {
        self.current_phase
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    fn set_current_phase(&self, phase: Option<String>) {
        *self
            .current_phase
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = phase;
    }

    fn render(&self, snapshot: &ProgressSnapshot, force: bool) {
        let mut renderer = self
            .renderer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _ = renderer.render(snapshot, force);
    }
}
