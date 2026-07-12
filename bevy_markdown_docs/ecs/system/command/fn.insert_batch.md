[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function insert\_batch 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#147-150)

```rust
pub fn insert_batch<I, B>(batch: I, insert_mode: InsertMode) -> impl Commandwhere
    I: IntoIterator<Item = (Entity, B)> + Send + Sync + 'static,
    B: Bundle,
    <B as DynamicBundle>::Effect: NoBundleEffect,
```

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that consumes an iterator to add a series of [`Bundles`](../../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") to a set of entities.

If any entities do not exist in the world, this command will return a [`TryInsertBatchError`](../../world/error/struct.TryInsertBatchError.html "struct bevy::ecs::world::error::TryInsertBatchError").

This is more efficient than inserting the bundles individually.