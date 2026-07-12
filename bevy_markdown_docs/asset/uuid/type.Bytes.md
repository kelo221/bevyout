[bevy](../../index.html)::[asset](../index.html)::[uuid](index.html)

# Type Alias Bytes 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/lib.rs.html#288)

```rust
pub type Bytes = [u8; 16];
```

A 128-bit (16 byte) buffer containing the UUID.

## ABI

The `Bytes` type is always guaranteed to be have the same ABI as [`Uuid`](struct.Uuid.html "struct bevy::asset::uuid::Uuid").