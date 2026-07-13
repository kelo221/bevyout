use super::*;

fn parse_plugin(bytes: &[u8], target_cell: u32) -> Result<ParsedPlugin> {
    parse_content_set(
        &[PluginSource {
            name: "Fallout3.esm",
            bytes,
        }],
        &CellSelector::FormId(target_cell),
    )
}
use std::io::Write;

#[test]
fn parses_fo3_lighting_data_in_legacy_and_complete_lengths() {
    let mut data = vec![0_u8; 40];
    data[0..4].copy_from_slice(&[16, 32, 64, 255]);
    data[4..8].copy_from_slice(&[32, 64, 128, 255]);
    data[8..12].copy_from_slice(&[8, 16, 32, 255]);
    data[12..16].copy_from_slice(&1.5_f32.to_le_bytes());
    data[16..20].copy_from_slice(&25.0_f32.to_le_bytes());
    data[20..24].copy_from_slice(&(-10_i32).to_le_bytes());
    data[24..28].copy_from_slice(&20_i32.to_le_bytes());
    data[28..32].copy_from_slice(&0.75_f32.to_le_bytes());
    data[32..36].copy_from_slice(&40.0_f32.to_le_bytes());
    data[36..40].copy_from_slice(&2.0_f32.to_le_bytes());

    let complete = parse_lighting_data(&data).unwrap();
    let legacy = parse_lighting_data(&data[..36]).unwrap();
    assert_eq!(complete.ambient_rgba[0], 16.0 / 255.0);
    assert_eq!(complete.rotation_xy, -10);
    assert_eq!(complete.rotation_z, 20);
    assert_eq!(complete.fog_power, 2.0);
    assert_eq!(legacy.fog_power, 1.0);
    assert!(parse_lighting_data(&data[..35]).is_none());
    let mut ambiguous = data[..37].to_vec();
    assert!(parse_lighting_data(&ambiguous).is_none());
    ambiguous.resize(40, 0);
    assert!(parse_lighting_data(&ambiguous).is_some());
    let template = parse_lighting_template(
        &[Subrecord {
            signature: "DATA".into(),
            data,
        }],
        0x123,
        0,
    );
    assert_eq!(template.lighting.unwrap().fog_power, 2.0);
}

#[test]
fn applies_every_named_lighting_template_field() {
    let mut cell = LightingData {
        ambient_rgba: [1.0; 4],
        directional_rgba: [1.0; 4],
        fog_rgba: [1.0; 4],
        fog_near: 1.0,
        fog_far: 1.0,
        rotation_xy: 1,
        rotation_z: 1,
        fog_directional_fade: 1.0,
        fog_clip_distance: 1.0,
        fog_power: 1.0,
    };
    let template = LightingData {
        ambient_rgba: [2.0; 4],
        directional_rgba: [3.0; 4],
        fog_rgba: [4.0; 4],
        fog_near: 5.0,
        fog_far: 6.0,
        rotation_xy: 7,
        rotation_z: 8,
        fog_directional_fade: 9.0,
        fog_clip_distance: 10.0,
        fog_power: 11.0,
    };
    cell.apply_template(template, 0x1ff);
    assert_eq!(cell, template);
}

fn subrecord(signature: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut result = signature.to_vec();
    result.extend_from_slice(&(data.len() as u16).to_le_bytes());
    result.extend_from_slice(data);
    result
}

fn record(signature: &[u8; 4], flags: u32, form_id: u32, data: &[u8]) -> Vec<u8> {
    let mut result = signature.to_vec();
    result.extend_from_slice(&(data.len() as u32).to_le_bytes());
    result.extend_from_slice(&flags.to_le_bytes());
    result.extend_from_slice(&form_id.to_le_bytes());
    result.extend_from_slice(&[0; 8]);
    result.extend_from_slice(data);
    result
}

