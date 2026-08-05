//! Pure feature-guided A-Trous denoising for charted RGB lightmap texels.
//!
//! Chart ownership is an absolute barrier. Position, normal, material,
//! relative variance, and luminance are soft feature guides applied to a
//! fixed five-tap A-Trous kernel, keeping each pass deterministic.

const KERNEL: [f32; 5] = [0.0625, 0.25, 0.375, 0.25, 0.0625];
const POSITION_SCALE: f32 = 0.5;
const NORMAL_SCALE: f32 = 0.25;
const VARIANCE_SCALE: f32 = 0.25;
const COVERAGE_SCALE: f32 = 0.25;
const LUMINANCE_SCALE: f32 = 0.25;
const DIFFERENT_MATERIAL_WEIGHT: f32 = 0.05;
const FEATURE_EPSILON: f32 = 1.0e-6;

/// Per-texel features used to preserve geometric and material edges.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct DenoiseFeature {
    pub(crate) position: [f32; 3],
    pub(crate) normal: [f32; 3],
    pub(crate) material_id: u32,
    pub(crate) relative_variance: f32,
    pub(crate) coverage: f32,
    pub(crate) sample_count: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DenoiseError {
    DimensionsOverflow,
    BufferLengthMismatch,
}

/// Denoises charted RGB texels in place.
///
/// `chart_owners` must contain `Some(chart_id)` for valid texels. A `None`
/// owner is preserved and excluded from every neighborhood. The three input
/// slices must all have `width * height` elements. Zero iterations validates
/// the inputs and leaves `pixels` unchanged.
pub(crate) fn denoise(
    pixels: &mut [[f32; 3]],
    chart_owners: &[Option<u32>],
    features: &[DenoiseFeature],
    width: usize,
    height: usize,
    iterations: u32,
) -> Result<(), DenoiseError> {
    let expected = width
        .checked_mul(height)
        .ok_or(DenoiseError::DimensionsOverflow)?;
    if pixels.len() != expected || chart_owners.len() != expected || features.len() != expected {
        return Err(DenoiseError::BufferLengthMismatch);
    }

    if iterations == 0 || expected == 0 {
        return Ok(());
    }

    let mut source = pixels.to_vec();
    let mut destination = vec![[0.0; 3]; expected];
    for iteration in 0..iterations {
        let Some(step) = 1usize.checked_shl(iteration) else {
            break;
        };
        denoise_pass(
            &source,
            &mut destination,
            chart_owners,
            features,
            width,
            height,
            step,
        );
        std::mem::swap(&mut source, &mut destination);
    }
    pixels.copy_from_slice(&source);
    Ok(())
}

fn denoise_pass(
    source: &[[f32; 3]],
    destination: &mut [[f32; 3]],
    chart_owners: &[Option<u32>],
    features: &[DenoiseFeature],
    width: usize,
    height: usize,
    step: usize,
) {
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let Some(chart_owner) = chart_owners[index] else {
                destination[index] = source[index];
                continue;
            };
            let center_feature = features[index];
            let center_color = source[index];
            if !center_feature.is_finite() || !rgb_is_finite(center_color) {
                destination[index] = center_color;
                continue;
            }

            let mut weighted_color = [0.0; 3];
            let mut total_weight = 0.0;
            for (kernel_y, offset_y) in (-2_i32..=2).enumerate() {
                let Some(neighbor_y) = offset_coordinate(y, offset_y, height, step) else {
                    continue;
                };
                for (kernel_x, offset_x) in (-2_i32..=2).enumerate() {
                    let Some(neighbor_x) = offset_coordinate(x, offset_x, width, step) else {
                        continue;
                    };
                    let neighbor_index = neighbor_y * width + neighbor_x;
                    if chart_owners[neighbor_index] != Some(chart_owner) {
                        continue;
                    }
                    let neighbor_color = source[neighbor_index];
                    let neighbor_feature = features[neighbor_index];
                    if !neighbor_feature.is_finite() || !rgb_is_finite(neighbor_color) {
                        continue;
                    }

                    let kernel_weight = KERNEL[kernel_x] * KERNEL[kernel_y];
                    let feature_weight = feature_weight(
                        center_feature,
                        neighbor_feature,
                        center_color,
                        neighbor_color,
                    );
                    let weight = kernel_weight * feature_weight;
                    for channel in 0..3 {
                        weighted_color[channel] += neighbor_color[channel] * weight;
                    }
                    total_weight += weight;
                }
            }

            destination[index] = if total_weight > FEATURE_EPSILON {
                weighted_color.map(|value| value / total_weight)
            } else {
                center_color
            };
        }
    }
}

