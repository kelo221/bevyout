[bevy](../../index.html)::[ecs](../index.html)

# Module name 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#43)

Provides the [`Name`](../../prelude/struct.Name.html "struct bevy::prelude::Name") [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component"), used for identifying an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

## Structs

[HashedStr](struct.HashedStr.html "struct bevy::ecs::name::HashedStr")

A wrapper over Hashed. This exists to make Name(“value”.into()) possible, which plays nicely with contexts like the `bsn!` macro.

[Name](struct.Name.html "struct bevy::ecs::name::Name")

Component used to identify an entity. Stores a hash for faster comparisons.

[NameOrEntity](struct.NameOrEntity.html "struct bevy::ecs::name::NameOrEntity")

Convenient query for giving a human friendly name to an entity.

[NameOrEntityItem](struct.NameOrEntityItem.html "struct bevy::ecs::name::NameOrEntityItem")

Automatically generated [`WorldQuery`](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") item type for [`NameOrEntity`](../../prelude/struct.NameOrEntity.html "struct bevy::prelude::NameOrEntity"), returned when iterating over query results.