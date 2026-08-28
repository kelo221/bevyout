use super::*;
use crate::item_transaction::ItemExtraEntry;
use std::time::{SystemTime, UNIX_EPOCH};

fn sample_save() -> SaveGame {
    let mut references = BTreeMap::new();
    references.insert(
        0x0100_0020,
        PersistentReferenceDelta {
            enabled: Some(false),
            deleted: false,
            activated: Some(true),
            lock_level: Some(50),
            enable_root_form_id: Some(0x0100_0010),
            transform: Some(SavedTransform {
                translation: [1.0, 2.0, 3.0],
                rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                scale: [1.0, 1.0, 1.0],
            }),
            inventory: Some(vec![ItemStack {
                base_form_id: 0x0000_0042,
                count: 3,
                condition: None,
            }]),
            leveled_resolved: Some(true),
            body: Some(SavedBodyState {
                linear_velocity: [0.1, 0.2, 0.3],
                angular_velocity: [0.4, 0.5, 0.6],
                sleeping: true,
            }),
        },
    );
    let mut cells = BTreeMap::new();
    let mut dropped_items = BTreeMap::new();
    dropped_items.insert(
        5,
        DroppedItemState {
            runtime_id: 5,
            stack: ItemStack {
                base_form_id: 0x0000_0011,
                count: 4,
                condition: Some(80),
            },
            transform: SavedTransform {
                translation: [4.0, 5.0, 6.0],
                rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                scale: [1.0; 3],
            },
            body: SavedBodyState {
                linear_velocity: [0.1, 0.0, 0.0],
                angular_velocity: [0.0, 0.2, 0.0],
                sleeping: false,
            },
        },
    );
    cells.insert(
        0x0001_51e3,
        PersistentCellState {
            references,
            dropped_items,
            actors: BTreeMap::from([(
                0x0004_1600,
                ActorInstanceState {
                    reference_form_id: 0x0004_1600,
                    life_state: ActorLifeState::Alive,
                    value_mutations: BTreeMap::from([(
                        bevyout_core::actor_state::ActorValue::Health,
                        -12.0,
                    )]),
                    package: Some(ActorPackageCheckpoint {
                        package_form_id: 0x0002_c6f1,
                        procedure_index: 3,
                        elapsed_seconds: 4.5,
                    }),
                    limbs: bevyout_core::combat::LimbState::healthy(),
                    awareness: Default::default(),
                },
            )]),
        },
    );
    SaveGame {
        header: SaveGameHeader {
            format_version: CURRENT_SAVE_FORMAT_VERSION,
            content_fingerprint: "content-hash".into(),
            plugins: vec![SavePlugin {
                name: "Fallout3.esm".into(),
                fingerprint: "plugin-hash".into(),
            }],
            current_cell: 0x0001_51e3,
            play_time_seconds: 12.5,
            description: "test save".into(),
        },
        world: PersistentWorldState { cells },
        player: Some(PlayerState {
            inventory: vec![
                ItemStack {
                    base_form_id: 0x0000_0011,
                    count: 1,
                    condition: Some(100),
                },
                ItemStack {
                    base_form_id: 0x0000_0042,
                    count: 3,
                    condition: None,
                },
            ],
            equipped: vec![
                EquippedItem {
                    kind: EquippedKind::Apparel,
                    base_form_id: 0x0000_0011,
                    condition: Some(100),
                },
                EquippedItem {
                    kind: EquippedKind::Weapon,
                    base_form_id: 0x0000_0042,
                    condition: None,
                },
                EquippedItem {
                    kind: EquippedKind::Ammo,
                    base_form_id: 0x0000_0055,
                    condition: None,
                },
            ],
            hotkeys: [
                Some(HotkeyBinding {
                    base_form_id: 0x0000_0011,
                    condition: Some(100),
                }),
                None,
                Some(HotkeyBinding {
                    base_form_id: 0x0000_0042,
                    condition: None,
                }),
                None,
                None,
                None,
                None,
                None,
            ],
        }),
        next_runtime_item_id: 6,
        rng_state: 0x0123_4567_89ab_cdef,
        combat_rng: bevyout_core::combat::CombatRngState::from_seed(0x0123_4567_89ab_cdef),
        canonical: None,
        dialogue: Default::default(),
        location: None,
        rpg: RpgSaveState::default(),
    }
}

