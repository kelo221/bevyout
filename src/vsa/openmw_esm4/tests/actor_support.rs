//! Tests for `RACE`/`CLAS`/`FACT`/`PACK` decoding (M4 wave 1 task B, issue
//! #103). Synthetic byte-level fixtures only, built with the `subrecord`/
//! `record`/`group`/`tes4` helpers and `direct_subrecord`/`direct_resolver`
//! from `tests/mod.rs` -- no real game data.

use super::*;

// ---------------------------------------------------------------------
// Fixture builders
// ---------------------------------------------------------------------

fn race_data(
    male_height: f32,
    female_height: f32,
    male_weight: f32,
    female_weight: f32,
    flags: u32,
) -> Vec<u8> {
    let mut data = vec![0_u8; 16];
    data.extend_from_slice(&male_height.to_le_bytes());
    data.extend_from_slice(&female_height.to_le_bytes());
    data.extend_from_slice(&male_weight.to_le_bytes());
    data.extend_from_slice(&female_weight.to_le_bytes());
    data.extend_from_slice(&flags.to_le_bytes());
    data
}

fn class_data(
    tag_skills: [i32; 4],
    flags: u32,
    services: u32,
    teaches: i8,
    max_training_level: u8,
) -> Vec<u8> {
    let mut data = Vec::new();
    for skill in tag_skills {
        data.extend_from_slice(&skill.to_le_bytes());
    }
    data.extend_from_slice(&flags.to_le_bytes());
    data.extend_from_slice(&services.to_le_bytes());
    data.push(teaches as u8);
    data.push(max_training_level);
    data.extend_from_slice(&[0, 0]);
    data
}

fn fact_data(flags1: u8, flags2: u8) -> Vec<u8> {
    vec![flags1, flags2, 0, 0]
}

fn xnam(faction: u32, modifier: i32, group_combat_reaction: u32) -> Vec<u8> {
    [
        faction.to_le_bytes().as_slice(),
        modifier.to_le_bytes().as_slice(),
        group_combat_reaction.to_le_bytes().as_slice(),
    ]
    .concat()
}

fn pkdt12(general_flags: u32, package_type: u8) -> Vec<u8> {
    let mut data = general_flags.to_le_bytes().to_vec();
    data.push(package_type);
    data.push(0);
    data.extend_from_slice(&0_u16.to_le_bytes());
    data.extend_from_slice(&0_u16.to_le_bytes());
    data.extend_from_slice(&[0, 0]);
    data
}

fn pkdt4(flags: u32) -> Vec<u8> {
    flags.to_le_bytes().to_vec()
}

fn psdt(month: i8, day_of_week: i8, date: u8, time: i8, duration: i32) -> Vec<u8> {
    let mut data = vec![month as u8, day_of_week as u8, date, time as u8];
    data.extend_from_slice(&duration.to_le_bytes());
    data
}

fn pldt(location_type: u32, location: u32, radius: i32) -> Vec<u8> {
    let mut data = location_type.to_le_bytes().to_vec();
    data.extend_from_slice(&location.to_le_bytes());
    data.extend_from_slice(&radius.to_le_bytes());
    data
}

fn ptdt(target_type: i32, target: u32, count_or_distance: i32, unknown: f32) -> Vec<u8> {
    let mut data = target_type.to_le_bytes().to_vec();
    data.extend_from_slice(&target.to_le_bytes());
    data.extend_from_slice(&count_or_distance.to_le_bytes());
    data.extend_from_slice(&unknown.to_le_bytes());
    data
}

// ---------------------------------------------------------------------
// RACE
// ---------------------------------------------------------------------

