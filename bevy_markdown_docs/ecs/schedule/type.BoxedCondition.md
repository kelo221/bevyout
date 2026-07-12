[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Type Alias BoxedCondition 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#11)

```rust
pub type BoxedCondition<In = ()> = Box<dyn ReadOnlySystem<Out = bool, In = In>>;
```

A type-erased run condition stored in a [`Box`](../../prelude/struct.Box.html "struct bevy::prelude::Box").

## Aliased Type

```rust
pub struct BoxedCondition<In = ()>(/* private fields */);
```