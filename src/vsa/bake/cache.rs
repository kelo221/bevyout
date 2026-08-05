//! Deterministic, resumable surface-lightmap tile cache.
//!
//! The cache stores raw pre-denoise tile payloads. Its identity is supplied by
//! the caller and is deliberately independent of denoiser and KTX encoding
//! settings, so those later stages can be changed without retracing transport.

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

const META_MAGIC: &[u8; 8] = b"BVOACM01";
const TILE_MAGIC: &[u8; 8] = b"BVOACT01";
const FORMAT_VERSION: u32 = 2;
const CHECKSUM_SIZE: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TileRecord {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TileKey {
    pub(crate) primitive: usize,
    pub(crate) tile_x: u32,
    pub(crate) tile_y: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct TileCacheStats {
    pub(crate) hits: usize,
    pub(crate) misses: usize,
    pub(crate) writes: usize,
}

pub(crate) struct TileCache {
    root: PathBuf,
    stats: TileCacheStats,
}

impl TileCache {
    pub(crate) fn open(root: &Path, fingerprint: &str, clear: bool) -> Result<Self> {
        fs::create_dir_all(root)
            .with_context(|| format!("creating lightmap accumulation cache {}", root.display()))?;
        let meta_path = root.join("cache.meta");
        let stored_fingerprint = if clear {
            None
        } else {
            read_metadata(&meta_path)?
        };
        if clear || stored_fingerprint.as_deref() != Some(fingerprint) {
            clear_cache_entries(root)?;
            write_metadata(&meta_path, fingerprint)?;
        }
        Ok(Self {
            root: root.to_path_buf(),
            stats: TileCacheStats::default(),
        })
    }

    pub(crate) fn read(&mut self, key: TileKey, fingerprint: &str) -> Result<Option<TileRecord>> {
        let path = self.tile_path(key);
        if !path.is_file() {
            self.stats.misses += 1;
            return Ok(None);
        }
        let bytes = fs::read(&path)
            .with_context(|| format!("reading lightmap accumulation tile {}", path.display()))?;
        let record = decode_tile(&bytes, fingerprint, key)
            .with_context(|| format!("validating lightmap accumulation tile {}", path.display()))?;
        let Some(record) = record else {
            self.stats.misses += 1;
            return Ok(None);
        };
        self.stats.hits += 1;
        Ok(Some(record))
    }

    pub(crate) fn write(
        &mut self,
        key: TileKey,
        fingerprint: &str,
        width: u32,
        height: u32,
        payload: &[u8],
    ) -> Result<()> {
        let path = self.tile_path(key);
        let bytes = encode_tile(
            fingerprint,
            key.primitive,
            key.tile_x,
            key.tile_y,
            width,
            height,
            payload,
        )?;
        atomic_write(&path, &bytes)?;
        self.stats.writes += 1;
        Ok(())
    }

    pub(crate) fn stats(&self) -> TileCacheStats {
        self.stats
    }

    pub(crate) fn tile_path(&self, key: TileKey) -> PathBuf {
        self.root.join(format!(
            "page_{:04}_tile_{:04}_{:04}.bin",
            key.primitive, key.tile_x, key.tile_y
        ))
    }
}

fn read_metadata(path: &Path) -> Result<Option<String>> {
    if !path.is_file() {
        return Ok(None);
    }
    let bytes = fs::read(path)
        .with_context(|| format!("reading lightmap cache metadata {}", path.display()))?;
    if bytes.len() < META_MAGIC.len() + 4 || &bytes[..META_MAGIC.len()] != META_MAGIC {
        bail!("lightmap cache metadata has an invalid header");
    }
    let version = read_u32(&bytes, META_MAGIC.len())?;
    if version != FORMAT_VERSION {
        return Ok(None);
    }
    let length = read_u32(&bytes, META_MAGIC.len() + 4)? as usize;
    let start = META_MAGIC.len() + 8;
    let end = start
        .checked_add(length)
        .context("lightmap cache fingerprint length overflowed")?;
    if end != bytes.len() {
        bail!("lightmap cache metadata is truncated or has trailing bytes");
    }
    Ok(Some(
        String::from_utf8(bytes[start..end].to_vec())
            .context("lightmap cache fingerprint is not UTF-8")?,
    ))
}

fn write_metadata(path: &Path, fingerprint: &str) -> Result<()> {
    let fingerprint = fingerprint.as_bytes();
    let mut bytes = Vec::with_capacity(META_MAGIC.len() + 8 + fingerprint.len());
    bytes.extend_from_slice(META_MAGIC);
    bytes.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    bytes.extend_from_slice(
        &u32::try_from(fingerprint.len())
            .context("lightmap cache fingerprint is too long")?
            .to_le_bytes(),
    );
    bytes.extend_from_slice(fingerprint);
    atomic_write(path, &bytes)
}

fn encode_tile(
    fingerprint: &str,
    primitive: usize,
    tile_x: u32,
    tile_y: u32,
    width: u32,
    height: u32,
    payload: &[u8],
) -> Result<Vec<u8>> {
    let fingerprint = fingerprint.as_bytes();
    let primitive = u32::try_from(primitive).context("lightmap primitive index exceeds u32")?;
    let payload_length = u64::try_from(payload.len()).context("lightmap tile is too large")?;
    let mut header = Vec::with_capacity(56 + fingerprint.len());
    header.extend_from_slice(TILE_MAGIC);
    header.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    header.extend_from_slice(&primitive.to_le_bytes());
    header.extend_from_slice(&tile_x.to_le_bytes());
    header.extend_from_slice(&tile_y.to_le_bytes());
    header.extend_from_slice(&width.to_le_bytes());
    header.extend_from_slice(&height.to_le_bytes());
    header.extend_from_slice(
        &u32::try_from(fingerprint.len())
            .context("lightmap cache fingerprint is too long")?
            .to_le_bytes(),
    );
    header.extend_from_slice(fingerprint);
    header.extend_from_slice(&payload_length.to_le_bytes());
    let mut checksum_input = header.clone();
    checksum_input.extend_from_slice(payload);
    let checksum = Sha256::digest(&checksum_input);
    header.extend_from_slice(&checksum);
    header.extend_from_slice(payload);
    Ok(header)
}

fn decode_tile(bytes: &[u8], fingerprint: &str, key: TileKey) -> Result<Option<TileRecord>> {
    let mut offset = 0;
    let magic = take(bytes, &mut offset, TILE_MAGIC.len())?;
    if magic != TILE_MAGIC {
        bail!("invalid lightmap tile header");
    }
    if read_u32(bytes, offset)? != FORMAT_VERSION {
        bail!("unsupported lightmap tile version");
    }
    offset += 4;
    let stored_primitive = read_u32(bytes, offset)?;
    offset += 4;
    let stored_tile_x = read_u32(bytes, offset)?;
    offset += 4;
    let stored_tile_y = read_u32(bytes, offset)?;
    offset += 4;
    let width = read_u32(bytes, offset)?;
    offset += 4;
    let height = read_u32(bytes, offset)?;
    offset += 4;
    let fingerprint_length = read_u32(bytes, offset)? as usize;
    offset += 4;
    let stored_fingerprint = take(bytes, &mut offset, fingerprint_length)?;
    let payload_length_offset = offset;
    let payload_length = read_u64(bytes, offset)? as usize;
    offset += 8;
    let stored_checksum = take(bytes, &mut offset, CHECKSUM_SIZE)?;
    let payload = take(bytes, &mut offset, payload_length)?;
    if offset != bytes.len() {
        bail!("lightmap tile has trailing bytes");
    }
    let expected_primitive =
        u32::try_from(key.primitive).context("lightmap primitive index overflowed")?;
    if stored_primitive != expected_primitive
        || stored_tile_x != key.tile_x
        || stored_tile_y != key.tile_y
    {
        bail!("lightmap tile identity does not match its path");
    }
    if stored_fingerprint != fingerprint.as_bytes() {
        return Ok(None);
    }
    let checksum_end = payload_length_offset + 8;
    let mut checksum_input = bytes[..checksum_end].to_vec();
    checksum_input.extend_from_slice(payload);
    let expected_checksum = Sha256::digest(&checksum_input);
    if stored_checksum != expected_checksum.as_slice() {
        bail!("lightmap tile checksum mismatch");
    }
    Ok(Some(TileRecord {
        width,
        height,
        payload: payload.to_vec(),
    }))
}

fn take<'a>(bytes: &'a [u8], offset: &mut usize, length: usize) -> Result<&'a [u8]> {
    let end = offset
        .checked_add(length)
        .context("lightmap cache record length overflowed")?;
    let value = bytes
        .get(*offset..end)
        .context("lightmap cache record is truncated")?;
    *offset = end;
    Ok(value)
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32> {
    let value = bytes
        .get(offset..offset + 4)
        .context("lightmap cache record is truncated")?;
    Ok(u32::from_le_bytes(
        value.try_into().expect("u32 slice length"),
    ))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64> {
    let value = bytes
        .get(offset..offset + 8)
        .context("lightmap cache record is truncated")?;
    Ok(u64::from_le_bytes(
        value.try_into().expect("u64 slice length"),
    ))
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let temporary = path.with_file_name(format!(
        ".{}.{}.tmp",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("lightmap-cache"),
        std::process::id()
    ));
    let mut file = File::create(&temporary)
        .with_context(|| format!("creating temporary cache file {}", temporary.display()))?;
    file.write_all(bytes)
        .with_context(|| format!("writing temporary cache file {}", temporary.display()))?;
    file.sync_all()
        .with_context(|| format!("flushing temporary cache file {}", temporary.display()))?;
    drop(file);
    if path.exists() {
        fs::remove_file(path)
            .with_context(|| format!("replacing cache file {}", path.display()))?;
    }
    fs::rename(&temporary, path)
        .with_context(|| format!("publishing cache file {}", path.display()))
}

fn is_owned_cache_name(name: &str) -> bool {
    name == "cache.meta"
        || (name.starts_with("page_")
            && name.ends_with(".bin")
            && name.len() == "page_0000_tile_0000_0000.bin".len()
            && name[5..9].chars().all(|c| c.is_ascii_digit())
            && name[15..19].chars().all(|c| c.is_ascii_digit())
            && name[20..24].chars().all(|c| c.is_ascii_digit()))
}

fn clear_cache_entries(root: &Path) -> Result<()> {
    for entry in fs::read_dir(root)
        .with_context(|| format!("listing lightmap accumulation cache {}", root.display()))?
    {
        let path = entry?.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let owned_temporary = name.starts_with('.')
            && name.ends_with(".tmp")
            && name[1..name.len() - 4]
                .rsplit_once('.')
                .is_some_and(|(owned_name, pid)| {
                    is_owned_cache_name(owned_name)
                        && !pid.is_empty()
                        && pid.chars().all(|c| c.is_ascii_digit())
                });
        if (is_owned_cache_name(name) || owned_temporary) && path.is_file() {
            fs::remove_file(&path)
                .with_context(|| format!("invalidating cache file {}", path.display()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests/cache.rs"]
mod tests;
