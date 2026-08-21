//! Tests for `NAVM`/`NAVI` subrecord decoding (issue #111, M4 wave 2).
//! Synthetic fixtures only, built byte-by-byte with the shared
//! `tests/mod.rs` builders -- no real game data. End-to-end scenarios
//! (count mismatches, truncation, NAVI override/delete semantics, and the
//! prepared graph) live in `features/nav_graph.feature`; these unit tests
//! cover the seams the cucumber steps do not reach: direct
//! `parse_navmesh`/`parse_navi` calls with a master-adjusting resolver,
//! malformed `DATA`/`NVGD`/`NVMI` sizes, and the unsupported-subrecord
//! diagnostic for `NVCI`.

use super::*;

fn master_resolver() -> FormIdResolver {
    // A plugin loaded at global index 1 whose only master sits at global
    // index 0: local FormIDs with file index 0 stay on the master, local
    // file index 1 maps to this plugin's global index 1.
    FormIdResolver::new(1, vec![0])
}

fn navm_data(cell: u32, counts: [u32; 5]) -> Vec<u8> {
    let mut data = cell.to_le_bytes().to_vec();
    for count in counts {
        data.extend_from_slice(&count.to_le_bytes());
    }
    data
}

#[test]
fn decodes_every_navm_subrecord_and_adjusts_form_ids() {
    let resolver = master_resolver();
    let vertices: Vec<u8> = [70.0_f32, 140.0, -210.0, 0.0, 0.0, 0.0, 35.0, 0.0, 0.0]
        .into_iter()
        .flat_map(f32::to_le_bytes)
        .collect();
    let mut triangle = Vec::new();
    for value in [0_i16, 1, 2, -1, -1, -1] {
        triangle.extend_from_slice(&value.to_le_bytes());
    }
    triangle.extend_from_slice(&0x0000_0640_u32.to_le_bytes());
    let mut door = 0x0100_0d00_u32.to_le_bytes().to_vec(); // file index 1 -> stays 1
    door.extend_from_slice(&0_u16.to_le_bytes());
    door.extend_from_slice(&[0, 0]);
    let mut grid = 2_u32.to_le_bytes().to_vec();
    for value in [140.0_f32, 140.0, -70.0, -70.0, 0.0, 70.0, 70.0, 0.0] {
        grid.extend_from_slice(&value.to_le_bytes());
    }
    let mut external = vec![0_u8; 4];
    external.extend_from_slice(&0x0000_0501_u32.to_le_bytes()); // file index 0 -> master
    external.extend_from_slice(&0_u16.to_le_bytes());

    let subs = vec![
        direct_subrecord("NVER", 12_u32.to_le_bytes().to_vec()),
        direct_subrecord("DATA", navm_data(0x0000_0c00, [3, 1, 1, 2, 1])),
        direct_subrecord("NVVX", vertices),
        direct_subrecord("NVTR", triangle),
        direct_subrecord(
            "NVCA",
            [0_i16, 0].iter().flat_map(|v| v.to_le_bytes()).collect(),
        ),
        direct_subrecord("NVDP", door),
        direct_subrecord("NVGD", grid),
        direct_subrecord("NVEX", external),
    ];
    let navmesh = parse_navmesh(&subs, 0x500, 0, &resolver, Vec::new());

    assert_eq!(navmesh.version, Some(12));
    // DATA's cell FormID has local file index 0 -> resolved onto the master
    // (global index 0), unchanged here.
    assert_eq!(navmesh.cell_form_id, Some(0x0000_0c00));
    assert_eq!(navmesh.vertices.len(), 3);
    assert_eq!(navmesh.vertices[0], [70.0, 140.0, -210.0]);
    assert_eq!(navmesh.triangles.len(), 1);
    assert_eq!(navmesh.triangles[0].vertex_indices, [0, 1, 2]);
    assert_eq!(navmesh.triangles[0].edge_neighbors, [-1, -1, -1]);
    assert_eq!(navmesh.triangles[0].flags, 0x0000_0640);
    assert_eq!(navmesh.cover_triangle_ids, [0, 0]);
    // NVDP's door reference had local file index 1 (this plugin itself);
    // the resolver keeps it on global index 1.
    assert_eq!(navmesh.doors[0].door_reference_form_id, Some(0x0100_0d00));
    assert_eq!(navmesh.doors[0].triangle, 0);
    assert_eq!(navmesh.grid.unwrap().divisor, 2);
    assert_eq!(navmesh.grid.unwrap().min, [-70.0, -70.0, 0.0]);
    assert_eq!(navmesh.grid.unwrap().max, [70.0, 70.0, 0.0]);
    // NVEX's target had local file index 0 -> adjusted onto the master.
    assert_eq!(
        navmesh.external_connections[0].target_navmesh_form_id,
        Some(0x0000_0501)
    );
    assert!(navmesh.diagnostics.is_empty(), "{:?}", navmesh.diagnostics);
}

