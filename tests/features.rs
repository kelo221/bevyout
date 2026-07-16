//! Executable-spec seed (WP6): runs the three `features/*.feature` files
//! against real `bevyout` code via the `cucumber` crate.
//!
//! `src/lib.rs` keeps `vsa` private, so this integration test cannot
//! `use bevyout::vsa::...`. Rather than widen the library's public surface,
//! the modules under test are pulled in verbatim with `#[path]` so the exact
//! same source compiles into this test binary. Concretely:
//!
//! - `vsa::manifest` is self-contained (only depends on `bevy`/`serde`, both
//!   already real dependencies), so it is included as-is.
//! - `vsa::paths`, `vsa::bsa`, and `vsa::assets` form a small dependency
//!   chain (`assets` -> `bsa`/`manifest`/`paths`, `paths` -> `plugin`) that
//!   is also included as-is *except* for `vsa::plugin`, which in the real
//!   crate is a facade re-exporting the ~2200-line attributed ESM4 reader
//!   (`vsa::openmw_esm4`). Pulling that whole parser in just so `paths.rs`
//!   type-checks a `&ReferenceRecord` parameter on a function this suite
//!   never calls (`placement_transform`) would be all cost, no signal. So
//!   `plugin` below is a small stand-in exposing only the `ReferenceRecord`
//!   fields `paths.rs` itself touches (its own `#[cfg(test)]` module
//!   constructs one). This is scoped narrowly to unblock compilation, not to
//!   re-implement plugin/ESM4 behaviour -- none of that behaviour is
//!   exercised here.
/// Stand-in for `vsa::plugin`'s re-export of `openmw_esm4::ReferenceRecord`,
/// scoped to the handful of fields `paths.rs` reads. See the module-level
/// comment above for why.
mod plugin {
    #[derive(Debug, Clone, Default)]
    #[allow(dead_code)]
    pub(crate) struct ReferenceRecord {
        pub(crate) form_id: u32,
        pub(crate) base_form_id: u32,
        pub(crate) position: [f32; 3],
        pub(crate) rotation: [f32; 3],
        pub(crate) scale: f32,
        pub(crate) flags: u32,
    }
}

// M3/#95 canonical item instances and atomic holder transactions are pure
// serde/std policy, so the executable-spec harness drives the same source as
// the Bevy runtime instead of maintaining a test-only model.
#[path = "../src/item_transaction.rs"]
#[allow(dead_code, unused_imports)]
mod item_transaction;

// These files are pulled in verbatim and cover far more ground than the three
// pure seams this suite drives (placement math, cell selectors, manifest
// (de)serialization, conversion-profile selection). Everything else in them
// -- BSA archive I/O, Blender job orchestration, GLB validation, and so on --
// is legitimately unused from here, so allow dead_code per included module
// rather than mask it crate-wide.
#[path = "../src/vsa/paths.rs"]
#[allow(dead_code, unused_imports)]
mod paths;

#[path = "../src/vsa/manifest/mod.rs"]
#[allow(dead_code, unused_imports)]
mod manifest;

#[path = "../src/vsa/bsa.rs"]
#[allow(dead_code, unused_imports)]
mod bsa;

// `manifest` and `assets` both lean on the physics sidecar module since the
// native-Havok refactor, so it rides along on the same verbatim-include basis.
#[path = "../src/vsa/physics.rs"]
#[allow(dead_code, unused_imports)]
mod physics;

#[path = "../src/vsa/assets/mod.rs"]
#[allow(dead_code, unused_imports)]
mod assets;

// `cell_map` (issue #45) is deliberately dependency-free (serde + std only,
// no Bevy, no openmw_esm4 types in its public surface -- see its module
// doc comment), so unlike the modules above it needs no stand-ins to
// include verbatim.
#[path = "../src/vsa/cell_map.rs"]
#[allow(dead_code, unused_imports)]
mod cell_map;

// `world::policy` (issue #51) is likewise dependency-free (std only, no
// Bevy) -- see its module doc comment -- so it is included verbatim too.
#[path = "../src/viewer/world/policy.rs"]
#[allow(dead_code, unused_imports)]
mod policy;

// `world::swap_policy` (issue #52) is likewise dependency-free (std only, no
// Bevy) -- see its module doc comment -- so it is included verbatim too.
#[path = "../src/viewer/world/swap_policy.rs"]
#[allow(dead_code, unused_imports)]
mod swap_policy;

// `world::reveal_policy` (issue #55) is likewise dependency-free (std only,
// no Bevy) -- see its module doc comment -- so it is included verbatim too.
#[path = "../src/viewer/world/reveal_policy.rs"]
#[allow(dead_code, unused_imports)]
mod reveal_policy;

// `animation::policy` (issue #57) is likewise dependency-free (std only, no
// Bevy) -- see its module doc comment -- so it is included verbatim too.
#[path = "../src/viewer/animation/policy.rs"]
#[allow(dead_code, unused_imports)]
mod animation_policy;

#[path = "../src/vsa/bake/policy.rs"]
#[allow(dead_code, unused_imports)]
mod bake_policy;

// `world::persist_policy` (issues #60/#61) is likewise dependency-free (std
// only, no Bevy) -- see its module doc comment -- so it is included verbatim
// too.
#[path = "../src/viewer/world/persist_policy.rs"]
#[allow(dead_code, unused_imports)]
mod persist_policy;

// `world::ownership_policy` (issue #63) is dependency-free too (std only,
// generic over the collider id types), included verbatim the same way.
#[path = "../src/viewer/world/ownership_policy.rs"]
#[allow(dead_code, unused_imports)]
mod ownership_policy;

// M3 wave 1 inventory policy is intentionally std-only so the executable
// feature suite can drive the same stack and quantity decisions as Bevy.
#[path = "../src/viewer/inventory.rs"]
#[allow(dead_code, unused_imports)]
mod inventory_policy;

// `viewer::performance_policy` is the std-only frame selection/statistics
// seam consumed by the BRP performance probe.
#[path = "../src/viewer/performance_policy.rs"]
#[allow(dead_code, unused_imports)]
mod performance_policy;

// `interaction::container_policy` (issue #75) is dependency-free too (std
// only, no Bevy) -- see its module doc comment -- so it is included
// verbatim here too.
#[path = "../src/viewer/interaction/container_policy.rs"]
#[allow(dead_code, unused_imports)]
mod container_policy;

// `vsa::prepare::selectors` reuses the selector grammar from `vsa::paths`
// via a relative `super::super::paths` import, and `vsa::prepare::batch_cache`
// (issue #47) similarly reuses `vsa::cell_map` via a relative
// `super::super::cell_map` import, so both are nested one module deep here to
// make those paths land on the `mod paths`/`mod cell_map` includes above.
#[path = "."]
mod prepare {
    #[path = "../src/vsa/prepare/selectors.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod selectors;

    #[path = "../src/vsa/prepare/batch_cache.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod batch_cache;

    // `vsa::prepare::jobs` (issue #48) has no relative `super::super::`
    // imports of its own -- it is std/serde/ron only -- but lives here too
    // for consistency with its sibling pure `prepare` seams above.
    #[path = "../src/vsa/prepare/jobs.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod jobs;

    // `vsa::prepare::fingerprints` (issue #49) reuses `vsa::assets` (for
    // `NIF_CONVERTER_REVISION`) via a relative `super::super::assets`
    // import, so it is nested here too -- same pattern as `selectors`/
    // `batch_cache` above -- to land that path on the `mod assets` include
    // near the top of this file. `jobs` depends on it (`use
    // super::fingerprints::...`), so both must live in this same block.
    #[path = "../src/vsa/prepare/fingerprints.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod fingerprints;

    #[path = "../src/vsa/prepare/container_audio_policy.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod container_audio_policy;
}
use prepare::batch_cache;
use prepare::container_audio_policy;
use prepare::fingerprints;
use prepare::jobs;
use prepare::selectors;

// `vsa::bake::plan` (issue #62) reuses the resumable job-manifest machinery
// from `vsa::prepare::jobs` (#48) through relative
// `super::super::prepare::...` imports (in the real crate those names are
// re-exported at `prepare`'s module level by `prepare/mod.rs`'s
// `pub(crate) use jobs::*`), plus `vsa::manifest` for `PreparedBake`. It is
// nested two modules deep here, with small stand-in `manifest`/`prepare`
// re-export modules alongside it, so both relative paths land on the
// verbatim includes above rather than needing `prepare/mod.rs` itself.
#[path = "."]
mod vsa_bake {
    pub mod manifest {
        pub(crate) use crate::manifest::PreparedBake;
    }
    pub mod prepare {
        pub(crate) use crate::prepare::jobs::*;
    }
    #[path = "."]
    pub mod bake {
        #[path = "../src/vsa/bake/plan.rs"]
        #[allow(dead_code, unused_imports)]
        pub mod plan;
    }
}
use vsa_bake::bake::plan as bake_plan;

// `viewer::interaction::leveled` (issue #74) is dependency-free (std only,
// no Bevy, no `vsa::manifest` import -- see its module doc comment for why
// it mirrors `PreparedLeveledList`/`PreparedLeveledEntry` with local plain
// types), so unlike the modules above it needs no nesting or stand-ins to
// include verbatim.
#[path = "../src/viewer/interaction/leveled.rs"]
mod leveled;

// `viewer::interaction::item_rules` (issue #81) is dependency-free (std
// only, no Bevy) like `container_policy`, so it is included verbatim too.
#[path = "../src/viewer/interaction/item_rules.rs"]
#[allow(dead_code, unused_imports)]
mod item_rules;

// Drop placement is a Bevy-free candidate policy, so the runtime and the
// executable spec share the same retreat/fallback decision logic.
#[path = "../src/viewer/world_items/drop_policy.rs"]
#[allow(dead_code, unused_imports)]
mod drop_policy;
// `viewer::interaction::item_use` (issue #99) is dependency-free (std only,
// no Bevy) like `item_rules`, so it is included verbatim too.
#[path = "../src/viewer/interaction/item_use.rs"]
#[allow(dead_code, unused_imports)]
mod item_use;
// `viewer::player::equipment` (issue #98) reuses `viewer::inventory::StackKey`
// via a relative `super::super::inventory::StackKey` import (the same
// nesting-depth trick `vsa::prepare::selectors` uses for `vsa::paths`), so it
// is nested two modules deep here behind a `viewer_player` stand-in that
// aliases the existing `inventory_policy` include as `inventory` -- mirroring
// how `vsa_bake` above aliases `manifest`/`prepare` for `bake::plan`.
#[path = "."]
mod viewer_player {
    pub mod inventory {
        pub(crate) use crate::inventory_policy::*;
    }
    #[path = "."]
    pub mod player {
        #[path = "../src/viewer/player/equipment.rs"]
        #[allow(dead_code, unused_imports)]
        pub mod equipment;
    }
}
use viewer_player::player::equipment;
// The serde/std-only recipe validation seam keeps this executable-spec
// fixture independent of Bevy and game data. Parser/catalog integration is
// covered by the recipe unit tests.
#[path = "../src/vsa/recipe.rs"]
#[allow(dead_code, unused_imports)]
mod recipe_policy;

use assets::AssetConversion;
use cucumber::{World as _, given, then, when};
use item_transaction::{
    HolderId, ItemHolderState, ItemInstance, ItemInstanceId, ItemLedger, ItemState,
    TransactionError, TransactionRequest,
};
use manifest::{PreparedPlacement, PreparedSceneManifest, PreparedSemantic};
use paths::{CellSelector, normalize_asset_path, parse_cell_selector, placement_transform_parts};
use selectors::{CellSummary, SelectionSpec, resolve_selection};

#[derive(Debug, Default, cucumber::World)]
struct BevyoutWorld {
    // -- coordinates.feature --
    position: [f32; 3],
    rotation: [f32; 3],
    scale: f32,
    translation: [f32; 3],
    rotation_xyzw: [f32; 4],
    scale_out: f32,
    recovered_position: [f32; 3],
    selector: Option<CellSelector>,
    normalized_path: Option<String>,

    // -- manifest.feature --
    manifest: Option<PreparedSceneManifest>,

    // -- asset_materials.feature --
    is_static: bool,
    conversion: Option<AssetConversion>,
    authored_emission: Option<[f32; 3]>,
    source_emission_strength: Option<f32>,
    explicit_emission: bool,
    bulb_emission_override: bool,
    glow_emission_override: bool,
    emission_policy: Option<assets::MaterialEmissionPolicy>,

    // -- cell_map.feature --
    cell_map_cells: Vec<cell_map::CellMapEntry>,
    cell_map_worldspaces: Vec<cell_map::WorldspaceEntry>,
    cell_map_doors: Vec<cell_map::DoorEdge>,
    cell_map_unresolved: u32,
    cell_map: Option<cell_map::CellMap>,
    cell_map_ron_a: Option<String>,
    cell_map_ron_b: Option<String>,

    // -- prepare_selectors.feature --
    cells: Vec<CellSummary>,
    worldspace_names: Vec<(u32, String)>,
    selection_result: Option<Result<Vec<u32>, String>>,

    // -- batch_session.feature --
    batch_physics_cache: batch_cache::KeyedBatchCache<()>,
    batch_asset_totals: batch_cache::BatchAssetTotals,
    batch_cache_dir: Option<std::path::PathBuf>,
    written_cell_map_path: Option<std::path::PathBuf>,
    written_cell_map: Option<cell_map::CellMap>,

    // -- preload_policy.feature --
    preload_doors: Vec<policy::DoorLink>,
    preload_prepared: std::collections::HashSet<u32>,
    preload_resident: Vec<u32>,
    preload_active_cell: u32,
    preload_budget: usize,
    preload_plan: Option<policy::PreloadPlan>,

    // -- resumable_prepare.feature (issue #48) --
    job_manifest: Option<jobs::JobManifest>,
    job_manifest_path: Option<std::path::PathBuf>,
    job_resume_result: Option<(Vec<u32>, usize)>,

    // -- instant_swap.feature --
    swap_residency: Option<swap_policy::Residency>,
    swap_manifest_exists: bool,
    swap_decision: Option<swap_policy::SwapDecision>,
    swap_fallback_load_ok: bool,
    swap_fallback_outcome: Option<swap_policy::FallbackOutcome>,
    collider_work: Vec<(usize, bool)>,
    collider_phase_partitions: Option<(Vec<usize>, Vec<usize>)>,

    // -- fingerprints.feature (issue #49) --
    fingerprint_current: Option<fingerprints::CellFingerprints>,
    fingerprint_stale_components: Option<Vec<fingerprints::FingerprintComponent>>,
    fingerprint_resume_result: Option<(Vec<u32>, usize, fingerprints::StaleCells)>,

    // -- first_reveal.feature (issue #55) --
    reveal_candidates: Vec<reveal_policy::RevealCandidate>,
    reveal_budget: usize,
    reveal_chunks: Vec<Vec<usize>>,

    // -- door_animation.feature (issue #57) --
    animation_clip_names: Vec<String>,
    animation_selected_clip: Option<Option<String>>,
    animation_open_clip_seconds: Option<f32>,
    animation_open_lead: Option<f32>,

    // -- rust_irradiance.feature --
    bake_volume_scale: [f32; 3],
    bake_probe_spacing: f32,
    bake_resolution: [u32; 3],
    bake_atlas_dimensions: [u32; 3],
    bake_samples: u32,
    bake_primary_rays: usize,

    // -- loading_fallback.feature (issue #59) --
    fallback_state: Option<swap_policy::FallbackState>,
    fallback_lifecycle_outcome: Option<swap_policy::FallbackLifecycleOutcome>,
    overlay_fade_duration: f32,
    overlay_fade_max_alpha: f32,
    overlay_alpha: Option<f32>,

    // -- resumable_bake.feature (issue #62) --
    bake_recorded: std::collections::BTreeMap<u32, Option<manifest::PreparedBake>>,
    bake_validity: std::collections::BTreeMap<u32, bool>,
    bake_valid_result: Option<bool>,
    bake_resume_result: Option<(Vec<u32>, usize, Vec<u32>)>,

    // -- state_persistence.feature (issues #60/#61) --
    persist_placements: Vec<persist_policy::PlacementInfo>,
    persist_deltas: std::collections::HashMap<u32, persist_policy::ReferenceDelta>,
    persist_effective: Option<std::collections::HashMap<u32, bool>>,
    persist_baselines: Vec<persist_policy::BaselinePlacement>,
    persist_snapshots: Vec<persist_policy::RuntimeSnapshot>,
    persist_captured: Option<std::collections::HashMap<u32, persist_policy::ReferenceDelta>>,
    persist_applications: Vec<persist_policy::PlacementApplication>,

    // -- collider_ownership.feature (issue #63) --
    ownership_ledger: ownership_policy::CellColliderLedger<u64, u64>,
    ownership_released: Option<Option<ownership_policy::CellColliders<u64, u64>>>,
    ownership_next_id: u64,

    // -- container_audio.feature --
    container_audio_is_container: bool,
    container_audio_record_open: Option<u32>,
    container_audio_record_close: Option<u32>,
    container_audio_cues: Vec<container_audio_policy::AnimationSoundCue>,
    container_audio_selected: Option<container_audio_policy::SelectedContainerAudio>,
    container_audio_resolved_open: Option<u32>,
    container_audio_resolved_close: Option<u32>,
    container_audio_prepared_open: Option<u32>,
    container_audio_prepared_close: Option<u32>,

    // -- inventory.feature (M3 wave 1, issues #71/#72) --
    player_inventory: inventory_policy::Inventory,
    inventory_weights: std::collections::HashMap<u32, f32>,
    inventory_drop_action: Option<inventory_policy::DropAction>,
    inventory_transfer: Option<inventory_policy::TransferResult>,

    // -- performance_probe.feature --
    performance_samples: Vec<performance_policy::FrameSample>,
    performance_summary: Option<performance_policy::FrameProbeSummary>,

    // -- leveled_lists.feature (issue #74) --
    leveled_lists: std::collections::BTreeMap<u32, leveled::PreparedLeveledList>,
    leveled_seeds: std::collections::HashMap<String, leveled::LeveledSeed>,
    leveled_last_resolution: Option<Vec<(u32, i32)>>,
    // -- container_transfer.feature (issue #75) --
    container_seed_entries: Vec<container_policy::SeedEntry>,
    container_resolver_lists: std::collections::BTreeMap<u32, Vec<(u32, i32)>>,
    container_resolver_calls: u32,
    container_state: Option<container_policy::ContainerState>,
    container_stacks: Vec<(u32, i32)>,
    player_stacks: Vec<(u32, i32)>,
    transfer_result: Option<Result<i32, container_policy::TransferError>>,
    // -- container_persistence.feature (issue #76) --
    container_baselines: std::collections::HashMap<u32, persist_policy::ContainerBaseline>,
    container_snapshots: std::collections::HashMap<u32, persist_policy::ContainerSnapshot>,
    container_captured: Option<std::collections::HashMap<u32, persist_policy::ContainerDelta>>,
    container_deltas: std::collections::HashMap<u32, persist_policy::ContainerDelta>,
    container_seeded: Option<std::collections::HashMap<u32, persist_policy::ContainerSnapshot>>,
    // -- item_flags.feature (issue #81) --
    flag_record_flags: u32,
    rule_result: Option<Result<(), item_rules::TransferRejection>>,
    carried_stacks: Vec<(i32, bool, f32)>,
    carried_total: Option<f32>,
    take_classification: Option<item_rules::TakeClassification>,

