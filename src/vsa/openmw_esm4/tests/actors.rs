//! Tests for `NPC_`/`CREA` actor subrecord decoding (issue #103, M4 wave 1
//! task A). Synthetic fixtures only, built byte-by-byte with the shared
//! `tests/mod.rs` builders -- no real game data.

use super::super::*;
use super::*;

fn form_id_bytes(id: u32) -> Vec<u8> {
    id.to_le_bytes().to_vec()
}

#[test]
fn appearance_part_records_retain_actor_assembly_assets() {
    let resolver = direct_resolver();
    for (kind, path) in [
        ("HDPT", "characters/head/humanhead.nif"),
        ("HAIR", "characters/hair/messy.nif"),
    ] {
        let record = parse_base(
            kind,
            &[
                direct_subrecord("EDID", format!("Test{kind}\0").into_bytes()),
                direct_subrecord("MODL", format!("{path}\0").into_bytes()),
            ],
            &resolver,
        )
        .unwrap_or_else(|| panic!("{kind} must be retained as a supported actor part"));

        assert_eq!(record.kind, kind);
        assert_eq!(record.model.as_deref(), Some(path));
        assert!(record.ignored_subrecords.is_empty());
    }

    let eyes = parse_base(
        "EYES",
        &[
            direct_subrecord("EDID", b"TestEYES\0".to_vec()),
            direct_subrecord("ICON", b"characters/eyes/blue.dds\0".to_vec()),
        ],
        &resolver,
    )
    .expect("EYES must be retained as a supported actor appearance record");
    assert_eq!(eyes.icon.as_deref(), Some("characters/eyes/blue.dds"));
    assert!(eyes.model.is_none(), "EYES selects texture, not geometry");
    assert!(eyes.ignored_subrecords.is_empty());
}

