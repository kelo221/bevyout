//! Fallout 3 `RACE`/`CLAS`/`FACT`/`PACK` supporting-record decoding for the
//! M4 actor catalog (issue #103, M4 wave 1 task B).
//!
//! `RACE`/`CLAS`/`PACK` byte layouts are verified against the fopdoc
//! Fallout 3 pages and adapted from OpenMW's
//! `components/esm4/loadrace.cpp`/`loadclas.cpp`/`loadpack.cpp` where OpenMW
//! actually decodes the field; several FO3 fields these loaders skip or stub
//! are this project's own fopdoc-sourced additions layered onto OpenMW's
//! traversal approach. `FACT` has no OpenMW ESM4 loader at all and is
//! decoded purely from the fopdoc Fallout 3 FACT page. See NOTICE.md.

use super::*;

// ---------------------------------------------------------------------
// RACE
// ---------------------------------------------------------------------

/// One head/body part model reference: the `INDX` value paired with the
/// `MODL` mesh path that follows it, if any. Per-part `ICON`/`MICO` and the
/// rest of the `MODL`/`MODB`/`MODT`/`MODS`/`MODD` model-data collection are
/// intentionally not retained; the M4 catalog only needs a deterministic
/// index -> model path mapping (M4_WAVE1_PLAN.md task B).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RacePartEntry {
    pub(crate) index: u32,
    pub(crate) model_path: Option<String>,
}

/// Opaque FaceGen coefficient bytes for one sex: `FGGS`/`FGGA`/`FGTS`. Not
/// decoded -- a later facial-generation feature would need to give them
/// semantics; retained losslessly in the meantime.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct RaceFaceGenData {
    pub(crate) geometry_symmetric: Option<Vec<u8>>,
    pub(crate) geometry_asymmetric: Option<Vec<u8>>,
    pub(crate) texture_symmetric: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RaceSkillBoost {
    pub(crate) actor_value: i8,
    pub(crate) boost: i8,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RaceRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) male_height: f32,
    pub(crate) female_height: f32,
    pub(crate) male_weight: f32,
    pub(crate) female_weight: f32,
    pub(crate) flags: u32,
    /// Seven signed `(actor value id, boost)` pairs from `RACE.DATA`.
    /// Zero boosts are omitted; unknown non-zero IDs are retained so the
    /// preparation boundary can emit a stable unsupported-value diagnostic.
    pub(crate) skill_boosts: Vec<RaceSkillBoost>,
    pub(crate) older_race_form_id: Option<u32>,
    pub(crate) younger_race_form_id: Option<u32>,
    pub(crate) male_default_hair_form_id: Option<u32>,
    pub(crate) female_default_hair_form_id: Option<u32>,
    pub(crate) male_default_hair_color: Option<u8>,
    pub(crate) female_default_hair_color: Option<u8>,
    pub(crate) hair_form_ids: Vec<u32>,
    pub(crate) eye_form_ids: Vec<u32>,
    pub(crate) head_parts_male: Vec<RacePartEntry>,
    pub(crate) head_parts_female: Vec<RacePartEntry>,
    pub(crate) body_parts_male: Vec<RacePartEntry>,
    pub(crate) body_parts_female: Vec<RacePartEntry>,
    pub(crate) face_gen_male: RaceFaceGenData,
    pub(crate) face_gen_female: RaceFaceGenData,
    pub(crate) ignored_subrecords: Vec<String>,
}

/// RACE head/body part collections are introduced by `NAM0`/`NAM1` and their
/// sex by the (repeated, context-dependent) `MNAM`/`FNAM` markers, tracked
/// positionally rather than through `sub()` lookups -- OpenMW's
/// `ESM4::Race::load` tracks the same state with `currPart`/`isMale`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RacePartSection {
    Head,
    Body,
}