    // -- item_transactions.feature (M3/#95) --
    canonical_ledger: item_transaction::ItemLedger,
    canonical_result:
        Option<Result<item_transaction::TransactionReceipt, item_transaction::TransactionError>>,

    // -- drop_placement.feature (M3/#95) --
    drop_blocked_count: usize,
    drop_all_blocked: bool,
    drop_decision: Option<drop_policy::DropPlacementDecision>,

    // -- item_use.feature (issue #99) --
    item_use_stats: Option<item_use::ItemStats>,
    item_use_quest_item: bool,
    item_use_action: Option<item_use::ItemUseAction>,

    // -- equipment.feature (issue #98) --
    equipment_state: equipment::EquipmentState,
    equip_result: Option<Result<equipment::EquipOutcome, equipment::EquipError>>,

    // -- recipes.feature (issue #117) --
    recipe_under_test: Option<recipe_policy::PreparedRecipe>,
    recipe_available_items: std::collections::BTreeSet<u32>,
    recipe_validation: Option<Result<(), recipe_policy::RecipeValidationError>>,
}

fn find_placement<'a>(
    manifest: &'a PreparedSceneManifest,
    editor_id: &str,
) -> &'a PreparedPlacement {
    manifest
        .placements
        .iter()
        .find(|placement| placement.editor_id.as_deref() == Some(editor_id))
        .unwrap_or_else(|| panic!("no placement with editor id {editor_id:?} in fixture"))
}

// ---------------------------------------------------------------------
// coordinates.feature
// ---------------------------------------------------------------------

#[given(regex = r"^a Fallout position of (-?[\d.]+), (-?[\d.]+), (-?[\d.]+) world units$")]
async fn given_position(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    world.position = [x, y, z];
}

#[given(regex = r"^a Fallout rotation of (-?[\d.]+), (-?[\d.]+), (-?[\d.]+) radians$")]
async fn given_rotation(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    world.rotation = [x, y, z];
}

#[given(regex = r"^a Fallout scale of (-?[\d.]+)$")]
async fn given_scale(world: &mut BevyoutWorld, scale: f32) {
    world.scale = scale;
}

#[when("it is converted to a Bevy placement transform")]
async fn when_converted(world: &mut BevyoutWorld) {
    let scale = if world.scale == 0.0 { 1.0 } else { world.scale };
    let (translation, rotation_xyzw, scale_out) =
        placement_transform_parts(world.position, world.rotation, scale);
    world.translation = translation;
    world.rotation_xyzw = rotation_xyzw;
    world.scale_out = scale_out;
}

#[when("the Bevy translation is converted back to Fallout world units")]
async fn when_converted_back(world: &mut BevyoutWorld) {
    // `placement_transform_parts` maps Fallout (x, y, z) to Bevy
    // (x, z, -y) * FO3_SCALE; invert that mapping and the scale to recover
    // the original Fallout world units.
    let [bx, by, bz] = world.translation;
    let scale = paths::FO3_SCALE;
    world.recovered_position = [bx / scale, -bz / scale, by / scale];
}

#[then(regex = r"^the recovered position matches the original Fallout position$")]
async fn then_recovered_matches_original(world: &mut BevyoutWorld) {
    for (recovered, original) in world.recovered_position.iter().zip(world.position.iter()) {
        assert!(
            (recovered - original).abs() < 0.01,
            "recovered {:?} != original {:?}",
            world.recovered_position,
            world.position
        );
    }
}

#[then(regex = r"^the Bevy translation is (-?[\d.]+), (-?[\d.]+), (-?[\d.]+) metres$")]
async fn then_translation_is(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    let expected = [x, y, z];
    for (actual, expected) in world.translation.iter().zip(expected.iter()) {
        assert!(
            (actual - expected).abs() < 1e-4,
            "translation {:?} != expected {:?}",
            world.translation,
            expected
        );
    }
}

#[then("the Bevy rotation is the identity quaternion")]
async fn then_rotation_is_identity(world: &mut BevyoutWorld) {
    let expected = [0.0, 0.0, 0.0, 1.0];
    for (actual, expected) in world.rotation_xyzw.iter().zip(expected.iter()) {
        assert!((actual - expected).abs() < 1e-4);
    }
}

#[then(regex = r"^the Bevy scale is (-?[\d.]+)$")]
async fn then_scale_is(world: &mut BevyoutWorld, scale: f32) {
    assert!((world.scale_out - scale).abs() < 1e-4);
}

#[given(regex = r#"^the cell selector text "([^"]*)"$"#)]
async fn given_cell_selector_text(world: &mut BevyoutWorld, text: String) {
    world.selector = Some(parse_cell_selector(&text).expect("selector should parse"));
}

#[then(regex = r"^it parses as the hexadecimal FormID 0x([0-9a-fA-F]+)$")]
async fn then_parses_as_form_id(world: &mut BevyoutWorld, hex: String) {
    let expected = u32::from_str_radix(&hex, 16).unwrap();
    assert_eq!(world.selector, Some(CellSelector::FormId(expected)));
}

#[then(regex = r#"^it parses as the editor ID "([^"]*)"$"#)]
async fn then_parses_as_editor_id(world: &mut BevyoutWorld, editor_id: String) {
    assert_eq!(world.selector, Some(CellSelector::EditorId(editor_id)));
}

#[given(regex = r#"^the raw asset path "(.*)"$"#)]
async fn given_raw_asset_path(world: &mut BevyoutWorld, raw: String) {
    world.normalized_path = Some(normalize_asset_path(&raw));
}

#[then(regex = r#"^the normalized asset path is "([^"]*)"$"#)]
async fn then_normalized_asset_path_is(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(world.normalized_path.as_deref(), Some(expected.as_str()));
}

// ---------------------------------------------------------------------
// manifest.feature
// ---------------------------------------------------------------------

#[given(regex = r#"^the golden manifest fixture "([^"]*)"$"#)]
async fn given_golden_manifest_fixture(world: &mut BevyoutWorld, path: String) {
    let root = env!("CARGO_MANIFEST_DIR");
    let text = std::fs::read_to_string(std::path::Path::new(root).join(&path))
        .unwrap_or_else(|error| panic!("reading golden fixture {path:?}: {error}"));
    let manifest: PreparedSceneManifest = ron::de::from_str(&text).unwrap_or_else(|error| {
        panic!(
            "golden fixture {path:?} no longer parses as PreparedSceneManifest \
             (schema drift): {error}"
        )
    });
    world.manifest = Some(manifest);
}

#[then("it parses as a PreparedSceneManifest")]
async fn then_it_parses(world: &mut BevyoutWorld) {
    assert!(world.manifest.is_some());
}

#[then(regex = r"^the schema version is (\d+)$")]
async fn then_schema_version_is(world: &mut BevyoutWorld, version: u32) {
    assert_eq!(world.manifest.as_ref().unwrap().schema_version, version);
}

#[then(regex = r#"^the placement "([^"]*)" is a Container$"#)]
async fn then_placement_is_container(world: &mut BevyoutWorld, editor_id: String) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    assert_eq!(placement.semantic, PreparedSemantic::Container);
}

#[then(regex = r#"^the placement "([^"]*)" has (\d+) inventory entr(?:y|ies)$"#)]
async fn then_placement_inventory_count(world: &mut BevyoutWorld, editor_id: String, count: usize) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    assert_eq!(placement.inventory.len(), count);
}

#[then(regex = r#"^the placement "([^"]*)" inventory includes "([^"]*)"$"#)]
async fn then_placement_inventory_includes(
    world: &mut BevyoutWorld,
    editor_id: String,
    item_editor_id: String,
) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    assert!(
        placement
            .inventory
            .iter()
            .any(|entry| entry.editor_id.as_deref() == Some(item_editor_id.as_str())),
        "{editor_id} inventory does not include {item_editor_id}"
    );
}

#[then(regex = r#"^the placement "([^"]*)" has an owner faction rank of (-?\d+)$"#)]
async fn then_placement_owner_faction_rank(world: &mut BevyoutWorld, editor_id: String, rank: i32) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    assert_eq!(placement.owner_faction_rank, Some(rank));
}

#[then(regex = r#"^the placement "([^"]*)" has an enable parent that pops in$"#)]
async fn then_placement_enable_parent_pops_in(world: &mut BevyoutWorld, editor_id: String) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    let enable_parent = placement
        .enable_parent
        .as_ref()
        .unwrap_or_else(|| panic!("{editor_id} has no enable_parent"));
    assert!(enable_parent.pop_in);
}

#[then(regex = r#"^the placement "([^"]*)" is a Door$"#)]
async fn then_placement_is_door(world: &mut BevyoutWorld, editor_id: String) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    assert!(matches!(placement.semantic, PreparedSemantic::Door(_)));
}

#[then(regex = r#"^the placement "([^"]*)" has a lock level of (\d+)$"#)]
async fn then_placement_lock_level(world: &mut BevyoutWorld, editor_id: String, level: i8) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    let PreparedSemantic::Door(door) = &placement.semantic else {
        panic!("{editor_id} is not a Door");
    };
    assert_eq!(door.lock_level, Some(level));
}

#[then(regex = r#"^the placement "([^"]*)" destination cell is 0x([0-9a-fA-F]+)$"#)]
async fn then_placement_destination_cell(world: &mut BevyoutWorld, editor_id: String, hex: String) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    let PreparedSemantic::Door(door) = &placement.semantic else {
        panic!("{editor_id} is not a Door");
    };
    let expected = u32::from_str_radix(&hex, 16).unwrap();
    let destination = door
        .destination
        .as_ref()
        .unwrap_or_else(|| panic!("{editor_id} door has no destination"));
    assert_eq!(destination.cell_form_id, expected);
}

#[then(regex = r"^the cell acoustic environment type is (\d+)$")]
async fn then_cell_acoustic_environment_type(world: &mut BevyoutWorld, expected: u32) {
    let manifest = world.manifest.as_ref().unwrap();
    assert_eq!(
        manifest.cell_audio.acoustic_environment_type,
        Some(expected)
    );
}

#[then(regex = r#"^the footstep set "([^"]*)" has a land clip$"#)]
async fn then_footstep_set_has_land_clip(world: &mut BevyoutWorld, surface: String) {
    let manifest = world.manifest.as_ref().unwrap();
    let set = manifest
        .footstep_sets
        .iter()
        .find(|set| set.surface == surface)
        .unwrap_or_else(|| panic!("no footstep set for surface {surface:?}"));
    assert!(!set.land.is_empty());
}

#[then(regex = r"^there is (\d+) hard landing clips?$")]
async fn then_hard_landing_clip_count(world: &mut BevyoutWorld, count: usize) {
    let manifest = world.manifest.as_ref().unwrap();
    assert_eq!(manifest.hard_landing_clips.len(), count);
}

#[then(regex = r#"^there is a retained NAVM payload with signature "([^"]*)"$"#)]
async fn then_navm_payload_signature(world: &mut BevyoutWorld, signature: String) {
    let manifest = world.manifest.as_ref().unwrap();
    assert!(
        manifest
            .navmeshes
            .iter()
            .flat_map(|navmesh| &navmesh.chunks)
            .any(|chunk| chunk.signature == signature),
        "no NAVM chunk with signature {signature:?}"
    );
}

// ---------------------------------------------------------------------
// asset_materials.feature
// ---------------------------------------------------------------------

#[given("an asset is static")]
async fn given_asset_is_static(world: &mut BevyoutWorld) {
    world.is_static = true;
}

#[given("an asset is dynamic")]
async fn given_asset_is_dynamic(world: &mut BevyoutWorld) {
    world.is_static = false;
}

#[when("its conversion profile is selected")]
async fn when_conversion_profile_selected(world: &mut BevyoutWorld) {
    world.conversion = Some(assets::asset_conversion(world.is_static));
}

#[then(regex = r"^the conversion is (QuickAo|Preserve)$")]
async fn then_conversion_is(world: &mut BevyoutWorld, expected: String) {
    let expected = match expected.as_str() {
        "QuickAo" => AssetConversion::QuickAo,
        "Preserve" => AssetConversion::Preserve,
        other => panic!("unknown conversion {other:?}"),
    };
    assert_eq!(world.conversion, Some(expected));
}

#[then(regex = r#"^the profile tag is "([^"]*)"$"#)]
async fn then_profile_tag_is(world: &mut BevyoutWorld, expected: String) {
    let conversion = world
        .conversion
        .expect("conversion profile not selected yet");
    assert_eq!(conversion.profile_tag(), expected);
}

#[given(
    regex = r#"^an imported material has NIFTools emissive color \(([\d.]+), ([\d.]+), ([\d.]+)\)$"#
)]
async fn given_niftools_emissive_color(
    world: &mut BevyoutWorld,
    red: String,
    green: String,
    blue: String,
) {
    world.authored_emission = Some([
        red.parse().unwrap(),
        green.parse().unwrap(),
        blue.parse().unwrap(),
    ]);
}

#[given(regex = r"^the source emission multiplier is ([\d.]+)$")]
async fn given_source_emission_multiplier(world: &mut BevyoutWorld, strength: String) {
    world.source_emission_strength = Some(strength.parse().unwrap());
}

#[given("an explicit emission is present")]
async fn given_explicit_emission_present(world: &mut BevyoutWorld) {
    world.explicit_emission = true;
}

#[given("an emissive bulb override is present")]
async fn given_emissive_bulb_override_present(world: &mut BevyoutWorld) {
    world.bulb_emission_override = true;
}

#[given("a glow texture override is present")]
async fn given_glow_texture_override_present(world: &mut BevyoutWorld) {
    world.glow_emission_override = true;
}

#[when("its material emission policy is evaluated")]
async fn when_material_emission_policy_evaluated(world: &mut BevyoutWorld) {
    let color = world
        .authored_emission
        .expect("authored emission color was not provided");
    world.emission_policy = Some(assets::material_emission_policy(
        color,
        world.source_emission_strength.unwrap_or(1.0),
        world.explicit_emission,
        world.bulb_emission_override,
        world.glow_emission_override,
    ));
}

#[then(regex = r#"^the exported emission color is \(([\d.]+), ([\d.]+), ([\d.]+)\)$"#)]
async fn then_exported_emission_color(
    world: &mut BevyoutWorld,
    red: String,
    green: String,
    blue: String,
) {
    let expected = [
        red.parse::<f32>().unwrap(),
        green.parse::<f32>().unwrap(),
        blue.parse::<f32>().unwrap(),
    ];
    match world.emission_policy {
        Some(assets::MaterialEmissionPolicy::Authored(emission)) => {
            assert_eq!(emission.color, expected);
        }
        other => panic!("expected authored emission, got {other:?}"),
    }
}

#[then(regex = r"^the exported emission strength is ([\d.]+)$")]
async fn then_exported_emission_strength(world: &mut BevyoutWorld, expected: String) {
    let expected = expected.parse::<f32>().unwrap();
    match world.emission_policy {
        Some(assets::MaterialEmissionPolicy::Authored(emission)) => {
            assert_eq!(emission.strength, expected);
        }
        other => panic!("expected authored emission, got {other:?}"),
    }
}

#[then("the exported material has no emission")]
async fn then_exported_material_has_no_emission(world: &mut BevyoutWorld) {
    assert_eq!(
        world.emission_policy,
        Some(assets::MaterialEmissionPolicy::None)
    );
}

#[then(regex = r"^the selected emission source is (Authored|Explicit|Bulb|Glow|None)$")]
async fn then_selected_emission_source(world: &mut BevyoutWorld, expected: String) {
    let actual = world
        .emission_policy
        .expect("emission policy was not evaluated");
    let matches = match expected.as_str() {
        "Authored" => matches!(actual, assets::MaterialEmissionPolicy::Authored(_)),
        "Explicit" => actual == assets::MaterialEmissionPolicy::Explicit,
        "Bulb" => actual == assets::MaterialEmissionPolicy::Bulb,
        "Glow" => actual == assets::MaterialEmissionPolicy::Glow,
        "None" => actual == assets::MaterialEmissionPolicy::None,
        other => panic!("unknown emission source {other}"),
    };
    assert!(matches, "expected {expected}, got {actual:?}");
}

// ---------------------------------------------------------------------
// cell_map.feature
// ---------------------------------------------------------------------

fn parse_hex(hex: &str) -> u32 {
    u32::from_str_radix(hex, 16)
        .unwrap_or_else(|error| panic!("invalid hex FormID {hex:?}: {error}"))
}

#[given(regex = r#"^a cell map with an interior cell "([^"]*)" 0x([0-9a-fA-F]+) and no grid$"#)]
async fn given_interior_cell(world: &mut BevyoutWorld, editor_id: String, hex: String) {
    world.cell_map_cells.push(cell_map::CellMapEntry {
        form_id: parse_hex(&hex),
        editor_id: Some(editor_id),
        interior: true,
        worldspace_form_id: None,
        grid: None,
    });
}

#[given(
    regex = r#"^the cell map has an exterior cell "([^"]*)" 0x([0-9a-fA-F]+) with grid (-?\d+), (-?\d+) in worldspace 0x([0-9a-fA-F]+)$"#
)]
async fn given_exterior_cell(
    world: &mut BevyoutWorld,
    editor_id: String,
    hex: String,
    x: i32,
    y: i32,
    worldspace_hex: String,
) {
    let worldspace_form_id = parse_hex(&worldspace_hex);
    world.cell_map_cells.push(cell_map::CellMapEntry {
        form_id: parse_hex(&hex),
        editor_id: Some(editor_id),
        interior: false,
        worldspace_form_id: Some(worldspace_form_id),
        grid: Some((x, y)),
    });
    if !world
        .cell_map_worldspaces
        .iter()
        .any(|worldspace| worldspace.form_id == worldspace_form_id)
    {
        world.cell_map_worldspaces.push(cell_map::WorldspaceEntry {
            form_id: worldspace_form_id,
            editor_id: None,
            name: None,
        });
    }
}

#[given(
    regex = r"^a door edge from cell 0x([0-9a-fA-F]+) door 0x([0-9a-fA-F]+) to cell 0x([0-9a-fA-F]+) door 0x([0-9a-fA-F]+)$"
)]
async fn given_door_edge(
    world: &mut BevyoutWorld,
    source_cell: String,
    source_door: String,
    destination_cell: String,
    destination_door: String,
) {
    world.cell_map_doors.push(cell_map::DoorEdge {
        source_cell_form_id: parse_hex(&source_cell),
        door_reference_form_id: parse_hex(&source_door),
        destination_cell_form_id: parse_hex(&destination_cell),
        destination_door_reference_form_id: parse_hex(&destination_door),
        position: [1.0, 2.0, 3.0],
        rotation: [0.0, 0.0, 0.0],
    });
}

#[given(regex = r"^(\d+) unresolved door teleports?$")]
async fn given_unresolved_teleports(world: &mut BevyoutWorld, count: u32) {
    world.cell_map_unresolved += count;
}

fn build_cell_map(world: &BevyoutWorld) -> cell_map::CellMap {
    cell_map::CellMap::build(
        "fingerprint".into(),
        world.cell_map_worldspaces.clone(),
        world.cell_map_cells.clone(),
        world.cell_map_doors.clone(),
        world.cell_map_unresolved,
    )
}

#[when("the cell map is built")]
async fn when_cell_map_built(world: &mut BevyoutWorld) {
    world.cell_map = Some(build_cell_map(world));
}

#[when("the cell map is built twice from the same input")]
async fn when_cell_map_built_twice(world: &mut BevyoutWorld) {
    world.cell_map_ron_a = Some(build_cell_map(world).to_ron().unwrap());
    world.cell_map_ron_b = Some(build_cell_map(world).to_ron().unwrap());
}

#[then(regex = r"^cell 0x([0-9a-fA-F]+) has no worldspace$")]
async fn then_cell_has_no_worldspace(world: &mut BevyoutWorld, hex: String) {
    let form_id = parse_hex(&hex);
    let map = world.cell_map.as_ref().expect("cell map not built yet");
    let cell = map
        .cells
        .iter()
        .find(|cell| cell.form_id == form_id)
        .unwrap_or_else(|| panic!("no cell {form_id:08x} in built map"));
    assert_eq!(cell.worldspace_form_id, None);
}

#[then(
    regex = r"^cell 0x([0-9a-fA-F]+) is in worldspace 0x([0-9a-fA-F]+) with grid (-?\d+), (-?\d+)$"
)]
async fn then_cell_in_worldspace(
    world: &mut BevyoutWorld,
    hex: String,
    worldspace_hex: String,
    x: i32,
    y: i32,
) {
    let form_id = parse_hex(&hex);
    let worldspace_form_id = parse_hex(&worldspace_hex);
    let map = world.cell_map.as_ref().expect("cell map not built yet");
    let cell = map
        .cells
        .iter()
        .find(|cell| cell.form_id == form_id)
        .unwrap_or_else(|| panic!("no cell {form_id:08x} in built map"));
    assert_eq!(cell.worldspace_form_id, Some(worldspace_form_id));
    assert_eq!(cell.grid, Some((x, y)));
}

#[then(regex = r"^there (?:is|are) (\d+) door edges?$")]
async fn then_door_edge_count(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(
        world
            .cell_map
            .as_ref()
            .expect("cell map not built yet")
            .doors
            .len(),
        count
    );
}

#[then(regex = r"^there (?:is|are) (\d+) unresolved doors?$")]
async fn then_unresolved_door_count(world: &mut BevyoutWorld, count: u32) {
    assert_eq!(
        world
            .cell_map
            .as_ref()
            .expect("cell map not built yet")
            .unresolved_door_count,
        count
    );
}

#[then("both RON outputs are byte-identical")]
async fn then_ron_outputs_are_byte_identical(world: &mut BevyoutWorld) {
    assert_eq!(world.cell_map_ron_a, world.cell_map_ron_b);
}

// ---------------------------------------------------------------------
// prepare_selectors.feature
// ---------------------------------------------------------------------

#[given(regex = r#"^cell 0x([0-9a-fA-F]+) "([^"]*)" is an interior cell$"#)]
async fn given_selector_interior_cell(world: &mut BevyoutWorld, hex: String, editor_id: String) {
    world.cells.push(CellSummary {
        form_id: parse_hex(&hex),
        editor_id: Some(editor_id),
        name: None,
        interior: true,
        worldspace_form_id: None,
    });
}

#[given(regex = r#"^cell 0x([0-9a-fA-F]+) "([^"]*)" is an exterior cell$"#)]
async fn given_selector_exterior_cell(world: &mut BevyoutWorld, hex: String, editor_id: String) {
    world.cells.push(CellSummary {
        form_id: parse_hex(&hex),
        editor_id: Some(editor_id),
        name: None,
        interior: false,
        worldspace_form_id: None,
    });
}

#[given(
    regex = r#"^cell 0x([0-9a-fA-F]+) "([^"]*)" is an exterior cell in worldspace 0x([0-9a-fA-F]+)$"#
)]
async fn given_exterior_cell_in_worldspace(
    world: &mut BevyoutWorld,
    hex: String,
    editor_id: String,
    worldspace_hex: String,
) {
    world.cells.push(CellSummary {
        form_id: parse_hex(&hex),
        editor_id: Some(editor_id),
        name: None,
        interior: false,
        worldspace_form_id: Some(parse_hex(&worldspace_hex)),
    });
}

#[given(regex = r#"^worldspace 0x([0-9a-fA-F]+) is named "([^"]*)"$"#)]
async fn given_worldspace_named(world: &mut BevyoutWorld, hex: String, name: String) {
    world.worldspace_names.push((parse_hex(&hex), name));
}

#[when("cells are selected with --all-interiors")]
async fn when_selected_all_interiors(world: &mut BevyoutWorld) {
    let spec = SelectionSpec {
        all_interiors: true,
        ..Default::default()
    };
    world.selection_result = Some(
        resolve_selection(&world.cells, &world.worldspace_names, &spec)
            .map_err(|error| error.to_string()),
    );
}

#[when(regex = r#"^cells are selected with selectors "([^"]*)"$"#)]
async fn when_selected_explicit(world: &mut BevyoutWorld, list: String) {
    let explicit = list
        .split(',')
        .map(|entry| entry.trim().to_string())
        .collect();
    let spec = SelectionSpec {
        explicit,
        ..Default::default()
    };
    world.selection_result = Some(
        resolve_selection(&world.cells, &world.worldspace_names, &spec)
            .map_err(|error| error.to_string()),
    );
}

#[when(regex = r#"^cells are selected with worldspace "([^"]*)"$"#)]
async fn when_selected_worldspace(world: &mut BevyoutWorld, name: String) {
    let spec = SelectionSpec {
        worldspace: Some(name),
        ..Default::default()
    };
    world.selection_result = Some(
        resolve_selection(&world.cells, &world.worldspace_names, &spec)
            .map_err(|error| error.to_string()),
    );
}

#[then(regex = r#"^the resolved cell selection is "([^"]*)"$"#)]
async fn then_resolved_selection_is(world: &mut BevyoutWorld, list: String) {
    let expected: Vec<u32> = list
        .split(',')
        .map(|entry| parse_hex(entry.trim()))
        .collect();
    let resolved = world
        .selection_result
        .take()
        .expect("a selection was not resolved")
        .expect("selection resolution failed");
    assert_eq!(resolved, expected);
}

#[then(regex = r#"^the cell selection fails naming worldspace "([^"]*)"$"#)]
async fn then_selection_fails_naming_worldspace(world: &mut BevyoutWorld, name: String) {
    let error = world
        .selection_result
        .take()
        .expect("a selection was not attempted")
        .expect_err("expected selection resolution to fail");
    assert!(
        error.contains(&name),
        "error {error:?} does not mention worldspace {name:?}"
    );
}

// ---------------------------------------------------------------------
// batch_session.feature
// ---------------------------------------------------------------------

#[given("a fresh batch cache")]
async fn given_fresh_batch_cache(world: &mut BevyoutWorld) {
    world.batch_physics_cache = batch_cache::KeyedBatchCache::default();
    world.batch_asset_totals = batch_cache::BatchAssetTotals::default();
}

#[when(regex = r#"^cell "[^"]*" reads physics key "([^"]*)"$"#)]
async fn when_cell_reads_physics_key(world: &mut BevyoutWorld, key: String) {
    world
        .batch_physics_cache
        .get_or_insert_with(&key, || Ok(()))
        .expect("synthetic build never fails");
}

#[when(
    regex = r#"^cell "[^"]*" reports asset cache reused (\d+), built (\d+), invalid (\d+), explicit (\d+)$"#
)]
async fn when_cell_reports_asset_cache(
    world: &mut BevyoutWorld,
    reused: usize,
    built: usize,
    invalid: usize,
    explicit: usize,
) {
    world
        .batch_asset_totals
        .add(reused, built, invalid, explicit);
}

#[then(regex = r"^physics reads is (\d+)$")]
async fn then_physics_reads_is(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(world.batch_physics_cache.accesses(), count);
}

#[then(regex = r"^physics hits is (\d+)$")]
async fn then_physics_hits_is(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(world.batch_physics_cache.hits, count);
}

#[then(regex = r#"^the batch cache summary line is "([^"]*)"$"#)]
async fn then_batch_cache_summary_line_is(world: &mut BevyoutWorld, expected: String) {
    let line = batch_cache::batch_cache_summary_line(
        world.batch_asset_totals,
        world.batch_physics_cache.accesses(),
        world.batch_physics_cache.hits,
    );
    assert_eq!(line, expected);
}

#[when("the cell map is written to the batch cache dir")]
async fn when_cell_map_written_to_batch_cache_dir(world: &mut BevyoutWorld) {
    let map = build_cell_map(world);
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "bevyout-cucumber-batch-cache-{}-{unique}",
        std::process::id()
    ));
    let path = batch_cache::write_cell_map(&dir, &map).expect("writing the cell map must succeed");
    world.batch_cache_dir = Some(dir);
    world.written_cell_map_path = Some(path);
    // Reuses `cell_map.feature`'s own `Then there are N door edges` step,
    // which reads from `world.cell_map`.
    world.cell_map = Some(map.clone());
    world.written_cell_map = Some(map);
}

#[then(regex = r#"^the written cell map file exists at "([^"]*)" under the batch cache dir$"#)]
async fn then_written_cell_map_file_exists(world: &mut BevyoutWorld, file_name: String) {
    let path = world
        .written_cell_map_path
        .as_ref()
        .expect("cell map was not written yet");
    let dir = world
        .batch_cache_dir
        .as_ref()
        .expect("cell map was not written yet");
    assert_eq!(path, &dir.join(&file_name));
    assert!(path.is_file(), "{path:?} was not written");
    std::fs::remove_dir_all(dir).ok();
}

#[then(regex = r"^the written cell map has (\d+) cells$")]
async fn then_written_cell_map_has_cells(world: &mut BevyoutWorld, count: usize) {
    let map = world
        .written_cell_map
        .as_ref()
        .expect("cell map was not written yet");
    assert_eq!(map.cells.len(), count);
}

// ---------------------------------------------------------------------
// preload_policy.feature (issue #51)
// ---------------------------------------------------------------------

#[given(regex = r"^a door edge from cell 0x([0-9a-fA-F]+) to cell 0x([0-9a-fA-F]+)$")]
async fn given_preload_door_edge(world: &mut BevyoutWorld, source: String, destination: String) {
    world.preload_doors.push(policy::DoorLink {
        source_cell_form_id: parse_hex(&source),
        destination_cell_form_id: parse_hex(&destination),
    });
}

#[given(regex = r"^cell 0x([0-9a-fA-F]+) is prepared$")]
async fn given_preload_cell_prepared(world: &mut BevyoutWorld, hex: String) {
    world.preload_prepared.insert(parse_hex(&hex));
}

#[given(regex = r"^cell 0x([0-9a-fA-F]+) is resident$")]
async fn given_preload_cell_resident(world: &mut BevyoutWorld, hex: String) {
    world.preload_resident.push(parse_hex(&hex));
}

#[given(regex = r"^the active cell is 0x([0-9a-fA-F]+)$")]
async fn given_preload_active_cell(world: &mut BevyoutWorld, hex: String) {
    world.preload_active_cell = parse_hex(&hex);
}

#[given(regex = r"^the resident cell limit is (\d+)$")]
async fn given_preload_resident_cell_limit(world: &mut BevyoutWorld, budget: usize) {
    world.preload_budget = budget;
}

#[when("the preload plan is computed")]
async fn when_preload_plan_computed(world: &mut BevyoutWorld) {
    let graph = policy::CellGraph::build(&world.preload_doors);
    world.preload_plan = Some(graph.plan(
        world.preload_active_cell,
        &world.preload_resident,
        &world.preload_prepared,
        world.preload_budget,
    ));
}

#[then(regex = r"^the plan loads cell 0x([0-9a-fA-F]+)$")]
async fn then_plan_loads_cell(world: &mut BevyoutWorld, hex: String) {
    let form_id = parse_hex(&hex);
    let plan = world
        .preload_plan
        .as_ref()
        .expect("preload plan not computed yet");
    assert!(
        plan.load.contains(&form_id),
        "expected plan to load {form_id:08x}, load = {:?}",
        plan.load
    );
}

#[then(regex = r"^the plan does not load cell 0x([0-9a-fA-F]+)$")]
async fn then_plan_does_not_load_cell(world: &mut BevyoutWorld, hex: String) {
    let form_id = parse_hex(&hex);
    let plan = world
        .preload_plan
        .as_ref()
        .expect("preload plan not computed yet");
    assert!(
        !plan.load.contains(&form_id),
        "expected plan not to load {form_id:08x}, load = {:?}",
        plan.load
    );
}

#[then(regex = r"^the plan evicts cell 0x([0-9a-fA-F]+)$")]
async fn then_plan_evicts_cell(world: &mut BevyoutWorld, hex: String) {
    let form_id = parse_hex(&hex);
    let plan = world
        .preload_plan
        .as_ref()
        .expect("preload plan not computed yet");
    assert!(
        plan.evict.contains(&form_id),
        "expected plan to evict {form_id:08x}, evict = {:?}",
        plan.evict
    );
}

#[then(regex = r"^the plan does not evict cell 0x([0-9a-fA-F]+)$")]
async fn then_plan_does_not_evict_cell(world: &mut BevyoutWorld, hex: String) {
    let form_id = parse_hex(&hex);
    let plan = world
        .preload_plan
        .as_ref()
        .expect("preload plan not computed yet");
    assert!(
        !plan.evict.contains(&form_id),
        "expected plan not to evict {form_id:08x}, evict = {:?}",
        plan.evict
    );
}

#[then("the plan loads nothing")]
async fn then_plan_loads_nothing(world: &mut BevyoutWorld) {
    let plan = world
        .preload_plan
        .as_ref()
        .expect("preload plan not computed yet");
    assert!(
        plan.load.is_empty(),
        "expected no loads, got {:?}",
        plan.load
    );
}

#[then("the plan evicts nothing")]
async fn then_plan_evicts_nothing(world: &mut BevyoutWorld) {
    let plan = world
        .preload_plan
        .as_ref()
        .expect("preload plan not computed yet");
    assert!(
        plan.evict.is_empty(),
        "expected no evictions, got {:?}",
        plan.evict
    );
}

// ---------------------------------------------------------------------
// resumable_prepare.feature (issue #48)
// ---------------------------------------------------------------------

fn job_manifest_temp_path() -> std::path::PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir()
        .join(format!(
            "bevyout-cucumber-jobs-{}-{unique}",
            std::process::id()
        ))
        .join("prepare_jobs.ron")
}

fn parse_hex_list(list: &str) -> Vec<u32> {
    list.split(',')
        .map(|entry| parse_hex(entry.trim()))
        .collect()
}

#[given(regex = r#"^a fresh job manifest with fingerprint "([^"]*)"$"#)]
async fn given_fresh_job_manifest(world: &mut BevyoutWorld, fingerprint: String) {
    world.job_manifest = Some(jobs::JobManifest::new(fingerprint));
}

#[given(regex = r"^cell 0x([0-9a-fA-F]+) is marked done$")]
async fn given_job_cell_done(world: &mut BevyoutWorld, hex: String) {
    world
        .job_manifest
        .as_mut()
        .expect("job manifest not created yet")
        .set_status(parse_hex(&hex), jobs::JobStatus::Done);
}

#[given(regex = r"^cell 0x([0-9a-fA-F]+) is marked pending$")]
async fn given_job_cell_pending(world: &mut BevyoutWorld, hex: String) {
    world
        .job_manifest
        .as_mut()
        .expect("job manifest not created yet")
        .set_status(parse_hex(&hex), jobs::JobStatus::Pending);
}

#[given(regex = r#"^cell 0x([0-9a-fA-F]+) is marked failed with reason "([^"]*)"$"#)]
async fn given_job_cell_failed(world: &mut BevyoutWorld, hex: String, reason: String) {
    world
        .job_manifest
        .as_mut()
        .expect("job manifest not created yet")
        .set_status(parse_hex(&hex), jobs::JobStatus::Failed(reason));
}

#[given("the manifest is written and reloaded")]
async fn given_job_manifest_written_and_reloaded(world: &mut BevyoutWorld) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    let path = job_manifest_temp_path();
    manifest
        .write_atomic(&path)
        .expect("writing the job manifest must succeed");
    let reloaded = jobs::JobManifest::load_or_new(&path, &manifest.content_fingerprint)
        .expect("reloading the job manifest must succeed");
    world.job_manifest_path = Some(path);
    world.job_manifest = Some(reloaded);
}

#[given("the manifest is written to disk")]
async fn given_job_manifest_written_to_disk(world: &mut BevyoutWorld) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    let path = job_manifest_temp_path();
    manifest
        .write_atomic(&path)
        .expect("writing the job manifest must succeed");
    world.job_manifest_path = Some(path);
}

#[when(regex = r#"^the manifest is reloaded with fingerprint "([^"]*)"$"#)]
async fn when_job_manifest_reloaded_with_fingerprint(
    world: &mut BevyoutWorld,
    fingerprint: String,
) {
    let path = world
        .job_manifest_path
        .as_ref()
        .expect("job manifest was not written yet");
    world.job_manifest = Some(
        jobs::JobManifest::load_or_new(path, &fingerprint)
            .expect("reloading the job manifest must succeed"),
    );
}

#[when(regex = r#"^cells "([^"]*)" are resumed without force$"#)]
async fn when_job_cells_resumed_without_force(world: &mut BevyoutWorld, list: String) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    let selection = parse_hex_list(&list);
    world.job_resume_result = Some(jobs::filter_resume(manifest, &selection, false));
}

#[when(regex = r#"^cells "([^"]*)" are resumed with force$"#)]
async fn when_job_cells_resumed_with_force(world: &mut BevyoutWorld, list: String) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    let selection = parse_hex_list(&list);
    world.job_resume_result = Some(jobs::filter_resume(manifest, &selection, true));
}

#[then(regex = r"^cell 0x([0-9a-fA-F]+) has status done$")]
async fn then_job_cell_status_done(world: &mut BevyoutWorld, hex: String) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    assert_eq!(
        manifest.status(parse_hex(&hex)),
        Some(&jobs::JobStatus::Done)
    );
}

#[then(regex = r#"^cell 0x([0-9a-fA-F]+) has status failed with reason "([^"]*)"$"#)]
async fn then_job_cell_status_failed(world: &mut BevyoutWorld, hex: String, reason: String) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    assert_eq!(
        manifest.status(parse_hex(&hex)),
        Some(&jobs::JobStatus::Failed(reason))
    );
}

#[then(regex = r#"^the cells to run are "([^"]*)"$"#)]
async fn then_job_cells_to_run_are(world: &mut BevyoutWorld, list: String) {
    let expected = parse_hex_list(&list);
    let (to_run, _) = world
        .job_resume_result
        .as_ref()
        .expect("cells were not resumed yet");
    assert_eq!(to_run, &expected);
}

#[then(regex = r"^(\d+) cell\(s\) were skipped$")]
async fn then_job_cells_skipped(world: &mut BevyoutWorld, count: usize) {
    let (_, skipped) = world
        .job_resume_result
        .as_ref()
        .expect("cells were not resumed yet");
    assert_eq!(*skipped, count);
}

#[then(regex = r#"^the failed cells are "([^"]*)"$"#)]
async fn then_job_failed_cells_are(world: &mut BevyoutWorld, list: String) {
    let expected = parse_hex_list(&list);
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    assert_eq!(manifest.failed_form_ids(), expected);
}

#[then("the manifest has no recorded cells")]
async fn then_job_manifest_has_no_recorded_cells(world: &mut BevyoutWorld) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    assert!(manifest.jobs.is_empty());
}

// ---------------------------------------------------------------------
// instant_swap.feature (issue #52)
// ---------------------------------------------------------------------

#[given(regex = r"^the destination cell residency is (Ready|Loading|Absent)$")]
async fn given_swap_residency(world: &mut BevyoutWorld, residency: String) {
    world.swap_residency = Some(match residency.as_str() {
        "Ready" => swap_policy::Residency::Ready,
        "Loading" => swap_policy::Residency::Loading,
        _ => swap_policy::Residency::Absent,
    });
}

#[given("the destination manifest exists on disk")]
async fn given_swap_manifest_exists(world: &mut BevyoutWorld) {
    world.swap_manifest_exists = true;
}

#[given("the destination manifest does not exist on disk")]
async fn given_swap_manifest_does_not_exist(world: &mut BevyoutWorld) {
    world.swap_manifest_exists = false;
}

#[when("the swap decision is computed")]
async fn when_swap_decision_computed(world: &mut BevyoutWorld) {
    let residency = world
        .swap_residency
        .expect("destination cell residency not given");
    world.swap_decision = Some(swap_policy::swap_decision(
        world.swap_manifest_exists,
        residency,
    ));
}

#[then(regex = r"^the swap decision is (Instant|Fallback)$")]
async fn then_swap_decision_is(world: &mut BevyoutWorld, expected: String) {
    let decision = world.swap_decision.expect("swap decision not computed yet");
    let expected = match expected.as_str() {
        "Instant" => swap_policy::SwapDecision::Instant,
        _ => swap_policy::SwapDecision::Fallback,
    };
    assert_eq!(decision, expected);
}

#[given("a fallback load that succeeds")]
async fn given_fallback_load_succeeds(world: &mut BevyoutWorld) {
    world.swap_fallback_load_ok = true;
}

#[given("a fallback load that fails")]
async fn given_fallback_load_fails(world: &mut BevyoutWorld) {
    world.swap_fallback_load_ok = false;
}

#[when("the fallback outcome is computed")]
async fn when_fallback_outcome_computed(world: &mut BevyoutWorld) {
    world.swap_fallback_outcome = Some(swap_policy::fallback_outcome(world.swap_fallback_load_ok));
}

#[then(regex = r"^the fallback outcome is (Proceed|ReturnToSource)$")]
async fn then_fallback_outcome_is(world: &mut BevyoutWorld, expected: String) {
    let outcome = world
        .swap_fallback_outcome
        .expect("fallback outcome not computed yet");
    let expected = match expected.as_str() {
        "Proceed" => swap_policy::FallbackOutcome::Proceed,
        _ => swap_policy::FallbackOutcome::ReturnToSource,
    };
    assert_eq!(outcome, expected);
}

// ---------------------------------------------------------------------
// physics_readiness.feature
// ---------------------------------------------------------------------

#[given(regex = r#"^collider placements have kinds "([^"]*)"$"#)]
async fn given_collider_placements_have_kinds(world: &mut BevyoutWorld, kinds: String) {
    world.collider_work = kinds
        .split(',')
        .enumerate()
        .map(|(index, kind)| {
            let dynamic = match kind.trim() {
                "dynamic" => true,
                "static" | "keyframed" => false,
                other => panic!("unknown collider kind {other:?}"),
            };
            (index, dynamic)
        })
        .collect();
}

#[when("collider placement work is partitioned")]
async fn when_collider_work_partitioned(world: &mut BevyoutWorld) {
    world.collider_phase_partitions = Some(swap_policy::partition_collider_indices(
        world.collider_work.iter().copied(),
    ));
}

fn parse_index_list(list: &str) -> Vec<usize> {
    list.split(',')
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.trim().parse().expect("valid collider index"))
        .collect()
}

#[then(regex = r#"^static collider indices are "([^"]*)"$"#)]
async fn then_static_collider_indices_are(world: &mut BevyoutWorld, expected: String) {
    let (static_indices, _) = world
        .collider_phase_partitions
        .as_ref()
        .expect("collider work was not partitioned");
    assert_eq!(*static_indices, parse_index_list(&expected));
}

#[then(regex = r#"^dynamic collider indices are "([^"]*)"$"#)]
async fn then_dynamic_collider_indices_are(world: &mut BevyoutWorld, expected: String) {
    let (_, dynamic_indices) = world
        .collider_phase_partitions
        .as_ref()
        .expect("collider work was not partitioned");
    assert_eq!(*dynamic_indices, parse_index_list(&expected));
}

#[then("static collision is required before dynamic bodies")]
async fn then_static_collision_is_required_before_dynamic_bodies(world: &mut BevyoutWorld) {
    let (static_indices, dynamic_indices) = world
        .collider_phase_partitions
        .as_ref()
        .expect("collider work was not partitioned");
    assert!(!static_indices.is_empty());
    assert!(!dynamic_indices.is_empty());
    assert_eq!(
        swap_policy::next_collider_build_phase(false, true, true),
        swap_policy::ColliderBuildPhase::Dynamic
    );
    assert_ne!(
        swap_policy::next_collider_build_phase(false, true, false),
        swap_policy::ColliderBuildPhase::Ready
    );
}

// ---------------------------------------------------------------------
// fingerprints.feature (issue #49) -- appended section, do not interleave
// with steps above; new steps for this issue belong below this marker.
// ---------------------------------------------------------------------

fn parse_fingerprints(
    plugin: String,
    converter: String,
    physics: String,
    prepare_pipeline: String,
) -> fingerprints::CellFingerprints {
    fingerprints::CellFingerprints {
        plugin_content_set: plugin,
        converter,
        physics,
        prepare_pipeline,
    }
}

#[given(
    regex = r#"^cell 0x([0-9a-fA-F]+) has recorded fingerprints plugin "([^"]*)" converter "([^"]*)" physics "([^"]*)" prepare_pipeline "([^"]*)"$"#
)]
async fn given_cell_recorded_fingerprints(
    world: &mut BevyoutWorld,
    hex: String,
    plugin: String,
    converter: String,
    physics: String,
    prepare_pipeline: String,
) {
    world
        .job_manifest
        .as_mut()
        .expect("job manifest not created yet")
        .record_fingerprints(
            parse_hex(&hex),
            parse_fingerprints(plugin, converter, physics, prepare_pipeline),
        );
}

#[when(
    regex = r#"^cell 0x([0-9a-fA-F]+) is checked against current fingerprints plugin "([^"]*)" converter "([^"]*)" physics "([^"]*)" prepare_pipeline "([^"]*)"$"#
)]
async fn when_cell_checked_against_current(
    world: &mut BevyoutWorld,
    hex: String,
    plugin: String,
    converter: String,
    physics: String,
    prepare_pipeline: String,
) {
    let current = parse_fingerprints(plugin, converter, physics, prepare_pipeline);
    let recorded = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet")
        .fingerprints_for(parse_hex(&hex))
        .cloned();
    world.fingerprint_stale_components =
        Some(fingerprints::stale_components(recorded.as_ref(), &current));
    world.fingerprint_current = Some(current);
}

#[then("the cell is valid")]
async fn then_cell_is_valid(world: &mut BevyoutWorld) {
    let stale = world
        .fingerprint_stale_components
        .as_ref()
        .expect("cell was not checked yet");
    assert!(
        stale.is_empty(),
        "expected no stale components, got {stale:?}"
    );
}

#[then(regex = r#"^the cell is stale in component "([^"]*)"$"#)]
async fn then_cell_is_stale_in_component(world: &mut BevyoutWorld, component: String) {
    let stale = world
        .fingerprint_stale_components
        .as_ref()
        .expect("cell was not checked yet");
    let labels: Vec<&str> = stale.iter().map(|c| c.label()).collect();
    assert!(
        labels.contains(&component.as_str()),
        "expected {component:?} among stale components, got {labels:?}"
    );
}

#[when(
    regex = r#"^cells "([^"]*)" are resumed without force against current fingerprints plugin "([^"]*)" converter "([^"]*)" physics "([^"]*)" prepare_pipeline "([^"]*)"$"#
)]
async fn when_cells_resumed_checked(
    world: &mut BevyoutWorld,
    list: String,
    plugin: String,
    converter: String,
    physics: String,
    prepare_pipeline: String,
) {
    let current = parse_fingerprints(plugin, converter, physics, prepare_pipeline);
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    let selection = parse_hex_list(&list);
    world.fingerprint_resume_result = Some(jobs::filter_resume_checked(
        manifest, &selection, false, &current,
    ));
}

#[then(regex = r#"^the checked cells to run are "([^"]*)"$"#)]
async fn then_checked_cells_to_run_are(world: &mut BevyoutWorld, list: String) {
    let expected = parse_hex_list(&list);
    let (to_run, _, _) = world
        .fingerprint_resume_result
        .as_ref()
        .expect("cells were not resumed yet");
    assert_eq!(to_run, &expected);
}

#[then(regex = r"^(\d+) cell\(s\) were checked as skipped$")]
async fn then_checked_cells_skipped(world: &mut BevyoutWorld, count: usize) {
    let (_, skipped, _) = world
        .fingerprint_resume_result
        .as_ref()
        .expect("cells were not resumed yet");
    assert_eq!(*skipped, count);
}

#[then(regex = r"^(\d+) cell\(s\) were stale$")]
async fn then_checked_cells_stale(world: &mut BevyoutWorld, count: usize) {
    let (_, _, stale) = world
        .fingerprint_resume_result
        .as_ref()
        .expect("cells were not resumed yet");
    assert_eq!(stale.len(), count);
}

// ---------------------------------------------------------------------
// first_reveal.feature (issue #55)
// ---------------------------------------------------------------------

#[given(regex = r"^(\d+) reveal candidates evenly spaced from the arrival point$")]
async fn given_reveal_candidates_evenly_spaced(world: &mut BevyoutWorld, count: usize) {
    for i in 0..count {
        world
            .reveal_candidates
            .push(reveal_policy::RevealCandidate {
                index: world.reveal_candidates.len(),
                position: [i as f32, 0.0, 0.0],
            });
    }
}

#[given(regex = r"^a reveal candidate at distance (\d+) from the arrival point$")]
async fn given_reveal_candidate_at_distance(world: &mut BevyoutWorld, distance: f32) {
    world
        .reveal_candidates
        .push(reveal_policy::RevealCandidate {
            index: world.reveal_candidates.len(),
            position: [distance, 0.0, 0.0],
        });
}

#[given(regex = r"^the reveal budget is (\d+)$")]
async fn given_reveal_budget(world: &mut BevyoutWorld, budget: usize) {
    world.reveal_budget = budget;
}

#[when("the reveal chunks are planned")]
async fn when_reveal_chunks_planned(world: &mut BevyoutWorld) {
    world.reveal_chunks = reveal_policy::plan_reveal_chunks(
        &world.reveal_candidates,
        [0.0, 0.0, 0.0],
        world.reveal_budget,
    );
}

#[then(regex = r"^there (?:is|are) (\d+) reveal chunks?$")]
async fn then_reveal_chunk_count_is(world: &mut BevyoutWorld, expected: usize) {
    assert_eq!(
        world.reveal_chunks.len(),
        expected,
        "expected {expected} reveal chunks, got {:?}",
        world.reveal_chunks
    );
}

#[then("every reveal candidate appears in exactly one chunk")]
async fn then_every_reveal_candidate_appears_once(world: &mut BevyoutWorld) {
    let mut seen: Vec<usize> = world
        .reveal_chunks
        .iter()
        .flat_map(|chunk| chunk.iter().copied())
        .collect();
    seen.sort_unstable();
    let expected: Vec<usize> = (0..world.reveal_candidates.len()).collect();
    assert_eq!(
        seen, expected,
        "reveal chunks did not partition every candidate exactly once"
    );
}

#[then(regex = r"^the first reveal chunk contains the candidate at distance (\d+)$")]
async fn then_first_chunk_contains_candidate_at_distance(world: &mut BevyoutWorld, distance: f32) {
    let candidate = world
        .reveal_candidates
        .iter()
        .find(|candidate| candidate.position[0] == distance)
        .expect("no reveal candidate at that distance");
    let first_chunk = world
        .reveal_chunks
        .first()
        .expect("reveal chunks not planned yet");
    assert!(
        first_chunk.contains(&candidate.index),
        "expected first chunk {first_chunk:?} to contain candidate index {}",
        candidate.index
    );
}

// ---------------------------------------------------------------------
// door_animation.feature (issue #57)
// ---------------------------------------------------------------------

#[given(regex = r#"^a placement with clips "([^"]*)"$"#)]
async fn given_animation_clips(world: &mut BevyoutWorld, clips: String) {
    world.animation_clip_names = clips
        .split(',')
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .collect();
}

#[given("a placement with no clips")]
async fn given_animation_no_clips(world: &mut BevyoutWorld) {
    world.animation_clip_names = Vec::new();
}

#[when("the placement is opened")]
async fn when_animation_opened(world: &mut BevyoutWorld) {
    world.animation_selected_clip = Some(animation_policy::select_clip(
        animation_policy::ClipTransition::Opening,
        &world.animation_clip_names,
    ));
}

#[when("the placement is closed")]
async fn when_animation_closed(world: &mut BevyoutWorld) {
    world.animation_selected_clip = Some(animation_policy::select_clip(
        animation_policy::ClipTransition::Closing,
        &world.animation_clip_names,
    ));
}

#[then(regex = r#"^the selected clip is "([^"]*)"$"#)]
async fn then_animation_selected_clip_is(world: &mut BevyoutWorld, expected: String) {
    let selected = world
        .animation_selected_clip
        .clone()
        .expect("clip selection not computed yet");
    assert_eq!(selected, Some(expected));
}

#[then("no clip is selected")]
async fn then_animation_no_clip_selected(world: &mut BevyoutWorld) {
    let selected = world
        .animation_selected_clip
        .clone()
        .expect("clip selection not computed yet");
    assert_eq!(selected, None);
}

#[given(regex = r"^a travel door with an Open clip lasting ([\d.]+) seconds$")]
async fn given_animation_open_clip_seconds(world: &mut BevyoutWorld, seconds: f32) {
    world.animation_open_clip_seconds = Some(seconds);
}

#[given("a travel door with no Open clip")]
async fn given_animation_no_open_clip(world: &mut BevyoutWorld) {
    world.animation_open_clip_seconds = None;
}

#[when("the open lead is computed")]
async fn when_animation_open_lead_computed(world: &mut BevyoutWorld) {
    world.animation_open_lead = Some(animation_policy::open_lead_seconds(
        world.animation_open_clip_seconds,
        animation_policy::OPEN_LEAD_CAP_SECONDS,
    ));
}

#[then(regex = r"^the open lead is ([\d.]+) seconds$")]
async fn then_animation_open_lead_is(world: &mut BevyoutWorld, expected: f32) {
    let lead = world
        .animation_open_lead
        .expect("open lead not computed yet");
    assert!(
        (lead - expected).abs() < 1e-4,
        "open lead {lead} != expected {expected}"
    );
}

// ---------------------------------------------------------------------
// rust_irradiance.feature
// ---------------------------------------------------------------------

#[given(regex = r"^a Rust bake volume scale of ([\d.]+), ([\d.]+), ([\d.]+) metres$")]
async fn given_bake_volume_scale(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    world.bake_volume_scale = [x, y, z];
}

#[given(regex = r"^irradiance probe spacing is ([\d.]+) metres$")]
async fn given_bake_probe_spacing(world: &mut BevyoutWorld, spacing: f32) {
    world.bake_probe_spacing = spacing;
}

#[given(regex = r"^a probe resolution of (\d+), (\d+), (\d+)$")]
async fn given_bake_resolution(world: &mut BevyoutWorld, x: u32, y: u32, z: u32) {
    world.bake_resolution = [x, y, z];
}

#[given(regex = r"^irradiance sample count is (\d+)$")]
async fn given_bake_sample_count(world: &mut BevyoutWorld, samples: u32) {
    world.bake_samples = samples;
}

#[when("the Rust irradiance layout is planned")]
async fn when_bake_layout_planned(world: &mut BevyoutWorld) {
    world.bake_resolution =
        bake_policy::volume_resolution(world.bake_volume_scale, world.bake_probe_spacing);
}

#[when("the Rust irradiance atlas is planned")]
async fn when_bake_atlas_planned(world: &mut BevyoutWorld) {
    world.bake_atlas_dimensions = bake_policy::atlas_dimensions(world.bake_resolution);
}

#[when("the Rust irradiance ray count is planned")]
async fn when_bake_ray_count_planned(world: &mut BevyoutWorld) {
    world.bake_primary_rays =
        bake_policy::primary_ray_count(world.bake_resolution, world.bake_samples);
}

#[then(regex = r"^the probe resolution is (\d+), (\d+), (\d+)$")]
async fn then_bake_resolution_is(world: &mut BevyoutWorld, x: u32, y: u32, z: u32) {
    assert_eq!(world.bake_resolution, [x, y, z]);
}

#[then(regex = r"^the atlas dimensions are (\d+), (\d+), (\d+)$")]
async fn then_bake_atlas_dimensions_are(world: &mut BevyoutWorld, x: u32, y: u32, z: u32) {
    assert_eq!(world.bake_atlas_dimensions, [x, y, z]);
}

