//! Reusable load-order-aware ESM4 record traversal.
//!
//! Payload borrows are valid only for the callback invocation. This permits
//! compressed records to be decoded once without retaining arbitrary payloads
//! in `ContentIndex` or forcing later collectors to rescan the load order.

use std::collections::HashMap;

use anyhow::{Context, Result, bail};
use bevyout_core::form_id::{FormId, FormIdResolver};
use sha2::{Digest, Sha256};

use super::openmw_esm4::{RECORD_DELETED, read_u32, record_payload};
use super::plugin::{PluginSource, read_master_names};

pub(crate) mod winners;

#[derive(Debug, Clone, Copy)]
pub(crate) enum RecordPayload<'a> {
    Decoded(&'a [u8]),
    Unavailable(&'a str),
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RecordEnvelope<'a> {
    pub(crate) form_id: FormId,
    pub(crate) signature: &'a str,
    // Retained for script and compatibility collectors introduced later in
    // this wave; ContentIndex itself has no flag-specific metadata.
    #[allow(dead_code)]
    pub(crate) flags: u32,
    pub(crate) source_plugin: &'a str,
    pub(crate) payload: RecordPayload<'a>,
    pub(crate) deleted: bool,
    #[allow(dead_code)]
    resolver: &'a FormIdResolver,
}

impl RecordEnvelope<'_> {
    /// Resolve a payload-local FormID through this record's plugin master
    /// table. Keeping the resolver private prevents plugin-relative IDs from
    /// leaking past the callback-scoped preparation boundary.
    #[allow(dead_code)]
    pub(crate) fn resolve_form_id(&self, raw: u32) -> FormId {
        self.resolver.resolve(FormId(raw))
    }
}

pub(crate) fn walk_resolved_records(
    sources: &[PluginSource<'_>],
    mut on_record: impl FnMut(RecordEnvelope<'_>),
) -> Result<()> {
    validate_load_order(sources)?;
    let positions = sources
        .iter()
        .enumerate()
        .map(|(index, source)| (source.name.to_ascii_lowercase(), index))
        .collect::<HashMap<_, _>>();

    for (index, source) in sources.iter().enumerate() {
        let masters = read_master_names(source.bytes)
            .with_context(|| format!("reading masters from {}", source.name))?;
        let master_indices = masters
            .iter()
            .map(|master| {
                positions
                    .get(&master.to_ascii_lowercase())
                    .copied()
                    .unwrap_or(index) as u8
            })
            .collect();
        let resolver = FormIdResolver::new(index as u8, master_indices);
        walk_plugin(source, &resolver, &mut on_record)
            .with_context(|| format!("parsing {}", source.name))?;
    }
    Ok(())
}

fn walk_plugin(
    source: &PluginSource<'_>,
    resolver: &FormIdResolver,
    on_record: &mut impl FnMut(RecordEnvelope<'_>),
) -> Result<()> {
    walk_range(
        source.bytes,
        0,
        source.bytes.len(),
        source.name,
        resolver,
        on_record,
    )
}

fn walk_range(
    bytes: &[u8],
    mut offset: usize,
    end: usize,
    source_plugin: &str,
    resolver: &FormIdResolver,
    on_record: &mut impl FnMut(RecordEnvelope<'_>),
) -> Result<()> {
    while offset + 4 <= end {
        let signature_bytes = &bytes[offset..offset + 4];
        if signature_bytes == b"GRUP" {
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
                source_plugin,
                resolver,
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
        let signature = String::from_utf8_lossy(signature_bytes);
        if signature != "TES4" {
            let form_id = FormId(resolver.adjust(raw_form_id));
            match record_payload(
                &bytes[offset + 24..record_end],
                flags,
                &signature,
                form_id.0,
            ) {
                Ok(payload) => on_record(RecordEnvelope {
                    form_id,
                    signature: &signature,
                    flags,
                    source_plugin,
                    payload: RecordPayload::Decoded(&payload),
                    deleted: flags & RECORD_DELETED != 0,
                    resolver,
                }),
                Err(error) => {
                    let diagnostic = format!(
                        "{} record {} in {} has unavailable payload: {error}",
                        signature, form_id, source_plugin
                    );
                    on_record(RecordEnvelope {
                        form_id,
                        signature: &signature,
                        flags,
                        source_plugin,
                        payload: RecordPayload::Unavailable(&diagnostic),
                        deleted: flags & RECORD_DELETED != 0,
                        resolver,
                    });
                }
            }
        }
        offset = record_end;
    }
    Ok(())
}

fn validate_load_order(sources: &[PluginSource<'_>]) -> Result<()> {
    if sources.len() > 256 {
        bail!("ESM4 content set exceeds the 256-file FormID limit")
    }
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
            match seen.get(&master.to_ascii_lowercase()) {
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

pub(crate) fn content_fingerprint(sources: &[PluginSource<'_>]) -> String {
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
