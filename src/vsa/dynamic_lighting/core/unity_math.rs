const PERMUTATION: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

#[inline]
pub(crate) fn clamp01(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

#[inline]
pub(crate) fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * clamp01(t)
}

#[inline]
pub(crate) fn move_towards(current: f32, target: f32, maximum_delta: f32) -> f32 {
    let delta = target - current;
    if delta.abs() <= maximum_delta {
        target
    } else {
        current + delta.signum() * maximum_delta
    }
}

#[inline]
pub(crate) fn unity_clamp(value: f32, minimum: f32, maximum: f32) -> f32 {
    if value < minimum {
        minimum
    } else if value > maximum {
        maximum
    } else {
        value
    }
}

#[inline]
fn fade(value: f32) -> f32 {
    value * value * value * (value * (value * 6.0 - 15.0) + 10.0)
}

#[inline]
fn gradient(hash: u8, x: f32, y: f32, z: f32) -> f32 {
    let hash = hash & 15;
    let u = if hash < 8 { x } else { y };
    let v = if hash < 4 {
        y
    } else if hash == 12 || hash == 14 {
        x
    } else {
        z
    };
    (if hash & 1 == 0 { u } else { -u }) + if hash & 2 == 0 { v } else { -v }
}

#[inline]
fn permutation(index: i32) -> u8 {
    PERMUTATION[(index & 255) as usize]
}

/// Unity-compatible 2D `Mathf.PerlinNoise`.
///
/// Unity double-hashes each 2D corner through Ken Perlin's fixed permutation
/// table, then applies its historical output normalization. This deliberately
/// produces `0.4652731` rather than `0.5` at integer coordinates.
pub(crate) fn perlin_noise(x: f32, y: f32) -> f32 {
    // Unity mirrors negative coordinates before evaluating the lattice.
    let x = x.abs();
    let y = y.abs();
    let floor_x = x.floor();
    let floor_y = y.floor();
    let xi = floor_x as i32;
    let yi = floor_y as i32;
    let xf = x - floor_x;
    let yf = y - floor_y;
    let u = fade(xf);
    let v = fade(yf);

    let a = i32::from(permutation(xi)) + yi;
    let b = i32::from(permutation(xi + 1)) + yi;
    let aa = permutation(i32::from(permutation(a)));
    let ab = permutation(i32::from(permutation(a + 1)));
    let ba = permutation(i32::from(permutation(b)));
    let bb = permutation(i32::from(permutation(b + 1)));

    let x1 = lerp(
        gradient(aa, xf, yf, 0.0),
        gradient(ba, xf - 1.0, yf, 0.0),
        u,
    );
    let x2 = lerp(
        gradient(ab, xf, yf - 1.0, 0.0),
        gradient(bb, xf - 1.0, yf - 1.0, 0.0),
        u,
    );
    (lerp(x1, x2, v) + 0.69) / 1.483
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perlin_origin_matches_unity() {
        assert!((perlin_noise(0.0, 0.0) - 0.465_273_08).abs() <= 1.0e-7);
    }

    #[test]
    fn perlin_samples_match_unity_6000_3_fixture() {
        #[derive(serde::Deserialize)]
        struct Fixture {
            samples: Vec<Sample>,
        }

        #[derive(serde::Deserialize)]
        struct Sample {
            x: f32,
            y: f32,
            value: f32,
        }

        let fixture: Fixture =
            serde_json::from_str(include_str!("../tests/golden/unity_perlin_v1.json")).unwrap();
        for sample in fixture.samples {
            let actual = perlin_noise(sample.x, sample.y);
            assert!(
                (actual - sample.value).abs() <= 2.0e-5,
                "Perlin({}, {}): Rust {} != Unity {}",
                sample.x,
                sample.y,
                actual,
                sample.value,
            );
        }
    }

    #[test]
    fn move_towards_does_not_overshoot() {
        assert_eq!(move_towards(0.0, 0.25, 1.0), 0.25);
        assert_eq!(move_towards(1.0, 0.0, 0.1), 0.9);
    }
}
