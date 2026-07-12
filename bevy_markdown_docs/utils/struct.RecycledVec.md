[bevy](../index.html)::[utils](index.html)

# Struct RecycledVec 

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#96)

```rust
pub struct RecycledVec<'a, T>where
    T: Send,{ /* private fields */ }
```

A wrapper around a `Vec<T>` that automatically returns it to the [`BufferedChannel`](struct.BufferedChannel.html "struct bevy::utils::BufferedChannel")’s pool when dropped.

## Implementations

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#101)

### impl<'a, T> [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#104)

#### pub fn [drain](#method.drain)(&mut self) -> [Drain](../prelude/vec/struct.Drain.html "struct bevy::prelude::vec::Drain")<'\_, T> [ⓘ](#)

Drains the elements from the buffer as an iterator, keeping the allocation so it can be recycled when this [`RecycledVec`](struct.RecycledVec.html "struct bevy::utils::RecycledVec") is dropped.

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

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#178)

#### pub fn [first\_mut](#method.first_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Returns a mutable reference to the first element of the slice, or `None` if it is empty.

##### Examples

```rust
let x = &mut [0, 1, 2];

if let Some(first) = x.first_mut() {
    *first = 5;
}
assert_eq!(x, &[5, 1, 2]);

let y: &mut [i32] = &mut [];
assert_eq!(None, y.first_mut());
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#198)

#### pub fn [split\_first](#method.split_first)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

Returns the first and all the rest of the elements of the slice, or `None` if it is empty.

##### Examples

```rust
let x = &[0, 1, 2];

if let Some((first, elements)) = x.split_first() {
    assert_eq!(first, &0);
    assert_eq!(elements, &[1, 2]);
}
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#220)

#### pub fn [split\_first\_mut](#method.split_first_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

Returns the first and all the rest of the elements of the slice, or `None` if it is empty.

##### Examples

```rust
let x = &mut [0, 1, 2];

if let Some((first, elements)) = x.split_first_mut() {
    *first = 3;
    elements[0] = 4;
    elements[1] = 5;
}
assert_eq!(x, &[3, 4, 5]);
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#240)

#### pub fn [split\_last](#method.split_last)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

Returns the last and all the rest of the elements of the slice, or `None` if it is empty.

##### Examples

```rust
let x = &[0, 1, 2];

if let Some((last, elements)) = x.split_last() {
    assert_eq!(last, &2);
    assert_eq!(elements, &[0, 1]);
}
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#262)

#### pub fn [split\_last\_mut](#method.split_last_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

Returns the last and all the rest of the elements of the slice, or `None` if it is empty.

##### Examples

```rust
let x = &mut [0, 1, 2];

if let Some((last, elements)) = x.split_last_mut() {
    *last = 3;
    elements[0] = 4;
    elements[1] = 5;
}
assert_eq!(x, &[4, 5, 3]);
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

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#304)

#### pub fn [last\_mut](#method.last_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Returns a mutable reference to the last item in the slice, or `None` if it is empty.

##### Examples

```rust
let x = &mut [0, 1, 2];

if let Some(last) = x.last_mut() {
    *last = 10;
}
assert_eq!(x, &[0, 1, 10]);

let y: &mut [i32] = &mut [];
assert_eq!(None, y.last_mut());
```

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#327)

#### pub fn [first\_chunk](#method.first_chunk)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>

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

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#357)

#### pub fn [first\_chunk\_mut](#method.first_chunk_mut)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>

Returns a mutable array reference to the first `N` items in the slice.

If the slice is not at least `N` in length, this will return `None`.

##### Examples

```rust
let x = &mut [0, 1, 2];

if let Some(first) = x.first_chunk_mut::<2>() {
    first[0] = 5;
    first[1] = 4;
}
assert_eq!(x, &[5, 4, 2]);

assert_eq!(None, x.first_chunk_mut::<4>());
```

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#387)

#### pub fn [split\_first\_chunk](#method.split_first_chunk)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

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

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#417-419)

#### pub fn [split\_first\_chunk\_mut](#method.split_first_chunk_mut)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&mut [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

Returns a mutable array reference to the first `N` items in the slice and the remaining slice.

If the slice is not at least `N` in length, this will return `None`.

##### Examples

```rust
let x = &mut [0, 1, 2];

if let Some((first, elements)) = x.split_first_chunk_mut::<2>() {
    first[0] = 3;
    first[1] = 4;
    elements[0] = 5;
}
assert_eq!(x, &[3, 4, 5]);

assert_eq!(None, x.split_first_chunk_mut::<4>());
```

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#447)

#### pub fn [split\_last\_chunk](#method.split_last_chunk)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html))>

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

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#478-480)

#### pub fn [split\_last\_chunk\_mut](#method.split_last_chunk_mut)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &mut [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html))>

Returns a mutable array reference to the last `N` items in the slice and the remaining slice.

If the slice is not at least `N` in length, this will return `None`.

##### Examples

```rust
let x = &mut [0, 1, 2];

if let Some((elements, last)) = x.split_last_chunk_mut::<2>() {
    last[0] = 3;
    last[1] = 4;
    elements[0] = 5;
}
assert_eq!(x, &[5, 3, 4]);

assert_eq!(None, x.split_last_chunk_mut::<4>());
```

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#509)

#### pub fn [last\_chunk](#method.last_chunk)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>

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

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#539)

#### pub fn [last\_chunk\_mut](#method.last_chunk_mut)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>

Returns a mutable array reference to the last `N` items in the slice.

If the slice is not at least `N` in length, this will return `None`.

##### Examples

```rust
let x = &mut [0, 1, 2];

if let Some(last) = x.last_chunk_mut::<2>() {
    last[0] = 10;
    last[1] = 20;
}
assert_eq!(x, &[0, 10, 20]);

assert_eq!(None, x.last_chunk_mut::<4>());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#572-574)

#### pub fn [get](#method.get)<I>(&self, index: I) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&<I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")\>

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

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#600-602)

#### pub fn [get\_mut](#method.get_mut)<I>( &mut self, index: I, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut <I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")\>

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>,

Returns a mutable reference to an element or subslice depending on the type of index (see [`get`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.get "method slice::get")) or `None` if the index is out of bounds.

##### Examples

```rust
let x = &mut [0, 1, 2];

if let Some(elem) = x.get_mut(1) {
    *elem = 42;
}
assert_eq!(x, &[0, 42, 2]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#640-642)

#### pub unsafe fn [get\_unchecked](#method.get_unchecked)<I>( &self, index: I, ) -> &<I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")

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

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#686-688)

#### pub unsafe fn [get\_unchecked\_mut](#method.get_unchecked_mut)<I>( &mut self, index: I, ) -> &mut <I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>,

Returns a mutable reference to an element or subslice, without doing bounds checking.

For a safe alternative see [`get_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.get_mut "method slice::get_mut").

##### Safety

Calling this method with an out-of-bounds index is _[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)_ even if the resulting reference is not used.

You can think of this like `.get_mut(index).unwrap_unchecked()`. It’s UB to call `.get_unchecked_mut(len)`, even if you immediately convert to a pointer. And it’s UB to call `.get_unchecked_mut(..len + 1)`, `.get_unchecked_mut(..=len)`, or similar.

##### Examples

```rust
let x = &mut [1, 2, 4];

unsafe {
    let elem = x.get_unchecked_mut(1);
    *elem = 13;
}
assert_eq!(x, &[1, 13, 4]);
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

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#760)

#### pub fn [as\_mut\_ptr](#method.as_mut_ptr)(&mut self) -> [\*mut T](https://doc.rust-lang.org/nightly/std/primitive.pointer.html)

Returns an unsafe mutable pointer to the slice’s buffer.

The caller must ensure that the slice outlives the pointer this function returns, or else it will end up dangling.

Modifying the container referenced by this slice may cause its buffer to be reallocated, which would also make any pointers to it invalid.

##### Examples

```rust
let x = &mut [1, 2, 4];
let x_ptr = x.as_mut_ptr();

unsafe {
    for i in 0..x.len() {
        *x_ptr.add(i) += 2;
    }
}
assert_eq!(x, &[3, 4, 6]);
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

1.48.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#839)

#### pub fn [as\_mut\_ptr\_range](#method.as_mut_ptr_range)(&mut self) -> [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[\*mut T](https://doc.rust-lang.org/nightly/std/primitive.pointer.html)\> [ⓘ](#)

Returns the two unsafe mutable pointers spanning the slice.

The returned range is half-open, which means that the end pointer points _one past_ the last element of the slice. This way, an empty slice is represented by two equal pointers, and the difference between the two pointers represents the size of the slice.

See [`as_mut_ptr`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_mut_ptr "method slice::as_mut_ptr") for warnings on using these pointers. The end pointer requires extra caution, as it does not point to a valid element in the slice.

This function is useful for interacting with foreign interfaces which use two pointers to refer to a range of elements in memory, as is common in C++.

1.93.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#853)

#### pub fn [as\_array](#method.as_array)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>

Gets a reference to the underlying array.

If `N` is not exactly equal to the length of `self`, then this method returns `None`.

1.93.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#872)

#### pub fn [as\_mut\_array](#method.as_mut_array)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>

Gets a mutable reference to the slice’s underlying array.

If `N` is not exactly equal to the length of `self`, then this method returns `None`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#908)

#### pub fn [swap](#method.swap)(&mut self, a: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), b: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Swaps two elements in the slice.

If `a` equals to `b`, it’s guaranteed that elements won’t change value.

##### Arguments

*   a - The index of the first element
*   b - The index of the second element

##### Panics

Panics if `a` or `b` are out of bounds.

##### Examples

```rust
let mut v = ["a", "b", "c", "d", "e"];
v.swap(2, 4);
assert!(v == ["a", "b", "e", "d", "c"]);
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#951)

#### pub unsafe fn [swap\_unchecked](#method.swap_unchecked)(&mut self, a: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), b: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`slice_swap_unchecked`)

Swaps two elements in the slice, without doing bounds checking.

For a safe alternative see [`swap`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.swap "method slice::swap").

##### Arguments

*   a - The index of the first element
*   b - The index of the second element

##### Safety

Calling this method with an out-of-bounds index is _[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)_. The caller has to ensure that `a < self.len()` and `b < self.len()`.

##### Examples

```rust
#![feature(slice_swap_unchecked)]

