use bevyout_core::manifest::exterior::{ExteriorCoordinatePolicy, GridCoordinate, PreparedTerrain};

use super::super::openmw_esm4::LandRecord;

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
        let mut layers = land
            .texture_assignments
            .iter()
            .filter(|assignment| assignment.quadrant == quadrant)
            .map(|assignment| assignment.form_id)
            .collect::<Vec<_>>();
        layers.sort_unstable();
        layers.dedup();
        for (channel, form_id) in layers.into_iter().take(4).enumerate() {
            if land.texture_layers.first().copied() == Some(form_id) {
                weights[channel] = 255;
                break;
            }
            weights[channel] = 192;
        }
        let total = weights.iter().map(|value| u16::from(*value)).sum::<u16>();
        if total > 255 {
            let scale = 255.0 / f32::from(total);
            for value in weights.iter_mut() {
                *value = (f32::from(*value) * scale).round() as u8;
            }
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
        collision_heights,
    }
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
