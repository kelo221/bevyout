[bevy](../../index.html)::[render](../index.html)::[slab\_allocator](index.html)

# Trait SlabItem 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/slab_allocator.rs.html#86)

```rust
pub trait SlabItem {
    type Key: Clone + PartialEq + Eq + Hash;
    type Layout: SlabItemLayout;

    // Required method
    fn label() -> Cow<'static, str>;
}
```

Describes the type of the data that a [`SlabAllocator`](struct.SlabAllocator.html "struct bevy::render::slab_allocator::SlabAllocator") will store.

The actual type that you implement this trait on doesn’t matter; only the associated types [`Self::Key`](trait.SlabItem.html#associatedtype.Key "associated type bevy::render::slab_allocator::SlabItem::Key") and [`Self::Layout`](trait.SlabItem.html#associatedtype.Layout "associated type bevy::render::slab_allocator::SlabItem::Layout") do. Typically, you implement this trait on a unit struct.

See [`crate::mesh::allocator::MeshSlabItem`](../mesh/allocator/struct.MeshSlabItem.html "struct bevy::render::mesh::allocator::MeshSlabItem") for an example of usage.

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/slab_allocator.rs.html#88)

#### type [Key](#associatedtype.Key): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash")

The key that’s used to look up items in the allocator.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/slab_allocator.rs.html#99)

#### type [Layout](#associatedtype.Layout): [SlabItemLayout](trait.SlabItemLayout.html "trait bevy::render::slab_allocator::SlabItemLayout")

A type that describes the layout of items within a single slab.

If this slab allocator only allocates items of a single type, this type can simply be a unit struct. However, if you wish to have a single slab allocator that manages slabs of differing types, you can store metadata within values of this type that describes the size and alignment requirements of the objects within the slab. Each slab that the slab allocator manages contains an instance of this value so that it can track size and alignment requirements for that slab.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/slab_allocator.rs.html#103)

#### fn [label](#tymethod.label)() -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns a suitable debugging label describing the type of elements that this slab item stores.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/allocator.rs.html#104)

### impl [SlabItem](trait.SlabItem.html "trait bevy::render::slab_allocator::SlabItem") for [MeshSlabItem](../mesh/allocator/struct.MeshSlabItem.html "struct bevy::render::mesh::allocator::MeshSlabItem")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/allocator.rs.html#105)

#### type [Key](#associatedtype.Key) = [MeshAllocationKey](../mesh/allocator/struct.MeshAllocationKey.html "struct bevy::render::mesh::allocator::MeshAllocationKey")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/allocator.rs.html#106)

#### type [Layout](#associatedtype.Layout) = [ElementLayout](../mesh/allocator/struct.ElementLayout.html "struct bevy::render::mesh::allocator::ElementLayout")