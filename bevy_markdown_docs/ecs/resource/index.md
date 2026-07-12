[bevy](../../index.html)::[ecs](../index.html)

# Module resource 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#50)

Resources are unique, singleton-like data types that can be accessed from systems and stored in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

## Structs

[IsResource](struct.IsResource.html "struct bevy::ecs::resource::IsResource")

A marker component for entities that have a Resource component.

[ResourceEntities](struct.ResourceEntities.html "struct bevy::ecs::resource::ResourceEntities")

A cache that links each `ComponentId` from a resource to the corresponding entity.

## Constants

[IS\_RESOURCE](constant.IS_RESOURCE.html "constant bevy::ecs::resource::IS_RESOURCE")

[`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") of the [`IsResource`](struct.IsResource.html "struct bevy::ecs::resource::IsResource") component.

## Traits

[Resource](trait.Resource.html "trait bevy::ecs::resource::Resource")

A type that can be inserted into a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") as a singleton.

## Derive Macros

[Resource](derive.Resource.html "derive bevy::ecs::resource::Resource")

Implement the `Resource` trait.