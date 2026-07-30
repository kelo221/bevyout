use anyhow::{Result, bail};
use flate2::read::ZlibDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use super::paths::normalize_asset_path;

#[derive(Debug, Clone)]
struct ArchiveEntry {
    offset: u64,
    size: u32,
    compressed: bool,
}

#[derive(Debug, Default)]
pub(crate) struct BsaArchive {
    file: PathBuf,
    entries: HashMap<String, ArchiveEntry>,
}

impl BsaArchive {
    pub(crate) fn open(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut header = [0u8; 36];
        file.read_exact(&mut header)?;
        if &header[0..4] != b"BSA\0" {
            bail!("invalid BSA magic")
        }
        let version = u32::from_le_bytes(header[4..8].try_into().unwrap());
        if version != 104 {
            bail!("unsupported BSA version {version}")
        }
        let folder_count = u32::from_le_bytes(header[16..20].try_into().unwrap()) as usize;
        let file_count = u32::from_le_bytes(header[20..24].try_into().unwrap()) as usize;
        let flags = u32::from_le_bytes(header[8..12].try_into().unwrap());
        let mut folders = Vec::with_capacity(folder_count);
        for _ in 0..folder_count {
            let mut row = [0u8; 16];
            file.read_exact(&mut row)?;
            folders.push((
                u64::from_le_bytes(row[0..8].try_into().unwrap()),
                u32::from_le_bytes(row[8..12].try_into().unwrap()) as usize,
                u32::from_le_bytes(row[12..16].try_into().unwrap()) as u64,
            ));
        }
        let mut files = Vec::with_capacity(file_count);
        let mut folder_names = Vec::with_capacity(folder_count);
        for (_, count, _) in &folders {
            folder_names.push(read_cstring(&mut file)?);
            for _ in 0..*count {
                let mut row = [0u8; 16];
                file.read_exact(&mut row)?;
                files.push((
                    u32::from_le_bytes(row[8..12].try_into().unwrap()),
                    u32::from_le_bytes(row[12..16].try_into().unwrap()) as u64,
                ));
            }
        }
        let mut file_names = Vec::with_capacity(file_count);
        for _ in 0..file_count {
            file_names.push(read_nul_string(&mut file)?);
        }
        let mut entries = HashMap::new();
        let mut file_index = 0;
        for (folder_index, (_, count, _)) in folders.iter().enumerate() {
            for _ in 0..*count {
                let (size, offset) = files[file_index];
                let compressed = (size & 0x4000_0000 == 0) && (flags & 0x0000_0004 != 0);
                entries.insert(
                    format!(
                        "{}/{}",
                        normalize_asset_path(&folder_names[folder_index]),
                        normalize_asset_path(&file_names[file_index]),
                    ),
                    ArchiveEntry {
                        offset,
                        size: size & 0x3fff_ffff,
                        compressed,
                    },
                );
                file_index += 1;
            }
        }
        Ok(Self {
            file: path.to_path_buf(),
            entries,
        })
    }

    pub(crate) fn read(&self, path: &str) -> Result<Option<Vec<u8>>> {
        let normalized = normalize_asset_path(path);
        let entry = self.entries.get(&normalized).or_else(|| {
            normalized
                .strip_prefix("sound/")
                .and_then(|path| self.entries.get(path))
        });
        let Some(entry) = entry else {
            return Ok(None);
        };
        Ok(Some(self.read_entry(entry)?))
    }

    /// Read the lexicographically first direct file below a virtual directory.
    /// Bethesda sound records sometimes store a directory in `FNAM` and leave
    /// variant selection to the game. Preparation has one clip slot per sound
    /// FormID, so choose one stable direct child rather than depending on
    /// `HashMap` iteration order.
    pub(crate) fn read_first_with_prefix(
        &self,
        directory: &str,
    ) -> Result<Option<(String, Vec<u8>)>> {
        let prefix = format!("{}/", normalize_asset_path(directory).trim_end_matches('/'));
        let mut matches = self
            .entries
            .keys()
            .filter(|path| {
                path.strip_prefix(&prefix)
                    .is_some_and(|suffix| !suffix.is_empty() && !suffix.contains('/'))
            })
            .collect::<Vec<_>>();
        matches.sort_unstable();
        let Some(path) = matches.first() else {
            return Ok(None);
        };
        let entry = self
            .entries
            .get(*path)
            .expect("prefix match must remain in the archive index");
        Ok(Some((path.to_string(), self.read_entry(entry)?)))
    }

