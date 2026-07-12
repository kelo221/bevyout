[bevy](../../index.html)::[remote](../index.html)::[builtin\_methods](index.html)

# Function process\_remote\_list\_components\_request 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/builtin_methods.rs.html#1378-1381)

```rust
pub fn process_remote_list_components_request(
    _: In<Option<Value>>,
    world: &World,
) -> Result<Value, BrpError>
```

Handles a `world.list_components` request (list all components) coming from a client.