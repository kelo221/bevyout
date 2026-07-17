use serde::{Deserialize, Serialize};

/// UnityEngine.Random's Xorshift128 stream and InitState seed expansion.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct UnityRandom {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl UnityRandom {
    const MT19937_INITIALIZER: u32 = 1_812_433_253;

    pub(crate) const fn from_seed(seed: i32) -> Self {
        let x = seed as u32;
        let y = Self::MT19937_INITIALIZER.wrapping_mul(x).wrapping_add(1);
        let z = Self::MT19937_INITIALIZER.wrapping_mul(y).wrapping_add(1);
        let w = Self::MT19937_INITIALIZER.wrapping_mul(z).wrapping_add(1);
        Self { x, y, z, w }
    }

    fn next_u32(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);
        self.w
    }

    pub(crate) fn value(&mut self) -> f32 {
        self.next_u32().wrapping_shl(9) as f32 / u32::MAX as f32
    }

    pub(crate) fn range_i32(&mut self, minimum: i32, maximum: i32) -> i32 {
        if minimum == maximum {
            return minimum;
        }
        let minimum = i64::from(minimum);
        let maximum = i64::from(maximum);
        let random = i64::from(self.next_u32());
        if maximum < minimum {
            (minimum - random % (maximum - minimum)) as i32
        } else {
            (minimum + random % (maximum - minimum)) as i32
        }
    }
}

impl Default for UnityRandom {
    fn default() -> Self {
        Self::from_seed(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_12345_matches_unity_6000_3() {
        let mut random = UnityRandom::from_seed(12345);
        let expected = [
            0.586_850_64,
            0.899_560_45,
            0.532_516_36,
            0.915_485_6,
            0.257_733_94,
        ];
        for expected in expected {
            assert!((random.value() - expected).abs() <= 1.0e-7);
        }
    }
}
