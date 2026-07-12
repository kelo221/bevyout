[bevy](../index.html)

# Crate world\_serialization 

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/lib.rs.html#1-446)

Provides dynamic world definition, instantiation, and serialization/deserialization.

[`DynamicWorld`](../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld")s are collections of entities and their associated components that can be instantiated or removed from a world to allow composition. [`DynamicWorld`](../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld")s can be serialized/deserialized, for example to save part of the world state to a file.

## Modules

[prelude](prelude/index.html "mod bevy::world_serialization::prelude")

The `bevy_world_serialization` prelude.

[serde](serde/index.html "mod bevy::world_serialization::serde")`serialize`

`serde` serialization and deserialization implementation for Bevy worlds.

## Structs

[DynamicEntity](struct.DynamicEntity.html "struct bevy::world_serialization::DynamicEntity")

A reflection-powered serializable representation of an entity and its components.

[DynamicWorld](struct.DynamicWorld.html "struct bevy::world_serialization::DynamicWorld")

A collection of serializable resources and dynamic entities.

[DynamicWorldBuilder](struct.DynamicWorldBuilder.html "struct bevy::world_serialization::DynamicWorldBuilder")

A [`DynamicWorld`](../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld") builder, used to build a [`DynamicWorld`](../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld") from a [`World`](../prelude/struct.World.html "struct bevy::prelude::World") by extracting some entities and resources.

[DynamicWorldRoot](struct.DynamicWorldRoot.html "struct bevy::world_serialization::DynamicWorldRoot")

Adding this component will spawn the world as a child of that entity. Once it’s spawned, the entity will have a [`WorldInstance`](struct.WorldInstance.html "struct bevy::world_serialization::WorldInstance") component.

[DynamicWorldRootTemplate](struct.DynamicWorldRootTemplate.html "struct bevy::world_serialization::DynamicWorldRootTemplate")

[InstanceId](struct.InstanceId.html "struct bevy::world_serialization::InstanceId")

Unique id identifying a world instance.

[WorldAsset](struct.WorldAsset.html "struct bevy::world_serialization::WorldAsset")

A composition of [`World`](../prelude/struct.World.html "struct bevy::prelude::World") objects.

[WorldAssetLoader](struct.WorldAssetLoader.html "struct bevy::world_serialization::WorldAssetLoader")

Asset loader for a Bevy dynamic world (`.scn` / `.scn.ron`).

[WorldAssetRoot](struct.WorldAssetRoot.html "struct bevy::world_serialization::WorldAssetRoot")

Adding this component will spawn the world as a child of that entity. Once it’s spawned, the entity will have a [`WorldInstance`](struct.WorldInstance.html "struct bevy::world_serialization::WorldInstance") component.

[WorldAssetRootTemplate](struct.WorldAssetRootTemplate.html "struct bevy::world_serialization::WorldAssetRootTemplate")

[WorldInstance](struct.WorldInstance.html "struct bevy::world_serialization::WorldInstance")

[`InstanceId`](struct.InstanceId.html "struct bevy::world_serialization::InstanceId") of a spawned world asset. It can be used with the [`WorldInstanceSpawner`](../prelude/struct.WorldInstanceSpawner.html "struct bevy::prelude::WorldInstanceSpawner") to interact with the spawned world asset.

[WorldInstanceReady](struct.WorldInstanceReady.html "struct bevy::world_serialization::WorldInstanceReady")

Triggered on a world instance’s parent entity when [`WorldInstance`](struct.WorldInstance.html "struct bevy::world_serialization::WorldInstance") becomes ready to use.

[WorldInstanceSpawner](struct.WorldInstanceSpawner.html "struct bevy::world_serialization::WorldInstanceSpawner")

Handles spawning and despawning world instances, either synchronously or batched through the [`world_instance_spawner_system`](fn.world_instance_spawner_system.html "fn bevy::world_serialization::world_instance_spawner_system").

[WorldSerializationPlugin](struct.WorldSerializationPlugin.html "struct bevy::world_serialization::WorldSerializationPlugin")

Plugin that provides world serialization functionality to an [`App`](../prelude/struct.App.html "struct bevy::prelude::App").

## Enums

[WorldAssetLoaderError](enum.WorldAssetLoaderError.html "enum bevy::world_serialization::WorldAssetLoaderError")`serialize`

Possible errors that can be produced by [`WorldAssetLoader`](struct.WorldAssetLoader.html "struct bevy::world_serialization::WorldAssetLoader")

[WorldFilter](enum.WorldFilter.html "enum bevy::world_serialization::WorldFilter")

A filter used to control which types can be added to a [`DynamicWorld`](../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld").

[WorldInstanceSpawnError](enum.WorldInstanceSpawnError.html "enum bevy::world_serialization::WorldInstanceSpawnError")

Errors that can occur when spawning a world asset.

## Functions

[serialize\_ron](fn.serialize_ron.html "fn bevy::world_serialization::serialize_ron")`serialize`

Serialize a given Rust data structure into rust object notation (ron).

[world\_instance\_spawner](fn.world_instance_spawner.html "fn bevy::world_serialization::world_instance_spawner")

System that will spawn instances from the [`WorldAssetRoot`](../prelude/struct.WorldAssetRoot.html "struct bevy::prelude::WorldAssetRoot") and [`DynamicWorldRoot`](../prelude/struct.DynamicWorldRoot.html "struct bevy::prelude::DynamicWorldRoot") components.

[world\_instance\_spawner\_system](fn.world_instance_spawner_system.html "fn bevy::world_serialization::world_instance_spawner_system")

System that handles scheduled world asset instance spawning and despawning through a [`WorldInstanceSpawner`](../prelude/struct.WorldInstanceSpawner.html "struct bevy::prelude::WorldInstanceSpawner").