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
}
use prepare::batch_cache;
use prepare::fingerprints;
use prepare::jobs;
use prepare::selectors;

use assets::AssetConversion;
use cucumber::{World as _, given, then, when};
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
    swap_placements: Vec<swap_policy::PlacementRef>,
    swap_deltas: std::collections::HashMap<u32, swap_policy::ReferenceDelta>,
    swap_applications: Vec<swap_policy::PlacementApplication>,
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

#[given(regex = r"^placement reference 0x([0-9a-fA-F]+) is spawned in the destination cell$")]
async fn given_swap_placement_spawned(world: &mut BevyoutWorld, hex: String) {
    world.swap_placements.push(swap_policy::PlacementRef {
        reference_form_id: parse_hex(&hex),
    });
}

#[given(regex = r"^the saved state disables reference 0x([0-9a-fA-F]+)$")]
async fn given_swap_state_disables(world: &mut BevyoutWorld, hex: String) {
    world.swap_deltas.insert(
        parse_hex(&hex),
        swap_policy::ReferenceDelta {
            enabled: Some(false),
            ..Default::default()
        },
    );
}

#[given(regex = r"^the saved state deletes reference 0x([0-9a-fA-F]+)$")]
async fn given_swap_state_deletes(world: &mut BevyoutWorld, hex: String) {
    world.swap_deltas.insert(
        parse_hex(&hex),
        swap_policy::ReferenceDelta {
            deleted: true,
            ..Default::default()
        },
    );
}

#[given(
    regex = r"^the saved state moves reference 0x([0-9a-fA-F]+) to \[(-?[\d.]+), (-?[\d.]+), (-?[\d.]+)\]$"
)]
async fn given_swap_state_moves(world: &mut BevyoutWorld, hex: String, x: f32, y: f32, z: f32) {
    world.swap_deltas.insert(
        parse_hex(&hex),
        swap_policy::ReferenceDelta {
            transform: Some(swap_policy::TransformDelta {
                translation: [x, y, z],
                rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                scale: [1.0, 1.0, 1.0],
            }),
            ..Default::default()
        },
    );
}

#[when("the persistent cell state is applied")]
async fn when_persistent_cell_state_applied(world: &mut BevyoutWorld) {
    world.swap_applications =
        swap_policy::apply_persistent_cell_state(&world.swap_deltas, &world.swap_placements);
}

fn find_swap_application(world: &BevyoutWorld, form_id: u32) -> &swap_policy::PlacementApplication {
    world
        .swap_applications
        .iter()
        .find(|application| application.reference_form_id == form_id)
        .expect("persistent cell state not applied yet, or reference not spawned")
}

#[then(regex = r"^placement reference 0x([0-9a-fA-F]+) is hidden$")]
async fn then_swap_placement_is_hidden(world: &mut BevyoutWorld, hex: String) {
    let application = find_swap_application(world, parse_hex(&hex));
    assert_eq!(
        application.visibility,
        swap_policy::VisibilityDecision::Hidden
    );
}

#[then(regex = r"^placement reference 0x([0-9a-fA-F]+) is visible$")]
async fn then_swap_placement_is_visible(world: &mut BevyoutWorld, hex: String) {
    let application = find_swap_application(world, parse_hex(&hex));
    assert_eq!(
        application.visibility,
        swap_policy::VisibilityDecision::Visible
    );
}

#[then(
    regex = r"^placement reference 0x([0-9a-fA-F]+) has translation \[(-?[\d.]+), (-?[\d.]+), (-?[\d.]+)\]$"
)]
async fn then_swap_placement_has_translation(
    world: &mut BevyoutWorld,
    hex: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let application = find_swap_application(world, parse_hex(&hex));
    let transform = application
        .transform
        .expect("expected a transform delta on this application");
    assert_eq!(transform.translation, [x, y, z]);
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

fn main() {
    futures::executor::block_on(async {
        BevyoutWorld::cucumber()
            .fail_on_skipped()
            .run_and_exit("features")
            .await;
    });
}
