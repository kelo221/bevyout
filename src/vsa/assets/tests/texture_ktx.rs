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
