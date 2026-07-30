use super::*;

// T52.1: resident Ready -> Instant.
#[test]
fn ready_residency_is_an_instant_swap() {
    assert_eq!(swap_decision(true, Residency::Ready), SwapDecision::Instant);
}

// T52.1: Loading -> Fallback.
#[test]
fn loading_residency_is_a_fallback_swap() {
    assert_eq!(
        swap_decision(true, Residency::Loading),
        SwapDecision::Fallback
    );
}

// T52.1: Absent + manifest-exists -> Fallback.
#[test]
fn absent_with_manifest_on_disk_is_a_fallback_swap() {
    assert_eq!(
        swap_decision(true, Residency::Absent),
        SwapDecision::Fallback
    );
}

// T52.1: Absent + no manifest -> Fallback, whose failure -> ReturnToSource.
#[test]
fn absent_with_no_manifest_is_a_fallback_swap_that_fails_to_return_to_source() {
    assert_eq!(
        swap_decision(false, Residency::Absent),
        SwapDecision::Fallback
    );
    assert_eq!(fallback_outcome(false), FallbackOutcome::ReturnToSource);
}

#[test]
fn a_successful_fallback_load_proceeds() {
    assert_eq!(fallback_outcome(true), FallbackOutcome::Proceed);
}

#[test]
fn collider_build_queue_drains_at_most_the_budget_per_call() {
    let mut queue = ColliderBuildQueue::new(0..10);
    assert_eq!(queue.len(), 10);
    let first = queue.drain_budget(4);
    assert_eq!(first, vec![0, 1, 2, 3]);
    assert_eq!(queue.len(), 6);
    let second = queue.drain_budget(100);
    assert_eq!(second, vec![4, 5, 6, 7, 8, 9]);
    assert!(queue.is_empty());
}

#[test]
fn collider_build_queue_drain_budget_of_zero_is_a_no_op() {
    let mut queue = ColliderBuildQueue::new([1, 2, 3]);
    assert!(queue.drain_budget(0).is_empty());
    assert_eq!(queue.len(), 3);
}

// T59.1: no fallback in flight -> every event is ignored.
#[test]
fn idle_state_ignores_every_event() {
    for event in [
        FallbackEvent::DestinationReady,
        FallbackEvent::ParseFailed,
        FallbackEvent::PlayerCancelled,
        FallbackEvent::SupersedingRequest,
    ] {
        assert_eq!(
            fallback_lifecycle_outcome(FallbackState::Idle, event),
            FallbackLifecycleOutcome::Ignore
        );
    }
}

// T59.1: an in-flight fallback whose destination becomes ready proceeds.
#[test]
fn in_flight_destination_ready_proceeds() {
    assert_eq!(
        fallback_lifecycle_outcome(FallbackState::InFlight, FallbackEvent::DestinationReady),
        FallbackLifecycleOutcome::Proceed
    );
}

// T59.1: an in-flight fallback whose parse fails returns to source.
#[test]
fn in_flight_parse_failed_returns_to_source() {
    assert_eq!(
        fallback_lifecycle_outcome(FallbackState::InFlight, FallbackEvent::ParseFailed),
        FallbackLifecycleOutcome::ReturnToSource
    );
}

// T59.1: an in-flight fallback the player cancels (Esc) cancels cleanly.
#[test]
fn in_flight_player_cancelled_cancels() {
    assert_eq!(
        fallback_lifecycle_outcome(FallbackState::InFlight, FallbackEvent::PlayerCancelled),
        FallbackLifecycleOutcome::Cancel
    );
}

// T59.1: a superseding travel request cancels the old fallback and
// starts the new one -- the caller (`swap.rs`) implements "start the
// new one" by re-running `swap_decision` for the superseding request
// once this table says `Supersede`.
#[test]
fn in_flight_superseding_request_supersedes() {
    assert_eq!(
        fallback_lifecycle_outcome(FallbackState::InFlight, FallbackEvent::SupersedingRequest),
        FallbackLifecycleOutcome::Supersede
    );
}

// T59.2: fade progress is monotonic and clamped to [0, 1].
#[test]
fn fade_progress_is_monotonic_and_clamped() {
    assert_eq!(fade_progress(-1.0, 0.25), 0.0);
    assert_eq!(fade_progress(0.0, 0.25), 0.0);
    let quarter = fade_progress(0.0625, 0.25);
    let half = fade_progress(0.125, 0.25);
    let three_quarter = fade_progress(0.1875, 0.25);
    assert!(quarter < half);
    assert!(half < three_quarter);
    assert!(three_quarter < 1.0);
    assert_eq!(fade_progress(0.25, 0.25), 1.0);
    assert_eq!(fade_progress(10.0, 0.25), 1.0);
}

// T59.2: a non-positive duration never divides by zero -- it is
// instantaneous instead.
#[test]
fn fade_progress_treats_non_positive_duration_as_instantaneous() {
    assert_eq!(fade_progress(0.0, 0.0), 1.0);
    assert_eq!(fade_progress(0.1, -1.0), 1.0);
}

// T59.2: fade-in reaches 0 at the start and max_alpha at the duration.
#[test]
fn fade_in_alpha_spans_zero_to_max() {
    assert_eq!(fade_in_alpha(0.0, 0.25, 0.85), 0.0);
    assert_eq!(fade_in_alpha(0.25, 0.25, 0.85), 0.85);
}

// T59.2: fade-out is the symmetric mirror of fade-in -- at any elapsed
// time `t`, `fade_out_alpha(t)` equals `fade_in_alpha(duration - t)`.
#[test]
fn fade_out_alpha_is_the_symmetric_mirror_of_fade_in_alpha() {
    let duration = 0.25;
    let max_alpha = 0.85;
    for elapsed in [0.0, 0.05, 0.125, 0.2, 0.25] {
        let out = fade_out_alpha(elapsed, duration, max_alpha);
        let mirrored_in = fade_in_alpha(duration - elapsed, duration, max_alpha);
        assert!(
            (out - mirrored_in).abs() < 1e-6,
            "fade_out_alpha({elapsed}) = {out} != fade_in_alpha({}) = {mirrored_in}",
            duration - elapsed
        );
    }
    assert_eq!(fade_out_alpha(0.0, duration, max_alpha), max_alpha);
    assert_eq!(fade_out_alpha(duration, duration, max_alpha), 0.0);
}

#[test]
fn collider_work_partitions_static_before_dynamic() {
    assert_eq!(
        partition_collider_indices([(0, false), (1, true), (2, false), (3, true)]),
        (vec![0, 2], vec![1, 3])
    );
}

#[test]
fn collider_phase_does_not_report_ready_while_dynamic_work_is_pending() {
    assert_eq!(
        next_collider_build_phase(false, true, true),
        ColliderBuildPhase::Dynamic
    );
    assert_eq!(
        next_collider_build_phase(false, false, true),
        ColliderBuildPhase::Ready
    );
    assert_eq!(
        next_collider_build_phase(true, true, false),
        ColliderBuildPhase::Static
    );
}
