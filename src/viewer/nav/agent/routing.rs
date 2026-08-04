/// Sets `target` on `agent_entity` and resets the per-route bookkeeping: the
/// merge-portal quarantine (issue #162), the path-latency timer, and the pure
/// stuck-tracking window. The single routing seam every caller that hands an
/// agent a new destination goes through -- `goto_agent` (the `tna goto`
/// console command) and the AI package families (`ai::family_runtime`,
/// #196/#197) both call this so neither can drift into a different notion of
/// "a fresh route intent".
use super::*;
use crate::viewer::nav::doors::access::apply_door_lock_overrides;
use crate::viewer::nav::traversal::clear_merge_link_quarantine;
use crate::viewer::nav::world::state::{LinkKind, NavArchipelagoState};

pub(crate) fn route_agent_to_target(
    world: &mut World,
    agent_entity: Entity,
    target: AgentTarget3d,
) -> Result<RouteGeneration, api::NavError> {
    match target {
        AgentTarget3d::None => replace_goal(world, agent_entity, None),
        AgentTarget3d::Point(point) => {
            replace_goal(world, agent_entity, Some(api::NavGoal::Point(point)))
        }
        AgentTarget3d::Entity(target) => {
            replace_goal(world, agent_entity, Some(api::NavGoal::Entity(target)))
        }
    }
}

/// The sole route transition seam. It validates the complete actor and goal
/// before mutating anything, invalidates all state owned by the previous goal,
/// then initializes the new target and its generation through one path.
pub(crate) fn replace_goal(
    world: &mut World,
    agent_entity: Entity,
    goal: Option<api::NavGoal>,
) -> Result<RouteGeneration, api::NavError> {
    if world.get_entity(agent_entity).is_err() {
        return Err(api::NavError::ActorUnavailable(agent_entity));
    }
    if world.get::<NavAgent>(agent_entity).is_none()
        || world.get::<AgentRuntime>(agent_entity).is_none()
        || world.get::<AgentKcc>(agent_entity).is_none()
    {
        return Err(api::NavError::ActorNotBound(agent_entity));
    }
    if world.get_resource::<NavArchipelagoState>().is_none() {
        return Err(api::NavError::WorldUnavailable);
    }

    let resolved_goal = match goal {
        Some(api::NavGoal::Entity(target)) => {
            if world.get_entity(target).is_err() {
                return Err(api::NavError::TargetUnavailable(target));
            }
            Some((AgentTarget3d::Entity(target), None))
        }
        Some(api::NavGoal::Point(point)) => Some((AgentTarget3d::Point(point), None)),
        Some(api::NavGoal::TravelDoor(door)) => {
            let link = super::super::doors::travel::resolve_travel_door(world, door.into())?;
            Some((
                AgentTarget3d::Point(link.triangle_midpoint),
                Some(door.into()),
            ))
        }
        None => None,
    };

    let generation = {
        let mut runtime = world
            .get_mut::<AgentRuntime>(agent_entity)
            .expect("validated AgentRuntime exists");
        runtime.route_generation =
            RouteGeneration(runtime.route_generation.0.wrapping_add(1).max(1));
        let generation = runtime.route_generation;
        runtime.door_link = door_link::DoorLinkState::Idle;
        runtime.pending_traversal = None;
        runtime.active_link = None;
        runtime.travel_intent = None;
        runtime.goto_started_at = None;
        runtime.latency_logged = false;
        generation
    };

    // These components are the physical and Landmass sides of an active
    // crossing. Remove them immediately so a replacement cannot be consumed
    // by a later traversal system in the same frame.
    world
        .entity_mut(agent_entity)
        .remove::<PauseAgent>()
        .remove::<DoorTraversal>()
        .remove::<MergeTraversal>()
        .remove::<PendingMergeRepath>()
        .remove::<SuspendedLandmassTypeIndexCosts>()
        .remove::<UsingAnimationLink>()
        .remove::<ReachedAnimationLink3d>()
        .remove::<RefreshLandmassAnimationLinkInput>()
        .insert(AgentTarget3d::None);

    clear_merge_link_quarantine(world, agent_entity);
    if let Some(mut kcc) = world.get_mut::<AgentKcc>(agent_entity) {
        kcc.best_distance = f32::MAX;
        kcc.ticks_without_progress = 0;
        kcc.recovery_active = false;
        kcc.stuck = false;
        kcc.route_progress = 0.0;
    }

    let Some((target, travel_door)) = resolved_goal else {
        return Ok(generation);
    };

    // All goal kinds use this common initialization path. Travel-door
    // resolution only changes the target and intent attached after it.
    world.entity_mut(agent_entity).insert(target);
    apply_door_lock_overrides(world, agent_entity);
    let elapsed = world.get_resource::<Time>().map_or(0.0, Time::elapsed_secs);
    if let Some(mut runtime) = world.get_mut::<AgentRuntime>(agent_entity) {
        runtime.goto_started_at = Some(elapsed);
        runtime.latency_logged = false;
        runtime.travel_intent = travel_door.map(|door_form_id| TravelIntent {
            generation,
            door_form_id,
        });
    }
    Ok(generation)
}