#[test]
fn race_decodes_stats_links_hair_eyes_parts_and_facegen() {
    let resolver = direct_resolver();
    let mut data = race_data(72.0, 68.0, 180.0, 140.0, 0x5);
    data[0] = 32;
    data[1] = 5;
    data[2] = 45;
    data[3] = (-2_i8) as u8;
    let subs = vec![
        direct_subrecord("EDID", b"RaceTest\0".to_vec()),
        direct_subrecord("FULL", b"Test Race\0".to_vec()),
        direct_subrecord("DATA", data),
        direct_subrecord("ONAM", 0x100_u32.to_le_bytes().to_vec()),
        direct_subrecord("YNAM", 0x101_u32.to_le_bytes().to_vec()),
        direct_subrecord(
            "DNAM",
            [0x200_u32.to_le_bytes(), 0x201_u32.to_le_bytes()].concat(),
        ),
        direct_subrecord("CNAM", vec![3, 7]),
        direct_subrecord("NAM0", vec![]),
        direct_subrecord("MNAM", vec![]),
        direct_subrecord("INDX", 0_u32.to_le_bytes().to_vec()),
        direct_subrecord("MODL", b"male_head0.nif\0".to_vec()),
        direct_subrecord("INDX", 1_u32.to_le_bytes().to_vec()),
        direct_subrecord("MODL", b"male_head1.nif\0".to_vec()),
        direct_subrecord("FNAM", vec![]),
        direct_subrecord("INDX", 0_u32.to_le_bytes().to_vec()),
        direct_subrecord("MODL", b"female_head0.nif\0".to_vec()),
        direct_subrecord("NAM1", vec![]),
        direct_subrecord("MNAM", vec![]),
        direct_subrecord("INDX", 0_u32.to_le_bytes().to_vec()),
        direct_subrecord("MODL", b"male_body0.nif\0".to_vec()),
        direct_subrecord("FNAM", vec![]),
        direct_subrecord("INDX", 0_u32.to_le_bytes().to_vec()),
        direct_subrecord("MODL", b"female_body0.nif\0".to_vec()),
        direct_subrecord(
            "HNAM",
            [0x300_u32.to_le_bytes(), 0x301_u32.to_le_bytes()].concat(),
        ),
        direct_subrecord(
            "ENAM",
            [0x400_u32.to_le_bytes(), 0x401_u32.to_le_bytes()].concat(),
        ),
        direct_subrecord("MNAM", vec![]),
        direct_subrecord("FGGS", vec![1, 2, 3]),
        direct_subrecord("FGGA", vec![4, 5, 6]),
        direct_subrecord("FGTS", vec![7, 8, 9]),
        direct_subrecord("FNAM", vec![]),
        direct_subrecord("FGGS", vec![11, 12]),
        direct_subrecord("FGGA", vec![13, 14]),
        direct_subrecord("FGTS", vec![15, 16]),
    ];

    let race = parse_race(&subs, 0x9000, 0, &resolver).unwrap();
    assert_eq!(race.editor_id.as_deref(), Some("RaceTest"));
    assert_eq!(race.name.as_deref(), Some("Test Race"));
    assert_eq!(race.male_height, 72.0);
    assert_eq!(race.female_height, 68.0);
    assert_eq!(race.male_weight, 180.0);
    assert_eq!(race.female_weight, 140.0);
    assert_eq!(race.flags, 0x5);
    assert_eq!(
        race.skill_boosts,
        vec![
            RaceSkillBoost {
                actor_value: 32,
                boost: 5,
            },
            RaceSkillBoost {
                actor_value: 45,
                boost: -2,
            },
        ]
    );
    assert_eq!(race.older_race_form_id, Some(0x100));
    assert_eq!(race.younger_race_form_id, Some(0x101));
    assert_eq!(race.male_default_hair_form_id, Some(0x200));
    assert_eq!(race.female_default_hair_form_id, Some(0x201));
    assert_eq!(race.male_default_hair_color, Some(3));
    assert_eq!(race.female_default_hair_color, Some(7));
    assert_eq!(race.hair_form_ids, vec![0x300, 0x301]);
    assert_eq!(race.eye_form_ids, vec![0x400, 0x401]);
    assert_eq!(
        race.head_parts_male,
        vec![
            RacePartEntry {
                index: 0,
                model_path: Some("male_head0.nif".into()),
            },
            RacePartEntry {
                index: 1,
                model_path: Some("male_head1.nif".into()),
            },
        ]
    );
    assert_eq!(
        race.head_parts_female,
        vec![RacePartEntry {
            index: 0,
            model_path: Some("female_head0.nif".into()),
        }]
    );
    assert_eq!(
        race.body_parts_male,
        vec![RacePartEntry {
            index: 0,
            model_path: Some("male_body0.nif".into()),
        }]
    );
    assert_eq!(
        race.body_parts_female,
        vec![RacePartEntry {
            index: 0,
            model_path: Some("female_body0.nif".into()),
        }]
    );
    assert_eq!(race.face_gen_male.geometry_symmetric, Some(vec![1, 2, 3]));
    assert_eq!(race.face_gen_male.geometry_asymmetric, Some(vec![4, 5, 6]));
    assert_eq!(race.face_gen_male.texture_symmetric, Some(vec![7, 8, 9]));
    assert_eq!(race.face_gen_female.geometry_symmetric, Some(vec![11, 12]));
    assert_eq!(race.face_gen_female.geometry_asymmetric, Some(vec![13, 14]));
    assert_eq!(race.face_gen_female.texture_symmetric, Some(vec![15, 16]));
    assert!(race.ignored_subrecords.is_empty());
}

