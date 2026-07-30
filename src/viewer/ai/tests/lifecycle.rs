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
