use std::io::{self, Write};
use std::time::{Duration, Instant};

use super::model::{ProgressMode, ProgressSnapshot, WorkEstimate};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RenderKind {
    Tty,
    Plain,
    Off,
}

pub(crate) struct ProgressRenderer {
    kind: RenderKind,
    writer: Box<dyn Write + Send>,
    interval: Duration,
    last_render: Option<Instant>,
}

impl ProgressRenderer {
    pub(crate) fn new<W: Write + Send + 'static>(
        mode: ProgressMode,
        writer: W,
        interactive: bool,
        interval: Duration,
    ) -> Self {
        let kind = match mode {
            ProgressMode::Auto if interactive => RenderKind::Tty,
            ProgressMode::Auto => RenderKind::Plain,
            ProgressMode::Tty => RenderKind::Tty,
            ProgressMode::Plain => RenderKind::Plain,
            ProgressMode::Off => RenderKind::Off,
        };
        Self {
            kind,
            writer: Box::new(writer),
            interval,
            last_render: None,
        }
    }

    pub(crate) fn render(&mut self, snapshot: &ProgressSnapshot, force: bool) -> io::Result<()> {
        if self.kind == RenderKind::Off {
            return Ok(());
        }
        let now = Instant::now();
        if !force
            && self
                .last_render
                .is_some_and(|last| now.duration_since(last) < self.interval)
        {
            return Ok(());
        }
        self.last_render = Some(now);
        let line = format_snapshot(snapshot);
        match self.kind {
            RenderKind::Tty => {
                if snapshot.finished {
                    write!(self.writer, "\r{line}\n")?;
                } else {
                    write!(self.writer, "\r{line}")?;
                }
            }
            RenderKind::Plain => writeln!(self.writer, "{line}")?,
            RenderKind::Off => unreachable!(),
        }
        self.writer.flush()
    }
}

fn format_snapshot(snapshot: &ProgressSnapshot) -> String {
    let mut parts = Vec::new();
    let mut operation_contains_backend = false;
    if let Some(operation) = &snapshot.operation {
        let rendered_operation = if matches!(
            operation.as_str(),
            "Bake" | "GPU bake" | "CPU bake" | "Solari bake"
        ) {
            snapshot
                .backend
                .as_ref()
                .map_or_else(|| operation.clone(), |backend| format!("{backend} bake"))
        } else {
            operation.clone()
        };
        operation_contains_backend = snapshot.backend.as_ref().is_some_and(|backend| {
            rendered_operation
                .to_ascii_lowercase()
                .contains(&backend.to_ascii_lowercase())
        });
        parts.push(rendered_operation);
    }

    let phase_start = snapshot.phases.len().saturating_sub(2);
    for phase in snapshot.phases.iter().skip(phase_start) {
        parts.push(format_work(&phase.name, phase.work));
    }
    if parts.len() == 1 && snapshot.phases.is_empty() {
        parts.push(format_work("work", snapshot.operation_work));
    }
    if snapshot.cache_hits != 0 || snapshot.cache_misses != 0 {
        parts.push(format!(
            "cache {} hit / {} miss",
            snapshot.cache_hits, snapshot.cache_misses
        ));
    }
    if snapshot.jobs_completed != 0 || snapshot.jobs_failed != 0 {
        parts.push(format!(
            "jobs {} done / {} failed",
            snapshot.jobs_completed, snapshot.jobs_failed
        ));
    }
    if let Some(backend) = &snapshot.backend
        && !operation_contains_backend
    {
        parts.push(backend.clone());
    }
    if let Some(samples) = snapshot.samples {
        parts.push(format!(
            "{samples} sample{}",
            if samples == 1 { "" } else { "s" }
        ));
    }
    if let Some(bounces) = snapshot.bounces {
        parts.push(format!(
            "{bounces} bounce{}",
            if bounces == 1 { "" } else { "s" }
        ));
    }
    if let Some(eta_ms) = snapshot.current_timing.eta_ms {
        parts.push(format!("eta {}", format_duration(eta_ms)));
    }
    if let Some(message) = &snapshot.message {
        parts.push(message.clone());
    }
    if snapshot.finished && snapshot.success == Some(false) {
        parts.push("failed".into());
    }
    parts.join(" | ")
}

fn format_work(name: &str, work: WorkEstimate) -> String {
    match work.total {
        Some(total) => format!("{name} {}/{}", work.completed, total),
        None => format!("{name} {}", work.completed),
    }
}

fn format_duration(milliseconds: u64) -> String {
    let total_seconds = milliseconds.div_ceil(1_000);
    let hours = total_seconds / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;
    if hours != 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

#[cfg(test)]
pub(crate) fn format_snapshot_for_test(snapshot: &ProgressSnapshot) -> String {
    format_snapshot(snapshot)
}
