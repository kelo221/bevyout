[bevy](../../index.html)::[render](../index.html)::[slab\_allocator](index.html)

# Trait SlabItemLayout 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/slab_allocator.rs.html#108)

```rust
pub trait SlabItemLayout:
    Clone
    + PartialEq
    + Eq
    + Hash {
    // Required methods
    fn size(&self) -> u64;
    fn elements_per_slot(&self) -> u32;
    fn buffer_usages(&self) -> BufferUsages;
}
```

A trait that defines information necessary to determine the size and alignment of objects within a slab.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/slab_allocator.rs.html#113)

#### fn [size](#tymethod.size)(&self) -> [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

The size in bytes of a single element.

This is the smallest size that this allocator can allocate, and all allocations must have a byte size that is a multiple of this value.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/slab_allocator.rs.html#116)

#### fn [elements\_per\_slot](#tymethod.elements_per_slot)(&self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

The number of elements that make up a single slot.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/slab_allocator.rs.html#123)

#### fn [buffer\_usages](#tymethod.buffer_usages)(&self) -> [BufferUsages](../render_resource/struct.BufferUsages.html "struct bevy::render::render_resource::BufferUsages")

The `wgpu` buffer usages that the slab allocator will specify when creating buffers.

`BufferUsages::COPY_DST` and `BufferUsages::COPY_SRC` are always included, regardless of what you specify here.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/allocator.rs.html#564)

### impl [SlabItemLayout](trait.SlabItemLayout.html "trait bevy::render::slab_allocator::SlabItemLayout") for [ElementLayout](../mesh/allocator/struct.ElementLayout.html "struct bevy::render::mesh::allocator::ElementLayout")