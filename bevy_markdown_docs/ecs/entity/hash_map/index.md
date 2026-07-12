[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)

# Module hash\_map 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#98)

Contains the [`EntityHashMap`](../struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap") type, a [`HashMap`](../../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap") pre-configured to use [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

This module is a lightweight wrapper around Bevy’s [`HashMap`](../../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap") that is more performant for [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity") keys.

## Structs

[EntityHashMap](struct.EntityHashMap.html "struct bevy::ecs::entity::hash_map::EntityHashMap")

A [`HashMap`](../../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap") pre-configured to use [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

[IntoKeys](struct.IntoKeys.html "struct bevy::ecs::entity::hash_map::IntoKeys")

An owning iterator over the keys of a [`EntityHashMap`](../struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap") in arbitrary order. The iterator element type is [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[Keys](struct.Keys.html "struct bevy::ecs::entity::hash_map::Keys")

An iterator over the keys of a [`EntityHashMap`](../struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap") in arbitrary order. The iterator element type is `&'a Entity`.