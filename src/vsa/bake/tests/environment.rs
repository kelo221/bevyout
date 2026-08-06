use super::*;
use std::fs::File;

#[test]
fn loads_an_authored_hdr_file_as_float_radiance() {
    let path = std::env::temp_dir().join(format!(
        "bevyout-environment-map-{}.hdr",
        std::process::id()
    ));
    let pixels = vec![image::Rgb([1.5_f32, 0.75, 0.25]); 4 * 2];
    image::codecs::hdr::HdrEncoder::new(File::create(&path).unwrap())
        .encode(&pixels, 4, 2)
        .unwrap();
    let map = EnvironmentMap::load(&path).unwrap();
    let sampled = map.sample([1.0, 0.0, 0.0]);
    let _ = std::fs::remove_file(path);
    assert!((sampled[0] - 1.5).abs() < 0.03);
    assert!((sampled[1] - 0.75).abs() < 0.03);
    assert!((sampled[2] - 0.25).abs() < 0.03);
}

#[test]
fn constant_radiance_is_constant_for_all_directions() {
    let map = EnvironmentMap::from_pixels(4, 2, vec![[2.0, 1.0, 0.5]; 8]).unwrap();
    for direction in [
        [1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, -1.0, 0.0],
    ] {
        assert_eq!(map.sample(direction), [2.0, 1.0, 0.5]);
    }
}

#[test]
fn horizontal_sampling_wraps_and_vertical_sampling_clamps() {
    let map = EnvironmentMap::from_pixels(
        4,
        2,
        vec![
            [1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
            [4.0, 0.0, 0.0],
            [5.0, 0.0, 0.0],
            [6.0, 0.0, 0.0],
            [7.0, 0.0, 0.0],
            [8.0, 0.0, 0.0],
        ],
    )
    .unwrap();
    let right = map.sample([1.0, 0.0, 0.0])[0];
    let wrapped = map.sample([1.0, 0.0, -0.0001])[0];
    assert!((right - 4.5).abs() < 0.01);
    assert!((wrapped - right).abs() < 0.01);
    assert_eq!(map.sample([0.0, 1.0, 0.0]), [2.5, 0.0, 0.0]);
    assert_eq!(map.sample([0.0, -1.0, 0.0]), [6.5, 0.0, 0.0]);
}

#[test]
fn importance_sampling_is_deterministic_and_has_a_solid_angle_pdf() {
    let map = EnvironmentMap::from_pixels(
        4,
        3,
        vec![
            [1.0, 0.5, 0.25],
            [2.0, 1.0, 0.5],
            [4.0, 2.0, 1.0],
            [8.0, 4.0, 2.0],
            [0.5, 0.25, 0.125],
            [1.5, 0.75, 0.375],
            [3.0, 1.5, 0.75],
            [6.0, 3.0, 1.5],
            [0.25, 0.125, 0.0625],
            [1.25, 0.625, 0.3125],
            [2.5, 1.25, 0.625],
            [5.0, 2.5, 1.25],
        ],
    )
    .unwrap();
    let first = map.sample_importance(0.123, 0.456);
    assert_eq!(first, map.sample_importance(0.123, 0.456));
    assert!(first.pdf_solid_angle.is_finite());
    assert!(first.pdf_solid_angle > 0.0);

    let mut integrated_mass = 0.0;
    for y in 0..map.height {
        for x in 0..map.width {
            let u = (x as f32 + 0.5) / map.width as f32;
            let v = (y as f32 + 0.5) / map.height as f32;
            let direction = EnvironmentMap::direction_from_uv(u, v);
            integrated_mass += map.pdf_solid_angle(direction) * map.pixel_solid_angle(y);
        }
    }
    assert!((integrated_mass - 1.0).abs() < 1e-5);
}

#[test]
fn importance_sampling_uses_exact_solid_angle_and_cosine_uniform_rows() {
    let width = 4;
    let height = 4;
    let mut pixels = vec![[0.0; 3]; (width * height) as usize];
    pixels[0] = [1.0, 1.0, 1.0];
    let map = EnvironmentMap::from_pixels(width, height, pixels).unwrap();

    let expected_solid_angle =
        std::f32::consts::TAU / width as f32 * (1.0 - (std::f32::consts::PI / height as f32).cos());
    assert!((map.pixel_solid_angle(0) - expected_solid_angle).abs() < 1.0e-6);
    assert!((map.importance_total - expected_solid_angle).abs() < 1.0e-6);

    for residual in [0.1, 0.5, 0.9] {
        let residual = residual * 0.9;
        let sample = map.sample_importance(residual, 0.25);
        let expected_cosine =
            (1.0 - residual) * 1.0 + (std::f32::consts::PI / height as f32).cos() * residual;
        assert!((sample.direction[1] - expected_cosine).abs() < 1.0e-5);
        assert!(sample.pdf_solid_angle.is_finite() && sample.pdf_solid_angle > 0.0);
    }
}

#[test]
fn invalid_dimensions_and_radiance_are_rejected() {
    assert!(EnvironmentMap::from_pixels(0, 1, Vec::new()).is_err());
    assert!(EnvironmentMap::from_pixels(1, 1, vec![[-1.0, 0.0, 0.0]]).is_err());
    assert!(EnvironmentMap::from_pixels(1, 1, vec![[f32::NAN, 0.0, 0.0]]).is_err());
}
