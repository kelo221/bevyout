use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;

use bevy::prelude::*;
use bevy_boxddd::boxddd;
use bevy_boxddd::prelude::BoxdddPhysicsContext;
use bevy_landmass::NavMeshHandle;
use bevy_landmass::coords::ThreeD;
use bevy_landmass::prelude::*;

use crate::viewer::nav::agent::{
    AGENT_HEIGHT, AGENT_RADIUS, NavCellFallBounds, PREFERRED_PATHING_TYPE_INDEX_COST,
    archipelago_options,
};
use crate::viewer::nav::api;
use crate::viewer::nav::landmass_graph;
use crate::viewer::nav::world::portals::{
    exterior_portal_merge_inputs, validate_exterior_merge_link_collision,
    validate_merge_link_collision,
};
use crate::viewer::nav::world::state::{
    BlockedDoorLink, DoorLockInfo, LinkKind, MidRouteDoor, NavArchipelagoState, TravelDoorLink,
};
use crate::viewer::player;
use crate::viewer::player::{CellPhysicsReadiness, PhysicsDisabled};

use super::{
    exterior::{
        exterior_resident_grid_signature, read_resident_exterior_graph,
        retarget_live_exterior_agents,
    },
    links::{spawn_exterior_link_pair, spawn_link_pair},
    player_obstacle::spawn_player_nav_character,
};
use crate::viewer::nav::agent::fall_guard;

pub(crate) fn no_nav_graph_error() -> api::NavError {
    api::NavError::new("no_nav_graph", "no nav graph prepared for this cell")
}

/// `tna` capsules store their Bevy transform at the capsule centre, while
/// prepared NAVM points (including animation-link portals) are feet-level.
/// `landmass` measures animation-link arrival in full 3D and otherwise uses
/// only the agent radius, so its default reach distance cannot bridge that
/// intentional half-height offset on a shallow portal.
pub(crate) const TEST_AGENT_ANIMATION_LINK_REACHED_DISTANCE: f32 =
    AGENT_HEIGHT * 0.5 + AGENT_RADIUS;

pub(crate) fn teardown_archipelago(world: &mut World) {
    let state = std::mem::take(&mut *world.resource_mut::<NavArchipelagoState>());
    for entity in state
        .links
        .into_iter()
        .chain(state.islands)
        .chain(state.player_character)
        .chain(state.archipelago)
    {
        if let Ok(entity) = world.get_entity_mut(entity) {
            entity.despawn();
        }
    }
}

