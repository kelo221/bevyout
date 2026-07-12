[bevy](../../index.html)::[ecs](../index.html)::[entity](index.html)

# Struct UniqueEntityEquivalentSlice 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#40)

```rust
pub struct UniqueEntityEquivalentSlice<T>(/* private fields */)
where
    T: EntityEquivalent;
```

A slice that contains only unique entities.

This can be obtained by slicing [`UniqueEntityEquivalentVec`](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec").

When `T` is [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), use [`UniqueEntitySlice`](type.UniqueEntitySlice.html "type bevy::ecs::entity::UniqueEntitySlice").

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#47)

### impl<T> [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#53)

#### pub const unsafe fn [from\_slice\_unchecked](#method.from_slice_unchecked)( slice: &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), ) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Constructs a `UniqueEntityEquivalentSlice` from a \[`&[T]`\] unsafely.

##### Safety

`slice` must contain only unique elements.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#63)

#### pub const unsafe fn [from\_slice\_unchecked\_mut](#method.from_slice_unchecked_mut)( slice: &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), ) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Constructs a `UniqueEntityEquivalentSlice` from a \[`&mut [T]`\] unsafely.

##### Safety

`slice` must contain only unique elements.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#69)

#### pub const fn [as\_inner](#method.as_inner)(&self) -> &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

Casts to `self` to a standard slice.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#78)

#### pub unsafe fn [from\_boxed\_slice\_unchecked](#method.from_boxed_slice_unchecked)( slice: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>, ) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

Constructs a `UniqueEntityEquivalentSlice` from a \[`Box<[T]>`\] unsafely.

##### Safety

`slice` must contain only unique elements.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#84)

#### pub fn [into\_boxed\_inner](#method.into_boxed_inner)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

Casts `self` to the inner slice.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#94)

#### pub unsafe fn [from\_arc\_slice\_unchecked](#method.from_arc_slice_unchecked)( slice: [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>, ) -> [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> [ⓘ](#)

Constructs a `UniqueEntityEquivalentSlice` from a \[`Arc<[T]>`\] unsafely.

##### Safety

`slice` must contain only unique elements.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#100)

#### pub fn [into\_arc\_inner](#method.into_arc_inner)(this: [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>) -> [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\> [ⓘ](#)

Casts `self` to the inner slice.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#110)

#### pub unsafe fn [from\_rc\_slice\_unchecked](#method.from_rc_slice_unchecked)( slice: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>, ) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

##### Safety

`slice` must contain only unique elements.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#116)

#### pub fn [into\_rc\_inner](#method.into_rc_inner)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

Casts `self` to the inner slice.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#124)

#### pub const fn [split\_first](#method.split_first)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>)>

Returns the first and all the rest of the elements of the slice, or `None` if it is empty.

Equivalent to [`[T]::split_first`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_first "method slice::split_first").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#135)

#### pub const fn [split\_last](#method.split_last)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>)>

Returns the last and all the rest of the elements of the slice, or `None` if it is empty.

Equivalent to [`[T]::split_last`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_last "method slice::split_last").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#146)

#### pub const fn [first\_chunk](#method.first_chunk)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &self, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>>

Returns an array reference to the first `N` items in the slice.

Equivalent to [`[T]::first_chunk`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.first_chunk "method slice::first_chunk").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#157-162)

#### pub const fn [split\_first\_chunk](#method.split_first_chunk)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &self, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>, &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>)>

Returns an array reference to the first `N` items in the slice and the remaining slice.

Equivalent to [`[T]::split_first_chunk`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_first_chunk "method slice::split_first_chunk").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#178-183)

#### pub const fn [split\_last\_chunk](#method.split_last_chunk)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &self, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, &[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>)>

Returns an array reference to the last `N` items in the slice and the remaining slice.

Equivalent to [`[T]::split_last_chunk`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_last_chunk "method slice::split_last_chunk").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#199)

#### pub const fn [last\_chunk](#method.last_chunk)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &self, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>>

Returns an array reference to the last `N` items in the slice.

Equivalent to [`[T]::last_chunk`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.last_chunk "method slice::last_chunk").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#214-217)

#### pub fn [get](#method.get)<I>(&self, index: I) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>: [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<I>, I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), Output = [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>,

Returns a reference to a subslice.

Equivalent to the range functionality of \[`[T]::get`\].

Note that only the inner \[`[T]::get`\] supports indexing with a [`usize`](https://doc.rust-lang.org/nightly/std/primitive.usize.html "primitive usize").

\[`[T]::get`\]: `slice::get`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#231-234)

#### pub fn [get\_mut](#method.get_mut)<I>( &mut self, index: I, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>: [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<I>, I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), Output = [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>,

Returns a mutable reference to a subslice.

Equivalent to the range functionality of \[`[T]::get_mut`\].

Note that `UniqueEntityEquivalentSlice::get_mut` cannot be called with a [`usize`](https://doc.rust-lang.org/nightly/std/primitive.usize.html "primitive usize").

\[`[T]::get_mut`\]: `slice::get_mut`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#252-255)

#### pub unsafe fn [get\_unchecked](#method.get_unchecked)<I>( &self, index: I, ) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>: [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<I>, I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), Output = [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>,

Returns a reference to a subslice, without doing bounds checking.

Equivalent to the range functionality of \[`[T]::get_unchecked`\].

Note that only the inner \[`[T]::get_unchecked`\] supports indexing with a [`usize`](https://doc.rust-lang.org/nightly/std/primitive.usize.html "primitive usize").

##### Safety

`index` must be safe to use with \[`[T]::get_unchecked`\]

\[`[T]::get_unchecked`\]: `slice::get_unchecked`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#271-274)

#### pub unsafe fn [get\_unchecked\_mut](#method.get_unchecked_mut)<I>( &mut self, index: I, ) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>: [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<I>, I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), Output = [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>,

Returns a mutable reference to a subslice, without doing bounds checking.

Equivalent to the range functionality of \[`[T]::get_unchecked_mut`\].

Note that `UniqueEntityEquivalentSlice::get_unchecked_mut` cannot be called with an index.

##### Safety

`index` must be safe to use with \[`[T]::get_unchecked_mut`\]

\[`[T]::get_unchecked_mut`\]: `slice::get_unchecked_mut`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#281)

#### pub const fn [as\_mut\_ptr](#method.as_mut_ptr)(&mut self) -> [\*mut T](https://doc.rust-lang.org/nightly/std/primitive.pointer.html)

Returns an unsafe mutable pointer to the slice’s buffer.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#286)

#### pub const fn [as\_mut\_ptr\_range](#method.as_mut_ptr_range)(&mut self) -> [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[\*mut T](https://doc.rust-lang.org/nightly/std/primitive.pointer.html)\> [ⓘ](#)

Returns the two unsafe mutable pointers spanning the slice.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#291)

#### pub fn [swap](#method.swap)(&mut self, a: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), b: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Swaps two elements in the slice.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#296)

#### pub fn [reverse](#method.reverse)(&mut self)

Reverses the order of elements in the slice, in place.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#301)

#### pub fn [iter](#method.iter)(&self) -> [UniqueEntityIter](struct.UniqueEntityIter.html "struct bevy::ecs::entity::UniqueEntityIter")<[Iter](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html "struct core::slice::iter::Iter")<'\_, T>> [ⓘ](#)

Returns an iterator over the slice.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#312)

#### pub fn [windows](#method.windows)( &self, size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [UniqueEntityEquivalentSliceIter](unique_slice/struct.UniqueEntityEquivalentSliceIter.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIter")<'\_, T, [Windows](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Windows.html "struct core::slice::iter::Windows")<'\_, T>> [ⓘ](#)

Returns an iterator over all contiguous windows of length `size`.

Equivalent to \[`[T]::windows`\].

\[`[T]::windows`\]: `slice::windows`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#323)

#### pub fn [chunks](#method.chunks)( &self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [UniqueEntityEquivalentSliceIter](unique_slice/struct.UniqueEntityEquivalentSliceIter.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIter")<'\_, T, [Chunks](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Chunks.html "struct core::slice::iter::Chunks")<'\_, T>> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

Equivalent to \[`[T]::chunks`\].

\[`[T]::chunks`\]: `slice::chunks`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#336)

#### pub fn [chunks\_mut](#method.chunks_mut)( &mut self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [UniqueEntityEquivalentSliceIterMut](unique_slice/struct.UniqueEntityEquivalentSliceIterMut.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIterMut")<'\_, T, [ChunksMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksMut.html "struct core::slice::iter::ChunksMut")<'\_, T>> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

Equivalent to \[`[T]::chunks_mut`\].

\[`[T]::chunks_mut`\]: `slice::chunks_mut`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#351)

#### pub fn [chunks\_exact](#method.chunks_exact)( &self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [UniqueEntityEquivalentSliceIter](unique_slice/struct.UniqueEntityEquivalentSliceIter.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIter")<'\_, T, [ChunksExact](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExact.html "struct core::slice::iter::ChunksExact")<'\_, T>> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

Equivalent to \[`[T]::chunks_exact`\].

\[`[T]::chunks_exact`\]: `slice::chunks_exact`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#366)

#### pub fn [chunks\_exact\_mut](#method.chunks_exact_mut)( &mut self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [UniqueEntityEquivalentSliceIterMut](unique_slice/struct.UniqueEntityEquivalentSliceIterMut.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIterMut")<'\_, T, [ChunksExactMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExactMut.html "struct core::slice::iter::ChunksExactMut")<'\_, T>> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

Equivalent to \[`[T]::chunks_exact_mut`\].

\[`[T]::chunks_exact_mut`\]: `slice::chunks_exact_mut`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#381)

#### pub fn [rchunks](#method.rchunks)( &self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [UniqueEntityEquivalentSliceIter](unique_slice/struct.UniqueEntityEquivalentSliceIter.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIter")<'\_, T, [RChunks](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunks.html "struct core::slice::iter::RChunks")<'\_, T>> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

Equivalent to \[`[T]::rchunks`\].

\[`[T]::rchunks`\]: `slice::rchunks`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#394)

#### pub fn [rchunks\_mut](#method.rchunks_mut)( &mut self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [UniqueEntityEquivalentSliceIterMut](unique_slice/struct.UniqueEntityEquivalentSliceIterMut.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIterMut")<'\_, T, [RChunksMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksMut.html "struct core::slice::iter::RChunksMut")<'\_, T>> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

Equivalent to \[`[T]::rchunks_mut`\].

\[`[T]::rchunks_mut`\]: `slice::rchunks_mut`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#409)

#### pub fn [rchunks\_exact](#method.rchunks_exact)( &self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [UniqueEntityEquivalentSliceIter](unique_slice/struct.UniqueEntityEquivalentSliceIter.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIter")<'\_, T, [RChunksExact](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExact.html "struct core::slice::iter::RChunksExact")<'\_, T>> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

Equivalent to \[`[T]::rchunks_exact`\].

\[`[T]::rchunks_exact`\]: `slice::rchunks_exact`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#424)

#### pub fn [rchunks\_exact\_mut](#method.rchunks_exact_mut)( &mut self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [UniqueEntityEquivalentSliceIterMut](unique_slice/struct.UniqueEntityEquivalentSliceIterMut.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIterMut")<'\_, T, [RChunksExactMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExactMut.html "struct core::slice::iter::RChunksExactMut")<'\_, T>> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

Equivalent to \[`[T]::rchunks_exact_mut`\].

\[`[T]::rchunks_exact_mut`\]: `slice::rchunks_exact_mut`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#439-441)

#### pub fn [chunk\_by](#method.chunk_by)<F>( &self, pred: F, ) -> [UniqueEntityEquivalentSliceIter](unique_slice/struct.UniqueEntityEquivalentSliceIter.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIter")<'\_, T, [ChunkBy](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkBy.html "struct core::slice::iter::ChunkBy")<'\_, T, F>> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over the slice producing non-overlapping runs of elements using the predicate to separate them.

Equivalent to \[`[T]::chunk_by`\].

\[`[T]::chunk_by`\]: `slice::chunk_by`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#453-455)

#### pub fn [chunk\_by\_mut](#method.chunk_by_mut)<F>( &mut self, pred: F, ) -> [UniqueEntityEquivalentSliceIterMut](unique_slice/struct.UniqueEntityEquivalentSliceIterMut.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIterMut")<'\_, T, [ChunkByMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkByMut.html "struct core::slice::iter::ChunkByMut")<'\_, T, F>> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over the slice producing non-overlapping mutable runs of elements using the predicate to separate them.

Equivalent to \[`[T]::chunk_by_mut`\].

\[`[T]::chunk_by_mut`\]: `slice::chunk_by_mut`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#468)

#### pub const fn [split\_at](#method.split_at)( &self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> (&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>)

Divides one slice into two at an index.

Equivalent to [`[T]::split_at`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at "method slice::split_at").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#482)

#### pub const fn [split\_at\_mut](#method.split_at_mut)( &mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> (&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>)

Divides one mutable slice into two at an index.

Equivalent to [`[T]::split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut "method slice::split_at_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#502)

#### pub const unsafe fn [split\_at\_unchecked](#method.split_at_unchecked)( &self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> (&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>)

Divides one slice into two at an index, without doing bounds checking.

Equivalent to [`[T]::split_at_unchecked`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_unchecked "method slice::split_at_unchecked").

##### Safety

`mid` must be safe to use in \[`[T]::split_at_unchecked`\].

\[`[T]::split_at_unchecked`\]: `slice::split_at_unchecked`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#523)

#### pub const unsafe fn [split\_at\_mut\_unchecked](#method.split_at_mut_unchecked)( &mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> (&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>)

Divides one mutable slice into two at an index, without doing bounds checking.

Equivalent to [`[T]::split_at_mut_unchecked`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut_unchecked "method slice::split_at_mut_unchecked").

##### Safety

`mid` must be safe to use in \[`[T]::split_at_mut_unchecked`\].

\[`[T]::split_at_mut_unchecked`\]: `slice::split_at_mut_unchecked`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#539)

#### pub const fn [split\_at\_checked](#method.split_at_checked)( &self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>)>

Divides one slice into two at an index, returning `None` if the slice is too short.

Equivalent to [`[T]::split_at_checked`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_checked "method slice::split_at_checked").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#556)

#### pub const fn [split\_at\_mut\_checked](#method.split_at_mut_checked)( &mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>)>

Divides one mutable slice into two at an index, returning `None` if the slice is too short.

Equivalent to [`[T]::split_at_mut_checked`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut_checked "method slice::split_at_mut_checked").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#575-577)

#### pub fn [split](#method.split)<F>( &self, pred: F, ) -> [UniqueEntityEquivalentSliceIter](unique_slice/struct.UniqueEntityEquivalentSliceIter.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIter")<'\_, T, [Split](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Split.html "struct core::slice::iter::Split")<'\_, T, F>> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over subslices separated by elements that match `pred`.

Equivalent to \[`[T]::split`\].

\[`[T]::split`\]: `slice::split`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#589-591)

#### pub fn [split\_mut](#method.split_mut)<F>( &mut self, pred: F, ) -> [UniqueEntityEquivalentSliceIterMut](unique_slice/struct.UniqueEntityEquivalentSliceIterMut.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIterMut")<'\_, T, [SplitMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitMut.html "struct core::slice::iter::SplitMut")<'\_, T, F>> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over mutable subslices separated by elements that match `pred`.

Equivalent to \[`[T]::split_mut`\].

\[`[T]::split_mut`\]: `slice::split_mut`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#607-609)

#### pub fn [split\_inclusive](#method.split_inclusive)<F>( &self, pred: F, ) -> [UniqueEntityEquivalentSliceIter](unique_slice/struct.UniqueEntityEquivalentSliceIter.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIter")<'\_, T, [SplitInclusive](https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusive.html "struct core::slice::iter::SplitInclusive")<'\_, T, F>> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over subslices separated by elements that match `pred`.

Equivalent to \[`[T]::split_inclusive`\].

\[`[T]::split_inclusive`\]: `slice::split_inclusive`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#623-625)

#### pub fn [split\_inclusive\_mut](#method.split_inclusive_mut)<F>( &mut self, pred: F, ) -> [UniqueEntityEquivalentSliceIterMut](unique_slice/struct.UniqueEntityEquivalentSliceIterMut.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIterMut")<'\_, T, [SplitInclusiveMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusiveMut.html "struct core::slice::iter::SplitInclusiveMut")<'\_, T, F>> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over mutable subslices separated by elements that match `pred`.

Equivalent to \[`[T]::split_inclusive_mut`\].

\[`[T]::split_inclusive_mut`\]: `slice::split_inclusive_mut`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#641-643)

#### pub fn [rsplit](#method.rsplit)<F>( &self, pred: F, ) -> [UniqueEntityEquivalentSliceIter](unique_slice/struct.UniqueEntityEquivalentSliceIter.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIter")<'\_, T, [RSplit](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplit.html "struct core::slice::iter::RSplit")<'\_, T, F>> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over subslices separated by elements that match `pred`, starting at the end of the slice and working backwards.

Equivalent to \[`[T]::rsplit`\].

\[`[T]::rsplit`\]: `slice::rsplit`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#656-658)

#### pub fn [rsplit\_mut](#method.rsplit_mut)<F>( &mut self, pred: F, ) -> [UniqueEntityEquivalentSliceIterMut](unique_slice/struct.UniqueEntityEquivalentSliceIterMut.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIterMut")<'\_, T, [RSplitMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitMut.html "struct core::slice::iter::RSplitMut")<'\_, T, F>> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over mutable subslices separated by elements that match `pred`, starting at the end of the slice and working backwards.

Equivalent to \[`[T]::rsplit_mut`\].

\[`[T]::rsplit_mut`\]: `slice::rsplit_mut`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#674-676)

#### pub fn [splitn](#method.splitn)<F>( &self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pred: F, ) -> [UniqueEntityEquivalentSliceIter](unique_slice/struct.UniqueEntityEquivalentSliceIter.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIter")<'\_, T, [SplitN](https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitN.html "struct core::slice::iter::SplitN")<'\_, T, F>> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over subslices separated by elements that match `pred`, limited to returning at most `n` items.

Equivalent to \[`[T]::splitn`\].

\[`[T]::splitn`\]: `slice::splitn`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#690-692)

#### pub fn [splitn\_mut](#method.splitn_mut)<F>( &mut self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pred: F, ) -> [UniqueEntityEquivalentSliceIterMut](unique_slice/struct.UniqueEntityEquivalentSliceIterMut.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIterMut")<'\_, T, [SplitNMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitNMut.html "struct core::slice::iter::SplitNMut")<'\_, T, F>> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over mutable subslices separated by elements that match `pred`, limited to returning at most `n` items.

Equivalent to \[`[T]::splitn_mut`\].

\[`[T]::splitn_mut`\]: `slice::splitn_mut`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#708-710)

#### pub fn [rsplitn](#method.rsplitn)<F>( &self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pred: F, ) -> [UniqueEntityEquivalentSliceIter](unique_slice/struct.UniqueEntityEquivalentSliceIter.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIter")<'\_, T, [RSplitN](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitN.html "struct core::slice::iter::RSplitN")<'\_, T, F>> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over subslices separated by elements that match `pred` limited to returning at most `n` items.

Equivalent to \[`[T]::rsplitn`\].

\[`[T]::rsplitn`\]: `slice::rsplitn`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#724-726)

#### pub fn [rsplitn\_mut](#method.rsplitn_mut)<F>( &mut self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pred: F, ) -> [UniqueEntityEquivalentSliceIterMut](unique_slice/struct.UniqueEntityEquivalentSliceIterMut.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIterMut")<'\_, T, [RSplitNMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitNMut.html "struct core::slice::iter::RSplitNMut")<'\_, T, F>> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over subslices separated by elements that match `pred` limited to returning at most `n` items.

Equivalent to \[`[T]::rsplitn_mut`\].

\[`[T]::rsplitn_mut`\]: `slice::rsplitn_mut`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#739-741)

#### pub fn [sort\_unstable](#method.sort_unstable)(&mut self)

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Sorts the slice **without** preserving the initial order of equal elements.

Equivalent to [`[T]::sort_unstable`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable "method slice::sort_unstable").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#750-752)

#### pub fn [sort\_unstable\_by](#method.sort_unstable_by)<F>(&mut self, compare: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Sorts the slice with a comparison function, **without** preserving the initial order of equal elements.

Equivalent to [`[T]::sort_unstable_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable_by "method slice::sort_unstable_by").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#761-764)

#### pub fn [sort\_unstable\_by\_key](#method.sort_unstable_by_key)<K, F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> K, K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Sorts the slice with a key extraction function, **without** preserving the initial order of equal elements.

Equivalent to [`[T]::sort_unstable_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable_by_key "method slice::sort_unstable_by_key").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#774)

#### pub fn [rotate\_left](#method.rotate_left)(&mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Rotates the slice in-place such that the first `mid` elements of the slice move to the end while the last `self.len() - mid` elements move to the front.

Equivalent to [`[T]::rotate_left`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rotate_left "method slice::rotate_left").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#783)

#### pub fn [rotate\_right](#method.rotate_right)(&mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Rotates the slice in-place such that the first `self.len() - k` elements of the slice move to the end while the last `k` elements move to the front.

Equivalent to [`[T]::rotate_right`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rotate_right "method slice::rotate_right").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#790-792)

#### pub fn [sort](#method.sort)(&mut self)

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Sorts the slice, preserving initial order of equal elements.

Equivalent to [`[T]::sort`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort "method slice::sort").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#800-802)

#### pub fn [sort\_by](#method.sort_by)<F>(&mut self, compare: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Sorts the slice with a comparison function, preserving initial order of equal elements.

Equivalent to [`[T]::sort_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_by "method slice::sort_by").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#810-813)

#### pub fn [sort\_by\_key](#method.sort_by_key)<K, F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> K, K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Sorts the slice with a key extraction function, preserving initial order of equal elements.

Equivalent to [`[T]::sort_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_by_key "method slice::sort_by_key").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#821-824)

#### pub fn [sort\_by\_cached\_key](#method.sort_by_cached_key)<K, F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> K, K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Equivalent to [`[T]::sort_by_cached_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_by_cached_key "method slice::sort_by_cached_key").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#830-832)

#### pub fn [to\_vec](#method.to_vec)(&self) -> [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Copies self into a new `UniqueEntityEquivalentVec`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#841)

#### pub fn [into\_vec](#method.into_vec)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>, ) -> [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

Converts `self` into a vector without clones or allocation.

Equivalent to [`[T]::into_vec`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.into_vec "method slice::into_vec").

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#116)

#### pub fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements in the slice.

##### Examples

```rust
let a = [1, 2, 3];
assert_eq!(a.len(), 3);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#136)

#### pub fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the slice has a length of 0.

##### Examples

```rust
let a = [1, 2, 3];
assert!(!a.is_empty());

let b: &[i32] = &[];
assert!(b.is_empty());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#155)

#### pub fn [first](#method.first)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Returns the first element of the slice, or `None` if it is empty.

##### Examples

```rust
let v = [10, 40, 30];
assert_eq!(Some(&10), v.first());

let w: &[i32] = &[];
assert_eq!(None, w.first());
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#198)

#### pub fn [split\_first](#method.split_first-1)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

Returns the first and all the rest of the elements of the slice, or `None` if it is empty.

##### Examples

```rust
let x = &[0, 1, 2];

if let Some((first, elements)) = x.split_first() {
    assert_eq!(first, &0);
    assert_eq!(elements, &[1, 2]);
}
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#240)

#### pub fn [split\_last](#method.split_last-1)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

Returns the last and all the rest of the elements of the slice, or `None` if it is empty.

##### Examples

```rust
let x = &[0, 1, 2];

if let Some((last, elements)) = x.split_last() {
    assert_eq!(last, &2);
    assert_eq!(elements, &[0, 1]);
}
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#281)

#### pub fn [last](#method.last)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Returns the last element of the slice, or `None` if it is empty.

##### Examples

```rust
let v = [10, 40, 30];
assert_eq!(Some(&30), v.last());

let w: &[i32] = &[];
assert_eq!(None, w.last());
```

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#327)

#### pub fn [first\_chunk](#method.first_chunk-1)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>

Returns an array reference to the first `N` items in the slice.

If the slice is not at least `N` in length, this will return `None`.

##### Examples

```rust
let u = [10, 40, 30];
assert_eq!(Some(&[10, 40]), u.first_chunk::<2>());

let v: &[i32] = &[10];
assert_eq!(None, v.first_chunk::<2>());

let w: &[i32] = &[];
assert_eq!(Some(&[]), w.first_chunk::<0>());
```

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#387)

#### pub fn [split\_first\_chunk](#method.split_first_chunk-1)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

Returns an array reference to the first `N` items in the slice and the remaining slice.

If the slice is not at least `N` in length, this will return `None`.

##### Examples

```rust
let x = &[0, 1, 2];

if let Some((first, elements)) = x.split_first_chunk::<2>() {
    assert_eq!(first, &[0, 1]);
    assert_eq!(elements, &[2]);
}

assert_eq!(None, x.split_first_chunk::<4>());
```

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#447)

#### pub fn [split\_last\_chunk](#method.split_last_chunk-1)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html))>

Returns an array reference to the last `N` items in the slice and the remaining slice.

If the slice is not at least `N` in length, this will return `None`.

##### Examples

```rust
let x = &[0, 1, 2];

if let Some((elements, last)) = x.split_last_chunk::<2>() {
    assert_eq!(elements, &[0]);
    assert_eq!(last, &[1, 2]);
}

assert_eq!(None, x.split_last_chunk::<4>());
```

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#509)

#### pub fn [last\_chunk](#method.last_chunk-1)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>

Returns an array reference to the last `N` items in the slice.

If the slice is not at least `N` in length, this will return `None`.

##### Examples

```rust
let u = [10, 40, 30];
assert_eq!(Some(&[40, 30]), u.last_chunk::<2>());

let v: &[i32] = &[10];
assert_eq!(None, v.last_chunk::<2>());

let w: &[i32] = &[];
assert_eq!(Some(&[]), w.last_chunk::<0>());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#572-574)

#### pub fn [get](#method.get-1)<I>(&self, index: I) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&<I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")\>

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>,

Returns a reference to an element or subslice depending on the type of index.

*   If given a position, returns a reference to the element at that position or `None` if out of bounds.
*   If given a range, returns the subslice corresponding to that range, or `None` if out of bounds.

##### Examples

```rust
let v = [10, 40, 30];
assert_eq!(Some(&40), v.get(1));
assert_eq!(Some(&[10, 40][..]), v.get(0..2));
assert_eq!(None, v.get(3));
assert_eq!(None, v.get(0..4));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#640-642)

#### pub unsafe fn [get\_unchecked](#method.get_unchecked-1)<I>( &self, index: I, ) -> &<I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>,

Returns a reference to an element or subslice, without doing bounds checking.

For a safe alternative see [`get`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.get "method slice::get").

##### Safety

Calling this method with an out-of-bounds index is _[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)_ even if the resulting reference is not used.

You can think of this like `.get(index).unwrap_unchecked()`. It’s UB to call `.get_unchecked(len)`, even if you immediately convert to a pointer. And it’s UB to call `.get_unchecked(..len + 1)`, `.get_unchecked(..=len)`, or similar.

##### Examples

```rust
let x = &[1, 2, 4];

unsafe {
    assert_eq!(x.get_unchecked(1), &2);
}
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#728)

#### pub fn [as\_ptr](#method.as_ptr)(&self) -> [\*const T](https://doc.rust-lang.org/nightly/std/primitive.pointer.html)

Returns a raw pointer to the slice’s buffer.

The caller must ensure that the slice outlives the pointer this function returns, or else it will end up dangling.

The caller must also ensure that the memory the pointer (non-transitively) points to is never written to (except inside an `UnsafeCell`) using this pointer or any pointer derived from it. If you need to mutate the contents of the slice, use [`as_mut_ptr`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_mut_ptr "method slice::as_mut_ptr").

Modifying the container referenced by this slice may cause its buffer to be reallocated, which would also make any pointers to it invalid.

##### Examples

```rust
let x = &[1, 2, 4];
let x_ptr = x.as_ptr();

unsafe {
    for i in 0..x.len() {
        assert_eq!(x.get_unchecked(i), &*x_ptr.add(i));
    }
}
```

1.48.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#796)

#### pub fn [as\_ptr\_range](#method.as_ptr_range)(&self) -> [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[\*const T](https://doc.rust-lang.org/nightly/std/primitive.pointer.html)\> [ⓘ](#)

Returns the two raw pointers spanning the slice.

The returned range is half-open, which means that the end pointer points _one past_ the last element of the slice. This way, an empty slice is represented by two equal pointers, and the difference between the two pointers represents the size of the slice.

See [`as_ptr`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_ptr "method slice::as_ptr") for warnings on using these pointers. The end pointer requires extra caution, as it does not point to a valid element in the slice.

This function is useful for interacting with foreign interfaces which use two pointers to refer to a range of elements in memory, as is common in C++.

It can also be useful to check if a pointer to an element refers to an element of this slice:

```rust
let a = [1, 2, 3];
let x = &a[1] as *const _;
let y = &5 as *const _;

assert!(a.as_ptr_range().contains(&x));
assert!(!a.as_ptr_range().contains(&y));
```

1.93.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#853)

#### pub fn [as\_array](#method.as_array)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>

Gets a reference to the underlying array.

If `N` is not exactly equal to the length of `self`, then this method returns `None`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1043)

#### pub fn [iter](#method.iter-1)(&self) -> [Iter](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html "struct core::slice::iter::Iter")<'\_, T> [ⓘ](#)

Returns an iterator over the slice.

The iterator yields all items from start to end.

##### Examples

```rust
let x = &[1, 2, 4];
let mut iterator = x.iter();

assert_eq!(iterator.next(), Some(&1));
assert_eq!(iterator.next(), Some(&2));
assert_eq!(iterator.next(), Some(&4));
assert_eq!(iterator.next(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1118)

#### pub fn [windows](#method.windows-1)(&self, size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Windows](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Windows.html "struct core::slice::iter::Windows")<'\_, T> [ⓘ](#)

Returns an iterator over all contiguous windows of length `size`. The windows overlap. If the slice is shorter than `size`, the iterator returns no values.

##### Panics

Panics if `size` is zero.

##### Examples

```rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.windows(3);
assert_eq!(iter.next().unwrap(), &['l', 'o', 'r']);
assert_eq!(iter.next().unwrap(), &['o', 'r', 'e']);
assert_eq!(iter.next().unwrap(), &['r', 'e', 'm']);
assert!(iter.next().is_none());
```

If the slice is shorter than `size`:

```rust
let slice = ['f', 'o', 'o'];
let mut iter = slice.windows(4);
assert!(iter.next().is_none());
```

Because the [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") trait cannot represent the required lifetimes, there is no `windows_mut` analog to `windows`; `[0,1,2].windows_mut(2).collect()` would violate [the rules of references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references) (though a [LendingIterator](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html) analog is possible). You can sometimes use [`Cell::as_slice_of_cells`](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html#method.as_slice_of_cells "method core::cell::Cell::as_slice_of_cells") in conjunction with `windows` instead:

```rust
use std::cell::Cell;

let mut array = ['R', 'u', 's', 't', ' ', '2', '0', '1', '5'];
let slice = &mut array[..];
let slice_of_cells: &[Cell<char>] = Cell::from_mut(slice).as_slice_of_cells();
for w in slice_of_cells.windows(3) {
    Cell::swap(&w[0], &w[2]);
}
assert_eq!(array, ['s', 't', ' ', '2', '0', '1', '5', 'u', 'R']);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1158)

#### pub fn [chunks](#method.chunks-1)(&self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Chunks](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Chunks.html "struct core::slice::iter::Chunks")<'\_, T> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the slice, then the last chunk will not have length `chunk_size`.

See [`chunks_exact`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_exact "method slice::chunks_exact") for a variant of this iterator that returns chunks of always exactly `chunk_size` elements, and [`rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks "method slice::rchunks") for the same iterator but starting at the end of the slice.

If your `chunk_size` is a constant, consider using [`as_chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks "method slice::as_chunks") instead, which will give references to arrays of exactly that length, rather than slices.

##### Panics

Panics if `chunk_size` is zero.

##### Examples

```rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.chunks(2);
assert_eq!(iter.next().unwrap(), &['l', 'o']);
assert_eq!(iter.next().unwrap(), &['r', 'e']);
assert_eq!(iter.next().unwrap(), &['m']);
assert!(iter.next().is_none());
```

1.31.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1245)

#### pub fn [chunks\_exact](#method.chunks_exact-1)(&self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [ChunksExact](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExact.html "struct core::slice::iter::ChunksExact")<'\_, T> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved from the `remainder` function of the iterator.

Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the resulting code better than in the case of [`chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks "method slice::chunks").

See [`chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks "method slice::chunks") for a variant of this iterator that also returns the remainder as a smaller chunk, and [`rchunks_exact`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_exact "method slice::rchunks_exact") for the same iterator but starting at the end of the slice.

If your `chunk_size` is a constant, consider using [`as_chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks "method slice::as_chunks") instead, which will give references to arrays of exactly that length, rather than slices.

##### Panics

Panics if `chunk_size` is zero.

##### Examples

```rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.chunks_exact(2);
assert_eq!(iter.next().unwrap(), &['l', 'o']);
assert_eq!(iter.next().unwrap(), &['r', 'e']);
assert!(iter.next().is_none());
assert_eq!(iter.remainder(), &['m']);
```

1.88.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1341)

#### pub unsafe fn [as\_chunks\_unchecked](#method.as_chunks_unchecked)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> &\[[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Splits the slice into a slice of `N`\-element arrays, assuming that there’s no remainder.

This is the inverse operation to [`as_flattened`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened "method slice::as_flattened").

As this is `unsafe`, consider whether you could use [`as_chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks "method slice::as_chunks") or [`as_rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks "method slice::as_rchunks") instead, perhaps via something like `if let (chunks, []) = slice.as_chunks()` or `let (chunks, []) = slice.as_chunks() else { unreachable!() };`.

##### Safety

This may only be called when

*   The slice splits exactly into `N`\-element chunks (aka `self.len() % N == 0`).
*   `N != 0`.

##### Examples

```rust
let slice: &[char] = &['l', 'o', 'r', 'e', 'm', '!'];
let chunks: &[[char; 1]] =
    // SAFETY: 1-element chunks never have remainder
    unsafe { slice.as_chunks_unchecked() };
assert_eq!(chunks, &[['l'], ['o'], ['r'], ['e'], ['m'], ['!']]);
let chunks: &[[char; 3]] =
    // SAFETY: The slice length (6) is a multiple of 3
    unsafe { slice.as_chunks_unchecked() };
assert_eq!(chunks, &[['l', 'o', 'r'], ['e', 'm', '!']]);

// These would be unsound:
// let chunks: &[[_; 5]] = slice.as_chunks_unchecked() // The slice length is not a multiple of 5
// let chunks: &[[_; 0]] = slice.as_chunks_unchecked() // Zero-length chunks are never allowed
```

1.88.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1399)

#### pub fn [as\_chunks](#method.as_chunks)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> (&\[[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

Splits the slice into a slice of `N`\-element arrays, starting at the beginning of the slice, and a remainder slice with length strictly less than `N`.

The remainder is meaningful in the division sense. Given `let (chunks, remainder) = slice.as_chunks()`, then:

*   `chunks.len()` equals `slice.len() / N`,
*   `remainder.len()` equals `slice.len() % N`, and
*   `slice.len()` equals `chunks.len() * N + remainder.len()`.

You can flatten the chunks back into a slice-of-`T` with [`as_flattened`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened "method slice::as_flattened").

##### Panics

Panics if `N` is zero.

Note that this check is against a const generic parameter, not a runtime value, and thus a particular monomorphization will either always panic or it will never panic.

##### Examples

```rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let (chunks, remainder) = slice.as_chunks();
assert_eq!(chunks, &[['l', 'o'], ['r', 'e']]);
assert_eq!(remainder, &['m']);
```

If you expect the slice to be an exact multiple, you can combine `let`\-`else` with an empty slice pattern:

```rust
let slice = ['R', 'u', 's', 't'];
let (chunks, []) = slice.as_chunks::<2>() else {
    panic!("slice didn't have even length")
};
assert_eq!(chunks, &[['R', 'u'], ['s', 't']]);
```

1.88.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1446)

#### pub fn [as\_rchunks](#method.as_rchunks)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> (&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &\[[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\])

Splits the slice into a slice of `N`\-element arrays, starting at the end of the slice, and a remainder slice with length strictly less than `N`.

The remainder is meaningful in the division sense. Given `let (remainder, chunks) = slice.as_rchunks()`, then:

*   `remainder.len()` equals `slice.len() % N`,
*   `chunks.len()` equals `slice.len() / N`, and
*   `slice.len()` equals `chunks.len() * N + remainder.len()`.

You can flatten the chunks back into a slice-of-`T` with [`as_flattened`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened "method slice::as_flattened").

##### Panics

Panics if `N` is zero.

Note that this check is against a const generic parameter, not a runtime value, and thus a particular monomorphization will either always panic or it will never panic.

##### Examples

```rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let (remainder, chunks) = slice.as_rchunks();
assert_eq!(remainder, &['l']);
assert_eq!(chunks, &[['o', 'r'], ['e', 'm']]);
```

1.94.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1649)

#### pub fn [array\_windows](#method.array_windows)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> [ArrayWindows](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ArrayWindows.html "struct core::slice::iter::ArrayWindows")<'\_, T, N> [ⓘ](#)

Returns an iterator over overlapping windows of `N` elements of a slice, starting at the beginning of the slice.

This is the const generic equivalent of [`windows`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.windows "method slice::windows").

If `N` is greater than the size of the slice, it will return no windows.

##### Panics

Panics if `N` is zero.

Note that this check is against a const generic parameter, not a runtime value, and thus a particular monomorphization will either always panic or it will never panic.

##### Examples

```rust
let slice = [0, 1, 2, 3];
let mut iter = slice.array_windows();
assert_eq!(iter.next().unwrap(), &[0, 1]);
assert_eq!(iter.next().unwrap(), &[1, 2]);
assert_eq!(iter.next().unwrap(), &[2, 3]);
assert!(iter.next().is_none());
```

1.31.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1689)

#### pub fn [rchunks](#method.rchunks-1)(&self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [RChunks](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunks.html "struct core::slice::iter::RChunks")<'\_, T> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the slice, then the last chunk will not have length `chunk_size`.

See [`rchunks_exact`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_exact "method slice::rchunks_exact") for a variant of this iterator that returns chunks of always exactly `chunk_size` elements, and [`chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks "method slice::chunks") for the same iterator but starting at the beginning of the slice.

If your `chunk_size` is a constant, consider using [`as_rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks "method slice::as_rchunks") instead, which will give references to arrays of exactly that length, rather than slices.

##### Panics

Panics if `chunk_size` is zero.

##### Examples

```rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.rchunks(2);
assert_eq!(iter.next().unwrap(), &['e', 'm']);
assert_eq!(iter.next().unwrap(), &['o', 'r']);
assert_eq!(iter.next().unwrap(), &['l']);
assert!(iter.next().is_none());
```

1.31.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1778)

#### pub fn [rchunks\_exact](#method.rchunks_exact-1)(&self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [RChunksExact](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExact.html "struct core::slice::iter::RChunksExact")<'\_, T> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved from the `remainder` function of the iterator.

Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the resulting code better than in the case of [`rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks "method slice::rchunks").

See [`rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks "method slice::rchunks") for a variant of this iterator that also returns the remainder as a smaller chunk, and [`chunks_exact`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_exact "method slice::chunks_exact") for the same iterator but starting at the beginning of the slice.

If your `chunk_size` is a constant, consider using [`as_rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks "method slice::as_rchunks") instead, which will give references to arrays of exactly that length, rather than slices.

##### Panics

Panics if `chunk_size` is zero.

##### Examples

```rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.rchunks_exact(2);
assert_eq!(iter.next().unwrap(), &['e', 'm']);
assert_eq!(iter.next().unwrap(), &['o', 'r']);
assert!(iter.next().is_none());
assert_eq!(iter.remainder(), &['l']);
```

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1867-1869)