let mut v = ["a", "b", "c", "d"];
// SAFETY: we know that 1 and 3 are both indices of the slice
unsafe { v.swap_unchecked(1, 3) };
assert!(v == ["a", "d", "c", "b"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#981)

#### pub fn [reverse](#method.reverse)(&mut self)

Reverses the order of elements in the slice, in place.

##### Examples

```rust
let mut v = [1, 2, 3];
v.reverse();
assert!(v == [3, 2, 1]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1043)

#### pub fn [iter](#method.iter)(&self) -> [Iter](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html "struct core::slice::iter::Iter")<'\_, T> [ⓘ](#)

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

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1063)

#### pub fn [iter\_mut](#method.iter_mut)(&mut self) -> [IterMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.IterMut.html "struct core::slice::iter::IterMut")<'\_, T> [ⓘ](#)

Returns an iterator that allows modifying each value.

The iterator yields all items from start to end.

##### Examples

```rust
let x = &mut [1, 2, 4];
for elem in x.iter_mut() {
    *elem += 2;
}
assert_eq!(x, &[3, 4, 6]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1118)

#### pub fn [windows](#method.windows)(&self, size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Windows](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Windows.html "struct core::slice::iter::Windows")<'\_, T> [ⓘ](#)

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

#### pub fn [chunks](#method.chunks)(&self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Chunks](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Chunks.html "struct core::slice::iter::Chunks")<'\_, T> [ⓘ](#)

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

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1202)

#### pub fn [chunks\_mut](#method.chunks_mut)(&mut self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [ChunksMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksMut.html "struct core::slice::iter::ChunksMut")<'\_, T> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the length of the slice, then the last chunk will not have length `chunk_size`.

See [`chunks_exact_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_exact_mut "method slice::chunks_exact_mut") for a variant of this iterator that returns chunks of always exactly `chunk_size` elements, and [`rchunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_mut "method slice::rchunks_mut") for the same iterator but starting at the end of the slice.

If your `chunk_size` is a constant, consider using [`as_chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks_mut "method slice::as_chunks_mut") instead, which will give references to arrays of exactly that length, rather than slices.

##### Panics

Panics if `chunk_size` is zero.

##### Examples

```rust
let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

for chunk in v.chunks_mut(2) {
    for elem in chunk.iter_mut() {
        *elem += count;
    }
    count += 1;
}
assert_eq!(v, &[1, 1, 2, 2, 3]);
```

1.31.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1245)

#### pub fn [chunks\_exact](#method.chunks_exact)(&self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [ChunksExact](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExact.html "struct core::slice::iter::ChunksExact")<'\_, T> [ⓘ](#)

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

1.31.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1293)

#### pub fn [chunks\_exact\_mut](#method.chunks_exact_mut)(&mut self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [ChunksExactMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExactMut.html "struct core::slice::iter::ChunksExactMut")<'\_, T> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved from the `into_remainder` function of the iterator.

Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the resulting code better than in the case of [`chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_mut "method slice::chunks_mut").

See [`chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_mut "method slice::chunks_mut") for a variant of this iterator that also returns the remainder as a smaller chunk, and [`rchunks_exact_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_exact_mut "method slice::rchunks_exact_mut") for the same iterator but starting at the end of the slice.

If your `chunk_size` is a constant, consider using [`as_chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks_mut "method slice::as_chunks_mut") instead, which will give references to arrays of exactly that length, rather than slices.

##### Panics

Panics if `chunk_size` is zero.

##### Examples

```rust
let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

for chunk in v.chunks_exact_mut(2) {
    for elem in chunk.iter_mut() {
        *elem += count;
    }
    count += 1;
}
assert_eq!(v, &[1, 1, 2, 2, 0]);
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

1.88.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1501)

#### pub unsafe fn [as\_chunks\_unchecked\_mut](#method.as_chunks_unchecked_mut)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ) -> &mut \[[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Splits the slice into a slice of `N`\-element arrays, assuming that there’s no remainder.

This is the inverse operation to [`as_flattened_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened_mut "method slice::as_flattened_mut").

As this is `unsafe`, consider whether you could use [`as_chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks_mut "method slice::as_chunks_mut") or [`as_rchunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks_mut "method slice::as_rchunks_mut") instead, perhaps via something like `if let (chunks, []) = slice.as_chunks_mut()` or `let (chunks, []) = slice.as_chunks_mut() else { unreachable!() };`.

##### Safety

This may only be called when

*   The slice splits exactly into `N`\-element chunks (aka `self.len() % N == 0`).
*   `N != 0`.

##### Examples

```rust
let slice: &mut [char] = &mut ['l', 'o', 'r', 'e', 'm', '!'];
let chunks: &mut [[char; 1]] =
    // SAFETY: 1-element chunks never have remainder
    unsafe { slice.as_chunks_unchecked_mut() };
chunks[0] = ['L'];
assert_eq!(chunks, &[['L'], ['o'], ['r'], ['e'], ['m'], ['!']]);
let chunks: &mut [[char; 3]] =
    // SAFETY: The slice length (6) is a multiple of 3
    unsafe { slice.as_chunks_unchecked_mut() };
chunks[1] = ['a', 'x', '?'];
assert_eq!(slice, &['L', 'o', 'r', 'a', 'x', '?']);

// These would be unsound:
// let chunks: &[[_; 5]] = slice.as_chunks_unchecked_mut() // The slice length is not a multiple of 5
// let chunks: &[[_; 0]] = slice.as_chunks_unchecked_mut() // Zero-length chunks are never allowed
```

1.88.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1555)

#### pub fn [as\_chunks\_mut](#method.as_chunks_mut)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self) -> (&mut \[[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

Splits the slice into a slice of `N`\-element arrays, starting at the beginning of the slice, and a remainder slice with length strictly less than `N`.

The remainder is meaningful in the division sense. Given `let (chunks, remainder) = slice.as_chunks_mut()`, then:

*   `chunks.len()` equals `slice.len() / N`,
*   `remainder.len()` equals `slice.len() % N`, and
*   `slice.len()` equals `chunks.len() * N + remainder.len()`.

You can flatten the chunks back into a slice-of-`T` with [`as_flattened_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened_mut "method slice::as_flattened_mut").

##### Panics

Panics if `N` is zero.

Note that this check is against a const generic parameter, not a runtime value, and thus a particular monomorphization will either always panic or it will never panic.

##### Examples

```rust
let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

let (chunks, remainder) = v.as_chunks_mut();
remainder[0] = 9;
for chunk in chunks {
    *chunk = [count; 2];
    count += 1;
}
assert_eq!(v, &[1, 1, 2, 2, 9]);
```

1.88.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1608)

#### pub fn [as\_rchunks\_mut](#method.as_rchunks_mut)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self) -> (&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &mut \[[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\])

Splits the slice into a slice of `N`\-element arrays, starting at the end of the slice, and a remainder slice with length strictly less than `N`.

The remainder is meaningful in the division sense. Given `let (remainder, chunks) = slice.as_rchunks_mut()`, then:

*   `remainder.len()` equals `slice.len() % N`,
*   `chunks.len()` equals `slice.len() / N`, and
*   `slice.len()` equals `chunks.len() * N + remainder.len()`.

You can flatten the chunks back into a slice-of-`T` with [`as_flattened_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened_mut "method slice::as_flattened_mut").

##### Panics

Panics if `N` is zero.

Note that this check is against a const generic parameter, not a runtime value, and thus a particular monomorphization will either always panic or it will never panic.

##### Examples

```rust
let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

let (remainder, chunks) = v.as_rchunks_mut();
remainder[0] = 9;
for chunk in chunks {
    *chunk = [count; 2];
    count += 1;
}
assert_eq!(v, &[9, 1, 1, 2, 2]);
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

#### pub fn [rchunks](#method.rchunks)(&self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [RChunks](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunks.html "struct core::slice::iter::RChunks")<'\_, T> [ⓘ](#)

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

1.31.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1733)

#### pub fn [rchunks\_mut](#method.rchunks_mut)(&mut self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [RChunksMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksMut.html "struct core::slice::iter::RChunksMut")<'\_, T> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the length of the slice, then the last chunk will not have length `chunk_size`.

See [`rchunks_exact_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_exact_mut "method slice::rchunks_exact_mut") for a variant of this iterator that returns chunks of always exactly `chunk_size` elements, and [`chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_mut "method slice::chunks_mut") for the same iterator but starting at the beginning of the slice.

If your `chunk_size` is a constant, consider using [`as_rchunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks_mut "method slice::as_rchunks_mut") instead, which will give references to arrays of exactly that length, rather than slices.

##### Panics

Panics if `chunk_size` is zero.

##### Examples

```rust
let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

for chunk in v.rchunks_mut(2) {
    for elem in chunk.iter_mut() {
        *elem += count;
    }
    count += 1;
}
assert_eq!(v, &[3, 2, 2, 1, 1]);
```

1.31.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1778)

#### pub fn [rchunks\_exact](#method.rchunks_exact)(&self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [RChunksExact](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExact.html "struct core::slice::iter::RChunksExact")<'\_, T> [ⓘ](#)

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

1.31.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1827)

#### pub fn [rchunks\_exact\_mut](#method.rchunks_exact_mut)(&mut self, chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [RChunksExactMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExactMut.html "struct core::slice::iter::RChunksExactMut")<'\_, T> [ⓘ](#)

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved from the `into_remainder` function of the iterator.

Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the resulting code better than in the case of [`chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_mut "method slice::chunks_mut").

See [`rchunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_mut "method slice::rchunks_mut") for a variant of this iterator that also returns the remainder as a smaller chunk, and [`chunks_exact_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_exact_mut "method slice::chunks_exact_mut") for the same iterator but starting at the beginning of the slice.

If your `chunk_size` is a constant, consider using [`as_rchunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks_mut "method slice::as_rchunks_mut") instead, which will give references to arrays of exactly that length, rather than slices.

##### Panics

Panics if `chunk_size` is zero.

##### Examples

```rust
let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

for chunk in v.rchunks_exact_mut(2) {
    for elem in chunk.iter_mut() {
        *elem += count;
    }
    count += 1;
}
assert_eq!(v, &[0, 2, 2, 1, 1]);
```

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1867-1869)

#### pub fn [chunk\_by](#method.chunk_by)<F>(&self, pred: F) -> [ChunkBy](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkBy.html "struct core::slice::iter::ChunkBy")<'\_, T, F> [ⓘ](#)

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

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1909-1911)

#### pub fn [chunk\_by\_mut](#method.chunk_by_mut)<F>(&mut self, pred: F) -> [ChunkByMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkByMut.html "struct core::slice::iter::ChunkByMut")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over the slice producing non-overlapping mutable runs of elements using the predicate to separate them.

The predicate is called for every pair of consecutive elements, meaning that it is called on `slice[0]` and `slice[1]`, followed by `slice[1]` and `slice[2]`, and so on.

##### Examples

```rust
let slice = &mut [1, 1, 1, 3, 3, 2, 2, 2];

let mut iter = slice.chunk_by_mut(|a, b| a == b);

assert_eq!(iter.next(), Some(&mut [1, 1, 1][..]));
assert_eq!(iter.next(), Some(&mut [3, 3][..]));
assert_eq!(iter.next(), Some(&mut [2, 2, 2][..]));
assert_eq!(iter.next(), None);
```

This method can be used to extract the sorted subslices:

```rust
let slice = &mut [1, 1, 2, 3, 2, 3, 2, 3, 4];

let mut iter = slice.chunk_by_mut(|a, b| a <= b);

assert_eq!(iter.next(), Some(&mut [1, 1, 2, 3][..]));
assert_eq!(iter.next(), Some(&mut [2, 3][..]));
assert_eq!(iter.next(), Some(&mut [2, 3, 4][..]));
assert_eq!(iter.next(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1955)

#### pub fn [split\_at](#method.split_at)(&self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

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

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1989)

#### pub fn [split\_at\_mut](#method.split_at_mut)(&mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

Divides one mutable slice into two at an index.

The first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

##### Panics

Panics if `mid > len`. For a non-panicking alternative see [`split_at_mut_checked`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut_checked "method slice::split_at_mut_checked").

##### Examples

```rust
let mut v = [1, 0, 3, 0, 5, 6];
let (left, right) = v.split_at_mut(2);
assert_eq!(left, [1, 0]);
assert_eq!(right, [3, 0, 5, 6]);
left[1] = 2;
right[1] = 4;
assert_eq!(v, [1, 2, 3, 4, 5, 6]);
```

1.79.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2041)

