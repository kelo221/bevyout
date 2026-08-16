//! Pure retention policy for prepared-cache garbage collection.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CacheEntryClass {
    Object,
    LegacyTerrain,
    LegacyExteriorPackage { current_revision: bool },
    RebuildableAsset,
    Staging,
    Recipe,
    Quarantine,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CacheEntryFacts {
    pub(crate) class: CacheEntryClass,
    pub(crate) reachable: bool,
    pub(crate) age_seconds: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GcPolicy {
    pub(crate) grace_seconds: u64,
    pub(crate) include_rebuildable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GcReason {
    UnreferencedObject,
    LegacyTerrain,
    LegacyExteriorPackage,
    UnreferencedRebuildableAsset,
    StaleStaging,
    StaleRecipe,
}

impl GcReason {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::UnreferencedObject => "unreferenced-object",
            Self::LegacyTerrain => "legacy-terrain",
            Self::LegacyExteriorPackage => "legacy-exterior-package",
            Self::UnreferencedRebuildableAsset => "unreferenced-rebuildable-asset",
            Self::StaleStaging => "stale-staging",
            Self::StaleRecipe => "stale-recipe",
        }
    }
}

pub(crate) fn gc_reason(facts: CacheEntryFacts, policy: GcPolicy) -> Option<GcReason> {
    if facts.reachable || facts.age_seconds < policy.grace_seconds {
        return None;
    }
    match facts.class {
        CacheEntryClass::Object => Some(GcReason::UnreferencedObject),
        CacheEntryClass::LegacyTerrain => Some(GcReason::LegacyTerrain),
        CacheEntryClass::LegacyExteriorPackage {
            current_revision: false,
        } => Some(GcReason::LegacyExteriorPackage),
        CacheEntryClass::RebuildableAsset if policy.include_rebuildable => {
            Some(GcReason::UnreferencedRebuildableAsset)
        }
        CacheEntryClass::Staging => Some(GcReason::StaleStaging),
        CacheEntryClass::Recipe => Some(GcReason::StaleRecipe),
        CacheEntryClass::LegacyExteriorPackage {
            current_revision: true,
        }
        | CacheEntryClass::RebuildableAsset
        | CacheEntryClass::Quarantine
        | CacheEntryClass::Other => None,
    }
}
