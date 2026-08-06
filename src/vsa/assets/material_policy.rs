//! Pure Fallout-to-glTF material conversion policy.

use std::collections::BTreeMap;

pub(crate) const DEFAULT_GLOSSINESS_EXPONENT: f32 = 10.0;
pub(crate) const MATERIAL_POLICY_REVISION: &str =
    "fallout-pbr-materials-v7-legacy-glossiness-v1-1.75x-roughness";
pub(crate) const METALLIC_MATERIALS_CSV: &str = include_str!("metallic_materials.csv");

/// Converts a Blinn-Phong exponent into tuned glTF/Bevy perceptual GGX roughness.
///
/// Preserve Fallout's authored gloss hierarchy while translating its
/// Blinn-Phong exponent to glTF/Bevy perceptual GGX roughness with a 1.75x modifier.
pub(crate) fn perceptual_roughness_from_glossiness(glossiness: Option<f32>) -> f32 {
    let exponent = sanitized_glossiness_exponent(glossiness);
    (1.75 * (2.0 / (exponent + 2.0)).powf(0.25)).clamp(0.0, 1.0)
}

/// Preserves a usable Fallout Blinn-Phong exponent for legacy direct lighting.
pub(crate) fn sanitized_glossiness_exponent(glossiness: Option<f32>) -> f32 {
    glossiness
        .filter(|value| value.is_finite() && *value >= 0.0)
        .unwrap_or(DEFAULT_GLOSSINESS_EXPONENT)
}

/// Converts Fallout's Blinn-Phong exponent into the matching micro-roughness.
#[allow(dead_code)] // Canonical CPU mirror of the direct-light WGSL policy.
pub(crate) fn legacy_micro_roughness_from_glossiness(glossiness: Option<f32>) -> f32 {
    let exponent = sanitized_glossiness_exponent(glossiness);
    (2.0 / (exponent + 2.0)).sqrt()
}

/// Scales the authored-material Chan contribution by the viewer master control.
#[allow(dead_code)] // Canonical CPU mirror of the direct-light WGSL policy.
pub(crate) fn legacy_chan_weight(glossiness: Option<f32>, master_strength: f32) -> f32 {
    let master_strength = if master_strength.is_finite() {
        master_strength.clamp(0.0, 1.0)
    } else {
        1.0
    };
    (legacy_micro_roughness_from_glossiness(glossiness) * master_strength).clamp(0.0, 1.0)
}

/// Selects Fallout 3's authored specular-strength payload.
///
/// FO3 stores specular strength in the alpha channel of texture slot 1, the
/// normal map. The payload is valid only when the material enables specular
/// shading and a normal texture is present.
pub(crate) fn fallout_specular_texture_path(
    specular_enabled: bool,
    normal_texture: Option<&str>,
) -> Option<String> {
    specular_enabled
        .then(|| normal_texture.map(str::to_owned))
        .flatten()
}

pub(crate) fn normalize_diffuse_texture_path(path: &str) -> Result<String, String> {
    let mut normalized = path.trim().replace('\\', "/").to_ascii_lowercase();
    while normalized.contains("//") {
        normalized = normalized.replace("//", "/");
    }
    let normalized = normalized.trim_start_matches('/');
    let normalized = normalized
        .split('/')
        .position(|segment| segment == "textures")
        .map(|index| {
            normalized
                .split('/')
                .skip(index)
                .collect::<Vec<_>>()
                .join("/")
        })
        .unwrap_or_else(|| normalized.to_owned());
    if !normalized.starts_with("textures/") || normalized.len() == "textures/".len() {
        return Err(format!(
            "diffuse texture path must be rooted at textures/: {path:?}"
        ));
    }
    Ok(normalized.to_owned())
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct MetallicMaterialTable {
    entries: BTreeMap<String, bool>,
}

impl MetallicMaterialTable {
    pub(crate) fn parse(csv: &str) -> Result<Self, String> {
        let mut lines = csv.lines();
        let header = lines
            .next()
            .map(str::trim)
            .ok_or_else(|| "metallic material CSV is empty".to_owned())?;
        if header != "diffuse_texture,object_name,metallic" {
            return Err(format!(
                "unexpected metallic material CSV header: {header:?}"
            ));
        }

        let mut entries = BTreeMap::new();
        for (offset, line) in lines.enumerate() {
            let line_number = offset + 2;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut fields = line.split(',');
            let path = fields.next().unwrap_or_default();
            let object_name = fields.next().ok_or_else(|| {
                format!("metallic material CSV line {line_number} has fewer than three columns")
            })?;
            let metallic = fields.next().ok_or_else(|| {
                format!("metallic material CSV line {line_number} has fewer than three columns")
            })?;
            if fields.next().is_some() {
                return Err(format!(
                    "metallic material CSV line {line_number} has more than two columns"
                ));
            }
            let path = normalize_diffuse_texture_path(path)?;
            if object_name.trim().is_empty() {
                return Err(format!(
                    "metallic material CSV line {line_number} must include an object name"
                ));
            }
            let metallic = match metallic.trim() {
                "0" => false,
                "1" => true,
                value => {
                    return Err(format!(
                        "metallic material CSV line {line_number} must use 0 or 1, got {value:?}"
                    ));
                }
            };
            if entries.insert(path.clone(), metallic).is_some() {
                return Err(format!(
                    "metallic material CSV contains duplicate diffuse texture {path:?}"
                ));
            }
        }
        Ok(Self { entries })
    }

    pub(crate) fn built_in() -> Result<Self, String> {
        Self::parse(METALLIC_MATERIALS_CSV)
    }

    pub(crate) fn metallic_factor(&self, diffuse_texture: Option<&str>) -> f32 {
        let Some(path) = diffuse_texture.and_then(|path| normalize_diffuse_texture_path(path).ok())
        else {
            return 0.0;
        };
        f32::from(self.entries.get(&path).copied().unwrap_or(false))
    }
}

#[cfg(test)]
#[path = "tests/material_policy.rs"]
mod tests;
