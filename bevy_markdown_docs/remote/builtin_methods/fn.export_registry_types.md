[bevy](../../index.html)::[remote](../index.html)::[builtin\_methods](index.html)

# Function export\_registry\_types 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/builtin_methods.rs.html#1668)

```rust
pub fn export_registry_types(
    _: In<Option<Value>>,
    world: &World,
) -> Result<Value, BrpError>
```

Handles a `registry.schema` request (list all registry types in form of schema) coming from a client.