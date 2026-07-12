[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Type Alias AndEager 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1388)

```rust
pub type AndEager<A, B> = CombinatorSystem<AndEagerMarker, A, B>;
```

Combines the outputs of two systems using the `&` operator (eagerly evaluated).

## Aliased Type

```rust
pub struct AndEager<A, B> { /* private fields */ }
```