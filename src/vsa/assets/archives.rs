//! BSA archive loading and asset lookup.

use super::*;

pub(crate) fn load_archives(
    data_root: &Path,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Vec<BsaArchive>> {
    let names = ["Fallout - Meshes.bsa", "Fallout - Textures.bsa"];
    let mut archives = Vec::new();
    for name in names {
        let path = data_root.join(name);
        if path.exists() {
            match BsaArchive::open(&path) {
                Ok(archive) => archives.push(archive),
                Err(error) => diagnostics.push(Diagnostic {
                    severity: "warning".into(),
                    message: format!("could not index {name}: {error}"),
                }),
            }
        }
    }
    Ok(archives)
}

pub(crate) fn resolve_asset(
    data_root: &Path,
    archives: &[BsaArchive],
    normalized: &str,
) -> Result<Option<Vec<u8>>> {
    let normalized = normalize_asset_path(normalized);
    let normalized = normalized
        .strip_prefix("data/")
        .unwrap_or(&normalized)
        .to_owned();
    let candidates = if normalized.starts_with("meshes/") || normalized.starts_with("textures/") {
        vec![normalized]
    } else {
        vec![
            normalized.clone(),
            format!("meshes/{normalized}"),
            format!("textures/{normalized}"),
        ]
    };
    for candidate in candidates {
        if let Some(bytes) = read_loose_asset_case_insensitive(data_root, &candidate)? {
            return Ok(Some(bytes));
        }
        for archive in archives {
            if let Some(bytes) = archive
                .read(&candidate)
                .with_context(|| format!("reading archive asset {candidate}"))?
            {
                return Ok(Some(bytes));
            }
        }
    }
    Ok(None)
}

/// Resolve loose files with the same case-insensitive path semantics as BSA
/// entries. Windows provides this at the filesystem layer, but preparation
/// and its synthetic fixtures also run on case-sensitive hosts; walking one
/// directory component at a time keeps the lookup deterministic there too.
fn read_loose_asset_case_insensitive(data_root: &Path, candidate: &str) -> Result<Option<Vec<u8>>> {
    let mut current = data_root.to_owned();
    for component in candidate
        .split('/')
        .filter(|component| !component.is_empty())
    {
        let exact = current.join(component);
        if exact.exists() {
            current = exact;
            continue;
        }

        let mut matches = match fs::read_dir(&current) {
            Ok(entries) => entries
                .filter_map(std::result::Result::ok)
                .filter(|entry| {
                    entry
                        .file_name()
                        .to_string_lossy()
                        .eq_ignore_ascii_case(component)
                })
                .collect::<Vec<_>>(),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(error).with_context(|| {
                    format!("reading loose asset directory {}", current.display())
                });
            }
        };
        matches.sort_by_key(|entry| entry.file_name().to_string_lossy().to_ascii_lowercase());
        let Some(entry) = matches.into_iter().next() else {
            return Ok(None);
        };
        current = entry.path();
    }

    if current.is_file() {
        return Ok(Some(fs::read(current)?));
    }
    Ok(None)
}
