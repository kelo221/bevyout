[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias IterMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1489)

```rust
pub type IterMut<'a, T> = UniqueEntityIter<IterMut<'a, T>>;
```

Mutable slice iterator.

## Aliased Type

```rust
pub struct IterMut<'a, T> { /* private fields */ }
```