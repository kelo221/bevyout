use super::model::{CacheFileReport, CacheStatsReport};
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub(super) fn print_summary(report: &CacheStatsReport) {
    println!("cache stats: root {}", report.cache_root);
    println!(
        "cache stats: files {}, dirs {}, logical {}, allocated {}",
        report.storage.file_count,
        report.directory_count,
        report.storage.logical_bytes,
        report.storage.allocated_bytes
    );
    println!(
        "cache stats: unique {}, duplicate logical {}, duplicate allocated {}, clusters {}",
        report.storage.unique_payload_bytes,
        report.storage.duplicate_logical_bytes,
        report.storage.duplicate_allocated_bytes,
        report.storage.duplicate_clusters.len()
    );
    for category in &report.storage.categories {
        println!(
            "cache category: {} files {}, logical {}, allocated {}",
            category.category,
            category.file_count,
            category.logical_bytes,
            category.allocated_bytes
        );
    }
    println!(
        "cache glb: files {}, logical {}, embedded images {}, embedded ktx2 {}, geometry {}, animation {}, other {}, failures {}",
        report.glb.file_count,
        report.glb.logical_bytes,
        report.glb.embedded_image_bytes,
        report.glb.embedded_ktx2_bytes,
        report.glb.geometry_bytes,
        report.glb.animation_bytes,
        report.glb.other_buffer_bytes,
        report.glb.parse_failures
    );
    for cluster in report.storage.duplicate_clusters.iter().take(10) {
        println!(
            "cache duplicate: {} copies {}, logical {}, allocated {}, first {}",
            cluster.payload_id,
            cluster.copy_count,
            cluster.duplicate_logical_bytes,
            cluster.duplicate_allocated_bytes,
            cluster.paths.first().map(String::as_str).unwrap_or("")
        );
    }
    println!(
        "cache stats: textures {}, diagnostics {}",
        report.textures.len(),
        report.diagnostics.len()
    );
}

pub(super) fn write_json(path: &Path, report: &CacheStatsReport) -> Result<()> {
    ensure_parent(path)?;
    let mut bytes = serde_json::to_vec_pretty(report)?;
    bytes.push(b'\n');
    fs::write(path, bytes)
        .with_context(|| format!("could not write cache JSON report {}", path.display()))
}

pub(super) fn write_csv(path: &Path, files: &[CacheFileReport]) -> Result<()> {
    ensure_parent(path)?;
    let mut csv = String::from(
        "relative_path,category,logical_bytes,allocated_bytes,sha256,duplicate_copies,glb_json_bytes,glb_binary_bytes,glb_geometry_bytes,glb_animation_bytes,glb_embedded_image_bytes,glb_embedded_ktx2_bytes,glb_other_buffer_bytes,glb_padding_bytes\n",
    );
    for file in files {
        let glb = file.glb.as_ref();
        let values = [
            csv_field(&file.relative_path),
            csv_field(&file.category),
            file.logical_bytes.to_string(),
            file.allocated_bytes.to_string(),
            csv_field(file.sha256.as_deref().unwrap_or("")),
            file.duplicate_copies.to_string(),
            optional_number(glb.map(|value| value.json_bytes)),
            optional_number(glb.map(|value| value.binary_bytes)),
            optional_number(glb.map(|value| value.geometry_bytes)),
            optional_number(glb.map(|value| value.animation_bytes)),
            optional_number(glb.map(|value| value.embedded_image_bytes)),
            optional_number(glb.map(|value| value.embedded_ktx2_bytes)),
            optional_number(glb.map(|value| value.other_buffer_bytes)),
            optional_number(glb.map(|value| value.padding_bytes)),
        ];
        csv.push_str(&values.join(","));
        csv.push('\n');
    }
    fs::write(path, csv)
        .with_context(|| format!("could not write cache CSV report {}", path.display()))
}

fn optional_number(value: Option<u64>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

fn csv_field(value: &str) -> String {
    if value
        .chars()
        .any(|character| matches!(character, ',' | '"' | '\n' | '\r'))
    {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn ensure_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .with_context(|| format!("could not create report directory {}", parent.display()))?;
    }
    Ok(())
}
