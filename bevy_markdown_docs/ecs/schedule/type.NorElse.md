[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Type Alias NorElse 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1397)

```rust
pub type NorElse<A, B> = CombinatorSystem<NorElseMarker, A, B>;
```

Combines and inverts the outputs of two systems using the `||` and `!` operators (short-circuiting).

## Aliased Type

```rust
pub struct NorElse<A, B> { /* private fields */ }
```