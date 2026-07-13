use super::*;

#[test]
fn valid_cached_assets_are_reused_even_when_manifest_is_forced() {
    assert_eq!(
        asset_cache_decision(true, true, false),
        AssetCacheDecision::Reuse
    );
    assert_eq!(
        asset_cache_decision(true, true, true),
        AssetCacheDecision::RebuildRequested
    );
}

#[test]
fn missing_and_invalid_cached_assets_are_rebuilt() {
    assert_eq!(
        asset_cache_decision(false, false, false),
        AssetCacheDecision::BuildMissing
    );
    assert_eq!(
        asset_cache_decision(true, false, false),
        AssetCacheDecision::RebuildInvalid
    );
    assert_eq!(
        asset_cache_decision(true, false, true),
        AssetCacheDecision::RebuildInvalid
    );
}
