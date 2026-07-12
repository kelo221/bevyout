[bevy](../../../index.html)::[render](../../index.html)::[mesh](../index.html)::[allocator](index.html)

# Type Alias MeshSlabId 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/allocator.rs.html#94)

```rust
pub type MeshSlabId = SlabId<MeshSlabItem>;
```

The ID of a single slab.

## Aliased Type

```rust
#[repr(transparent)]pub struct MeshSlabId {
    pub id: NonMaxU32,
    /* private fields */
}
```

## Fields

`id: [NonMaxU32](https://docs.rs/nonmax/0.5.5/x86_64-unknown-linux-gnu/nonmax/struct.NonMaxU32.html "struct nonmax::NonMaxU32")`

A value that represents the ID of the slab.