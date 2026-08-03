//! Fallout `IDLE` record decoding for authored animation preparation.

use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct IdleRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) model_path: Option<String>,
    pub(crate) parent_form_id: Option<u32>,
    pub(crate) previous_sibling_form_id: Option<u32>,
    pub(crate) conditions: Vec<Vec<u8>>,
    pub(crate) group_section_raw: u8,
    pub(crate) group_section: u8,
    pub(crate) loop_min: u8,
    pub(crate) loop_max: u8,
    pub(crate) replay_delay_seconds: i16,
    pub(crate) flags: u8,
    pub(crate) diagnostics: Vec<String>,
    pub(crate) ignored_subrecords: Vec<String>,
}

fn first_count(subs: &[Subrecord], signature: &str) -> usize {
    subs.iter()
        .filter(|subrecord| subrecord.signature == signature)
        .count()
}

fn optional_form_id(data: &[u8], offset: usize, resolver: &FormIdResolver) -> Option<u32> {
    u32_at(data, offset)
        .filter(|form_id| *form_id != 0)
        .map(|form_id| resolver.adjust(form_id))
}

fn known_group_raw(raw: u8) -> bool {
    matches!(
        bevyout_core::actor_animation::canonical_idle_group_section(raw),
        0..=7 | 20 | 21
    )
}

fn distinctive_group_raw(raw: u8) -> bool {
    known_group_raw(raw) && (raw & 0xc0 != 0 || raw == 0x54)
}

fn parse_idle_data(data: &[u8], diagnostics: &mut Vec<String>) -> (u8, u8, u8, i16, u8) {
    if !matches!(data.len(), 6 | 8) {
        diagnostics.push(format!(
            "DATA malformed: expected 6 or 8 bytes, got {}",
            data.len()
        ));
    }
    if data.len() >= 8 {
        let raw = data[0];
        let loop_min = data[1];
        let loop_max = data[2];
        let replay = i16_at(data, 4).unwrap_or_else(|| {
            diagnostics.push("DATA truncated: replay delay is missing".into());
            0
        });
        let flags = data.get(6).copied().unwrap_or_else(|| {
            diagnostics.push("DATA truncated: flags are missing".into());
            0
        });
        return (raw, loop_min, loop_max, replay, flags);
    }

    if data.len() == 6 {
        // The compact FO3 spelling drops the two unused padding bytes from
        // the documented 8-byte layout: group, min, max, replay, flags.
        // Some older tools emitted the commonlib field order instead; accept
        // that form when its group byte is unambiguous (the real authored
        // 0x47/0x87/0x54 values make this distinction safe).
        if distinctive_group_raw(data[3]) && !distinctive_group_raw(data[0]) {
            return (
                data[3],
                data[0],
                data[1],
                i16::from_le_bytes([data[4], data[5]]),
                data[2],
            );
        }
        return (
            data[0],
            data[1],
            data[2],
            i16::from_le_bytes([data[3], data[4]]),
            data[5],
        );
    }

    let raw = data.first().copied().unwrap_or_else(|| {
        diagnostics.push("DATA truncated: group section is missing".into());
        0
    });
    let loop_min = data.get(1).copied().unwrap_or_else(|| {
        diagnostics.push("DATA truncated: loop minimum is missing".into());
        0
    });
    let loop_max = data.get(2).copied().unwrap_or_else(|| {
        diagnostics.push("DATA truncated: loop maximum is missing".into());
        0
    });
    let replay = if data.len() >= 5 {
        i16::from_le_bytes([data[3], data[4]])
    } else {
        diagnostics.push("DATA truncated: replay delay is missing".into());
        0
    };
    let flags = data.get(5).copied().unwrap_or_else(|| {
        diagnostics.push("DATA truncated: flags are missing".into());
        0
    });
    (raw, loop_min, loop_max, replay, flags)
}

/// Decodes one winning IDLE record without making malformed optional fields
/// fatal. The caller owns load-order override/deletion handling.
pub(crate) fn parse_idle(
    subs: &[Subrecord],
    form_id: u32,
    record_flags: u32,
    resolver: &FormIdResolver,
) -> IdleRecord {
    let mut diagnostics = Vec::new();
    for signature in ["EDID", "MODL", "ANAM", "DATA"] {
        let count = first_count(subs, signature);
        if count > 1 {
            diagnostics.push(format!(
                "duplicate {signature} subrecords: {count}; first value used"
            ));
        }
    }

    let (group_section_raw, loop_min, loop_max, replay_delay_seconds, flags) =
        match sub(subs, "DATA") {
            Some(data) => parse_idle_data(data, &mut diagnostics),
            None => {
                diagnostics.push("DATA missing: using zero metadata".into());
                (0, 0, 0, 0, 0)
            }
        };
    if !known_group_raw(group_section_raw) {
        diagnostics.push(format!(
            "DATA unknown group section raw 0x{group_section_raw:02x} (canonical {})",
            bevyout_core::actor_animation::canonical_idle_group_section(group_section_raw)
        ));
    }
    let anam = sub(subs, "ANAM");
    if let Some(data) = anam
        && !matches!(data.len(), 0 | 8)
    {
        diagnostics.push(format!(
            "ANAM malformed: expected 8 bytes, got {}",
            data.len()
        ));
    }
    let model_path = sub(subs, "MODL")
        .map(cstring)
        .filter(|path| !path.is_empty());
    let ignored_subrecords = ignored_signatures(subs, &["EDID", "MODL", "CTDA", "ANAM", "DATA"]);
    diagnostics.extend(
        ignored_subrecords
            .iter()
            .map(|signature| format!("ignored unsupported {signature} subrecord")),
    );
    diagnostics.sort();
    diagnostics.dedup();

    IdleRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        model_path,
        parent_form_id: anam.and_then(|data| optional_form_id(data, 0, resolver)),
        previous_sibling_form_id: anam.and_then(|data| optional_form_id(data, 4, resolver)),
        conditions: subs
            .iter()
            .filter(|subrecord| subrecord.signature == "CTDA")
            .map(|subrecord| subrecord.data.clone())
            .collect(),
        group_section_raw,
        group_section: bevyout_core::actor_animation::canonical_idle_group_section(
            group_section_raw,
        ),
        loop_min,
        loop_max,
        replay_delay_seconds,
        flags,
        diagnostics,
        ignored_subrecords,
    }
}
