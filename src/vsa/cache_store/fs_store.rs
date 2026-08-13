use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use super::model::{
    CandidateObject, CandidatePayload, PREPARED_RECIPE_RECORD_REVISION, PreparedObjectKind,
    PreparedObjectRef, PreparedObjectStore, PreparedRecipeInputs, PreparedRecipeRecord,
    Verification,
};
use super::policy::is_canonical_sha256;

static TEMPORARY_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub(crate) struct FsPreparedObjectStore {
    root: PathBuf,
}

impl FsPreparedObjectStore {
    pub(crate) fn open(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        for directory in ["objects", "recipes", "staging", "quarantine/corrupt"] {
            fs::create_dir_all(root.join(directory))
                .with_context(|| format!("creating object-store directory {directory}"))?;
        }
        Ok(Self { root })
    }

    pub(crate) fn object_path(&self, object: &PreparedObjectRef) -> PathBuf {
        sharded_path(
            &self.root.join("objects"),
            object.kind,
            &object.sha256,
            &object.extension,
        )
    }

    pub(crate) fn object_asset_path(&self, object: &PreparedObjectRef) -> String {
        format!(
            "objects/{}/{}/{}/{}.{}",
            object.kind.tag(),
            &object.sha256[0..2],
            &object.sha256[2..4],
            object.sha256,
            object.extension
        )
    }

    fn recipe_path(&self, kind: PreparedObjectKind, recipe_id: &str) -> PathBuf {
        sharded_path(&self.root.join("recipes"), kind, recipe_id, "ron")
    }

    fn temporary_path(&self, recipe_id: &str, extension: &str) -> Result<PathBuf> {
        let directory = self
            .root
            .join("staging")
            .join(std::process::id().to_string())
            .join(recipe_id);
        fs::create_dir_all(&directory).with_context(|| {
            format!("creating object staging directory {}", directory.display())
        })?;
        let sequence = TEMPORARY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        Ok(directory.join(format!(".candidate-{sequence}.{extension}.tmp")))
    }

    fn quarantine_path(&self, object: &PreparedObjectRef) -> Result<PathBuf> {
        let directory = self.root.join("quarantine/corrupt").join(object.kind.tag());
        fs::create_dir_all(&directory)?;
        let sequence = TEMPORARY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        Ok(directory.join(format!(
            "{}.{}.{sequence}.corrupt",
            object.sha256, object.extension
        )))
    }

    #[cfg(test)]
    pub(crate) fn object_count(&self) -> Result<usize> {
        count_files(&self.root.join("objects"))
    }

    #[cfg(test)]
    pub(crate) fn temporary_file_count(&self) -> Result<usize> {
        count_files(&self.root.join("staging"))
    }

    #[cfg(test)]
    pub(crate) fn quarantined_file_count(&self) -> Result<usize> {
        count_files(&self.root.join("quarantine"))
    }

    fn ensure_object(&self, temporary: &Path, object: &PreparedObjectRef) -> Result<()> {
        let destination = self.object_path(object);
        let parent = destination.parent().context("object path has no parent")?;
        fs::create_dir_all(parent)
            .with_context(|| format!("creating object shard {}", parent.display()))?;

        if destination.is_file() {
            let verification = self.verify(object)?;
            if verification.valid {
                fs::remove_file(temporary).ok();
                return Ok(());
            }
            let quarantine = self.quarantine_path(object)?;
            match fs::rename(&destination, &quarantine) {
                Ok(()) => {}
                Err(error) if !destination.exists() => {
                    let _ = error;
                }
                Err(error) => {
                    return Err(error).with_context(|| {
                        format!("quarantining corrupt object {}", destination.display())
                    });
                }
            }
        }

        match fs::rename(temporary, &destination) {
            Ok(()) => Ok(()),
            Err(error) if destination.is_file() => {
                let verification = self.verify(object)?;
                if verification.valid {
                    fs::remove_file(temporary).ok();
                    Ok(())
                } else {
                    Err(error).with_context(|| {
                        format!(
                            "concurrent publication left corrupt object {}",
                            destination.display()
                        )
                    })
                }
            }
            Err(error) => {
                Err(error).with_context(|| format!("publishing object {}", destination.display()))
            }
        }
    }

