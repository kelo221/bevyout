//! Fallout interface sprite discovery and staging.

use super::*;

/// Direct, non-atlas sprites used by the current Pip-Boy status surface.
const PIPBOY_SPRITES: [&str; 8] = [
    "interface/shared/background/pipboy.dds",
    "interface/stats/face_00.dds",
    "interface/stats/head.dds",
    "interface/stats/left_arm.dds",
    "interface/stats/left_leg.dds",
    "interface/stats/right_arm.dds",
    "interface/stats/right_leg.dds",
    "interface/stats/torso.dds",
];

pub(crate) fn stage_pipboy_sprites(
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    staging_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    for source_path in PIPBOY_SPRITES {
        let Some(bytes) = resolve_asset(data_root, archives, source_path)
            .with_context(|| format!("reading interface sprite {source_path}"))?
        else {
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!("missing interface sprite {source_path}"),
            });
            continue;
        };
        let destination = staging_dir.join(source_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        if !destination.is_file() {
            fs::write(destination, bytes)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipboy_sprite_allowlist_uses_direct_interface_dds_paths() {
        assert!(PIPBOY_SPRITES.iter().all(|path| {
            path.starts_with("interface/") && path.ends_with(".dds") && !path.contains("..")
        }));
        assert!(PIPBOY_SPRITES.contains(&"interface/shared/background/pipboy.dds"));
        assert!(PIPBOY_SPRITES.contains(&"interface/stats/head.dds"));
    }
}
