use super::model::GcReport;
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub(crate) fn print_report(report: &GcReport) {
    println!(
        "cache gc: mode {} live {} examined {} candidates {} logical {} allocated {}",
        if report.dry_run { "dry-run" } else { "sweep" },
        report.live_file_count,
        report.examined_file_count,
        report.candidate_file_count,
        report.candidate_logical_bytes,
        report.candidate_allocated_bytes
    );
    for (reason, summary) in &report.candidates_by_reason {
        println!(
            "cache gc class: {reason} files {} logical {} allocated {}",
            summary.file_count, summary.logical_bytes, summary.allocated_bytes
        );
    }
    if !report.dry_run {
        println!(
            "cache gc deleted: files {} logical {} allocated {}",
            report.deleted_file_count, report.deleted_logical_bytes, report.deleted_allocated_bytes
        );
    }
}

pub(crate) fn write_json(path: &Path, report: &GcReport) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(report)?;
    fs::write(path, bytes)
        .with_context(|| format!("could not write cache GC report {}", path.display()))?;
    Ok(())
}
