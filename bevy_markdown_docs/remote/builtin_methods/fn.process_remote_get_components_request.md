[bevy](../../index.html)::[remote](../index.html)::[builtin\_methods](index.html)

# Function process\_remote\_get\_components\_request 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/builtin_methods.rs.html#598-601)

```rust
pub fn process_remote_get_components_request(
    _: In<Option<Value>>,
    world: &World,
) -> Result<Value, BrpError>
```

Handles a `world.get_components` request coming from a client.