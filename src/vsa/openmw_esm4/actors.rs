//! Fallout 3 `NPC_`/`CREA` actor subrecord decoding.
//!
//! Field layouts are verified against the Fallout 3 fopdoc pages
//! (`Records/NPC_.html`, `Records/CREA.html`, and the linked
//! `Records/Subrecords/{ACBS,AIDT,SNAM (CREA, NPC_)}.html` pages) and OpenMW's
//! `components/esm4/{loadnpc,loadcrea,actor}.{hpp,cpp}` (see `NOTICE.md`).
//! Where the two disagree -- notably `AIDT` and `FGGS`/`FGGA`/`FGTS`, where
//! OpenMW's reader only implements the older TES4 layout -- the FO3 fopdoc
//! layout wins, per this task's brief.

use super::*;

/// One `SNAM` faction membership entry (8 bytes: FormID + rank + 3 unused).
/// OpenMW's `ESM4::ActorFaction` (`components/esm4/actor.hpp`) declares
/// `rank` as `std::int8_t`; fopdoc's `SNAM` page describes it as `uint8` but
/// the OpenMW source is the more precise reference for signedness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FactionMembership {
    pub(crate) faction_form_id: u32,
    pub(crate) rank: i8,
}

/// `ACBS` (24 bytes on FO3/FNV): OpenMW's `ESM4::ACBS_FO3`
/// (`components/esm4/actor.hpp`), confirmed against fopdoc's `ACBS` page.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ActorBaseConfig {
    pub(crate) flags: u32,
    pub(crate) fatigue: u16,
    pub(crate) barter_gold: u16,
    pub(crate) level_or_mult: i16,
    pub(crate) calc_min_level: u16,
    pub(crate) calc_max_level: u16,
    pub(crate) speed_multiplier: u16,
    pub(crate) karma: f32,
    pub(crate) disposition_base: i16,
    pub(crate) template_flags: u16,
}

// Named template-flag bits and `uses_template_flag` are consumed by task C's
// template-inheritance resolution (issue #103); unused for now within this
// task's own decode-only scope, matching the `#[allow(dead_code)]` precedent
// already used for other not-yet-consumed records in this file (e.g.
// `SoundRecord`, `MusicRecord`).
#[allow(dead_code)]
impl ActorBaseConfig {
    pub(crate) const TEMPLATE_USE_TRAITS: u16 = 0x0001;
    pub(crate) const TEMPLATE_USE_STATS: u16 = 0x0002;
    pub(crate) const TEMPLATE_USE_FACTIONS: u16 = 0x0004;
    pub(crate) const TEMPLATE_USE_ACTOR_EFFECT_LIST: u16 = 0x0008;
    pub(crate) const TEMPLATE_USE_AI_DATA: u16 = 0x0010;
    pub(crate) const TEMPLATE_USE_AI_PACKAGES: u16 = 0x0020;
    pub(crate) const TEMPLATE_USE_MODEL_ANIMATION: u16 = 0x0040;
    pub(crate) const TEMPLATE_USE_BASE_DATA: u16 = 0x0080;
    pub(crate) const TEMPLATE_USE_INVENTORY: u16 = 0x0100;
    pub(crate) const TEMPLATE_USE_SCRIPT: u16 = 0x0200;

    pub(crate) fn uses_template_flag(self, flag: u16) -> bool {
        self.template_flags & flag != 0
    }
}

/// `AIDT` (20 bytes on FO3/FNV, per fopdoc's `AIDT` page). OpenMW's
/// `ESM4::AIData` (`components/esm4/actor.hpp`) is the older 12-byte TES4
/// layout and its `loadnpc.cpp`/`loadcrea.cpp` readers skip any other size
/// ("FIXME: process the subrecord rather than skip"); this FO3-specific
/// layout is an intentional extension beyond that OpenMW snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ActorAiData {
    pub(crate) aggression: u8,
    pub(crate) confidence: u8,
    pub(crate) energy_level: u8,
    pub(crate) responsibility: u8,
    pub(crate) mood: u8,
    pub(crate) services: u32,
    pub(crate) teaches: i8,
    pub(crate) max_training_level: u8,
    pub(crate) assistance: i8,
    pub(crate) aggro_radius_behavior: u8,
    pub(crate) aggro_radius: i32,
}

