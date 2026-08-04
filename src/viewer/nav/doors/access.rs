use std::collections::HashMap;

use bevy::prelude::*;
use bevy_landmass::AgentTypeIndexCostOverrides;

use crate::viewer::interaction;
use crate::viewer::nav::agent::{CLOSED_DOOR_TYPE_INDEX_COST, LOCKED_DOOR_TYPE_INDEX_COST};
use crate::viewer::nav::world::state::{DoorLockInfo, NavArchipelagoState};
use crate::viewer::nav::{openmw_doors, repath};

/// Whether `agent_entity` currently holds an item stack whose base FormID is
/// `form_id`, in its own canonical inventory (issue #185).
///
/// A bound actor's canonical holder is always `HolderId::Actor {
/// reference_form_id }` (`viewer::actor::project_prepared_actors`), so this
/// resolves straight from the agent entity's own `ActorRuntime` -- an
/// unbound `tna spawn` debug capsule (no `ActorRuntime` at all) has no
/// inventory of its own and therefore never holds any key, which is the
/// same conservative default `door_open_and_locked` fell back to pre-#185
/// when no inventory resource was available.
pub(crate) fn agent_holds_item(world: &World, agent_entity: Entity, form_id: u32) -> bool {
    let Some(actor) = world.get::<crate::viewer::actor::ActorRuntime>(agent_entity) else {
        return false;
    };
    let holder = bevyout_core::item_transaction::HolderId::Actor {
        reference_form_id: actor.reference_form_id,
    };
    world
        .get_resource::<interaction::CanonicalItemLedger>()
        .and_then(|ledger| ledger.ledger.holders().get(&holder))
        .is_some_and(|state| state.items.iter().any(|item| item.base_form_id == form_id))
}

/// The live `(open, locked)` observation for `door_form_id`, for the
/// specific `agent` asking (issue #185: locked-with-a-key is a fact about
/// the *pair* of door and actor, not the door alone -- mirroring OpenMW's
/// `AiPackage::openDoors()`, which searches the routing actor's own
/// inventory, never the player's). `open` reads the runtime
/// `InteractionState.open` set (guarded on `RefRegistry` being present --
/// `resolve_reference` panics without one, which minimal test worlds may
/// not have); `locked` runs `openmw_doors::door_openable` against the
/// door's prepared lock/key/trap data and whether `agent` (when given) holds
/// the key. `agent: None` is the conservative, actor-independent baseline
/// `ensure_archipelago`'s initial build and `door_availability_system`'s
/// change-detection poll use -- no specific actor to check a key against,
/// so a keyed lock is never lifted. A door with no prepared lock info is
/// never locked.
pub(crate) fn door_open_and_locked(
    world: &World,
    agent: Option<Entity>,
    door_form_id: u32,
    door_lock_info: &HashMap<u32, DoorLockInfo>,
) -> (bool, bool) {
    let open = world
        .get_resource::<crate::console::RefRegistry>()
        .is_some()
        && crate::console::resolve_reference(world, &format!("{door_form_id:08x}"))
            .ok()
            .is_some_and(|entity| {
                world
                    .get_resource::<interaction::InteractionState>()
                    .is_some_and(|state| state.open.contains(&entity))
            });
    let locked = door_lock_info.get(&door_form_id).is_some_and(|info| {
        let holder_has_key = info.key_form_id.is_some_and(|key_form_id| {
            agent.is_some_and(|agent| agent_holds_item(world, agent, key_form_id))
        });
        !openmw_doors::door_openable(openmw_doors::DoorAccessObservation {
            lock_level: info.lock_level,
            trapped: info.trapped,
            key_form_id: info.key_form_id,
            holder_has_key,
        })
    });
    (open, locked)
}

/// Whether `door_form_id` is currently usable for route planning, for the
/// specific `agent` asking (see [`door_open_and_locked`]): already open, or
/// not locked (`repath::door_usable`'s rule). A door with no prepared lock
/// info is usable.
pub(crate) fn door_usable_now(
    world: &World,
    agent: Option<Entity>,
    door_form_id: u32,
    door_lock_info: &HashMap<u32, DoorLockInfo>,
) -> bool {
    let (open, locked) = door_open_and_locked(world, agent, door_form_id, door_lock_info);
    repath::door_usable(repath::DoorObservation { locked, open })
}

