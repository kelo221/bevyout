[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function unregister\_system\_cached 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#256-261)

```rust
pub fn unregister_system_cached<I, O, M, S>(system: S) -> impl Commandwhere
    I: SystemInput + Send + 'static,
    O: 'static,
    M: 'static,
    S: IntoSystem<I, O, M> + Send + 'static,
```

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that removes a system previously registered with one of the following:

*   [`Commands::run_system_cached`](../../../prelude/struct.Commands.html#method.run_system_cached "method bevy::prelude::Commands::run_system_cached")
*   [`World::run_system_cached`](../../../prelude/struct.World.html#method.run_system_cached "method bevy::prelude::World::run_system_cached")
*   [`World::register_system_cached`](../../../prelude/struct.World.html#method.register_system_cached "method bevy::prelude::World::register_system_cached")