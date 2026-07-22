//! ESM4 record-specific decoding.

use super::*;

/// `LVLI`/`LVLN`/`LVLC` leveled-list body (issue #74): `LVLD` (chance-none
/// percentage), `LVLF` (flags byte), and zero or more `LVLO` entries. OpenMW
/// `components/esm4/loadlvli.cpp` (`ESM4::LevelledItem::load`),
/// `loadlvlc.cpp` (`ESM4::LevelledCreature::load`), and `loadlvln.cpp`
/// (`ESM4::LevelledNpc::load`) all read the same three subrecords for their
/// respective record kinds.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct LeveledListData {
    pub(crate) chance_none: u8,
    pub(crate) flags: u8,
    pub(crate) entries: Vec<LeveledListEntry>,
}

/// One `LVLO` entry. OpenMW's `struct LVLO` (`components/esm4/inventory.hpp`)
/// is `{ i16 level; u16 unknown; FormId32 item; i16 count; u16 unknown2; }`
/// (12 bytes); FO3 content also carries an 8-byte legacy layout dropping
/// both `unknown` fields (`level`, `item`, `count` only), which OpenMW's
/// three leveled-list loaders special-case identically.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LeveledListEntry {
    pub(crate) level: u16,
    pub(crate) item_form_id: u32,
    pub(crate) count: i32,
}

pub(crate) fn is_leveled_list(sig: &str) -> bool {
    matches!(sig, "LVLI" | "LVLN" | "LVLC")
}

pub(crate) fn parse_base(
    sig: &str,
    subs: &[Subrecord],
    resolver: &FormIdResolver,
) -> Option<BaseRecord> {
    if !is_supported_base(sig) {
        return None;
    }
    let model_value = |signature| {
        sub(subs, signature)
            .map(cstring)
            .filter(|value| !value.is_empty())
    };
    let apparel_models = (sig == "ARMO").then(|| ApparelModelSet {
        male_worn: model_value("MODL"),
        male_world: model_value("MOD2"),
        female_worn: model_value("MOD3"),
        female_world: model_value("MOD4"),
    });
    // Standalone ARMO placements/drops use a world model. Actor preparation
    // reads `apparel_models` and never consumes this fallback.
    let model = apparel_models
        .as_ref()
        .and_then(|models| {
            models
                .male_world
                .clone()
                .or_else(|| models.female_world.clone())
                .or_else(|| models.male_worn.clone())
                .or_else(|| models.female_worn.clone())
        })
        .or_else(|| model_value("MODL"));
    let light = (sig == "LIGH").then(|| parse_light_data(subs)).flatten();
    let (value, weight) = parse_value_weight(sig, subs);
    let item_stats = parse_item_stats(sig, subs, resolver);
    let inventory = subs
        .iter()
        .filter(|subrecord| subrecord.signature == "CNTO")
        .filter_map(|subrecord| parse_inventory_item(&subrecord.data, resolver))
        .collect();
    let audio = parse_base_audio(sig, subs, resolver);
    let leveled = is_leveled_list(sig).then(|| parse_leveled_list(subs, resolver));
    // Issue #103 (M4 wave 1 task A): NPC_/CREA actor subrecords decode into
    // `BaseRecord::actor`. Their supported signatures extend the diagnostics
    // allowlist below, and any malformed ACBS/AIDT/DATA payload contributes
    // its own stable diagnostic rather than panicking.
    let mut supported_signatures = vec![
        "EDID", "FULL", "MODL", "MOD2", "MOD3", "MOD4", "DATA", "ENIT", "TPLT", "CNTO", "SNAM",
        "ANAM", "BNAM", "QNAM", "VNAM", "YNAM", "ZNAM", "LVLD", "LVLF", "LVLO", "ICON", "MICO",
        "ICO2", "MIC2", "DESC", "EFID", "EFIT", "SCIT", "DNAM",
        // Issue #98 (F98.1): now decoded by `parse_item_stats`'s
        // WEAP/ARMO arms rather than falling through to diagnostics.
        "BMDT", "NAM0",
        // Issue #123: NOTE.DATA's type enum and NOTE.TNAM's text-note
        // content, decoded by `note_text` above.
        "TNAM",
    ];
    supported_signatures.extend_from_slice(actor_supported_signatures(sig));
    let mut ignored_subrecords = ignored_signatures(subs, &supported_signatures);
    let actor = parse_actor(sig, subs, resolver).map(|(actor, diagnostics)| {
        ignored_subrecords.extend(diagnostics);
        actor
    });
    Some(BaseRecord {
        kind: sig.to_string(),
        record_flags: 0,
        editor_id: sub(subs, "EDID").map(cstring),
        name: sub(subs, "FULL").map(cstring),
        model,
        apparel_models,
        icon: sub(subs, "ICON")
            .map(cstring)
            .filter(|value| !value.is_empty()),
        mini_icon: sub(subs, "MICO")
            .map(cstring)
            .filter(|value| !value.is_empty()),
        value,
        weight,
        item_stats,
        base_template_form_id: sub_form_id(subs, "TPLT", resolver),
        light,
        inventory,
        audio,
        leveled,
        actor,
        ignored_subrecords,
    })
}

