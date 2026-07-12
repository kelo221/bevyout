[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function run\_system\_cached\_with 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#226-230)

```rust
pub fn run_system_cached_with<I, M, S>(
    system: S,
    input: <I as SystemInput>::Inner<'static>,
) -> impl Commandwhere
    I: SystemInput + Send + 'static,
    <I as SystemInput>::Inner<'static>: Send,
    M: 'static,
    S: IntoSystem<I, (), M> + Send + 'static,
```

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that runs the given system with the given input value, caching its [`SystemId`](../struct.SystemId.html "struct bevy::ecs::system::SystemId") in a [`CachedSystemId`](../struct.CachedSystemId.html "struct bevy::ecs::system::CachedSystemId") resource.

To use the supplied input, the system should have a [`SystemInput`](../../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") as the first parameter.