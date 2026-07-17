use anyhow::{Result, bail};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

pub(crate) const CURRENT_MANIFEST_SCHEMA_VERSION: u32 = 16;
pub(crate) const CURRENT_PREPARE_REVISION: &str = "prepare-items-v1";
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
