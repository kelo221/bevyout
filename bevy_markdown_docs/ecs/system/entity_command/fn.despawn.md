[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function despawn 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#264)

```rust
pub fn despawn() -> impl EntityCommand
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that despawns an entity.

## Note

This will also despawn the entities in any [`RelationshipTarget`](../../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") that is configured to despawn descendants.

For example, this will recursively despawn [`Children`](../../../prelude/struct.Children.html "struct bevy::prelude::Children").