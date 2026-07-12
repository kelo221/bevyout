use anyhow::{Context, Result, bail};
use ron::de::from_str;
use std::fs;
use std::path::{Path, PathBuf};

use super::manifest::PreparedSceneManifest;
use super::paths::{CellSelector, absolutize, parse_cell_selector};

pub(crate) fn resolve_cached_manifest(cache_dir: &Path, selector_input: &str) -> Result<PathBuf> {
    let cache_dir = absolutize(cache_dir)?;
    let selector = parse_cell_selector(selector_input)?;
    match find_cached_manifest_for_selector(&cache_dir, &selector)? {
        Some(path) => Ok(path),
        None => match selector {
            CellSelector::FormId(form_id) => {
                let path = cache_dir
                    .join("scenes")
                    .join(format!("{form_id:08x}"))
                    .join("scene.ron");
                bail!(
                    "prepared scene '{selector_input}' was not found at {}; run `prepare {selector_input}` first",
                    path.display()
                )
            }
            CellSelector::EditorId(_) => bail!(
                "prepared scene for GECK EditorID '{selector_input}' was not found under {}; run `prepare {selector_input}` first",
                cache_dir.join("scenes").display()
            ),
        },
    }
}

pub(crate) fn find_cached_manifest(
    cache_dir: &Path,
    selector_input: &str,
) -> Result<Option<PathBuf>> {
    let cache_dir = absolutize(cache_dir)?;
    let selector = parse_cell_selector(selector_input)?;
    find_cached_manifest_for_selector(&cache_dir, &selector)
}

fn find_cached_manifest_for_selector(
    cache_dir: &Path,
    selector: &CellSelector,
) -> Result<Option<PathBuf>> {
    match selector {
        CellSelector::FormId(form_id) => {
            let path = cache_dir
                .join("scenes")
                .join(format!("{form_id:08x}"))
                .join("scene.ron");
            path.is_file()
                .then(|| fs::canonicalize(path))
                .transpose()
                .context("could not resolve cached scene manifest")
        }
        CellSelector::EditorId(editor_id) => {
            let scenes_dir = cache_dir.join("scenes");
            let entries = match fs::read_dir(&scenes_dir) {
                Ok(entries) => entries,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
                Err(error) => return Err(error.into()),
            };
            let mut matches = Vec::new();
            for entry in entries {
                let entry = entry?;
                if !entry.file_type()?.is_dir() {
                    continue;
                }
                let manifest_path = entry.path().join("scene.ron");
                if !manifest_path.is_file() {
                    continue;
                }
                let text = match fs::read_to_string(&manifest_path) {
                    Ok(text) => text,
                    Err(_) => continue,
                };
                let manifest = match from_str::<PreparedSceneManifest>(&text) {
                    Ok(manifest) => manifest,
                    Err(_) => continue,
                };
                if manifest
                    .cell
                    .editor_id
                    .as_deref()
                    .is_some_and(|value| value.eq_ignore_ascii_case(editor_id))
                {
                    matches.push((manifest.cell.form_id, manifest_path));
                }
            }
            match matches.as_slice() {
                [] => Ok(None),
                [(form_id, path)] => {
                    let path = fs::canonicalize(path)?;
                    if path.is_file() {
                        Ok(Some(path))
                    } else {
                        bail!(
                            "cached scene for GECK EditorID '{editor_id}' ({form_id:08x}) is not a file"
                        )
                    }
                }
                _ => {
                    let form_ids = matches
                        .iter()
                        .map(|(form_id, _)| format!("{form_id:08x}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    bail!(
                        "cached GECK EditorID '{editor_id}' is ambiguous; matching FormIDs: {form_ids}"
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_cache() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("bevyout-scene-resolver-{stamp}"))
    }

    fn write_manifest(cache_dir: &Path) {
        let scene_dir = cache_dir.join("scenes").join("00017f37");
        fs::create_dir_all(&scene_dir).unwrap();
        fs::write(
            scene_dir.join("scene.ron"),
            r#"(
                schema_version: 8,
                asset_root: "cache",
                source_plugin: "Fallout3.esm",
                source_fingerprint: "fingerprint",
                cell: (
                    form_id: 98103,
                    editor_id: Some("SuperDuperMart"),
                    name: Some("Super-Duper Mart"),
                    interior: true,
                    ambient_rgba: (0.0, 0.0, 0.0, 0.0),
                    directional_rgba: (0.0, 0.0, 0.0, 0.0),
                ),
                placements: [],
                lights: [],
                diagnostics: [],
            )"#,
        )
        .unwrap();
    }

    #[test]
    fn resolves_editor_id_from_cached_manifest() {
        let cache_dir = temporary_cache();
        write_manifest(&cache_dir);

        let path = resolve_cached_manifest(&cache_dir, "superdupermart").unwrap();
        assert_eq!(
            path,
            fs::canonicalize(cache_dir.join("scenes/00017f37/scene.ron")).unwrap()
        );

        fs::remove_dir_all(cache_dir).unwrap();
    }

    #[test]
    fn reports_missing_cached_manifest_without_error_for_recovery() {
        let cache_dir = temporary_cache();
        assert_eq!(
            find_cached_manifest(&cache_dir, "RooseveltHS04").unwrap(),
            None
        );
    }
}