#### pub fn [chunk\_by](#method.chunk_by-1)<F>(&self, pred: F) -> [ChunkBy](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkBy.html "struct core::slice::iter::ChunkBy")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over the slice producing non-overlapping runs of elements using the predicate to separate them.

The predicate is called for every pair of consecutive elements, meaning that it is called on `slice[0]` and `slice[1]`, followed by `slice[1]` and `slice[2]`, and so on.

##### Examples

```rust
let slice = &[1, 1, 1, 3, 3, 2, 2, 2];

let mut iter = slice.chunk_by(|a, b| a == b);

assert_eq!(iter.next(), Some(&[1, 1, 1][..]));
assert_eq!(iter.next(), Some(&[3, 3][..]));
assert_eq!(iter.next(), Some(&[2, 2, 2][..]));
assert_eq!(iter.next(), None);
```

This method can be used to extract the sorted subslices:

```rust
let slice = &[1, 1, 2, 3, 2, 3, 2, 3, 4];

let mut iter = slice.chunk_by(|a, b| a <= b);

assert_eq!(iter.next(), Some(&[1, 1, 2, 3][..]));
assert_eq!(iter.next(), Some(&[2, 3][..]));
assert_eq!(iter.next(), Some(&[2, 3, 4][..]));
assert_eq!(iter.next(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1955)

#### pub fn [split\_at](#method.split_at-1)(&self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

Divides one slice into two at an index.

The first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

##### Panics

Panics if `mid > len`. For a non-panicking alternative see [`split_at_checked`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_checked "method slice::split_at_checked").

##### Examples

```rust
let v = ['a', 'b', 'c'];

