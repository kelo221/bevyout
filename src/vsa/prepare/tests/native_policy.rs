use super::*;

#[test]
fn requested_worker_count_is_bounded_by_work_and_never_zero() {
    assert_eq!(native_worker_count_with_host(Some(0), 8, 24), 1);
    assert_eq!(native_worker_count_with_host(Some(8), 3, 24), 3);
    assert_eq!(native_worker_count_with_host(Some(8), 0, 24), 1);
}

#[test]
fn omitted_worker_count_uses_every_host_processor() {
    assert_eq!(native_worker_count_with_host(None, 24, 24), 24);
    assert_eq!(native_worker_count_with_host(None, 3, 24), 3);
}

#[test]
fn summary_and_order_do_not_depend_on_completion_order() {
    let outcomes = vec![
        NativeJobOutcome {
            index: 2,
            model: "c.nif".into(),
            status: NativeJobStatus::Failed,
            stage: "parse".into(),
            error: Some("bad data".into()),
        },
        NativeJobOutcome {
            index: 0,
            model: "a.nif".into(),
            status: NativeJobStatus::Converted,
            stage: "complete".into(),
            error: None,
        },
        NativeJobOutcome {
            index: 1,
            model: "actors/assembled".into(),
            status: NativeJobStatus::Unsupported,
            stage: "input".into(),
            error: Some("actor assembly".into()),
        },
    ];
    assert_eq!(
        summarize_native_jobs(&outcomes),
        NativeBatchSummary {
            total: 3,
            converted: 1,
            failed: 1,
            unsupported: 1,
        }
    );
    assert_eq!(
        sorted_native_outcomes(&outcomes)
            .into_iter()
            .map(|outcome| outcome.index)
            .collect::<Vec<_>>(),
        vec![0, 1, 2]
    );
}
