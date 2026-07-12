[bevy](../../index.html)::[ecs](../index.html)::[message](index.html)

# Function message\_update\_system 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/update.rs.html#27)

```rust
pub fn message_update_system(
    world: &mut World,
    last_change_tick: Local<'_, Tick>,
)
```

A system that calls [`Messages::update`](../../prelude/struct.Messages.html#method.update "method bevy::prelude::Messages::update") on all registered [`Messages`](../../prelude/struct.Messages.html "struct bevy::prelude::Messages") in the world.