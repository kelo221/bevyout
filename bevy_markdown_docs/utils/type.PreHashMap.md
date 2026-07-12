[bevy](../index.html)::[utils](index.html)

# Type Alias PreHashMap 

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#14)

```rust
pub type PreHashMap<K, V> = HashMap<Hashed<K>, V, PassHash>;
```

A [`HashMap`](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap") pre-configured to use [`Hashed`](../platform/hash/struct.Hashed.html "struct bevy::platform::hash::Hashed") keys and [`PassHash`](../platform/hash/struct.PassHash.html "struct bevy::platform::hash::PassHash") passthrough hashing. Iteration order only depends on the order of insertions and deletions.

## Aliased Type

```rust
pub struct PreHashMap<K, V>(/* private fields */);
```