{
   let (left, right) = v.split_at(0);
   assert_eq!(left, []);
   assert_eq!(right, ['a', 'b', 'c']);
}

{
    let (left, right) = v.split_at(2);
    assert_eq!(left, ['a', 'b']);
    assert_eq!(right, ['c']);
}

{
    let (left, right) = v.split_at(3);
    assert_eq!(left, ['a', 'b', 'c']);
    assert_eq!(right, []);
}
```

1.79.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2041)

#### pub unsafe fn [split\_at\_unchecked](#method.split_at_unchecked-1)(&self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

Divides one slice into two at an index, without doing bounds checking.

The first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

For a safe alternative see [`split_at`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at "method slice::split_at").

##### Safety

Calling this method with an out-of-bounds index is _[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)_ even if the resulting reference is not used. The caller has to ensure that `0 <= mid <= self.len()`.

##### Examples

```rust
let v = ['a', 'b', 'c'];

unsafe {
   let (left, right) = v.split_at_unchecked(0);
   assert_eq!(left, []);
   assert_eq!(right, ['a', 'b', 'c']);
}

unsafe {
    let (left, right) = v.split_at_unchecked(2);
    assert_eq!(left, ['a', 'b']);
    assert_eq!(right, ['c']);
}

unsafe {
    let (left, right) = v.split_at_unchecked(3);
    assert_eq!(left, ['a', 'b', 'c']);
    assert_eq!(right, []);
}
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2156)

