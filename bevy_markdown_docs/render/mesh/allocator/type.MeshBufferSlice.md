[bevy](../../../index.html)::[render](../../index.html)::[mesh](../index.html)::[allocator](index.html)

# Type Alias MeshBufferSlice 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/allocator.rs.html#98)

```rust
pub type MeshBufferSlice<'a> = SlabAllocationBufferSlice<'a, MeshSlabItem>;
```

The slab buffer and location within that slab in which each mesh is allocated.

## Aliased Type

```rust
pub struct MeshBufferSlice<'a> {
    pub buffer: &'a Buffer,
    pub range: Range<u32>,
    /* private fields */
}
```

## Fields

`buffer: &'a [Buffer](../../render_resource/struct.Buffer.html "struct bevy::render::render_resource::Buffer")`

The buffer that the data resides in.

`range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)>`

The range of elements within this buffer that the data resides in, measured in elements.

This is an element range, not a byte range. For vertex data, this is measured in increments of a single vertex. (Thus, if a vertex is 32 bytes long, then this range is in units of 32 bytes each.) For index data, this is measured in increments of a single index value (2 or 4 bytes). Draw commands generally take their ranges in elements, not bytes, so this is the most convenient unit in this case.