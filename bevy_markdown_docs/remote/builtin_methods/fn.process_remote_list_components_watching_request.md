[bevy](../../index.html)::[remote](../index.html)::[builtin\_methods](index.html)

# Function process\_remote\_list\_components\_watching\_request 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/builtin_methods.rs.html#1435-1439)

```rust
pub fn process_remote_list_components_watching_request(
    _: In<Option<Value>>,
    world: &World,
    removal_cursors: Local<'_, HashMap<ComponentId, MessageCursor<RemovedComponentEntity>>>,
) -> Result<Option<Value>, BrpError>
```

Handles a `world.list_components+watch` request coming from a client.