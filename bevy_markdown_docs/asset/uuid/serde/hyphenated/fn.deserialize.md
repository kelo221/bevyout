[bevy](../../../../index.html)::[asset](../../../index.html)::[uuid](../../index.html)::[serde](../index.html)::[hyphenated](index.html)

# Function deserialize 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/external/serde_support.rs.html#672-674)

```rust
pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<Uuid, <D as Deserializer<'de>>::Error>where
    D: Deserializer<'de>,
```

Available on **crate feature `serde`** only.

Deserialize a hyphenated-formatted string as a [`Uuid`](../../struct.Uuid.html).