use crate::viewer::actor::ActorRuntime;
use crate::viewer::nav::agent::*;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_landmass::AgentState;

pub(crate) fn format_agent_id(
    roster_index: Option<usize>,
    reference_form_id: Option<u32>,
    entity: Entity,
) -> String {
    match (roster_index, reference_form_id) {
        (Some(index), _) => index.to_string(),
        (None, Some(form_id)) => format!("{form_id:08x}"),
        (None, None) => format!("e{entity}"),
    }
}

/// [`format_agent_id`] resolved against a whole world, for the exclusive
/// (`&mut World`) systems that have no query params to read the two sources
/// from.
pub(crate) fn agent_log_id(world: &World, entity: Entity) -> String {
    format_agent_id(
        world
            .get::<DebugNavAgent>(entity)
            .map(|debug_agent| debug_agent.index)
            .or_else(|| {
                world
                    .get_resource::<DebugAgentRoster>()
                    .and_then(|roster| roster.index_of(entity))
            }),
        world
            .get::<ActorRuntime>(entity)
            .map(|actor| actor.reference_form_id),
        entity,
    )
}

/// Every live nav agent, in a deterministic spawn-ish order, for the systems
/// that used to walk `DebugAgentRoster::active()` instead. Sorted rather
/// than left in archetype order so a tick that drives several agents through
/// shared state (the door lifecycle) stays reproducible. Keyed on
/// `Entity::index_u32`, not `Entity`'s own `Ord`: the latter compares
/// `to_bits`, which bevy documents as opaque and which in practice orders
/// indices *backwards*.
pub(crate) fn all_agent_entities(world: &mut World) -> Vec<Entity> {
    let mut agents: Vec<Entity> = world
        .query_filtered::<Entity, With<NavAgent>>()
        .iter(world)
        .collect();
    agents.sort_unstable_by_key(|entity| entity.index_u32());
    agents
}

/// Intercell nav-agent ledger (issue #134): survives cell-swap teardown
/// (an ordinary Bevy `Resource`, untouched by `teardown_archipelago`) so an
/// agent handed off through a travel door, or frozen in place by a
/// player-initiated swap, can be restored once its cell is active again.
pub(crate) const HUD_AGENT_LINE_LIMIT: usize = 8;

pub(crate) struct HudAgentRow {
    pub(crate) roster_index: Option<usize>,
    pub(crate) entity_index: u32,
    pub(crate) line: String,
}

pub(crate) fn hud_agent_lines_from_rows(mut rows: Vec<HudAgentRow>) -> Vec<String> {
    rows.sort_by_key(|row| (row.roster_index.unwrap_or(usize::MAX), row.entity_index));
    let hidden = rows.len().saturating_sub(HUD_AGENT_LINE_LIMIT);
    let mut lines: Vec<String> = rows
        .into_iter()
        .take(HUD_AGENT_LINE_LIMIT)
        .map(|row| row.line)
        .collect();
    if hidden > 0 {
        lines.push(format!("nav agent +{hidden} more"));
    }
    lines
}

pub(crate) type HudAgentQueryData = (
    Entity,
    Option<&'static GlobalTransform>,
    Option<&'static AgentState>,
    Option<&'static AgentRuntime>,
    Option<&'static AgentKcc>,
    Option<&'static ActorRuntime>,
);

/// Read-only debug-info projection for live navigation agents.
#[derive(SystemParam)]
pub(crate) struct HudAgentProjection<'w, 's> {
    state: Option<Res<'w, DebugAgentRoster>>,
    agents: Query<'w, 's, HudAgentQueryData, With<NavAgent>>,
}

impl HudAgentProjection<'_, '_> {
    pub(crate) fn status_lines(&self) -> Vec<String> {
        let Some(state) = self.state.as_deref() else {
            return Vec::new();
        };
        let rows: Vec<HudAgentRow> = self
            .agents
            .iter()
            .map(|(entity, transform, landmass_state, runtime, kcc, actor)| {
                let roster_index = state.index_of(entity);
                let id = format_agent_id(
                    roster_index,
                    actor.map(|actor| actor.reference_form_id),
                    entity,
                );
                let position = transform
                    .map(|transform| transform.translation())
                    .unwrap_or_default();
                let landmass_state = landmass_state.copied().unwrap_or_default();
                let door_link_state = runtime.map(|runtime| runtime.door_link).unwrap_or_default();
                let (grounded, stuck, collision_blocked) = kcc
                    .map(|kcc| (kcc.grounded, kcc.stuck, kcc.collision_blocked))
                    .unwrap_or_default();
                let status = resolve_status(landmass_state, door_link_state);
                HudAgentRow {
                    roster_index,
                    entity_index: entity.index_u32(),
                    line: format!(
                        "nav agent {id} status={} position=({:.2},{:.2},{:.2}) grounded={grounded} stuck={stuck} blocked={collision_blocked}",
                        status.as_str(),
                        position.x,
                        position.y,
                        position.z,
                    ),
                }
            })
            .collect();
        hud_agent_lines_from_rows(rows)
    }
}