#[test]
fn malformed_data_and_short_nvgd_are_diagnosed_not_fatal() {
    let resolver = direct_resolver();
    let subs = vec![
        direct_subrecord("DATA", vec![0; 20]), // must be exactly 24
        direct_subrecord("NVGD", vec![0; 35]), // header needs 36
    ];
    let navmesh = parse_navmesh(&subs, 0x500, 0, &resolver, Vec::new());
    assert!(navmesh.cell_form_id.is_none());
    assert!(navmesh.grid.is_none());
    assert!(
        navmesh
            .diagnostics
            .iter()
            .any(|message| message == "DATA malformed: expected 24 bytes, got 20")
    );
    assert!(
        navmesh
            .diagnostics
            .iter()
            .any(|message| message == "NVGD malformed: expected at least 36 bytes, got 35")
    );
}

#[test]
fn null_door_and_external_form_ids_decode_as_none() {
    let resolver = direct_resolver();
    let mut door = 0_u32.to_le_bytes().to_vec();
    door.extend_from_slice(&3_u16.to_le_bytes());
    door.extend_from_slice(&[0, 0]);
    let mut external = vec![0_u8; 4];
    external.extend_from_slice(&0_u32.to_le_bytes());
    external.extend_from_slice(&7_u16.to_le_bytes());
    let subs = vec![
        direct_subrecord("NVDP", door),
        direct_subrecord("NVEX", external),
    ];
    let navmesh = parse_navmesh(&subs, 0x500, 0, &resolver, Vec::new());
    assert_eq!(navmesh.doors[0].door_reference_form_id, None);
    assert_eq!(navmesh.doors[0].triangle, 3);
    assert_eq!(navmesh.external_connections[0].target_navmesh_form_id, None);
    assert_eq!(navmesh.external_connections[0].triangle, 7);
}

/// Builds one `NVMI` entry's 16-byte header (issue #111 layout, unchanged by
/// #113).
fn nvmi_header(navmesh: u32, location: u32, grid_x: i16, grid_y: i16) -> Vec<u8> {
    let mut nvmi = vec![9, 9, 9, 9];
    nvmi.extend_from_slice(&navmesh.to_le_bytes());
    nvmi.extend_from_slice(&location.to_le_bytes());
    nvmi.extend_from_slice(&grid_x.to_le_bytes());
    nvmi.extend_from_slice(&grid_y.to_le_bytes());
    nvmi
}

fn push_f32_3(data: &mut Vec<u8>, value: [f32; 3]) {
    for component in value {
        data.extend_from_slice(&component.to_le_bytes());
    }
}

