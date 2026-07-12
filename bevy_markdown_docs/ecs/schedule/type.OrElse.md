[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Type Alias OrElse 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1403)

```rust
pub type OrElse<A, B> = CombinatorSystem<OrElseMarker, A, B>;
```

Combines the outputs of two systems using the `||` operator (short-circuiting).

## Aliased Type

```rust
pub struct OrElse<A, B> { /* private fields */ }
```