[bevy](../index.html)::[remote](index.html)

# Type Alias RemoteWatchingMethodSystemId 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#950)

```rust
pub type RemoteWatchingMethodSystemId = SystemId<In<Option<Value>>, Result<Option<Value>, BrpError>>;
```

The [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId") of a function that implements a remote watching method (`world.get_components+watch`, `world.list_components+watch`, etc.)

The first parameter is the JSON value of the `params`. Typically, an implementation will deserialize these as the first thing they do.

The optional returned JSON value will be sent as a response. If no changes were detected this should be [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"). Re-running of this handler is done in the [`RemotePlugin`](struct.RemotePlugin.html "struct bevy::remote::RemotePlugin").

## Aliased Type

```rust
pub struct RemoteWatchingMethodSystemId { /* private fields */ }
```