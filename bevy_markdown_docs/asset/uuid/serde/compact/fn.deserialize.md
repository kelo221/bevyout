[bevy](../../../../index.html)::[asset](../../../index.html)::[uuid](../../index.html)::[serde](../index.html)::[compact](index.html)

# Function deserialize 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/external/serde_support.rs.html#301-303)

```rust
pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<Uuid, <D as Deserializer<'de>>::Error>where
    D: Deserializer<'de>,
```

Available on **crate feature `serde`** only.

Deserialize a `[u8; 16]` as a [`Uuid`](../../struct.Uuid.html)