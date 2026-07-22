//! Pure AI package lifecycle state machine (issue #194, depends on #193).
//!
//! Drives one actor's active package through
//! `select -> start -> tick -> pause/preempt -> complete -> fail -> retry`.
//! Selection (#193) decides *which* package should be active; this machine
//! owns *what happens to it over time*, with explicit preemption and retry
//! semantics:
//!
//! - **Preemption**: when selection picks a package other than the one
//!   running, the running package is *paused* (its step and elapsed time
//!   preserved) and the newly-selected one starts. A single paused package is
//!   retained; re-selecting it later **resumes** it at its saved step rather
//!   than restarting. Re-selecting `None` (a schedule gap) pauses the running
//!   package without starting anything.
//! - **Failure & retry**: `fail()` is a first-class outcome. A failed package
//!   enters a backoff (doubling from a base, capped) before a *restart* (step
//!   0, a fresh attempt -- distinct from resume). Backoff means a failing
//!   package does not spin every tick. Once `max_retries` restarts are
//!   exhausted the package is terminally `Failed` and is cleared.
//!
//! Persistence is **minimal**: only the active-or-paused package's identity,
//! step, and elapsed time are snapshotted -- exactly the fields the existing
//! `bevyout_core::actor_state::ActorPackageCheckpoint` (save format v4)
//! already carries, so no save-shape change and no revision bump. On reload a
//! snapshot resumes as `Running` at the saved step; re-selection then
//! re-decides. Tests assert the *observable* resumed step/elapsed, never the
//! machine's private phase bookkeeping.
//!
//! std/serde-only (no Bevy) so it compiles verbatim into `tests/features.rs`
//! via `#[path]`. The `ActorPackageCheckpoint` bridge lives in the pure core
//! crate, itself Bevy-free.

// The full state machine is the #194 deliverable and is exercised end-to-end
// by this module's unit tests. Its time-driven transitions (`tick`,
// `complete`, `fail`, and the retry path) have no non-test caller in this wave
// -- the current visible surface (`showpackages`) only drives selection and
// checkpoint resume, and the per-package families that tick/complete/fail a
// package are #196/#197/#198. Allow the resulting dead-code until then rather
// than fake a caller.
#![allow(dead_code)]

use bevyout_core::actor_state::ActorPackageCheckpoint;

/// Default retry ceiling: after this many restarts a package is terminally
/// failed. Small so a genuinely broken package gives up quickly.
pub const DEFAULT_MAX_RETRIES: u32 = 3;
/// Default backoff base in seconds; doubles per retry (1s, 2s, 4s, ...).
pub const DEFAULT_BASE_BACKOFF_SECONDS: f32 = 1.0;
/// Backoff never exceeds this, so the doubling cannot overflow into minutes.
pub const MAX_BACKOFF_SECONDS: f32 = 60.0;

/// The observable phase of the lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecyclePhase {
    /// No active package.
    Idle,
    /// A package is active and ticking.
    Running,
    /// A package was preempted; its progress is retained for resume.
    Paused,
    /// The active package finished successfully (terminal until re-selected).
    Completed,
    /// The active package failed and is counting down to a retry restart.
    AwaitingRetry,
    /// The active package exhausted its retries (terminal until re-selected).
    Failed,
}

impl LifecyclePhase {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Completed => "completed",
            Self::AwaitingRetry => "awaiting-retry",
            Self::Failed => "failed",
        }
    }
}

/// The mutable progress of one package instance.
#[derive(Debug, Clone, Copy, PartialEq)]
struct ActivePackage {
    form_id: u32,
    /// The procedure index a package family (#196+) advances; the resume
    /// point across preemption and save/load.
    step: u32,
    elapsed_seconds: f32,
    retry_count: u32,
    backoff_remaining: f32,
}

impl ActivePackage {
    fn fresh(form_id: u32) -> Self {
        Self {
            form_id,
            step: 0,
            elapsed_seconds: 0.0,
            retry_count: 0,
            backoff_remaining: 0.0,
        }
    }
}

