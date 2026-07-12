[bevy](../../../index.html)::[asset](../../index.html)::[uuid](../index.html)::[timestamp](index.html)

# Constant UUID\_TICKS\_BETWEEN\_EPOCHS 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#30)

```rust
pub const UUID_TICKS_BETWEEN_EPOCHS: u64 = 0x01B2_1DD2_1381_4000; // 122_192_928_000_000_000u64
```

The number of 100 nanosecond ticks between the RFC 9562 epoch (`1582-10-15 00:00:00`) and the Unix epoch (`1970-01-01 00:00:00`).