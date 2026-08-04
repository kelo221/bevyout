//! Cell lifecycle boundary for stale navigation worlds.

use bevy::prelude::*;

use crate::viewer::nav::agent::*;
use crate::viewer::nav::world::exterior::exterior_resident_grid_signature;

pub(crate) fn note_player_swap_door(world: &mut World, door_form_id: u32) {
    world.resource_mut::<PendingPlayerSwapDoor>().0 = Some(door_form_id);
}

/// Called from `world::swap::activate_resident_cell` (issue #134) once a
/// player-initiated cell swap is definitely proceeding: records which
/// origin door reference the player used so the next
/// `despawn_stale_navmesh_archipelago` pass (whichever frame it detects
/// the resulting stale archipelago) can decide follow-through vs. freeze
/// for any live nav agent still in the departing cell.
/// Mirrors `nav_overlay::despawn_stale_nav_overlay`'s pattern: the moment
/// the active cell no longer matches the archipelago's cell, tear it down.
/// Exterior navigation has one additional ownership boundary: a changed
/// collision-ready resident set invalidates the old archipelago's tile/link
/// set even while the active manifest remains the same. Rebuild that set at
/// the lifecycle boundary, before a later solve can observe an evicted tile.
/// `PreparedSceneManifest` is optional so this system never panics in a
/// console-harness test world that never inserted one.
///
/// Issue #134 shipped amendment: wave 3's original behaviour despawned any
/// live test agent along with the stale archipelago, silently losing it.
/// That is replaced with `ledger_departing_agent`, which runs *before*
/// `teardown_archipelago` (it needs `NavArchipelagoState.travel_doors`,
/// still populated for the departing cell) and ledgers the agent instead --
/// follow-through to the destination cell if `note_player_swap_door` noted
/// the exact door the agent's active route was targeting, otherwise frozen
/// in place in the departing cell.
/// Logs the stable evidence lines exactly once per actual state change, for
/// every currently-spawned agent. `bevy_landmass`'s `sync_agent_state`
/// rewrites `AgentState` every frame (Bevy change detection triggers on the
/// write, not the value), so a `Changed<AgentState>` filter would re-log
/// every frame; the previous value is tracked per agent in `AgentRuntime`
/// instead. `AgentState::AgentNotOnNavMesh` is the off-navmesh diagnostic
/// now that physics (#114), not navmesh sampling, is ground authority --
/// its own stable `nav agent off-navmesh <id>` line.
///
/// Issue #241: an agent with no roster index used to be `continue`d over
/// here, so an autonomously-bound actor never reported `reached`,
/// `off-navmesh` or `unreachable` -- see [`format_agent_id`].
pub(crate) fn despawn_stale_navmesh_archipelago(world: &mut World) {
    let Some(manifest) = world.get_resource::<crate::viewer::LoadedSceneManifest>() else {
        return;
    };
    let current_cell = manifest.cell.form_id;
    let exterior_mode = manifest.exterior.is_some();
    let resident_signature = exterior_mode.then(|| exterior_resident_grid_signature(world));
    let (source_cell, exterior_resident_set_changed) = {
        let state = world.resource::<NavArchipelagoState>();
        let source_cell = state.cell_form_id.filter(|&cell| cell != current_cell);
        let exterior_resident_set_changed = exterior_mode
            && state.archipelago.is_some()
            && state.cell_form_id == Some(current_cell)
            && resident_signature
                .as_ref()
                .is_some_and(|signature| signature != &state.exterior_resident_grids);
        (source_cell, exterior_resident_set_changed)
    };
    if source_cell.is_none() && !exterior_resident_set_changed {
        return;
    }
    if let Some(source_cell) = source_cell {
        let used_door = world.resource_mut::<PendingPlayerSwapDoor>().0.take();
        ledger_departing_agent(world, source_cell, used_door);
    } else {
        info!("exterior nav resident set changed; rebuilding owned islands and border links");
    }
    teardown_archipelago(world);
    // Cell swaps restore ledgered agents in the following system. A resident
    // set change keeps the current entities in place, so rebuild immediately
    // and retarget their `ArchipelagoRef3d` before the next landmass solve.
    if exterior_resident_set_changed && let Err(error) = ensure_archipelago(world) {
        warn!(
            "exterior nav resident-set rebuild failed: {}",
            error.message
        );
    }
}