#[allow(dead_code)]
impl ActorAiData {
    pub(crate) const AGGRO_RADIUS_BEHAVIOR: u8 = 0x01;
}

/// `NPC_.DATA` (at least 11 bytes: base health + the 7 S.P.E.C.I.A.L.
/// stats). Older record versions append trailing unused padding, which is
/// tolerated and ignored.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct NpcBaseStats {
    pub(crate) base_health: i32,
    pub(crate) strength: u8,
    pub(crate) perception: u8,
    pub(crate) endurance: u8,
    pub(crate) charisma: u8,
    pub(crate) intelligence: u8,
    pub(crate) agility: u8,
    pub(crate) luck: u8,
}

/// Number of named FO3 skills in `NPC_.DNAM` (Barter, Big Guns, Energy
/// Weapons, Explosives, Lockpick, Medicine, Melee Weapons, Repair, Science,
/// Small Guns, Sneak, Speech, Throwing, Unarmed).
pub(crate) const NPC_SKILL_COUNT: usize = 14;

/// `NPC_.DNAM` (28 bytes: 14 skill values followed by 14 skill offsets, in
/// the skill order documented on fopdoc's `NPC_` page).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NpcSkills {
    pub(crate) values: [u8; NPC_SKILL_COUNT],
    pub(crate) offsets: [u8; NPC_SKILL_COUNT],
}

/// `CREA.DATA` (at least 17 bytes).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct CreatureStats {
    pub(crate) creature_type: u8,
    pub(crate) combat_skill: u8,
    pub(crate) magic_skill: u8,
    pub(crate) stealth_skill: u8,
    pub(crate) health: i16,
    pub(crate) damage: i16,
    pub(crate) strength: u8,
    pub(crate) perception: u8,
    pub(crate) endurance: u8,
    pub(crate) charisma: u8,
    pub(crate) intelligence: u8,
    pub(crate) agility: u8,
    pub(crate) luck: u8,
}

/// One `CSDT`-delimited block of inherited creature sounds: the sound-type
/// enum value plus its `CSDI` (`SOUN` FormID) / `CSDC` (chance byte) entries,
/// kept as parallel lists rather than modelling the pairing precisely.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct CreatureSoundType {
    pub(crate) sound_type: u32,
    pub(crate) sound_form_ids: Vec<u32>,
    pub(crate) chances: Vec<u8>,
}

/// `CREA`-only fields (issue #103, M4 wave 1 task A).
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct CreatureData {
    /// `NIFZ`: zero-terminated array of model filenames.
    pub(crate) model_list: Vec<String>,
    /// `NIFT`: opaque texture-file-hash bytes.
    pub(crate) texture_file_hashes: Option<Vec<u8>>,
    /// `KFFZ`: zero-terminated array of animation filenames.
    pub(crate) animation_files: Vec<String>,
    pub(crate) stats: Option<CreatureStats>,
    /// `RNAM`: attack reach (distinct from `NPC_.RNAM`, which is race).
    pub(crate) attack_reach: Option<u8>,
    /// `TNAM`.
    pub(crate) turning_speed: Option<f32>,
    /// `BNAM`.
    pub(crate) base_scale: Option<f32>,
    /// `WNAM`.
    pub(crate) foot_weight: Option<f32>,
    /// `CSCR`: another `CREA` record this one inherits sounds from.
    pub(crate) inherits_sounds_from_form_id: Option<u32>,
    pub(crate) sound_types: Vec<CreatureSoundType>,
}