#[test]
fn round_trip_is_deterministic() {
    let save = sample_save();
    let first = encode_save(&save).unwrap();
    let second = encode_save(&save).unwrap();
    assert_eq!(first, second);
    assert_eq!(decode_save(&first).unwrap(), save);
}

#[test]
fn v8_round_trip_preserves_exact_world_location() {
    let mut save = sample_save();
    save.location = Some(bevyout_core::manifest::exterior::WorldLocation::Exterior(
        bevyout_core::manifest::exterior::WorldLocationExterior {
            worldspace_form_id: 0x0001_51e3,
            position: [12.0, 3.5, -8.0],
            rotation_xyzw: [0.0, 0.707, 0.0, 0.707],
        },
    ));
    let bytes = encode_save(&save).unwrap();
    assert_eq!(decode_save(&bytes).unwrap(), save);
}

#[test]
fn v9_round_trip_preserves_player_limbs_and_rpg() {
    let mut save = sample_save();
    save.rpg
        .limbs
        .part_mut(bevyout_core::combat::BodyPartId::Head)
        .current_milli = 0;
    save.rpg
        .limbs
        .part_mut(bevyout_core::combat::BodyPartId::Head)
        .crippled = true;
    save.rpg.current_health = Some(70.0);
    let bytes = encode_save(&save).unwrap();
    let decoded = decode_save(&bytes).unwrap();
    assert_eq!(decoded, save);
    assert!(
        decoded
            .rpg
            .limbs
            .part(bevyout_core::combat::BodyPartId::Head)
            .crippled
    );
}

#[test]
fn v9_round_trip_preserves_optional_time_and_lifecycle() {
    let mut save = sample_save();
    save.rpg.clock.absolute_game_ms = 3_600_000;
    save.rpg.clock.timescale = 30;
    save.rpg.lifecycle.revision = bevyout_core::lifecycle::LIFECYCLE_SNAPSHOT_REVISION;
    save.rpg.lifecycle.clock = save.rpg.clock;
    save.rpg.lifecycle.encounter_zones.insert(
        0x0002_a4a0,
        bevyout_core::lifecycle::EncounterZoneState {
            zone_form_id: 0x0002_a4a0,
            first_entered_game_ms: 0,
            locked_level: 6,
            min_level: 2,
            max_level: 10,
        },
    );
    let bytes = encode_save(&save).unwrap();
    let decoded = decode_save(&bytes).unwrap();
    assert_eq!(decoded.rpg.clock.absolute_game_ms, 3_600_000);
    assert_eq!(
        decoded.rpg.lifecycle.encounter_zones[&0x0002_a4a0].locked_level,
        6
    );
    assert_eq!(decoded.rpg, save.rpg);
}

#[test]
fn v8_decode_defaults_missing_rpg_and_actor_limbs() {
    let mut save = sample_save();
    save.header.format_version = 8;
    save.rpg = RpgSaveState::default();
    let bytes = encode_save(&save).unwrap();
    let decoded = decode_save(&bytes).unwrap();
    assert_eq!(decoded.rpg, RpgSaveState::default());
    let actor = decoded
        .world
        .cells
        .get(&0x0001_51e3)
        .and_then(|cell| cell.actors.get(&0x0004_1600))
        .expect("sample actor");
    assert_eq!(actor.limbs, bevyout_core::combat::LimbState::healthy());
}

#[test]
fn decode_rpg_skips_unknown_subrecords() {
    let mut payload = encode_rpg(&RpgSaveState::default()).unwrap();
    write_subrecord(&mut payload, tag("UNKN"), b"future").unwrap();
    let decoded = decode_rpg(&payload).unwrap();
    assert_eq!(decoded, RpgSaveState::default());
}