#### pub unsafe fn [split\_at\_unchecked](#method.split_at_unchecked)(&self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

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

1.79.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2095)

#### pub unsafe fn [split\_at\_mut\_unchecked](#method.split_at_mut_unchecked)( &mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> (&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

Divides one mutable slice into two at an index, without doing bounds checking.

The first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

For a safe alternative see [`split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut "method slice::split_at_mut").

##### Safety

Calling this method with an out-of-bounds index is _[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)_ even if the resulting reference is not used. The caller has to ensure that `0 <= mid <= self.len()`.

##### Examples

```rust
let mut v = [1, 0, 3, 0, 5, 6];
// scoped to restrict the lifetime of the borrows
unsafe {
    let (left, right) = v.split_at_mut_unchecked(2);
    assert_eq!(left, [1, 0]);
    assert_eq!(right, [3, 0, 5, 6]);
    left[1] = 2;
    right[1] = 4;
}
assert_eq!(v, [1, 2, 3, 4, 5, 6]);
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2156)

#### pub fn [split\_at\_checked](#method.split_at_checked)(&self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

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

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2195)

#### pub fn [split\_at\_mut\_checked](#method.split_at_mut_checked)( &mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))>

Divides one mutable slice into two at an index, returning `None` if the slice is too short.

If `mid ≤ len` returns a pair of slices where the first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

Otherwise, if `mid > len`, returns `None`.

##### Examples

```rust
let mut v = [1, 0, 3, 0, 5, 6];

if let Some((left, right)) = v.split_at_mut_checked(2) {
    assert_eq!(left, [1, 0]);
    assert_eq!(right, [3, 0, 5, 6]);
    left[1] = 2;
    right[1] = 4;
}
assert_eq!(v, [1, 2, 3, 4, 5, 6]);

assert_eq!(None, v.split_at_mut_checked(7));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2247-2249)

#### pub fn [split](#method.split)<F>(&self, pred: F) -> [Split](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Split.html "struct core::slice::iter::Split")<'\_, T, F> [ⓘ](#)

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

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2269-2271)

#### pub fn [split\_mut](#method.split_mut)<F>(&mut self, pred: F) -> [SplitMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitMut.html "struct core::slice::iter::SplitMut")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over mutable subslices separated by elements that match `pred`. The matched element is not contained in the subslices.

##### Examples

```rust
let mut v = [10, 40, 30, 20, 60, 50];

for group in v.split_mut(|num| *num % 3 == 0) {
    group[0] = 1;
}
assert_eq!(v, [1, 40, 30, 1, 60, 1]);
```

1.51.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2305-2307)

#### pub fn [split\_inclusive](#method.split_inclusive)<F>(&self, pred: F) -> [SplitInclusive](https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusive.html "struct core::slice::iter::SplitInclusive")<'\_, T, F> [ⓘ](#)

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

1.51.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2329-2331)

#### pub fn [split\_inclusive\_mut](#method.split_inclusive_mut)<F>(&mut self, pred: F) -> [SplitInclusiveMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusiveMut.html "struct core::slice::iter::SplitInclusiveMut")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over mutable subslices separated by elements that match `pred`. The matched element is contained in the previous subslice as a terminator.

##### Examples

```rust
let mut v = [10, 40, 30, 20, 60, 50];

for group in v.split_inclusive_mut(|num| *num % 3 == 0) {
    let terminator_idx = group.len()-1;
    group[terminator_idx] = 1;
}
assert_eq!(v, [10, 40, 1, 20, 1, 1]);
```

1.27.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2365-2367)

#### pub fn [rsplit](#method.rsplit)<F>(&self, pred: F) -> [RSplit](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplit.html "struct core::slice::iter::RSplit")<'\_, T, F> [ⓘ](#)

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

1.27.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2391-2393)

#### pub fn [rsplit\_mut](#method.rsplit_mut)<F>(&mut self, pred: F) -> [RSplitMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitMut.html "struct core::slice::iter::RSplitMut")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over mutable subslices separated by elements that match `pred`, starting at the end of the slice and working backwards. The matched element is not contained in the subslices.

##### Examples

```rust
let mut v = [100, 400, 300, 200, 600, 500];

let mut count = 0;
for group in v.rsplit_mut(|num| *num % 3 == 0) {
    count += 1;
    group[0] = count;
}
assert_eq!(v, [3, 400, 300, 2, 600, 1]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2419-2421)

#### pub fn [splitn](#method.splitn)<F>(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pred: F) -> [SplitN](https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitN.html "struct core::slice::iter::SplitN")<'\_, T, F> [ⓘ](#)

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

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2445-2447)

#### pub fn [splitn\_mut](#method.splitn_mut)<F>(&mut self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pred: F) -> [SplitNMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitNMut.html "struct core::slice::iter::SplitNMut")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over mutable subslices separated by elements that match `pred`, limited to returning at most `n` items. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

##### Examples

```rust
let mut v = [10, 40, 30, 20, 60, 50];

for group in v.splitn_mut(2, |num| *num % 3 == 0) {
    group[0] = 1;
}
assert_eq!(v, [1, 40, 30, 1, 60, 50]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2474-2476)

#### pub fn [rsplitn](#method.rsplitn)<F>(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pred: F) -> [RSplitN](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitN.html "struct core::slice::iter::RSplitN")<'\_, T, F> [ⓘ](#)

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

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2501-2503)

#### pub fn [rsplitn\_mut](#method.rsplitn_mut)<F>(&mut self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pred: F) -> [RSplitNMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitNMut.html "struct core::slice::iter::RSplitNMut")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator over subslices separated by elements that match `pred` limited to returning at most `n` items. This starts at the end of the slice and works backwards. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

##### Examples

```rust
let mut s = [10, 40, 30, 20, 60, 50];

for group in s.rsplitn_mut(2, |num| *num % 3 == 0) {
    group[0] = 1;
}
assert_eq!(s, [1, 40, 30, 20, 60, 1]);
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

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3134-3136)

#### pub fn [sort\_unstable](#method.sort_unstable)(&mut self)

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Sorts the slice in ascending order **without** preserving the initial order of equal elements.

This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not allocate), and _O_(_n_ \* log(_n_)) worst-case.

If the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

For example `|a, b| (a - b).cmp(a)` is a comparison function that is neither transitive nor reflexive nor total, `a < b < c < a` with `a = 1, b = 2, c = 3`. For more information and examples see the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") documentation.

All original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. Same is true if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` panics.

Sorting types that only implement [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") such as [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") and [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64") require additional precautions. For example, `f32::NAN != f32::NAN`, which doesn’t fulfill the reflexivity requirement of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"). By using an alternative comparison function with `slice::sort_unstable_by` such as [`f32::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.total_cmp "method f32::total_cmp") or [`f64::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.total_cmp "method f64::total_cmp") that defines a [total order](https://en.wikipedia.org/wiki/Total_order) users can sort slices containing floating-point values. Alternatively, if all values in the slice are guaranteed to be in a subset for which [`PartialOrd::partial_cmp`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp "method core::cmp::PartialOrd::partial_cmp") forms a [total order](https://en.wikipedia.org/wiki/Total_order), it’s possible to sort the slice with `sort_unstable_by(|a, b| a.partial_cmp(b).unwrap())`.

##### Current implementation

The current implementation is based on [ipnsort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll and Orson Peters, which combines the fast average case of quicksort with the fast worst case of heapsort, achieving linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the expected time to sort the data is _O_(_n_ \* log(_k_)).

It is typically faster than stable sorting, except in a few special cases, e.g., when the slice is partially sorted.

##### Panics

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") implementation panics.

##### Examples

```rust
let mut v = [4, -5, 1, -3, 2];

v.sort_unstable();
assert_eq!(v, [-5, -3, 1, 2, 4]);
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3189-3191)

#### pub fn [sort\_unstable\_by](#method.sort_unstable_by)<F>(&mut self, compare: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Sorts the slice in ascending order with a comparison function, **without** preserving the initial order of equal elements.

This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not allocate), and _O_(_n_ \* log(_n_)) worst-case.

If the comparison function `compare` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

For example `|a, b| (a - b).cmp(a)` is a comparison function that is neither transitive nor reflexive nor total, `a < b < c < a` with `a = 1, b = 2, c = 3`. For more information and examples see the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") documentation.

All original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. Same is true if `compare` panics.

##### Current implementation

