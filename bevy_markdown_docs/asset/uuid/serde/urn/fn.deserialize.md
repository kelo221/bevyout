[bevy](../../../../index.html)::[asset](../../../index.html)::[uuid](../../index.html)::[serde](../index.html)::[urn](index.html)

# Function deserialize 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/external/serde_support.rs.html#800-802)

```rust
pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<Uuid, <D as Deserializer<'de>>::Error>where
    D: Deserializer<'de>,
```

Available on **crate feature `serde`** only.

Deserialize a URN-formatted string as a [`Uuid`](../../struct.Uuid.html).