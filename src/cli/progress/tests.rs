use super::render::format_snapshot_for_test;
use super::*;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct SharedBuffer(Arc<Mutex<Vec<u8>>>);

impl Write for SharedBuffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn output(buffer: &Arc<Mutex<Vec<u8>>>) -> String {
    String::from_utf8(buffer.lock().unwrap().clone()).unwrap()
}

#[test]
fn aggregator_keeps_nested_phase_totals_and_cache_counters() {
    let aggregate = ProgressAggregator::new();
    aggregate.record(ProgressEvent::Started {
        operation: "prepare".into(),
        total: Some(2),
    });
    aggregate.record(ProgressEvent::PhaseStarted {
        phase: "cell Tenpenny01".into(),
        total: Some(2),
    });
    aggregate.record(ProgressEvent::PhaseStarted {
        phase: "cell Tenpenny01/native jobs".into(),
        total: Some(3),
    });
    let snapshot = aggregate.record(ProgressEvent::UnitCompleted {
        completed: 2,
        total: Some(3),
        cache_hit: Some(true),
    });

    assert_eq!(snapshot.operation_work.total, Some(2));
    assert_eq!(snapshot.phases.len(), 2);
    assert_eq!(
        snapshot.current_phase.as_deref(),
        Some("cell Tenpenny01/native jobs")
    );
    assert_eq!(
        snapshot.current_work,
        WorkEstimate {
            completed: 2,
            total: Some(3)
        }
    );
    assert_eq!(snapshot.cache_hits, 1);
    assert_eq!(snapshot.cache_misses, 0);
}

#[test]
fn aggregator_supports_unknown_totals_and_parallel_unit_completion() {
    let aggregate = ProgressAggregator::new();
    aggregate.record(ProgressEvent::Started {
        operation: "bake".into(),
        total: None,
    });
    aggregate.record(ProgressEvent::PhaseStarted {
        phase: "tiles".into(),
        total: None,
    });

    let reporter = ProgressReporter {
        aggregate: aggregate.clone(),
        renderer: Arc::new(Mutex::new(super::render::ProgressRenderer::new(
            ProgressMode::Off,
            io::sink(),
            false,
            Duration::ZERO,
        ))),
        current_phase: Arc::new(Mutex::new(None)),
        cpu_warning_emitted: Arc::new(std::sync::atomic::AtomicBool::new(false)),
    };
    thread::scope(|scope| {
        for index in 0..8 {
            let reporter = reporter.clone();
            scope.spawn(move || reporter.unit_completed(None, Some(index % 2 == 0)));
        }
    });

    let snapshot = aggregate.snapshot();
    assert_eq!(snapshot.current_work.completed, 8);
    assert_eq!(snapshot.current_work.total, None);
    assert_eq!(snapshot.cache_hits, 4);
    assert_eq!(snapshot.cache_misses, 4);
}

#[test]
fn parallel_scopes_keep_nested_completions_with_their_own_cells() {
    let reporter = ProgressReporter::with_writer(ProgressMode::Off, io::sink(), false);
    reporter.started("prepare", Some(4));

    thread::scope(|scope| {
        for cell in 0..4 {
            let reporter = reporter.clone();
            scope.spawn(move || {
                let worker = reporter.scoped();
                worker.phase_started(format!("cell {cell}/native jobs"), Some(2));
                worker.unit_completed(Some(2), None);
                worker.unit_completed(Some(2), None);
            });
        }
    });

    let snapshot = reporter.snapshot();
    assert_eq!(snapshot.phases.len(), 4);
    assert!(snapshot.phases.iter().all(|phase| phase.work
        == WorkEstimate {
            completed: 2,
            total: Some(2)
        }));
}

#[test]
fn failed_cell_scope_retains_job_failure_and_failed_operation_state() {
    let reporter = ProgressReporter::with_writer(ProgressMode::Off, io::sink(), false);
    let worker = reporter.scoped();
    reporter.started("prepare", Some(1));
    worker.phase_started("cell BrokenCell/native jobs", Some(1));
    worker.job_completed(false);
    worker.unit_completed(Some(1), None);
    reporter.unit_completed_in_phase("cell", Some(1), None);
    reporter.finished(false);

    let snapshot = reporter.snapshot();
    assert_eq!(snapshot.jobs_completed, 0);
    assert_eq!(snapshot.jobs_failed, 1);
    assert_eq!(snapshot.cache_hits, 0);
    assert_eq!(snapshot.cache_misses, 0);
    assert_eq!(snapshot.success, Some(false));
    assert!(snapshot.finished);
}

#[test]
fn reporter_tty_updates_one_line_and_finishes_with_a_newline() {
    let bytes = Arc::new(Mutex::new(Vec::new()));
    let reporter =
        ProgressReporter::with_writer(ProgressMode::Tty, SharedBuffer(bytes.clone()), false);
    reporter.started("Solari bake Tenpenny01", None);
    reporter.phase_started("primitive", Some(96));
    reporter.unit_completed(Some(96), None);
    reporter.finished(true);

    let text = output(&bytes);
    assert!(text.contains('\r'));
    assert_eq!(text.matches('\n').count(), 1);
    assert!(!text.contains('\u{1b}'));
}