The current implementation is based on [ipnsort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll and Orson Peters, which combines the fast average case of quicksort with the fast worst case of heapsort, achieving linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the expected time to sort the data is _O_(_n_ \* log(_k_)).

It is typically faster than stable sorting, except in a few special cases, e.g., when the slice is partially sorted.

##### Panics

May panic if the `compare` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if the `compare` itself panics.

##### Examples

```rust
let mut v = [4, -5, 1, -3, 2];
v.sort_unstable_by(|a, b| a.cmp(b));
assert_eq!(v, [-5, -3, 1, 2, 4]);

// reverse sorting
v.sort_unstable_by(|a, b| b.cmp(a));
assert_eq!(v, [4, 2, 1, -3, -5]);
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3241-3244)

#### pub fn [sort\_unstable\_by\_key](#method.sort_unstable_by_key)<K, F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> K, K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Sorts the slice in ascending order with a key extraction function, **without** preserving the initial order of equal elements.

This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not allocate), and _O_(_n_ \* log(_n_)) worst-case.

If the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

For example `|a, b| (a - b).cmp(a)` is a comparison function that is neither transitive nor reflexive nor total, `a < b < c < a` with `a = 1, b = 2, c = 3`. For more information and examples see the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") documentation.

All original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. Same is true if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` panics.

##### Current implementation

The current implementation is based on [ipnsort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll and Orson Peters, which combines the fast average case of quicksort with the fast worst case of heapsort, achieving linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the expected time to sort the data is _O_(_n_ \* log(_k_)).

It is typically faster than stable sorting, except in a few special cases, e.g., when the slice is partially sorted.

##### Panics

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") implementation panics.

##### Examples

```rust
let mut v = [4i32, -5, 1, -3, 2];

v.sort_unstable_by_key(|k| k.abs());
assert_eq!(v, [1, 2, -3, 4, -5]);
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3311-3314)

#### pub fn [partial\_sort\_unstable](#method.partial_sort_unstable)<R>(&mut self, range: R)

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

🔬This is a nightly-only experimental API. (`slice_partial_sort_unstable`)

Partially sorts the slice in ascending order **without** preserving the initial order of equal elements.

Upon completion, for the specified range `start..end`, it’s guaranteed that:

1.  Every element in `self[..start]` is smaller than or equal to
2.  Every element in `self[start..end]`, which is sorted, and smaller than or equal to
3.  Every element in `self[end..]`.

This partial sort is unstable, meaning it may reorder equal elements in the specified range. It may reorder elements outside the specified range as well, but the guarantees above still hold.

This partial sort is in-place (i.e., does not allocate), and _O_(_n_ + _k_ \* log(_k_)) worst-case, where _n_ is the length of the slice and _k_ is the length of the specified range.

See the documentation of [`sort_unstable`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable "method slice::sort_unstable") for implementation notes.

##### Panics

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` does not implement a total order, or if the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") implementation panics, or if the specified range is out of bounds.

##### Examples

```rust
#![feature(slice_partial_sort_unstable)]

let mut v = [4, -5, 1, -3, 2];

// empty range at the beginning, nothing changed
v.partial_sort_unstable(0..0);
assert_eq!(v, [4, -5, 1, -3, 2]);

// empty range in the middle, partitioning the slice
v.partial_sort_unstable(2..2);
for i in 0..2 {
   assert!(v[i] <= v[2]);
}
for i in 3..v.len() {
  assert!(v[2] <= v[i]);
}

// single element range, same as select_nth_unstable
v.partial_sort_unstable(2..3);
for i in 0..2 {
   assert!(v[i] <= v[2]);
}
for i in 3..v.len() {
  assert!(v[2] <= v[i]);
}

// partial sort a subrange
v.partial_sort_unstable(1..4);
assert_eq!(&v[1..4], [-3, 1, 2]);

// partial sort the whole range, same as sort_unstable
v.partial_sort_unstable(..);
assert_eq!(v, [-5, -3, 1, 2, 4]);
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3382-3385)

#### pub fn [partial\_sort\_unstable\_by](#method.partial_sort_unstable_by)<F, R>(&mut self, range: R, compare: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"), R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

🔬This is a nightly-only experimental API. (`slice_partial_sort_unstable`)

Partially sorts the slice in ascending order with a comparison function, **without** preserving the initial order of equal elements.

Upon completion, for the specified range `start..end`, it’s guaranteed that:

1.  Every element in `self[..start]` is smaller than or equal to
2.  Every element in `self[start..end]`, which is sorted, and smaller than or equal to
3.  Every element in `self[end..]`.

This partial sort is unstable, meaning it may reorder equal elements in the specified range. It may reorder elements outside the specified range as well, but the guarantees above still hold.

This partial sort is in-place (i.e., does not allocate), and _O_(_n_ + _k_ \* log(_k_)) worst-case, where _n_ is the length of the slice and _k_ is the length of the specified range.

See the documentation of [`sort_unstable_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable_by "method slice::sort_unstable_by") for implementation notes.

##### Panics

May panic if the `compare` does not implement a total order, or if the `compare` itself panics, or if the specified range is out of bounds.

##### Examples

```rust
#![feature(slice_partial_sort_unstable)]

let mut v = [4, -5, 1, -3, 2];

// empty range at the beginning, nothing changed
v.partial_sort_unstable_by(0..0, |a, b| b.cmp(a));
assert_eq!(v, [4, -5, 1, -3, 2]);

// empty range in the middle, partitioning the slice
v.partial_sort_unstable_by(2..2, |a, b| b.cmp(a));
for i in 0..2 {
   assert!(v[i] >= v[2]);
}
for i in 3..v.len() {
  assert!(v[2] >= v[i]);
}

// single element range, same as select_nth_unstable
v.partial_sort_unstable_by(2..3, |a, b| b.cmp(a));
for i in 0..2 {
   assert!(v[i] >= v[2]);
}
for i in 3..v.len() {
  assert!(v[2] >= v[i]);
}

// partial sort a subrange
v.partial_sort_unstable_by(1..4, |a, b| b.cmp(a));
assert_eq!(&v[1..4], [2, 1, -3]);

// partial sort the whole range, same as sort_unstable
v.partial_sort_unstable_by(.., |a, b| b.cmp(a));
assert_eq!(v, [4, 2, 1, -3, -5]);
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3453-3457)

#### pub fn [partial\_sort\_unstable\_by\_key](#method.partial_sort_unstable_by_key)<K, F, R>(&mut self, range: R, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> K, K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

🔬This is a nightly-only experimental API. (`slice_partial_sort_unstable`)

Partially sorts the slice in ascending order with a key extraction function, **without** preserving the initial order of equal elements.

Upon completion, for the specified range `start..end`, it’s guaranteed that:

1.  Every element in `self[..start]` is smaller than or equal to
2.  Every element in `self[start..end]`, which is sorted, and smaller than or equal to
3.  Every element in `self[end..]`.

This partial sort is unstable, meaning it may reorder equal elements in the specified range. It may reorder elements outside the specified range as well, but the guarantees above still hold.

This partial sort is in-place (i.e., does not allocate), and _O_(_n_ + _k_ \* log(_k_)) worst-case, where _n_ is the length of the slice and _k_ is the length of the specified range.

See the documentation of [`sort_unstable_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable_by_key "method slice::sort_unstable_by_key") for implementation notes.

##### Panics

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a total order, or if the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") implementation panics, or if the specified range is out of bounds.

##### Examples

```rust
#![feature(slice_partial_sort_unstable)]

let mut v = [4i32, -5, 1, -3, 2];

// empty range at the beginning, nothing changed
v.partial_sort_unstable_by_key(0..0, |k| k.abs());
assert_eq!(v, [4, -5, 1, -3, 2]);

// empty range in the middle, partitioning the slice
v.partial_sort_unstable_by_key(2..2, |k| k.abs());
for i in 0..2 {
   assert!(v[i].abs() <= v[2].abs());
}
for i in 3..v.len() {
  assert!(v[2].abs() <= v[i].abs());
}

// single element range, same as select_nth_unstable
v.partial_sort_unstable_by_key(2..3, |k| k.abs());
for i in 0..2 {
   assert!(v[i].abs() <= v[2].abs());
}
for i in 3..v.len() {
  assert!(v[2].abs() <= v[i].abs());
}

// partial sort a subrange
v.partial_sort_unstable_by_key(1..4, |k| k.abs());
assert_eq!(&v[1..4], [2, -3, 4]);

// partial sort the whole range, same as sort_unstable
v.partial_sort_unstable_by_key(.., |k| k.abs());
assert_eq!(v, [1, 2, -3, 4, -5]);
```

1.49.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3517-3519)

#### pub fn [select\_nth\_unstable](#method.select_nth_unstable)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> (&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Reorders the slice such that the element at `index` is at a sort-order position. All elements before `index` will be `<=` to this value, and all elements after will be `>=` to it.

This reordering is unstable (i.e. any element that compares equal to the nth element may end up at that position), in-place (i.e. does not allocate), and runs in _O_(_n_) time. This function is also known as “kth element” in other libraries.

Returns a triple that partitions the reordered slice:

*   The unsorted subslice before `index`, whose elements all satisfy `x <= self[index]`.
    
*   The element at `index`.
    
*   The unsorted subslice after `index`, whose elements all satisfy `x >= self[index]`.
    

##### Current implementation

The current algorithm is an introselect implementation based on [ipnsort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll and Orson Peters, which is also the basis for [`sort_unstable`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable "method slice::sort_unstable"). The fallback algorithm is Median of Medians using Tukey’s Ninther for pivot selection, which guarantees linear runtime for all inputs.

##### Panics

Panics when `index >= len()`, and so always panics on empty slices.

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order).

##### Examples

```rust
let mut v = [-5i32, 4, 2, -3, 1];

// Find the items `<=` to the median, the median itself, and the items `>=` to it.
let (lesser, median, greater) = v.select_nth_unstable(2);

assert!(lesser == [-3, -5] || lesser == [-5, -3]);
assert_eq!(median, &mut 1);
assert!(greater == [4, 2] || greater == [2, 4]);

// We are only guaranteed the slice will be one of the following, based on the way we sort
// about the specified index.
assert!(v == [-3, -5, 1, 2, 4] ||
        v == [-5, -3, 1, 2, 4] ||
        v == [-3, -5, 1, 4, 2] ||
        v == [-5, -3, 1, 4, 2]);
```

1.49.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3582-3588)

#### pub fn [select\_nth\_unstable\_by](#method.select_nth_unstable_by)<F>( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), compare: F, ) -> (&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Reorders the slice with a comparator function such that the element at `index` is at a sort-order position. All elements before `index` will be `<=` to this value, and all elements after will be `>=` to it, according to the comparator function.

This reordering is unstable (i.e. any element that compares equal to the nth element may end up at that position), in-place (i.e. does not allocate), and runs in _O_(_n_) time. This function is also known as “kth element” in other libraries.

Returns a triple partitioning the reordered slice:

*   The unsorted subslice before `index`, whose elements all satisfy `compare(x, self[index]).is_le()`.
    
*   The element at `index`.
    
*   The unsorted subslice after `index`, whose elements all satisfy `compare(x, self[index]).is_ge()`.
    

##### Current implementation

The current algorithm is an introselect implementation based on [ipnsort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll and Orson Peters, which is also the basis for [`sort_unstable`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable "method slice::sort_unstable"). The fallback algorithm is Median of Medians using Tukey’s Ninther for pivot selection, which guarantees linear runtime for all inputs.

##### Panics

Panics when `index >= len()`, and so always panics on empty slices.

May panic if `compare` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order).

##### Examples

```rust
let mut v = [-5i32, 4, 2, -3, 1];

// Find the items `>=` to the median, the median itself, and the items `<=` to it, by using
// a reversed comparator.
let (before, median, after) = v.select_nth_unstable_by(2, |a, b| b.cmp(a));

assert!(before == [4, 2] || before == [2, 4]);
assert_eq!(median, &mut 1);
assert!(after == [-3, -5] || after == [-5, -3]);

// We are only guaranteed the slice will be one of the following, based on the way we sort
// about the specified index.
assert!(v == [2, 4, 1, -5, -3] ||
        v == [2, 4, 1, -3, -5] ||
        v == [4, 2, 1, -5, -3] ||
        v == [4, 2, 1, -3, -5]);
```

1.49.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3649-3656)

