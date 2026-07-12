[bevy](../../index.html)::[render](../index.html)

# Module sync\_world 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#64)

## Structs

[MainEntity](struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

Component added on the render world entities to keep track of the corresponding main world entity.

[RenderEntity](struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

Component added on the main world entities that are synced to the Render World in order to keep track of the corresponding render world entity.

[SyncToRenderWorld](struct.SyncToRenderWorld.html "struct bevy::render::sync_world::SyncToRenderWorld")

Marker component that indicates that its entity needs to be synchronized to the render world.

[SyncWorldPlugin](struct.SyncWorldPlugin.html "struct bevy::render::sync_world::SyncWorldPlugin")

A plugin that synchronizes entities with [`SyncToRenderWorld`](struct.SyncToRenderWorld.html "struct bevy::render::sync_world::SyncToRenderWorld") between the main world and the render world.

[TemporaryRenderEntity](struct.TemporaryRenderEntity.html "struct bevy::render::sync_world::TemporaryRenderEntity")

Marker component that indicates that its entity needs to be despawned at the end of the frame.

## Type Aliases

[MainEntityHashMap](type.MainEntityHashMap.html "type bevy::render::sync_world::MainEntityHashMap")

A [`HashMap`](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap") pre-configured to use [`EntityHash`](../../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing with a [`MainEntity`](struct.MainEntity.html "struct bevy::render::sync_world::MainEntity").

[MainEntityHashSet](type.MainEntityHashSet.html "type bevy::render::sync_world::MainEntityHashSet")

A [`HashSet`](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet") pre-configured to use [`EntityHash`](../../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing with a [`MainEntity`](struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")..