fn group(label: u32, group_type: i32, children: &[u8]) -> Vec<u8> {
    let mut result = b"GRUP".to_vec();
    result.extend_from_slice(&((children.len() + 24) as u32).to_le_bytes());
    result.extend_from_slice(&label.to_le_bytes());
    result.extend_from_slice(&group_type.to_le_bytes());
    result.extend_from_slice(&[0; 8]);
    result.extend_from_slice(children);
    result
}

fn tes4(masters: &[&str]) -> Vec<u8> {
    let mut data = Vec::new();
    for master in masters {
        data.extend(subrecord(b"MAST", format!("{master}\0").as_bytes()));
        data.extend(subrecord(b"DATA", &[0; 8]));
    }
    record(b"TES4", 0, 0, &data)
}

fn transform() -> Vec<u8> {
    [1.0_f32, 2.0, 3.0, 0.1, 0.2, 0.3]
        .into_iter()
        .flat_map(f32::to_le_bytes)
        .collect()
}

#[test]
fn reads_master_dependencies() {
    assert_eq!(
        read_master_names(&tes4(&["Fallout3.esm", "Update.esp"])).unwrap(),
        ["Fallout3.esm", "Update.esp"]
    );
}

#[test]
fn extended_subrecords_use_xxxx_size() {
    let mut data = subrecord(b"XXXX", &70_000_u32.to_le_bytes());
    data.extend_from_slice(b"BIGG");
    data.extend_from_slice(&0_u16.to_le_bytes());
    data.extend(vec![7; 70_000]);
    let parsed = parse_subrecords(&data).unwrap();
    assert_eq!(parsed[0].signature, "BIGG");
    assert_eq!(parsed[0].data.len(), 70_000);
}

#[test]
fn compressed_records_are_validated_and_decoded() {
    let original = subrecord(b"EDID", b"Compressed\0");
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(&original).unwrap();
    let compressed = encoder.finish().unwrap();
    let mut payload = (original.len() as u32).to_le_bytes().to_vec();
    payload.extend(compressed);
    assert_eq!(
        record_payload(&payload, RECORD_COMPRESSED, "TEST", 1).unwrap(),
        original
    );
}

#[test]
fn parses_audio_cell_and_enable_parent_metadata() {
    let resolver = FormIdResolver {
        current_index: 0,
        master_indices: Vec::new(),
    };
    let sound = parse_sound(
        &[
            Subrecord {
                signature: "EDID".into(),
                data: b"RoomTone\0".to_vec(),
            },
            Subrecord {
                signature: "FNAM".into(),
                data: b"fx/roomtone.wav\0".to_vec(),
            },
            Subrecord {
                signature: "SNDX".into(),
                data: [2_u8, 9, 0, 0, 0x50, 0, 0, 0, 0x6a, 0x02, 0, 0].to_vec(),
            },
        ],
        0x100,
        0,
    );
    assert_eq!(sound.file.as_deref(), Some("fx/roomtone.wav"));
    assert!(sound.parameters.unwrap().is_looping());
    assert!(sound.parameters.unwrap().is_two_dimensional());

    let acoustic = parse_acoustic_space(
        &[
            Subrecord {
                signature: "EDID".into(),
                data: b"Metal\0".to_vec(),
            },
            Subrecord {
                signature: "ANAM".into(),
                data: 7_u32.to_le_bytes().to_vec(),
            },
            Subrecord {
                signature: "SNAM".into(),
                data: 0x200_u32.to_le_bytes().to_vec(),
            },
        ],
        0x101,
        0,
        &resolver,
    );
    assert_eq!(acoustic.environment_type, Some(7));
    assert_eq!(acoustic.ambient_loop_sound_form_ids, [0x200]);

    let metadata = parse_cell_metadata(
        &[
            Subrecord {
                signature: "XCAS".into(),
                data: 0x101_u32.to_le_bytes().to_vec(),
            },
            Subrecord {
                signature: "XCMO".into(),
                data: 0x300_u32.to_le_bytes().to_vec(),
            },
            Subrecord {
                signature: "XCLW".into(),
                data: 1.5_f32.to_le_bytes().to_vec(),
            },
        ],
        &resolver,
    );
    assert_eq!(metadata.acoustic_space_form_id, Some(0x101));
    assert_eq!(metadata.music_form_id, Some(0x300));
    assert_eq!(metadata.water_height, Some(1.5));

    let reference = parse_reference(
        &[
            Subrecord {
                signature: "NAME".into(),
                data: 0x400_u32.to_le_bytes().to_vec(),
            },
            Subrecord {
                signature: "DATA".into(),
                data: transform(),
            },
            Subrecord {
                signature: "XOWN".into(),
                data: 0x500_u32.to_le_bytes().to_vec(),
            },
            Subrecord {
                signature: "XRNK".into(),
                data: (-2_i32).to_le_bytes().to_vec(),
            },
            Subrecord {
                signature: "XESP".into(),
                data: [0x600_u32.to_le_bytes(), 2_u32.to_le_bytes()].concat(),
            },
        ],
        0x700,
        0x800,
        0,
        ReferenceKind::Object,
        &resolver,
    )
    .unwrap()
    .unwrap();
    assert_eq!(reference.owner_form_id, Some(0x500));
    assert_eq!(reference.owner_faction_rank, Some(-2));
    assert_eq!(
        reference.enable_parent.unwrap().parent_reference_form_id,
        0x600
    );
}

