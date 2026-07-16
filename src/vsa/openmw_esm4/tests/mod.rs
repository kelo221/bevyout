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

fn direct_subrecord(signature: &str, data: Vec<u8>) -> Subrecord {
    Subrecord {
        signature: signature.into(),
        data,
    }
}

fn direct_resolver() -> FormIdResolver {
    FormIdResolver {
        current_index: 0,
        master_indices: Vec::new(),
    }
}

#[test]
fn parses_openmw_inventory_layouts_and_icon_fallback_fields() {
    let resolver = direct_resolver();
    let mut weapon_data = Vec::new();
    weapon_data.extend_from_slice(&125_u32.to_le_bytes());
    weapon_data.extend_from_slice(&250_u32.to_le_bytes());
    weapon_data.extend_from_slice(&6.5_f32.to_le_bytes());
    weapon_data.extend_from_slice(&34_u16.to_le_bytes());
    weapon_data.push(8);
    let weapon = parse_base(
        "WEAP",
        &[
            direct_subrecord("ICON", b"interface/icons/rifle.dds\0".to_vec()),
            direct_subrecord("MICO", b"interface/icons/rifle_small.dds\0".to_vec()),
            direct_subrecord("DATA", weapon_data),
            // Issue #98 (F98.1): NAM0 is the FO3 "Ammo" FormID field.
            direct_subrecord("NAM0", 0x0001_2ab3_u32.to_le_bytes().to_vec()),
        ],
        &resolver,
    )
    .unwrap();
    assert_eq!(weapon.value, Some(125));
    assert_eq!(weapon.weight, Some(6.5));
    assert_eq!(weapon.icon.as_deref(), Some("interface/icons/rifle.dds"));
    assert_eq!(
        weapon.item_stats,
        OpenMwItemStats::Weapon {
            damage: Some(34),
            max_condition: Some(250),
            clip_size: Some(8),
            speed: None,
            reach: None,
            ammo_form_id: Some(0x0001_2ab3),
        }
    );
    assert!(
        !weapon.ignored_subrecords.contains(&"NAM0".to_string()),
        "NAM0 is now decoded, not merely ignored"
    );

    let mut armor_data = Vec::new();
    armor_data.extend_from_slice(&80_u32.to_le_bytes());
    armor_data.extend_from_slice(&400_u32.to_le_bytes());
    armor_data.extend_from_slice(&12.0_f32.to_le_bytes());
    // Issue #98 (F98.1): FO3's 8-byte BMDT is armor-flags (biped-slot mask)
    // + general-flags (masked to its low byte by the reader, per
    // `ESM4::Armor::load`'s BMDT case).
    let mut armor_bmdt = 0x0000_0025_u32.to_le_bytes().to_vec(); // Head|UpperBody|Weapon
    armor_bmdt.extend_from_slice(&0_u32.to_le_bytes());
    let armor = parse_base(
        "ARMO",
        &[
            direct_subrecord("MICO", b"interface/icons/armor_small.dds\0".to_vec()),
            direct_subrecord("DATA", armor_data),
            direct_subrecord("DNAM", 18_u16.to_le_bytes().to_vec()),
            direct_subrecord("BMDT", armor_bmdt),
        ],
        &resolver,
    )
    .unwrap();
    assert_eq!(armor.icon, None);
    assert_eq!(
        armor.mini_icon.as_deref(),
        Some("interface/icons/armor_small.dds")
    );
    assert_eq!(
        armor.item_stats,
        OpenMwItemStats::Apparel {
            armor_rating: Some(18.0),
            max_condition: Some(400),
            biped_slot_mask: Some(0x0000_0025),
        }
    );
    assert!(
        !armor.ignored_subrecords.contains(&"BMDT".to_string()),
        "BMDT is now decoded, not merely ignored"
    );

    let mut ammo_data = Vec::new();
    ammo_data.extend_from_slice(&3200.0_f32.to_le_bytes());
    ammo_data.extend_from_slice(&0_u32.to_le_bytes());
    ammo_data.extend_from_slice(&2_u32.to_le_bytes());
    ammo_data.push(12);
    let mut ammo_dnam = vec![0; 8];
    ammo_dnam.extend_from_slice(&9.5_f32.to_le_bytes());
    ammo_dnam.extend_from_slice(&100_u32.to_le_bytes());
    let ammo = parse_base(
        "AMMO",
        &[
            direct_subrecord("DATA", ammo_data),
            direct_subrecord("DNAM", ammo_dnam),
        ],
        &resolver,
    )
    .unwrap();
    assert_eq!(ammo.value, Some(2));
    assert_eq!(
        ammo.item_stats,
        OpenMwItemStats::Ammo {
            damage: Some(9.5),
            speed: Some(3200.0),
        }
    );

    let mut aid_enit = Vec::new();
    aid_enit.extend_from_slice(&35_i32.to_le_bytes());
    aid_enit.resize(20, 0);
    let aid = parse_base(
        "ALCH",
        &[
            direct_subrecord("DATA", 0.5_f32.to_le_bytes().to_vec()),
            direct_subrecord("ENIT", aid_enit),
            direct_subrecord("EFID", 0x0001_2345_u32.to_le_bytes().to_vec()),
        ],
        &resolver,
    )
    .unwrap();
    assert_eq!(aid.value, Some(35));
    assert_eq!(aid.weight, Some(0.5));
    assert_eq!(
        aid.item_stats,
        OpenMwItemStats::Aid {
            effect_form_ids: vec![0x0001_2345],
        }
    );

    let mut book_data = vec![3, 0];
    book_data.extend_from_slice(&15_i32.to_le_bytes());
    book_data.extend_from_slice(&1.25_f32.to_le_bytes());
    let book = parse_base(
        "BOOK",
        &[
            direct_subrecord("DATA", book_data),
            direct_subrecord("DESC", b"Synthetic book text\0".to_vec()),
        ],
        &resolver,
    )
    .unwrap();
    assert_eq!(book.value, Some(15));
    assert_eq!(book.weight, Some(1.25));
    assert_eq!(
        book.item_stats,
        OpenMwItemStats::Book {
            flags: Some(3),
            text: Some("Synthetic book text".into()),
        }
    );

    for kind in ["MISC", "KEYM"] {
        let mut data = Vec::new();
        data.extend_from_slice(&7_i32.to_le_bytes());
        data.extend_from_slice(&0.1_f32.to_le_bytes());
        let item = parse_base(kind, &[direct_subrecord("DATA", data)], &resolver).unwrap();
        assert_eq!(item.value, Some(7));
        assert_eq!(item.weight, Some(0.1));
    }
    let note = parse_base("NOTE", &[], &resolver).unwrap();
    assert_eq!(note.item_stats, OpenMwItemStats::Note { text: None });
}

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

