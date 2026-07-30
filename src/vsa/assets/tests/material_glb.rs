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
