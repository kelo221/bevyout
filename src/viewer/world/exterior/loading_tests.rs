use std::path::Path;

use bevyout_core::manifest::exterior::{
    EXTERIOR_CELL_PACKAGE_REVISION, ExteriorCellPackage, GridCoordinate,
    PreparedExteriorEnvironment,
};

use super::decode_package;

#[test]
fn decoded_package_preserves_the_exact_serialized_byte_length() {
    let package = ExteriorCellPackage {
        revision: EXTERIOR_CELL_PACKAGE_REVISION.into(),
        content_fingerprint: "synthetic".into(),
        cell_form_id: 0x10,
        worldspace_form_id: 0x20,
        grid: GridCoordinate::new(-1, 2),
        origin: [0.0; 3],
        terrain: None,
        water: None,
        static_objects: Vec::new(),
        dynamic_objects: Vec::new(),
        distant_objects: Vec::new(),
        local_lights: Vec::new(),
        navigation: None,
        environment: PreparedExteriorEnvironment::default(),
        diagnostics: Vec::new(),
    };
    let canonical = ron::ser::to_string(&package).expect("synthetic package serializes");
    let bytes = format!("\n  {canonical}\n").into_bytes();

    let loaded = decode_package(Path::new("synthetic-package.ron"), &bytes)
        .expect("synthetic package parses");

    assert_eq!(loaded.serialized_bytes, bytes.len() as u64);
    assert_ne!(loaded.serialized_bytes, canonical.len() as u64);
    assert_eq!(loaded.package, package);
}
