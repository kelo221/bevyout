[bevy](../../index.html)::[ecs](../index.html)::[event](index.html)

# Derive Macro EntityEvent 

[Source](https://docs.rs/bevy_ecs_macros/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs_macros/lib.rs.html#555)

```rust
#[derive(EntityEvent)]
{
    // Attributes available to this derive:
    #[entity_event]
    #[event_target]
}
```

Cheat sheet for derive syntax, see full explanation on `EntityEvent` trait docs.

[ⓘ](# "This example is not tested")

```rust
#[derive(EntityEvent)]
/// Enable propagation, which defaults to using the ChildOf component
#[entity_event(propagate)]
/// Enable propagation using the given Traversal implementation
#[entity_event(propagate = &'static ChildOf)]
/// Always propagate
#[entity_event(auto_propagate)]
struct MyEvent;
```