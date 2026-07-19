//! Post-processes converter GLBs with the shared authored material policy.

use super::*;

const DIFFUSE_PATH_EXTRA: &str = "bevyout_diffuse_texture_path";
const ROUGHNESS_EXTRA: &str = "bevyout_perceptual_roughness";

pub(crate) fn apply_material_policy_to_glb_file(path: &Path) -> Result<()> {
    let bytes = fs::read(path).with_context(|| format!("reading GLB {}", path.display()))?;
    let table = MetallicMaterialTable::built_in().map_err(anyhow::Error::msg)?;
    let patched = patch_glb_material_policy(&bytes, &table)?;
    let temporary = path.with_extension(format!("material-{}.tmp.glb", std::process::id()));
    fs::write(&temporary, patched)
        .with_context(|| format!("writing material-patched GLB {}", temporary.display()))?;
    if let Err(error) = atomic_replace(&temporary, path) {
        let _ = fs::remove_file(&temporary);
        return Err(error);
    }
    Ok(())
}

pub(crate) fn patch_glb_material_policy(
    bytes: &[u8],
    table: &MetallicMaterialTable,
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
        for material in materials {
            let Some(material_object) = material.as_object_mut() else {
                continue;
            };
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
                if extras.is_empty() {
                    material_object.remove("extras");
                }
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
mod tests {
    use super::*;

    fn synthetic_glb(document: serde_json::Value) -> Vec<u8> {
        let mut json = serde_json::to_vec(&document).unwrap();
        while !json.len().is_multiple_of(4) {
            json.push(b' ');
        }
        let length = 20 + json.len();
        let mut bytes = Vec::with_capacity(length);
        bytes.extend_from_slice(b"glTF");
        bytes.extend_from_slice(&2_u32.to_le_bytes());
        bytes.extend_from_slice(&(length as u32).to_le_bytes());
        bytes.extend_from_slice(&(json.len() as u32).to_le_bytes());
        bytes.extend_from_slice(b"JSON");
        bytes.extend_from_slice(&json);
        bytes
    }

    #[test]
    fn metallic_patch_preserves_roughness_and_specular_without_making_a_texture() {
        let source = synthetic_glb(serde_json::json!({
            "asset": {"version": "2.0"},
            "images": [{"name": "textures/fixtures/bare_metal.dds", "uri": "data:image/png;base64,iVBORw0KGgo="}],
            "textures": [{"source": 0}],
            "materials": [{
                "pbrMetallicRoughness": {
                    "baseColorTexture": {"index": 0},
                    "roughnessFactor": 0.42
                },
                "extensions": {"KHR_materials_specular": {"specularFactor": 0.75}}
            }]
        }));
        let table = MetallicMaterialTable::parse(
            "diffuse_texture,object_name,metallic\ntextures/fixtures/bare_metal.dds,Bare Metal Fixture,1\n",
        )
        .unwrap();
        let patched = patch_glb_material_policy(&source, &table).unwrap();
        let json_length = u32::from_le_bytes(patched[12..16].try_into().unwrap()) as usize;
        let document: serde_json::Value =
            serde_json::from_slice(&patched[20..20 + json_length]).unwrap();
        let material = &document["materials"][0];
        assert_eq!(material["pbrMetallicRoughness"]["metallicFactor"], 1.0);
        assert_eq!(material["pbrMetallicRoughness"]["roughnessFactor"], 0.42);
        assert!(
            material["pbrMetallicRoughness"]
                .get("metallicRoughnessTexture")
                .is_none()
        );
        assert_eq!(
            material["extensions"]["KHR_materials_specular"]["specularFactor"],
            0.75
        );
    }

    #[test]
    fn transient_diffuse_annotation_is_consumed() {
        let source = synthetic_glb(serde_json::json!({
            "asset": {"version": "2.0"},
            "materials": [{
                "extras": {
                    DIFFUSE_PATH_EXTRA: "textures/fixtures/bare_metal.dds",
                    ROUGHNESS_EXTRA: 0.5
                },
                "pbrMetallicRoughness": {"roughnessFactor": 0.5}
            }]
        }));
        let table = MetallicMaterialTable::parse(
            "diffuse_texture,object_name,metallic\ntextures/fixtures/bare_metal.dds,Bare Metal Fixture,1\n",
        )
        .unwrap();
        let patched = patch_glb_material_policy(&source, &table).unwrap();
        let json_length = u32::from_le_bytes(patched[12..16].try_into().unwrap()) as usize;
        let document: serde_json::Value =
            serde_json::from_slice(&patched[20..20 + json_length]).unwrap();
        assert_eq!(
            document["materials"][0]["pbrMetallicRoughness"]["metallicFactor"],
            1.0
        );
        assert!(document["materials"][0].get("extras").is_none());
    }
}
