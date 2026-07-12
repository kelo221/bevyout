[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function remove 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#210)

```rust
pub fn remove<T>() -> impl EntityCommandwhere
    T: Bundle,
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that removes the components in a [`Bundle`](../../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") from an entity.