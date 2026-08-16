use serde::Serialize;
use std::{collections::BTreeMap, path::PathBuf, time::SystemTime};

pub(crate) const CACHE_GC_SCHEMA_VERSION: &str = "cache-gc-v1";

#[derive(Debug, Clone, Serialize)]
pub(crate) struct GcCandidate {
    pub(crate) relative_path: String,
    pub(crate) reason: String,
    pub(crate) logical_bytes: u64,
    pub(crate) allocated_bytes: u64,
    pub(crate) age_seconds: u64,
    #[serde(skip)]
    pub(crate) absolute_path: PathBuf,
    #[serde(skip)]
    pub(crate) modified: SystemTime,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct GcReport {
    pub(crate) schema_version: String,
    pub(crate) cache_root: String,
    pub(crate) dry_run: bool,
    pub(crate) grace_hours: u64,
    pub(crate) include_rebuildable: bool,
    pub(crate) live_file_count: u64,
    pub(crate) examined_file_count: u64,
    pub(crate) candidate_file_count: u64,
    pub(crate) candidate_logical_bytes: u64,
    pub(crate) candidate_allocated_bytes: u64,
    pub(crate) candidates_by_reason: BTreeMap<String, GcReasonSummary>,
    pub(crate) deleted_file_count: u64,
    pub(crate) deleted_logical_bytes: u64,
    pub(crate) deleted_allocated_bytes: u64,
    pub(crate) candidates: Vec<GcCandidate>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct GcReasonSummary {
    pub(crate) file_count: u64,
    pub(crate) logical_bytes: u64,
    pub(crate) allocated_bytes: u64,
}