#[then(regex = r"^the primary ray count is (\d+)$")]
async fn then_bake_primary_ray_count_is(world: &mut BevyoutWorld, expected: usize) {
    assert_eq!(world.bake_primary_rays, expected);
}

// loading_fallback.feature (issue #59)
// ---------------------------------------------------------------------

#[given("no fallback is in flight")]
async fn given_no_fallback_in_flight(world: &mut BevyoutWorld) {
    world.fallback_state = Some(swap_policy::FallbackState::Idle);
}

#[given("a fallback is in flight")]
async fn given_fallback_in_flight(world: &mut BevyoutWorld) {
    world.fallback_state = Some(swap_policy::FallbackState::InFlight);
}

#[when("the destination becomes ready")]
async fn when_destination_becomes_ready(world: &mut BevyoutWorld) {
    world.fallback_lifecycle_outcome = Some(swap_policy::fallback_lifecycle_outcome(
        world.fallback_state.expect("fallback state not given"),
        swap_policy::FallbackEvent::DestinationReady,
    ));
}

#[when("the fallback parse fails")]
async fn when_fallback_parse_fails(world: &mut BevyoutWorld) {
    world.fallback_lifecycle_outcome = Some(swap_policy::fallback_lifecycle_outcome(
        world.fallback_state.expect("fallback state not given"),
        swap_policy::FallbackEvent::ParseFailed,
    ));
}

#[when("the player cancels the fallback")]
async fn when_player_cancels_fallback(world: &mut BevyoutWorld) {
    world.fallback_lifecycle_outcome = Some(swap_policy::fallback_lifecycle_outcome(
        world.fallback_state.expect("fallback state not given"),
        swap_policy::FallbackEvent::PlayerCancelled,
    ));
}

#[when("a superseding travel request arrives")]
async fn when_superseding_travel_request_arrives(world: &mut BevyoutWorld) {
    world.fallback_lifecycle_outcome = Some(swap_policy::fallback_lifecycle_outcome(
        world.fallback_state.expect("fallback state not given"),
        swap_policy::FallbackEvent::SupersedingRequest,
    ));
}

#[then(
    regex = r"^the fallback lifecycle outcome is (Ignore|Proceed|ReturnToSource|Cancel|Supersede)$"
)]
async fn then_fallback_lifecycle_outcome_is(world: &mut BevyoutWorld, expected: String) {
    let outcome = world
        .fallback_lifecycle_outcome
        .expect("fallback lifecycle outcome not computed yet");
    let expected = match expected.as_str() {
        "Ignore" => swap_policy::FallbackLifecycleOutcome::Ignore,
        "Proceed" => swap_policy::FallbackLifecycleOutcome::Proceed,
        "ReturnToSource" => swap_policy::FallbackLifecycleOutcome::ReturnToSource,
        "Cancel" => swap_policy::FallbackLifecycleOutcome::Cancel,
        _ => swap_policy::FallbackLifecycleOutcome::Supersede,
    };
    assert_eq!(outcome, expected);
}

#[given(regex = r"^an overlay fade duration of ([\d.]+) seconds and max alpha ([\d.]+)$")]
async fn given_overlay_fade_duration(world: &mut BevyoutWorld, duration: f32, max_alpha: f32) {
    world.overlay_fade_duration = duration;
    world.overlay_fade_max_alpha = max_alpha;
}

#[when(regex = r"^the overlay has been fading in for ([\d.]+) seconds$")]
async fn when_overlay_fading_in(world: &mut BevyoutWorld, elapsed: f32) {
    world.overlay_alpha = Some(swap_policy::fade_in_alpha(
        elapsed,
        world.overlay_fade_duration,
        world.overlay_fade_max_alpha,
    ));
}

#[when(regex = r"^the overlay has been fading out for ([\d.]+) seconds$")]
async fn when_overlay_fading_out(world: &mut BevyoutWorld, elapsed: f32) {
    world.overlay_alpha = Some(swap_policy::fade_out_alpha(
        elapsed,
        world.overlay_fade_duration,
        world.overlay_fade_max_alpha,
    ));
}

#[then(regex = r"^the overlay alpha is ([\d.]+)$")]
async fn then_overlay_alpha_is(world: &mut BevyoutWorld, expected: f32) {
    let alpha = world.overlay_alpha.expect("overlay alpha not computed yet");
    assert!(
        (alpha - expected).abs() < 1e-4,
        "alpha {alpha} != expected {expected}"
    );
}

#[then(regex = r"^the overlay alpha matches fading in for ([\d.]+) seconds$")]
async fn then_overlay_alpha_matches_fading_in(world: &mut BevyoutWorld, elapsed: f32) {
    let alpha = world.overlay_alpha.expect("overlay alpha not computed yet");
    let expected = swap_policy::fade_in_alpha(
        elapsed,
        world.overlay_fade_duration,
        world.overlay_fade_max_alpha,
    );
    assert!(
        (alpha - expected).abs() < 1e-4,
        "alpha {alpha} != expected {expected}"
    );
}

// ---------------------------------------------------------------------
// resumable_bake.feature (issue #62) -- appended section, do not interleave
// ---------------------------------------------------------------------

/// The synthetic "current toolchain" every bake-validity check in this
/// section compares against; scenarios vary the *checked* revision and job
/// fingerprint instead.
const BAKE_CURRENT_REVISION: &str = "bake-current";
const BAKE_CURRENT_JOB_FINGERPRINT: &str = "job-current";

#[given(
    regex = r#"^cell 0x([0-9a-fA-F]+) has a recorded bake with revision "([^"]*)" and job fingerprint "([^"]*)"$"#
)]
async fn given_bake_recorded(
    world: &mut BevyoutWorld,
    hex: String,
    revision: String,
    job_fingerprint: String,
) {
    world.bake_recorded.insert(
        parse_hex(&hex),
        Some(manifest::PreparedBake {
            bake_revision: Some(revision),
            source_fingerprint: job_fingerprint,
            scene_path: "scenes/00000001/baked/scene.glb".into(),
            irradiance_volume: None,
        }),
    );
}

#[given(regex = r"^cell 0x([0-9a-fA-F]+) has no recorded bake$")]
async fn given_bake_not_recorded(world: &mut BevyoutWorld, hex: String) {
    world.bake_recorded.insert(parse_hex(&hex), None);
}

#[given(regex = r"^cell 0x([0-9a-fA-F]+)'s recorded bake is currently (valid|stale)$")]
async fn given_bake_validity(world: &mut BevyoutWorld, hex: String, validity: String) {
    let form_id = parse_hex(&hex);
    // Drive the validity through the real `bake_is_valid` check on a
    // recorded `PreparedBake` rather than asserting the boolean directly: a
    // "valid" cell's recorded bake matches the current toolchain, a "stale"
    // one carries an outdated bake revision.
    let recorded_revision = if validity == "valid" {
        BAKE_CURRENT_REVISION
    } else {
        "bake-outdated"
    };
    let recorded = manifest::PreparedBake {
        bake_revision: Some(recorded_revision.into()),
        source_fingerprint: BAKE_CURRENT_JOB_FINGERPRINT.into(),
        scene_path: "scenes/00000001/baked/scene.glb".into(),
        irradiance_volume: None,
    };
    let valid = bake_plan::bake_is_valid(
        Some(&recorded),
        BAKE_CURRENT_REVISION,
        BAKE_CURRENT_JOB_FINGERPRINT,
    );
    world.bake_recorded.insert(form_id, Some(recorded));
    world.bake_validity.insert(form_id, valid);
}

#[when(
    regex = r#"^cell 0x([0-9a-fA-F]+)'s bake is checked against revision "([^"]*)" and job fingerprint "([^"]*)"$"#
)]
async fn when_bake_checked(
    world: &mut BevyoutWorld,
    hex: String,
    revision: String,
    job_fingerprint: String,
) {
    let recorded = world
        .bake_recorded
        .get(&parse_hex(&hex))
        .expect("no recorded bake state for that cell");
    world.bake_valid_result = Some(bake_plan::bake_is_valid(
        recorded.as_ref(),
        &revision,
        &job_fingerprint,
    ));
}

#[then(regex = r"^the recorded bake is (valid|stale)$")]
async fn then_bake_validity_is(world: &mut BevyoutWorld, expected: String) {
    let valid = world
        .bake_valid_result
        .expect("bake validity not checked yet");
    assert_eq!(valid, expected == "valid");
}

#[when(regex = r#"^cells "([^"]*)" are bake-resumed (without|with) force$"#)]
async fn when_bake_cells_resumed(world: &mut BevyoutWorld, list: String, force: String) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    let selection = parse_hex_list(&list);
    world.bake_resume_result = Some(bake_plan::filter_bake_resume(
        manifest,
        &selection,
        force == "with",
        &world.bake_validity,
    ));
}

#[then(regex = r#"^the bake cells to run are "([^"]*)"$"#)]
async fn then_bake_cells_to_run_are(world: &mut BevyoutWorld, list: String) {
    let expected = parse_hex_list(&list);
    let (to_run, _, _) = world
        .bake_resume_result
        .as_ref()
        .expect("cells were not bake-resumed yet");
    assert_eq!(to_run, &expected);
}

#[then(regex = r"^(\d+) cell\(s\) were skipped as validly baked$")]
async fn then_bake_cells_skipped(world: &mut BevyoutWorld, count: usize) {
    let (_, skipped, _) = world
        .bake_resume_result
        .as_ref()
        .expect("cells were not bake-resumed yet");
    assert_eq!(*skipped, count);
}

#[then(regex = r#"^the stale bake cells are "([^"]*)"$"#)]
async fn then_stale_bake_cells_are(world: &mut BevyoutWorld, list: String) {
    let expected = parse_hex_list(&list);
    let (_, _, stale) = world
        .bake_resume_result
        .as_ref()
        .expect("cells were not bake-resumed yet");
    assert_eq!(stale, &expected);
}

#[then("no bake cells were stale")]
async fn then_no_bake_cells_stale(world: &mut BevyoutWorld) {
    let (_, _, stale) = world
        .bake_resume_result
        .as_ref()
        .expect("cells were not bake-resumed yet");
    assert!(stale.is_empty(), "unexpected stale bake cells: {stale:?}");
}

#[then(
    regex = r#"^the bake batch summary line for (\d+) baked, (\d+) skipped, and (\d+) failed is "([^"]*)"$"#
)]
async fn then_bake_batch_summary_line_is(
    world: &mut BevyoutWorld,
    baked: usize,
    skipped: usize,
    failed: usize,
    expected: String,
) {
    let _ = world;
    assert_eq!(
        bake_plan::bake_batch_summary_line(baked, skipped, failed),
        expected
    );
}

#[then(regex = r#"^the stale bake line for cell 0x([0-9a-fA-F]+) is "([^"]*)"$"#)]
async fn then_stale_bake_line_is(world: &mut BevyoutWorld, hex: String, expected: String) {
    let _ = world;
    assert_eq!(bake_plan::stale_bake_line(parse_hex(&hex)), expected);
}

// ---------------------------------------------------------------------
// state_persistence.feature (issues #60/#61)
// ---------------------------------------------------------------------

fn persist_transform(translation: [f32; 3]) -> persist_policy::TransformDelta {
    persist_policy::TransformDelta {
        translation,
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: [1.0, 1.0, 1.0],
    }
}

#[given(regex = r"^a placement 0x([0-9a-fA-F]+) that is initially (enabled|disabled)$")]
async fn given_persist_placement(world: &mut BevyoutWorld, hex: String, state: String) {
    world
        .persist_placements
        .push(persist_policy::PlacementInfo {
            reference_form_id: parse_hex(&hex),
            initially_enabled: state == "enabled",
            enable_parent: None,
        });
}

#[given(regex = r"^a placement 0x([0-9a-fA-F]+) enable-parented to 0x([0-9a-fA-F]+)$")]
async fn given_persist_enable_parented(world: &mut BevyoutWorld, hex: String, parent: String) {
    world
        .persist_placements
        .push(persist_policy::PlacementInfo {
            reference_form_id: parse_hex(&hex),
            initially_enabled: true,
            enable_parent: Some(persist_policy::EnableParentLink {
                reference_form_id: parse_hex(&parent),
                inverted: false,
            }),
        });
}

#[given(
    regex = r"^a placement 0x([0-9a-fA-F]+) enable-parented to 0x([0-9a-fA-F]+) with the inverted flag$"
)]
async fn given_persist_enable_parented_inverted(
    world: &mut BevyoutWorld,
    hex: String,
    parent: String,
) {
    world
        .persist_placements
        .push(persist_policy::PlacementInfo {
            reference_form_id: parse_hex(&hex),
            initially_enabled: true,
            enable_parent: Some(persist_policy::EnableParentLink {
                reference_form_id: parse_hex(&parent),
                inverted: true,
            }),
        });
}

#[given(regex = r"^a save delta disabling reference 0x([0-9a-fA-F]+)$")]
async fn given_persist_delta_disables(world: &mut BevyoutWorld, hex: String) {
    world.persist_deltas.insert(
        parse_hex(&hex),
        persist_policy::ReferenceDelta {
            enabled: Some(false),
            ..Default::default()
        },
    );
}

#[given(regex = r"^a save delta deleting reference 0x([0-9a-fA-F]+)$")]
async fn given_persist_delta_deletes(world: &mut BevyoutWorld, hex: String) {
    world.persist_deltas.insert(
        parse_hex(&hex),
        persist_policy::ReferenceDelta {
            deleted: true,
            ..Default::default()
        },
    );
}

#[given(
    regex = r"^a save delta pointing reference 0x([0-9a-fA-F]+) at enable root 0x([0-9a-fA-F]+)$"
)]
async fn given_persist_delta_enable_root(world: &mut BevyoutWorld, hex: String, root: String) {
    world.persist_deltas.insert(
        parse_hex(&hex),
        persist_policy::ReferenceDelta {
            enable_root_form_id: Some(parse_hex(&root)),
            ..Default::default()
        },
    );
}

#[when("effective enabled state is resolved")]
async fn when_persist_effective_resolved(world: &mut BevyoutWorld) {
    world.persist_effective = Some(persist_policy::resolve_effective_enabled(
        &world.persist_placements,
        &world.persist_deltas,
    ));
}

#[then(regex = r"^placement 0x([0-9a-fA-F]+) resolves (enabled|disabled)$")]
async fn then_persist_resolves(world: &mut BevyoutWorld, hex: String, state: String) {
    let effective = world
        .persist_effective
        .as_ref()
        .expect("effective enabled state not resolved yet");
    let enabled = effective
        .get(&parse_hex(&hex))
        .copied()
        .expect("placement not resolved");
    assert_eq!(enabled, state == "enabled");
}

#[given(
    regex = r"^a baseline placement 0x([0-9a-fA-F]+) at \[(-?[\d.]+), (-?[\d.]+), (-?[\d.]+)\]$"
)]
async fn given_persist_baseline(world: &mut BevyoutWorld, hex: String, x: f32, y: f32, z: f32) {
    world
        .persist_baselines
        .push(persist_policy::BaselinePlacement {
            reference_form_id: parse_hex(&hex),
            transform: persist_transform([x, y, z]),
        });
}

#[given(
    regex = r"^a runtime snapshot of 0x([0-9a-fA-F]+) at \[(-?[\d.]+), (-?[\d.]+), (-?[\d.]+)\] with linear velocity \[(-?[\d.]+), (-?[\d.]+), (-?[\d.]+)\]$"
)]
#[allow(clippy::too_many_arguments)]
async fn given_persist_snapshot_moving(
    world: &mut BevyoutWorld,
    hex: String,
    x: f32,
    y: f32,
    z: f32,
    vx: f32,
    vy: f32,
    vz: f32,
) {
    world
        .persist_snapshots
        .push(persist_policy::RuntimeSnapshot {
            reference_form_id: parse_hex(&hex),
            present: true,
            transform: Some(persist_transform([x, y, z])),
            activated: None,
            body: Some(persist_policy::BodyDelta {
                linear_velocity: [vx, vy, vz],
                angular_velocity: [0.0, 0.0, 0.0],
                sleeping: false,
            }),
        });
}

#[given(
    regex = r"^a runtime snapshot of 0x([0-9a-fA-F]+) at \[(-?[\d.]+), (-?[\d.]+), (-?[\d.]+)\] at rest$"
)]
async fn given_persist_snapshot_at_rest(
    world: &mut BevyoutWorld,
    hex: String,
    x: f32,
    y: f32,
    z: f32,
) {
    world
        .persist_snapshots
        .push(persist_policy::RuntimeSnapshot {
            reference_form_id: parse_hex(&hex),
            present: true,
            transform: Some(persist_transform([x, y, z])),
            activated: None,
            body: Some(persist_policy::BodyDelta::default()),
        });
}

#[given(regex = r"^a runtime snapshot of 0x([0-9a-fA-F]+) that is open$")]
async fn given_persist_snapshot_open(world: &mut BevyoutWorld, hex: String) {
    world
        .persist_snapshots
        .push(persist_policy::RuntimeSnapshot {
            reference_form_id: parse_hex(&hex),
            present: true,
            transform: None,
            activated: Some(true),
            body: None,
        });
}

#[given(regex = r"^a runtime snapshot of 0x([0-9a-fA-F]+) that is no longer present$")]
async fn given_persist_snapshot_absent(world: &mut BevyoutWorld, hex: String) {
    world
        .persist_snapshots
        .push(persist_policy::RuntimeSnapshot {
            reference_form_id: parse_hex(&hex),
            present: false,
            transform: None,
            activated: None,
            body: None,
        });
}

#[when("the cell state is captured")]
async fn when_persist_captured(world: &mut BevyoutWorld) {
    world.persist_captured = Some(persist_policy::diff_capture(
        &world.persist_baselines,
        &world.persist_snapshots,
    ));
}

fn persist_captured_delta(world: &BevyoutWorld, form_id: u32) -> persist_policy::ReferenceDelta {
    world
        .persist_captured
        .as_ref()
        .expect("cell state not captured yet")
        .get(&form_id)
        .copied()
        .expect("no delta captured for reference")
}

#[then(regex = r"^the captured delta for 0x([0-9a-fA-F]+) has a transform$")]
async fn then_persist_captured_has_transform(world: &mut BevyoutWorld, hex: String) {
    assert!(
        persist_captured_delta(world, parse_hex(&hex))
            .transform
            .is_some()
    );
}

#[then(regex = r"^the captured delta for 0x([0-9a-fA-F]+) has a body state$")]
async fn then_persist_captured_has_body(world: &mut BevyoutWorld, hex: String) {
    assert!(
        persist_captured_delta(world, parse_hex(&hex))
            .body
            .is_some()
    );
}