/// Decodes the documented Fallout 3/New Vegas `RCPE` layout: a 16-byte
/// `DATA` struct followed by `RCIL`/`RCQY` ingredient pairs and
/// `RCOD`/`RCQY` output pairs. Pairing is order-sensitive in the source
/// stream, while the prepare slice later applies its own deterministic sort.
pub(crate) fn parse_recipe(
    subs: &[Subrecord],
    form_id: u32,
    record_flags: u32,
    resolver: &FormIdResolver,
) -> std::result::Result<RecipeRecord, String> {
    let mut data = None;
    for subrecord in subs
        .iter()
        .filter(|subrecord| subrecord.signature == "DATA")
    {
        if data.replace(subrecord.data.as_slice()).is_some() {
            return Err("RCPE contains duplicate DATA subrecords".into());
        }
    }
    let data = data.ok_or_else(|| "missing DATA subrecord".to_string())?;
    if data.len() != 16 {
        return Err(format!("DATA must be exactly 16 bytes, got {}", data.len()));
    }

    #[derive(Clone, Copy)]
    enum ItemKind {
        Ingredient,
        Output,
    }

    let mut ingredients = Vec::new();
    let mut outputs = Vec::new();
    let mut pending = None;
    for subrecord in subs {
        match subrecord.signature.as_str() {
            "RCIL" | "RCOD" => {
                if pending.is_some() {
                    return Err("item subrecord is missing its RCQY quantity".into());
                }
                if subrecord.data.len() != 4 {
                    return Err(format!(
                        "{} must be exactly 4 bytes, got {}",
                        subrecord.signature,
                        subrecord.data.len()
                    ));
                }
                let raw = u32::from_le_bytes(subrecord.data[..4].try_into().unwrap());
                let item_form_id = if raw == 0 { 0 } else { resolver.adjust(raw) };
                let kind = if subrecord.signature == "RCIL" {
                    ItemKind::Ingredient
                } else {
                    ItemKind::Output
                };
                pending = Some((kind, item_form_id));
            }
            "RCQY" => {
                let Some((kind, item_form_id)) = pending.take() else {
                    return Err("RCQY quantity has no preceding item subrecord".into());
                };
                if subrecord.data.len() != 4 {
                    return Err(format!(
                        "RCQY must be exactly 4 bytes, got {}",
                        subrecord.data.len()
                    ));
                }
                let raw_quantity = u32::from_le_bytes(subrecord.data[..4].try_into().unwrap());
                let quantity = i32::try_from(raw_quantity).map_err(|_| {
                    format!("RCQY quantity {raw_quantity} exceeds the supported signed count")
                })?;
                match kind {
                    ItemKind::Ingredient => ingredients.push(RecipeItemRecord {
                        item_form_id,
                        quantity,
                        order: ingredients.len() as u32,
                    }),
                    ItemKind::Output => outputs.push(RecipeItemRecord {
                        item_form_id,
                        quantity,
                        order: outputs.len() as u32,
                    }),
                }
            }
            _ => {}
        }
    }
    if pending.is_some() {
        return Err("item subrecord is missing its RCQY quantity".into());
    }

    let form_id_at = |offset: usize| {
        let raw = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        (raw != 0).then(|| resolver.adjust(raw))
    };
    Ok(RecipeRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        name: sub(subs, "FULL").map(cstring),
        skill: i32::from_le_bytes(data[0..4].try_into().unwrap()),
        level: u32::from_le_bytes(data[4..8].try_into().unwrap()),
        category_form_id: form_id_at(8),
        sub_category_form_id: form_id_at(12),
        ingredients,
        outputs,
        conditions: subs
            .iter()
            .filter(|subrecord| subrecord.signature == "CTDA")
            .map(|subrecord| subrecord.data.clone())
            .collect(),
        ignored_subrecords: ignored_signatures(
            subs,
            &["EDID", "FULL", "CTDA", "DATA", "RCIL", "RCOD", "RCQY"],
        ),
    })
}

pub(crate) fn parse_leveled_list(subs: &[Subrecord], resolver: &FormIdResolver) -> LeveledListData {
    LeveledListData {
        chance_none: sub(subs, "LVLD")
            .and_then(|data| data.first().copied())
            .unwrap_or(0),
        flags: sub(subs, "LVLF")
            .and_then(|data| data.first().copied())
            .unwrap_or(0),
        entries: subs
            .iter()
            .filter(|subrecord| subrecord.signature == "LVLO")
            .filter_map(|subrecord| parse_leveled_entry(&subrecord.data, resolver))
            .collect(),
    }
}

