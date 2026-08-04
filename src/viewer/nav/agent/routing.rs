use super::actor_binding;
/// Sets `target` on `agent_entity` and resets the per-route bookkeeping: the
/// merge-portal quarantine (issue #162), the path-latency timer, and the pure
/// stuck-tracking window. The single routing seam every caller that hands an
/// agent a new destination goes through -- `goto_agent` (the `tna goto`
/// console command) and the AI package families (`ai::family_runtime`,
/// #196/#197) both call this so neither can drift into a different notion of
/// "a fresh route intent".
use super::*;

pub(crate) fn route_agent_to_target(
    world: &mut World,
    agent_entity: Entity,
    target: AgentTarget3d,
) {
    world.entity_mut(agent_entity).insert(target);
    // Issue #162 feature 2: a fresh target is a new routing intent -- any
    // merge-portal quarantine from a previous route no longer applies.
    clear_merge_link_quarantine(world, agent_entity);
    // Issue #185: a fresh target is also a fresh chance for this agent's
    // key-aware door-lock overrides to reflect whatever the agent is
    // holding right now (e.g. a key granted since the last route) rather
    // than whatever was last computed at spawn or the last door flip.
    apply_door_lock_overrides(world, agent_entity);
    let elapsed = world.resource::<Time>().elapsed_secs();
    if let Some(mut runtime) = world.get_mut::<AgentRuntime>(agent_entity) {
        runtime.goto_started_at = Some(elapsed);
        runtime.latency_logged = false;
    }
    // A fresh target resets the pure stuck-tracking window (movement_policy)
    // -- the agent gets a clean run at the new waypoint.
    if let Some(mut kcc) = world.get_mut::<AgentKcc>(agent_entity) {
        kcc.best_distance = f32::MAX;
        kcc.ticks_without_progress = 0;
        kcc.recovery_active = false;
        kcc.stuck = false;
    }
}

/// Routes `agent_entity` to a fixed world point (the AI package families'
/// `FamilyRequest::Route`). Thin wrapper over [`route_agent_to_target`].
pub(crate) fn route_agent_to_point(world: &mut World, agent_entity: Entity, point: Vec3) {
    route_agent_to_target(world, agent_entity, AgentTarget3d::Point(point));
}

/// Routes `agent_entity` to another entity's current position. This is the
/// entity-target half of the navigation façade; the Landmass target component
/// stays private to the runtime adapter.
pub(crate) fn route_agent_to_entity(world: &mut World, agent_entity: Entity, target: Entity) {
    route_agent_to_target(world, agent_entity, AgentTarget3d::Entity(target));
}

/// Clears any nav route on `agent_entity` (the families' `FamilyRequest::Stop`
/// on travel arrival / package completion), so landmass stops steering it and
/// the locomotion policy idles the now-stationary actor.
pub(crate) fn clear_agent_target(world: &mut World, agent_entity: Entity) {
    if let Ok(mut entity) = world.get_entity_mut(agent_entity) {
        entity.insert(AgentTarget3d::None);
    }
}

/// Whether `entity` currently owns a nav agent (`tna bind`/`tna spawn`
/// inserted the `AgentKcc`). The AI package families route only nav-bound
/// actors; the `runpackage` console command checks this before starting one.
pub(crate) fn is_nav_bound(world: &World, entity: Entity) -> bool {
    world.get::<AgentKcc>(entity).is_some()
}

/// Projects the current Landmass/door-link state into the navigation API's
/// backend-independent observation. Door-link state takes precedence over the
/// backend state because it owns waiting, traversal, travel arrival, and the
/// named blocked-door failure terminal.
pub(crate) fn nav_observation(world: &World, agent_entity: Entity) -> api::NavObservation {
    if world.get::<AgentKcc>(agent_entity).is_none() {
        return api::NavObservation {
            status: api::NavStatus::Failed(api::NavFailureReason::WorldUnavailable),
        };
    }

    let door_state = world
        .get::<AgentRuntime>(agent_entity)
        .map(|runtime| runtime.door_link);
    let status = match door_state {
        Some(door_link::DoorLinkState::Paused { door_form_id, .. }) => {
            api::NavStatus::WaitingForDoor(door_form_id.into())
        }
        Some(door_link::DoorLinkState::Traversing { door_form_id, .. }) => {
            api::NavStatus::TraversingDoor(door_form_id.into())
        }
        Some(door_link::DoorLinkState::TravelReached { door_form_id, .. }) => {
            api::NavStatus::TravelReady(door_form_id.into())
        }
        Some(door_link::DoorLinkState::Failed { door_form_id }) => {
            api::NavStatus::Failed(api::NavFailureReason::BlockedDoor(door_form_id.into()))
        }
        Some(door_link::DoorLinkState::Idle) | None => {
            match world.get::<AgentState>(agent_entity).copied() {
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
            }
        }
    };
    api::NavObservation { status }
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