/// Builds (or reuses, if already current for this cell) the archipelago,
/// islands, and door-link entities for the active cell's prepared nav
/// graph. Lazy: only called from `tna spawn`, never eagerly per cell swap.
pub(crate) fn ensure_archipelago(world: &mut World) -> Result<(), api::NavError> {
    let (current_cell, travel_destinations, mut door_lock_info, door_positions, exterior_mode) = {
        let manifest = world
            .get_resource::<crate::viewer::LoadedSceneManifest>()
            .ok_or_else(no_nav_graph_error)?;
        let exterior_mode = manifest.exterior.is_some();
        let travel_destinations = crate::viewer::nav::travel_door_destinations(manifest);
        let mut door_lock_info = HashMap::new();
        let mut door_positions = HashMap::new();
        for placement in &manifest.placements {
            if let crate::vsa::PreparedSemantic::Door(door) = &placement.semantic {
                door_lock_info.insert(
                    placement.reference_form_id,
                    DoorLockInfo {
                        lock_level: door.lock_level,
                        key_form_id: door.key_form_id,
                        trapped: door.trapped,
                    },
                );
                door_positions.insert(
                    placement.reference_form_id,
                    Vec3::from_array(placement.translation),
                );
            }
        }
        (
            manifest
                .exterior
                .as_ref()
                .map(|package| package.cell_form_id)
                .unwrap_or(manifest.cell.form_id),
            travel_destinations,
            door_lock_info,
            door_positions,
            exterior_mode,
        )
    };

    let exterior_resident_grids = if exterior_mode {
        exterior_resident_grid_signature(world)
    } else {
        Default::default()
    };
    let already_current = {
        let state = world.resource::<NavArchipelagoState>();
        state.cell_form_id == Some(current_cell)
            && state.archipelago.is_some()
            && (!exterior_mode || state.exterior_resident_grids == exterior_resident_grids)
    };
    if already_current {
        return Ok(());
    }

    // Issue #169: a runtime lock change already recorded before this build
    // -- an early `setlock` issued before any archipelago exists yet (this
    // resource is `init_resource`d at plugin install, so it exists and is
    // writable long before the first `tna spawn`), or one that landed
    // earlier in the same session -- must survive the rebuild below. The
    // authored baseline just read from the manifest is the fallback, not
    // the winner: overlay whatever `NavArchipelagoState.door_lock_info`
    // (the exact map `set_door_lock_level`/`setlock` writes into) already
    // holds for each door FormID before `teardown_archipelago` resets the
    // resource to its default a few lines down. A door this session never
    // touched keeps its authored value untouched.
    for (&door_form_id, &info) in &world.resource::<NavArchipelagoState>().door_lock_info {
        door_lock_info.insert(door_form_id, info);
    }

    let manifest = world
        .get_resource::<crate::viewer::LoadedSceneManifest>()
        .ok_or_else(no_nav_graph_error)?;
    let (graph, exterior_packages) = if exterior_mode {
        read_resident_exterior_graph(world)?
    } else {
        let graph = crate::viewer::nav::read_nav_graph_for_manifest(manifest).map_err(|error| {
            warn!("nav graph read failed for cell {current_cell:08x}: {error:#}");
            no_nav_graph_error()
        })?;
        (graph, Vec::new())
    };
    // Issue #164: capture this cell's lowest prepared geometry Y so the fall
    // guard can derive its kill plane from real per-cell bounds rather than a
    // hard-coded world Y (see `fall_guard`'s module doc).
    let graph_min_y = graph.bounds.min[1];
    if let Some(position) = player_transform_query(world)
        && (!position.is_finite() || position.y < fall_guard::fall_kill_z(graph.bounds.min[1]))
    {
        let kill_z = fall_guard::fall_kill_z(graph.bounds.min[1]);
        warn!(
            "nav agent build rejected off-world player position=({:.2},{:.2},{:.2}) kill_z={kill_z:.2}",
            position.x, position.y, position.z
        );
        return Err(api::NavError::new(
            "player_off_navmesh",
            format!(
                "the FPS player is outside the prepared navigation bounds at ({:.2}, {:.2}, {:.2}); reposition onto prepared terrain before tna spawn",
                position.x, position.y, position.z
            ),
        ));
    }
    let mesh_inputs = crate::viewer::nav::mesh_inputs(&graph);
    let mut merge_inputs = crate::viewer::nav::merge_inputs(&graph);
    let mut border_link_keys = BTreeSet::new();
    if exterior_mode {
        let border_links = exterior_portal_merge_inputs(&exterior_packages, &graph);
        let border_link_count = border_links.len();
        border_link_keys.extend(border_links.iter().map(|link| {
            (
                link.mesh_a_form_id,
                link.triangle_a,
                link.mesh_b_form_id,
                link.triangle_b,
            )
        }));
        warn!(
            "exterior nav resident graphs={} border_links={} meshes={} polygons={}",
            exterior_packages.len(),
            border_link_count,
            graph.counters.meshes,
            graph.counters.clearance_walkable_total,
        );
        merge_inputs.extend(border_links);
    }
    // Issue #155 feature 1: one archipelago-wide door FormID -> type index
    // mapping, computed once before any mesh's conversion (every mesh must
    // agree on the same door's type index -- see `door_type_indices`'s doc
    // comment).
    let door_type_indices = landmass_graph::door_type_indices(&mesh_inputs);
    // Issue #177: the blocking derived-association class takes its own
    // indices, allocated above every `door_type_indices` one.
    let closed_door_type_indices =
        landmass_graph::closed_door_type_indices(&mesh_inputs, &door_type_indices);
    let openable_blockers = landmass_graph::openable_blockers(&mesh_inputs);

    // Widened sample distances plus the clamped border-avoidance horizon --
    // see `archipelago_options`, which every build (runtime and test) shares.
    // Stage every fallible graph conversion before touching the currently
    // active Bevy entities. A streamed exterior rebuild that cannot produce a
    // valid replacement therefore leaves the old archipelago authoritative.
    let mut prepared_nav_meshes = Vec::new();
    for mesh in &mesh_inputs {
        let result = landmass_graph::build_navigation_mesh(
            mesh,
            &merge_inputs,
            &door_type_indices,
            &closed_door_type_indices,
        );
        for diagnostic in &result.diagnostics {
            warn!(
                "nav landmass conversion mesh {:08x}: {}",
                mesh.form_id, diagnostic.message
            );
        }
        if let Some(valid) = result.nav_mesh {
            prepared_nav_meshes.push(Arc::new(valid));
        }
    }
    if prepared_nav_meshes.is_empty() {
        return Err(api::NavError::new(
            "nav_mesh_invalid",
            "prepared nav graph produced no valid landmass islands",
        ));
    }

    // Commit boundary: all graph input, player validation, mesh conversion,
    // and the non-mutating link validation above have succeeded. Only now may
    // the old owned world be retired and the pending replacement spawned.
    teardown_archipelago(world);
    let archipelago_entity = world.spawn(Archipelago3d::new(archipelago_options())).id();
    apply_preferred_pathing_base_cost(
        world,
        archipelago_entity,
        &door_type_indices,
        &closed_door_type_indices,
    );

    let mut islands = Vec::new();
    for nav_mesh in prepared_nav_meshes {
        let handle = world
            .resource_mut::<Assets<NavMesh3d>>()
            .add(NavMesh3d { nav_mesh });
        let island_entity = world
            .spawn(Island3dBundle {
                island: Island,
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
                nav_mesh: NavMeshHandle::<ThreeD>(handle),
            })
            .id();
        islands.push(island_entity);
    }

    if islands.is_empty() {
        if let Ok(entity) = world.get_entity_mut(archipelago_entity) {
            entity.despawn();
        }
        return Err(api::NavError::new(
            "nav_mesh_invalid",
            "prepared nav graph produced no valid landmass islands",
        ));
    }

    // Issue #114 added scope: the player's mirrored landmass character
    // lives exactly as long as this archipelago (see `NavArchipelagoState::
    // player_character`'s doc comment).
    let player_character = spawn_player_nav_character(world, archipelago_entity);

    let mut links = Vec::new();
    let mut link_kinds = HashMap::new();
    let mut blocked_door_links = Vec::new();
    let mut door_usable = HashMap::new();
    let mut door_open = HashMap::new();

    // Same-cell cross-mesh merge links (issue #113 feature 2). Real FO3
    // meshes never share seam vertex positions, so landmass's native island
    // boundary linking cannot connect them (it needs coincident boundary
    // vertices); generated walk-through animation links across the matched
    // boundary edges are the path real data takes.
    //
    // Runtime collision-visibility validation (issue #154 real-data
    // acceptance correction) runs as its own pass first, in a scoped
    // borrow of the physics world that cannot overlap the `world` borrow
    // `spawn_link_pair` needs below -- see `validate_merge_link_collision`'s
    // doc comment for why prepare-side geometry alone is not enough. A
    // cell whose static collision has not finished building yet (rare in
    // practice: `tna spawn` is a manual console action issued after the
    // cell is already visibly loaded) skips validation rather than
    // dropping every merge link's connectivity for the session.
    let candidate_merge_descriptors =
        landmass_graph::merge_link_descriptors(&mesh_inputs, &merge_inputs);
    let candidate_border_count = candidate_merge_descriptors
        .iter()
        .filter(|descriptor| {
            border_link_keys.contains(&(
                descriptor.side_a.mesh_form_id,
                descriptor.side_a.polygon_index,
                descriptor.side_b.mesh_form_id,
                descriptor.side_b.polygon_index,
            ))
        })
        .count();
    let candidate_merge_count = candidate_merge_descriptors.len();
    let candidate_border_pairs = candidate_merge_descriptors
        .iter()
        .filter(|descriptor| {
            border_link_keys.contains(&(
                descriptor.side_a.mesh_form_id,
                descriptor.side_a.polygon_index,
                descriptor.side_b.mesh_form_id,
                descriptor.side_b.polygon_index,
            ))
        })
        .map(|descriptor| {
            format!(
                "{:08x}:{}<->{:08x}:{}@({:.2},{:.2})-({:.2},{:.2})",
                descriptor.side_a.mesh_form_id,
                descriptor.side_a.polygon_index,
                descriptor.side_b.mesh_form_id,
                descriptor.side_b.polygon_index,
                descriptor.side_a.midpoint[0],
                descriptor.side_a.midpoint[2],
                descriptor.side_b.midpoint[0],
                descriptor.side_b.midpoint[2],
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let physics_disabled = world.resource::<PhysicsDisabled>().0;
    let physics_ready = !physics_disabled
        && world
            .get_resource::<CellPhysicsReadiness>()
            .is_some_and(|readiness| readiness.static_collision_ready());
    let validated_merge_descriptors = if !physics_ready {
        candidate_merge_descriptors
    } else if let Some(physics_world) = world
        .get_non_send_mut::<BoxdddPhysicsContext>()
        .as_deref_mut()
        .and_then(BoxdddPhysicsContext::world_mut)
    {
        let mover = boxddd::Capsule::new(
            [0.0, -(AGENT_HEIGHT * 0.5 - AGENT_RADIUS), 0.0],
            [0.0, AGENT_HEIGHT * 0.5 - AGENT_RADIUS, 0.0],
            AGENT_RADIUS,
        );
        let collision_filter = player::player_collision_filter();
        let support_filter = player::stair_support_filter();
        let mut validated = Vec::with_capacity(candidate_merge_descriptors.len());
        for descriptor in candidate_merge_descriptors {
            let start = Vec3::from_array(descriptor.side_a.midpoint);
            let end = Vec3::from_array(descriptor.side_b.midpoint);
            let is_exterior_border = border_link_keys.contains(&(
                descriptor.side_a.mesh_form_id,
                descriptor.side_a.polygon_index,
                descriptor.side_b.mesh_form_id,
                descriptor.side_b.polygon_index,
            ));
            let result = if is_exterior_border {
                // Adjacent Fallout cells share a terrain border but do not
                // promise a cooked support triangle at the exact midpoint
                // between the two inset endpoints. Endpoint support plus
                // the swept crossing is the meaningful test for this seam;
                // same-cell merge links retain the stricter midpoint probe.
                validate_exterior_merge_link_collision(
                    physics_world,
                    &mover,
                    collision_filter,
                    support_filter,
                    start,
                    end,
                )
            } else {
                validate_merge_link_collision(
                    physics_world,
                    &mover,
                    collision_filter,
                    support_filter,
                    start,
                    end,
                )
            };
            match result {
                Ok(()) => validated.push(descriptor),
                Err(reason) => {
                    warn!(
                        "nav merge link mesh {:08x} triangle {} <-> mesh {:08x} triangle {}: dropped ({})",
                        descriptor.side_a.mesh_form_id,
                        descriptor.side_a.polygon_index,
                        descriptor.side_b.mesh_form_id,
                        descriptor.side_b.polygon_index,
                        reason.as_str(),
                    );
                }
            }
        }
        validated
    } else {
        candidate_merge_descriptors
    };

    if exterior_mode {
        let validated_border_count = validated_merge_descriptors
            .iter()
            .filter(|descriptor| {
                border_link_keys.contains(&(
                    descriptor.side_a.mesh_form_id,
                    descriptor.side_a.polygon_index,
                    descriptor.side_b.mesh_form_id,
                    descriptor.side_b.polygon_index,
                ))
            })
            .count();
        warn!(
            "exterior nav merge candidates border_inputs={} candidates={} border_candidates={} validated={} validated_border={}",
            border_link_keys.len(),
            candidate_merge_count,
            candidate_border_count,
            validated_merge_descriptors.len(),
            validated_border_count,
        );
        warn!("exterior nav border descriptor pairs={candidate_border_pairs}");
    }

    // Issue #162 feature 1: each validated merge candidate gets its own
    // `landmass` animation-link kind (`landmass_graph::merge_link_kind`,
    // deterministic by position in this already-deterministic order), the
    // identity a per-agent quarantine (`PermittedAnimationLinks`) can
    // exclude without touching any other link -- see that function's doc
    // comment for why this achieves exact single-link granularity, unlike
    // door locking's polygon type-index scheme.
    let merge_link_kind_count = validated_merge_descriptors.len();
    for (index, descriptor) in validated_merge_descriptors.into_iter().enumerate() {
        let start = Vec3::from_array(descriptor.side_a.midpoint);
        let end = Vec3::from_array(descriptor.side_b.midpoint);
        // Issue #154 feature 3: real traversal-distance cost, floored well
        // above zero -- `AnimationLink3d::cost` must stay strictly positive
        // regardless of how tight a validated portal's overlap ended up.
        let cost = descriptor.distance.max(0.01);
        let kind = landmass_graph::merge_link_kind(index);
        let is_exterior_border = border_link_keys.contains(&(
            descriptor.side_a.mesh_form_id,
            descriptor.side_a.polygon_index,
            descriptor.side_b.mesh_form_id,
            descriptor.side_b.polygon_index,
        ));
        let link_entities = if is_exterior_border {
            // Point destinations are retained for same-cell and door links,
            // where the upstream point-destination workaround avoids a
            // boundary-clipping NaN. Exterior endpoints have now been
            // selected inside their named triangles; a tiny finite segment
            // lets landmass's sample_edge retain that attached polygon
            // instead of silently dropping a zero-width destination.
            spawn_exterior_link_pair(world, archipelago_entity, start, end, cost, kind)
        } else {
            spawn_link_pair(world, archipelago_entity, start, end, cost, kind)
        };
        for link_entity in link_entities {
            link_kinds.insert(link_entity, LinkKind::Merge { kind });
            links.push(link_entity);
        }
    }

    // Two-sided intra-cell door links (wave 3). A locked door is excluded
    // from route planning as blocked until usable (issue #113 feature 3);
    // `door_availability_system` spawns the link when the door becomes
    // usable and triggers a repath.
    for descriptor in landmass_graph::door_link_descriptors(&mesh_inputs) {
        let start = Vec3::from_array(descriptor.side_a.midpoint);
        let end = Vec3::from_array(descriptor.side_b.midpoint);
        let usable = door_usable_now(world, None, descriptor.door_form_id, &door_lock_info);
        door_usable.insert(descriptor.door_form_id, usable);
        if usable {
            // Kind 0 (issue #162): every door link shares the reserved
            // "never quarantined" kind, so a blocked merge portal can never
            // make a door impassable for an agent that has nothing to do
            // with it.
            for link_entity in spawn_link_pair(world, archipelago_entity, start, end, 1.0, 0) {
                link_kinds.insert(
                    link_entity,
                    LinkKind::Door {
                        form_id: descriptor.door_form_id,
                    },
                );
                links.push(link_entity);
            }
        } else {
            info!(
                "nav agent door link {:08x} blocked: locked",
                descriptor.door_form_id
            );
            blocked_door_links.push(BlockedDoorLink {
                door_form_id: descriptor.door_form_id,
                start,
                end,
            });
        }
    }

    // Single-sided door triangles (issue #113 feature 3 / #137). Every one
    // is a crossing-gate candidate (issue #137: real data shows nearly all
    // of them also resolve to a travel destination, so the candidate set
    // cannot be limited to the non-travel subset -- see this file's module
    // doc). One whose reference *does* resolve to a travel door additionally
    // gets a terminal travel link: no landmass animation link is spawned
    // for it -- the far side lives in another cell's NAVM, so there is
    // nothing on-mesh to link to; the agent routes *to* the triangle and
    // the travel-arrival lifecycle runs there when it is the agent's own
    // `travel_intent` target, or the crossing gate runs when it is merely
    // on the way to somewhere else.
    let mut travel_doors = HashMap::new();
    let mut mid_route_doors = Vec::new();
    for door in landmass_graph::single_sided_doors(&mesh_inputs) {
        let triangle_midpoint = Vec3::from_array(door.side.midpoint);
        door_usable.insert(
            door.door_form_id,
            door_usable_now(world, None, door.door_form_id, &door_lock_info),
        );
        mid_route_doors.push(MidRouteDoor {
            door_form_id: door.door_form_id,
            vertices: door.vertices.map(Vec3::from_array),
        });
        if let Some(&destination) = travel_destinations.get(&door.door_form_id) {
            let door_position = door_positions
                .get(&door.door_form_id)
                .copied()
                .unwrap_or(triangle_midpoint);
            info!(
                "nav agent travel door {:08x} -> cell {:08x}",
                door.door_form_id, destination.cell_form_id
            );
            travel_doors.insert(
                door.door_form_id,
                TravelDoorLink {
                    triangle_midpoint,
                    door_position,
                    destination_cell_form_id: destination.cell_form_id,
                    destination_door_form_id: destination.door_reference_form_id,
                },
            );
        }
    }

    // Derived blocker crossing gates (issue #177). Ordinary in-cell doors
    // carry no authored `NVDP` triangle association at all, so none of the
    // loops above ever saw them; `prepare` now derives the association from
    // the door's own collision footprint and this feeds it into the exact
    // same `mid_route_doors` crossing-gate set an authored travel-door
    // triangle uses -- pause -> request open -> wait -> traverse -> resume.
    // A blocker's *interior* polygons are deliberately absent here: they are
    // priced impassable while it is closed instead (`closed_door_type_
    // indices`), so an agent is never asked to stop inside a solid slab.
    for gate in landmass_graph::derived_door_gates(&mesh_inputs) {
        door_usable
            .entry(gate.door_form_id)
            .or_insert_with(|| door_usable_now(world, None, gate.door_form_id, &door_lock_info));
        mid_route_doors.push(MidRouteDoor {
            door_form_id: gate.door_form_id,
            vertices: gate.vertices.map(Vec3::from_array),
        });
    }
    // Every blocker with a closed-state override needs an open-state entry,
    // whether or not it also has a crossing gate: `door_availability_system`
    // polls exactly the tracked set.
    for &blocker_form_id in closed_door_type_indices.keys() {
        door_usable
            .entry(blocker_form_id)
            .or_insert_with(|| door_usable_now(world, None, blocker_form_id, &door_lock_info));
    }
    for &door_form_id in door_usable.keys() {
        let (open, _) = door_open_and_locked(world, None, door_form_id, &door_lock_info);
        door_open.insert(door_form_id, open);
    }

    *world.resource_mut::<NavArchipelagoState>() = NavArchipelagoState {
        cell_form_id: Some(current_cell),
        exterior_resident_grids,
        archipelago: Some(archipelago_entity),
        player_character: Some(player_character),
        islands,
        links,
        link_kinds,
        blocked_door_links,
        travel_doors,
        mid_route_doors,
        door_usable,
        door_lock_info,
        door_type_indices,
        closed_door_type_indices,
        openable_blockers,
        door_open,
        merge_link_kind_count,
    };
    world.resource_mut::<NavCellFallBounds>().min_y = Some(graph_min_y);
    if exterior_mode {
        retarget_live_exterior_agents(world, archipelago_entity);
    }
    Ok(())
}

/// Issue #168: applies [`PREFERRED_PATHING_TYPE_INDEX_COST`] as the
/// archipelago-wide base cost for `landmass_graph::preferred_pathing_type_
/// index`'s type index, so authored preferred-pathing polygons (#156
/// feature 1) actually route cheaper instead of the type existing but never
/// being priced. Called once per archipelago build (`ensure_archipelago`,
/// right after the entity is spawned), not per-agent: this is a shared
/// terrain preference every agent in this archipelago gets, unlike door
/// locking (#155) or merge-portal quarantine (#162), which are per-agent
/// `AgentTypeIndexCostOverrides`/`PermittedAnimationLinks` exceptions to
/// this shared baseline. `set_type_index_cost` only errors on a
/// non-positive cost (`landmass` 0.9.2's `SetTypeIndexCostError::
/// NonPositiveCost`, see the constant's own doc comment for the
/// verification) -- unreachable for this fixed, positive, compile-time
/// constant, so the `Result` is discarded rather than propagated.
pub(crate) fn apply_preferred_pathing_base_cost(
    world: &mut World,
    archipelago_entity: Entity,
    door_type_indices: &BTreeMap<u32, usize>,
    closed_door_type_indices: &BTreeMap<u32, usize>,
) {
    let preferred_pathing_index =
        landmass_graph::preferred_pathing_type_index(door_type_indices, closed_door_type_indices);
    if let Ok(mut entity) = world.get_entity_mut(archipelago_entity)
        && let Some(mut archipelago) = entity.get_mut::<Archipelago3d>()
    {
        let _ = archipelago
            .set_type_index_cost(preferred_pathing_index, PREFERRED_PATHING_TYPE_INDEX_COST);
    }
}

pub(crate) use crate::viewer::nav::debug::player::player_transform_query;
pub(crate) use crate::viewer::nav::doors::access::{door_open_and_locked, door_usable_now};
