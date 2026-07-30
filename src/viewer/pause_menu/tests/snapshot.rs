use super::*;

fn solid_image(width: u32, height: u32, format: TextureFormat, color: [u8; 4]) -> Image {
    let data = match format {
        TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
            // Store as BGRA in memory.
            [color[2], color[1], color[0], color[3]]
                .iter()
                .cycle()
                .take((width * height * 4) as usize)
                .copied()
                .collect()
        }
        _ => color
            .iter()
            .cycle()
            .take((width * height * 4) as usize)
            .copied()
            .collect(),
    };
    Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        format,
        RenderAssetUsages::MAIN_WORLD,
    )
}

#[test]
fn full_resolution_blur_preserves_native_dimensions() {
    let source = solid_image(320, 180, TextureFormat::Rgba8UnormSrgb, [10, 20, 30, 255]);
    let blurred = process_snapshot_blur(&source).expect("process snapshot blur");
    assert_eq!(blurred.width(), 320);
    assert_eq!(blurred.height(), 180);
}

#[test]
fn full_resolution_blur_accepts_bgra_window_format() {
    let source = solid_image(64, 32, TextureFormat::Bgra8UnormSrgb, [10, 20, 30, 255]);
    let blurred = process_snapshot_blur(&source).expect("bgra process snapshot blur");
    assert_eq!(blurred.width(), 64);
    assert_eq!(blurred.height(), 32);
    // First pixel should be near the sepia-graded source RGB (not swapped).
    let data = blurred.data.as_ref().expect("cpu bytes");
    assert!(data[0] > 5, "red channel present: {:?}", &data[..4]);
    assert!(data[1] > 10, "green channel present: {:?}", &data[..4]);
}
