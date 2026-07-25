//! Pure Fallout-to-glTF material conversion policy.

use std::collections::BTreeMap;

pub(crate) const DEFAULT_GLOSSINESS_EXPONENT: f32 = 10.0;
pub(crate) const MATERIAL_POLICY_REVISION: &str = "fallout-pbr-materials-v4-authored-roughness";
pub(crate) const METALLIC_MATERIALS_CSV: &str = include_str!("metallic_materials.csv");

/// Converts a Blinn-Phong exponent into tuned glTF/Bevy perceptual GGX roughness.
///
/// Preserve Fallout's authored gloss hierarchy while translating its
/// Blinn-Phong exponent to glTF/Bevy perceptual GGX roughness.
pub(crate) fn perceptual_roughness_from_glossiness(glossiness: Option<f32>) -> f32 {
    let exponent = glossiness
        .filter(|value| value.is_finite() && *value >= 0.0)
        .unwrap_or(DEFAULT_GLOSSINESS_EXPONENT);
    (2.0 / (exponent + 2.0)).powf(0.25).clamp(0.0, 1.0)
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
mod tests {
    use super::*;

    #[test]
    fn known_blinn_phong_exponents_map_to_perceptual_ggx_roughness() {
        for (exponent, expected) in [
            (0.0, 1.0),
            (10.0, 0.638_943_1),
            (70.0, 0.408_248_3),
            (100.0, 0.374_203_18),
        ] {
            let actual = perceptual_roughness_from_glossiness(Some(exponent));
            assert!((actual - expected).abs() < 0.000_001);
        }
    }

    #[test]
    fn invalid_glossiness_uses_exponent_ten() {
        let expected = perceptual_roughness_from_glossiness(Some(10.0));
        assert_eq!(perceptual_roughness_from_glossiness(None), expected);
        assert_eq!(perceptual_roughness_from_glossiness(Some(-1.0)), expected);
        assert_eq!(
            perceptual_roughness_from_glossiness(Some(f32::NAN)),
            expected
        );
        assert_eq!(
            perceptual_roughness_from_glossiness(Some(f32::INFINITY)),
            expected
        );
    }

    #[test]
    fn exact_normalized_texture_paths_select_binary_metalness() {
        let table = MetallicMaterialTable::parse(
            "diffuse_texture,object_name,metallic\ntextures/weapons/test.dds,Test Weapon,1\n",
        )
        .unwrap();
        assert_eq!(
            table.metallic_factor(Some(r"Data\Textures\Weapons\TEST.DDS")),
            1.0
        );
        assert_eq!(
            table.metallic_factor(Some("textures/weapons/other.dds")),
            0.0
        );
        assert_eq!(table.metallic_factor(None), 0.0);
    }

    #[test]
    fn rusted_megaton_shingles_are_not_treated_as_bare_metal() {
        let table = MetallicMaterialTable::built_in().unwrap();
        for texture in [
            "textures/architecture/megaton/metalscrapshingle08.dds",
            "textures/architecture/megaton/metalscrapshingles04.dds",
            "textures/architecture/megaton/metalscrapshingles05.dds",
            "textures/architecture/megaton/metalscrapshingles06.dds",
        ] {
            assert_eq!(table.metallic_factor(Some(texture)), 0.0, "{texture}");
        }
    }

    #[test]
    fn malformed_and_duplicate_rows_are_rejected() {
        assert!(
            MetallicMaterialTable::parse(
                "diffuse_texture,object_name,metallic\ntextures/weapons/test.dds,Test Weapon,0.5\n"
            )
            .is_err()
        );
        assert!(
            MetallicMaterialTable::parse(
                "diffuse_texture,object_name,metallic\ntextures/weapons/test.dds,Test Weapon,1\nTEXTURES\\WEAPONS\\TEST.DDS,Duplicate Weapon,0\n"
            )
            .is_err()
        );
        assert!(
            MetallicMaterialTable::parse(
                "diffuse_texture,object_name,metallic\ntextures/weapons/test.dds,,1\n"
            )
            .is_err()
        );
    }
}