    /// Enumerates normalized archive paths with one extension in stable order.
    /// The BSA index is case-insensitive because keys are normalized at load.
    pub(crate) fn paths_with_extension(&self, extension: &str) -> Vec<String> {
        let extension = extension.trim_start_matches('.').to_ascii_lowercase();
        let suffix = format!(".{extension}");
        let mut paths = self
            .entries
            .keys()
            .filter(|path| path.ends_with(&suffix))
            .cloned()
            .collect::<Vec<_>>();
        paths.sort();
        paths
    }

    fn read_entry(&self, entry: &ArchiveEntry) -> Result<Vec<u8>> {
        let mut file = File::open(&self.file)?;
        file.seek(SeekFrom::Start(entry.offset))?;
        let mut bytes = vec![0; entry.size as usize];
        file.read_exact(&mut bytes)?;
        if entry.compressed {
            // Fallout's sound archive marks several already-compressed media
            // payloads with the archive-wide compression flag even though the
            // bytes are complete RIFF/Ogg/MP3 streams. Preserve those streams
            // instead of treating them as zlib records.
            if bytes.starts_with(b"RIFF")
                || bytes.starts_with(b"OggS")
                || bytes.starts_with(b"ID3")
                || looks_like_mp3_frame(&bytes)
            {
                return Ok(bytes);
            }
            // FO3's texture archive embeds the original path before the
            // four-byte unpacked-size field, while the mesh archive usually
            // starts with that field. Locate the zlib header instead of
            // assuming a fixed prefix.
            let starts = bytes.windows(2).enumerate().filter_map(|(index, window)| {
                (window[0] == 0x78 && matches!(window[1], 0x01 | 0x5e | 0x9c | 0xda | 0x20 | 0x7e))
                    .then_some(index)
            });
            for start in starts {
                let mut decoder = ZlibDecoder::new(Cursor::new(&bytes[start..]));
                let mut decoded = Vec::new();
                if decoder.read_to_end(&mut decoded).is_ok() {
                    return Ok(decoded);
                }
            }
            bail!("compressed BSA entry has no valid zlib stream")
        } else {
            Ok(bytes)
        }
    }
}

/// Returns whether the bytes begin with a structurally valid MPEG audio frame
/// header. A sync word alone is not enough: compressed NIF entries can begin
/// with `FF F3 00 00`, which otherwise looks like an MP3 to a loose detector.
fn looks_like_mp3_frame(bytes: &[u8]) -> bool {
    if bytes.len() < 4 || bytes[0] != 0xff || bytes[1] & 0xe0 != 0xe0 {
        return false;
    }
    let version = (bytes[1] >> 3) & 0b11;
    let layer = (bytes[1] >> 1) & 0b11;
    let bitrate_index = bytes[2] >> 4;
    let sample_rate_index = (bytes[2] >> 2) & 0b11;
    version != 0b01
        && layer != 0
        && bitrate_index != 0
        && bitrate_index != 0b1111
        && sample_rate_index != 0b11
}

fn read_cstring(file: &mut File) -> Result<String> {
    let mut length = [0u8; 1];
    file.read_exact(&mut length)?;
    let mut bytes = vec![0; length[0] as usize];
    file.read_exact(&mut bytes)?;
    Ok(String::from_utf8_lossy(&bytes)
        .trim_end_matches('\0')
        .to_string())
}

fn read_nul_string(file: &mut File) -> Result<String> {
    let mut bytes = Vec::new();
    let mut byte = [0u8; 1];
    loop {
        file.read_exact(&mut byte)?;
        if byte[0] == 0 {
            break;
        }
        bytes.push(byte[0]);
    }
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

#[cfg(test)]
#[path = "tests/bsa.rs"]
mod tests;
