[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function trigger\_with 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#297-300)

```rust
pub fn trigger_with<E>(
    event: E,
    trigger: <E as Event>::Trigger<'static>,
) -> impl Commandwhere
    E: Event,
    <E as Event>::Trigger<'static>: Send + Sync,
```

Triggers the given [`Event`](../../../prelude/trait.Event.html "trait bevy::prelude::Event") using the given [`Trigger`](../../event/trait.Trigger.html "trait bevy::ecs::event::Trigger"), which will run any [`Observer`](../../../prelude/struct.Observer.html "struct bevy::prelude::Observer")s watching for it.