/// One actor's package lifecycle.
#[derive(Debug, Clone, PartialEq)]
pub struct PackageLifecycle {
    phase: LifecyclePhase,
    active: Option<ActivePackage>,
    /// A single preempted package retained for resume.
    paused: Option<ActivePackage>,
    max_retries: u32,
    base_backoff_seconds: f32,
}

impl Default for PackageLifecycle {
    fn default() -> Self {
        Self {
            phase: LifecyclePhase::Idle,
            active: None,
            paused: None,
            max_retries: DEFAULT_MAX_RETRIES,
            base_backoff_seconds: DEFAULT_BASE_BACKOFF_SECONDS,
        }
    }
}

impl PackageLifecycle {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the retry policy (mainly for deterministic tests).
    #[must_use]
    pub fn with_retry_policy(mut self, max_retries: u32, base_backoff_seconds: f32) -> Self {
        self.max_retries = max_retries;
        self.base_backoff_seconds = base_backoff_seconds;
        self
    }

    #[must_use]
    pub fn phase(&self) -> LifecyclePhase {
        self.phase
    }

    /// The FormID of the active package, if any (also the paused one while
    /// nothing else is active, so `showpackages` can name it).
    #[must_use]
    pub fn active_form_id(&self) -> Option<u32> {
        self.active
            .as_ref()
            .or(self.paused.as_ref())
            .map(|package| package.form_id)
    }

    /// The current resume step of the active package, if any.
    #[must_use]
    pub fn step(&self) -> Option<u32> {
        self.active
            .as_ref()
            .or(self.paused.as_ref())
            .map(|p| p.step)
    }

    /// Elapsed seconds in the current attempt of the active package.
    #[must_use]
    pub fn elapsed_seconds(&self) -> Option<f32> {
        self.active
            .as_ref()
            .or(self.paused.as_ref())
            .map(|p| p.elapsed_seconds)
    }

    /// Retry attempts consumed by the active package.
    #[must_use]
    pub fn retry_count(&self) -> Option<u32> {
        self.active
            .as_ref()
            .or(self.paused.as_ref())
            .map(|p| p.retry_count)
    }

    /// The FormID of the preempted, resumable package, if one is held.
    #[must_use]
    pub fn paused_form_id(&self) -> Option<u32> {
        self.paused.as_ref().map(|package| package.form_id)
    }

    /// Applies the selection result for this evaluation. Decides start,
    /// continue, preempt, or resume. Returns `true` when the active package
    /// changed (started/resumed/preempted), for logging.
    pub fn on_select(&mut self, desired: Option<u32>) -> bool {
        let Some(desired) = desired else {
            // Schedule gap: pause whatever is running; nothing new starts.
            return self.preempt_active(None);
        };
        if let Some(active) = self.active
            && active.form_id == desired
            && matches!(
                self.phase,
                LifecyclePhase::Running | LifecyclePhase::AwaitingRetry
            )
        {
            // Same package already active -- continue without disturbing it.
            return false;
        }
        // Resume a paused package selected again -- restore its saved step.
        if let Some(paused) = self.paused
            && paused.form_id == desired
        {
            self.preempt_active(Some(paused.form_id));
            self.active = Some(paused);
            self.paused = None;
            self.phase = LifecyclePhase::Running;
            return true;
        }
        // A brand-new selection preempts whatever is active and starts fresh.
        self.preempt_active(Some(desired));
        self.active = Some(ActivePackage::fresh(desired));
        self.phase = LifecyclePhase::Running;
        true
    }

    /// Moves the current active package (if any) into the paused slot, unless
    /// it is the one about to become active again. Returns whether anything
    /// was paused.
    fn preempt_active(&mut self, becoming_active: Option<u32>) -> bool {
        let Some(active) = self.active.take() else {
            if matches!(
                self.phase,
                LifecyclePhase::Completed | LifecyclePhase::Failed
            ) && becoming_active.is_none()
            {
                self.phase = LifecyclePhase::Idle;
            }
            return false;
        };
        if Some(active.form_id) == becoming_active {
            // The active package is being re-selected as-is; caller replaces
            // `self.active`. Do not stash it as paused.
            return false;
        }
        // Only a genuinely running/paused/awaiting package is worth resuming.
        if matches!(
            self.phase,
            LifecyclePhase::Running | LifecyclePhase::AwaitingRetry
        ) {
            self.paused = Some(active);
            self.phase = if becoming_active.is_some() {
                self.phase
            } else {
                LifecyclePhase::Paused
            };
            true
        } else {
            // Completed/Failed active is discarded, not paused.
            self.phase = if becoming_active.is_some() {
                self.phase
            } else {
                LifecyclePhase::Idle
            };
            false
        }
    }

