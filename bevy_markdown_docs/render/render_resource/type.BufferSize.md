[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias BufferSize 

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/lib.rs.html#147)

```rust
pub type BufferSize = NonZero<u64>;
```

Integral type used for [`BufferSlice`](../wgpu/struct.BufferSlice.html) sizes.

Note that while this type is non-zero, a [`Buffer`](../wgpu/struct.Buffer.html) _per se_ can have a size of zero, but no slice or mapping can be created from it.

## Aliased Type

```rust
pub struct BufferSize(/* private fields */);
```