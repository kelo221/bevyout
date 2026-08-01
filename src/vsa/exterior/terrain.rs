use bevyout_core::manifest::exterior::{ExteriorCoordinatePolicy, GridCoordinate, PreparedTerrain};

use super::super::openmw_esm4::{LandRecord, LandTextureAssignment};

pub(crate) fn terrain_from_land(land: &LandRecord, grid: GridCoordinate) -> PreparedTerrain {
    let policy = ExteriorCoordinatePolicy::default();
    let count = LandRecord::GRID_SIZE * LandRecord::GRID_SIZE;
    let origin = policy.grid_origin(grid);
    let spacing = policy.cell_span_metres() / 32.0;
    let mut positions = Vec::with_capacity(count);
    let mut normals = Vec::with_capacity(count);
    let mut colors = Vec::with_capacity(count);
    let mut collision_heights = Vec::with_capacity(count);
    for index in 0..count {
        let x = index % LandRecord::GRID_SIZE;
        let y = index / LandRecord::GRID_SIZE;
        let height = land.heights.get(index).copied().unwrap_or_default() / 70.0;
        positions.push([
            (origin[0] + f64::from(x as u32) * spacing) as f32,
            (origin[1] + f64::from(height)) as f32,
            (origin[2] - f64::from(y as u32) * spacing) as f32,
        ]);
        collision_heights.push(positions[index][1]);
        let normal = land
            .normals
            .get(index)
            .copied()
            .map(|normal| {
                normalize([
                    f32::from(normal[0]) / 127.0,
                    f32::from(normal[2]) / 127.0,
                    -f32::from(normal[1]) / 127.0,
                ])
            })
            .unwrap_or_else(|| derived_normal(&land.heights, x, y));
        normals.push(normal);
        let color = land.colors.get(index).copied().unwrap_or([255; 3]);
        colors.push([color[0], color[1], color[2], 255]);
    }
    let mut blend_weights = vec![[255, 0, 0, 0]; count];
    for (index, weights) in blend_weights.iter_mut().enumerate() {
        let x = index % LandRecord::GRID_SIZE;
        let y = index / LandRecord::GRID_SIZE;
        let quadrant = (x / 16).min(1) as u8 + ((y / 16).min(1) as u8 * 2);
        let local_x = (x % 16).min(16) as u8;
        let local_y = (y % 16).min(16) as u8;
        let mut values = [0.0_f32; 4];
        let mut base_channel = None;
        for (channel, form_id) in land.texture_layers.iter().take(4).enumerate() {
            let assignments = land.texture_assignments.iter().filter(|assignment| {
                assignment.quadrant == quadrant && assignment.form_id == *form_id
            });
            for assignment in assignments {
                if assignment.base {
                    base_channel = Some(channel);
                    values[channel] = 1.0;
                } else {
                    values[channel] =
                        values[channel].max(sample_assignment_weight(assignment, local_x, local_y));
                }
            }
        }
        let overlay_total = values
            .iter()
            .enumerate()
            .filter(|(channel, _)| Some(*channel) != base_channel)
            .map(|(_, value)| *value)
            .sum::<f32>();
        if let Some(base_channel) = base_channel {
            if overlay_total > 1.0 {
                for (channel, value) in values.iter_mut().enumerate() {
                    if channel != base_channel {
                        *value /= overlay_total;
                    }
                }
                values[base_channel] = 0.0;
            } else {
                values[base_channel] = 1.0 - overlay_total;
            }
        } else if overlay_total <= f32::EPSILON {
            values[0] = 1.0;
        } else {
            for value in &mut values {
                *value /= overlay_total;
            }
        }
        for (channel, value) in values.into_iter().enumerate() {
            weights[channel] = (value.clamp(0.0, 1.0) * 255.0).round() as u8;
        }
        // Keep the quantized weights normalized. Rounding four channels can
        // otherwise leave a one-byte seam between independently prepared
        // cells.
        let total = weights.iter().map(|value| u16::from(*value)).sum::<u16>();
        if total != 255 {
            let correction = 255_i32 - i32::from(total);
            let first = i32::from(weights[0]) + correction;
            weights[0] = first.clamp(0, 255) as u8;
        }
    }
    PreparedTerrain {
        width: LandRecord::GRID_SIZE as u16,
        height: LandRecord::GRID_SIZE as u16,
        positions,
        normals,
        colors,
        blend_weights,
        texture_layers: land.texture_layers.clone(),
        albedo_asset_path: None,
        normal_asset_path: None,
        collision_heights,
    }
}

fn sample_assignment_weight(assignment: &LandTextureAssignment, local_x: u8, local_y: u8) -> f32 {
    if assignment.weights.is_empty() {
        return 0.0;
    }
    let mut samples = [0.0_f32; 17 * 17];
    for weight in &assignment.weights {
        let index = usize::from(weight.position);
        if index < samples.len() {
            samples[index] = weight.opacity.clamp(0.0, 1.0);
        }
    }
    let x = usize::from(local_x.min(16));
    let y = usize::from(local_y.min(16));
    samples[y * 17 + x]
}

fn derived_normal(heights: &[f32], x: usize, y: usize) -> [f32; 3] {
    let at = |x: usize, y: usize| {
        heights
            .get(y * LandRecord::GRID_SIZE + x)
            .copied()
            .unwrap_or_default()
    };
    let left = at(x.saturating_sub(1), y);
    let right = at((x + 1).min(LandRecord::GRID_SIZE - 1), y);
    let down = at(x, y.saturating_sub(1));
    let up = at(x, (y + 1).min(LandRecord::GRID_SIZE - 1));
    normalize([(left - right) / 70.0, 16.0, (down - up) / 70.0])
}

fn normalize(value: [f32; 3]) -> [f32; 3] {
    let length = (value[0] * value[0] + value[1] * value[1] + value[2] * value[2]).sqrt();
    if length.is_finite() && length > f32::EPSILON {
        [value[0] / length, value[1] / length, value[2] / length]
    } else {
        [0.0, 1.0, 0.0]
    }
}