// T38.2 support: multi-hop enable-parent chains resolve to one stable root,
// and cycles/unresolved parents are reported rather than silently accepted.
#[test]
fn resolve_enable_root_walks_multi_hop_chains_to_a_stable_ancestor() {
    fn reference(form_id: u32, parent: Option<u32>) -> ReferenceRecord {
        ReferenceRecord {
            form_id,
            enable_parent: parent.map(|parent_reference_form_id| EnableParentRecord {
                parent_reference_form_id,
                flags: 0,
            }),
            ..Default::default()
        }
    }

    let mut references = HashMap::new();
    references.insert(0x1, reference(0x1, None)); // root: no enable_parent
    references.insert(0x2, reference(0x2, Some(0x1))); // mid: enabled by root
    references.insert(0x3, reference(0x3, Some(0x2))); // leaf: enabled by mid

    assert_eq!(resolve_enable_root(0x2, &references).unwrap(), 0x1);
    assert_eq!(resolve_enable_root(0x3, &references).unwrap(), 0x1);
    // A reference with no enable_parent is trivially its own root.
    assert_eq!(resolve_enable_root(0x1, &references).unwrap(), 0x1);
}

#[test]
fn resolve_enable_root_reports_cycles_and_unresolved_parents() {
    fn reference(form_id: u32, parent: Option<u32>) -> ReferenceRecord {
        ReferenceRecord {
            form_id,
            enable_parent: parent.map(|parent_reference_form_id| EnableParentRecord {
                parent_reference_form_id,
                flags: 0,
            }),
            ..Default::default()
        }
    }

    let mut cyclic = HashMap::new();
    cyclic.insert(0x1, reference(0x1, Some(0x2)));
    cyclic.insert(0x2, reference(0x2, Some(0x1)));
    assert!(matches!(
        resolve_enable_root(0x1, &cyclic),
        Err(EnableResolutionError::Cycle(_))
    ));

    let mut dangling = HashMap::new();
    dangling.insert(0x1, reference(0x1, Some(0x999)));
    assert!(matches!(
        resolve_enable_root(0x1, &dangling),
        Err(EnableResolutionError::Unresolved(0x999))
    ));
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

// T45.1: XCLC bytes -> grid coords. OpenMW `components/esm4/loadcell.cpp`
// (`case ESM4::SUB_XCLC`): two little-endian i32s, optionally followed by a
// u32 "force hide land quad" flags word this catalogue does not need.
#[test]
fn xclc_parses_grid_coordinates_and_tolerates_short_or_legacy_payloads() {
    let mut data = (-2_i32).to_le_bytes().to_vec();
    data.extend_from_slice(&5_i32.to_le_bytes());
    assert_eq!(parse_grid(&data), Some((-2, 5)));

    // Trailing "force hide land quad" flags word is tolerated (ignored).
    let mut with_flags = data.clone();
    with_flags.extend_from_slice(&0x0f_u32.to_le_bytes());
    assert_eq!(parse_grid(&with_flags), Some((-2, 5)));

    // Short/legacy payloads are skipped gracefully rather than erroring.
    assert_eq!(parse_grid(&data[..4]), None);
    assert_eq!(parse_grid(&[]), None);

    let cell = parse_cell(
        &[Subrecord {
            signature: "XCLC".into(),
            data,
        }],
        0x200,
        &FormIdResolver {
            current_index: 0,
            master_indices: Vec::new(),
        },
    )
    .unwrap();
    assert_eq!(cell.grid, Some((-2, 5)));
    assert_eq!(cell.worldspace_form_id, None);
}

// T45.2: a cell discovered under a GRUP type-1 ("World Children") group
// gets that WRLD's FormID as its worldspace, propagated through the nested
// type-4/5 exterior block/sub-block groups a real plugin uses; a cell
// outside any such group (interiors) gets `None`.
#[test]
fn cells_under_world_children_group_get_that_worldspace_interiors_get_none() {
    let interior_cell_id = 0x100;
    let exterior_cell_id = 0x200;
    let worldspace_id = 0x10;

    let mut plugin = tes4(&[]);
    plugin.extend(record(
        b"CELL",
        0,
        interior_cell_id,
        &[
            subrecord(b"EDID", b"VaultInterior\0"),
            subrecord(b"DATA", &[1]),
        ]
        .concat(),
    ));
    plugin.extend(record(
        b"WRLD",
        0,
        worldspace_id,
        &[
            subrecord(b"EDID", b"Wasteland\0"),
            subrecord(b"FULL", b"The Wasteland\0"),
        ]
        .concat(),
    ));
    let mut xclc = (-2_i32).to_le_bytes().to_vec();
    xclc.extend_from_slice(&5_i32.to_le_bytes());
    let exterior_cell = record(
        b"CELL",
        0,
        exterior_cell_id,
        &[
            subrecord(b"EDID", b"Wasteland01\0"),
            subrecord(b"DATA", &[0]),
            subrecord(b"XCLC", &xclc),
        ]
        .concat(),
    );
    let sub_block = group(0, 5, &exterior_cell);
    let block = group(0, 4, &sub_block);
    plugin.extend(group(worldspace_id, 1, &block));

    let parsed = parse_content_set_all(&[PluginSource {
        name: "Fallout3.esm",
        bytes: &plugin,
    }])
    .unwrap();
    let cells: HashMap<_, _> = parsed.cells().map(|(id, cell)| (*id, cell)).collect();
    assert_eq!(cells[&interior_cell_id].worldspace_form_id, None);
    assert_eq!(cells[&interior_cell_id].grid, None);
    assert_eq!(
        cells[&exterior_cell_id].worldspace_form_id,
        Some(worldspace_id)
    );
    assert_eq!(cells[&exterior_cell_id].grid, Some((-2, 5)));
    assert_eq!(
        parsed.worldspaces().map(|(id, _)| *id).collect::<Vec<_>>(),
        vec![worldspace_id]
    );
}

// T45.3: two cells joined by opposing XTEL doors produce two directed
// content-set-wide edges with the correct destination cell/door; a dangling
// XTEL (destination reference not present) is counted as unresolved rather
// than failing the pass.
#[test]
fn door_edges_cover_the_whole_content_set_and_count_unresolved_teleports() {
    let cell_a = 0x100_u32;
    let cell_b = 0x200_u32;
    let door_a = 0x101_u32; // in cell_a, teleports to door_b in cell_b
    let door_b = 0x201_u32; // in cell_b, teleports back to door_a in cell_a
    let dangling_door = 0x102_u32; // in cell_a, teleports to a reference that does not exist
    let door_base = 0x400_u32;

    let mut plugin = tes4(&[]);
    plugin.extend(record(
        b"CELL",
        0,
        cell_a,
        &[subrecord(b"EDID", b"CellA\0"), subrecord(b"DATA", &[1])].concat(),
    ));
    plugin.extend(record(
        b"CELL",
        0,
        cell_b,
        &[subrecord(b"EDID", b"CellB\0"), subrecord(b"DATA", &[1])].concat(),
    ));
    plugin.extend(record(
        b"DOOR",
        0,
        door_base,
        &subrecord(b"EDID", b"DoorBase\0"),
    ));

    let mut xtel_to_b = door_b.to_le_bytes().to_vec();
    xtel_to_b.extend(transform());
    let refr_a = record(
        b"REFR",
        0,
        door_a,
        &[
            subrecord(b"NAME", &door_base.to_le_bytes()),
            subrecord(b"XTEL", &xtel_to_b),
            subrecord(b"DATA", &transform()),
        ]
        .concat(),
    );

    let mut xtel_to_a = door_a.to_le_bytes().to_vec();
    xtel_to_a.extend(transform());
    let refr_b = record(
        b"REFR",
        0,
        door_b,
        &[
            subrecord(b"NAME", &door_base.to_le_bytes()),
            subrecord(b"XTEL", &xtel_to_a),
            subrecord(b"DATA", &transform()),
        ]
        .concat(),
    );

    let mut xtel_dangling = 0x999_u32.to_le_bytes().to_vec();
    xtel_dangling.extend(transform());
    let refr_dangling = record(
        b"REFR",
        0,
        dangling_door,
        &[
            subrecord(b"NAME", &door_base.to_le_bytes()),
            subrecord(b"XTEL", &xtel_dangling),
            subrecord(b"DATA", &transform()),
        ]
        .concat(),
    );

    plugin.extend(group(cell_a, 6, &[refr_a, refr_dangling].concat()));
    plugin.extend(group(cell_b, 6, &refr_b));

    let parsed = parse_content_set_all(&[PluginSource {
        name: "Fallout3.esm",
        bytes: &plugin,
    }])
    .unwrap();
    let (edges, unresolved) = parsed.door_edges();
    assert_eq!(unresolved, 1);
    assert_eq!(edges.len(), 2);
    let a_to_b = edges
        .iter()
        .find(|edge| edge.source_cell_form_id == cell_a && edge.door_reference_form_id == door_a)
        .unwrap();
    assert_eq!(a_to_b.destination_cell_form_id, cell_b);
    assert_eq!(a_to_b.destination_door_reference_form_id, door_b);
    let b_to_a = edges
        .iter()
        .find(|edge| edge.source_cell_form_id == cell_b && edge.door_reference_form_id == door_b)
        .unwrap();
    assert_eq!(b_to_a.destination_cell_form_id, cell_a);
    assert_eq!(b_to_a.destination_door_reference_form_id, door_a);
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

// T74.1: `LVLI`/`LVLN`/`LVLC` leveled-list parsing (issue #74). OpenMW
// `components/esm4/loadlvli.cpp` (`ESM4::LevelledItem::load`),
// `loadlvlc.cpp` (`ESM4::LevelledCreature::load`), and `loadlvln.cpp`
// (`ESM4::LevelledNpc::load`): `LVLD` chance-none, `LVLF` flags, and one or
// more `LVLO` entries (12-byte layout, or the 8-byte legacy layout dropping
// both `unknown`/`unknown2` fields -- see `components/esm4/inventory.hpp`).

fn lvlo12(level: i16, item: u32, count: i16) -> Vec<u8> {
    let mut data = level.to_le_bytes().to_vec();
    data.extend_from_slice(&0_u16.to_le_bytes());
    data.extend_from_slice(&item.to_le_bytes());
    data.extend_from_slice(&count.to_le_bytes());
    data.extend_from_slice(&0_u16.to_le_bytes());
    data
}

fn lvlo8(level: i16, item: u32, count: i16) -> Vec<u8> {
    let mut data = level.to_le_bytes().to_vec();
    data.extend_from_slice(&item.to_le_bytes());
    data.extend_from_slice(&count.to_le_bytes());
    data
}

fn no_masters_resolver() -> FormIdResolver {
    FormIdResolver {
        current_index: 0,
        master_indices: Vec::new(),
    }
}

#[test]
fn parses_flat_leveled_item_list_with_chance_none_and_flags() {
    let resolver = no_masters_resolver();
    let base = parse_base(
        "LVLI",
        &[
            Subrecord {
                signature: "EDID".into(),
                data: b"LootList\0".to_vec(),
            },
            Subrecord {
                signature: "LVLD".into(),
                data: vec![25],
            },
            Subrecord {
                signature: "LVLF".into(),
                data: vec![0x02], // LVLF: "calculate for each item" bit; interpreted by the resolver, not this parser.
            },
            Subrecord {
                signature: "LVLO".into(),
                data: lvlo12(1, 0x100, 1),
            },
            Subrecord {
                signature: "LVLO".into(),
                data: lvlo12(5, 0x101, 2),
            },
        ],
        &resolver,
    )
    .unwrap();
    let leveled = base.leveled.as_ref().unwrap();
    assert_eq!(leveled.chance_none, 25);
    assert_eq!(leveled.flags, 0x02);
    assert_eq!(
        leveled.entries,
        vec![
            LeveledListEntry {
                level: 1,
                item_form_id: 0x100,
                count: 1
            },
            LeveledListEntry {
                level: 5,
                item_form_id: 0x101,
                count: 2
            },
        ]
    );
    assert!(base.ignored_subrecords.is_empty());
}

#[test]
fn parses_legacy_eight_byte_lvlo_and_leveled_creature_npc_kinds() {
    let resolver = no_masters_resolver();
    let creature = parse_base(
        "LVLC",
        &[Subrecord {
            signature: "LVLO".into(),
            data: lvlo8(3, 0x200, 1),
        }],
        &resolver,
    )
    .unwrap();
    assert_eq!(
        creature.leveled.unwrap().entries,
        vec![LeveledListEntry {
            level: 3,
            item_form_id: 0x200,
            count: 1
        }]
    );

    let npc = parse_base(
        "LVLN",
        &[Subrecord {
            signature: "LVLO".into(),
            data: lvlo8(2, 0x201, 1),
        }],
        &resolver,
    )
    .unwrap();
    assert_eq!(
        npc.leveled.unwrap().entries,
        vec![LeveledListEntry {
            level: 2,
            item_form_id: 0x201,
            count: 1
        }]
    );
}

#[test]
fn nested_leveled_list_entry_points_at_another_leveled_list_form_id() {
    let resolver = no_masters_resolver();
    let outer = parse_base(
        "LVLI",
        &[
            Subrecord {
                signature: "LVLD".into(),
                data: vec![0],
            },
            Subrecord {
                signature: "LVLO".into(),
                data: lvlo12(1, 0x300, 1),
            },
        ],
        &resolver,
    )
    .unwrap();
    // The parser does not know or care that 0x300 is itself a leveled list;
    // that transitive expansion happens in `prepare::placements` and the
    // resolver only sees FormIDs here.
    assert_eq!(outer.leveled.unwrap().entries[0].item_form_id, 0x300);
}

#[test]
fn chance_none_one_hundred_is_preserved_verbatim() {
    let resolver = no_masters_resolver();
    let base = parse_base(
        "LVLI",
        &[Subrecord {
            signature: "LVLD".into(),
            data: vec![100],
        }],
        &resolver,
    )
    .unwrap();
    assert_eq!(base.leveled.unwrap().chance_none, 100);
}

#[test]
fn mutually_referencing_leveled_lists_parse_independently() {
    // Parser-level cycle tolerance: list A's LVLO points at list B's FormID
    // and vice versa. Parsing either record is independent of the other's
    // contents, so both parse cleanly here; cycle *resolution* safety
    // (warn-and-skip instead of looping) is the resolver's job, exercised in
    // `viewer::interaction::leveled`'s own tests.
    let resolver = no_masters_resolver();
    let list_a = parse_base(
        "LVLI",
        &[Subrecord {
            signature: "LVLO".into(),
            data: lvlo12(1, 0x9002, 1),
        }],
        &resolver,
    )
    .unwrap();
    let list_b = parse_base(
        "LVLI",
        &[Subrecord {
            signature: "LVLO".into(),
            data: lvlo12(1, 0x9001, 1),
        }],
        &resolver,
    )
    .unwrap();
    assert_eq!(
        list_a.leveled.as_ref().unwrap().entries[0].item_form_id,
        0x9002
    );
    assert_eq!(
        list_b.leveled.as_ref().unwrap().entries[0].item_form_id,
        0x9001
    );
}

#[test]
fn non_leveled_records_have_no_leveled_body() {
    let resolver = no_masters_resolver();
    let base = parse_base(
        "MISC",
        &[Subrecord {
            signature: "FULL".into(),
            data: b"Scrap\0".to_vec(),
        }],
        &resolver,
    )
    .unwrap();
    assert!(base.leveled.is_none());
}

#[test]
fn full_plugin_parse_stores_leveled_list_base_by_form_id() {
    let list_id = 0x100;
    let list_record = record(
        b"LVLI",
        0,
        list_id,
        &[
            subrecord(b"EDID", b"LootList\0"),
            subrecord(b"LVLD", &[10]),
            subrecord(b"LVLO", &lvlo12(1, 0x200, 1)),
        ]
        .concat(),
    );
    let cell = record(
        b"CELL",
        0,
        0x1,
        &[subrecord(b"EDID", b"TestCell\0"), subrecord(b"DATA", &[1])].concat(),
    );
    let mut plugin = tes4(&[]);
    plugin.extend(cell);
    plugin.extend(list_record);

    let parsed = parse_plugin(&plugin, 0x1).unwrap();
    let list = parsed.bases.get(&list_id).unwrap();
    assert_eq!(list.kind, "LVLI");
    let leveled = list.leveled.as_ref().unwrap();
    assert_eq!(leveled.chance_none, 10);
    assert_eq!(
        leveled.entries,
        vec![LeveledListEntry {
            level: 1,
            item_form_id: 0x200,
            count: 1
        }]
    );
}

// Issue #117: RCPE fixtures stay synthetic and exercise the raw ESM4 reader
// before recipe preparation is layered on top of it.
fn recipe_data(skill: i32, level: u32, category: u32, sub_category: u32) -> Vec<u8> {
    [
        skill.to_le_bytes().as_slice(),
        level.to_le_bytes().as_slice(),
        category.to_le_bytes().as_slice(),
        sub_category.to_le_bytes().as_slice(),
    ]
    .concat()
}

fn recipe_item(subrecord_signature: &[u8; 4], form_id: u32) -> Vec<u8> {
    subrecord(subrecord_signature, &form_id.to_le_bytes())
}

fn recipe_quantity(quantity: u32) -> Vec<u8> {
    subrecord(b"RCQY", &quantity.to_le_bytes())
}

#[test]
fn parses_valid_rcpe_fields_and_ordered_ingredient_output_pairs() {
    let recipe_id = 0x700_u32;
    let cell_id = 0x701_u32;
    let mut recipe_payload = Vec::new();
    recipe_payload.extend(subrecord(b"EDID", b"RecipeSynthetic\0"));
    recipe_payload.extend(subrecord(b"FULL", b"Synthetic Recipe\0"));
    recipe_payload.extend(subrecord(b"DATA", &recipe_data(-3, 12, 0x400, 0x401)));
    recipe_payload.extend(recipe_item(b"RCIL", 0x100));
    recipe_payload.extend(recipe_quantity(2));
    recipe_payload.extend(recipe_item(b"RCIL", 0x101));
    recipe_payload.extend(recipe_quantity(1));
    recipe_payload.extend(recipe_item(b"RCOD", 0x200));
    recipe_payload.extend(recipe_quantity(3));

    let mut plugin = tes4(&[]);
    plugin.extend(record(
        b"CELL",
        0,
        cell_id,
        &[
            subrecord(b"EDID", b"RecipeCell\0"),
            subrecord(b"DATA", &[1]),
        ]
        .concat(),
    ));
    plugin.extend(record(b"RCPE", 0, recipe_id, &recipe_payload));

    let parsed = parse_plugin(&plugin, cell_id).expect("valid synthetic RCPE must parse");
    let recipe = parsed
        .recipes
        .get(&recipe_id)
        .expect("recipe record must be retained");
    assert_eq!(recipe.editor_id.as_deref(), Some("RecipeSynthetic"));
    assert_eq!(recipe.name.as_deref(), Some("Synthetic Recipe"));
    assert_eq!(recipe.skill, -3);
    assert_eq!(recipe.level, 12);
    assert_eq!(recipe.category_form_id, Some(0x400));
    assert_eq!(recipe.sub_category_form_id, Some(0x401));
    assert_eq!(
        recipe.ingredients,
        vec![
            RecipeItemRecord {
                item_form_id: 0x100,
                quantity: 2,
                order: 0,
            },
            RecipeItemRecord {
                item_form_id: 0x101,
                quantity: 1,
                order: 1,
            },
        ]
    );
    assert_eq!(
        recipe.outputs,
        vec![RecipeItemRecord {
            item_form_id: 0x200,
            quantity: 3,
            order: 0,
        }]
    );
}

#[test]
fn malformed_rcpe_is_reported_and_does_not_leave_partial_recipe_state() {
    let recipe_id = 0x710_u32;
    let cell_id = 0x711_u32;
    let malformed = [
        subrecord(b"EDID", b"MalformedRecipe\0"),
        subrecord(b"DATA", &recipe_data(1, 2, 0, 0)[..8]),
        recipe_item(b"RCIL", 0x100),
    ]
    .concat();
    let mut plugin = tes4(&[]);
    plugin.extend(record(
        b"CELL",
        0,
        cell_id,
        &[
            subrecord(b"EDID", b"RecipeCell\0"),
            subrecord(b"DATA", &[1]),
        ]
        .concat(),
    ));
    plugin.extend(record(b"RCPE", 0, recipe_id, &malformed));

    let parsed = parse_plugin(&plugin, cell_id).expect("malformed RCPE boundary is diagnosable");
    assert!(!parsed.recipes.contains_key(&recipe_id));
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|message| message.contains("RCPE 00000710") && message.contains("DATA"))
    );
}

#[test]
fn duplicate_rcpe_data_is_reported_without_partial_recipe_state() {
    let recipe_id = 0x715_u32;
    let cell_id = 0x716_u32;
    let payload = [
        subrecord(b"DATA", &recipe_data(1, 2, 0, 0)),
        subrecord(b"DATA", &recipe_data(3, 4, 0, 0)),
        recipe_item(b"RCIL", 0x100),
        recipe_quantity(1),
        recipe_item(b"RCOD", 0x200),
        recipe_quantity(1),
    ]
    .concat();
    let mut plugin = tes4(&[]);
    plugin.extend(record(
        b"CELL",
        0,
        cell_id,
        &[subrecord(b"EDID", b"RecipeCell "), subrecord(b"DATA", &[1])].concat(),
    ));
    plugin.extend(record(b"RCPE", 0, recipe_id, &payload));

    let parsed = parse_plugin(&plugin, cell_id).expect("duplicate RCPE fields are diagnosable");
    assert!(!parsed.recipes.contains_key(&recipe_id));
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|message| message.contains("RCPE 00000715") && message.contains("duplicate DATA"))
    );
}

#[test]
fn unknown_rcpe_subrecords_are_retained_as_diagnostics() {
    let recipe_id = 0x720_u32;
    let cell_id = 0x721_u32;
    let payload = [
        subrecord(b"EDID", b"RecipeWithUnknown\0"),
        subrecord(b"DATA", &recipe_data(1, 2, 0, 0)),
        recipe_item(b"RCIL", 0x100),
        recipe_quantity(1),
        recipe_item(b"RCOD", 0x200),
        recipe_quantity(1),
        subrecord(b"ZZZZ", &[1, 2, 3]),
    ]
    .concat();
    let mut plugin = tes4(&[]);
    plugin.extend(record(
        b"CELL",
        0,
        cell_id,
        &[
            subrecord(b"EDID", b"RecipeCell\0"),
            subrecord(b"DATA", &[1]),
        ]
        .concat(),
    ));
    plugin.extend(record(b"RCPE", 0, recipe_id, &payload));

    let parsed = parse_plugin(&plugin, cell_id).expect("unknown RCPE fields are diagnosable");
    assert!(parsed.recipes.contains_key(&recipe_id));
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|message| message.contains("RCPE 00000720") && message.contains("ZZZZ"))
    );
}

mod actor_support;

mod actors;