#[test]
fn parses_actors_items_doors_and_navmesh() {
    let cell_id = 0x100;
    let destination_cell = 0x200;
    let door_ref = 0x300;
    let destination_ref = 0x301_u32;
    let door_base = 0x400;
    let npc_base = 0x401;
    let item_base = 0x402;
    let creature_base = 0x403;

    let cell = record(
        b"CELL",
        0,
        cell_id,
        &[subrecord(b"EDID", b"TestCell\0"), subrecord(b"DATA", &[1])].concat(),
    );
    let destination = record(
        b"CELL",
        0,
        destination_cell,
        &[
            subrecord(b"EDID", b"Destination\0"),
            subrecord(b"DATA", &[1]),
        ]
        .concat(),
    );
    let door = record(
        b"DOOR",
        0,
        door_base,
        &[
            subrecord(b"EDID", b"DoorBase\0"),
            subrecord(b"FULL", b"Door\0"),
            subrecord(b"MODL", b"door.nif\0"),
        ]
        .concat(),
    );
    let npc = record(
        b"NPC_",
        0,
        npc_base,
        &[
            subrecord(b"FULL", b"Dweller\0"),
            subrecord(b"TPLT", &9_u32.to_le_bytes()),
        ]
        .concat(),
    );
    let item = record(
        b"MISC",
        0,
        item_base,
        &[
            subrecord(b"FULL", b"Scrap\0"),
            subrecord(b"MODL", b"scrap.nif\0"),
            subrecord(
                b"DATA",
                &[
                    12_i32.to_le_bytes().as_slice(),
                    2.5_f32.to_le_bytes().as_slice(),
                ]
                .concat(),
            ),
        ]
        .concat(),
    );
    let creature = record(
        b"CREA",
        0,
        creature_base,
        &[
            subrecord(b"FULL", b"Mole Rat\0"),
            subrecord(b"MODL", b"molerat.nif\0"),
        ]
        .concat(),
    );
    let mut xtel = destination_ref.to_le_bytes().to_vec();
    xtel.extend(transform());
    let mut lock = vec![50, 0, 0, 0];
    lock.extend(item_base.to_le_bytes());
    lock.extend([0; 12]);
    let placed_door = record(
        b"REFR",
        0,
        door_ref,
        &[
            subrecord(b"NAME", &door_base.to_le_bytes()),
            subrecord(b"XTEL", &xtel),
            subrecord(b"XLOC", &lock),
            subrecord(b"UNKN", &[1, 2, 3]),
            subrecord(b"DATA", &transform()),
        ]
        .concat(),
    );
    let placed_item = record(
        b"REFR",
        0,
        0x302,
        &[
            subrecord(b"NAME", &item_base.to_le_bytes()),
            subrecord(b"XCNT", &3_i32.to_le_bytes()),
            subrecord(b"DATA", &transform()),
        ]
        .concat(),
    );
    let placed_npc = record(
        b"ACHR",
        0,
        0x303,
        &[
            subrecord(b"NAME", &npc_base.to_le_bytes()),
            subrecord(b"DATA", &transform()),
        ]
        .concat(),
    );
    let placed_creature = record(
        b"ACRE",
        0,
        0x304,
        &[
            subrecord(b"NAME", &creature_base.to_le_bytes()),
            subrecord(b"DATA", &transform()),
        ]
        .concat(),
    );
    let navmesh = record(
        b"NAVM",
        0,
        0x500,
        &[
            subrecord(b"NVER", &12_u32.to_le_bytes()),
            subrecord(b"NVVX", &[0; 12]),
        ]
        .concat(),
    );
    let destination_door = record(
        b"REFR",
        0,
        destination_ref,
        &[
            subrecord(b"NAME", &door_base.to_le_bytes()),
            subrecord(b"DATA", &transform()),
        ]
        .concat(),
    );
    let mut plugin = tes4(&[]);
    plugin.extend(cell);
    plugin.extend(destination);
    plugin.extend(door);
    plugin.extend(npc);
    plugin.extend(item);
    plugin.extend(creature);
    plugin.extend(group(
        cell_id,
        6,
        &[
            placed_door,
            placed_item,
            placed_npc,
            placed_creature,
            navmesh,
        ]
        .concat(),
    ));
    plugin.extend(group(destination_cell, 6, &destination_door));

    let parsed = parse_plugin(&plugin, cell_id).unwrap();
    assert_eq!(parsed.references.len(), 4);
    assert_eq!(parsed.navmeshes.len(), 1);
    assert_eq!(parsed.navmeshes[0].version, Some(12));
    let item_ref = parsed
        .references
        .iter()
        .find(|reference| reference.base_form_id == item_base)
        .unwrap();
    assert_eq!(item_ref.count, 3);
    assert_eq!(parsed.bases[&item_base].value, Some(12));
    assert_eq!(parsed.bases[&item_base].weight, Some(2.5));
    let door = parsed
        .references
        .iter()
        .find(|reference| reference.form_id == door_ref)
        .unwrap()
        .door
        .as_ref()
        .unwrap();
    assert_eq!(
        door.destination.as_ref().unwrap().cell_form_id,
        destination_cell
    );
    assert_eq!(door.lock_level, Some(50));
    assert_eq!(door.key_form_id, Some(item_base));
    assert_eq!(
        parsed
            .references
            .iter()
            .find(|reference| reference.kind == ReferenceKind::Npc)
            .unwrap()
            .base_form_id,
        npc_base
    );
    assert!(
        parsed
            .references
            .iter()
            .any(|reference| reference.kind == ReferenceKind::Creature)
    );
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.contains("REFR.UNKN"))
    );

    let parsed_by_editor_id = parse_content_set(
        &[PluginSource {
            name: "Fallout3.esm",
            bytes: &plugin,
        }],
        &CellSelector::EditorId("testcell".into()),
    )
    .unwrap();
    assert_eq!(
        parsed_by_editor_id.cell.as_ref().map(|cell| cell.form_id),
        Some(cell_id)
    );
    assert!(
        parse_content_set(
            &[PluginSource {
                name: "Fallout3.esm",
                bytes: &plugin,
            }],
            &CellSelector::EditorId("missing-cell".into()),
        )
        .is_err()
    );
}

