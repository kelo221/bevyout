[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Type Alias BoxedSystem 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#224)

```rust
pub type BoxedSystem<In = (), Out = ()> = Box<dyn System<Out = Out, In = In>>;
```

A convenience type alias for a boxed [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") trait object.

## Aliased Type

```rust
pub struct BoxedSystem<In = (), Out = ()>(/* private fields */);
```