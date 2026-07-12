[bevy](../../index.html)::[ecs](../index.html)::[lifecycle](index.html)

# Type Alias RemovedIter 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#519)

```rust
pub type RemovedIter<'a> = Map<Flatten<IntoIter<Cloned<MessageIterator<'a, RemovedComponentEntity>>>>, fn(RemovedComponentEntity) -> Entity>;
```

Iterator over entities that had a specific component removed.

See [`RemovedComponents`](../../prelude/struct.RemovedComponents.html "struct bevy::prelude::RemovedComponents").

## Aliased Type

```rust
pub struct RemovedIter<'a> { /* private fields */ }
```