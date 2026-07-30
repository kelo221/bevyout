use super::*;

#[test]
fn empty_window_has_no_numeric_statistics() {
    let summary = summarize_frame_window(&[], None, 10, 16.0);
    assert_eq!(summary.sample_count, 0);
    assert_eq!(summary.average_ms, None);
    assert_eq!(summary.p95_ms, None);
    assert_eq!(summary.max_ms, None);
}

#[test]
fn invalid_samples_are_excluded() {
    let summary = summarize_frame_window(
        &[
            FrameSample {
                sample: 0,
                frame_time_ms: f64::NAN,
            },
            FrameSample {
                sample: 1,
                frame_time_ms: -1.0,
            },
            FrameSample {
                sample: 2,
                frame_time_ms: 12.0,
            },
        ],
        None,
        10,
        16.0,
    );
    assert_eq!(
        summary.samples,
        vec![FrameSample {
            sample: 2,
            frame_time_ms: 12.0
        }]
    );
}

#[test]
fn nearest_rank_percentiles_are_deterministic_for_short_windows() {
    let samples = [8.0, 16.0, 20.0, 40.0]
        .into_iter()
        .enumerate()
        .map(|(sample, frame_time_ms)| FrameSample {
            sample: sample as u64,
            frame_time_ms,
        })
        .collect::<Vec<_>>();
    let summary = summarize_frame_window(&samples, None, 10, 16.0);
    assert_eq!(summary.p50_ms, Some(16.0));
    assert_eq!(summary.p95_ms, Some(40.0));
    assert_eq!(summary.p99_ms, Some(40.0));
    assert_eq!(summary.over_budget_count, 2);
}
