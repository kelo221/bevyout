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
        Some(
            "marker_creature.nif"
                | "markerx.nif"
                | "markerxheading.nif"
                | "marker_north.nif"
                | "markercocheading.nif",
        )
    )
}

pub(crate) fn is_non_rendering_effect(path: &str) -> bool {
    path.rsplit('/').next().is_some_and(|name| {
        name.starts_with("fx")
            || name.starts_with("spraymeshconnect")
            // Fallout helper/rig assets below have no render mesh by design.
            || name == "fakefog01.nif"
            || matches!(
                path,
                "clutter/grocery/grocerydisplaycountercubeshado01.nif"
                    | "clutter/grocery/groceryshelvestiltedshadow01.nif"
                    | "creatures/protectron/skeleton.nif"
            )
    })
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
    let rotation = fallout_rotation_to_bevy(rotation_euler).to_array();
    (translation, rotation, scale)
}

/// Convert an ESM object rotation to the Bevy/glTF basis used by prepared placements.
///
/// The ESM Euler values use the established Fallout 3 placement convention.
/// Keep the basis conjugation at this boundary so the manifest remains a Bevy
/// quaternion and both dynamic and baked placement paths consume the same value.
fn fallout_rotation_to_bevy(rotation_euler: [f32; 3]) -> Quat {
    let fallout = Quat::from_euler(
        EulerRot::XYZ,
        rotation_euler[0],
        rotation_euler[1],
        rotation_euler[2],
    );
    let basis = Mat3::from_cols(Vec3::X, Vec3::Z, -Vec3::Y);
    let converted = basis * Mat3::from_quat(fallout) * basis.transpose();
    Quat::from_mat3(&converted)
}

#[cfg(test)]
#[path = "paths/tests/mod.rs"]
mod tests;
