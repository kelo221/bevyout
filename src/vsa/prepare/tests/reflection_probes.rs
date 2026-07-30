use super::*;

fn placement(reference_form_id: u32, asset_path: &str) -> JobPlacement {
    JobPlacement {
        reference_form_id,
        asset_path: asset_path.into(),
        ao_mode: "ao-none".into(),
        batchable_static: true,
        translation: [0.0; 3],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
    }
}

#[test]
fn animated_static_assets_are_excluded_once_per_asset() {
    let mut inspected = Vec::new();
    let retained = exclude_animated_static_placements(
        vec![
            placement(0x10, "assets/animated.glb"),
            placement(0x11, "assets/animated.glb"),
            placement(0x20, "assets/static.glb"),
        ],
        |asset_path| {
            inspected.push(asset_path.to_owned());
            Ok(asset_path.contains("animated"))
        },
    )
    .unwrap();

    assert_eq!(inspected, ["assets/animated.glb", "assets/static.glb"]);
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].reference_form_id, 0x20);
}

#[test]
fn a_64px_probe_has_seven_specular_levels() {
    let faces = vec![vec![pack_rgb9e5([1.0, 0.5, 0.25]); 64 * 64]; 6];
    let levels = box_filter_mip_chain(&faces, 64);
    assert_eq!(levels.len(), 7);
    assert_eq!(levels.last().unwrap()[0].len(), 1);
}

#[test]
fn rgb9e5_round_trip_keeps_hdr_channels() {
    let value = [1.0, 2.0, 4.0];
    let decoded = unpack_rgb9e5(pack_rgb9e5(value));
    for axis in 0..3 {
        assert!((decoded[axis] - value[axis]).abs() < 0.02);
    }
}
