//! Disposable debug-capsule construction and teardown.

use bevy::prelude::*;
use bevy_landmass::AnimationLinkReachedDistance;

use crate::viewer::nav::agent::{AGENT_HEIGHT, AGENT_RADIUS, NavAgent, agent_components};
use crate::viewer::nav::doors::access::apply_door_lock_overrides;
use crate::viewer::nav::world::build::TEST_AGENT_ANIMATION_LINK_REACHED_DISTANCE;
use crate::viewer::nav::world::state::NavArchipelagoState;

pub(crate) fn spawn_debug_capsule(world: &mut World, position: Vec3) -> Entity {
    let archipelago_entity = world
        .resource::<NavArchipelagoState>()
        .archipelago
        .expect("ensure_archipelago populated the archipelago");
    let cylinder_height = (AGENT_HEIGHT - 2.0 * AGENT_RADIUS).max(0.0);
    let mesh = world
        .resource_mut::<Assets<Mesh>>()
        .add(Capsule3d::new(AGENT_RADIUS, cylinder_height));
    let material = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.85, 0.9),
            ..default()
        });
    let agent_entity = world
        .spawn((
            NavAgent,
            Transform::from_translation(position),
            Visibility::Inherited,
            agent_components(archipelago_entity, 0.0),
            AnimationLinkReachedDistance(TEST_AGENT_ANIMATION_LINK_REACHED_DISTANCE),
        ))
        .id();
    let visual = world
        .spawn((Mesh3d(mesh), MeshMaterial3d(material), Transform::IDENTITY))
        .id();
    world.entity_mut(agent_entity).add_child(visual);
    apply_door_lock_overrides(world, agent_entity);
    agent_entity
}

pub(crate) fn despawn_debug_capsule(world: &mut World, entity: Entity) {
    if let Ok(entity) = world.get_entity_mut(entity) {
        entity.despawn();
    }
}
