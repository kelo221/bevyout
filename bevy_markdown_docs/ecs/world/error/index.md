[bevy](../../../index.html)::[ecs](../../index.html)::[world](../index.html)

# Module error 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#16)

Contains error types returned by bevy’s schedule.

## Structs

[EntityDespawnError](struct.EntityDespawnError.html "struct bevy::ecs::world::error::EntityDespawnError")

An error that occurs when a specified [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity") could not be despawned.

[TryInsertBatchError](struct.TryInsertBatchError.html "struct bevy::ecs::world::error::TryInsertBatchError")

The error type returned by [`World::try_insert_batch`](../../../prelude/struct.World.html#method.try_insert_batch "method bevy::prelude::World::try_insert_batch") and [`World::try_insert_batch_if_new`](../../../prelude/struct.World.html#method.try_insert_batch_if_new "method bevy::prelude::World::try_insert_batch_if_new") if any of the provided entities do not exist.

[TryRunScheduleError](struct.TryRunScheduleError.html "struct bevy::ecs::world::error::TryRunScheduleError")

The error type returned by [`World::try_run_schedule`](../../../prelude/struct.World.html#method.try_run_schedule "method bevy::prelude::World::try_run_schedule") if the provided schedule does not exist.

## Enums

[EntityComponentError](enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")

An error that occurs when dynamically retrieving components from an entity.

[EntityMutableFetchError](enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")

An error that occurs when fetching entities mutably from a world.

[ResourceFetchError](enum.ResourceFetchError.html "enum bevy::ecs::world::error::ResourceFetchError")

An error that occurs when getting a resource of a given type in a world.