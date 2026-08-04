use bevy::prelude::*;
use bevy_landmass::prelude::*;
use bevyout_core::manifest::exterior::{
    ExteriorCellLifecycle, ExteriorCellPackage, GridCoordinate,
};

use crate::viewer::nav::agent::NavAgent;
use crate::viewer::nav::api;
use crate::viewer::nav::world::build::no_nav_graph_error;
use crate::viewer::world::exterior::ExteriorStreamState;
use crate::vsa::PreparedNavGraph;

pub(crate) fn exterior_resident_grid_signature(world: &World) -> Vec<GridCoordinate> {
    let Some(state) = world.get_resource::<ExteriorStreamState>() else {
        return Vec::new();
    };
    state
        .cells
        .iter()
        .filter(|(_, cell)| {
            cell.collision_ready
                && matches!(
                    cell.state.lifecycle,
                    ExteriorCellLifecycle::Ready | ExteriorCellLifecycle::Resident
                )
                && cell.package.is_some()
        })
        .map(|(grid, _)| *grid)
        .collect()
}

/// Reads every collision-ready resident exterior graph into one landmass
/// input. The package lifecycle is the ownership authority: a graph is never
/// admitted while its package is still loading or its BoxDDD collider is not
/// attached. This also makes a newly resident neighbor visible to the next
/// `tna` command without rebuilding from only the active manifest.
pub(crate) fn read_resident_exterior_graph(
    world: &World,
) -> Result<(PreparedNavGraph, Vec<(GridCoordinate, ExteriorCellPackage)>), api::NavError> {
    let state = world.get_resource::<ExteriorStreamState>().ok_or_else(|| {
        api::NavError::new(
            "exterior_nav_not_ready",
            "exterior stream is not initialized",
        )
    })?;
    let asset_root = state.asset_root.clone().ok_or_else(|| {
        api::NavError::new(
            "exterior_nav_not_ready",
            "exterior asset root is unavailable",
        )
    })?;
    let mut residents = state
        .cells
        .iter()
        .filter(|(_, cell)| {
            cell.collision_ready
                && matches!(
                    cell.state.lifecycle,
                    ExteriorCellLifecycle::Ready | ExteriorCellLifecycle::Resident
                )
        })
        .filter_map(|(grid, cell)| cell.package.clone().map(|package| (*grid, package)))
        .collect::<Vec<_>>();
    residents.sort_by_key(|(grid, package)| (*grid, package.cell_form_id));
    if residents.is_empty() {
        return Err(api::NavError::new(
            "exterior_nav_not_ready",
            "no collision-ready resident exterior package",
        ));
    }

    let mut combined = None;
    for (_grid, package) in &residents {
        let Some(navigation) = package.navigation.as_ref() else {
            return Err(api::NavError::new(
                "exterior_nav_not_ready",
                format!(
                    "cell {:08x} has no prepared navigation tile",
                    package.cell_form_id
                ),
            ));
        };
        if !navigation.clearance_ready || navigation.graph_asset_path.is_none() {
            return Err(api::NavError::new(
                "exterior_nav_not_ready",
                format!(
                    "cell {:08x} has no collision-cleared semantic NAVM artifact",
                    package.cell_form_id
                ),
            ));
        }
        let graph = crate::viewer::nav::read_nav_graph_for_exterior_package(&asset_root, package)
            .map_err(|error| {
            warn!(
                "exterior nav graph read failed for cell {:08x}: {error:#}",
                package.cell_form_id
            );
            api::NavError::new(
                "exterior_nav_not_ready",
                format!(
                    "cell {:08x} graph artifact is unreadable",
                    package.cell_form_id
                ),
            )
        })?;
        if let Some(target) = combined.as_mut() {
            append_exterior_nav_graph(target, graph);
        } else {
            combined = Some(graph);
        }
    }
    let mut combined = combined.ok_or_else(no_nav_graph_error)?;
    if let Some(current) = state.cells.get(&state.current_grid) {
        combined.cell_form_id = current.state.cell_form_id;
    }
    Ok((combined, residents))
}

pub(crate) fn append_exterior_nav_graph(target: &mut PreparedNavGraph, source: PreparedNavGraph) {
    let was_empty = target.meshes.is_empty();
    if was_empty {
        target.revision = source.revision.clone();
        target.bounds = source.bounds;
        target.counters = source.counters;
    } else {
        for axis in 0..3 {
            target.bounds.min[axis] = target.bounds.min[axis].min(source.bounds.min[axis]);
            target.bounds.max[axis] = target.bounds.max[axis].max(source.bounds.max[axis]);
        }
        macro_rules! add_counter {
            ($($field:ident),+ $(,)?) => {$(
                target.counters.$field += source.counters.$field;
            )+};
        }
        add_counter!(
            meshes,
            polygons,
            vertices,
            doors,
            external_connections,
            mesh_merges,
            mesh_merges_rejected,
            diagnostics_warning,
            diagnostics_error,
            mesh_merges_authored,
            mesh_merges_geometric,
            merge_candidates_authored,
            merge_candidates_geometric,
            nvex_targets_inside_cell,
            nvex_targets_outside_cell,
            nvci_subrecords,
            nvci_entries,
            nvci_door_matches,
            nvci_navmesh_matches,
            clearance_removed_unsupported,
            clearance_cut_obstructed,
            clearance_dropped_unfit,
            clearance_walkable_total,
            clearance_collision_triangles,
            clearance_clipped_polygons,
            clearance_added_vertices,
        );
    }
    target.meshes.extend(source.meshes);
    target.diagnostics.extend(source.diagnostics);
    target.mesh_merges.extend(source.mesh_merges);
}

/// Rebinds agents that survived an exterior resident-set rebuild to the new
/// archipelago. Their transforms, targets, KCC state, and actor identity stay
/// untouched; only the landmass ownership reference changes. Cell swaps use
/// the existing ledger path instead because those agents may need a door
/// marker or frozen-position restore in a different active manifest.
pub(crate) fn retarget_live_exterior_agents(world: &mut World, archipelago_entity: Entity) {
    let mut agents = world
        .query_filtered::<Entity, With<NavAgent>>()
        .iter(world)
        .collect::<Vec<_>>();
    agents.sort_unstable_by_key(|entity| entity.index_u32());
    for entity in agents {
        if let Ok(mut agent) = world.get_entity_mut(entity) {
            agent.insert(ArchipelagoRef3d::new(archipelago_entity));
        }
    }
}
