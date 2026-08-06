const INTEGRATOR_REVISION: u64 = 3;

/// Derive a stable sample seed from scene identity, spatial index, sample
/// index, and the transport revision. It is independent of Rayon scheduling.
pub(crate) fn sample_seed(scene_seed: u64, spatial_index: usize, sample_index: u32) -> u32 {
    let mut value = scene_seed ^ INTEGRATOR_REVISION;
    value ^= spatial_index as u64;
    value = value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    value ^= u64::from(sample_index).wrapping_mul(0xd1b5_4a32_d192_ed03);
    hash_u32((value ^ (value >> 32)) as u32)
}

/// Returns a deterministic Owen-free uniform variate for a transport path.
/// The half-open interval keeps roulette decisions away from an exact zero
/// while remaining independent of Rayon scheduling.
pub(crate) fn sample_uniform_1d(scene_seed: u64, spatial_index: usize, sample_index: u32) -> f32 {
    let seed = f64::from(sample_seed(scene_seed, spatial_index, sample_index));
    let value = ((seed + 0.5) / 4_294_967_296.0) as f32;
    value.min(f32::from_bits(1.0_f32.to_bits() - 1))
}

pub(crate) fn seed_from_fingerprint(fingerprint: &str) -> u64 {
    // FNV-1a is small, deterministic, and sufficient here because the seed
    // is not a security boundary or a cache identity.
    fingerprint
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x1000_0000_01b3)
        })
}

pub(crate) fn cosine_hemisphere_direction(
    normal: bevy::math::Vec3,
    seed: u32,
    sample: u32,
    count: u32,
) -> bevy::math::Vec3 {
    // Stratify the radial dimension but jitter it for every spatial/sample
    // identity. This keeps multi-sample convergence while avoiding the
    // single-sample fixed-polar ring produced by a constant 0.5 offset.
    let radial_jitter = (hash_u32(seed ^ 0x4f1b_2d39) as f32 + 0.5) * (1.0 / 4_294_967_296.0);
    let u = (sample as f32 + radial_jitter) / count.max(1) as f32;
    let scramble = hash_u32(seed);
    let v = radical_inverse(sample ^ scramble);
    let radius = u.sqrt();
    let angle = std::f32::consts::TAU * v;
    let local = bevy::math::Vec3::new(radius * angle.cos(), radius * angle.sin(), (1.0 - u).sqrt());
    let tangent = if normal.z.abs() < 0.999 {
        normal.cross(bevy::math::Vec3::Z).normalize()
    } else {
        normal.cross(bevy::math::Vec3::Y).normalize()
    };
    let bitangent = normal.cross(tangent);
    (tangent * local.x + bitangent * local.y + normal * local.z).normalize()
}

fn radical_inverse(bits: u32) -> f32 {
    bits.reverse_bits() as f32 * 2.328_306_4e-10
}

fn hash_u32(mut value: u32) -> u32 {
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^ (value >> 16)
}
