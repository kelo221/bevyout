[bevy](../index.html)::[world\_serialization](index.html)

# Function serialize\_ron 

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/dynamic_world.rs.html#236-238)

```rust
pub fn serialize_ron<S>(serialize: S) -> Result<String, Error>where
    S: Serialize,
```

Available on **crate feature `serialize`** only.

Serialize a given Rust data structure into rust object notation (ron).