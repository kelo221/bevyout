//! Minimal Fallout NIF vertex-color mode inspection.
//!
//! This is deliberately limited to the record header and
//! `NiVertexColorProperty` fields needed by the asset conversion profile.  It
//! does not attempt to load geometry, materials, animation, OSG state, or
//! collision data.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VertexColorMode {
    Ignore,
    Emissive,
    AmbientDiffuse,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VertexColorSemantics {
    pub(crate) mode: VertexColorMode,
    pub(crate) property_count: usize,
    pub(crate) malformed: bool,
}

impl Default for VertexColorSemantics {
    fn default() -> Self {
        Self {
            mode: VertexColorMode::Unknown,
            property_count: 0,
            malformed: false,
        }
    }
}

const VER_OB_OLD: u32 = 0x0a00_0102;
const VER_OB: u32 = 0x1400_0005;
const VER_BGS: u32 = 0x1402_0007;
const VER_5_0_0_1: u32 = 0x0500_0001;
const VER_5_0_0_6: u32 = 0x0500_0006;
const VER_10_0_0_0: u32 = 0x0a00_0000;
const VER_10_0_1_8: u32 = 0x0a00_0108;
const VER_10_1_0_0: u32 = 0x0a01_0000;
const VER_10_2_0_0: u32 = 0x0a02_0000;
const VER_20_0_0_4: u32 = 0x1400_0004;
const VER_20_1_0_1: u32 = 0x1401_0001;
const VER_20_2_0_5: u32 = 0x1402_0005;
const VER_20_3_1_2: u32 = 0x1403_0102;

#[derive(Clone, Copy)]
struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8], offset: usize) -> Self {
        Self { bytes, offset }
    }

    fn take(&mut self, length: usize) -> Option<&'a [u8]> {
        let end = self.offset.checked_add(length)?;
        let value = self.bytes.get(self.offset..end)?;
        self.offset = end;
        Some(value)
    }

    fn u8(&mut self) -> Option<u8> {
        self.take(1).map(|bytes| bytes[0])
    }

    fn u16(&mut self) -> Option<u16> {
        self.take(2)
            .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn u32(&mut self) -> Option<u32> {
        let bytes = self.take(4)?;
        Some(u32::from_le_bytes(bytes.try_into().ok()?))
    }

    fn i32(&mut self) -> Option<i32> {
        let bytes = self.take(4)?;
        Some(i32::from_le_bytes(bytes.try_into().ok()?))
    }

    fn sized_string(&mut self) -> Option<&'a [u8]> {
        let length = usize::try_from(self.u32()?).ok()?;
        self.take(length)
    }

    fn export_string(&mut self) -> Option<&'a [u8]> {
        let length = usize::from(self.u8()?);
        self.take(length)
    }

    fn skip_u32s(&mut self, count: usize) -> Option<()> {
        self.take(count.checked_mul(4)?).map(|_| ())
    }
}

fn has_string_table(version: u32) -> bool {
    version >= VER_20_1_0_1
}

fn read_record_string(cursor: &mut Cursor<'_>, version: u32) -> Option<()> {
    if has_string_table(version) {
        cursor.u32().map(|_| ())
    } else {
        cursor.sized_string().map(|_| ())
    }
}

fn property_mode(payload: &[u8], version: u32) -> Option<VertexColorMode> {
    let mut cursor = Cursor::new(payload, 0);

    // NiObjectNET: name, extra list, controller.
    read_record_string(&mut cursor, version)?;
    let extra_count = cursor.u32().and_then(|value| usize::try_from(value).ok())?;
    if cursor.skip_u32s(extra_count).is_none() || cursor.i32().is_none() {
        return None;
    }

    let flags = cursor.u16()?;
    let (vertex_mode, lighting_mode) = if version <= VER_OB {
        (cursor.u32()?, cursor.u32()?)
    } else {
        (
            ((u32::from(flags) >> 4) & 0x3),
            ((u32::from(flags) >> 3) & 0x1),
        )
    };

    match vertex_mode {
        0 => Some(VertexColorMode::Ignore),
        1 => Some(VertexColorMode::Emissive),
        2 if lighting_mode == 1 => Some(VertexColorMode::AmbientDiffuse),
        2 => Some(VertexColorMode::Emissive),
        _ => Some(VertexColorMode::Unknown),
    }
}

