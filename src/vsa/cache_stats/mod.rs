mod formats;
mod model;
mod output;
pub(crate) mod policy;
pub(crate) mod scan;

use crate::{
    cli::{CacheArgs, CacheCommand, CacheStatsArgs},
    vsa::cache_gc,
};
use anyhow::Result;
use model::{CACHE_STATS_SCHEMA_VERSION, CacheFileReport, CacheStatsReport};
use policy::{CacheFileFacts, classify_cache_path, summarize_cache_files};
use std::collections::BTreeMap;

pub fn cache(args: CacheArgs) -> Result<()> {
    match args.command {
        CacheCommand::Stats(args) => cache_stats(args),
        CacheCommand::Gc(args) => cache_gc::cache_gc(args),
    }
}

fn cache_stats(args: CacheStatsArgs) -> Result<()> {
    let scan = scan::scan_cache(&args.cache, args.manifest_set.as_deref())?;
    let facts = scan
        .files
        .iter()
        .map(|file| CacheFileFacts {
            relative_path: file.relative_path.clone(),
            logical_bytes: file.logical_bytes,
            allocated_bytes: file.allocated_bytes,
            payload_id: file.payload_id.clone(),
        })
        .collect::<Vec<_>>();
    let storage = summarize_cache_files(&facts);
    let duplicate_counts = storage
        .duplicate_clusters
        .iter()
        .flat_map(|cluster| {
            cluster
                .paths
                .iter()
                .map(move |path| (path.clone(), cluster.copy_count))
        })
        .collect::<BTreeMap<_, _>>();
    let (glb_files, glb, textures, diagnostics) = formats::inspect_formats(&scan.files);
    let files = scan
        .files
        .iter()
        .zip(glb_files)
        .map(|(file, glb)| CacheFileReport {
            relative_path: file.relative_path.clone(),
            category: classify_cache_path(&file.relative_path).to_string(),
            logical_bytes: file.logical_bytes,
            allocated_bytes: file.allocated_bytes,
            sha256: file.sha256.clone(),
            duplicate_copies: duplicate_counts
                .get(&file.relative_path)
                .copied()
                .unwrap_or(1),
            glb,
        })
        .collect();
    let report = CacheStatsReport {
        schema_version: CACHE_STATS_SCHEMA_VERSION.into(),
        cache_root: scan.cache_root.to_string_lossy().replace('\\', "/"),
        manifest_set: args
            .manifest_set
            .as_ref()
            .map(|path| path.to_string_lossy().replace('\\', "/")),
        directory_count: scan.directory_count,
        storage,
        glb,
        textures,
        files,
        diagnostics,
    };

    output::print_summary(&report);
    if let Some(path) = args.json.as_deref() {
        output::write_json(path, &report)?;
        println!("cache report: json {}", path.display());
    }
    if let Some(path) = args.csv.as_deref() {
        output::write_csv(path, &report.files)?;
        println!("cache report: csv {}", path.display());
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

#[cfg(test)]
#[path = "tests/policy.rs"]
mod policy_tests;