#[test]
fn reporter_plain_never_uses_carriage_returns_or_ansi() {
    let bytes = Arc::new(Mutex::new(Vec::new()));
    let reporter = ProgressReporter::with_writer_and_interval(
        ProgressMode::Plain,
        SharedBuffer(bytes.clone()),
        true,
        Duration::ZERO,
    );
    reporter.started("CPU bake", Some(1));
    reporter.phase_started("tile", Some(8));
    reporter.unit_completed(Some(8), Some(false));
    reporter.finished(true);

    let text = output(&bytes);
    assert!(!text.contains('\r'));
    assert!(!text.contains('\u{1b}'));
    assert!(text.lines().count() >= 3);
}

#[test]
fn auto_uses_plain_lines_when_stderr_is_not_interactive() {
    let bytes = Arc::new(Mutex::new(Vec::new()));
    let reporter = ProgressReporter::with_writer_and_interval(
        ProgressMode::Auto,
        SharedBuffer(bytes.clone()),
        false,
        Duration::ZERO,
    );
    reporter.started("prepare", Some(1));
    reporter.finished(true);

    assert!(!output(&bytes).contains('\r'));
}

#[test]
fn off_mode_preserves_a_silent_progress_stream() {
    let bytes = Arc::new(Mutex::new(Vec::new()));
    let reporter =
        ProgressReporter::with_writer(ProgressMode::Off, SharedBuffer(bytes.clone()), false);
    reporter.started("prepare", Some(1));
    reporter.phase_started("cell", Some(1));
    reporter.unit_completed(Some(1), None);
    reporter.finished(true);

    assert!(output(&bytes).is_empty());
}

#[test]
fn snapshot_formats_backend_sampling_and_cache_context() {
    let reporter = ProgressReporter {
        aggregate: ProgressAggregator::new(),
        renderer: Arc::new(Mutex::new(super::render::ProgressRenderer::new(
            ProgressMode::Off,
            io::sink(),
            false,
            Duration::ZERO,
        ))),
        current_phase: Arc::new(Mutex::new(None)),
        cpu_warning_emitted: Arc::new(std::sync::atomic::AtomicBool::new(false)),
    };
    reporter.started("Solari bake Tenpenny01", None);
    reporter.set_backend("Solari");
    reporter.set_sampling(Some(1), Some(1));
    reporter.phase_started("tile", Some(8));
    reporter.unit_completed(Some(8), Some(true));
    reporter.unit_completed(Some(8), Some(false));

    let text = format_snapshot_for_test(&reporter.snapshot());
    assert!(text.contains("Solari bake Tenpenny01"));
    assert!(text.contains("tile 2/8"));
    assert!(text.contains("cache 1 hit / 1 miss"));
    assert!(text.contains("1 sample"));
    assert!(text.contains("1 bounce"));
}

#[test]
fn bake_progress_uses_the_resolved_gpu_or_cpu_label() {
    let reporter = ProgressReporter::with_writer_and_interval(
        ProgressMode::Off,
        io::sink(),
        false,
        Duration::ZERO,
    );
    reporter.started("Bake", None);
    reporter.set_backend("GPU");
    assert!(format_snapshot_for_test(&reporter.snapshot()).contains("GPU bake"));

    reporter.set_backend("CPU");
    let text = format_snapshot_for_test(&reporter.snapshot());
    assert!(text.contains("CPU bake"));
    assert!(!text.contains("Auto"));

    reporter.started("GPU bake", None);
    reporter.set_backend("CPU");
    assert!(format_snapshot_for_test(&reporter.snapshot()).contains("CPU bake"));
}

#[test]
fn eta_appears_only_after_multiple_known_units() {
    let reporter = ProgressReporter::with_writer_and_interval(
        ProgressMode::Off,
        io::sink(),
        false,
        Duration::ZERO,
    );
    reporter.started("CPU bake", None);
    reporter.phase_started("tiles", Some(4));
    assert_eq!(reporter.snapshot().current_timing.eta_ms, None);
    reporter.unit_completed(Some(4), None);
    assert_eq!(reporter.snapshot().current_timing.eta_ms, None);
    reporter.unit_completed(Some(4), None);
    assert!(reporter.snapshot().current_timing.eta_ms.is_some());
    assert!(format_snapshot_for_test(&reporter.snapshot()).contains("eta "));
}

#[test]
fn messages_are_not_lost_to_render_throttling() {
    let bytes = Arc::new(Mutex::new(Vec::new()));
    let reporter = ProgressReporter::with_writer_and_interval(
        ProgressMode::Plain,
        SharedBuffer(bytes.clone()),
        false,
        Duration::from_secs(60),
    );
    reporter.started("Auto bake", None);
    reporter.message("GPU bake failed; retrying on CPU");

    assert!(output(&bytes).contains("retrying on CPU"));
}

#[test]
fn cpu_warning_is_emitted_once() {
    let bytes = Arc::new(Mutex::new(Vec::new()));
    let reporter = ProgressReporter::with_writer_and_interval(
        ProgressMode::Plain,
        SharedBuffer(bytes.clone()),
        false,
        Duration::ZERO,
    );
    reporter.started("CPU bake", None);
    reporter.set_backend("CPU");
    reporter.warn_cpu_bake();
    reporter.warn_cpu_bake();
    reporter.phase_started("tiles", Some(1));
    reporter.unit_completed(Some(1), None);

    let text = output(&bytes);
    assert_eq!(text.matches("warning: CPU bake").count(), 1);
    assert!(text.contains("may not saturate all CPU cores"));
}