#[then(regex = r"^the captured delta for 0x([0-9a-fA-F]+) is activated$")]
async fn then_persist_captured_activated(world: &mut BevyoutWorld, hex: String) {
    assert_eq!(
        persist_captured_delta(world, parse_hex(&hex)).activated,
        Some(true)
    );
}

#[then(regex = r"^the captured delta for 0x([0-9a-fA-F]+) is deleted$")]
async fn then_persist_captured_deleted(world: &mut BevyoutWorld, hex: String) {
    assert!(persist_captured_delta(world, parse_hex(&hex)).deleted);
}

#[then(regex = r"^no delta is captured for 0x([0-9a-fA-F]+)$")]
async fn then_persist_no_delta(world: &mut BevyoutWorld, hex: String) {
    let captured = world
        .persist_captured
        .as_ref()
        .expect("cell state not captured yet");
    assert!(!captured.contains_key(&parse_hex(&hex)));
}

#[when("the captured state is applied")]
async fn when_persist_captured_applied(world: &mut BevyoutWorld) {
    let placements: Vec<persist_policy::PlacementInfo> = world
        .persist_baselines
        .iter()
        .map(|baseline| persist_policy::PlacementInfo {
            reference_form_id: baseline.reference_form_id,
            initially_enabled: true,
            enable_parent: None,
        })
        .collect();
    let deltas = world
        .persist_captured
        .clone()
        .expect("cell state not captured yet");
    world.persist_applications = persist_policy::plan_apply(&placements, &deltas);
}

#[when("the save state is applied")]
async fn when_persist_save_state_applied(world: &mut BevyoutWorld) {
    world.persist_applications =
        persist_policy::plan_apply(&world.persist_placements, &world.persist_deltas);
}

fn persist_application(
    world: &BevyoutWorld,
    form_id: u32,
) -> &persist_policy::PlacementApplication {
    world
        .persist_applications
        .iter()
        .find(|application| application.reference_form_id == form_id)
        .expect("save state not applied yet, or placement unknown")
}

#[then(regex = r"^the applied placement 0x([0-9a-fA-F]+) is visible$")]
async fn then_persist_applied_visible(world: &mut BevyoutWorld, hex: String) {
    assert_eq!(
        persist_application(world, parse_hex(&hex)).visibility,
        persist_policy::VisibilityDecision::Visible
    );
}

#[then(regex = r"^the applied placement 0x([0-9a-fA-F]+) is hidden$")]
async fn then_persist_applied_hidden(world: &mut BevyoutWorld, hex: String) {
    assert_eq!(
        persist_application(world, parse_hex(&hex)).visibility,
        persist_policy::VisibilityDecision::Hidden
    );
}

#[then(
    regex = r"^the applied placement 0x([0-9a-fA-F]+) has translation \[(-?[\d.]+), (-?[\d.]+), (-?[\d.]+)\]$"
)]
async fn then_persist_applied_translation(
    world: &mut BevyoutWorld,
    hex: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let transform = persist_application(world, parse_hex(&hex))
        .transform
        .expect("applied placement has no transform delta");
    assert_eq!(transform.translation, [x, y, z]);
}

// ---------------------------------------------------------------------
// collider_ownership.feature (issue #63) -- appended section, do not
// interleave; new steps for this issue belong below this marker.
// ---------------------------------------------------------------------

fn ownership_record(
    world: &mut BevyoutWorld,
    cell: u32,
    statics: usize,
    keyframed: usize,
    dynamics: usize,
) {
    for _ in 0..statics {
        world.ownership_next_id += 1;
        let id = world.ownership_next_id;
        world.ownership_ledger.record_static_shape(cell, id);
    }
    for _ in 0..keyframed {
        world.ownership_next_id += 1;
        let id = world.ownership_next_id;
        world.ownership_ledger.record_keyframed_body(cell, id);
    }
    for _ in 0..dynamics {
        world.ownership_next_id += 1;
        let id = world.ownership_next_id;
        world.ownership_ledger.record_dynamic_body(cell, id);
    }
}

#[given(
    regex = r"^cell 0x([0-9a-fA-F]+) recorded (\d+) static shapes?, (\d+) keyframed bod(?:y|ies), and (\d+) dynamic bod(?:y|ies)$"
)]
async fn given_cell_recorded_colliders(
    world: &mut BevyoutWorld,
    hex: String,
    statics: usize,
    keyframed: usize,
    dynamics: usize,
) {
    ownership_record(world, parse_hex(&hex), statics, keyframed, dynamics);
}

#[when(regex = r"^cell 0x([0-9a-fA-F]+) records (\d+) more static shapes?$")]
async fn when_cell_records_more(world: &mut BevyoutWorld, hex: String, statics: usize) {
    ownership_record(world, parse_hex(&hex), statics, 0, 0);
}

#[when(regex = r"^cell 0x([0-9a-fA-F]+) is released$")]
async fn when_cell_released(world: &mut BevyoutWorld, hex: String) {
    world.ownership_released = Some(world.ownership_ledger.release(parse_hex(&hex)));
}

#[then(regex = r"^the released set has (\d+) static shapes? and (\d+) bod(?:y|ies)$")]
async fn then_released_set_counts(world: &mut BevyoutWorld, shapes: usize, bodies: usize) {
    let released = world
        .ownership_released
        .as_ref()
        .expect("no release performed yet")
        .as_ref()
        .expect("release returned nothing");
    assert_eq!(released.shape_count(), shapes);
    assert_eq!(released.body_count(), bodies);
}

#[then("nothing is released")]
async fn then_nothing_released(world: &mut BevyoutWorld) {
    assert!(
        world
            .ownership_released
            .as_ref()
            .expect("no release performed yet")
            .is_none()
    );
}

#[then(regex = r"^cell 0x([0-9a-fA-F]+) is no longer tracked$")]
async fn then_cell_untracked(world: &mut BevyoutWorld, hex: String) {
    assert!(!world.ownership_ledger.is_tracked(parse_hex(&hex)));
}

#[then(regex = r"^cell 0x([0-9a-fA-F]+) is still tracked$")]
async fn then_cell_still_tracked(world: &mut BevyoutWorld, hex: String) {
    assert!(world.ownership_ledger.is_tracked(parse_hex(&hex)));
}

// ---------------------------------------------------------------------
// container_audio.feature -- appended section, do not interleave.
// ---------------------------------------------------------------------

#[given("an animated container with no record open or close sound")]
async fn given_container_without_record_audio(world: &mut BevyoutWorld) {
    world.container_audio_is_container = true;
    world.container_audio_record_open = None;
    world.container_audio_record_close = None;
}

#[given(
    regex = r"^an animated container with record open sound 0x([0-9a-fA-F]+) and record close sound 0x([0-9a-fA-F]+)$"
)]
async fn given_container_with_record_audio(world: &mut BevyoutWorld, open: String, close: String) {
    world.container_audio_is_container = true;
    world.container_audio_record_open = Some(parse_hex(&open));
    world.container_audio_record_close = Some(parse_hex(&close));
}

#[given("a non-container placement with no record open or close sound")]
async fn given_non_container_without_record_audio(world: &mut BevyoutWorld) {
    world.container_audio_is_container = false;
    world.container_audio_record_open = None;
    world.container_audio_record_close = None;
}

#[given(regex = r#"^its animation sound cues are "([^"]*)"$"#)]
async fn given_animation_sound_cues(world: &mut BevyoutWorld, encoded: String) {
    world.container_audio_cues = encoded
        .split(',')
        .map(|entry| {
            let (sequence_time, editor_id) = entry.split_once('=').expect("cue editor id");
            let (sequence, time) = sequence_time.split_once('@').expect("cue time");
            container_audio_policy::AnimationSoundCue {
                sequence: sequence.into(),
                time: time.parse().expect("numeric cue time"),
                editor_id: editor_id.into(),
            }
        })
        .collect();
}

#[given(
    regex = r"^its resolved animation sound ids are open 0x([0-9a-fA-F]+) and close 0x([0-9a-fA-F]+)$"
)]
async fn given_resolved_animation_sound_ids(world: &mut BevyoutWorld, open: String, close: String) {
    world.container_audio_resolved_open = Some(parse_hex(&open));
    world.container_audio_resolved_close = Some(parse_hex(&close));
}

#[when("authored container animation audio is resolved")]
async fn when_authored_container_audio_resolved(world: &mut BevyoutWorld) {
    world.container_audio_selected = Some(container_audio_policy::select_container_audio(
        &world.container_audio_cues,
    ));
}

#[when("resolved container sound ids are applied")]
async fn when_resolved_container_audio_applied(world: &mut BevyoutWorld) {
    let (open, close) = container_audio_policy::apply_container_audio_fallback(
        world.container_audio_is_container,
        world.container_audio_record_open,
        world.container_audio_record_close,
        world.container_audio_resolved_open,
        world.container_audio_resolved_close,
    );
    world.container_audio_prepared_open = open;
    world.container_audio_prepared_close = close;
}

#[then(regex = r#"^the selected open sound is "([^"]*)"$"#)]
async fn then_selected_open_sound(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .container_audio_selected
            .as_ref()
            .expect("container audio not selected")
            .open_editor_id
            .as_deref(),
        Some(expected.as_str())
    );
}

#[then(regex = r#"^the selected close sound is "([^"]*)"$"#)]
async fn then_selected_close_sound(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .container_audio_selected
            .as_ref()
            .expect("container audio not selected")
            .close_editor_id
            .as_deref(),
        Some(expected.as_str())
    );
}

#[then(regex = r"^the prepared open sound id is 0x([0-9a-fA-F]+)$")]
async fn then_prepared_open_sound(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.container_audio_prepared_open,
        Some(parse_hex(&expected))
    );
}

#[then(regex = r"^the prepared close sound id is 0x([0-9a-fA-F]+)$")]
async fn then_prepared_close_sound(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.container_audio_prepared_close,
        Some(parse_hex(&expected))
    );
}

#[then("the prepared open sound id is absent")]
async fn then_prepared_open_sound_absent(world: &mut BevyoutWorld) {
    assert_eq!(world.container_audio_prepared_open, None);
}

#[then("the prepared close sound id is absent")]
async fn then_prepared_close_sound_absent(world: &mut BevyoutWorld) {
    assert_eq!(world.container_audio_prepared_close, None);
}

// ---------------------------------------------------------------------
// inventory.feature (M3 wave 1, issues #71/#72) -- appended section.
// ---------------------------------------------------------------------

#[given("an empty player inventory")]
async fn given_empty_player_inventory(world: &mut BevyoutWorld) {
    world.player_inventory = inventory_policy::Inventory::default();
}

#[given(regex = r"^an inventory stack of (\d+) items$")]
async fn given_inventory_stack(world: &mut BevyoutWorld, count: i32) {
    world.player_inventory =
        inventory_policy::Inventory::from_stacks([inventory_policy::InventoryStack {
            base_form_id: 0x10,
            count,
            condition: None,
        }]);
}

#[when(regex = r"^(\d+) items? 0x([0-9a-fA-F]+) at condition (\d+) (?:is|are) added$")]
async fn when_condition_items_added(
    world: &mut BevyoutWorld,
    count: i32,
    form_id: String,
    condition: u32,
) {
    world
        .player_inventory
        .add(inventory_policy::InventoryStack {
            base_form_id: parse_hex(&form_id),
            count,
            condition: Some(condition),
        });
}

#[when(regex = r"^(\d+) items? 0x([0-9a-fA-F]+) without condition (?:is|are) added$")]
async fn when_conditionless_items_added(world: &mut BevyoutWorld, count: i32, form_id: String) {
    world
        .player_inventory
        .add(inventory_policy::InventoryStack {
            base_form_id: parse_hex(&form_id),
            count,
            condition: None,
        });
}

#[when(regex = r"^item 0x([0-9a-fA-F]+) weighs ([\d.]+)$")]
async fn when_item_weight(world: &mut BevyoutWorld, form_id: String, weight: f32) {
    world.inventory_weights.insert(parse_hex(&form_id), weight);
}

#[when("its right-click drop policy is evaluated")]
async fn when_drop_policy_evaluated(world: &mut BevyoutWorld) {
    let count = world.player_inventory.stacks()[0].count;
    world.inventory_drop_action = inventory_policy::drop_action(count);
}

#[when(regex = r"^removing (\d+) items is attempted$")]
async fn when_removing_items_attempted(world: &mut BevyoutWorld, count: i32) {
    world.inventory_transfer = Some(world.player_inventory.remove(
        inventory_policy::StackKey {
            base_form_id: 0x10,
            condition: None,
        },
        count,
    ));
}

#[then(regex = r"^the inventory has (\d+) stacks? and (\d+) total items$")]
async fn then_inventory_shape(world: &mut BevyoutWorld, stacks: usize, total: i32) {
    assert_eq!(world.player_inventory.stacks().len(), stacks);
    assert_eq!(world.player_inventory.count(0x10), total);
}

#[then(regex = r"^carried weight is ([\d.]+)$")]
async fn then_carried_weight(world: &mut BevyoutWorld, expected: f32) {
    let actual = world
        .player_inventory
        .total_weight(|form_id| world.inventory_weights.get(&form_id).copied());
    assert!((actual - expected).abs() < f32::EPSILON);
}

#[then("one item is selected for dropping")]
async fn then_drop_one(world: &mut BevyoutWorld) {
    assert_eq!(
        world.inventory_drop_action,
        Some(inventory_policy::DropAction::DropOne)
    );
}

#[then(regex = r"^a quantity from (\d+) through (\d+) is requested with default (\d+)$")]
async fn then_quantity_requested(world: &mut BevyoutWorld, min: i32, max: i32, default: i32) {
    assert_eq!(
        world.inventory_drop_action,
        Some(inventory_policy::DropAction::ChooseQuantity { min, max, default })
    );
}

#[then(regex = r"^the transfer is rejected and (\d+) items remain$")]
async fn then_transfer_rejected(world: &mut BevyoutWorld, remaining: i32) {
    assert_eq!(
        world.inventory_transfer,
        Some(inventory_policy::TransferResult::InsufficientItems)
    );
    assert_eq!(world.player_inventory.count(0x10), remaining);
}

// performance_probe.feature -- appended section, do not interleave;
// new steps for this feature belong below this marker.
// ---------------------------------------------------------------------

#[given(regex = r#"^frame-time samples "([^"]+)"$"#)]
async fn given_frame_time_samples(world: &mut BevyoutWorld, encoded: String) {
    world.performance_samples = encoded
        .split(',')
        .map(|pair| {
            let (sample, frame_time_ms) = pair
                .split_once(':')
                .expect("frame sample must use <sample>:<milliseconds>");
            performance_policy::FrameSample {
                sample: sample.parse().expect("sample id must be an integer"),
                frame_time_ms: frame_time_ms.parse().expect("frame time must be a number"),
            }
        })
        .collect();
}

#[when(
    regex = r"^frames after sample (\d+) are summarized with latest limit (\d+) and budget ([\d.]+) ms$"
)]
async fn when_frames_after_marker_are_summarized(
    world: &mut BevyoutWorld,
    marker: u64,
    limit: usize,
    budget_ms: f64,
) {
    world.performance_summary = Some(performance_policy::summarize_frame_window(
        &world.performance_samples,
        Some(marker),
        limit,
        budget_ms,
    ));
}

#[when(regex = r"^all frames are summarized with latest limit (\d+) and budget ([\d.]+) ms$")]
async fn when_all_frames_are_summarized(world: &mut BevyoutWorld, limit: usize, budget_ms: f64) {
    world.performance_summary = Some(performance_policy::summarize_frame_window(
        &world.performance_samples,
        None,
        limit,
        budget_ms,
    ));
}

fn performance_summary(world: &BevyoutWorld) -> &performance_policy::FrameProbeSummary {
    world
        .performance_summary
        .as_ref()
        .expect("frame samples have not been summarized")
}

#[then(regex = r"^the frame probe covers samples (\d+) through (\d+)$")]
async fn then_frame_probe_sample_range(world: &mut BevyoutWorld, first: u64, last: u64) {
    let summary = performance_summary(world);
    assert_eq!(summary.first_sample, Some(first));
    assert_eq!(summary.last_sample, Some(last));
}

#[then(regex = r"^the frame probe has (\d+) samples$")]
async fn then_frame_probe_sample_count(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(performance_summary(world).sample_count, count);
}

#[then(regex = r"^the frame probe p95 and max are ([\d.]+) and ([\d.]+) ms$")]
async fn then_frame_probe_p95_and_max(world: &mut BevyoutWorld, p95: f64, max: f64) {
    let summary = performance_summary(world);
    assert_eq!(summary.p95_ms, Some(p95));
    assert_eq!(summary.max_ms, Some(max));
}

#[then(regex = r"^(\d+) frames? exceed(?:s)? the probe budget$")]
async fn then_frames_exceed_budget(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(performance_summary(world).over_budget_count, count);
}

// ---------------------------------------------------------------------
// leveled_lists.feature (issue #74)
// ---------------------------------------------------------------------

/// Parses "level:base_form_id:count" entries, comma-separated; an empty
/// string yields no entries.
fn parse_leveled_entries(text: &str) -> Vec<leveled::PreparedLeveledEntry> {
    text.split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            let fields: Vec<&str> = part.split(':').collect();
            let [level, base_form_id, count] = fields.as_slice() else {
                panic!("leveled entry must be \"level:base_form_id:count\", got {part:?}");
            };
            leveled::PreparedLeveledEntry {
                level: level.parse().expect("entry level must be a u16"),
                base_form_id: base_form_id.parse().expect("entry form id must be a u32"),
                count: count.parse().expect("entry count must be an i32"),
            }
        })
        .collect()
}

/// Parses "base_form_id x count" stacks, comma-separated; an empty string
/// yields no stacks.
fn parse_resolved_stacks(text: &str) -> Vec<(u32, i32)> {
    text.split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            let (form_id, count) = part
                .split_once('x')
                .expect("resolved stack must be \"base_form_id x count\"");
            (
                form_id.trim().parse().expect("stack form id must be a u32"),
                count.trim().parse().expect("stack count must be an i32"),
            )
        })
        .collect()
}

fn parse_leveled_flags(token: &str) -> u8 {
    match token {
        "use-all" => leveled::LEVELED_USE_ALL,
        "calculate-for-each-item" => leveled::LEVELED_CALCULATE_FOR_EACH_ITEM,
        numeric => numeric
            .parse()
            .expect("flags must be numeric or a known name"),
    }
}

#[given(
    regex = r#"^leveled list (\d+) with chance-none (\d+) and flags (\S+) and entries "([^"]*)"$"#
)]
async fn given_leveled_list(
    world: &mut BevyoutWorld,
    form_id: u32,
    chance_none: u8,
    flags: String,
    entries: String,
) {
    world.leveled_lists.insert(
        form_id,
        leveled::PreparedLeveledList {
            chance_none,
            flags: parse_leveled_flags(&flags),
            entries: parse_leveled_entries(&entries),
        },
    );
}

