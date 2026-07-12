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
    for candidate in sound_path_candidates(recorded_path) {
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
        }
    }

    Ok(None)
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
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn normalizes_sound_paths_and_adds_mp3_fallback() {
        assert_eq!(
            sound_path_candidates(r"\FX\AMB\Room.WAV"),
            vec![
                "sound/fx/amb/room.wav".to_string(),
                "sound/fx/amb/room.mp3".to_string(),
            ]
        );
        assert_eq!(
            sound_path_candidates(r"sound\music\theme.mp3"),
            vec!["sound/music/theme.mp3".to_string()]
        );
        assert!(sound_path_candidates("../outside.wav").is_empty());
    }

    #[test]
    fn emits_base_dlc_and_plugin_stem_archive_names() {
        assert_eq!(
            audio_archive_candidate_names("Fallout3.esm"),
            vec![
                "Fallout - Sound.bsa",
                "Fallout - Sounds.bsa",
                "Fallout3 - Sound.bsa",
                "Fallout3 - Sounds.bsa",
                "Fallout3.bsa",
            ]
        );
        assert_eq!(
            audio_archive_candidate_names("BrokenSteel.esm"),
            vec![
                "BrokenSteel - Sound.bsa",
                "BrokenSteel - Sounds.bsa",
                "BrokenSteel.bsa",
            ]
        );
    }

    #[test]
    fn case_insensitive_dedup_keeps_the_highest_precedence_name() {
        assert_eq!(
            deduplicate_case_insensitive(vec![
                "Selected - Sounds.bsa".into(),
                "selected - sounds.BSA".into(),
                "Master - Sounds.bsa".into(),
            ]),
            vec!["Selected - Sounds.bsa", "Master - Sounds.bsa"]
        );
    }

    #[test]
    fn stages_identical_content_once_and_preserves_extension() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "bevyout-audio-assets-{}-{nonce}",
            std::process::id()
        ));
        let first = ResolvedAudioAsset {
            source_path: "sound/fx/first.wav".into(),
            origin: AudioAssetOrigin::Archive(PathBuf::from("first.bsa")),
            bytes: b"audio bytes".to_vec(),
        };
        let second = ResolvedAudioAsset {
            source_path: "sound/fx/second.wav".into(),
            origin: AudioAssetOrigin::Loose(PathBuf::from("second.wav")),
            bytes: first.bytes.clone(),
        };

        let staged_first = stage_audio_asset(&first, &root).unwrap();
        let staged_second = stage_audio_asset(&second, &root).unwrap();
        assert_eq!(staged_first, staged_second);
        assert_eq!(
            staged_first
                .path
                .extension()
                .and_then(|value| value.to_str()),
            Some("wav")
        );
        assert_eq!(fs::read(&staged_first.path).unwrap(), first.bytes);

        fs::remove_dir_all(root).unwrap();
    }
}