/// Whether `entity` currently owns a nav agent (`tna bind`/`tna spawn`
/// inserted the `AgentKcc`). The AI package families route only nav-bound
/// actors; the `runpackage` console command checks this before starting one.
pub(crate) fn is_nav_bound(world: &World, entity: Entity) -> bool {
    world.get::<NavAgent>(entity).is_some()
        && world.get::<AgentRuntime>(entity).is_some()
        && world.get::<AgentKcc>(entity).is_some()
}

/// Projects the current Landmass/door-link state into the navigation API's
/// backend-independent observation. Door-link state takes precedence over the
/// backend state because it owns waiting, traversal, travel arrival, and the
/// named blocked-door failure terminal.
pub(crate) fn nav_observation(world: &World, agent_entity: Entity) -> api::NavObservation {
    if world.get::<AgentKcc>(agent_entity).is_none() {
        return api::NavObservation {
            status: api::NavStatus::Failed(api::NavFailureReason::WorldUnavailable),
            generation: RouteGeneration::default(),
        };
    }

    let (door_state, generation) = world
        .get::<AgentRuntime>(agent_entity)
        .map(|runtime| (runtime.door_link, runtime.route_generation))
        .unwrap_or((door_link::DoorLinkState::Idle, RouteGeneration::default()));
    let status = match door_state {
        door_link::DoorLinkState::Paused { door_form_id, .. } => {
            api::NavStatus::WaitingForDoor(door_form_id.into())
        }
        door_link::DoorLinkState::Traversing { door_form_id, .. } => {
            api::NavStatus::TraversingDoor(door_form_id.into())
        }
        door_link::DoorLinkState::TravelReached { door_form_id, .. } => {
            api::NavStatus::TravelReady(door_form_id.into())
        }
        door_link::DoorLinkState::Failed { door_form_id } => {
            api::NavStatus::Failed(api::NavFailureReason::BlockedDoor(door_form_id.into()))
        }
        door_link::DoorLinkState::Idle => match world.get::<AgentState>(agent_entity).copied() {
            Some(AgentState::Idle) => api::NavStatus::Idle,
            Some(AgentState::ReachedTarget) => api::NavStatus::Reached,
            Some(AgentState::NoPath) => api::NavStatus::Failed(api::NavFailureReason::NoPath),
            Some(AgentState::AgentNotOnNavMesh) => {
                api::NavStatus::Failed(api::NavFailureReason::AgentOffNavmesh)
            }
            Some(AgentState::TargetNotOnNavMesh) => {
                api::NavStatus::Failed(api::NavFailureReason::TargetOffNavmesh)
            }
            Some(
                AgentState::Moving
                | AgentState::ReachedAnimationLink
                | AgentState::UsingAnimationLink
                | AgentState::Paused,
            )
            | None => api::NavStatus::Routing,
        },
    };
    api::NavObservation { status, generation }
}