#[given(
    regex = r#"^a leveled seed from playthrough (\d+), cell (\d+), reference (\d+) named "([^"]+)"$"#
)]
async fn given_leveled_seed(
    world: &mut BevyoutWorld,
    playthrough_seed: u64,
    cell_form_id: u32,
    reference_form_id: u32,
    name: String,
) {
    world.leveled_seeds.insert(
        name,
        leveled::LeveledSeed::derive(playthrough_seed, cell_form_id, reference_form_id),
    );
}

#[when(regex = r#"^list (\d+) is resolved for player level (\d+) using seed "([^"]+)"$"#)]
async fn when_leveled_list_resolved(
    world: &mut BevyoutWorld,
    list_form_id: u32,
    player_level: u16,
    seed_name: String,
) {
    let seed = *world
        .leveled_seeds
        .get(&seed_name)
        .unwrap_or_else(|| panic!("no leveled seed named {seed_name:?}"));
    world.leveled_last_resolution = Some(leveled::resolve_leveled(
        list_form_id,
        &world.leveled_lists,
        seed,
        player_level,
    ));
}

#[then("the resolved stacks are empty")]
async fn then_resolved_stacks_are_empty(world: &mut BevyoutWorld) {
    assert!(
        world
            .leveled_last_resolution
            .as_ref()
            .expect("no leveled resolution computed yet")
            .is_empty()
    );
}

#[then(regex = r#"^the resolved stacks are "([^"]*)"$"#)]
async fn then_resolved_stacks_are(world: &mut BevyoutWorld, expected: String) {
    let resolved = world
        .leveled_last_resolution
        .as_ref()
        .expect("no leveled resolution computed yet");
    assert_eq!(*resolved, parse_resolved_stacks(&expected));
}

#[then(regex = r#"^seeds "([^"]+)" and "([^"]+)" are identical$"#)]
async fn then_leveled_seeds_are_identical(world: &mut BevyoutWorld, a: String, b: String) {
    assert_eq!(world.leveled_seeds[&a], world.leveled_seeds[&b]);
}

#[then(regex = r#"^seeds "([^"]+)" and "([^"]+)" are different$"#)]
async fn then_leveled_seeds_are_different(world: &mut BevyoutWorld, a: String, b: String) {
    assert_ne!(world.leveled_seeds[&a], world.leveled_seeds[&b]);
}

// ---------------------------------------------------------------------
// container_transfer.feature (issue #75)
// ---------------------------------------------------------------------

fn parse_container_form_id(hex: &str) -> u32 {
    u32::from_str_radix(hex, 16).expect("hex form id")
}

#[given(regex = r"^a container inventory entry 0x([0-9a-fA-F]+) x(-?\d+) leveled (yes|no)$")]
async fn given_container_inventory_entry(
    world: &mut BevyoutWorld,
    form_id: String,
    count: i32,
    leveled: String,
) {
    world
        .container_seed_entries
        .push(container_policy::SeedEntry {
            base_form_id: parse_container_form_id(&form_id),
            count,
            leveled: leveled == "yes",
        });
}

#[given(
    regex = r"^the leveled resolver for list 0x([0-9a-fA-F]+) returns 0x([0-9a-fA-F]+) x(-?\d+)$"
)]
async fn given_leveled_resolver_returns(
    world: &mut BevyoutWorld,
    list_form_id: String,
    item_form_id: String,
    count: i32,
) {
    world
        .container_resolver_lists
        .entry(parse_container_form_id(&list_form_id))
        .or_default()
        .push((parse_container_form_id(&item_form_id), count));
}

/// Shared by both "opened for the first time" and "reopened" steps: runs
/// `container_policy::open_container` against the world's current state,
/// counting resolver calls and syncing `container_stacks` (the single
/// source of truth the transfer `Then` steps below read) from the result.
fn open_world_container(world: &mut BevyoutWorld) {
    let entries = world.container_seed_entries.clone();
    let lists = world.container_resolver_lists.clone();
    let existing = world.container_state.take();
    let mut calls = 0u32;
    let resolved = container_policy::open_container(existing, &entries, |list_form_id| {
        calls += 1;
        lists.get(&list_form_id).cloned().unwrap_or_default()
    });
    world.container_resolver_calls += calls;
    world.container_stacks = resolved.stacks.clone();
    world.container_state = Some(resolved);
}

#[when("the container is opened for the first time")]
async fn when_container_opened_first_time(world: &mut BevyoutWorld) {
    open_world_container(world);
}

#[when("the container is reopened")]
async fn when_container_reopened(world: &mut BevyoutWorld) {
    open_world_container(world);
}

#[then("the container is resolved")]
async fn then_container_is_resolved(world: &mut BevyoutWorld) {
    assert!(
        world
            .container_state
            .as_ref()
            .expect("the container has not been opened")
            .resolved
    );
}

#[then(regex = r"^the resolver was called (\d+) times?$")]
async fn then_resolver_called_n_times(world: &mut BevyoutWorld, count: u32) {
    assert_eq!(world.container_resolver_calls, count);
}

#[given(regex = r"^a container stack of 0x([0-9a-fA-F]+) x(-?\d+)$")]
async fn given_container_stack(world: &mut BevyoutWorld, form_id: String, count: i32) {
    world
        .container_stacks
        .push((parse_container_form_id(&form_id), count));
}

#[given(regex = r"^a player stack of 0x([0-9a-fA-F]+) x(-?\d+)$")]
async fn given_player_stack(world: &mut BevyoutWorld, form_id: String, count: i32) {
    world
        .player_stacks
        .push((parse_container_form_id(&form_id), count));
}

#[when(regex = r"^one 0x([0-9a-fA-F]+) is taken from the container$")]
async fn when_take_one(world: &mut BevyoutWorld, form_id: String) {
    let form_id = parse_container_form_id(&form_id);
    world.transfer_result = Some(container_policy::take_one(
        &mut world.container_stacks,
        &mut world.player_stacks,
        form_id,
    ));
}

#[when(regex = r"^a stack of (-?\d+) 0x([0-9a-fA-F]+) is taken from the container$")]
async fn when_take_stack(world: &mut BevyoutWorld, count: i32, form_id: String) {
    let form_id = parse_container_form_id(&form_id);
    world.transfer_result = Some(container_policy::take_stack(
        &mut world.container_stacks,
        &mut world.player_stacks,
        form_id,
        count,
    ));
}

#[when(regex = r"^all 0x([0-9a-fA-F]+) is taken from the container$")]
async fn when_take_all(world: &mut BevyoutWorld, form_id: String) {
    let form_id = parse_container_form_id(&form_id);
    world.transfer_result = Some(container_policy::take_all(
        &mut world.container_stacks,
        &mut world.player_stacks,
        form_id,
    ));
}

#[when(regex = r"^one 0x([0-9a-fA-F]+) is stored into the container$")]
async fn when_store_one(world: &mut BevyoutWorld, form_id: String) {
    let form_id = parse_container_form_id(&form_id);
    world.transfer_result = Some(container_policy::store_one(
        &mut world.player_stacks,
        &mut world.container_stacks,
        form_id,
    ));
}

#[when(regex = r"^a stack of (-?\d+) 0x([0-9a-fA-F]+) is stored into the container$")]
async fn when_store_stack(world: &mut BevyoutWorld, count: i32, form_id: String) {
    let form_id = parse_container_form_id(&form_id);
    world.transfer_result = Some(container_policy::store_stack(
        &mut world.player_stacks,
        &mut world.container_stacks,
        form_id,
        count,
    ));
}

#[then(regex = r"^the container stack for 0x([0-9a-fA-F]+) is (-?\d+)$")]
async fn then_container_stack_is(world: &mut BevyoutWorld, form_id: String, count: i32) {
    let form_id = parse_container_form_id(&form_id);
    assert_eq!(
        container_policy::stack_count(&world.container_stacks, form_id),
        count
    );
}

#[then(regex = r"^the player stack for 0x([0-9a-fA-F]+) is (-?\d+)$")]
async fn then_player_stack_is(world: &mut BevyoutWorld, form_id: String, count: i32) {
    let form_id = parse_container_form_id(&form_id);
    assert_eq!(
        container_policy::stack_count(&world.player_stacks, form_id),
        count
    );
}

#[then("the transfer is rejected")]
async fn then_transfer_is_rejected(world: &mut BevyoutWorld) {
    assert!(
        world
            .transfer_result
            .as_ref()
            .expect("no transfer op has run yet")
            .is_err()
    );
}

// ---------------------------------------------------------------------
// container_persistence.feature (issue #76) -- appended section, do not
// interleave; new steps for this issue belong below this marker.
// ---------------------------------------------------------------------

/// Parses `0x<hex> x <count>` (e.g. "0x00000010 x 3") into a single-entry
/// stack list, matching the `Vec<(u32, i32)>` shape `persist_policy`'s
/// container types use.
fn parse_single_stack(hex: &str, count: i32) -> Vec<(u32, i32)> {
    vec![(parse_hex(hex), count)]
}

#[given(regex = r"^a container baseline 0x([0-9a-fA-F]+) with stack 0x([0-9a-fA-F]+) x (-?\d+)$")]
async fn given_container_baseline(
    world: &mut BevyoutWorld,
    container_hex: String,
    item_hex: String,
    count: i32,
) {
    world.container_baselines.insert(
        parse_hex(&container_hex),
        persist_policy::ContainerBaseline {
            stacks: parse_single_stack(&item_hex, count),
        },
    );
}

#[given(regex = r"^a container snapshot 0x([0-9a-fA-F]+) with stack 0x([0-9a-fA-F]+) x (-?\d+)$")]
async fn given_container_snapshot(
    world: &mut BevyoutWorld,
    container_hex: String,
    item_hex: String,
    count: i32,
) {
    world.container_snapshots.insert(
        parse_hex(&container_hex),
        persist_policy::ContainerSnapshot {
            stacks: parse_single_stack(&item_hex, count),
            resolved: false,
        },
    );
}

#[given(
    regex = r"^a resolved container snapshot 0x([0-9a-fA-F]+) with stack 0x([0-9a-fA-F]+) x (-?\d+)$"
)]
async fn given_resolved_container_snapshot(
    world: &mut BevyoutWorld,
    container_hex: String,
    item_hex: String,
    count: i32,
) {
    world.container_snapshots.insert(
        parse_hex(&container_hex),
        persist_policy::ContainerSnapshot {
            stacks: parse_single_stack(&item_hex, count),
            resolved: true,
        },
    );
}

#[when("container state is captured")]
async fn when_container_state_captured(world: &mut BevyoutWorld) {
    world.container_captured = Some(persist_policy::diff_capture_containers(
        &world.container_baselines,
        &world.container_snapshots,
    ));
}

fn captured_container_delta(world: &BevyoutWorld, form_id: u32) -> persist_policy::ContainerDelta {
    world
        .container_captured
        .as_ref()
        .expect("container state not captured yet")
        .get(&form_id)
        .cloned()
        .expect("no container delta captured for reference")
}

#[then(regex = r"^no container delta is captured for 0x([0-9a-fA-F]+)$")]
async fn then_no_container_delta(world: &mut BevyoutWorld, hex: String) {
    let captured = world
        .container_captured
        .as_ref()
        .expect("container state not captured yet");
    assert!(!captured.contains_key(&parse_hex(&hex)));
}

#[then(
    regex = r"^the captured container delta for 0x([0-9a-fA-F]+) has stack 0x([0-9a-fA-F]+) x (-?\d+)$"
)]
async fn then_captured_container_delta_has_stack(
    world: &mut BevyoutWorld,
    container_hex: String,
    item_hex: String,
    count: i32,
) {
    let delta = captured_container_delta(world, parse_hex(&container_hex));
    assert_eq!(delta.inventory, Some(parse_single_stack(&item_hex, count)));
}

#[then(regex = r"^the captured container delta for 0x([0-9a-fA-F]+) has no resolved marker$")]
async fn then_captured_container_delta_has_no_resolved_marker(
    world: &mut BevyoutWorld,
    hex: String,
) {
    let delta = captured_container_delta(world, parse_hex(&hex));
    assert_eq!(delta.leveled_resolved, None);
}

#[then(regex = r"^the captured container delta for 0x([0-9a-fA-F]+) has no inventory override$")]
async fn then_captured_container_delta_has_no_inventory(world: &mut BevyoutWorld, hex: String) {
    let delta = captured_container_delta(world, parse_hex(&hex));
    assert_eq!(delta.inventory, None);
}

#[then(regex = r"^the captured container delta for 0x([0-9a-fA-F]+) is resolved$")]
async fn then_captured_container_delta_is_resolved(world: &mut BevyoutWorld, hex: String) {
    let delta = captured_container_delta(world, parse_hex(&hex));
    assert_eq!(delta.leveled_resolved, Some(true));
}

#[given(
    regex = r"^a container delta 0x([0-9a-fA-F]+) with stack 0x([0-9a-fA-F]+) x (-?\d+) and resolved$"
)]
async fn given_container_delta(
    world: &mut BevyoutWorld,
    container_hex: String,
    item_hex: String,
    count: i32,
) {
    world.container_deltas.insert(
        parse_hex(&container_hex),
        persist_policy::ContainerDelta {
            inventory: Some(parse_single_stack(&item_hex, count)),
            leveled_resolved: Some(true),
        },
    );
}

#[given(regex = r"^a resolved-only container delta 0x([0-9a-fA-F]+)$")]
async fn given_resolved_only_container_delta(world: &mut BevyoutWorld, hex: String) {
    world.container_deltas.insert(
        parse_hex(&hex),
        persist_policy::ContainerDelta {
            inventory: None,
            leveled_resolved: Some(true),
        },
    );
}

#[when("the container state is applied")]
async fn when_container_state_applied(world: &mut BevyoutWorld) {
    world.container_seeded = Some(persist_policy::plan_apply_containers(
        &world.container_baselines,
        &world.container_deltas,
    ));
}

#[then(regex = r"^no container is seeded for 0x([0-9a-fA-F]+)$")]
async fn then_no_container_seeded(world: &mut BevyoutWorld, hex: String) {
    let seeded = world
        .container_seeded
        .as_ref()
        .expect("container state not applied yet");
    assert!(!seeded.contains_key(&parse_hex(&hex)));
}

fn seeded_container(world: &BevyoutWorld, form_id: u32) -> persist_policy::ContainerSnapshot {
    world
        .container_seeded
        .as_ref()
        .expect("container state not applied yet")
        .get(&form_id)
        .cloned()
        .expect("no container seeded for reference")
}

#[then(regex = r"^the seeded container 0x([0-9a-fA-F]+) has stack 0x([0-9a-fA-F]+) x (-?\d+)$")]
async fn then_seeded_container_has_stack(
    world: &mut BevyoutWorld,
    container_hex: String,
    item_hex: String,
    count: i32,
) {
    let snapshot = seeded_container(world, parse_hex(&container_hex));
    assert_eq!(snapshot.stacks, parse_single_stack(&item_hex, count));
}

#[then(regex = r"^the seeded container 0x([0-9a-fA-F]+) is resolved$")]
async fn then_seeded_container_is_resolved(world: &mut BevyoutWorld, hex: String) {
    let snapshot = seeded_container(world, parse_hex(&hex));
    assert!(snapshot.resolved);
}

// ---------------------------------------------------------------------
// item_flags.feature (issue #81)
// ---------------------------------------------------------------------

#[given(regex = r"^a base record with header flags 0x([0-9a-fA-F]+)$")]
async fn given_header_flags(world: &mut BevyoutWorld, hex: String) {
    world.flag_record_flags = parse_hex(&hex);
}

#[then("the record is a quest item")]
async fn then_record_is_quest_item(world: &mut BevyoutWorld) {
    assert!(item_rules::is_quest_item(world.flag_record_flags));
}

#[then("the record is not a quest item")]
async fn then_record_is_not_quest_item(world: &mut BevyoutWorld) {
    assert!(!item_rules::is_quest_item(world.flag_record_flags));
}

#[when(regex = r"^dropping item 0x([0-9a-fA-F]+) quest (yes|no) is checked$")]
async fn when_drop_checked(world: &mut BevyoutWorld, hex: String, quest: String) {
    world.rule_result = Some(item_rules::can_drop(parse_hex(&hex), quest == "yes"));
}

#[when(regex = r"^storing item 0x([0-9a-fA-F]+) quest (yes|no) is checked$")]
async fn when_store_checked(world: &mut BevyoutWorld, _hex: String, quest: String) {
    world.rule_result = Some(item_rules::can_store(quest == "yes"));
}

#[then("the transfer is allowed")]
async fn then_transfer_allowed(world: &mut BevyoutWorld) {
    assert_eq!(world.rule_result, Some(Ok(())));
}

#[then("the transfer is rejected as quest item")]
async fn then_transfer_rejected_quest(world: &mut BevyoutWorld) {
    assert_eq!(
        world.rule_result,
        Some(Err(item_rules::TransferRejection::QuestItem))
    );
}

#[then("the transfer is rejected as caps")]
async fn then_transfer_rejected_caps(world: &mut BevyoutWorld) {
    assert_eq!(
        world.rule_result,
        Some(Err(item_rules::TransferRejection::Caps))
    );
}

#[given(regex = r"^a carried stack of (\d+) weighing ([\d.]+) each quest (yes|no)$")]
async fn given_carried_stack(world: &mut BevyoutWorld, count: i32, weight: f32, quest: String) {
    world.carried_stacks.push((count, quest == "yes", weight));
}

#[when("the carried weight is totaled")]
async fn when_carried_weight_totaled(world: &mut BevyoutWorld) {
    world.carried_total = Some(
        world
            .carried_stacks
            .iter()
            .map(|&(count, quest, weight)| {
                count as f32 * item_rules::carried_weight(quest, Some(weight)).unwrap_or(0.0)
            })
            .sum(),
    );
}

#[then(regex = r"^the carried weight is ([\d.]+)$")]
async fn then_carried_weight_excluding_quest(world: &mut BevyoutWorld, expected: f32) {
    let total = world.carried_total.expect("carried weight not totaled yet");
    assert!((total - expected).abs() < 1e-4, "carried weight {total}");
}

#[when("taking a reference with no owner is classified")]
async fn when_take_unowned_classified(world: &mut BevyoutWorld) {
    world.take_classification = Some(item_rules::classify_take(None));
}

#[when(regex = r"^taking a reference owned by 0x([0-9a-fA-F]+) is classified$")]
async fn when_take_owned_classified(world: &mut BevyoutWorld, hex: String) {
    world.take_classification = Some(item_rules::classify_take(Some(parse_hex(&hex))));
}

#[then("the take is not theft")]
async fn then_take_not_theft(world: &mut BevyoutWorld) {
    assert_eq!(
        world.take_classification,
        Some(item_rules::TakeClassification::Take)
    );
}