#### pub fn [select\_nth\_unstable\_by\_key](#method.select_nth_unstable_by_key)<K, F>( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), f: F, ) -> (&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> K, K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Reorders the slice with a key extraction function such that the element at `index` is at a sort-order position. All elements before `index` will have keys `<=` to the key at `index`, and all elements after will have keys `>=` to it.

This reordering is unstable (i.e. any element that compares equal to the nth element may end up at that position), in-place (i.e. does not allocate), and runs in _O_(_n_) time. This function is also known as “kth element” in other libraries.

Returns a triple partitioning the reordered slice:

*   The unsorted subslice before `index`, whose elements all satisfy `f(x) <= f(self[index])`.
    
*   The element at `index`.
    
*   The unsorted subslice after `index`, whose elements all satisfy `f(x) >= f(self[index])`.
    

##### Current implementation

The current algorithm is an introselect implementation based on [ipnsort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll and Orson Peters, which is also the basis for [`sort_unstable`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable "method slice::sort_unstable"). The fallback algorithm is Median of Medians using Tukey’s Ninther for pivot selection, which guarantees linear runtime for all inputs.

##### Panics

Panics when `index >= len()`, meaning it always panics on empty slices.

May panic if `K: Ord` does not implement a total order.

##### Examples

```rust
let mut v = [-5i32, 4, 1, -3, 2];

// Find the items `<=` to the absolute median, the absolute median itself, and the items
// `>=` to it.
let (lesser, median, greater) = v.select_nth_unstable_by_key(2, |a| a.abs());

assert!(lesser == [1, 2] || lesser == [2, 1]);
assert_eq!(median, &mut -3);
assert!(greater == [4, -5] || greater == [-5, 4]);

// We are only guaranteed the slice will be one of the following, based on the way we sort
// about the specified index.
assert!(v == [1, 2, -3, 4, -5] ||
        v == [1, 2, -3, -5, 4] ||
        v == [2, 1, -3, 4, -5] ||
        v == [2, 1, -3, -5, 4]);
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3683-3685)

#### pub fn [partition\_dedup](#method.partition_dedup)(&mut self) -> (&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

🔬This is a nightly-only experimental API. (`slice_partition_dedup`)

Moves all consecutive repeated elements to the end of the slice according to the [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") trait implementation.

Returns two slices. The first contains no consecutive repeated elements. The second contains all the duplicates in no specified order.

If the slice is sorted, the first returned slice contains no duplicates.

##### Examples

```rust
#![feature(slice_partition_dedup)]

let mut slice = [1, 2, 2, 3, 3, 2, 1, 1];

let (dedup, duplicates) = slice.partition_dedup();

assert_eq!(dedup, [1, 2, 3, 2, 1]);
assert_eq!(duplicates, [2, 3, 1]);
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3717-3719)

#### pub fn [partition\_dedup\_by](#method.partition_dedup_by)<F>(&mut self, same\_bucket: F) -> (&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

🔬This is a nightly-only experimental API. (`slice_partition_dedup`)

Moves all but the first of consecutive elements to the end of the slice satisfying a given equality relation.

Returns two slices. The first contains no consecutive repeated elements. The second contains all the duplicates in no specified order.

The `same_bucket` function is passed references to two elements from the slice and must determine if the elements compare equal. The elements are passed in opposite order from their order in the slice, so if `same_bucket(a, b)` returns `true`, `a` is moved at the end of the slice.

If the slice is sorted, the first returned slice contains no duplicates.

##### Examples

```rust
#![feature(slice_partition_dedup)]

let mut slice = ["foo", "Foo", "BAZ", "Bar", "bar", "baz", "BAZ"];

let (dedup, duplicates) = slice.partition_dedup_by(|a, b| a.eq_ignore_ascii_case(b));

assert_eq!(dedup, ["foo", "BAZ", "Bar", "baz"]);
assert_eq!(duplicates, ["bar", "Foo", "BAZ"]);
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3843-3846)

#### pub fn [partition\_dedup\_by\_key](#method.partition_dedup_by_key)<K, F>(&mut self, key: F) -> (&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> K, K: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

🔬This is a nightly-only experimental API. (`slice_partition_dedup`)

Moves all but the first of consecutive elements to the end of the slice that resolve to the same key.

Returns two slices. The first contains no consecutive repeated elements. The second contains all the duplicates in no specified order.

If the slice is sorted, the first returned slice contains no duplicates.

##### Examples

```rust
#![feature(slice_partition_dedup)]

let mut slice = [10, 20, 21, 30, 30, 20, 11, 13];

let (dedup, duplicates) = slice.partition_dedup_by_key(|i| *i / 10);

assert_eq!(dedup, [10, 20, 30, 20, 11]);
assert_eq!(duplicates, [21, 30, 13]);
```

1.26.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3885)

#### pub fn [rotate\_left](#method.rotate_left)(&mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Rotates the slice in-place such that the first `mid` elements of the slice move to the end while the last `self.len() - mid` elements move to the front.

After calling `rotate_left`, the element previously at index `mid` will become the first element in the slice.

##### Panics

This function will panic if `mid` is greater than the length of the slice. Note that `mid == self.len()` does _not_ panic and is a no-op rotation.

##### Complexity

Takes linear (in `self.len()`) time.

##### Examples

```rust
let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
a.rotate_left(2);
assert_eq!(a, ['c', 'd', 'e', 'f', 'a', 'b']);
```

Rotating a subslice:

```rust
let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
a[1..5].rotate_left(1);
assert_eq!(a, ['a', 'c', 'd', 'e', 'b', 'f']);
```

1.26.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3931)

#### pub fn [rotate\_right](#method.rotate_right)(&mut self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Rotates the slice in-place such that the first `self.len() - k` elements of the slice move to the end while the last `k` elements move to the front.

After calling `rotate_right`, the element previously at index `self.len() - k` will become the first element in the slice.

##### Panics

This function will panic if `k` is greater than the length of the slice. Note that `k == self.len()` does _not_ panic and is a no-op rotation.

##### Complexity

Takes linear (in `self.len()`) time.

##### Examples

```rust
let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
a.rotate_right(2);
assert_eq!(a, ['e', 'f', 'a', 'b', 'c', 'd']);
```

Rotating a subslice:

```rust
let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
a[1..5].rotate_right(1);
assert_eq!(a, ['a', 'e', 'b', 'c', 'd', 'f']);
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4005)

#### pub fn [shift\_left](#method.shift_left)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self, inserted: [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)) -> [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

🔬This is a nightly-only experimental API. (`slice_shift`)

Moves the elements of this slice `N` places to the left, returning the ones that “fall off” the front, and putting `inserted` at the end.

Equivalently, you can think of concatenating `self` and `inserted` into one long sequence, then returning the left-most `N` items and the rest into `self`:

```
self (before)    inserted
          vvvvvvvvvvvvvvv  vvv
          [1, 2, 3, 4, 5]  [9]
       ↙   ↙  ↙  ↙  ↙   ↙
     [1]  [2, 3, 4, 5, 9]
     ^^^  ^^^^^^^^^^^^^^^
returned  self (after)
```

See also [`Self::shift_right`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.shift_right "method slice::shift_right") and compare [`Self::rotate_left`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rotate_left "method slice::rotate_left").

##### Examples

```rust
#![feature(slice_shift)]

// Same as the diagram above
let mut a = [1, 2, 3, 4, 5];
let inserted = [9];
let returned = a.shift_left(inserted);
assert_eq!(returned, [1]);
assert_eq!(a, [2, 3, 4, 5, 9]);

// You can shift multiple items at a time
let mut a = *b"Hello world";
assert_eq!(a.shift_left(*b" peace"), *b"Hello ");
assert_eq!(a, *b"world peace");

// The name comes from this operation's similarity to bitshifts
let mut a: u8 = 0b10010110;
a <<= 3;
assert_eq!(a, 0b10110000_u8);
let mut a: [_; 8] = [1, 0, 0, 1, 0, 1, 1, 0];
a.shift_left([0; 3]);
assert_eq!(a, [1, 0, 1, 1, 0, 0, 0, 0]);

// Remember you can sub-slice to affect less that the whole slice.
// For example, this is similar to `.remove(1)` + `.insert(4, 'Z')`
let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
assert_eq!(a[1..=4].shift_left(['Z']), ['b']);
assert_eq!(a, ['a', 'c', 'd', 'e', 'Z', 'f']);

// If the size matches it's equivalent to `mem::replace`
let mut a = [1, 2, 3];
assert_eq!(a.shift_left([7, 8, 9]), [1, 2, 3]);
assert_eq!(a, [7, 8, 9]);

// Some of the "inserted" elements end up returned if the slice is too short
let mut a = [];
assert_eq!(a.shift_left([1, 2, 3]), [1, 2, 3]);
let mut a = [9];
assert_eq!(a.shift_left([1, 2, 3]), [9, 1, 2]);
assert_eq!(a, [3]);
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4109)

#### pub fn [shift\_right](#method.shift_right)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self, inserted: [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)) -> [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

🔬This is a nightly-only experimental API. (`slice_shift`)

Moves the elements of this slice `N` places to the right, returning the ones that “fall off” the back, and putting `inserted` at the beginning.

Equivalently, you can think of concatenating `inserted` and `self` into one long sequence, then returning the right-most `N` items and the rest into `self`:

```
inserted  self (before)
     vvv  vvvvvvvvvvvvvvv
     [0]  [5, 6, 7, 8, 9]
       ↘   ↘  ↘  ↘  ↘   ↘
          [0, 5, 6, 7, 8]  [9]
          ^^^^^^^^^^^^^^^  ^^^
          self (after)     returned
```

See also [`Self::shift_left`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.shift_left "method slice::shift_left") and compare [`Self::rotate_right`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rotate_right "method slice::rotate_right").

##### Examples

```rust
#![feature(slice_shift)]

// Same as the diagram above
let mut a = [5, 6, 7, 8, 9];
let inserted = [0];
let returned = a.shift_right(inserted);
assert_eq!(returned, [9]);
assert_eq!(a, [0, 5, 6, 7, 8]);

// The name comes from this operation's similarity to bitshifts
let mut a: u8 = 0b10010110;
a >>= 3;
assert_eq!(a, 0b00010010_u8);
let mut a: [_; 8] = [1, 0, 0, 1, 0, 1, 1, 0];
a.shift_right([0; 3]);
assert_eq!(a, [0, 0, 0, 1, 0, 0, 1, 0]);

// Remember you can sub-slice to affect less that the whole slice.
// For example, this is similar to `.remove(4)` + `.insert(1, 'Z')`
let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
assert_eq!(a[1..=4].shift_right(['Z']), ['e']);
assert_eq!(a, ['a', 'Z', 'b', 'c', 'd', 'f']);

// If the size matches it's equivalent to `mem::replace`
let mut a = [1, 2, 3];
assert_eq!(a.shift_right([7, 8, 9]), [1, 2, 3]);
assert_eq!(a, [7, 8, 9]);

