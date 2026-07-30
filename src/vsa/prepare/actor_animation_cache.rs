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
#[path = "tests/actor_animation_cache.rs"]
mod tests;