#[then(regex = r"^the take is theft from 0x([0-9a-fA-F]+)$")]
async fn then_take_theft(world: &mut BevyoutWorld, hex: String) {
    assert_eq!(
        world.take_classification,
        Some(item_rules::TakeClassification::Steal {
            owner_form_id: parse_hex(&hex)
        })
    );
}

#[then(regex = r"^the player caps total is (-?\d+)$")]
async fn then_player_caps_total(world: &mut BevyoutWorld, expected: i32) {
    assert_eq!(
        container_policy::stack_count(&world.player_stacks, item_rules::CAPS_FORM_ID),
        expected
    );
}

// ---------------------------------------------------------------------
// item_transactions.feature (M3/#95)
// ---------------------------------------------------------------------

#[given(
    regex = r"^the canonical player holds item 0x([0-9a-fA-F]+) form 0x([0-9a-fA-F]+) x(\d+) condition (none|\d+)$"
)]
async fn given_canonical_player_item(
    world: &mut BevyoutWorld,
    item_hex: String,
    form_hex: String,
    count: u32,
    condition: String,
) {
    world.canonical_ledger = ItemLedger::new();
    let item = ItemInstance::new(
        ItemInstanceId(parse_hex(&item_hex) as u64),
        parse_hex(&form_hex),
        count,
        ItemState {
            condition: (condition != "none").then(|| condition.parse().unwrap()),
            ..Default::default()
        },
    )
    .unwrap();
    world
        .canonical_ledger
        .insert_holder(
            HolderId::Player,
            ItemHolderState {
                items: vec![item],
                ..Default::default()
            },
        )
        .unwrap();
}

#[given(regex = r"^the canonical holder 0x([0-9a-fA-F]+) is empty$")]
async fn given_empty_canonical_holder(world: &mut BevyoutWorld, reference_hex: String) {
    world
        .canonical_ledger
        .insert_holder(
            HolderId::FixtureContainer {
                reference_form_id: parse_hex(&reference_hex),
            },
            ItemHolderState::default(),
        )
        .unwrap();
}

#[when(
    regex = r"^transferring (\d+) of item 0x([0-9a-fA-F]+) to canonical holder 0x([0-9a-fA-F]+)$"
)]
async fn when_canonical_transfer(
    world: &mut BevyoutWorld,
    count: u32,
    item_hex: String,
    reference_hex: String,
) {
    world.canonical_result = Some(
        world
            .canonical_ledger
            .execute(TransactionRequest::Transfer {
                source: HolderId::Player,
                destination: HolderId::FixtureContainer {
                    reference_form_id: parse_hex(&reference_hex),
                },
                item_id: ItemInstanceId(parse_hex(&item_hex) as u64),
                count,
            }),
    );
}

#[then(regex = r"^the canonical player item 0x([0-9a-fA-F]+) has count (\d+)$")]
async fn then_canonical_player_count(world: &mut BevyoutWorld, item_hex: String, count: u32) {
    let item_id = ItemInstanceId(parse_hex(&item_hex) as u64);
    let item = world
        .canonical_ledger
        .holders()
        .get(&HolderId::Player)
        .and_then(|state| state.find(item_id))
        .unwrap_or_else(|| panic!("canonical player item {item_id:?} is missing"));
    assert_eq!(item.count, count);
}

#[then(regex = r"^canonical holder 0x([0-9a-fA-F]+) has item count (\d+)$")]
async fn then_canonical_holder_count(world: &mut BevyoutWorld, reference_hex: String, count: u32) {
    let holder = HolderId::FixtureContainer {
        reference_form_id: parse_hex(&reference_hex),
    };
    let actual = world
        .canonical_ledger
        .holders()
        .get(&holder)
        .and_then(|state| state.items.first())
        .map_or(0, |item| item.count);
    assert_eq!(actual, count);
}

#[then(regex = r"^the transaction moved item id 0x([0-9a-fA-F]+)$")]
async fn then_canonical_moved_id(world: &mut BevyoutWorld, item_hex: String) {
    let receipt = world
        .canonical_result
        .as_ref()
        .expect("canonical transaction was not run")
        .as_ref()
        .expect("canonical transaction failed");
    assert_eq!(
        receipt.moved[0].0,
        ItemInstanceId(parse_hex(&item_hex) as u64)
    );
}

#[then("the canonical transaction is rejected")]
async fn then_canonical_rejected(world: &mut BevyoutWorld) {
    assert!(matches!(
        world.canonical_result.as_ref(),
        Some(Err(TransactionError::InsufficientItems))
    ));
}

#[then(regex = r"^canonical holder 0x([0-9a-fA-F]+) is empty$")]
async fn then_canonical_holder_empty(world: &mut BevyoutWorld, reference_hex: String) {
    let holder = HolderId::FixtureContainer {
        reference_form_id: parse_hex(&reference_hex),
    };
    assert!(
        world
            .canonical_ledger
            .holders()
            .get(&holder)
            .is_some_and(|state| state.items.is_empty())
    );
}

// ---------------------------------------------------------------------
// drop_placement.feature (M3/#95)
// ---------------------------------------------------------------------

#[given("no drop candidates are blocked")]
async fn given_no_drop_candidates_blocked(world: &mut BevyoutWorld) {
    world.drop_blocked_count = 0;
    world.drop_all_blocked = false;
}

#[given(regex = r"^the first (\d+) drop candidates are blocked$")]
async fn given_first_drop_candidates_blocked(world: &mut BevyoutWorld, count: usize) {
    world.drop_blocked_count = count;
    world.drop_all_blocked = false;
}

#[given("every drop candidate is blocked")]
async fn given_every_drop_candidate_blocked(world: &mut BevyoutWorld) {
    world.drop_blocked_count = 0;
    world.drop_all_blocked = true;
}

#[when("the drop placement candidates are evaluated")]
async fn when_drop_placement_candidates_are_evaluated(world: &mut BevyoutWorld) {
    let blocked_count = world.drop_blocked_count;
    let all_blocked = world.drop_all_blocked;
    world.drop_decision = Some(drop_policy::choose_candidate(|distance| {
        if all_blocked {
            return true;
        }
        let candidate_index = ((drop_policy::DROP_DISTANCE_METERS - distance)
            / drop_policy::DROP_RETREAT_STEP_METERS)
            .round() as usize;
        candidate_index < blocked_count
    }));
}

#[then(regex = r"^the drop placement mode is (Camera|Retreat|PlayerFallback)$")]
async fn then_drop_placement_mode(world: &mut BevyoutWorld, mode: String) {
    let expected = match mode.as_str() {
        "Camera" => drop_policy::DropPlacementMode::Camera,
        "Retreat" => drop_policy::DropPlacementMode::Retreat,
        "PlayerFallback" => drop_policy::DropPlacementMode::PlayerFallback,
        other => panic!("unknown drop placement mode {other}"),
    };
    let decision = world
        .drop_decision
        .as_ref()
        .expect("drop placement was not evaluated");
    assert_eq!(decision.mode, expected);
}

#[then(regex = r"^the selected drop distance is ([\d.]+) metres$")]
async fn then_selected_drop_distance(world: &mut BevyoutWorld, expected: f32) {
    let actual = world
        .drop_decision
        .as_ref()
        .and_then(|decision| decision.distance)
        .expect("drop placement did not select a camera distance");
    assert!(
        (actual - expected).abs() < 0.001,
        "expected {expected}, got {actual}"
    );
}

#[then("the drop distance is the player fallback")]
async fn then_drop_distance_is_player_fallback(world: &mut BevyoutWorld) {
    let decision = world
        .drop_decision
        .as_ref()
        .expect("drop placement was not evaluated");
    assert_eq!(
        decision.mode,
        drop_policy::DropPlacementMode::PlayerFallback
    );
    assert_eq!(decision.distance, None);
}

// item_use.feature (issue #99) -- appended section, do not interleave;
// new steps for later issues belong below this marker.
// ---------------------------------------------------------------------

#[given(regex = r"^an item with stats (Aid|Key|Misc) quest (yes|no)$")]
async fn given_item_stats_flat(world: &mut BevyoutWorld, stats: String, quest: String) {
    world.item_use_stats = Some(match stats.as_str() {
        "Aid" => item_use::ItemStats::Aid,
        "Key" => item_use::ItemStats::Key,
        "Misc" => item_use::ItemStats::Misc,
        other => panic!("unexpected item stats {other}"),
    });
    world.item_use_quest_item = quest == "yes";
}

#[given(regex = r"^an item with stats (Book|Note) text (yes|no) quest (yes|no)$")]
async fn given_item_stats_with_text(
    world: &mut BevyoutWorld,
    stats: String,
    has_text: String,
    quest: String,
) {
    let has_text = has_text == "yes";
    world.item_use_stats = Some(match stats.as_str() {
        "Book" => item_use::ItemStats::Book { has_text },
        "Note" => item_use::ItemStats::Note { has_text },
        other => panic!("unexpected item stats {other}"),
    });
    world.item_use_quest_item = quest == "yes";
}

#[then(regex = r"^the item use action is (Use|Read|Inert)$")]
async fn then_item_use_action_is(world: &mut BevyoutWorld, expected: String) {
    let stats = world
        .item_use_stats
        .expect("item stats not given for this scenario");
    world.item_use_action = Some(item_use::classify(stats, world.item_use_quest_item));
    let expected = match expected.as_str() {
        "Use" => item_use::ItemUseAction::Use,
        "Read" => item_use::ItemUseAction::Read,
        "Inert" => item_use::ItemUseAction::Inert,
        other => panic!("unexpected item use action {other}"),
    };
    assert_eq!(world.item_use_action, Some(expected));
}

#[when(regex = r"^item 0x([0-9a-fA-F]+) with stats (Aid|Key) quest (yes|no) is used$")]
async fn when_item_is_used(
    world: &mut BevyoutWorld,
    form_id: String,
    stats: String,
    quest: String,
) {
    let stats = match stats.as_str() {
        "Aid" => item_use::ItemStats::Aid,
        "Key" => item_use::ItemStats::Key,
        other => panic!("unexpected item stats {other}"),
    };
    if item_use::classify(stats, quest == "yes") == item_use::ItemUseAction::Use {
        world.player_inventory.remove(
            inventory_policy::StackKey {
                base_form_id: parse_hex(&form_id),
                condition: None,
            },
            item_use::USE_CONSUMES_COUNT,
        );
    }
}

// equipment.feature (issue #98)
// ---------------------------------------------------------------------

fn equipment_stack_key(hex: &str, condition: &str) -> viewer_player::inventory::StackKey {
    viewer_player::inventory::StackKey {
        base_form_id: parse_hex(hex),
        condition: if condition == "none" {
            None
        } else {
            Some(condition.parse().expect("condition must be a whole number"))
        },
    }
}

#[given(regex = r"^apparel 0x([0-9a-fA-F]+) condition (\S+) mask 0x([0-9a-fA-F]+) is equipped$")]
async fn given_apparel_equipped(
    world: &mut BevyoutWorld,
    hex: String,
    condition: String,
    mask: String,
) {
    when_apparel_equipped(world, hex, condition, mask).await;
}

#[when(regex = r"^apparel 0x([0-9a-fA-F]+) condition (\S+) mask 0x([0-9a-fA-F]+) is equipped$")]
async fn when_apparel_equipped(
    world: &mut BevyoutWorld,
    hex: String,
    condition: String,
    mask: String,
) {
    let key = equipment_stack_key(&hex, &condition);
    let biped_slot_mask = parse_hex(&mask);
    world.equip_result = Some(
        world
            .equipment_state
            .equip(key, equipment::EquipKind::Apparel { biped_slot_mask }),
    );
}

#[given(
    regex = r"^weapon 0x([0-9a-fA-F]+) condition (\S+) requiring ammo 0x([0-9a-fA-F]+) is equipped$"
)]
async fn given_weapon_equipped(
    world: &mut BevyoutWorld,
    hex: String,
    condition: String,
    ammo_hex: String,
) {
    when_weapon_equipped(world, hex, condition, ammo_hex).await;
}

#[when(
    regex = r"^weapon 0x([0-9a-fA-F]+) condition (\S+) requiring ammo 0x([0-9a-fA-F]+) is equipped$"
)]
async fn when_weapon_equipped(
    world: &mut BevyoutWorld,
    hex: String,
    condition: String,
    ammo_hex: String,
) {
    let key = equipment_stack_key(&hex, &condition);
    let ammo_form_id = Some(parse_hex(&ammo_hex));
    world.equip_result = Some(
        world
            .equipment_state
            .equip(key, equipment::EquipKind::Weapon { ammo_form_id }),
    );
}

#[given(regex = r"^ammo 0x([0-9a-fA-F]+) condition (\S+) is equipped$")]
async fn given_ammo_equipped(world: &mut BevyoutWorld, hex: String, condition: String) {
    when_ammo_equipped(world, hex, condition).await;
}

#[when(regex = r"^ammo 0x([0-9a-fA-F]+) condition (\S+) is equipped$")]
async fn when_ammo_equipped(world: &mut BevyoutWorld, hex: String, condition: String) {
    let key = equipment_stack_key(&hex, &condition);
    world.equip_result = Some(world.equipment_state.equip(key, equipment::EquipKind::Ammo));
}

#[then(regex = r"^(?:apparel|weapon|ammo) 0x([0-9a-fA-F]+) condition (\S+) is equipped$")]
async fn then_stack_is_equipped(world: &mut BevyoutWorld, hex: String, condition: String) {
    let key = equipment_stack_key(&hex, &condition);
    assert!(world.equipment_state.is_equipped(key));
}

#[then(regex = r"^(?:apparel|weapon|ammo) 0x([0-9a-fA-F]+) condition (\S+) is evicted$")]
async fn then_stack_is_evicted(world: &mut BevyoutWorld, hex: String, condition: String) {
    let key = equipment_stack_key(&hex, &condition);
    assert!(!world.equipment_state.is_equipped(key));
    let outcome = world
        .equip_result
        .clone()
        .expect("no equip has been performed yet")
        .expect("the last equip did not succeed");
    assert!(
        outcome.evicted.contains(&key),
        "expected {key:?} in evicted {:?}",
        outcome.evicted
    );
}

#[then("the equip attempt is rejected as not equippable")]
async fn then_rejected_not_equippable(world: &mut BevyoutWorld) {
    assert_eq!(
        world.equip_result,
        Some(Err(equipment::EquipError::NotEquippable))
    );
}

#[then("the equip attempt is rejected as incompatible ammo")]
async fn then_rejected_incompatible_ammo(world: &mut BevyoutWorld) {
    assert_eq!(
        world.equip_result,
        Some(Err(equipment::EquipError::IncompatibleAmmo))
    );
}

#[then("the equip attempt is rejected with no weapon equipped")]
async fn then_rejected_no_weapon_equipped(world: &mut BevyoutWorld) {
    assert_eq!(
        world.equip_result,
        Some(Err(equipment::EquipError::NoWeaponEquipped))
    );
}

#[then(regex = r"^dropping apparel 0x([0-9a-fA-F]+) condition (\S+) is refused while equipped$")]
async fn then_drop_refused_while_equipped(
    world: &mut BevyoutWorld,
    hex: String,
    condition: String,
) {
    let key = equipment_stack_key(&hex, &condition);
    assert!(world.equipment_state.is_equipped(key));
}

#[then(regex = r"^dropping apparel 0x([0-9a-fA-F]+) condition (\S+) is allowed$")]
async fn then_drop_allowed(world: &mut BevyoutWorld, hex: String, condition: String) {
    let key = equipment_stack_key(&hex, &condition);
    assert!(!world.equipment_state.is_equipped(key));
}

// recipes.feature (issue #117) -- appended section, do not interleave.
// ---------------------------------------------------------------------

#[given(
    regex = r"^a recipe with ingredient 0x([0-9a-fA-F]+) quantity (-?\d+) and output 0x([0-9a-fA-F]+) quantity (-?\d+)$"
)]
async fn given_recipe(
    world: &mut BevyoutWorld,
    ingredient_form_id: String,
    ingredient_quantity: i32,
    output_form_id: String,
    output_quantity: i32,
) {
    world.recipe_under_test = Some(recipe_policy::PreparedRecipe {
        form_id: 0x100,
        ingredients: vec![recipe_policy::PreparedRecipeItem {
            item_form_id: parse_hex(&ingredient_form_id),
            quantity: ingredient_quantity,
            order: 0,
        }],
        outputs: vec![recipe_policy::PreparedRecipeItem {
            item_form_id: parse_hex(&output_form_id),
            quantity: output_quantity,
            order: 0,
        }],
        ..Default::default()
    });
}

#[given(regex = r#"^recipe items "([^"]*)" are available$"#)]
async fn given_recipe_items_available(world: &mut BevyoutWorld, items: String) {
    world.recipe_available_items = items
        .split(',')
        .map(|item| parse_hex(item.trim()))
        .collect();
}

#[given(regex = r"^the recipe also has ingredient 0x([0-9a-fA-F]+) quantity (-?\d+)$")]
async fn given_duplicate_recipe_ingredient(
    world: &mut BevyoutWorld,
    item_form_id: String,
    quantity: i32,
) {
    world
        .recipe_under_test
        .as_mut()
        .expect("recipe must be created first")
        .ingredients
        .push(recipe_policy::PreparedRecipeItem {
            item_form_id: parse_hex(&item_form_id),
            quantity,
            order: 1,
        });
}

#[when("the recipe is validated")]
async fn when_recipe_validated(world: &mut BevyoutWorld) {
    let recipe = world
        .recipe_under_test
        .as_ref()
        .expect("recipe must be created first");
    world.recipe_validation = Some(recipe_policy::validate_recipe(
        recipe,
        &world.recipe_available_items,
    ));
}

#[then("recipe validation rejects a non-positive quantity")]
async fn then_recipe_rejects_non_positive_quantity(world: &mut BevyoutWorld) {
    assert!(matches!(
        world.recipe_validation.as_ref(),
        Some(Err(
            recipe_policy::RecipeValidationError::NonPositiveQuantity { .. }
        ))
    ));
}

#[then("recipe validation rejects duplicate ingredients")]
async fn then_recipe_rejects_duplicate_ingredients(world: &mut BevyoutWorld) {
    assert!(matches!(
        world.recipe_validation.as_ref(),
        Some(Err(
            recipe_policy::RecipeValidationError::DuplicateIngredient { .. }
        ))
    ));
}

#[then(regex = r"^the recipe ingredient quantity remains (-?\d+)$")]
async fn then_recipe_quantity_unchanged(world: &mut BevyoutWorld, expected: i32) {
    assert_eq!(
        world
            .recipe_under_test
            .as_ref()
            .expect("recipe must be created first")
            .ingredients[0]
            .quantity,
        expected
    );
}

fn main() {
    futures::executor::block_on(async {
        BevyoutWorld::cucumber()
            .fail_on_skipped()
            .run_and_exit("features")
            .await;
    });
}