/// Decoded `NPC_`/`CREA` actor subrecords, attached to `BaseRecord::actor`.
/// Fields not applicable to a given record kind (e.g. `skills` for `CREA`,
/// `creature` for `NPC_`) stay at their default.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ActorData {
    pub(crate) base_config: Option<ActorBaseConfig>,
    /// `SNAM` (repeated).
    pub(crate) factions: Vec<FactionMembership>,
    /// `INAM`.
    pub(crate) death_item_form_id: Option<u32>,
    /// `VTCK`. `NPC_` only.
    pub(crate) voice_form_id: Option<u32>,
    /// `RNAM`. `NPC_` only (`CREA`'s `RNAM` is attack reach, see
    /// `CreatureData::attack_reach`).
    pub(crate) race_form_id: Option<u32>,
    /// `EITM`. `NPC_` only.
    pub(crate) unarmed_attack_effect_form_id: Option<u32>,
    /// `SCRI`. `NPC_` only.
    pub(crate) script_form_id: Option<u32>,
    pub(crate) ai_data: Option<ActorAiData>,
    /// `PKID` (repeated).
    pub(crate) package_form_ids: Vec<u32>,
    /// `CNAM`. `NPC_` only.
    pub(crate) class_form_id: Option<u32>,
    /// `NPC_.DATA`. `NPC_` only (`CREA`'s `DATA` is `CreatureData::stats`).
    pub(crate) base_stats: Option<NpcBaseStats>,
    /// `NPC_.DNAM`. `NPC_` only.
    pub(crate) skills: Option<NpcSkills>,
    /// `PNAM` (repeated). `NPC_` only.
    pub(crate) head_part_form_ids: Vec<u32>,
    /// `HNAM`. `NPC_` only.
    pub(crate) hair_form_id: Option<u32>,
    /// `ENAM`. `NPC_` only.
    pub(crate) eyes_form_id: Option<u32>,
    /// `LNAM`. `NPC_` only.
    pub(crate) hair_length: Option<f32>,
    /// `HCLR`. `NPC_` only.
    pub(crate) hair_color_rgba: Option<[f32; 4]>,
    /// `ZNAM`.
    pub(crate) combat_style_form_id: Option<u32>,
    /// `NAM6`. `NPC_` only.
    pub(crate) height: Option<f32>,
    /// `NAM7`. `NPC_` only.
    pub(crate) weight: Option<f32>,
    /// `FGGS`, opaque per this task's brief (FO3 documents it as `uint8[]`,
    /// unlike OpenMW's TES4/TES5 50-`float32` reader).
    pub(crate) facegen_geometry_symmetric: Option<Vec<u8>>,
    /// `FGGA`, opaque; see `facegen_geometry_symmetric`.
    pub(crate) facegen_geometry_asymmetric: Option<Vec<u8>>,
    /// `FGTS`, opaque; see `facegen_geometry_symmetric`.
    pub(crate) facegen_texture_symmetric: Option<Vec<u8>>,
    /// `CREA`-only fields.
    pub(crate) creature: Option<CreatureData>,
}

/// Subrecord signatures this module decodes for `sig`, so `parse_base`'s
/// unsupported-signature diagnostics do not flag them. Empty for any other
/// record kind.
pub(crate) fn actor_supported_signatures(sig: &str) -> &'static [&'static str] {
    match sig {
        "NPC_" => &[
            "ACBS", "SNAM", "INAM", "VTCK", "RNAM", "EITM", "SCRI", "AIDT", "PKID", "CNAM", "DATA",
            "DNAM", "PNAM", "HNAM", "ENAM", "LNAM", "HCLR", "ZNAM", "NAM6", "NAM7", "FGGS", "FGGA",
            "FGTS",
        ],
        "CREA" => &[
            "NIFZ", "NIFT", "KFFZ", "ACBS", "SNAM", "INAM", "PKID", "AIDT", "DATA", "RNAM", "ZNAM",
            "TNAM", "BNAM", "WNAM", "CSCR", "CSDT", "CSDI", "CSDC",
        ],
        _ => &[],
    }
}

