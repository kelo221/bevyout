[bevy](../../../../index.html)::[asset](../../../index.html)::[uuid](../../index.html)::[serde](../index.html)

# Module urn 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/external/serde_support.rs.html#773)

Available on **crate feature `serde`** only.

Serialize a [`Uuid`](../../struct.Uuid.html) as \[`uuid::fmt::Urn`\].

### Examples

Serialize and deserialize using the URN format, failing to deserialize any other format:

```rust
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct StructA {
    #[serde(with = "uuid::serde::urn")]
    id: uuid::Uuid,
}
```

Serialize using the URN format, but deserialize any format:

```rust
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct StructB {
    #[serde(serialize_with = "uuid::serde::urn::serialize")]
    id: uuid::Uuid,
}
```

## Functions

[deserialize](fn.deserialize.html "fn bevy::asset::uuid::serde::urn::deserialize")

Deserialize a URN-formatted string as a [`Uuid`](../../struct.Uuid.html).

[serialize](fn.serialize.html "fn bevy::asset::uuid::serde::urn::serialize")

Serialize a [`Uuid`](../../struct.Uuid.html) as a URN string.