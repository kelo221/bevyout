[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function clone\_components 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#323)

```rust
pub fn clone_components<B>(target: Entity) -> impl EntityCommandwhere
    B: Bundle,
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that clones the specified components of an entity and inserts them into another entity.