#[test]
fn decode_rpg_rejects_missing_head() {
    let mut payload = Vec::new();
    write_subrecord(&mut payload, tag("UNKN"), b"no-head").unwrap();
    let error = decode_rpg(&payload).unwrap_err().to_string();
    assert!(error.contains("RPGS is missing HEAD"), "{error}");
}

#[test]
fn decode_rpg_rejects_unsupported_head_revision() {
    let mut payload = Vec::new();
    write_subrecord(&mut payload, tag("HEAD"), &0u32.to_le_bytes()).unwrap();
    let zero = decode_rpg(&payload).unwrap_err().to_string();
    assert!(zero.contains("unsupported"), "{zero}");
    let mut payload = Vec::new();
    write_subrecord(&mut payload, tag("HEAD"), &2u32.to_le_bytes()).unwrap();
    let future = decode_rpg(&payload).unwrap_err().to_string();
    assert!(future.contains("unsupported"), "{future}");
}

#[test]
fn version_one_inventory_loads_without_condition_or_runtime_drops() {
    let mut save = sample_save();
    save.header.format_version = 1;
    save.next_runtime_item_id = 1;
    save.world.cells.values_mut().for_each(|cell| {
        cell.dropped_items.clear();
        cell.actors.clear();
    });
    for stack in save
        .player
        .as_mut()
        .expect("sample player")
        .inventory
        .iter_mut()
    {
        stack.condition = None;
    }
    if let Some(inventory) = save.world.cells.values_mut().find_map(|cell| {
        cell.references
            .values_mut()
            .find_map(|delta| delta.inventory.as_mut())
    }) {
        for stack in inventory {
            stack.condition = None;
        }
    }
    // Issue #98 (F98.4): equipment/hotkeys did not exist at format v1 --
    // a v1 writer cannot encode them, so the decoded round trip must
    // come back empty.
    let player = save.player.as_mut().expect("sample player");
    player.equipped.clear();
    player.hotkeys = Default::default();
    let bytes = encode_save(&save).unwrap();
    assert_eq!(decode_save(&bytes).unwrap(), save);
}

// Issue #98 (F98.4): a v2 save (equipment/hotkeys did not exist yet)
// loads with an empty equipped set and no hotkey bindings, not an error.
#[test]
fn version_two_save_loads_with_empty_equipment_and_hotkeys() {
    let mut save = sample_save();
    save.header.format_version = 2;
    for cell in save.world.cells.values_mut() {
        cell.actors.clear();
    }
    let player = save.player.as_mut().expect("sample player");
    player.equipped.clear();
    player.hotkeys = Default::default();
    let bytes = encode_save(&save).unwrap();
    let decoded = decode_save(&bytes).unwrap();
    assert_eq!(decoded, save);
    let decoded_player = decoded.player.expect("decoded player");
    assert!(decoded_player.equipped.is_empty());
    assert_eq!(decoded_player.hotkeys, [None; 8]);
}

// Issue #98 (F98.4): a v3 save round-trips its equipped set and hotkey
// bindings byte-identically (the general `round_trip_is_deterministic`
// test above already exercises this via `sample_save`; this test pins
// the shape narrowly).
#[test]
fn version_three_save_round_trips_equipment_and_hotkeys() {
    let mut save = sample_save();
    save.header.format_version = 3;
    for cell in save.world.cells.values_mut() {
        cell.actors.clear();
    }
    assert_eq!(save.header.format_version, 3);
    let bytes = encode_save(&save).unwrap();
    let decoded = decode_save(&bytes).unwrap();
    let player = decoded.player.expect("decoded player");
    assert_eq!(
        player.equipped,
        vec![
            EquippedItem {
                kind: EquippedKind::Apparel,
                base_form_id: 0x0000_0011,
                condition: Some(100),
            },
            EquippedItem {
                kind: EquippedKind::Weapon,
                base_form_id: 0x0000_0042,
                condition: None,
            },
            EquippedItem {
                kind: EquippedKind::Ammo,
                base_form_id: 0x0000_0055,
                condition: None,
            },
        ]
    );
    assert_eq!(player.hotkeys[0].unwrap().base_form_id, 0x0000_0011);
    assert_eq!(player.hotkeys[1], None);
    assert_eq!(player.hotkeys[2].unwrap().base_form_id, 0x0000_0042);
}

