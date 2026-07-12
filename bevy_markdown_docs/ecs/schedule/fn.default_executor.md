[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Function default\_executor 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/mod.rs.html#49)

```rust
pub fn default_executor() -> Box<dyn SystemExecutor>
```

Returns the default executor for the current platform.

On Wasm or when the `multi_threaded` feature is disabled, this returns a [`SingleThreadedExecutor`](struct.SingleThreadedExecutor.html "struct bevy::ecs::schedule::SingleThreadedExecutor"). Otherwise it returns a [`MultiThreadedExecutor`](struct.MultiThreadedExecutor.html "struct bevy::ecs::schedule::MultiThreadedExecutor").