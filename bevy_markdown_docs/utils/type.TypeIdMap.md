[bevy](../index.html)::[utils](index.html)

# Type Alias TypeIdMap 

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#43)

```rust
pub type TypeIdMap<V> = IndexMap<TypeId, V, NoOpHash>;
```

A specialized map type with Key of [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") Iteration order only depends on the order of insertions and deletions.

## Aliased Type

```rust
pub struct TypeIdMap<V> { /* private fields */ }
```