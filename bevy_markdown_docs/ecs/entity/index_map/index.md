[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)

# Module index\_map 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#104)

Contains the [`EntityIndexMap`](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap") type, an [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap") pre-configured to use [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

This module is a lightweight wrapper around `indexmap`’s [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap") that is more performant for [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity") keys.

## Structs

[Drain](struct.Drain.html "struct bevy::ecs::entity::index_map::Drain")

A draining iterator over the entries of an [`EntityIndexMap`](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap").

[EntityIndexMap](struct.EntityIndexMap.html "struct bevy::ecs::entity::index_map::EntityIndexMap")

A [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap") pre-configured to use [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

[IntoIter](struct.IntoIter.html "struct bevy::ecs::entity::index_map::IntoIter")

An owning iterator over the entries of an [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap").

[IntoKeys](struct.IntoKeys.html "struct bevy::ecs::entity::index_map::IntoKeys")

An owning iterator over the keys of an [`EntityIndexMap`](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap").

[Iter](struct.Iter.html "struct bevy::ecs::entity::index_map::Iter")

An iterator over the entries of an [`EntityIndexMap`](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap").

[IterMut](struct.IterMut.html "struct bevy::ecs::entity::index_map::IterMut")

A mutable iterator over the entries of an [`EntityIndexMap`](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap").

[Keys](struct.Keys.html "struct bevy::ecs::entity::index_map::Keys")

An iterator over the keys of an [`EntityIndexMap`](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap").

[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")

A dynamically-sized slice of key-value pairs in an [`EntityIndexMap`](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap").

## Enums

[Entry](enum.Entry.html "enum bevy::ecs::entity::index_map::Entry")

Entry for an existing key-value pair in an [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap") or a vacant location to insert one.