#[test]
fn adjusts_master_form_ids_and_applies_later_overrides() {
    let cell_id = 0x100;
    let base_id = 0x400;
    let reference_id = 0x0100_0300;
    let mut master = tes4(&[]);
    master.extend(record(b"CELL", 0, cell_id, &subrecord(b"DATA", &[1])));
    master.extend(record(
        b"MISC",
        0,
        base_id,
        &[
            subrecord(b"FULL", b"Original\0"),
            subrecord(b"DATA", &[0; 8]),
        ]
        .concat(),
    ));

    let mut override_plugin = tes4(&["Master.esm"]);
    override_plugin.extend(record(
        b"MISC",
        0,
        base_id,
        &[
            subrecord(b"FULL", b"Overridden\0"),
            subrecord(b"DATA", &[0; 8]),
        ]
        .concat(),
    ));
    override_plugin.extend(group(
        cell_id,
        6,
        &record(
            b"REFR",
            0,
            reference_id,
            &[
                subrecord(b"NAME", &base_id.to_le_bytes()),
                subrecord(b"DATA", &transform()),
            ]
            .concat(),
        ),
    ));

    let parsed = parse_content_set(
        &[
            PluginSource {
                name: "Master.esm",
                bytes: &master,
            },
            PluginSource {
                name: "Patch.esp",
                bytes: &override_plugin,
            },
        ],
        &CellSelector::FormId(cell_id),
    )
    .unwrap();
    assert_eq!(parsed.references[0].form_id, reference_id);
    assert_eq!(parsed.references[0].base_form_id, base_id);
    assert_eq!(parsed.bases[&base_id].name.as_deref(), Some("Overridden"));
}

