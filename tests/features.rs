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

use assets::AssetConversion;
use cucumber::{World as _, given, then, when};
use manifest::{PreparedPlacement, PreparedSceneManifest, PreparedSemantic};
use paths::{CellSelector, normalize_asset_path, parse_cell_selector, placement_transform_parts};

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

fn main() {
    futures::executor::block_on(async {
        BevyoutWorld::cucumber()
            .fail_on_skipped()
            .run_and_exit("features")
            .await;
    });
}
