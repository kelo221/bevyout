//! Pure irradiance-volume layout policy shared by the baker and feature tests.

pub(crate) const AMBIENT_CUBE_FACE_COUNT: usize = 6;

pub(crate) fn volume_resolution(scale: [f32; 3], spacing: f32) -> [u32; 3] {
    scale.map(|extent| ((extent / spacing).ceil() as u32 + 1).max(2))
}

pub(crate) fn atlas_dimensions(resolution: [u32; 3]) -> [u32; 3] {
    [resolution[0], 2 * resolution[1], 3 * resolution[2]]
}

pub(crate) fn primary_ray_count(resolution: [u32; 3], samples: u32) -> usize {
    resolution.iter().product::<u32>() as usize * AMBIENT_CUBE_FACE_COUNT * samples as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_bevy_xyz_order_in_volume_and_atlas() {
        let resolution = volume_resolution([94.528_31, 31.064_644, 87.114_3], 8.0);
        assert_eq!(resolution, [13, 5, 12]);
        assert_eq!(atlas_dimensions(resolution), [13, 10, 36]);
        assert_eq!(primary_ray_count(resolution, 64), 299_520);
    }
}
