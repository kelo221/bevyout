use anyhow::{Context, Result, bail};
use blend::Blend;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const SH_L0_M0: f32 = 0.282_094_8;
const SH_L1: f32 = 0.488_602_52;

const BEVY_DIRECTIONS_IN_BLENDER: [[f32; 3]; 6] = [
    [1.0, 0.0, 0.0],  // +X
    [0.0, 0.0, 1.0],  // +Y
    [0.0, -1.0, 0.0], // +Z
    [-1.0, 0.0, 0.0], // -X
    [0.0, 0.0, -1.0], // -Y
    [0.0, 1.0, 0.0],  // -Z
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IrradianceVolumeDimensions {
    pub(crate) resolution: [u32; 3],
    pub(crate) raw_slices: Vec<PathBuf>,
}

/// Extract Blender 4.5's baked L0/L1 cache and write Bevy's RGB9E5 ambient
/// cube atlas as little-endian packed words for `ktx create --raw`.
pub(crate) fn export_rgb9e5_atlas(
    blend_path: &Path,
    raw_path: &Path,
    expected_resolution: [u32; 3],
) -> Result<IrradianceVolumeDimensions> {
    let bytes = fs::read(blend_path)
        .with_context(|| format!("could not read Blender cache {}", blend_path.display()))?;
    let blend = Blend::new(&bytes[..]).map_err(|error| {
        anyhow::anyhow!(
            "could not parse Blender cache {}; ensure Blender saved it uncompressed: {error:?}",
            blend_path.display()
        )
    })?;

    for object in blend.instances_with_code(*b"OB") {
        let name = object.get("id").get_string("name");
        if !name.contains("BevyOutIrradianceVolume") || !object.is_valid("lightprobe_cache") {
            continue;
        }
        let cache = object.get("lightprobe_cache");
        if !cache.is_valid("grid_static_cache") {
            continue;
        }
        let grid = cache.get("grid_static_cache");
        let size = grid.get_i32_vec("size");
        if size.len() != 3 || size.iter().any(|value| *value <= 0) {
            bail!("Blender irradiance cache has invalid resolution: {size:?}");
        }
        let resolution = [size[0] as u32, size[1] as u32, size[2] as u32];
        if resolution != expected_resolution {
            bail!(
                "Blender irradiance cache resolution {:?} differs from requested {:?}",
                resolution,
                expected_resolution
            );
        }
        let layout = grid.get_i32("data_layout");
        if layout != 0 {
            bail!(
                "Blender irradiance cache uses unsupported adaptive layout {layout}; use Blender 4.5 uniform volume baking"
            );
        }

        let irradiance = grid.get("irradiance");
        let l0 = irradiance.get_f32_vec("L0");
        let l1_a = irradiance.get_f32_vec("L1_a");
        let l1_b = irradiance.get_f32_vec("L1_b");
        let l1_c = irradiance.get_f32_vec("L1_c");
        let sample_count = resolution.iter().product::<u32>() as usize;
        let expected_coefficients = sample_count * 3;
        for (label, values) in [
            ("L0", &l0),
            ("L1_a", &l1_a),
            ("L1_b", &l1_b),
            ("L1_c", &l1_c),
        ] {
            if values.len() != expected_coefficients {
                bail!(
                    "Blender irradiance cache {label} has {} values; expected {expected_coefficients}",
                    values.len()
                );
            }
        }

        let raw_slices = write_atlas(raw_path, resolution, &l0, &l1_a, &l1_b, &l1_c)?;
        return Ok(IrradianceVolumeDimensions {
            resolution,
            raw_slices,
        });
    }

    bail!("Blender cache contains no baked BevyOutIrradianceVolume irradiance data")
}

fn write_atlas(
    raw_path: &Path,
    resolution: [u32; 3],
    l0: &[f32],
    l1_a: &[f32],
    l1_b: &[f32],
    l1_c: &[f32],
) -> Result<Vec<PathBuf>> {
    let [rx, ry, rz] = resolution;
    let width = rx as usize;
    let height = (2 * ry) as usize;
    let depth = (3 * rz) as usize;
    let mut words = vec![vec![0_u32; width * height]; depth];

    for z in 0..rz as usize {
        for y in 0..ry as usize {
            for x in 0..rx as usize {
                let sample = (z * ry as usize * rx as usize + y * rx as usize + x) * 3;
                for (side, direction) in BEVY_DIRECTIONS_IN_BLENDER.iter().enumerate() {
                    let color = evaluate_lambertian(
                        [l0[sample], l0[sample + 1], l0[sample + 2]],
                        [l1_a[sample], l1_a[sample + 1], l1_a[sample + 2]],
                        [l1_b[sample], l1_b[sample + 1], l1_b[sample + 2]],
                        [l1_c[sample], l1_c[sample + 1], l1_c[sample + 2]],
                        *direction,
                    );
                    let (side_y, side_z) = match side {
                        0 => (y + ry as usize, z),
                        1 => (y + ry as usize, z + rz as usize),
                        2 => (y + ry as usize, z + 2 * rz as usize),
                        3 => (y, z),
                        4 => (y, z + rz as usize),
                        5 => (y, z + 2 * rz as usize),
                        _ => unreachable!(),
                    };
                    let index = side_y * width + x;
                    words[side_z][index] = pack_rgb9e5(color);
                }
            }
        }
    }

    let stem = raw_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("irradiance");
    let extension = raw_path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("raw");
    let mut raw_slices = Vec::with_capacity(depth);
    for (slice, words) in words.into_iter().enumerate() {
        let path = if depth == 1 {
            raw_path.to_path_buf()
        } else {
            raw_path.with_file_name(format!("{stem}_{slice:04}.{extension}"))
        };
        let mut output = fs::File::create(&path)
            .with_context(|| format!("could not create {}", path.display()))?;
        for word in words {
            output.write_all(&word.to_le_bytes())?;
        }
        raw_slices.push(path);
    }
    Ok(raw_slices)
}

fn evaluate_lambertian(
    l0: [f32; 3],
    l1_a: [f32; 3],
    l1_b: [f32; 3],
    l1_c: [f32; 3],
    direction: [f32; 3],
) -> [f32; 3] {
    std::array::from_fn(|channel| {
        let value = SH_L0_M0 * l0[channel]
            + (2.0 / 3.0)
                * (-SH_L1 * direction[1] * l1_a[channel] + SH_L1 * direction[2] * l1_b[channel]
                    - SH_L1 * direction[0] * l1_c[channel]);
        value.max(0.0)
    })
}

fn pack_rgb9e5(color: [f32; 3]) -> u32 {
    let color = color.map(|value| value.clamp(0.0, 65_408.0));
    let maximum = color.iter().copied().fold(0.0_f32, f32::max);
    if maximum <= f32::MIN_POSITIVE {
        return 0;
    }

    let mut exponent = (maximum.log2().floor() as i32 + 16).clamp(1, 31);
    let mut scale = 2.0_f32.powi(9 - (exponent - 15));
    let mut mantissas = color.map(|value| (value * scale).round() as u32);
    if mantissas.iter().any(|value| *value > 511) && exponent < 31 {
        exponent += 1;
        scale = 2.0_f32.powi(9 - (exponent - 15));
        mantissas = color.map(|value| (value * scale).round().min(511.0) as u32);
    }
    mantissas[0] | (mantissas[1] << 9) | (mantissas[2] << 18) | ((exponent as u32) << 27)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_l0_produces_equal_ambient_cube_sides() {
        let color = evaluate_lambertian(
            [1.0, 0.5, 0.25],
            [0.0; 3],
            [0.0; 3],
            [0.0; 3],
            [1.0, 0.0, 0.0],
        );
        assert_eq!(color, [SH_L0_M0, SH_L0_M0 * 0.5, SH_L0_M0 * 0.25]);
    }

    #[test]
    fn rgb9e5_zero_is_zero_and_positive_values_have_an_exponent() {
        assert_eq!(pack_rgb9e5([0.0; 3]), 0);
        assert_ne!(pack_rgb9e5([1.0, 0.5, 0.25]), 0);
    }

    #[test]
    fn atlas_writes_one_raw_file_per_3d_slice_for_ktx() {
        let directory =
            std::env::temp_dir().join(format!("bevyout-irradiance-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).expect("create test directory");
        let raw_path = directory.join("irradiance.raw");
        let slices = write_atlas(
            &raw_path,
            [1, 1, 1],
            &[1.0, 1.0, 1.0],
            &[0.0; 3],
            &[0.0; 3],
            &[0.0; 3],
        )
        .expect("write test slices");
        assert_eq!(slices.len(), 3);
        assert!(
            slices
                .iter()
                .all(|path| { std::fs::metadata(path).expect("slice exists").len() == 8 })
        );
        std::fs::remove_dir_all(directory).expect("remove test directory");
    }
}