    /// Advances the active package's procedure step (a package family, #196+,
    /// calls this as it progresses). No-op unless running.
    pub fn advance_step(&mut self) {
        if self.phase == LifecyclePhase::Running
            && let Some(active) = self.active.as_mut()
        {
            active.step += 1;
            active.elapsed_seconds = 0.0;
        }
    }

    /// Advances time. Running accrues elapsed time; a package awaiting retry
    /// counts its backoff down and restarts (step 0) when it expires.
    pub fn tick(&mut self, dt_seconds: f32) {
        let dt = dt_seconds.max(0.0);
        match self.phase {
            LifecyclePhase::Running => {
                if let Some(active) = self.active.as_mut() {
                    active.elapsed_seconds += dt;
                }
            }
            LifecyclePhase::AwaitingRetry => {
                if let Some(active) = self.active.as_mut() {
                    active.backoff_remaining -= dt;
                    if active.backoff_remaining <= 0.0 {
                        // Retry is a *restart*: fresh step and elapsed, retry
                        // count preserved.
                        active.step = 0;
                        active.elapsed_seconds = 0.0;
                        active.backoff_remaining = 0.0;
                        self.phase = LifecyclePhase::Running;
                    }
                }
            }
            _ => {}
        }
    }

    /// The active package finished successfully.
    pub fn complete(&mut self) {
        if self.phase == LifecyclePhase::Running && self.active.is_some() {
            self.phase = LifecyclePhase::Completed;
        }
    }

    /// The active package failed. Schedules a retry with exponential backoff,
    /// or terminally fails once retries are exhausted. Returns the new phase.
    pub fn fail(&mut self) -> LifecyclePhase {
        if self.phase != LifecyclePhase::Running {
            return self.phase;
        }
        let Some(active) = self.active.as_mut() else {
            return self.phase;
        };
        if active.retry_count >= self.max_retries {
            self.phase = LifecyclePhase::Failed;
            return self.phase;
        }
        let exponent = active.retry_count;
        let backoff =
            (self.base_backoff_seconds * 2f32.powi(exponent as i32)).min(MAX_BACKOFF_SECONDS);
        active.retry_count += 1;
        active.backoff_remaining = backoff;
        self.phase = LifecyclePhase::AwaitingRetry;
        self.phase
    }

    /// Snapshots the active-or-paused package for persistence, or `None` when
    /// there is nothing worth resuming (idle/completed/terminally-failed).
    #[must_use]
    pub fn to_checkpoint(&self) -> Option<ActorPackageCheckpoint> {
        let package = match self.phase {
            LifecyclePhase::Running | LifecyclePhase::Paused | LifecyclePhase::AwaitingRetry => {
                self.active.as_ref().or(self.paused.as_ref())
            }
            LifecyclePhase::Idle | LifecyclePhase::Completed | LifecyclePhase::Failed => {
                // Even when terminal, a package preempted earlier may still be
                // resumable.
                self.paused.as_ref()
            }
        }?;
        Some(ActorPackageCheckpoint {
            package_form_id: package.form_id,
            procedure_index: package.step,
            elapsed_seconds: package.elapsed_seconds,
        })
    }