pub(crate) fn parse_leveled_entry(
    data: &[u8],
    resolver: &FormIdResolver,
) -> Option<LeveledListEntry> {
    let (level, item, count) = match data.len() {
        8 => (
            i16::from_le_bytes(data[0..2].try_into().ok()?),
            u32::from_le_bytes(data[2..6].try_into().ok()?),
            i16::from_le_bytes(data[6..8].try_into().ok()?),
        ),
        12 => (
            i16::from_le_bytes(data[0..2].try_into().ok()?),
            u32::from_le_bytes(data[4..8].try_into().ok()?),
            i16::from_le_bytes(data[8..10].try_into().ok()?),
        ),
        _ => return None,
    };
    Some(LeveledListEntry {
        level: level.max(0) as u16,
        item_form_id: resolver.adjust(item),
        count: i32::from(count),
    })
}

pub(crate) fn parse_item_stats(
    sig: &str,
    subs: &[Subrecord],
    resolver: &FormIdResolver,
) -> OpenMwItemStats {
    let data = sub(subs, "DATA");
    match sig {
        "WEAP" => {
            let (damage, max_condition, clip_size, speed, reach) = match data {
                Some(data) if data.len() == 15 => (
                    u16_at(data, 12),
                    u32_at_option(data, 4),
                    data.get(14).copied(),
                    None,
                    None,
                ),
                Some(data) if data.len() >= 30 => (
                    u16_at(data, 28),
                    u32_at_option(data, 20),
                    None,
                    f32_at_option(data, 4),
                    f32_at_option(data, 8),
                ),
                Some(data) if data.len() == 10 => (u16_at(data, 8), None, None, None, None),
                _ => (None, None, None, None, None),
            };
            OpenMwItemStats::Weapon {
                damage,
                max_condition,
                clip_size,
                speed,
                reach,
                // Issue #98 (F98.1): FO3's WEAP.NAM0 "Ammo" field, a plain
                // FormID reference like YNAM/ZNAM.
                ammo_form_id: sub_form_id(subs, "NAM0", resolver),
                animation_type: sub(subs, "DNAM").and_then(|data| u32_at_option(data, 0)),
            }
        }
        "ARMO" => OpenMwItemStats::Apparel {
            // OpenMW retains FO3 ARMO.DNAM without decoding it. Fallout 3
            // writes the displayed DR as a 16-bit integer; retain unknown
            // layouts instead of borrowing another game's interpretation.
            armor_rating: sub(subs, "DNAM")
                .filter(|data| data.len() == 2)
                .and_then(|data| u16_at(data, 0))
                .map(f32::from),
            max_condition: data.and_then(|data| u32_at_option(data, 4)),
            // Issue #98 (F98.1): BMDT's first four bytes are the biped-slot
            // mask in both the FO3 (8-byte) and TES4 (4-byte) layouts (see
            // `ESM4::Armor::load`'s BMDT case in OpenMW's
            // `components/esm4/loadarmo.cpp`).
            biped_slot_mask: sub(subs, "BMDT").and_then(|data| u32_at_option(data, 0)),
        },
        "AMMO" => OpenMwItemStats::Ammo {
            damage: sub(subs, "DNAM")
                .filter(|data| data.len() >= 12)
                .and_then(|data| f32_at_option(data, 8))
                .or_else(|| {
                    data.filter(|data| data.len() >= 18)
                        .and_then(|data| u16_at(data, 16))
                        .map(f32::from)
                }),
            speed: data.and_then(|data| f32_at_option(data, 0)),
        },
        "ALCH" => OpenMwItemStats::Aid {
            effect_form_ids: subs
                .iter()
                .filter(|subrecord| subrecord.signature == "EFID")
                .filter_map(|subrecord| u32_at_option(&subrecord.data, 0))
                .map(|form_id| resolver.adjust(form_id))
                .collect(),
        },
        "BOOK" => OpenMwItemStats::Book {
            flags: data.and_then(|data| data.first().copied()),
            text: sub(subs, "DESC").map(cstring),
        },
        "NOTE" => OpenMwItemStats::Note {
            text: note_text(subs),
        },
        "KEYM" => OpenMwItemStats::Key,
        _ => OpenMwItemStats::Misc,
    }
}

/// FO3 `NOTE.DATA`'s type enum value for a text note (0 Sound, 1 Text,
/// 2 Image, 3 Voice -- fopdoc's Fallout3 `NOTE` page, `TES5Edit/fopdoc`).
const NOTE_TYPE_TEXT: u8 = 1;

/// Issue #123: the supplied OpenMW snapshot's `loadnote.cpp` explicitly
/// `skipSubRecordData()`s every FO3/FNV `NOTE` field beyond `EDID`/`FULL`/
/// `MODL`/`ICON`/`MODB`/`YNAM`/`ZNAM` -- `DATA`, `TNAM`, `XNAM`, `SNAM`, and
/// `ONAM` are all unrecognized there, so this decode is a fopdoc-sourced
/// extension, not an OpenMW port (see NOTICE.md). fopdoc documents `DATA`
/// as a `uint8` type enum and `TNAM` as "a text string, or the FormID of a
/// DIAL record" -- the two are read together: only a type-`Text` (`1`) note
/// stores its content as a plain cstring in `TNAM`; other types (Sound,
/// Image, Voice) use `TNAM` for a DIAL-topic FormID reference or leave it
/// unused, and decoding those bytes as text would fabricate garbage. Notes
/// with no `DATA` subrecord, or a non-text type, stay `None` rather than
/// guessing.
fn note_text(subs: &[Subrecord]) -> Option<String> {
    let note_type = sub(subs, "DATA").and_then(|data| data.first().copied())?;
    if note_type != NOTE_TYPE_TEXT {
        return None;
    }
    sub(subs, "TNAM").map(cstring)
}

fn u16_at(data: &[u8], offset: usize) -> Option<u16> {
    Some(u16::from_le_bytes(
        data.get(offset..offset + 2)?.try_into().ok()?,
    ))
}

fn u32_at_option(data: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        data.get(offset..offset + 4)?.try_into().ok()?,
    ))
}

pub(crate) fn parse_inventory_item(
    data: &[u8],
    resolver: &FormIdResolver,
) -> Option<InventoryItemRecord> {
    let item = data.get(..4)?;
    let count = data.get(4..8)?;
    Some(InventoryItemRecord {
        item_form_id: resolver.adjust(u32::from_le_bytes(item.try_into().ok()?)),
        count: u32::from_le_bytes(count.try_into().ok()?),
    })
}

pub(crate) fn parse_base_audio(
    sig: &str,
    subs: &[Subrecord],
    resolver: &FormIdResolver,
) -> BaseAudioRecord {
    let mut audio = BaseAudioRecord::default();
    match sig {
        "DOOR" => {
            audio.open_sound_form_id = sub_form_id(subs, "SNAM", resolver);
            audio.close_sound_form_id = sub_form_id(subs, "ANAM", resolver);
            audio.loop_sound_form_id = sub_form_id(subs, "BNAM", resolver);
        }
        "CONT" => {
            audio.open_sound_form_id = sub_form_id(subs, "SNAM", resolver);
            audio.close_sound_form_id = sub_form_id(subs, "QNAM", resolver);
        }
        "ACTI" => {
            audio.loop_sound_form_id = sub_form_id(subs, "SNAM", resolver);
            audio.activation_sound_form_id = sub_form_id(subs, "VNAM", resolver);
        }
        "MSTT" | "TACT" | "LIGH" => {
            audio.loop_sound_form_id = sub_form_id(subs, "SNAM", resolver);
        }
        "TERM" => {
            audio.activation_sound_form_id = sub_form_id(subs, "SNAM", resolver);
        }
        _ if is_pickup_base(sig) => {
            audio.pickup_sound_form_id = sub_form_id(subs, "YNAM", resolver);
            audio.drop_sound_form_id = sub_form_id(subs, "ZNAM", resolver);
        }
        _ => {}
    }
    audio
}

pub(crate) fn is_pickup_base(sig: &str) -> bool {
    matches!(
        sig,
        "WEAP" | "AMMO" | "ARMO" | "ALCH" | "MISC" | "BOOK" | "NOTE" | "KEYM"
    )
}

pub(crate) fn is_supported_base(sig: &str) -> bool {
    matches!(
        sig,
        "STAT"
            | "MSTT"
            | "LIGH"
            | "DOOR"
            | "CONT"
            | "ACTI"
            | "TACT"
            | "FURN"
            | "TERM"
            | "WEAP"
            | "AMMO"
            | "ARMO"
            | "ALCH"
            | "MISC"
            | "BOOK"
            | "NOTE"
            | "KEYM"
            | "NPC_"
            | "CREA"
            | "HDPT"
            | "HAIR"
            | "EYES"
            | "LVLI"
            | "LVLN"
            | "LVLC"
    )
}

