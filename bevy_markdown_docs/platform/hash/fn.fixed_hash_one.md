[bevy](../../index.html)::[platform](../index.html)::[hash](index.html)

# Function fixed\_hash\_one 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/hash.rs.html#34)

```rust
pub fn fixed_hash_one(x: impl Hash) -> u64
```

Hashes one value with the deterministic [`FixedHasher`](struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher").