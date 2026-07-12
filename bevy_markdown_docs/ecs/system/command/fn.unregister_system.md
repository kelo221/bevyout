[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function unregister\_system 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#241-244)

```rust
pub fn unregister_system<I, O>(system_id: SystemId<I, O>) -> impl Commandwhere
    I: SystemInput + Send + 'static,
    O: Send + 'static,
```

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that removes a system previously registered with [`Commands::register_system`](../../../prelude/struct.Commands.html#method.register_system "method bevy::prelude::Commands::register_system") or [`World::register_system`](../../../prelude/struct.World.html#method.register_system "method bevy::prelude::World::register_system").