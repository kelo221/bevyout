[bevy](../../index.html)::[remote](../index.html)::[builtin\_methods](index.html)

# Function process\_remote\_observe\_watching\_request 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/builtin_methods.rs.html#1570-1573)

```rust
pub fn process_remote_observe_watching_request(
    _: In<Option<Value>>,
    world: &mut World,
) -> Result<Option<Value>, BrpError>
```

Handles a `world.observe+watch` request coming from a client.

On the first call for a given event/entity combination, this registers an observer that captures triggered events. On each subsequent poll, it returns any events that have been captured since the last poll.

When `entity` is provided, the observer is scoped to that entity. Otherwise a global observer is registered.