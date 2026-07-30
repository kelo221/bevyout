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
#[path = "tests/native_policy.rs"]
mod tests;