/// Decodes `NPC_`/`CREA` actor subrecords, returning `None` for any other
/// record kind. The second tuple element carries stable diagnostics for
/// truncated/malformed `ACBS`/`AIDT`/`DATA` payloads (never a panic); they
/// are merged into `BaseRecord::ignored_subrecords` by the caller.
pub(crate) fn parse_actor(
    sig: &str,
    subs: &[Subrecord],
    resolver: &FormIdResolver,
) -> Option<(ActorData, Vec<String>)> {
    match sig {
        "NPC_" => Some(parse_npc(subs, resolver)),
        "CREA" => Some(parse_creature(subs, resolver)),
        _ => None,
    }
}

fn parse_npc(subs: &[Subrecord], resolver: &FormIdResolver) -> (ActorData, Vec<String>) {
    let mut diagnostics = Vec::new();
    let base_config = decode(sub(subs, "ACBS"), "ACBS", parse_acbs, &mut diagnostics);
    let ai_data = decode(sub(subs, "AIDT"), "AIDT", parse_aidt, &mut diagnostics);
    let base_stats = decode(sub(subs, "DATA"), "DATA", parse_npc_data, &mut diagnostics);
    let skills = decode(
        sub(subs, "DNAM"),
        "DNAM",
        parse_npc_skills,
        &mut diagnostics,
    );

    let actor = ActorData {
        base_config,
        factions: parse_factions(subs, resolver),
        death_item_form_id: sub_form_id(subs, "INAM", resolver),
        voice_form_id: sub_form_id(subs, "VTCK", resolver),
        race_form_id: sub_form_id(subs, "RNAM", resolver),
        unarmed_attack_effect_form_id: sub_form_id(subs, "EITM", resolver),
        script_form_id: sub_form_id(subs, "SCRI", resolver),
        ai_data,
        package_form_ids: form_id_list(subs, "PKID", resolver),
        class_form_id: sub_form_id(subs, "CNAM", resolver),
        base_stats,
        skills,
        head_part_form_ids: form_id_list(subs, "PNAM", resolver),
        hair_form_id: sub_form_id(subs, "HNAM", resolver),
        eyes_form_id: sub_form_id(subs, "ENAM", resolver),
        hair_length: sub(subs, "LNAM").and_then(|data| f32_at_option(data, 0)),
        hair_color_rgba: sub(subs, "HCLR").filter(|data| data.len() >= 4).map(rgba8),
        combat_style_form_id: sub_form_id(subs, "ZNAM", resolver),
        height: sub(subs, "NAM6").and_then(|data| f32_at_option(data, 0)),
        weight: sub(subs, "NAM7").and_then(|data| f32_at_option(data, 0)),
        facegen_geometry_symmetric: sub(subs, "FGGS").map(<[u8]>::to_vec),
        facegen_geometry_asymmetric: sub(subs, "FGGA").map(<[u8]>::to_vec),
        facegen_texture_symmetric: sub(subs, "FGTS").map(<[u8]>::to_vec),
        creature: None,
    };
    (actor, diagnostics)
}

