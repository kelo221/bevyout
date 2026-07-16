//! Tests for `NAVM`/`NAVI` subrecord decoding (issue #111, M4 wave 2).
//! Synthetic fixtures only, built byte-by-byte with the shared
//! `tests/mod.rs` builders -- no real game data. End-to-end scenarios
//! (count mismatches, truncation, NAVI override/delete semantics, and the
//! prepared graph) live in `features/nav_graph.feature`; these unit tests
//! cover the seams the cucumber steps do not reach: direct
//! `parse_navmesh`/`parse_navi` calls with a master-adjusting resolver,
//! malformed `DATA`/`NVGD`/`NVMI` sizes, and the unsupported-subrecord
//! diagnostic for `NVCI`.

use super::super::*;
use super::*;

fn master_resolver() -> FormIdResolver {
    // A plugin loaded at global index 1 whose only master sits at global
    // index 0: local FormIDs with file index 0 stay on the master, local
    // file index 1 maps to this plugin's global index 1.
    FormIdResolver {
        current_index: 1,
        master_indices: vec![0],
    }
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

#[test]
fn parses_navi_entries_with_undocumented_tail_retained() {
    let resolver = direct_resolver();
    let mut nvmi = vec![9, 9, 9, 9];
    nvmi.extend_from_slice(&0x500_u32.to_le_bytes());
    nvmi.extend_from_slice(&0xc00_u32.to_le_bytes());
    nvmi.extend_from_slice(&3_i16.to_le_bytes());
    nvmi.extend_from_slice(&(-4_i16).to_le_bytes());
    nvmi.extend_from_slice(&[1, 2, 3]); // undocumented uint8[] tail
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
    assert_eq!(entry.tail, vec![1, 2, 3]);
}

#[test]
fn short_nvmi_and_unsupported_nvci_are_diagnosed() {
    let resolver = direct_resolver();
    let subs = vec![
        direct_subrecord("NVMI", vec![0; 15]), // header needs 16
        direct_subrecord("NVCI", vec![0; 16]),
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
            .any(|message| message.contains("ignored unsupported NAVI.NVCI subrecord"))
    );
}
