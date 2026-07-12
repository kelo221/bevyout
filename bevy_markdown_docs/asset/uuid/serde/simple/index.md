[bevy](../../../../index.html)::[asset](../../../index.html)::[uuid](../../index.html)::[serde](../index.html)

# Module simple 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/external/serde_support.rs.html#389)

Available on **crate feature `serde`** only.

Serialize a [`Uuid`](../../struct.Uuid.html) as \[`uuid::fmt::Simple`\].

### Examples

Serialize and deserialize using the simple format, failing to deserialize any other format:

```rust
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct StructA {
    #[serde(with = "uuid::serde::simple")]
    id: uuid::Uuid,
}
```

Serialize using the simple format, but deserialize any format:

```rust
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct StructB {
    #[serde(serialize_with = "uuid::serde::simple::serialize")]
    id: uuid::Uuid,
}
```

## Functions

[deserialize](fn.deserialize.html "fn bevy::asset::uuid::serde::simple::deserialize")

Deserialize a simple-formatted string as a [`Uuid`](../../struct.Uuid.html).

[serialize](fn.serialize.html "fn bevy::asset::uuid::serde::simple::serialize")

Serialize a [`Uuid`](../../struct.Uuid.html) as a simple string.