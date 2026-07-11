use anyhow::{Context, Result, anyhow, bail};
use flate2::read::ZlibDecoder;
use std::collections::HashMap;
use std::io::{Cursor, Read};

use super::manifest::CellInfo;

const RECORD_COMPRESSED: u32 = 0x0004_0000;
pub(crate) const RECORD_DELETED: u32 = 0x0000_0020;
pub(crate) const RECORD_DISABLED: u32 = 0x0000_0800;

#[derive(Debug, Clone)]
pub(crate) struct BaseRecord {
    pub(crate) kind: String,
    pub(crate) model: Option<String>,
    pub(crate) light: Option<LightData>,
}

#[derive(Debug, Clone)]
pub(crate) struct LightData {
    pub(crate) radius: f32,
    pub(crate) color_rgba: [f32; 4],
}

#[derive(Debug, Clone)]
pub(crate) struct ReferenceRecord {
    pub(crate) form_id: u32,
    pub(crate) base_form_id: u32,
    pub(crate) position: [f32; 3],
    pub(crate) rotation: [f32; 3],
    pub(crate) scale: f32,
    pub(crate) flags: u32,
}

#[derive(Debug, Default)]
pub(crate) struct ParsedPlugin {
    pub(crate) bases: HashMap<u32, BaseRecord>,
    pub(crate) references: Vec<ReferenceRecord>,
    pub(crate) cell: Option<CellInfo>,
}

pub(crate) fn parse_plugin(bytes: &[u8], target_cell: u32) -> Result<ParsedPlugin> {
    let mut parsed = ParsedPlugin::default();
    walk_container(bytes, 0, bytes.len(), target_cell, false, &mut parsed)?;
    Ok(parsed)
}

fn walk_container(
    bytes: &[u8],
    mut offset: usize,
    end: usize,
    target_cell: u32,
    in_target_cell: bool,
    parsed: &mut ParsedPlugin,
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
            let label = read_u32(bytes, offset + 8)?;
            let group_type = read_i32(bytes, offset + 12)?;
            let child_context = in_target_cell || (group_type == 6 && label == target_cell);
            walk_container(
                bytes,
                offset + 24,
                offset + size,
                target_cell,
                child_context,
                parsed,
            )?;
            offset += size;
        } else {
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
            let mut data = bytes[offset + 24..record_end].to_vec();
            if flags & RECORD_COMPRESSED != 0 {
                if data.len() < 4 {
                    bail!("compressed record is missing decompressed size")
                }
                let mut decoder = ZlibDecoder::new(Cursor::new(&data[4..]));
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed).map_err(|error| {
                    anyhow!("decompressing {sig} record {form_id:08x}: {error}")
                })?;
                data = decompressed;
            }
            let subs = parse_subrecords(&data)?;
            if sig == "CELL" && form_id == target_cell {
                parsed.cell = Some(parse_cell(&subs, form_id)?);
            }
            if sig == "REFR"
                && in_target_cell
                && let Some(reference) = parse_reference(&subs, form_id, flags)?
            {
                parsed.references.push(reference);
            }
            if !in_target_cell && let Some(base) = parse_base(&sig, &subs) {
                parsed.bases.insert(form_id, base);
            }
            offset = record_end;
        }
    }
    Ok(())
}

fn parse_subrecords(data: &[u8]) -> Result<Vec<(String, Vec<u8>)>> {
    let mut result = Vec::new();
    let mut offset = 0;
    let mut extended = None;
    while offset < data.len() {
        if offset + 6 > data.len() {
            bail!("truncated subrecord")
        }
        let sig = String::from_utf8_lossy(&data[offset..offset + 4]).to_string();
        let size = u16::from_le_bytes(data[offset + 4..offset + 6].try_into().unwrap()) as usize;
        offset += 6;
        let actual_size = if sig == "XXXX" {
            if size != 4 || offset + 4 > data.len() {
                bail!("invalid XXXX")
            }
            let extended_size = read_u32(data, offset)? as usize;
            offset += 4;
            extended = Some(extended_size);
            continue;
        } else {
            extended.take().unwrap_or(size)
        };
        if offset + actual_size > data.len() {
            bail!("subrecord exceeds record")
        }
        result.push((sig, data[offset..offset + actual_size].to_vec()));
        offset += actual_size;
    }
    Ok(result)
}

fn parse_base(sig: &str, subs: &[(String, Vec<u8>)]) -> Option<BaseRecord> {
    let model = subs
        .iter()
        .find(|(name, _)| name == "MODL")
        .map(|(_, data)| cstring(data));
    let light = (sig == "LIGH").then(|| parse_light_data(subs)).flatten();
    if model.is_some() || sig == "LIGH" {
        Some(BaseRecord {
            kind: sig.to_string(),
            model,
            light,
        })
    } else {
        None
    }
}

fn parse_light_data(subs: &[(String, Vec<u8>)]) -> Option<LightData> {
    let data = subs.iter().find(|(name, _)| name == "DATA")?.1.as_slice();
    if data.len() < 12 {
        return None;
    }
    let radius = u32::from_le_bytes(data[4..8].try_into().ok()?) as f32;
    let color_rgba = [
        data[8] as f32 / 255.0,
        data[9] as f32 / 255.0,
        data[10] as f32 / 255.0,
        1.0,
    ];
    Some(LightData { radius, color_rgba })
}

fn parse_cell(subs: &[(String, Vec<u8>)], form_id: u32) -> Result<CellInfo> {
    let editor_id = subs
        .iter()
        .find(|(name, _)| name == "EDID")
        .map(|(_, data)| cstring(data));
    let name = subs
        .iter()
        .find(|(name, _)| name == "FULL")
        .map(|(_, data)| cstring(data));
    let interior = subs
        .iter()
        .find(|(name, _)| name == "DATA")
        .and_then(|(_, data)| data.first())
        .map(|flags| flags & 1 != 0)
        .unwrap_or(false);
    let mut ambient = [0.18, 0.18, 0.18, 1.0];
    let mut directional = [0.8, 0.8, 0.8, 1.0];
    if let Some((_, data)) = subs.iter().find(|(name, _)| name == "XCLL")
        && data.len() >= 8
    {
        ambient = [
            data[0] as f32 / 255.0,
            data[1] as f32 / 255.0,
            data[2] as f32 / 255.0,
            data[3] as f32 / 255.0,
        ];
        directional = [
            data[4] as f32 / 255.0,
            data[5] as f32 / 255.0,
            data[6] as f32 / 255.0,
            data[7] as f32 / 255.0,
        ];
    }
    Ok(CellInfo {
        form_id,
        editor_id,
        name,
        interior,
        ambient_rgba: ambient,
        directional_rgba: directional,
    })
}

fn parse_reference(
    subs: &[(String, Vec<u8>)],
    form_id: u32,
    flags: u32,
) -> Result<Option<ReferenceRecord>> {
    let Some((_, name)) = subs.iter().find(|(sig, _)| sig == "NAME") else {
        return Ok(None);
    };
    if name.len() < 4 {
        return Ok(None);
    }
    let Some((_, data)) = subs.iter().find(|(sig, _)| sig == "DATA") else {
        return Ok(None);
    };
    if data.len() < 24 {
        return Ok(None);
    }
    let position = [f32_at(data, 0)?, f32_at(data, 4)?, f32_at(data, 8)?];
    let rotation = [f32_at(data, 12)?, f32_at(data, 16)?, f32_at(data, 20)?];
    let scale = subs
        .iter()
        .find(|(sig, _)| sig == "XSCL")
        .and_then(|(_, d)| d.get(..4))
        .map(|d| f32::from_le_bytes(d.try_into().unwrap()))
        .unwrap_or(1.0);
    Ok(Some(ReferenceRecord {
        form_id,
        base_form_id: u32::from_le_bytes(name[0..4].try_into().unwrap()),
        position,
        rotation,
        scale,
        flags,
    }))
}

fn cstring(data: &[u8]) -> String {
    String::from_utf8_lossy(data)
        .trim_end_matches('\0')
        .to_string()
}

fn read_u32(data: &[u8], offset: usize) -> Result<u32> {
    data.get(offset..offset + 4)
        .context("u32 out of bounds")
        .map(|d| u32::from_le_bytes(d.try_into().unwrap()))
}

fn read_i32(data: &[u8], offset: usize) -> Result<i32> {
    Ok(read_u32(data, offset)? as i32)
}

fn f32_at(data: &[u8], offset: usize) -> Result<f32> {
    data.get(offset..offset + 4)
        .context("f32 out of bounds")
        .map(|d| f32::from_le_bytes(d.try_into().unwrap()))
}
