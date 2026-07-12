[bevy](../../index.html)::[remote](../index.html)::[builtin\_methods](index.html)

# Function process\_remote\_insert\_components\_request 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/builtin_methods.rs.html#1117-1120)

```rust
pub fn process_remote_insert_components_request(
    _: In<Option<Value>>,
    world: &mut World,
) -> Result<Value, BrpError>
```

Handles a `world.insert_components` request (insert components) coming from a client.