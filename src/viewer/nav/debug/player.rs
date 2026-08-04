use bevy::prelude::*;

use crate::viewer::player;

pub(crate) fn player_transform_query(world: &mut World) -> Option<Vec3> {
    let mut query = world.query_filtered::<&GlobalTransform, With<player::FpsPlayer>>();
    query.single(world).ok().map(|t| t.translation())
}

pub(crate) fn player_entity_query(world: &mut World) -> Option<Entity> {
    let mut query = world.query_filtered::<Entity, With<player::FpsPlayer>>();
    query.single(world).ok()
}
