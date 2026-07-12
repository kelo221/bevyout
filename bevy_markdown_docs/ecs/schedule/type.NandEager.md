[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Type Alias NandEager 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1394)

```rust
pub type NandEager<A, B> = CombinatorSystem<NandEagerMarker, A, B>;
```

Combines and inverts the outputs of two systems using the `&` and `!` operators (eagerly evaluated).

## Aliased Type

```rust
pub struct NandEager<A, B> { /* private fields */ }
```