fn offset_coordinate(coordinate: usize, offset: i32, limit: usize, step: usize) -> Option<usize> {
    let distance = step.checked_mul(offset.unsigned_abs() as usize)?;
    let result = if offset < 0 {
        coordinate.checked_sub(distance)?
    } else {
        coordinate.checked_add(distance)?
    };
    (result < limit).then_some(result)
}

fn feature_weight(
    center: DenoiseFeature,
    neighbor: DenoiseFeature,
    center_color: [f32; 3],
    neighbor_color: [f32; 3],
) -> f32 {
    let position_distance = squared_distance(center.position, neighbor.position);
    let normal_difference = 1.0 - normal_dot(center.normal, neighbor.normal);
    let variance_difference = (center.relative_variance - neighbor.relative_variance).abs();
    let coverage_difference = (center.coverage - neighbor.coverage).abs();
    let luminance_difference = (luminance(center_color) - luminance(neighbor_color)).abs();

    let position_weight = reciprocal_feature_weight(position_distance, POSITION_SCALE);
    let normal_weight = reciprocal_feature_weight(normal_difference, NORMAL_SCALE);
    let variance_weight = reciprocal_feature_weight(variance_difference, VARIANCE_SCALE);
    let coverage_weight = reciprocal_feature_weight(coverage_difference, COVERAGE_SCALE);
    let luminance_weight = reciprocal_feature_weight(luminance_difference, LUMINANCE_SCALE);
    let material_weight = if center.material_id == neighbor.material_id {
        1.0
    } else {
        DIFFERENT_MATERIAL_WEIGHT
    };
    (position_weight
        * normal_weight
        * variance_weight
        * coverage_weight
        * luminance_weight
        * material_weight)
        .clamp(0.0, 1.0)
}

fn reciprocal_feature_weight(distance: f32, scale: f32) -> f32 {
    if !distance.is_finite() || !scale.is_finite() || scale <= FEATURE_EPSILON {
        return 0.0;
    }
    (1.0 + distance / (scale * scale)).recip()
}

fn squared_distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    let difference = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    difference[0] * difference[0] + difference[1] * difference[1] + difference[2] * difference[2]
}

fn normal_dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    let a_length = squared_length(a).sqrt();
    let b_length = squared_length(b).sqrt();
    if a_length <= FEATURE_EPSILON || b_length <= FEATURE_EPSILON {
        return 0.0;
    }
    ((a[0] * b[0] + a[1] * b[1] + a[2] * b[2]) / (a_length * b_length)).clamp(-1.0, 1.0)
}

fn squared_length(value: [f32; 3]) -> f32 {
    value[0] * value[0] + value[1] * value[1] + value[2] * value[2]
}

fn luminance(rgb: [f32; 3]) -> f32 {
    rgb[0] * 0.2126 + rgb[1] * 0.7152 + rgb[2] * 0.0722
}

fn rgb_is_finite(rgb: [f32; 3]) -> bool {
    rgb.into_iter().all(f32::is_finite)
}

impl DenoiseFeature {
    fn is_finite(self) -> bool {
        self.position.into_iter().all(f32::is_finite)
            && self.normal.into_iter().all(f32::is_finite)
            && self.relative_variance.is_finite()
            && self.coverage.is_finite()
    }
}