// Some of the "inserted" elements end up returned if the slice is too short
let mut a = [];
assert_eq!(a.shift_right([1, 2, 3]), [1, 2, 3]);
let mut a = [9];
assert_eq!(a.shift_right([1, 2, 3]), [2, 3, 9]);
assert_eq!(a, [1]);
```

1.50.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4167-4169)

#### pub fn [fill](#method.fill)(&mut self, value: T)

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Fills `self` with elements by cloning `value`.

##### Examples

```rust
let mut buf = vec![0; 10];
buf.fill(1);
assert_eq!(buf, vec![1; 10]);
```

1.51.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4191-4193)

#### pub fn [fill\_with](#method.fill_with)<F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")() -> T,

Fills `self` with elements returned by calling a closure repeatedly.

This method uses a closure to create new values. If you’d rather [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") a given value, use [`fill`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.fill "method slice::fill"). If you want to use the [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") trait to generate values, you can pass [`Default::default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default") as the argument.

##### Examples

```rust
let mut buf = vec![1; 10];
buf.fill_with(Default::default);
assert_eq!(buf, vec![0; 10]);
```

1.7.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4255-4257)

#### pub fn [clone\_from\_slice](#method.clone_from_slice)(&mut self, src: &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Copies the elements from `src` into `self`.

The length of `src` must be the same as `self`.

##### Panics

This function will panic if the two slices have different lengths.

##### Examples

Cloning two elements from a slice into another:

```rust
let src = [1, 2, 3, 4];
let mut dst = [0, 0];

// Because the slices have to be the same length,
// we slice the source slice from four elements
// to two. It will panic if we don't do this.
dst.clone_from_slice(&src[2..]);

assert_eq!(src, [1, 2, 3, 4]);
assert_eq!(dst, [3, 4]);
```

Rust enforces that there can only be one mutable reference with no immutable references to a particular piece of data in a particular scope. Because of this, attempting to use `clone_from_slice` on a single slice will result in a compile failure:

[ⓘ](# "This example deliberately fails to compile")

```rust
let mut slice = [1, 2, 3, 4, 5];

slice[..2].clone_from_slice(&slice[3..]); // compile fail!
```

To work around this, we can use [`split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut "method slice::split_at_mut") to create two distinct sub-slices from a slice:

```rust
let mut slice = [1, 2, 3, 4, 5];

{
    let (left, right) = slice.split_at_mut(2);
    left.clone_from_slice(&right[1..]);
}

assert_eq!(slice, [4, 5, 3, 4, 5]);
```

1.9.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4321-4323)

#### pub fn [copy\_from\_slice](#method.copy_from_slice)(&mut self, src: &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

where T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"),

Copies all elements from `src` into `self`, using a memcpy.

The length of `src` must be the same as `self`.

If `T` does not implement `Copy`, use [`clone_from_slice`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.clone_from_slice "method slice::clone_from_slice").

##### Panics

This function will panic if the two slices have different lengths.

##### Examples

Copying two elements from a slice into another:

```rust
let src = [1, 2, 3, 4];
let mut dst = [0, 0];

// Because the slices have to be the same length,
// we slice the source slice from four elements
// to two. It will panic if we don't do this.
dst.copy_from_slice(&src[2..]);

assert_eq!(src, [1, 2, 3, 4]);
assert_eq!(dst, [3, 4]);
```

Rust enforces that there can only be one mutable reference with no immutable references to a particular piece of data in a particular scope. Because of this, attempting to use `copy_from_slice` on a single slice will result in a compile failure:

[ⓘ](# "This example deliberately fails to compile")

```rust
let mut slice = [1, 2, 3, 4, 5];

slice[..2].copy_from_slice(&slice[3..]); // compile fail!
```

To work around this, we can use [`split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut "method slice::split_at_mut") to create two distinct sub-slices from a slice:

```rust
let mut slice = [1, 2, 3, 4, 5];

{
    let (left, right) = slice.split_at_mut(2);
    left.copy_from_slice(&right[1..]);
}

assert_eq!(slice, [4, 5, 3, 4, 5]);
```

1.37.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4356-4358)

#### pub fn [copy\_within](#method.copy_within)<R>(&mut self, src: R, dest: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"),

Copies elements from one part of the slice to another part of itself, using a memmove.

`src` is the range within `self` to copy from. `dest` is the starting index of the range within `self` to copy to, which will have the same length as `src`. The two ranges may overlap. The ends of the two ranges must be less than or equal to `self.len()`.

##### Panics

This function will panic if either range exceeds the end of the slice, or if the end of `src` is before the start.

##### Examples

Copying four bytes within a slice:

```rust
let mut bytes = *b"Hello, World!";

bytes.copy_within(1..5, 8);

assert_eq!(&bytes, b"Hello, Wello!");
```

1.27.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4424)

#### pub fn [swap\_with\_slice](#method.swap_with_slice)(&mut self, other: &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

Swaps all elements in `self` with those in `other`.

The length of `other` must be the same as `self`.

##### Panics

This function will panic if the two slices have different lengths.

##### Example

Swapping two elements across slices:

```rust
let mut slice1 = [0, 0];
let mut slice2 = [1, 2, 3, 4];

slice1.swap_with_slice(&mut slice2[2..]);

assert_eq!(slice1, [3, 4]);
assert_eq!(slice2, [1, 2, 0, 0]);
```

Rust enforces that there can only be one mutable reference to a particular piece of data in a particular scope. Because of this, attempting to use `swap_with_slice` on a single slice will result in a compile failure:

[ⓘ](# "This example deliberately fails to compile")

```rust
let mut slice = [1, 2, 3, 4, 5];
slice[..2].swap_with_slice(&mut slice[3..]); // compile fail!
```

To work around this, we can use [`split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut "method slice::split_at_mut") to create two distinct mutable sub-slices from a slice:

```rust
let mut slice = [1, 2, 3, 4, 5];

{
    let (left, right) = slice.split_at_mut(2);
    left.swap_with_slice(&mut right[1..]);
}

assert_eq!(slice, [4, 5, 3, 1, 2]);
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

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4566)

#### pub unsafe fn [align\_to\_mut](#method.align_to_mut)<U>(&mut self) -> (&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &mut [\[U\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

Transmutes the mutable slice to a mutable slice of another type, ensuring alignment of the types is maintained.

This method splits the slice into three distinct slices: prefix, correctly aligned middle slice of a new type, and the suffix slice. The middle part will be as big as possible under the given alignment constraint and element size.

This method has no purpose when either input element `T` or output element `U` are zero-sized and will return the original slice without splitting anything.

##### Safety

This method is essentially a `transmute` with respect to the elements in the returned middle slice, so all the usual caveats pertaining to `transmute::<T, U>` also apply here.

##### Examples

Basic usage:

```rust
unsafe {
    let mut bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
    let (prefix, shorts, suffix) = bytes.align_to_mut::<u16>();
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

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4692-4695)

#### pub fn [as\_simd\_mut](#method.as_simd_mut)<const LANES: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ) -> (&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &mut \[[Simd](https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html "struct core::core_simd::vector::Simd")<T, LANES>\], &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

where [Simd](https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html "struct core::core_simd::vector::Simd")<T, LANES>: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<[\[T; LANES\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>, T: [SimdElement](https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html "trait core::core_simd::vector::SimdElement"),

🔬This is a nightly-only experimental API. (`portable_simd`)

Splits a mutable slice into a mutable prefix, a middle of aligned SIMD types, and a mutable suffix.

This is a safe wrapper around [`slice::align_to_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.align_to_mut "method slice::align_to_mut"), so inherits the same guarantees as that method.

This is the mutable version of [`slice::as_simd`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_simd "method slice::as_simd"); see that for examples.

##### Panics

This will panic if the size of the SIMD type is different from `LANES` times that of the scalar.

At the time of writing, the trait restrictions on `Simd<T, LANES>` keeps that from ever happening, as only power-of-two numbers of lanes are supported. It’s possible that, in the future, those restrictions might be lifted in a way that would make it possible to see panics from this method for something like `LANES == 3`.

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

1.87.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4908-4911)

#### pub fn [split\_off](#method.split_off)<'a, R>(self: &mut &'a [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), range: R) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

where R: [OneSidedRange](https://doc.rust-lang.org/nightly/core/ops/range/trait.OneSidedRange.html "trait core::ops::range::OneSidedRange")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Removes the subslice corresponding to the given range and returns a reference to it.

Returns `None` and does not modify the slice if the given range is out of bounds.

Note that this method only accepts one-sided ranges such as `2..` or `..6`, but not `2..6`.

##### Examples

Splitting off the first three elements of a slice:

```rust
let mut slice: &[_] = &['a', 'b', 'c', 'd'];
let mut first_three = slice.split_off(..3).unwrap();

assert_eq!(slice, &['d']);
assert_eq!(first_three, &['a', 'b', 'c']);
```

Splitting off a slice starting with the third element:

```rust
let mut slice: &[_] = &['a', 'b', 'c', 'd'];
let mut tail = slice.split_off(2..).unwrap();

assert_eq!(slice, &['a', 'b']);
assert_eq!(tail, &['c', 'd']);
```

Getting `None` when `range` is out of bounds:

```rust
let mut slice: &[_] = &['a', 'b', 'c', 'd'];

assert_eq!(None, slice.split_off(5..));
assert_eq!(None, slice.split_off(..5));
assert_eq!(None, slice.split_off(..=4));
let expected: &[char] = &['a', 'b', 'c', 'd'];
assert_eq!(Some(expected), slice.split_off(..4));
```

1.87.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4974-4977)

#### pub fn [split\_off\_mut](#method.split_off_mut)<'a, R>( self: &mut &'a mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), range: R, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

where R: [OneSidedRange](https://doc.rust-lang.org/nightly/core/ops/range/trait.OneSidedRange.html "trait core::ops::range::OneSidedRange")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Removes the subslice corresponding to the given range and returns a mutable reference to it.

Returns `None` and does not modify the slice if the given range is out of bounds.

Note that this method only accepts one-sided ranges such as `2..` or `..6`, but not `2..6`.

##### Examples

Splitting off the first three elements of a slice:

```rust
let mut slice: &mut [_] = &mut ['a', 'b', 'c', 'd'];
let mut first_three = slice.split_off_mut(..3).unwrap();

assert_eq!(slice, &mut ['d']);
assert_eq!(first_three, &mut ['a', 'b', 'c']);
```

Splitting off a slice starting with the third element:

```rust
let mut slice: &mut [_] = &mut ['a', 'b', 'c', 'd'];
let mut tail = slice.split_off_mut(2..).unwrap();

assert_eq!(slice, &mut ['a', 'b']);
assert_eq!(tail, &mut ['c', 'd']);
```

Getting `None` when `range` is out of bounds:

```rust
let mut slice: &mut [_] = &mut ['a', 'b', 'c', 'd'];

assert_eq!(None, slice.split_off_mut(5..));
assert_eq!(None, slice.split_off_mut(..5));
assert_eq!(None, slice.split_off_mut(..=4));
let expected: &mut [_] = &mut ['a', 'b', 'c', 'd'];
assert_eq!(Some(expected), slice.split_off_mut(..4));
```

1.87.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5012)

