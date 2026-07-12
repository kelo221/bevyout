[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias AccelerationStructureUpdateMode 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/blas.rs.html#28)

```rust
pub type AccelerationStructureUpdateMode = AccelerationStructureUpdateMode;
```

Update mode for acceleration structure builds.

## Aliased Type

```rust
#[repr(u8)]pub enum AccelerationStructureUpdateMode {
    Build = 0,
    PreferUpdate = 1,
}
```

## Variants

### Build = 0

Always perform a full build.

### PreferUpdate = 1

If possible, perform an incremental update.

Not advised for major topology changes. (Useful for e.g. skinning)