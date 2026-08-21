mod assets;
mod audio_assets;
mod bake;
mod bsa;
mod cache_stats;
mod cache_store;
mod catalog;
mod cell_map;
mod content_index;
#[allow(dead_code)]
pub(crate) mod dialogue;
mod exterior;
mod manifest;
mod nif_convert;
mod openmw_esm4;
mod overlay_policy;
#[cfg(test)]
#[path = "tests/overlay_policy.rs"]
mod overlay_policy_tests;
mod paths;
mod physics;
mod plugin;
mod prepare;
mod recipe;
mod record_stream;
mod report;
mod scenes;
mod scripts;

#[cfg(test)]
pub(crate) use assets::PREPARED_CONVERTER_REVISION;
pub(crate) use assets::SUPPORTED_PREPARED_CONVERTER_REVISIONS;
pub use bake::bake;
pub(crate) use bake::is_bake_static;
pub use cache_stats::cache;
pub use catalog::cells;
// Issue #51's runtime preloader (`viewer::world`) reads the door-graph
// connectivity `cells --map` (issue #45) emits at prepare time.
pub(crate) use cell_map::CellMap;
pub(crate) use exterior::{
    apply_staged_assets, build_cell_package, build_worldspace_indexes, prepare_terrain_albedo,
    terrain_from_land,
};
pub use exterior::{exterior_catalog, exterior_conversion_report};
/// Test-only re-export (issue #120): `viewer::scene`'s spawn-path unit tests
/// construct a `PreparedSemantic::Npc(PreparedActor { .. })` placement to
/// prove living-actor placements stay unspawned exactly like before this
/// issue; production code only ever matches on `PreparedSemantic::Npc`'s
/// outer variant (never names `PreparedActor` itself), so a plain re-export
/// would trip `unused_imports` on non-test builds.
#[cfg(test)]
pub(crate) use manifest::PreparedActor;
/// Test-only re-export (issue #99): `viewer::pipboy`'s use-path unit tests
/// construct `PreparedItemStats::Aid` effect labels; nothing outside tests
/// names the type through `vsa`, so a plain re-export would trip
/// `unused_imports` on non-test builds.
#[cfg(test)]
pub(crate) use manifest::PreparedItemEffect;
/// Test-only re-export (issue #128): `viewer::nav_overlay`'s unit tests build
/// a synthetic `nav_graph` manifest entry; production code only ever reads
/// `manifest.nav_graph`'s fields (never names the type), so a plain
/// re-export would trip `unused_imports` on non-test builds.
#[cfg(test)]
pub(crate) use manifest::PreparedNavGraphSource;
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
    ensure_prepared_manifest_compatible_any, hydrate_exterior_package,
};
pub use nif_convert::nif_convert;
pub(crate) use overlay_policy::{FalloutOverlayKind, classify_fallout_overlay};
pub(crate) use paths::{FO3_SCALE, fingerprint};
pub(crate) use physics::{
    PHYSICS_ASSET_SCHEMA_VERSION, PreparedPhysicsAsset, PreparedPhysicsBody, PreparedPhysicsShape,
    PreparedPhysicsSource, body_blocks_player, read_physics_asset,
};
// Issue #176's `showpackages` console command reads the per-cell actor
// catalog (`actors.ron`) and the content-set-wide package catalog
// (`packages.ron`) straight off disk on demand -- see
// `viewer::console::ai_package_commands` -- the same way `nav_overlay`'s
// `tnm` reads `navgraph.ron` on demand rather than through a preloaded
// resource.
pub(crate) use prepare::ITEM_CATALOG_REVISION;
pub(crate) use prepare::{
    ACTOR_ANIMATION_CATALOG_REVISION, ACTOR_ANIMATION_NATIVE_CONVERTER_REVISION,
    ACTOR_CATALOG_REVISION, ActorBlueprint, ActorCatalogEntry, GMST_CATALOG_REVISION,
    PACKAGE_CATALOG_REVISION, PreparedActorCatalog, PreparedGmstCatalog, PreparedPackageCatalog,
    PreparedPackageEntry,
};
// Issue #128's `tnm` console command decodes `navgraph.ron` (issue #111)
// straight into these types -- see `viewer::nav_overlay`.
/// Test-only re-export (issue #128): only `viewer::nav_overlay`'s unit
/// tests construct a `PreparedNavPolygon` fixture directly; production code
/// only ever iterates `PreparedNavMesh::polygons` without naming the
/// element type.
#[cfg(test)]
pub(crate) use prepare::PreparedNavPolygon;
pub use prepare::prepare;
pub(crate) use prepare::{PreparedNavGraph, PreparedNavMesh, exterior_nav_graph};
pub use report::report;
pub(crate) use scenes::{find_cached_manifest, resolve_cached_manifest};