#### pub fn [split\_off\_first](#method.split_off_first)<'a>(self: &mut &'a [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Removes the first element of the slice and returns a reference to it.

Returns `None` if the slice is empty.

##### Examples

```rust
let mut slice: &[_] = &['a', 'b', 'c'];
let first = slice.split_off_first().unwrap();

assert_eq!(slice, &['b', 'c']);
assert_eq!(first, &'a');
```

1.87.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5037)

#### pub fn [split\_off\_first\_mut](#method.split_off_first_mut)<'a>(self: &mut &'a mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Removes the first element of the slice and returns a mutable reference to it.

Returns `None` if the slice is empty.

##### Examples

```rust
let mut slice: &mut [_] = &mut ['a', 'b', 'c'];
let first = slice.split_off_first_mut().unwrap();
*first = 'd';

assert_eq!(slice, &['b', 'c']);
assert_eq!(first, &'d');
```

1.87.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5062)

#### pub fn [split\_off\_last](#method.split_off_last)<'a>(self: &mut &'a [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Removes the last element of the slice and returns a reference to it.

Returns `None` if the slice is empty.

##### Examples

```rust
let mut slice: &[_] = &['a', 'b', 'c'];
let last = slice.split_off_last().unwrap();

assert_eq!(slice, &['a', 'b']);
assert_eq!(last, &'c');
```

1.87.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5087)

#### pub fn [split\_off\_last\_mut](#method.split_off_last_mut)<'a>(self: &mut &'a mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Removes the last element of the slice and returns a mutable reference to it.

Returns `None` if the slice is empty.

##### Examples

```rust
let mut slice: &mut [_] = &mut ['a', 'b', 'c'];
let last = slice.split_off_last_mut().unwrap();
*last = 'd';

assert_eq!(slice, &['a', 'b']);
assert_eq!(last, &'d');
```

1.86.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5144-5149)

#### pub unsafe fn [get\_disjoint\_unchecked\_mut](#method.get_disjoint_unchecked_mut)<I, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, indices: [\[I; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html), ) -> \[&mut <I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where I: [GetDisjointMutIndex](https://doc.rust-lang.org/nightly/core/slice/trait.GetDisjointMutIndex.html "trait core::slice::GetDisjointMutIndex") + [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>,

Returns mutable references to many indices at once, without doing any checks.

An index can be either a `usize`, a [`Range`](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range") or a [`RangeInclusive`](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive"). Note that this method takes an array, so all indices must be of the same type. If passed an array of `usize`s this method gives back an array of mutable references to single elements, while if passed an array of ranges it gives back an array of mutable references to slices.

For a safe alternative see [`get_disjoint_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.get_disjoint_mut "method slice::get_disjoint_mut").

##### Safety

Calling this method with overlapping or out-of-bounds indices is _[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)_ even if the resulting references are not used.

##### Examples

```rust
let x = &mut [1, 2, 4];

unsafe {
    let [a, b] = x.get_disjoint_unchecked_mut([0, 2]);
    *a *= 10;
    *b *= 100;
}
assert_eq!(x, &[10, 2, 400]);

unsafe {
    let [a, b] = x.get_disjoint_unchecked_mut([0..1, 1..3]);
    a[0] = 8;
    b[0] = 88;
    b[1] = 888;
}
assert_eq!(x, &[8, 88, 888]);

unsafe {
    let [a, b] = x.get_disjoint_unchecked_mut([1..=2, 0..=0]);
    a[0] = 11;
    a[1] = 111;
    b[0] = 1;
}
assert_eq!(x, &[1, 11, 111]);
```

1.86.0 · [Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5211-5216)

#### pub fn [get\_disjoint\_mut](#method.get_disjoint_mut)<I, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, indices: [\[I; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[&mut <I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [GetDisjointMutError](https://doc.rust-lang.org/nightly/core/slice/enum.GetDisjointMutError.html "enum core::slice::GetDisjointMutError")\>

where I: [GetDisjointMutIndex](https://doc.rust-lang.org/nightly/core/slice/trait.GetDisjointMutIndex.html "trait core::slice::GetDisjointMutIndex") + [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>,

Returns mutable references to many indices at once.

An index can be either a `usize`, a [`Range`](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range") or a [`RangeInclusive`](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive"). Note that this method takes an array, so all indices must be of the same type. If passed an array of `usize`s this method gives back an array of mutable references to single elements, while if passed an array of ranges it gives back an array of mutable references to slices.

Returns an error if any index is out-of-bounds, or if there are overlapping indices. An empty range is not considered to overlap if it is located at the beginning or at the end of another range, but is considered to overlap if it is located in the middle.

This method does a O(n^2) check to check that there are no overlapping indices, so be careful when passing many indices.

##### Examples

```rust
let v = &mut [1, 2, 3];
if let Ok([a, b]) = v.get_disjoint_mut([0, 2]) {
    *a = 413;
    *b = 612;
}
assert_eq!(v, &[413, 2, 612]);

if let Ok([a, b]) = v.get_disjoint_mut([0..1, 1..3]) {
    a[0] = 8;
    b[0] = 88;
    b[1] = 888;
}
assert_eq!(v, &[8, 88, 888]);

if let Ok([a, b]) = v.get_disjoint_mut([1..=2, 0..=0]) {
    a[0] = 11;
    a[1] = 111;
    b[0] = 1;
}
assert_eq!(v, &[1, 11, 111]);
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

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5359)

#### pub fn [as\_mut\_slice](#method.as_mut_slice)(&mut self) -> &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

🔬This is a nightly-only experimental API. (`str_as_str`)

Returns the same slice `&mut [T]`.

This method is redundant when used directly on `&mut [T]`, but it helps dereferencing other “container” types to slices, for example `Box<[T]>` or `MutexGuard<[T]>`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#131-133)

#### pub fn [sort](#method.sort)(&mut self)

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **non-`no_global_oom_handling`** only.

Sorts the slice in ascending order, preserving initial order of equal elements.

This sort is stable (i.e., does not reorder equal elements) and _O_(_n_ \* log(_n_)) worst-case.

If the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

When applicable, unstable sorting is preferred because it is generally faster than stable sorting and it doesn’t allocate auxiliary memory. See [`sort_unstable`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable "method slice::sort_unstable"). The exception are partially sorted slices, which may be better served with `slice::sort`.