#[test]
fn race_truncated_data_produces_diagnostic_without_panicking() {
    let resolver = direct_resolver();
    let error = parse_race(
        &[direct_subrecord("DATA", vec![0; 20])],
        0x9001,
        0,
        &resolver,
    )
    .unwrap_err();
    assert!(error.contains("36 bytes"));
}

#[test]
fn race_missing_data_is_diagnosed() {
    let resolver = direct_resolver();
    let error = parse_race(
        &[direct_subrecord("EDID", b"NoData\0".to_vec())],
        0x9002,
        0,
        &resolver,
    )
    .unwrap_err();
    assert!(error.contains("missing DATA"));
}

// ---------------------------------------------------------------------
// CLAS
// ---------------------------------------------------------------------

#[test]
fn class_decodes_tag_skills_flags_services_teaches_and_training() {
    let subs = vec![
        direct_subrecord("EDID", b"ClassTest\0".to_vec()),
        direct_subrecord("FULL", b"Test Class\0".to_vec()),
        direct_subrecord("DATA", class_data([7, 12, -1, 20], 0x1, 0x4, 5, 50)),
    ];
    let class = parse_class(&subs, 0x9100, 0).unwrap();
    assert_eq!(class.editor_id.as_deref(), Some("ClassTest"));
    assert_eq!(class.name.as_deref(), Some("Test Class"));
    assert_eq!(class.tag_skills, [7, 12, -1, 20]);
    assert_eq!(class.flags, 0x1);
    assert_eq!(class.services, 0x4);
    assert_eq!(class.teaches, 5);
    assert_eq!(class.max_training_level, 50);
    assert!(class.ignored_subrecords.is_empty());
}

#[test]
fn class_truncated_data_produces_diagnostic_without_panicking() {
    let error = parse_class(&[direct_subrecord("DATA", vec![0; 10])], 0x9101, 0).unwrap_err();
    assert!(error.contains("28 bytes"));
}

// ---------------------------------------------------------------------
// FACT
// ---------------------------------------------------------------------

