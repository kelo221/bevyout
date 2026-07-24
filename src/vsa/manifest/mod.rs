use anyhow::{Result, bail};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

pub(crate) const CURRENT_MANIFEST_SCHEMA_VERSION: u32 = 19;
/// Gates *manifest schema/meaning* compatibility for a cached `scene.ron`
/// (checked by `compatibility::ensure_prepared_manifest_compatible` against
/// `manifest.prepare_revision`, independent of
/// `prepare::fingerprints::PREPARE_PIPELINE_REVISION`, which gates the
/// resumable *batch* prepare job's own skip-if-unchanged decision). Bump
/// whenever a prepared placement's serialized shape OR its *meaning*
/// changes. This revision combines two bumps that raced on parallel
/// branches: issue #120 (M4 wave 6) reclassifies source-dead NPC
/// references as `PreparedSemantic::Corpse` (a meaning change with no new
/// field -- a stale `scene.ron` would parse cleanly with the wrong
/// classification, exactly the trap AGENTS.md's "Prepared asset revisions"
/// section names), M4 wave 7's actor assembly (#107/#108), and the actor
/// animation catalog link added for the KF compatibility spike (#104).
/// Bumped again for issue #185: `PreparedDoor` grew a `trapped` field --
/// `#[serde(default)]` alone would let a stale `scene.ron` decode cleanly
/// with every door silently reported untrapped, exactly the AGENTS.md
/// "Prepared asset revisions" trap this bump exists to close.
/// Bumped again for issue #213: `PreparedPlacement` grew
/// `linked_reference_form_id` (`XLKR`) -- a stale `scene.ron` would
/// otherwise decode cleanly with every placement silently reporting no
/// linked reference, breaking Patrol marker chain-walks and
/// near-linked-reference package resolution without any parse error.
pub(crate) const CURRENT_PREPARE_REVISION: &str =
    "prepare-v6-corpse-actor-assembly-animation-catalog-door-trap-linked-ref";
pub(crate) const CURRENT_BAKE_REVISION: &str = "rust-cpu-irradiance-v16-material-extensions-local-thickness-emissive-quarter-cap-shader-emission-gate-v2-physical-effect-bulb-v1-effect-emission-control-v1-light-card-promotion-v1-env-light-emission-v1-17f5769";
pub(crate) const STATIC_POINT_SHADOW_REVISION: &str = "bvh-d32-v7-light-cards-v1";

pub(crate) use bevyout_core::manifest::*;

/// Content-fingerprinted item catalogue loaded as a viewer resource.
///
/// Item definitions are engine-independent core values, while the catalogue
/// itself remains the Bevy-facing load unit used by the current viewer.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub(crate) struct PreparedItemCatalog {
    pub(crate) revision: String,
    pub(crate) source_fingerprint: String,
    pub(crate) items: Vec<PreparedItemDefinition>,
}

mod compatibility;

pub(crate) use compatibility::*;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