#[test]
fn decodes_every_supported_npc_subrecord() {
    let resolver = direct_resolver();

    let mut acbs = 0x0000_0011_u32.to_le_bytes().to_vec();
    acbs.extend_from_slice(&50_u16.to_le_bytes());
    acbs.extend_from_slice(&100_u16.to_le_bytes());
    acbs.extend_from_slice(&5_i16.to_le_bytes());
    acbs.extend_from_slice(&1_u16.to_le_bytes());
    acbs.extend_from_slice(&10_u16.to_le_bytes());
    acbs.extend_from_slice(&100_u16.to_le_bytes());
    acbs.extend_from_slice(&50.0_f32.to_le_bytes());
    acbs.extend_from_slice(&(-10_i16).to_le_bytes());
    let template_flags = ActorBaseConfig::TEMPLATE_USE_TRAITS
        | ActorBaseConfig::TEMPLATE_USE_STATS
        | ActorBaseConfig::TEMPLATE_USE_AI_DATA;
    acbs.extend_from_slice(&template_flags.to_le_bytes());
    assert_eq!(acbs.len(), 24);

    let mut aidt = vec![1_u8, 2, 3, 4, 5, 0, 0, 0];
    aidt.extend_from_slice(&0x0000_00ff_u32.to_le_bytes());
    aidt.push(7);
    aidt.push(4);
    aidt.push(1);
    aidt.push(1);
    aidt.extend_from_slice(&500_i32.to_le_bytes());
    assert_eq!(aidt.len(), 20);

    let mut npc_data = 200_i32.to_le_bytes().to_vec();
    npc_data.extend_from_slice(&[5, 6, 7, 4, 5, 6, 7]);
    assert_eq!(npc_data.len(), 11);

    let mut dnam = vec![10_u8, 20, 30, 40, 50, 60, 70, 80, 90, 15, 25, 35, 45, 55];
    dnam.extend_from_slice(&[1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    assert_eq!(dnam.len(), 28);

    let subs = vec![
        direct_subrecord("ACBS", acbs),
        direct_subrecord("SNAM", [form_id_bytes(0x1000), vec![2, 0, 0, 0]].concat()),
        direct_subrecord(
            "SNAM",
            [form_id_bytes(0x1001), vec![0xff, 0, 0, 0]].concat(),
        ),
        direct_subrecord("INAM", form_id_bytes(0x500)),
        direct_subrecord("VTCK", form_id_bytes(0x200)),
        direct_subrecord("RNAM", form_id_bytes(0x100)),
        direct_subrecord("EITM", form_id_bytes(0x300)),
        direct_subrecord("SCRI", form_id_bytes(0x400)),
        direct_subrecord("AIDT", aidt),
        direct_subrecord("PKID", form_id_bytes(0x2000)),
        direct_subrecord("PKID", form_id_bytes(0x2001)),
        direct_subrecord("CNAM", form_id_bytes(0x800)),
        direct_subrecord("DATA", npc_data),
        direct_subrecord("DNAM", dnam),
        direct_subrecord("PNAM", form_id_bytes(0x3000)),
        direct_subrecord("PNAM", form_id_bytes(0x3001)),
        direct_subrecord("HNAM", form_id_bytes(0x600)),
        direct_subrecord("ENAM", form_id_bytes(0x700)),
        direct_subrecord("LNAM", 12.5_f32.to_le_bytes().to_vec()),
        direct_subrecord("HCLR", vec![255, 128, 64, 255]),
        direct_subrecord("ZNAM", form_id_bytes(0x900)),
        direct_subrecord("NAM6", 1.05_f32.to_le_bytes().to_vec()),
        direct_subrecord("NAM7", 0.98_f32.to_le_bytes().to_vec()),
        direct_subrecord("FGGS", vec![1, 2, 3, 4, 5]),
        direct_subrecord("FGGA", vec![9, 8, 7]),
        direct_subrecord("FGTS", vec![6, 5, 4, 3, 2, 1]),
    ];

    let base = parse_base("NPC_", &subs, &resolver).unwrap();
    let actor = base.actor.as_ref().expect("NPC_ must decode actor data");

    let config = actor.base_config.expect("ACBS must decode");
    assert_eq!(config.flags, 0x0000_0011);
    assert_eq!(config.fatigue, 50);
    assert_eq!(config.barter_gold, 100);
    assert_eq!(config.level_or_mult, 5);
    assert_eq!(config.calc_min_level, 1);
    assert_eq!(config.calc_max_level, 10);
    assert_eq!(config.speed_multiplier, 100);
    assert_eq!(config.karma, 50.0);
    assert_eq!(config.disposition_base, -10);
    assert_eq!(config.template_flags, template_flags);
    assert!(config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_TRAITS));
    assert!(config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_STATS));
    assert!(config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_AI_DATA));
    assert!(!config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_FACTIONS));

    assert_eq!(
        actor.factions,
        vec![
            FactionMembership {
                faction_form_id: 0x1000,
                rank: 2,
            },
            FactionMembership {
                faction_form_id: 0x1001,
                rank: -1,
            },
        ]
    );
    assert_eq!(actor.death_item_form_id, Some(0x500));
    assert_eq!(actor.voice_form_id, Some(0x200));
    assert_eq!(actor.race_form_id, Some(0x100));
    assert_eq!(actor.unarmed_attack_effect_form_id, Some(0x300));
    assert_eq!(actor.script_form_id, Some(0x400));

    let ai = actor.ai_data.expect("AIDT must decode");
    assert_eq!(ai.aggression, 1);
    assert_eq!(ai.confidence, 2);
    assert_eq!(ai.energy_level, 3);
    assert_eq!(ai.responsibility, 4);
    assert_eq!(ai.mood, 5);
    assert_eq!(ai.services, 0x0000_00ff);
    assert_eq!(ai.teaches, 7);
    assert_eq!(ai.max_training_level, 4);
    assert_eq!(ai.assistance, 1);
    assert_eq!(ai.aggro_radius_behavior, 1);
    assert_eq!(ai.aggro_radius, 500);

    assert_eq!(actor.package_form_ids, vec![0x2000, 0x2001]);
    assert_eq!(actor.class_form_id, Some(0x800));

    let stats = actor.base_stats.expect("DATA must decode");
    assert_eq!(stats.base_health, 200);
    assert_eq!(stats.strength, 5);
    assert_eq!(stats.perception, 6);
    assert_eq!(stats.endurance, 7);
    assert_eq!(stats.charisma, 4);
    assert_eq!(stats.intelligence, 5);
    assert_eq!(stats.agility, 6);
    assert_eq!(stats.luck, 7);

    let skills = actor.skills.expect("DNAM must decode");
    assert_eq!(
        skills.values,
        [10, 20, 30, 40, 50, 60, 70, 80, 90, 15, 25, 35, 45, 55]
    );
    assert_eq!(
        skills.offsets,
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
    );

    assert_eq!(actor.head_part_form_ids, vec![0x3000, 0x3001]);
    assert_eq!(actor.hair_form_id, Some(0x600));
    assert_eq!(actor.eyes_form_id, Some(0x700));
    assert_eq!(actor.hair_length, Some(12.5));
    assert_eq!(
        actor.hair_color_rgba,
        Some([1.0, 128.0 / 255.0, 64.0 / 255.0, 1.0])
    );
    assert_eq!(actor.combat_style_form_id, Some(0x900));
    assert_eq!(actor.height, Some(1.05));
    assert_eq!(actor.weight, Some(0.98));
    assert_eq!(actor.facegen_geometry_symmetric, Some(vec![1, 2, 3, 4, 5]));
    assert_eq!(actor.facegen_geometry_asymmetric, Some(vec![9, 8, 7]));
    assert_eq!(
        actor.facegen_texture_symmetric,
        Some(vec![6, 5, 4, 3, 2, 1])
    );
    assert!(actor.creature.is_none());
    assert!(base.ignored_subrecords.is_empty());
}

