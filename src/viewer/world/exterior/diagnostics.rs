//! Stable, compact exterior streaming diagnostics.

use serde_json::json;

use super::lifecycle::ExteriorStreamState;

pub(crate) fn status(state: &ExteriorStreamState) -> serde_json::Value {
    let mut counts = [0usize; 7];
    for cell in state.cells.values() {
        let index = match cell.state.lifecycle {
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Unloaded => 0,
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Queued => 1,
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Loading => 2,
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Ready => 3,
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Resident => 4,
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Evicting => 5,
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Failed => 6,
        };
        counts[index] += 1;
    }
    json!({
        "initialized": state.initialized,
        "worldspace": state.worldspace_form_id,
        "grid": [state.current_grid.x, state.current_grid.y],
        "unloaded": counts[0],
        "queued": counts[1],
        "loading": counts[2],
        "ready": counts[3],
        "resident": counts[4],
        "evicting": counts[5],
        "failed": counts[6],
        "requests": state.requests,
        "ready_total": state.ready,
        "evictions": state.evictions,
        "cancellations": state.cancellations,
        "stale_completions": state.stale_completions,
        "failures": state.failures,
        "collision_tracked": state.collision_cells.len(),
        "collision_pending": state
            .cells
            .values()
            .filter(|cell| {
                cell.state.lifecycle == bevyout_core::manifest::exterior::ExteriorCellLifecycle::Loading
                    && !cell.collision_ready
            })
            .count(),
        "resident_budget": state.resident_budget,
        "byte_budget": state.byte_budget,
        "resident_bytes": state.resident_bytes,
        "peak_resident_cells": state.peak_resident_cells,
        "peak_memory": state.peak_memory,
    })
}

pub(crate) fn cells(state: &ExteriorStreamState) -> serde_json::Value {
    serde_json::Value::Array(
        state
            .cells
            .iter()
            .map(|(grid, cell)| {
                json!({
                    "grid": [grid.x, grid.y],
                    "form_id": format!("{:08x}", cell.state.cell_form_id),
                    "lifecycle": format!("{:?}", cell.state.lifecycle),
                    "generation": cell.state.generation,
                    "bytes": cell.state.estimated_bytes,
                    "collision_ready": cell.collision_ready,
                })
            })
            .collect(),
    )
}
