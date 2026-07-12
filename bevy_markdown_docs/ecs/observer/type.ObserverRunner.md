[bevy](../../index.html)::[ecs](../index.html)::[observer](index.html)

# Type Alias ObserverRunner 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/runner.rs.html#22)

```rust
pub type ObserverRunner = unsafe fn(DeferredWorld<'_>, Entity, &TriggerContext, PtrMut<'_>, PtrMut<'_>);
```

Type for function that is run when an observer is triggered.

Typically refers to the default runner that runs the system stored in the associated [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") component, but can be overridden for custom behavior.

See `observer_system_runner` for safety considerations.