/// Rebuilds `agent_entity`'s `AgentTypeIndexCostOverrides` wholesale from
/// `NavArchipelagoState::door_usable`/`door_type_indices` (issue #155
/// feature 2): every currently-unusable (locked, closed) door with a
/// resolved type index gets [`LOCKED_DOOR_TYPE_INDEX_COST`]; everything
/// else -- including a door that *was* locked and just became usable --
/// gets no entry. Replacing the whole component rather than patching it is
/// deliberate: `bevy_landmass::AgentTypeIndexCostOverrides` only exposes
/// `set_type_index_cost` (insert/overwrite) publicly, with no matching
/// "remove one override" call, so the only way to actually *clear* a
/// stale locked-door entry from outside `bevy_landmass` is to insert a
/// fresh component that never had it. `bevy_landmass`'s own sync system
/// only re-applies this component to the underlying `landmass::Agent` when
/// it is `Changed<_>` -- inserting a fresh value every call always
/// satisfies that, so this is safe to call unconditionally (at spawn) or
/// on every door-usability flip (`door_availability_system`) without
/// needing its own separate change-tracking.
///
/// Called both at spawn time (`spawn_test_agent`, so a freshly spawned
/// agent's very first path query already respects whatever is locked) and
/// on every `door_availability_system` flip (so an agent that is idle, or
/// already `NoPath`/`Unreachable`, picks up the change on its next solve --
/// `landmass`'s own `does_agent_need_repath` retries every tick whenever
/// `current_path` is `None`, with no explicit retarget needed for that
/// case). An agent already mid-transit through a door at the exact instant
/// it locks keeps following its already-computed, structurally still-valid
/// path until its next genuine repath -- see this file's module doc
/// comment for why that narrower case is a documented scope cut rather
/// than fixed here.
pub(crate) fn apply_door_lock_overrides(world: &mut World, agent_entity: Entity) {
    let mut overrides = AgentTypeIndexCostOverrides::default();
    // Issue #185: cloned out up front (rather than held as a live borrow of
    // `NavArchipelagoState` for the whole function) so the per-agent
    // re-checks below (`door_usable_now`, which needs `&World`) are free to
    // read `world` without fighting this borrow.
    // No archipelago means no prepared door graph (e.g. a minimal-`App` route
    // test, or a cell with no navmesh yet): there is nothing to override, so
    // leave the agent's costs untouched rather than panicking on a missing
    // resource.
    let Some((
        door_usable,
        door_type_indices,
        closed_door_type_indices,
        openable_blockers,
        door_open,
        door_lock_info,
    )) = world.get_resource::<NavArchipelagoState>().map(|state| {
        (
            state.door_usable.clone(),
            state.door_type_indices.clone(),
            state.closed_door_type_indices.clone(),
            state.openable_blockers.clone(),
            state.door_open.clone(),
            state.door_lock_info.clone(),
        )
    })
    else {
        return;
    };
    // Issue #185, the main gap: the shared cache above answers "usable by an
    // actor with no particular key" (see `door_open_and_locked`'s `agent:
    // None` case) -- but OpenMW's `AiPackage::openDoors()` tries the routing
    // actor's *own* inventory before giving up, so a door the shared cache
    // calls unusable may still be usable for THIS agent specifically. Only
    // worth re-checking when the shared default already says no; if it is
    // already usable for nobody-in-particular, it is usable for everyone.
    //
    // Deliberately consults `door_lock_info` directly (`openmw_doors::
    // door_openable`) rather than the more general `door_usable_now`/
    // `door_open_and_locked`: those treat "no prepared lock info at all" as
    // "never locked" (harmlessly true for a real `PreparedSemantic::Door`
    // with no `XLOC`, but this same `door_usable` cache is shared with the
    // #177 derived-blocker class below, e.g. a vault gear activator, which
    // is closed/open-gated with no lock or key concept and so *never* has a
    // `door_lock_info` entry). Re-deriving "usable" for those from an absent
    // entry would spuriously read as "openable" and lift an override that
    // has nothing to do with a key. A door with no lock info therefore
    // simply is not re-examined here at all -- the shared cache's `false`
    // stands, exactly as it did before this issue.
    let agent_may_open_with_key = |world: &World, door_form_id: u32| -> bool {
        let Some(&info) = door_lock_info.get(&door_form_id) else {
            return false;
        };
        let holder_has_key = info
            .key_form_id
            .is_some_and(|key_form_id| agent_holds_item(world, agent_entity, key_form_id));
        openmw_doors::door_openable(openmw_doors::DoorAccessObservation {
            lock_level: info.lock_level,
            trapped: info.trapped,
            key_form_id: info.key_form_id,
            holder_has_key,
        })
    };
    for (&door_form_id, &usable) in &door_usable {
        if usable || agent_may_open_with_key(world, door_form_id) {
            continue;
        }
        if let Some(&type_index) = door_type_indices.get(&door_form_id) {
            overrides.set_type_index_cost(type_index, LOCKED_DOOR_TYPE_INDEX_COST);
        }
    }
    // Issue #177: a blocker's *interior* polygons (derived associations that
    // lie wholly inside its collision volume) are never freely traversable
    // while it is closed -- pricing them only on `door_usable` (lock) is what
    // let an agent plan straight through a shut door and wedge against it.
    //
    // How expensive depends on whether the blocker can be opened at all,
    // which is the correction the first cut of this issue needed:
    //
    // - **Openable and unlocked** -> [`CLOSED_DOOR_TYPE_INDEX_COST`], a
    //   strong but finite penalty. The route stays plannable, the agent walks
    //   to the doorway, and the mid-route crossing gate runs the existing
    //   pause -> request open -> wait -> traverse -> resume lifecycle. An
    //   unbounded cost here would stop the agent ever reaching the door it is
    //   supposed to open.
    // - **Locked, or not openable at all** (the ungated kinematic-activator
    //   class, e.g. a vault gear door with no open/close FSM) ->
    //   [`LOCKED_DOOR_TYPE_INDEX_COST`]. There is no sanctioned crossing, so
    //   the route must fail fast rather than walk the agent into a solid.
    //
    // Opening the blocker clears the entry entirely, through the same
    // rebuild-the-whole-component path a lock change takes.
    for (&blocker_form_id, &type_index) in &closed_door_type_indices {
        if door_open.get(&blocker_form_id).copied().unwrap_or(false) {
            continue;
        }
        let openable = openable_blockers.contains(&blocker_form_id);
        // Issue #185: same per-agent key exception as the loop above --
        // `usable` here means "not locked", the identical fact
        // `agent_may_open_with_key` re-derives for this specific agent (a
        // no-op for a lock-less activator blocker, which has no
        // `door_lock_info` entry to re-check in the first place).
        let usable = door_usable.get(&blocker_form_id).copied().unwrap_or(true)
            || agent_may_open_with_key(world, blocker_form_id);
        let cost = if openable && usable {
            CLOSED_DOOR_TYPE_INDEX_COST
        } else {
            LOCKED_DOOR_TYPE_INDEX_COST
        };
        overrides.set_type_index_cost(type_index, cost);
    }
    if let Ok(mut entity) = world.get_entity_mut(agent_entity) {
        entity.insert(overrides);
    }
}

