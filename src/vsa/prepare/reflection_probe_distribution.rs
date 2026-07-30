/// Allocate a bounded probe budget across disconnected walkable regions.
///
/// Every region receives one probe while budget remains, largest regions first.
/// Remaining probes are distributed toward each region's area-derived target.
/// Equal scores retain source order so preparation stays deterministic.
pub fn allocate_probe_counts(
    region_areas: &[f32],
    spacing_metres: f32,
    max_probes: usize,
) -> Vec<usize> {
    let mut counts = vec![0; region_areas.len()];
    if region_areas.is_empty() || max_probes == 0 {
        return counts;
    }

    let spacing_area = spacing_metres.max(0.1).powi(2);
    let desired: Vec<usize> = region_areas
        .iter()
        .map(|area| (area.max(0.0) / spacing_area).ceil().max(1.0) as usize)
        .collect();
    let mut region_order: Vec<usize> = (0..region_areas.len()).collect();
    region_order.sort_by(|left, right| {
        region_areas[*right]
            .total_cmp(&region_areas[*left])
            .then_with(|| left.cmp(right))
    });

    for region_index in region_order.into_iter().take(max_probes) {
        counts[region_index] = 1;
    }

    while counts.iter().sum::<usize>() < max_probes {
        let candidate = (0..region_areas.len())
            .filter(|index| counts[*index] < desired[*index])
            .max_by(|left, right| {
                let left_score = region_areas[*left].max(0.0) / (counts[*left] + 1) as f32;
                let right_score = region_areas[*right].max(0.0) / (counts[*right] + 1) as f32;
                left_score
                    .total_cmp(&right_score)
                    .then_with(|| right.cmp(left))
            });
        let Some(region_index) = candidate else {
            break;
        };
        counts[region_index] += 1;
    }

    counts
}

#[cfg(test)]
#[allow(dead_code, unused_imports)]
#[path = "tests/reflection_probe_distribution.rs"]
mod tests;
