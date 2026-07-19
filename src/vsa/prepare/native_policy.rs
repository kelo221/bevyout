//! Deterministic, dependency-free policy for native NIF batch execution.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NativeJobStatus {
    Converted,
    Failed,
    #[allow(dead_code)]
    Unsupported,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NativeJobOutcome {
    pub(crate) index: usize,
    pub(crate) model: String,
    pub(crate) status: NativeJobStatus,
    pub(crate) stage: String,
    pub(crate) error: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct NativeBatchSummary {
    pub(crate) total: usize,
    pub(crate) converted: usize,
    pub(crate) failed: usize,
    pub(crate) unsupported: usize,
}

impl NativeBatchSummary {
    pub(crate) fn line(self) -> String {
        format!(
            "native conversion: completed {}/{} ok={} failed={} unsupported={}",
            self.total, self.total, self.converted, self.failed, self.unsupported
        )
    }

    pub(crate) fn has_failures(self) -> bool {
        self.failed != 0 || self.unsupported != 0
    }
}

pub(crate) fn native_worker_count(requested: Option<usize>, total: usize) -> usize {
    let host_workers = std::thread::available_parallelism()
        .map(std::num::NonZeroUsize::get)
        .unwrap_or(1);
    native_worker_count_with_host(requested, total, host_workers)
}

pub(crate) fn native_worker_count_with_host(
    requested: Option<usize>,
    total: usize,
    host_workers: usize,
) -> usize {
    requested.unwrap_or(host_workers).max(1).min(total.max(1))
}

pub(crate) fn summarize_native_jobs(outcomes: &[NativeJobOutcome]) -> NativeBatchSummary {
    let mut summary = NativeBatchSummary {
        total: outcomes.len(),
        ..Default::default()
    };
    for outcome in outcomes {
        match outcome.status {
            NativeJobStatus::Converted => summary.converted += 1,
            NativeJobStatus::Failed => summary.failed += 1,
            NativeJobStatus::Unsupported => summary.unsupported += 1,
        }
    }
    summary
}

pub(crate) fn sorted_native_outcomes(outcomes: &[NativeJobOutcome]) -> Vec<NativeJobOutcome> {
    let mut sorted = outcomes.to_vec();
    sorted.sort_by(|left, right| {
        left.index
            .cmp(&right.index)
            .then_with(|| left.model.cmp(&right.model))
    });
    sorted
}

#[cfg(test)]
mod tests {
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
}