#### pub fn [split\_at\_checked](#method.split_at_checked-1)(&self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

Divides one slice into two at an index, returning `None` if the slice is too short.

If `mid ≤ len` returns a pair of slices where the first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

Otherwise, if `mid > len`, returns `None`.

##### Examples

```rust
let v = [1, -2, 3, -4, 5, -6];

{
   let (left, right) = v.split_at_checked(0).unwrap();
   assert_eq!(left, []);
   assert_eq!(right, [1, -2, 3, -4, 5, -6]);
}

{
    let (left, right) = v.split_at_checked(2).unwrap();
    assert_eq!(left, [1, -2]);
    assert_eq!(right, [3, -4, 5, -6]);
}

{
    let (left, right) = v.split_at_checked(6).unwrap();
    assert_eq!(left, [1, -2, 3, -4, 5, -6]);
    assert_eq!(right, []);
}

assert_eq!(None, v.split_at_checked(7));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2247-2249)

#### pub fn [split](#method.split-1)<F>(&self, pred: F) -> [Split](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Split.html "struct core::slice::iter::Split")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over subslices separated by elements that match `pred`. The matched element is not contained in the subslices.

##### Examples

```rust
let slice = [10, 40, 33, 20];
let mut iter = slice.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());
```

If the first element is matched, an empty slice will be the first item returned by the iterator. Similarly, if the last element in the slice is matched, an empty slice will be the last item returned by the iterator:

```rust
let slice = [10, 40, 33];
let mut iter = slice.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40]);
assert_eq!(iter.next().unwrap(), &[]);
assert!(iter.next().is_none());
```

If two matched elements are directly adjacent, an empty slice will be present between them:

```rust
let slice = [10, 6, 33, 20];
let mut iter = slice.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10]);
assert_eq!(iter.next().unwrap(), &[]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());
```

1.51.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2305-2307)

#### pub fn [split\_inclusive](#method.split_inclusive-1)<F>(&self, pred: F) -> [SplitInclusive](https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusive.html "struct core::slice::iter::SplitInclusive")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over subslices separated by elements that match `pred`. The matched element is contained in the end of the previous subslice as a terminator.

##### Examples

```rust
let slice = [10, 40, 33, 20];
let mut iter = slice.split_inclusive(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());
```

If the last element of the slice is matched, that element will be considered the terminator of the preceding slice. That slice will be the last item returned by the iterator.

```rust
let slice = [3, 10, 40, 33];
let mut iter = slice.split_inclusive(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[3]);
assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
assert!(iter.next().is_none());
```

1.27.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2365-2367)

#### pub fn [rsplit](#method.rsplit-1)<F>(&self, pred: F) -> [RSplit](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplit.html "struct core::slice::iter::RSplit")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over subslices separated by elements that match `pred`, starting at the end of the slice and working backwards. The matched element is not contained in the subslices.

##### Examples

```rust
let slice = [11, 22, 33, 0, 44, 55];
let mut iter = slice.rsplit(|num| *num == 0);

