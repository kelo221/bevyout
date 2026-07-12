[bevy](../index.html)::[remote](index.html)

# Type Alias RemoteInstantMethodSystemId 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#940)

```rust
pub type RemoteInstantMethodSystemId = SystemId<In<Option<Value>>, Result<Value, BrpError>>;
```

The [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId") of a function that implements a remote instant method (`world.get_components`, `world.query`, etc.)

The first parameter is the JSON value of the `params`. Typically, an implementation will deserialize these as the first thing they do.

The returned JSON value will be returned as the response. Bevy will automatically populate the `id` field before sending.

## Aliased Type

```rust
pub struct RemoteInstantMethodSystemId { /* private fields */ }
```