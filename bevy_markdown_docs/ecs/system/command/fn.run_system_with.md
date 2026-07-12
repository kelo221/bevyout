[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function run\_system\_with 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#195-200)

```rust
pub fn run_system_with<I>(
    id: impl Into<SystemId<I>> + Send,
    input: <I as SystemInput>::Inner<'static>,
) -> impl Commandwhere
    I: SystemInput + 'static,
    <I as SystemInput>::Inner<'static>: Send,
```

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that runs the system corresponding to the given [`SystemId`](../struct.SystemId.html "struct bevy::ecs::system::SystemId") and provides the given input value.