[bevy](../../../../index.html)::[core\_pipeline](../../../index.html)::[mip\_generation](../../index.html)::[experimental](../index.html)::[depth](index.html)

# Constant DEPTH\_PYRAMID\_MIP\_COUNT 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/experimental/depth.rs.html#49)

```rust
pub const DEPTH_PYRAMID_MIP_COUNT: usize = 12; // 12usize
```

The maximum number of mip levels that we can produce.

2^12 is 4096, so that’s the maximum size of the depth buffer that we support.