use anyhow::{Result, bail};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

pub(crate) const CURRENT_MANIFEST_SCHEMA_VERSION: u32 = 28;
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
/// Bumped for M6: `PreparedSceneManifest` now carries the optional exterior
/// package contract, whose shape must never be silently defaulted on old data.
/// Bumped again while merging M5 screen feedback: the manifest now also
/// carries `screen_fx_catalog_path`, so neither branch's previously current
/// cache may silently stand in for the combined shape.
/// Bumped for M4 issue #292: package catalog contents and package-linked
/// marker retention now change prepared placement meaning.
/// Bumped for Lane C issue #290: authored IDLE definitions and their
/// animation-catalog meaning are now part of prepared content.
/// Bumped for issue #299 (M6 W3-C stage 1): the exterior prepare path now
/// populates `actor_catalog_path`/`actor_animation_catalog_path` instead of
/// hardcoding `None`, and the exterior package's ACHR/ACRE references carry
/// a resolved actor assembly. A stale exterior `scene.ron` would otherwise
/// decode cleanly with no actor catalog pointer and no gameplay actors.
pub(crate) const CURRENT_PREPARE_REVISION: &str = "prepare-v28-exterior-actor-catalog";
pub(crate) const CURRENT_BAKE_REVISION: &str = "rust-cpu-irradiance-v16-material-extensions-local-thickness-emissive-quarter-cap-shader-emission-gate-v2-physical-effect-bulb-v1-effect-emission-control-v1-light-card-promotion-v1-env-light-emission-v1-17f5769-lighting-contract-v1-uv1-vendored-xatlas-v1-direct-surface-lightmaps-v7-edge-supersample-two-bounce-shared-volume-geometry-aware-ray-bias-invalid-normal-v1-in-tree-rgba16f-rgb9e5-ktx2-v1-adaptive-sampling-v1-feature-guided-atrous-denoise-v1-spot-cone-v1-configurable-bounces-v1-density-overrides-v1-partial-primitive-light-invalidation-v1-debug-images-v1-area-emissive-mis-rr-variance-v1-variance-manifest-v1-environment-map-hdr-v1-environment-mis-v1-linear-factor-env-single-count-solari-transport-v2-solari-p1-transport-correctness-v1-overlay-exclusion-v1";
pub(crate) const STATIC_POINT_SHADOW_REVISION: &str =
    "bvh-d32-v8-light-cards-v1-overlay-exclusion-v1";
pub(crate) const REFLECTION_PROBE_REVISION: &str = "nav-room-farthest-v2-rgb9e5-64-eye-1p65-spacing-12-cap-16-specular-only-lighting-contract-v1-spot-cone-v1-overlay-exclusion-v1";

pub(crate) use bevyout_core::manifest::*;

mod exterior_package;
pub(crate) use exterior_package::hydrate_exterior_package;

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