    /// Rebuilds a lifecycle from a persisted checkpoint, resuming as `Running`
    /// at the saved step and elapsed time. Deterministic resumption.
    #[must_use]
    pub fn from_checkpoint(checkpoint: ActorPackageCheckpoint) -> Self {
        Self {
            phase: LifecyclePhase::Running,
            active: Some(ActivePackage {
                form_id: checkpoint.package_form_id,
                step: checkpoint.procedure_index,
                elapsed_seconds: checkpoint.elapsed_seconds,
                retry_count: 0,
                backoff_remaining: 0.0,
            }),
            paused: None,
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_starts_a_package_running_at_step_zero() {
        let mut lifecycle = PackageLifecycle::new();
        assert_eq!(lifecycle.phase(), LifecyclePhase::Idle);
        assert!(lifecycle.on_select(Some(0x10)));
        assert_eq!(lifecycle.phase(), LifecyclePhase::Running);
        assert_eq!(lifecycle.active_form_id(), Some(0x10));
        assert_eq!(lifecycle.step(), Some(0));
    }

    #[test]
    fn re_selecting_the_running_package_continues_without_reset() {
        let mut lifecycle = PackageLifecycle::new();
        lifecycle.on_select(Some(0x10));
        lifecycle.advance_step();
        lifecycle.tick(2.0);
        // Same selection again -- must not restart.
        assert!(!lifecycle.on_select(Some(0x10)));
        assert_eq!(lifecycle.step(), Some(1));
        assert_eq!(lifecycle.elapsed_seconds(), Some(2.0));
    }

    #[test]
    fn tick_accrues_elapsed_time_while_running() {
        let mut lifecycle = PackageLifecycle::new();
        lifecycle.on_select(Some(0x10));
        lifecycle.tick(1.5);
        lifecycle.tick(0.5);
        assert_eq!(lifecycle.elapsed_seconds(), Some(2.0));
    }

    #[test]
    fn complete_moves_to_completed_and_stops_ticking() {
        let mut lifecycle = PackageLifecycle::new();
        lifecycle.on_select(Some(0x10));
        lifecycle.tick(1.0);
        lifecycle.complete();
        assert_eq!(lifecycle.phase(), LifecyclePhase::Completed);
        lifecycle.tick(5.0);
        assert_eq!(lifecycle.elapsed_seconds(), Some(1.0));
    }

    #[test]
    fn preempt_then_resume_restores_the_saved_step_not_a_restart() {
        let mut lifecycle = PackageLifecycle::new();
        // A running at step 2, 3s elapsed.
        lifecycle.on_select(Some(0xA));
        lifecycle.advance_step();
        lifecycle.advance_step();
        lifecycle.tick(3.0);
        assert_eq!(lifecycle.step(), Some(2));

        // B preempts A -> A paused with its progress retained.
        assert!(lifecycle.on_select(Some(0xB)));
        assert_eq!(lifecycle.active_form_id(), Some(0xB));
        assert_eq!(lifecycle.paused_form_id(), Some(0xA));
        assert_eq!(lifecycle.phase(), LifecyclePhase::Running);

        // B completes, then A is re-selected -> RESUME at step 2, elapsed 3.
        lifecycle.complete();
        assert!(lifecycle.on_select(Some(0xA)));
        assert_eq!(lifecycle.active_form_id(), Some(0xA));
        assert_eq!(lifecycle.phase(), LifecyclePhase::Running);
        assert_eq!(lifecycle.step(), Some(2), "resumed, not restarted");
        assert_eq!(lifecycle.elapsed_seconds(), Some(3.0));
        assert_eq!(lifecycle.paused_form_id(), None);
    }

    #[test]
    fn a_schedule_gap_pauses_the_running_package() {
        let mut lifecycle = PackageLifecycle::new();
        lifecycle.on_select(Some(0xA));
        lifecycle.advance_step();
        assert!(lifecycle.on_select(None));
        assert_eq!(lifecycle.phase(), LifecyclePhase::Paused);
        assert_eq!(lifecycle.paused_form_id(), Some(0xA));
        // Re-selecting it resumes at the saved step.
        lifecycle.on_select(Some(0xA));
        assert_eq!(lifecycle.step(), Some(1));
        assert_eq!(lifecycle.phase(), LifecyclePhase::Running);
    }

    #[test]
    fn fail_schedules_a_backoff_and_does_not_spin() {
        let mut lifecycle = PackageLifecycle::new().with_retry_policy(3, 1.0);
        lifecycle.on_select(Some(0x10));
        assert_eq!(lifecycle.fail(), LifecyclePhase::AwaitingRetry);
        // Backoff is 1s; a shorter tick must NOT restart (no spin).
        lifecycle.tick(0.5);
        assert_eq!(lifecycle.phase(), LifecyclePhase::AwaitingRetry);
        // Once the backoff elapses it restarts fresh (step 0).
        lifecycle.tick(0.6);
        assert_eq!(lifecycle.phase(), LifecyclePhase::Running);
        assert_eq!(lifecycle.step(), Some(0));
        assert_eq!(lifecycle.retry_count(), Some(1));
    }

    #[test]
    fn backoff_doubles_each_retry() {
        let mut lifecycle = PackageLifecycle::new().with_retry_policy(5, 1.0);
        lifecycle.on_select(Some(0x10));
        // First failure: 1s backoff.
        lifecycle.fail();
        lifecycle.tick(1.0);
        assert_eq!(lifecycle.phase(), LifecyclePhase::Running);
        // Second failure: 2s backoff -- 1s is not enough.
        lifecycle.fail();
        lifecycle.tick(1.0);
        assert_eq!(lifecycle.phase(), LifecyclePhase::AwaitingRetry);
        lifecycle.tick(1.1);
        assert_eq!(lifecycle.phase(), LifecyclePhase::Running);
        assert_eq!(lifecycle.retry_count(), Some(2));
    }

    #[test]
    fn retry_exhaustion_terminally_fails() {
        let mut lifecycle = PackageLifecycle::new().with_retry_policy(2, 0.1);
        lifecycle.on_select(Some(0x10));
        // Two retries allowed, then terminal.
        for _ in 0..2 {
            lifecycle.fail();
            lifecycle.tick(0.2); // burn the backoff, back to Running
            assert_eq!(lifecycle.phase(), LifecyclePhase::Running);
        }
        assert_eq!(lifecycle.fail(), LifecyclePhase::Failed);
        assert_eq!(lifecycle.phase(), LifecyclePhase::Failed);
        // A terminally failed package no longer spins.
        lifecycle.tick(10.0);
        assert_eq!(lifecycle.phase(), LifecyclePhase::Failed);
    }

    #[test]
    fn checkpoint_round_trip_resumes_at_the_right_step() {
        let mut lifecycle = PackageLifecycle::new();
        lifecycle.on_select(Some(0x1234));
        lifecycle.advance_step();
        lifecycle.advance_step();
        lifecycle.advance_step();
        lifecycle.tick(4.5);
        let checkpoint = lifecycle
            .to_checkpoint()
            .expect("running package snapshots");
        assert_eq!(checkpoint.package_form_id, 0x1234);
        assert_eq!(checkpoint.procedure_index, 3);
        assert_eq!(checkpoint.elapsed_seconds, 4.5);

        let resumed = PackageLifecycle::from_checkpoint(checkpoint);
        assert_eq!(resumed.phase(), LifecyclePhase::Running);
        assert_eq!(resumed.active_form_id(), Some(0x1234));
        assert_eq!(resumed.step(), Some(3), "resumed at the saved step");
        assert_eq!(resumed.elapsed_seconds(), Some(4.5));
    }

    #[test]
    fn idle_and_completed_snapshot_to_nothing() {
        let mut lifecycle = PackageLifecycle::new();
        assert_eq!(lifecycle.to_checkpoint(), None);
        lifecycle.on_select(Some(0x10));
        lifecycle.complete();
        assert_eq!(lifecycle.to_checkpoint(), None);
    }

    #[test]
    fn paused_package_snapshots_for_resume() {
        let mut lifecycle = PackageLifecycle::new();
        lifecycle.on_select(Some(0xA));
        lifecycle.advance_step();
        lifecycle.on_select(None); // pause A
        let checkpoint = lifecycle.to_checkpoint().expect("paused package snapshots");
        assert_eq!(checkpoint.package_form_id, 0xA);
        assert_eq!(checkpoint.procedure_index, 1);
    }
}
