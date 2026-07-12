[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Type Alias Xor 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1412)

```rust
pub type Xor<A, B> = CombinatorSystem<XorMarker, A, B>;
```

Combines the outputs of two systems using the `^` operator (eagerly evaluated).

## Aliased Type

```rust
pub struct Xor<A, B> { /* private fields */ }
```