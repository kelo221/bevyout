[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function retain 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#248)

```rust
pub fn retain<T>() -> impl EntityCommandwhere
    T: Bundle,
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that removes all components from an entity, except for those in the given [`Bundle`](../../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle").