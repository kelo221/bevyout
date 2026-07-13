//! Whole-plugin record/subrecord signature inventory.
//!
//! Unlike [`super::parse_content_set`], which only retains records the typed
//! importer understands, this walk tallies *every* top-level record and
//! subrecord signature encountered in a plugin, independent of whether
//! bevyout currently supports it. It exists so the compatibility-report
//! tooling slice (`src/vsa/report/`) can see undeclared/unsupported content
//! instead of having it silently dropped by the typed reader.

use super::*;

/// One record instance as seen by the inventory walk: its four-character
/// signature, raw (unresolved) FormID, and the signature/payload of every
/// subrecord it carries.
#[derive(Debug, Clone)]
pub(crate) struct RecordSignature {
    pub(crate) kind: String,
    pub(crate) form_id: u32,
    pub(crate) subrecords: Vec<(String, Vec<u8>)>,
}

/// Walks every GRUP/record in `bytes` and returns one [`RecordSignature`]
/// per non-deleted record, in file order.
pub(crate) fn inventory_records(bytes: &[u8]) -> Result<Vec<RecordSignature>> {
    let mut records = Vec::new();
    walk_inventory(bytes, 0, bytes.len(), &mut records)?;
    Ok(records)
}

fn walk_inventory(
    bytes: &[u8],
    mut offset: usize,
    end: usize,
    out: &mut Vec<RecordSignature>,
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
            walk_inventory(bytes, offset + 24, offset + size, out)?;
            offset += size;
            continue;
        }

        if offset + 24 > end {
            bail!("truncated record header")
        }
        let data_size = read_u32(bytes, offset + 4)? as usize;
        let flags = read_u32(bytes, offset + 8)?;
        let form_id = read_u32(bytes, offset + 12)?;
        let record_end = offset + 24 + data_size;
        if record_end > end {
            bail!("record exceeds containing group")
        }
        let sig = String::from_utf8_lossy(signature).to_string();
        if flags & RECORD_DELETED == 0 {
            let payload = record_payload(&bytes[offset + 24..record_end], flags, &sig, form_id)?;
            let subrecords = parse_subrecords(&payload)?
                .into_iter()
                .map(|subrecord| (subrecord.signature, subrecord.data))
                .collect();
            out.push(RecordSignature {
                kind: sig,
                form_id,
                subrecords,
            });
        }
        offset = record_end;
    }
    Ok(())
}
