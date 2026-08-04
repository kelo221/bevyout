use super::actor_binding;
use super::fall_guard;
use super::*;

pub(crate) fn nav_fall_guard_system(world: &mut World) {
    let Some(min_y) = world.resource::<NavCellFallBounds>().min_y else {
        return;
    };
    // Issue #241: every nav agent, not just the roster-indexed ones -- an
    // autonomously-bound actor falling through missing collision must be
    // caught by the same guard.
    let fallen: Vec<(Entity, f32)> = all_agent_entities(world)
        .into_iter()
        .filter_map(|entity| {
            let agent_y = world.get::<Transform>(entity)?.translation.y;
            (fall_guard::evaluate_fall(min_y, agent_y) == fall_guard::FallVerdict::FellOutOfWorld)
                .then_some((entity, agent_y))
        })
        .collect();
    if fallen.is_empty() {
        return;
    }
    let kill_z = fall_guard::fall_kill_z(min_y);
    for (entity, agent_y) in fallen {
        let id = agent_log_id(world, entity);
        warn!("nav agent fell out of world {id} y={agent_y} kill_z={kill_z}");
        // Issue #188 feature 5: a fallen *bound actor* releases its agent
        // and its locomotion animation state instead of being deleted --
        // this guard exists to end a runaway descent, not to destroy an NPC
        // the world/actor slice owns. A `tna` capsule still despawns whole.
        if world.get::<actor_binding::NavBoundActor>(entity).is_some() {
            release_bound_actor(world, entity);
        } else if let Ok(entity_mut) = world.get_entity_mut(entity) {
            entity_mut.despawn();
        }
        // Only a *console-addressed* agent owns a roster slot to free; an
        // autonomously-bound one never had one (issue #241).
        if let Some(index) = world.resource::<DebugAgentRoster>().index_of(entity) {
            world.resource_mut::<DebugAgentRoster>().set(index, None);
        }
    }
}
