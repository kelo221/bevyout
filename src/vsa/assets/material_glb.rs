//! Post-processes converter GLBs with the shared authored material policy.

use super::*;

const DIFFUSE_PATH_EXTRA: &str = "bevyout_diffuse_texture_path";
const ROUGHNESS_EXTRA: &str = "bevyout_perceptual_roughness";
const GLOSSINESS_EXTRA: &str = "bevyout_glossiness_exponent";
const FALLOUT_MATERIAL_EXTRA: &str = "bevyout_fallout_material";

#[cfg(test)]
pub(crate) fn patch_glb_material_policy(
    bytes: &[u8],
    table: &MetallicMaterialTable,
) -> Result<Vec<u8>> {
    patch_glb_material_policy_with_glossiness(bytes, table, None)
}

pub(crate) fn patch_glb_material_policy_with_glossiness(
    bytes: &[u8],
    table: &MetallicMaterialTable,
    glossiness_exponents: Option<&[f32]>,
) -> Result<Vec<u8>> {
    if bytes.len() < 20 || &bytes[0..4] != b"glTF" {
        bail!("material policy received an invalid GLB header");
    }
    let old_json_length = usize::try_from(u32::from_le_bytes(bytes[12..16].try_into().unwrap()))
        .context("GLB JSON length exceeds the addressable range")?;
    if &bytes[16..20] != b"JSON" || 20 + old_json_length > bytes.len() {
        bail!("material policy received an invalid GLB JSON chunk");
    }
    let remaining_offset = 20 + old_json_length;
    let mut document: serde_json::Value = serde_json::from_slice(&bytes[20..remaining_offset])
        .context("decoding GLB material JSON")?;

    let texture_sources = document
        .get("textures")
        .and_then(serde_json::Value::as_array)
        .map(|textures| {
            textures
                .iter()
                .map(|texture| texture.get("source").and_then(serde_json::Value::as_u64))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let image_paths = document
        .get("images")
        .and_then(serde_json::Value::as_array)
        .map(|images| {
            images
                .iter()
                .map(|image| {
                    image
                        .get("name")
                        .or_else(|| image.get("uri"))
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if let Some(materials) = document
        .get_mut("materials")
        .and_then(serde_json::Value::as_array_mut)
    {
        for (material_index, material) in materials.iter_mut().enumerate() {
            let Some(material_object) = material.as_object_mut() else {
                continue;
            };
            let annotated_glossiness = material_object
                .get("extras")
                .and_then(serde_json::Value::as_object)
                .and_then(|extras| extras.get(GLOSSINESS_EXTRA))
                .and_then(serde_json::Value::as_f64)
                .map(|value| value as f32);
            let glossiness = sanitized_glossiness_exponent(
                glossiness_exponents
                    .and_then(|values| values.get(material_index).copied())
                    .or(annotated_glossiness),
            );
            let annotated_path = material_object
                .get("extras")
                .and_then(serde_json::Value::as_object)
                .and_then(|extras| extras.get(DIFFUSE_PATH_EXTRA))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned);
            let pbr = material_object
                .entry("pbrMetallicRoughness")
                .or_insert_with(|| serde_json::json!({}));
            let Some(pbr_object) = pbr.as_object_mut() else {
                bail!("GLB material pbrMetallicRoughness is not an object");
            };
            let image_path = pbr_object
                .get("baseColorTexture")
                .and_then(|texture| texture.get("index"))
                .and_then(serde_json::Value::as_u64)
                .and_then(|texture| texture_sources.get(texture as usize).copied().flatten())
                .and_then(|image| image_paths.get(image as usize).cloned().flatten());
            let diffuse_path = annotated_path.as_deref().or(image_path.as_deref());
            pbr_object.insert(
                "metallicFactor".into(),
                serde_json::Value::from(table.metallic_factor(diffuse_path)),
            );

            if let Some(extras) = material_object
                .get_mut("extras")
                .and_then(serde_json::Value::as_object_mut)
            {
                extras.remove(DIFFUSE_PATH_EXTRA);
                extras.remove(ROUGHNESS_EXTRA);
                extras.remove(GLOSSINESS_EXTRA);
                let encoded_as_string = extras
                    .get(FALLOUT_MATERIAL_EXTRA)
                    .is_some_and(serde_json::Value::is_string);
                let mut semantics = extras
                    .get(FALLOUT_MATERIAL_EXTRA)
                    .and_then(|value| match value {
                        serde_json::Value::Object(_) => Some(value.clone()),
                        serde_json::Value::String(value) => serde_json::from_str(value).ok(),
                        _ => None,
                    })
                    .unwrap_or_else(|| serde_json::json!({}));
                let semantics_object = semantics
                    .as_object_mut()
                    .context("Fallout material metadata is not an object")?;
                semantics_object.insert("schema".into(), serde_json::Value::from(2));
                semantics_object.insert(
                    "glossiness_exponent".into(),
                    serde_json::Value::from(glossiness),
                );
                let semantics = if encoded_as_string {
                    serde_json::Value::String(serde_json::to_string(&semantics)?)
                } else {
                    semantics
                };
                extras.insert(FALLOUT_MATERIAL_EXTRA.into(), semantics);
                if extras.is_empty() {
                    material_object.remove("extras");
                }
            } else {
                material_object.insert(
                    "extras".into(),
                    serde_json::json!({
                        FALLOUT_MATERIAL_EXTRA: {
                            "schema": 2,
                            "glossiness_exponent": glossiness,
                        }
                    }),
                );
            }
        }
    }

    let mut json = serde_json::to_vec(&document)?;
    while !json.len().is_multiple_of(4) {
        json.push(b' ');
    }
    let total_length = 20_usize
        .checked_add(json.len())
        .and_then(|length| length.checked_add(bytes.len() - remaining_offset))
        .context("material-patched GLB length overflow")?;
    let mut patched = Vec::with_capacity(total_length);
    patched.extend_from_slice(&bytes[0..8]);
    patched.extend_from_slice(&u32::try_from(total_length)?.to_le_bytes());
    patched.extend_from_slice(&u32::try_from(json.len())?.to_le_bytes());
    patched.extend_from_slice(b"JSON");
    patched.extend_from_slice(&json);
    patched.extend_from_slice(&bytes[remaining_offset..]);
    gltf::Gltf::from_slice(&patched).context("validating material-patched GLB")?;
    Ok(patched)
}

#[cfg(test)]
#[path = "tests/material_glb.rs"]
mod tests;
