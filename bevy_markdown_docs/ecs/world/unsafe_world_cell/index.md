[bevy](../../../index.html)::[ecs](../../index.html)::[world](../index.html)

# Module unsafe\_world\_cell 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#19)

Contains types that allow disjoint mutable access to a [`World`](../../../prelude/struct.World.html "struct bevy::prelude::World").

## Structs

[UnsafeEntityCell](struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")

An interior-mutable reference to a particular [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity") and all of its components

[UnsafeWorldCell](struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")

Variant of the [`World`](../../../prelude/struct.World.html "struct bevy::prelude::World") where resource and component accesses take `&self`, and the responsibility to avoid aliasing violations are given to the caller instead of being checked at compile-time by rust’s unique XOR shared rule.

## Enums

[GetEntityMutByIdError](enum.GetEntityMutByIdError.html "enum bevy::ecs::world::unsafe_world_cell::GetEntityMutByIdError")

Error that may be returned when calling [`UnsafeEntityCell::get_mut_by_id`](struct.UnsafeEntityCell.html#method.get_mut_by_id "method bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell::get_mut_by_id").