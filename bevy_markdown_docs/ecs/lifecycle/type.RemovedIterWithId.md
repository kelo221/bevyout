[bevy](../../index.html)::[ecs](../index.html)::[lifecycle](index.html)

# Type Alias RemovedIterWithId 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#527)

```rust
pub type RemovedIterWithId<'a> = Map<Flatten<IntoIter<MessageIteratorWithId<'a, RemovedComponentEntity>>>, fn((&RemovedComponentEntity, MessageId<RemovedComponentEntity>)) -> (Entity, MessageId<RemovedComponentEntity>)>;
```

Iterator over entities that had a specific component removed.

See [`RemovedComponents`](../../prelude/struct.RemovedComponents.html "struct bevy::prelude::RemovedComponents").

## Aliased Type

```rust
pub struct RemovedIterWithId<'a> { /* private fields */ }
```