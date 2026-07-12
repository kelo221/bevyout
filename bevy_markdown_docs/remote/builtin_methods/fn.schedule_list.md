[bevy](../../index.html)::[remote](../index.html)::[builtin\_methods](index.html)

# Function schedule\_list 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/builtin_methods.rs.html#1722)

```rust
pub fn schedule_list(
    _: In<Option<Value>>,
    world: &World,
) -> Result<Value, BrpError>
```

Handles a `schedule.list` request coming from a client.