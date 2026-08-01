use super::*;

use bevyout_core::image_space::{ImageSpaceModifierCurveOperation, ImageSpaceModifierProperty};

fn write_f32(data: &mut [u8], offset: usize, value: f32) {
    data[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_u32(data: &mut [u8], offset: usize, value: u32) {
    data[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn time_curve(values: &[(f32, f32)]) -> Vec<u8> {
    let mut data = Vec::with_capacity(values.len() * 8);
    for (time, value) in values {
        data.extend_from_slice(&time.to_le_bytes());
        data.extend_from_slice(&value.to_le_bytes());
    }
    data
}

fn color_curve(values: &[(f32, [f32; 4])]) -> Vec<u8> {
    let mut data = Vec::with_capacity(values.len() * 20);
    for (time, rgba) in values {
        data.extend_from_slice(&time.to_le_bytes());
        for channel in rgba {
            data.extend_from_slice(&channel.to_le_bytes());
        }
    }
    data
}

#[test]
fn parses_imad_dnam_defaults_named_curves_colors_and_sounds() {
    let resolver = direct_resolver();
    let mut dnam = vec![0_u8; 244];
    dnam[0..4].copy_from_slice(&0x0000_0005_u32.to_le_bytes());
    write_f32(&mut dnam, 4, 0.25);
    write_u32(&mut dnam, 144, 2);
    write_u32(&mut dnam, 152, 3);
    write_u32(&mut dnam, 160, 4);
    dnam[176..180].copy_from_slice(&[255, 0, 0, 128]);
    write_u32(&mut dnam, 180, 2);
    write_u32(&mut dnam, 184, 3);
    write_u32(&mut dnam, 188, 4);
    write_u32(&mut dnam, 192, 5);
    write_u32(&mut dnam, 196, 6);
    write_u32(&mut dnam, 200, 7);
    write_f32(&mut dnam, 204, 0.2);
    write_f32(&mut dnam, 208, 0.3);
    write_u32(&mut dnam, 212, 8);
    write_u32(&mut dnam, 216, 9);
    write_u32(&mut dnam, 220, 10);
    write_u32(&mut dnam, 224, 11);
    write_u32(&mut dnam, 228, 12);
    write_u32(&mut dnam, 232, 13);
    dnam[236..240].copy_from_slice(&[0, 0, 0, 64]);
    write_u32(&mut dnam, 240, 14);

    let modifier = parse_image_space_modifier(
        &[
            direct_subrecord("EDID", b"HitBlur\0".to_vec()),
            direct_subrecord("DNAM", dnam),
            direct_subrecord("BNAM", time_curve(&[(0.0, 0.1), (1.0, 0.5)])),
            direct_subrecord(
                "TNAM",
                color_curve(&[(0.0, [1.0, 0.0, 0.0, 0.5]), (1.0, [0.0, 1.0, 0.0, 1.0])]),
            ),
            direct_subrecord("NAM3", color_curve(&[(0.0, [0.0, 0.0, 0.0, 0.8])])),
            direct_subrecord("\x11IAD", time_curve(&[(0.0, 0.5), (1.0, 0.8)])),
            direct_subrecord("QIAD", time_curve(&[(0.0, 0.1), (1.0, 0.2)])),
            direct_subrecord("\x14IAD", time_curve(&[(0.0, 0.6), (1.0, 0.9)])),
            direct_subrecord("TIAD", time_curve(&[(0.0, 0.2), (1.0, 0.3)])),
            direct_subrecord("RDSD", 0x100_u32.to_le_bytes().to_vec()),
            direct_subrecord("RDSI", 0x101_u32.to_le_bytes().to_vec()),
        ],
        0x200,
        &resolver,
    );

    assert_eq!(modifier.form_id, 0x200);
    assert_eq!(modifier.editor_id.as_deref(), Some("HitBlur"));
    assert_eq!(modifier.flags, 5);
    assert_eq!(modifier.duration_ms, 250);
    assert_eq!(modifier.static_values.blur, 2.0);
    assert_eq!(modifier.static_values.double_vision, 3.0);
    assert_eq!(modifier.static_values.radial_blur, 4.0);
    assert_eq!(modifier.static_values.radial_ramp_up, 5.0);
    assert_eq!(modifier.static_values.radial_start, 6.0);
    assert_eq!(modifier.radial_blur_flags, 7);
    assert_eq!(modifier.static_values.depth_of_field_strength, 8.0);
    assert_eq!(modifier.static_values.depth_of_field_distance, 9.0);
    assert_eq!(modifier.static_values.depth_of_field_range, 10.0);
    assert_eq!(modifier.depth_of_field_flags, 11);
    assert_eq!(modifier.static_values.radial_ramp_down, 12.0);
    assert_eq!(modifier.static_values.radial_down_start, 13.0);
    assert_eq!(modifier.static_values.motion_blur, 14.0);
    assert_eq!(modifier.static_values.saturation, 1.0);
    assert_eq!(modifier.static_values.brightness, 1.0);
    assert_eq!(modifier.static_values.contrast, 1.0);
    assert_eq!(
        modifier.static_values.tint_rgba,
        [1.0, 0.0, 0.0, 128.0 / 255.0]
    );
    assert_eq!(modifier.color_keyframes.len(), 2);
    assert_eq!(modifier.fade_color_keyframes.len(), 1);
    assert_eq!(modifier.sound_form_ids, vec![0x100, 0x101]);
    assert!(modifier.curves.iter().any(|curve| {
        curve.property == ImageSpaceModifierProperty::Blur
            && curve.operation == ImageSpaceModifierCurveOperation::Set
            && curve.keyframes[1].time_ms == 250
    }));
    assert!(modifier.curves.iter().any(|curve| {
        curve.property == ImageSpaceModifierProperty::Saturation
            && curve.operation == ImageSpaceModifierCurveOperation::Multiplier
    }));
    assert!(modifier.curves.iter().any(|curve| {
        curve.property == ImageSpaceModifierProperty::Saturation
            && curve.operation == ImageSpaceModifierCurveOperation::Additive
    }));
    assert!(modifier.curves.iter().any(|curve| {
        curve.property == ImageSpaceModifierProperty::Brightness
            && curve.operation == ImageSpaceModifierCurveOperation::Additive
    }));
}

#[test]
fn malformed_optional_imad_data_is_diagnosed_without_panicking() {
    let modifier = parse_image_space_modifier(
        &[
            direct_subrecord("DNAM", vec![1, 2]),
            direct_subrecord("TNAM", vec![1, 2]),
            direct_subrecord("BNAM", vec![0; 3]),
            direct_subrecord("\x04IAD", vec![0; 3]),
            direct_subrecord("\x0fIAD", vec![0; 8]),
        ],
        0x201,
        &direct_resolver(),
    );
    assert_eq!(modifier.form_id, 0x201);
    assert!(
        modifier
            .diagnostics
            .iter()
            .any(|message| message.contains("DNAM"))
    );
    assert!(
        modifier
            .diagnostics
            .iter()
            .any(|message| message.contains("TNAM"))
    );
    assert!(
        modifier
            .diagnostics
            .iter()
            .any(|message| message.contains("trailing") || message.contains("unsupported"))
    );
    assert!(modifier.curves.is_empty());
}
