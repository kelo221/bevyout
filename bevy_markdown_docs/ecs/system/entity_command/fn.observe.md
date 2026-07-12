[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function observe 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#279)

```rust
pub fn observe<M>(observer: impl IntoEntityObserver<M>) -> impl EntityCommand
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that creates an [`Observer`](../../../prelude/struct.Observer.html "struct bevy::prelude::Observer") watching for an [`EntityEvent`](../../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") of type `E` whose [`event_target`](../../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") targets this entity.

Accepts any type that implements [`IntoEntityObserver`](../../observer/trait.IntoEntityObserver.html "trait bevy::ecs::observer::IntoEntityObserver"), including:

*   Observer systems (closures or functions implementing [`IntoObserverSystem`](../trait.IntoObserverSystem.html "trait bevy::ecs::system::IntoObserverSystem"))
*   Observer systems with run conditions (via `.run_if()`)