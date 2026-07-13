//! Load-order validation and the generic single-pass record walker.
//!
//! The GRUP/record header layout and subrecord decoding here mirror
//! `openmw_esm4`'s `walk_container`/`parse_subrecords` exactly, reusing that
//! module's `read_u32`, `record_payload`, `parse_subrecords`, `sub`,
//! `cstring`, and `RECORD_DELETED` directly rather than re-deriving the
//! byte format. What is *not* reused is `FormIdResolver`: its fields and
//! `adjust` method are private to `openmw_esm4`, so the (tiny) remap
//! arithmetic is duplicated here in `resolve_form_id`.

use std::collections::{BTreeMap, HashMap};

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};

use crate::vsa::openmw_esm4::{
    RECORD_DELETED, cstring, parse_subrecords, read_u32, record_payload, sub,
};
use crate::vsa::plugin::read_master_names;

use super::{ContentIndex, FormId, IndexedRecord, PluginSource};

struct RawRecordEvent {
    form_id: u32,
    record_type: String,
    editor_id: Option<String>,
    deleted: bool,
}

pub(super) fn build(sources: &[PluginSource<'_>]) -> Result<ContentIndex> {
    validate_load_order(sources)?;

    let positions: HashMap<String, usize> = sources
        .iter()
        .enumerate()
        .map(|(index, source)| (source.name.to_ascii_lowercase(), index))
        .collect();

    let mut records: HashMap<u32, IndexedRecord> = HashMap::new();
    let mut type_counts: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut diagnostics = Vec::new();

    for (index, source) in sources.iter().enumerate() {
        let masters = read_master_names(source.bytes)
            .with_context(|| format!("reading masters from {}", source.name))?;
        let master_indices: Vec<u8> = masters
            .iter()
            .map(|master| {
                positions
                    .get(&master.to_ascii_lowercase())
                    .copied()
                    .unwrap_or(index) as u8
            })
            .collect();
        let current_index = index as u8;

        walk_plugin(source.bytes, current_index, &master_indices, |event| {
            *type_counts
                .entry((source.name.to_string(), event.record_type.clone()))
                .or_insert(0) += 1;
            if !is_well_formed_signature(&event.record_type) {
                diagnostics.push(format!(
                    "unsupported record signature '{}' at FormID {:08x} in {}",
                    event.record_type, event.form_id, source.name
                ));
            }
            if event.deleted {
                records.remove(&event.form_id);
                return;
            }
            let provenance_entry = records
                .entry(event.form_id)
                .or_insert_with(|| IndexedRecord {
                    form_id: FormId(event.form_id),
                    record_type: event.record_type.clone(),
                    editor_id: None,
                    winning_plugin: source.name.to_string(),
                    provenance: Vec::new(),
                });
            provenance_entry.record_type = event.record_type;
            provenance_entry.editor_id = event.editor_id;
            provenance_entry.winning_plugin = source.name.to_string();
            provenance_entry.provenance.push(source.name.to_string());
        })
        .with_context(|| format!("parsing {}", source.name))?;
    }

    Ok(ContentIndex {
        records,
        load_order: sources
            .iter()
            .map(|source| source.name.to_string())
            .collect(),
        type_counts,
        diagnostics,
        fingerprint: load_order_fingerprint(sources),
    })
}

/// Configured-list validation (F39.1/F39.6): no duplicate plugin entries,
/// every declared master present in the list, and every master ordered
/// before its dependent.
fn validate_load_order(sources: &[PluginSource<'_>]) -> Result<()> {
    let mut seen = HashMap::<String, usize>::new();
    for (position, source) in sources.iter().enumerate() {
        let key = source.name.to_ascii_lowercase();
        if seen.contains_key(&key) {
            bail!(
                "duplicate plugin entry '{}' in configured load order",
                source.name
            )
        }
        seen.insert(key, position);
    }
    for (position, source) in sources.iter().enumerate() {
        let masters = read_master_names(source.bytes)
            .with_context(|| format!("reading masters from {}", source.name))?;
        for master in masters {
            let master_key = master.to_ascii_lowercase();
            match seen.get(&master_key) {
                None => bail!(
                    "'{}' requires master '{}', which is not in the configured load order",
                    source.name,
                    master
                ),
                Some(&master_position) if master_position >= position => bail!(
                    "'{}' loads before its master '{}'; invalid load order",
                    source.name,
                    master
                ),
                _ => {}
            }
        }
    }
    Ok(())
}

fn walk_plugin(
    bytes: &[u8],
    current_index: u8,
    master_indices: &[u8],
    mut on_record: impl FnMut(RawRecordEvent),
) -> Result<()> {
    walk_range(
        bytes,
        0,
        bytes.len(),
        current_index,
        master_indices,
        &mut on_record,
    )
}

fn walk_range(
    bytes: &[u8],
    mut offset: usize,
    end: usize,
    current_index: u8,
    master_indices: &[u8],
    on_record: &mut impl FnMut(RawRecordEvent),
) -> Result<()> {
    while offset + 4 <= end {
        let signature = &bytes[offset..offset + 4];
        if signature == b"GRUP" {
            if offset + 24 > end {
                bail!("truncated GRUP header")
            }
            let size = read_u32(bytes, offset + 4)? as usize;
            if size < 24 || offset + size > end {
                bail!("invalid GRUP size")
            }
            walk_range(
                bytes,
                offset + 24,
                offset + size,
                current_index,
                master_indices,
                on_record,
            )?;
            offset += size;
            continue;
        }

        if offset + 24 > end {
            bail!("truncated record header")
        }
        let data_size = read_u32(bytes, offset + 4)? as usize;
        let flags = read_u32(bytes, offset + 8)?;
        let raw_form_id = read_u32(bytes, offset + 12)?;
        let record_end = offset + 24 + data_size;
        if record_end > end {
            bail!("record exceeds containing group")
        }
        let record_type = String::from_utf8_lossy(signature).to_string();
        if record_type != "TES4" {
            let form_id = resolve_form_id(raw_form_id, current_index, master_indices);
            let editor_id = record_payload(
                &bytes[offset + 24..record_end],
                flags,
                &record_type,
                form_id,
            )
            .ok()
            .and_then(|data| parse_subrecords(&data).ok())
            .and_then(|subs| sub(&subs, "EDID").map(cstring));
            on_record(RawRecordEvent {
                form_id,
                record_type,
                editor_id,
                deleted: flags & RECORD_DELETED != 0,
            });
        }
        offset = record_end;
    }
    Ok(())
}

/// Mirrors `openmw_esm4::FormIdResolver::adjust`; duplicated here (five
/// lines) because that resolver's fields and method are private to the
/// reader module, and this slice must not reach into ESM4 parser internals
/// to get at them.
fn resolve_form_id(raw: u32, current_index: u8, master_indices: &[u8]) -> u32 {
    let local_file_index = (raw >> 24) as usize;
    let object_index = raw & 0x00ff_ffff;
    let global_file_index = master_indices
        .get(local_file_index)
        .copied()
        .unwrap_or(current_index);
    (u32::from(global_file_index) << 24) | object_index
}

fn is_well_formed_signature(signature: &str) -> bool {
    signature.len() == 4
        && signature
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
}

fn load_order_fingerprint(sources: &[PluginSource<'_>]) -> String {
    let mut hasher = Sha256::new();
    for source in sources {
        hasher.update(source.name.to_ascii_lowercase().as_bytes());
        hasher.update([0]);
        hasher.update((source.bytes.len() as u64).to_le_bytes());
        hasher.update(source.bytes);
        hasher.update([0]);
    }
    format!("{:x}", hasher.finalize())
}