#[test]
fn parses_navi_header_fields_and_short_tail_with_no_island() {
    // Real-data shape (issue #113): center point + a bare 4-byte trailing
    // field, no bounds/island block -- e.g. FranklinMetro02's `0005429f`
    // entry (real tail length 16).
    let resolver = direct_resolver();
    let mut nvmi = nvmi_header(0x500, 0xc00, 3, -4);
    push_f32_3(&mut nvmi, [1.0, 2.0, 3.0]);
    nvmi.extend_from_slice(&[0, 0, 0, 0]);
    let subs = vec![
        direct_subrecord("NVER", 12_u32.to_le_bytes().to_vec()),
        direct_subrecord("NVMI", nvmi),
    ];
    let (navi, diagnostics) = parse_navi(&subs, 0xE00, 0, &resolver);
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    assert_eq!(navi.version, Some(12));
    assert_eq!(navi.entries.len(), 1);
    let entry = &navi.entries[0];
    assert_eq!(entry.unknown, [9, 9, 9, 9]);
    assert_eq!(entry.navmesh_form_id, Some(0x500));
    assert_eq!(entry.location_form_id, Some(0xc00));
    assert_eq!(entry.grid_x, 3);
    assert_eq!(entry.grid_y, -4);
    assert_eq!(entry.center, Some([1.0, 2.0, 3.0]));
    assert_eq!(entry.bounds, None);
    assert!(entry.island.is_none());
    assert_eq!(entry.trailing, [0, 0, 0, 0]);
    assert!(entry.tail.is_empty());
}

#[test]
fn parses_navi_tail_bounds_with_an_empty_island() {
    // Real-data shape: center + bounds + zero vertex/triangle counts --
    // e.g. the Capital Wasteland exterior grid entries sampled for issue
    // #113 (real tail length 44).
    let resolver = direct_resolver();
    let mut nvmi = nvmi_header(0x510, 0x3c, 5, -6);
    push_f32_3(&mut nvmi, [10.0, 20.0, 30.0]);
    push_f32_3(&mut nvmi, [0.0, 0.0, 0.0]); // bounds min
    push_f32_3(&mut nvmi, [1.0, 1.0, 1.0]); // bounds max
    nvmi.extend_from_slice(&0_u16.to_le_bytes()); // vertex_count
    nvmi.extend_from_slice(&0_u16.to_le_bytes()); // triangle_count
    nvmi.extend_from_slice(&[0, 0, 0, 0]); // trailing
    let subs = vec![direct_subrecord("NVMI", nvmi)];
    let (navi, diagnostics) = parse_navi(&subs, 0xE00, 0, &resolver);
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    let entry = &navi.entries[0];
    assert_eq!(
        entry.bounds,
        Some(NaviBounds {
            min: [0.0, 0.0, 0.0],
            max: [1.0, 1.0, 1.0],
        })
    );
    assert_eq!(
        entry.island,
        Some(NaviIsland {
            vertices: Vec::new(),
            triangles: Vec::new(),
        })
    );
    assert!(entry.tail.is_empty());
}

#[test]
fn parses_navi_tail_island_vertices_and_triangles() {
    // Real-data shape: center + bounds + a nonzero local vertex/triangle
    // sub-mesh -- e.g. FranklinMetro02's `0005429e` entry (real tail length
    // 284, 14 vertices/12 triangles). This fixture uses a small 3-vertex/
    // 1-triangle island to keep the byte math legible.
    let resolver = direct_resolver();
    let mut nvmi = nvmi_header(0x520, 0x1a273, 0, 0);
    push_f32_3(&mut nvmi, [5.0, 6.0, 7.0]); // center
    push_f32_3(&mut nvmi, [0.0, 0.0, 0.0]); // bounds min
    push_f32_3(&mut nvmi, [9.0, 9.0, 9.0]); // bounds max
    nvmi.extend_from_slice(&3_u16.to_le_bytes()); // vertex_count
    nvmi.extend_from_slice(&1_u16.to_le_bytes()); // triangle_count
    push_f32_3(&mut nvmi, [0.0, 0.0, 0.0]);
    push_f32_3(&mut nvmi, [1.0, 0.0, 0.0]);
    push_f32_3(&mut nvmi, [0.0, 1.0, 0.0]);
    for index in [0_u16, 1, 2] {
        nvmi.extend_from_slice(&index.to_le_bytes());
    }
    nvmi.extend_from_slice(&[7, 0, 0, 0]); // trailing (non-zero to prove it's retained raw)
    let subs = vec![direct_subrecord("NVMI", nvmi)];
    let (navi, diagnostics) = parse_navi(&subs, 0xE00, 0, &resolver);
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    let entry = &navi.entries[0];
    assert_eq!(
        entry.island,
        Some(NaviIsland {
            vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            triangles: vec![[0, 1, 2]],
        })
    );
    assert_eq!(entry.trailing, [7, 0, 0, 0]);
    assert!(entry.tail.is_empty());
}

