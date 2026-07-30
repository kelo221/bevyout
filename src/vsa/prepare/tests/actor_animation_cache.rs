use super::*;

#[test]
fn reuse_requires_both_artifacts_and_successful_validation() {
    let valid = ActorAnimationPackCacheState {
        output_present: true,
        report_present: true,
        validation_passed: true,
        ..Default::default()
    };
    assert_eq!(
        actor_animation_pack_cache_decision(valid),
        ActorAnimationPackCacheDecision::Reuse
    );
    assert_eq!(
        actor_animation_pack_cache_decision(ActorAnimationPackCacheState {
            report_present: false,
            ..valid
        }),
        ActorAnimationPackCacheDecision::Build
    );
    assert_eq!(
        actor_animation_pack_cache_decision(ActorAnimationPackCacheState {
            validation_passed: false,
            ..valid
        }),
        ActorAnimationPackCacheDecision::Build
    );
}

#[test]
fn rebuild_request_always_builds() {
    assert_eq!(
        actor_animation_pack_cache_decision(ActorAnimationPackCacheState {
            rebuild_requested: true,
            output_present: true,
            report_present: true,
            validation_passed: true,
        }),
        ActorAnimationPackCacheDecision::Build
    );
}
