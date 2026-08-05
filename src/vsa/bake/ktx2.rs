//! Minimal in-tree KTX2 writers for prepared bake outputs.
//!
//! The CPU baker keeps its raw texel layout explicit and writes the final
//! container here so a prepared bake does not depend on an installed KTX
//! executable. The first writer targets the uncompressed RGBA16F lightmap
//! format; mip generation is intentionally left to the runtime because the
//! lightmap shader currently samples the single authored level.

use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;

const RGBA16F_BYTES_PER_TEXEL: usize = 8;

pub(crate) fn write_rgba16f(
    raw_path: &Path,
    output_path: &Path,
    width: u32,
    height: u32,
) -> Result<()> {
    let data = fs::read(raw_path)
        .with_context(|| format!("could not read raw KTX2 data {}", raw_path.display()))?;
    let expected = (width as usize)
        .checked_mul(height as usize)
        .and_then(|pixels| pixels.checked_mul(RGBA16F_BYTES_PER_TEXEL))
        .context("RGBA16F KTX2 image size overflowed")?;
    if data.len() != expected {
        bail!(
            "raw RGBA16F KTX2 data has {} bytes, expected {}",
            data.len(),
            expected
        );
    }

    write_uncompressed(
        output_path,
        ::ktx2::Format::R16G16B16A16_SFLOAT,
        2,
        width,
        height,
        0,
        data,
    )
}

pub(crate) fn write_rgb9e5_volume(
    raw_paths: &[impl AsRef<Path>],
    output_path: &Path,
    width: u32,
    height: u32,
    depth: u32,
) -> Result<()> {
    let expected_slice = (width as usize)
        .checked_mul(height as usize)
        .and_then(|pixels| pixels.checked_mul(4))
        .context("RGB9E5 KTX2 slice size overflowed")?;
    if raw_paths.len() != depth as usize {
        bail!(
            "RGB9E5 KTX2 volume has {} raw slices, expected {}",
            raw_paths.len(),
            depth
        );
    }
    let expected_total = expected_slice
        .checked_mul(depth as usize)
        .context("RGB9E5 KTX2 volume size overflowed")?;
    let mut data = Vec::with_capacity(expected_total);
    for raw_path in raw_paths {
        let raw_path = raw_path.as_ref();
        let slice = fs::read(raw_path)
            .with_context(|| format!("could not read raw KTX2 data {}", raw_path.display()))?;
        if slice.len() != expected_slice {
            bail!(
                "raw RGB9E5 KTX2 slice {} has {} bytes, expected {}",
                raw_path.display(),
                slice.len(),
                expected_slice
            );
        }
        data.extend_from_slice(&slice);
    }
    write_uncompressed(
        output_path,
        ::ktx2::Format::E5B9G9R9_UFLOAT_PACK32,
        4,
        width,
        height,
        depth,
        data,
    )
}

fn write_uncompressed(
    output_path: &Path,
    format: ::ktx2::Format,
    type_size: u32,
    width: u32,
    height: u32,
    depth: u32,
    data: Vec<u8>,
) -> Result<()> {
    let (basic_dfd, generated_type_size) = ::ktx2::dfd::Basic::from_format(format)
        .map_err(|error| anyhow::anyhow!("could not build KTX2 DFD: {error}"))?;
    if generated_type_size != type_size {
        bail!(
            "KTX2 format {:?} requires type size {}, got {}",
            format,
            generated_type_size,
            type_size
        );
    }
    let dfd_block = ::ktx2::dfd::Block::Basic(basic_dfd).to_vec();
    let dfd_byte_length = 4usize
        .checked_add(dfd_block.len())
        .context("KTX2 DFD size overflowed")?;
    let dfd_byte_offset = (::ktx2::Header::LENGTH + ::ktx2::LevelIndex::LENGTH) as u32;
    let level_byte_offset =
        (::ktx2::Header::LENGTH + ::ktx2::LevelIndex::LENGTH + dfd_byte_length) as u64;
    let header = ::ktx2::Header {
        format: Some(format),
        type_size,
        pixel_width: width,
        pixel_height: height,
        pixel_depth: depth,
        layer_count: 0,
        face_count: 1,
        level_count: 1,
        supercompression_scheme: None,
        index: ::ktx2::Index {
            dfd_byte_offset,
            dfd_byte_length: dfd_byte_length as u32,
            kvd_byte_offset: 0,
            kvd_byte_length: 0,
            sgd_byte_offset: 0,
            sgd_byte_length: 0,
        },
    };
    let level = ::ktx2::LevelIndex {
        byte_offset: level_byte_offset,
        byte_length: data.len() as u64,
        uncompressed_byte_length: data.len() as u64,
    };

    let mut encoded = Vec::with_capacity(level_byte_offset as usize + data.len());
    encoded.extend_from_slice(&header.as_bytes());
    encoded.extend_from_slice(&level.as_bytes());
    encoded.extend_from_slice(&(dfd_byte_length as u32).to_le_bytes());
    encoded.extend_from_slice(&dfd_block);
    encoded.extend_from_slice(&data);
    fs::write(output_path, encoded)
        .with_context(|| format!("could not write KTX2 output {}", output_path.display()))?;
    Ok(())
}