#[test]
fn decodes_every_supported_crea_subrecord() {
    let resolver = direct_resolver();

    let mut acbs = 0x0000_0004_u32.to_le_bytes().to_vec();
    acbs.extend_from_slice(&30_u16.to_le_bytes());
    acbs.extend_from_slice(&0_u16.to_le_bytes());
    acbs.extend_from_slice(&3_i16.to_le_bytes());
    acbs.extend_from_slice(&1_u16.to_le_bytes());
    acbs.extend_from_slice(&5_u16.to_le_bytes());
    acbs.extend_from_slice(&90_u16.to_le_bytes());
    acbs.extend_from_slice(&0.0_f32.to_le_bytes());
    acbs.extend_from_slice(&0_i16.to_le_bytes());
    acbs.extend_from_slice(&0_u16.to_le_bytes());
    assert_eq!(acbs.len(), 24);

    let mut aidt = vec![2_u8, 1, 4, 0, 0, 0, 0, 0];
    aidt.extend_from_slice(&0_u32.to_le_bytes());
    aidt.push(0);
    aidt.push(0);
    aidt.push(0);
    aidt.push(0);
    aidt.extend_from_slice(&1200_i32.to_le_bytes());
    assert_eq!(aidt.len(), 20);

    let mut crea_data = vec![1_u8, 20, 0, 0];
    crea_data.extend_from_slice(&300_i16.to_le_bytes());
    crea_data.extend_from_slice(&[0, 0]);
    crea_data.extend_from_slice(&25_i16.to_le_bytes());
    crea_data.extend_from_slice(&[8, 5, 9, 3, 2, 7, 4]);
    assert_eq!(crea_data.len(), 17);

    let mut nifz = b"mesh/creature/a.nif\0".to_vec();
    nifz.extend_from_slice(b"mesh/creature/b.nif\0");
    let mut kffz = b"anim/idle.kf\0".to_vec();
    kffz.extend_from_slice(b"anim/attack.kf\0");

    let subs = vec![
        direct_subrecord("NIFZ", nifz),
        direct_subrecord("NIFT", vec![0xaa, 0xbb, 0xcc]),
        direct_subrecord("KFFZ", kffz),
        direct_subrecord("ACBS", acbs),
        direct_subrecord("SNAM", [form_id_bytes(0x1100), vec![1, 0, 0, 0]].concat()),
        direct_subrecord("SNAM", [form_id_bytes(0x1101), vec![3, 0, 0, 0]].concat()),
        direct_subrecord("INAM", form_id_bytes(0x510)),
        direct_subrecord("PKID", form_id_bytes(0x2100)),
        direct_subrecord("PKID", form_id_bytes(0x2101)),
        direct_subrecord("AIDT", aidt),
        direct_subrecord("DATA", crea_data),
        direct_subrecord("RNAM", vec![2]),
        direct_subrecord("ZNAM", form_id_bytes(0x910)),
        direct_subrecord("TNAM", 45.0_f32.to_le_bytes().to_vec()),
        direct_subrecord("BNAM", 1.5_f32.to_le_bytes().to_vec()),
        direct_subrecord("WNAM", 2.25_f32.to_le_bytes().to_vec()),
        direct_subrecord("CSCR", form_id_bytes(0x1200)),
        direct_subrecord("CSDT", 4_u32.to_le_bytes().to_vec()),
        direct_subrecord("CSDI", form_id_bytes(0x1300)),
        direct_subrecord("CSDC", vec![50]),
        direct_subrecord("CSDI", form_id_bytes(0x1301)),
        direct_subrecord("CSDC", vec![60]),
        direct_subrecord("CSDT", 8_u32.to_le_bytes().to_vec()),
        direct_subrecord("CSDI", form_id_bytes(0x1400)),
        direct_subrecord("CSDC", vec![100]),
    ];

    let base = parse_base("CREA", &subs, &resolver).unwrap();
    let actor = base.actor.as_ref().expect("CREA must decode actor data");

    assert!(actor.base_config.is_some());
    assert!(actor.ai_data.is_some());
    assert_eq!(
        actor.factions,
        vec![
            FactionMembership {
                faction_form_id: 0x1100,
                rank: 1,
            },
            FactionMembership {
                faction_form_id: 0x1101,
                rank: 3,
            },
        ]
    );
    assert_eq!(actor.death_item_form_id, Some(0x510));
    assert_eq!(actor.package_form_ids, vec![0x2100, 0x2101]);
    assert_eq!(actor.combat_style_form_id, Some(0x910));
    assert!(actor.race_form_id.is_none());
    assert!(actor.base_stats.is_none());

    let creature = actor
        .creature
        .as_ref()
        .expect("CREA must decode creature data");
    assert_eq!(
        creature.model_list,
        vec![
            "mesh/creature/a.nif".to_string(),
            "mesh/creature/b.nif".to_string()
        ]
    );
    assert_eq!(creature.texture_file_hashes, Some(vec![0xaa, 0xbb, 0xcc]));
    assert_eq!(
        creature.animation_files,
        vec!["anim/idle.kf".to_string(), "anim/attack.kf".to_string()]
    );

    let stats = creature.stats.expect("CREA DATA must decode");
    assert_eq!(stats.creature_type, 1);
    assert_eq!(stats.combat_skill, 20);
    assert_eq!(stats.magic_skill, 0);
    assert_eq!(stats.stealth_skill, 0);
    assert_eq!(stats.health, 300);
    assert_eq!(stats.damage, 25);
    assert_eq!(stats.strength, 8);
    assert_eq!(stats.perception, 5);
    assert_eq!(stats.endurance, 9);
    assert_eq!(stats.charisma, 3);
    assert_eq!(stats.intelligence, 2);
    assert_eq!(stats.agility, 7);
    assert_eq!(stats.luck, 4);

    assert_eq!(creature.attack_reach, Some(2));
    assert_eq!(creature.turning_speed, Some(45.0));
    assert_eq!(creature.base_scale, Some(1.5));
    assert_eq!(creature.foot_weight, Some(2.25));
    assert_eq!(creature.inherits_sounds_from_form_id, Some(0x1200));
    assert_eq!(
        creature.sound_types,
        vec![
            CreatureSoundType {
                sound_type: 4,
                sound_form_ids: vec![0x1300, 0x1301],
                chances: vec![50, 60],
            },
            CreatureSoundType {
                sound_type: 8,
                sound_form_ids: vec![0x1400],
                chances: vec![100],
            },
        ]
    );
    assert!(base.ignored_subrecords.is_empty());
}

