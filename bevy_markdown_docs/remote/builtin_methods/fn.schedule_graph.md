[bevy](../../index.html)::[remote](../index.html)::[builtin\_methods](index.html)

# Function schedule\_graph 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/builtin_methods.rs.html#1748)

```rust
pub fn schedule_graph(
    _: In<Option<Value>>,
    world: &mut World,
) -> Result<Value, BrpError>
```

Handles a `schedule.graph` request coming from a client.

Bevy removes a schedule from the world before running it, meaning that not all Schedules are available.