//! Deterministic report for the selected native exterior conversion corpus.
//! The command consumes reports emitted by offline preparation; it never adds
//! a runtime converter abstraction or invokes conversion while viewing or
//! streaming.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::cli::ExteriorConversionReportArgs;

pub(crate) const EXTERIOR_CONVERSION_REPORT_REVISION: &str = "exterior-conversion-report-v1";

#[derive(Debug, Clone, Deserialize)]
struct CorpusInput {
    selected_pipeline: String,
    #[serde(default)]
    corpus_name: Option<String>,
    rows: Vec<CorpusRowInput>,
}

#[derive(Debug, Clone, Deserialize)]
struct CorpusRowInput {
    source: String,
    source_path: Option<PathBuf>,
    native_report: Option<PathBuf>,
    native_output: Option<PathBuf>,
    native_cache_dir: Option<PathBuf>,
    #[serde(default)]
    native_cold_seconds: Option<f64>,
    #[serde(default)]
    native_warm_seconds: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ExteriorConversionReport {
    pub(crate) revision: String,
    pub(crate) corpus_name: Option<String>,
    pub(crate) selected_pipeline: String,
    pub(crate) rows: Vec<ExteriorConversionRow>,
    pub(crate) summary: ExteriorConversionSummary,
    pub(crate) diagnostics: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ExteriorConversionRow {
    pub(crate) source: String,
    pub(crate) source_hash: Option<String>,
    pub(crate) native: BackendObservation,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct BackendObservation {
    pub(crate) status: String,
    pub(crate) report_hash: Option<String>,
    pub(crate) output_hash: Option<String>,
    pub(crate) output_bytes: u64,
    pub(crate) cache_bytes: u64,
    pub(crate) cold_seconds: Option<f64>,
    pub(crate) warm_seconds: Option<f64>,
    pub(crate) meshes: Option<usize>,
    pub(crate) vertices: Option<usize>,
    pub(crate) triangles: Option<usize>,
    pub(crate) materials: Option<usize>,
    pub(crate) textures: Option<usize>,
    pub(crate) missing_textures: usize,
    pub(crate) lossy_features: usize,
    pub(crate) physics_bodies: Option<usize>,
    pub(crate) physics_shapes: Option<usize>,
    pub(crate) physics_joints: Option<usize>,
    pub(crate) diagnostic: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct ExteriorConversionSummary {
    pub(crate) total: usize,
    pub(crate) native_success: usize,
    pub(crate) native_failures: usize,
    pub(crate) lossy_assets: usize,
    pub(crate) native_cache_bytes: u64,
}

pub fn exterior_conversion_report(args: ExteriorConversionReportArgs) -> Result<()> {
    let corpus_path = fs::canonicalize(&args.corpus).with_context(|| {
        format!(
            "conversion corpus does not exist: {}",
            args.corpus.display()
        )
    })?;
    let corpus: CorpusInput = serde_json::from_slice(
        &fs::read(&corpus_path).with_context(|| format!("reading {}", corpus_path.display()))?,
    )
    .with_context(|| format!("parsing conversion corpus {}", corpus_path.display()))?;
    let report = build_report(corpus, corpus_path.parent().unwrap_or(Path::new(".")))?;
    let output = serde_json::to_vec_pretty(&report)?;
    if let Some(path) = args.out {
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, output).with_context(|| format!("writing {}", path.display()))?;
        println!(
            "exterior conversion report: {} rows -> {}",
            report.rows.len(),
            path.display()
        );
    } else {
        println!("{}", String::from_utf8(output)?);
    }
    println!(
        "exterior conversion summary: native_ok={} native_failed={} lossy={}",
        report.summary.native_success, report.summary.native_failures, report.summary.lossy_assets,
    );
    Ok(())
}

fn build_report(corpus: CorpusInput, root: &Path) -> Result<ExteriorConversionReport> {
    let selected_pipeline = corpus.selected_pipeline.to_ascii_lowercase();
    if selected_pipeline != "native" {
        bail!("selected_pipeline must be native; Blender is no longer a production path");
    }
    let mut rows = Vec::with_capacity(corpus.rows.len());
    let mut diagnostics = Vec::new();
    for row in corpus.rows {
        let source_path = resolve_optional(root, row.source_path.as_deref());
        let source_hash = source_path.as_deref().and_then(file_hash);
        let native = observe_backend(
            "native",
            root,
            row.native_report.as_deref(),
            row.native_output.as_deref(),
            row.native_cache_dir.as_deref(),
            row.native_cold_seconds,
            row.native_warm_seconds,
        );
        if let Some(native_source) = report_source(root, row.native_report.as_deref())
            && !same_source(&row.source, &native_source)
        {
            diagnostics.push(format!(
                "{}: native report source identity mismatch: {native_source}",
                row.source
            ));
        }
        rows.push(ExteriorConversionRow {
            source: row.source,
            source_hash,
            native,
        });
    }
    rows.sort_by(|left, right| left.source.cmp(&right.source));
    let mut summary = ExteriorConversionSummary {
        total: rows.len(),
        ..Default::default()
    };
    for row in &rows {
        if row.native.status == "ok" {
            summary.native_success += 1;
        } else {
            summary.native_failures += 1;
        }
        if row.native.lossy_features > 0 {
            summary.lossy_assets += 1;
        }
        summary.native_cache_bytes += row.native.cache_bytes;
    }
    diagnostics.sort();
    Ok(ExteriorConversionReport {
        revision: EXTERIOR_CONVERSION_REPORT_REVISION.into(),
        corpus_name: corpus.corpus_name,
        selected_pipeline,
        rows,
        summary,
        diagnostics,
    })
}

fn observe_backend(
    backend: &str,
    root: &Path,
    report: Option<&Path>,
    output: Option<&Path>,
    cache_dir: Option<&Path>,
    cold_seconds: Option<f64>,
    warm_seconds: Option<f64>,
) -> BackendObservation {
    let report_path = resolve_optional(root, report);
    let output_path = resolve_optional(root, output);
    let cache_path = resolve_optional(root, cache_dir);
    let Some(report_path) = report_path else {
        return BackendObservation {
            status: "missing".into(),
            diagnostic: Some(format!("{backend} report was not supplied")),
            cold_seconds,
            warm_seconds,
            ..Default::default()
        };
    };
    let report_bytes = match fs::read(&report_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return BackendObservation {
                status: "failed".into(),
                diagnostic: Some(format!("reading report {}: {error}", report_path.display())),
                cold_seconds,
                warm_seconds,
                ..Default::default()
            };
        }
    };
    let value: Value = match serde_json::from_slice(&report_bytes) {
        Ok(value) => value,
        Err(error) => {
            return BackendObservation {
                status: "failed".into(),
                report_hash: Some(hash_bytes(&report_bytes)),
                diagnostic: Some(format!("parsing report {}: {error}", report_path.display())),
                cold_seconds,
                warm_seconds,
                ..Default::default()
            };
        }
    };
    let missing_textures = array_len(&value, "missing_textures");
    let lossy_features = array_len(&value, "lossy_issues");
    let status = if value.get("conversion").is_some() {
        "ok"
    } else {
        "failed"
    };
    BackendObservation {
        status: status.into(),
        report_hash: Some(hash_bytes(&report_bytes)),
        output_hash: output_path.as_deref().and_then(file_hash),
        output_bytes: output_path
            .as_deref()
            .and_then(|path| fs::metadata(path).ok())
            .map_or(0, |metadata| metadata.len()),
        cache_bytes: cache_path.as_deref().map_or(0, directory_bytes),
        cold_seconds,
        warm_seconds,
        meshes: usize_field(&value, "meshes"),
        vertices: usize_field(&value, "vertices"),
        triangles: usize_field(&value, "triangles"),
        materials: usize_field(&value, "materials"),
        textures: usize_field(&value, "embedded_textures"),
        missing_textures,
        lossy_features,
        physics_bodies: usize_field(&value, "physics_bodies"),
        physics_shapes: usize_field(&value, "physics_shapes"),
        physics_joints: usize_field(&value, "physics_joints"),
        diagnostic: (status == "failed")
            .then(|| format!("{backend} report has no conversion result")),
    }
}

fn report_source(root: &Path, path: Option<&Path>) -> Option<String> {
    let path = resolve_optional(root, path)?;
    let value: Value = serde_json::from_slice(&fs::read(path).ok()?).ok()?;
    value.get("source")?.as_str().map(str::to_owned)
}

fn same_source(expected: &str, actual: &str) -> bool {
    expected.eq_ignore_ascii_case(actual)
        || Path::new(expected)
            .file_name()
            .zip(Path::new(actual).file_name())
            .is_some_and(|(left, right)| left.eq_ignore_ascii_case(right))
}

fn resolve_optional(root: &Path, path: Option<&Path>) -> Option<PathBuf> {
    let path = path?;
    Some(if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    })
}

fn file_hash(path: &Path) -> Option<String> {
    Some(hash_bytes(&fs::read(path).ok()?))
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn directory_bytes(path: &Path) -> u64 {
    let mut total: u64 = 0;
    let mut stack = vec![path.to_path_buf()];
    while let Some(path) = stack.pop() {
        let Ok(entries) = fs::read_dir(path) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if let Ok(metadata) = entry.metadata() {
                total = total.saturating_add(metadata.len());
            }
        }
    }
    total
}

fn usize_field(value: &Value, name: &str) -> Option<usize> {
    value
        .get(name)?
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())
}

fn array_len(value: &Value, name: &str) -> usize {
    value
        .get(name)
        .and_then(Value::as_array)
        .map_or(0, Vec::len)
}

#[cfg(test)]
#[path = "conversion_tests.rs"]
mod tests;