#[test]
fn template_flags_are_individually_queryable() {
    let config = ActorBaseConfig {
        flags: 0,
        fatigue: 0,
        barter_gold: 0,
        level_or_mult: 0,
        calc_min_level: 0,
        calc_max_level: 0,
        speed_multiplier: 0,
        karma: 0.0,
        disposition_base: 0,
        template_flags: ActorBaseConfig::TEMPLATE_USE_FACTIONS
            | ActorBaseConfig::TEMPLATE_USE_SCRIPT,
    };
    assert!(config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_FACTIONS));
    assert!(config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_SCRIPT));
    for flag in [
        ActorBaseConfig::TEMPLATE_USE_TRAITS,
        ActorBaseConfig::TEMPLATE_USE_STATS,
        ActorBaseConfig::TEMPLATE_USE_ACTOR_EFFECT_LIST,
        ActorBaseConfig::TEMPLATE_USE_AI_DATA,
        ActorBaseConfig::TEMPLATE_USE_AI_PACKAGES,
        ActorBaseConfig::TEMPLATE_USE_MODEL_ANIMATION,
        ActorBaseConfig::TEMPLATE_USE_BASE_DATA,
        ActorBaseConfig::TEMPLATE_USE_INVENTORY,
    ] {
        assert!(!config.uses_template_flag(flag));
    }
}