Sorting types that only implement [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") such as [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") and [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64") require additional precautions. For example, `f32::NAN != f32::NAN`, which doesn’t fulfill the reflexivity requirement of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"). By using an alternative comparison function with `slice::sort_by` such as [`f32::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.total_cmp "method f32::total_cmp") or [`f64::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.total_cmp "method f64::total_cmp") that defines a [total order](https://en.wikipedia.org/wiki/Total_order) users can sort slices containing floating-point values. Alternatively, if all values in the slice are guaranteed to be in a subset for which [`PartialOrd::partial_cmp`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp "method core::cmp::PartialOrd::partial_cmp") forms a [total order](https://en.wikipedia.org/wiki/Total_order), it’s possible to sort the slice with `sort_by(|a, b| a.partial_cmp(b).unwrap())`.

##### Current implementation

The current implementation is based on [driftsort](https://github.com/Voultapher/driftsort) by Orson Peters and Lukas Bergdoll, which combines the fast average case of quicksort with the fast worst case and partial run detection of mergesort, achieving linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the expected time to sort the data is _O_(_n_ \* log(_k_)).

The auxiliary memory allocation behavior depends on the input length. Short slices are handled without allocation, medium sized slices allocate `self.len()` and beyond that it clamps at `self.len() / 2`.

##### Panics

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") implementation itself panics.

All safe functions on slices preserve the invariant that even if the function panics, all original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. This ensures that recovery code (for instance inside of a `Drop` or following a `catch_unwind`) will still have access to all the original elements. For instance, if the slice belongs to a `Vec`, the `Vec::drop` method will be able to dispose of all contained elements.

##### Examples

```rust
let mut v = [4, -5, 1, -3, 2];

v.sort();
assert_eq!(v, [-5, -3, 1, 2, 4]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#192-194)

#### pub fn [sort\_by](#method.sort_by)<F>(&mut self, compare: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Available on **non-`no_global_oom_handling`** only.

Sorts the slice in ascending order with a comparison function, preserving initial order of equal elements.

This sort is stable (i.e., does not reorder equal elements) and _O_(_n_ \* log(_n_)) worst-case.

If the comparison function `compare` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

For example `|a, b| (a - b).cmp(a)` is a comparison function that is neither transitive nor reflexive nor total, `a < b < c < a` with `a = 1, b = 2, c = 3`. For more information and examples see the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") documentation.

##### Current implementation

The current implementation is based on [driftsort](https://github.com/Voultapher/driftsort) by Orson Peters and Lukas Bergdoll, which combines the fast average case of quicksort with the fast worst case and partial run detection of mergesort, achieving linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the expected time to sort the data is _O_(_n_ \* log(_k_)).

The auxiliary memory allocation behavior depends on the input length. Short slices are handled without allocation, medium sized slices allocate `self.len()` and beyond that it clamps at `self.len() / 2`.

##### Panics

May panic if `compare` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if `compare` itself panics.

All safe functions on slices preserve the invariant that even if the function panics, all original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. This ensures that recovery code (for instance inside of a `Drop` or following a `catch_unwind`) will still have access to all the original elements. For instance, if the slice belongs to a `Vec`, the `Vec::drop` method will be able to dispose of all contained elements.

##### Examples

```rust
let mut v = [4, -5, 1, -3, 2];
v.sort_by(|a, b| a.cmp(b));
assert_eq!(v, [-5, -3, 1, 2, 4]);

// reverse sorting
v.sort_by(|a, b| b.cmp(a));
assert_eq!(v, [4, 2, 1, -3, -5]);
```

1.7.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#247-250)

#### pub fn [sort\_by\_key](#method.sort_by_key)<K, F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> K, K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **non-`no_global_oom_handling`** only.

Sorts the slice in ascending order with a key extraction function, preserving initial order of equal elements.

This sort is stable (i.e., does not reorder equal elements) and _O_(_m_ \* _n_ \* log(_n_)) worst-case, where the key function is _O_(_m_).

If the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

##### Current implementation

The current implementation is based on [driftsort](https://github.com/Voultapher/driftsort) by Orson Peters and Lukas Bergdoll, which combines the fast average case of quicksort with the fast worst case and partial run detection of mergesort, achieving linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the expected time to sort the data is _O_(_n_ \* log(_k_)).

The auxiliary memory allocation behavior depends on the input length. Short slices are handled without allocation, medium sized slices allocate `self.len()` and beyond that it clamps at `self.len() / 2`.

##### Panics

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") implementation or the key-function `f` panics.

All safe functions on slices preserve the invariant that even if the function panics, all original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. This ensures that recovery code (for instance inside of a `Drop` or following a `catch_unwind`) will still have access to all the original elements. For instance, if the slice belongs to a `Vec`, the `Vec::drop` method will be able to dispose of all contained elements.

##### Examples

```rust
let mut v = [4i32, -5, 1, -3, 2];

v.sort_by_key(|k| k.abs());
assert_eq!(v, [1, 2, -3, 4, -5]);
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#312-315)

#### pub fn [sort\_by\_cached\_key](#method.sort_by_cached_key)<K, F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> K, K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **non-`no_global_oom_handling`** only.

Sorts the slice in ascending order with a key extraction function, preserving initial order of equal elements.

This sort is stable (i.e., does not reorder equal elements) and _O_(_m_ \* _n_ + _n_ \* log(_n_)) worst-case, where the key function is _O_(_m_).

During sorting, the key function is called at most once per element, by using temporary storage to remember the results of key evaluation. The order of calls to the key function is unspecified and may change in future versions of the standard library.

If the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

For simple key functions (e.g., functions that are property accesses or basic operations), [`sort_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_by_key "method slice::sort_by_key") is likely to be faster.

##### Current implementation

The current implementation is based on [instruction-parallel-network sort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll, which combines the fast average case of randomized quicksort with the fast worst case of heapsort, while achieving linear time on fully sorted and reversed inputs. And _O_(_k_ \* log(_n_)) where _k_ is the number of distinct elements in the input. It leverages superscalar out-of-order execution capabilities commonly found in CPUs, to efficiently perform the operation.

In the worst case, the algorithm allocates temporary storage in a `Vec<(K, usize)>` the length of the slice.

##### Panics

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") implementation panics.

All safe functions on slices preserve the invariant that even if the function panics, all original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. This ensures that recovery code (for instance inside of a `Drop` or following a `catch_unwind`) will still have access to all the original elements. For instance, if the slice belongs to a `Vec`, the `Vec::drop` method will be able to dispose of all contained elements.

##### Examples

```rust
let mut v = [4i32, -5, 1, -3, 2, 10];

// Strings are sorted by lexicographical order.
v.sort_by_cached_key(|k| k.to_string());
assert_eq!(v, [-3, -5, 1, 10, 2, 4]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#372-374)

#### pub fn [to\_vec](#method.to_vec)(&self) -> [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

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

#### pub fn [to\_vec\_in](#method.to_vec_in)<A>(&self, alloc: A) -> [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T, A>

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

#### pub fn [repeat](#method.repeat)(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

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

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#109)

### impl<'a, T> [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#110)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#111)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T> as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#116)

### impl<'a, T> [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#117)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T> as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#140)

### impl<'a, T> [Drop](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html "trait core::ops::drop::Drop") for [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#141)

#### fn [drop](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#method.drop)(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#method.drop)

[Source](https://doc.rust-lang.org/nightly/src/core/ops/drop.rs.html#267)

#### fn [pin\_drop](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#method.pin_drop)(self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>)

🔬This is a nightly-only experimental API. (`pin_ergonomics`)

Execute the destructor for this type, but different to [`Drop::drop`](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#method.drop "method core::ops::drop::Drop::drop"), it requires `self` to be pinned. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#method.pin_drop)

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#122)

### impl<'a, 'b, T> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'b [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#123)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = [&'b T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#124)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [Iter](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html "struct core::slice::iter::Iter")<'b, T>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#126)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <&'b [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#131)

### impl<'a, 'b, T> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'b mut [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#132)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = [&'b mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#133)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [IterMut](https://doc.rust-lang.org/nightly/core/slice/iter/struct.IterMut.html "struct core::slice::iter::IterMut")<'b, T>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/buffered_channel.rs.html#135)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <&'b mut [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

## Auto Trait Implementations

### impl<'a, T> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

### impl<'a, T> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

where T: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"),

### impl<'a, T> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

### impl<'a, T> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

### impl<'a, T> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

where T: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

### impl<'a, T> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

### impl<'a, T> [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")<'a, T>

where T: [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

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

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

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

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../reflect/trait.Is.html#tymethod.is)

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

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"<\[T\] as Concat<Item>>::Output":"<h3>Notable traits for <code>&amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for &amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for &amp;mut \[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div>","<\[T\] as Join<Separator>>::Output":"<h3>Notable traits for <code>&amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for &amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for &amp;mut \[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div>","Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","ArrayWindows<'\_, T, N>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ArrayWindows.html\\" title=\\"struct core::slice::iter::ArrayWindows\\">ArrayWindows</a>&lt;'a, T, N&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, const N: <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ArrayWindows.html\\" title=\\"struct core::slice::iter::ArrayWindows\\">ArrayWindows</a>&lt;'a, T, N&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.array.html\\">\[T; N\]</a>;</div>","ChunkBy<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkBy.html\\" title=\\"struct core::slice::iter::ChunkBy\\">ChunkBy</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkBy.html\\" title=\\"struct core::slice::iter::ChunkBy\\">ChunkBy</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n T: 'a,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","ChunkByMut<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkByMut.html\\" title=\\"struct core::slice::iter::ChunkByMut\\">ChunkByMut</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkByMut.html\\" title=\\"struct core::slice::iter::ChunkByMut\\">ChunkByMut</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n T: 'a,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","Chunks<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Chunks.html\\" title=\\"struct core::slice::iter::Chunks\\">Chunks</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Chunks.html\\" title=\\"struct core::slice::iter::Chunks\\">Chunks</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","ChunksExact<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExact.html\\" title=\\"struct core::slice::iter::ChunksExact\\">ChunksExact</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExact.html\\" title=\\"struct core::slice::iter::ChunksExact\\">ChunksExact</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","ChunksExactMut<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExactMut.html\\" title=\\"struct core::slice::iter::ChunksExactMut\\">ChunksExactMut</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExactMut.html\\" title=\\"struct core::slice::iter::ChunksExactMut\\">ChunksExactMut</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","ChunksMut<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksMut.html\\" title=\\"struct core::slice::iter::ChunksMut\\">ChunksMut</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksMut.html\\" title=\\"struct core::slice::iter::ChunksMut\\">ChunksMut</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","Drain<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../prelude/vec/struct.Drain.html\\" title=\\"struct bevy::prelude::vec::Drain\\">Drain</a>&lt;'\_, T, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../prelude/vec/struct.Drain.html\\" title=\\"struct bevy::prelude::vec::Drain\\">Drain</a>&lt;'\_, T, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Iter<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html\\" title=\\"struct core::slice::iter::Iter\\">Iter</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html\\" title=\\"struct core::slice::iter::Iter\\">Iter</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>;</div>","IterMut<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.IterMut.html\\" title=\\"struct core::slice::iter::IterMut\\">IterMut</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.IterMut.html\\" title=\\"struct core::slice::iter::IterMut\\">IterMut</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a mut T</a>;</div>","RChunks<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunks.html\\" title=\\"struct core::slice::iter::RChunks\\">RChunks</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunks.html\\" title=\\"struct core::slice::iter::RChunks\\">RChunks</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","RChunksExact<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExact.html\\" title=\\"struct core::slice::iter::RChunksExact\\">RChunksExact</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExact.html\\" title=\\"struct core::slice::iter::RChunksExact\\">RChunksExact</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","RChunksExactMut<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExactMut.html\\" title=\\"struct core::slice::iter::RChunksExactMut\\">RChunksExactMut</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExactMut.html\\" title=\\"struct core::slice::iter::RChunksExactMut\\">RChunksExactMut</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","RChunksMut<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksMut.html\\" title=\\"struct core::slice::iter::RChunksMut\\">RChunksMut</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksMut.html\\" title=\\"struct core::slice::iter::RChunksMut\\">RChunksMut</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","RSplit<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplit.html\\" title=\\"struct core::slice::iter::RSplit\\">RSplit</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplit.html\\" title=\\"struct core::slice::iter::RSplit\\">RSplit</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","RSplitMut<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitMut.html\\" title=\\"struct core::slice::iter::RSplitMut\\">RSplitMut</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitMut.html\\" title=\\"struct core::slice::iter::RSplitMut\\">RSplitMut</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","RSplitN<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitN.html\\" title=\\"struct core::slice::iter::RSplitN\\">RSplitN</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitN.html\\" title=\\"struct core::slice::iter::RSplitN\\">RSplitN</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","RSplitNMut<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitNMut.html\\" title=\\"struct core::slice::iter::RSplitNMut\\">RSplitNMut</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitNMut.html\\" title=\\"struct core::slice::iter::RSplitNMut\\">RSplitNMut</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","Range<\*const T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html\\" title=\\"struct core::ops::range::Range\\">Range</a>&lt;A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html\\" title=\\"struct core::ops::range::Range\\">Range</a>&lt;A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/range/trait.Step.html\\" title=\\"trait core::iter::range::Step\\">Step</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = A;</div>","Range<\*mut T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html\\" title=\\"struct core::ops::range::Range\\">Range</a>&lt;A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html\\" title=\\"struct core::ops::range::Range\\">Range</a>&lt;A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/range/trait.Step.html\\" title=\\"trait core::iter::range::Step\\">Step</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = A;</div>","Split<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Split.html\\" title=\\"struct core::slice::iter::Split\\">Split</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Split.html\\" title=\\"struct core::slice::iter::Split\\">Split</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","SplitInclusive<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusive.html\\" title=\\"struct core::slice::iter::SplitInclusive\\">SplitInclusive</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusive.html\\" title=\\"struct core::slice::iter::SplitInclusive\\">SplitInclusive</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","SplitInclusiveMut<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusiveMut.html\\" title=\\"struct core::slice::iter::SplitInclusiveMut\\">SplitInclusiveMut</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusiveMut.html\\" title=\\"struct core::slice::iter::SplitInclusiveMut\\">SplitInclusiveMut</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","SplitMut<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitMut.html\\" title=\\"struct core::slice::iter::SplitMut\\">SplitMut</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitMut.html\\" title=\\"struct core::slice::iter::SplitMut\\">SplitMut</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","SplitN<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitN.html\\" title=\\"struct core::slice::iter::SplitN\\">SplitN</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitN.html\\" title=\\"struct core::slice::iter::SplitN\\">SplitN</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","SplitNMut<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitNMut.html\\" title=\\"struct core::slice::iter::SplitNMut\\">SplitNMut</a>&lt;'a, T, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitNMut.html\\" title=\\"struct core::slice::iter::SplitNMut\\">SplitNMut</a>&lt;'a, T, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a mut <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","Windows<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Windows.html\\" title=\\"struct core::slice::iter::Windows\\">Windows</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/slice/iter/struct.Windows.html\\" title=\\"struct core::slice::iter::Windows\\">Windows</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\\">\[T\]</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}