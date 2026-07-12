[bevy](../../index.html)::[asset](../index.html)::[io](index.html)

# Constant STACK\_FUTURE\_SIZE 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#87)

```rust
pub const STACK_FUTURE_SIZE: usize = _; // 80usize
```

The maximum size of a future returned from [`Reader::read_to_end`](trait.Reader.html#method.read_to_end "method bevy::asset::io::Reader::read_to_end"). This is large enough to fit ten references.