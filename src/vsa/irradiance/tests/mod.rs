use super::*;

#[test]
fn constant_l0_produces_equal_ambient_cube_sides() {
    let color = evaluate_lambertian(
        [1.0, 0.5, 0.25],
        [0.0; 3],
        [0.0; 3],
        [0.0; 3],
        [1.0, 0.0, 0.0],
    );
    assert_eq!(color, [SH_L0_M0, SH_L0_M0 * 0.5, SH_L0_M0 * 0.25]);
}

#[test]
fn rgb9e5_zero_is_zero_and_positive_values_have_an_exponent() {
    assert_eq!(pack_rgb9e5([0.0; 3]), 0);
    assert_ne!(pack_rgb9e5([1.0, 0.5, 0.25]), 0);
}

#[test]
fn atlas_writes_one_raw_file_per_3d_slice_for_ktx() {
    let directory =
        std::env::temp_dir().join(format!("bevyout-irradiance-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("create test directory");
    let raw_path = directory.join("irradiance.raw");
    let slices = write_atlas(
        &raw_path,
        [1, 1, 1],
        &[1.0, 1.0, 1.0],
        &[0.0; 3],
        &[0.0; 3],
        &[0.0; 3],
    )
    .expect("write test slices");
    assert_eq!(slices.len(), 3);
    assert!(
        slices
            .iter()
            .all(|path| { std::fs::metadata(path).expect("slice exists").len() == 8 })
    );
    std::fs::remove_dir_all(directory).expect("remove test directory");
}
