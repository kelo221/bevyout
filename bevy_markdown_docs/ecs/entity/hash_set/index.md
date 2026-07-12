[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)

# Module hash\_set 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#99)

Contains the [`EntityHashSet`](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") type, a [`HashSet`](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet") pre-configured to use [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

This module is a lightweight wrapper around Bevy’s [`HashSet`](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet") that is more performant for [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity") keys.

## Structs

[Drain](struct.Drain.html "struct bevy::ecs::entity::hash_set::Drain")

A draining iterator over the items of an [`EntityHashSet`](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet").

[EntityHashSet](struct.EntityHashSet.html "struct bevy::ecs::entity::hash_set::EntityHashSet")

A [`HashSet`](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet") pre-configured to use [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

[ExtractIf](struct.ExtractIf.html "struct bevy::ecs::entity::hash_set::ExtractIf")

A draining iterator over entries of a [`EntityHashSet`](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") which don’t satisfy the predicate `f`.

[IntoIter](struct.IntoIter.html "struct bevy::ecs::entity::hash_set::IntoIter")

Owning iterator over the items of an [`EntityHashSet`](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet").

[Iter](struct.Iter.html "struct bevy::ecs::entity::hash_set::Iter")

An iterator over the items of an [`EntityHashSet`](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet").