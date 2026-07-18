use anyhow::{Result, bail};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

pub(crate) const CURRENT_MANIFEST_SCHEMA_VERSION: u32 = 16;
/// Gates *manifest schema/meaning* compatibility for a cached `scene.ron`
/// (checked by `compatibility::ensure_prepared_manifest_compatible` against
/// `manifest.prepare_revision`, independent of
/// `prepare::fingerprints::PREPARE_PIPELINE_REVISION`, which gates the
/// resumable *batch* prepare job's own skip-if-unchanged decision). Bump
/// whenever a prepared placement's serialized shape OR its *meaning*
/// changes -- issue #120 (M4 wave 6) bumped this without any new field: a
/// source-dead NPC reference now classifies as `PreparedSemantic::Corpse`
/// instead of `Npc`, so a `scene.ron` cached before this change would
/// otherwise still parse cleanly (same enum shape) with the wrong,
/// stale classification -- exactly the trap AGENTS.md's "Prepared asset
/// revisions" section names.
pub(crate) const CURRENT_PREPARE_REVISION: &str = "prepare-corpse-v2";
pub(crate) const CURRENT_BAKE_REVISION: &str = "rust-cpu-irradiance-v12-seam-stitch";
pub(crate) const STATIC_POINT_SHADOW_REVISION: &str = "bvh-d32-v6";

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