/// Test-only: seeds the empty [`NavArchipelagoState`] a minimal-`World` needs
/// before [`route_agent_to_target`] (its `apply_door_lock_overrides` reads it).
/// Lets the AI-family adapter tests exercise the real routing seam without
/// standing up the whole `NavBackendPlugin`.
#[cfg(test)]
pub(crate) fn insert_test_archipelago_state(world: &mut World) {
    world.init_resource::<NavArchipelagoState>();
}

/// Test-only cross-module support (issue #218's autonomous package driver
/// tests): marks a `NavArchipelagoState` already current for `cell_form_id`
/// with `archipelago` as its entity, so [`ensure_archipelago`]'s
/// already-current check short-circuits without a real nav-graph file --
/// exactly [`bind_agent_entity`]'s own test harness, exposed for other
/// modules' tests since neither `NavArchipelagoState` nor its fields are
/// nameable outside this module.
#[cfg(test)]
pub(crate) fn mark_test_archipelago_current(
    world: &mut World,
    cell_form_id: u32,
    archipelago: Entity,
) {
    let mut state = world.get_resource_or_insert_with(NavArchipelagoState::default);
    state.cell_form_id = Some(cell_form_id);
    state.archipelago = Some(archipelago);
}

pub(crate) fn resolve_status(
    landmass_state: AgentState,
    door_link_state: door_link::DoorLinkState,
) -> landmass_graph::NavAgentStatus {
    if door_link::is_travel_reached(door_link_state) {
        return landmass_graph::NavAgentStatus::TravelReached;
    }
    // Issue #155 feature 4: a door lifecycle that gave up waiting
    // (`MAX_WAIT_TICKS` exhausted, whether from a locked mid-route crossing
    // or a two-sided link) is a distinct, terminal failure -- not the same
    // "temporarily waiting" status a `Paused` agent still recovering from is
    // in. `Unreachable` (not a brand-new variant) is deliberate: it is the
    // exact word both call sites' `nav agent unreachable` log line already
    // uses (AGENTS.md: stable log wording), so `tna status`/the HUD finally
    // agree with the log instead of contradicting it.
    if door_link::is_failed(door_link_state) {
        return landmass_graph::NavAgentStatus::Unreachable;
    }
    if door_link::is_paused(door_link_state) {
        return landmass_graph::NavAgentStatus::Paused;
    }
    landmass_graph::map_agent_state(landmass_state)
}

/// The `link=` suffix for `tna status` (issue #113 feature 5): the active
/// link kind while interacting with one (`merge` while crossing a merge
/// seam, `door <formid>` through a door lifecycle), else `None`.
pub(crate) fn active_link_description(runtime: &AgentRuntime) -> Option<String> {
    match runtime.active_link {
        Some(LinkKind::Merge { .. }) => Some("merge".to_string()),
        Some(LinkKind::Door { form_id }) => Some(format!("door {form_id:08x}")),
        None => match runtime.door_link {
            door_link::DoorLinkState::TravelReached {
                door_form_id,
                destination_cell_form_id,
            } => Some(format!(
                "door {door_form_id:08x} cell {destination_cell_form_id:08x}"
            )),
            _ => None,
        },
    }
}

pub(crate) fn describe_target(target: &AgentTarget3d) -> String {
    match target {
        AgentTarget3d::None => "none".to_string(),
        AgentTarget3d::Point(point) => format!("({:.2}, {:.2}, {:.2})", point.x, point.y, point.z),
        AgentTarget3d::Entity(entity) => format!("entity:{entity:?}"),
    }
}

// ---------------------------------------------------------------------
// Runtime systems
// ---------------------------------------------------------------------
