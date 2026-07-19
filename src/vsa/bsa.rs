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
mod tests {
    use super::*;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;
    use std::io::Write as _;

    const ARCHIVE_COMPRESSED: u32 = 0x0000_0004;

    /// A single file entry to embed in a hand-assembled BSA for tests.
    struct TestFile {
        name: &'static str,
        data: Vec<u8>,
        /// Sets the per-entry 0x4000_0000 bit on the stored size field.
        size_bit: bool,
    }

    impl TestFile {
        fn new(name: &'static str, data: Vec<u8>) -> Self {
            Self {
                name,
                data,
                size_bit: false,
            }
        }

        fn with_size_bit(name: &'static str, data: Vec<u8>) -> Self {
            Self {
                name,
                data,
                size_bit: true,
            }
        }
    }

    /// Hand-assembles a minimal BSA v104 archive (header, folder records,
    /// interleaved folder-name/file-record blocks, file names, then data) in
    /// the exact layout `BsaArchive::open` expects.
    fn build_bsa(flags: u32, folders: &[(&str, Vec<TestFile>)]) -> Vec<u8> {
        let folder_count = folders.len() as u32;
        let file_count: u32 = folders.iter().map(|(_, files)| files.len() as u32).sum();

        let header_len = 36usize;
        let folder_records_len = folder_count as usize * 16;
        let middle_len: usize = folders
            .iter()
            .map(|(name, files)| 1 + name.len() + 1 + files.len() * 16)
            .sum();
        let filenames_len: usize = folders
            .iter()
            .flat_map(|(_, files)| files.iter())
            .map(|file| file.name.len() + 1)
            .sum();
        let data_start = header_len + folder_records_len + middle_len + filenames_len;

        let mut out = Vec::new();
        out.extend_from_slice(b"BSA\0");
        out.extend_from_slice(&104u32.to_le_bytes());
        out.extend_from_slice(&flags.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes()); // unused [12..16]
        out.extend_from_slice(&folder_count.to_le_bytes());
        out.extend_from_slice(&file_count.to_le_bytes());
        out.extend_from_slice(&[0u8; 12]); // pad header to 36 bytes

        for (_, files) in folders {
            out.extend_from_slice(&0u64.to_le_bytes()); // hash, unused by the reader
            out.extend_from_slice(&(files.len() as u32).to_le_bytes());
            out.extend_from_slice(&0u32.to_le_bytes()); // folder data offset, unused by the reader
        }

        let mut cursor = data_start as u64;
        let mut data_blocks: Vec<&[u8]> = Vec::new();
        for (name, files) in folders {
            let mut name_bytes = name.as_bytes().to_vec();
            name_bytes.push(0);
            out.push(name_bytes.len() as u8);
            out.extend_from_slice(&name_bytes);
            for file in files {
                let mut size = file.data.len() as u32;
                if file.size_bit {
                    size |= 0x4000_0000;
                }
                out.extend_from_slice(&0u64.to_le_bytes()); // hash, unused by the reader
                out.extend_from_slice(&size.to_le_bytes());
                out.extend_from_slice(&(cursor as u32).to_le_bytes());
                data_blocks.push(&file.data);
                cursor += file.data.len() as u64;
            }
        }

        for (_, files) in folders {
            for file in files {
                out.extend_from_slice(file.name.as_bytes());
                out.push(0);
            }
        }

        for block in data_blocks {
            out.extend_from_slice(block);
        }

        out
    }

