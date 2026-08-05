//! Deterministic adaptive sampling policy for CPU lightmap transport.
//!
//! The estimator keeps a per-channel running mean and Welford's centered
//! second-moment sum (`M2`). Keeping the centered sum instead of subtracting
//! two large raw moments makes the variance estimate stable for bright
//! samples with small differences between them.

const MIN_VARIANCE_SAMPLES: u32 = 2;
const MEAN_EPSILON: f64 = 1.0e-6;

/// Running RGB irradiance estimator for one lightmap sample location.
///
/// The RGB vector is represented as `[f32; 3]` so this module stays pure and
/// Bevy-independent. A Bevy caller can pass `Vec3::to_array()` to [`add`]
/// and reconstruct the result with `Vec3::from_array` from [`mean`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct AdaptiveEstimator {
    min_samples: u32,
    max_samples: u32,
    variance_threshold: f64,
    sample_count: u32,
    mean: [f64; 3],
    m2: [f64; 3],
}

impl AdaptiveEstimator {
    pub(crate) const fn new(min_samples: u32, max_samples: u32, variance_threshold: f32) -> Self {
        Self {
            min_samples,
            max_samples,
            variance_threshold: variance_threshold as f64,
            sample_count: 0,
            mean: [0.0; 3],
            m2: [0.0; 3],
        }
    }

    /// Adds one irradiance sample using Welford's online update.
    pub(crate) fn add(&mut self, irradiance: [f32; 3]) {
        let sample_count = self
            .sample_count
            .checked_add(1)
            .expect("adaptive irradiance sample count overflowed");
        let count = f64::from(sample_count);

        for (channel, sample) in irradiance.into_iter().enumerate() {
            let sample = f64::from(sample);
            let delta = sample - self.mean[channel];
            self.mean[channel] += delta / count;
            let delta_after_mean = sample - self.mean[channel];
            self.m2[channel] += delta * delta_after_mean;
        }
        self.sample_count = sample_count;
    }

    pub(crate) fn mean(&self) -> [f32; 3] {
        self.mean.map(|value| value as f32)
    }

    pub(crate) fn sample_count(&self) -> u32 {
        self.sample_count
    }

    /// Returns the largest per-channel relative variance of the estimated
    /// mean. Before two samples or for non-finite input, no stable estimate
    /// exists and this returns positive infinity.
    pub(crate) fn relative_variance(&self) -> f32 {
        if !self.is_finite() {
            return f32::INFINITY;
        }
        relative_variance_of_mean(self.sample_count, self.mean, self.variance())
            .map_or(f32::INFINITY, |value| value as f32)
    }

    /// Returns the unbiased per-channel sample variance.
    pub(crate) fn variance(&self) -> [f64; 3] {
        if self.sample_count < MIN_VARIANCE_SAMPLES {
            return [0.0; 3];
        }
        let divisor = f64::from(self.sample_count - 1);
        self.m2.map(|value| value.max(0.0) / divisor)
    }

    /// Returns the raw second moment E[x²], derived from the stable centered
    /// accumulator rather than used to calculate the variance.
    #[cfg(test)]
    pub(crate) fn second_moment(&self) -> [f64; 3] {
        if self.sample_count == 0 {
            return [0.0; 3];
        }
        let count = f64::from(self.sample_count);
        std::array::from_fn(|channel| {
            self.mean[channel] * self.mean[channel] + self.m2[channel] / count
        })
    }

    /// Returns true when the estimate is converged or the hard sample cap is
    /// reached. The cap wins when `min_samples > max_samples`, so the policy
    /// always has a deterministic upper bound.
    pub(crate) fn should_stop(&self) -> bool {
        if self.sample_count >= self.max_samples {
            return true;
        }
        if self.sample_count < self.min_samples.max(MIN_VARIANCE_SAMPLES)
            || !self.is_finite()
            || self.variance_threshold.is_nan()
            || self.variance_threshold < 0.0
        {
            return false;
        }

        relative_variance_of_mean(self.sample_count, self.mean, self.variance())
            .is_some_and(|relative_variance| relative_variance <= self.variance_threshold)
    }

    fn is_finite(&self) -> bool {
        self.mean.into_iter().all(f64::is_finite) && self.m2.into_iter().all(f64::is_finite)
    }
}

fn relative_variance_of_mean(sample_count: u32, mean: [f64; 3], variance: [f64; 3]) -> Option<f64> {
    if sample_count < MIN_VARIANCE_SAMPLES {
        return None;
    }
    let count = f64::from(sample_count);
    let mut largest: f64 = 0.0;
    for channel in 0..3 {
        let mean = mean[channel];
        let variance = variance[channel];
        if !mean.is_finite() || !variance.is_finite() || variance < 0.0 {
            return None;
        }
        let scale = mean.abs().max(MEAN_EPSILON);
        let relative_variance = variance / (count * scale * scale);
        if !relative_variance.is_finite() {
            return None;
        }
        largest = largest.max(relative_variance);
    }
    Some(largest)
}
