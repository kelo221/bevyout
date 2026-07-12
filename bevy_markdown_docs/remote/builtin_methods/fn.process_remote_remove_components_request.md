[bevy](../../index.html)::[remote](../index.html)::[builtin\_methods](index.html)

# Function process\_remote\_remove\_components\_request 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/builtin_methods.rs.html#1284-1287)

```rust
pub fn process_remote_remove_components_request(
    _: In<Option<Value>>,
    world: &mut World,
) -> Result<Value, BrpError>
```

Handles a `world.remove_components` request (remove components) coming from a client.