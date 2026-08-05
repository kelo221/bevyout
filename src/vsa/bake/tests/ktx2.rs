use super::super::ktx2::write_rgba16f;
use std::fs;

#[test]
fn writes_valid_npot_single_level_rgba16f_ktx2() {
    let root = std::env::temp_dir().join(format!("bevyout-ktx2-test-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let raw = root.join("image.raw");
    let output = root.join("image.ktx2");
    let width = 3;
    let height = 5;
    let data = (0..(width * height * 8))
        .map(|value| value as u8)
        .collect::<Vec<_>>();
    fs::write(&raw, &data).unwrap();

    write_rgba16f(&raw, &output, width, height).unwrap();
    let encoded = fs::read(&output).unwrap();
    let reader = ::ktx2::Reader::new(&encoded).unwrap();
    let header = reader.header();
    assert_eq!(header.format, Some(::ktx2::Format::R16G16B16A16_SFLOAT));
    assert_eq!(header.pixel_width, width);
    assert_eq!(header.pixel_height, height);
    assert_eq!(header.level_count, 1);
    assert!(header.supercompression_scheme.is_none());
    assert_eq!(reader.levels().next().unwrap().data, data.as_slice());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn writes_valid_rgb9e5_volume_from_raw_slices() {
    let root =
        std::env::temp_dir().join(format!("bevyout-ktx2-volume-test-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let paths = (0..2)
        .map(|slice| {
            let path = root.join(format!("slice-{slice}.raw"));
            fs::write(&path, vec![slice as u8; 3 * 4 * 4]).unwrap();
            path
        })
        .collect::<Vec<_>>();
    let output = root.join("volume.ktx2");

    super::super::ktx2::write_rgb9e5_volume(&paths, &output, 3, 4, 2).unwrap();
    let encoded = fs::read(&output).unwrap();
    let reader = ::ktx2::Reader::new(&encoded).unwrap();
    let header = reader.header();
    assert_eq!(header.format, Some(::ktx2::Format::E5B9G9R9_UFLOAT_PACK32));
    assert_eq!(header.pixel_width, 3);
    assert_eq!(header.pixel_height, 4);
    assert_eq!(header.pixel_depth, 2);
    assert_eq!(reader.levels().next().unwrap().data.len(), 2 * 3 * 4 * 4);

    fs::remove_dir_all(root).unwrap();
}