fn parse_creature(subs: &[Subrecord], resolver: &FormIdResolver) -> (ActorData, Vec<String>) {
    let mut diagnostics = Vec::new();
    let base_config = decode(sub(subs, "ACBS"), "ACBS", parse_acbs, &mut diagnostics);
    let ai_data = decode(sub(subs, "AIDT"), "AIDT", parse_aidt, &mut diagnostics);
    let stats = decode(
        sub(subs, "DATA"),
        "DATA",
        parse_creature_data,
        &mut diagnostics,
    );

    let creature = CreatureData {
        model_list: sub(subs, "NIFZ").map(cstring_list).unwrap_or_default(),
        texture_file_hashes: sub(subs, "NIFT").map(<[u8]>::to_vec),
        animation_files: sub(subs, "KFFZ").map(cstring_list).unwrap_or_default(),
        stats,
        attack_reach: sub(subs, "RNAM").and_then(|data| data.first().copied()),
        turning_speed: sub(subs, "TNAM").and_then(|data| f32_at_option(data, 0)),
        base_scale: sub(subs, "BNAM").and_then(|data| f32_at_option(data, 0)),
        foot_weight: sub(subs, "WNAM").and_then(|data| f32_at_option(data, 0)),
        inherits_sounds_from_form_id: sub_form_id(subs, "CSCR", resolver),
        sound_types: parse_creature_sound_types(subs, resolver),
    };
    let actor = ActorData {
        base_config,
        factions: parse_factions(subs, resolver),
        death_item_form_id: sub_form_id(subs, "INAM", resolver),
        ai_data,
        package_form_ids: form_id_list(subs, "PKID", resolver),
        combat_style_form_id: sub_form_id(subs, "ZNAM", resolver),
        creature: Some(creature),
        ..ActorData::default()
    };
    (actor, diagnostics)
}

/// Runs `parser` over `data` (when present), pushing a
/// `"<signature> malformed: <reason>"` diagnostic on failure instead of
/// propagating a panic.
fn decode<T>(
    data: Option<&[u8]>,
    signature: &str,
    parser: fn(&[u8]) -> Result<T, String>,
    diagnostics: &mut Vec<String>,
) -> Option<T> {
    match data.map(parser) {
        Some(Ok(value)) => Some(value),
        Some(Err(error)) => {
            diagnostics.push(format!("{signature} malformed: {error}"));
            None
        }
        None => None,
    }
}

fn parse_factions(subs: &[Subrecord], resolver: &FormIdResolver) -> Vec<FactionMembership> {
    subs.iter()
        .filter(|subrecord| subrecord.signature == "SNAM")
        .filter_map(|subrecord| {
            let data = &subrecord.data;
            let raw = u32::from_le_bytes(data.get(..4)?.try_into().ok()?);
            let form_id = resolver.adjust(raw);
            let rank = *data.get(4)? as i8;
            (form_id != 0).then_some(FactionMembership {
                faction_form_id: form_id,
                rank,
            })
        })
        .collect()
}

fn form_id_list(subs: &[Subrecord], signature: &str, resolver: &FormIdResolver) -> Vec<u32> {
    subs.iter()
        .filter(|subrecord| subrecord.signature == signature)
        .filter_map(|subrecord| subrecord.data.get(..4))
        .map(|data| resolver.adjust(u32::from_le_bytes(data.try_into().unwrap())))
        .filter(|id| *id != 0)
        .collect()
}

/// Splits a `NIFZ`/`KFFZ`-style zero-terminated string array (OpenMW's
/// `Reader::getZeroTerminatedStringArray`) into its component filenames.
fn cstring_list(data: &[u8]) -> Vec<String> {
    data.split(|&byte| byte == 0)
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .filter(|value| !value.is_empty())
        .collect()
}

fn u16_at(data: &[u8], offset: usize) -> Option<u16> {
    data.get(offset..offset + 2)
        .map(|bytes| u16::from_le_bytes(bytes.try_into().unwrap()))
}

fn parse_acbs(data: &[u8]) -> Result<ActorBaseConfig, String> {
    if data.len() != 24 {
        return Err(format!("expected 24 bytes, got {}", data.len()));
    }
    Ok(ActorBaseConfig {
        flags: u32_at(data, 0).unwrap_or_default(),
        fatigue: u16_at(data, 4).unwrap_or_default(),
        barter_gold: u16_at(data, 6).unwrap_or_default(),
        level_or_mult: i16_at(data, 8).unwrap_or_default(),
        calc_min_level: u16_at(data, 10).unwrap_or_default(),
        calc_max_level: u16_at(data, 12).unwrap_or_default(),
        speed_multiplier: u16_at(data, 14).unwrap_or_default(),
        karma: f32_at_option(data, 16).unwrap_or_default(),
        disposition_base: i16_at(data, 20).unwrap_or_default(),
        template_flags: u16_at(data, 22).unwrap_or_default(),
    })
}