#[test]
fn current_format_actor_item_dialogue_location_and_combat_rng_round_trip_deterministically() {
    let save = sample_save();
    assert_eq!(save.header.format_version, CURRENT_SAVE_FORMAT_VERSION);
    let first = encode_save(&save).unwrap();
    let second = encode_save(&save).unwrap();
    assert_eq!(first, second);
    let decoded = decode_save(&first).unwrap();
    assert_eq!(decoded, save);
    let actor = &decoded.world.cells[&0x0001_51e3].actors[&0x0004_1600];
    assert_eq!(actor.life_state, ActorLifeState::Alive);
    assert_eq!(actor.package.unwrap().procedure_index, 3);
}

#[test]
fn version_seven_migrates_combat_rng_from_the_playthrough_seed() {
    let mut save = sample_save();
    save.header.format_version = 7;
    save.combat_rng = bevyout_core::combat::CombatRngState {
        revision: bevyout_core::combat::COMBAT_RNG_REVISION.into(),
        seed: 99,
        draw_index: 12,
    };
    let decoded = decode_save(&encode_save(&save).unwrap()).unwrap();
    assert_eq!(
        decoded.combat_rng,
        bevyout_core::combat::CombatRngState::from_seed(save.rng_state)
    );
}

#[test]
fn version_five_save_loads_with_empty_dialogue_state() {
    let mut save = sample_save();
    save.header.format_version = 5;
    let bytes = encode_save(&save).unwrap();
    let decoded = decode_save(&bytes).unwrap();
    assert_eq!(
        decoded.dialogue,
        bevyout_core::dialogue::DialogueSnapshot::default()
    );
    assert!(!bytes.windows(4).any(|tag| tag == b"DLOG"));
}

#[test]
fn version_six_writes_persistent_dialogue_variables_in_dlog() {
    let mut save = sample_save();
    save.dialogue.variables.set(
        "$global_opened_gate",
        bevyout_core::dialogue::NarrativeValue::Bool(true),
    );
    let bytes = encode_save(&save).unwrap();
    assert!(bytes.windows(4).any(|tag| tag == b"DLOG"));
    let decoded = decode_save(&bytes).unwrap();
    assert_eq!(decoded, save);
}

#[test]
fn version_three_migrates_to_an_empty_actor_state_map() {
    let mut save = sample_save();
    save.header.format_version = 3;
    for cell in save.world.cells.values_mut() {
        cell.actors.clear();
    }
    save.canonical = Some(migrate_legacy(&save).unwrap());
    let decoded = decode_save(&encode_save(&save).unwrap()).unwrap();
    assert!(
        decoded
            .world
            .cells
            .values()
            .all(|cell| cell.actors.is_empty())
    );
    assert_eq!(decoded.canonical, save.canonical);
}

#[test]
fn malformed_actor_state_is_rejected() {
    let mut save = sample_save();
    save.world
        .cells
        .get_mut(&0x0001_51e3)
        .unwrap()
        .actors
        .get_mut(&0x0004_1600)
        .unwrap()
        .value_mutations
        .insert(bevyout_core::actor_state::ActorValue::Health, f32::NAN);
    assert!(encode_save(&save).is_err());

    let mut payload = Vec::new();
    write_subrecord(&mut payload, tag("CELL"), &1u32.to_le_bytes()).unwrap();
    write_subrecord(&mut payload, tag("REFR"), &2u32.to_le_bytes()).unwrap();
    write_subrecord(&mut payload, tag("LIFE"), &[7]).unwrap();
    assert!(decode_actor(&payload).is_err());
}