assert_eq!(iter.next().unwrap(), &[44, 55]);
assert_eq!(iter.next().unwrap(), &[11, 22, 33]);
assert_eq!(iter.next(), None);
```

As with `split()`, if the first or last element is matched, an empty slice will be the first (or last) item returned by the iterator.

```rust
let v = &[0, 1, 1, 2, 3, 5, 8];
let mut it = v.rsplit(|n| *n % 2 == 0);
assert_eq!(it.next().unwrap(), &[]);
assert_eq!(it.next().unwrap(), &[3, 5]);
assert_eq!(it.next().unwrap(), &[1, 1]);
assert_eq!(it.next().unwrap(), &[]);
assert_eq!(it.next(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2419-2421)

#### pub fn [splitn](#method.splitn-1)<F>(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pred: F) -> [SplitN](https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitN.html "struct core::slice::iter::SplitN")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over subslices separated by elements that match `pred`, limited to returning at most `n` items. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

##### Examples

Print the slice split once by numbers divisible by 3 (i.e., `[10, 40]`, `[20, 60, 50]`):

```rust
let v = [10, 40, 30, 20, 60, 50];

for group in v.splitn(2, |num| *num % 3 == 0) {
    println!("{group:?}");
}
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2474-2476)

#### pub fn [rsplitn](#method.rsplitn-1)<F>(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pred: F) -> [RSplitN](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitN.html "struct core::slice::iter::RSplitN")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over subslices separated by elements that match `pred` limited to returning at most `n` items. This starts at the end of the slice and works backwards. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

##### Examples

Print the slice split once, starting from the end, by numbers divisible by 3 (i.e., `[50]`, `[10, 40, 30, 20]`):

```rust
let v = [10, 40, 30, 20, 60, 50];

for group in v.rsplitn(2, |num| *num % 3 == 0) {
    println!("{group:?}");
}
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2528-2530)

#### pub fn [split\_once](#method.split_once)<F>(&self, pred: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

🔬This is a nightly-only experimental API. (`slice_split_once`)

Splits the slice on the first element that matches the specified predicate.

If any matching elements are present in the slice, returns the prefix before the match and suffix after. The matching element itself is not included. If no elements match, returns `None`.

##### Examples

```rust
#![feature(slice_split_once)]
let s = [1, 2, 3, 2, 4];
assert_eq!(s.split_once(|&x| x == 2), Some((
    &[1][..],
    &[3, 2, 4][..]
)));
assert_eq!(s.split_once(|&x| x == 0), None);
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2556-2558)

#### pub fn [rsplit\_once](#method.rsplit_once)<F>(&self, pred: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

🔬This is a nightly-only experimental API. (`slice_split_once`)

Splits the slice on the last element that matches the specified predicate.

If any matching elements are present in the slice, returns the prefix before the match and suffix after. The matching element itself is not included. If no elements match, returns `None`.

##### Examples

```rust
#![feature(slice_split_once)]
let s = [1, 2, 3, 2, 4];
assert_eq!(s.rsplit_once(|&x| x == 2), Some((
    &[1, 2, 3][..],
    &[4][..]
)));
assert_eq!(s.rsplit_once(|&x| x == 0), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2592-2594)

#### pub fn [contains](#method.contains)(&self, x: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Returns `true` if the slice contains an element with the given value.

This operation is _O_(_n_).

Note that if you have a sorted slice, [`binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search") may be faster.

##### Examples

```rust
let v = [10, 40, 30];
assert!(v.contains(&30));
assert!(!v.contains(&50));
```

If you do not have a `&T`, but some other value that you can compare with one (for example, `String` implements `PartialEq<str>`), you can use `iter().any`:

```rust
let v = [String::from("hello"), String::from("world")]; // slice of `String`
assert!(v.iter().any(|e| e == "hello")); // search with `&str`
assert!(!v.iter().any(|e| e == "hi"));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2622-2624)

#### pub fn [starts\_with](#method.starts_with)(&self, needle: &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Returns `true` if `needle` is a prefix of the slice or equal to the slice.

##### Examples

```rust
let v = [10, 40, 30];
assert!(v.starts_with(&[10]));
assert!(v.starts_with(&[10, 40]));
assert!(v.starts_with(&v));
assert!(!v.starts_with(&[50]));
assert!(!v.starts_with(&[10, 50]));
```

Always returns `true` if `needle` is an empty slice:

```rust
let v = &[10, 40, 30];
assert!(v.starts_with(&[]));
let v: &[u8] = &[];
assert!(v.starts_with(&[]));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2653-2655)

#### pub fn [ends\_with](#method.ends_with)(&self, needle: &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Returns `true` if `needle` is a suffix of the slice or equal to the slice.

##### Examples

```rust
let v = [10, 40, 30];
assert!(v.ends_with(&[30]));
assert!(v.ends_with(&[40, 30]));
assert!(v.ends_with(&v));
assert!(!v.ends_with(&[50]));
assert!(!v.ends_with(&[50, 30]));
```

Always returns `true` if `needle` is an empty slice:

```rust
let v = &[10, 40, 30];
assert!(v.ends_with(&[]));
let v: &[u8] = &[];
assert!(v.ends_with(&[]));
```

1.51.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2685-2687)

#### pub fn [strip\_prefix](#method.strip_prefix)<P>(&self, prefix: [&P](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

where P: [SlicePattern](https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html "trait core::slice::SlicePattern")<Item = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Returns a subslice with the prefix removed.

If the slice starts with `prefix`, returns the subslice after the prefix, wrapped in `Some`. If `prefix` is empty, simply returns the original slice. If `prefix` is equal to the original slice, returns an empty slice.

If the slice does not start with `prefix`, returns `None`.

##### Examples

```rust
let v = &[10, 40, 30];
assert_eq!(v.strip_prefix(&[10]), Some(&[40, 30][..]));
assert_eq!(v.strip_prefix(&[10, 40]), Some(&[30][..]));
assert_eq!(v.strip_prefix(&[10, 40, 30]), Some(&[][..]));
assert_eq!(v.strip_prefix(&[50]), None);
assert_eq!(v.strip_prefix(&[10, 50]), None);

let prefix : &str = "he";
assert_eq!(b"hello".strip_prefix(prefix.as_bytes()),
           Some(b"llo".as_ref()));
```

1.51.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2721-2723)

#### pub fn [strip\_suffix](#method.strip_suffix)<P>(&self, suffix: [&P](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

where P: [SlicePattern](https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html "trait core::slice::SlicePattern")<Item = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Returns a subslice with the suffix removed.

If the slice ends with `suffix`, returns the subslice before the suffix, wrapped in `Some`. If `suffix` is empty, simply returns the original slice. If `suffix` is equal to the original slice, returns an empty slice.

If the slice does not end with `suffix`, returns `None`.

##### Examples

```rust
let v = &[10, 40, 30];
assert_eq!(v.strip_suffix(&[30]), Some(&[10, 40][..]));
assert_eq!(v.strip_suffix(&[40, 30]), Some(&[10][..]));
assert_eq!(v.strip_suffix(&[10, 40, 30]), Some(&[][..]));
assert_eq!(v.strip_suffix(&[50]), None);
assert_eq!(v.strip_suffix(&[50, 30]), None);
```

1.98.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2758-2762)

#### pub fn [strip\_circumfix](#method.strip_circumfix)<S, P>(&self, prefix: [&P](https://doc.rust-lang.org/nightly/std/primitive.reference.html), suffix: [&S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"), S: [SlicePattern](https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html "trait core::slice::SlicePattern")<Item = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [SlicePattern](https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html "trait core::slice::SlicePattern")<Item = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a subslice with the prefix and suffix removed.

If the slice starts with `prefix` and ends with `suffix`, returns the subslice after the prefix and before the suffix, wrapped in `Some`.

If the slice does not start with `prefix` or does not end with `suffix`, returns `None`.

##### Examples

```rust
let v = &[10, 50, 40, 30];
assert_eq!(v.strip_circumfix(&[10], &[30]), Some(&[50, 40][..]));
assert_eq!(v.strip_circumfix(&[10], &[40, 30]), Some(&[50][..]));
assert_eq!(v.strip_circumfix(&[10, 50], &[40, 30]), Some(&[][..]));
assert_eq!(v.strip_circumfix(&[50], &[30]), None);
assert_eq!(v.strip_circumfix(&[10], &[40]), None);
assert_eq!(v.strip_circumfix(&[], &[40, 30]), Some(&[10, 50][..]));
assert_eq!(v.strip_circumfix(&[10, 50], &[]), Some(&[40, 30][..]));
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2794-2796)

#### pub fn [trim\_prefix](#method.trim_prefix)<P>(&self, prefix: [&P](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