pub(crate) fn parse_value_weight(sig: &str, subs: &[Subrecord]) -> (Option<i32>, Option<f32>) {
    let data = sub(subs, "DATA");
    match sig {
        "MISC" | "KEYM" => data
            .filter(|data| data.len() >= 8)
            .map(|data| (i32_at(data, 0), f32_at_option(data, 4)))
            .unwrap_or((None, None)),
        "WEAP" => match data.map(<[u8]>::len) {
            Some(10) => (
                data.and_then(|data| i32_at(data, 0)),
                data.and_then(|data| f32_at_option(data, 4)),
            ),
            Some(15) => (
                data.and_then(|data| i32_at(data, 0)),
                data.and_then(|data| f32_at_option(data, 8)),
            ),
            Some(length) if length >= 30 => (
                data.and_then(|data| i32_at(data, 16)),
                data.and_then(|data| f32_at_option(data, 24)),
            ),
            _ => (None, None),
        },
        "AMMO" => data
            .filter(|data| data.len() >= 12)
            .map(|data| (i32_at(data, 8), None))
            .unwrap_or((None, None)),
        "ARMO" => data
            .filter(|data| data.len() >= 12)
            .map(|data| (i32_at(data, 0), f32_at_option(data, 8)))
            .unwrap_or((None, None)),
        "ALCH" => (
            sub(subs, "ENIT").and_then(|data| i32_at(data, 0)),
            data.and_then(|data| f32_at_option(data, 0)),
        ),
        "BOOK" => data
            .filter(|data| data.len() >= 10)
            .map(|data| (i32_at(data, 2), f32_at_option(data, 6)))
            .unwrap_or((None, None)),
        _ => (None, None),
    }
}

pub(crate) fn parse_light_data(subs: &[Subrecord]) -> Option<LightData> {
    let data = sub(subs, "DATA")?;
    if data.len() < 12 {
        return None;
    }
    Some(LightData {
        radius: u32::from_le_bytes(data[4..8].try_into().ok()?) as f32,
        color_rgba: [
            data[8] as f32 / 255.0,
            data[9] as f32 / 255.0,
            data[10] as f32 / 255.0,
            1.0,
        ],
    })
}

pub(crate) fn parse_music(subs: &[Subrecord], form_id: u32, record_flags: u32) -> MusicRecord {
    MusicRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        file: sub(subs, "FNAM")
            .or_else(|| sub(subs, "MNAM"))
            .map(cstring)
            .filter(|value| !value.is_empty()),
        ignored_subrecords: ignored_signatures(subs, &["EDID", "FNAM", "MNAM", "DATA"]),
    }
}

pub(crate) fn parse_sound(subs: &[Subrecord], form_id: u32, record_flags: u32) -> SoundRecord {
    let mut parameters = None;
    let mut extra = None;
    for subrecord in subs {
        match subrecord.signature.as_str() {
            "SNDX" => parameters = parse_sound_parameters(&subrecord.data),
            "SNDD" => {
                parameters = parse_sound_parameters(&subrecord.data);
                extra = parse_sound_extra(&subrecord.data);
            }
            _ => {}
        }
    }
    SoundRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        file: sub(subs, "FNAM")
            .map(cstring)
            .filter(|value| !value.is_empty()),
        parameters,
        extra,
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "FNAM", "SNDX", "SNDD", "OBND", "SDSC", "ANAM", "GNAM", "HNAM", "RNAM",
                "REPT",
            ],
        ),
    }
}

pub(crate) fn parse_sound_parameters(data: &[u8]) -> Option<SoundParameters> {
    if data.len() < 8 {
        return None;
    }
    Some(SoundParameters {
        byte_len: data.len() as u32,
        min_attenuation: data[0],
        max_attenuation: data[1],
        frequency_adjustment: data[2] as i8,
        flags: u16::from_le_bytes(data[4..6].try_into().ok()?),
        static_attenuation: data
            .get(8..10)
            .map(|bytes| u16::from_le_bytes(bytes.try_into().unwrap())),
        stop_time: data.get(10).copied(),
        start_time: data.get(11).copied(),
    })
}

pub(crate) fn parse_sound_extra(data: &[u8]) -> Option<SoundExtraData> {
    let data = data.get(12..36)?;
    Some(SoundExtraData {
        attenuation_points: [
            i16_at(data, 0)?,
            i16_at(data, 2)?,
            i16_at(data, 4)?,
            i16_at(data, 6)?,
            i16_at(data, 8)?,
        ],
        reverb_attenuation_control: i16_at(data, 10)?,
        priority: i32_at(data, 12)?,
        unknown_x: i32_at(data, 16)?,
        unknown_y: i32_at(data, 20)?,
    })
}

pub(crate) fn parse_sound_reference(
    subs: &[Subrecord],
    form_id: u32,
    record_flags: u32,
    resolver: &FormIdResolver,
) -> SoundReferenceRecord {
    let bnam = sub(subs, "BNAM");
    SoundReferenceRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        sound_category_form_id: sub_form_id(subs, "GNAM", resolver),
        sound_reference_form_id: sub_form_id(subs, "SNAM", resolver),
        output_model_form_id: sub_form_id(subs, "ONAM", resolver),
        base_descriptor_form_id: bnam
            .filter(|data| data.len() == 4)
            .map(|data| resolver.adjust(u32::from_le_bytes(data.try_into().unwrap())))
            .filter(|form_id| *form_id != 0),
        file: sub(subs, "ANAM")
            .map(cstring)
            .filter(|value| !value.is_empty()),
        loop_info: sub(subs, "LNAM")
            .filter(|data| data.len() >= 4)
            .map(|data| SoundLoopInfo {
                flags: u16::from_le_bytes(data[0..2].try_into().unwrap()),
                unknown: data[2],
                rumble: data[3],
            }),
        sound_info: bnam.filter(|data| data.len() == 6).map(|data| SoundInfo {
            frequency_adjustment: data[0] as i8,
            frequency_variance: data[1],
            priority: data[2],
            decibel_variance: data[3],
            static_attenuation: u16::from_le_bytes(data[4..6].try_into().unwrap()),
        }),
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "GNAM", "SNAM", "ONAM", "ANAM", "LNAM", "BNAM", "CTDA", "CIS1", "CIS2",
                "CNAM", "DNAM", "FNAM", "INTV", "ITMC", "ITME", "ITMS", "NNAM",
            ],
        ),
    }
}

pub(crate) fn parse_acoustic_space(
    subs: &[Subrecord],
    form_id: u32,
    record_flags: u32,
    resolver: &FormIdResolver,
) -> AcousticSpaceRecord {
    let mut is_interior = None;
    for subrecord in subs {
        match subrecord.signature.as_str() {
            "INAM" if subrecord.data.len() >= 4 => {
                is_interior =
                    Some(u32::from_le_bytes(subrecord.data[0..4].try_into().unwrap()) != 0)
            }
            "XTRI" if !subrecord.data.is_empty() => is_interior = Some(subrecord.data[0] != 0),
            _ => {}
        }
    }
    AcousticSpaceRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        environment_type: sub(subs, "ANAM").and_then(|data| u32_at(data, 0)),
        ambient_loop_sound_form_ids: subs
            .iter()
            .filter(|subrecord| subrecord.signature == "SNAM")
            .filter_map(|subrecord| u32_at(&subrecord.data, 0))
            .map(|form_id| resolver.adjust(form_id))
            .filter(|form_id| *form_id != 0)
            .collect(),
        sound_region_form_id: sub_form_id(subs, "RDAT", resolver),
        is_interior,
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "ANAM", "SNAM", "RDAT", "INAM", "XTRI", "WNAM", "BNAM", "OBND",
            ],
        ),
    }
}

pub(crate) fn parse_lighting_template(
    subs: &[Subrecord],
    form_id: u32,
    record_flags: u32,
) -> LightingTemplateRecord {
    LightingTemplateRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        lighting: sub(subs, "DATA").and_then(parse_lighting_data),
        ignored_subrecords: ignored_signatures(subs, &["EDID", "DATA", "DALC"]),
    }
}

pub(crate) fn parse_lighting_data(data: &[u8]) -> Option<LightingData> {
    // OpenMW accepts the TES4 36-byte layout and the FO3/FNV 40-byte layout
    // (and skips the tail of newer layouts).  Reject the otherwise ambiguous
    // 37-39 byte range instead of partially applying malformed lighting.
    if data.len() < 36 || (data.len() > 36 && data.len() < 40) {
        return None;
    }
    Some(LightingData {
        ambient_rgba: rgba8(&data[0..4]),
        directional_rgba: rgba8(&data[4..8]),
        fog_rgba: rgba8(&data[8..12]),
        fog_near: f32_at_option(data, 12)?,
        fog_far: f32_at_option(data, 16)?,
        rotation_xy: i32_at(data, 20)?,
        rotation_z: i32_at(data, 24)?,
        fog_directional_fade: f32_at_option(data, 28)?,
        fog_clip_distance: f32_at_option(data, 32)?,
        fog_power: f32_at_option(data, 36).unwrap_or(1.0),
    })
}

pub(crate) fn parse_cell(
    subs: &[Subrecord],
    form_id: u32,
    resolver: &FormIdResolver,
) -> Result<CellInfo> {
    let interior = sub(subs, "DATA")
        .and_then(|data| data.first())
        .is_some_and(|flags| flags & 1 != 0);
    let mut ambient = [0.18, 0.18, 0.18, 1.0];
    let mut directional = [0.8, 0.8, 0.8, 1.0];
    if let Some(lighting) = sub(subs, "XCLL").and_then(parse_lighting_data) {
        ambient = lighting.ambient_rgba;
        directional = lighting.directional_rgba;
    }
    Ok(CellInfo {
        form_id,
        editor_id: sub(subs, "EDID").map(cstring),
        name: sub(subs, "FULL").map(cstring),
        interior,
        ambient_rgba: ambient,
        directional_rgba: directional,
        image_space_form_id: sub_form_id(subs, "XCIM", resolver),
        image_space: None,
        lighting_template_form_id: sub_form_id(subs, "LTMP", resolver),
        lighting_template_flags: sub(subs, "LNAM")
            .and_then(|data| u32_at(data, 0))
            .unwrap_or_default(),
        lighting_template: None,
        raw_lighting: None,
        effective_lighting: None,
        water_form_id: sub_form_id(subs, "XCWT", resolver),
        water_height: sub(subs, "XCLW").and_then(|data| f32_at_option(data, 0)),
        grid: sub(subs, "XCLC").and_then(parse_grid),
        // Filled in by `walk_container` from the enclosing group-type-1
        // ("world children") GRUP label, once traversal knows the context a
        // record-local parse function like this one cannot see.
        worldspace_form_id: None,
    })
}

