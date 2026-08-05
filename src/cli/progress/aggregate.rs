use std::sync::{Arc, Mutex};

use super::model::{PhaseSnapshot, ProgressEvent, ProgressSnapshot, WorkEstimate};

#[derive(Clone, Default)]
pub struct ProgressAggregator {
    state: Arc<Mutex<ProgressState>>,
}

#[derive(Default)]
struct ProgressState {
    operation: Option<String>,
    operation_work: WorkEstimate,
    phases: Vec<PhaseState>,
    current_phase: Option<String>,
    cache_hits: u64,
    cache_misses: u64,
    jobs_completed: u64,
    jobs_failed: u64,
    backend: Option<String>,
    samples: Option<u32>,
    bounces: Option<u32>,
    message: Option<String>,
    finished: bool,
    success: Option<bool>,
}

#[derive(Clone)]
struct PhaseState {
    name: String,
    work: WorkEstimate,
}

impl ProgressAggregator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&self, event: ProgressEvent) -> ProgressSnapshot {
        let mut state = self.lock();
        state.apply(event);
        state.snapshot()
    }

    /// Atomically completes one unit in the current phase. This is the safe
    /// worker-facing API: callers do not need to coordinate an absolute
    /// `completed` value before sending a notification.
    pub(crate) fn complete_unit(
        &self,
        total: Option<u64>,
        cache_hit: Option<bool>,
    ) -> ProgressSnapshot {
        let mut state = self.lock();
        if let Some(phase) = state.current_phase_mut() {
            phase.work.complete_unit(total);
        } else {
            state.operation_work.complete_unit(total);
        }
        match cache_hit {
            Some(true) => state.cache_hits = state.cache_hits.saturating_add(1),
            Some(false) => state.cache_misses = state.cache_misses.saturating_add(1),
            None => {}
        }
        state.snapshot()
    }

    pub(crate) fn complete_unit_in_phase(
        &self,
        phase: impl Into<String>,
        total: Option<u64>,
        cache_hit: Option<bool>,
    ) -> ProgressSnapshot {
        let mut state = self.lock();
        let phase = phase.into();
        state.current_phase = Some(phase.clone());
        let phase_state = if let Some(existing) = state
            .phases
            .iter_mut()
            .find(|existing| existing.name == phase)
        {
            existing
        } else {
            state.phases.push(PhaseState {
                name: phase,
                work: WorkEstimate::default(),
            });
            state.phases.last_mut().expect("phase was inserted")
        };
        phase_state.work.complete_unit(total);
        match cache_hit {
            Some(true) => state.cache_hits = state.cache_hits.saturating_add(1),
            Some(false) => state.cache_misses = state.cache_misses.saturating_add(1),
            None => {}
        }
        state.snapshot()
    }

    pub(crate) fn cache_counts(&self, hits: u64, misses: u64) -> ProgressSnapshot {
        let mut state = self.lock();
        state.cache_hits = state.cache_hits.saturating_add(hits);
        state.cache_misses = state.cache_misses.saturating_add(misses);
        state.snapshot()
    }

    pub(crate) fn job_completed(&self, success: bool) -> ProgressSnapshot {
        let mut state = self.lock();
        if success {
            state.jobs_completed = state.jobs_completed.saturating_add(1);
        } else {
            state.jobs_failed = state.jobs_failed.saturating_add(1);
        }
        state.snapshot()
    }

    pub(crate) fn set_backend(&self, backend: impl Into<String>) -> ProgressSnapshot {
        let mut state = self.lock();
        state.backend = Some(backend.into());
        state.snapshot()
    }

    pub(crate) fn set_sampling(
        &self,
        samples: Option<u32>,
        bounces: Option<u32>,
    ) -> ProgressSnapshot {
        let mut state = self.lock();
        state.samples = samples;
        state.bounces = bounces;
        state.snapshot()
    }

    pub fn snapshot(&self) -> ProgressSnapshot {
        self.lock().snapshot()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, ProgressState> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl ProgressState {
    fn apply(&mut self, event: ProgressEvent) {
        match event {
            ProgressEvent::Started { operation, total } => {
                self.operation = Some(operation);
                self.operation_work = WorkEstimate {
                    completed: 0,
                    total,
                };
                self.phases.clear();
                self.current_phase = None;
                self.cache_hits = 0;
                self.cache_misses = 0;
                self.jobs_completed = 0;
                self.jobs_failed = 0;
                self.backend = None;
                self.samples = None;
                self.bounces = None;
                self.message = None;
                self.finished = false;
                self.success = None;
            }
            ProgressEvent::PhaseStarted { phase, total } => {
                if let Some(existing) = self.phases.iter_mut().find(|item| item.name == phase) {
                    existing.work = WorkEstimate {
                        completed: 0,
                        total,
                    };
                } else {
                    self.phases.push(PhaseState {
                        name: phase.clone(),
                        work: WorkEstimate {
                            completed: 0,
                            total,
                        },
                    });
                }
                self.current_phase = Some(phase);
            }
            ProgressEvent::UnitCompleted {
                completed,
                total,
                cache_hit,
            } => {
                if let Some(phase) = self.current_phase_mut() {
                    phase.work.completed = completed;
                    if total.is_some() {
                        phase.work.total = total;
                    }
                } else {
                    self.operation_work.completed = completed;
                    if total.is_some() {
                        self.operation_work.total = total;
                    }
                }
                match cache_hit {
                    Some(true) => self.cache_hits = self.cache_hits.saturating_add(1),
                    Some(false) => self.cache_misses = self.cache_misses.saturating_add(1),
                    None => {}
                }
            }
            ProgressEvent::Message { text } => self.message = Some(text),
            ProgressEvent::Finished { success } => {
                self.finished = true;
                self.success = Some(success);
            }
        }
    }

    fn current_phase_mut(&mut self) -> Option<&mut PhaseState> {
        let current = self.current_phase.as_deref()?;
        self.phases.iter_mut().find(|phase| phase.name == current)
    }

    fn snapshot(&self) -> ProgressSnapshot {
        let phases = self
            .phases
            .iter()
            .map(|phase| PhaseSnapshot {
                name: phase.name.clone(),
                work: phase.work,
            })
            .collect::<Vec<_>>();
        let current_work = self
            .current_phase
            .as_deref()
            .and_then(|name| self.phases.iter().find(|phase| phase.name == name))
            .map(|phase| phase.work)
            .unwrap_or(self.operation_work);
        ProgressSnapshot {
            operation: self.operation.clone(),
            operation_work: self.operation_work,
            phases,
            current_phase: self.current_phase.clone(),
            current_work,
            cache_hits: self.cache_hits,
            cache_misses: self.cache_misses,
            jobs_completed: self.jobs_completed,
            jobs_failed: self.jobs_failed,
            backend: self.backend.clone(),
            samples: self.samples,
            bounces: self.bounces,
            message: self.message.clone(),
            finished: self.finished,
            success: self.success,
        }
    }
}
