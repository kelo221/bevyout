use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_landmass::prelude::*;
use bevy_landmass::{PauseAgent, UsingAnimationLink};

use crate::viewer::interaction;
use crate::viewer::nav::agent::{
    AGENT_HEIGHT, AgentKcc, AgentRuntime, DOOR_TRAVERSAL_SECONDS, DoorTraversal, MergeTraversal,
    NavAgent, NavAgentLedger, RefreshLandmassAnimationLinkInput, TRAVEL_ARRIVAL_DISTANCE,
    agent_ledger_id, write_agent_translation,
};
use crate::viewer::nav::debug::DebugAgentRoster;
use crate::viewer::nav::diagnostics::all_agent_entities;
use crate::viewer::nav::doors::access::door_open_and_locked;
use crate::viewer::nav::traversal::{
    merge_crossing_already_started, merge_traversal_reached_distance, merge_traversal_timeout,
};
use crate::viewer::nav::world::state::{LinkKind, MidRouteDoor, NavArchipelagoState};
use crate::viewer::nav::{door_link, landmass_graph, ledger_policy, movement_policy};

use crate::viewer::nav::doors::runtime::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DoorLinkObservation {
    pub(crate) state: door_link::DoorLinkState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DoorLinkDecision {
    InspectArrival,
    TickPaused {
        door_form_id: u32,
        destination: door_link::LinkDestination,
    },
    WaitForTraversal,
}

pub(crate) fn observe_door_link(
    world: &World,
    agent_entity: Entity,
) -> Option<DoorLinkObservation> {
    world
        .get::<AgentRuntime>(agent_entity)
        .map(|runtime| DoorLinkObservation {
            state: runtime.door_link,
        })
}

pub(crate) fn decide_door_link(observation: DoorLinkObservation) -> DoorLinkDecision {
    match observation.state {
        door_link::DoorLinkState::Idle
        | door_link::DoorLinkState::Failed { .. }
        | door_link::DoorLinkState::TravelReached { .. } => DoorLinkDecision::InspectArrival,
        door_link::DoorLinkState::Paused {
            door_form_id,
            destination,
            ..
        } => DoorLinkDecision::TickPaused {
            door_form_id,
            destination,
        },
        door_link::DoorLinkState::Traversing { .. } => DoorLinkDecision::WaitForTraversal,
    }
}

pub(crate) fn door_traversal_system(
    time: Res<Time>,
    mut agents: Query<
        (
            Entity,
            &mut Transform,
            &mut DoorTraversal,
            &mut AgentRuntime,
        ),
        With<NavAgent>,
    >,
    mut roster: ResMut<DebugAgentRoster>,
    archipelago_state: Res<NavArchipelagoState>,
    mut ledger: ResMut<NavAgentLedger>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut traversal, mut runtime) in &mut agents {
        traversal.elapsed += time.delta_secs();
        let t = (traversal.elapsed / DOOR_TRAVERSAL_SECONDS).clamp(0.0, 1.0);
        write_agent_translation(&mut transform, traversal.start.lerp(traversal.end, t));
        if t >= 1.0 {
            commands
                .entity(entity)
                .remove::<DoorTraversal>()
                .remove::<UsingAnimationLink>();
            match runtime.active_link {
                // Issue #154: a merge-seam crossing no longer drives
                // `DoorTraversal` at all (see `merge_traversal_system`), so
                // `Some(LinkKind::Merge)` should never actually reach here
                // in practice -- kept alongside `None` defensively (just
                // clear the link) rather than treated as an invariant
                // violation, the same conservative stance this match
                // already took for the "no active link" case.
                Some(LinkKind::Merge { .. }) | None => {
                    runtime.active_link = None;
                }
                Some(LinkKind::Door { .. }) => {
                    let new_state = door_link::transition(
                        runtime.door_link,
                        door_link::DoorLinkEvent::TraversalComplete,
                    );
                    let Some((door_form_id, destination_cell_form_id)) = (match new_state {
                        door_link::DoorLinkState::TravelReached {
                            door_form_id,
                            destination_cell_form_id,
                        } => Some((door_form_id, destination_cell_form_id)),
                        _ => None,
                    }) else {
                        runtime.door_link = new_state;
                        runtime.active_link = None;
                        continue;
                    };
                    let travel_matches_generation = runtime.travel_intent.is_some_and(|intent| {
                        intent.generation == runtime.route_generation
                            && intent.door_form_id == door_form_id
                    });
                    if !travel_matches_generation {
                        // A stale physical completion must never hand off an
                        // actor after its route was replaced. Intra-cell
                        // door crossings do not reach TravelReached, so this
                        // guard is specific to the intercell handoff path.
                        runtime.door_link = door_link::DoorLinkState::Idle;
                        runtime.active_link = None;
                        runtime.travel_intent = None;
                        continue;
                    }
                    // Issue #241: the intercell ledger is keyed by console
                    // index (`agent_ledger_id`), so only a roster agent can
                    // actually be handed off to another cell -- an
                    // autonomously-bound actor belongs to the AI/actor slice,
                    // which respawns it with its own cell. It must still
                    // *finish* the lifecycle rather than being skipped: the
                    // old `continue` here left it latched in `Traversing`
                    // forever with its travel intent intact, i.e. a
                    // permanently wedged actor -- exactly the failure mode
                    // this issue is about. Fall through to the same "left at
                    // the travel door" terminal the missing-metadata branch
                    // below already used.
                    let ledger_target = roster
                        .index_of(entity)
                        .filter(|&index| roster.is_spawned_capsule(index))
                        .and_then(|index| {
                            archipelago_state
                                .travel_doors
                                .get(&door_form_id)
                                .map(|link| {
                                    (agent_ledger_id(index), index, link.destination_door_form_id)
                                })
                        });
                    // Issue #113's terminal travel seam: the agent stopped at
                    // the traversed door. Issue #134 owns what happens next:
                    // the agent leaves the active cell entirely, ledgered for
                    // the destination cell at that door's own paired marker.
                    info!(
                        "nav agent travel reached {door_form_id:08x} -> cell {destination_cell_form_id:08x}"
                    );
                    match ledger_target {
                        Some((agent_id, index, destination_door_form_id)) => {
                            ledger.0.record(ledger_policy::LedgerEntry {
                                agent_id,
                                cell_form_id: destination_cell_form_id,
                                spawn_kind: ledger_policy::SpawnKind::DoorMarker {
                                    destination_door_form_id,
                                },
                                remaining_target: None,
                            });
                            info!(
                                "nav agent handoff {agent_id:08x} -> cell {destination_cell_form_id:08x}"
                            );
                            commands.entity(entity).despawn();
                            roster.set(index, None);
                        }
                        None => {
                            // Either no destination-door metadata (defensive:
                            // `travel_doors` always carries it once
                            // `TravelReached` fired through it) or a
                            // non-roster, autonomously-bound actor that has
                            // no ledger identity -- both end the lifecycle
                            // here, standing at the door, rather than losing
                            // or wedging the agent.
                            warn!(
                                "nav agent handoff {door_form_id:08x}: not ledgered; agent left at the travel door"
                            );
                            commands.entity(entity).remove::<AgentTarget3d>();
                            runtime.travel_intent = None;
                            runtime.door_link = new_state;
                            runtime.active_link = None;
                        }
                    }
                }
            }
        }
    }
}