/// Decodes a Fallout 3 `RACE` record. `DATA` is required and must be the
/// documented 36-byte TES4/FO3/FONV layout (OpenMW `loadrace.cpp`'s `DATA`
/// case, `subHdr.dataSize == 36`): 8 skill-boost pairs (16 bytes, not needed
/// by this catalog) followed by the four height/weight floats and the flags
/// u32. `ONAM`/`YNAM`/`VTCK`/`NAM2` are skipped outright by OpenMW's FO3
/// path (grouped with its `skipSubRecordData()` cases); `ONAM`/`YNAM` are
/// decoded here directly from fopdoc's documented plain-FormID layout.
pub(crate) fn parse_race(
    subs: &[Subrecord],
    form_id: u32,
    record_flags: u32,
    resolver: &FormIdResolver,
) -> std::result::Result<RaceRecord, String> {
    let data = sub(subs, "DATA").ok_or_else(|| "missing DATA subrecord".to_string())?;
    if data.len() != 36 {
        return Err(format!("DATA must be exactly 36 bytes, got {}", data.len()));
    }
    // Bytes 0..16 are seven signed skill-boost pairs plus two unused bytes.
    // OpenMW models the prefix as eight pairs, but fopdoc identifies the last
    // pair as padding; retaining only the documented seven avoids inventing
    // an eighth actor-value modifier.
    let skill_boosts = data[..14]
        .chunks_exact(2)
        .filter_map(|pair| {
            let boost = pair[1] as i8;
            (boost != 0).then_some(RaceSkillBoost {
                actor_value: pair[0] as i8,
                boost,
            })
        })
        .collect();
    let male_height = f32_at_option(data, 16).ok_or_else(|| "truncated DATA".to_string())?;
    let female_height = f32_at_option(data, 20).ok_or_else(|| "truncated DATA".to_string())?;
    let male_weight = f32_at_option(data, 24).ok_or_else(|| "truncated DATA".to_string())?;
    let female_weight = f32_at_option(data, 28).ok_or_else(|| "truncated DATA".to_string())?;
    let flags = u32_at(data, 32).ok_or_else(|| "truncated DATA".to_string())?;

    let mut older_race_form_id = None;
    let mut younger_race_form_id = None;
    let mut male_default_hair_form_id = None;
    let mut female_default_hair_form_id = None;
    let mut male_default_hair_color = None;
    let mut female_default_hair_color = None;
    let mut hair_form_ids = Vec::new();
    let mut eye_form_ids = Vec::new();
    let mut head_parts_male = Vec::new();
    let mut head_parts_female = Vec::new();
    let mut body_parts_male = Vec::new();
    let mut body_parts_female = Vec::new();
    let mut face_gen_male = RaceFaceGenData::default();
    let mut face_gen_female = RaceFaceGenData::default();

    let mut section: Option<RacePartSection> = None;
    let mut is_male = true;
    let mut has_current_index = false;

    for subrecord in subs {
        match subrecord.signature.as_str() {
            "ONAM" => {
                older_race_form_id = u32_at(&subrecord.data, 0)
                    .map(|raw| resolver.adjust(raw))
                    .filter(|id| *id != 0);
            }
            "YNAM" => {
                younger_race_form_id = u32_at(&subrecord.data, 0)
                    .map(|raw| resolver.adjust(raw))
                    .filter(|id| *id != 0);
            }
            "DNAM" if subrecord.data.len() >= 8 => {
                male_default_hair_form_id = u32_at(&subrecord.data, 0)
                    .map(|raw| resolver.adjust(raw))
                    .filter(|id| *id != 0);
                female_default_hair_form_id = u32_at(&subrecord.data, 4)
                    .map(|raw| resolver.adjust(raw))
                    .filter(|id| *id != 0);
            }
            "CNAM" if subrecord.data.len() >= 2 => {
                male_default_hair_color = subrecord.data.first().copied();
                female_default_hair_color = subrecord.data.get(1).copied();
            }
            "HNAM" => {
                hair_form_ids = subrecord
                    .data
                    .chunks_exact(4)
                    .map(|chunk| resolver.adjust(u32::from_le_bytes(chunk.try_into().unwrap())))
                    .collect();
            }
            "ENAM" => {
                eye_form_ids = subrecord
                    .data
                    .chunks_exact(4)
                    .map(|chunk| resolver.adjust(u32::from_le_bytes(chunk.try_into().unwrap())))
                    .collect();
            }
            "NAM0" => {
                section = Some(RacePartSection::Head);
                has_current_index = false;
            }
            "NAM1" => {
                section = Some(RacePartSection::Body);
                has_current_index = false;
            }
            "MNAM" => is_male = true,
            "FNAM" => is_male = false,
            "INDX" => {
                let Some(section) = section else { continue };
                let Some(index) = u32_at(&subrecord.data, 0) else {
                    continue;
                };
                has_current_index = true;
                let entry = RacePartEntry {
                    index,
                    model_path: None,
                };
                let list = match (section, is_male) {
                    (RacePartSection::Head, true) => &mut head_parts_male,
                    (RacePartSection::Head, false) => &mut head_parts_female,
                    (RacePartSection::Body, true) => &mut body_parts_male,
                    (RacePartSection::Body, false) => &mut body_parts_female,
                };
                list.push(entry);
            }
            "MODL" => {
                let Some(section) = section else { continue };
                if !has_current_index {
                    continue;
                }
                let list = match (section, is_male) {
                    (RacePartSection::Head, true) => &mut head_parts_male,
                    (RacePartSection::Head, false) => &mut head_parts_female,
                    (RacePartSection::Body, true) => &mut body_parts_male,
                    (RacePartSection::Body, false) => &mut body_parts_female,
                };
                if let Some(last) = list.last_mut() {
                    last.model_path = Some(cstring(&subrecord.data));
                }
            }
            "FGGS" => {
                let target = if is_male {
                    &mut face_gen_male
                } else {
                    &mut face_gen_female
                };
                target.geometry_symmetric = Some(subrecord.data.clone());
            }
            "FGGA" => {
                let target = if is_male {
                    &mut face_gen_male
                } else {
                    &mut face_gen_female
                };
                target.geometry_asymmetric = Some(subrecord.data.clone());
            }
            "FGTS" => {
                let target = if is_male {
                    &mut face_gen_male
                } else {
                    &mut face_gen_female
                };
                target.texture_symmetric = Some(subrecord.data.clone());
            }
            _ => {}
        }
    }

    Ok(RaceRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        name: sub(subs, "FULL").map(cstring),
        male_height,
        female_height,
        male_weight,
        female_weight,
        flags,
        skill_boosts,
        older_race_form_id,
        younger_race_form_id,
        male_default_hair_form_id,
        female_default_hair_form_id,
        male_default_hair_color,
        female_default_hair_color,
        hair_form_ids,
        eye_form_ids,
        head_parts_male,
        head_parts_female,
        body_parts_male,
        body_parts_female,
        face_gen_male,
        face_gen_female,
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "FULL", "DATA", "ONAM", "YNAM", "DNAM", "CNAM", "HNAM", "ENAM", "NAM0",
                "NAM1", "MNAM", "FNAM", "INDX", "MODL", "ICON", "MICO", "MODB", "MODT", "MODS",
                "MODD", "FGGS", "FGGA", "FGTS",
            ],
        ),
    })
}

