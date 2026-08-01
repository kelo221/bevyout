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
        let loose = data_root.join(candidate.replace('/', std::path::MAIN_SEPARATOR_STR));
        if loose.exists() {
            return Ok(Some(fs::read(loose)?));
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
