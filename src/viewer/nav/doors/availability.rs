use bevy::prelude::*;
use bevy_landmass::prelude::*;

use crate::viewer::nav::agent::AgentRuntime;
use crate::viewer::nav::diagnostics::all_agent_entities;
use crate::viewer::nav::doors::runtime::request_door_open;
use crate::viewer::nav::world::links::spawn_link_pair;
use crate::viewer::nav::world::state::{BlockedDoorLink, LinkKind, NavArchipelagoState};
use crate::viewer::nav::{door_link, repath};

use crate::viewer::nav::doors::access::*;

/// Polls every tracked door's usability once per frame and reacts to
/// *changes* only (issue #113 feature 4): the pure `repath::decide` table
/// turns a flip into a repath, applied here as (a) spawning/despawning the
/// affected two-sided door link so route planning includes/excludes it, (b)
/// re-inserting the agent's current target so landmass replans, and (c)
/// while paused at that very door, requesting the (now unlocked) door open.
/// Exactly one repath per actual state change -- the cached `door_usable`
/// map is the change detector.
pub(crate) fn door_availability_system(world: &mut World) {
    let tracked: Vec<(u32, bool)> = world
        .resource::<NavArchipelagoState>()
        .door_usable
        .iter()
        .map(|(&form_id, &usable)| (form_id, usable))
        .collect();
    if tracked.is_empty() {
        return;
    }
    let lock_info = world
        .resource::<NavArchipelagoState>()
        .door_lock_info
        .clone();

    // Issue #177: open-state poll, tracked separately from usability. An
    // unlocked door that merely opens or shuts never flips `door_usable`, so
    // the loop below would miss it entirely -- but it is exactly the state
    // the closed-blocker cost override keys on. Flips rebuild every active
    // agent's overrides and re-insert its target, the same one-repath-per-
    // flip shape the usability loop uses.
    let open_flips: Vec<(u32, bool)> = tracked
        .iter()
        .filter_map(|&(door_form_id, _)| {
            let (open, _) = door_open_and_locked(world, None, door_form_id, &lock_info);
            let was_open = world
                .resource::<NavArchipelagoState>()
                .door_open
                .get(&door_form_id)
                .copied();
            (was_open != Some(open)).then_some((door_form_id, open))
        })
        .collect();
    if !open_flips.is_empty() {
        for (door_form_id, open) in &open_flips {
            world
                .resource_mut::<NavArchipelagoState>()
                .door_open
                .insert(*door_form_id, *open);
        }
        // Issue #241: every agent, not just roster-indexed ones -- an
        // autonomously-bound actor's route must replan on a door open/close
        // flip exactly like a `tna`-driven one.
        let active_agents = all_agent_entities(world);
        for agent_entity in &active_agents {
            apply_door_lock_overrides(world, *agent_entity);
            let target =
                world
                    .get::<AgentTarget3d>(*agent_entity)
                    .and_then(|target| match target {
                        AgentTarget3d::None => None,
                        AgentTarget3d::Point(point) => Some(AgentTarget3d::Point(*point)),
                        AgentTarget3d::Entity(entity) => Some(AgentTarget3d::Entity(*entity)),
                    });
            if let Some(target) = target {
                world.entity_mut(*agent_entity).insert(target);
            }
        }
    }

    for (door_form_id, was_usable) in tracked {
        let now_usable = door_usable_now(world, None, door_form_id, &lock_info);
        if now_usable == was_usable {
            continue;
        }
        world
            .resource_mut::<NavArchipelagoState>()
            .door_usable
            .insert(door_form_id, now_usable);

        let observation = repath::RepathObservation {
            door_became_blocked: !now_usable,
            door_became_unblocked: now_usable,
            ..Default::default()
        };
        if repath::decide(observation) != repath::RepathDecision::Repath {
            continue;
        }

        // Structural link update for two-sided door links.
        if now_usable {
            let blocked = {
                let mut state = world.resource_mut::<NavArchipelagoState>();
                let index = state
                    .blocked_door_links
                    .iter()
                    .position(|link| link.door_form_id == door_form_id);
                index.map(|index| state.blocked_door_links.remove(index))
            };
            if let Some(link) = blocked {
                let archipelago_entity = world
                    .resource::<NavArchipelagoState>()
                    .archipelago
                    .expect("availability tracking implies a built archipelago");
                for link_entity in
                    spawn_link_pair(world, archipelago_entity, link.start, link.end, 1.0, 0)
                {
                    let mut state = world.resource_mut::<NavArchipelagoState>();
                    state.link_kinds.insert(
                        link_entity,
                        LinkKind::Door {
                            form_id: door_form_id,
                        },
                    );
                    state.links.push(link_entity);
                }
            }
        } else {
            let removed: Vec<Entity> = {
                let state = world.resource::<NavArchipelagoState>();
                state
                    .link_kinds
                    .iter()
                    .filter(|(_, kind)| {
                        matches!(kind, LinkKind::Door { form_id } if *form_id == door_form_id)
                    })
                    .map(|(&entity, _)| entity)
                    .collect()
            };
            // The door's link is spawned as a unidirectional pair (see
            // `spawn_link_pair`); despawn every entity but record only one
            // blocked entry, from the first entity's own orientation, so a
            // later unblock respawns exactly one pair.
            let mut recorded = false;
            for link_entity in removed {
                let (start, end) = world
                    .get::<AnimationLink3d>(link_entity)
                    .map(|link| (link.start_edge.0, link.end_edge.0))
                    .unwrap_or_default();
                if let Ok(entity) = world.get_entity_mut(link_entity) {
                    entity.despawn();
                }
                let mut state = world.resource_mut::<NavArchipelagoState>();
                state.link_kinds.remove(&link_entity);
                state.links.retain(|entity| *entity != link_entity);
                if !recorded {
                    state.blocked_door_links.push(BlockedDoorLink {
                        door_form_id,
                        start,
                        end,
                    });
                    recorded = true;
                }
            }
        }

        // Route refresh: re-insert every active agent's current target so
        // landmass replans with the updated link set. `AgentTarget3d` is not
        // `Clone`; rebuild the equivalent value by matching its variants.
        let active_agents = all_agent_entities(world);
        // Issue #155 feature 2: every active agent's lock-cost overrides
        // must reflect this exact flip before landmass's next solve --
        // `NavArchipelagoState::door_usable` was already updated above, so
        // this rebuild picks up `door_form_id`'s new state along with any
        // other door's existing one.
        for agent_entity in &active_agents {
            apply_door_lock_overrides(world, *agent_entity);
        }
        for agent_entity in &active_agents {
            let target =
                world
                    .get::<AgentTarget3d>(*agent_entity)
                    .and_then(|target| match target {
                        AgentTarget3d::None => None,
                        AgentTarget3d::Point(point) => Some(AgentTarget3d::Point(*point)),
                        AgentTarget3d::Entity(entity) => Some(AgentTarget3d::Entity(*entity)),
                    });
            if let Some(target) = target {
                world.entity_mut(*agent_entity).insert(target);
            }
        }

        // Any agent paused waiting on this exact door can now proceed.
        // Issue #185: this door may have become usable *for the shared,
        // no-particular-actor baseline* (an ordinary unlock), or it may
        // still show unusable there while being usable for one specific
        // paused agent that holds its key -- so every paused agent gets its
        // own `request_door_open` attempt rather than one shared check.
        for agent_entity in active_agents.iter().copied() {
            let paused_on_this_door = matches!(
                world.get::<AgentRuntime>(agent_entity).map(|r| r.door_link),
                Some(door_link::DoorLinkState::Paused { door_form_id: paused, .. }) if paused == door_form_id
            );
            if paused_on_this_door {
                request_door_open(world, agent_entity, door_form_id);
            }
        }

        info!(
            "nav agent repath door {door_form_id:08x} {}",
            if now_usable { "unblocked" } else { "blocked" }
        );
    }
}