// ---------------------------------------------------------------------
// CLAS
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ClassRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) tag_skills: [i32; 4],
    pub(crate) flags: u32,
    pub(crate) services: u32,
    pub(crate) teaches: i8,
    pub(crate) max_training_level: u8,
    pub(crate) ignored_subrecords: Vec<String>,
}

/// Decodes a Fallout 3 `CLAS` record. OpenMW's `ESM4::Class::load` (see
/// `loadclas.cpp`) calls `reader.skipSubRecordData()` for `DATA`/`ATTR`/
/// `PRPS` outright, so this 28-byte FO3 `DATA` layout (four tag-skill
/// `Actor Value` int32s, flags u32, services u32, teaches int8, max
/// training level u8, 2 unused bytes) is decoded purely from the fopdoc
/// Fallout3 CLAS page.
pub(crate) fn parse_class(
    subs: &[Subrecord],
    form_id: u32,
    record_flags: u32,
) -> std::result::Result<ClassRecord, String> {
    let data = sub(subs, "DATA").ok_or_else(|| "missing DATA subrecord".to_string())?;
    if data.len() != 28 {
        return Err(format!("DATA must be exactly 28 bytes, got {}", data.len()));
    }
    let tag_skill = |offset: usize| i32_at(data, offset).unwrap();
    Ok(ClassRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        name: sub(subs, "FULL").map(cstring),
        tag_skills: [tag_skill(0), tag_skill(4), tag_skill(8), tag_skill(12)],
        flags: u32_at(data, 16).unwrap(),
        services: u32_at(data, 20).unwrap(),
        teaches: data[24] as i8,
        max_training_level: data[25],
        ignored_subrecords: ignored_signatures(
            subs,
            &["EDID", "FULL", "DESC", "ICON", "MICO", "DATA", "ATTR"],
        ),
    })
}

// ---------------------------------------------------------------------
// FACT
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FactionRelation {
    pub(crate) faction_form_id: u32,
    pub(crate) modifier: i32,
    pub(crate) group_combat_reaction: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct FactionRank {
    pub(crate) rank_number: i32,
    pub(crate) male_title: Option<String>,
    pub(crate) female_title: Option<String>,
    pub(crate) insignia: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FactionRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) flags1: u8,
    pub(crate) flags2: u8,
    pub(crate) relations: Vec<FactionRelation>,
    pub(crate) ranks: Vec<FactionRank>,
    pub(crate) ignored_subrecords: Vec<String>,
}

/// Decodes a Fallout 3 `FACT` record. OpenMW has no `FACT` ESM4 loader at
/// all (no `loadfact.cpp`/`.hpp` in the supplied snapshot); every field here
/// is decoded purely from the fopdoc Fallout3 FACT page.
///
/// `DATA` is documented optional (fopdoc's blank `Count` column, unlike
/// `RACE`/`CLAS`/`PACK`'s required `+`), so a missing `DATA` defaults both
/// flag bytes to zero rather than failing the record; a `DATA` that is
/// present but too short to hold even the two flag bytes is malformed.
///
/// fopdoc's FO3 FACT page documents no crime-related subrecords (`DATA.CNAM`
/// is "Unused float32"; there is no `CRVA`-style crime-gold subrecord as in
/// later games), so none are decoded here -- there is nothing present to
/// decode.
pub(crate) fn parse_faction(
    subs: &[Subrecord],
    form_id: u32,
    record_flags: u32,
    resolver: &FormIdResolver,
) -> std::result::Result<FactionRecord, String> {
    let (flags1, flags2) = match sub(subs, "DATA") {
        Some(data) if data.len() >= 2 => (data[0], data[1]),
        Some(data) => {
            return Err(format!("DATA must be at least 2 bytes, got {}", data.len()));
        }
        None => (0, 0),
    };

    let relations = subs
        .iter()
        .filter(|subrecord| subrecord.signature == "XNAM")
        .filter_map(|subrecord| {
            let data = &subrecord.data;
            if data.len() < 12 {
                return None;
            }
            Some(FactionRelation {
                faction_form_id: resolver.adjust(u32_at(data, 0)?),
                modifier: i32_at(data, 4)?,
                group_combat_reaction: u32_at(data, 8)?,
            })
        })
        .collect();

    let mut ranks: Vec<FactionRank> = Vec::new();
    for subrecord in subs {
        match subrecord.signature.as_str() {
            "RNAM" => {
                let Some(rank_number) = i32_at(&subrecord.data, 0) else {
                    continue;
                };
                ranks.push(FactionRank {
                    rank_number,
                    ..Default::default()
                });
            }
            "MNAM" => {
                if let Some(rank) = ranks.last_mut() {
                    rank.male_title = Some(cstring(&subrecord.data));
                }
            }
            "FNAM" => {
                if let Some(rank) = ranks.last_mut() {
                    rank.female_title = Some(cstring(&subrecord.data));
                }
            }
            "INAM" => {
                if let Some(rank) = ranks.last_mut() {
                    rank.insignia = Some(cstring(&subrecord.data));
                }
            }
            _ => {}
        }
    }

    Ok(FactionRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        name: sub(subs, "FULL").map(cstring),
        flags1,
        flags2,
        relations,
        ranks,
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "FULL", "XNAM", "DATA", "CNAM", "RNAM", "MNAM", "FNAM", "INAM",
            ],
        ),
    })
}

// ---------------------------------------------------------------------
// PACK
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PackageSchedule {
    pub(crate) month: i8,
    pub(crate) day_of_week: i8,
    pub(crate) date: u8,
    pub(crate) time: i8,
    pub(crate) duration: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PackageLocation {
    pub(crate) location_type: u32,
    /// Resolved only when `location_type != 5` ("Object Type" enum, not a
    /// FormID per fopdoc) and the raw value is non-zero.
    pub(crate) form_id: Option<u32>,
    pub(crate) raw_value: u32,
    pub(crate) radius: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PackageTarget {
    pub(crate) target_type: i32,
    /// Resolved only when `target_type != 2` ("Object Type" enum, not a
    /// FormID per fopdoc) and the raw value is non-zero.
    pub(crate) form_id: Option<u32>,
    pub(crate) raw_value: u32,
    pub(crate) count_or_distance: i32,
    pub(crate) unknown: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PackageRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) general_flags: u32,
    pub(crate) package_type: u8,
    pub(crate) location: Option<PackageLocation>,
    pub(crate) schedule: Option<PackageSchedule>,
    pub(crate) target: Option<PackageTarget>,
    /// Raw `CTDA` payloads, retained opaquely in stream order exactly like
    /// `RecipeRecord::conditions` -- no condition evaluator exists yet.
    pub(crate) conditions: Vec<Vec<u8>>,
    pub(crate) ignored_subrecords: Vec<String>,
}

/// Decodes a Fallout 3 `PACK` (AI package) record.
///
/// `PKDT` is required. OpenMW's own `AIPackage::load` (`loadpack.cpp`) only
/// special-cases a legacy 4-byte "flags only" `PKDT` (`dataSize == 4`,
/// `type` defaulted to 0); for every other size, including FO3's documented
/// 12-byte layout, it calls `reader.skipSubRecordData()` (marked
/// `// FIXME: FO3` in the source) -- OpenMW does not actually decode FO3
/// `PKDT`. The 12-byte decode below (general flags u32, type u8, 1 unused
/// byte, fallout-behaviour flags u16, type-specific flags u16, 2 unused
/// bytes) is this project's own addition from the fopdoc Fallout3 PACK page.
///
/// `PLDT`/`PSDT`/`PTDT` are decoded using fopdoc's FO3 sizes (12/8/16 bytes
/// respectively), which differ from OpenMW's TES4-only structs in
/// `loadpack.hpp` (`PLDT` there has no extra field but matches at 12 bytes;
/// `PTDT` there is 12 bytes without the trailing FO3 `Unknown` float32) --
/// per this task's "FO3 wins" rule. `PLDT.form_id` is gated on
/// `location_type != 5`, matching OpenMW's own `PLDT` handling
/// (`if (mLocation.type != 5) reader.adjustFormId(mLocation.location);`).
/// `PTDT.form_id` is gated on `target_type != 2`: OpenMW's own `PTDT` case
/// gates this on `mLocation.type` instead of `mTarget.type`, which looks
/// like a copy-paste bug (`PLDT`'s `Type` value 5 and `PTDT`'s `Type` value
/// 2 are unrelated "Object Type" enums, from unrelated subrecords) -- not
/// reproduced here; this port gates on `PTDT`'s own type field per the
/// documented semantics.
///
/// A malformed `PLDT`/`PSDT`/`PTDT` (present but the wrong size) degrades to
/// that field being absent (`None`) with no diagnostic, since only
/// `PKDT` is a hard requirement for this record type; `PLDT`/`PTDT` are
/// themselves documented optional by fopdoc.
pub(crate) fn parse_package(
    subs: &[Subrecord],
    form_id: u32,
    record_flags: u32,
    resolver: &FormIdResolver,
) -> std::result::Result<PackageRecord, String> {
    let pkdt = sub(subs, "PKDT").ok_or_else(|| "missing PKDT subrecord".to_string())?;
    let (general_flags, package_type) = match pkdt.len() {
        4 => (
            u32_at(pkdt, 0).ok_or_else(|| "truncated PKDT".to_string())?,
            0,
        ),
        12 => (
            u32_at(pkdt, 0).ok_or_else(|| "truncated PKDT".to_string())?,
            pkdt[4],
        ),
        length => return Err(format!("PKDT must be 4 or 12 bytes, got {length}")),
    };

    let location = sub(subs, "PLDT").and_then(|data| {
        if data.len() != 12 {
            return None;
        }
        let location_type = u32_at(data, 0)?;
        let raw_value = u32_at(data, 4)?;
        Some(PackageLocation {
            location_type,
            form_id: (location_type != 5 && raw_value != 0).then(|| resolver.adjust(raw_value)),
            raw_value,
            radius: i32_at(data, 8)?,
        })
    });

    let schedule = sub(subs, "PSDT").and_then(|data| {
        if data.len() != 8 {
            return None;
        }
        Some(PackageSchedule {
            month: data[0] as i8,
            day_of_week: data[1] as i8,
            date: data[2],
            time: data[3] as i8,
            duration: i32_at(data, 4)?,
        })
    });

    let target = sub(subs, "PTDT").and_then(|data| {
        if data.len() != 16 {
            return None;
        }
        let target_type = i32_at(data, 0)?;
        let raw_value = u32_at(data, 4)?;
        Some(PackageTarget {
            target_type,
            form_id: (target_type != 2 && raw_value != 0).then(|| resolver.adjust(raw_value)),
            raw_value,
            count_or_distance: i32_at(data, 8)?,
            unknown: f32_at_option(data, 12)?,
        })
    });

    Ok(PackageRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        general_flags,
        package_type,
        location,
        schedule,
        target,
        conditions: subs
            .iter()
            .filter(|subrecord| subrecord.signature == "CTDA")
            .map(|subrecord| subrecord.data.clone())
            .collect(),
        ignored_subrecords: ignored_signatures(
            subs,
            &["EDID", "PKDT", "PLDT", "PSDT", "PTDT", "CTDA"],
        ),
    })
}
