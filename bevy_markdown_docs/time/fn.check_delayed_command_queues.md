[bevy](../index.html)::[time](index.html)

# Function check\_delayed\_command\_queues 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/delayed_commands.rs.html#146-150)

```rust
pub fn check_delayed_command_queues(
    queues: Query<'_, '_, (Entity, &mut DelayedCommandQueue)>,
    time: Res<'_, Time>,
    commands: Commands<'_, '_>,
)
```

The system used to check [`DelayedCommandQueue`](struct.DelayedCommandQueue.html "struct bevy::time::DelayedCommandQueue")s, which are usually spawned by [`DelayedCommands`](struct.DelayedCommands.html "struct bevy::time::DelayedCommands"). When the elapsed time exceeds a queue’s `submit_at` time, the contained `queue` is appended to the system’s [`Commands`](../prelude/struct.Commands.html "struct bevy::prelude::Commands").