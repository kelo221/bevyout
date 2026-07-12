[bevy](../../index.html)::[ecs](../index.html)::[observer](index.html)

# Type Alias ObserverMap 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/centralized_storage.rs.html#148)

```rust
pub type ObserverMap = EntityHashMap<unsafe fn(DeferredWorld<'_>, Entity, &TriggerContext, PtrMut<'_>, PtrMut<'_>)>;
```

Map between an observer entity and its [`ObserverRunner`](type.ObserverRunner.html "type bevy::ecs::observer::ObserverRunner")

## Aliased Type

```rust
pub struct ObserverMap(/* private fields */);
```