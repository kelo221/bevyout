//! Compatibility report tooling slice (issue #37).
//!
//! Generates a deterministic, machine-readable compatibility report plus a
//! human-readable summary for a single Fallout plugin file, driven by the
//! same OpenMW-derived ESM4 reader the Fallout cell slice uses
//! (`src/vsa/openmw_esm4/`). Reports derived from real (licensed) plugin
//! data are written under `.bevyout/` (gitignored); nothing here commits
//! Bethesda-derived output.

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

mod generator;
mod registry;
mod schema;

pub(crate) use generator::generate_report;

use crate::cli::ReportArgs;

/// Default directory (relative to the working directory) that reports are
/// written under when `--out-dir` is not given. Always inside the
/// gitignored `.bevyout/` tree (F37.6).
const DEFAULT_OUT_DIR: &str = ".bevyout/reports";

pub fn report(args: ReportArgs) -> Result<()> {
    let plugin = args.plugin.unwrap_or_else(|| PathBuf::from("Fallout3.esm"));
    let plugin_path = if plugin.is_absolute() {
        plugin
    } else if let Some(root) = &args.game_root {
        root.join("Data").join(&plugin)
    } else {
        plugin
    };
    let plugin_path = fs::canonicalize(&plugin_path)
        .with_context(|| format!("plugin does not exist: {}", plugin_path.display()))?;
    let plugin_name = plugin_path
        .file_name()
        .and_then(|name| name.to_str())
        .context("plugin path has no UTF-8 filename")?
        .to_string();
    let bytes = fs::read(&plugin_path)
        .with_context(|| format!("failed to read plugin {}", plugin_path.display()))?;

    let compatibility_report = generate_report(&plugin_name, &bytes)?;

    let out_dir = args
        .out_dir
        .unwrap_or_else(|| PathBuf::from(DEFAULT_OUT_DIR));
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("failed to create report directory {}", out_dir.display()))?;
    let stem = plugin_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("report");
    let json_path = out_dir.join(format!("{stem}.report.json"));
    let summary_path = out_dir.join(format!("{stem}.summary.txt"));
    fs::write(&json_path, compatibility_report.to_json())
        .with_context(|| format!("failed to write {}", json_path.display()))?;
    let summary = compatibility_report.summary();
    fs::write(&summary_path, &summary)
        .with_context(|| format!("failed to write {}", summary_path.display()))?;

    print!("{summary}");
    println!("wrote {}", json_path.display());
    println!("wrote {}", summary_path.display());
    Ok(())
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