#[test]
fn faction_decodes_flags_repeated_relations_and_ranks() {
    let resolver = direct_resolver();
    let subs = vec![
        direct_subrecord("EDID", b"FactionTest\0".to_vec()),
        direct_subrecord("FULL", b"Test Faction\0".to_vec()),
        direct_subrecord("DATA", fact_data(0x1, 0x2)),
        direct_subrecord("XNAM", xnam(0x500, -10, 1)),
        direct_subrecord("XNAM", xnam(0x501, 20, 2)),
        direct_subrecord("RNAM", 0_i32.to_le_bytes().to_vec()),
        direct_subrecord("MNAM", b"Initiate\0".to_vec()),
        direct_subrecord("FNAM", b"Initiate (f)\0".to_vec()),
        direct_subrecord("INAM", b"insignia0.dds\0".to_vec()),
        direct_subrecord("RNAM", 1_i32.to_le_bytes().to_vec()),
        direct_subrecord("MNAM", b"Veteran\0".to_vec()),
        direct_subrecord("FNAM", b"Veteran (f)\0".to_vec()),
    ];

    let faction = parse_faction(&subs, 0x9200, 0, &resolver).unwrap();
    assert_eq!(faction.editor_id.as_deref(), Some("FactionTest"));
    assert_eq!(faction.name.as_deref(), Some("Test Faction"));
    assert_eq!(faction.flags1, 0x1);
    assert_eq!(faction.flags2, 0x2);
    assert_eq!(
        faction.relations,
        vec![
            FactionRelation {
                faction_form_id: 0x500,
                modifier: -10,
                group_combat_reaction: 1,
            },
            FactionRelation {
                faction_form_id: 0x501,
                modifier: 20,
                group_combat_reaction: 2,
            },
        ]
    );
    assert_eq!(faction.ranks.len(), 2);
    assert_eq!(faction.ranks[0].rank_number, 0);
    assert_eq!(faction.ranks[0].male_title.as_deref(), Some("Initiate"));
    assert_eq!(
        faction.ranks[0].female_title.as_deref(),
        Some("Initiate (f)")
    );
    assert_eq!(faction.ranks[0].insignia.as_deref(), Some("insignia0.dds"));
    assert_eq!(faction.ranks[1].rank_number, 1);
    assert_eq!(faction.ranks[1].male_title.as_deref(), Some("Veteran"));
    assert_eq!(
        faction.ranks[1].female_title.as_deref(),
        Some("Veteran (f)")
    );
    assert_eq!(faction.ranks[1].insignia, None);
    assert!(faction.ignored_subrecords.is_empty());
}

#[test]
fn faction_missing_data_defaults_to_zero_flags() {
    let resolver = direct_resolver();
    let faction = parse_faction(
        &[direct_subrecord("EDID", b"NoData\0".to_vec())],
        0x9201,
        0,
        &resolver,
    )
    .unwrap();
    assert_eq!(faction.flags1, 0);
    assert_eq!(faction.flags2, 0);
}

#[test]
fn faction_truncated_data_produces_diagnostic_without_panicking() {
    let resolver = direct_resolver();
    let error =
        parse_faction(&[direct_subrecord("DATA", vec![1])], 0x9202, 0, &resolver).unwrap_err();
    assert!(error.contains("at least 2 bytes"));
}

// ---------------------------------------------------------------------
// PACK
// ---------------------------------------------------------------------

#[test]
fn package_decodes_pkdt_location_schedule_target_and_opaque_conditions() {
    let resolver = direct_resolver();
    let subs = vec![
        direct_subrecord("EDID", b"PackageTest\0".to_vec()),
        direct_subrecord("PKDT", pkdt12(0x21, 5)),
        direct_subrecord("PLDT", pldt(0, 0x600, 512)),
        direct_subrecord("PSDT", psdt(-1, -1, 0, -1, 3600)),
        direct_subrecord("PTDT", ptdt(0, 0x601, 1, 2.5)),
        direct_subrecord("CTDA", vec![1, 2, 3, 4]),
        direct_subrecord("CTDA", vec![5, 6, 7, 8]),
    ];

    let package = parse_package(&subs, 0x9300, 0, &resolver).unwrap();
    assert_eq!(package.editor_id.as_deref(), Some("PackageTest"));
    assert_eq!(package.general_flags, 0x21);
    assert_eq!(package.package_type, 5);

    let location = package.location.unwrap();
    assert_eq!(location.location_type, 0);
    assert_eq!(location.form_id, Some(0x600));
    assert_eq!(location.radius, 512);

    let schedule = package.schedule.unwrap();
    assert_eq!(schedule.month, -1);
    assert_eq!(schedule.day_of_week, -1);
    assert_eq!(schedule.date, 0);
    assert_eq!(schedule.time, -1);
    assert_eq!(schedule.duration, 3600);

    let target = package.target.unwrap();
    assert_eq!(target.target_type, 0);
    assert_eq!(target.form_id, Some(0x601));
    assert_eq!(target.count_or_distance, 1);
    assert_eq!(target.unknown, 2.5);

    // Opaque CTDA passthrough, exactly like RecipeRecord::conditions: raw
    // bytes preserved in stream order, no interpretation.
    assert_eq!(package.conditions, vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]]);
    assert!(package.ignored_subrecords.is_empty());
}

