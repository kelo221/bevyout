[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_array](index.html)

# Type Alias IntoIter 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#581)

```rust
pub type IntoIter<const N: usize, T = Entity> = UniqueEntityIter<IntoIter<T, N>>;
```

A by-value array iterator.

Equivalent to [`array::IntoIter`](https://doc.rust-lang.org/nightly/core/array/iter/struct.IntoIter.html "struct core::array::iter::IntoIter").

## Aliased Type

```rust
pub struct IntoIter<const N: usize, T = Entity> { /* private fields */ }
```