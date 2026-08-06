use super::*;

#[test]
fn known_blinn_phong_exponents_map_to_perceptual_ggx_roughness() {
    for (exponent, expected) in [
        (0.0, 1.0),
        (10.0, 1.0),
        (40.0, 0.817_491_5),
        (70.0, 0.714_434_5),
        (100.0, 0.654_855_6),
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
fn legacy_micro_roughness_matches_authored_blinn_phong_exponents() {
    let broad = legacy_micro_roughness_from_glossiness(Some(4.0));
    let tight = legacy_micro_roughness_from_glossiness(Some(128.0));
    assert!((broad - 0.577_350_26).abs() < 0.000_001);
    assert!((tight - 0.124_034_73).abs() < 0.000_001);
}

#[test]
fn chan_master_scales_authored_micro_roughness() {
    let exponent = Some(4.0);
    assert_eq!(legacy_chan_weight(exponent, 0.0), 0.0);
    assert!((legacy_chan_weight(exponent, 0.5) - 0.288_675_13).abs() < 0.000_001);
    assert!((legacy_chan_weight(exponent, 1.0) - 0.577_350_26).abs() < 0.000_001);
}

#[test]
fn legacy_exponent_sanitization_uses_ten_for_invalid_values() {
    for value in [None, Some(-1.0), Some(f32::NAN), Some(f32::INFINITY)] {
        assert_eq!(sanitized_glossiness_exponent(value), 10.0);
        assert!((legacy_micro_roughness_from_glossiness(value) - 0.408_248_3).abs() < 0.000_001);
    }
}

#[test]
fn fallout_specular_strength_uses_normal_alpha_only_when_eligible() {
    let normal = "textures/furniture/chair03_n.dds";
    assert_eq!(
        fallout_specular_texture_path(true, Some(normal)).as_deref(),
        Some(normal)
    );
    assert_eq!(fallout_specular_texture_path(false, Some(normal)), None);
    assert_eq!(fallout_specular_texture_path(true, None), None);
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
