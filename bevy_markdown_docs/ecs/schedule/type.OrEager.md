[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Type Alias OrEager 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1406)

```rust
pub type OrEager<A, B> = CombinatorSystem<OrEagerMarker, A, B>;
```

Combines the outputs of two systems using the `|` operator (short-circuiting).

## Aliased Type

```rust
pub struct OrEager<A, B> { /* private fields */ }
```