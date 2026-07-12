[bevy](../../../../index.html)::[asset](../../../index.html)::[uuid](../../index.html)::[serde](../index.html)

# Module hyphenated 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/external/serde_support.rs.html#644)

Available on **crate feature `serde`** only.

Serialize a [`Uuid`](../../struct.Uuid.html) as \[`uuid::fmt::Hyphenated`\].

### Examples

Serialize and deserialize using the hyphenated format, failing to deserialize any other format:

```rust
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct StructA {
    #[serde(with = "uuid::serde::hyphenated")]
    id: uuid::Uuid,
}
```

Serialize using the hyphenated format, but deserialize any format:

```rust
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct StructB {
    #[serde(serialize_with = "uuid::serde::hyphenated::serialize")]
    id: uuid::Uuid,
}
```

## Functions

[deserialize](fn.deserialize.html "fn bevy::asset::uuid::serde::hyphenated::deserialize")

Deserialize a hyphenated-formatted string as a [`Uuid`](../../struct.Uuid.html).

[serialize](fn.serialize.html "fn bevy::asset::uuid::serde::hyphenated::serialize")

Serialize a [`Uuid`](../../struct.Uuid.html) as a hyphenated string.