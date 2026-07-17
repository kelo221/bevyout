mod assets;
mod audio_assets;
mod bake;
mod bsa;
mod catalog;
mod cell_map;
mod content_index;
mod dynamic_lighting;
mod manifest;
mod openmw_esm4;
mod paths;
mod physics;
mod plugin;
mod prepare;
mod recipe;
mod report;
mod scenes;

pub(crate) use assets::NIF_CONVERTER_REVISION;
pub use bake::bake;
pub(crate) use bake::is_bake_static;
pub use catalog::cells;
pub(crate) use dynamic_lighting::{DynamicLight, LightEffect, update_dynamic_lights};
// Issue #51's runtime preloader (`viewer::world`) reads the door-graph
// connectivity `cells --map` (issue #45) emits at prepare time.
pub(crate) use cell_map::CellMap;
/// Test-only re-export (issue #99): `viewer::pipboy`'s use-path unit tests
/// construct `PreparedItemStats::Aid` effect labels; nothing outside tests
/// names the type through `vsa`, so a plain re-export would trip
/// `unused_imports` on non-test builds.
#[cfg(test)]
pub(crate) use manifest::PreparedItemEffect;
#[cfg(test)]
pub(crate) use manifest::{
    CURRENT_BAKE_REVISION, CURRENT_MANIFEST_SCHEMA_VERSION, CURRENT_PREPARE_REVISION, PreparedBake,
    PreparedDoorDestination, PreparedIrradianceVolume,
};
pub(crate) use manifest::{
    CellInfo, ImageSpaceInfo, PreparedAudioClip, PreparedCellLighting, PreparedDoor,
    PreparedDropCollider, PreparedFootstepSet, PreparedInventoryEntry, PreparedItemCatalog,
    PreparedItemCategory, PreparedItemDefinition, PreparedItemStats, PreparedLeveledList,
    PreparedPhysicsClassification, PreparedPickup, PreparedPlacement, PreparedRuntimeMutability,
    PreparedSceneManifest, PreparedSemantic, cell_label, ensure_baked_scene_compatible,
    ensure_prepared_manifest_compatible,
};
pub(crate) use paths::{FO3_SCALE, fingerprint};
pub(crate) use physics::{
    PHYSICS_ASSET_SCHEMA_VERSION, PreparedPhysicsAsset, PreparedPhysicsBody, PreparedPhysicsShape,
    PreparedPhysicsSource, body_blocks_player, read_physics_asset,
};
pub use prepare::prepare;
pub(crate) use prepare::{
    ITEM_CATALOG_REVISION, STATIC_POINT_SHADOW_NEAR_Z, StaticShadowBakeLight,
    bake_static_point_shadow_bytes,
};
pub use report::report;
pub(crate) use scenes::{find_cached_manifest, resolve_cached_manifest};