#[test]
fn duplicate_actor_state_records_are_rejected() {
    let save = sample_save();
    let encoded = encode_save(&save).unwrap();
    let checksum_start = encoded.len() - 40;
    let mut bytes = encoded[..checksum_start].to_vec();
    let actor = &save.world.cells[&0x0001_51e3].actors[&0x0004_1600];
    write_record(
        &mut bytes,
        tag("ACTR"),
        &encode_actor(0x0001_51e3, 0x0004_1600, actor).unwrap(),
    )
    .unwrap();
    let checksum: [u8; 32] = Sha256::digest(&bytes).into();
    write_record(&mut bytes, tag("CHKS"), &checksum).unwrap();

    let error = decode_save(&bytes).unwrap_err().to_string();
    assert!(error.contains("duplicate ACTR"), "{error}");
}

// Issue #98 (F98.4): equipped items must stay strictly sorted by kind,
// FormID, and condition, exactly like inventory stacks.
#[test]
fn an_invalid_equipped_set_is_rejected() {
    let mut save = sample_save();
    save.player.as_mut().unwrap().equipped = vec![
        EquippedItem {
            kind: EquippedKind::Weapon,
            base_form_id: 0x2,
            condition: None,
        },
        EquippedItem {
            kind: EquippedKind::Weapon,
            base_form_id: 0x1,
            condition: None,
        },
    ];
    assert!(encode_save(&save).is_err());
}

#[test]
fn unknown_records_are_skipped_when_checksum_is_recomputed() {
    let encoded = encode_save(&sample_save()).unwrap();
    let checksum_start = encoded.len() - 40;
    let mut bytes = encoded[..checksum_start].to_vec();
    write_record(&mut bytes, tag("FUTR"), b"future data").unwrap();
    let checksum: [u8; 32] = Sha256::digest(&bytes).into();
    write_record(&mut bytes, tag("CHKS"), &checksum).unwrap();
    assert_eq!(decode_save(&bytes).unwrap(), sample_save());
}

// F60.4: a save with no player record round-trips as `None`, so
// pre-player saves stay loadable.
#[test]
fn a_save_without_a_player_record_round_trips_as_none() {
    let mut save = sample_save();
    save.player = None;
    let bytes = encode_save(&save).unwrap();
    let decoded = decode_save(&bytes).unwrap();
    assert_eq!(decoded.player, None);
    assert_eq!(decoded, save);
}

// F60.4: the player inventory rides the same validation as reference
// inventories -- zero counts and unsorted stacks are rejected before
// encoding.
#[test]
fn an_invalid_player_inventory_is_rejected() {
    let mut save = sample_save();
    save.player = Some(PlayerState {
        inventory: vec![ItemStack {
            base_form_id: 0x1,
            count: 0,
            condition: None,
        }],
        ..Default::default()
    });
    assert!(encode_save(&save).is_err());
    save.player = Some(PlayerState {
        inventory: vec![
            ItemStack {
                base_form_id: 0x2,
                count: 1,
                condition: None,
            },
            ItemStack {
                base_form_id: 0x1,
                count: 1,
                condition: None,
            },
        ],
        ..Default::default()
    });
    assert!(encode_save(&save).is_err());
}

// Issue #76 (T76.1): a delta carrying container stacks plus the
// leveled-resolved marker survives encode/decode and re-encodes
// byte-identically (the general `round_trip_is_deterministic` test above
// already exercises this combination via `sample_save`; this test pins
// it narrowly so the LVLR-only case -- no other delta field set -- is
// covered too, matching capture's "resolved-only" delta shape).
#[test]
fn a_leveled_resolved_only_delta_round_trips_byte_identically() {
    let mut save = sample_save();
    let mut references = BTreeMap::new();
    references.insert(
        0x0200_0000,
        PersistentReferenceDelta {
            leveled_resolved: Some(true),
            ..Default::default()
        },
    );
    save.world.cells.insert(
        0x0002_0000,
        PersistentCellState {
            references,
            ..Default::default()
        },
    );

    let first = encode_save(&save).unwrap();
    let second = encode_save(&save).unwrap();
    assert_eq!(first, second);
    let decoded = decode_save(&first).unwrap();
    assert_eq!(decoded, save);
    assert_eq!(
        decoded.world.cells[&0x0002_0000].references[&0x0200_0000].leveled_resolved,
        Some(true)
    );
}

