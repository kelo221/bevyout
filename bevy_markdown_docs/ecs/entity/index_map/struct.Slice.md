[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[index\_map](index.html)

# Struct Slice 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#382)

```rust
pub struct Slice<V, S = EntityHash>(/* private fields */);
```

A dynamically-sized slice of key-value pairs in an [`EntityIndexMap`](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap").

Equivalent to an [`indexmap::map::Slice<V>`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice") whose source [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap") uses [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash").

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#384)

### impl<V> [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#388)

#### pub const fn [new](#method.new)<'a>() -> &'a [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Returns an empty slice.

Equivalent to [`map::Slice::new`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.new "associated function indexmap::map::slice::Slice::new").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#396)

#### pub fn [new\_mut](#method.new_mut)<'a>() -> &'a mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Returns an empty mutable slice.

Equivalent to [`map::Slice::new_mut`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.new_mut "associated function indexmap::map::slice::Slice::new_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#408)

#### pub const unsafe fn [from\_slice\_unchecked](#method.from_slice_unchecked)(slice: &[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V>) -> &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Constructs a [`entity::index_map::Slice`](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice") from a [`indexmap::map::Slice`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice") unsafely.

##### Safety

`slice` must stem from an [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap") using [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#420)

#### pub const unsafe fn [from\_slice\_unchecked\_mut](#method.from_slice_unchecked_mut)( slice: &mut [Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V>, ) -> &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Constructs a [`entity::index_map::Slice`](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice") from a [`indexmap::map::Slice`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice") unsafely.

##### Safety

`slice` must stem from an [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap") using [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#426)

#### pub const fn [as\_inner](#method.as_inner)(&self) -> &[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V>

Casts `self` to the inner slice.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#437)

#### pub unsafe fn [from\_boxed\_slice\_unchecked](#method.from_boxed_slice_unchecked)( slice: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V>>, ) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

Constructs a boxed [`entity::index_map::Slice`](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice") from a boxed [`indexmap::map::Slice`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice") unsafely.

##### Safety

`slice` must stem from an [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap") using [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#447)

#### pub const fn [as\_boxed\_inner](#method.as_boxed_inner)(self: &[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>) -> &[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V>>

Casts a reference to `self` to the inner slice.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#453)

#### pub fn [into\_boxed\_inner](#method.into_boxed_inner)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V>>

Casts `self` to the inner slice.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#461)

#### pub fn [get\_index\_mut](#method.get_index_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Get a key-value pair by index, with mutable access to the value.

Equivalent to [`map::Slice::get_index_mut`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.get_index_mut "method indexmap::map::slice::Slice::get_index_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#468)

#### pub fn [get\_range](#method.get_range)<R>(&self, range: R) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Returns a slice of key-value pairs in the given range of indices.

Equivalent to [`map::Slice::get_range`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.get_range "method indexmap::map::slice::Slice::get_range").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#477)

#### pub fn [get\_range\_mut](#method.get_range_mut)<R>(&mut self, range: R) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Returns a mutable slice of key-value pairs in the given range of indices.

Equivalent to [`map::Slice::get_range_mut`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.get_range_mut "method indexmap::map::slice::Slice::get_range_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#486)

#### pub fn [first\_mut](#method.first_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Get the first key-value pair, with mutable access to the value.

Equivalent to [`map::Slice::first_mut`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.first_mut "method indexmap::map::slice::Slice::first_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#493)

#### pub fn [last\_mut](#method.last_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Get the last key-value pair, with mutable access to the value.

Equivalent to [`map::Slice::last_mut`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.last_mut "method indexmap::map::slice::Slice::last_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#500)

#### pub fn [split\_at](#method.split_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>, &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>)

Divides one slice into two at an index.

Equivalent to [`map::Slice::split_at`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.split_at "method indexmap::map::slice::Slice::split_at").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#514)

#### pub fn [split\_at\_mut](#method.split_at_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>, &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>)

Divides one mutable slice into two at an index.

Equivalent to [`map::Slice::split_at_mut`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.split_at_mut "method indexmap::map::slice::Slice::split_at_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#529)

#### pub fn [split\_first](#method.split_first)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<((&[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)), &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>)>

Returns the first key-value pair and the rest of the slice, or `None` if it is empty.

Equivalent to [`map::Slice::split_first`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.split_first "method indexmap::map::slice::Slice::split_first").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#543)

#### pub fn [split\_first\_mut](#method.split_first_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<((&[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)), &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>)>

Returns the first key-value pair and the rest of the slice, with mutable access to the value, or `None` if it is empty.

Equivalent to [`map::Slice::split_first_mut`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.split_first_mut "method indexmap::map::slice::Slice::split_first_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#557)

#### pub fn [split\_last](#method.split_last)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<((&[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)), &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>)>

Returns the last key-value pair and the rest of the slice, or `None` if it is empty.

Equivalent to [`map::Slice::split_last`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.split_last "method indexmap::map::slice::Slice::split_last").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#571)

#### pub fn [split\_last\_mut](#method.split_last_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<((&[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)), &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>)>

Returns the last key-value pair and the rest of the slice, with mutable access to the value, or `None` if it is empty.

Equivalent to [`map::Slice::split_last_mut`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.split_last_mut "method indexmap::map::slice::Slice::split_last_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#584)

#### pub fn [iter](#method.iter)(&self) -> [Iter](struct.Iter.html "struct bevy::ecs::entity::index_map::Iter")<'\_, V> [ⓘ](#)

Return an iterator over the key-value pairs of the map slice.

Equivalent to [`map::Slice::iter`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.iter "method indexmap::map::slice::Slice::iter").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#591)

#### pub fn [iter\_mut](#method.iter_mut)(&mut self) -> [IterMut](struct.IterMut.html "struct bevy::ecs::entity::index_map::IterMut")<'\_, V> [ⓘ](#)

Return an iterator over the key-value pairs of the map slice.

Equivalent to [`map::Slice::iter_mut`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.iter_mut "method indexmap::map::slice::Slice::iter_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#598)

#### pub fn [keys](#method.keys)(&self) -> [Keys](struct.Keys.html "struct bevy::ecs::entity::index_map::Keys")<'\_, V> [ⓘ](#)

Return an iterator over the keys of the map slice.

Equivalent to [`map::Slice::keys`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.keys "method indexmap::map::slice::Slice::keys").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#605)

#### pub fn [into\_keys](#method.into_keys)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>) -> [IntoKeys](struct.IntoKeys.html "struct bevy::ecs::entity::index_map::IntoKeys")<V> [ⓘ](#)

Return an owning iterator over the keys of the map slice.

Equivalent to [`map::Slice::into_keys`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.into_keys "method indexmap::map::slice::Slice::into_keys").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#612)

#### pub fn [values\_mut](#method.values_mut)(&mut self) -> [ValuesMut](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.ValuesMut.html "struct indexmap::map::iter::ValuesMut")<'\_, [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V> [ⓘ](#)

Return an iterator over mutable references to the values of the map slice.

Equivalent to [`map::Slice::values_mut`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.values_mut "method indexmap::map::slice::Slice::values_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#619)

#### pub fn [into\_values](#method.into_values)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>) -> [IntoValues](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.IntoValues.html "struct indexmap::map::iter::IntoValues")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V> [ⓘ](#)

Return an owning iterator over the values of the map slice.

Equivalent to [`map::Slice::into_values`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.into_values "method indexmap::map::slice::Slice::into_values").

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V>>

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#64)

#### pub fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Return the number of key-value pairs in the map slice.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#70)

#### pub fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the map slice contains no elements.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#77)

#### pub fn [get\_index](#method.get_index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Get a key-value pair by index.

Valid indices are `0 <= index < self.len()`.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#91)

#### pub fn [get\_range](#method.get_range-1)<R>(&self, range: R) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<K, V>>

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Returns a slice of key-value pairs in the given range of indices.

Valid indices are `0 <= index < self.len()`.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#105)

#### pub fn [first](#method.first)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Get the first key-value pair.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#123)

#### pub fn [last](#method.last)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Get the last key-value pair.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#145)

#### pub fn [split\_at](#method.split_at-1)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<K, V>, &[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<K, V>)

Divides one slice into two at an index.

_**Panics**_ if `index > len`. For a non-panicking alternative see [`split_at_checked`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html#method.split_at_checked "method indexmap::map::slice::Slice::split_at_checked").

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#163)

#### pub fn [split\_at\_checked](#method.split_at_checked)( &self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<K, V>, &[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<K, V>)>

Divides one slice into two at an index.

Returns `None` if `index > len`.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#184)

#### pub fn [split\_first](#method.split_first-1)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)), &[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<K, V>)>

Returns the first key-value pair and the rest of the slice, or `None` if it is empty.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#204)

#### pub fn [split\_last](#method.split_last-1)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)), &[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<K, V>)>

Returns the last key-value pair and the rest of the slice, or `None` if it is empty.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#223)

#### pub fn [iter](#method.iter-1)(&self) -> [Iter](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Iter.html "struct indexmap::map::iter::Iter")<'\_, K, V> [ⓘ](#)

Return an iterator over the key-value pairs of the map slice.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#233)

#### pub fn [keys](#method.keys-1)(&self) -> [Keys](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Keys.html "struct indexmap::map::iter::Keys")<'\_, K, V> [ⓘ](#)

Return an iterator over the keys of the map slice.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#243)

#### pub fn [values](#method.values)(&self) -> [Values](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Values.html "struct indexmap::map::iter::Values")<'\_, K, V> [ⓘ](#)

Return an iterator over the values of the map slice.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#265-267)

#### pub fn [binary\_search\_keys](#method.binary_search_keys)(&self, x: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Search over a sorted map for a key.

Returns the position where that key is present, or the position where it can be inserted to maintain the sort. See [`slice::binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search") for more details.

Computes in **O(log(n))** time, which is notably less scalable than looking the key up in the map this is a slice from using [`IndexMap::get_index_of`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.get_index_of "method indexmap::map::IndexMap::get_index_of"), but this can also position missing keys.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#279-281)

#### pub fn [binary\_search\_by](#method.binary_search_by)<'a, F>(&'a self, f: F) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Search over a sorted map with a comparator function.

Returns the position where that value is present, or the position where it can be inserted to maintain the sort. See [`slice::binary_search_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by "method slice::binary_search_by") for more details.

Computes in **O(log(n))** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#293-296)

#### pub fn [binary\_search\_by\_key](#method.binary_search_by_key)<'a, B, F>( &'a self, b: [&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html), f: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> B, B: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Search over a sorted map with an extraction function.

Returns the position where that value is present, or the position where it can be inserted to maintain the sort. See [`slice::binary_search_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by_key "method slice::binary_search_by_key") for more details.

Computes in **O(log(n))** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#303-305)

#### pub fn [is\_sorted](#method.is_sorted)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where K: [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Checks if the keys of this slice are sorted.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#312-314)

#### pub fn [is\_sorted\_by](#method.is_sorted_by)<'a, F>(&'a self, cmp: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Checks if this slice is sorted using the given comparator function.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#322-325)

#### pub fn [is\_sorted\_by\_key](#method.is_sorted_by_key)<'a, F, T>(&'a self, sort\_key: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> T, T: [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Checks if this slice is sorted using the given sort-key function.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map/slice.rs.html#338-340)

#### pub fn [partition\_point](#method.partition_point)<P>(&self, pred: P) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns the index of the partition point of a sorted map according to the given predicate (the index of the first element of the second partition).

See [`slice::partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point") for more details.

Computes in **O(log(n))** time.

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#641)

### impl<V> [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

where V: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#642)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#632)

### impl<V> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

where V: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#633)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#648)

### impl<V> [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#649)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#655)

### impl<V> [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#656)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#662)

### impl<V> [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#663)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#624)

### impl<V> [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#625)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V>

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#627)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V> as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#727)

### impl<V> [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

where V: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#669)

### impl<V> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>> for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

where V: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#670)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#676)

### impl<V> [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

where V: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#677)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#729)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#730)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#732)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: ([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)) -> &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#738)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#739)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#741)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#747)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#748)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#750)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#756)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")\> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#757)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#759)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")) -> &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#765)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#766)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#768)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#774)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#775)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#777)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#783)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#784)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#786)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#792)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#793)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = V

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#795)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#800)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#801)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, key: ([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)) -> &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#807)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#808)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, key: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#814)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#815)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, key: [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#821)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")\> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#822)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, key: [RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")) -> &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#828)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#829)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, key: [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#835)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#836)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, key: [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#842)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#843)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, key: [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#849)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#850)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, key: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#682)

### impl<'a, V> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'a [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#683)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = (&'a [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#684)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [Iter](struct.Iter.html "struct bevy::ecs::entity::index_map::Iter")<'a, V>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#686)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <&'a [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#691)

### impl<'a, V> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'a mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#692)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = (&'a [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&'a mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#693)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [IterMut](struct.IterMut.html "struct bevy::ecs::entity::index_map::IterMut")<'a, V>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#695)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <&'a mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#700)

### impl<V> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#701)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = ([Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V)

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#702)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [IntoIter](struct.IntoIter.html "struct bevy::ecs::entity::index_map::IntoIter")<V>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#704)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#715)

### impl<V> [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

where V: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#716)

#### fn [cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)(&self, other: &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#721)

### impl<V> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

where V: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#722)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#709)

### impl<V> [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

where V: [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#710)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1410)

#### fn [lt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1428)

#### fn [le](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1446)

#### fn [gt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1464)

#### fn [ge](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

## Auto Trait Implementations

### impl<V, S = [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\> ![Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V, S>

### impl<V, S> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V, S>

where V: [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze"),

### impl<V, S> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V, S>

where S: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"), V: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"),

### impl<V, S> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V, S>

where S: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"), V: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

### impl<V, S> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V, S>

where S: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

### impl<V, S> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V, S>

where S: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), V: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

### impl<V, S> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V, S>

where V: [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin"),

### impl<V, S> [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V, S>

where S: [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"), V: [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#104-107)

### impl<Q, K> [Comparable](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Comparable.html "trait equivalent::Comparable")<K> for Q

where Q: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#110)

#### fn [compare](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Comparable.html#tymethod.compare)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

Compare self to `key` and return their ordering.

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)

### impl<Q, K> [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)

#### fn [equivalent](../../../platform/collections/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Compare self to `key` and return `true` if they are equal.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#151-154)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#156)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

{"IntoKeys<V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.IntoKeys.html\\" title=\\"struct bevy::ecs::entity::index\_map::IntoKeys\\">IntoKeys</a>&lt;V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.IntoKeys.html\\" title=\\"struct bevy::ecs::entity::index\_map::IntoKeys\\">IntoKeys</a>&lt;V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>;</div>","IntoValues<Entity, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.IntoValues.html\\" title=\\"struct indexmap::map::iter::IntoValues\\">IntoValues</a>&lt;K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.IntoValues.html\\" title=\\"struct indexmap::map::iter::IntoValues\\">IntoValues</a>&lt;K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = V;</div>","Iter<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Iter.html\\" title=\\"struct indexmap::map::iter::Iter\\">Iter</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Iter.html\\" title=\\"struct indexmap::map::iter::Iter\\">Iter</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a K</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a V</a>);</div>","Iter<'\_, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Iter.html\\" title=\\"struct bevy::ecs::entity::index\_map::Iter\\">Iter</a>&lt;'a, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.Iter.html\\" title=\\"struct bevy::ecs::entity::index\_map::Iter\\">Iter</a>&lt;'a, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a V</a>);</div>","IterMut<'\_, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.IterMut.html\\" title=\\"struct bevy::ecs::entity::index\_map::IterMut\\">IterMut</a>&lt;'a, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.IterMut.html\\" title=\\"struct bevy::ecs::entity::index\_map::IterMut\\">IterMut</a>&lt;'a, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a mut V</a>);</div>","Keys<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Keys.html\\" title=\\"struct indexmap::map::iter::Keys\\">Keys</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Keys.html\\" title=\\"struct indexmap::map::iter::Keys\\">Keys</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a K</a>;</div>","Keys<'\_, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Keys.html\\" title=\\"struct bevy::ecs::entity::index\_map::Keys\\">Keys</a>&lt;'a, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.Keys.html\\" title=\\"struct bevy::ecs::entity::index\_map::Keys\\">Keys</a>&lt;'a, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>;</div>","Values<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Values.html\\" title=\\"struct indexmap::map::iter::Values\\">Values</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Values.html\\" title=\\"struct indexmap::map::iter::Values\\">Values</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a V</a>;</div>","ValuesMut<'\_, Entity, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.ValuesMut.html\\" title=\\"struct indexmap::map::iter::ValuesMut\\">ValuesMut</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.ValuesMut.html\\" title=\\"struct indexmap::map::iter::ValuesMut\\">ValuesMut</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a mut V</a>;</div>"}