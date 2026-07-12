[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)

# Module index\_set 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#105)

Contains the [`EntityIndexSet`](../struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet") type, a [`IndexSet`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet") pre-configured to use [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

This module is a lightweight wrapper around `indexmap`’ss [`IndexSet`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet") that is more performant for [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity") keys.

## Structs

[Drain](struct.Drain.html "struct bevy::ecs::entity::index_set::Drain")

A draining iterator over the items of an [`EntityIndexSet`](../struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet").

[EntityIndexSet](struct.EntityIndexSet.html "struct bevy::ecs::entity::index_set::EntityIndexSet")

An [`IndexSet`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet") pre-configured to use [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

[IntoIter](struct.IntoIter.html "struct bevy::ecs::entity::index_set::IntoIter")

Owning iterator over the items of an [`EntityIndexSet`](../struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet").

[Iter](struct.Iter.html "struct bevy::ecs::entity::index_set::Iter")

An iterator over the items of an [`EntityIndexSet`](../struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet").

[Slice](struct.Slice.html "struct bevy::ecs::entity::index_set::Slice")

A dynamically-sized slice of values in an [`EntityIndexSet`](../struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet").