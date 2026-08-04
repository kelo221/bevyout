//! Agent ledger persistence across player-driven cell swaps.

use bevy::prelude::*;
use bevy_landmass::prelude::AgentTarget3d;

use crate::viewer::nav::agent::{
    AgentRuntime, DebugNavAgent, MAX_AGENT_INDEX, NavAgentLedger, agent_ledger_id,
};
use crate::viewer::nav::debug::{
    DebugAgentEntry, DebugAgentOrigin, DebugAgentRoster, spawn_test_agent,
};
use crate::viewer::nav::ledger_policy;
use crate::viewer::nav::world::build::ensure_archipelago;
use crate::viewer::nav::world::state::NavArchipelagoState;

/// The point-target component of `AgentTarget3d`, if any -- an `Entity`
/// target (e.g. `tna goto player`) cannot be meaningfully frozen, since the
/// target entity will not exist once the agent is restored, so it is
/// dropped rather than ledgered.
pub(crate) fn point_target(target: &AgentTarget3d) -> Option<[f32; 3]> {
    match target {
        AgentTarget3d::Point(point) => Some(point.to_array()),
        _ => None,
    }
}

/// Issue #134's player-initiated-swap ledgering (multi-agent since #114):
/// despawns every live test agent into `NavAgentLedger`, deciding
/// follow-through vs. freeze per agent via
/// `ledger_policy::decide_swap_eligibility`. Must run before
/// `teardown_archipelago` clears `NavArchipelagoState.travel_doors`, the
/// source of a follow-through's destination-door metadata.
pub(crate) fn ledger_departing_agent(world: &mut World, source_cell: u32, used_door: Option<u32>) {
    let active: Vec<(usize, DebugAgentEntry)> = world
        .resource::<DebugAgentRoster>()
        .active_entries()
        .collect();
    let mut retained_bound = Vec::new();
    for (index, entry) in active {
        match entry.origin {
            DebugAgentOrigin::SpawnedCapsule => {
                ledger_departing_one_agent(world, index, entry.entity, source_cell, used_door);
            }
            DebugAgentOrigin::BoundActor { .. } => {
                // Production actors are owned by the actor/world slices, not
                // by the disposable debug ledger. Keep the live entity and
                // its roster address; a failed handoff must never recreate it
                // as a cyan capsule.
                warn!(
                    "nav agent handoff index={index}: bound actor is not supported by debug ledger; retaining production actor"
                );
                retained_bound.push((index, entry));
            }
        }
    }
    *world.resource_mut::<DebugAgentRoster>() = DebugAgentRoster::default();
    for (index, entry) in retained_bound {
        world.resource_mut::<DebugAgentRoster>().set_with_origin(
            index,
            Some(entry.entity),
            entry.origin,
        );
    }
}

pub(crate) fn ledger_departing_one_agent(
    world: &mut World,
    index: usize,
    agent_entity: Entity,
    source_cell: u32,
    used_door: Option<u32>,
) {
    let agent_id = agent_ledger_id(index);
    let route_door = world
        .get::<AgentRuntime>(agent_entity)
        .and_then(|runtime| runtime.travel_intent)
        .map(|intent| intent.door_form_id);
    let position = world
        .get::<Transform>(agent_entity)
        .map(|transform| transform.translation.to_array())
        .unwrap_or_default();
    let remaining_target = world
        .get::<AgentTarget3d>(agent_entity)
        .and_then(point_target);

    let follow_through_link = used_door.and_then(|door| {
        let eligible = ledger_policy::decide_swap_eligibility(route_door, door)
            == ledger_policy::SwapEligibility::FollowThrough;
        eligible
            .then(|| {
                world
                    .resource::<NavArchipelagoState>()
                    .travel_doors
                    .get(&door)
                    .copied()
            })
            .flatten()
    });

    if let Some(link) = follow_through_link {
        world
            .resource_mut::<NavAgentLedger>()
            .0
            .record(ledger_policy::LedgerEntry {
                agent_id,
                cell_form_id: link.destination_cell_form_id,
                spawn_kind: ledger_policy::SpawnKind::DoorMarker {
                    destination_door_form_id: link.destination_door_form_id,
                },
                remaining_target: None,
            });
        info!(
            "nav agent handoff {agent_id:08x} -> cell {:08x}",
            link.destination_cell_form_id
        );
    } else {
        world
            .resource_mut::<NavAgentLedger>()
            .0
            .record(ledger_policy::LedgerEntry {
                agent_id,
                cell_form_id: source_cell,
                spawn_kind: ledger_policy::SpawnKind::FrozenPosition { position },
                remaining_target,
            });
        info!("nav agent freeze {agent_id:08x} cell {source_cell:08x}");
    }

    if let Ok(entity) = world.get_entity_mut(agent_entity) {
        entity.despawn();
    }
}

