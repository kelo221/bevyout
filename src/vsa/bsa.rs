use anyhow::{Context, Result, bail};
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
        let Some(entry) = self.entries.get(&normalize_asset_path(path)) else {
            return Ok(None);
        };
        let mut file = File::open(&self.file)?;
        file.seek(SeekFrom::Start(entry.offset))?;
        let mut bytes = vec![0; entry.size as usize];
        file.read_exact(&mut bytes)?;
        if entry.compressed {
            // FO3's texture archive embeds the original path before the
            // four-byte unpacked-size field, while the mesh archive usually
            // starts with that field. Locate the zlib header instead of
            // assuming a fixed prefix.
            let start = bytes
                .windows(2)
                .position(|window| {
                    window[0] == 0x78
                        && matches!(window[1], 0x01 | 0x5e | 0x9c | 0xda | 0x20 | 0x7e)
                })
                .context("compressed BSA entry has no zlib header")?;
            let mut decoder = ZlibDecoder::new(Cursor::new(&bytes[start..]));
            let mut decoded = Vec::new();
            decoder.read_to_end(&mut decoded)?;
            Ok(Some(decoded))
        } else {
            Ok(Some(bytes))
        }
    }
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