/// Drives the door-link lifecycle for every currently-spawned agent:
/// detects each reaching an off-mesh link (a merge seam is crossed
/// directly; a door link runs the pause -> scripted-open -> wait ->
/// traverse lifecycle) or arriving at a travel door's triangle (issue #113
/// feature 3), requests the door open through the same boundary the
/// `activate` console command uses (`interaction::scripted_door_open`),
/// polls `InteractionState.open`, and starts the kinematic crossing once
/// the door is open. An exclusive (`&mut World`) system since it needs to
/// both query components and call into `interaction`'s `&mut World`-based
/// scripted door boundary in the same step.
pub(crate) fn door_link_system(world: &mut World) {
    // Issue #241: driven off the agent component set, not the console roster
    // -- an autonomously-bound actor whose route crosses a door used to get
    // no door lifecycle at all (no pause, no open request, no traversal, and
    // no travel arrival), which is a mid-route wedge with zero diagnostics.
    for agent_entity in all_agent_entities(world) {
        if world.get_entity(agent_entity).is_err() {
            continue;
        }
        drive_door_link_for_agent(world, agent_entity);
    }
}

pub(crate) fn drive_door_link_for_agent(world: &mut World, agent_entity: Entity) {
    let Some(observation) = observe_door_link(world, agent_entity) else {
        return;
    };
    let current_state = observation.state;

    match decide_door_link(observation) {
        DoorLinkDecision::InspectArrival => {
            // Travel arrival (issue #113 feature 3): a pending travel
            // intent whose door triangle the agent has reached starts the
            // door lifecycle with a Travel destination.
            let travel_arrival = world
                .get::<AgentRuntime>(agent_entity)
                .and_then(|runtime| runtime.travel_intent)
                .and_then(|intent| {
                    let door_form_id = intent.door_form_id;
                    let link = world
                        .resource::<NavArchipelagoState>()
                        .travel_doors
                        .get(&door_form_id)
                        .copied()?;
                    let position = world.get::<Transform>(agent_entity)?.translation;
                    movement_policy::nav_point_reached(
                        position.to_array(),
                        link.triangle_midpoint.to_array(),
                        TRAVEL_ARRIVAL_DISTANCE,
                        AGENT_HEIGHT,
                    )
                    .then_some((intent.generation, door_form_id, link))
                });
            if let Some((generation, door_form_id, link)) = travel_arrival {
                let current_generation = world
                    .get::<AgentRuntime>(agent_entity)
                    .map(|runtime| runtime.route_generation);
                if current_generation != Some(generation) {
                    // A completion from a replaced route is inert. The
                    // replacement seam normally removes this state eagerly;
                    // this guard covers a completion already observed by a
                    // scheduler before the replacement was committed.
                    return;
                }
                // Issue #165: consult the same lock/open source of truth the
                // mid-route crossing gate (`crossing_gate`, just below) and
                // `request_door_open`'s own internal `door_usable_now` check
                // already use, instead of relying solely on the latter's
                // implicit refusal -- the decision is now explicit and
                // symmetric with the mid-route gate rather than something a
                // reader has to trust `request_door_open` enforces
                // correctly. A locked door still routes through the normal
                // `Paused` -> `MAX_WAIT_TICKS` -> `Failed` terminal (`gate`
                // only changes whether `request_door_open` is even worth
                // calling this tick, mirroring the mid-route gate's own
                // `gate != Pass` guard), never an immediate short-circuit --
                // see `door_link::MAX_WAIT_TICKS`'s doc comment for why that
                // deterministic wait bound is the contract, not a fast
                // fail.
                let lock_info = world
                    .resource::<NavArchipelagoState>()
                    .door_lock_info
                    .clone();
                let (door_open, door_locked) =
                    door_open_and_locked(world, Some(agent_entity), door_form_id, &lock_info);
                let gate = door_link::crossing_gate(door_link::CrossingObservation {
                    door_open,
                    door_locked,
                });
                let new_state = door_link::transition(
                    current_state,
                    door_link::DoorLinkEvent::LinkReached {
                        door_form_id,
                        destination: door_link::LinkDestination::Travel {
                            destination_cell_form_id: link.destination_cell_form_id,
                        },
                    },
                );
                world.entity_mut(agent_entity).insert(PauseAgent);
                if gate != door_link::CrossingGate::Pass {
                    request_door_open(world, agent_entity, door_form_id);
                }
                info!("nav agent door wait {door_form_id:08x}");
                let mut runtime = world.get_mut::<AgentRuntime>(agent_entity).unwrap();
                runtime.door_link = new_state;
                runtime.active_link = Some(LinkKind::Door {
                    form_id: door_form_id,
                });
                runtime.pending_traversal = Some((link.triangle_midpoint, link.door_position));
                return;
            }

            // Mid-route door crossing (issue #137): a door's triangle
            // reached by ordinary walking -- not a #113 link endpoint, and
            // not (see the exclusion below) the agent's own active travel
            // terminal -- has no `ReachedAnimationLink3d` event to trigger
            // on. Proximity to its midpoint is the trigger instead,
            // mirroring the travel-door arrival check just above. Only
            // considered while the agent is actually routed somewhere: an
            // idle agent standing near a closed door should not be paused.
            // Skipped in the same tick a travel arrival already fired (that
            // already `return`ed above).
            let has_target = !matches!(
                world.get::<AgentTarget3d>(agent_entity),
                None | Some(AgentTarget3d::None)
            );
            // The agent's own travel terminal (if any) is excluded from the
            // candidate set: that door's full pause -> open -> traverse ->
            // handoff lifecycle is the travel-arrival check's job, not this
            // one's -- double-gating the same door for the same agent would
            // fight it. A *different* agent's travel intent, or this
            // agent's `goto` merely crossing someone else's travel door, is
            // not excluded.
            let travel_target_door = world
                .get::<AgentRuntime>(agent_entity)
                .and_then(|runtime| runtime.travel_intent)
                .map(|intent| intent.door_form_id);
            // Issue #155 feature 3: corridor-based containment
            // (`landmass_graph::point_in_door_triangle` against the door's
            // own un-eroded triangle vertices), not the earlier
            // `MID_ROUTE_DOOR_GATE_DISTANCE` centroid-proximity scan -- a
            // route that merely passes near a doorway without its corridor
            // ever crossing it must not gate.
            let mid_route_crossing = has_target
                .then(|| world.get::<Transform>(agent_entity).map(|t| t.translation))
                .flatten()
                .and_then(|position| {
                    world
                        .resource::<NavArchipelagoState>()
                        .mid_route_doors
                        .iter()
                        .find(|door| {
                            Some(door.door_form_id) != travel_target_door
                                && landmass_graph::point_in_door_triangle(
                                    position.to_array(),
                                    door.vertices.map(|vertex| vertex.to_array()),
                                    AGENT_HEIGHT,
                                )
                        })
                        .copied()
                });
            // Issue #177: containment is a trigger that can be starved.
            // Real data (Vault 101, `VDoor01`) showed an agent routed at a
            // closed in-cell door halting ~2 m short of its crossing with a
            // completely free collision sweep -- never entering the polygon,
            // so never gating, never requesting the open, and never
            // continuing. When the agent has stopped making progress, fall
            // back to the nearest crossing its own route continues through
            // (`door_link::approach_gate`), which is the door it is stalled
            // against. Only consulted when containment found nothing, so the
            // normal case keeps issue #155's exact corridor semantics.
            let mid_route_crossing = mid_route_crossing.or_else(|| {
                let stalled = world
                    .get::<AgentKcc>(agent_entity)
                    .is_some_and(|kcc| kcc.collision_blocked || kcc.stuck);
                if !stalled || !has_target {
                    return None;
                }
                let position = world.get::<Transform>(agent_entity)?.translation;
                let target = match world.get::<AgentTarget3d>(agent_entity)? {
                    AgentTarget3d::Point(point) => *point,
                    AgentTarget3d::Entity(entity) => {
                        world.get::<GlobalTransform>(*entity)?.translation()
                    }
                    AgentTarget3d::None => return None,
                };
                let flat = |a: Vec3, b: Vec3| Vec2::new(a.x - b.x, a.z - b.z).length();
                let agent_distance_to_target = flat(position, target);
                let mut best: Option<(f32, MidRouteDoor)> = None;
                for door in &world.resource::<NavArchipelagoState>().mid_route_doors {
                    if Some(door.door_form_id) == travel_target_door {
                        continue;
                    }
                    let vertices = door.vertices.map(|vertex| vertex.to_array());
                    let Some(distance) = landmass_graph::distance_to_door_triangle(
                        position.to_array(),
                        vertices,
                        AGENT_HEIGHT,
                    ) else {
                        continue;
                    };
                    let centroid = (door.vertices[0] + door.vertices[1] + door.vertices[2]) / 3.0;
                    if !door_link::approach_gate(door_link::ApproachObservation {
                        distance_to_crossing: distance,
                        agent_distance_to_target,
                        crossing_distance_to_target: flat(centroid, target),
                        stalled,
                    }) {
                        continue;
                    }
                    if best.as_ref().is_none_or(|(best, _)| distance < *best) {
                        best = Some((distance, *door));
                    }
                }
                best.map(|(_, door)| door)
            });

            if let Some(door) = mid_route_crossing {
                let lock_info = world
                    .resource::<NavArchipelagoState>()
                    .door_lock_info
                    .clone();
                let (door_open, door_locked) =
                    door_open_and_locked(world, Some(agent_entity), door.door_form_id, &lock_info);
                let gate = door_link::crossing_gate(door_link::CrossingObservation {
                    door_open,
                    door_locked,
                });
                if gate != door_link::CrossingGate::Pass {
                    let new_state = door_link::transition(
                        current_state,
                        door_link::DoorLinkEvent::LinkReached {
                            door_form_id: door.door_form_id,
                            destination: door_link::LinkDestination::IntraCell,
                        },
                    );
                    world.entity_mut(agent_entity).insert(PauseAgent);
                    request_door_open(world, agent_entity, door.door_form_id);
                    info!("nav agent door wait {:08x}", door.door_form_id);
                    let mut runtime = world.get_mut::<AgentRuntime>(agent_entity).unwrap();
                    runtime.door_link = new_state;
                    runtime.active_link = Some(LinkKind::Door {
                        form_id: door.door_form_id,
                    });
                    // No `pending_traversal`: the agent is already standing
                    // on continuous walkable ground (there is no off-mesh
                    // gap to lerp across). The `Paused` arm below treats an
                    // absent `pending_traversal` as an instant
                    // `TraversalComplete` on resume.
                    return;
                }
                // Already open: plain walkable ground, nothing to gate.
                // Fall through to the (extremely unlikely) chance the same
                // tick also reached an unrelated animation link.
            }

            let Some(reached) = world.get::<ReachedAnimationLink3d>(agent_entity) else {
                return;
            };
            let link_entity = reached.link_entity;
            let start_point = reached.start_point;
            let end_point = reached.end_point;
            let Some(&link_kind) = world
                .resource::<NavArchipelagoState>()
                .link_kinds
                .get(&link_entity)
            else {
                return;
            };
            match link_kind {
                LinkKind::Merge { kind } => {
                    // `ReachedAnimationLink3d` remains present while the
                    // crossing system physically sweeps the capsule. The
                    // link system runs before `merge_traversal_system` in
                    // `FixedUpdate`, so without this guard every tick would
                    // replace the in-flight traversal and reset its elapsed
                    // timeout before it could ever reach the far endpoint.
                    if world.get::<MergeTraversal>(agent_entity).is_some() {
                        return;
                    }
                    // A merge seam has no door to wait on (issue #154
                    // feature 4): sweep the agent to the far portal point
                    // with the physics KCC (`merge_traversal_system`)
                    // instead of the door lifecycle's scripted lerp -- a
                    // portal whose far side is actually blocked must stop
                    // the agent for real, not clip it through. Landmass can
                    // report the link while the capsule is still offset from
                    // `start_point`, so the traversal first aligns to that
                    // validated source and then sweeps source -> end. The
                    // fixed timeout covers both legs.
                    let current_position = world
                        .get::<Transform>(agent_entity)
                        .map(|transform| transform.translation);
                    let source_alignment_distance = current_position.map_or(0.0, |position| {
                        movement_policy::horizontal_distance(
                            position.to_array(),
                            start_point.to_array(),
                        )
                    });
                    let crossing_distance = movement_policy::horizontal_distance(
                        start_point.to_array(),
                        end_point.to_array(),
                    );
                    let total_distance = source_alignment_distance + crossing_distance;
                    let crossing_started = current_position.is_none_or(|position| {
                        merge_crossing_already_started(position, start_point, end_point)
                    });
                    info!(
                        "nav agent merge start entity={agent_entity:?} source=({:.2},{:.2},{:.2}) target=({:.2},{:.2},{:.2}) source_distance={source_alignment_distance:.2} crossing_distance={crossing_distance:.2} crossing_started={crossing_started} timeout={:.2}s",
                        start_point.x,
                        start_point.y,
                        start_point.z,
                        end_point.x,
                        end_point.y,
                        end_point.z,
                        merge_traversal_timeout(total_distance),
                    );
                    world.entity_mut(agent_entity).insert((
                        UsingAnimationLink,
                        RefreshLandmassAnimationLinkInput,
                        MergeTraversal {
                            source: start_point,
                            target: end_point,
                            crossing_started,
                            reached_distance: merge_traversal_reached_distance(crossing_distance),
                            elapsed: 0.0,
                            timeout: merge_traversal_timeout(total_distance),
                            link_kind: kind,
                        },
                    ));
                    world
                        .get_mut::<AgentRuntime>(agent_entity)
                        .unwrap()
                        .active_link = Some(LinkKind::Merge { kind });
                }
                LinkKind::Door {
                    form_id: door_form_id,
                } => {
                    let new_state = door_link::transition(
                        current_state,
                        door_link::DoorLinkEvent::LinkReached {
                            door_form_id,
                            destination: door_link::LinkDestination::IntraCell,
                        },
                    );
                    world.entity_mut(agent_entity).insert(PauseAgent);
                    request_door_open(world, agent_entity, door_form_id);
                    info!("nav agent door wait {door_form_id:08x}");
                    let mut runtime = world.get_mut::<AgentRuntime>(agent_entity).unwrap();
                    runtime.door_link = new_state;
                    runtime.active_link = Some(link_kind);
                    runtime.pending_traversal = Some((start_point, end_point));
                }
            }
        }
        DoorLinkDecision::TickPaused {
            door_form_id,
            destination,
        } => {
            let physically_open = world
                .get_resource::<crate::console::RefRegistry>()
                .is_some()
                && crate::console::resolve_reference(world, &format!("{door_form_id:08x}"))
                    .ok()
                    .is_some_and(|door_entity| {
                        world
                            .resource::<interaction::InteractionState>()
                            .open
                            .contains(&door_entity)
                    });
            // Issue #165 (real-data acceptance, contaminated-leg-B
            // measurement): the door's raw physical open flag alone is not
            // enough for a `Travel` destination -- a prior successful
            // hand-off through this exact door leaves it physically open
            // forever, and a later `setlock` + reissued `tna travel` must
            // not let that stale physical state complete a scripted
            // hand-off through what is now a locked door. See
            // `door_link::effective_door_open`'s doc comment for the full
            // rationale (why `IntraCell` keeps the plain physical-open
            // rule and `Travel` does not).
            let lock_info = world
                .resource::<NavArchipelagoState>()
                .door_lock_info
                .clone();
            let (_, door_locked) =
                door_open_and_locked(world, Some(agent_entity), door_form_id, &lock_info);
            let door_open =
                door_link::effective_door_open(destination, physically_open, door_locked);
            let new_state =
                door_link::transition(current_state, door_link::DoorLinkEvent::Tick { door_open });
            if door_link::is_traversing(new_state) {
                world.entity_mut(agent_entity).remove::<PauseAgent>();
                let pending = world
                    .get_mut::<AgentRuntime>(agent_entity)
                    .unwrap()
                    .pending_traversal
                    .take();
                match pending {
                    Some((start, end)) => {
                        world.entity_mut(agent_entity).insert((
                            UsingAnimationLink,
                            DoorTraversal {
                                start,
                                end,
                                elapsed: 0.0,
                            },
                        ));
                        world
                            .get_mut::<AgentRuntime>(agent_entity)
                            .unwrap()
                            .door_link = new_state;
                    }
                    None => {
                        // Mid-route door (issue #137): no off-mesh gap to
                        // cross -- the agent is already standing on
                        // continuous walkable ground, so resuming
                        // completes the crossing in the same tick instead
                        // of waiting on `door_traversal_system` (which only
                        // drives entities carrying `DoorTraversal`).
                        let mut runtime = world.get_mut::<AgentRuntime>(agent_entity).unwrap();
                        runtime.door_link = door_link::transition(
                            new_state,
                            door_link::DoorLinkEvent::TraversalComplete,
                        );
                        runtime.active_link = None;
                    }
                }
                info!("nav agent door resume {door_form_id:08x}");
            } else if door_link::is_failed(new_state) {
                warn!(
                    "nav agent door {door_form_id:08x}: gave up waiting for it to open; agent stopped at the link"
                );
                info!("nav agent unreachable");
                // Issue #165: clearing `travel_intent` alone left
                // `AgentTarget3d` still pointed at this exact door's
                // triangle. The very next tick, `drive_door_link_for_agent`
                // re-enters this `Idle | Failed | TravelReached` match arm
                // (current state is now `Failed`), finds no travel arrival
                // (intent is gone), but the mid-route crossing gate no
                // longer excludes this door either -- its exclusion is keyed
                // on `travel_intent`, which is now `None` -- so it
                // "re-discovers" the agent standing in the door's own
                // triangle and restarts the *whole* pause -> wait ->
                // `Failed` cycle via the `IntraCell` destination, forever:
                // `tna status` observed alternating between `Paused` and
                // `Unreachable` on a live locked travel door instead of
                // settling at the documented deterministic terminal
                // (confirmed with a real `NavBackendPlugin` schedule, not
                // just the FSM in isolation -- see this file's
                // `locked_travel_arrival_settles_at_a_stable_unreachable_
                // terminal_not_an_oscillation` test below). A door-link
                // failure has nowhere useful left to walk regardless of
                // which destination type it failed as,
                // so clearing the target here (not just the intent) is the
                // fix: `has_target` in the mid-route check below then
                // reads false and the gate leaves a failed agent alone.
                //
                // Real-data correction (M4 wave 10 post-#153 verification):
                // `PauseAgent` -- inserted when this door-link cycle first
                // paused the agent (`is_traversing(new_state)`'s own arm
                // above is the *only* other place this component is
                // touched) -- was never removed on this `Failed` terminal.
                // `landmass` treats a `PauseAgent`-carrying entity as
                // permanently `AgentState::Paused` (skips its own
                // path/movement solving every tick, `landmass::lib::update`),
                // so a fresh `tna goto`/`tna travel` reissued after the
                // failure (e.g. once a `setlock` unblocks this exact door)
                // set a brand-new `AgentTarget3d` that `landmass` then never
                // even looked at -- confirmed live on FranklinMetro02
                // (0001a273): the agent physically froze at the door's own
                // triangle and stayed `paused` forever after `setlock
                // 0007f7e3 0` + a reissued `tna goto`, despite the door-link
                // FSM itself correctly reaching `Idle`/`Traversing` on later
                // cycles. `PauseAgent` must be removed here too, mirroring
                // the `is_traversing` arm's own removal.
                world.entity_mut(agent_entity).remove::<PauseAgent>();
                world.entity_mut(agent_entity).insert(AgentTarget3d::None);
                let mut runtime = world.get_mut::<AgentRuntime>(agent_entity).unwrap();
                runtime.active_link = None;
                runtime.travel_intent = None;
                runtime.door_link = new_state;
            } else {
                world
                    .get_mut::<AgentRuntime>(agent_entity)
                    .unwrap()
                    .door_link = new_state;
            }
        }
        DoorLinkDecision::WaitForTraversal => {
            // `door_traversal_system` owns the crossing and emits
            // `TraversalComplete` once it finishes.
        }
    }
}
