[bevy](../../index.html)::[ecs](../index.html)::[storage](index.html)

# Trait SparseSetIndex 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#751)

```rust
pub trait SparseSetIndex:
    Clone
    + PartialEq
    + Eq
    + Hash {
    // Required methods
    fn sparse_set_index(&self) -> usize;
    fn get_sparse_set_index(value: usize) -> Self;
}
```

Represents something that can be stored in a [`SparseSet`](struct.SparseSet.html "struct bevy::ecs::storage::SparseSet") as an integer.

Ideally, the `usize` values should be very small (ie: incremented starting from zero), as the number of bits needed to represent a `SparseSetIndex` in a `FixedBitSet` is proportional to the **value** of those `usize`.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#753)

#### fn [sparse\_set\_index](#tymethod.sparse_set_index)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Gets the sparse set index corresponding to this instance.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#755)

#### fn [get\_sparse\_set\_index](#tymethod.get_sparse_set_index)(value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> Self

Creates a new instance of this type with the specified index.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

### impl [SparseSetIndex](trait.SparseSetIndex.html "trait bevy::ecs::storage::SparseSetIndex") for [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

#### fn [sparse\_set\_index](#tymethod.sparse_set_index)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

#### fn [get\_sparse\_set\_index](#tymethod.get_sparse_set_index)(value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

### impl [SparseSetIndex](trait.SparseSetIndex.html "trait bevy::ecs::storage::SparseSetIndex") for [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

#### fn [sparse\_set\_index](#tymethod.sparse_set_index)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

#### fn [get\_sparse\_set\_index](#tymethod.get_sparse_set_index)(value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

### impl [SparseSetIndex](trait.SparseSetIndex.html "trait bevy::ecs::storage::SparseSetIndex") for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

#### fn [sparse\_set\_index](#tymethod.sparse_set_index)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

#### fn [get\_sparse\_set\_index](#tymethod.get_sparse_set_index)(value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

### impl [SparseSetIndex](trait.SparseSetIndex.html "trait bevy::ecs::storage::SparseSetIndex") for [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

#### fn [sparse\_set\_index](#tymethod.sparse_set_index)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

#### fn [get\_sparse\_set\_index](#tymethod.get_sparse_set_index)(value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

### impl [SparseSetIndex](trait.SparseSetIndex.html "trait bevy::ecs::storage::SparseSetIndex") for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

#### fn [sparse\_set\_index](#tymethod.sparse_set_index)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/storage/sparse_set.rs.html#774)

#### fn [get\_sparse\_set\_index](#tymethod.get_sparse_set_index)(value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/info.rs.html#39)

### impl [SparseSetIndex](trait.SparseSetIndex.html "trait bevy::ecs::storage::SparseSetIndex") for [BundleId](../bundle/struct.BundleId.html "struct bevy::ecs::bundle::BundleId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/info.rs.html#200)

### impl [SparseSetIndex](trait.SparseSetIndex.html "trait bevy::ecs::storage::SparseSetIndex") for [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#687)

### impl [SparseSetIndex](trait.SparseSetIndex.html "trait bevy::ecs::storage::SparseSetIndex") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#214)

### impl [SparseSetIndex](trait.SparseSetIndex.html "trait bevy::ecs::storage::SparseSetIndex") for [EntityIndex](../entity/struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/identifier.rs.html#97)

### impl [SparseSetIndex](trait.SparseSetIndex.html "trait bevy::ecs::storage::SparseSetIndex") for [WorldId](../world/struct.WorldId.html "struct bevy::ecs::world::WorldId")