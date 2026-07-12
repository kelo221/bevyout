[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function run\_system\_cached 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#211-214)

```rust
pub fn run_system_cached<M, S>(system: S) -> impl Commandwhere
    M: 'static,
    S: IntoSystem<(), (), M> + Send + 'static,
```

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that runs the given system, caching its [`SystemId`](../struct.SystemId.html "struct bevy::ecs::system::SystemId") in a [`CachedSystemId`](../struct.CachedSystemId.html "struct bevy::ecs::system::CachedSystemId") resource.