fn combine_mode(current: Option<VertexColorMode>, next: VertexColorMode) -> VertexColorMode {
    match (current, next) {
        (_, VertexColorMode::Unknown) => VertexColorMode::Unknown,
        (None, mode) => mode,
        (Some(VertexColorMode::Unknown), _) => VertexColorMode::Unknown,
        (Some(VertexColorMode::AmbientDiffuse), _) => VertexColorMode::AmbientDiffuse,
        (_, VertexColorMode::AmbientDiffuse) => VertexColorMode::AmbientDiffuse,
        (Some(VertexColorMode::Emissive), _) => VertexColorMode::Emissive,
        (_, VertexColorMode::Emissive) => VertexColorMode::Emissive,
        _ => VertexColorMode::Ignore,
    }
}

fn inspect_records(bytes: &[u8]) -> Option<VertexColorSemantics> {
    let header_end = bytes.iter().position(|byte| *byte == b'\n')?;
    let mut cursor = Cursor::new(bytes, header_end + 1);
    let version = cursor.u32()?;

    if version >= VER_20_0_0_4 && cursor.u8()? == 0 {
        return None;
    }
    let user_version = if version >= VER_10_0_1_8 {
        cursor.u32()?
    } else {
        0
    };
    let record_count = usize::try_from(cursor.u32()?).ok()?;

    let has_bethesda_header = if version == VER_OB_OLD {
        true
    } else if user_version >= 3 && version >= VER_10_1_0_0 {
        (version <= VER_OB || version == VER_BGS) && (user_version <= 11 || version >= VER_OB)
    } else {
        false
    };
    if has_bethesda_header {
        let bethesda_version = cursor.u32()?;
        cursor.export_string()?;
        if bethesda_version >= 131 {
            cursor.u32()?;
        } else {
            cursor.export_string()?;
        }
        cursor.export_string()?;
        if bethesda_version >= 103 {
            cursor.export_string()?;
        }
    }

    if version < VER_5_0_0_1 || version == VER_20_3_1_2 {
        return None;
    }
    let type_count = usize::from(cursor.u16()?);
    let mut record_types = Vec::with_capacity(type_count);
    for _ in 0..type_count {
        record_types.push(cursor.sized_string()?);
    }
    let mut type_indices = Vec::with_capacity(record_count);
    for _ in 0..record_count {
        type_indices.push(usize::from(cursor.u16()?));
    }

    if version < VER_20_2_0_5 {
        return None;
    }
    let mut record_sizes = Vec::with_capacity(record_count);
    for _ in 0..record_count {
        record_sizes.push(usize::try_from(cursor.u32()?).ok()?);
    }

    if has_string_table(version) {
        let string_count = usize::try_from(cursor.u32()?).ok()?;
        cursor.u32()?;
        for _ in 0..string_count {
            cursor.sized_string()?;
        }
    }
    if version >= VER_5_0_0_6 {
        let group_count = usize::try_from(cursor.u32()?).ok()?;
        cursor.skip_u32s(group_count)?;
    }

    let has_separators = (VER_10_0_0_0..VER_10_2_0_0).contains(&version);
    let mut mode = None;
    let mut property_count = 0;
    let mut malformed = false;
    for (index, record_size) in record_sizes.into_iter().enumerate() {
        let type_index = *type_indices.get(index)?;
        let record_type = record_types.get(type_index)?;
        if has_separators && !record_type.starts_with(b"bhk") && cursor.i32()? != 0 {
            return None;
        }
        let payload = cursor.take(record_size)?;
        if record_type == b"NiVertexColorProperty" {
            property_count += 1;
            match property_mode(payload, version) {
                Some(next) => mode = Some(combine_mode(mode, next)),
                None => malformed = true,
            }
        }
    }

    Some(VertexColorSemantics {
        mode: if malformed {
            VertexColorMode::Unknown
        } else {
            mode.unwrap_or(VertexColorMode::Unknown)
        },
        property_count,
        malformed,
    })
}

