//! Preparation cache decisions.

use super::*;

pub(crate) fn asset_cache_decision(
    output_exists: bool,
    cache_valid: bool,
    rebuild_assets: bool,
) -> AssetCacheDecision {
    if !output_exists {
        AssetCacheDecision::BuildMissing
    } else if !cache_valid {
        AssetCacheDecision::RebuildInvalid
    } else if rebuild_assets {
        AssetCacheDecision::RebuildRequested
    } else {
        AssetCacheDecision::Reuse
    }
}
