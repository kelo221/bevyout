[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Type Alias Xnor 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1409)

```rust
pub type Xnor<A, B> = CombinatorSystem<XnorMarker, A, B>;
```

Combines and inverts the outputs of two systems using the `^` and `!` operators (eagerly evaluated).

## Aliased Type

```rust
pub struct Xnor<A, B> { /* private fields */ }
```