where P: [SlicePattern](https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html "trait core::slice::SlicePattern")<Item = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a subslice with the optional prefix removed.

If the slice starts with `prefix`, returns the subslice after the prefix. If `prefix` is empty or the slice does not start with `prefix`, simply returns the original slice. If `prefix` is equal to the original slice, returns an empty slice.

##### Examples

```rust
#![feature(trim_prefix_suffix)]

let v = &[10, 40, 30];

// Prefix present - removes it
assert_eq!(v.trim_prefix(&[10]), &[40, 30][..]);
assert_eq!(v.trim_prefix(&[10, 40]), &[30][..]);
assert_eq!(v.trim_prefix(&[10, 40, 30]), &[][..]);

// Prefix absent - returns original slice
assert_eq!(v.trim_prefix(&[50]), &[10, 40, 30][..]);
assert_eq!(v.trim_prefix(&[10, 50]), &[10, 40, 30][..]);

let prefix : &str = "he";
assert_eq!(b"hello".trim_prefix(prefix.as_bytes()), b"llo".as_ref());
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2834-2836)

#### pub fn [trim\_suffix](#method.trim_suffix)<P>(&self, suffix: [&P](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

where P: [SlicePattern](https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html "trait core::slice::SlicePattern")<Item = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a subslice with the optional suffix removed.

If the slice ends with `suffix`, returns the subslice before the suffix. If `suffix` is empty or the slice does not end with `suffix`, simply returns the original slice. If `suffix` is equal to the original slice, returns an empty slice.

##### Examples

```rust
#![feature(trim_prefix_suffix)]

let v = &[10, 40, 30];

// Suffix present - removes it
assert_eq!(v.trim_suffix(&[30]), &[10, 40][..]);
assert_eq!(v.trim_suffix(&[40, 30]), &[10][..]);
assert_eq!(v.trim_suffix(&[10, 40, 30]), &[][..]);

// Suffix absent - returns original slice
assert_eq!(v.trim_suffix(&[50]), &[10, 40, 30][..]);
assert_eq!(v.trim_suffix(&[50, 30]), &[10, 40, 30][..]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2920-2922)

#### pub fn [binary\_search](#method.binary_search)(&self, x: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Binary searches this slice for a given element. If the slice is not sorted, the returned result is unspecified and meaningless.