fn parse_aidt(data: &[u8]) -> Result<ActorAiData, String> {
    if data.len() != 20 {
        return Err(format!("expected 20 bytes, got {}", data.len()));
    }
    Ok(ActorAiData {
        aggression: data[0],
        confidence: data[1],
        energy_level: data[2],
        responsibility: data[3],
        mood: data[4],
        services: u32_at(data, 8).unwrap_or_default(),
        teaches: data[12] as i8,
        max_training_level: data[13],
        assistance: data[14] as i8,
        aggro_radius_behavior: data[15],
        aggro_radius: i32_at(data, 16).unwrap_or_default(),
    })
}

fn parse_npc_data(data: &[u8]) -> Result<NpcBaseStats, String> {
    if data.len() < 11 {
        return Err(format!("expected at least 11 bytes, got {}", data.len()));
    }
    Ok(NpcBaseStats {
        base_health: i32_at(data, 0).unwrap_or_default(),
        strength: data[4],
        perception: data[5],
        endurance: data[6],
        charisma: data[7],
        intelligence: data[8],
        agility: data[9],
        luck: data[10],
    })
}

fn parse_npc_skills(data: &[u8]) -> Result<NpcSkills, String> {
    if data.len() != NPC_SKILL_COUNT * 2 {
        return Err(format!(
            "expected {} bytes, got {}",
            NPC_SKILL_COUNT * 2,
            data.len()
        ));
    }
    let mut values = [0_u8; NPC_SKILL_COUNT];
    let mut offsets = [0_u8; NPC_SKILL_COUNT];
    values.copy_from_slice(&data[..NPC_SKILL_COUNT]);
    offsets.copy_from_slice(&data[NPC_SKILL_COUNT..]);
    Ok(NpcSkills { values, offsets })
}

fn parse_creature_data(data: &[u8]) -> Result<CreatureStats, String> {
    if data.len() < 17 {
        return Err(format!("expected at least 17 bytes, got {}", data.len()));
    }
    Ok(CreatureStats {
        creature_type: data[0],
        combat_skill: data[1],
        magic_skill: data[2],
        stealth_skill: data[3],
        health: i16_at(data, 4).unwrap_or_default(),
        damage: i16_at(data, 8).unwrap_or_default(),
        strength: data[10],
        perception: data[11],
        endurance: data[12],
        charisma: data[13],
        intelligence: data[14],
        agility: data[15],
        luck: data[16],
    })
}

/// Groups `CSDT` (sound-type enum) with the `CSDI`/`CSDC` entries that
/// follow it, per fopdoc's CREA "Sound Type Subrecord Collection".
fn parse_creature_sound_types(
    subs: &[Subrecord],
    resolver: &FormIdResolver,
) -> Vec<CreatureSoundType> {
    let mut result = Vec::new();
    let mut current: Option<CreatureSoundType> = None;
    for subrecord in subs {
        match subrecord.signature.as_str() {
            "CSDT" => {
                if let Some(block) = current.take() {
                    result.push(block);
                }
                if let Some(sound_type) = u32_at(&subrecord.data, 0) {
                    current = Some(CreatureSoundType {
                        sound_type,
                        sound_form_ids: Vec::new(),
                        chances: Vec::new(),
                    });
                }
            }
            "CSDI" => {
                if let Some(block) = current.as_mut()
                    && let Some(bytes) = subrecord.data.get(..4)
                {
                    let form_id = resolver.adjust(u32::from_le_bytes(bytes.try_into().unwrap()));
                    if form_id != 0 {
                        block.sound_form_ids.push(form_id);
                    }
                }
            }
            "CSDC" => {
                if let Some(block) = current.as_mut()
                    && let Some(&chance) = subrecord.data.first()
                {
                    block.chances.push(chance);
                }
            }
            _ => {}
        }
    }
    if let Some(block) = current.take() {
        result.push(block);
    }
    result
}
