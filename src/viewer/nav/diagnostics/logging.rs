use bevy::prelude::*;
use bevy_landmass::prelude::*;

use crate::viewer::actor::ActorRuntime;
use crate::viewer::nav::agent::*;

pub(crate) fn log_agent_state_changes(
    mut agents: Query<
        (
            Entity,
            &AgentState,
            &mut AgentRuntime,
            Option<&ActorRuntime>,
        ),
        With<NavAgent>,
    >,
    roster: Res<DebugAgentRoster>,
) {
    for (entity, agent_state, mut runtime, actor) in &mut agents {
        if runtime.last_logged_state == Some(*agent_state) {
            continue;
        }
        runtime.last_logged_state = Some(*agent_state);
        let id = format_agent_id(
            roster.index_of(entity),
            actor.map(|actor| actor.reference_form_id),
            entity,
        );
        match agent_state {
            AgentState::ReachedTarget => info!("nav agent {id} reached"),
            AgentState::AgentNotOnNavMesh => info!("nav agent off-navmesh {id}"),
            AgentState::TargetNotOnNavMesh | AgentState::NoPath => {
                info!("nav agent {id} unreachable state={agent_state:?}");
            }
            _ => {}
        }
    }
}

pub(crate) fn log_path_latency(
    time: Res<Time>,
    mut agents: Query<
        (
            Entity,
            &AgentState,
            &mut AgentRuntime,
            Option<&ActorRuntime>,
        ),
        With<NavAgent>,
    >,
    roster: Res<DebugAgentRoster>,
) {
    for (entity, agent_state, mut runtime, actor) in &mut agents {
        if runtime.latency_logged {
            continue;
        }
        let Some(started_at) = runtime.goto_started_at else {
            continue;
        };
        if matches!(agent_state, AgentState::Moving | AgentState::ReachedTarget) {
            let id = format_agent_id(
                roster.index_of(entity),
                actor.map(|actor| actor.reference_form_id),
                entity,
            );
            let latency_ms = (time.elapsed_secs() - started_at) * 1000.0;
            info!("nav agent {id} path latency_ms={latency_ms:.1}");
            runtime.latency_logged = true;
        }
    }
}
