use anyhow::{Context, Result};
use bevy::prelude::*;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

use super::plugin::ReferenceRecord;

const FO3_SCALE: f32 = 0.1;

pub(crate) fn parse_form_id(value: &str) -> Result<u32> {
    let value = value
        .trim()
        .trim_start_matches("0x")
        .trim_start_matches("0X");
    u32::from_str_radix(value, 16).context("cell must be a hexadecimal FormID")
}

pub(crate) fn absolutize(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

pub(crate) fn fingerprint(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

pub(crate) fn normalize_asset_path(value: &str) -> String {
    value
        .replace('\\', "/")
        .trim_start_matches('/')
        .to_ascii_lowercase()
}

pub(crate) fn is_editor_marker(path: &str) -> bool {
    matches!(
        path.rsplit('/').next(),
        Some("markerx.nif" | "markerxheading.nif" | "marker_north.nif" | "markercocheading.nif",)
    )
}

pub(crate) fn is_non_rendering_effect(path: &str) -> bool {
    path.rsplit('/')
        .next()
        .is_some_and(|name| name.starts_with("fx") || name.starts_with("spraymeshconnect"))
}

pub(crate) fn placement_transform(reference: &ReferenceRecord) -> ([f32; 3], [f32; 4], f32) {
    let p = reference.position;
    let translation = [p[0] * FO3_SCALE, p[2] * FO3_SCALE, -p[1] * FO3_SCALE];
    let q = Quat::from_euler(
        EulerRot::XYZ,
        reference.rotation[0],
        reference.rotation[1],
        reference.rotation[2],
    );
    let basis = Mat3::from_cols(Vec3::X, Vec3::Z, -Vec3::Y);
    let converted = basis * Mat3::from_quat(q) * basis.transpose();
    let rotation = Quat::from_mat3(&converted).to_array();
    (translation, rotation, reference.scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_form_ids_with_optional_hex_prefix() {
        assert_eq!(parse_form_id("0x000151e3").unwrap(), 0x0001_51e3);
        assert_eq!(parse_form_id("151E3").unwrap(), 0x0001_51e3);
    }

    #[test]
    fn normalizes_game_asset_paths() {
        assert_eq!(
            normalize_asset_path("\\Textures\\Foo\\BAR.DDS"),
            "textures/foo/bar.dds"
        );
    }

    #[test]
    fn identifies_non_rendering_editor_markers() {
        assert!(is_editor_marker("meshes/markerx.nif"));
        assert!(is_editor_marker("meshes/markerxheading.nif"));
        assert!(is_editor_marker("marker_north.nif"));
        assert!(!is_editor_marker("meshes/furniture/table01.nif"));
    }

    #[test]
    fn identifies_non_rendering_effects() {
        assert!(is_non_rendering_effect(
            "effects/ambient/fxglowsimplefill.nif"
        ));
        assert!(is_non_rendering_effect(
            "effects/ambient/fxdustsimple01.nif"
        ));
        assert!(is_non_rendering_effect("effects/ambient/fxlightbeam05.nif"));
        assert!(is_non_rendering_effect(
            "effects/ambient/spraymeshconnect.nif"
        ));
        assert!(!is_non_rendering_effect("meshes/clutter/lampgeneric01.nif"));
    }
}