/// The placed position of the door reference `door_form_id` in the
/// *active* cell's manifest, if it exists there (issue #134's `DoorMarker`
/// spawn resolution).
pub(crate) fn door_position_in_active_cell(world: &World, door_form_id: u32) -> Option<Vec3> {
    world
        .get_resource::<crate::viewer::LoadedSceneManifest>()?
        .placements
        .iter()
        .find(|placement| placement.reference_form_id == door_form_id)
        .map(|placement| Vec3::from_array(placement.translation))
}

/// Issue #134's restore side: once a cell containing ledgered entries
/// becomes active, claims them (`ledger_policy::Ledger::claim_for_
/// activation`) and spawns each restored entry, or diagnoses a stale
/// `DoorMarker` entry whose destination door is missing from this cell's
/// manifest. Cheap no-op the overwhelming majority of frames: bails before
/// touching the archipelago unless the ledger actually holds an entry for
/// the active cell.
pub(crate) fn restore_ledgered_agents_system(world: &mut World) {
    let Some(current_cell) = world
        .get_resource::<crate::viewer::LoadedSceneManifest>()
        .map(|manifest| manifest.cell.form_id)
    else {
        return;
    };
    if !world
        .resource::<NavAgentLedger>()
        .0
        .has_entry_for_cell(current_cell)
    {
        return;
    }

    let known_door_form_ids: std::collections::HashSet<u32> = world
        .resource::<crate::viewer::LoadedSceneManifest>()
        .placements
        .iter()
        .filter(|placement| matches!(placement.semantic, crate::vsa::PreparedSemantic::Door(_)))
        .map(|placement| placement.reference_form_id)
        .collect();

    let claim = world
        .resource_mut::<NavAgentLedger>()
        .0
        .claim_for_activation(current_cell, &known_door_form_ids);

    for stale in claim.stale {
        warn!(
            "nav agent ledger stale {:08x} cell {:08x}: destination door {:08x} absent from the active cell",
            stale.agent_id, stale.cell_form_id, stale.missing_door_form_id
        );
    }

    for entry in claim.restored {
        // Guard both the "no id" sentinel and the dense roster's defensive
        // allocation ceiling before restoring an external ledger value.
        let Some(index) = entry
            .agent_id
            .checked_sub(1)
            .map(|zero_based| zero_based as usize)
            .filter(|index| *index <= MAX_AGENT_INDEX)
        else {
            warn!(
                "nav agent restore {:08x} cell {:08x}: agent id outside the supported roster; entry dropped",
                entry.agent_id, entry.cell_form_id
            );
            continue;
        };
        // An entry is only ever ledgered while no entity exists at that
        // index, so a live entity here means restoration already happened
        // this activation (or `tna spawn` ran first) -- do not double-spawn.
        if world.resource::<DebugAgentRoster>().is_occupied(index) {
            continue;
        }
        restore_ledgered_agent(world, index, entry);
    }
}

pub(crate) fn restore_ledgered_agent(
    world: &mut World,
    index: usize,
    entry: ledger_policy::LedgerEntry,
) {
    if ensure_archipelago(world).is_err() {
        warn!(
            "nav agent restore {:08x} cell {:08x}: no nav graph for this cell; ledger entry dropped",
            entry.agent_id, entry.cell_form_id
        );
        return;
    }
    let position = match entry.spawn_kind {
        ledger_policy::SpawnKind::FrozenPosition { position } => Vec3::from_array(position),
        ledger_policy::SpawnKind::DoorMarker {
            destination_door_form_id,
        } => match door_position_in_active_cell(world, destination_door_form_id) {
            Some(position) => position,
            None => {
                // `restore_ledgered_agents_system` already filtered stale
                // entries against `known_door_form_ids` from the same
                // manifest this reads, so this should not happen; guarded
                // defensively rather than trusted.
                warn!(
                    "nav agent restore {:08x} cell {:08x}: destination door {destination_door_form_id:08x} placement missing; ledger entry dropped",
                    entry.agent_id, entry.cell_form_id
                );
                return;
            }
        },
    };
    let agent_entity = spawn_test_agent(world, position);
    if let Some(target) = entry.remaining_target {
        world
            .entity_mut(agent_entity)
            .insert(AgentTarget3d::Point(Vec3::from_array(target)));
    }
    world
        .resource_mut::<DebugAgentRoster>()
        .set(index, Some(agent_entity));
    world
        .entity_mut(agent_entity)
        .insert(DebugNavAgent { index });
    info!(
        "nav agent restore {:08x} cell {:08x}",
        entry.agent_id, entry.cell_form_id
    );
}