#[test]
fn package_pkdt_legacy_four_byte_layout_defaults_type_to_zero() {
    let resolver = direct_resolver();
    let package = parse_package(
        &[direct_subrecord("PKDT", pkdt4(0x9))],
        0x9301,
        0,
        &resolver,
    )
    .unwrap();
    assert_eq!(package.general_flags, 0x9);
    assert_eq!(package.package_type, 0);
}

#[test]
fn package_location_type_5_and_target_type_2_are_not_resolved_as_form_ids() {
    let resolver = direct_resolver();
    let subs = vec![
        direct_subrecord("PKDT", pkdt12(0, 0)),
        direct_subrecord("PLDT", pldt(5, 0x77, 0)),
        direct_subrecord("PTDT", ptdt(2, 0x88, 0, 0.0)),
    ];
    let package = parse_package(&subs, 0x9302, 0, &resolver).unwrap();
    assert_eq!(package.location.unwrap().form_id, None);
    assert_eq!(package.target.unwrap().form_id, None);
    // The raw value is still retained even when it is not resolved as a
    // FormID (fopdoc: type 5 / type 2 hold an "Object Type" enum there).
    assert_eq!(package.location.unwrap().raw_value, 0x77);
    assert_eq!(package.target.unwrap().raw_value, 0x88);
}

#[test]
fn package_truncated_pkdt_produces_diagnostic_without_panicking() {
    let resolver = direct_resolver();
    let error = parse_package(
        &[direct_subrecord("PKDT", vec![0; 7])],
        0x9303,
        0,
        &resolver,
    )
    .unwrap_err();
    assert!(error.contains("4, 8, or 12 bytes"));
}

#[test]
fn package_missing_pkdt_is_diagnosed() {
    let resolver = direct_resolver();
    let error = parse_package(
        &[direct_subrecord("EDID", b"NoPkdt\0".to_vec())],
        0x9304,
        0,
        &resolver,
    )
    .unwrap_err();
    assert!(error.contains("missing PKDT"));
}

// ---------------------------------------------------------------------
// Full-plugin: FormID master-index adjustment, load-order override,
// deleted records, and diagnostics surfaced through parse_content_set.
// ---------------------------------------------------------------------

