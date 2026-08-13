use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CacheFileFacts {
    pub(crate) relative_path: String,
    pub(crate) logical_bytes: u64,
    pub(crate) allocated_bytes: u64,
    pub(crate) payload_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CacheCategorySummary {
    pub(crate) category: String,
    pub(crate) logical_bytes: u64,
    pub(crate) allocated_bytes: u64,
    pub(crate) file_count: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CacheDuplicateCluster {
    pub(crate) payload_id: String,
    pub(crate) logical_bytes_per_copy: u64,
    pub(crate) copy_count: u64,
    pub(crate) duplicate_logical_bytes: u64,
    pub(crate) duplicate_allocated_bytes: u64,
    pub(crate) paths: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CacheStorageSummary {
    pub(crate) logical_bytes: u64,
    pub(crate) allocated_bytes: u64,
    pub(crate) unique_payload_bytes: u64,
    pub(crate) duplicate_logical_bytes: u64,
    pub(crate) duplicate_allocated_bytes: u64,
    pub(crate) file_count: u64,
    pub(crate) categories: Vec<CacheCategorySummary>,
    pub(crate) duplicate_clusters: Vec<CacheDuplicateCluster>,
}

pub(crate) fn classify_cache_path(relative_path: &str) -> &'static str {
    let normalized = relative_path.replace('\\', "/").to_ascii_lowercase();
    let components = normalized.split('/').collect::<Vec<_>>();
    let file_name = components.last().copied().unwrap_or_default();

    if components.contains(&"staging") || file_name.ends_with(".tmp") {
        return "staging";
    }
    if file_name == "scene.ron" {
        return "manifest";
    }
    if matches!(
        file_name,
        "actors.ron"
            | "actor_animations.ron"
            | "items.ron"
            | "packages.ron"
            | "dialogue.ron"
            | "sounds.ron"
    ) || components.contains(&"catalogs")
    {
        return "catalog";
    }
    if file_name.contains("navgraph")
        || file_name.contains("navmesh")
        || components
            .iter()
            .any(|component| component.starts_with("nav"))
    {
        return "navigation";
    }
    if file_name.ends_with(".physics.json.gz") || components.contains(&"physics") {
        return "physics";
    }
    if file_name.ends_with(".glb") {
        return "glb";
    }
    if components.contains(&"objects") && components.contains(&"texture") {
        return "texture";
    }
    if file_name.ends_with(".ktx2") {
        if components.contains(&"shadows") {
            return "shadow";
        }
        if components.iter().any(|component| {
            matches!(
                *component,
                "bake" | "bakes" | "lightmaps" | "irradiance" | "reflection_probes"
            )
        }) {
            return "bake";
        }
        return "texture";
    }
    if [".png", ".dds", ".tga", ".bmp"]
        .iter()
        .any(|extension| file_name.ends_with(extension))
    {
        return "texture-source";
    }
    if [".wav", ".ogg", ".mp3", ".xwm"]
        .iter()
        .any(|extension| file_name.ends_with(extension))
        || components.contains(&"audio")
    {
        return "audio";
    }
    if components.iter().any(|component| {
        matches!(
            *component,
            "bake" | "bakes" | "lightmaps" | "irradiance" | "reflection_probes"
        )
    }) || [".blend", ".exr", ".hdr"]
        .iter()
        .any(|extension| file_name.ends_with(extension))
    {
        return "bake";
    }
    if matches!(
        file_name,
        "cellmap.ron" | "prepare_jobs.ron" | "bake_jobs.ron" | "index.ron"
    ) || components.contains(&"indexes")
    {
        return "index";
    }
    if components.contains(&"worldspaces") {
        return "worldspace";
    }
    "other"
}

pub(crate) fn summarize_cache_files(files: &[CacheFileFacts]) -> CacheStorageSummary {
    let mut summary = CacheStorageSummary::default();
    let mut categories = BTreeMap::<String, CacheCategorySummary>::new();
    let mut payloads = BTreeMap::<String, Vec<&CacheFileFacts>>::new();

    for file in files {
        summary.logical_bytes = summary.logical_bytes.saturating_add(file.logical_bytes);
        summary.allocated_bytes = summary.allocated_bytes.saturating_add(file.allocated_bytes);
        summary.file_count = summary.file_count.saturating_add(1);

        let category = classify_cache_path(&file.relative_path).to_string();
        let entry = categories
            .entry(category.clone())
            .or_insert_with(|| CacheCategorySummary {
                category,
                ..Default::default()
            });
        entry.logical_bytes = entry.logical_bytes.saturating_add(file.logical_bytes);
        entry.allocated_bytes = entry.allocated_bytes.saturating_add(file.allocated_bytes);
        entry.file_count = entry.file_count.saturating_add(1);

        payloads
            .entry(file.payload_id.clone())
            .or_default()
            .push(file);
    }

    for (payload_id, mut copies) in payloads {
        copies.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
        let logical_bytes_per_copy = copies
            .iter()
            .map(|file| file.logical_bytes)
            .min()
            .unwrap_or_default();
        let allocated_bytes_to_keep = copies
            .iter()
            .map(|file| file.allocated_bytes)
            .min()
            .unwrap_or_default();
        let total_logical = copies
            .iter()
            .fold(0u64, |total, file| total.saturating_add(file.logical_bytes));
        let total_allocated = copies.iter().fold(0u64, |total, file| {
            total.saturating_add(file.allocated_bytes)
        });
        let duplicate_logical_bytes = total_logical.saturating_sub(logical_bytes_per_copy);
        let duplicate_allocated_bytes = total_allocated.saturating_sub(allocated_bytes_to_keep);

        summary.unique_payload_bytes = summary
            .unique_payload_bytes
            .saturating_add(logical_bytes_per_copy);
        summary.duplicate_logical_bytes = summary
            .duplicate_logical_bytes
            .saturating_add(duplicate_logical_bytes);
        summary.duplicate_allocated_bytes = summary
            .duplicate_allocated_bytes
            .saturating_add(duplicate_allocated_bytes);

        if copies.len() > 1 {
            summary.duplicate_clusters.push(CacheDuplicateCluster {
                payload_id,
                logical_bytes_per_copy,
                copy_count: copies.len() as u64,
                duplicate_logical_bytes,
                duplicate_allocated_bytes,
                paths: copies
                    .iter()
                    .map(|file| file.relative_path.clone())
                    .collect(),
            });
        }
    }

    summary.categories = categories.into_values().collect();
    summary.duplicate_clusters.sort_by(|left, right| {
        right
            .duplicate_allocated_bytes
            .cmp(&left.duplicate_allocated_bytes)
            .then_with(|| left.payload_id.cmp(&right.payload_id))
    });
    summary
}