If the value is found then [`Result::Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. The index is chosen deterministically, but is subject to change in future versions of Rust. If the value is not found then [`Result::Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, containing the index where a matching element could be inserted while maintaining sorted order.

See also [`binary_search_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by "method slice::binary_search_by"), [`binary_search_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by_key "method slice::binary_search_by_key"), and [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point").

##### Examples

Looks up a series of four elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in `[1, 4]`.

```rust
let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

assert_eq!(s.binary_search(&13),  Ok(9));
assert_eq!(s.binary_search(&4),   Err(7));
assert_eq!(s.binary_search(&100), Err(13));
let r = s.binary_search(&1);
assert!(match r { Ok(1..=4) => true, _ => false, });
```

If you want to find that whole _range_ of matching items, rather than an arbitrary matching one, that can be done using [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point"):

```rust
let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

let low = s.partition_point(|x| x < &1);
assert_eq!(low, 1);
let high = s.partition_point(|x| x <= &1);
assert_eq!(high, 5);
let r = s.binary_search(&1);
assert!((low..high).contains(&r.unwrap()));

assert!(s[..low].iter().all(|&x| x < 1));
assert!(s[low..high].iter().all(|&x| x == 1));
assert!(s[high..].iter().all(|&x| x > 1));

// For something not found, the "range" of equal items is empty
assert_eq!(s.partition_point(|x| x < &11), 9);
assert_eq!(s.partition_point(|x| x <= &11), 9);
assert_eq!(s.binary_search(&11), Err(9));
```

If you want to insert an item to a sorted vector, while maintaining sort order, consider using [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point"):

```rust
let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
let num = 42;
let idx = s.partition_point(|&x| x <= num);
// If `num` is unique, `s.partition_point(|&x| x < num)` (with `<`) is equivalent to
// `s.binary_search(&num).unwrap_or_else(|x| x)`, but using `<=` will allow `insert`
// to shift less elements.
s.insert(idx, num);
assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2971-2973)

#### pub fn [binary\_search\_by](#method.binary_search_by)<'a, F>(&'a self, f: F) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Binary searches this slice with a comparator function.

The comparator function should return an order code that indicates whether its argument is `Less`, `Equal` or `Greater` the desired target. If the slice is not sorted or if the comparator function does not implement an order consistent with the sort order of the underlying slice, the returned result is unspecified and meaningless.

If the value is found then [`Result::Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. The index is chosen deterministically, but is subject to change in future versions of Rust. If the value is not found then [`Result::Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, containing the index where a matching element could be inserted while maintaining sorted order.

See also [`binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search"), [`binary_search_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by_key "method slice::binary_search_by_key"), and [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point").

##### Examples

Looks up a series of four elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in `[1, 4]`.

```rust
let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

let seek = 13;
assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
let seek = 4;
assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
let seek = 100;
assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
let seek = 1;
let r = s.binary_search_by(|probe| probe.cmp(&seek));
assert!(match r { Ok(1..=4) => true, _ => false, });
```

1.10.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3072-3075)

#### pub fn [binary\_search\_by\_key](#method.binary_search_by_key)<'a, B, F>( &'a self, b: [&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html), f: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> B, B: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Binary searches this slice with a key extraction function.

Assumes that the slice is sorted by the key, for instance with [`sort_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_by_key "method slice::sort_by_key") using the same key extraction function. If the slice is not sorted by the key, the returned result is unspecified and meaningless.

If the value is found then [`Result::Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. The index is chosen deterministically, but is subject to change in future versions of Rust. If the value is not found then [`Result::Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, containing the index where a matching element could be inserted while maintaining sorted order.

See also [`binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search"), [`binary_search_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by "method slice::binary_search_by"), and [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point").

##### Examples

Looks up a series of four elements in a slice of pairs sorted by their second elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in `[1, 4]`.

```rust
let s = [(0, 0), (2, 1), (4, 1), (5, 1), (3, 1),
         (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
         (1, 21), (2, 34), (4, 55)];

assert_eq!(s.binary_search_by_key(&13, |&(a, b)| b),  Ok(9));
assert_eq!(s.binary_search_by_key(&4, |&(a, b)| b),   Err(7));
assert_eq!(s.binary_search_by_key(&100, |&(a, b)| b), Err(13));
let r = s.binary_search_by_key(&1, |&(a, b)| b);
assert!(match r { Ok(1..=4) => true, _ => false, });
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4501)

#### pub unsafe fn [align\_to](#method.align_to)<U>(&self) -> (&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[U\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

Transmutes the slice to a slice of another type, ensuring alignment of the types is maintained.

This method splits the slice into three distinct slices: prefix, correctly aligned middle slice of a new type, and the suffix slice. The middle part will be as big as possible under the given alignment constraint and element size.

This method has no purpose when either input element `T` or output element `U` are zero-sized and will return the original slice without splitting anything.

##### Safety

This method is essentially a `transmute` with respect to the elements in the returned middle slice, so all the usual caveats pertaining to `transmute::<T, U>` also apply here.

##### Examples

Basic usage:

```rust
unsafe {
    let bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
    let (prefix, shorts, suffix) = bytes.align_to::<u16>();
    // less_efficient_algorithm_for_bytes(prefix);
    // more_efficient_algorithm_for_aligned_shorts(shorts);
    // less_efficient_algorithm_for_bytes(suffix);
}
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4657-4660)

#### pub fn [as\_simd](#method.as_simd)<const LANES: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> (&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &\[[Simd](https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html "struct core::core_simd::vector::Simd")<T, LANES>\], &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

where [Simd](https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html "struct core::core_simd::vector::Simd")<T, LANES>: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[\[T; LANES\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>, T: [SimdElement](https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html "trait core::core_simd::vector::SimdElement"),

🔬This is a nightly-only experimental API. (`portable_simd`)

Splits a slice into a prefix, a middle of aligned SIMD types, and a suffix.

This is a safe wrapper around [`slice::align_to`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.align_to "method slice::align_to"), so inherits the same guarantees as that method.

##### Panics

This will panic if the size of the SIMD type is different from `LANES` times that of the scalar.

At the time of writing, the trait restrictions on `Simd<T, LANES>` keeps that from ever happening, as only power-of-two numbers of lanes are supported. It’s possible that, in the future, those restrictions might be lifted in a way that would make it possible to see panics from this method for something like `LANES == 3`.

##### Examples

```rust
#![feature(portable_simd)]
use core::simd::prelude::*;

let short = &[1, 2, 3];
let (prefix, middle, suffix) = short.as_simd::<4>();
assert_eq!(middle, []); // Not enough elements for anything in the middle

// They might be split in any possible way between prefix and suffix
let it = prefix.iter().chain(suffix).copied();
assert_eq!(it.collect::<Vec<_>>(), vec![1, 2, 3]);

fn basic_simd_sum(x: &[f32]) -> f32 {
    use std::ops::Add;
    let (prefix, middle, suffix) = x.as_simd();
    let sums = f32x4::from_array([
        prefix.iter().copied().sum(),
        0.0,
        0.0,
        suffix.iter().copied().sum(),
    ]);
    let sums = middle.iter().copied().fold(sums, f32x4::add);
    sums.reduce_sum()
}

let numbers: Vec<f32> = (1..101).map(|x| x as _).collect();
assert_eq!(basic_simd_sum(&numbers[1..99]), 4949.0);
```

1.82.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4730-4732)

#### pub fn [is\_sorted](#method.is_sorted)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Checks if the elements of this slice are sorted.

That is, for each element `a` and its following element `b`, `a <= b` must hold. If the slice yields exactly zero or one element, `true` is returned.

Note that if `Self::Item` is only `PartialOrd`, but not `Ord`, the above definition implies that this function returns `false` if any two consecutive items are not comparable.

##### Examples

```rust
let empty: [i32; 0] = [];

assert!([1, 2, 2, 9].is_sorted());
assert!(![1, 3, 2, 4].is_sorted());
assert!([0].is_sorted());
assert!(empty.is_sorted());
assert!(![0.0, 1.0, f32::NAN].is_sorted());
```

1.82.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4773-4775)

#### pub fn [is\_sorted\_by](#method.is_sorted_by)<'a, F>(&'a self, compare: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Checks if the elements of this slice are sorted using the given comparator function.

Instead of using `PartialOrd::partial_cmp`, this function uses the given `compare` function to determine whether two elements are to be considered in sorted order.

##### Examples

```rust
assert!([1, 2, 2, 9].is_sorted_by(|a, b| a <= b));
assert!(![1, 2, 2, 9].is_sorted_by(|a, b| a < b));

assert!([0].is_sorted_by(|a, b| true));
assert!([0].is_sorted_by(|a, b| false));

let empty: [i32; 0] = [];
assert!(empty.is_sorted_by(|a, b| false));
assert!(empty.is_sorted_by(|a, b| true));
```

1.82.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4797-4800)

#### pub fn [is\_sorted\_by\_key](#method.is_sorted_by_key)<'a, F, K>(&'a self, f: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> K, K: [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Checks if the elements of this slice are sorted using the given key extraction function.

Instead of comparing the slice’s elements directly, this function compares the keys of the elements, as determined by `f`. Apart from that, it’s equivalent to [`is_sorted`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.is_sorted "method slice::is_sorted"); see its documentation for more information.

##### Examples

```rust
assert!(["c", "bb", "aaa"].is_sorted_by_key(|s| s.len()));
assert!(![-2i32, -1, 0, 3].is_sorted_by_key(|n| n.abs()));
```

1.52.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4856-4858)

#### pub fn [partition\_point](#method.partition_point)<P>(&self, pred: P) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns the index of the partition point according to the given predicate (the index of the first element of the second partition).

The slice is assumed to be partitioned according to the given predicate. This means that all elements for which the predicate returns true are at the start of the slice and all elements for which the predicate returns false are at the end. For example, `[7, 15, 3, 5, 4, 12, 6]` is partitioned under the predicate `x % 2 != 0` (all odd numbers are at the start, all even at the end).

If this slice is not partitioned, the returned result is unspecified and meaningless, as this method performs a kind of binary search.

See also [`binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search"), [`binary_search_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by "method slice::binary_search_by"), and [`binary_search_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by_key "method slice::binary_search_by_key").

##### Examples

```rust
let v = [1, 2, 3, 3, 5, 6, 7];
let i = v.partition_point(|&x| x < 5);

assert_eq!(i, 4);
assert!(v[..i].iter().all(|&x| x < 5));
assert!(v[i..].iter().all(|&x| !(x < 5)));
```

If all elements of the slice match the predicate, including if the slice is empty, then the length of the slice will be returned:

```rust
let a = [2, 4, 8];
assert_eq!(a.partition_point(|x| x < &100), a.len());
let a: [i32; 0] = [];
assert_eq!(a.partition_point(|x| x < &100), 0);
```

If you want to insert an item to a sorted vector, while maintaining sort order:

```rust
let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
let num = 42;
let idx = s.partition_point(|&x| x <= num);
s.insert(idx, num);
assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
```

1.94.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5262)

#### pub fn [element\_offset](#method.element_offset)(&self, element: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Returns the index that an element reference points to.

Returns `None` if `element` does not point to the start of an element within the slice.

This method is useful for extending slice iterators like [`slice::split`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split "method slice::split").

Note that this uses pointer arithmetic and **does not compare elements**. To find the index of an element via comparison, use [`.iter().position()`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position "method core::iter::traits::iterator::Iterator::position") instead.

##### Panics

Panics if `T` is zero-sized.

##### Examples

Basic usage:

```rust
let nums: &[u32] = &[1, 7, 1, 1];
let num = &nums[2];

assert_eq!(num, &1);
assert_eq!(nums.element_offset(num), Some(2));
```

Returning `None` with an unaligned element:

```rust
let arr: &[[u32; 2]] = &[[0, 1], [2, 3]];
let flat_arr: &[u32] = arr.as_flattened();

let ok_elm: &[u32; 2] = flat_arr[0..2].try_into().unwrap();
let weird_elm: &[u32; 2] = flat_arr[1..3].try_into().unwrap();

assert_eq!(ok_elm, &[0, 1]);
assert_eq!(weird_elm, &[1, 2]);

assert_eq!(arr.element_offset(ok_elm), Some(0)); // Points to element 0
assert_eq!(arr.element_offset(weird_elm), None); // Points between element 0 and 1
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5317)

#### pub fn [subslice\_range](#method.subslice_range)(&self, subslice: &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Range](https://doc.rust-lang.org/nightly/core/range/struct.Range.html "struct core::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>

🔬This is a nightly-only experimental API. (`substr_range`)

Returns the range of indices that a subslice points to.

Returns `None` if `subslice` does not point within the slice or if it is not aligned with the elements in the slice.

This method **does not compare elements**. Instead, this method finds the location in the slice that `subslice` was obtained from. To find the index of a subslice via comparison, instead use [`.windows()`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.windows "method slice::windows")[`.position()`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position "method core::iter::traits::iterator::Iterator::position").

This method is useful for extending slice iterators like [`slice::split`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split "method slice::split").

Note that this may return a false positive (either `Some(0..0)` or `Some(self.len()..self.len())`) if `subslice` has a length of zero and points to the beginning or end of another, separate, slice.

##### Panics

Panics if `T` is zero-sized.

##### Examples

Basic usage:

```rust
#![feature(substr_range)]
use core::range::Range;

let nums = &[0, 5, 10, 0, 0, 5];

let mut iter = nums
    .split(|t| *t == 0)
    .map(|n| nums.subslice_range(n).unwrap());

assert_eq!(iter.next(), Some(Range { start: 0, end: 0 }));
assert_eq!(iter.next(), Some(Range { start: 1, end: 3 }));
assert_eq!(iter.next(), Some(Range { start: 4, end: 4 }));
assert_eq!(iter.next(), Some(Range { start: 5, end: 6 }));
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5348)

#### pub fn [as\_slice](#method.as_slice)(&self) -> &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

🔬This is a nightly-only experimental API. (`str_as_str`)

Returns the same slice `&[T]`.

This method is redundant when used directly on `&[T]`, but it helps dereferencing other “container” types to slices, for example `Box<[T]>` or `Arc<[T]>`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#372-374)

#### pub fn [to\_vec](#method.to_vec-1)(&self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Available on **non-`no_global_oom_handling`** only.

Copies `self` into a new `Vec`.

##### Examples

```rust
let s = [10, 40, 30];
let x = s.to_vec();
// Here, `s` and `x` can be modified independently.
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#396-398)

#### pub fn [to\_vec\_in](#method.to_vec_in)<A>(&self, alloc: A) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T, A>

where A: [Allocator](https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html "trait core::alloc::Allocator"), T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

🔬This is a nightly-only experimental API. (`allocator_api`)

Available on **non-`no_global_oom_handling`** only.

Copies `self` into a new `Vec` with an allocator.

##### Examples

```rust
#![feature(allocator_api)]

use std::alloc::System;

let s = [10, 40, 30];
let x = s.to_vec_in(System);
// Here, `s` and `x` can be modified independently.
```

1.40.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#509-511)

#### pub fn [repeat](#method.repeat)(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"),

Available on **non-`no_global_oom_handling`** only.

Creates a vector by copying a slice `n` times.

##### Panics

This function will panic if the capacity would overflow.

##### Examples

```rust
assert_eq!([1, 2].repeat(3), vec![1, 2, 1, 2, 1, 2]);
```

A panic upon overflow:

[ⓘ](# "This example panics")

```rust
// this will panic at runtime
b"0123456789abcdef".repeat(usize::MAX);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#577-579)

#### pub fn [concat](#method.concat)<Item>(&self) -> <[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html) as [Concat](https://doc.rust-lang.org/nightly/alloc/slice/trait.Concat.html "trait alloc::slice::Concat")<Item>>::[Output](https://doc.rust-lang.org/nightly/alloc/slice/trait.Concat.html#associatedtype.Output "type alloc::slice::Concat::Output") [ⓘ](#)

where [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html): [Concat](https://doc.rust-lang.org/nightly/alloc/slice/trait.Concat.html "trait alloc::slice::Concat")<Item>, Item: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Flattens a slice of `T` into a single value `Self::Output`.

##### Examples

```rust
assert_eq!(["hello", "world"].concat(), "helloworld");
assert_eq!([[1, 2], [3, 4]].concat(), [1, 2, 3, 4]);
```

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#596-598)

#### pub fn [join](#method.join)<Separator>( &self, sep: Separator, ) -> <[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html) as [Join](https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html "trait alloc::slice::Join")<Separator>>::[Output](https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html#associatedtype.Output "type alloc::slice::Join::Output") [ⓘ](#)

where [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html): [Join](https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html "trait alloc::slice::Join")<Separator>,

Flattens a slice of `T` into a single value `Self::Output`, placing a given separator between each.

##### Examples

```rust
assert_eq!(["hello", "world"].join(" "), "hello world");
assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3, 4]);
assert_eq!([[1, 2], [3, 4]].join(&[0, 0][..]), [1, 2, 0, 0, 3, 4]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#616-618)

#### pub fn [connect](#method.connect)<Separator>( &self, sep: Separator, ) -> <[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html) as [Join](https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html "trait alloc::slice::Join")<Separator>>::[Output](https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html#associatedtype.Output "type alloc::slice::Join::Output") [ⓘ](#)

where [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html): [Join](https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html "trait alloc::slice::Join")<Separator>,

👎Deprecated since 1.3.0:

renamed to join

Flattens a slice of `T` into a single value `Self::Output`, placing a given separator between each.

##### Examples

```rust
assert_eq!(["hello", "world"].connect(" "), "hello world");
assert_eq!([[1, 2], [3, 4]].connect(&0), [1, 2, 0, 3, 4]);
```

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#471-472)

### impl<T, I> [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityIter](struct.UniqueEntityIter.html "struct bevy::ecs::entity::UniqueEntityIter")<I>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"), I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") + [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>, <I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#474)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#196-197)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#199)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#985)

### impl<T> [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#986)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#497)

### impl<T> [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#498)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#462-463)

### impl<T, I> [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityIter](struct.UniqueEntityIter.html "struct bevy::ecs::entity::UniqueEntityIter")<I>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"), I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") + [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>, <I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#465)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#188-189)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#191)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#979)

### impl<T> [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#980)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#527)

### impl<T> [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#528)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#973)

### impl<T> [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#974)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#204-205)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#207)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#539)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#540)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#991)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#992)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#212-213)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#215)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#545-546)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#548)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#997)

### impl<T> [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#998)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#39)

### impl<T> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#39)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1003)

### impl<T> [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1004)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1010)

### impl<T> [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1011)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1017)

### impl<T> [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1018)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#965)

### impl<T> [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#966)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#968)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T> as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#39)

### impl<T> [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1051-1052)

### impl<'a, T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1054)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)( value: &'a [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, ) -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1024-1025)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1027)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)( value: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, ) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1033-1034)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1036)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)( value: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, ) -> [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> [ⓘ](#)

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1042-1043)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1045)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)( value: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, ) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#689-690)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#692)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>) -> [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#697-698)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#700)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)( value: &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, ) -> [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1070-1071)

### impl<'a, T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1073)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)( value: [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>, ) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1059-1060)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1062)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)( value: [UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>, ) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1081-1082)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1084)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)( value: [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>, ) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1097)

### impl<T> [FromEntitySetIterator](trait.FromEntitySetIterator.html "trait bevy::ecs::entity::FromEntitySetIterator")<T> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1098)

#### fn [from\_entity\_set\_iter](trait.FromEntitySetIterator.html#tymethod.from_entity_set_iter)<I>(iter: I) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where I: [EntitySet](trait.EntitySet.html "trait bevy::ecs::entity::EntitySet")<Item = T>,

Creates a value from an [`EntitySetIterator`](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1089)

### impl<T> [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<T> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1090)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(iter: I) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = T>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#39)