#[test]
fn race_overrides_apply_and_fact_xnam_form_ids_are_adjusted_through_master_indices() {
    let mut master = tes4(&[]);
    master.extend(record(
        b"RACE",
        0,
        0x9400,
        &[
            subrecord(b"EDID", b"OriginalRace\0"),
            subrecord(b"DATA", &race_data(60.0, 58.0, 140.0, 120.0, 0x1)),
        ]
        .concat(),
    ));
    master.extend(record(
        b"FACT",
        0,
        0x9402,
        &subrecord(b"EDID", b"MasterOnlyFaction\0"),
    ));

    let mut override_plugin = tes4(&["Master.esm"]);
    override_plugin.extend(record(
        b"RACE",
        0,
        0x9400,
        &[
            subrecord(b"EDID", b"OverriddenRace\0"),
            subrecord(b"DATA", &race_data(61.0, 59.0, 141.0, 121.0, 0x5)),
        ]
        .concat(),
    ));
    override_plugin.extend(record(
        b"FACT",
        0,
        0x9401,
        &[
            subrecord(b"EDID", b"OverridePluginFaction\0"),
            subrecord(b"XNAM", &xnam(0x9402, -5, 1)),
        ]
        .concat(),
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
        &CellSelector::FormId(0x1),
    )
    .unwrap();

    let race = parsed.races.get(&0x9400).unwrap();
    assert_eq!(race.editor_id.as_deref(), Some("OverriddenRace"));
    assert_eq!(race.male_height, 61.0);

    let faction = parsed.factions.get(&0x9401).unwrap();
    assert_eq!(faction.relations[0].faction_form_id, 0x9402);
}

#[test]
fn deleted_race_class_faction_package_records_are_removed() {
    let mut plugin = tes4(&[]);
    plugin.extend(record(
        b"RACE",
        0,
        0x9500,
        &[
            subrecord(b"EDID", b"ToDelete\0"),
            subrecord(b"DATA", &race_data(60.0, 60.0, 150.0, 150.0, 0)),
        ]
        .concat(),
    ));
    plugin.extend(record(b"RACE", RECORD_DELETED, 0x9500, &[]));

    plugin.extend(record(
        b"CLAS",
        0,
        0x9501,
        &[
            subrecord(b"EDID", b"ClassToDelete\0"),
            subrecord(b"DATA", &class_data([0, 0, 0, 0], 0, 0, -1, 0)),
        ]
        .concat(),
    ));
    plugin.extend(record(b"CLAS", RECORD_DELETED, 0x9501, &[]));

    plugin.extend(record(
        b"FACT",
        0,
        0x9502,
        &subrecord(b"EDID", b"FactionToDelete\0"),
    ));
    plugin.extend(record(b"FACT", RECORD_DELETED, 0x9502, &[]));

    plugin.extend(record(
        b"PACK",
        0,
        0x9503,
        &[
            subrecord(b"EDID", b"PackageToDelete\0"),
            subrecord(b"PKDT", &pkdt12(0, 0)),
        ]
        .concat(),
    ));
    plugin.extend(record(b"PACK", RECORD_DELETED, 0x9503, &[]));

    let parsed = parse_plugin(&plugin, 0x1).unwrap();
    assert!(!parsed.races.contains_key(&0x9500));
    assert!(!parsed.classes.contains_key(&0x9501));
    assert!(!parsed.factions.contains_key(&0x9502));
    assert!(!parsed.packages.contains_key(&0x9503));
}

#[test]
fn malformed_supporting_records_are_reported_and_removed_without_partial_state() {
    let mut plugin = tes4(&[]);
    plugin.extend(record(b"RACE", 0, 0x9600, &subrecord(b"DATA", &[0; 10])));
    plugin.extend(record(b"CLAS", 0, 0x9601, &subrecord(b"DATA", &[0; 5])));
    plugin.extend(record(b"PACK", 0, 0x9602, &subrecord(b"PKDT", &[0; 3])));

    let parsed = parse_plugin(&plugin, 0x1).unwrap();
    assert!(!parsed.races.contains_key(&0x9600));
    assert!(!parsed.classes.contains_key(&0x9601));
    assert!(!parsed.packages.contains_key(&0x9602));
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|message| message.contains("RACE 00009600") && message.contains("malformed"))
    );
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|message| message.contains("CLAS 00009601") && message.contains("malformed"))
    );
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|message| message.contains("PACK 00009602") && message.contains("malformed"))
    );
}

#[test]
fn unknown_race_subrecords_are_retained_as_diagnostics() {
    let mut plugin = tes4(&[]);
    plugin.extend(record(
        b"RACE",
        0,
        0x9700,
        &[
            subrecord(b"EDID", b"RaceWithUnknown\0"),
            subrecord(b"DATA", &race_data(60.0, 60.0, 150.0, 150.0, 0)),
            subrecord(b"ZZZZ", &[1, 2, 3]),
        ]
        .concat(),
    ));

    let parsed = parse_plugin(&plugin, 0x1).unwrap();
    assert!(parsed.races.contains_key(&0x9700));
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|message| message.contains("RACE 00009700") && message.contains("ZZZZ"))
    );
}

