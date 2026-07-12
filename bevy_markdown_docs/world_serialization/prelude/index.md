[bevy](../../index.html)::[world\_serialization](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/lib.rs.html#38)

The `bevy_world_serialization` prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[DynamicWorld](struct.DynamicWorld.html "struct bevy::world_serialization::prelude::DynamicWorld")

A collection of serializable resources and dynamic entities.

[DynamicWorldBuilder](struct.DynamicWorldBuilder.html "struct bevy::world_serialization::prelude::DynamicWorldBuilder")

A [`DynamicWorld`](../../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld") builder, used to build a [`DynamicWorld`](../../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld") from a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") by extracting some entities and resources.

[DynamicWorldRoot](struct.DynamicWorldRoot.html "struct bevy::world_serialization::prelude::DynamicWorldRoot")

Adding this component will spawn the world as a child of that entity. Once it’s spawned, the entity will have a [`WorldInstance`](../struct.WorldInstance.html "struct bevy::world_serialization::WorldInstance") component.

[WorldAsset](struct.WorldAsset.html "struct bevy::world_serialization::prelude::WorldAsset")

A composition of [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") objects.

[WorldAssetRoot](struct.WorldAssetRoot.html "struct bevy::world_serialization::prelude::WorldAssetRoot")

Adding this component will spawn the world as a child of that entity. Once it’s spawned, the entity will have a [`WorldInstance`](../struct.WorldInstance.html "struct bevy::world_serialization::WorldInstance") component.

[WorldInstanceSpawner](struct.WorldInstanceSpawner.html "struct bevy::world_serialization::prelude::WorldInstanceSpawner")

Handles spawning and despawning world instances, either synchronously or batched through the [`world_instance_spawner_system`](../fn.world_instance_spawner_system.html "fn bevy::world_serialization::world_instance_spawner_system").

## Enums

[WorldFilter](enum.WorldFilter.html "enum bevy::world_serialization::prelude::WorldFilter")

A filter used to control which types can be added to a [`DynamicWorld`](../../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld").