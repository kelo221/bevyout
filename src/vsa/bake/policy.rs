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
#[path = "tests/policy.rs"]
mod tests;