// Issue #76 (T76.3): a save encoded before this field existed never
// wrote an LVLR subrecord; decoding a delta that omits it must produce
// `None`, not a default `Some(false)`, so pre-#76 saves keep loading
// unchanged.
#[test]
fn a_delta_without_leveled_resolved_decodes_as_none() {
    let mut save = sample_save();
    for cell in save.world.cells.values_mut() {
        for delta in cell.references.values_mut() {
            delta.leveled_resolved = None;
        }
    }
    let bytes = encode_save(&save).unwrap();
    let decoded = decode_save(&bytes).unwrap();
    assert_eq!(decoded, save);
    assert!(
        decoded.world.cells[&0x0001_51e3].references[&0x0100_0020]
            .leveled_resolved
            .is_none()
    );
}

// F118.3: corpse holders deliberately reuse the existing reference
// inventory subrecord; no corpse-specific save record is needed, so an
// old reader/writer can round-trip the exact stacks unchanged.
#[test]
fn corpse_inventory_delta_round_trips_through_the_legacy_reference_seam() {
    let mut save = sample_save();
    save.world
        .cells
        .get_mut(&0x0001_51e3)
        .unwrap()
        .references
        .insert(
            0x0000_C0DE,
            PersistentReferenceDelta {
                inventory: Some(vec![
                    ItemStack {
                        base_form_id: 0x10,
                        count: 1,
                        condition: None,
                    },
                    ItemStack {
                        base_form_id: 0x11,
                        count: 2,
                        condition: Some(80),
                    },
                ]),
                ..Default::default()
            },
        );
    let bytes = encode_save(&save).unwrap();
    assert_eq!(decode_save(&bytes).unwrap(), save);
}

// F118.3 compatibility: a save written before corpse support has no
// corpse-specific section and still decodes as the same empty holder
// state rather than requiring a new record.
#[test]
fn old_save_without_corpse_sections_remains_loadable() {
    let mut save = sample_save();
    for cell in save.world.cells.values_mut() {
        cell.references.clear();
    }
    let bytes = encode_save(&save).unwrap();
    let decoded = decode_save(&bytes).unwrap();
    assert!(
        decoded
            .world
            .cells
            .values()
            .all(|cell| cell.references.is_empty())
    );
}

// Issue #76 (T76.3): a truncated OBJE.INVT payload (claims one item but
// carries only the four-byte count header) fails through the existing
// error path rather than silently producing a corrupt inventory.
#[test]
fn a_truncated_container_inventory_payload_is_rejected() {
    let mut payload = Vec::new();
    write_subrecord(&mut payload, tag("CELL"), &1u32.to_le_bytes()).unwrap();
    write_subrecord(&mut payload, tag("REFR"), &2u32.to_le_bytes()).unwrap();
    write_subrecord(&mut payload, tag("FLAG"), &0u32.to_le_bytes()).unwrap();
    write_subrecord(&mut payload, tag("INVT"), &1u32.to_le_bytes()).unwrap();
    assert!(decode_reference(&payload, CURRENT_SAVE_FORMAT_VERSION).is_err());
}

#[test]
fn malformed_dropped_items_are_rejected() {
    let mut save = sample_save();
    let dropped = save
        .world
        .cells
        .get_mut(&0x0001_51e3)
        .unwrap()
        .dropped_items
        .get_mut(&5)
        .unwrap();
    dropped.stack.count = 0;
    assert!(encode_save(&save).is_err());

    let mut save = sample_save();
    save.world
        .cells
        .get_mut(&0x0001_51e3)
        .unwrap()
        .dropped_items
        .get_mut(&5)
        .unwrap()
        .transform
        .translation[0] = f32::NAN;
    assert!(encode_save(&save).is_err());

    let mut save = sample_save();
    save.world
        .cells
        .get_mut(&0x0001_51e3)
        .unwrap()
        .dropped_items
        .get_mut(&5)
        .unwrap()
        .body
        .linear_velocity[1] = f32::INFINITY;
    assert!(encode_save(&save).is_err());

    let mut save = sample_save();
    save.next_runtime_item_id = 5;
    assert!(encode_save(&save).is_err());
}

