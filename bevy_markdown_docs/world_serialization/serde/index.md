[bevy](../../index.html)::[world\_serialization](../index.html)

# Module serde 

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/lib.rs.html#25)

Available on **crate feature `serialize`** only.

`serde` serialization and deserialization implementation for Bevy worlds.

## Structs

[DynamicWorldSerializer](struct.DynamicWorldSerializer.html "struct bevy::world_serialization::serde::DynamicWorldSerializer")

Serializer for a [`DynamicWorld`](../../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld").

[EntitiesSerializer](struct.EntitiesSerializer.html "struct bevy::world_serialization::serde::EntitiesSerializer")

Handles serialization of multiple entities as a map of entity id to serialized entity.

[EntitySerializer](struct.EntitySerializer.html "struct bevy::world_serialization::serde::EntitySerializer")

Handles entity serialization as a map of component type to component value.

[WorldDeserializer](struct.WorldDeserializer.html "struct bevy::world_serialization::serde::WorldDeserializer")

Handles world deserialization.

[WorldEntitiesDeserializer](struct.WorldEntitiesDeserializer.html "struct bevy::world_serialization::serde::WorldEntitiesDeserializer")

Handles deserialization for a collection of entities.

[WorldEntityDeserializer](struct.WorldEntityDeserializer.html "struct bevy::world_serialization::serde::WorldEntityDeserializer")

Handle deserialization of an entity and its components.

[WorldMapDeserializer](struct.WorldMapDeserializer.html "struct bevy::world_serialization::serde::WorldMapDeserializer")

Handles deserialization of a sequence of values with unique types.

[WorldMapSerializer](struct.WorldMapSerializer.html "struct bevy::world_serialization::serde::WorldMapSerializer")

Handles serializing a list of values with a unique type as a map of type to value.

## Constants

[ENTITY\_FIELD\_COMPONENTS](constant.ENTITY_FIELD_COMPONENTS.html "constant bevy::world_serialization::serde::ENTITY_FIELD_COMPONENTS")

Name of the serialized component field in an entity struct.

[ENTITY\_STRUCT](constant.ENTITY_STRUCT.html "constant bevy::world_serialization::serde::ENTITY_STRUCT")

Name of the serialized entity struct type.

[WORLD\_ENTITIES](constant.WORLD_ENTITIES.html "constant bevy::world_serialization::serde::WORLD_ENTITIES")

Name of the serialized entities field in a world struct.

[WORLD\_RESOURCES](constant.WORLD_RESOURCES.html "constant bevy::world_serialization::serde::WORLD_RESOURCES")

Name of the serialized resources field in a world struct.

[WORLD\_STRUCT](constant.WORLD_STRUCT.html "constant bevy::world_serialization::serde::WORLD_STRUCT")

Name of the serialized world struct type.