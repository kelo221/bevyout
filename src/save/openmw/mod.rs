//! Small record/subrecord primitives used by the save slice.
//!
//! The organization follows OpenMW's ESM save reader/writer: a stream of
//! length-delimited FourCC records, each of which may contain tagged
//! subrecords. This module intentionally contains only the isolated format
//! primitives; bevyout's state model lives in the parent module.

use anyhow::{Result, bail};

pub(crate) const MAX_RECORD_BYTES: usize = 64 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Record {
    pub(crate) tag: [u8; 4],
    pub(crate) payload: Vec<u8>,
    pub(crate) raw: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Subrecord {
    pub(crate) tag: [u8; 4],
    pub(crate) payload: Vec<u8>,
}

pub(crate) fn write_record(output: &mut Vec<u8>, tag: [u8; 4], payload: &[u8]) -> Result<()> {
    write_framed(output, tag, payload)
}

pub(crate) fn write_subrecord(output: &mut Vec<u8>, tag: [u8; 4], payload: &[u8]) -> Result<()> {
    write_framed(output, tag, payload)
}

fn write_framed(output: &mut Vec<u8>, tag: [u8; 4], payload: &[u8]) -> Result<()> {
    if payload.len() > MAX_RECORD_BYTES || payload.len() > u32::MAX as usize {
        bail!("record {:?} payload is too large", tag);
    }
    output.extend_from_slice(&tag);
    output.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    output.extend_from_slice(payload);
    Ok(())
}

pub(crate) fn read_records(bytes: &[u8]) -> Result<Vec<Record>> {
    read_framed(bytes, |tag, payload| Subrecord { tag, payload }).map(|subrecords| {
        subrecords
            .into_iter()
            .map(|subrecord| Record {
                raw: framed_bytes(subrecord.tag, &subrecord.payload),
                tag: subrecord.tag,
                payload: subrecord.payload,
            })
            .collect()
    })
}

pub(crate) fn read_subrecords(bytes: &[u8]) -> Result<Vec<Subrecord>> {
    read_framed(bytes, |tag, payload| Subrecord { tag, payload })
}

fn read_framed<T>(bytes: &[u8], mut map: impl FnMut([u8; 4], Vec<u8>) -> T) -> Result<Vec<T>> {
    let mut offset = 0;
    let mut result = Vec::new();
    while offset < bytes.len() {
        if bytes.len() - offset < 8 {
            bail!("truncated FourCC frame header at byte {offset}");
        }
        let tag = bytes[offset..offset + 4]
            .try_into()
            .expect("FourCC slice has exact length");
        let payload_len = u32::from_le_bytes(
            bytes[offset + 4..offset + 8]
                .try_into()
                .expect("length slice has exact length"),
        ) as usize;
        if payload_len > MAX_RECORD_BYTES {
            bail!("FourCC frame {:?} exceeds the size limit", tag);
        }
        let payload_start = offset + 8;
        let payload_end = payload_start
            .checked_add(payload_len)
            .ok_or_else(|| anyhow::anyhow!("FourCC frame length overflow"))?;
        if payload_end > bytes.len() {
            bail!("FourCC frame {:?} exceeds the containing stream", tag);
        }
        result.push(map(tag, bytes[payload_start..payload_end].to_vec()));
        offset = payload_end;
    }
    Ok(result)
}

fn framed_bytes(tag: [u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(8 + payload.len());
    bytes.extend_from_slice(&tag);
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

pub(crate) fn tag(value: &str) -> [u8; 4] {
    let bytes = value.as_bytes();
    assert!(bytes.len() == 4, "save tags must be four ASCII bytes");
    [bytes[0], bytes[1], bytes[2], bytes[3]]
}
