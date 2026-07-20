//! Pure cache-selection policy for prepared external-KF clip packs.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct ActorAnimationPackCacheState {
    pub(crate) rebuild_requested: bool,
    pub(crate) output_present: bool,
    pub(crate) report_present: bool,
    pub(crate) validation_passed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ActorAnimationPackCacheDecision {
    Reuse,
    Build,
}

#[must_use]
pub(crate) const fn actor_animation_pack_cache_decision(
    state: ActorAnimationPackCacheState,
) -> ActorAnimationPackCacheDecision {
    if !state.rebuild_requested
        && state.output_present
        && state.report_present
        && state.validation_passed
    {
        ActorAnimationPackCacheDecision::Reuse
    } else {
        ActorAnimationPackCacheDecision::Build
    }
}

#[cfg(test)]
mod tests {
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
}
