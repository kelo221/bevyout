use super::*;

#[test]
fn ktx_arguments_select_uastc_zstd_mips_and_color_space() {
    let srgb = ktx_create_arguments(
        Path::new("input.png"),
        Path::new("output.ktx2"),
        TextureColorSpace::Srgb,
    );
    let srgb = srgb
        .iter()
        .map(|value| value.to_string_lossy())
        .collect::<Vec<_>>();
    assert_eq!(srgb[0], "create");
    assert!(srgb.contains(&std::borrow::Cow::Borrowed("R8G8B8A8_SRGB")));
    assert!(srgb.contains(&std::borrow::Cow::Borrowed("uastc")));
    assert!(srgb.contains(&std::borrow::Cow::Borrowed("9")));
    assert!(srgb.contains(&std::borrow::Cow::Borrowed("--generate-mipmap")));

    let linear = ktx_create_arguments(
        Path::new("input.png"),
        Path::new("output.ktx2"),
        TextureColorSpace::Linear,
    );
    assert!(
        linear
            .iter()
            .any(|value| value == std::ffi::OsStr::new("R8G8B8A8_UNORM"))
    );
}

#[test]
fn ktx_validation_rejects_bad_identifiers_and_placeholders() {
    assert!(validate_ktx2_payload(b"not a ktx texture").is_err());
    let mut header = vec![0; 48];
    header[..12].copy_from_slice(KTX2_IDENTIFIER);
    header[20..24].copy_from_slice(&1u32.to_le_bytes());
    header[24..28].copy_from_slice(&1u32.to_le_bytes());
    header[40..44].copy_from_slice(&1u32.to_le_bytes());
    assert!(validate_ktx2_payload(&header).is_err());
    header[20..24].copy_from_slice(&2u32.to_le_bytes());
    header[24..28].copy_from_slice(&2u32.to_le_bytes());
    assert_eq!(validate_ktx2_payload(&header).unwrap(), (2, 2));
}

fn glb_with_embedded_ktx2() -> Vec<u8> {
    let geometry = [1u8, 2, 3, 4, 5, 6, 7, 8];
    let mut ktx2 = vec![0; 80];
    ktx2[..12].copy_from_slice(KTX2_IDENTIFIER);
    ktx2[20..24].copy_from_slice(&2u32.to_le_bytes());
    ktx2[24..28].copy_from_slice(&2u32.to_le_bytes());
    ktx2[40..44].copy_from_slice(&1u32.to_le_bytes());
    let mut binary = geometry.to_vec();
    binary.extend_from_slice(&ktx2);
    let document = serde_json::json!({
        "asset": {"version": "2.0"},
        "buffers": [{"byteLength": binary.len()}],
        "bufferViews": [
            {"buffer": 0, "byteOffset": 0, "byteLength": geometry.len()},
            {"buffer": 0, "byteOffset": geometry.len(), "byteLength": ktx2.len()}
        ],
        "images": [{"bufferView": 1, "mimeType": "image/ktx2", "name": "shared"}],
        "textures": [{"source": 0}]
    });
    let mut json = serde_json::to_vec(&document).unwrap();
    while !json.len().is_multiple_of(4) {
        json.push(b' ');
    }
    let total = 20 + json.len() + 8 + binary.len();
    let mut result = Vec::new();
    result.extend_from_slice(b"glTF");
    result.extend_from_slice(&2u32.to_le_bytes());
    result.extend_from_slice(&(total as u32).to_le_bytes());
    result.extend_from_slice(&(json.len() as u32).to_le_bytes());
    result.extend_from_slice(b"JSON");
    result.extend_from_slice(&json);
    result.extend_from_slice(&(binary.len() as u32).to_le_bytes());
    result.extend_from_slice(b"BIN\0");
    result.extend_from_slice(&binary);
    result
}

#[test]
fn external_texture_rewrite_preserves_geometry_and_replaces_embedded_payload() {
    let source = glb_with_embedded_ktx2();
    let mut published = Vec::new();
    let rewritten = externalize_glb_ktx2_images(&source, |image_index, bytes| {
        assert_eq!(image_index, 0);
        validate_ktx2_payload(bytes)?;
        published.push(bytes.to_vec());
        Ok("objects/texture/aa/bb/shared.ktx2".into())
    })
    .unwrap();

    assert_eq!(published.len(), 1);
    gltf::Gltf::from_slice(&rewritten).unwrap();
    let json_len = u32::from_le_bytes(rewritten[12..16].try_into().unwrap()) as usize;
    let binary_len =
        u32::from_le_bytes(rewritten[20 + json_len..24 + json_len].try_into().unwrap()) as usize;
    assert_eq!(
        binary_len, 12,
        "geometry plus one four-byte image placeholder"
    );
    let document: serde_json::Value =
        serde_json::from_slice(&rewritten[20..20 + json_len]).unwrap();
    assert_eq!(
        document["images"][0]["uri"].as_str(),
        Some("/objects/texture/aa/bb/shared.ktx2")
    );
    assert!(document["images"][0].get("bufferView").is_none());
    assert_eq!(document["bufferViews"][1]["byteLength"], 4);
    let binary_start = 20 + json_len + 8;
    assert_eq!(
        &rewritten[binary_start..binary_start + 8],
        &[1, 2, 3, 4, 5, 6, 7, 8]
    );
}