/// Issue #163 (`setlock`): the narrow external mutation point for a door's
/// prepared lock level, callable from `console::world_commands` without
/// exposing `NavArchipelagoState` itself. Inserts/replaces the door's
/// `door_lock_info` entry -- the exact shape `ensure_archipelago` populates
/// from the manifest above -- preserving whatever `key_form_id` was already
/// recorded (a runtime lock change never invents a new key requirement). A
/// missing resource (no archipelago built yet for this cell, or a console
/// harness without the nav plugin) is a no-op: there is no `door_usable`
/// entry to flip either in that case, and the interaction-side write in the
/// same console command is still the ultimate consistent state for anything
/// reading only `PlacementRoot`. Once the resource exists,
/// `door_availability_system`'s next poll (`door_usable_now` ->
/// `door_open_and_locked`) reads this updated map and treats a runtime lock
/// exactly like an authored one -- no separate repath plumbing needed.
pub(crate) fn set_door_lock_level(world: &mut World, door_form_id: u32, lock_level: Option<i8>) {
    let Some(mut state) = world.get_resource_mut::<NavArchipelagoState>() else {
        return;
    };
    let (key_form_id, trapped) = state
        .door_lock_info
        .get(&door_form_id)
        .map(|info| (info.key_form_id, info.trapped))
        .unwrap_or_default();
    state.door_lock_info.insert(
        door_form_id,
        DoorLockInfo {
            lock_level,
            key_form_id,
            trapped,
        },
    );
}

/// Issue #185: the nav-side mirror of [`set_door_lock_level`] for a door's
/// key requirement -- `console::world_commands::setlock`'s optional key
/// argument writes both the interaction-side `PlacementRoot` (the player's
/// own activation check) and this `door_lock_info` entry (nav route
/// planning/door-open requests) the same way a lock-level change already
/// does. Preserves whatever `lock_level`/`trapped` was already recorded.
pub(crate) fn set_door_key_form_id(world: &mut World, door_form_id: u32, key_form_id: Option<u32>) {
    let Some(mut state) = world.get_resource_mut::<NavArchipelagoState>() else {
        return;
    };
    let (lock_level, trapped) = state
        .door_lock_info
        .get(&door_form_id)
        .map(|info| (info.lock_level, info.trapped))
        .unwrap_or_default();
    state.door_lock_info.insert(
        door_form_id,
        DoorLockInfo {
            lock_level,
            key_form_id,
            trapped,
        },
    );
}

/// Test-only support for `console::world_commands`'s `setlock` tests, which
/// run in the lighter console harness (`test_app` in
/// `console::tests`) that never builds a real archipelago. Neither
/// `NavArchipelagoState` nor `DoorLockInfo` is nameable outside this module,
/// so a console test cannot construct or inspect them directly.
#[cfg(test)]
pub(crate) fn init_test_archipelago_state(world: &mut World) {
    world.init_resource::<NavArchipelagoState>();
}

/// Test-only companion to [`init_test_archipelago_state`]: the locked-level
/// currently recorded for `door_form_id`, or `None` if it is absent or
/// recorded unlocked -- either way, "not locked" for route planning.
#[cfg(test)]
pub(crate) fn door_lock_level_for_test(world: &World, door_form_id: u32) -> Option<i8> {
    world
        .get_resource::<NavArchipelagoState>()
        .and_then(|state| state.door_lock_info.get(&door_form_id))
        .and_then(|info| info.lock_level)
}
