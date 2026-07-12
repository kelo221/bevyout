[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function trigger 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#281)

```rust
pub fn trigger<'a, E>(event: E) -> impl Commandwhere
    E: Event,
    <E as Event>::Trigger<'a>: Default,
```

Triggers the given [`Event`](../../../prelude/trait.Event.html "trait bevy::prelude::Event"), which will run any [`Observer`](../../../prelude/struct.Observer.html "struct bevy::prelude::Observer")s watching for it.