[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function remove\_with\_requires 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#220)

```rust
pub fn remove_with_requires<T>() -> impl EntityCommandwhere
    T: Bundle,
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that removes the components in a [`Bundle`](../../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") from an entity, as well as the required components for each component removed.