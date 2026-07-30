use super::*;

#[test]
fn keeps_bevy_xyz_order_in_volume_and_atlas() {
    let resolution = volume_resolution([94.528_31, 31.064_644, 87.114_3], 8.0);
    assert_eq!(resolution, [13, 5, 12]);
    assert_eq!(atlas_dimensions(resolution), [13, 10, 36]);
    assert_eq!(primary_ray_count(resolution, 64), 299_520);
}
