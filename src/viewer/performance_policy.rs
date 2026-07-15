//! Pure frame-window selection and statistics for runtime performance probes.
//!
//! Keep this module independent of Bevy so the executable feature suite can
//! pin probe semantics without constructing an app or renderer.

use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub(crate) struct FrameSample {
    pub(crate) sample: u64,
    pub(crate) frame_time_ms: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct FrameProbeSummary {
    pub(crate) sample_count: usize,
    pub(crate) first_sample: Option<u64>,
    pub(crate) last_sample: Option<u64>,
    pub(crate) budget_ms: f64,
    pub(crate) over_budget_count: usize,
    pub(crate) min_ms: Option<f64>,
    pub(crate) average_ms: Option<f64>,
    pub(crate) p50_ms: Option<f64>,
    pub(crate) p95_ms: Option<f64>,
    pub(crate) p99_ms: Option<f64>,
    pub(crate) max_ms: Option<f64>,
    pub(crate) samples: Vec<FrameSample>,
}

/// Select samples strictly newer than `after_sample`, then retain at most the
/// newest `latest_limit` entries. Percentiles use the nearest-rank method.
pub(crate) fn summarize_frame_window(
    samples: &[FrameSample],
    after_sample: Option<u64>,
    latest_limit: usize,
    budget_ms: f64,
) -> FrameProbeSummary {
    let eligible = samples
        .iter()
        .copied()
        .filter(|sample| {
            sample.frame_time_ms.is_finite()
                && sample.frame_time_ms >= 0.0
                && after_sample.is_none_or(|marker| sample.sample > marker)
        })
        .collect::<Vec<_>>();
    let keep_from = eligible.len().saturating_sub(latest_limit);
    let selected = eligible[keep_from..].to_vec();
    let mut sorted_times = selected
        .iter()
        .map(|sample| sample.frame_time_ms)
        .collect::<Vec<_>>();
    sorted_times.sort_by(f64::total_cmp);

    let average_ms = (!sorted_times.is_empty())
        .then(|| sorted_times.iter().sum::<f64>() / sorted_times.len() as f64);
    FrameProbeSummary {
        sample_count: selected.len(),
        first_sample: selected.first().map(|sample| sample.sample),
        last_sample: selected.last().map(|sample| sample.sample),
        budget_ms,
        over_budget_count: selected
            .iter()
            .filter(|sample| sample.frame_time_ms > budget_ms)
            .count(),
        min_ms: sorted_times.first().copied(),
        average_ms,
        p50_ms: nearest_rank(&sorted_times, 0.50),
        p95_ms: nearest_rank(&sorted_times, 0.95),
        p99_ms: nearest_rank(&sorted_times, 0.99),
        max_ms: sorted_times.last().copied(),
        samples: selected,
    }
}

fn nearest_rank(sorted: &[f64], percentile: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let rank = (percentile * sorted.len() as f64).ceil() as usize;
    sorted.get(rank.saturating_sub(1)).copied()
}

#[cfg(test)]
mod tests {
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
}
