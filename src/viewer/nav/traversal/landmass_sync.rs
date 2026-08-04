//! Landmass input-refresh workaround and portal quarantine projection.

use std::collections::BTreeSet;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_landmass::prelude::AgentTarget3d;
use bevy_landmass::{AgentTypeIndexCostOverrides, PermittedAnimationLinks};

use crate::viewer::nav::agent::{
    AgentRuntime, PendingMergeRepath, RefreshLandmassAnimationLinkInput,
    SuspendedLandmassTypeIndexCosts,
};
use crate::viewer::nav::landmass_graph;

pub(crate) fn copied_type_index_cost_overrides(
    current: &AgentTypeIndexCostOverrides,
) -> AgentTypeIndexCostOverrides {
    let mut copied = AgentTypeIndexCostOverrides::default();
    for (&type_index, &cost) in current.iter() {
        let inserted = copied.set_type_index_cost(type_index, cost);
        debug_assert!(inserted, "existing Landmass override must stay positive");
    }
    copied
}

/// Forces Landmass's next input sync to process `UsingAnimationLink` after a
/// merge-link start or finish. In `bevy_landmass` 0.12.0, an unchanged
/// `AgentTypeIndexCostOverrides` causes that sync to return before it compares
/// the animation-link marker. This system temporarily exposes the normal
/// `None` branch immediately before `SyncValues`; that branch performs the
/// animation-link comparison without the early return.
pub(crate) fn refresh_landmass_animation_link_input(world: &mut World) {
    let entities: Vec<Entity> = world
        .query_filtered::<Entity, With<RefreshLandmassAnimationLinkInput>>()
        .iter(world)
        .collect();
    for entity in entities {
        let suspended = world
            .get::<AgentTypeIndexCostOverrides>(entity)
            .map(copied_type_index_cost_overrides);
        world
            .entity_mut(entity)
            .remove::<AgentTypeIndexCostOverrides>()
            .insert(SuspendedLandmassTypeIndexCosts(suspended))
            .remove::<RefreshLandmassAnimationLinkInput>();
    }
}

/// Restores the exact per-agent costs after Landmass has consumed the marker.
/// The internal Landmass costs are synchronized from the changed component on
/// the next fixed tick; while the agent is entering or leaving an animation
/// link, path solving cannot consume a normal polygon step in between.
pub(crate) fn restore_landmass_type_index_costs(world: &mut World) {
    let entities: Vec<Entity> = world
        .query_filtered::<Entity, With<SuspendedLandmassTypeIndexCosts>>()
        .iter(world)
        .collect();
    for entity in entities {
        let suspended = world
            .entity_mut(entity)
            .take::<SuspendedLandmassTypeIndexCosts>()
            .expect("queried suspended Landmass costs");
        if let Some(overrides) = suspended.0 {
            world.entity_mut(entity).insert(overrides);
        }
    }
}

/// Rebuilds an agent's `PermittedAnimationLinks` from its current
/// `AgentRuntime::quarantined_merge_link_kinds` (issue #162): thin wrapper
/// over the pure `landmass_graph::permitted_animation_link_kinds` that
/// converts its `Option<BTreeSet<usize>>` into the actual `bevy_landmass`
/// component -- `None` (nothing quarantined) becomes the cheap `All`
/// default rather than materializing an equivalent full allow-list.
pub(crate) fn permitted_animation_links_for(
    quarantined: &BTreeSet<usize>,
    merge_link_kind_count: usize,
) -> PermittedAnimationLinks {
    match landmass_graph::permitted_animation_link_kinds(quarantined, merge_link_kind_count) {
        None => PermittedAnimationLinks::All,
        Some(kinds) => PermittedAnimationLinks::Kinds(Arc::new(kinds.into_iter().collect())),
    }
}

/// Issue #162 feature 2: resets `agent_entity`'s merge-portal quarantine to
/// empty, called whenever the agent gets a genuinely new destination
/// (`goto_agent`/`request_travel`) -- a fresh `tna goto`/`tna travel` is a
/// new routing intent, so whatever previously blocked links this agent
/// steered around no longer apply to it. Despawn/hand-off need no
/// equivalent call: `AgentRuntime`/`PermittedAnimationLinks` are ordinary
/// components on the agent entity, gone the moment it despawns.
pub(crate) fn clear_merge_link_quarantine(world: &mut World, agent_entity: Entity) {
    if let Some(mut runtime) = world.get_mut::<AgentRuntime>(agent_entity) {
        runtime.quarantined_merge_link_kinds.clear();
    }
    if let Ok(mut entity) = world.get_entity_mut(agent_entity) {
        entity.insert(PermittedAnimationLinks::All);
    }
}

pub(crate) fn resume_pending_merge_repath_system(
    mut commands: Commands,
    agents: Query<(Entity, &PendingMergeRepath, Option<&AgentTarget3d>)>,
) {
    for (entity, pending, current_target) in &agents {
        commands.entity(entity).remove::<PendingMergeRepath>();
        if matches!(current_target, Some(AgentTarget3d::None) | None) {
            commands
                .entity(entity)
                .insert(pending.target.to_agent_target());
        }
    }
}