#[test]
fn navi_tail_truncation_is_diagnosed_never_panics_and_retains_the_remainder() {
    let resolver = direct_resolver();

    // Fewer than 12 bytes: can't even hold the center point.
    let mut short_center = nvmi_header(0x1, 0x2, 0, 0);
    short_center.extend_from_slice(&[1, 2, 3]);
    // Between a bare trailing field and a full island block (neither 0, 4,
    // nor >= 28 bytes remain after the center point).
    let mut partial_island = nvmi_header(0x3, 0x4, 0, 0);
    push_f32_3(&mut partial_island, [0.0, 0.0, 0.0]);
    partial_island.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    // Declares more vertices than the remaining bytes can hold.
    let mut truncated_island = nvmi_header(0x5, 0x6, 0, 0);
    push_f32_3(&mut truncated_island, [0.0, 0.0, 0.0]);
    push_f32_3(&mut truncated_island, [0.0, 0.0, 0.0]);
    push_f32_3(&mut truncated_island, [0.0, 0.0, 0.0]);
    truncated_island.extend_from_slice(&5_u16.to_le_bytes()); // declares 5 vertices
    truncated_island.extend_from_slice(&0_u16.to_le_bytes());
    push_f32_3(&mut truncated_island, [1.0, 1.0, 1.0]); // only 1 actually present

    let subs = vec![
        direct_subrecord("NVMI", short_center),
        direct_subrecord("NVMI", partial_island),
        direct_subrecord("NVMI", truncated_island),
    ];
    let (navi, diagnostics) = parse_navi(&subs, 0xE00, 0, &resolver);
    assert_eq!(
        navi.entries.len(),
        3,
        "malformed tails never drop the entry"
    );

    assert_eq!(navi.entries[0].center, None);
    assert_eq!(navi.entries[0].tail, vec![1, 2, 3]);

    assert_eq!(navi.entries[1].center, Some([0.0, 0.0, 0.0]));
    assert_eq!(navi.entries[1].bounds, None);
    assert_eq!(navi.entries[1].tail, vec![1, 2, 3, 4, 5, 6, 7, 8]);

    assert_eq!(navi.entries[2].center, Some([0.0, 0.0, 0.0]));
    assert!(navi.entries[2].bounds.is_some());
    assert!(navi.entries[2].island.is_none());
    assert_eq!(navi.entries[2].tail.len(), 12);

    assert!(diagnostics.iter().any(|message| message.contains("NVMI 0:")
        && message.contains("too short for the 12-byte center point")));
    assert!(diagnostics.iter().any(|message| message.contains("NVMI 1:")
        && message.contains("do not form a complete island block")));
    assert!(diagnostics.iter().any(|message| message.contains("NVMI 2:")
        && message.contains("declares 5 vertex(es)/0 triangle(s) needing 60 byte(s)")));
}

#[test]
fn short_nvmi_and_unsupported_subrecords_are_diagnosed() {
    let resolver = direct_resolver();
    let subs = vec![
        direct_subrecord("NVMI", vec![0; 15]), // header needs 16
        direct_subrecord("NVSI", vec![0; 4]),  // still undocumented for FO3/FNV
    ];
    let (navi, diagnostics) = parse_navi(&subs, 0xE00, 0, &resolver);
    assert!(navi.entries.is_empty());
    assert!(
        diagnostics
            .iter()
            .any(|message| message == "NVMI malformed: expected at least 16 bytes, got 15")
    );
    assert!(
        diagnostics
            .iter()
            .any(|message| message.contains("ignored unsupported NAVI.NVSI subrecord"))
    );
}

// -------------------------------------------------------------
// NVCI (issue #156, F156.3): correlation-only decode, following fopdoc's
// literal documented byte layout (not real-data-verified -- see
// `decode_navi_correlation`'s doc comment).
// -------------------------------------------------------------

