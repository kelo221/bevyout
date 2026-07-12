[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias BlasGeometrySizeDescriptors 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/blas.rs.html#16)

```rust
pub type BlasGeometrySizeDescriptors = BlasGeometrySizeDescriptors;
```

Descriptor for the size defining attributes, for a bottom level acceleration structure.

## Aliased Type

```rust
pub enum BlasGeometrySizeDescriptors {
    Triangles {
        descriptors: Vec<BlasTriangleGeometrySizeDescriptor>,
    },
}
```

## Variants

### Triangles

Triangle geometry version.

#### Fields

`descriptors: [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[BlasTriangleGeometrySizeDescriptor](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/ray_tracing/struct.BlasTriangleGeometrySizeDescriptor.html "struct wgpu_types::ray_tracing::BlasTriangleGeometrySizeDescriptor")>`

Descriptor for each triangle geometry.