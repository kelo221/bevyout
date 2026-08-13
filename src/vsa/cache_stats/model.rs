use super::policy::CacheStorageSummary;
use serde::Serialize;

pub(crate) const CACHE_STATS_SCHEMA_VERSION: &str = "cache-stats-v1";

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct CacheStatsReport {
    pub(crate) schema_version: String,
    pub(crate) cache_root: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) manifest_set: Option<String>,
    pub(crate) directory_count: u64,
    pub(crate) storage: CacheStorageSummary,
    pub(crate) glb: GlbSummary,
    pub(crate) textures: Vec<TextureReport>,
    pub(crate) files: Vec<CacheFileReport>,
    pub(crate) diagnostics: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct CacheFileReport {
    pub(crate) relative_path: String,
    pub(crate) category: String,
    pub(crate) logical_bytes: u64,
    pub(crate) allocated_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sha256: Option<String>,
    pub(crate) duplicate_copies: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) glb: Option<GlbFileStats>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct GlbSummary {
    pub(crate) file_count: u64,
    pub(crate) logical_bytes: u64,
    pub(crate) json_bytes: u64,
    pub(crate) binary_bytes: u64,
    pub(crate) geometry_bytes: u64,
    pub(crate) animation_bytes: u64,
    pub(crate) embedded_image_bytes: u64,
    pub(crate) embedded_ktx2_bytes: u64,
    pub(crate) other_buffer_bytes: u64,
    pub(crate) padding_bytes: u64,
    pub(crate) parse_failures: u64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct GlbFileStats {
    pub(crate) json_bytes: u64,
    pub(crate) binary_bytes: u64,
    pub(crate) geometry_bytes: u64,
    pub(crate) animation_bytes: u64,
    pub(crate) embedded_image_bytes: u64,
    pub(crate) embedded_ktx2_bytes: u64,
    pub(crate) other_buffer_bytes: u64,
    pub(crate) padding_bytes: u64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct TextureReport {
    pub(crate) location: String,
    pub(crate) container: String,
    pub(crate) role: String,
    pub(crate) encoded_bytes: u64,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) depth: u32,
    pub(crate) layer_count: u32,
    pub(crate) face_count: u32,
    pub(crate) mip_count: u32,
    pub(crate) channel_count: u32,
    pub(crate) vk_format: u32,
    pub(crate) supercompression: String,
    pub(crate) color_model: String,
    pub(crate) color_space: String,
}