#[test]
fn decodes_nvci_leading_field_and_repeating_entries() {
    let resolver = master_resolver();
    let mut nvci = 0x0100_0900_u32.to_le_bytes().to_vec(); // leading: file index 1
    nvci.extend_from_slice(&0x0000_0a00_u32.to_le_bytes()); // entry.navmesh: master
    nvci.extend_from_slice(&0x0100_0b00_u32.to_le_bytes()); // entry.other_navmesh: this plugin
    nvci.extend_from_slice(&0x0000_0c00_u32.to_le_bytes()); // entry.door: master
    let subs = vec![direct_subrecord("NVCI", nvci)];
    let (navi, diagnostics) = parse_navi(&subs, 0xE00, 0, &resolver);
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    assert_eq!(navi.correlations.len(), 1);
    let correlation = &navi.correlations[0];
    assert_eq!(correlation.leading_navmesh_form_id, Some(0x0100_0900));
    assert_eq!(correlation.entries.len(), 1);
    assert_eq!(correlation.entries[0].navmesh_form_id, Some(0x0000_0a00));
    assert_eq!(
        correlation.entries[0].other_navmesh_form_id,
        Some(0x0100_0b00)
    );
    assert_eq!(correlation.entries[0].door_form_id, Some(0x0000_0c00));
}

#[test]
fn nvci_with_no_entries_decodes_the_leading_field_only() {
    let resolver = direct_resolver();
    let nvci = 0x500_u32.to_le_bytes().to_vec();
    let subs = vec![direct_subrecord("NVCI", nvci)];
    let (navi, diagnostics) = parse_navi(&subs, 0xE00, 0, &resolver);
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    assert_eq!(navi.correlations.len(), 1);
    assert_eq!(navi.correlations[0].leading_navmesh_form_id, Some(0x500));
    assert!(navi.correlations[0].entries.is_empty());
}

#[test]
fn null_nvci_form_ids_decode_as_none() {
    let resolver = direct_resolver();
    let mut nvci = 0_u32.to_le_bytes().to_vec();
    nvci.extend_from_slice(&0_u32.to_le_bytes());
    nvci.extend_from_slice(&0_u32.to_le_bytes());
    nvci.extend_from_slice(&0_u32.to_le_bytes());
    let subs = vec![direct_subrecord("NVCI", nvci)];
    let (navi, diagnostics) = parse_navi(&subs, 0xE00, 0, &resolver);
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    assert_eq!(navi.correlations[0].leading_navmesh_form_id, None);
    assert_eq!(navi.correlations[0].entries[0].navmesh_form_id, None);
    assert_eq!(navi.correlations[0].entries[0].other_navmesh_form_id, None);
    assert_eq!(navi.correlations[0].entries[0].door_form_id, None);
}

#[test]
fn malformed_nvci_is_diagnosed_never_panics_and_retains_the_leading_field() {
    let resolver = direct_resolver();
    // Too short even for the leading FormID.
    let too_short = direct_subrecord("NVCI", vec![1, 2, 3]);
    // Leading field plus a partial (non-12-byte-multiple) trailing entry.
    let mut partial_entry = 0x500_u32.to_le_bytes().to_vec();
    partial_entry.extend_from_slice(&[1, 2, 3, 4, 5]);
    let subs = vec![too_short, direct_subrecord("NVCI", partial_entry)];
    let (navi, diagnostics) = parse_navi(&subs, 0xE00, 0, &resolver);
    assert_eq!(
        navi.correlations.len(),
        2,
        "malformed NVCI is never dropped"
    );
    assert_eq!(navi.correlations[0], NaviCorrelation::default());
    assert_eq!(navi.correlations[1].leading_navmesh_form_id, Some(0x500));
    assert!(navi.correlations[1].entries.is_empty());
    assert!(
        diagnostics
            .iter()
            .any(|message| message == "NVCI 0: malformed: expected at least 4 bytes, got 3")
    );
    assert!(diagnostics.iter().any(|message| message.contains("NVCI 1:")
        && message.contains("5 trailing byte(s)")
        && message.contains("do not form a complete 12-byte correlation entry")));
}
