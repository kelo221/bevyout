use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use super::model::{PhaseSnapshot, ProgressEvent, ProgressSnapshot, ProgressTiming, WorkEstimate};

const ETA_SAMPLE_LIMIT: usize = 16;
const ETA_MIN_SAMPLES: usize = 1;

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
    started_at: Instant,
    last_unit_at: Option<Instant>,
    recent_unit_ms: VecDeque<u64>,
}

impl PhaseState {
    fn new(name: String, total: Option<u64>) -> Self {
        Self {
            name,
            work: WorkEstimate {
                completed: 0,
                total,
            },
            started_at: Instant::now(),
            last_unit_at: None,
            recent_unit_ms: VecDeque::with_capacity(ETA_SAMPLE_LIMIT),
        }
    }

    fn reset(&mut self, total: Option<u64>) {
        self.work = WorkEstimate {
            completed: 0,
            total,
        };
        self.started_at = Instant::now();
        self.last_unit_at = None;
        self.recent_unit_ms.clear();
    }

    fn complete_unit(&mut self, total: Option<u64>, now: Instant) {
        self.work.complete_unit(total);
        self.record_unit_timing(now);
    }

    fn set_completed(&mut self, completed: u64, total: Option<u64>, now: Instant) {
        self.work.completed = completed;
        if total.is_some() {
            self.work.total = total;
        }
        self.record_unit_timing(now);
    }

    fn record_unit_timing(&mut self, now: Instant) {
        if let Some(previous) = self.last_unit_at {
            let elapsed_ms = now
                .duration_since(previous)
                .as_millis()
                .min(u128::from(u64::MAX)) as u64;
            if self.recent_unit_ms.len() == ETA_SAMPLE_LIMIT {
                self.recent_unit_ms.pop_front();
            }
            self.recent_unit_ms.push_back(elapsed_ms);
        }
        self.last_unit_at = Some(now);
    }

    fn timing(&self, now: Instant) -> ProgressTiming {
        let elapsed_ms = now
            .duration_since(self.started_at)
            .as_millis()
            .min(u128::from(u64::MAX)) as u64;
        let eta_ms = self.work.total.and_then(|total| {
            let remaining = total.saturating_sub(self.work.completed);
            if remaining == 0 || self.recent_unit_ms.len() < ETA_MIN_SAMPLES {
                return None;
            }
            let total_ms: u128 = self
                .recent_unit_ms
                .iter()
                .map(|value| u128::from(*value))
                .sum();
            let average_ms = total_ms / self.recent_unit_ms.len() as u128;
            Some(
                average_ms
                    .saturating_mul(u128::from(remaining))
                    .min(u128::from(u64::MAX)) as u64,
            )
        });
        ProgressTiming { elapsed_ms, eta_ms }
    }
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
        let now = Instant::now();
        if let Some(phase) = state.current_phase_mut() {
            phase.complete_unit(total, now);
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
        let now = Instant::now();
        let phase = phase.into();
        state.current_phase = Some(phase.clone());
        let phase_state = if let Some(existing) = state
            .phases
            .iter_mut()
            .find(|existing| existing.name == phase)
        {
            existing
        } else {
            state.phases.push(PhaseState::new(phase, total));
            state.phases.last_mut().expect("phase was inserted")
        };
        phase_state.complete_unit(total, now);
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

    pub(crate) fn clear_message(&self) -> ProgressSnapshot {
        let mut state = self.lock();
        state.message = None;
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
                    existing.reset(total);
                } else {
                    self.phases.push(PhaseState::new(phase.clone(), total));
                }
                self.current_phase = Some(phase);
            }
            ProgressEvent::UnitCompleted {
                completed,
                total,
                cache_hit,
            } => {
                let now = Instant::now();
                if let Some(phase) = self.current_phase_mut() {
                    phase.set_completed(completed, total, now);
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
        let now = Instant::now();
        let phases = self
            .phases
            .iter()
            .map(|phase| PhaseSnapshot {
                name: phase.name.clone(),
                work: phase.work,
                timing: phase.timing(now),
            })
            .collect::<Vec<_>>();
        let (current_work, current_timing) = self
            .current_phase
            .as_deref()
            .and_then(|name| self.phases.iter().find(|phase| phase.name == name))
            .map(|phase| (phase.work, phase.timing(now)))
            .unwrap_or((self.operation_work, ProgressTiming::default()));
        ProgressSnapshot {
            operation: self.operation.clone(),
            operation_work: self.operation_work,
            phases,
            current_phase: self.current_phase.clone(),
            current_work,
            current_timing,
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