// T103.1: an actor FormID authored in a plugin with more than one declared
// master adjusts through that plugin's own master-index table, not just the
// trivial single-master case the other ESM4 tests already cover.
#[test]
fn actor_form_ids_adjust_across_a_multi_master_chain() {
    let npc_id = 0x0200_0777;
    let master = tes4(&[]);
    let extra = tes4(&[]);
    let mut patch = tes4(&["Extra.esm", "Master.esm"]);
    let rnam_raw = 0x0000_0555_u32; // local index 0 -> "Extra.esm"
    let vtck_raw = 0x0100_0666_u32; // local index 1 -> "Master.esm"
    patch.extend(record(
        b"NPC_",
        0,
        npc_id,
        &[
            subrecord(b"RNAM", &rnam_raw.to_le_bytes()),
            subrecord(b"VTCK", &vtck_raw.to_le_bytes()),
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
                name: "Extra.esm",
                bytes: &extra,
            },
            PluginSource {
                name: "Patch.esp",
                bytes: &patch,
            },
        ],
        &CellSelector::FormId(0),
    )
    .unwrap();

    let actor = parsed.bases[&npc_id].actor.as_ref().unwrap();
    // Extra.esm is global index 1.
    assert_eq!(actor.race_form_id, Some(0x0100_0555));
    // Master.esm is global index 0.
    assert_eq!(actor.voice_form_id, Some(0x0000_0666));
}

#[test]
fn load_order_override_replaces_actor_fields() {
    let npc_id = 0x0410;
    let mut master = tes4(&[]);
    master.extend(record(
        b"NPC_",
        0,
        npc_id,
        &[
            subrecord(b"FULL", b"Original\0"),
            subrecord(b"RNAM", &0x100_u32.to_le_bytes()),
        ]
        .concat(),
    ));
    let mut patch = tes4(&["Master.esm"]);
    patch.extend(record(
        b"NPC_",
        0,
        npc_id,
        &[
            subrecord(b"FULL", b"Overridden\0"),
            subrecord(b"RNAM", &0x101_u32.to_le_bytes()),
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
                bytes: &patch,
            },
        ],
        &CellSelector::FormId(0),
    )
    .unwrap();

    let base = &parsed.bases[&npc_id];
    assert_eq!(base.name.as_deref(), Some("Overridden"));
    assert_eq!(base.actor.as_ref().unwrap().race_form_id, Some(0x101));
}

#[test]
fn deleted_actor_record_is_removed() {
    let npc_id = 0x0420;
    let mut master = tes4(&[]);
    master.extend(record(b"NPC_", 0, npc_id, &subrecord(b"FULL", b"Doomed\0")));
    let mut patch = tes4(&["Master.esm"]);
    patch.extend(record(b"NPC_", RECORD_DELETED, npc_id, &[]));

    let parsed = parse_content_set(
        &[
            PluginSource {
                name: "Master.esm",
                bytes: &master,
            },
            PluginSource {
                name: "Patch.esp",
                bytes: &patch,
            },
        ],
        &CellSelector::FormId(0),
    )
    .unwrap();

    assert!(!parsed.bases.contains_key(&npc_id));
}

#[test]
fn truncated_acbs_aidt_data_produce_diagnostics_not_panics() {
    let resolver = direct_resolver();
    let subs = vec![
        direct_subrecord("ACBS", vec![0; 23]), // one byte short of 24
        direct_subrecord("AIDT", vec![0; 19]), // one byte short of 20
        direct_subrecord("DATA", vec![0; 5]),  // short of 11
    ];
    let base = parse_base("NPC_", &subs, &resolver).unwrap();
    let actor = base.actor.as_ref().unwrap();
    assert!(actor.base_config.is_none());
    assert!(actor.ai_data.is_none());
    assert!(actor.base_stats.is_none());
    assert!(
        base.ignored_subrecords
            .iter()
            .any(|message| message.starts_with("ACBS malformed"))
    );
    assert!(
        base.ignored_subrecords
            .iter()
            .any(|message| message.starts_with("AIDT malformed"))
    );
    assert!(
        base.ignored_subrecords
            .iter()
            .any(|message| message.starts_with("DATA malformed"))
    );

    let crea_subs = vec![direct_subrecord("DATA", vec![0; 16])]; // one byte short of 17
    let crea_base = parse_base("CREA", &crea_subs, &resolver).unwrap();
    let crea_actor = crea_base.actor.as_ref().unwrap();
    assert!(crea_actor.creature.as_ref().unwrap().stats.is_none());
    assert!(
        crea_base
            .ignored_subrecords
            .iter()
            .any(|message| message.starts_with("DATA malformed"))
    );
}

#[test]
fn non_actor_records_have_no_actor_data() {
    let resolver = direct_resolver();
    let base = parse_base(
        "MISC",
        &[direct_subrecord("FULL", b"Scrap\0".to_vec())],
        &resolver,
    )
    .unwrap();
    assert!(base.actor.is_none());
}
