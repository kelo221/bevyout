//! Raw ESM4 byte and subrecord decoding.

use super::*;

pub(crate) fn read_master_names(bytes: &[u8]) -> Result<Vec<String>> {
    if bytes.len() < 24 || &bytes[0..4] != b"TES4" {
        bail!("plugin does not start with a TES4 header")
    }
    let size = read_u32(bytes, 4)? as usize;
    let end = 24usize
        .checked_add(size)
        .filter(|end| *end <= bytes.len())
        .context("TES4 header exceeds plugin")?;
    Ok(parse_subrecords(&bytes[24..end])?
        .into_iter()
        .filter(|sub| sub.signature == "MAST")
        .map(|sub| cstring(&sub.data))
        .collect())
}

pub(crate) fn record_payload(data: &[u8], flags: u32, sig: &str, form_id: u32) -> Result<Vec<u8>> {
    if flags & RECORD_COMPRESSED == 0 {
        return Ok(data.to_vec());
    }
    if data.len() < 4 {
        bail!("compressed record is missing decompressed size")
    }
    let expected = read_u32(data, 0)? as usize;
    let mut decoder = ZlibDecoder::new(Cursor::new(&data[4..]));
    let mut decompressed = Vec::with_capacity(expected);
    decoder
        .read_to_end(&mut decompressed)
        .map_err(|error| anyhow!("decompressing {sig} record {form_id:08x}: {error}"))?;
    if decompressed.len() != expected {
        bail!(
            "decompressed {sig} record {form_id:08x} to {} bytes, expected {expected}",
            decompressed.len()
        )
    }
    Ok(decompressed)
}

pub(crate) fn parse_subrecords(data: &[u8]) -> Result<Vec<Subrecord>> {
    let mut result = Vec::new();
    let mut offset = 0;
    let mut extended = None;
    while offset < data.len() {
        if offset + 6 > data.len() {
            bail!("truncated subrecord")
        }
        let signature = String::from_utf8_lossy(&data[offset..offset + 4]).to_string();
        let size = u16::from_le_bytes(data[offset + 4..offset + 6].try_into().unwrap()) as usize;
        offset += 6;
        if signature == "XXXX" {
            if size != 4 || offset + 4 > data.len() {
                bail!("invalid XXXX")
            }
            extended = Some(read_u32(data, offset)? as usize);
            offset += 4;
            continue;
        }
        let actual_size = extended.take().unwrap_or(size);
        if offset + actual_size > data.len() {
            bail!("subrecord exceeds record")
        }
        result.push(Subrecord {
            signature,
            data: data[offset..offset + actual_size].to_vec(),
        });
        offset += actual_size;
    }
    Ok(result)
}

pub(crate) fn sub<'a>(subs: &'a [Subrecord], signature: &str) -> Option<&'a [u8]> {
    subs.iter()
        .find(|sub| sub.signature == signature)
        .map(|sub| sub.data.as_slice())
}

pub(crate) fn sub_form_id(
    subs: &[Subrecord],
    signature: &str,
    resolver: &FormIdResolver,
) -> Option<u32> {
    sub(subs, signature)
        .and_then(|data| data.get(..4))
        .map(|data| resolver.adjust(u32::from_le_bytes(data.try_into().unwrap())))
        .filter(|id| *id != 0)
}

pub(crate) fn cstring(data: &[u8]) -> String {
    String::from_utf8_lossy(data)
        .trim_end_matches('\0')
        .to_string()
}

pub(crate) fn rgba8(data: &[u8]) -> [f32; 4] {
    [
        data[0] as f32 / 255.0,
        data[1] as f32 / 255.0,
        data[2] as f32 / 255.0,
        data[3] as f32 / 255.0,
    ]
}

pub(crate) fn read_u32(data: &[u8], offset: usize) -> Result<u32> {
    data.get(offset..offset + 4)
        .context("u32 out of bounds")
        .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
}

pub(crate) fn read_i32(data: &[u8], offset: usize) -> Result<i32> {
    Ok(read_u32(data, offset)? as i32)
}

pub(crate) fn f32_at(data: &[u8], offset: usize) -> Result<f32> {
    data.get(offset..offset + 4)
        .context("f32 out of bounds")
        .map(|bytes| f32::from_le_bytes(bytes.try_into().unwrap()))
}

pub(crate) fn i32_at(data: &[u8], offset: usize) -> Option<i32> {
    data.get(offset..offset + 4)
        .map(|bytes| i32::from_le_bytes(bytes.try_into().unwrap()))
}

pub(crate) fn i16_at(data: &[u8], offset: usize) -> Option<i16> {
    data.get(offset..offset + 2)
        .map(|bytes| i16::from_le_bytes(bytes.try_into().unwrap()))
}

pub(crate) fn u32_at(data: &[u8], offset: usize) -> Option<u32> {
    data.get(offset..offset + 4)
        .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
}

pub(crate) fn f32_at_option(data: &[u8], offset: usize) -> Option<f32> {
    data.get(offset..offset + 4)
        .map(|bytes| f32::from_le_bytes(bytes.try_into().unwrap()))
}

pub(crate) fn f32_or(data: &[u8], offset: usize, fallback: f32) -> f32 {
    f32_at_option(data, offset).unwrap_or(fallback)
}

pub(crate) fn rgb_or(data: &[u8], offset: usize, fallback: [f32; 3]) -> [f32; 3] {
    [
        f32_or(data, offset, fallback[0]),
        f32_or(data, offset + 4, fallback[1]),
        f32_or(data, offset + 8, fallback[2]),
    ]
}