#[test]
fn editor_id_resolution_rejects_ambiguous_cells() {
    let mut plugin = tes4(&[]);
    for cell_id in [0x100_u32, 0x200_u32] {
        plugin.extend(record(
            b"CELL",
            0,
            cell_id,
            &[
                subrecord(b"EDID", b"DuplicateCell\0"),
                subrecord(b"DATA", &[1]),
            ]
            .concat(),
        ));
    }

    let error = parse_content_set(
        &[PluginSource {
            name: "Fallout3.esm",
            bytes: &plugin,
        }],
        &CellSelector::EditorId("duplicatecell".into()),
    )
    .unwrap_err();
    assert!(error.to_string().contains("ambiguous"));
    assert!(error.to_string().contains("00000100"));
    assert!(error.to_string().contains("00000200"));
}

#[test]
#[ignore = "requires BEVYOUT_FALLOUT3_ESM pointing to local Fallout3.esm"]
fn fallout3_cell_smoke_baselines() {
    let path =
        std::env::var("BEVYOUT_FALLOUT3_ESM").expect("set BEVYOUT_FALLOUT3_ESM to Fallout3.esm");
    let bytes = std::fs::read(path).unwrap();
    for (cell, refr, achr, acre, navm, teleports) in [
        (0x0001_51e3, 928, 0, 1, 1, 1),
        (0x0001_7f37, 1_756, 10, 1, 1, 2),
        (0x000c_c067, 483, 1, 0, 1, 1),
    ] {
        let parsed = parse_plugin(&bytes, cell).unwrap();
        assert_eq!(
            parsed
                .references
                .iter()
                .filter(|reference| reference.kind == ReferenceKind::Object)
                .count(),
            refr
        );
        assert_eq!(
            parsed
                .references
                .iter()
                .filter(|reference| reference.kind == ReferenceKind::Npc)
                .count(),
            achr
        );
        assert_eq!(
            parsed
                .references
                .iter()
                .filter(|reference| reference.kind == ReferenceKind::Creature)
                .count(),
            acre
        );
        assert_eq!(parsed.navmeshes.len(), navm);
        assert_eq!(
            parsed
                .references
                .iter()
                .filter(|reference| reference
                    .door
                    .as_ref()
                    .and_then(|door| door.destination.as_ref())
                    .is_some())
                .count(),
            teleports
        );
    }
}
