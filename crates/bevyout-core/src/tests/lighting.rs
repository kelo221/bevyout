use super::super::lighting::{
    AMBIENT_BRIGHTNESS, AUTHORED_LIGHTING_SCALE, DEFAULT_AMBIENT_SCALE, DEFAULT_LIGHTING_SCALE,
    ambient_irradiance, cell_directional_illuminance, point_light_intensity, srgb_to_linear_rgb,
};

#[test]
fn point_light_conversion_has_one_authority_for_explicit_and_fallback_values() {
    let fallback = point_light_intensity(4.0, 0.0, DEFAULT_LIGHTING_SCALE);
    assert_eq!(fallback, 4.0 * 4.0 * 2.0 * DEFAULT_LIGHTING_SCALE);

    let explicit = point_light_intensity(4.0, AUTHORED_LIGHTING_SCALE, DEFAULT_LIGHTING_SCALE);
    assert_eq!(explicit, DEFAULT_LIGHTING_SCALE);
}

#[test]
fn point_light_conversion_rejects_invalid_inputs_without_nan() {
    assert!(point_light_intensity(f32::NAN, f32::NAN, 128.0).is_finite());
    assert_eq!(point_light_intensity(-4.0, -1.0, 128.0), 0.0256);
    assert!(point_light_intensity(4.0, 100.0, f32::INFINITY).is_infinite());
}

#[test]
fn ambient_irradiance_is_linear_and_uses_the_runtime_contract() {
    let [red, green, blue] = ambient_irradiance(
        [0.5, 0.25, 0.0, 1.0],
        DEFAULT_LIGHTING_SCALE,
        DEFAULT_AMBIENT_SCALE,
    );
    let expected_red = srgb_to_linear_rgb([0.5, 0.25, 0.0])[0]
        * AMBIENT_BRIGHTNESS
        * DEFAULT_LIGHTING_SCALE
        * DEFAULT_AMBIENT_SCALE;
    assert!((red - expected_red).abs() < 1e-5);
    assert!(green > 0.0);
    assert_eq!(blue, 0.0);
}

#[test]
fn directional_illuminance_is_zero_for_invalid_or_dark_colors() {
    assert_eq!(cell_directional_illuminance([0.0, 0.0, 0.0, 1.0]), 0.0);
    assert_eq!(cell_directional_illuminance([f32::NAN, 0.0, 0.0, 1.0]), 0.0);
    assert!(cell_directional_illuminance([0.1, 0.2, 0.3, 1.0]) > 0.0);
}
