[bevy](../../index.html)::[ecs](../index.html)::[message](index.html)

# Function message\_update\_condition 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/update.rs.html#49)

```rust
pub fn message_update_condition(
    maybe_signal: Option<Res<'_, MessageRegistry>>,
) -> bool
```

A run condition for [`message_update_system`](fn.message_update_system.html "fn bevy::ecs::message::message_update_system").

If [`signal_message_update_system`](fn.signal_message_update_system.html "fn bevy::ecs::message::signal_message_update_system") has been run at least once, we will wait for it to be run again before updating the messages.

Otherwise, we will always update the messages.