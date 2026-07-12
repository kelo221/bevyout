[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias BlasTriangleGeometrySizeDescriptor 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/blas.rs.html#12)

```rust
pub type BlasTriangleGeometrySizeDescriptor = BlasTriangleGeometrySizeDescriptor;
```

Descriptor for the size defining attributes of a triangle geometry, for a bottom level acceleration structure.

## Aliased Type

```rust
pub struct BlasTriangleGeometrySizeDescriptor {
    pub vertex_format: VertexFormat,
    pub vertex_count: u32,
    pub index_format: Option<IndexFormat>,
    pub index_count: Option<u32>,
    pub flags: AccelerationStructureGeometryFlags,
}
```

## Fields

`vertex_format: [VertexFormat](../../mesh/enum.VertexFormat.html "enum bevy::mesh::VertexFormat")`

Format of a vertex position, must be \[`VertexFormat::Float32x3`\] with just \[`Features::EXPERIMENTAL_RAY_QUERY`\] but \[`Features::EXTENDED_ACCELERATION_STRUCTURE_VERTEX_FORMATS`\] adds more.

`vertex_count: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)`

Number of vertices.

`index_format: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IndexFormat](enum.IndexFormat.html "enum bevy::render::render_resource::IndexFormat")>`

Format of an index. Only needed if an index buffer is used. If `index_format` is provided `index_count` is required.

`index_count: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)>`

Number of indices. Only needed if an index buffer is used. If `index_count` is provided `index_format` is required.

`flags: [AccelerationStructureGeometryFlags](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/ray_tracing/struct.AccelerationStructureGeometryFlags.html "struct wgpu_types::ray_tracing::AccelerationStructureGeometryFlags")`

Flags for the geometry.