/// `CELL.XCLC`: exterior grid (X, Y), OpenMW `components/esm4/loadcell.cpp`
/// (`case ESM4::SUB_XCLC`). Two little-endian `i32`s, optionally followed by
/// a `u32` "force hide land quad" flags word that this catalogue does not
/// need. Interior cells never carry this subrecord; short/legacy payloads
/// (fewer than 8 bytes) are skipped rather than treated as an error.
pub(crate) fn parse_grid(data: &[u8]) -> Option<(i32, i32)> {
    Some((i32_at(data, 0)?, i32_at(data, 4)?))
}

pub(crate) fn parse_cell_metadata(subs: &[Subrecord], resolver: &FormIdResolver) -> CellMetadata {
    CellMetadata {
        acoustic_space_form_id: sub_form_id(subs, "XCAS", resolver),
        music_form_id: sub_form_id(subs, "XCMO", resolver),
        lighting_template_form_id: sub_form_id(subs, "LTMP", resolver),
        lighting_template_flags: sub(subs, "LNAM")
            .and_then(|data| u32_at(data, 0))
            .unwrap_or_default(),
        water_form_id: sub_form_id(subs, "XCWT", resolver),
        water_height: sub(subs, "XCLW").and_then(|data| f32_at_option(data, 0)),
        lighting: sub(subs, "XCLL").and_then(parse_lighting_data),
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "FULL", "DATA", "XCLL", "XCIM", "XCAS", "XCMO", "LTMP", "LNAM", "XCWT",
                "XCLW", "XCLC",
            ],
        ),
    }
}

pub(crate) fn parse_reference(
    subs: &[Subrecord],
    form_id: u32,
    parent_cell_form_id: u32,
    flags: u32,
    kind: ReferenceKind,
    resolver: &FormIdResolver,
) -> Result<Option<ReferenceRecord>> {
    let Some(base_form_id) = sub_form_id(subs, "NAME", resolver) else {
        return Ok(None);
    };
    let Some(data) = sub(subs, "DATA").filter(|data| data.len() >= 24) else {
        return Ok(None);
    };
    let teleport = sub(subs, "XTEL")
        .filter(|data| data.len() >= 28)
        .map(|data| TeleportRecord {
            door_reference_form_id: resolver
                .adjust(u32::from_le_bytes(data[0..4].try_into().unwrap())),
            position: [
                f32_at(data, 4).unwrap_or_default(),
                f32_at(data, 8).unwrap_or_default(),
                f32_at(data, 12).unwrap_or_default(),
            ],
            rotation: [
                f32_at(data, 16).unwrap_or_default(),
                f32_at(data, 20).unwrap_or_default(),
                f32_at(data, 24).unwrap_or_default(),
            ],
        });
    let lock = sub(subs, "XLOC").filter(|data| data.len() >= 8);
    let door = (teleport.is_some() || lock.is_some()).then(|| DoorRecord {
        lock_level: lock.map(|data| data[0] as i8),
        key_form_id: lock
            .map(|data| resolver.adjust(u32::from_le_bytes(data[4..8].try_into().unwrap())))
            .filter(|id| *id != 0),
        // Issue #185: no identified FO3 trap subrecord yet -- see
        // `DoorRecord::trapped`'s doc comment.
        trapped: false,
        destination: None,
        teleport,
    });
    Ok(Some(ReferenceRecord {
        kind,
        form_id,
        parent_cell_form_id,
        base_form_id,
        position: [f32_at(data, 0)?, f32_at(data, 4)?, f32_at(data, 8)?],
        rotation: [f32_at(data, 12)?, f32_at(data, 16)?, f32_at(data, 20)?],
        scale: sub(subs, "XSCL")
            .and_then(|data| f32_at_option(data, 0))
            .unwrap_or(1.0),
        count: sub(subs, "XCNT")
            .and_then(|data| i32_at(data, 0))
            .unwrap_or(1),
        flags,
        door,
        owner_form_id: sub_form_id(subs, "XOWN", resolver),
        owner_faction_rank: sub(subs, "XRNK").and_then(|data| i32_at(data, 0)),
        enable_parent: sub(subs, "XESP")
            .filter(|data| data.len() >= 8)
            .map(|data| EnableParentRecord {
                parent_reference_form_id: resolver
                    .adjust(u32::from_le_bytes(data[0..4].try_into().unwrap())),
                flags: u32::from_le_bytes(data[4..8].try_into().unwrap()),
            }),
        initially_enabled: flags & RECORD_DISABLED == 0,
        enable_root_form_id: None,
        linked_reference_form_id: sub_form_id(subs, "XLKR", resolver),
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "NAME", "DATA", "XSCL", "XCNT", "XTEL", "XLOC", "XOWN", "XRNK", "XESP",
                "XLKR",
            ],
        ),
    }))
}

