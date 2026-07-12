[bevy](../../index.html)::[ecs](../index.html)

# Module hierarchy 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#38)

The canonical “parent-child” [`Relationship`](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship") for entities, driven by the [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") [`Relationship`](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship") and the [`Children`](../../prelude/struct.Children.html "struct bevy::prelude::Children") [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget").

See [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") for a full description of the relationship and how to use it.

## Structs

[ChildOf](struct.ChildOf.html "struct bevy::ecs::hierarchy::ChildOf")

Stores the parent entity of this child entity with this component.

[ChildOfTemplate](struct.ChildOfTemplate.html "struct bevy::ecs::hierarchy::ChildOfTemplate")

[Children](struct.Children.html "struct bevy::ecs::hierarchy::Children")

Tracks which entities are children of this parent entity.

## Type Aliases

[ChildSpawner](type.ChildSpawner.html "type bevy::ecs::hierarchy::ChildSpawner")

A type alias over [`RelatedSpawner`](../relationship/struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner") used to spawn child entities containing a [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship.

[ChildSpawnerCommands](type.ChildSpawnerCommands.html "type bevy::ecs::hierarchy::ChildSpawnerCommands")

A type alias over [`RelatedSpawnerCommands`](../relationship/struct.RelatedSpawnerCommands.html "struct bevy::ecs::relationship::RelatedSpawnerCommands") used to spawn child entities containing a [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship.