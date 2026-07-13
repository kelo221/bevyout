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

#[test]
fn structural_static_paths_are_the_only_stair_support_candidates() {
    for path in [
        "architecture/megaton/interior/stairs.nif",
        "dungeons/vault/stairwell.nif",
        "landscape/rocks/cliffstep.nif",
    ] {
        assert!(is_structural_step_support(&PreparedSemantic::Static, path));
    }

    for path in [
        "furniture/chair01.nif",
        "clutter/office/shelf01.nif",
        "vehicles/car01.nif",
        "terminals/terminal01.nif",
    ] {
        assert!(!is_structural_step_support(&PreparedSemantic::Static, path));
    }
}

#[test]
fn non_static_semantics_never_become_stair_support() {
    for semantic in [
        PreparedSemantic::Furniture,
        PreparedSemantic::Door(PreparedDoor {
            lock_level: None,
            key_form_id: None,
            destination: None,
        }),
        PreparedSemantic::Activator,
        PreparedSemantic::Container,
        PreparedSemantic::Unsupported,
    ] {
        assert!(!is_structural_step_support(
            &semantic,
            "architecture/megaton/interior/stairs.nif"
        ));
    }
}

#[test]
fn non_static_physics_classifications_remove_stair_support() {
    assert!(retain_static_step_support(
        true,
        PreparedPhysicsClassification::Static
    ));
    assert!(!retain_static_step_support(
        true,
        PreparedPhysicsClassification::Kinematic
    ));
    assert!(!retain_static_step_support(
        true,
        PreparedPhysicsClassification::Dynamic
    ));
    assert!(!retain_static_step_support(
        false,
        PreparedPhysicsClassification::Static
    ));
}
