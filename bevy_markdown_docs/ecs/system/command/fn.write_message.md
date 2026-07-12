[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function write\_message 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#309)

```rust
pub fn write_message<M>(message: M) -> impl Commandwhere
    M: Message,
```

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that writes an arbitrary [`Message`](../../../prelude/trait.Message.html "trait bevy::prelude::Message").