    fn zlib_compress(data: &[u8]) -> Vec<u8> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    }

    /// A hermetic per-test path in the OS temp dir; combines the process id
    /// with a caller-supplied tag so parallel tests never collide.
    fn temp_path(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!("bevyout-bsa-{tag}-{}.bsa", std::process::id()))
    }

    /// Owns the temp file backing a parsed `BsaArchive` for the test's
    /// lifetime; `BsaArchive::read` reopens the file by path on every call,
    /// so it must still exist. Removes the file on drop.
    #[derive(Debug)]
    struct TempArchive {
        archive: BsaArchive,
        path: PathBuf,
    }

    impl std::ops::Deref for TempArchive {
        type Target = BsaArchive;

        fn deref(&self) -> &BsaArchive {
            &self.archive
        }
    }

    impl Drop for TempArchive {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    /// Writes `bytes` to a fresh temp file and opens it as a `BsaArchive`,
    /// keeping the file alive (and cleaning it up on drop) for as long as the
    /// returned value is in scope.
    fn open_bsa(tag: &str, bytes: &[u8]) -> Result<TempArchive> {
        let path = temp_path(tag);
        std::fs::write(&path, bytes)?;
        match BsaArchive::open(&path) {
            Ok(archive) => Ok(TempArchive { archive, path }),
            Err(error) => {
                let _ = std::fs::remove_file(&path);
                Err(error)
            }
        }
    }

    #[test]
    fn rejects_wrong_magic() {
        let mut bytes = vec![0u8; 36];
        bytes[0..4].copy_from_slice(b"XXX\0");
        bytes[4..8].copy_from_slice(&104u32.to_le_bytes());
        let error = open_bsa("wrong-magic", &bytes).unwrap_err();
        assert!(error.to_string().contains("invalid BSA magic"));
    }

    #[test]
    fn rejects_unsupported_version() {
        let mut bytes = vec![0u8; 36];
        bytes[0..4].copy_from_slice(b"BSA\0");
        bytes[4..8].copy_from_slice(&99u32.to_le_bytes());
        let error = open_bsa("wrong-version", &bytes).unwrap_err();
        assert!(error.to_string().contains("unsupported BSA version"));
    }

    #[test]
    fn uncompressed_entry_round_trips_and_normalizes_key() {
        let data = b"hello world exact bytes".to_vec();
        let bytes = build_bsa(
            0,
            &[("MESHES\\Foo", vec![TestFile::new("BAR.NIF", data.clone())])],
        );
        let archive = open_bsa("uncompressed", &bytes).unwrap();
        assert_eq!(archive.read("meshes/foo/bar.nif").unwrap().unwrap(), data);
    }

    #[test]
    fn archive_wide_compression_flag_decodes_zlib_payload() {
        let original = b"this is the original uncompressed payload".to_vec();
        let compressed = zlib_compress(&original);
        let bytes = build_bsa(
            ARCHIVE_COMPRESSED,
            &[("sounds", vec![TestFile::new("clip.wav", compressed)])],
        );
        let archive = open_bsa("archive-flag", &bytes).unwrap();
        assert_eq!(archive.read("sounds/clip.wav").unwrap().unwrap(), original);
    }

    #[test]
    fn per_entry_size_bit_inverts_archive_wide_compression() {
        // Archive-wide compression is on, but this entry's size bit marks it
        // as an exception: it must be returned verbatim, not zlib-decoded.
        let raw = b"stored raw despite archive-wide compression flag".to_vec();
        let bytes = build_bsa(
            ARCHIVE_COMPRESSED,
            &[(
                "meshes",
                vec![TestFile::with_size_bit("exception.nif", raw.clone())],
            )],
        );
        let archive = open_bsa("size-bit-invert", &bytes).unwrap();
        assert_eq!(archive.read("meshes/exception.nif").unwrap().unwrap(), raw);
    }

    #[test]
    fn embedded_name_prefix_before_zlib_stream_still_decodes() {
        let original = b"fo3 texture archive payload behind a junk prefix".to_vec();
        let mut payload = b"FO3TEXPATH\0".to_vec();
        payload.extend_from_slice(&zlib_compress(&original));
        let bytes = build_bsa(
            ARCHIVE_COMPRESSED,
            &[("textures", vec![TestFile::new("prefixed.dds", payload)])],
        );
        let archive = open_bsa("embedded-name", &bytes).unwrap();
        assert_eq!(
            archive.read("textures/prefixed.dds").unwrap().unwrap(),
            original
        );
    }

    #[test]
    fn compressed_flagged_media_streams_pass_through_verbatim() {
        let riff = {
            let mut bytes = b"RIFF".to_vec();
            bytes.extend_from_slice(b"not really compressed wav bytes");
            bytes
        };
        let ogg = {
            let mut bytes = b"OggS".to_vec();
            bytes.extend_from_slice(b"not really compressed ogg bytes");
            bytes
        };
        let mut mp3 = vec![0xff, 0xfb, 0x90, 0x64];
        mp3.resize(20, 0);
        let bytes = build_bsa(
            ARCHIVE_COMPRESSED,
            &[(
                "sound",
                vec![
                    TestFile::new("clip.wav", riff.clone()),
                    TestFile::new("clip.ogg", ogg.clone()),
                    TestFile::new("clip.mp3", mp3.clone()),
                ],
            )],
        );
        let archive = open_bsa("media-passthrough", &bytes).unwrap();
        assert_eq!(archive.read("sound/clip.wav").unwrap().unwrap(), riff);
        assert_eq!(archive.read("sound/clip.ogg").unwrap().unwrap(), ogg);
        assert_eq!(archive.read("sound/clip.mp3").unwrap().unwrap(), mp3);
    }

    #[test]
    fn compressed_nif_prefix_is_decompressed_not_treated_as_mp3() {
        let original = b"Gamebryo File Format, Version 20.2.0.7".to_vec();
        let mut payload = vec![0xff, 0xf3, 0x00, 0x00];
        payload.extend_from_slice(&zlib_compress(&original));
        let bytes = build_bsa(
            ARCHIVE_COMPRESSED,
            &[("meshes", vec![TestFile::new("rcclothwall01.nif", payload)])],
        );
        let archive = open_bsa("nif-mp3-prefix", &bytes).unwrap();
        assert_eq!(
            archive.read("meshes/rcclothwall01.nif").unwrap().unwrap(),
            original
        );
    }

    #[test]
    fn sound_path_falls_back_to_entry_without_sound_prefix() {
        let data = b"footstep clip bytes".to_vec();
        let bytes = build_bsa(0, &[("fx", vec![TestFile::new("foo.wav", data.clone())])]);
        let archive = open_bsa("sound-fallback", &bytes).unwrap();
        assert_eq!(archive.read("sound/fx/foo.wav").unwrap().unwrap(), data);
    }

    #[test]
    fn missing_path_returns_none() {
        let bytes = build_bsa(
            0,
            &[("meshes", vec![TestFile::new("present.nif", b"x".to_vec())])],
        );
        let archive = open_bsa("missing-path", &bytes).unwrap();
        assert!(archive.read("meshes/absent.nif").unwrap().is_none());
    }

    #[test]
    fn extension_listing_is_case_insensitive_sorted_and_complete() {
        let bytes = build_bsa(
            0,
            &[
                (
                    "Meshes\\Characters\\_Male",
                    vec![
                        TestFile::new("Walk.KF", b"walk".to_vec()),
                        TestFile::new("skeleton.NIF", b"skeleton".to_vec()),
                    ],
                ),
                (
                    "meshes\\creatures\\molerat",
                    vec![TestFile::new("Attack.kf", b"attack".to_vec())],
                ),
            ],
        );
        let archive = open_bsa("extension-list", &bytes).unwrap();
        assert_eq!(
            archive.paths_with_extension(".KF"),
            [
                "meshes/characters/_male/walk.kf",
                "meshes/creatures/molerat/attack.kf"
            ]
        );
    }

    #[test]
    fn compressed_garbage_without_zlib_header_errors() {
        let garbage =
            b"not a legitimate zlib stream at all, just filler bytes 1234567890!!!".to_vec();
        let bytes = build_bsa(
            ARCHIVE_COMPRESSED,
            &[("meshes", vec![TestFile::new("broken.nif", garbage)])],
        );
        let archive = open_bsa("garbage-compressed", &bytes).unwrap();
        let error = archive.read("meshes/broken.nif").unwrap_err();
        assert!(
            error
                .to_string()
                .contains("compressed BSA entry has no valid zlib stream")
        );
    }
}