### impl<T> [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#39)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<\_\_H>(&self, state: [&mut \_\_H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where \_\_H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1348)

### impl<T> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1349)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1351)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)( &self, key: ([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>), ) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1357)

### impl<T> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1358)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1360)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1366)

### impl<T> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1367)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1369)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1375)

### impl<T> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")\> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1376)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1378)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1384)

### impl<T> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1385)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1387)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1393)

### impl<T> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1394)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1396)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1402)

### impl<T> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1403)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1405)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1411)

### impl<T> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1412)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = T

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1414)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1419-1420)

### impl<T> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1422)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: ([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>), ) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1428)

### impl<T> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1429)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1435)

### impl<T> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1436)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1442)

### impl<T> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")\> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1443)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, key: [RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1449)

### impl<T> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1450)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1456)

### impl<T> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1457)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1463)

### impl<T> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1464)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#935)

### impl<'a, T> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'a [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#936)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#938)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [UniqueEntityIter](struct.UniqueEntityIter.html "struct bevy::ecs::entity::UniqueEntityIter")<[Iter](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html "struct core::slice::iter::Iter")<'a, T>>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#940)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)( self, ) -> <&'a [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#945)

### impl<'a, T> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'a [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#946)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#948)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [UniqueEntityIter](struct.UniqueEntityIter.html "struct bevy::ecs::entity::UniqueEntityIter")<[Iter](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html "struct core::slice::iter::Iter")<'a, T>>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#950)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)( self, ) -> <&'a [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#955)

### impl<T> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#956)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = T

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#958)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [UniqueEntityIter](struct.UniqueEntityIter.html "struct bevy::ecs::entity::UniqueEntityIter")<[IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<T>>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#960)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)( self, ) -> <[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#39)

### impl<T> [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") + [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#39)

#### fn [cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)(&self, other: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#39)

### impl<T> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#39)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#522-523)

### impl<T, U, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#525)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1129-1130)

### impl<T, U, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1132)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1137-1138)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1140)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1145-1146)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1148)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1153)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1154)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1159-1160)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1162)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#565-566)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#568)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1167-1168)

### impl<T, U, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1170)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1175-1176)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1178)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1183-1184)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1186)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1199-1200)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1202)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1207-1208)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1210)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#579-580)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#582)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1263-1264)

### impl<T, U, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<U, N>> for &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1266)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<U, N>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1271-1272)

### impl<T, U, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<U, N>> for &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1274)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<U, N>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1279-1280)

### impl<T, U, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<U, N>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1282)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<U, N>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#530-531)

### impl<T, U, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#533)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1215-1216)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1218)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1223-1224)

### impl<T, U, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1226)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1231-1232)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1234)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#625-626)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>> for [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#628)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1105-1106)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<U>> for &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1108)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1113-1114)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<U>> for &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1116)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1121-1122)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<U>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>, U: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1124)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1287)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<U>> for &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1288)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1293-1294)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<U>> for &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1296)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1301)

### impl<T, U> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<U>> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1302)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<U>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1239-1240)

### impl<T, U, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[\[U; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\> for &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1242)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[\[U; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1247-1248)

### impl<T, U, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[\[U; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\> for &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1250)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[\[U; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1255-1256)

### impl<T, U, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[\[U; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\> for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<U>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1258)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[\[U; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#39)

### impl<T> [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") + [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#39)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)( &self, other: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#39)

### impl<T> [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1307)

### impl<T> [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1308)

#### type [Owned](../../prelude/trait.ToOwned.html#associatedtype.Owned) = [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

The resulting type after obtaining ownership.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1310)

#### fn [to\_owned](../../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> <[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T> as [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned")\>::[Owned](../../prelude/trait.ToOwned.html#associatedtype.Owned "type bevy::prelude::ToOwned::Owned")

Creates owned data from borrowed data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#tymethod.to_owned)

1.63.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#66)

#### fn [clone\_into](../../prelude/trait.ToOwned.html#method.clone_into)(&self, target: &mut Self::[Owned](../../prelude/trait.ToOwned.html#associatedtype.Owned "type bevy::prelude::ToOwned::Owned"))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#method.clone_into)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1316-1317)

### impl<'a, T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&'a [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for &'a [UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1319)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromSliceError](https://doc.rust-lang.org/nightly/core/array/struct.TryFromSliceError.html "struct core::array::TryFromSliceError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1321)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: &'a [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&'a [UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>, <&'a [UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&'a [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1328-1329)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1331)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromSliceError](https://doc.rust-lang.org/nightly/core/array/struct.TryFromSliceError.html "struct core::array::TryFromSliceError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1333)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: &[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>, <[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1338-1339)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>> for [UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1341)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromSliceError](https://doc.rust-lang.org/nightly/core/array/struct.TryFromSliceError.html "struct core::array::TryFromSliceError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1343)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: &mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N>, <[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<T, N> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&mut [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

## Auto Trait Implementations

### impl<T> ![Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

### impl<T> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze"),

### impl<T> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"),

### impl<T> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

### impl<T> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

### impl<T> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

### impl<T> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin"),

### impl<T> [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

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

### impl<Q, K> [Equivalent](../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)

#### fn [equivalent](../../platform/collections/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

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

{"<\[T\] as Concat<Item>>::Output":"<h3>Notable traits for <code>&amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for &amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for &amp;mut \[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div>","<\[T\] as Join<Separator>>::Output":"<h3>Notable traits for <code>&amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for &amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for &amp;mut \[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div>","Arc<UniqueEntityEquivalentSlice<T>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Arc<\[T\]>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","ArrayWindows<'\_, T, N>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ArrayWindows.html\\" title=\\"struct core::slice::iter::ArrayWindows\\">ArrayWindows</a>&lt;'a, T, N&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, const N: <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ArrayWindows.html\\" title=\\"struct core::slice::iter::ArrayWindows\\">ArrayWindows</a>&lt;'a, T, N&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.array.html\\">\[T; N\]</a>;</div>","ChunkBy<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkBy.html\\" title=\\"struct core::slice::iter::ChunkBy\\">ChunkBy</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkBy.html\\" title=\\"struct core::slice::iter::ChunkBy\\">ChunkBy</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n T: 'a,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","Chunks<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Chunks.html\\" title=\\"struct core::slice::iter::Chunks\\">Chunks</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Chunks.html\\" title=\\"struct core::slice::iter::Chunks\\">Chunks</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","ChunksExact<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExact.html\\" title=\\"struct core::slice::iter::ChunksExact\\">ChunksExact</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExact.html\\" title=\\"struct core::slice::iter::ChunksExact\\">ChunksExact</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","Iter<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html\\" title=\\"struct core::slice::iter::Iter\\">Iter</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html\\" title=\\"struct core::slice::iter::Iter\\">Iter</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>;</div>","RChunks<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunks.html\\" title=\\"struct core::slice::iter::RChunks\\">RChunks</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunks.html\\" title=\\"struct core::slice::iter::RChunks\\">RChunks</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","RChunksExact<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExact.html\\" title=\\"struct core::slice::iter::RChunksExact\\">RChunksExact</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExact.html\\" title=\\"struct core::slice::iter::RChunksExact\\">RChunksExact</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","RSplit<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplit.html\\" title=\\"struct core::slice::iter::RSplit\\">RSplit</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplit.html\\" title=\\"struct core::slice::iter::RSplit\\">RSplit</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","RSplitN<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitN.html\\" title=\\"struct core::slice::iter::RSplitN\\">RSplitN</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitN.html\\" title=\\"struct core::slice::iter::RSplitN\\">RSplitN</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","Range<\*const T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html\\" title=\\"struct core::ops::range::Range\\">Range</a>&lt;A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html\\" title=\\"struct core::ops::range::Range\\">Range</a>&lt;A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/range/trait.Step.html\\" title=\\"trait core::iter::range::Step\\">Step</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = A;</div>","Range<\*mut T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html\\" title=\\"struct core::ops::range::Range\\">Range</a>&lt;A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html\\" title=\\"struct core::ops::range::Range\\">Range</a>&lt;A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/range/trait.Step.html\\" title=\\"trait core::iter::range::Step\\">Step</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = A;</div>","Split<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Split.html\\" title=\\"struct core::slice::iter::Split\\">Split</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Split.html\\" title=\\"struct core::slice::iter::Split\\">Split</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","SplitInclusive<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusive.html\\" title=\\"struct core::slice::iter::SplitInclusive\\">SplitInclusive</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusive.html\\" title=\\"struct core::slice::iter::SplitInclusive\\">SplitInclusive</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","SplitN<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitN.html\\" title=\\"struct core::slice::iter::SplitN\\">SplitN</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitN.html\\" title=\\"struct core::slice::iter::SplitN\\">SplitN</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","UniqueEntityEquivalentSliceIter<'\_, T, ChunkBy<'\_, T, F>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIter<'\_, T, Chunks<'\_, T>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIter<'\_, T, ChunksExact<'\_, T>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIter<'\_, T, RChunks<'\_, T>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIter<'\_, T, RChunksExact<'\_, T>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIter<'\_, T, RSplit<'\_, T, F>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIter<'\_, T, RSplitN<'\_, T, F>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIter<'\_, T, Split<'\_, T, F>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIter<'\_, T, SplitInclusive<'\_, T, F>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIter<'\_, T, SplitN<'\_, T, F>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIter<'\_, T, Windows<'\_, T>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIter.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIter\\">UniqueEntityEquivalentSliceIter</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIterMut<'\_, T, ChunkByMut<'\_, T, F>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIterMut<'\_, T, ChunksExactMut<'\_, T>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIterMut<'\_, T, ChunksMut<'\_, T>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIterMut<'\_, T, RChunksExactMut<'\_, T>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIterMut<'\_, T, RChunksMut<'\_, T>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIterMut<'\_, T, RSplitMut<'\_, T, F>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIterMut<'\_, T, RSplitNMut<'\_, T, F>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIterMut<'\_, T, SplitInclusiveMut<'\_, T, F>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIterMut<'\_, T, SplitMut<'\_, T, F>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityEquivalentSliceIterMut<'\_, T, SplitNMut<'\_, T, F>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"unique\_slice/struct.UniqueEntityEquivalentSliceIterMut.html\\" title=\\"struct bevy::ecs::entity::unique\_slice::UniqueEntityEquivalentSliceIterMut\\">UniqueEntityEquivalentSliceIterMut</a>&lt;'a, T, I&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a> + 'a,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"struct\\" href=\\"struct.UniqueEntityEquivalentSlice.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityEquivalentSlice\\">UniqueEntityEquivalentSlice</a>&lt;T&gt;;</div>","UniqueEntityIter<Iter<'\_, T>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.UniqueEntityIter.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityIter\\">UniqueEntityIter</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.UniqueEntityIter.html\\" title=\\"struct bevy::ecs::entity::UniqueEntityIter\\">UniqueEntityIter</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Windows<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Windows.html\\" title=\\"struct core::slice::iter::Windows\\">Windows</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Windows.html\\" title=\\"struct core::slice::iter::Windows\\">Windows</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>"}