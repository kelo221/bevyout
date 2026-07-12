[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function run\_system 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#185)

```rust
pub fn run_system<O>(id: impl Into<SystemId<(), O>> + Send) -> impl Commandwhere
    O: 'static,
```

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that runs the system corresponding to the given [`SystemId`](../struct.SystemId.html "struct bevy::ecs::system::SystemId").