#[test]
fn package_pkdt_eight_byte_layout_decodes_real_eat_and_sleep_types() {
    let resolver = direct_resolver();
    for (form_id, package_type) in [(0x0001_ff1e, 3_u8), (0x0001_ff1f, 4_u8)] {
        let mut pkdt = 0x0102_0304_u32.to_le_bytes().to_vec();
        pkdt.extend_from_slice(&[package_type, 0]);
        pkdt.extend_from_slice(&0x1234_u16.to_le_bytes());
        let package =
            parse_package(&[direct_subrecord("PKDT", pkdt)], form_id, 0, &resolver).unwrap();
        assert_eq!(package.form_id, form_id);
        assert_eq!(package.general_flags, 0x0102_0304);
        assert_eq!(package.package_type, package_type);
    }
}

#[test]
fn package_pkdt_twelve_byte_layout_ignores_absent_type_specific_bytes() {
    let resolver = direct_resolver();
    let package = parse_package(
        &[direct_subrecord("PKDT", pkdt12(0x99, 7))],
        0x0001_ff20,
        0,
        &resolver,
    )
    .unwrap();
    assert_eq!(package.general_flags, 0x99);
    assert_eq!(package.package_type, 7);
}

#[test]
fn package_idle_collection_decodes_and_resolves_all_valid_idla_entries() {
    let resolver = direct_resolver();
    let mut idla = Vec::new();
    idla.extend_from_slice(&0x0006_7941_u32.to_le_bytes());
    idla.extend_from_slice(&0x0006_7942_u32.to_le_bytes());
    let package = parse_package(
        &[
            direct_subrecord("PKDT", pkdt8(0, 5)),
            direct_subrecord("IDLF", vec![0x05]),
            direct_subrecord("IDLC", vec![2]),
            direct_subrecord("IDLT", 12.5_f32.to_le_bytes().to_vec()),
            direct_subrecord("IDLA", idla),
        ],
        0x0001_ff30,
        0,
        &resolver,
    )
    .unwrap();
    let collection = package.idle_collection.expect("IDLE collection");
    assert_eq!(collection.flags, 0x05);
    assert_eq!(collection.timer_seconds, 12.5);
    assert_eq!(
        collection.animation_form_ids,
        vec![0x0006_7941, 0x0006_7942]
    );
    assert!(package.diagnostics.is_empty());
    assert!(package.ignored_subrecords.is_empty());
}

#[test]
fn package_idle_collection_count_mismatch_is_diagnosed_without_truncating_ids() {
    let resolver = direct_resolver();
    let mut idla = Vec::new();
    idla.extend_from_slice(&0x100_u32.to_le_bytes());
    idla.extend_from_slice(&0x101_u32.to_le_bytes());
    let package = parse_package(
        &[
            direct_subrecord("PKDT", pkdt8(0, 5)),
            direct_subrecord("IDLC", vec![1]),
            direct_subrecord("IDLA", idla),
        ],
        0x0001_ff31,
        0,
        &resolver,
    )
    .unwrap();
    assert_eq!(
        package
            .idle_collection
            .expect("IDLE collection")
            .animation_form_ids,
        vec![0x100, 0x101]
    );
    assert!(
        package
            .diagnostics
            .iter()
            .any(|message| message.contains("IDLC count mismatch"))
    );
}

fn pkdt8(general_flags: u32, package_type: u8) -> Vec<u8> {
    let mut data = general_flags.to_le_bytes().to_vec();
    data.extend_from_slice(&[package_type, 0]);
    data.extend_from_slice(&0_u16.to_le_bytes());
    data
}
