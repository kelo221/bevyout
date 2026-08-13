use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy_landmass::prelude::*;
use bevyout_core::manifest::exterior::{
    ExteriorCellLifecycle, ExteriorCellPackage, ExteriorCoordinatePolicy, GridCoordinate,
};

use crate::viewer::nav::agent::NavAgent;
use crate::viewer::nav::api;
use crate::viewer::nav::landmass_graph::{
    MergeInput, ResidentNavCell, ResidentNavCellKey, ResidentNavState, ResidentNavTopology,
    ResidentPortal,
};
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

/// The lifecycle-owned view of one resident cell's navigation, in the shape
/// W3-B's pure resident-topology policy consumes. `Valid` is only reported
/// with `navigation_ready` when the package's own semantic NAVM artifact is
/// collision-cleared and present, which is the same condition
/// [`read_resident_exterior_graph`] requires before admitting a graph.
pub(crate) fn observe_resident_nav_cells(world: &World) -> Vec<ResidentNavCell> {
    let Some(state) = world.get_resource::<ExteriorStreamState>() else {
        return Vec::new();
    };
    state
        .cells
        .values()
        .map(|cell| {
            let navigation_ready = cell.collision_ready
                && cell.package.as_ref().is_some_and(|package| {
                    package.navigation.as_ref().is_some_and(|navigation| {
                        navigation.clearance_ready && navigation.graph_asset_path.is_some()
                    })
                });
            let state = match cell.state.lifecycle {
                ExteriorCellLifecycle::Unloaded => ResidentNavState::Missing,
                ExteriorCellLifecycle::Queued | ExteriorCellLifecycle::Loading => {
                    ResidentNavState::Loading
                }
                ExteriorCellLifecycle::Failed => ResidentNavState::Failed,
                ExteriorCellLifecycle::Evicting => ResidentNavState::Evicting,
                ExteriorCellLifecycle::Ready | ExteriorCellLifecycle::Resident => {
                    ResidentNavState::Valid { navigation_ready }
                }
            };
            ResidentNavCell {
                key: ResidentNavCellKey {
                    cell_form_id: cell.state.cell_form_id,
                    generation: cell.state.generation,
                },
                state,
            }
        })
        .collect()
}

/// Pairs each cross-cell border link with the two generation-exact resident
/// cells that own its meshes. A link whose mesh owner is unknown is dropped
/// rather than guessed: an unowned side can never be proven still valid.
pub(crate) fn resident_border_portals(
    border_links: &[MergeInput],
    mesh_owners: &BTreeMap<u32, ResidentNavCellKey>,
) -> Vec<ResidentPortal> {
    border_links
        .iter()
        .filter_map(|merge| {
            let side_a = *mesh_owners.get(&merge.mesh_a_form_id)?;
            let side_b = *mesh_owners.get(&merge.mesh_b_form_id)?;
            Some(ResidentPortal {
                side_a,
                side_b,
                merge: *merge,
            })
        })
        .collect()
}

/// Reads every collision-ready resident exterior graph into one landmass
/// input. The package lifecycle is the ownership authority: a graph is never
/// admitted while its package is still loading or its BoxDDD collider is not
/// attached. This also makes a newly resident neighbor visible to the next
/// `tna` command without rebuilding from only the active manifest.
/// One resident-set read: the combined graph, the packages it came from, and
/// which generation-exact resident cell owns each NAVM mesh. The ownership map
/// is what lets a cross-cell border link be validated against the W3-B
/// resident topology instead of being trusted for as long as the archipelago
/// exists.
pub(crate) struct ResidentExteriorGraph {
    pub(crate) graph: PreparedNavGraph,
    pub(crate) packages: Vec<(GridCoordinate, ExteriorCellPackage)>,
    pub(crate) mesh_owners: BTreeMap<u32, ResidentNavCellKey>,
}

pub(crate) fn read_resident_exterior_graph(
    world: &World,
) -> Result<ResidentExteriorGraph, api::NavError> {
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
        .filter_map(|(grid, cell)| {
            cell.package
                .clone()
                .map(|package| (*grid, package, cell.state.generation))
        })
        .collect::<Vec<_>>();
    residents.sort_by_key(|(grid, package, _)| (*grid, package.cell_form_id));
    let mut mesh_owners = BTreeMap::new();
    if residents.is_empty() {
        return Err(api::NavError::new(
            "exterior_nav_not_ready",
            "no collision-ready resident exterior package",
        ));
    }

    let mut combined = None;
    for (_grid, package, generation) in &residents {
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
        for mesh in &graph.meshes {
            mesh_owners.insert(
                mesh.form_id,
                ResidentNavCellKey {
                    cell_form_id: package.cell_form_id,
                    generation: *generation,
                },
            );
        }
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
    Ok(ResidentExteriorGraph {
        graph: combined,
        packages: residents
            .into_iter()
            .map(|(grid, package, _)| (grid, package))
            .collect(),
        mesh_owners,
    })
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
pub(crate) fn retarget_live_exterior_agents(
    world: &mut World,
    archipelago_entity: Entity,
    topology: &ResidentNavTopology,
) {
    // W3-C defect fix: the resident topology, not the mere fact that a
    // rebuild happened, decides whether an agent may be re-pointed. An agent
    // standing in a cell that is evicting, still loading, or already on a
    // newer generation would otherwise adopt an archipelago its own island is
    // not (or no longer) part of, mid border crossing.
    let Some(archipelago) = topology.archipelago.as_ref() else {
        warn!("exterior nav retarget skipped: no navigation-ready resident cell");
        return;
    };
    let ready = archipelago
        .resident_cells
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let policy = ExteriorCoordinatePolicy::default();
    let mut agents = world
        .query_filtered::<Entity, With<NavAgent>>()
        .iter(world)
        .collect::<Vec<_>>();
    agents.sort_unstable_by_key(|entity| entity.index_u32());
    for entity in agents {
        let position = world
            .get::<GlobalTransform>(entity)
            .map(|transform| transform.translation());
        let Some(position) = position else {
            continue;
        };
        let grid = policy.grid_for_bevy([
            f64::from(position.x),
            f64::from(position.y),
            f64::from(position.z),
        ]);
        let key = world
            .get_resource::<ExteriorStreamState>()
            .and_then(|state| state.cells.get(&grid))
            .map(|cell| ResidentNavCellKey {
                cell_form_id: cell.state.cell_form_id,
                generation: cell.state.generation,
            });
        let Some(key) = key.filter(|key| ready.contains(key)) else {
            warn!(
                "exterior nav retarget skipped for agent {entity:?}: grid {},{} is not a navigation-ready resident generation",
                grid.x, grid.y
            );
            continue;
        };
        if let Ok(mut agent) = world.get_entity_mut(entity) {
            agent.insert(ArchipelagoRef3d::new(archipelago_entity));
        }
        debug!(
            "exterior nav retarget agent {entity:?} cell={:08x} generation={}",
            key.cell_form_id, key.generation
        );
    }
}

#[cfg(test)]
#[path = "exterior_tests.rs"]
mod tests;
