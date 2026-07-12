[bevy](../../index.html)::[remote](../index.html)::[builtin\_methods](index.html)

# Function process\_remote\_mutate\_components\_request 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/builtin_methods.rs.html#1168-1171)

```rust
pub fn process_remote_mutate_components_request(
    _: In<Option<Value>>,
    world: &mut World,
) -> Result<Value, BrpError>
```

Handles a `world.mutate_components` request coming from a client.

This method allows you to mutate a single field inside an Entity’s component.