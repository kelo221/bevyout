[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function spawn\_batch 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#129-132)

```rust
pub fn spawn_batch<I>(bundles_iter: I) -> impl Commandwhere
    I: IntoIterator + Send + Sync + 'static,
    <I as IntoIterator>::Item: Bundle,
    <<I as IntoIterator>::Item as DynamicBundle>::Effect: NoBundleEffect,
```

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that consumes an iterator of [`Bundles`](../../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") to spawn a series of entities.

This is more efficient than spawning the entities individually.