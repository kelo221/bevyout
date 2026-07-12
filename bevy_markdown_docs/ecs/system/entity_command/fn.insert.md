[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function insert 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#132)

```rust
pub fn insert(bundle: impl Bundle, mode: InsertMode) -> impl EntityCommand
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that adds the components in a [`Bundle`](../../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") to an entity.