#[test]
fn duplicate_dropped_runtime_ids_are_rejected_on_decode() {
    let save = sample_save();
    let encoded = encode_save(&save).unwrap();
    let checksum_start = encoded.len() - 40;
    let mut bytes = encoded[..checksum_start].to_vec();
    let dropped = save.world.cells[&0x0001_51e3].dropped_items[&5].clone();
    write_record(
        &mut bytes,
        tag("DROP"),
        &encode_dropped(0x0001_51e3, &dropped).unwrap(),
    )
    .unwrap();
    let checksum: [u8; 32] = Sha256::digest(&bytes).into();
    write_record(&mut bytes, tag("CHKS"), &checksum).unwrap();
    assert!(decode_save(&bytes).is_err());
}

#[test]
fn corruption_is_rejected() {
    let mut bytes = encode_save(&sample_save()).unwrap();
    bytes[12] ^= 0x80;
    assert!(decode_save(&bytes).is_err());
    assert!(decode_save(&bytes[..bytes.len() - 1]).is_err());
}

#[test]
fn content_fingerprint_and_plugins_are_checked_before_apply() {
    let save = sample_save();
    assert!(
        save.ensure_compatible(
            "content-hash",
            &[SavePlugin {
                name: "Fallout3.esm".into(),
                fingerprint: "plugin-hash".into(),
            }]
        )
        .is_ok()
    );
    assert!(save.ensure_compatible("other-hash", &[]).is_err());
}

#[test]
fn backup_is_used_when_primary_is_corrupt() {
    let root = std::env::temp_dir().join(format!(
        "bevyout-save-test-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let store = SaveStore::new(&root);
    let first = sample_save();
    let mut second = sample_save();
    second.rng_state = 42;
    store.write_slot("slot", &first).unwrap();
    store.write_slot("slot", &second).unwrap();
    fs::write(store.primary_path("slot"), b"corrupt").unwrap();
    let outcome = store.read_slot("slot").unwrap();
    assert_eq!(outcome.source, SaveSlotSource::Backup);
    assert_eq!(outcome.save, first);
    assert!(outcome.warning.is_some());
    let _ = fs::remove_dir_all(root);
}

#[test]
fn canonical_v3_round_trip_preserves_ids_conditions_and_opaque_state() {
    let mut save = SaveGame::default();
    save.header.content_fingerprint = "content".into();
    let mut item = ItemInstance::new(
        ItemInstanceId(42),
        0x1234,
        2,
        ItemState {
            condition: Some(77),
            extras: vec![ItemExtraEntry {
                namespace_form_id: 0x99,
                tag: *b"READ",
                payload: vec![1, 2, 3, 4],
            }],
            ..Default::default()
        },
    )
    .unwrap();
    item.state.combat.magazine.ammo_form_id = Some(0x0000_4241);
    item.state.combat.magazine.loaded = 7;
    item.state.combat.jam = Some(bevyout_core::combat::JamReason::Reload);
    let mut holders = BTreeMap::new();
    holders.insert(
        HolderId::Player,
        ItemHolderState {
            items: vec![item],
            caps: 50,
            revision: 3,
        },
    );
    save.canonical = Some(ItemLedgerSnapshot {
        holders,
        next_item_id: ItemInstanceId(43),
        next_transaction_id: TransactionId(1),
        ..Default::default()
    });
    let bytes = encode_save(&save).unwrap();
    let decoded = decode_save(&bytes).unwrap();
    assert_eq!(decoded.canonical, save.canonical);
    assert_eq!(encode_save(&decoded).unwrap(), bytes);
}