    fn publish_recipe(&self, recipe_id: &str, record: &PreparedRecipeRecord) -> Result<()> {
        let path = self.recipe_path(record.recipe.kind, recipe_id);
        let parent = path.parent().context("recipe path has no parent")?;
        fs::create_dir_all(parent)
            .with_context(|| format!("creating recipe shard {}", parent.display()))?;
        if recipe_record_matches(&path, record)? {
            return Ok(());
        }

        let serialized = ron::ser::to_string_pretty(record, ron::ser::PrettyConfig::default())?;
        let temporary = parent.join(format!(
            ".{recipe_id}.{}.tmp",
            TEMPORARY_SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        write_synced(&temporary, serialized.as_bytes())?;

        if path.exists() && !recipe_record_matches(&path, record)? {
            let quarantine = self.root.join("quarantine/corrupt/recipe");
            fs::create_dir_all(&quarantine)?;
            let quarantined = quarantine.join(format!(
                "{recipe_id}.{}.ron.corrupt",
                TEMPORARY_SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::rename(&path, quarantined) {
                Ok(()) => {}
                Err(error) if !path.exists() => {
                    let _ = error;
                }
                Err(error) => {
                    fs::remove_file(&temporary).ok();
                    return Err(error).context("quarantining corrupt recipe record");
                }
            }
        }

        match fs::rename(&temporary, &path) {
            Ok(()) => Ok(()),
            Err(_error) if recipe_record_matches(&path, record)? => {
                fs::remove_file(&temporary).ok();
                Ok(())
            }
            Err(error) => {
                fs::remove_file(&temporary).ok();
                Err(error).with_context(|| format!("publishing recipe {}", path.display()))
            }
        }
    }
}

impl PreparedObjectStore for FsPreparedObjectStore {
    fn resolve_recipe(&self, recipe_id: &str) -> Result<Option<PreparedObjectRef>> {
        if !is_canonical_sha256(recipe_id) {
            bail!("recipe id is not lowercase SHA-256")
        }
        for kind in all_object_kinds() {
            let path = self.recipe_path(kind, recipe_id);
            if !path.is_file() {
                continue;
            }
            let text = match fs::read_to_string(&path) {
                Ok(text) => text,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => {
                    return Err(error)
                        .with_context(|| format!("reading recipe record {}", path.display()));
                }
            };
            let Ok(record) = ron::from_str::<PreparedRecipeRecord>(&text) else {
                continue;
            };
            if record.revision != PREPARED_RECIPE_RECORD_REVISION {
                continue;
            }
            if record.recipe.try_id()? != recipe_id {
                bail!("recipe record identity does not match its path")
            }
            if record.recipe.kind != kind || record.output.kind != kind {
                bail!("recipe record kind does not match its shard")
            }
            return Ok(Some(record.output));
        }
        Ok(None)
    }

    fn publish(
        &self,
        recipe: &PreparedRecipeInputs,
        candidate: CandidateObject,
    ) -> Result<PreparedObjectRef> {
        if candidate.kind != recipe.kind {
            bail!("candidate kind does not match recipe kind")
        }
        let recipe_id = recipe.try_id()?;
        let extension = normalize_extension(&candidate.extension)?;
        let temporary = self.temporary_path(&recipe_id, &extension)?;
        let publication = (|| -> Result<PreparedObjectRef> {
            let (sha256, byte_len) = write_and_hash_candidate(&candidate.payload, &temporary)?;
            validate_candidate(candidate.kind, &extension, &temporary)?;
            let object = PreparedObjectRef {
                kind: candidate.kind,
                sha256,
                byte_len,
                extension,
            };
            self.ensure_object(&temporary, &object)?;
            self.publish_recipe(
                &recipe_id,
                &PreparedRecipeRecord {
                    revision: PREPARED_RECIPE_RECORD_REVISION.into(),
                    recipe: recipe.clone(),
                    output: object.clone(),
                },
            )?;
            Ok(object)
        })();
        if publication.is_err() {
            fs::remove_file(&temporary).ok();
        }
        publication
    }

    fn open(&self, object: &PreparedObjectRef) -> Result<File> {
        validate_object_ref(object)?;
        let path = self.object_path(object);
        File::open(&path).with_context(|| format!("opening prepared object {}", path.display()))
    }

    fn verify(&self, object: &PreparedObjectRef) -> Result<Verification> {
        validate_object_ref(object)?;
        let mut input = BufReader::new(self.open(object)?);
        let mut hasher = Sha256::new();
        let mut byte_len = 0u64;
        let mut buffer = [0u8; 64 * 1024];
        loop {
            let read = input.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
            byte_len = byte_len.saturating_add(read as u64);
        }
        let actual_sha256 = format!("{:x}", hasher.finalize());
        Ok(Verification {
            valid: byte_len == object.byte_len && actual_sha256 == object.sha256,
            actual_sha256,
            actual_byte_len: byte_len,
        })
    }
}

fn sharded_path(root: &Path, kind: PreparedObjectKind, sha256: &str, extension: &str) -> PathBuf {
    root.join(kind.tag())
        .join(&sha256[0..2])
        .join(&sha256[2..4])
        .join(format!("{sha256}.{extension}"))
}

fn validate_object_ref(object: &PreparedObjectRef) -> Result<()> {
    if !is_canonical_sha256(&object.sha256) {
        bail!("object hash is not lowercase SHA-256")
    }
    if normalize_extension(&object.extension)? != object.extension {
        bail!("object extension is not canonical")
    }
    Ok(())
}

fn normalize_extension(extension: &str) -> Result<String> {
    let extension = extension.trim_start_matches('.').to_ascii_lowercase();
    if extension.is_empty()
        || extension.starts_with('.')
        || extension.ends_with('.')
        || !extension
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'.')
    {
        bail!("object extension is invalid")
    }
    Ok(extension)
}

fn write_and_hash_candidate(
    candidate: &CandidatePayload,
    destination: &Path,
) -> Result<(String, u64)> {
    let output = File::create(destination)
        .with_context(|| format!("creating temporary object {}", destination.display()))?;
    let mut output = BufWriter::new(output);
    let mut hasher = Sha256::new();
    let mut byte_len = 0u64;
    match candidate {
        CandidatePayload::Path(source) => {
            let input = File::open(source)
                .with_context(|| format!("opening candidate object {}", source.display()))?;
            let mut input = BufReader::new(input);
            let mut buffer = [0u8; 64 * 1024];
            loop {
                let read = input.read(&mut buffer)?;
                if read == 0 {
                    break;
                }
                output.write_all(&buffer[..read])?;
                hasher.update(&buffer[..read]);
                byte_len = byte_len.saturating_add(read as u64);
            }
        }
        CandidatePayload::Bytes(bytes) => {
            output.write_all(bytes)?;
            hasher.update(bytes);
            byte_len = bytes.len() as u64;
        }
    }
    if byte_len == 0 {
        bail!("candidate object is empty")
    }
    output.flush()?;
    output.get_ref().sync_all()?;
    Ok((format!("{:x}", hasher.finalize()), byte_len))
}

fn write_synced(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

fn validate_candidate(kind: PreparedObjectKind, extension: &str, path: &Path) -> Result<()> {
    let bytes = fs::read(path)?;
    match kind {
        PreparedObjectKind::Glb => validate_glb(&bytes),
        PreparedObjectKind::Texture | PreparedObjectKind::Shadow => {
            validate_texture(extension, &bytes)
        }
        PreparedObjectKind::Physics
        | PreparedObjectKind::Catalog
        | PreparedObjectKind::Navigation => {
            ron::de::from_bytes::<ron::Value>(&bytes).context("prepared RON object is invalid")?;
            Ok(())
        }
        PreparedObjectKind::Audio => validate_audio(extension, &bytes),
        PreparedObjectKind::Bake | PreparedObjectKind::Other => Ok(()),
    }
}

fn validate_glb(bytes: &[u8]) -> Result<()> {
    if bytes.len() < 20 || &bytes[..4] != b"glTF" {
        bail!("prepared GLB has an invalid header")
    }
    if u32::from_le_bytes(bytes[4..8].try_into().unwrap()) != 2 {
        bail!("prepared GLB is not version 2")
    }
    if u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize != bytes.len() {
        bail!("prepared GLB length does not match its header")
    }
    if &bytes[16..20] != b"JSON" {
        bail!("prepared GLB does not begin with a JSON chunk")
    }
    let json_len = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    let json_end = 20usize
        .checked_add(json_len)
        .context("GLB JSON length overflow")?;
    let json = bytes
        .get(20..json_end)
        .context("GLB JSON chunk is truncated")?;
    serde_json::from_slice::<serde_json::Value>(json).context("GLB JSON chunk is invalid")?;
    Ok(())
}

fn validate_texture(extension: &str, bytes: &[u8]) -> Result<()> {
    let valid = match extension {
        "ktx2" => bytes.len() >= 80 && bytes.starts_with(b"\xABKTX 20\xBB\r\n\x1A\n"),
        "png" => bytes.starts_with(b"\x89PNG\r\n\x1A\n"),
        "dds" => bytes.starts_with(b"DDS "),
        _ => false,
    };
    if !valid {
        bail!("prepared texture payload does not match extension {extension}")
    }
    Ok(())
}

fn validate_audio(extension: &str, bytes: &[u8]) -> Result<()> {
    let valid = match extension {
        "wav" => bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WAVE",
        "ogg" => bytes.starts_with(b"OggS"),
        "xwm" => bytes.starts_with(b"RIFF"),
        "mp3" => bytes.starts_with(b"ID3") || bytes.starts_with(&[0xff, 0xfb]),
        _ => false,
    };
    if !valid {
        bail!("prepared audio payload does not match extension {extension}")
    }
    Ok(())
}

fn recipe_record_matches(path: &Path, expected: &PreparedRecipeRecord) -> Result<bool> {
    if !path.is_file() {
        return Ok(false);
    }
    let Ok(text) = fs::read_to_string(path) else {
        return Ok(false);
    };
    let Ok(actual) = ron::from_str::<PreparedRecipeRecord>(&text) else {
        return Ok(false);
    };
    Ok(actual == *expected)
}

#[cfg(test)]
fn count_files(root: &Path) -> Result<usize> {
    if !root.exists() {
        return Ok(0);
    }
    let mut count = 0usize;
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                pending.push(entry.path());
            } else if file_type.is_file() {
                count += 1;
            }
        }
    }
    Ok(count)
}

#[allow(dead_code)]
fn all_object_kinds() -> [PreparedObjectKind; 9] {
    [
        PreparedObjectKind::Glb,
        PreparedObjectKind::Texture,
        PreparedObjectKind::Physics,
        PreparedObjectKind::Audio,
        PreparedObjectKind::Shadow,
        PreparedObjectKind::Bake,
        PreparedObjectKind::Catalog,
        PreparedObjectKind::Navigation,
        PreparedObjectKind::Other,
    ]
}
