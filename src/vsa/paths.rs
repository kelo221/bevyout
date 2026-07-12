use anyhow::{Context, Result, bail};
use bevy::prelude::*;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

use super::plugin::ReferenceRecord;

/// Fallout 3 uses roughly 70 world units per metre. Keep the offline placement
/// conversion in metres so Bevy physics and the FPS controller use metric
/// dimensions as well.
pub(crate) const FO3_SCALE: f32 = 1.0 / 70.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CellSelector {
    FormId(u32),
    EditorId(String),
}

impl std::fmt::Display for CellSelector {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FormId(form_id) => write!(formatter, "{form_id:08x}"),
            Self::EditorId(editor_id) => formatter.write_str(editor_id),
        }
    }
}

pub(crate) fn parse_form_id(value: &str) -> Result<u32> {
    let value = value
        .trim()
        .trim_start_matches("0x")
        .trim_start_matches("0X");
    u32::from_str_radix(value, 16).context("cell must be a hexadecimal FormID")
}

pub(crate) fn parse_cell_selector(value: &str) -> Result<CellSelector> {
    let value = value.trim();
    if value.is_empty() {
        bail!("cell selector must be a GECK EditorID or hexadecimal FormID")
    }
    let form_id_value = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"));
    let looks_like_form_id = form_id_value.is_some_and(|value| {
        !value.is_empty()
            && value.len() <= 8
            && value.chars().all(|character| character.is_ascii_hexdigit())
    }) || (value.len() <= 8
        && value.chars().all(|character| character.is_ascii_hexdigit()));
    if looks_like_form_id {
        return Ok(CellSelector::FormId(parse_form_id(value)?));
    }
    Ok(CellSelector::EditorId(value.to_owned()))
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
    placement_transform_parts(reference.position, reference.rotation, reference.scale)
}

pub(crate) fn placement_transform_parts(
    position: [f32; 3],
    rotation_euler: [f32; 3],
    scale: f32,
) -> ([f32; 3], [f32; 4], f32) {
    let p = position;
    let translation = [p[0] * FO3_SCALE, p[2] * FO3_SCALE, -p[1] * FO3_SCALE];
    let q = Quat::from_euler(
        EulerRot::XYZ,
        rotation_euler[0],
        rotation_euler[1],
        rotation_euler[2],
    );
    let basis = Mat3::from_cols(Vec3::X, Vec3::Z, -Vec3::Y);
    let converted = basis * Mat3::from_quat(q) * basis.transpose();
    let rotation = Quat::from_mat3(&converted).to_array();
    (translation, rotation, scale)
}

#[cfg(test)]
mod tests {
    use super::super::plugin::ReferenceRecord;

    #[test]
    fn parses_editor_id_and_form_id_selectors() {
        assert_eq!(
            parse_cell_selector("SuperDuperMart").unwrap(),
            CellSelector::EditorId("SuperDuperMart".into())
        );
        assert_eq!(
            parse_cell_selector("00017f37").unwrap(),
            CellSelector::FormId(0x0001_7f37)
        );
        assert_eq!(
            parse_cell_selector("0x151e3").unwrap(),
            CellSelector::FormId(0x0001_51e3)
        );
        assert_eq!(
            parse_cell_selector("151e3").unwrap(),
            CellSelector::FormId(0x0001_51e3)
        );
    }

    #[test]
    fn rejects_empty_cell_selectors() {
        assert!(parse_cell_selector(" ").is_err());
    }
    use super::*;

    #[test]
    fn parses_form_ids_with_optional_hex_prefix() {
        assert_eq!(parse_form_id("0x000151e3").unwrap(), 0x0001_51e3);
        assert_eq!(parse_form_id("151E3").unwrap(), 0x0001_51e3);
    }

    #[test]
    fn placement_transform_uses_metric_fallout_scale() {
        let reference = ReferenceRecord {
            form_id: 1,
            base_form_id: 2,
            position: [70.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: 1.0,
            flags: 0,
            ..Default::default()
        };
        let (translation, _, _) = placement_transform(&reference);
        assert!((translation[0] - 1.0).abs() < f32::EPSILON);
        assert!((translation[1]).abs() < f32::EPSILON);
        assert!((translation[2]).abs() < f32::EPSILON);
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
