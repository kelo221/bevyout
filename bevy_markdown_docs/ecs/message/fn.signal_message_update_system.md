[bevy](../../index.html)::[ecs](../index.html)::[message](index.html)

# Function signal\_message\_update\_system 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/update.rs.html#20)

```rust
pub fn signal_message_update_system(signal: Option<ResMut<'_, MessageRegistry>>)
```

Signals the [`message_update_system`](fn.message_update_system.html "fn bevy::ecs::message::message_update_system") to run after `FixedUpdate` systems.

This will change the behavior of the [`MessageRegistry`](struct.MessageRegistry.html "struct bevy::ecs::message::MessageRegistry") to only run after a fixed update cycle has passed. Normally, this will simply run every frame.