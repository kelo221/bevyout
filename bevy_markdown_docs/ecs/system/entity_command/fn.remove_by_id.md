[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function remove\_by\_id 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#229)

```rust
pub fn remove_by_id(component_id: ComponentId) -> impl EntityCommand
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that removes a dynamic component from an entity.