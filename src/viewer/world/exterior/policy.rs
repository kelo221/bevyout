//! Bevy-free adapters for the exterior residency contract.

use bevyout_core::manifest::exterior::{
    ExteriorCellState, ExteriorCoordinatePolicy, ExteriorResidencyInput, ExteriorResidencyPlan,
    ExteriorWorldspaceIndex, GridCoordinate, plan_residency,
};

pub(crate) fn index_cells(
    index: &ExteriorWorldspaceIndex,
) -> std::collections::BTreeMap<GridCoordinate, u32> {
    let mut cells = std::collections::BTreeMap::new();
    for cell in &index.cells {
        cells
            .entry(cell.grid)
            .and_modify(|cell_form_id: &mut u32| {
                *cell_form_id = (*cell_form_id).min(cell.cell_form_id)
            })
            .or_insert(cell.cell_form_id);
    }
    cells
}

pub(crate) fn grid_for_translation(
    policy: &ExteriorCoordinatePolicy,
    translation: [f32; 3],
) -> GridCoordinate {
    policy.grid_for_bevy([
        f64::from(translation[0]),
        f64::from(translation[1]),
        f64::from(translation[2]),
    ])
}

pub(crate) fn desired_plan(
    index: &ExteriorWorldspaceIndex,
    current_grid: GridCoordinate,
    velocity_grid: (i32, i32),
    states: &[ExteriorCellState],
    resident_budget: usize,
    byte_budget: u64,
) -> ExteriorResidencyPlan {
    plan_residency(
        ExteriorResidencyInput {
            current_grid,
            velocity_grid,
            resident_budget,
            byte_budget,
            near_radius: 1,
            prefetch_radius: 1,
            distant_radius: Some(2),
        },
        &index_cells(index),
        states,
    )
}

#[cfg(test)]
#[path = "policy_tests.rs"]
mod tests;
