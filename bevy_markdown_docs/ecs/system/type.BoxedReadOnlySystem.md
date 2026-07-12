[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Type Alias BoxedReadOnlySystem 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#227)

```rust
pub type BoxedReadOnlySystem<In = (), Out = ()> = Box<dyn ReadOnlySystem<Out = Out, In = In>>;
```

A convenience type alias for a boxed [`ReadOnlySystem`](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem") trait object.

## Aliased Type

```rust
pub struct BoxedReadOnlySystem<In = (), Out = ()>(/* private fields */);
```