pub(crate) fn inspect_nif_vertex_colors(bytes: &[u8]) -> VertexColorSemantics {
    inspect_records(bytes).unwrap_or(VertexColorSemantics {
        mode: VertexColorMode::Unknown,
        property_count: 0,
        malformed: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_u16(bytes: &mut Vec<u8>, value: u16) {
        bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn push_u32(bytes: &mut Vec<u8>, value: u32) {
        bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn push_sized(bytes: &mut Vec<u8>, value: &[u8]) {
        push_u32(bytes, value.len() as u32);
        bytes.extend_from_slice(value);
    }

    fn push_export(bytes: &mut Vec<u8>, value: &[u8]) {
        bytes.push(value.len() as u8);
        bytes.extend_from_slice(value);
    }

    fn fixture(vertex_mode: u32, lighting_mode: u32, truncate: bool) -> Vec<u8> {
        let mut bytes = b"Gamebryo File Format, Version 20.2.0.7\n".to_vec();
        push_u32(&mut bytes, VER_BGS);
        bytes.push(1);
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 34);
        push_export(&mut bytes, b"author");
        push_export(&mut bytes, b"process");
        push_export(&mut bytes, b"export");
        push_u16(&mut bytes, 1);
        push_sized(&mut bytes, b"NiVertexColorProperty");
        push_u16(&mut bytes, 0);

        let mut payload = Vec::new();
        push_u32(&mut payload, 0);
        push_u32(&mut payload, 0);
        push_u32(&mut payload, u32::MAX);
        let flags = ((vertex_mode & 0x3) << 4) | ((lighting_mode & 0x1) << 3);
        push_u16(&mut payload, flags as u16);
        push_u32(
            &mut bytes,
            if truncate {
                (payload.len() - 1) as u32
            } else {
                payload.len() as u32
            },
        );
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        if truncate {
            bytes.extend_from_slice(&payload[..payload.len() - 1]);
        } else {
            bytes.extend_from_slice(&payload);
        }
        push_u32(&mut bytes, 0);
        bytes
    }

    #[test]
    fn reads_all_vertex_color_modes() {
        assert_eq!(
            inspect_nif_vertex_colors(&fixture(0, 0, false)).mode,
            VertexColorMode::Ignore
        );
        assert_eq!(
            inspect_nif_vertex_colors(&fixture(1, 0, false)).mode,
            VertexColorMode::Emissive
        );
        assert_eq!(
            inspect_nif_vertex_colors(&fixture(2, 1, false)).mode,
            VertexColorMode::AmbientDiffuse
        );
        assert_eq!(
            inspect_nif_vertex_colors(&fixture(2, 0, false)).mode,
            VertexColorMode::Emissive
        );
    }

    #[test]
    fn malformed_property_is_unknown() {
        let semantics = inspect_nif_vertex_colors(&fixture(2, 1, true));
        assert_eq!(semantics.mode, VertexColorMode::Unknown);
        assert!(semantics.malformed);
    }

    #[test]
    fn files_without_the_property_are_ambiguous_but_not_malformed() {
        let mut bytes = fixture(0, 0, false);
        let type_name = b"NiVertexColorProperty";
        let start = bytes
            .windows(type_name.len())
            .position(|window| window == type_name)
            .unwrap();
        bytes[start..start + type_name.len()].fill(b'X');
        let semantics = inspect_nif_vertex_colors(&bytes);
        assert_eq!(semantics.mode, VertexColorMode::Unknown);
        assert_eq!(semantics.property_count, 0);
        assert!(!semantics.malformed);
    }
}
