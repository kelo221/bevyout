[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function insert\_resource 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#170)

```rust
pub fn insert_resource<R>(resource: R) -> impl Commandwhere
    R: Resource,
```

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that inserts a [`Resource`](../../../prelude/trait.Resource.html "trait bevy::prelude::Resource") into the world.