pub(crate) fn ignored_signatures(subs: &[Subrecord], supported: &[&str]) -> Vec<String> {
    let mut signatures = subs
        .iter()
        .filter(|sub| !supported.contains(&sub.signature.as_str()))
        .map(|sub| sub.signature.clone())
        .collect::<Vec<_>>();
    signatures.sort();
    signatures.dedup();
    signatures
}

pub(crate) fn parse_image_space(subs: &[Subrecord], form_id: u32) -> Option<ImageSpaceInfo> {
    let data = sub(subs, "DNAM")?;
    let mut image_space = ImageSpaceInfo {
        form_id,
        editor_id: sub(subs, "EDID").map(cstring),
        ..ImageSpaceInfo::default()
    };
    image_space.eye_adapt_speed = f32_or(data, 0, image_space.eye_adapt_speed);
    image_space.hdr_blur_radius = f32_or(data, 4, image_space.hdr_blur_radius);
    image_space.hdr_blur_passes = f32_or(data, 8, image_space.hdr_blur_passes);
    image_space.hdr_emissive_multiplier = f32_or(data, 12, image_space.hdr_emissive_multiplier);
    image_space.hdr_target_lum = f32_or(data, 16, image_space.hdr_target_lum);
    image_space.hdr_upper_lum_clamp = f32_or(data, 20, image_space.hdr_upper_lum_clamp);
    image_space.hdr_bright_scale = f32_or(data, 24, image_space.hdr_bright_scale);
    image_space.hdr_bright_clamp = f32_or(data, 28, image_space.hdr_bright_clamp);
    image_space.hdr_lum_ramp_no_tex = f32_or(data, 32, image_space.hdr_lum_ramp_no_tex);
    image_space.hdr_lum_ramp_min = f32_or(data, 36, image_space.hdr_lum_ramp_min);
    image_space.hdr_lum_ramp_max = f32_or(data, 40, image_space.hdr_lum_ramp_max);
    image_space.hdr_sunlight_dimmer = f32_or(data, 44, image_space.hdr_sunlight_dimmer);
    image_space.hdr_grass_dimmer = f32_or(data, 48, image_space.hdr_grass_dimmer);
    image_space.hdr_tree_dimmer = f32_or(data, 52, image_space.hdr_tree_dimmer);
    image_space.hdr_skin_dimmer = f32_or(data, 56, image_space.hdr_skin_dimmer);
    image_space.bloom_blur_radius = f32_or(data, 60, image_space.bloom_blur_radius);
    image_space.bloom_alpha_mult_interior = f32_or(data, 64, image_space.bloom_alpha_mult_interior);
    image_space.bloom_alpha_mult_exterior = f32_or(data, 68, image_space.bloom_alpha_mult_exterior);
    image_space.get_hit_blur_radius = f32_or(data, 72, image_space.get_hit_blur_radius);
    image_space.get_hit_blur_damping_constant =
        f32_or(data, 76, image_space.get_hit_blur_damping_constant);
    image_space.get_hit_damping_constant = f32_or(data, 80, image_space.get_hit_damping_constant);
    image_space.night_eye_tint_rgb = rgb_or(data, 84, image_space.night_eye_tint_rgb);
    image_space.brightness = f32_or(data, 96, image_space.brightness);
    image_space.cinematic_saturation = f32_or(data, 100, image_space.cinematic_saturation);
    image_space.cinematic_contrast_avg_lum =
        f32_or(data, 104, image_space.cinematic_contrast_avg_lum);
    image_space.cinematic_contrast = f32_or(data, 108, image_space.cinematic_contrast);
    image_space.cinematic_brightness_tint_rgb =
        rgb_or(data, 112, image_space.cinematic_brightness_tint_rgb);
    image_space.cinematic_brightness_tint_value =
        f32_or(data, 124, image_space.cinematic_brightness_tint_value);
    image_space.flags = data.get(144).copied().unwrap_or_default();
    Some(image_space)
}

/// `WRLD` worldspace record: FormID, `EDID`, `FULL`. OpenMW
/// `components/esm4/loadwrld.hpp` (`struct World`). Only the fields the
/// cell map artifact needs (issue #45) are decoded; the many worldspace
/// gameplay flags/parent/climate fields are left unparsed.
pub(crate) fn parse_worldspace(subs: &[Subrecord], form_id: u32) -> WorldspaceRecord {
    WorldspaceRecord {
        form_id,
        editor_id: sub(subs, "EDID").map(cstring),
        name: sub(subs, "FULL").map(cstring),
    }
}
