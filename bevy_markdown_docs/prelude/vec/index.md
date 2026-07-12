[bevy](../../index.html)::[prelude](../index.html)

# Module vec 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/lib.rs.html#246)

A contiguous growable array type with heap-allocated contents, written `Vec<T>`.

Vectors have _O_(1) indexing, amortized _O_(1) push (to the end) and _O_(1) pop (from the end).

Vectors ensure they never allocate more than `isize::MAX` bytes.

## Examples

You can explicitly create a [`Vec`](../struct.Vec.html "struct bevy::prelude::Vec") with [`Vec::new`](../struct.Vec.html#method.new "associated function bevy::prelude::Vec::new"):

```rust
let v: Vec<i32> = Vec::new();
```

…or by using the [`vec!`](../macro.vec.html "macro bevy::prelude::vec") macro:

```rust
let v: Vec<i32> = vec![];

let v = vec![1, 2, 3, 4, 5];

let v = vec![0; 10]; // ten zeroes
```

You can [`push`](../struct.Vec.html#method.push "method bevy::prelude::Vec::push") values onto the end of a vector (which will grow the vector as needed):

```rust
let mut v = vec![1, 2];

v.push(3);
```

Popping values works in much the same way:

```rust
let mut v = vec![1, 2];

let two = v.pop();
```

Vectors also support indexing (through the [`Index`](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index") and [`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut") traits):

```rust
let mut v = vec![1, 2, 3];
let three = v[2];
v[1] = v[1] + 5;
```

## Memory layout

When the type is non-zero-sized and the capacity is nonzero, [`Vec`](../struct.Vec.html "struct bevy::prelude::Vec") uses the [`Global`](https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html "struct alloc::alloc::Global") allocator for its allocation. It is valid to convert both ways between such a [`Vec`](../struct.Vec.html "struct bevy::prelude::Vec") and a raw pointer allocated with the [`Global`](https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html "struct alloc::alloc::Global") allocator, provided that the [`Layout`](https://doc.rust-lang.org/nightly/core/alloc/layout/struct.Layout.html "struct core::alloc::layout::Layout") used with the allocator is correct for a sequence of `capacity` elements of the type, and the first `len` values pointed to by the raw pointer are valid. More precisely, a `ptr: *mut T` that has been allocated with the [`Global`](https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html "struct alloc::alloc::Global") allocator with [`Layout::array::<T>(capacity)`](https://doc.rust-lang.org/nightly/core/alloc/layout/struct.Layout.html#method.array "associated function core::alloc::layout::Layout::array") may be converted into a vec using [`Vec::<T>::from_raw_parts(ptr, len, capacity)`](../struct.Vec.html#method.from_raw_parts "associated function bevy::prelude::Vec::from_raw_parts"). Conversely, the memory backing a `value: *mut T` obtained from [`Vec::<T>::as_mut_ptr`](../struct.Vec.html#method.as_mut_ptr "method bevy::prelude::Vec::as_mut_ptr") may be deallocated using the [`Global`](https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html "struct alloc::alloc::Global") allocator with the same layout.

For zero-sized types (ZSTs), or when the capacity is zero, the `Vec` pointer must be non-null and sufficiently aligned. The recommended way to build a `Vec` of ZSTs if [`vec!`](../macro.vec.html "macro bevy::prelude::vec") cannot be used is to use [`ptr::NonNull::dangling`](https://doc.rust-lang.org/nightly/core/ptr/non_null/struct.NonNull.html#method.dangling "associated function core::ptr::non_null::NonNull::dangling").

## Structs

[Drain](struct.Drain.html "struct bevy::prelude::vec::Drain")

A draining iterator for `Vec<T>`.

[ExtractIf](struct.ExtractIf.html "struct bevy::prelude::vec::ExtractIf")

An iterator which uses a closure to determine if an element should be removed.

[IntoIter](struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")

An iterator that moves out of a vector.

[Splice](struct.Splice.html "struct bevy::prelude::vec::Splice")

A splicing iterator for `Vec`.

[Vec](struct.Vec.html "struct bevy::prelude::vec::Vec")

A contiguous growable array type, written as `Vec<T>`, short for ‘vector’.

[PeekMut](struct.PeekMut.html "struct bevy::prelude::vec::PeekMut")Experimental

Structure wrapping a mutable reference to the last item in a `Vec`.