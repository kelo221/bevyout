use super::*;
use serde_json::json;

#[test]
fn native_material_extensions_are_supported_when_required() {
    assert!(
        unsupported_required_extensions([
            "KHR_materials_unlit",
            "KHR_materials_specular",
            "KHR_materials_emissive_strength",
            "KHR_materials_volume",
        ])
        .is_empty()
    );
}

#[test]
fn unknown_required_extensions_remain_deterministic_errors() {
    assert_eq!(
        unsupported_required_extensions(["VENDOR_unknown", "VENDOR_unknown"]),
        ["VENDOR_unknown"]
    );
}

#[test]
fn composed_scene_declares_every_preserved_material_extension() {
    assert_eq!(
        material_extensions_used(&[
            json!({"extensions":{"KHR_materials_unlit":{}}}),
            json!({"extensions":{"KHR_materials_specular":{}}}),
            json!({"extensions":{"KHR_materials_unlit":{}}}),
        ]),
        ["KHR_materials_specular", "KHR_materials_unlit"]
    );
}
