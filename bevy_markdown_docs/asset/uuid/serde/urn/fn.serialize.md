[bevy](../../../../index.html)::[asset](../../../index.html)::[uuid](../../index.html)::[serde](../index.html)::[urn](index.html)

# Function serialize 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/external/serde_support.rs.html#790-792)

```rust
pub fn serialize<S>(
    u: &Uuid,
    serializer: S,
) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where
    S: Serializer,
```

Available on **crate feature `serde`** only.

Serialize a [`Uuid`](../../struct.Uuid.html) as a URN string.

## Examples

```rust
#[derive(serde_derive::Serialize)]
struct Struct {
    #[serde(serialize_with = "uuid::serde::urn::serialize")]
    id: uuid::Uuid,
}
```