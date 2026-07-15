use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::bsa::BsaArchive;
use super::paths::{fingerprint, normalize_asset_path};

/// An indexed Fallout sound archive. Entries earlier in a slice have higher
/// precedence than entries later in it.
#[derive(Debug)]
pub(crate) struct AudioArchive {
    pub(crate) path: PathBuf,
    archive: BsaArchive,
}

/// Archive discovery never fails solely because an optional archive is absent
/// or malformed. Callers can surface these strings through their own
/// diagnostic type.
#[derive(Debug, Default)]
pub(crate) struct AudioArchiveLoad {
    pub(crate) archives: Vec<AudioArchive>,
    pub(crate) diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AudioAssetOrigin {
    Loose(PathBuf),
    Archive(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedAudioAsset {
    /// Normalized virtual path used for the successful lookup.
    pub(crate) source_path: String,
    pub(crate) origin: AudioAssetOrigin,
    pub(crate) bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StagedAudioAsset {
    pub(crate) content_hash: String,
    pub(crate) path: PathBuf,
}

/// Index sound archives associated with `plugin_names`. The plugin list must
/// be ordered from highest to lowest precedence (selected plugin first, then
/// its masters). That order is preserved for asset lookup.
///
/// Both Bethesda's base-game singular `Fallout - Sound.bsa`, the DLC plural
/// `* - Sounds.bsa` convention, and ordinary plugin-stem BSAs are considered.
pub(crate) fn load_audio_archives(data_root: &Path, plugin_names: &[String]) -> AudioArchiveLoad {
    let mut load = AudioArchiveLoad::default();
    let mut seen = HashSet::new();

    for plugin_name in plugin_names {
        let candidates = audio_archive_candidate_names(plugin_name);
        let mut found_for_plugin = false;
        let mut tried = Vec::new();

        for name in candidates {
            if !seen.insert(name.to_ascii_lowercase()) {
                continue;
            }
            tried.push(name.clone());
            let path = data_root.join(&name);
            if !path.is_file() {
                continue;
            }
            found_for_plugin = true;
            match BsaArchive::open(&path) {
                Ok(archive) => load.archives.push(AudioArchive { path, archive }),
                Err(error) => load.diagnostics.push(format!(
                    "could not index audio archive {}: {error}",
                    path.display()
                )),
            }
        }

        if !found_for_plugin && !tried.is_empty() {
            load.diagnostics.push(format!(
                "no audio archive found for {plugin_name}; tried {}",
                tried.join(", ")
            ));
        }
    }

    load
}

/// Resolve an OpenMW-style sound path. Paths are normalized beneath `sound/`.
/// The recorded extension is attempted first, followed by an `.mp3` variant.
/// For each virtual-path candidate, a loose Data file takes precedence over
/// all archives, and archives are searched in caller-provided precedence.
pub(crate) fn resolve_audio_asset(
    data_root: &Path,
    archives: &[AudioArchive],
    recorded_path: &str,
) -> Result<Option<ResolvedAudioAsset>> {
    let directory_form = normalize_asset_path(recorded_path).ends_with('/');
    for (index, candidate) in sound_path_candidates(recorded_path).into_iter().enumerate() {
        let loose = data_root.join(candidate.replace('/', std::path::MAIN_SEPARATOR_STR));
        if loose.is_file() {
            let bytes = fs::read(&loose)
                .with_context(|| format!("reading loose audio asset {}", loose.display()))?;
            return Ok(Some(ResolvedAudioAsset {
                source_path: candidate,
                origin: AudioAssetOrigin::Loose(loose),
                bytes,
            }));
        }

        if directory_form
            && index == 0
            && let Some(asset) = resolve_loose_directory(&loose, &candidate)?
        {
            return Ok(Some(asset));
        }

        for archive in archives {
            if let Some(bytes) = archive.archive.read(&candidate).with_context(|| {
                format!(
                    "reading audio asset {candidate} from {}",
                    archive.path.display()
                )
            })? {
                return Ok(Some(ResolvedAudioAsset {
                    source_path: candidate,
                    origin: AudioAssetOrigin::Archive(archive.path.clone()),
                    bytes,
                }));
            }
            if directory_form
                && index == 0
                && let Some((source_path, bytes)) = archive
                    .archive
                    .read_first_with_prefix(&candidate)
                    .with_context(|| {
                        format!(
                            "reading audio directory {candidate} from {}",
                            archive.path.display()
                        )
                    })?
            {
                return Ok(Some(ResolvedAudioAsset {
                    source_path,
                    origin: AudioAssetOrigin::Archive(archive.path.clone()),
                    bytes,
                }));
            }
        }
    }

    Ok(None)
}

fn resolve_loose_directory(
    directory: &Path,
    virtual_directory: &str,
) -> Result<Option<ResolvedAudioAsset>> {
    if !directory.is_dir() {
        return Ok(None);
    }
    let mut entries = fs::read_dir(directory)
        .with_context(|| format!("reading audio directory {}", directory.display()))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| {
        normalize_asset_path(&left.file_name().to_string_lossy())
            .cmp(&normalize_asset_path(&right.file_name().to_string_lossy()))
            .then_with(|| left.file_name().cmp(&right.file_name()))
    });
    let Some(entry) = entries.into_iter().next() else {
        return Ok(None);
    };
    let path = entry.path();
    let bytes =
        fs::read(&path).with_context(|| format!("reading audio asset {}", path.display()))?;
    let source_path = format!(
        "{}/{}",
        virtual_directory.trim_end_matches('/'),
        normalize_asset_path(&entry.file_name().to_string_lossy())
    );
    Ok(Some(ResolvedAudioAsset {
        source_path,
        origin: AudioAssetOrigin::Loose(path),
        bytes,
    }))
}

/// Stage an audio asset under a content-addressed filename. The normalized
/// source extension is retained so Bevy can choose the appropriate decoder.
/// Repeated staging of identical bytes and an identical extension is a no-op.
pub(crate) fn stage_audio_asset(
    asset: &ResolvedAudioAsset,
    audio_dir: &Path,
) -> Result<StagedAudioAsset> {
    let content_hash = fingerprint(&asset.bytes);
    let extension = source_extension(&asset.source_path).unwrap_or("bin");
    let path = audio_dir.join(format!("{content_hash}.{extension}"));

    if !path.is_file() {
        fs::create_dir_all(audio_dir)
            .with_context(|| format!("creating audio cache {}", audio_dir.display()))?;
        fs::write(&path, &asset.bytes)
            .with_context(|| format!("staging audio asset {}", path.display()))?;
    }

    Ok(StagedAudioAsset { content_hash, path })
}

/// Generate archive names for one plugin. This is crate-visible so preparation
/// can report exactly what was searched without knowing the discovery rules.
pub(crate) fn audio_archive_candidate_names(plugin_name: &str) -> Vec<String> {
    let Some(stem) = Path::new(plugin_name)
        .file_stem()
        .and_then(|value| value.to_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return Vec::new();
    };

    let mut candidates = Vec::new();
    if stem.eq_ignore_ascii_case("fallout3") {
        candidates.push("Fallout - Sound.bsa".to_string());
        candidates.push("Fallout - Sounds.bsa".to_string());
    }
    candidates.push(format!("{stem} - Sound.bsa"));
    candidates.push(format!("{stem} - Sounds.bsa"));
    candidates.push(format!("{stem}.bsa"));
    deduplicate_case_insensitive(candidates)
}

/// Generate normalized virtual paths for a record's sound filename.
pub(crate) fn sound_path_candidates(recorded_path: &str) -> Vec<String> {
    let normalized = normalize_asset_path(recorded_path);
    if normalized.is_empty()
        || normalized
            .split('/')
            .any(|part| part == "." || part == ".." || part.contains(':'))
    {
        return Vec::new();
    }
    let normalized = normalized
        .split('/')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("/");
    if normalized.is_empty() {
        return Vec::new();
    }
    let exact = if normalized.starts_with("sound/") {
        normalized
    } else {
        format!("sound/{normalized}")
    };

    let mut candidates = vec![exact.clone()];
    let slash = exact.rfind('/');
    let dot = exact
        .rfind('.')
        .filter(|dot| slash.is_none_or(|slash| *dot > slash));
    let mp3 = match dot {
        Some(dot) => format!("{}.mp3", &exact[..dot]),
        None => format!("{exact}.mp3"),
    };
    if !mp3.eq_ignore_ascii_case(&exact) {
        candidates.push(mp3);
    }
    candidates
}

fn source_extension(source_path: &str) -> Option<&str> {
    source_path
        .rsplit_once('.')
        .map(|(_, extension)| extension)
        .filter(|extension| {
            !extension.is_empty()
                && extension.len() <= 8
                && extension.bytes().all(|byte| byte.is_ascii_alphanumeric())
        })
}

fn deduplicate_case_insensitive(values: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    values
        .into_iter()
        .filter(|value| seen.insert(value.to_ascii_lowercase()))
        .collect()
}

#[cfg(test)]
#[path = "audio_assets/tests/mod.rs"]
mod tests;
